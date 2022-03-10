use anyhow::Result;
use caster_users::role_grant_model::CreateRoleGrantInput;
use hyper::body::to_bytes;
use serde_json::{json, Value};

use caster_utils::test::oauth2::{Credentials, User as TestUser};

mod test_utils;
use test_utils::TestUtils;

/***
 * Query: `getCurrentUser`
 */

const GET_CURRENT_USER: &str = "
    query GetCurrentUser {
        getCurrentUser {
            id
            username
            isActive
            roles {
                roleKey
                resourceTable
                resourceId
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
        role_grants,
        ..
    } = TestUtils::init().await?;

    let Credentials {
        access_token: token,
        username,
        ..
    } = oauth.get_credentials(TestUser::Test).await;

    // Create a user with this username
    let user = users.create(username).await?;

    // Create a sample RoleGrant to test the relation
    let role_grant = role_grants
        .create(&CreateRoleGrantInput {
            user_id: user.id.clone(),
            role_key: "test".to_string(),
            resource_table: "users".to_string(),
            resource_id: user.id.clone(),
        })
        .await?;

    let req = graphql.query(GET_CURRENT_USER, Value::Null, Some(token))?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    let json_user = &json["data"]["getCurrentUser"];
    let json_roles = &json_user["roles"];

    assert_eq!(status, 200);
    assert_eq!(json_user["id"], user.id);
    assert_eq!(json_user["username"], user.username);
    assert!(json_user["isActive"].as_bool().unwrap());
    assert_eq!(json_roles[0]["roleKey"], role_grant.role_key);

    // Clean up
    users.delete(&user.id).await?;

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
    } = TestUtils::init().await?;

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
    } = TestUtils::init().await?;

    let req = graphql.query(GET_CURRENT_USER, Value::Null, None)?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(json["errors"][0]["message"], "Unauthorized");
    assert_eq!(json["errors"][0]["extensions"]["code"], 401);

    Ok(())
}

/***
 * Mutation: `getOrCreateCurrentUser`
 */
const GET_OR_CREATE_CURRENT_USER: &str = "
    mutation GetOrCreateCurrentUser($input: CreateUserInput!) {
        getOrCreateCurrentUser(input: $input) {
            user {
                id
                username
                isActive
                roles {
                    roleKey
                    resourceTable
                    resourceId
                }
            }
        }
    }
";

/// It retrieves the currently authenticated user
#[tokio::test]
#[ignore]
async fn test_get_or_create_current_user_existing() -> Result<()> {
    let utils = TestUtils::init().await?;

    let Credentials {
        access_token: token,
        username,
        ..
    } = utils.oauth.get_credentials(TestUser::Test).await;

    // Create a user
    let user = utils.users.create(username).await?;

    // Create a sample RoleGrant to test the relation
    let role_grant = utils
        .role_grants
        .create(&CreateRoleGrantInput {
            user_id: user.id.clone(),
            role_key: "test".to_string(),
            resource_table: "users".to_string(),
            resource_id: user.id.clone(),
        })
        .await?;

    let req = utils.graphql.query(
        GET_OR_CREATE_CURRENT_USER,
        json!({ "input": {}}),
        Some(token),
    )?;

    let resp = utils.http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    let json_user = &json["data"]["getOrCreateCurrentUser"]["user"];
    let json_roles = &json_user["roles"];

    assert_eq!(status, 200);
    assert_eq!(json_user["id"], user.id);
    assert_eq!(json_user["username"], user.username);
    assert_eq!(json_roles[0]["roleKey"], role_grant.role_key);

    // Clean up
    utils.users.delete(&user.id).await?;

    Ok(())
}

/// It uses the input to create one when no user is found
#[tokio::test]
#[ignore]
async fn test_get_or_create_current_user_create() -> Result<()> {
    let utils = TestUtils::init().await?;

    let Credentials {
        access_token: token,
        username,
        email,
    } = utils.oauth.get_credentials(TestUser::Test).await;

    let req = utils.graphql.query(
        GET_OR_CREATE_CURRENT_USER,
        json!({ "input": {
           "profile": {
               "email": email,
           }
        }}),
        Some(token),
    )?;

    let resp = utils.http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    let json_user = &json["data"]["getOrCreateCurrentUser"]["user"];

    assert_eq!(status, 200);
    assert_eq!(json_user["username"], username.to_string());

    let user_id = json_user["id"].as_str().expect("No user id found");

    // Ensure that a related Profile was created inline
    let profile = utils
        .profiles
        .get_by_user_id(user_id, &false)
        .await?
        .expect("No profile id found");

    // Clean up
    utils.users.delete(user_id).await?;
    utils.profiles.delete(&profile.id).await?;

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
    } = TestUtils::init().await?;

    let req = graphql.query(GET_OR_CREATE_CURRENT_USER, json!({ "input": {}}), None)?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(json["errors"][0]["message"], "Unauthorized");
    assert_eq!(json["errors"][0]["extensions"]["code"], 401);

    Ok(())
}

/***
 * Query: `updateCurrentUser`
 */
const UPDATE_CURRENT_USER: &str = "
    mutation UpdateCurrentUser($input: UpdateUserInput!) {
        updateCurrentUser(input: $input) {
            user {
                id
                username
                isActive
                roles {
                    roleKey
                    resourceTable
                    resourceId
                }
            }
        }
    }
";

/// It updates the currently authenticated user
#[tokio::test]
#[ignore]
async fn test_update_current_user() -> Result<()> {
    let utils = TestUtils::init().await?;

    let Credentials {
        access_token: token,
        username,
        ..
    } = utils.oauth.get_credentials(TestUser::Test).await;

    // Create a user with this username
    let user = utils.users.create(username).await?;

    // Create a sample RoleGrant to test the relation
    let role_grant = utils
        .role_grants
        .create(&CreateRoleGrantInput {
            user_id: user.id.clone(),
            role_key: "test".to_string(),
            resource_table: "users".to_string(),
            resource_id: user.id.clone(),
        })
        .await?;

    let req = utils.graphql.query(
        UPDATE_CURRENT_USER,
        json!({ "input": {
           "isActive": false
        }}),
        Some(token),
    )?;

    let resp = utils.http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    let json_user = &json["data"]["updateCurrentUser"]["user"];
    let json_roles = &json_user["roles"];

    assert_eq!(status, 200);
    assert_eq!(json_user["username"], username.to_string());
    assert!(!json_user["isActive"].as_bool().unwrap());
    assert_eq!(json_roles[0]["roleKey"], role_grant.role_key);

    // Clean up
    utils.users.delete(&user.id).await?;

    Ok(())
}

/// It requires authentication
#[tokio::test]
#[ignore]
async fn test_update_current_user_requires_authn() -> Result<()> {
    let TestUtils {
        http_client,
        graphql,
        ..
    } = TestUtils::init().await?;

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
    assert_eq!(json["errors"][0]["message"], "Unauthorized");
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
    } = TestUtils::init().await?;

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
