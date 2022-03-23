use anyhow::Result;
use caster_utils::pagination::ManyResponse;
use sea_orm::{DatabaseBackend, JsonValue, MockDatabase, Transaction, Value};
use std::sync::Arc;

use caster_shows::{
    show_queries::{ShowCondition, ShowsOrderBy},
    shows_service::{DefaultShowsService, ShowsService},
};

mod shows_factory;

#[tokio::test]
async fn test_shows_service_get_model() -> Result<()> {
    let show = shows_factory::create_show();

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![show.clone()]])
            .into_connection(),
    );

    let service = DefaultShowsService::new(db.clone());

    let result = service.get_model(&show.id).await?;

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
async fn test_shows_service_get() -> Result<()> {
    let show = shows_factory::create_show();

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![show.clone()]])
            .into_connection(),
    );

    let service = DefaultShowsService::new(db.clone());

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
    let show = shows_factory::create_show();
    let other_show = shows_factory::create_show();

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![show.clone(), other_show.clone()]])
            .into_connection(),
    );

    let service = DefaultShowsService::new(db.clone());

    let result = service
        .get_many(
            Some(ShowCondition {
                title: Some("Test Title".to_string()),
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
            vec!["Test Title".into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_shows_service_get_many_pagination() -> Result<()> {
    let shows = vec![
        shows_factory::create_show(),
        shows_factory::create_show(),
        shows_factory::create_show(),
        shows_factory::create_show(),
        shows_factory::create_show(),
    ];

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![maplit::btreemap! {
                "num_items" => Into::<Value>::into(11),
            }]])
            .append_query_results(vec![shows.clone()])
            .into_connection(),
    );

    let service = DefaultShowsService::new(db.clone());

    let result = service
        .get_many(
            None,
            Some(vec![ShowsOrderBy::CreatedAtDesc]),
            Some(5),
            Some(2),
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = Arc::try_unwrap(db).expect("Unable to unwrap the DatabaseConnection");

    assert_eq!(
        result,
        ManyResponse {
            data: shows,
            count: 11,
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
            vec!["Test Title".into()]
        )]
    );

    Ok(())
}
