use anyhow::Result;
use async_graphql::{dataloader::DataLoader, EmptySubscription, Request, Schema, Variables};
use fake::{Fake, Faker};
use mockall::predicate::*;
use pretty_assertions::assert_eq;
use serde_json::json;
use std::sync::Arc;

use crate::{
    episodes::{
        model::Episode,
        resolver::{EpisodesMutation, EpisodesQuery},
        service::{EpisodesService, MockEpisodesService},
    },
    shows::service::{MockShowsService, ShowLoader, ShowsService},
};

fn init(
    service: MockEpisodesService,
) -> Schema<EpisodesQuery, EpisodesMutation, EmptySubscription> {
    let service: Arc<dyn EpisodesService> = Arc::new(service);

    let shows_service: Arc<dyn ShowsService> = Arc::new(MockShowsService::new());
    let show_loader = ShowLoader::new(&shows_service);

    Schema::build(
        EpisodesQuery::default(),
        EpisodesMutation::default(),
        EmptySubscription,
    )
    .data(service)
    .data(DataLoader::new(show_loader, tokio::spawn))
    .finish()
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
            show {
                id
            }
        }
    }
";

#[tokio::test]
async fn test_episodes_resolver_get_simple() -> Result<()> {
    let episode_id = "Test Episode";
    let episode_title = "Test Episode 1";

    let mut episode: Episode = Faker.fake();
    episode.id = episode_id.to_string();
    episode.title = episode_title.to_string();
    episode.show = Some(Faker.fake());

    let mut service = MockEpisodesService::new();
    service
        .expect_get()
        .with(eq(episode_id), eq(&true))
        .times(1)
        .returning(move |_, _| Ok(Some(episode.clone())));

    let schema = init(service);

    let result = schema
        .execute(
            Request::new(GET_EPISODE).variables(Variables::from_json(json!({ "id": episode_id }))),
        )
        .await;

    let data = result.data.into_json()?;
    let json_episode = &data["getEpisode"];

    assert_eq!(json_episode["id"], episode_id);
    assert_eq!(json_episode["title"], episode_title);

    Ok(())
}
