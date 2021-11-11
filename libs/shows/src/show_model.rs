use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use serde_json;

/// The Show model
#[derive(Clone, Eq, PartialEq, SimpleObject)]
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
    pub created_at: NaiveDateTime,

    /// The date the Show was last updated
    pub updated_at: NaiveDateTime,
}
