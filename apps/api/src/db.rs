use sqlx::Error;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub async fn init() -> Result<Pool<Postgres>, Error> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://caster:caster@localhost:1701/caster".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
