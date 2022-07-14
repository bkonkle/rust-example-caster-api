use anyhow::Result;
use caster_shows::show_mutations::CreateShowInput;
use futures::executor::block_on;
use hyper::body::to_bytes;
use pretty_assertions::assert_eq;
use serde_json::{json, Value};
use std::panic;

use caster_testing::oauth2::{Credentials, User as TestUser};
use caster_users::role_grant_model::CreateRoleGrantInput;

#[cfg(test)]
mod test_utils;

use test_utils::TestUtils;

fn create_show_input(title: &str) -> CreateShowInput {
    CreateShowInput {
        title: title.to_string(),
        ..Default::default()
    }
}

/***
 * Mutation: `createEpisode`
 */

const CREATE_EPISODE: &str = "
    mutation CreateEpisode($input: CreateEpisodeInput!) {
        createEpisode(input: $input) {
            episode {
                id
                title
                summary
                picture
                content
                show {
                    id
                }
            }
        }
    }
";

/// It creates a new episode
#[tokio::test]
#[ignore]
async fn test_create_episode() -> Result<()> {
    let utils = TestUtils::init().await?;
    let ctx = utils.ctx.clone();

    let Credentials {
        access_token: token,
        username,
        ..
    } = utils.oauth.get_credentials(TestUser::Test).await;

    // Create a user and a show
    let user = ctx.users.create(username).await?;
    let show = ctx.shows.create(&create_show_input("Test Show")).await?;

    // Grant the manager role to this user for this episode's show
    ctx.role_grants
        .create(&CreateRoleGrantInput {
            role_key: "manager".to_string(),
            user_id: user.id.clone(),
            resource_table: "shows".to_string(),
            resource_id: show.id.clone(),
        })
        .await?;

    let req = utils.graphql.query(
        CREATE_EPISODE,
        json!({
            "input": {
                "title": "Test Episode 1",
                "showId": show.id.clone(),
            }
        }),
        Some(token),
    )?;

    let resp = utils.http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    let json_episode = &json["data"]["createEpisode"]["episode"];
    let json_show = &json_episode["show"];

    assert_eq!(status, 200);
    assert_eq!(json_episode["title"], "Test Episode 1");
    assert_eq!(json_show["id"], show.id.clone());

    // Clean up
    ctx.users.delete(&user.id).await?;
    ctx.episodes
        .delete(json_episode["id"].as_str().unwrap())
        .await?;
    ctx.shows.delete(&show.id).await?;

    Ok(())
}

/// It requires a title and a showId
#[tokio::test]
#[ignore]
async fn test_create_episode_requires_title_show_id() -> Result<()> {
    let utils = TestUtils::init().await?;

    let Credentials {
        access_token: token,
        ..
    } = utils.oauth.get_credentials(TestUser::Test).await;

    let req = utils
        .graphql
        .query(CREATE_EPISODE, json!({ "input": {}}), Some(token))?;

    let resp = utils.http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(
        json["errors"][0]["message"],
        r#"Invalid value for argument "input", field "title" of type "CreateEpisodeInput" is required but not provided"#
    );

    // Now provide the "email" and try again
    let req = utils.graphql.query(
        CREATE_EPISODE,
        json!({
            "input": {
                "title": "Test Episode 1",
            }
        }),
        Some(token),
    )?;

    let resp = utils.http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(
        json["errors"][0]["message"],
        r#"Invalid value for argument "input", field "showId" of type "CreateEpisodeInput" is required but not provided"#
    );

    Ok(())
}

/// It requires authentication
#[tokio::test]
#[ignore]
async fn test_create_episode_authn() -> Result<()> {
    let TestUtils {
        http_client,
        graphql,
        ctx,
        ..
    } = TestUtils::init().await?;

    let show = ctx.shows.create(&create_show_input("Test Show")).await?;

    let req = graphql.query(
        CREATE_EPISODE,
        json!({
            "input": {
                "title": "dummy-title",
                "showId": show.id
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
async fn test_create_episode_authz() -> Result<()> {
    let utils = TestUtils::init().await?;
    let ctx = utils.ctx.clone();

    let Credentials {
        access_token: token,
        username,
        ..
    } = utils.oauth.get_credentials(TestUser::Test).await;

    let show = ctx.shows.create(&create_show_input("Test Show")).await?;

    // Create a user with this username
    let user = ctx.users.create(username).await?;

    let req = utils.graphql.query(
        CREATE_EPISODE,
        json!({
            "input": {
                "title": "Test Episode 1",
                "showId": show.id,
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
    ctx.shows.delete(&show.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/***
 * Query: `getEpisode`
 */

const GET_EPISODE: &str = "
    query GetEpisode($id: ID!) {
        getEpisode(id: $id) {
            id
            title
            summary
            picture
            content
            show {
                id
            }
        }
    }
";

/// It retrieves an existing episode
#[tokio::test]
#[ignore]
async fn test_get_episode() -> Result<()> {
    let utils = TestUtils::init().await?;
    let ctx = utils.ctx.clone();

    let Credentials {
        access_token: token,
        ..
    } = utils.oauth.get_credentials(TestUser::Test).await;

    let (show, episode) = utils
        .create_show_and_episode("Test Show", "Test Episode 1")
        .await?;

    let req = utils
        .graphql
        .query(GET_EPISODE, json!({ "id": episode.id,}), Some(token))?;

    let resp = utils.http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;

    let result = panic::catch_unwind(|| {
        block_on(async {
            let json: Value = serde_json::from_slice(&body)?;

            let json_episode = &json["data"]["getEpisode"];
            let json_show = &json_episode["show"];

            assert_eq!(status, 200);
            assert_eq!(json_episode["id"], episode.id);
            assert_eq!(json_episode["title"], "Test Episode 1");
            assert_eq!(json_show["id"], show.id);

            Ok(()) as Result<()>
        })
    });

    // Clean up
    ctx.episodes.delete(&episode.id).await?;
    ctx.shows.delete(&show.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/// It returns nothing when no episode is found
#[tokio::test]
#[ignore]
async fn test_get_episode_empty() -> Result<()> {
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

    let req = graphql.query(GET_EPISODE, json!({ "id": "dummy-id",}), Some(token))?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(json["data"]["getEpisode"], Value::Null);

    Ok(())
}

/***
 * Query: `getManyEpisodes`
 */

const GET_MANY_EPISODES: &str = "
    query GetManyEpisodes(
        $where: EpisodeCondition
        $orderBy: [EpisodesOrderBy!]
        $pageSize: Int
        $page: Int
    ) {
        getManyEpisodes(
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
                show {
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

/// It queries existing episodes
#[tokio::test]
#[ignore]
async fn test_get_many_episodes() -> Result<()> {
    let utils = TestUtils::init().await?;
    let ctx = utils.ctx.clone();

    let Credentials {
        access_token: token,
        ..
    } = utils.oauth.get_credentials(TestUser::Test).await;

    let (show, episode) = utils
        .create_show_and_episode("Test Show", "Test Episode 1")
        .await?;

    let (other_show, other_episode) = utils
        .create_show_and_episode("Test Show 2", "Test Episode 1")
        .await?;

    let req = utils
        .graphql
        .query(GET_MANY_EPISODES, Value::Null, Some(token))?;
    let resp = utils.http_client.request(req).await?;

    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;

    let result = panic::catch_unwind(|| {
        block_on(async {
            let json: Value = serde_json::from_slice(&body)?;

            let json_episode = &json["data"]["getManyEpisodes"]["data"][0];
            let json_show = &json_episode["show"];

            let json_other_episode = &json["data"]["getManyEpisodes"]["data"][1];
            let json_other_show = &json_other_episode["show"];

            assert_eq!(status, 200);

            assert_eq!(json["data"]["getManyEpisodes"]["count"], 2);
            assert_eq!(json["data"]["getManyEpisodes"]["total"], 2);
            assert_eq!(json["data"]["getManyEpisodes"]["page"], 1);
            assert_eq!(json["data"]["getManyEpisodes"]["pageCount"], 1);

            assert_eq!(json_episode["id"], episode.id);
            assert_eq!(json_episode["title"], "Test Episode 1");
            assert_eq!(json_show["id"], show.id);

            assert_eq!(json_other_episode["id"], other_episode.id);
            assert_eq!(json_other_episode["title"], "Test Episode 1");
            assert_eq!(json_other_show["id"], other_show.id);

            Ok(()) as Result<()>
        })
    });

    // Clean up
    ctx.episodes.delete(&episode.id).await?;
    ctx.shows.delete(&show.id).await?;
    ctx.episodes.delete(&other_episode.id).await?;
    ctx.shows.delete(&other_show.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/***
 * Mutation: `updateEpisode`
 */

const UPDATE_EPISODE: &str = "
    mutation UpdateEpisode($id: ID!, $input: UpdateEpisodeInput!) {
        updateEpisode(id: $id, input: $input) {
            episode {
                id
                title
                summary
                picture
                content
                show {
                    id
                }
            }
        }
    }
";

/// It updates an existing episode
#[tokio::test]
#[ignore]
async fn test_update_episode() -> Result<()> {
    let utils = TestUtils::init().await?;
    let ctx = utils.ctx.clone();

    let Credentials {
        access_token: token,
        username,
        ..
    } = utils.oauth.get_credentials(TestUser::Test).await;

    // Create a user with this username
    let user = ctx.users.create(username).await?;

    let (show, episode) = utils
        .create_show_and_episode("Test Show", "Test Episode 1")
        .await?;

    // Grant the manager role to this user for this episode's show
    ctx.role_grants
        .create(&CreateRoleGrantInput {
            role_key: "manager".to_string(),
            user_id: user.id.clone(),
            resource_table: "shows".to_string(),
            resource_id: show.id.clone(),
        })
        .await?;

    let req = utils.graphql.query(
        UPDATE_EPISODE,
        json!({
            "id": episode.id,
            "input": {
                "summary": "Test Summary"
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

            let json_episode = &json["data"]["updateEpisode"]["episode"];
            let json_show = &json_episode["show"];

            assert_eq!(status, 200);

            assert_eq!(json_episode["id"], episode.id);
            assert_eq!(json_episode["title"], "Test Episode 1");
            assert_eq!(json_episode["summary"], "Test Summary");
            assert_eq!(json_show["id"], show.id);

            Ok(()) as Result<()>
        })
    });

    // Clean up
    ctx.users.delete(&user.id).await?;
    ctx.episodes.delete(&episode.id).await?;
    ctx.shows.delete(&show.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/// It returns an error if no existing episode was found
#[tokio::test]
#[ignore]
async fn test_update_episode_not_found() -> Result<()> {
    let TestUtils {
        http_client,
        graphql,
        ..
    } = TestUtils::init().await?;

    let req = graphql.query(
        UPDATE_EPISODE,
        json!({
            "id": "test-id",
            "input": {
                "summary": "Test Summary"
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
        "Unable to find existing Episode"
    );
    assert_eq!(json["errors"][0]["extensions"]["code"], 404);

    Ok(())
}

/// It requires authentication
#[tokio::test]
#[ignore]
async fn test_update_episode_authn() -> Result<()> {
    let utils = TestUtils::init().await?;
    let ctx = utils.ctx.clone();

    let (show, episode) = utils
        .create_show_and_episode("Test Show", "Test Episode 1")
        .await?;

    let req = utils.graphql.query(
        UPDATE_EPISODE,
        json!({
            "id": episode.id,
            "input": {
                "summary": "Test Summary"
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
    ctx.episodes.delete(&episode.id).await?;
    ctx.shows.delete(&show.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/// It requires authorization
#[tokio::test]
#[ignore]
async fn test_update_episode_authz() -> Result<()> {
    let utils = TestUtils::init().await?;
    let ctx = utils.ctx.clone();

    let Credentials {
        access_token: token,
        username,
        ..
    } = utils.oauth.get_credentials(TestUser::Alt).await;

    // Create a user with this username
    let user = ctx.users.create(username).await?;

    let (show, episode) = utils
        .create_show_and_episode("Test Show 2", "Test Episode 1")
        .await?;

    let req = utils.graphql.query(
        UPDATE_EPISODE,
        json!({
            "id": episode.id,
            "input": {
                "summary": "Test Summary"
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
    ctx.episodes.delete(&episode.id).await?;
    ctx.shows.delete(&show.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/***
 * Mutation: `deleteEpisode`
 */

const DELETE_EPISODE: &str = "
    mutation DeleteEpisode($id: ID!) {
        deleteEpisode(id: $id)
    }
";

/// It deletes an existing user episode
#[tokio::test]
#[ignore]
async fn test_delete_episode() -> Result<()> {
    let utils = TestUtils::init().await?;
    let ctx = utils.ctx.clone();

    let Credentials {
        access_token: token,
        username,
        ..
    } = utils.oauth.get_credentials(TestUser::Test).await;

    // Create a user with this username
    let user = ctx.users.create(username).await?;

    let (show, episode) = utils
        .create_show_and_episode("Test Show", "Test Episode 1")
        .await?;

    // Grant the manager role to this user for this episode's show
    ctx.role_grants
        .create(&CreateRoleGrantInput {
            role_key: "manager".to_string(),
            user_id: user.id.clone(),
            resource_table: "shows".to_string(),
            resource_id: show.id.clone(),
        })
        .await?;

    let req = utils
        .graphql
        .query(DELETE_EPISODE, json!({"id": episode.id}), Some(token))?;
    let resp = utils.http_client.request(req).await?;

    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;

    let result = panic::catch_unwind(|| {
        block_on(async {
            let json: Value = serde_json::from_slice(&body)?;

            assert_eq!(status, 200);
            assert!(json["data"]["deleteEpisode"].as_bool().unwrap());

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

/// It returns an error if no existing episode was found
#[tokio::test]
#[ignore]
async fn test_delete_episode_not_found() -> Result<()> {
    let TestUtils {
        http_client,
        graphql,
        ..
    } = TestUtils::init().await?;

    let req = graphql.query(DELETE_EPISODE, json!({"id": "test-id"}), None)?;

    let resp = http_client.request(req).await?;
    let status = resp.status();

    let body = to_bytes(resp.into_body()).await?;
    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(
        json["errors"][0]["message"],
        "Unable to find existing Episode"
    );
    assert_eq!(json["errors"][0]["extensions"]["code"], 404);

    Ok(())
}

/// It requires authentication
#[tokio::test]
#[ignore]
async fn test_delete_episode_authn() -> Result<()> {
    let utils = TestUtils::init().await?;
    let ctx = utils.ctx.clone();

    let (show, episode) = utils
        .create_show_and_episode("Test Show", "Test Episode 1")
        .await?;

    let req = utils
        .graphql
        .query(DELETE_EPISODE, json!({"id": episode.id}), None)?;
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
    ctx.episodes.delete(&episode.id).await?;
    ctx.shows.delete(&show.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}

/// It requires authorization
#[tokio::test]
#[ignore]
async fn test_delete_episode_authz() -> Result<()> {
    let utils = TestUtils::init().await?;
    let ctx = utils.ctx.clone();

    let Credentials {
        access_token: token,
        username,
        ..
    } = utils.oauth.get_credentials(TestUser::Alt).await;

    // Create a user with this username
    let user = ctx.users.create(username).await?;

    let (show, episode) = utils
        .create_show_and_episode("Test Show 2", "Test Episode 1")
        .await?;

    let req = utils
        .graphql
        .query(DELETE_EPISODE, json!({"id": episode.id}), Some(token))?;

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
    ctx.episodes.delete(&episode.id).await?;
    ctx.shows.delete(&show.id).await?;

    if let Err(err) = result {
        panic::resume_unwind(err);
    }

    Ok(())
}
