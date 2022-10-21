use anyhow::Result;
use async_graphql::MaybeUndefined;
use fake::{Fake, Faker};
use pretty_assertions::assert_eq;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult, Transaction, Value};
use std::sync::Arc;

use crate::shows::{
    model::Show,
    mutations::{CreateShowInput, UpdateShowInput},
    queries::{ShowCondition, ShowsOrderBy},
    service::{DefaultShowsService, ShowsService},
};
use caster_utils::pagination::ManyResponse;

#[tokio::test]
async fn test_shows_service_get() -> Result<()> {
    let mut show: Show = Faker.fake();
    show.title = "Test Show".to_string();

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

    assert_eq!(result, Some(show.clone()));

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "shows"."id", "shows"."created_at", "shows"."updated_at", "shows"."title", "shows"."summary", "shows"."picture" FROM "shows" WHERE "shows"."id" = $1 LIMIT $2"#,
            vec![show.id.into(), 1u64.into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_shows_service_get_many() -> Result<()> {
    let mut show: Show = Faker.fake();
    show.title = "Test Show".to_string();

    let mut other_show: Show = Faker.fake();
    other_show.title = "Other Show".to_string();

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
            r#"SELECT "shows"."id", "shows"."created_at", "shows"."updated_at", "shows"."title", "shows"."summary", "shows"."picture" FROM "shows" WHERE "shows"."title" = $1"#,
            vec!["Test Show".into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_shows_service_get_many_pagination() -> Result<()> {
    let mut show1: Show = Faker.fake();
    show1.title = "Test Show 1".to_string();

    let mut show2: Show = Faker.fake();
    show2.title = "Test Show 2".to_string();

    let mut show3: Show = Faker.fake();
    show3.title = "Test Show 3".to_string();

    let mut show4: Show = Faker.fake();
    show4.title = "Test Show 4".to_string();

    let mut show5: Show = Faker.fake();
    show5.title = "Test Show 5".to_string();

    let shows = vec![show1, show2, show3, show4, show5];

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
                r#"SELECT COUNT(*) AS num_items FROM (SELECT "shows"."id", "shows"."created_at", "shows"."updated_at", "shows"."title", "shows"."summary", "shows"."picture" FROM "shows" ORDER BY "shows"."created_at" DESC) AS "sub_query""#,
                vec![]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "shows"."id", "shows"."created_at", "shows"."updated_at", "shows"."title", "shows"."summary", "shows"."picture" FROM "shows" ORDER BY "shows"."created_at" DESC LIMIT $1 OFFSET $2"#,
                vec![5u64.into(), 5u64.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_shows_service_create() -> Result<()> {
    let mut show: Show = Faker.fake();
    show.title = "Test Show".to_string();

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
            r#"INSERT INTO "shows" ("title", "summary", "picture") VALUES ($1, $2, $3) RETURNING "id", "created_at", "updated_at", "title", "summary", "picture""#,
            vec![show.title.into(), show.summary.into(), show.picture.into(),]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_shows_service_update() -> Result<()> {
    let mut show: Show = Faker.fake();
    show.title = "Test Show".to_string();

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
                title: MaybeUndefined::Value(updated.title.clone()),
                summary: MaybeUndefined::Undefined,
                picture: MaybeUndefined::Undefined,
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
                r#"SELECT "shows"."id", "shows"."created_at", "shows"."updated_at", "shows"."title", "shows"."summary", "shows"."picture" FROM "shows" WHERE "shows"."id" = $1 LIMIT $2"#,
                vec![show.id.clone().into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"UPDATE "shows" SET "title" = $1 WHERE "shows"."id" = $2 RETURNING "id", "created_at", "updated_at", "title", "summary", "picture""#,
                vec![updated.title.into(), show.id.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_shows_service_delete() -> Result<()> {
    let mut show: Show = Faker.fake();
    show.title = "Test Show".to_string();

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
                r#"SELECT "shows"."id", "shows"."created_at", "shows"."updated_at", "shows"."title", "shows"."summary", "shows"."picture" FROM "shows" WHERE "shows"."id" = $1 LIMIT $2"#,
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
