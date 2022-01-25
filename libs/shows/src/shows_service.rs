use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

#[cfg(test)]
use mockall::automock;

use crate::{show_model::Show, shows_repository::ShowsRepository};

/// A ShowsService appliies business logic to a dynamic ShowsRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait ShowsService: Sync + Send {
    /// Get an individual Show by id
    async fn get(&self, id: String) -> Result<Option<Show>>;
}

/// The default `ShowsService` struct.
pub struct DefaultShowsService {
    /// The dynamic ShowsRepository instance.
    repo: Arc<dyn ShowsRepository>,
}

/// The default `ShowsService` implementation
impl DefaultShowsService {
    /// Create a new `ShowsService` instance with a `ShowsRepository` implementation
    pub fn new<Repo: ShowsRepository + 'static>(repo: &Arc<Repo>) -> Self {
        Self { repo: repo.clone() }
    }
}

#[async_trait]
impl ShowsService for DefaultShowsService {
    async fn get(&self, id: String) -> Result<Option<Show>> {
        let show = self.repo.get(id).await?;

        Ok(show)
    }
}
