use async_graphql::{Context, Object, Result};
use hyper::StatusCode;
use std::sync::Arc;

use crate::{
    profile_mutations::CreateProfileInput,
    profiles_service::ProfilesService,
    user_model::User,
    user_mutations::{CreateUserInput, MutateUserResult, UpdateUserInput},
    users_service::UsersService,
};
use caster_auth::Subject;
use caster_utils::errors::{as_graphql_error, graphql_error};

/// The Query segment for Users
#[derive(Default)]
pub struct UsersQuery {}

/// The Mutation segment for Users
#[derive(Default)]
pub struct UsersMutation {}

/// Queries for the User model
#[Object]
impl UsersQuery {
    /// Get the current User based on the current token username (the "sub" claim)
    async fn get_current_user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let users = ctx.data_unchecked::<Arc<dyn UsersService>>();
        let subject = ctx.data_unchecked::<Subject>();

        let with_profile = ctx.look_ahead().field("profile").exists();

        match subject {
            Subject(Some(username)) => users
                .get_by_username(username, &with_profile)
                .await
                .map_err(as_graphql_error(
                    "Error while retrieving User",
                    StatusCode::INTERNAL_SERVER_ERROR,
                )),
            _ => Err(graphql_error(
                "A valid JWT token is required",
                StatusCode::UNAUTHORIZED,
            )),
        }
    }
}

/// Mutations for the User model
#[Object]
impl UsersMutation {
    /// Get or create the current User based on the current token username (the "sub" claim)
    async fn get_or_create_current_user(
        &self,
        ctx: &Context<'_>,
        input: CreateUserInput,
    ) -> Result<MutateUserResult> {
        let users = ctx.data_unchecked::<Arc<dyn UsersService>>();
        let profiles = ctx.data_unchecked::<Arc<dyn ProfilesService>>();
        let subject = ctx.data_unchecked::<Subject>();

        // Check to see if the associated Profile is selected
        let with_profile = ctx.look_ahead().field("user").field("profile").exists();

        let username = match subject {
            Subject(Some(username)) => Ok(username),
            _ => Err(graphql_error(
                "A valid JWT token is required",
                StatusCode::UNAUTHORIZED,
            )),
        }?;

        let mut user =
            users
                .get_or_create(username, &with_profile)
                .await
                .map_err(as_graphql_error(
                    "Eror while creating User",
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))?;

        if let Some(profile) = input.profile {
            let created = profiles
                .get_or_create(
                    &user.id,
                    &CreateProfileInput {
                        user_id: Some(user.id.clone()),
                        ..profile
                    },
                    &false,
                )
                .await?;

            // Add the created Profile to the result
            user.profile = Some(created);
        }

        Ok(MutateUserResult { user: Some(user) })
    }

    /// Update the current User based on the current token username (the "sub" claim)
    async fn update_current_user(
        &self,
        ctx: &Context<'_>,
        input: UpdateUserInput,
    ) -> Result<MutateUserResult> {
        let users = ctx.data_unchecked::<Arc<dyn UsersService>>();
        let subject = ctx.data_unchecked::<Subject>();

        // Check to see if the associated Profile is selected
        let with_profile = ctx.look_ahead().field("user").field("profile").exists();

        let existing = match subject {
            Subject(Some(username)) => {
                users
                    .get_by_username(username, &false)
                    .await
                    .map_err(as_graphql_error(
                        "Error while retrieving existing User",
                        StatusCode::INTERNAL_SERVER_ERROR,
                    ))
            }
            _ => Err(graphql_error(
                "A valid JWT token is required",
                StatusCode::UNAUTHORIZED,
            )),
        }?;

        let user = match existing {
            Some(existing) => users
                .update(&existing.id, &input, &with_profile)
                .await
                .map_err(as_graphql_error(
                    "Error while updating User",
                    StatusCode::INTERNAL_SERVER_ERROR,
                )),
            _ => Err(graphql_error(
                "No existing User found for the current JWT token",
                StatusCode::BAD_REQUEST,
            )),
        }?;

        Ok(MutateUserResult { user: Some(user) })
    }
}
