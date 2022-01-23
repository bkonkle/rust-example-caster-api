use async_graphql::{Context, ErrorExtensions, Object, Result};
use hyper::StatusCode;
use std::sync::Arc;

use crate::{
    user_model::User,
    user_mutations::{CreateUserInput, UpdateUserInput},
    users_service::{Unique::Username, UsersService},
};
use caster_auth::Subject;
use caster_utils::errors::{as_graphql_error, graphql_error};

/// The Query segment owned by the Users library
#[derive(Default)]
pub struct UsersQuery {}

#[Object]
impl UsersQuery {
    async fn get_current_user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let users = ctx.data_unchecked::<Arc<dyn UsersService>>();
        let subject = ctx.data::<Subject>().ok();

        match subject {
            Some(Subject(Some(username))) => {
                users
                    .get(&Username(username.clone()))
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
        }
    }

    async fn get_or_create_current_user(
        &self,
        ctx: &Context<'_>,
        input: CreateUserInput,
    ) -> Result<User> {
        let users = ctx.data_unchecked::<Arc<dyn UsersService>>();
        let subject = ctx.data::<Subject>().ok();

        let username = match subject {
            Some(Subject(Some(username))) => Ok(username),
            _ => Err(graphql_error(
                "A valid JWT token is required",
                StatusCode::UNAUTHORIZED,
            )),
        }?;

        users.create(username, &input).await.map_err(|e| e.into())
    }

    async fn update_current_user(&self, ctx: &Context<'_>, input: UpdateUserInput) -> Result<User> {
        let users = ctx.data_unchecked::<Arc<dyn UsersService>>();
        let subject = ctx.data::<Subject>().ok();

        let existing =
            match subject {
                Some(Subject(Some(username))) => users
                    .get(&Username(username.clone()))
                    .await
                    .map_err(as_graphql_error(
                        "Error while retrieving existing User",
                        StatusCode::INTERNAL_SERVER_ERROR,
                    )),
                _ => Err(anyhow!("Unauthorized").extend_with(|_err, e| e.set("code", 401))),
            }?;

        match existing {
            Some(existing) => users
                .update(&existing.id, &input)
                .await
                .map_err(as_graphql_error(
                    "Error while updating User",
                    StatusCode::INTERNAL_SERVER_ERROR,
                )),
            _ => Err(graphql_error(
                "No existing User found for the current JWT token",
                StatusCode::BAD_REQUEST,
            )),
        }
    }
}
