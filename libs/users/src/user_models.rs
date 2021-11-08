use sqlx::types::chrono;

/// The User model
#[derive(Clone, Eq, PartialEq)]
#[allow(dead_code, non_snake_case)]
pub struct User {
    /// The User id
    pub id: String,

    /// The User's subscriber id
    pub username: String,

    /// Whether the User is active or disabled
    pub isActive: bool,

    /// The date the User was created
    pub createdAt: chrono::NaiveDateTime,

    /// The date the User was last updated
    pub updatedAt: chrono::NaiveDateTime,
}
