use anyhow::Result;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use std::sync::Arc;

use crate::{user_model::User, user_mutations::UpdateUserInput, users_repository::UsersRepository};

/// A UsersService appliies business logic to a dynamic UsersRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait UsersService: Sync + Send {
    /// Get an individual `User` by id
    async fn get(&self, id: &str) -> Result<Option<User>>;

    /// Get an individual `User` by username
    async fn get_by_username(&self, username: &str) -> Result<Option<User>>;

    /// Create a `User` with the given username
    async fn create(&self, username: &str) -> Result<User>;

    /// Create a `User` with the given username if one doesn't exist
    async fn get_or_create(&self, username: &str) -> Result<User>;

    /// Update an existing `User`
    async fn update(&self, id: &str, input: &UpdateUserInput) -> Result<User>;

    /// Delete an existing `User`
    async fn delete(&self, id: &str) -> Result<()>;
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
    async fn get(&self, id: &str) -> Result<Option<User>> {
        let user = self.repo.get(id).await?;

        Ok(user)
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>> {
        let user = self.repo.get_by_username(username).await?;

        Ok(user)
    }

    async fn create(&self, username: &str) -> Result<User> {
        self.repo.create(username).await
    }

    async fn get_or_create(&self, username: &str) -> Result<User> {
        let user = self.get_by_username(username).await?;

        if let Some(user) = user {
            return Ok(user);
        }

        self.create(username).await
    }

    async fn update(&self, id: &str, input: &UpdateUserInput) -> Result<User> {
        self.repo
            .update(id, &input.username, &input.is_active)
            .await
    }

    async fn delete(&self, id: &str) -> Result<()> {
        self.repo.delete(id).await
    }
}
