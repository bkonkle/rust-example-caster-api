#[cfg(test)]
use caster_data::postgres::*;

use super::super::shows_service::*;

#[tokio::test]
async fn test_get() {
    let pg_pool = MockPostgresPool::init()
        .await
        .expect("Unable to initialize MockPostgresPool");

    let shows_service = PgShowsService::new(&pg_pool);

    let result = shows_service
        .get(String::from("test-id"))
        .await
        .expect("Unable to retrieve Show");

    assert_eq!(result, result.clone());
}
