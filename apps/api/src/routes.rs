use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Data, EmptyMutation, EmptySubscription, Schema};
use async_graphql_warp::{graphql_subscription_with_data, BadRequest, Response};
use sqlx::PgPool;
use std::convert::Infallible;
use warp::{filters::BoxedFilter, Filter, Reply};
use warp::{http::Response as HttpResponse, http::StatusCode, Rejection};

use crate::graphql::{create_schema, GraphQLSchema, Query};

struct AuthToken(String);

/// Create a Warp filter to handle GraphQL routing based on the given `GraphQLSchema`.
pub fn create_filter(schema: GraphQLSchema) -> BoxedFilter<(impl Reply,)> {
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

    graphql_subscription_with_data(schema, |value| async {
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
    })
    .boxed()
}

/// A convenience wrapper to create a Warp filter from the base `Schema` requirements.
pub fn create_routes(pg_pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    create_filter(create_schema(pg_pool))
}
