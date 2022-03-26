use anyhow::Result;
use caster_utils::pagination::ManyResponse;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult, Transaction, Value};
use std::sync::Arc;

use caster_shows::{
    episode_model::Episode,
    episode_mutations::{CreateEpisodeInput, UpdateEpisodeInput},
    episode_queries::{EpisodeCondition, EpisodesOrderBy},
    episodes_service::{DefaultEpisodesService, EpisodesService},
};

mod shows_factory;

#[tokio::test]
async fn test_episodes_service_get_model() -> Result<()> {
    let show = shows_factory::create_show("Test Show");
    let episode = Episode {
        show: None,
        ..shows_factory::create_episode_for_show("Test Episode", show)
    };

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![episode.clone()]])
            .into_connection(),
    );

    let service = DefaultEpisodesService::new(db.clone());

    let result = service.get_model(&episode.id, &false).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, Some((episode, None)));

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "episodes"."id", "episodes"."created_at", "episodes"."updated_at", "episodes"."title", "episodes"."summary", "episodes"."picture", "episodes"."content", "episodes"."show_id" FROM "episodes" WHERE "episodes"."id" = $1 LIMIT $2"#,
            vec!["test-episode".into(), 1u64.into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_get_model_with_related() -> Result<()> {
    let show = shows_factory::create_show("Test Show");
    let episode = shows_factory::create_episode_for_show("Test Episode", show.clone());

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![(episode.clone(), show.clone())]])
            .into_connection(),
    );

    let service = DefaultEpisodesService::new(db.clone());

    let result = service.get_model(&episode.id, &true).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(
        result,
        Some((
            Episode {
                show: None,
                ..episode.clone()
            },
            episode.show
        ))
    );

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "episodes"."id" AS "A_id", "episodes"."created_at" AS "A_created_at", "episodes"."updated_at" AS "A_updated_at", "episodes"."title" AS "A_title", "episodes"."summary" AS "A_summary", "episodes"."picture" AS "A_picture", "episodes"."content" AS "A_content", "episodes"."show_id" AS "A_show_id", "shows"."id" AS "B_id", "shows"."created_at" AS "B_created_at", "shows"."updated_at" AS "B_updated_at", "shows"."title" AS "B_title", "shows"."summary" AS "B_summary", "shows"."picture" AS "B_picture", "shows"."content" AS "B_content" FROM "episodes" LEFT JOIN "shows" ON "episodes"."show_id" = "shows"."id" WHERE "episodes"."id" = $1 LIMIT $2"#,
            vec!["test-episode".into(), 1u64.into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_get() -> Result<()> {
    let show = shows_factory::create_show("Test Show");
    let episode = Episode {
        show: None,
        ..shows_factory::create_episode_for_show("Test Episode", show.clone())
    };

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![episode.clone()]])
            .into_connection(),
    );

    let service = DefaultEpisodesService::new(db.clone());

    let result = service.get(&episode.id, &false).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, Some(episode));

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "episodes"."id", "episodes"."created_at", "episodes"."updated_at", "episodes"."title", "episodes"."summary", "episodes"."picture", "episodes"."content", "episodes"."show_id" FROM "episodes" WHERE "episodes"."id" = $1 LIMIT $2"#,
            vec!["test-episode".into(), 1u64.into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_get_with_related() -> Result<()> {
    let show = shows_factory::create_show("Test Show");
    let episode = shows_factory::create_episode_for_show("Test Episode", show.clone());

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![(episode.clone(), show.clone())]])
            .into_connection(),
    );

    let service = DefaultEpisodesService::new(db.clone());

    let result = service.get(&episode.id, &true).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, Some(episode));

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "episodes"."id" AS "A_id", "episodes"."created_at" AS "A_created_at", "episodes"."updated_at" AS "A_updated_at", "episodes"."title" AS "A_title", "episodes"."summary" AS "A_summary", "episodes"."picture" AS "A_picture", "episodes"."content" AS "A_content", "episodes"."show_id" AS "A_show_id", "shows"."id" AS "B_id", "shows"."created_at" AS "B_created_at", "shows"."updated_at" AS "B_updated_at", "shows"."title" AS "B_title", "shows"."summary" AS "B_summary", "shows"."picture" AS "B_picture", "shows"."content" AS "B_content" FROM "episodes" LEFT JOIN "shows" ON "episodes"."show_id" = "shows"."id" WHERE "episodes"."id" = $1 LIMIT $2"#,
            vec!["test-episode".into(), 1u64.into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_get_many() -> Result<()> {
    let show = shows_factory::create_show("Test Show");
    let episode = Episode {
        show: None,
        ..shows_factory::create_episode_for_show("Test Episode", show)
    };

    let other_show = shows_factory::create_show("Test Show 2");
    let other_episode = Episode {
        show: None,
        ..shows_factory::create_episode_for_show("Test Episode 2", other_show)
    };

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![episode.clone(), other_episode.clone()]])
            .into_connection(),
    );

    let service = DefaultEpisodesService::new(db.clone());

    let result = service
        .get_many(
            Some(EpisodeCondition {
                title: Some("Test Episode".to_string()),
                show_id: None,
            }),
            None,
            None,
            None,
            &false,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(
        result,
        ManyResponse {
            data: vec![episode, other_episode],
            count: 2,
            total: 2,
            page: 1,
            page_count: 1,
        }
    );

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "episodes"."id", "episodes"."created_at", "episodes"."updated_at", "episodes"."title", "episodes"."summary", "episodes"."picture", "episodes"."content", "episodes"."show_id" FROM "episodes" WHERE "episodes"."title" = $1"#,
            vec!["Test Episode".into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_get_many_with_related() -> Result<()> {
    let show = shows_factory::create_show("Test Show");
    let episode = shows_factory::create_episode_for_show("Test Episode", show.clone());

    let other_show = shows_factory::create_show("Test Show 2");
    let other_episode =
        shows_factory::create_episode_for_show("Test Episode 2", other_show.clone());

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![
                (episode.clone(), show.clone()),
                (other_episode.clone(), other_show.clone()),
            ]])
            .into_connection(),
    );

    let service = DefaultEpisodesService::new(db.clone());

    let result = service
        .get_many(
            Some(EpisodeCondition {
                title: Some("Test Episode".to_string()),
                show_id: None,
            }),
            None,
            None,
            None,
            &true,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(
        result,
        ManyResponse {
            data: vec![episode, other_episode],
            count: 2,
            total: 2,
            page: 1,
            page_count: 1,
        }
    );

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "episodes"."id" AS "A_id", "episodes"."created_at" AS "A_created_at", "episodes"."updated_at" AS "A_updated_at", "episodes"."title" AS "A_title", "episodes"."summary" AS "A_summary", "episodes"."picture" AS "A_picture", "episodes"."content" AS "A_content", "episodes"."show_id" AS "A_show_id", "shows"."id" AS "B_id", "shows"."created_at" AS "B_created_at", "shows"."updated_at" AS "B_updated_at", "shows"."title" AS "B_title", "shows"."summary" AS "B_summary", "shows"."picture" AS "B_picture", "shows"."content" AS "B_content" FROM "episodes" LEFT JOIN "shows" ON "episodes"."show_id" = "shows"."id" WHERE "episodes"."title" = $1"#,
            vec!["Test Episode".into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_get_many_pagination() -> Result<()> {
    let shows = vec![
        shows_factory::create_show("Test Show 1"),
        shows_factory::create_show("Test Show 2"),
        shows_factory::create_show("Test Show 3"),
        shows_factory::create_show("Test Show 4"),
        shows_factory::create_show("Test Show 5"),
    ];

    let episodes: Vec<Episode> = shows
        .into_iter()
        .map(|show| Episode {
            show: None,
            ..shows_factory::create_episode_for_show("Test Episode 1", show)
        })
        .collect();

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![maplit::btreemap! {
                // First query result
                "num_items" => Into::<Value>::into(11i64),
            }]])
            .append_query_results(vec![
                // Second query result
                episodes.clone(),
            ])
            .into_connection(),
    );

    let service = DefaultEpisodesService::new(db.clone());

    let result = service
        .get_many(
            None,
            Some(vec![EpisodesOrderBy::CreatedAtDesc]),
            Some(2),
            Some(5),
            &false,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(
        result,
        ManyResponse {
            data: episodes,
            count: 5,
            total: 11,
            page: 2,
            page_count: 3,
        }
    );

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT COUNT(*) AS num_items FROM (SELECT "episodes"."id", "episodes"."created_at", "episodes"."updated_at", "episodes"."title", "episodes"."summary", "episodes"."picture", "episodes"."content", "episodes"."show_id" FROM "episodes" ORDER BY "episodes"."created_at" DESC) AS "sub_query""#,
                vec![]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "episodes"."id", "episodes"."created_at", "episodes"."updated_at", "episodes"."title", "episodes"."summary", "episodes"."picture", "episodes"."content", "episodes"."show_id" FROM "episodes" ORDER BY "episodes"."created_at" DESC LIMIT $1 OFFSET $2"#,
                vec![5u64.into(), 5u64.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_get_many_pagination_with_related() -> Result<()> {
    let shows = vec![
        shows_factory::create_show("Test Show 1"),
        shows_factory::create_show("Test Show 2"),
        shows_factory::create_show("Test Show 3"),
        shows_factory::create_show("Test Show 4"),
        shows_factory::create_show("Test Show 5"),
    ];

    let episodes = vec![
        (
            shows_factory::create_episode_for_show("Test Episode 1", shows[0].clone()),
            shows[0].clone(),
        ),
        (
            shows_factory::create_episode_for_show("Test Episode 1", shows[1].clone()),
            shows[1].clone(),
        ),
        (
            shows_factory::create_episode_for_show("Test Episode 1", shows[2].clone()),
            shows[2].clone(),
        ),
        (
            shows_factory::create_episode_for_show("Test Episode 1", shows[3].clone()),
            shows[3].clone(),
        ),
        (
            shows_factory::create_episode_for_show("Test Episode 1", shows[4].clone()),
            shows[4].clone(),
        ),
    ];

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![maplit::btreemap! {
                // First query result
                "num_items" => Into::<Value>::into(11i64),
            }]])
            .append_query_results(vec![
                // Second query result
                episodes.clone(),
            ])
            .into_connection(),
    );

    let service = DefaultEpisodesService::new(db.clone());

    let result = service
        .get_many(
            None,
            Some(vec![EpisodesOrderBy::CreatedAtDesc]),
            Some(2),
            Some(5),
            &true,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(
        result,
        ManyResponse {
            data: episodes
                .into_iter()
                .map(|(episode, _show)| episode)
                .collect(),
            count: 5,
            total: 11,
            page: 2,
            page_count: 3,
        }
    );

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT COUNT(*) AS num_items FROM (SELECT "episodes"."id" AS "A_id", "episodes"."created_at" AS "A_created_at", "episodes"."updated_at" AS "A_updated_at", "episodes"."title" AS "A_title", "episodes"."summary" AS "A_summary", "episodes"."picture" AS "A_picture", "episodes"."content" AS "A_content", "episodes"."show_id" AS "A_show_id", "shows"."id" AS "B_id", "shows"."created_at" AS "B_created_at", "shows"."updated_at" AS "B_updated_at", "shows"."title" AS "B_title", "shows"."summary" AS "B_summary", "shows"."picture" AS "B_picture", "shows"."content" AS "B_content" FROM "episodes" LEFT JOIN "shows" ON "episodes"."show_id" = "shows"."id" ORDER BY "episodes"."created_at" DESC) AS "sub_query""#,
                vec![]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "episodes"."id" AS "A_id", "episodes"."created_at" AS "A_created_at", "episodes"."updated_at" AS "A_updated_at", "episodes"."title" AS "A_title", "episodes"."summary" AS "A_summary", "episodes"."picture" AS "A_picture", "episodes"."content" AS "A_content", "episodes"."show_id" AS "A_show_id", "shows"."id" AS "B_id", "shows"."created_at" AS "B_created_at", "shows"."updated_at" AS "B_updated_at", "shows"."title" AS "B_title", "shows"."summary" AS "B_summary", "shows"."picture" AS "B_picture", "shows"."content" AS "B_content" FROM "episodes" LEFT JOIN "shows" ON "episodes"."show_id" = "shows"."id" ORDER BY "episodes"."created_at" DESC LIMIT $1 OFFSET $2"#,
                vec![5u64.into(), 5u64.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_create() -> Result<()> {
    let show = shows_factory::create_show("Test Show");
    let episode = Episode {
        show: None,
        ..shows_factory::create_episode_for_show("Test Episode", show.clone())
    };

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![episode.clone()]])
            .into_connection(),
    );

    let service = DefaultEpisodesService::new(db.clone());

    let result = service
        .create(
            &CreateEpisodeInput {
                title: episode.title.clone(),
                summary: episode.summary.clone(),
                picture: episode.picture.clone(),
                content: episode.content.clone(),
                show_id: show.id.clone(),
            },
            &false,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, episode);

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"INSERT INTO "episodes" ("title", "summary", "picture", "content", "show_id") VALUES ($1, $2, $3, $4, $5) RETURNING "id", "created_at", "updated_at", "title", "summary", "picture", "content", "show_id""#,
            vec![
                episode.title.into(),
                episode.summary.into(),
                episode.picture.into(),
                episode.content.into(),
                episode.show_id.into(),
            ]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_create_with_related() -> Result<()> {
    let show = shows_factory::create_show("Test Show");
    let episode = shows_factory::create_episode_for_show("Test Episode", show.clone());

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![episode.clone()]])
            .append_query_results(vec![vec![show.clone()]])
            .into_connection(),
    );

    let service = DefaultEpisodesService::new(db.clone());

    let result = service
        .create(
            &CreateEpisodeInput {
                title: episode.title.clone(),
                summary: episode.summary.clone(),
                picture: episode.picture.clone(),
                content: episode.content.clone(),
                show_id: show.id.clone(),
            },
            &true,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, episode);

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"INSERT INTO "episodes" ("title", "summary", "picture", "content", "show_id") VALUES ($1, $2, $3, $4, $5) RETURNING "id", "created_at", "updated_at", "title", "summary", "picture", "content", "show_id""#,
                vec![
                    episode.title.into(),
                    episode.summary.into(),
                    episode.picture.into(),
                    episode.content.into(),
                    episode.show_id.into(),
                ]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "shows"."id", "shows"."created_at", "shows"."updated_at", "shows"."title", "shows"."summary", "shows"."picture", "shows"."content" FROM "shows" WHERE "shows"."id" = $1 LIMIT $2"#,
                vec!["test-show".into(), 1u64.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_update_model() -> Result<()> {
    let show = shows_factory::create_show("Test Show");
    let episode = Episode {
        show: None,
        ..shows_factory::create_episode_for_show("Test Episode", show.clone())
    };

    let updated = Episode {
        title: "Updated Episode".to_string(),
        ..episode.clone()
    };

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![updated.clone()]])
            .into_connection(),
    );

    let service = DefaultEpisodesService::new(db.clone());

    let result = service
        .update_model(
            episode.clone(),
            &UpdateEpisodeInput {
                title: Some(updated.title.clone()),
                summary: None,
                picture: None,
                content: None,
                show_id: Some(show.id.clone()),
            },
            None,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, updated.clone());

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"UPDATE "episodes" SET "title" = $1, "show_id" = $2 WHERE "episodes"."id" = $3 RETURNING "id", "created_at", "updated_at", "title", "summary", "picture", "content", "show_id""#,
            vec![updated.title.into(), show.id.into(), episode.id.into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_update_model_with_related() -> Result<()> {
    let show = shows_factory::create_show("Test Show");
    let episode = shows_factory::create_episode_for_show("Test Episode", show.clone());

    let updated = Episode {
        title: "Updated Episode".to_string(),
        ..episode.clone()
    };

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![updated.clone()]])
            .append_query_results(vec![vec![show.clone()]])
            .into_connection(),
    );

    let service = DefaultEpisodesService::new(db.clone());

    let result = service
        .update_model(
            episode.clone(),
            &UpdateEpisodeInput {
                title: Some(updated.title.clone()),
                summary: None,
                picture: None,
                content: None,
                show_id: Some(show.id.clone()),
            },
            Some(show.clone()),
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, updated.clone());

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"UPDATE "episodes" SET "title" = $1, "show_id" = $2 WHERE "episodes"."id" = $3 RETURNING "id", "created_at", "updated_at", "title", "summary", "picture", "content", "show_id""#,
            vec![updated.title.into(), show.id.into(), episode.id.into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_update() -> Result<()> {
    let show = shows_factory::create_show("Test Show");
    let episode = Episode {
        show: None,
        ..shows_factory::create_episode_for_show("Test Episode", show.clone())
    };

    let updated = Episode {
        title: "Updated Episode".to_string(),
        ..episode.clone()
    };

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![episode.clone()], vec![updated.clone()]])
            .into_connection(),
    );

    let service = DefaultEpisodesService::new(db.clone());

    let result = service
        .update(
            &show.id,
            &UpdateEpisodeInput {
                title: Some(updated.title.clone()),
                summary: None,
                picture: None,
                content: None,
                show_id: Some(show.id.clone()),
            },
            &false,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, updated.clone());

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "episodes"."id", "episodes"."created_at", "episodes"."updated_at", "episodes"."title", "episodes"."summary", "episodes"."picture", "episodes"."content", "episodes"."show_id" FROM "episodes" WHERE "episodes"."id" = $1 LIMIT $2"#,
                vec![show.id.clone().into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"UPDATE "episodes" SET "title" = $1, "show_id" = $2 WHERE "episodes"."id" = $3 RETURNING "id", "created_at", "updated_at", "title", "summary", "picture", "content", "show_id""#,
                vec![updated.title.into(), show.id.into(), episode.id.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_update_with_related() -> Result<()> {
    let show = shows_factory::create_show("Test Show");
    let episode = shows_factory::create_episode_for_show("Test Episode", show.clone());

    let updated = Episode {
        title: "Updated Episode".to_string(),
        ..episode.clone()
    };

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![(episode.clone(), show.clone())]])
            .append_query_results(vec![vec![updated.clone()]])
            .into_connection(),
    );

    let service = DefaultEpisodesService::new(db.clone());

    let result = service
        .update(
            &show.id,
            &UpdateEpisodeInput {
                title: Some(updated.title.clone()),
                summary: None,
                picture: None,
                content: None,
                show_id: Some(show.id.clone()),
            },
            &true,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, updated.clone());

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "episodes"."id" AS "A_id", "episodes"."created_at" AS "A_created_at", "episodes"."updated_at" AS "A_updated_at", "episodes"."title" AS "A_title", "episodes"."summary" AS "A_summary", "episodes"."picture" AS "A_picture", "episodes"."content" AS "A_content", "episodes"."show_id" AS "A_show_id", "shows"."id" AS "B_id", "shows"."created_at" AS "B_created_at", "shows"."updated_at" AS "B_updated_at", "shows"."title" AS "B_title", "shows"."summary" AS "B_summary", "shows"."picture" AS "B_picture", "shows"."content" AS "B_content" FROM "episodes" LEFT JOIN "shows" ON "episodes"."show_id" = "shows"."id" WHERE "episodes"."id" = $1 LIMIT $2"#,
                vec![show.id.clone().into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"UPDATE "episodes" SET "title" = $1, "show_id" = $2 WHERE "episodes"."id" = $3 RETURNING "id", "created_at", "updated_at", "title", "summary", "picture", "content", "show_id""#,
                vec![updated.title.into(), show.id.into(), episode.id.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_delete() -> Result<()> {
    let show = shows_factory::create_show("Test Show");
    let episode = Episode {
        show: None,
        ..shows_factory::create_episode_for_show("Test Episode", show.clone())
    };

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![episode.clone()]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }])
            .into_connection(),
    );

    let service = DefaultEpisodesService::new(db.clone());

    service.delete(&episode.id).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "episodes"."id", "episodes"."created_at", "episodes"."updated_at", "episodes"."title", "episodes"."summary", "episodes"."picture", "episodes"."content", "episodes"."show_id" FROM "episodes" WHERE "episodes"."id" = $1 LIMIT $2"#,
                vec![episode.id.clone().into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"DELETE FROM "episodes" WHERE "episodes"."id" = $1"#,
                vec![episode.id.into()]
            )
        ]
    );

    Ok(())
}
