use async_graphql::*;

pub struct Query;

#[Object]
impl Query {
    async fn get_current_user(&self) -> Result<String, Error> {
        return Ok(String::from("test"));
    }
}
