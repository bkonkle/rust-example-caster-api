use anyhow;
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
    ) -> Result<Option<User>, anyhow::Error> {
        let users = ctx.data_unchecked::<Arc<dyn UsersService>>();

        Ok(users.get(id).await?)
    }
}
