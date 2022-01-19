use anyhow::Result;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde_derive::Deserialize;

/// Database pool config
#[derive(Debug, Deserialize)]
pub struct DbPool {
    /// Database pool min
    pub min: Option<i16>,
    /// Database pool max
    pub max: Option<i16>,
}

/// Database config
#[derive(Debug, Deserialize)]
pub struct Database {
    /// Database hostname/IP
    pub host: String,
    /// Database username
    pub username: String,
    /// Database password
    pub password: String,
    /// Database name
    pub name: String,
    /// Database port
    pub port: u16,
    /// Full database url
    pub url: String,
    /// Database debug logging
    pub debug: bool,
    /// Database pool config
    pub pool: DbPool,
}

/// Redis config
#[derive(Debug, Deserialize)]
pub struct Redis {
    /// Redis url
    pub url: String,
}

/// Auth client config
#[derive(Debug, Deserialize)]
pub struct AuthClient {
    /// OAuth2 client id
    pub id: Option<String>,
    /// OAuth2 client secret
    pub secret: Option<String>,
}

/// Auth test user config
#[derive(Debug, Deserialize)]
pub struct AuthTestUser {
    /// Test user username
    pub username: Option<String>,
    /// Test user password
    pub password: Option<String>,
}

/// Auth test config
#[derive(Debug, Deserialize)]
pub struct AuthTest {
    /// Auth test user config
    pub user: AuthTestUser,
    /// Auth alt test user config
    pub alt: AuthTestUser,
}

/// Auth config
#[derive(Debug, Deserialize)]
pub struct Auth {
    /// OAuth2 url
    pub url: String,
    /// OAuth2 audience
    pub audience: String,
    /// Auth client config
    pub client: AuthClient,
    /// Auth test config
    pub test: AuthTest,
}

/// Application Config
#[derive(Debug, Deserialize)]
pub struct Config {
    /// The application's run mode (typically "development" or "production")
    pub app_env: String,
    /// The port to bind to
    pub port: u16,
    /// Database config
    pub database: Database,
    /// Redis config
    pub redis: Redis,
    /// Auth config
    pub auth: Auth,
}

impl Config {
    /// Create a new Config by merging in various sources
    pub fn new() -> Result<Self> {
        let config: Config = Figment::new()
            .merge(Toml::file("config/default.toml"))
            .merge(Toml::file("config/local.toml"))
            .merge(Env::prefixed("APP_"))
            .merge(Env::prefixed("DATABASE_"))
            .merge(Env::prefixed("REDIS_"))
            .merge(Env::prefixed("AUTH_"))
            .merge(Env::raw().only(&["PORT"]))
            .extract()?;

        Ok(config)
    }
}
