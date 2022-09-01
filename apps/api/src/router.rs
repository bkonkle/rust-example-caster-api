use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_warp::{graphql, GraphQLResponse};
use serde_json::json;
use std::{convert::Infallible, sync::Arc};
use warp::{http::Response as HttpResponse, Rejection};
use warp::{Filter, Reply};

use crate::{
    graphql::{Mutation, Query},
    Context,
};
use caster_auth::{
    authenticate::{with_auth, Subject},
    jwks::JWKS,
};
use caster_domains::users::service::UsersServiceTrait;

/// Create a Warp filter to handle GraphQL routing based on the given `GraphQLSchema`.
pub fn create_routes(
    ctx: &Arc<Context>,
    schema: Schema<Query, Mutation, EmptySubscription>,
    jwks: &'static JWKS,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    health().or(gql(ctx, schema, jwks))
}

// Add the context to the handler
fn with_context(
    ctx: &Arc<Context>,
) -> impl Filter<Extract = (Arc<Context>,), Error = std::convert::Infallible> + Clone {
    let context = ctx.clone();

    warp::any().map(move || context.clone())
}

// Add the Users service to the handler
fn with_users(
    ctx: &Arc<Context>,
) -> impl Filter<Extract = (Arc<dyn UsersServiceTrait>,), Error = std::convert::Infallible> + Clone
{
    let context = ctx.clone();

    warp::any().map(move || context.users.clone())
}

// Health
// ------

/// The route for health checking
pub fn health() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("health")
        .and(warp::get())
        .and_then(health_handler)
}

pub async fn health_handler() -> Result<impl Reply, Infallible> {
    let res = json!({
        "code": "200",
        "success": true,
    });

    Ok(warp::reply::json(&res))
}

// GraphQL
// -------

pub fn gql(
    ctx: &Arc<Context>,
    schema: Schema<Query, Mutation, EmptySubscription>,
    jwks: &'static JWKS,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let graphql_post = graphql(schema)
        // Add the context to the handler
        .and(with_context(ctx))
        // Add the UsersService to the request handler
        .and(with_users(ctx))
        // Add the Subject to the request handler
        .and(with_auth(jwks))
        // Execute the GraphQL request
        .and_then(execute);

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    graphql_playground.or(graphql_post)
}

/// Execute the GraphQL request
async fn execute(
    (schema, request): (
        Schema<Query, Mutation, EmptySubscription>,
        async_graphql::Request,
    ),
    _ctx: Arc<Context>,
    users: Arc<dyn UsersServiceTrait>,
    sub: Subject,
) -> Result<GraphQLResponse, Infallible> {
    // Retrieve the request User, if username is present
    let user = if let Subject(Some(ref username)) = sub {
        users.get_by_username(username, &true).await.unwrap_or(None)
    } else {
        None
    };

    // Add the Subject and optional User to the context
    let request = request.data(sub).data(user);

    let response = schema.execute(request).await;

    Ok::<_, Infallible>(GraphQLResponse::from(response))
}
