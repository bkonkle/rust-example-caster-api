use dotenv::dotenv;
use sqlx::Error;
use std::env;
use std::net::SocketAddr;

use crate::{postgres, routes::create_routes};

#[tokio::test]
#[ignore]
async fn test_initial() -> Result<(), Error> {
    dotenv().ok();
    pretty_env_logger::init();

    let port = env::var("PORT").unwrap_or_else(|_| String::from("3000"));
    let addr = format!("http://localhost:{port}", port = port);

    let pg_pool = postgres::init().await?;
    let filter = create_routes(pg_pool);

    info!("Started at: {addr}", addr = addr);

    let socket_addr: SocketAddr = match addr.parse() {
        Ok(file) => file,
        Err(_) => ([0, 0, 0, 0], 3000).into(),
    };

    let _server = warp::serve(filter).run(socket_addr);

    Ok(())
}
