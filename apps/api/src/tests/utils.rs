use std::{net::SocketAddr, time::Duration};

use hyper::{client::HttpConnector, Body, Client as HyperClient};
use hyper_tls::HttpsConnector;
use tokio::time::sleep;

use crate::run;

pub async fn run_server() -> SocketAddr {
    let (addr, server) = run().await;

    // Spawn the server in the background
    tokio::spawn(server);

    // Wait for it to initialize
    sleep(Duration::from_millis(200)).await;

    // Return the bound address
    addr
}

pub fn http_client() -> HyperClient<HttpsConnector<HttpConnector>> {
    let https = HttpsConnector::new();
    HyperClient::builder().build::<_, Body>(https)
}
