use std::sync::Arc;

#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::{user_model::User, users_repository::UsersRepository};

/// The `User` entity service
pub struct UsersService {
    repo: Arc<dyn UsersRepository>,
}

#[cfg_attr(test, automock)]
impl UsersService {
    /// Create a new `UsersService` instance with a type that implements `UsersRepository`
    pub fn new(repo: &Arc<dyn UsersRepository>) -> Self {
        Self { repo: repo.clone() }
    }

    /// Get an individual User by id
    pub async fn get(&self, id: String) -> anyhow::Result<Option<User>> {
        let user = (&*self.repo).get(id).await?;

        Ok(user)
    }
}
