use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

#[cfg(test)]
use mockall::automock;

use crate::{user_model::User, users_repository::UsersRepository};

/// A UsersService appliies business logic to a dynamic UsersRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait UsersService: Sync + Send {
    /// Get an individual User by id
    async fn get(&self, id: String) -> Result<Option<User>>;
}

/// The default `UsersService` struct
pub struct DefaultUsersService {
    /// The UsersRepository instance
    repo: Arc<dyn UsersRepository>,
}

/// The default `UsersService` implementation
impl DefaultUsersService {
    /// Create a new `UsersService` instance with a `UsersRepository` implementation
    pub fn new<Repo: UsersRepository + 'static>(repo: &Arc<Repo>) -> Self {
        Self { repo: repo.clone() }
    }
}

#[async_trait]
impl UsersService for DefaultUsersService {
    async fn get(&self, id: String) -> Result<Option<User>> {
        let user = (&*self.repo).get(id).await?;

        Ok(user)
    }
}
