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
    /// Get the current User from the GraphQL context
    async fn get_current_user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let user = ctx.data_unchecked::<Option<User>>();

        Ok(user.clone())
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
        let user = ctx.data_unchecked::<Option<User>>();
        let users = ctx.data_unchecked::<Arc<dyn UsersService>>();
        let profiles = ctx.data_unchecked::<Arc<dyn ProfilesService>>();
        let subject = ctx.data_unchecked::<Subject>();

        // If the User exists in the GraphQL context, simply return it
        if let Some(user) = user {
            return Ok(MutateUserResult {
                user: Some(user.clone()),
            });
        }

        // Otherwise, check for a username so that it can be created
        let username = match subject {
            Subject(Some(username)) => Ok(username),
            _ => Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED)),
        }?;

        let user = users.create(username).await.map_err(as_graphql_error(
            "Eror while creating User",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))?;

        if let Some(profile) = input.profile {
            profiles
                .get_or_create(
                    &user.id,
                    &CreateProfileInput {
                        email: profile.email,
                        display_name: profile.display_name,
                        picture: profile.picture,
                        content: profile.content,
                        city: profile.city,
                        state_province: profile.state_province,
                        user_id: user.id.clone(),
                    },
                    &false,
                )
                .await?;
        }

        Ok(MutateUserResult { user: Some(user) })
    }

    /// Update the current User based on the current token username (the "sub" claim)
    async fn update_current_user(
        &self,
        ctx: &Context<'_>,
        input: UpdateUserInput,
    ) -> Result<MutateUserResult> {
        let user = ctx.data_unchecked::<Option<User>>();
        let users = ctx.data_unchecked::<Arc<dyn UsersService>>();

        // Check to see if the associated Profile is selected
        let with_roles = ctx.look_ahead().field("user").field("roles").exists();

        if let Some(user) = user {
            let updated =
                users
                    .update(&user.id, &input, &with_roles)
                    .await
                    .map_err(as_graphql_error(
                        "Error while updating User",
                        StatusCode::INTERNAL_SERVER_ERROR,
                    ))?;

            return Ok(MutateUserResult {
                user: Some(updated),
            });
        }

        Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED))
    }
}
