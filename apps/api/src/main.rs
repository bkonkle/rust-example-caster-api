//! # A GraphQL server written in Rust
#![forbid(unsafe_code)]

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};
use async_graphql_warp::{BadRequest, Response};
use dotenv::dotenv;
use sqlx::Error;
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use warp::Filter;
use warp::{http::Response as HttpResponse, http::StatusCode, Rejection};

use caster_shows::shows_resolver::ShowsQuery;
use caster_users::users_resolver::UsersQuery;

#[derive(MergedObject, Default)]
struct Query(UsersQuery, ShowsQuery);

mod db;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".into());
    let addr = format!("http://localhost:{port}", port = port);

    let pg_pool = db::init().await?;

    let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(pg_pool)
        .finish();

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
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

    let routes = graphql_playground
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
