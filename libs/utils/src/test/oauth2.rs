use anyhow::Result;
use hyper::{body::to_bytes, client::HttpConnector, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;

use crate::config::Config;
use crate::http::http_client;

static TEST_CREDENTIALS: OnceCell<Credentials> = OnceCell::const_new();
static ALT_CREDENTIALS: OnceCell<Credentials> = OnceCell::const_new();

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

/// Utils for interacting with an `OAuth2` service during integration testing
pub struct OAuth2Utils {
    config: &'static Config,
    client: Client<HttpsConnector<HttpConnector>>,
}

impl OAuth2Utils {
    /// Create a new instance of the `OAuth2Utils` with the given config Arc reference
    pub fn new(config: &'static Config) -> Self {
        OAuth2Utils {
            client: http_client(),
            config,
        }
    }

    async fn get_token(&self, username: String, password: String) -> Result<Option<String>> {
        let client_id = self
            .config
            .auth
            .client
            .id
            .as_ref()
            .expect("No client_id found in config")
            .clone();

        let client_secret = self
            .config
            .auth
            .client
            .secret
            .as_ref()
            .expect("No client_secret found in config")
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
    pub async fn get_credentials(&self, user: User) -> &'static Credentials {
        // Pick the user settings and OnceCell reference to use
        let (test_user, cell) = match user {
            User::Test => (&self.config.auth.test.user, &TEST_CREDENTIALS),
            User::Alt => (&self.config.auth.test.alt, &ALT_CREDENTIALS),
        };

        // Retrieve the requested credentials if not yet present
        cell.get_or_init(|| async {
            let username = test_user
                .username
                .as_ref()
                .unwrap_or_else(|| panic!("No username found for: {:?}", user))
                .to_string();
            let password = test_user
                .password
                .as_ref()
                .unwrap_or_else(|| panic!("No password found for: {:?}", user))
                .to_string();
            let access_token = self
                .get_token(username, password)
                .await
                .expect("Unable to get an access token")
                .expect("No access token returned");

            let user_info = self
                .get_user_info(&access_token)
                .await
                .expect("Unable to get user info");
            let username = user_info.sub.expect("No subject/username found");
            let email = user_info.email.expect("No email address found");

            Credentials {
                username,
                email,
                access_token,
            }
        })
        .await
    }
}
