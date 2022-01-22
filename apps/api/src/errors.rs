use std::convert::Infallible;

use async_graphql_warp::GraphQLBadRequest;
use hyper::StatusCode;
use serde::Serialize;
use warp::{reply, Rejection, Reply};

use caster_auth::errors::{from_auth_error, AuthError};

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

/// Handle any expected Warp rejections
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not found".to_string();
    } else if let Some(e) = err.find::<AuthError>() {
        let (auth_message, auth_code) = from_auth_error(e);
        code = auth_code;
        message = auth_message;
    } else if let Some(GraphQLBadRequest(err)) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = format!("GraphQLBadRequest: {}", err);
    } else {
        debug!("Unhandled Rejection: {:?}", err);

        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal server error".to_string();
    }

    let json = reply::json(&ErrorMessage {
        code: code.as_u16(),
        message,
    });

    Ok(reply::with_status(json, code))
}
