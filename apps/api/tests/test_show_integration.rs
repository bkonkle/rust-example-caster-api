use anyhow::Result;
use fake::{Fake, Faker};
use futures::executor::block_on;
use hyper::body::to_bytes;
use pretty_assertions::assert_eq;
use serde_json::{json, Value};
use std::panic;

use caster_domains::{role_grants::model::CreateRoleGrantInput, shows::mutations::CreateShowInput};
use caster_testing::oauth2::{Credentials, User as TestUser};

#[cfg(test)]
mod test_utils;

use test_utils::TestUtils;

/***
 * Mutation: `createShow`
 */

const CREATE_SHOW: &str = "
    mutation CreateShow($input: CreateShowInput!) {
        createShow(input: $input) {
            show {
                id
                title
                summary
                picture
            }
        }
    }
";

/// It creates a new show
#[tokio::test]
#[ignore]
async fn test_create_show() -> Result<()> {
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
        CREATE_SHOW,
        json!({
            "input": {
                "title": "Test Show"
            }
        }),
        Some(token),
    )?;

    let resp = utils.http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;

    let json: Value = serde_json::from_slice(&body)?;

    let json_show = &json["data"]["createShow"]["show"];

    assert_eq!(status, 200);
    assert_eq!(json_show["title"], "Test Show");

    // Clean up
    ctx.users.delete(&user.id).await?;
    ctx.profiles.delete(&profile.id).await?;
    ctx.shows.delete(json_show["id"].as_str().unwrap()).await?;

    Ok(())
}

/// It requires a title
#[tokio::test]
#[ignore]
async fn test_create_show_requires_title() -> Result<()> {
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

    let req = graphql.query(CREATE_SHOW, json!({ "input": {}}), Some(token))?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(
        json["errors"][0]["message"],
        r#"Invalid value for argument "input", field "title" of type "CreateShowInput" is required but not provided"#
    );

    Ok(())
}

/// It requires authentication
#[tokio::test]
#[ignore]
async fn test_create_show_requires_authn() -> Result<()> {
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
        CREATE_SHOW,
        json!({
            "input": {
                "title": "Test Show"
            }
        }),
        Some(token),
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

/***
 * Query: `getShow`
 */

const GET_SHOW: &str = "
    query GetShow($id: ID!) {
        getShow(id: $id) {
            id
            title
            summary
            picture
        }
    }
";

/// It retrieves an existing show
#[tokio::test]
#[ignore]
async fn test_get_show() -> Result<()> {
    let TestUtils {
        http_client,
        oauth,
        graphql,
        ctx,
    } = TestUtils::init().await?;

    let Credentials {
        access_token: token,
        ..
    } = oauth.get_credentials(TestUser::Test).await;

    let mut show_input: CreateShowInput = Faker.fake();
    show_input.title = "Test Show".to_string();

    let show = ctx.shows.create(&show_input).await?;

    let req = graphql.query(GET_SHOW, json!({ "id": show.id,}), Some(token))?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;

    let result = panic::catch_unwind(|| {
        block_on(async {
            let json: Value = serde_json::from_slice(&body)?;

            let json_show = &json["data"]["getShow"];

            assert_eq!(status, 200);
            assert_eq!(json_show["id"], show.id);
            assert_eq!(json_show["title"], "Test Show");

            Ok(()) as Result<()>
        })
    });

    // Clean up
    ctx.shows.delete(&show.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/// It returns nothing when no show is found
#[tokio::test]
#[ignore]
async fn test_get_show_empty() -> Result<()> {
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

    let req = graphql.query(GET_SHOW, json!({ "id": "dummy-id",}), Some(token))?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(json["data"]["getShow"], Value::Null);

    Ok(())
}

/***
 * Query: `getManyShows`
 */

const GET_MANY_SHOWS: &str = "
    query GetManyShows(
        $where: ShowCondition
        $orderBy: [ShowsOrderBy!]
        $pageSize: Int
        $page: Int
    ) {
        getManyShows(
            where: $where
            orderBy: $orderBy
            pageSize: $pageSize
            page: $page
        ) {
            data {
                id
                title
                summary
                picture
            }
            count
            total
            page
            pageCount
        }
    }
";

/// It queries existing shows
#[tokio::test]
#[ignore]
async fn test_get_many_shows() -> Result<()> {
    let TestUtils {
        http_client,
        graphql,
        ctx,
        ..
    } = TestUtils::init().await?;

    let mut show_input: CreateShowInput = Faker.fake();
    show_input.title = "Test Show".to_string();
    show_input.summary = Some("test-summary".to_string());

    let show = ctx.shows.create(&show_input).await?;

    let mut other_show_input: CreateShowInput = Faker.fake();
    other_show_input.title = "Test Show 2".to_string();
    other_show_input.summary = Some("test-summary-2".to_string());

    let other_show = ctx.shows.create(&other_show_input).await?;

    let req = graphql.query(GET_MANY_SHOWS, Value::Null, None)?;
    let resp = http_client.request(req).await?;

    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;

    let result = panic::catch_unwind(|| {
        block_on(async {
            let json: Value = serde_json::from_slice(&body)?;

            let json_result = &json["data"]["getManyShows"];
            let json_show = &json_result["data"][0];
            let json_other_show = &json_result["data"][1];

            assert_eq!(status, 200);

            assert_eq!(json_result["count"], 2);
            assert_eq!(json_result["total"], 2);
            assert_eq!(json_result["page"], 1);
            assert_eq!(json_result["pageCount"], 1);

            assert_eq!(json_show["id"], show.id);
            assert_eq!(json_show["title"], "Test Show");
            assert_eq!(json_show["summary"], show.summary.unwrap());

            assert_eq!(json_other_show["id"], other_show.id);
            assert_eq!(json_other_show["title"], "Test Show 2");
            assert_eq!(json_other_show["summary"], other_show.summary.unwrap());

            Ok(()) as Result<()>
        })
    });

    // Clean up
    ctx.shows.delete(&show.id).await?;
    ctx.shows.delete(&other_show.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/***
 * Mutation: `updateShow`
 */

const UPDATE_SHOW: &str = "
    mutation UpdateShow($id: ID!, $input: UpdateShowInput!) {
        updateShow(id: $id, input: $input) {
            show {
                id
                title
                summary
                picture
            }
        }
    }
";

/// It updates an existing show
#[tokio::test]
#[ignore]
async fn test_update_show() -> Result<()> {
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

    // Create a User
    let user = ctx.users.create(username).await?;

    let mut show_input: CreateShowInput = Faker.fake();
    show_input.title = "Test Show".to_string();

    let show = ctx.shows.create(&show_input).await?;

    // Grant the admin role to this User for this Show
    ctx.role_grants
        .create(&CreateRoleGrantInput {
            role_key: "admin".to_string(),
            user_id: user.id.clone(),
            resource_table: "shows".to_string(),
            resource_id: show.id.clone(),
        })
        .await?;

    let req = graphql.query(
        UPDATE_SHOW,
        json!({
            "id": show.id,
            "input": {
                "summary": "Something else"
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

            let json_show = &json["data"]["updateShow"]["show"];

            assert_eq!(status, 200);

            assert_eq!(json_show["id"], show.id);
            assert_eq!(json_show["title"], "Test Show");
            assert_eq!(json_show["summary"], "Something else");

            Ok(()) as Result<()>
        })
    });

    // Clean up
    ctx.users.delete(&user.id).await?;
    ctx.shows.delete(&show.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/// It returns an error if no existing show is found
#[tokio::test]
#[ignore]
async fn test_update_show_not_found() -> Result<()> {
    let TestUtils {
        http_client,
        graphql,
        ..
    } = TestUtils::init().await?;

    let req = graphql.query(
        UPDATE_SHOW,
        json!({
            "id": "test-id",
            "input": {
                "summary": "Something else"
            }
        }),
        None,
    )?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(json["errors"][0]["message"], "Unable to find existing Show");
    assert_eq!(json["errors"][0]["extensions"]["code"], 404);

    Ok(())
}

/// It requires authentication
#[tokio::test]
#[ignore]
async fn test_update_show_requires_authn() -> Result<()> {
    let TestUtils {
        http_client,
        graphql,
        ctx,
        ..
    } = TestUtils::init().await?;

    let mut show_input: CreateShowInput = Faker.fake();
    show_input.title = "Test Show".to_string();

    let show = ctx.shows.create(&show_input).await?;

    let req = graphql.query(
        UPDATE_SHOW,
        json!({
            "id": show.id,
            "input": {
                "summary": "Something else"
            }
        }),
        None,
    )?;
    let resp = http_client.request(req).await?;

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
    ctx.shows.delete(&show.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/// It requires authorization
#[tokio::test]
#[ignore]
async fn test_update_show_requires_authz() -> Result<()> {
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

    // Create a User
    let user = ctx.users.create(username).await?;

    let mut show_input: CreateShowInput = Faker.fake();
    show_input.title = "Test Show".to_string();

    let show = ctx.shows.create(&show_input).await?;

    let req = graphql.query(
        UPDATE_SHOW,
        json!({
            "id": show.id,
            "input": {
                "summary": "Something else"
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
    ctx.shows.delete(&show.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/***
 * Mutation: `deleteShow`
 */

const DELETE_SHOW: &str = "
    mutation DeleteShow($id: ID!) {
        deleteShow(id: $id)
    }
";

/// It deletes an existing show
#[tokio::test]
#[ignore]
async fn test_delete_show() -> Result<()> {
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

    // Create a User
    let user = ctx.users.create(username).await?;

    let mut show_input: CreateShowInput = Faker.fake();
    show_input.title = "Test Show".to_string();

    let show = ctx.shows.create(&show_input).await?;

    // Grant the admin role to this User for this Show
    ctx.role_grants
        .create(&CreateRoleGrantInput {
            role_key: "admin".to_string(),
            user_id: user.id.clone(),
            resource_table: "shows".to_string(),
            resource_id: show.id.clone(),
        })
        .await?;

    let req = graphql.query(DELETE_SHOW, json!({"id": show.id}), Some(token))?;
    let resp = http_client.request(req).await?;

    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;

    let result = panic::catch_unwind(|| {
        block_on(async {
            let json: Value = serde_json::from_slice(&body)?;

            assert_eq!(status, 200);
            assert!(json["data"]["deleteShow"].as_bool().unwrap());

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

/// It returns an error if no existing show is found
#[tokio::test]
#[ignore]
async fn test_delete_show_not_found() -> Result<()> {
    let TestUtils {
        http_client,
        graphql,
        ..
    } = TestUtils::init().await?;

    let req = graphql.query(DELETE_SHOW, json!({"id": "test-id"}), None)?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(json["errors"][0]["message"], "Unable to find existing Show");
    assert_eq!(json["errors"][0]["extensions"]["code"], 404);

    Ok(())
}

/// It requires authentication
#[tokio::test]
#[ignore]
async fn test_delete_show_requires_authn() -> Result<()> {
    let TestUtils {
        http_client,
        graphql,
        ctx,
        ..
    } = TestUtils::init().await?;

    let mut show_input: CreateShowInput = Faker.fake();
    show_input.title = "Test Show".to_string();

    let show = ctx.shows.create(&show_input).await?;

    let req = graphql.query(DELETE_SHOW, json!({"id": show.id}), None)?;
    let resp = http_client.request(req).await?;

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
    ctx.shows.delete(&show.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/// It requires authorization
#[tokio::test]
#[ignore]
async fn test_delete_show_requires_authz() -> Result<()> {
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

    // Create a User
    let user = ctx.users.create(username).await?;

    let mut show_input: CreateShowInput = Faker.fake();
    show_input.title = "Test Show".to_string();

    let show = ctx.shows.create(&show_input).await?;

    let req = graphql.query(DELETE_SHOW, json!({"id": show.id}), Some(token))?;
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
    ctx.shows.delete(&show.id).await?;
    ctx.users.delete(&user.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}
