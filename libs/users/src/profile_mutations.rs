use async_graphql::{InputObject, SimpleObject};
use serde_json;

use crate::profile_model::Profile;

/// The `CreateProfileInput` input type
#[derive(Clone, Eq, PartialEq, InputObject)]
pub struct CreateProfileInput {
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

    /// The Profile's User id
    pub user_id: String,
}

/// The `UpdateProfileInput` input type
#[derive(Clone, Eq, PartialEq, InputObject)]
pub struct UpdateProfileInput {
    /// The Profile's email address
    pub email: Option<String>,

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

    /// The Profile's User id
    pub user_id: Option<String>,
}

/// The `MutateProfileResult` type
#[derive(Clone, Eq, PartialEq, SimpleObject)]
pub struct MutateProfileResult {
    /// The Profile's subscriber id
    pub profile: Option<Profile>,
}
