use fake::{Fake, Faker};

use crate::user_factory;
use caster_users::{role_grant_model::RoleGrant, user_model::User};

/// Create a `RoleGrant`
pub fn create_role_grant_factory(
    table: Option<String>,
    id: Option<String>,
    user: Option<User>,
) -> RoleGrant {
    let table = table.unwrap_or_else(|| Faker.fake());
    let id = id.unwrap_or_else(|| Faker.fake());
    let user = user.unwrap_or_else(user_factory::create_user);

    RoleGrant {
        id: format!("{}-{}", user.id, id),
        created_at: Faker.fake(),
        updated_at: Faker.fake(),
        role_key: Faker.fake(),
        user_id: user.id,
        resource_table: table,
        resource_id: id,
    }
}

/// Create a `RoleGrant` for a `User`
pub fn create_role_grant_for_user(table: &str, id: &str, user: User) -> RoleGrant {
    create_role_grant_factory(Some(table.to_string()), Some(id.to_string()), Some(user))
}

/// Create a `RoleGrant` without any input
#[allow(dead_code)]
pub fn create_role_grant() -> RoleGrant {
    create_role_grant_factory(None, None, None)
}
