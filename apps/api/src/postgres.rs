use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Error;
use std::env;

/// Initialize a new Postgres pool
pub async fn init() -> Result<PgPool, Error> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| String::from("postgresql://caster:caster@localhost:1701/caster"));

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
