use async_graphql::{Context, Object, Result};
use caster_auth::Subject;
use caster_utils::errors::{as_graphql_error, graphql_error};
use hyper::StatusCode;
use std::sync::Arc;

use crate::{
    profile_model::Profile,
    profile_mutations::{CreateProfileInput, MutateProfileResult, UpdateProfileInput},
    profile_queries::{ProfileCondition, ProfilesOrderBy, ProfilesPage},
    profile_utils::{get_current_user, maybe_get_current_user},
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
        let user = maybe_get_current_user(subject, users).await?;

        // Check to see if the associated User is selected
        let with_user = ctx.look_ahead().field("user").exists();

        let profile = profiles.get(&id, &with_user).await?;

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
        r#where: Option<ProfileCondition>,
        order_by: Option<Vec<ProfilesOrderBy>>,
        page: Option<usize>,
        page_size: Option<usize>,
    ) -> Result<ProfilesPage> {
        let users = ctx.data_unchecked::<Arc<dyn UsersService>>();
        let profiles = ctx.data_unchecked::<Arc<dyn ProfilesService>>();
        let subject = ctx.data_unchecked::<Subject>();

        // Retrieve the current request User for authorization
        let user_id = maybe_get_current_user(subject, users).await?.map(|u| u.id);

        // Check to see if the associated User is selected
        let with_user = ctx.look_ahead().field("data").field("user").exists();

        let response = profiles
            .get_many(r#where, order_by, page, page_size, &with_user)
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
        let users = ctx.data_unchecked::<Arc<dyn UsersService>>();
        let profiles = ctx.data_unchecked::<Arc<dyn ProfilesService>>();
        let subject = ctx.data_unchecked::<Subject>();

        // Retrieve the current request User for authorization
        let user_id = get_current_user(subject, users).await?.id;

        // Make sure the current request User id matches the input
        if user_id != input.user_id {
            return Err(graphql_error(
                "The userId must match the currently logged-in User",
                StatusCode::FORBIDDEN,
            ));
        }

        // Check to see if the associated User is selected
        let with_user = ctx.look_ahead().field("profile").field("user").exists();

        let profile = profiles
            .create(&input, &with_user)
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
        let users = ctx.data_unchecked::<Arc<dyn UsersService>>();
        let profiles = ctx.data_unchecked::<Arc<dyn ProfilesService>>();
        let subject = ctx.data_unchecked::<Subject>();

        // Retrieve the existing Profile for authorization
        let (existing, existing_user) = profiles
            .get_model(&id, &true)
            .await
            .map_err(as_graphql_error(
                "Error while fetching Profile",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?
            .ok_or_else(|| {
                graphql_error(
                    "Unable to find existing Profile",
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            })?;

        // Retrieve the current request User for authorization
        let user_id = get_current_user(subject, users).await?.id;

        // Make sure the current request User id matches the existing user
        if existing_user.as_ref().map(|u| u.id.clone()) != Some(user_id) {
            return Err(graphql_error(
                "The user_id must match the currently logged-in User",
                StatusCode::FORBIDDEN,
            ));
        }

        // Use the already retrieved Profile to update the record
        let profile = profiles
            .update_model(existing, &input, existing_user)
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
