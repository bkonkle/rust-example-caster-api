use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Object, Result};
use hyper::StatusCode;
use std::sync::Arc;

use super::{
    model::Profile,
    mutations::{CreateProfileInput, MutateProfileResult, UpdateProfileInput},
    queries::{ProfileCondition, ProfilesOrderBy, ProfilesPage},
    service::ProfilesService,
};
use crate::users::{model::User, service::UserLoader};
use caster_utils::errors::{as_graphql_error, graphql_error};

/// The Query segment for Profiles
#[derive(Default)]
pub struct ProfilesQuery {}

/// The Mutation segment for Profiles
#[derive(Default)]
pub struct ProfilesMutation {}

/// Queries for the `Profile` model
#[Object]
impl ProfilesQuery {
    /// Get a single Profile
    async fn get_profile(&self, ctx: &Context<'_>, id: String) -> Result<Option<Profile>> {
        let user = ctx.data_unchecked::<Option<User>>();
        let profiles = ctx.data_unchecked::<Arc<dyn ProfilesService>>();

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
                        user: Some(user.clone()),
                        ..p
                    }
                    .censor(&Some(user_id))
                })
            }
            // If the User is absent, always censor the Profile
            None => profile.map(|p| p.censor(&None)),
        };

        Ok(censored)
    }

    /// Get multiple Profiles
    async fn get_many_profiles(
        &self,
        ctx: &Context<'_>,
        r#where: Option<ProfileCondition>,
        order_by: Option<Vec<ProfilesOrderBy>>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<ProfilesPage> {
        let user = ctx.data_unchecked::<Option<User>>();
        let profiles = ctx.data_unchecked::<Arc<dyn ProfilesService>>();

        // Retrieve the current request User id for authorization
        let user_id = user.clone().map(|u| u.id);

        // Check to see if the associated User is selected
        let with_user = ctx.look_ahead().field("data").field("user").exists();

        let response = profiles
            .get_many(r#where, order_by, page, page_size, &with_user)
            .await
            .map_err(as_graphql_error(
                "Error while listing Profiles",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        let censored = response.map(|p| p.censor(&user_id));

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
        let user = ctx.data_unchecked::<Option<User>>();
        let profiles = ctx.data_unchecked::<Arc<dyn ProfilesService>>();

        // Retrieve the current request User id for authorization
        let user_id = user.clone().map(|u| u.id);

        if let Some(user_id) = user_id {
            // Make sure the current request User id matches the input
            if user_id != input.user_id {
                return Err(graphql_error("Forbidden", StatusCode::FORBIDDEN));
            }
        } else {
            // If there is no request User, return a 401
            return Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED));
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
        let user = ctx.data_unchecked::<Option<User>>();
        let profiles = ctx.data_unchecked::<Arc<dyn ProfilesService>>();

        // Retrieve the existing Profile for authorization
        let existing = profiles
            .get(&id, &true)
            .await
            .map_err(as_graphql_error(
                "Error while fetching Profile",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?
            .ok_or_else(|| {
                graphql_error("Unable to find existing Profile", StatusCode::NOT_FOUND)
            })?;

        // Retrieve the current request User id for authorization
        let user_id = user.clone().map(|u| u.id);

        if let Some(user_id) = user_id {
            // Make sure the current request User id matches the existing user
            if existing.user.as_ref().map(|u| u.id.clone()) != Some(user_id) {
                return Err(graphql_error("Forbidden", StatusCode::FORBIDDEN));
            }
        } else {
            // If there is no request User, return a 401
            return Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED));
        }

        // Check to see if the associated User is selected
        let with_user = ctx.look_ahead().field("profile").field("user").exists();

        // Use the already retrieved Profile to update the record
        let profile = profiles
            .update(&existing.id, &input, &with_user)
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
    async fn delete_profile(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let user = ctx.data_unchecked::<Option<User>>();
        let profiles = ctx.data_unchecked::<Arc<dyn ProfilesService>>();

        // Retrieve the existing Profile for authorization
        let existing = profiles
            .get(&id, &true)
            .await
            .map_err(as_graphql_error(
                "Error while fetching Profile",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?
            .ok_or_else(|| {
                graphql_error("Unable to find existing Profile", StatusCode::NOT_FOUND)
            })?;

        // Retrieve the current request User id for authorization
        let user_id = user.clone().map(|u| u.id);

        if let Some(user_id) = user_id {
            // Make sure the current request User id matches the existing user
            if existing.user.as_ref().map(|u| u.id.clone()) != Some(user_id) {
                return Err(graphql_error("Forbidden", StatusCode::FORBIDDEN));
            }
        } else {
            // If there is no request User, return a 401
            return Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED));
        }

        profiles.delete(&id).await.map_err(as_graphql_error(
            "Error while deleting Profile",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))?;

        Ok(true)
    }
}

#[ComplexObject]
impl Profile {
    #[graphql(name = "user")]
    async fn resolve_user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        if let Some(user) = self.user.clone() {
            return Ok(Some(user));
        }

        if let Some(user_id) = self.user_id.clone() {
            let loader = ctx.data_unchecked::<DataLoader<UserLoader>>();
            let user = loader.load_one(user_id).await?;

            return Ok(user);
        }

        Ok(None)
    }
}
