use anyhow::Result;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use std::sync::Arc;

use crate::{
    profile_model::Profile,
    profile_mutations::{CreateProfileInput, UpdateProfileInput},
    profiles_repository::ProfilesRepository,
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
    /// The ProfilesRepository instance
    repo: Arc<dyn ProfilesRepository>,
}

/// The default `ProfilesService` implementation
impl DefaultProfilesService {
    /// Create a new `ProfilesService` instance with a `ProfilesRepository` implementation
    pub fn new<Repo: ProfilesRepository + 'static>(repo: &Arc<Repo>) -> Self {
        Self { repo: repo.clone() }
    }
}

#[async_trait]
impl ProfilesService for DefaultProfilesService {
    async fn get(&self, id: &str) -> Result<Option<Profile>> {
        let profile = self.repo.get(id).await?;

        Ok(profile.map(|p| p.into()))
    }

    async fn get_by_user_id(&self, user_id: &str) -> Result<Option<Profile>> {
        let profile = self.repo.get_by_user_id(user_id).await?;

        Ok(profile.map(|p| p.into()))
    }

    async fn create(&self, input: &CreateProfileInput) -> Result<Profile> {
        let profile = self.repo.create(input).await?;

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
        let profile = self.repo.update(id, input).await?;

        Ok(profile.into())
    }

    async fn delete(&self, id: &str) -> Result<()> {
        self.repo.delete(id).await
    }
}
