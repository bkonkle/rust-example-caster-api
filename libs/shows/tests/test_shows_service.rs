use caster_shows::shows_service::{DefaultShowsService, ShowsService};
use sea_orm::{tests_cfg::*, DatabaseBackend, MockDatabase};
use std::sync::Arc;

mod factories;

#[tokio::test]
async fn test_shows_service_get_show() {
    let show = factories::create_show();

    // shows_repo
    //     .expect_get()
    //     .times(1)
    //     .with(eq(String::from(&show.id)))
    //     .returning(move |_| Ok(Some(response.clone())));

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![show.clone()]])
            .into_connection(),
    );

    let service = DefaultShowsService::new(&db);

    let result = service.get_model(&show.id).await;

    println!("{:?}", db.into_transaction_log());

    match result {
        Ok(result_opt) => match result_opt {
            Some(result_show) => assert_eq!(result_show, show),
            None => panic!("Result was None"),
        },
        Err(_) => panic!("Result was not Ok"),
    };
}
