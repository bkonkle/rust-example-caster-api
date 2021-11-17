use std::sync::Arc;

#[cfg(test)]
use mockall::automock;

use crate::{show_model::Show, shows_repository::ShowsRepository};

/// The `Show` entity service
pub struct ShowsService {
    repo: Arc<dyn ShowsRepository>,
}

#[cfg_attr(test, automock)]
impl ShowsService {
    /// Create a new `ShowsService` instance with a type that implements `ShowsRepository`
    pub fn new<T: ShowsRepository + 'static>(repo: &Arc<T>) -> Self {
        Self { repo: repo.clone() }
    }

    /// Get an individual Show by id
    pub async fn get(&self, id: String) -> anyhow::Result<Option<Show>> {
        let show = (&*self.repo).get(id).await?;

        Ok(show)
    }
}

#[cfg(test)]
#[path = "../test/shows_service_tests.rs"]
mod shows_service_tests;
