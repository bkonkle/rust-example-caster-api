use fake::{Fake, Faker};
use slug::slugify;

use caster_users::{
    profile_model::Model as ProfileModel, role_grant_model::RoleGrant, user_model::User,
};

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
pub fn create_profile_for_user_option(email: &str, user: Option<User>) -> ProfileModel {
    let user = user.unwrap_or_else(|| create_user(email));

    ProfileModel {
        id: slugify(email),
        email: email.to_string(),
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

#[allow(dead_code)]
pub fn create_profile_for_user(email: &str, user: User) -> ProfileModel {
    create_profile_for_user_option(email, Some(user))
}

#[allow(dead_code)]
pub fn create_profile(email: &str) -> ProfileModel {
    create_profile_for_user_option(email, None)
}

#[allow(dead_code)]
pub fn create_role_grant_for_user(table: &str, id: &str, user: User) -> RoleGrant {
    RoleGrant {
        id: format!("{}-{}", user.id, id),
        created_at: Faker.fake(),
        updated_at: Faker.fake(),
        role_key: Faker.fake(),
        user_id: user.id,
        resource_table: table.to_string(),
        resource_id: id.to_string(),
    }
}
