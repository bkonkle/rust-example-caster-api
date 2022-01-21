//! # A GraphQL server written in Rust
#![forbid(unsafe_code)]

use anyhow::Result;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, sync::Arc};
use warp::{Filter, Future};

use crate::router::create_routes;
use caster_auth::jwks::get_jwks;
use caster_utils::config::get_config;

mod graphql;
mod router;

#[macro_use]
extern crate log;

#[cfg(test)]
mod tests;

/// Start the server and return the bound address and a `Future`.
pub async fn run() -> Result<(SocketAddr, impl Future<Output = ()>)> {
    let config = get_config();
    let port = config.port;
    let jwks = get_jwks(config).await;

    let pg_pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(10)
            .connect(&config.database.url)
            .await?,
    );
    let router = create_routes(pg_pool, config, jwks);

    Ok(warp::serve(router.with(warp::log("caster_api"))).bind_ephemeral(([0, 0, 0, 0], port)))
}

/// Run the server and log where to find it
#[tokio::main]
async fn main() -> Result<()> {
    // Load varoables from .env, failing silently
    dotenv().ok();

    // Set RUST_LOG=info (or your desired loglevel) to see logging
    pretty_env_logger::init();

    let (addr, server) = run().await?;

    info!("Started at: http://localhost:{port}", port = addr.port());

    info!(
        "GraphQL at: http://localhost:{port}/graphql",
        port = addr.port()
    );

    server.await;

    Ok(())
}
