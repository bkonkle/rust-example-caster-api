use anyhow::Result;
use async_graphql::{Context, Object};
use std::sync::Arc;

use crate::{show_model::Show, shows_service::ShowsService};

/// The Query segment owned by the Shows library
#[derive(Default)]
pub struct ShowsQuery {}

#[Object]
impl ShowsQuery {
    async fn get_show(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The Show id")] id: String,
    ) -> Result<Option<Show>> {
        let shows = ctx
            .data::<Arc<dyn ShowsService>>()
            .map_err(|err| anyhow!("ShowsService not found: {}", err.message))?;

        Ok(shows.get(id).await?)
    }
}
