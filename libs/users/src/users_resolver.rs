use anyhow::Result;
use async_graphql::{Context, Object};
use std::sync::Arc;

use crate::{user_model::User, users_service::UsersService};

/// The Query segment owned by the Users library
#[derive(Default)]
pub struct UsersQuery {}

#[Object]
impl UsersQuery {
    async fn get_user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The User id")] id: String,
    ) -> Result<Option<User>> {
        let users = ctx
            .data::<Arc<dyn UsersService>>()
            .map_err(|err| anyhow!("UsersService not found: {}", err.message))?;

        Ok(users.get(id).await?)
    }

    async fn get_current_user(&self, _ctx: &Context<'_>) -> Result<Option<User>, anyhow::Error> {
        Ok(None)
    }
}
