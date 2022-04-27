use fake::{Fake, Faker};
use slug::slugify;

use caster_users::user_model::User;

/// Create a `User`
pub fn create_user_factory(username: Option<String>) -> User {
    let username = username.map_or_else(|| Faker.fake(), slugify);

    User {
        id: slugify(username.clone()),
        username,
        is_active: true,
        roles: vec![],
        created_at: Faker.fake(),
        updated_at: Faker.fake(),
    }
}

/// Create a `User` with a username
pub fn create_user_with_username(username: &str) -> User {
    create_user_factory(Some(username.to_string()))
}

/// Create a `User` without any input
pub fn create_user() -> User {
    create_user_factory(None)
}
