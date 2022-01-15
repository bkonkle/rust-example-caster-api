use anyhow::Result;
use hyper::{body::to_bytes, client::HttpConnector, Body, Client, Method, Request, Response};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

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

#[derive(Debug)]
pub struct Credentials {
    username: String,
    email: String,
    access_token: String,
}

#[derive(Debug)]
pub enum Users {
    /// The default test user
    Test,
    /// The alternate test user
    Alt,
}

#[derive(Debug, Error)]
pub enum OAuth2UtilsError {
    #[error("No username found in config")]
    ConfigUsername,
    #[error("No password found in config")]
    ConfigPassword,
    #[error("No client_id found in config")]
    ConfigClientId,
    #[error("No client_secret found in config")]
    ConfigClientSecret,
    #[error("No access token found on result")]
    AccessToken,
    #[error("No sub found on result")]
    Sub,
    #[error("No email found on result")]
    Email,
}

pub struct OAuth2Utils {
    config: Arc<Config>,
    client: Client<HttpConnector>,
}

impl OAuth2Utils {
    pub fn new(config: &Arc<Config>) -> Result<Self> {
        let client = Client::new();

        Ok(OAuth2Utils {
            client,
            config: config.clone(),
        })
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
            .body(Body::from(body))?;

        let response = self.client.request(req).await?;
        let body = to_bytes(response.into_body()).await?;
        let json = serde_json::from_slice::<TokenResponse>(&body)?;

        Ok(json.access_token)
    }

    async fn get_user_info(&self) -> Result<UserInfoResponse> {
        let response = self
            .client
            .get(format!("{}/userinfo", &self.config.auth.url).parse()?)
            .await?;

        let body = to_bytes(response.into_body()).await?;

        Ok(serde_json::from_slice::<UserInfoResponse>(&body)?)
    }

    pub async fn get_credentials(&self, user: Users) -> Result<Credentials> {
        let user = match user {
            Users::Test => &self.config.auth.test.user,
            Users::Alt => &self.config.auth.test.alt,
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

        let user_info = self.get_user_info().await?;
        let username = user_info.sub.ok_or(OAuth2UtilsError::Sub)?;
        let email = user_info.email.ok_or(OAuth2UtilsError::Email)?;

        Ok(Credentials {
            username,
            email,
            access_token,
        })
    }
}
