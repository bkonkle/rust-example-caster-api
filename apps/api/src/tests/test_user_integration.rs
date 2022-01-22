use anyhow::Result;
use hyper::{body::to_bytes, Body, Method, Request};

use crate::tests::utils::{get_http_client, run_server};
use caster_utils::{
    config::get_config,
    test::oauth2_utils::{OAuth2Utils, User},
};

#[tokio::test]
#[ignore]
async fn test_initial() -> Result<()> {
    pretty_env_logger::init();
    let http_client = get_http_client();
    let config = get_config();
    let oauth = OAuth2Utils::new(config);
    let addr = run_server(config).await?;

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

    let resp = http_client.request(req).await?;
    // assert_eq!(resp.status(), 200);

    let body_bytes = to_bytes(resp.into_body()).await?;
    assert_eq!(body_bytes, r#"{"data":null}"#);

    Ok(())
}
