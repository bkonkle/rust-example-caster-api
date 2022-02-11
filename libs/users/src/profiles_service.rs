use anyhow::Result;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sea_orm::{entity::*, query::*, DatabaseConnection, EntityTrait};
use std::sync::Arc;

use crate::{
    profile_model::{self, Profile},
    profile_mutations::{CreateProfileInput, UpdateProfileInput},
};

/// A ProfilesService appliies business logic to a dynamic ProfilesRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait ProfilesService: Sync + Send {
    /// Get an individual `Profile` by id
    async fn get(&self, id: &str) -> Result<Option<Profile>>;

    /// Get the first `Profile` with this user_id
    async fn get_by_user_id(&self, user_id: &str) -> Result<Option<Profile>>;

    /// Get or create a `Profile`.
    async fn get_or_create(&self, user_id: &str, input: &CreateProfileInput) -> Result<Profile>;

    /// Create a `Profile` with the given input
    async fn create(&self, input: &CreateProfileInput) -> Result<Profile>;

    /// Update an existing `Profile`
    async fn update(&self, id: &str, input: &UpdateProfileInput) -> Result<Profile>;

    /// Delete an existing `Profile`
    async fn delete(&self, id: &str) -> Result<()>;
}

/// The default `ProfilesService` struct
pub struct DefaultProfilesService {
    /// The SeaOrm database connection
    db: Arc<DatabaseConnection>,
}

/// The default `ProfilesService` implementation
impl DefaultProfilesService {
    /// Create a new `ProfilesService` instance
    pub fn new(db: &Arc<DatabaseConnection>) -> Self {
        Self { db: db.clone() }
    }
}

#[async_trait]
impl ProfilesService for DefaultProfilesService {
    async fn get(&self, id: &str) -> Result<Option<Profile>> {
        let profile = profile_model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?;

        Ok(profile.map(|p| p.into()))
    }

    async fn get_by_user_id(&self, user_id: &str) -> Result<Option<Profile>> {
        let profile = profile_model::Entity::find()
            .filter(profile_model::Column::UserId.eq(user_id.to_owned()))
            .one(&*self.db)
            .await?;

        Ok(profile.map(|p| p.into()))
    }

    async fn create(&self, input: &CreateProfileInput) -> Result<Profile> {
        let profile = profile_model::ActiveModel {
            email: Set(input.email.clone()),
            display_name: Set(input.display_name.clone()),
            picture: Set(input.picture.clone()),
            content: Set(input.content.clone()),
            city: Set(input.city.clone()),
            state_province: Set(input.state_province.clone()),
            user_id: Set(input.user_id.clone()),
            ..Default::default()
        }
        .insert(&*self.db)
        .await?;

        Ok(profile.into())
    }

    async fn get_or_create(&self, user_id: &str, input: &CreateProfileInput) -> Result<Profile> {
        let profile = self.get_by_user_id(user_id).await?;

        if let Some(profile) = profile {
            return Ok(profile);
        }

        self.create(input).await
    }

    async fn update(&self, id: &str, input: &UpdateProfileInput) -> Result<Profile> {
        let profile = profile_model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?;
        let mut profile: profile_model::ActiveModel = profile.unwrap().into();

        if let Some(email) = &input.email {
            profile.email = Set(email.clone());
        }

        if let Some(display_name) = &input.display_name {
            profile.display_name = Set(Some(display_name.clone()));
        }

        if let Some(picture) = &input.picture {
            profile.picture = Set(Some(picture.clone()));
        }

        if let Some(content) = &input.content {
            profile.content = Set(Some(content.clone()));
        }

        if let Some(city) = &input.city {
            profile.city = Set(Some(city.clone()));
        }

        if let Some(state_province) = &input.state_province {
            profile.state_province = Set(Some(state_province.clone()));
        }

        if let Some(user_id) = &input.user_id {
            profile.user_id = Set(Some(user_id.clone()));
        }

        Ok(profile.update(&*self.db).await?.into())
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let profile = profile_model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?;

        let _res = profile.unwrap().delete(&*self.db).await?;

        Ok(())
    }
}
