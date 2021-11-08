use anyhow;
use async_graphql::{Context, Object};

use crate::{
    show_models::Show,
    shows_service::{PgShowsService, ShowsService},
};

/// The Query segment owned by the Shows library
#[derive(Default)]
pub struct ShowsQuery {}

#[Object]
impl ShowsQuery {
    async fn get_show(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The Show id")] id: String,
    ) -> Result<Option<Show>, anyhow::Error> {
        let shows = ctx.data_unchecked::<PgShowsService>();

        Ok(shows.get(id).await?)
    }
}
