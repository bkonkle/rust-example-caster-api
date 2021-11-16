//! # A GraphQL server written in Rust
#![forbid(unsafe_code)]

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Data, EmptyMutation, EmptySubscription, MergedObject, Schema};
use async_graphql_warp::{graphql_subscription_with_data, BadRequest, Response};
use dotenv::dotenv;
use sqlx::Error;
use std::env;
use std::net::SocketAddr;
use std::{convert::Infallible, sync::Arc};
use warp::Filter;
use warp::{http::Response as HttpResponse, http::StatusCode, Rejection};

use caster_shows::shows_service::ShowsService;
use caster_shows::{shows_repository::PgShowsRepository, shows_resolver::ShowsQuery};
use caster_users::users_service::UsersService;
use caster_users::{users_repository::PgUsersRepository, users_resolver::UsersQuery};

mod postgres;

struct AuthToken(String);

#[derive(MergedObject, Default)]
struct Query(UsersQuery, ShowsQuery);

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| String::from("3000"));
    let addr = format!("http://localhost:{port}", port = port);

    let pg_pool = Arc::new(postgres::init().await?);
    let shows_repo = Arc::new(PgShowsRepository::new(&pg_pool));
    let users_repo = Arc::new(PgUsersRepository::new(&pg_pool));
    let shows = ShowsService::new(&shows_repo);
    let users = UsersService::new(&users_repo);

    let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(shows)
        .data(users)
        .finish();

    let graphql_post = async_graphql_warp::graphql(schema.clone()).and_then(
        |(schema, request): (
            Schema<Query, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        )| async move { Ok::<_, Infallible>(Response::from(schema.execute(request).await)) },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    let routes = graphql_subscription_with_data(schema, |value| async {
        #[derive(serde_derive::Deserialize)]
        struct Payload {
            token: String,
        }

        if let Ok(payload) = serde_json::from_value::<Payload>(value) {
            let mut data = Data::default();
            data.insert(AuthToken(payload.token));
            Ok(data)
        } else {
            Err("An auth token is required".into())
        }
    })
    .or(graphql_playground)
    .or(graphql_post)
    .recover(|err: Rejection| async move {
        if let Some(BadRequest(err)) = err.find() {
            return Ok::<_, Infallible>(warp::reply::with_status(
                err.to_string(),
                StatusCode::BAD_REQUEST,
            ));
        }

        Ok(warp::reply::with_status(
            "INTERNAL_SERVER_ERROR".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    });

    println!("[Caster] Started at: {addr}", addr = addr);

    let socket_addr: SocketAddr = match addr.parse() {
        Ok(file) => file,
        Err(_) => ([0, 0, 0, 0], 3000).into(),
    };

    warp::serve(routes).run(socket_addr).await;

    Ok(())
}
