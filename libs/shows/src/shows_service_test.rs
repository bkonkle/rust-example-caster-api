use anyhow::Result;
use pretty_assertions::assert_eq;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult, Transaction, Value};
use std::sync::Arc;

use crate::{
    show_factory,
    show_model::Show,
    show_mutations::{CreateShowInput, UpdateShowInput},
    show_queries::{ShowCondition, ShowsOrderBy},
    shows_service::{DefaultShowsService, ShowsService},
};
use caster_utils::pagination::ManyResponse;

#[tokio::test]
async fn test_shows_service_get() -> Result<()> {
    let show = show_factory::create_show_with_title("Test Show");

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![show.clone()]])
            .into_connection(),
    );

    let service = DefaultShowsService::new(&db);

    let result = service.get(&show.id).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, Some(show));

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "shows"."id", "shows"."created_at", "shows"."updated_at", "shows"."title", "shows"."summary", "shows"."picture", "shows"."content" FROM "shows" WHERE "shows"."id" = $1 LIMIT $2"#,
            vec!["test-show".into(), 1u64.into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_shows_service_get_many() -> Result<()> {
    let show = show_factory::create_show_with_title("Test Show");
    let other_show = show_factory::create_show_with_title("Test Show");

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![show.clone(), other_show.clone()]])
            .into_connection(),
    );

    let service = DefaultShowsService::new(&db);

    let result = service
        .get_many(
            Some(ShowCondition {
                title: Some("Test Show".to_string()),
            }),
            None,
            None,
            None,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(
        result,
        ManyResponse {
            data: vec![show, other_show],
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
            r#"SELECT "shows"."id", "shows"."created_at", "shows"."updated_at", "shows"."title", "shows"."summary", "shows"."picture", "shows"."content" FROM "shows" WHERE "shows"."title" = $1"#,
            vec!["Test Show".into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_shows_service_get_many_pagination() -> Result<()> {
    let shows = vec![
        show_factory::create_show_with_title("Test Show 1"),
        show_factory::create_show_with_title("Test Show 2"),
        show_factory::create_show_with_title("Test Show 3"),
        show_factory::create_show_with_title("Test Show 4"),
        show_factory::create_show_with_title("Test Show 5"),
    ];

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![maplit::btreemap! {
                // First query result
                "num_items" => Into::<Value>::into(11i64),
            }]])
            .append_query_results(vec![
                // Second query result
                shows.clone(),
            ])
            .into_connection(),
    );

    let service = DefaultShowsService::new(&db);

    let result = service
        .get_many(
            None,
            Some(vec![ShowsOrderBy::CreatedAtDesc]),
            Some(2),
            Some(5),
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(
        result,
        ManyResponse {
            data: shows,
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
                r#"SELECT COUNT(*) AS num_items FROM (SELECT "shows"."id", "shows"."created_at", "shows"."updated_at", "shows"."title", "shows"."summary", "shows"."picture", "shows"."content" FROM "shows" ORDER BY "shows"."created_at" DESC) AS "sub_query""#,
                vec![]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "shows"."id", "shows"."created_at", "shows"."updated_at", "shows"."title", "shows"."summary", "shows"."picture", "shows"."content" FROM "shows" ORDER BY "shows"."created_at" DESC LIMIT $1 OFFSET $2"#,
                vec![5u64.into(), 5u64.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_shows_service_create() -> Result<()> {
    let show = show_factory::create_show_with_title("Test Show");

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![show.clone()]])
            .into_connection(),
    );

    let service = DefaultShowsService::new(&db);

    let result = service
        .create(&CreateShowInput {
            title: show.title.clone(),
            summary: show.summary.clone(),
            picture: show.picture.clone(),
            content: show.content.clone(),
        })
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(result, show);

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"INSERT INTO "shows" ("title", "summary", "picture", "content") VALUES ($1, $2, $3, $4) RETURNING "id", "created_at", "updated_at", "title", "summary", "picture", "content""#,
            vec![
                show.title.into(),
                show.summary.into(),
                show.picture.into(),
                show.content.into()
            ]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_shows_service_update() -> Result<()> {
    let show = show_factory::create_show_with_title("Test Show");
    let updated = Show {
        title: "Updated Show".to_string(),
        ..show.clone()
    };

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![show.clone()], vec![updated.clone()]])
            .into_connection(),
    );

    let service = DefaultShowsService::new(&db);

    let result = service
        .update(
            &show.id,
            &UpdateShowInput {
                title: Some(updated.title.clone()),
                summary: None,
                picture: None,
                content: None,
            },
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
                r#"SELECT "shows"."id", "shows"."created_at", "shows"."updated_at", "shows"."title", "shows"."summary", "shows"."picture", "shows"."content" FROM "shows" WHERE "shows"."id" = $1 LIMIT $2"#,
                vec![show.id.clone().into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"UPDATE "shows" SET "title" = $1 WHERE "shows"."id" = $2 RETURNING "id", "created_at", "updated_at", "title", "summary", "picture", "content""#,
                vec![updated.title.into(), show.id.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_shows_service_delete() -> Result<()> {
    let show = show_factory::create_show_with_title("Test Show");

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![show.clone()]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }])
            .into_connection(),
    );

    let service = DefaultShowsService::new(&db);

    service.delete(&show.id).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "shows"."id", "shows"."created_at", "shows"."updated_at", "shows"."title", "shows"."summary", "shows"."picture", "shows"."content" FROM "shows" WHERE "shows"."id" = $1 LIMIT $2"#,
                vec![show.id.clone().into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"DELETE FROM "shows" WHERE "shows"."id" = $1"#,
                vec![show.id.into()]
            )
        ]
    );

    Ok(())
}
