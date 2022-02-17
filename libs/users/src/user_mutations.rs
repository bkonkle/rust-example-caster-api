use async_graphql::{InputObject, SimpleObject};

use crate::user_model::User;

/// The `CreateUserProfileInput` input type
#[derive(Clone, Eq, PartialEq, InputObject)]
pub struct CreateUserProfileInput {
    /// The Profile's email address
    pub email: String,

    /// The Profile's display name
    pub display_name: Option<String>,

    /// The Profile's picture
    pub picture: Option<String>,

    /// The Profile json content
    pub content: Option<serde_json::Value>,

    /// The Profile's city
    pub city: Option<String>,

    /// The Profile's state or province
    pub state_province: Option<String>,
}

/// The `CreateUserInput` input type
#[derive(Clone, Eq, PartialEq, InputObject)]
pub struct CreateUserInput {
    /// The User's profile
    pub profile: Option<CreateUserProfileInput>,
}

/// The `UpdateUserInput` input type
#[derive(Clone, Eq, PartialEq, InputObject)]
pub struct UpdateUserInput {
    /// The User's subscriber id
    pub username: Option<String>,

    /// Whether the User is active or disabled
    pub is_active: Option<bool>,
}

/// The `MutateUserResult` input type
#[derive(Clone, Eq, PartialEq, SimpleObject)]
pub struct MutateUserResult {
    /// The User's subscriber id
    pub user: Option<User>,
}
