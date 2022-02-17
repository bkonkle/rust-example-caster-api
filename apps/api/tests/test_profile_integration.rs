use anyhow::Result;
use caster_users::profile_mutations::CreateProfileInput;
use hyper::body::to_bytes;
use log::info;
use serde_json::{json, Value};

use caster_utils::test::oauth2::{Credentials, User as TestUser};

mod test_utils;
use test_utils::{init_test, TestUtils};

/***
 * Mutation: `createProfile`
 */

static CREATE_PROFILE: &str = "
    mutation CreateProfile($input: CreateProfileInput!) {
        createProfile(input: $input) {
            profile {
                id
                email
                displayName
                picture
                user {
                    id
                }
            }
        }
    }
";

/// It creates a new user profile
#[tokio::test]
#[ignore]
async fn test_create_profile() -> Result<()> {
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

    let req = graphql.query(
        CREATE_PROFILE,
        json!({ "input": {
           "email": email,
           "userId": user.id.clone(),
        }}),
        Some(token),
    )?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    let json_profile = &json["data"]["createProfile"]["profile"];
    let json_user = &json_profile["user"];

    assert_eq!(status, 200);
    assert_eq!(json_profile["email"], email.clone());
    assert_eq!(json_user["id"], user.id.clone());

    // Clean up
    users.delete(&user.id).await?;
    profiles
        .delete(json_profile["id"].as_str().unwrap())
        .await?;

    Ok(())
}

/// It requires an email address and a userId
#[tokio::test]
#[ignore]
async fn test_create_profile_requires_email_user_id() -> Result<()> {
    let TestUtils {
        http_client,
        graphql,
        oauth,
        ..
    } = init_test().await?;

    let Credentials {
        access_token: token,
        email,
        ..
    } = oauth.get_credentials(TestUser::Test).await;

    let req = graphql.query(CREATE_PROFILE, json!({ "input": {}}), Some(token))?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(
        json["errors"][0]["message"],
        r#"Invalid value for argument "input", field "email" of type "CreateProfileInput" is required but not provided"#
    );

    // Now provide the "email" and try again
    let req = graphql.query(
        CREATE_PROFILE,
        json!({ "input": {
            "email": email,
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
        r#"Invalid value for argument "input", field "userId" of type "CreateProfileInput" is required but not provided"#
    );

    Ok(())
}

/// It requires authentication
#[tokio::test]
#[ignore]
async fn test_create_profile_authn() -> Result<()> {
    let TestUtils {
        http_client,
        graphql,
        ..
    } = init_test().await?;

    let req = graphql.query(
        CREATE_PROFILE,
        json!({ "input": {
            "email": "dummy-email",
            "userId": "dummy-user-id"
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

/// It requires authorization
#[tokio::test]
#[ignore]
async fn test_create_profile_authz() -> Result<()> {
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

    // Create a user with this username
    let user = users.create(username).await?;

    let req = graphql.query(
        CREATE_PROFILE,
        json!({ "input": {
           "email": email,
           "userId": "dummy-user-id",
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
        "The userId must match the currently logged-in User"
    );
    assert_eq!(json["errors"][0]["extensions"]["code"], 403);

    // Clean up
    users.delete(&user.id).await?;

    Ok(())
}

/// Query: getProfile

static GET_PROFILE: &str = "
    query GetProfile($id: ID!) {
        getProfile(id: $id) {
            id
            email
            displayName
            picture
            user {
                id
            }
        }
    }
";

/// It retrieves an existing user profile
#[tokio::test]
#[ignore]
async fn test_get_profile() -> Result<()> {
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
                user_id: user.id.clone(),
                display_name: None,
                picture: None,
                content: None,
                city: None,
                state_province: None,
            },
            &false,
        )
        .await?;

    let req = graphql.query(GET_PROFILE, json!({ "id": profile.id,}), Some(token))?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    let json_profile = &json["data"]["getProfile"];
    let json_user = &json_profile["user"];

    assert_eq!(status, 200);
    assert_eq!(json_profile["id"], profile.id);
    assert_eq!(json_profile["email"], email.clone());
    assert_eq!(json_user["id"], user.id);

    // Clean up
    users.delete(&user.id).await?;
    profiles.delete(&profile.id).await?;

    Ok(())
}

/// It returns nothing when no profile is found
#[tokio::test]
#[ignore]
async fn test_get_profile_empty() -> Result<()> {
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

    // Create a user with this username
    let user = users.create(username).await?;

    let req = graphql.query(GET_PROFILE, json!({ "id": "dummy-id",}), Some(token))?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(json["data"]["getProfile"], Value::Null);

    // Clean up
    users.delete(&user.id).await?;

    Ok(())
}

/// It censors responses for anonymous users
#[tokio::test]
#[ignore]
async fn test_get_profile_authn() -> Result<()> {
    let TestUtils {
        http_client,
        oauth,
        graphql,
        users,
        profiles,
        ..
    } = init_test().await?;

    let Credentials {
        username, email, ..
    } = oauth.get_credentials(TestUser::Test).await;

    // Create a user with this username
    let user = users.create(username).await?;
    let profile = profiles
        .create(
            &CreateProfileInput {
                email: email.clone(),
                user_id: user.id.clone(),
                display_name: None,
                picture: None,
                content: None,
                city: None,
                state_province: None,
            },
            &false,
        )
        .await?;

    let req = graphql.query(GET_PROFILE, json!({ "id": profile.id,}), None)?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    let json_profile = &json["data"]["getProfile"];
    let json_user = &json_profile["user"];

    assert_eq!(status, 200);
    assert_eq!(json_profile["id"], profile.id);
    assert_eq!(json_profile["email"], Value::Null);
    assert_eq!(json_user["id"], user.id);

    // Clean up
    users.delete(&user.id).await?;
    profiles.delete(&profile.id).await?;

    Ok(())
}

/// It censors responses for unauthorized users
#[tokio::test]
#[ignore]
async fn test_get_profile_authz() -> Result<()> {
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
        email,
        ..
    } = oauth.get_credentials(TestUser::Test).await;

    // Create a user with a different username
    let user = users.create("dummy-username").await?;
    let profile = profiles
        .create(
            &CreateProfileInput {
                email: email.clone(),
                user_id: user.id.clone(),
                display_name: None,
                picture: None,
                content: None,
                city: None,
                state_province: None,
            },
            &false,
        )
        .await?;

    let req = graphql.query(GET_PROFILE, json!({ "id": profile.id,}), Some(token))?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    let json_profile = &json["data"]["getProfile"];
    let json_user = &json_profile["user"];

    assert_eq!(status, 200);
    assert_eq!(json_profile["id"], profile.id);
    assert_eq!(json_profile["email"], Value::Null);
    assert_eq!(json_user["id"], user.id);

    // Clean up
    users.delete(&user.id).await?;
    profiles.delete(&profile.id).await?;

    Ok(())
}

//- Query: getManyProfiles

static GET_MANY_PROFILES: &str = "
    query GetManyProfiles(
        $where: ProfileCondition
        $orderBy: [ProfilesOrderBy!]
        $pageSize: Int
        $page: Int
    ) {
        getManyProfiles(
            where: $where
            orderBy: $orderBy
            pageSize: $pageSize
            page: $page
        ) {
            data {
                id
                email
                displayName
                picture
                user {
                    id
                }
            }
            count
            total
            page
            pageCount
        }
    }
";

//- It queries existing profiles

//- It censors responses for anonymous users

//- It censors responses for unauthorized users

//- Mutation: updateProfile

static UPDATE_PROFILE: &str = "
    mutation UpdateProfile($id: ID!, $input: UpdateProfileInput!) {
        updateProfile(id: $id, input: $input) {
            profile {
                id
                email
                displayName
                picture
                user {
                    id
                }
            }
        }
    }
";

//- It updates an existing user profile

//- It requires authentication

//- It returns an error if no existing profile was found

//- It requires authorization

//- Mutation: deleteProfile

static DELETE_PROFILE: &str = "
    mutation DeleteProfile($id: ID!) {
        deleteProfile(id: $id)
    }
";

//- It deletes an existing user profile

//- It requires authentication

//- It returns an error if no existing profile was found

//- It requires authorization
