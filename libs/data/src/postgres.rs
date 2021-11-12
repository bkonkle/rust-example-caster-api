use mockall::automock;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Error;
use std::env;
use std::sync::Arc;

/// Initialize a new Postgres pool, or mock one in testing
pub struct PostgresPool {}

#[automock]
impl PostgresPool {
    /// Initialize a new Postgres pool for Prod
    pub async fn init() -> Result<Arc<PgPool>, Error> {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| String::from("postgresql://caster:caster@localhost:1701/caster"));

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(Arc::new(pool))
    }
}
