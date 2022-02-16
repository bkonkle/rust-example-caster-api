use async_graphql::{Context, Object, Result};
use caster_auth::Subject;
use caster_utils::errors::{as_graphql_error, graphql_error};
use hyper::StatusCode;
use std::sync::Arc;

use crate::{
    profile_model::Profile,
    profile_mutations::{CreateProfileInput, MutateProfileResult, UpdateProfileInput},
    profile_queries::{ProfileCondition, ProfilesOrderBy, ProfilesPage},
    profiles_service::ProfilesService,
    users_service::UsersService,
};

/// The Query segment for Profiles
#[derive(Default)]
pub struct ProfilesQuery {}

/// The Mutation segment for Profiles
#[derive(Default)]
pub struct ProfilesMutation {}

/// Queries for the `User` model
#[Object]
impl ProfilesQuery {
    /// Get a single Profile
    async fn get_profile(&self, ctx: &Context<'_>, id: String) -> Result<Option<Profile>> {
        let users = ctx.data_unchecked::<Arc<dyn UsersService>>();
        let profiles = ctx.data_unchecked::<Arc<dyn ProfilesService>>();
        let subject = ctx.data_unchecked::<Subject>();

        // Retrieve the current request User for authorization
        let user = match subject {
            Subject(Some(username)) => {
                users
                    .get_by_username(username, &false)
                    .await
                    .map_err(as_graphql_error(
                        "Error while retrieving Profile",
                        StatusCode::INTERNAL_SERVER_ERROR,
                    ))
            }
            _ => Err(graphql_error(
                "A valid JWT token is required",
                StatusCode::UNAUTHORIZED,
            )),
        }?;

        let profile = profiles.get(&id, &false).await?;

        // Use the request User to decide if the Profile should be censored
        let censored = match user {
            Some(user) => {
                let user_id = user.id.clone();

                // If the User and Profile are present, censor the Profile based on the User id
                profile.map(|p| {
                    Profile {
                        user: Box::new(Some(user)),
                        ..p
                    }
                    .censor(Some(user_id))
                })
            }
            // If the User is absent, always censor the Profile
            None => profile.map(|p| p.censor(None)),
        };

        Ok(censored)
    }

    /// Get multiple Profiles
    async fn get_many_profiles(
        &self,
        ctx: &Context<'_>,
        condition: ProfileCondition,
        order_by: Option<Vec<ProfilesOrderBy>>,
        page: Option<usize>,
        page_size: Option<usize>,
    ) -> Result<ProfilesPage> {
        let users = ctx.data_unchecked::<Arc<dyn UsersService>>();
        let profiles = ctx.data_unchecked::<Arc<dyn ProfilesService>>();
        let subject = ctx.data_unchecked::<Subject>();

        // Retrieve the current request User for authorization
        let user_id = match subject {
            Subject(Some(username)) => {
                users
                    .get_by_username(username, &false)
                    .await
                    .map_err(as_graphql_error(
                        "Error while retrieving Profile",
                        StatusCode::INTERNAL_SERVER_ERROR,
                    ))
            }
            _ => Err(graphql_error(
                "A valid JWT token is required",
                StatusCode::UNAUTHORIZED,
            )),
        }?
        .map(|u| u.id);

        let response = profiles
            .get_many(condition, order_by, page, page_size, &false)
            .await
            .map_err(as_graphql_error(
                "Error while listing Profiles",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        let censored = response.map(|p| p.censor(user_id.clone()));

        Ok(censored.into())
    }
}

/// Mutations for the Profile model
#[Object]
impl ProfilesMutation {
    /// Create a new Profile
    async fn create_profile(
        &self,
        ctx: &Context<'_>,
        input: CreateProfileInput,
    ) -> Result<MutateProfileResult> {
        let profiles = ctx.data_unchecked::<Arc<dyn ProfilesService>>();

        // TODO: Authorization

        let profile = profiles
            .create(&input, &false)
            .await
            .map_err(as_graphql_error(
                "Error while creating Profile",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        Ok(MutateProfileResult {
            profile: Some(profile),
        })
    }

    /// Update an existing Profile
    async fn update_profile(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: UpdateProfileInput,
    ) -> Result<MutateProfileResult> {
        let profiles = ctx.data_unchecked::<Arc<dyn ProfilesService>>();

        // TODO: Authorization

        let profile = profiles
            .update(&id, &input, &false)
            .await
            .map_err(as_graphql_error(
                "Error while updating Profile",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        Ok(MutateProfileResult {
            profile: Some(profile),
        })
    }

    /// Remove an existing Profile
    async fn delete_profile(&self, ctx: &Context<'_>, id: String) -> Result<MutateProfileResult> {
        let profiles = ctx.data_unchecked::<Arc<dyn ProfilesService>>();

        // TODO: Authorization

        profiles.delete(&id).await.map_err(as_graphql_error(
            "Error while deleting Profile",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))?;

        Ok(MutateProfileResult { profile: None })
    }
}
