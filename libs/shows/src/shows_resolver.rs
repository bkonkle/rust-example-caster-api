use async_graphql::{Context, Object};
use sqlx::{Pool, Postgres};

#[derive(Default)]
pub struct ShowsQuery;

#[Object]
impl ShowsQuery {
    async fn get_show(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The Show id")] _id: String,
    ) -> &str {
        let _pg_pool = ctx.data_unchecked::<Pool<Postgres>>();

        "test"
    }
}
