//! # A GraphQL server written in Rust
use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, sync::Arc};
use warp::{Filter, Future};

use caster_auth::jwks::get_jwks;
use caster_utils::config::Config;
use router::create_routes;

mod errors;
mod graphql;
mod router;

#[macro_use]
extern crate log;

/// Start the server and return the bound address and a `Future`.
pub async fn run(config: &'static Config) -> Result<(SocketAddr, impl Future<Output = ()>)> {
    let port = config.port;
    let jwks = get_jwks(config).await;

    let pg_pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(10)
            .connect(&config.database.url)
            .await?,
    );
    let router = create_routes(pg_pool, config, jwks);

    Ok(warp::serve(
        router
            .with(warp::log("caster_api"))
            .recover(errors::handle_rejection),
    )
    .bind_ephemeral(([0, 0, 0, 0], port)))
}
