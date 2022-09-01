use async_graphql::{InputObject, SimpleObject};
use fake::{Dummy, Fake};

use super::model::Profile;

/// The `CreateProfileInput` input type
#[derive(Clone, Default, Dummy, Eq, PartialEq, InputObject)]
pub struct CreateProfileInput {
    /// The Profile's email address
    pub email: String,

    /// The Profile's display name
    pub display_name: Option<String>,

    /// The Profile's picture
    pub picture: Option<String>,

    /// The Profile's city
    pub city: Option<String>,

    /// The Profile's state or province
    pub state_province: Option<String>,

    /// The Profile's User id
    pub user_id: String,
}

/// The `UpdateProfileInput` input type
#[derive(Clone, Default, Dummy, Eq, PartialEq, InputObject)]
pub struct UpdateProfileInput {
    /// The Profile's email address
    pub email: Option<String>,

    /// The Profile's display name
    pub display_name: Option<String>,

    /// The Profile's picture
    pub picture: Option<String>,

    /// The Profile's city
    pub city: Option<String>,

    /// The Profile's state or province
    pub state_province: Option<String>,

    /// The Profile's User id
    pub user_id: Option<String>,
}

/// The `MutateProfileResult` type
#[derive(Clone, Default, Dummy, Eq, PartialEq, SimpleObject)]
pub struct MutateProfileResult {
    /// The Profile's subscriber id
    pub profile: Option<Profile>,
}
