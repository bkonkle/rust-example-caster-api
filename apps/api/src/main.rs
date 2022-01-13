//! # A GraphQL server written in Rust
#![forbid(unsafe_code)]

use dotenv::dotenv;
use std::net::SocketAddr;
use warp::{Filter, Future};

use crate::router::create_routes;
use caster_utils::config::Config;

mod graphql;
mod postgres;
mod router;

#[macro_use]
extern crate log;

#[cfg(test)]
mod tests;

/// Start the server and return the bound address and a `Future`.
pub async fn run() -> (SocketAddr, impl Future<Output = ()>) {
    let config = Config::new().unwrap();

    pretty_env_logger::init();

    let pg_pool = postgres::init()
        .await
        .expect("Unable to initialize Postgres pool.");
    let router = create_routes(pg_pool);

    warp::serve(router.with(warp::log("caster_api"))).bind_ephemeral(([0, 0, 0, 0], config.port))
}

/// Run the server and log where to find it
#[tokio::main]
async fn main() {
    dotenv().ok();

    let (addr, server) = run().await;

    info!("Started at: http://localhost:{port}", port = addr.port());

    info!(
        "GraphQL at: http://localhost:{port}/graphql",
        port = addr.port()
    );

    server.await;
}
