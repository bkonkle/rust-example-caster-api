use anyhow;
use async_graphql::{Context, Object};

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
    ) -> Result<Option<Show>, anyhow::Error> {
        let shows = ctx.data_unchecked::<ShowsService>();

        Ok(shows.get(id).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shows_service::*;

    #[tokio::test]
    async fn test_get_show() {}
}
