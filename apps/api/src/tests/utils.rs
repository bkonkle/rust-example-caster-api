use anyhow::Result;
use hyper::{client::HttpConnector, Client};
use hyper_tls::HttpsConnector;
use once_cell::sync::{Lazy, OnceCell};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::time::sleep;

use crate::run;
use caster_shows::shows_repository::PgShowsRepository;
use caster_users::users_repository::PgUsersRepository;
use caster_utils::{
    config::{get_config, Config},
    http::http_client,
    test::{graphql::GraphQL, oauth2::OAuth2Utils},
};

static HTTP_CLIENT: Lazy<Client<HttpsConnector<HttpConnector>>> = Lazy::new(http_client);
static OAUTH: OnceCell<OAuth2Utils> = OnceCell::new();

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

/// Common test utils
pub struct TestUtils {
    pub config: &'static Config,
    pub http_client: &'static Client<HttpsConnector<HttpConnector>>,
    pub oauth: &'static OAuth2Utils,
    pub graphql: GraphQL,
    pub pool: Arc<PgPool>,
    pub users: PgUsersRepository,
    pub shows: PgShowsRepository,
}

/// Initialize common test utils
pub async fn init_test() -> Result<TestUtils> {
    pretty_env_logger::init();

    let config = get_config();

    let http_client = get_http_client();
    let addr = run_server(config).await?;

    let oauth = OAUTH.get_or_init(|| OAuth2Utils::new(config));

    let graphql = GraphQL::new(format!(
        "http://localhost:{port}/graphql",
        port = addr.port()
    ));

    let pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(10)
            .connect(&config.database.url)
            .await?,
    );

    let shows = PgShowsRepository::new(&pool);
    let users = PgUsersRepository::new(&pool);

    Ok(TestUtils {
        config,
        http_client,
        oauth,
        graphql,
        pool,
        users,
        shows,
    })
}
