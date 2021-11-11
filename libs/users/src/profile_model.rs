use async_graphql::SimpleObject;
use chrono::NaiveDateTime;

/// The Profile model
#[derive(Clone, Eq, PartialEq, SimpleObject)]
pub struct Profile {
    /// The Profile id
    pub id: String,

    /// Nullable because this field may be censored for unauthorized users
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

    /// The date the Profile was created
    pub created_at: NaiveDateTime,

    /// The date the Profile was last updated
    pub updated_at: NaiveDateTime,
}
