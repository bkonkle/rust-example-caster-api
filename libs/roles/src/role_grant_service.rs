use anyhow::Result;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sea_orm::{entity::*, query::*, DatabaseConnection, EntityTrait};
use std::sync::Arc;

use crate::role_grant_model::{self, Role, RoleList};

/// A ShowsService appliies business logic to a dynamic ShowsRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait ShowsService: Sync + Send {
    /// Get a list of Roles granted to a Profile
    async fn get_roles_by_profile(&self, profile_id: &str) -> Result<Vec<Role>>;
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
    async fn get_roles_by_profile(&self, profile_id: &str) -> Result<Vec<Role>> {
        let query = role_grant_model::Entity::find()
            .filter(role_grant_model::Column::ProfileId.eq(profile_id.to_owned()));

        let role_grants = query.all(&*self.db).await?;
        let roles: RoleList = role_grants.into();

        Ok(roles.into())
    }
}
