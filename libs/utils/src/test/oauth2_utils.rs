use anyhow::Result;
use hyper::{body::to_bytes, client::HttpConnector, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

use super::http_utils::http_client;
use crate::config::Config;

#[derive(Debug, Serialize)]
struct TokenRequest {
    grant_type: &'static str,
    username: String,
    password: String,
    client_id: String,
    client_secret: String,
    scope: &'static str,
    audience: String,
}
#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UserInfoResponse {
    sub: Option<String>,
    email: Option<String>,
}

/// Credentials for test users
#[derive(Debug)]
pub struct Credentials {
    /// The test user subscriber id
    pub username: String,
    /// The test user email
    pub email: String,
    /// The access token generated for the test user
    pub access_token: String,
}

/// Which user to retrieve a token for
#[derive(Debug)]
pub enum User {
    /// The default test user
    Test,
    /// The alternate test user
    Alt,
}

/// Possible errors during token retrieval
#[derive(Debug, Error)]
pub enum OAuth2UtilsError {
    /// No username found in config
    #[error("No username found in config")]
    ConfigUsername,
    /// No password found in config
    #[error("No password found in config")]
    ConfigPassword,
    /// "No client_id found in config"
    #[error("No client_id found in config")]
    ConfigClientId,
    /// No client_secret found in config
    #[error("No client_secret found in config")]
    ConfigClientSecret,
    /// No access token found on result
    #[error("No access token found on result")]
    AccessToken,
    /// No sub found on result
    #[error("No sub found on result")]
    Sub,
    /// No email found on result
    #[error("No email found on result")]
    Email,
}

/// Utils for interacting with an `OAuth2` service during integration testing
pub struct OAuth2Utils {
    config: Arc<Config>,
    client: Client<HttpsConnector<HttpConnector>>,
}

impl OAuth2Utils {
    /// Create a new instance of the `OAuth2Utils` with the given config Arc reference
    pub fn new(config: &Arc<Config>) -> Self {
        let client = http_client();

        OAuth2Utils {
            client,
            config: config.clone(),
        }
    }

    async fn get_token(&self, username: String, password: String) -> Result<Option<String>> {
        let client_id = self
            .config
            .auth
            .client
            .id
            .as_ref()
            .ok_or(OAuth2UtilsError::ConfigClientId)?
            .clone();

        let client_secret = self
            .config
            .auth
            .client
            .secret
            .as_ref()
            .ok_or(OAuth2UtilsError::ConfigClientSecret)?
            .clone();

        let body = serde_json::to_string(&TokenRequest {
            grant_type: "password",
            username,
            password,
            client_id,
            client_secret,
            scope: "openid profile email",
            audience: self.config.auth.audience.clone(),
        })?;

        let req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}/oauth/token", &self.config.auth.url))
            .header("Content-Type", "application/json")
            .body(Body::from(body))?;

        let response = self.client.request(req).await?;
        let body = to_bytes(response.into_body()).await?;
        let json = serde_json::from_slice::<TokenResponse>(&body)?;

        Ok(json.access_token)
    }

    async fn get_user_info(&self, token: &str) -> Result<UserInfoResponse> {
        let req = Request::builder()
            .method(Method::GET)
            .uri(format!("{}/userinfo", &self.config.auth.url))
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())?;

        let response = self.client.request(req).await?;
        let body = to_bytes(response.into_body()).await?;
        let json = serde_json::from_slice::<UserInfoResponse>(&body)?;

        Ok(json)
    }

    /// Get credentials for one of the test users
    pub async fn get_credentials(&self, user: User) -> Result<Credentials> {
        let user = match user {
            User::Test => &self.config.auth.test.user,
            User::Alt => &self.config.auth.test.alt,
        };

        let username = user
            .username
            .as_ref()
            .ok_or(OAuth2UtilsError::ConfigUsername)?
            .clone();

        let password = user
            .password
            .as_ref()
            .ok_or(OAuth2UtilsError::ConfigPassword)?
            .clone();

        let access_token = self
            .get_token(username, password)
            .await?
            .ok_or(OAuth2UtilsError::AccessToken)?;

        let user_info = self.get_user_info(&access_token).await?;
        let username = user_info.sub.ok_or(OAuth2UtilsError::Sub)?;
        let email = user_info.email.ok_or(OAuth2UtilsError::Email)?;

        Ok(Credentials {
            username,
            email,
            access_token,
        })
    }
}
