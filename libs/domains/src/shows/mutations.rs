use async_graphql::{InputObject, SimpleObject};
use fake::{Dummy, Fake};

use super::model::Show;

/// The `CreateShowInput` input type
#[derive(Clone, Default, Dummy, Eq, PartialEq, InputObject)]
pub struct CreateShowInput {
    /// The Show's title
    pub title: String,

    /// The Show's description summary
    pub summary: Option<String>,

    /// The Show's picture
    pub picture: Option<String>,
}

/// The `UpdateShowInput` input type
#[derive(Clone, Default, Dummy, Eq, PartialEq, InputObject)]
pub struct UpdateShowInput {
    /// The Show's title
    pub title: Option<String>,

    /// The Show's description summary
    pub summary: Option<String>,

    /// The Show's picture
    pub picture: Option<String>,
}

/// The `MutateShowResult` type
#[derive(Clone, Default, Dummy, Eq, PartialEq, SimpleObject)]
pub struct MutateShowResult {
    /// The Show's subscriber id
    pub show: Option<Show>,
}
