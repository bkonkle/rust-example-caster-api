use anyhow::Result;
use hyper::{client::HttpConnector, Client};
use hyper_tls::HttpsConnector;
use once_cell::sync::{Lazy, OnceCell};
use sea_orm::DatabaseConnection;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::time::sleep;

use caster_api::{run, Dependencies};
use caster_shows::shows_service::ShowsService;
use caster_users::{profiles_service::ProfilesService, users_service::UsersService};
use caster_utils::{
    config::{get_config, Config},
    http::http_client,
    test::{graphql::GraphQL, oauth2::OAuth2Utils},
};

static HTTP_CLIENT: Lazy<Client<HttpsConnector<HttpConnector>>> = Lazy::new(http_client);
static OAUTH: OnceCell<OAuth2Utils> = OnceCell::new();

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
    pub db: Arc<DatabaseConnection>,
    pub users: Arc<dyn UsersService>,
    pub profiles: Arc<dyn ProfilesService>,
    pub shows: Arc<dyn ShowsService>,
}

/// Initialize common test utils
pub async fn init_test() -> Result<TestUtils> {
    let _ = pretty_env_logger::try_init();

    let config = get_config();

    let http_client = &HTTP_CLIENT;
    let addr = run_server(config).await?;

    let oauth = OAUTH.get_or_init(|| OAuth2Utils::new(config));

    let graphql = GraphQL::new(format!(
        "http://localhost:{port}/graphql",
        port = addr.port()
    ));

    // This needs to be created anew each time because it can't be shared when the Tokio runtime
    // is being stopped and re-started between tests
    let db = Arc::new(
        sea_orm::Database::connect(&config.database.url)
            .await
            .unwrap(),
    );

    let Dependencies {
        users,
        profiles,
        shows,
    } = Dependencies::new(&db);

    Ok(TestUtils {
        config,
        http_client,
        oauth,
        graphql,
        db,
        users,
        profiles,
        shows,
    })
}
