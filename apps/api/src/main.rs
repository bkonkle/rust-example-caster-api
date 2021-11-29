//! # A GraphQL server written in Rust
#![forbid(unsafe_code)]

use dotenv::dotenv;
use sqlx::Error;
use std::env;
use std::net::SocketAddr;
use warp::Filter;

use crate::router::create_routes;

mod graphql;
mod postgres;
mod router;

#[macro_use]
extern crate log;

#[cfg(test)]
mod tests;

#[macro_use]
#[cfg(test)]
extern crate lazy_static;

/// Use the current environment to get the socket address to start on
pub fn get_addr() -> SocketAddr {
    let port = env::var("PORT").unwrap_or_else(|_| String::from("3000"));
    let addr = format!("0.0.0.0:{port}", port = port);

    addr.parse().unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    pretty_env_logger::init();

    let pg_pool = postgres::init().await?;
    let router = create_routes(pg_pool);
    let addr = get_addr();

    info!("Starting at: http://localhost:{port}", port = addr.port());

    warp::serve(router.with(warp::log("caster_api")))
        .run(addr)
        .await;

    Ok(())
}
