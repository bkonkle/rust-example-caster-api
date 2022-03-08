use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_warp::{graphql, GraphQLResponse};
use caster_users::users_service::UsersService;
use std::{convert::Infallible, sync::Arc};
use warp::{http::Response as HttpResponse, Rejection};
use warp::{Filter, Reply};

use crate::graphql::{Mutation, Query};
use caster_auth::{jwks::JWKS, with_auth, Subject};

/// Create a Warp filter to handle GraphQL routing based on the given `GraphQLSchema`.
pub fn create_routes(
    users: Arc<dyn UsersService>,
    schema: Schema<Query, Mutation, EmptySubscription>,
    jwks: &'static JWKS,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let graphql_post = graphql(schema).and(with_auth(jwks)).and_then(
        move |(schema, request): (
            Schema<Query, Mutation, EmptySubscription>,
            async_graphql::Request,
        ),
              sub| async move {
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
        },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    graphql_playground.or(graphql_post)
}
