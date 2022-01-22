use anyhow::Result;
use hyper::{client::HttpConnector, Client};
use hyper_tls::HttpsConnector;
use once_cell::sync::Lazy;
use std::{net::SocketAddr, time::Duration};
use tokio::time::sleep;

use crate::run;
use caster_utils::{config::Config, http::http_client};

static HTTP_CLIENT: Lazy<Client<HttpsConnector<HttpConnector>>> = Lazy::new(http_client);

pub fn get_http_client() -> &'static Client<HttpsConnector<HttpConnector>> {
    &HTTP_CLIENT
}

pub async fn run_server(config: &'static Config) -> Result<SocketAddr> {
    let (addr, server) = run(config).await?;

    // Spawn the server in the background
    tokio::spawn(server);

    // Wait for it to initialize
    sleep(Duration::from_millis(200)).await;

    // Return the bound address
    Ok(addr)
}
