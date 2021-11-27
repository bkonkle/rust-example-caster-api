use async_trait::async_trait;
use sqlx::postgres::PgPool;
use std::sync::Arc;

#[cfg(test)]
use mockall::automock;

use crate::show_model::Show;

/// A ShowsRepository provides CRUD data access operations for the Show entity
#[cfg_attr(test, automock)]
#[async_trait]
pub trait ShowsRepository: Sync + Send {
    /// Get an individual Show by id
    async fn get(&self, id: String) -> anyhow::Result<Option<Show>>;
}

/// The default `ShowsRepository` instance based on Postgres
pub struct PgShowsRepository {
    /// The Postgres Pool
    pg_pool: Arc<PgPool>,
}

impl PgShowsRepository {
    /// Create a new `PgShowsRepository` instance with a `Pool<Postgres>`
    pub fn new(pg_pool: &Arc<PgPool>) -> Self {
        Self {
            pg_pool: pg_pool.clone(),
        }
    }
}

#[async_trait]
impl ShowsRepository for PgShowsRepository {
    async fn get(&self, id: String) -> anyhow::Result<Option<Show>> {
        let show = sqlx::query_as!(
            Show,
            r#"
                SELECT * FROM "shows"
                    WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&*self.pg_pool)
        .await?;

        Ok(show)
    }
}
