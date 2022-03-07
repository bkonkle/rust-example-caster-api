use async_graphql::{InputObject, SimpleObject};
use serde_json;

use crate::show_model::Show;

/// The `CreateShowInput` input type
#[derive(Clone, Eq, PartialEq, InputObject)]
pub struct CreateShowInput {
    /// The Show's title
    pub title: String,

    /// The Show's description summary
    pub summary: Option<String>,

    /// The Show's picture
    pub picture: Option<String>,

    /// The Show json content
    pub content: Option<serde_json::Value>,
}

/// The `UpdateShowInput` input type
#[derive(Clone, Eq, PartialEq, InputObject)]
pub struct UpdateShowInput {
    /// The Show's title
    pub title: Option<String>,

    /// The Show's description summary
    pub summary: Option<String>,

    /// The Show's picture
    pub picture: Option<String>,

    /// The Show json content
    pub content: Option<serde_json::Value>,
}

/// The `MutateShowResult` type
#[derive(Clone, Eq, PartialEq, SimpleObject)]
pub struct MutateShowResult {
    /// The Show's subscriber id
    pub show: Option<Show>,
}
