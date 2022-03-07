use async_graphql::{Context, Object, Result};
use hyper::StatusCode;
use std::sync::Arc;

use crate::{
    show_model::Show,
    show_mutations::{CreateShowInput, MutateShowResult, UpdateShowInput},
    show_queries::{ShowCondition, ShowsOrderBy, ShowsPage},
    shows_service::ShowsService,
};
use caster_utils::errors::{as_graphql_error, graphql_error};

/// The Query segment owned by the Shows library
#[derive(Default)]
pub struct ShowsQuery {}

/// The Mutation segment for Shows
#[derive(Default)]
pub struct ShowsMutation {}

/// Queries for the `Show` model
#[Object]
impl ShowsQuery {
    async fn get_show(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The Show id")] id: String,
    ) -> Result<Option<Show>> {
        let shows = ctx.data_unchecked::<Arc<dyn ShowsService>>();

        Ok(shows.get(&id).await?)
    }

    /// Get multiple Shows
    async fn get_many_shows(
        &self,
        ctx: &Context<'_>,
        r#where: Option<ShowCondition>,
        order_by: Option<Vec<ShowsOrderBy>>,
        page: Option<usize>,
        page_size: Option<usize>,
    ) -> Result<ShowsPage> {
        let shows = ctx.data_unchecked::<Arc<dyn ShowsService>>();

        let response = shows
            .get_many(r#where, order_by, page, page_size)
            .await
            .map_err(as_graphql_error(
                "Error while listing Shows",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        Ok(response.into())
    }
}

/// Mutations for the Show model
#[Object]
impl ShowsMutation {
    /// Create a new Show
    async fn create_show(
        &self,
        ctx: &Context<'_>,
        input: CreateShowInput,
    ) -> Result<MutateShowResult> {
        let shows = ctx.data_unchecked::<Arc<dyn ShowsService>>();

        let show = shows.create(&input).await.map_err(as_graphql_error(
            "Error while creating Show",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))?;

        Ok(MutateShowResult { show: Some(show) })
    }

    /// Update an existing Show
    async fn update_show(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: UpdateShowInput,
    ) -> Result<MutateShowResult> {
        let shows = ctx.data_unchecked::<Arc<dyn ShowsService>>();

        // Retrieve the existing Show for authorization (TODO)
        let _existing = shows
            .get_model(&id)
            .await
            .map_err(as_graphql_error(
                "Error while fetching Show",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?
            .ok_or_else(|| graphql_error("Unable to find existing Show", StatusCode::NOT_FOUND))?;

        let show = shows.update(&id, &input).await.map_err(as_graphql_error(
            "Error while updating Show",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))?;

        Ok(MutateShowResult { show: Some(show) })
    }

    /// Remove an existing Show
    async fn delete_show(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let shows = ctx.data_unchecked::<Arc<dyn ShowsService>>();

        // Retrieve the existing Show for authorization (TODO)
        let _existing = shows
            .get_model(&id)
            .await
            .map_err(as_graphql_error(
                "Error while fetching Show",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?
            .ok_or_else(|| graphql_error("Unable to find existing Show", StatusCode::NOT_FOUND))?;

        shows.delete(&id).await.map_err(as_graphql_error(
            "Error while deleting Show",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))?;

        Ok(true)
    }
}
