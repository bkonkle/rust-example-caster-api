use serde_json;
use sqlx::types::chrono;

/// The Show model
#[derive(Clone, Eq, PartialEq)]
#[allow(dead_code, non_snake_case)]
pub struct Show {
    /// The Show id
    pub id: String,

    /// The Show title
    pub title: String,

    /// An optional Show summary
    pub summary: Option<String>,

    /// An optional Show image
    pub picture: Option<String>,

    /// Optional Json content for a Show
    pub content: Option<serde_json::Value>,

    /// The date the Show was created
    pub createdAt: chrono::NaiveDateTime,

    /// The date the Show was last updated
    pub updatedAt: chrono::NaiveDateTime,
}
