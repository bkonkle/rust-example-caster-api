use async_graphql::{Context, Error, Object, Result};

pub struct Query;

#[Object]
impl Query {
    async fn get_show(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The Show id")] id: String,
    ) -> Vec<String> {
        "test"
    }
}
