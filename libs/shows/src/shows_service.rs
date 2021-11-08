use async_trait::async_trait;
use sqlx::postgres::PgPool;
use std::sync::Arc;

use crate::show_models::Show;

/// The Shows entity service
#[mockall::automock]
#[async_trait]
pub trait ShowsService {
    /// Get an individual Show by id
    async fn get(&self, id: String) -> anyhow::Result<Option<Show>>;
}

/// A `ShowsServices` instance based on Postgres
pub struct PgShowsService {
    pg_pool: Arc<PgPool>,
}

impl PgShowsService {
    /// Create a new `ShowsService` instance with a `Pool<Postgres>`
    pub fn new(pg_pool: &Arc<PgPool>) -> Self {
        Self {
            pg_pool: pg_pool.clone(),
        }
    }
}

#[async_trait]
impl ShowsService for PgShowsService {
    async fn get(&self, id: String) -> anyhow::Result<Option<Show>> {
        let show = sqlx::query_as!(
            Show,
            r#"
                SELECT * FROM "Show"
                    WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&*self.pg_pool)
        .await?;

        Ok(show)
    }
}
