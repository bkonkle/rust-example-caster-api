use anyhow::Result;
use hyper::{body::to_bytes, client::HttpConnector, Body, Client, Method, Request, Response};
use serde::{de, Deserialize, Serialize};
use std::sync::Arc;

use crate::config::Config;

#[derive(Debug, Serialize)]
pub struct TokenRequest {
    grant_type: &'static str,
    username: String,
    password: String,
    client_id: String,
    client_secret: String,
    scope: &'static str,
    audience: String,
}
#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    access_token: String,
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

    async fn get_token(&self, username: String, password: String) -> Result<()> {
        let body = serde_json::to_string(&TokenRequest {
            grant_type: "password",
            username,
            password,
            client_id: self
                .config
                .auth
                .client
                .id
                .as_ref()
                .expect("Auth client id is required")
                .into(),
            client_secret: self
                .config
                .auth
                .client
                .secret
                .as_ref()
                .expect("Auth client secret is required")
                .into(),
            scope: "openid profile email",
            audience: self.config.auth.audience.clone(),
        })?;

        let req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}/oauth/token", &self.config.auth.url))
            .body(Body::from(body))?;

        let response = self.client.request(req).await?;
        let body = to_bytes(response.into_body()).await?;

        Ok(())
    }

    fn get_test_token(&self) -> Result<()> {
        let username = self
            .config
            .auth
            .test
            .user
            .username
            .as_ref()
            .expect("A test user username is required")
            .into();

        let password = self
            .config
            .auth
            .test
            .user
            .password
            .as_ref()
            .expect("A test user password is required")
            .into();

        self.get_token(username, password)
    }

    fn get_alt_token(&self) -> Result<()> {
        let username = self
            .config
            .auth
            .test
            .user
            .username
            .as_ref()
            .expect("A test user username is required")
            .into();

        let password = self
            .config
            .auth
            .test
            .user
            .password
            .as_ref()
            .expect("A test user password is required")
            .into();

        self.get_token(username, password)
    }

    fn get_user_info(&self) -> Result<()> {
        Ok(())
    }

    fn get_credentials(&self) -> Result<()> {
        Ok(())
    }
}
