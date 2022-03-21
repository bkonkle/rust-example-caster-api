use anyhow::Result;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use once_cell::sync::Lazy;
use serde_derive::Deserialize;
use std::env;

/// The default `Config` instance
static CONFIG: Lazy<Config> = Lazy::new(|| Config::new().expect("Unable to retrieve config"));

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
    pub hostname: String,
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
    pub run_mode: String,
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
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".to_string());

        let config: Config = Figment::new()
            // Load defaults
            .merge(Toml::file("config/default.toml"))
            // Load local overrides
            .merge(Toml::file("config/local.toml"))
            // Load run mode overrides
            .merge(Toml::file(format!("config/{}.toml", run_mode)))
            // Load environment variables
            .merge(
                // Support the nested structure of the config manually
                Env::raw()
                    // Split the Database variables
                    .map(|key| {
                        key.as_str()
                            .replace("DATABASE_POOL_", "DATABASE.POOL.")
                            .into()
                    })
                    .map(|key| key.as_str().replace("DATABASE_", "DATABASE.").into())
                    // Split the Redis variables
                    .map(|key| key.as_str().replace("REDIS_", "REDIS.").into())
                    // Split the Auth variables
                    .map(|key| {
                        key.as_str()
                            .replace("AUTH_TEST_USER_", "AUTH.TEST.USER.")
                            .into()
                    })
                    .map(|key| {
                        key.as_str()
                            .replace("AUTH_TEST_ALT_", "AUTH.TEST.ALT.")
                            .into()
                    })
                    .map(|key| key.as_str().replace("AUTH_CLIENT_", "AUTH.CLIENT.").into())
                    .map(|key| key.as_str().replace("AUTH_", "AUTH.").into()),
            )
            // Serialize and freeze
            .extract()?;

        Ok(config)
    }

    /// Return true if the `run_mode` is "development"
    pub fn is_dev(&self) -> bool {
        self.run_mode == "development"
    }
}

/// Get the default static `Config`
pub fn get_config() -> &'static Config {
    &CONFIG
}
