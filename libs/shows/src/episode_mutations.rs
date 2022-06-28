use async_graphql::{InputObject, SimpleObject};
use serde_json::Value as Json;

use crate::episode_model::Episode;

/// The `CreateEpisodeInput` input type
#[derive(Clone, Default, Eq, PartialEq, InputObject)]
pub struct CreateEpisodeInput {
    /// The Episode's title
    pub title: String,

    /// The Episode's description summary
    pub summary: Option<String>,

    /// The Episode's picture
    pub picture: Option<String>,

    /// The Episode json content
    pub content: Option<Json>,

    /// The Episode's Show id
    pub show_id: String,
}

/// The `UpdateEpisodeInput` input type
#[derive(Clone, Default, Eq, PartialEq, InputObject)]
pub struct UpdateEpisodeInput {
    /// The Episode's title
    pub title: Option<String>,

    /// The Episode's description summary
    pub summary: Option<String>,

    /// The Episode's picture
    pub picture: Option<String>,

    /// The Episode json content
    pub content: Option<Json>,

    /// The Episode's Show id
    pub show_id: Option<String>,
}

/// The `MutateEpisodeResult` type
#[derive(Clone, Default, Eq, PartialEq, SimpleObject)]
pub struct MutateEpisodeResult {
    /// The Episode's subscriber id
    pub episode: Option<Episode>,
}
