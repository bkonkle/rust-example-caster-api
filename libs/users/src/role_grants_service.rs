use anyhow::Result;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sea_orm::{entity::*, DatabaseConnection, EntityTrait};
use std::sync::Arc;

use crate::role_grant_model::{self, CreateRoleGrantInput, RoleGrant};

/// A RoleGrantsService appliies business logic to a dynamic RoleGrantsRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait RoleGrantsService: Sync + Send {
    /// Get an individual `RoleGrant` by id
    async fn get(&self, id: &str) -> Result<Option<RoleGrant>>;

    /// Create a `RoleGrant` with the given input
    async fn create(&self, input: &CreateRoleGrantInput) -> Result<RoleGrant>;

    /// Delete an existing `RoleGrant`
    async fn delete(&self, id: &str) -> Result<()>;
}

/// The default `RoleGrantsService` struct.
pub struct DefaultRoleGrantsService {
    /// The SeaOrm database connection
    db: Arc<DatabaseConnection>,
}

/// The default `RoleGrantsService` implementation
impl DefaultRoleGrantsService {
    /// Create a new `RoleGrantsService` instance
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl RoleGrantsService for DefaultRoleGrantsService {
    async fn get(&self, id: &str) -> Result<Option<RoleGrant>> {
        let query = role_grant_model::Entity::find_by_id(id.to_owned());

        let role_grant = query.one(&*self.db).await?;

        Ok(role_grant)
    }

    async fn create(&self, input: &CreateRoleGrantInput) -> Result<RoleGrant> {
        let role_grant = role_grant_model::ActiveModel {
            role_key: Set(input.role_key.clone()),
            user_id: Set(input.user_id.clone()),
            resource_table: Set(input.resource_table.clone()),
            resource_id: Set(input.resource_id.clone()),
            ..Default::default()
        }
        .insert(&*self.db)
        .await?;

        let created: RoleGrant = role_grant;

        return Ok(created);
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let role_grant = role_grant_model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?
            .ok_or_else(|| anyhow!("Unable to find RoleGrant with id: {}", id))?;

        let _result = role_grant.delete(&*self.db).await?;

        Ok(())
    }
}
