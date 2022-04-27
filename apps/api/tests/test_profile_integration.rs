use anyhow::Result;
use futures::executor::block_on;
use hyper::body::to_bytes;
use pretty_assertions::assert_eq;
use serde_json::{json, Value};
use std::panic;

use caster_test::oauth2::{Credentials, User as TestUser};

mod test_utils;
use test_utils::TestUtils;

/***
 * Mutation: `createProfile`
 */

const CREATE_PROFILE: &str = "
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
        ctx,
    } = TestUtils::init().await?;

    let Credentials {
        access_token: token,
        username,
        email,
    } = oauth.get_credentials(TestUser::Test).await;

    // Create a user and profile with this username
    let user = ctx.users.create(username).await?;

    let req = graphql.query(
        CREATE_PROFILE,
        json!({
            "input": {
                "email": email,
                "userId": user.id.clone(),
            }
        }),
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
    ctx.users.delete(&user.id).await?;
    ctx.profiles
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
        oauth,
        graphql,
        ..
    } = TestUtils::init().await?;

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
        json!({
            "input": {
                "email": email,
            }
        }),
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
    } = TestUtils::init().await?;

    let req = graphql.query(
        CREATE_PROFILE,
        json!({
            "input": {
                "email": "dummy-email",
                "userId": "dummy-user-id"
            }
        }),
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

/// It requires authorization
#[tokio::test]
#[ignore]
async fn test_create_profile_authz() -> Result<()> {
    let TestUtils {
        http_client,
        oauth,
        graphql,
        ctx,
    } = TestUtils::init().await?;

    let Credentials {
        access_token: token,
        username,
        email,
    } = oauth.get_credentials(TestUser::Test).await;

    // Create a user and profile with this username
    let user = ctx.users.create(username).await?;

    let req = graphql.query(
        CREATE_PROFILE,
        json!({
            "input": {
                "email": email,
                "userId": "dummy-user-id",
            }
        }),
        Some(token),
    )?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;

    let result = panic::catch_unwind(|| {
        block_on(async {
            let json: Value = serde_json::from_slice(&body)?;

            assert_eq!(status, 200);
            assert_eq!(json["errors"][0]["message"], "Forbidden");
            assert_eq!(json["errors"][0]["extensions"]["code"], 403);

            Ok(()) as Result<()>
        })
    });

    // Clean up
    ctx.users.delete(&user.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/***
 * Query: `getProfile`
 */

const GET_PROFILE: &str = "
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
    let utils = TestUtils::init().await?;
    let ctx = utils.ctx.clone();

    let Credentials {
        access_token: token,
        username,
        email,
    } = utils.oauth.get_credentials(TestUser::Test).await;

    // Create a user and profile with this username
    let (user, profile) = utils.create_user_and_profile(username, email).await?;

    let req = utils
        .graphql
        .query(GET_PROFILE, json!({ "id": profile.id,}), Some(token))?;

    let resp = utils.http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;

    let result = panic::catch_unwind(|| {
        block_on(async {
            let json: Value = serde_json::from_slice(&body)?;

            let json_profile = &json["data"]["getProfile"];
            let json_user = &json_profile["user"];

            assert_eq!(status, 200);
            assert_eq!(json_profile["id"], profile.id);
            assert_eq!(json_profile["email"], email.clone());
            assert_eq!(json_user["id"], user.id);

            Ok(()) as Result<()>
        })
    });

    // Clean up
    ctx.users.delete(&user.id).await?;
    ctx.profiles.delete(&profile.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

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
        ctx,
    } = TestUtils::init().await?;

    let Credentials {
        access_token: token,
        username,
        ..
    } = oauth.get_credentials(TestUser::Test).await;

    // Create a user with this username
    let user = ctx.users.create(username).await?;

    let req = graphql.query(GET_PROFILE, json!({ "id": "dummy-id",}), Some(token))?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;

    let result = panic::catch_unwind(|| {
        block_on(async {
            let json: Value = serde_json::from_slice(&body)?;

            assert_eq!(status, 200);
            assert_eq!(json["data"]["getProfile"], Value::Null);

            Ok(()) as Result<()>
        })
    });

    // Clean up
    ctx.users.delete(&user.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/// It censors responses for anonymous users
#[tokio::test]
#[ignore]
async fn test_get_profile_authn() -> Result<()> {
    let utils = TestUtils::init().await?;
    let ctx = utils.ctx.clone();

    let Credentials {
        username, email, ..
    } = utils.oauth.get_credentials(TestUser::Test).await;

    // Create a user and profile with this username
    let (user, profile) = utils.create_user_and_profile(username, email).await?;

    let req = utils
        .graphql
        .query(GET_PROFILE, json!({ "id": profile.id,}), None)?;

    let resp = utils.http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;

    let result = panic::catch_unwind(|| {
        block_on(async {
            let json: Value = serde_json::from_slice(&body)?;

            let json_profile = &json["data"]["getProfile"];

            assert_eq!(status, 200);
            assert_eq!(json_profile["id"], profile.id);
            assert_eq!(json_profile["email"], Value::Null);
            assert_eq!(json_profile["user"], Value::Null);

            Ok(()) as Result<()>
        })
    });

    // Clean up
    ctx.users.delete(&user.id).await?;
    ctx.profiles.delete(&profile.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/// It censors responses for unauthorized users
#[tokio::test]
#[ignore]
async fn test_get_profile_authz() -> Result<()> {
    let utils = TestUtils::init().await?;
    let ctx = utils.ctx.clone();

    let Credentials {
        access_token: token,
        email,
        ..
    } = utils.oauth.get_credentials(TestUser::Test).await;

    // Create a user with a different username
    let (user, profile) = utils
        .create_user_and_profile("dummy-username", email)
        .await?;

    let req = utils
        .graphql
        .query(GET_PROFILE, json!({ "id": profile.id,}), Some(token))?;

    let resp = utils.http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;

    let result = panic::catch_unwind(|| {
        block_on(async {
            let json: Value = serde_json::from_slice(&body)?;

            let json_profile = &json["data"]["getProfile"];

            assert_eq!(status, 200);
            assert_eq!(json_profile["id"], profile.id);
            assert_eq!(json_profile["email"], Value::Null);
            assert_eq!(json_profile["user"], Value::Null);

            Ok(()) as Result<()>
        })
    });

    // Clean up
    ctx.users.delete(&user.id).await?;
    ctx.profiles.delete(&profile.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/***
 * Query: `getManyProfiles`
 */

const GET_MANY_PROFILES: &str = "
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

/// It queries existing profiles and censors responses for unauthorized users
#[tokio::test]
#[ignore]
async fn test_get_many_profiles() -> Result<()> {
    let utils = TestUtils::init().await?;
    let ctx = utils.ctx.clone();

    let Credentials {
        access_token: token,
        username,
        email,
    } = utils.oauth.get_credentials(TestUser::Test).await;

    // Create a user and profile with this username
    let (user, profile) = utils.create_user_and_profile(username, email).await?;

    // Create a user with another username
    let (other_user, other_profile) = utils
        .create_user_and_profile("dummy-username", "other@email.address")
        .await?;

    let req = utils
        .graphql
        .query(GET_MANY_PROFILES, Value::Null, Some(token))?;
    let resp = utils.http_client.request(req).await?;

    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;

    let result = panic::catch_unwind(|| {
        block_on(async {
            let json: Value = serde_json::from_slice(&body)?;

            let json_profile = &json["data"]["getManyProfiles"]["data"][0];
            let json_user = &json_profile["user"];

            let json_other_profile = &json["data"]["getManyProfiles"]["data"][1];

            assert_eq!(status, 200);

            assert_eq!(json["data"]["getManyProfiles"]["count"], 2);
            assert_eq!(json["data"]["getManyProfiles"]["total"], 2);
            assert_eq!(json["data"]["getManyProfiles"]["page"], 1);
            assert_eq!(json["data"]["getManyProfiles"]["pageCount"], 1);

            assert_eq!(json_profile["id"], profile.id);
            assert_eq!(json_profile["email"], email.clone());
            assert_eq!(json_user["id"], user.id);

            assert_eq!(json_other_profile["id"], other_profile.id);
            assert_eq!(json_other_profile["email"], Value::Null); // Because of censoring
            assert_eq!(json_other_profile["user"], Value::Null); // Because of censoring

            Ok(()) as Result<()>
        })
    });

    // Clean up
    ctx.users.delete(&user.id).await?;
    ctx.profiles.delete(&profile.id).await?;
    ctx.users.delete(&other_user.id).await?;
    ctx.profiles.delete(&other_profile.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/// It censors responses for anonymous users
#[tokio::test]
#[ignore]
async fn test_get_many_profiles_anon() -> Result<()> {
    let utils = TestUtils::init().await?;
    let ctx = utils.ctx.clone();

    let Credentials {
        username, email, ..
    } = utils.oauth.get_credentials(TestUser::Test).await;

    // Create a user and profile with this username
    let (user, profile) = utils.create_user_and_profile(username, email).await?;

    let req = utils.graphql.query(GET_MANY_PROFILES, Value::Null, None)?;
    let resp = utils.http_client.request(req).await?;

    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;

    let result = panic::catch_unwind(|| {
        block_on(async {
            let json: Value = serde_json::from_slice(&body)?;

            let json_profile = &json["data"]["getManyProfiles"]["data"][0];

            assert_eq!(status, 200);

            assert_eq!(json_profile["id"], profile.id);
            assert_eq!(json_profile["email"], Value::Null);
            assert_eq!(json_profile["user"], Value::Null);

            Ok(()) as Result<()>
        })
    });

    // Clean up
    ctx.users.delete(&user.id).await?;
    ctx.profiles.delete(&profile.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/***
 * Mutation: `updateProfile`
 */

const UPDATE_PROFILE: &str = "
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

/// It updates an existing user profile
#[tokio::test]
#[ignore]
async fn test_update_profile() -> Result<()> {
    let utils = TestUtils::init().await?;
    let ctx = utils.ctx.clone();

    let Credentials {
        access_token: token,
        username,
        email,
    } = utils.oauth.get_credentials(TestUser::Test).await;

    // Create a user and profile with this username
    let (user, profile) = utils.create_user_and_profile(username, email).await?;

    let req = utils.graphql.query(
        UPDATE_PROFILE,
        json!({
            "id": profile.id,
            "input": {
                "displayName": "Test Name"
            }
        }),
        Some(token),
    )?;
    let resp = utils.http_client.request(req).await?;

    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;

    let result = panic::catch_unwind(|| {
        block_on(async {
            let json: Value = serde_json::from_slice(&body)?;

            let json_profile = &json["data"]["updateProfile"]["profile"];
            let json_user = &json_profile["user"];

            assert_eq!(status, 200);

            assert_eq!(json_profile["id"], profile.id);
            assert_eq!(json_profile["email"], email.clone());
            assert_eq!(json_profile["displayName"], "Test Name");
            assert_eq!(json_user["id"], user.id);

            Ok(()) as Result<()>
        })
    });

    // Clean up
    ctx.users.delete(&user.id).await?;
    ctx.profiles.delete(&profile.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/// It requires authentication
#[tokio::test]
#[ignore]
async fn test_update_profile_authn() -> Result<()> {
    let utils = TestUtils::init().await?;
    let ctx = utils.ctx.clone();

    let Credentials {
        username, email, ..
    } = utils.oauth.get_credentials(TestUser::Test).await;

    // Create a user and profile with this username
    let (user, profile) = utils.create_user_and_profile(username, email).await?;

    let req = utils.graphql.query(
        UPDATE_PROFILE,
        json!({
            "id": profile.id,
            "input": {
                "displayName": "Test Name"
            }
        }),
        None,
    )?;

    let resp = utils.http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;

    let result = panic::catch_unwind(|| {
        block_on(async {
            let json: Value = serde_json::from_slice(&body)?;

            assert_eq!(status, 200);
            assert_eq!(json["errors"][0]["message"], "Unauthorized");
            assert_eq!(json["errors"][0]["extensions"]["code"], 401);

            Ok(()) as Result<()>
        })
    });

    // Clean up
    ctx.users.delete(&user.id).await?;
    ctx.profiles.delete(&profile.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/// It returns an error if no existing profile was found
#[tokio::test]
#[ignore]
async fn test_update_profile_not_found() -> Result<()> {
    let TestUtils {
        http_client,
        graphql,
        ..
    } = TestUtils::init().await?;

    let req = graphql.query(
        UPDATE_PROFILE,
        json!({
            "id": "test-id",
            "input": {
                "displayName": "Test Name"
            }
        }),
        None,
    )?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(
        json["errors"][0]["message"],
        "Unable to find existing Profile"
    );
    assert_eq!(json["errors"][0]["extensions"]["code"], 404);

    Ok(())
}

/// It requires authorization
#[tokio::test]
#[ignore]
async fn test_update_profile_authz() -> Result<()> {
    let utils = TestUtils::init().await?;
    let ctx = utils.ctx.clone();

    let Credentials {
        access_token: token,
        username,
        email,
    } = utils.oauth.get_credentials(TestUser::Alt).await;

    // Create a dummy user and profile
    let (user, profile) = utils
        .create_user_and_profile("dummy-username", "other@email.address")
        .await?;

    // Create a user and profile for the Alt user
    let (other_user, other_profile) = utils.create_user_and_profile(username, email).await?;

    let req = utils.graphql.query(
        UPDATE_PROFILE,
        json!({
            "id": profile.id,
            "input": {
                "displayName": "Test Name"
            }
        }),
        Some(token),
    )?;

    let resp = utils.http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;

    let result = panic::catch_unwind(|| {
        block_on(async {
            let json: Value = serde_json::from_slice(&body)?;

            assert_eq!(status, 200);
            assert_eq!(json["errors"][0]["message"], "Forbidden");
            assert_eq!(json["errors"][0]["extensions"]["code"], 403);

            Ok(()) as Result<()>
        })
    });

    // Clean up
    ctx.users.delete(&user.id).await?;
    ctx.profiles.delete(&profile.id).await?;
    ctx.users.delete(&other_user.id).await?;
    ctx.profiles.delete(&other_profile.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/***
 * Mutation: `deleteProfile`
 */

const DELETE_PROFILE: &str = "
    mutation DeleteProfile($id: ID!) {
        deleteProfile(id: $id)
    }
";

/// It deletes an existing user profile
#[tokio::test]
#[ignore]
async fn test_delete_profile() -> Result<()> {
    let utils = TestUtils::init().await?;
    let ctx = utils.ctx.clone();

    let Credentials {
        access_token: token,
        username,
        email,
    } = utils.oauth.get_credentials(TestUser::Test).await;

    // Create a user and profile with this username
    let (user, profile) = utils.create_user_and_profile(username, email).await?;

    let req = utils
        .graphql
        .query(DELETE_PROFILE, json!({"id": profile.id}), Some(token))?;
    let resp = utils.http_client.request(req).await?;

    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;

    let result = panic::catch_unwind(|| {
        block_on(async {
            let json: Value = serde_json::from_slice(&body)?;

            assert_eq!(status, 200);
            assert!(json["data"]["deleteProfile"].as_bool().unwrap());

            Ok(()) as Result<()>
        })
    });

    // Clean up
    ctx.users.delete(&user.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/// It requires authentication
#[tokio::test]
#[ignore]
async fn test_delete_profile_authn() -> Result<()> {
    let utils = TestUtils::init().await?;
    let ctx = utils.ctx.clone();

    let Credentials {
        username, email, ..
    } = utils.oauth.get_credentials(TestUser::Test).await;

    // Create a user and profile with this username
    let (user, profile) = utils.create_user_and_profile(username, email).await?;

    let req = utils
        .graphql
        .query(DELETE_PROFILE, json!({"id": profile.id}), None)?;
    let resp = utils.http_client.request(req).await?;

    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;

    let result = panic::catch_unwind(|| {
        block_on(async {
            let json: Value = serde_json::from_slice(&body)?;

            assert_eq!(status, 200);
            assert_eq!(json["errors"][0]["message"], "Unauthorized");
            assert_eq!(json["errors"][0]["extensions"]["code"], 401);

            Ok(()) as Result<()>
        })
    });

    // Clean up
    ctx.users.delete(&user.id).await?;
    ctx.profiles.delete(&profile.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/// It returns an error if no existing profile was found
#[tokio::test]
#[ignore]
async fn test_delete_profile_not_found() -> Result<()> {
    let TestUtils {
        http_client,
        graphql,
        ..
    } = TestUtils::init().await?;

    let req = graphql.query(DELETE_PROFILE, json!({"id": "test-id"}), None)?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(
        json["errors"][0]["message"],
        "Unable to find existing Profile"
    );
    assert_eq!(json["errors"][0]["extensions"]["code"], 404);

    Ok(())
}

/// It requires authorization
#[tokio::test]
#[ignore]
async fn test_delete_profile_authz() -> Result<()> {
    let utils = TestUtils::init().await?;
    let ctx = utils.ctx.clone();

    let Credentials {
        access_token: token,
        username,
        email,
    } = utils.oauth.get_credentials(TestUser::Alt).await;

    // Create a dummy user and profile
    let (user, profile) = utils
        .create_user_and_profile("dummy-username", "other@email.address")
        .await?;

    // Create a user and profile for the Alt user
    let (other_user, other_profile) = utils.create_user_and_profile(username, email).await?;

    let req = utils
        .graphql
        .query(DELETE_PROFILE, json!({"id": profile.id}), Some(token))?;

    let resp = utils.http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;

    let result = panic::catch_unwind(|| {
        block_on(async {
            let json: Value = serde_json::from_slice(&body)?;

            assert_eq!(status, 200);
            assert_eq!(json["errors"][0]["message"], "Forbidden");
            assert_eq!(json["errors"][0]["extensions"]["code"], 403);

            Ok(()) as Result<()>
        })
    });

    // Clean up
    ctx.users.delete(&user.id).await?;
    ctx.profiles.delete(&profile.id).await?;
    ctx.users.delete(&other_user.id).await?;
    ctx.profiles.delete(&other_profile.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}
