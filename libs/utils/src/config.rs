use config::{ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::env;

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
    /// The application environment
    pub env: String,
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
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = config::Config::new();

        // Start off by merging in the "default" configuration file
        config.merge(File::with_name("config/default"))?;

        // Add in the current environment-specific config file
        // Default to 'development' env
        // Note that this file is _optional_
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        config.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        config.merge(File::with_name("config/local").required(false))?;

        // Add in settings from the environment
        config.merge(Environment::new().separator("_"))?;

        config.try_into()
    }
}
