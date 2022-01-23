use anyhow::Result;
use hyper::body::to_bytes;
use serde_json::Value;

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

async fn delete_user(users: &PgUsersRepository, id: &str) -> Result<()> {
    let user = users.get(id).await?;

    if let Some(_user) = user {
        // TODO: Handle the Profile here as well
        users.delete(id).await?;
    }

    Ok(())
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

    let req = graphql.query(GET_CURRENT_USER, token)?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(json["data"]["getCurrentUser"]["id"], user.id);
    assert_eq!(json["data"]["getCurrentUser"]["username"], user.username);

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
        ..
    } = init_test().await?;

    let Credentials {
        access_token: token,
        ..
    } = oauth.get_credentials(TestUser::Test).await;

    let req = graphql.query(GET_CURRENT_USER, token)?;

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
async fn test_get_current_user_no_authn() -> Result<()> {
    let TestUtils {
        http_client,
        graphql,
        ..
    } = init_test().await?;

    let req = graphql.anon_query(GET_CURRENT_USER)?;

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

#[allow(dead_code)]
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

// It retrieves the currently authenticated user

// It uses the input to create one when no user is found

// It requires authentication

/***
 * Query: `updateCurrentUser`
 */

#[allow(dead_code)]
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

// It updates the currently authenticated user

// It requires authentication

// It requires a valid user record
