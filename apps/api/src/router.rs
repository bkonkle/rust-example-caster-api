use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_warp::{graphql, GraphQLBadRequest, GraphQLResponse};
use sqlx::PgPool;
use std::{convert::Infallible, sync::Arc};
use warp::{http::Response as HttpResponse, http::StatusCode, Rejection};
use warp::{Filter, Reply};

use crate::graphql::{create_schema, GraphQLSchema, Query};
use caster_auth::with_auth;
use caster_utils::config::Config;

/// Create a Warp filter to handle GraphQL routing based on the given `GraphQLSchema`.
pub fn create_filter(
    schema: GraphQLSchema,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let graphql_post = graphql(schema).and(with_auth()).and_then(
        |(schema, request): (
            Schema<Query, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        ),
         sub| async move {
            Ok::<_, Infallible>(GraphQLResponse::from(
                schema.execute(request.data(sub)).await,
            ))
        },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    graphql_playground
        .or(graphql_post)
        .recover(|err: Rejection| async move {
            if let Some(GraphQLBadRequest(err)) = err.find() {
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
pub fn create_routes(
    pg_pool: Arc<PgPool>,
    config: Arc<Config>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    create_filter(create_schema(pg_pool, config))
}
