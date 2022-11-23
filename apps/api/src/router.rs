use std::sync::Arc;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::{Extension, WebSocketUpgrade},
    response::{Html, IntoResponse, Response},
};
use serde_json::json;

use crate::{events, graphql::GraphQLSchema, Context};
use caster_auth::authenticate::Subject;

// Health
// ------

/// Handle health check requests
pub async fn health_handler() -> impl IntoResponse {
    json!({
        "code": "200",
        "success": true,
    })
    .to_string()
}

// GraphQL
// -------

/// Handle GraphiQL Requests
pub async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

/// Handle GraphQL Requests
pub async fn graphql_handler(
    Extension(schema): Extension<GraphQLSchema>,
    Extension(ctx): Extension<Arc<Context>>,
    sub: Subject,
    req: GraphQLRequest,
) -> GraphQLResponse {
    // Retrieve the request User, if username is present
    let user = if let Subject(Some(ref username)) = sub {
        ctx.users
            .get_by_username(username, &true)
            .await
            .unwrap_or(None)
    } else {
        None
    };

    // Add the Subject and optional User to the context
    let request = req.into_inner().data(sub).data(user);

    schema.execute(request).await.into()
}

// WebSocket
// ---------

/// Handle WebSocket upgrade requests
pub async fn events_handler(
    Extension(ctx): Extension<Arc<Context>>,
    sub: Subject,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(|socket| events::handler::handle(socket, ctx, sub))
}
