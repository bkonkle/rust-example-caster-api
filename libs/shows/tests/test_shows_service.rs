use anyhow::Result;
use sea_orm::{DatabaseBackend, MockDatabase};
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

    // println!("{:?}", db.into_transaction_log());

    if let Some(result_show) = result {
        assert_eq!(result_show, show);
    } else {
        panic!("Result was None");
    }

    Ok(())
}
