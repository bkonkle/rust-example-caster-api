use async_graphql::Result;
use hyper::StatusCode;
use std::sync::Arc;

use crate::{user_model::User, users_service::UsersService};
use caster_auth::Subject;
use caster_utils::errors::{as_graphql_error, graphql_error};

/// Get an existing User based on the token username (the "sub" claim)
pub async fn get_current_user(subject: &Subject, users: &Arc<dyn UsersService>) -> Result<User> {
    match subject {
        Subject(Some(username)) => {
            users
                .get_by_username(username, &false)
                .await
                .map_err(as_graphql_error(
                    "Error while retrieving User",
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
        }
        _ => Err(graphql_error(
            "A valid JWT token is required",
            StatusCode::UNAUTHORIZED,
        )),
    }?
    .ok_or_else(|| {
        graphql_error(
            "A currently logged-in User is required",
            StatusCode::UNAUTHORIZED,
        )
    })
}

/// Optionally get an existing User based on the token username (the "sub" claim)
pub async fn maybe_get_current_user(
    subject: &Subject,
    users: &Arc<dyn UsersService>,
) -> Result<Option<User>> {
    match subject {
        Subject(Some(username)) => {
            users
                .get_by_username(username, &false)
                .await
                .map_err(as_graphql_error(
                    "Error while retrieving User",
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
        }
        _ => Ok(None),
    }
}
