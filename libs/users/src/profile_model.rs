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

impl Profile {
    /// If not authorized, censor the Profile email
    pub fn censor(&self, current_user_id: Option<String>) -> Self {
        let mut profile = self.clone();

        let same_user = match current_user_id {
            Some(user_id) => self.user_id == Some(user_id),
            _ => false,
        };

        // If not same user, censor the email
        // TODO: Allow users with an admin role to read this regardless
        profile.email = if same_user { self.email.clone() } else { None };

        profile
    }
}

impl From<ProfileDB> for Profile {
    fn from(profile: ProfileDB) -> Self {
        Self {
            id: profile.id,
            email: Some(profile.email),
            display_name: profile.display_name,
            picture: profile.picture,
            content: profile.content,
            city: profile.city,
            state_province: profile.state_province,
            user_id: profile.user_id,
            created_at: profile.created_at,
            updated_at: profile.updated_at,
        }
    }
}
