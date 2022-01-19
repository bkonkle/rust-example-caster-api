//! # A GraphQL server written in Rust
#![forbid(unsafe_code)]

use anyhow::Result;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, sync::Arc};
use warp::{Filter, Future};

use crate::router::create_routes;
use caster_utils::config::Config;

mod graphql;
mod router;

#[macro_use]
extern crate log;

#[cfg(test)]
mod tests;

/// Start the server and return the bound address and a `Future`.
pub async fn run(config: Arc<Config>) -> Result<(SocketAddr, impl Future<Output = ()>)> {
    let port = config.port;

    let pg_pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(10)
            .connect(&config.database.url)
            .await?,
    );
    let router = create_routes(pg_pool, config);

    Ok(warp::serve(router.with(warp::log("caster_api"))).bind_ephemeral(([0, 0, 0, 0], port)))
}

/// Run the server and log where to find it
#[tokio::main]
async fn main() -> Result<()> {
    // Load varoables from .env, failing silently
    dotenv().ok();

    // Set RUST_LOG=info (or your desired loglevel) to see logging
    pretty_env_logger::init();

    let config = Arc::new(Config::new()?);
    let (addr, server) = run(config).await?;

    info!("Started at: http://localhost:{port}", port = addr.port());

    info!(
        "GraphQL at: http://localhost:{port}/graphql",
        port = addr.port()
    );

    server.await;

    Ok(())
}
