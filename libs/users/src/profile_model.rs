use async_graphql::SimpleObject;
use chrono::NaiveDateTime;

/// The Profile model
#[derive(Debug, Clone, Eq, PartialEq, SimpleObject)]
pub struct Profile {
    /// The Profile id
    pub id: String,

    /// The Profile's email address
    // Optional because this field may be censored for unauthorized users
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

/// The Profile DB model
#[allow(missing_docs)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProfileDB {
    pub id: String,
    pub email: String,
    pub display_name: Option<String>,
    pub picture: Option<String>,
    pub content: Option<serde_json::Value>,
    pub city: Option<String>,
    pub state_province: Option<String>,
    pub user_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<ProfileDB> for Profile {
    fn from(item: ProfileDB) -> Self {
        Profile {
            id: item.id,
            email: Some(item.email),
            display_name: item.display_name,
            picture: item.picture,
            content: item.content,
            city: item.city,
            state_province: item.state_province,
            user_id: item.user_id,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}
