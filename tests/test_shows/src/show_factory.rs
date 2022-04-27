use fake::{Fake, Faker};
use slug::slugify;

use caster_shows::{show_model::Show, show_mutations::CreateShowInput};

/// Create a `CreateShowInput`
pub fn create_show_input(show_title: &str) -> CreateShowInput {
    CreateShowInput {
        title: show_title.to_string(),
        summary: Some(Faker.fake()),
        picture: Some(Faker.fake()),
        content: None,
    }
}

/// Create a `Show`
pub fn create_show_factory(title: Option<String>) -> Show {
    let title = title.unwrap_or_else(|| Faker.fake());

    Show {
        id: slugify(title.clone()),
        title,
        summary: Some(Faker.fake()),
        picture: Some(Faker.fake()),
        content: None,
        created_at: Faker.fake(),
        updated_at: Faker.fake(),
    }
}

/// Create a `Show` with a title
pub fn create_show_with_title(title: &str) -> Show {
    create_show_factory(Some(title.to_string()))
}

/// Create a `Show` with no input
#[allow(dead_code)]
pub fn create_show() -> Show {
    create_show_factory(None)
}
