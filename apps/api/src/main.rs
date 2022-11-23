//! # A GraphQL server written in Rust
#![forbid(unsafe_code)]

use std::sync::Arc;

use anyhow::Result;

use caster_api::{run, Context};
use caster_utils::config::get_config;

#[macro_use]
extern crate log;

/// Run the server and log where to find it
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = get_config();
    let context = Arc::new(Context::init(config).await?);

    let server = run(context).await?;
    let addr = server.local_addr();

    if config.is_dev() {
        info!("Started at: http://localhost:{port}", port = addr.port());

        info!(
            "GraphQL at: http://localhost:{port}/graphql",
            port = addr.port()
        );
    } else {
        info!("Started on port: {port}", port = addr.port());
    };

    server.await?;

    Ok(())
}
