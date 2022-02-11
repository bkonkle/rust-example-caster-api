use anyhow::Result;
use caster_users::profile_mutations::CreateProfileInput;
use hyper::body::to_bytes;
use serde_json::{json, Value};

use caster_utils::test::oauth2::{Credentials, User as TestUser};

mod test_utils;
use test_utils::{init_test, TestUtils};

/***
 * Query: `getCurrentUser`
 */

static GET_CURRENT_USER: &str = "
    query GetCurrentUser {
        getCurrentUser {
            id
            username
            isActive
            profile {
                id
                email
            }
        }
    }
";

/// It retrieves the currently authenticated user
#[tokio::test]
#[ignore]
async fn test_get_current_user() -> Result<()> {
    let TestUtils {
        http_client,
        oauth,
        graphql,
        users,
        profiles,
        ..
    } = init_test().await?;

    let Credentials {
        access_token: token,
        username,
        email,
    } = oauth.get_credentials(TestUser::Test).await;

    // Create a user with this username
    let user = users.create(username).await?;
    let profile = profiles
        .create(
            &CreateProfileInput {
                email: email.clone(),
                user_id: Some(user.id.clone()),
                display_name: None,
                picture: None,
                content: None,
                city: None,
                state_province: None,
            },
            &false,
        )
        .await?;

    let req = graphql.query(GET_CURRENT_USER, Value::Null, Some(token))?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    let json_user = &json["data"]["getCurrentUser"];
    let json_profile = &json_user["profile"];

    assert_eq!(status, 200);
    assert_eq!(json_user["id"], user.id);
    assert_eq!(json_user["username"], user.username);
    assert_eq!(json_user["isActive"], true);
    assert_eq!(json_profile["email"], email.clone());

    // Clean up
    users.delete(&user.id).await?;
    profiles.delete(&profile.id).await?;

    Ok(())
}

/// It returns null when no user is found
#[tokio::test]
#[ignore]
async fn test_get_current_user_no_user() -> Result<()> {
    let TestUtils {
        http_client,
        oauth,
        graphql,
        ..
    } = init_test().await?;

    let Credentials {
        access_token: token,
        ..
    } = oauth.get_credentials(TestUser::Test).await;

    let req = graphql.query(GET_CURRENT_USER, Value::Null, Some(token))?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(json["data"]["getCurrentUser"], Value::Null);
    assert_eq!(json["errors"], Value::Null);

    Ok(())
}

/// It requires authentication
#[tokio::test]
#[ignore]
async fn test_get_current_user_requires_authn() -> Result<()> {
    let TestUtils {
        http_client,
        graphql,
        ..
    } = init_test().await?;

    let req = graphql.query(GET_CURRENT_USER, Value::Null, None)?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(
        json["errors"][0]["message"],
        "A valid JWT token is required"
    );
    assert_eq!(json["errors"][0]["extensions"]["code"], 401);

    Ok(())
}

/***
 * Mutation: `getOrCreateCurrentUser`
 */
static GET_OR_CREATE_CURRENT_USER: &str = "
    mutation GetOrCreateCurrentUser($input: CreateUserInput!) {
        getOrCreateCurrentUser(input: $input) {
            user {
                id
                username
                isActive
                profile {
                    id
                    email
                }
            }
        }
    }
";

/// It retrieves the currently authenticated user
#[tokio::test]
#[ignore]
async fn test_get_or_create_current_user() -> Result<()> {
    let TestUtils {
        http_client,
        oauth,
        graphql,
        users,
        profiles,
        ..
    } = init_test().await?;

    let Credentials {
        access_token: token,
        username,
        email,
    } = oauth.get_credentials(TestUser::Test).await;

    // Create a user and profile
    let user = users.create(username).await?;
    let profile = profiles
        .create(
            &CreateProfileInput {
                email: email.clone(),
                user_id: Some(user.id.clone()),
                display_name: None,
                picture: None,
                content: None,
                city: None,
                state_province: None,
            },
            &false,
        )
        .await?;

    let req = graphql.query(
        GET_OR_CREATE_CURRENT_USER,
        json!({ "input": {
           "profile": {
               "email": email,
           }
        }}),
        Some(token),
    )?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    let json_user = &json["data"]["getOrCreateCurrentUser"]["user"];
    let json_profile = &json_user["profile"];

    assert_eq!(status, 200);
    assert_eq!(json_user["id"], user.id);
    assert_eq!(json_user["username"], user.username);
    assert_eq!(json_profile["email"], email.clone());

    // Clean up
    users.delete(&user.id).await?;
    profiles.delete(&profile.id).await?;

    Ok(())
}

/// It uses the input to create one when no user is found
#[tokio::test]
#[ignore]
async fn test_get_or_create_current_user_create() -> Result<()> {
    let TestUtils {
        http_client,
        oauth,
        graphql,
        users,
        profiles,
        ..
    } = init_test().await?;

    let Credentials {
        access_token: token,
        username,
        email,
    } = oauth.get_credentials(TestUser::Test).await;

    let req = graphql.query(
        GET_OR_CREATE_CURRENT_USER,
        json!({ "input": {
           "profile": {
               "email": email,
           }
        }}),
        Some(token),
    )?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    let user = &json["data"]["getOrCreateCurrentUser"]["user"];
    let profile = &user["profile"];

    assert_eq!(status, 200);
    assert_eq!(user["username"], username.to_string());
    assert_eq!(profile["email"], email.to_string());

    // Clean up
    users
        .delete(user["id"].as_str().expect("No user id found"))
        .await?;
    profiles
        .delete(profile["id"].as_str().expect("No profile id found"))
        .await?;

    Ok(())
}

/// It requires authentication
#[tokio::test]
#[ignore]
async fn test_get_or_create_current_user_requires_authn() -> Result<()> {
    let TestUtils {
        http_client,
        graphql,
        ..
    } = init_test().await?;

    let req = graphql.query(GET_OR_CREATE_CURRENT_USER, json!({ "input": {}}), None)?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(
        json["errors"][0]["message"],
        "A valid JWT token is required"
    );
    assert_eq!(json["errors"][0]["extensions"]["code"], 401);

    Ok(())
}

/***
 * Query: `updateCurrentUser`
 */
static UPDATE_CURRENT_USER: &str = "
    mutation UpdateCurrentUser($input: UpdateUserInput!) {
        updateCurrentUser(input: $input) {
            user {
                id
                username
                isActive
                profile {
                    id
                    email
                }
            }
        }
    }
";

/// It updates the currently authenticated user
#[tokio::test]
#[ignore]
async fn test_update_current_user() -> Result<()> {
    let TestUtils {
        http_client,
        oauth,
        graphql,
        users,
        profiles,
        ..
    } = init_test().await?;

    let Credentials {
        access_token: token,
        username,
        email,
    } = oauth.get_credentials(TestUser::Test).await;

    // Create a user with this username
    let user = users.create(username).await?;
    let profile = profiles
        .create(
            &CreateProfileInput {
                email: email.clone(),
                user_id: Some(user.id.clone()),
                display_name: None,
                picture: None,
                content: None,
                city: None,
                state_province: None,
            },
            &false,
        )
        .await?;

    let req = graphql.query(
        UPDATE_CURRENT_USER,
        json!({ "input": {
           "isActive": false
        }}),
        Some(token),
    )?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    let json_user = &json["data"]["updateCurrentUser"]["user"];
    let json_profile = &json_user["profile"];

    assert_eq!(status, 200);
    assert_eq!(json_user["username"], username.to_string());
    assert_eq!(json_user["isActive"], false);
    assert_eq!(json_profile["email"], email.clone());

    // Clean up
    users.delete(&user.id).await?;
    profiles.delete(&profile.id).await?;

    Ok(())
}

/// It requires authentication
#[tokio::test]
#[ignore]
async fn test_update_current_user_requires_authentication() -> Result<()> {
    let TestUtils {
        http_client,
        graphql,
        ..
    } = init_test().await?;

    let req = graphql.query(
        UPDATE_CURRENT_USER,
        json!({ "input": {
           "isActive": false
        }}),
        None,
    )?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(
        json["errors"][0]["message"],
        "A valid JWT token is required"
    );
    assert_eq!(json["errors"][0]["extensions"]["code"], 401);

    Ok(())
}

/// It requires a valid user record
#[tokio::test]
#[ignore]
async fn test_update_current_user_requires_user() -> Result<()> {
    let TestUtils {
        http_client,
        oauth,
        graphql,
        ..
    } = init_test().await?;

    let Credentials {
        access_token: token,
        ..
    } = oauth.get_credentials(TestUser::Test).await;

    let req = graphql.query(
        UPDATE_CURRENT_USER,
        json!({ "input": {
           "isActive": false
        }}),
        Some(token),
    )?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(
        json["errors"][0]["message"],
        "No existing User found for the current JWT token"
    );
    assert_eq!(json["errors"][0]["extensions"]["code"], 400);

    Ok(())
}
