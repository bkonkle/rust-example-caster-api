use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_warp::{graphql, GraphQLResponse};
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
use caster_users::users_service::UsersServiceTrait;

/// Add context to the GraphQL Request
async fn with_context(
    (schema, request): (
        Schema<Query, Mutation, EmptySubscription>,
        async_graphql::Request,
    ),
    sub: Subject,
    users: Arc<dyn UsersServiceTrait>,
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

/// Create a Warp filter to handle GraphQL routing based on the given `GraphQLSchema`.
pub fn create_routes(
    context: Arc<Context>,
    schema: Schema<Query, Mutation, EmptySubscription>,
    jwks: &'static JWKS,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let graphql_post = graphql(schema)
        // Add the Subject to the request handler
        .and(with_auth(jwks))
        // Add the UsersService to the request handler
        .and(warp::any().map(move || context.users.clone()))
        // Add details to the GraphQL request context
        .and_then(with_context);

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    graphql_playground.or(graphql_post)
}
