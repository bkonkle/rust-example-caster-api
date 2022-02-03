use async_graphql::{InputObject, SimpleObject};

use crate::{profile_mutations::CreateProfileInput, user_model::User};

/// The `CreateUserInput` input type
#[derive(Clone, Eq, PartialEq, InputObject)]
pub struct CreateUserInput {
    /// The User's profile
    pub profile: Option<CreateProfileInput>,
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
