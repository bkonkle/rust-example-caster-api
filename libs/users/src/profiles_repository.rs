use anyhow::Result;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sqlx::postgres::PgPool;
use std::sync::Arc;

use crate::{
    profile_model::ProfileDB,
    profile_mutations::{CreateProfileInput, UpdateProfileInput},
};

/// The Profiles entity repository
#[cfg_attr(test, automock)]
#[async_trait]
pub trait ProfilesRepository: Sync + Send {
    /// Get a `Profile` from the "users" table by id
    async fn get(&self, id: &str) -> Result<Option<ProfileDB>>;

    /// Get a `Profile` from the "users" table by username
    async fn get_by_user_id(&self, user_id: &str) -> Result<Option<ProfileDB>>;

    /// Create a `Profile` from input
    async fn create(&self, input: &CreateProfileInput) -> Result<ProfileDB>;

    /// Update an existing `Profile` by id
    async fn update(&self, id: &str, input: &UpdateProfileInput) -> Result<ProfileDB>;

    /// Remove an existing `Profile` by id
    async fn delete(&self, id: &str) -> Result<()>;
}

/// A `ProfilesRepository` instance based on Postgres
pub struct PgProfilesRepository {
    /// The Postgres Pool
    pg_pool: Arc<PgPool>,
}

impl PgProfilesRepository {
    /// Create a new `PgProfilesRepository` instance with a `Pool<Postgres>`
    pub fn new(pg_pool: &Arc<PgPool>) -> Self {
        Self {
            pg_pool: pg_pool.clone(),
        }
    }
}

#[async_trait]
impl ProfilesRepository for PgProfilesRepository {
    async fn get(&self, id: &str) -> Result<Option<ProfileDB>> {
        Ok(sqlx::query_as!(
            ProfileDB,
            r#"
                SELECT id, email, display_name, picture, content, city, state_province, user_id, created_at, updated_at FROM profiles
                    WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&*self.pg_pool)
        .await?)
    }

    async fn get_by_user_id(&self, user_id: &str) -> Result<Option<ProfileDB>> {
        Ok(sqlx::query_as!(
            ProfileDB,
            r#"
                SELECT id, email, display_name, picture, content, city, state_province, user_id, created_at, updated_at FROM profiles
                    WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&*self.pg_pool)
        .await?)
    }

    async fn create(&self, input: &CreateProfileInput) -> Result<ProfileDB> {
        Ok(sqlx::query_as!(
            ProfileDB,
            r#"
                INSERT INTO profiles (email, display_name, picture, content, city, state_province, user_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id, email, display_name, picture, content, city, state_province, user_id, created_at, updated_at
            "#,
            input.email, input.display_name, input.picture, input.content, input.city, input.state_province, input.user_id
        )
        .fetch_one(&*self.pg_pool)
        .await?)
    }

    async fn update(&self, id: &str, input: &UpdateProfileInput) -> Result<ProfileDB> {
        Ok(sqlx::query_as!(
            ProfileDB,
            r#"
                    UPDATE profiles
                    SET email = COALESCE($2, email),
                        display_name = COALESCE($3, display_name),
                        picture = COALESCE($4, picture),
                        content = COALESCE($5, content),
                        city = COALESCE($6, city),
                        state_province = COALESCE($7, state_province),
                        user_id = COALESCE($8, user_id)
                    WHERE id = $1
                    RETURNING id, email, display_name, picture, content, city, state_province, user_id, created_at, updated_at
                "#,
            id,
            input.email as _,
            input.display_name,
            input.picture,
            input.content,
            input.city,
            input.state_province,
            input.user_id
        )
        .fetch_one(&*self.pg_pool)
        .await?)
    }

    async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query!(r#"DELETE FROM profiles WHERE id = $1"#, id)
            .fetch_optional(&*self.pg_pool)
            .await?;

        Ok(())
    }
}
