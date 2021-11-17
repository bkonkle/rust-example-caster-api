use mockall::predicate::*;

use crate::shows_repository::*;

use super::*;

#[path = "./show_model_factory.rs"]
mod factory;

#[tokio::test]
async fn test_get_show() {
    let show = factory::create_show();
    let response = show.clone();

    let mut shows_repo = MockShowsRepository::new();

    shows_repo
        .expect_get()
        .times(1)
        .with(eq(String::from(&show.id)))
        .returning(move |_| Ok(Some(response.clone())));

    let service = ShowsService::new(&Arc::new(shows_repo));

    let result = service.get(String::from(&show.id)).await;

    println!("{:?}", result);

    match result {
        Ok(result_opt) => match result_opt {
            Some(result_show) => assert_eq!(result_show, show),
            _ => panic!("Result was None"),
        },
        _ => panic!("Result was not Ok"),
    };
}
