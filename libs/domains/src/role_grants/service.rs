use anyhow::Result;
use async_graphql::{dataloader::Loader, FieldError};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sea_orm::{entity::*, query::*, Condition, DatabaseConnection, EntityTrait};
use std::{collections::HashMap, sync::Arc};

use super::model::{self, CreateRoleGrantInput, RoleGrant};

/// A RoleGrantsService appliies business logic to a dynamic RoleGrantsRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait RoleGrantsService: Sync + Send {
    /// Get an individual `RoleGrant` by id
    async fn get(&self, id: &str) -> Result<Option<RoleGrant>>;

    /// Get a list of `RoleGrant` results matching the given ids
    async fn get_by_ids(&self, ids: Vec<String>) -> Result<Vec<RoleGrant>>;

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
    pub fn new(db: &Arc<DatabaseConnection>) -> Self {
        Self { db: db.clone() }
    }
}

#[async_trait]
impl RoleGrantsService for DefaultRoleGrantsService {
    async fn get(&self, id: &str) -> Result<Option<RoleGrant>> {
        let query = model::Entity::find_by_id(id.to_owned());

        let role_grant = query.one(&*self.db).await?;

        Ok(role_grant)
    }

    async fn get_by_ids(&self, ids: Vec<String>) -> Result<Vec<RoleGrant>> {
        let mut condition = Condition::any();

        for id in ids {
            condition = condition.add(model::Column::Id.eq(id.clone()));
        }

        let role_grants = model::Entity::find()
            .filter(condition)
            .all(&*self.db)
            .await?;

        Ok(role_grants)
    }

    async fn create(&self, input: &CreateRoleGrantInput) -> Result<RoleGrant> {
        let role_grant = model::ActiveModel {
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
        let role_grant = model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?
            .ok_or_else(|| anyhow!("Unable to find RoleGrant with id: {}", id))?;

        let _result = role_grant.delete(&*self.db).await?;

        Ok(())
    }
}

/// A dataloader for `RoleGrant` instances
pub struct RoleGrantLoader {
    /// The SeaOrm database connection
    role_grants: Arc<dyn RoleGrantsService>,
}

/// The default implementation for the `RoleGrantLoader`
impl RoleGrantLoader {
    /// Create a new instance
    pub fn new(role_grants: &Arc<dyn RoleGrantsService>) -> Self {
        Self {
            role_grants: role_grants.clone(),
        }
    }
}

#[async_trait]
impl Loader<String> for RoleGrantLoader {
    type Value = RoleGrant;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let role_grants = self.role_grants.get_by_ids(keys.into()).await?;

        Ok(role_grants
            .into_iter()
            .map(|role_grant| (role_grant.id.clone(), role_grant))
            .collect())
    }
}
