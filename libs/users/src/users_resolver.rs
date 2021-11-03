use async_graphql::{Context, Error, Object, Result};

pub struct Query;

#[Object]
impl Query {
    async fn get_current_user<'ctx>(&self, ctx: &Context<'ctx>) -> Result<&str, Error> {
        return Ok("test");
    }
}
