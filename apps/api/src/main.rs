//! # A GraphQL server written in Rust
#![forbid(unsafe_code)]

use dotenv::dotenv;
use sqlx::Error;
use std::env;
use std::net::SocketAddr;
use warp::Filter;

use crate::routes::create_routes;

#[macro_use]
extern crate log;

mod graphql;
mod postgres;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    pretty_env_logger::init();

    let port = env::var("PORT").unwrap_or_else(|_| String::from("3000"));
    let addr = format!("http://localhost:{port}", port = port);

    let pg_pool = postgres::init().await?;
    let filter = create_routes(pg_pool);

    info!("Started at: {addr}", addr = addr);

    let socket_addr: SocketAddr = match addr.parse() {
        Ok(address) => address,
        Err(_) => ([0, 0, 0, 0], 3000).into(),
    };

    warp::serve(filter.with(warp::log("caster_api")))
        .run(socket_addr)
        .await;

    Ok(())
}
