use dotenv::dotenv;

use crate::{postgres, router::create_routes, server::get_addr};

#[tokio::test]
#[ignore]
async fn test_initial() {
    dotenv().ok();
    pretty_env_logger::init();

    let pg_pool = postgres::init()
        .await
        .expect("Unable to initialize Postgres Pool.");

    let filter = create_routes(pg_pool);

    warp::serve(filter).run(get_addr()).await;
}
