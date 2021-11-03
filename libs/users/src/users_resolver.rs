use async_graphql::{Context, Error, Object, Result};

#[derive(Default)]
pub struct UsersQuery;

#[Object]
impl UsersQuery {
    async fn get_current_user<'ctx>(&self, _ctx: &Context<'ctx>) -> Result<&str, Error> {
        return Ok("test");
    }
}
