use fake::{Fake, Faker};

use super::*;

pub fn create_show() -> Show {
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
