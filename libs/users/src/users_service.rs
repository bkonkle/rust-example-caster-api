use anyhow::Result;
use async_graphql::{dataloader::Loader, FieldError};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sea_orm::{entity::*, query::*, DatabaseConnection, EntityTrait};
use std::{collections::HashMap, sync::Arc};

use crate::{
    role_grant_model,
    user_model::{self, User, UserOption},
    user_mutations::UpdateUserInput,
};

/// A UsersService appliies business logic to a dynamic UsersRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait UsersService: Sync + Send {
    /// Get an individual `User` by id
    async fn get(&self, id: &str) -> Result<Option<User>>;

    /// Get a list of `User` results matching the given ids
    async fn get_by_ids(&self, ids: Vec<String>) -> Result<Vec<User>>;

    /// Get an individual `User` by username
    async fn get_by_username(&self, username: &str, with_roles: &bool) -> Result<Option<User>>;

    /// Create a `User` with the given username
    async fn create(&self, username: &str) -> Result<User>;

    /// Update an existing `User`
    async fn update(&self, id: &str, input: &UpdateUserInput, with_roles: &bool) -> Result<User>;

    /// Delete an existing `User`
    async fn delete(&self, id: &str) -> Result<()>;
}

/// The default `UsersService` struct
pub struct DefaultUsersService {
    /// The SeaOrm database connection
    db: Arc<DatabaseConnection>,
}

/// The default `UsersService` implementation
impl DefaultUsersService {
    /// Create a new `UsersService` instance
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UsersService for DefaultUsersService {
    async fn get(&self, id: &str) -> Result<Option<User>> {
        let user = user_model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?;

        Ok(user)
    }

    async fn get_by_ids(&self, ids: Vec<String>) -> Result<Vec<User>> {
        let mut condition = Condition::any();

        for id in ids {
            condition = condition.add(user_model::Column::Id.eq(id.clone()));
        }

        let users = user_model::Entity::find()
            .filter(condition)
            .all(&*self.db)
            .await?;

        Ok(users)
    }

    async fn get_by_username(&self, username: &str, with_roles: &bool) -> Result<Option<User>> {
        let query =
            user_model::Entity::find().filter(user_model::Column::Username.eq(username.to_owned()));

        let user: UserOption = if *with_roles {
            query
                .find_with_related(role_grant_model::Entity)
                .all(&*self.db)
                .await?
                .first()
                .map(|t| t.to_owned())
                .into()
        } else {
            query.one(&*self.db).await?.into()
        };

        Ok(user.into())
    }

    async fn create(&self, username: &str) -> Result<User> {
        let user = user_model::ActiveModel {
            username: Set(username.to_owned()),
            ..Default::default()
        }
        .insert(&*self.db)
        .await?;

        Ok(user)
    }

    async fn update(&self, id: &str, input: &UpdateUserInput, with_roles: &bool) -> Result<User> {
        let query = user_model::Entity::find_by_id(id.to_owned());

        // Pull out the `User` and the related `RoleGrants`, if selected
        let (user, roles) = if *with_roles {
            query
                .find_with_related(role_grant_model::Entity)
                .all(&*self.db)
                .await?
                .first()
                .map(|t| t.to_owned())
        } else {
            // If the Profile isn't requested, just map to None
            query.one(&*self.db).await?.map(|u| (u, vec![]))
        }
        .ok_or_else(|| anyhow!("Unable to find User with id: {}", id))?;

        let mut user: user_model::ActiveModel = user.into();

        if let Some(username) = &input.username {
            user.username = Set(username.clone());
        }

        if let Some(is_active) = &input.is_active {
            user.is_active = Set(is_active.to_owned());
        }

        let mut updated = user.update(&*self.db).await?;

        // Add back the RoleGrants from above
        updated.roles = roles;

        Ok(updated)
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let user = user_model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?
            .ok_or_else(|| anyhow!("Unable to find User with id: {}", id))?;

        let _result = user.delete(&*self.db).await?;

        Ok(())
    }
}

/// A dataloader for `User` instances
pub struct UserLoader {
    /// The SeaOrm database connection
    locations: Arc<dyn UsersService>,
}

/// The default implementation for the `UserLoader`
impl UserLoader {
    /// Create a new instance
    pub fn new(locations: Arc<dyn UsersService>) -> Self {
        Self { locations }
    }
}

#[async_trait]
impl Loader<String> for UserLoader {
    type Value = User;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let locations = self.locations.get_by_ids(keys.into()).await?;

        Ok(locations
            .into_iter()
            .map(|location| (location.id.clone(), location))
            .collect())
    }
}
