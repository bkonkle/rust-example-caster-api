use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::env;

pub struct AppConfig {}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        Ok(AppConfig {})
    }
}
