use async_trait::async_trait;
use mockall::automock;
use sqlx::postgres::PgPool;
use std::sync::Arc;

use crate::user_model::User;

/// The Users entity service
#[automock]
#[async_trait]
pub trait UsersService {
    /// Get an individual User by id
    async fn get(&self, id: String) -> anyhow::Result<Option<User>>;
}

/// A `UsersServices` instance based on Postgres
pub struct PgUsersService {
    pg_pool: Arc<PgPool>,
}

impl PgUsersService {
    /// Create a new `UsersService` instance with a `Pool<Postgres>`
    pub fn new(pg_pool: &Arc<PgPool>) -> Self {
        Self {
            pg_pool: pg_pool.clone(),
        }
    }
}

#[async_trait]
impl UsersService for PgUsersService {
    async fn get(&self, id: String) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
                SELECT * FROM "users"
                    WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&*self.pg_pool)
        .await?;

        Ok(user)
    }
}
