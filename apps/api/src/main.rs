//! # A GraphQL server written in Rust
#![forbid(unsafe_code)]

use dotenv::dotenv;
use sqlx::Error;
use warp::Filter;

use crate::{router::create_routes, server::get_addr};

#[macro_use]
extern crate log;

mod graphql;
mod postgres;
mod router;
mod server;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    pretty_env_logger::init();

    let pg_pool = postgres::init().await?;
    let router = create_routes(pg_pool);

    let addr = get_addr();

    info!("Started at: {addr}", addr = addr);

    warp::serve(router.with(warp::log("caster_api")))
        .run(addr)
        .await;

    Ok(())
}
