use fake::{Fake, Faker};
use slug::slugify;

use caster_users::{profile_model::Profile, role_grant_model::RoleGrant, user_model::User};

pub fn create_user(username: &str) -> User {
    User {
        id: slugify(username),
        username: username.to_string(),
        is_active: true,
        roles: vec![],
        created_at: Faker.fake(),
        updated_at: Faker.fake(),
    }
}

#[allow(dead_code)]
pub fn create_profile_for_user_option(email: &str, user: Option<User>) -> Profile {
    let user = user.unwrap_or_else(|| create_user(email));

    Profile {
        id: slugify(email),
        email: Some(email.to_string()),
        display_name: Faker.fake(),
        city: Faker.fake(),
        state_province: Faker.fake(),
        picture: Faker.fake(),
        content: None,
        user_id: Some(user.id.clone()),
        user: Some(user),
        created_at: Faker.fake(),
        updated_at: Faker.fake(),
    }
}

#[allow(dead_code)]
pub fn create_profile_for_user(email: &str, user: User) -> Profile {
    create_profile_for_user_option(email, Some(user))
}

#[allow(dead_code)]
pub fn create_profile(email: &str) -> Profile {
    create_profile_for_user_option(email, None)
}

#[allow(dead_code)]
pub fn create_role_grant_for_user(table: &str, id: &str, user: User) -> RoleGrant {
    RoleGrant {
        id: Faker.fake(),
        created_at: Faker.fake(),
        updated_at: Faker.fake(),
        role_key: Faker.fake(),
        user_id: user.id,
        resource_table: table.to_string(),
        resource_id: id.to_string(),
    }
}
