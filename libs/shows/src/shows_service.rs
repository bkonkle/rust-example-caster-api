use std::sync::Arc;

#[cfg(test)]
use mockall::automock;

use crate::{show_model::Show, shows_repository::ShowsRepository};

/// The `Show` entity service
pub struct ShowsService {
    repo: Arc<dyn ShowsRepository>,
}

#[cfg_attr(test, automock)]
impl ShowsService {
    /// Create a new `ShowsService` instance with a type that implements `ShowsRepository`
    pub fn new<T: ShowsRepository + 'static>(repo: &Arc<T>) -> Self {
        Self { repo: repo.clone() }
    }

    /// Get an individual Show by id
    pub async fn get(&self, id: String) -> anyhow::Result<Option<Show>> {
        let show = (&*self.repo).get(id).await?;

        Ok(show)
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};
    use mockall::predicate::*;

    use super::*;
    use crate::shows_repository::*;

    fn create_show() -> Show {
        Show {
            id: String::from("test-show"),
            title: String::from("Test Show"),
            summary: Faker.fake(),
            picture: Faker.fake(),
            content: None,
            created_at: Faker.fake(),
            updated_at: Faker.fake(),
        }
    }

    #[tokio::test]
    async fn test_get_show() {
        let show = create_show();

        let mut shows_repo = MockShowsRepository::new();

        shows_repo
            .expect_get()
            .times(1)
            .with(eq(show.id))
            .returning(|_| Ok(Some(show)));

        let service = ShowsService::new(&Arc::new(shows_repo));

        let result = service.get(show.id).await;

        match result {
            Ok(result_opt) => match result_opt {
                Some(result_show) => assert_eq!(result_show, show),
                _ => panic!("Result was None"),
            },
            _ => panic!("Result was not Ok"),
        };
    }
}
