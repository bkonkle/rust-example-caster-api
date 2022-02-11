use anyhow::Result;
use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait};
use std::sync::Arc;

#[cfg(test)]
use mockall::automock;

use crate::show_model::{self, Show};

/// A ShowsService appliies business logic to a dynamic ShowsRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait ShowsService: Sync + Send {
    /// Get an individual Show by id
    async fn get(&self, id: &str) -> Result<Option<Show>>;
}

/// The default `ShowsService` struct.
pub struct DefaultShowsService {
    /// The SeaOrm database connection
    db: Arc<DatabaseConnection>,
}

/// The default `ShowsService` implementation
impl DefaultShowsService {
    /// Create a new `ShowsService` instance
    pub fn new(db: &Arc<DatabaseConnection>) -> Self {
        Self { db: db.clone() }
    }
}

#[async_trait]
impl ShowsService for DefaultShowsService {
    async fn get(&self, id: &str) -> Result<Option<Show>> {
        let show = show_model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?;

        Ok(show)
    }
}
