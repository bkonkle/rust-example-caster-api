use anyhow::Result;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sea_orm::{entity::*, query::*, DatabaseConnection, EntityTrait};
use std::sync::Arc;

use crate::{
    profile_model::{self, Profile},
    profile_mutations::{CreateProfileInput, UpdateProfileInput},
    user_model::{self, User},
};

/// A ProfilesService appliies business logic to a dynamic ProfilesRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait ProfilesService: Sync + Send {
    /// Get an individual `Profile` by id
    async fn get(&self, id: &str, with_user: &bool) -> Result<Option<Profile>>;

    /// Get the first `Profile` with this user_id
    async fn get_by_user_id(&self, user_id: &str, with_user: &bool) -> Result<Option<Profile>>;

    /// Get or create a `Profile`.
    async fn get_or_create(
        &self,
        user_id: &str,
        input: &CreateProfileInput,
        with_user: &bool,
    ) -> Result<Profile>;

    /// Create a `Profile` with the given input
    async fn create(&self, input: &CreateProfileInput, with_user: &bool) -> Result<Profile>;

    /// Update an existing `Profile`
    async fn update(
        &self,
        id: &str,
        input: &UpdateProfileInput,
        with_user: &bool,
    ) -> Result<Profile>;

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
    async fn get(&self, id: &str, with_user: &bool) -> Result<Option<Profile>> {
        let query = profile_model::Entity::find_by_id(id.to_owned());

        let profile = match with_user {
            true => query
                .find_with_related(user_model::Entity)
                .one(&*self.db)
                .await?
                .map(|(profile, user)| Profile {
                    user: Box::new(user),
                    ..profile.into()
                }),
            false => query.one(&*self.db).await?.map(|p| p.into()),
        };

        Ok(profile)
    }

    async fn get_by_user_id(&self, user_id: &str, with_user: &bool) -> Result<Option<Profile>> {
        let query = profile_model::Entity::find()
            .filter(profile_model::Column::UserId.eq(user_id.to_owned()));

        let profile = match with_user {
            true => query
                .find_with_related(user_model::Entity)
                .one(&*self.db)
                .await?
                .map(|(profile, user)| Profile {
                    user: Box::new(user),
                    ..profile.into()
                }),
            false => query.one(&*self.db).await?.map(|p| p.into()),
        };

        Ok(profile)
    }

    async fn create(&self, input: &CreateProfileInput, with_user: &bool) -> Result<Profile> {
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

        let mut created: Profile = profile.into();

        if !with_user {
            return Ok(created);
        }

        if let Some(user_id) = &input.user_id {
            let user = user_model::Entity::find_by_id(user_id.clone())
                .one(&*self.db)
                .await?;

            created.user = Box::new(user);

            return Ok(created);
        }

        Ok(created)
    }

    async fn get_or_create(
        &self,
        user_id: &str,
        input: &CreateProfileInput,
        with_user: &bool,
    ) -> Result<Profile> {
        let profile = self.get_by_user_id(user_id, with_user).await?;

        if let Some(profile) = profile {
            return Ok(profile);
        }

        self.create(input, with_user).await
    }

    async fn update(
        &self,
        id: &str,
        input: &UpdateProfileInput,
        with_user: &bool,
    ) -> Result<Profile> {
        let query = profile_model::Entity::find_by_id(id.to_owned());
        let mut user: Option<User> = None;

        let profile = match with_user {
            true => query
                .find_with_related(user_model::Entity)
                .one(&*self.db)
                .await?
                .map(|(profile, related_user)| {
                    // Save the User for later
                    user = related_user;

                    profile
                }),
            false => query.one(&*self.db).await?,
        };

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

        let mut updated: Profile = profile.update(&*self.db).await?.into();

        // Add back the User from above
        updated.user = Box::new(user);

        Ok(updated)
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let profile = profile_model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?;

        let _res = profile.unwrap().delete(&*self.db).await?;

        Ok(())
    }
}
