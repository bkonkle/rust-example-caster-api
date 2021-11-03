use async_graphql::{Context, Error, Object, Result};
use sqlx::{Pool, Postgres};

#[derive(Default)]
pub struct UsersQuery;

#[Object]
impl UsersQuery {
    async fn get_current_user<'ctx>(&self, ctx: &Context<'ctx>) -> Result<&str, Error> {
        let _pg_pool = ctx.data_unchecked::<Pool<Postgres>>();

        return Ok("test");
    }
}
