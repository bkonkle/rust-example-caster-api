use anyhow::Result;
use hyper::{body::to_bytes, client::HttpConnector, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use once_cell::sync::Lazy;
use std::sync::Arc;

use crate::tests::utils::run_server;
use caster_utils::{
    config::Config,
    test::{
        http_utils::http_client,
        oauth2_utils::{OAuth2Utils, User},
    },
};

static HTTP_CLIENT: Lazy<Client<HttpsConnector<HttpConnector>>> = Lazy::new(http_client);

#[tokio::test]
#[ignore]
async fn test_initial() -> Result<()> {
    pretty_env_logger::init();

    let config = Arc::new(Config::new()?);
    let oauth = OAuth2Utils::new(&config);
    let addr = run_server(&config).await?;

    let token = oauth.get_credentials(User::Test).await?;

    let req = Request::builder()
        .method(Method::POST)
        .uri(format!(
            "http://localhost:{port}/graphql",
            port = addr.port()
        ))
        .header("Authorization", format!("Bearer {}", token.access_token))
        .body(Body::from(
            "
                query allSchemaTypes {
                    __schema {
                        types {
                            name
                            kind
                            description
                        }
                    }
                }
            ",
        ))?;

    let resp = HTTP_CLIENT.request(req).await?;
    debug!("Response: {:?}", resp);
    assert_eq!(resp.status(), 200);

    let body_bytes = to_bytes(resp.into_body()).await?;
    assert_eq!(body_bytes, r#"{"data":null}"#);

    Ok(())
}
