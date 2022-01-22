use anyhow::Result;
use async_graphql::{Context, Object};
use std::sync::Arc;

use caster_auth::Subject;

use crate::{
    user_model::User,
    user_mutations::{CreateUserInput, UpdateUserInput},
    users_service::{Unique::Username, UsersService},
};

/// The Query segment owned by the Users library
#[derive(Default)]
pub struct UsersQuery {}

#[Object]
impl UsersQuery {
    async fn get_current_user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let users = ctx.data_unchecked::<Arc<dyn UsersService>>();
        let subject = ctx.data::<Subject>().ok();

        match subject {
            Some(Subject(Some(username))) => users.get(&Username(username.clone())).await,
            _ => Ok(None),
        }
    }

    async fn get_or_create_current_user(
        &self,
        ctx: &Context<'_>,
        input: CreateUserInput,
    ) -> Result<User> {
        let users = ctx.data_unchecked::<Arc<dyn UsersService>>();
        let subject = ctx.data::<Subject>().ok();

        let token_username = match subject {
            Some(Subject(Some(username))) => Ok(username),
            _ => Err(anyhow!("A valid JWT token with a sub is required")),
        }?;

        if token_username != &input.username {
            return Err(anyhow!("Username must match JWT token sub"));
        };

        users.create(&input).await
    }

    async fn update_current_user(&self, ctx: &Context<'_>, input: UpdateUserInput) -> Result<User> {
        let users = ctx.data_unchecked::<Arc<dyn UsersService>>();
        let subject = ctx.data::<Subject>().ok();

        let existing = match subject {
            Some(Subject(Some(username))) => users.get(&Username(username.clone())).await,
            _ => Err(anyhow!("A valid JWT token with a sub is required")),
        }?;

        match existing {
            Some(existing) => users.update(&existing.id, &input).await,
            _ => Err(anyhow!("No existing User found for the current JWT token")),
        }
    }
}
