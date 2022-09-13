use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Object, Result};
use hyper::StatusCode;
use oso::Oso;
use std::sync::Arc;

use super::{
    model::Episode,
    mutations::{CreateEpisodeInput, MutateEpisodeResult, UpdateEpisodeInput},
    queries::{EpisodeCondition, EpisodesOrderBy, EpisodesPage},
    service::EpisodesService,
};
use crate::{
    shows::model::Show,
    shows::service::{ShowLoader, ShowsService},
    users::model::User,
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
    /// Get a sincle Episode
    pub async fn get_episode(
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
    pub async fn get_many_episodes(
        &self,
        ctx: &Context<'_>,
        r#where: Option<EpisodeCondition>,
        order_by: Option<Vec<EpisodesOrderBy>>,
        page: Option<usize>,
        page_size: Option<usize>,
    ) -> Result<EpisodesPage> {
        let episodes = ctx.data_unchecked::<Arc<dyn EpisodesService>>();

        // Check to see if the associated Show is selected
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
    pub async fn create_episode(
        &self,
        ctx: &Context<'_>,
        input: CreateEpisodeInput,
    ) -> Result<MutateEpisodeResult> {
        let shows = ctx.data_unchecked::<Arc<dyn ShowsService>>();
        let episodes = ctx.data_unchecked::<Arc<dyn EpisodesService>>();
        let user = ctx.data_unchecked::<Option<User>>();
        let oso = ctx.data_unchecked::<Oso>();

        // Retrieve the related Show for authorization
        let show = shows
            .get(&input.show_id)
            .await
            .map_err(as_graphql_error(
                "Unable while retrieving Show",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?
            .ok_or_else(|| graphql_error("Unable to find existing Show", StatusCode::NOT_FOUND))?;

        // Check authentication and authorization
        if let Some(user) = user {
            if !oso.is_allowed(user.clone(), "manage_episodes", show.clone())? {
                return Err(graphql_error("Forbidden", StatusCode::FORBIDDEN));
            }
        } else {
            return Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED));
        }

        let episode = episodes
            .create(&input, &false)
            .await
            .map_err(as_graphql_error(
                "Error while creating Episode",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        Ok(MutateEpisodeResult {
            episode: Some(Episode {
                show: Some(show),
                ..episode
            }),
        })
    }

    /// Update an existing Episode
    pub async fn update_episode(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: UpdateEpisodeInput,
    ) -> Result<MutateEpisodeResult> {
        let episodes = ctx.data_unchecked::<Arc<dyn EpisodesService>>();
        let user = ctx.data_unchecked::<Option<User>>();
        let oso = ctx.data_unchecked::<Oso>();

        // Retrieve the existing Episode for authorization
        let existing = episodes
            .get(&id, &true)
            .await
            .map_err(as_graphql_error(
                "Error while fetching Episode",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?
            .ok_or_else(|| {
                graphql_error("Unable to find existing Episode", StatusCode::NOT_FOUND)
            })?;

        let show = existing
            .show
            .ok_or_else(|| graphql_error("Unable to find existing Show", StatusCode::NOT_FOUND))?;

        // Check authentication and authorization
        if let Some(user) = user {
            if !oso.is_allowed(user.clone(), "manage_episodes", show.clone())? {
                return Err(graphql_error("Forbidden", StatusCode::FORBIDDEN));
            }
        } else {
            return Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED));
        }

        // Check to see if the associated User is selected
        let with_show = ctx.look_ahead().field("episode").field("show").exists();

        // Use the already retrieved Episode to update the record
        let episode = episodes
            .update(&existing.id, &input, &with_show)
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
    pub async fn delete_episode(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let episodes = ctx.data_unchecked::<Arc<dyn EpisodesService>>();
        let user = ctx.data_unchecked::<Option<User>>();
        let oso = ctx.data_unchecked::<Oso>();

        // Retrieve the related Show for authorization
        let episode = episodes
            .get(&id, &true)
            .await
            .map_err(as_graphql_error(
                "Error while fetching Episode",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?
            .ok_or_else(|| {
                graphql_error("Unable to find existing Episode", StatusCode::NOT_FOUND)
            })?;

        let show = episode
            .show
            .ok_or_else(|| graphql_error("Unable to find existing Show", StatusCode::NOT_FOUND))?;

        // Check authentication and authorization
        if let Some(user) = user {
            if !oso.is_allowed(user.clone(), "manage_episodes", show.clone())? {
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

#[ComplexObject]
impl Episode {
    #[graphql(name = "show")]
    async fn resolve_show(&self, ctx: &Context<'_>) -> Result<Option<Show>> {
        if let Some(show) = self.show.clone() {
            return Ok(Some(show));
        }

        let loader = ctx.data_unchecked::<DataLoader<ShowLoader>>();
        let show = loader.load_one(self.show_id.clone()).await?;

        Ok(show)
    }
}
