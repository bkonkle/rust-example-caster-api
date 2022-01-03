use anyhow::Result;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sqlx::postgres::PgPool;
use std::sync::Arc;

use crate::user_model::User;

/// The Users entity repository
#[cfg_attr(test, automock)]
#[async_trait]
pub trait UsersRepository: Sync + Send {
    /// Get a `User` from the "users" table by id
    async fn get(&self, id: &str) -> Result<Option<User>>;

    /// Get a `User` from the "users" table by username
    async fn get_by_username(&self, username: &str) -> Result<Option<User>>;

    /// Create a `User` from input
    async fn create(&self, username: &str) -> Result<User>;

    /// Update an existing `User` by id
    async fn update(
        &self,
        id: &str,
        username: &Option<String>,
        is_active: &Option<bool>,
    ) -> Result<User>;
}

/// A `UsersRepository` instance based on Postgres
pub struct PgUsersRepository {
    /// The Postgres Pool
    pg_pool: Arc<PgPool>,
}

impl PgUsersRepository {
    /// Create a new `PgUsersRepository` instance with a `Pool<Postgres>`
    pub fn new(pg_pool: &Arc<PgPool>) -> Self {
        Self {
            pg_pool: pg_pool.clone(),
        }
    }
}

#[async_trait]
impl UsersRepository for PgUsersRepository {
    async fn get(&self, id: &str) -> Result<Option<User>> {
        Ok(sqlx::query_as!(
            User,
            r#"
                SELECT * FROM users
                    WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&*self.pg_pool)
        .await?)
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>> {
        Ok(sqlx::query_as!(
            User,
            r#"
                SELECT * FROM users
                    WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&*self.pg_pool)
        .await?)
    }

    async fn create(&self, username: &str) -> Result<User> {
        Ok(sqlx::query_as!(
            User,
            r#"
                INSERT INTO users (username)
                VALUES ($1)
                RETURNING *
            "#,
            username
        )
        .fetch_one(&*self.pg_pool)
        .await?)
    }

    async fn update(
        &self,
        id: &str,
        username: &Option<String>,
        is_active: &Option<bool>,
    ) -> Result<User> {
        match (username, is_active) {
            (Some(username), Some(is_active)) => Ok(sqlx::query_as!(
                User,
                r#"
                    UPDATE users
                    SET username = $2, is_active = $3
                    WHERE id = $1
                    RETURNING *
                "#,
                id,
                username,
                is_active,
            )
            .fetch_one(&*self.pg_pool)
            .await?),

            (None, Some(is_active)) => Ok(sqlx::query_as!(
                User,
                r#"
                    UPDATE users
                    SET is_active = $2
                    WHERE id = $1
                    RETURNING *
                "#,
                id,
                is_active,
            )
            .fetch_one(&*self.pg_pool)
            .await?),

            (Some(username), None) => Ok(sqlx::query_as!(
                User,
                r#"
                    UPDATE users
                    SET username = $2
                    WHERE id = $1
                    RETURNING *
                "#,
                id,
                username,
            )
            .fetch_one(&*self.pg_pool)
            .await?),

            (None, None) => Err(anyhow!("Not implemented")),
        }
    }
}
