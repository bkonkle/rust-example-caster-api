use async_trait::async_trait;
use sqlx::postgres::PgPool;
use std::sync::Arc;

#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::user_model::User;

/// The Users entity repository
#[cfg_attr(test, automock)]
#[async_trait]
pub trait UsersRepository: Sync + Send {
    /// Get an individual User by id
    async fn get(&self, id: String) -> anyhow::Result<Option<User>>;
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
    async fn get(&self, id: String) -> anyhow::Result<Option<User>> {
        let show = sqlx::query_as!(
            User,
            r#"
                SELECT * FROM "users"
                    WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&*self.pg_pool)
        .await?;

        Ok(show)
    }
}
