use anyhow::Result;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use std::sync::Arc;

use self::{Unique::Id, Unique::Username};
use crate::{
    user_model::User,
    user_mutations::{CreateUserInput, UpdateUserInput},
    users_repository::UsersRepository,
};

/// Unique Criteria for finding a single User
pub enum Unique {
    /// Find a User based on a String id
    Id(String),

    /// Find a User based on a Subject as username
    Username(String),
}

/// A UsersService appliies business logic to a dynamic UsersRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait UsersService: Sync + Send {
    /// Get an individual `User` by a `Unique` criteria
    async fn get(&self, unique: &Unique) -> Result<Option<User>>;

    /// Create a `User` with the given username
    async fn create(&self, input: &CreateUserInput) -> Result<User>;

    /// Update an existing `User`
    async fn update(&self, id: &str, input: &UpdateUserInput) -> Result<User>;
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
    async fn get(&self, unique: &Unique) -> Result<Option<User>> {
        let user = match unique {
            Id(id) => self.repo.get(id).await?,
            Username(username) => self.repo.get_by_username(username).await?,
        };

        Ok(user)
    }

    async fn create(&self, input: &CreateUserInput) -> Result<User> {
        self.repo.create(&input.username).await
    }

    async fn update(&self, id: &str, input: &UpdateUserInput) -> Result<User> {
        self.repo
            .update(id, &input.username, &input.is_active)
            .await
    }
}
