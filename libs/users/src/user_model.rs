use async_graphql::SimpleObject;
use chrono::NaiveDateTime;

/// The User model
#[derive(Clone, Eq, PartialEq, SimpleObject)]
pub struct User {
    /// The User id
    pub id: String,

    /// The User's subscriber id
    pub username: String,

    /// Whether the User is active or disabled
    pub is_active: bool,

    /// The date the User was created
    pub created_at: NaiveDateTime,

    /// The date the User was last updated
    pub updated_at: NaiveDateTime,
}
