use anyhow::Result;
use hyper::body::to_bytes;

use super::utils::{init_test, TestUtils};
use caster_utils::test::oauth2::User;

#[tokio::test]
#[ignore]
async fn test_get_current_user() -> Result<()> {
    let TestUtils {
        http_client,
        oauth,
        graphql,
        ..
    } = init_test().await?;

    let credentials = oauth.get_credentials(User::Test).await?;
    let req = graphql.query(
        "
            query GetCurrentUser {
                getCurrentUser {
                id
                username
                isActive
                }
            }
        "
        .to_string(),
        Some(credentials.access_token),
    )?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body_bytes = to_bytes(resp.into_body()).await?;
    assert_eq!(body_bytes, r#"{"data":null}"#);
    assert_eq!(status, 200);

    Ok(())
}
