use async_graphql::{Context, Object};
use sqlx::{Pool, Postgres};

/// The Query segment owned by the Shows library
#[derive(Default)]
pub struct ShowsQuery;

#[Object]
impl ShowsQuery {
    async fn get_show(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The Show id")] _id: String,
    ) -> &str {
        let _pg_pool = ctx.data::<Pool<Postgres>>();

        "test"
    }
}
