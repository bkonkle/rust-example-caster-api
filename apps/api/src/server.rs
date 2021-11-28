//! # A GraphQL server written in Rust
#![forbid(unsafe_code)]

use std::env;
use std::net::SocketAddr;

pub fn get_addr() -> SocketAddr {
    let port = env::var("PORT").unwrap_or_else(|_| String::from("3000"));
    let addr = format!("http://localhost:{port}", port = port);

    addr.parse().unwrap_or_else(|_| ([0, 0, 0, 0], 3000).into())
}
