use async_graphql::{ComplexObject, Context, Object, Result};
use hyper::StatusCode;
use std::sync::Arc;

use crate::{
    profile_model::Profile,
    profile_mutations::CreateProfileInput,
    profiles_service::ProfilesService,
    user_model::User,
    user_mutations::{CreateUserInput, MutateUserResult, UpdateUserInput},
    users_service::UsersService,
};
use caster_auth::Subject;
use caster_utils::errors::{as_graphql_error, graphql_error};

/// The Query segment owned by the Users library
#[derive(Default)]
pub struct UsersQuery {}

/// The Mutation segment owned by the Users library
#[derive(Default)]
pub struct UsersMutation {}

/// Resolver fields for the User model
#[ComplexObject]
impl User {
    async fn profile(&self, ctx: &Context<'_>) -> Result<Option<Profile>> {
        let profiles = ctx.data_unchecked::<Arc<dyn ProfilesService>>();

        let result = profiles
            .get_by_user_id(&self.id)
            .await
            .map_err(as_graphql_error(
                "Eror while retrieving Profile",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        Ok(result)
    }
}

/// Queries for the User model
#[Object]
impl UsersQuery {
    async fn get_current_user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let users = ctx.data_unchecked::<Arc<dyn UsersService>>();
        let subject = ctx.data_unchecked::<Subject>();

        match subject {
            Subject(Some(username)) => {
                users
                    .get_by_username(username)
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
}

/// Mutations for the User model
#[Object]
impl UsersMutation {
    async fn get_or_create_current_user(
        &self,
        ctx: &Context<'_>,
        input: CreateUserInput,
    ) -> Result<MutateUserResult> {
        let users = ctx.data_unchecked::<Arc<dyn UsersService>>();
        let profiles = ctx.data_unchecked::<Arc<dyn ProfilesService>>();
        let subject = ctx.data_unchecked::<Subject>();

        let username = match subject {
            Subject(Some(username)) => Ok(username),
            _ => Err(graphql_error(
                "A valid JWT token is required",
                StatusCode::UNAUTHORIZED,
            )),
        }?;

        let user = users
            .get_or_create(username)
            .await
            .map_err(as_graphql_error(
                "Eror while creating User",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        if let Some(profile) = input.profile {
            profiles
                .get_or_create(
                    &user.id,
                    &CreateProfileInput {
                        user_id: Some(user.id.clone()),
                        ..profile
                    },
                )
                .await?;
        }

        Ok(MutateUserResult { user: Some(user) })
    }

    async fn update_current_user(
        &self,
        ctx: &Context<'_>,
        input: UpdateUserInput,
    ) -> Result<MutateUserResult> {
        let users = ctx.data_unchecked::<Arc<dyn UsersService>>();
        let subject = ctx.data_unchecked::<Subject>();

        let existing = match subject {
            Subject(Some(username)) => {
                users
                    .get_by_username(username)
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
        }?;

        Ok(MutateUserResult { user: Some(user) })
    }
}
