//! # A GraphQL server written in Rust
#![forbid(unsafe_code)]

use anyhow::Result;
use dotenv::dotenv;

use caster_api::run;
use caster_utils::config::get_config;

#[macro_use]
extern crate log;

/// Run the server and log where to find it
#[tokio::main]
async fn main() -> Result<()> {
    // Load variables from .env, failing silently
    dotenv().ok();

    // Set RUST_LOG=info (or your desired loglevel) to see logging
    pretty_env_logger::init();

    let config = get_config();

    let (addr, server) = run(config).await?;

    info!("Started at: http://localhost:{port}", port = addr.port());

    info!(
        "GraphQL at: http://localhost:{port}/graphql",
        port = addr.port()
    );

    server.await;

    Ok(())
}
