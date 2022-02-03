use anyhow::Result;
use hyper::body::to_bytes;
use serde_json::{json, Value};

use caster_users::{
    user_model::User,
    users_repository::{PgUsersRepository, UsersRepository},
};
use caster_utils::test::oauth2::{Credentials, User as TestUser};

mod test_utils;
use test_utils::{init_test, TestUtils};

async fn create_user(users: &PgUsersRepository, username: &str) -> Result<User> {
    let user = users.get_by_username(username).await?;

    if let Some(user) = user {
        return Ok(user);
    }

    users.create(username).await
}

async fn delete_user_with_profile(users: &PgUsersRepository, user: Option<User>) -> Result<()> {
    if let Some(user) = user {
        // TODO: Handle the Profile here as well
        users.delete(&user.id).await?;
    }

    Ok(())
}

async fn delete_user(users: &PgUsersRepository, id: &str) -> Result<()> {
    delete_user_with_profile(users, users.get(id).await?).await
}

async fn delete_user_by_username(users: &PgUsersRepository, username: &str) -> Result<()> {
    delete_user_with_profile(users, users.get_by_username(username).await?).await
}

/***
 * Query: `getCurrentUser`
 */

static GET_CURRENT_USER: &str = "
    query GetCurrentUser {
        getCurrentUser {
            id
            username
            isActive
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
        ..
    } = init_test().await?;

    let Credentials {
        access_token: token,
        username,
        ..
    } = oauth.get_credentials(TestUser::Test).await;

    // Create a user with this username if one doesn't already exist
    let user = create_user(&users, username).await?;

    let req = graphql.query(GET_CURRENT_USER, Value::Null, Some(token))?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(json["data"]["getCurrentUser"]["id"], user.id);
    assert_eq!(json["data"]["getCurrentUser"]["username"], user.username);
    assert_eq!(json["data"]["getCurrentUser"]["isActive"], true);

    // Clean up the user
    delete_user(&users, &user.id).await?;

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
        users,
        ..
    } = init_test().await?;

    let Credentials {
        access_token: token,
        username,
        ..
    } = oauth.get_credentials(TestUser::Test).await;

    // Make sure there's no leftover user
    delete_user_by_username(&users, username).await?;

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
        ..
    } = init_test().await?;

    let Credentials {
        access_token: token,
        username,
        email,
    } = oauth.get_credentials(TestUser::Test).await;

    // Create a user with this username if one doesn't already exist
    let user = create_user(&users, username).await?;

    let req = graphql.query(
        GET_OR_CREATE_CURRENT_USER,
        json!({ "input": {
           // TODO: Add inline profile
        }}),
        Some(token),
    )?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(
        json["data"]["getOrCreateCurrentUser"]["user"]["id"],
        user.id
    );
    assert_eq!(
        json["data"]["getOrCreateCurrentUser"]["user"]["username"],
        user.username
    );
    assert_eq!(
        json["data"]["getOrCreateCurrentUser"]["user"]["profile"]["email"],
        email.clone()
    );

    // Clean up the user
    delete_user(&users, &user.id).await?;

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
        ..
    } = init_test().await?;

    let Credentials {
        access_token: token,
        username,
        ..
    } = oauth.get_credentials(TestUser::Test).await;

    // Make sure there's no leftover user
    delete_user_by_username(&users, username).await?;

    let req = graphql.query(
        GET_OR_CREATE_CURRENT_USER,
        json!({ "input": {
           // TODO: Add inline profile
        }}),
        Some(token),
    )?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(
        json["data"]["getOrCreateCurrentUser"]["user"]["username"],
        username.to_string()
    );

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

    let req = graphql.query(
        GET_OR_CREATE_CURRENT_USER,
        json!({ "input": {
           // TODO: Add inline profile
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
        ..
    } = init_test().await?;

    let Credentials {
        access_token: token,
        username,
        ..
    } = oauth.get_credentials(TestUser::Test).await;

    // Create a user with this username if one doesn't already exist
    let user = create_user(&users, username).await?;

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
        json["data"]["updateCurrentUser"]["user"]["username"],
        username.to_string()
    );
    assert_eq!(json["data"]["updateCurrentUser"]["user"]["isActive"], false);

    // Clean up the user
    delete_user(&users, &user.id).await?;

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
        users,
        ..
    } = init_test().await?;

    let Credentials {
        access_token: token,
        username,
        ..
    } = oauth.get_credentials(TestUser::Test).await;

    // Make sure there's no leftover user
    delete_user_by_username(&users, username).await?;

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
