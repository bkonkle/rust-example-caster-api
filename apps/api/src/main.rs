//! # A GraphQL server written in Rust
#![forbid(unsafe_code)]

use dotenv::dotenv;
use std::{env, net::SocketAddr};
use warp::{Filter, Future};

use crate::router::create_routes;

mod graphql;
mod postgres;
mod router;

#[macro_use]
extern crate log;

#[cfg(test)]
mod tests;

/// Use the current environment to get the port to start on, defaulting to a random port.
pub fn get_port() -> u16 {
    env::var("PORT")
        .unwrap_or_else(|_| String::from("0"))
        .parse()
        .unwrap_or(0)
}

/// Start the server and return the bound address and a `Future`.
pub async fn run(port: u16) -> (SocketAddr, impl Future<Output = ()>) {
    dotenv().ok();
    pretty_env_logger::init();

    let pg_pool = postgres::init()
        .await
        .expect("Unable to initialize Postgres pool.");
    let router = create_routes(pg_pool);

    warp::serve(router.with(warp::log("caster_api"))).bind_ephemeral(([0, 0, 0, 0], port))
}

/// Run the server and log where to find it
#[tokio::main]
async fn main() {
    let (addr, server) = run(get_port()).await;

    info!("Started at: http://localhost:{port}", port = addr.port());

    info!(
        "GraphQL at: http://localhost:{port}/graphql",
        port = addr.port()
    );

    server.await;
}
