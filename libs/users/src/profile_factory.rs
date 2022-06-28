use fake::{Fake, Faker};
use slug::slugify;

use crate::{
    profile_model::Model as ProfileModel, profile_mutations::CreateProfileInput,
    user_factory::create_user_with_username, user_model::User,
};
use caster_utils::json::JsonOption;

/// Create a `CreateProfileInput`
pub fn create_profile_input(user_id: &str, email: &str) -> CreateProfileInput {
    CreateProfileInput {
        email: email.to_string(),
        user_id: user_id.to_string(),
        display_name: Faker.fake(),
        picture: Faker.fake(),
        content: JsonOption::new(None),
        city: Faker.fake(),
        state_province: Faker.fake(),
    }
}

/// Create a `Profile`
pub fn create_profile_factory(email: Option<String>, user: Option<User>) -> ProfileModel {
    let email = email.unwrap_or_else(|| Faker.fake());
    let user = user.unwrap_or_else(|| create_user_with_username(&email));

    ProfileModel {
        id: slugify(email.clone()),
        email,
        display_name: Faker.fake(),
        city: Faker.fake(),
        state_province: Faker.fake(),
        picture: Faker.fake(),
        content: None,
        user_id: Some(user.id),
        created_at: Faker.fake(),
        updated_at: Faker.fake(),
    }
}

/// Create a `Profile` for a `User`
pub fn create_profile_for_user(email: &str, user: User) -> ProfileModel {
    create_profile_factory(Some(email.to_string()), Some(user))
}

/// Create a `Profile` with an email
#[allow(dead_code)]
pub fn create_profile_with_email(email: &str) -> ProfileModel {
    create_profile_factory(Some(email.to_string()), None)
}

/// Create a `Profile` without any input
#[allow(dead_code)]
pub fn create_profile() -> ProfileModel {
    create_profile_factory(None, None)
}
