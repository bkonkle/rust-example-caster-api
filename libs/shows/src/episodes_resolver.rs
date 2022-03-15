use async_graphql::{Context, Object, Result};
use hyper::StatusCode;
use oso::Oso;
use std::sync::Arc;

use crate::{
    episode_model::Episode,
    episode_mutations::{CreateEpisodeInput, MutateEpisodeResult, UpdateEpisodeInput},
    episode_queries::{EpisodeCondition, EpisodesOrderBy, EpisodesPage},
    episodes_service::EpisodesService,
};
use caster_users::{
    role_grant_model::CreateRoleGrantInput, role_grants_service::RoleGrantsService,
    user_model::User,
};
use caster_utils::errors::{as_graphql_error, graphql_error};

/// The Query segment owned by the Episodes library
#[derive(Default)]
pub struct EpisodesQuery {}

/// The Mutation segment for Episodes
#[derive(Default)]
pub struct EpisodesMutation {}

/// Queries for the `Episode` model
#[Object]
impl EpisodesQuery {
    async fn get_episode(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The Episode id")] id: String,
    ) -> Result<Option<Episode>> {
        let episodes = ctx.data_unchecked::<Arc<dyn EpisodesService>>();

        // Check to see if the associated Show is selected
        let with_show = ctx.look_ahead().field("show").exists();

        Ok(episodes.get(&id, &with_show).await?)
    }

    /// Get multiple Episodes
    async fn get_many_episodes(
        &self,
        ctx: &Context<'_>,
        r#where: Option<EpisodeCondition>,
        order_by: Option<Vec<EpisodesOrderBy>>,
        page: Option<usize>,
        page_size: Option<usize>,
    ) -> Result<EpisodesPage> {
        let episodes = ctx.data_unchecked::<Arc<dyn EpisodesService>>();

        // Check to see if the associated User is selected
        let with_show = ctx.look_ahead().field("data").field("show").exists();

        let response = episodes
            .get_many(r#where, order_by, page, page_size, &with_show)
            .await
            .map_err(as_graphql_error(
                "Error while listing Episodes",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        Ok(response.into())
    }
}

/// Mutations for the Episode model
#[Object]
impl EpisodesMutation {
    /// Create a new Episode
    async fn create_episode(
        &self,
        ctx: &Context<'_>,
        input: CreateEpisodeInput,
    ) -> Result<MutateEpisodeResult> {
        let episodes = ctx.data_unchecked::<Arc<dyn EpisodesService>>();
        let role_grants = ctx.data_unchecked::<Arc<dyn RoleGrantsService>>();
        let user = ctx.data_unchecked::<Option<User>>();

        // Check to see if the associated User is selected
        let with_show = ctx.look_ahead().field("episode").field("show").exists();

        // Check authorization
        if let Some(user) = user {
            let episode = episodes
                .create(&input, &with_show)
                .await
                .map_err(as_graphql_error(
                    "Error while creating Episode",
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))?;

            // Grant the Admin role to the creator
            role_grants
                .create(&CreateRoleGrantInput {
                    role_key: "admin".to_string(),
                    user_id: user.id.clone(),
                    resource_table: "episodes".to_string(),
                    resource_id: episode.id.clone(),
                })
                .await
                .map_err(as_graphql_error(
                    "Error while granting the admin role for a Episode",
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))?;

            Ok(MutateEpisodeResult {
                episode: Some(episode),
            })
        } else {
            Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED))
        }
    }

    /// Update an existing Episode
    async fn update_episode(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: UpdateEpisodeInput,
    ) -> Result<MutateEpisodeResult> {
        let episodes = ctx.data_unchecked::<Arc<dyn EpisodesService>>();
        let user = ctx.data_unchecked::<Option<User>>();
        let oso = ctx.data_unchecked::<Oso>();

        // Check to see if the associated User is selected
        let with_show = ctx.look_ahead().field("episode").field("show").exists();

        // Retrieve the existing Episode for authorization
        let (existing, existing_show) = episodes
            .get_model(&id, &with_show)
            .await
            .map_err(as_graphql_error(
                "Error while fetching Episode",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?
            .ok_or_else(|| {
                graphql_error("Unable to find existing Episode", StatusCode::NOT_FOUND)
            })?;

        // Check authentication and authorization
        if let Some(user) = user {
            if !oso.is_allowed(user.clone(), "update", existing.clone())? {
                return Err(graphql_error("Forbidden", StatusCode::FORBIDDEN));
            }
        } else {
            return Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED));
        }

        // Use the already retrieved Episode to update the record
        let episode = episodes
            .update_model(existing, &input, existing_show)
            .await
            .map_err(as_graphql_error(
                "Error while updating Profile",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        Ok(MutateEpisodeResult {
            episode: Some(episode),
        })
    }

    /// Remove an existing Episode
    async fn delete_episode(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let episodes = ctx.data_unchecked::<Arc<dyn EpisodesService>>();
        let user = ctx.data_unchecked::<Option<User>>();
        let oso = ctx.data_unchecked::<Oso>();

        // Retrieve the existing Episode for authorization
        let (existing, _) = episodes
            .get_model(&id, &false)
            .await
            .map_err(as_graphql_error(
                "Error while fetching Episode",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?
            .ok_or_else(|| {
                graphql_error("Unable to find existing Episode", StatusCode::NOT_FOUND)
            })?;

        // Check authentication and authorization
        if let Some(user) = user {
            if !oso.is_allowed(user.clone(), "update", existing)? {
                return Err(graphql_error("Forbidden", StatusCode::FORBIDDEN));
            }
        } else {
            return Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED));
        }

        episodes.delete(&id).await.map_err(as_graphql_error(
            "Error while deleting Episode",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))?;

        Ok(true)
    }
}
