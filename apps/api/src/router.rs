use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_warp::{graphql, GraphQLResponse};
use sqlx::PgPool;
use std::{convert::Infallible, sync::Arc};
use warp::{http::Response as HttpResponse, Rejection};
use warp::{Filter, Reply};

use crate::graphql::{create_schema, Mutation, Query};
use caster_auth::{jwks::JWKS, with_auth};
use caster_utils::config::Config;

/// Create a Warp filter to handle GraphQL routing based on the given `GraphQLSchema`.
pub fn create_routes(
    pool: &Arc<PgPool>,
    config: &'static Config,
    jwks: &'static JWKS,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let schema = create_schema(pool, config);

    let graphql_post = graphql(schema).and(with_auth(jwks)).and_then(
        |(schema, request): (
            Schema<Query, Mutation, EmptySubscription>,
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

    graphql_playground.or(graphql_post)
}
