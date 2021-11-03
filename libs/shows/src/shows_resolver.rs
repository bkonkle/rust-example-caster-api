use async_graphql::{Context, Object};

#[derive(Default)]
pub struct ShowsQuery;

#[Object]
impl ShowsQuery {
    async fn get_show(
        &self,
        _ctx: &Context<'_>,
        #[graphql(desc = "The Show id")] _id: String,
    ) -> &str {
        "test"
    }
}
