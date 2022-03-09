use anyhow::Result;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sea_orm::{entity::*, query::*, DatabaseConnection, EntityTrait};
use std::sync::Arc;

use crate::{
    profile_model,
    user_model::{self, User, UserOption},
    user_mutations::UpdateUserInput,
};

/// A UsersService appliies business logic to a dynamic UsersRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait UsersService: Sync + Send {
    /// Get an individual `User` by id
    async fn get(&self, id: &str) -> Result<Option<User>>;

    /// Get an individual `User` by username
    // TODO: Add a "with_roles" flag here
    async fn get_by_username(&self, username: &str, with_profile: &bool) -> Result<Option<User>>;

    /// Create a `User` with the given username
    async fn create(&self, username: &str) -> Result<User>;

    /// Create a `User` with the given username if one doesn't exist
    async fn get_or_create(&self, username: &str, with_profile: &bool) -> Result<User>;

    /// Update an existing `User`
    async fn update(&self, id: &str, input: &UpdateUserInput, with_profile: &bool) -> Result<User>;

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
    pub fn new(db: &Arc<DatabaseConnection>) -> Self {
        Self { db: db.clone() }
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

    async fn get_by_username(&self, username: &str, with_profile: &bool) -> Result<Option<User>> {
        let query =
            user_model::Entity::find().filter(user_model::Column::Username.eq(username.to_owned()));

        let user: UserOption = if *with_profile {
            query
                .find_also_related(profile_model::Entity)
                .one(&*self.db)
                .await?
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

    async fn get_or_create(&self, username: &str, with_profile: &bool) -> Result<User> {
        let query = user_model::Entity::find().filter(user_model::Column::Username.eq(username));

        let user: UserOption = if *with_profile {
            query.one(&*self.db).await?.into()
        } else {
            query
                .find_also_related(profile_model::Entity)
                .one(&*self.db)
                .await?
                .into()
        };

        if let UserOption(Some(user)) = user {
            return Ok(user);
        }

        self.create(username).await
    }

    async fn update(&self, id: &str, input: &UpdateUserInput, with_profile: &bool) -> Result<User> {
        let query = user_model::Entity::find_by_id(id.to_owned());

        // Pull out the `User` and the related `Profile`, if selected
        let (user, profile) = if *with_profile {
            query
                .find_also_related(profile_model::Entity)
                .one(&*self.db)
                .await?
        } else {
            // If the Profile isn't requested, just map to None
            query.one(&*self.db).await?.map(|u| (u, None))
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

        // Add back the Profile from above
        updated.profile = profile.map(|p| p.into());

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
