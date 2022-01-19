use anyhow::Result;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::time::sleep;

use crate::run;
use caster_utils::config::Config;

pub async fn run_server(config: &Arc<Config>) -> Result<SocketAddr> {
    let (addr, server) = run(config.clone()).await?;

    // Spawn the server in the background
    tokio::spawn(server);

    // Wait for it to initialize
    sleep(Duration::from_millis(200)).await;

    // Return the bound address
    Ok(addr)
}
