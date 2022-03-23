use anyhow::Result;
use sea_orm::{DatabaseBackend, MockDatabase, Transaction};
use std::sync::Arc;

use caster_shows::shows_service::{DefaultShowsService, ShowsService};

mod factories;

#[tokio::test]
async fn test_shows_service_get_show() -> Result<()> {
    let show = factories::create_show();

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
        ),]
    );

    Ok(())
}
