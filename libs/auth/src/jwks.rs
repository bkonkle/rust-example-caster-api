use biscuit::{
    jwk::{AlgorithmParameters, JWKSet, JWK},
    jws::Secret,
    Empty,
};
use hyper::{body::to_bytes, client::HttpConnector, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use thiserror::Error;

use caster_utils::{config::Config, http::http_client};
use tokio::sync::OnceCell;

static JWKS: OnceCell<JWKS> = OnceCell::const_new();

/// A type alias for `JWKSet<Empty>`
pub type JWKS = JWKSet<Empty>;

/// Possible errors during jwks retrieval
#[derive(Debug, Error)]
pub enum JwksClientError {
    /// No key found with the given key_id
    #[error("No key found with the given key_id")]
    MissingKeyId,

    /// Unable to construct RSA public key secret
    #[error("Unable to construct RSA public key secret")]
    SecretKeyError,
}

/// A struct that can retrieve `JWKSet` from a configured Auth url
pub struct JwksClient {
    config: &'static Config,
    client: Client<HttpsConnector<HttpConnector>>,
}

impl JwksClient {
    /// Create a new instance of the `JwksClient` with the given config Arc reference
    pub fn new(config: &'static Config) -> Self {
        JwksClient {
            client: http_client(),
            config,
        }
    }

    /// Get a `JWKSet` from the configured Auth url
    pub async fn get_key_set(&self) -> anyhow::Result<JWKS> {
        let url = format!("{}/.well-known/jwks.json", &self.config.auth.url);

        debug!("Fetching keys from '{}'", url);

        let req = Request::builder()
            .method(Method::GET)
            .uri(url)
            .body(Body::empty())?;

        let response = self.client.request(req).await?;
        let body = to_bytes(response.into_body()).await?;
        let jwks = serde_json::from_slice::<JWKS>(&body)?;

        Ok(jwks)
    }
}

/// Get a particular key from a key set by id
pub fn get_key(jwks: &JWKS, key_id: &str) -> Result<JWK<Empty>, JwksClientError> {
    let key = jwks
        .find(key_id)
        .ok_or(JwksClientError::MissingKeyId)?
        .clone();

    Ok(key)
}

/// Convert a JWK into a Secret
pub fn get_secret(jwk: JWK<Empty>) -> Result<Secret, JwksClientError> {
    let secret = match jwk.algorithm {
        AlgorithmParameters::RSA(rsa_key) => rsa_key.jws_public_key_secret(),
        _ => return Err(JwksClientError::SecretKeyError),
    };

    Ok(secret)
}

/// A convenience function to get a particular key from a key set, and convert it into a secret
pub fn get_secret_from_key_set(jwks: &JWKS, key_id: &str) -> Result<Secret, JwksClientError> {
    let jwk = get_key(jwks, key_id)?;
    let secret = get_secret(jwk)?;

    Ok(secret)
}

async fn init_jwks(config: &'static Config) -> JWKS {
    let jwks_client = JwksClient::new(config);

    jwks_client
        .get_key_set()
        .await
        .expect("Unable to retrieve JWKS")
}

/// Get the default set of JWKS keys
pub async fn get_jwks(config: &'static Config) -> &'static JWKS {
    JWKS.get_or_init(|| init_jwks(config)).await
}
