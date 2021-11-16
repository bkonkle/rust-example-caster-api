use async_trait::async_trait;
use shaku::{Component, Interface};
use sqlx::postgres::PgPool;
use std::sync::Arc;

#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::show_model::Show;

/// The Shows entity repository
#[cfg_attr(test, automock)]
#[async_trait]
pub trait ShowsRepository: Interface {
    /// Get an individual Show by id
    async fn get(&self, id: String) -> anyhow::Result<Option<Show>>;
}

/// A `ShowsRepository` instance based on Postgres
#[derive(Component)]
#[shaku(interface = ShowsRepository)]
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
