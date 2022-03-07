use anyhow::Result;
use caster_shows::show_mutations::CreateShowInput;
use hyper::body::to_bytes;
use serde_json::{json, Value};

use caster_utils::test::oauth2::{Credentials, User as TestUser};

mod test_utils;
use test_utils::TestUtils;

/***
 * Mutation: `createShow`
 */

static CREATE_SHOW: &str = "
    mutation CreateShow($input: CreateShowInput!) {
        createShow(input: $input) {
            show {
                id
                title
                summary
                picture
                content
            }
        }
    }
";

/// It creates a new show
#[tokio::test]
#[ignore]
async fn test_create_show() -> Result<()> {
    let utils = TestUtils::init().await?;

    let Credentials {
        access_token: token,
        ..
    } = utils.oauth.get_credentials(TestUser::Test).await;

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
    utils
        .shows
        .delete(json_show["id"].as_str().unwrap())
        .await?;

    Ok(())
}

/// It requires a title
#[tokio::test]
#[ignore]
async fn test_create_show_requires_title() -> Result<()> {
    let utils = TestUtils::init().await?;

    let Credentials {
        access_token: token,
        ..
    } = utils.oauth.get_credentials(TestUser::Test).await;

    let req = utils
        .graphql
        .query(CREATE_SHOW, json!({ "input": {}}), Some(token))?;

    let resp = utils.http_client.request(req).await?;
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

//- TODO: It requires authentication

/***
 * Query: `getShow`
 */

static GET_SHOW: &str = "
    query GetShow($id: ID!) {
        getShow(id: $id) {
            id
            title
            summary
            picture
            content
        }
    }
";

/// It retrieves an existing show
#[tokio::test]
#[ignore]
async fn test_get_show() -> Result<()> {
    let utils = TestUtils::init().await?;

    let Credentials {
        access_token: token,
        ..
    } = utils.oauth.get_credentials(TestUser::Test).await;

    let show = utils
        .shows
        .create(&CreateShowInput {
            title: "Test Show".to_string(),
            summary: None,
            picture: None,
            content: None,
        })
        .await?;

    let req = utils
        .graphql
        .query(GET_SHOW, json!({ "id": show.id,}), Some(token))?;

    let resp = utils.http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    let json_show = &json["data"]["getShow"];

    assert_eq!(status, 200);
    assert_eq!(json_show["id"], show.id);
    assert_eq!(json_show["title"], "Test Show");

    // Clean up
    utils.shows.delete(&show.id).await?;

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

static GET_MANY_SHOWS: &str = "
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
                content
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
    let utils = TestUtils::init().await?;

    let show = utils
        .shows
        .create(&CreateShowInput {
            title: "Test Show".to_string(),
            summary: Some("Show with a summary".to_string()),
            picture: None,
            content: None,
        })
        .await?;

    let other_show = utils
        .shows
        .create(&CreateShowInput {
            title: "Test Show 2".to_string(),
            summary: None,
            picture: None,
            content: None,
        })
        .await?;

    let req = utils.graphql.query(GET_MANY_SHOWS, Value::Null, None)?;
    let resp = utils.http_client.request(req).await?;

    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
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
    assert_eq!(json_show["summary"], "Show with a summary");

    assert_eq!(json_other_show["id"], other_show.id);
    assert_eq!(json_other_show["title"], "Test Show 2");
    assert_eq!(json_other_show["summary"], Value::Null);

    // Clean up
    utils.shows.delete(&show.id).await?;
    utils.shows.delete(&other_show.id).await?;

    Ok(())
}

/***
 * Mutation: `updateShow`
 */

static UPDATE_SHOW: &str = "
    mutation UpdateShow($id: ID!, $input: UpdateShowInput!) {
        updateShow(id: $id, input: $input) {
            show {
                id
                title
                summary
                picture
                content
            }
        }
    }
";

/// It updates an existing show
#[tokio::test]
#[ignore]
async fn test_update_show() -> Result<()> {
    let utils = TestUtils::init().await?;

    let Credentials {
        access_token: token,
        ..
    } = utils.oauth.get_credentials(TestUser::Test).await;

    let show = utils
        .shows
        .create(&CreateShowInput {
            title: "Test Show".to_string(),
            summary: Some("Show with a summary".to_string()),
            picture: None,
            content: None,
        })
        .await?;

    let req = utils.graphql.query(
        UPDATE_SHOW,
        json!({
            "id": show.id,
            "input": {
                "summary": "Something else"
            }
        }),
        Some(token),
    )?;
    let resp = utils.http_client.request(req).await?;

    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    let json_show = &json["data"]["updateShow"]["show"];

    assert_eq!(status, 200);

    assert_eq!(json_show["id"], show.id);
    assert_eq!(json_show["title"], "Test Show");
    assert_eq!(json_show["summary"], "Something else");

    // Clean up
    utils.shows.delete(&show.id).await?;

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

//- TODO: It requires authentication

//- TODO: It requires authorization

/***
 * Mutation: `deleteShow`
 */

static DELETE_SHOW: &str = "
    mutation DeleteShow($id: ID!) {
        deleteShow(id: $id)
    }
";

/// It deletes an existing show
#[tokio::test]
#[ignore]
async fn test_delete_show() -> Result<()> {
    let utils = TestUtils::init().await?;

    let Credentials {
        access_token: token,
        ..
    } = utils.oauth.get_credentials(TestUser::Test).await;

    let show = utils
        .shows
        .create(&CreateShowInput {
            title: "Test Show".to_string(),
            summary: None,
            picture: None,
            content: None,
        })
        .await?;

    let req = utils
        .graphql
        .query(DELETE_SHOW, json!({"id": show.id}), Some(token))?;
    let resp = utils.http_client.request(req).await?;

    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert!(json["data"]["deleteShow"].as_bool().unwrap());

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

//- TODO: It requires authentication

//- TODO: It requires authorization
