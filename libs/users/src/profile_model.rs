#![allow(missing_docs)]

use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// The Profile model
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, SimpleObject)]
pub struct Profile {
    /// The Profile id
    pub id: String,

    /// The date the Profile was created
    pub created_at: DateTime,

    /// The date the Profile was last updated
    pub updated_at: DateTime,

    /// The Profile's email address
    // (differs from DB)
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

/// The Profile DB model
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "profiles")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Text")]
    #[serde(skip_deserializing)]
    pub id: String,

    pub created_at: DateTime,

    pub updated_at: DateTime,

    #[sea_orm(column_type = "Text")]
    pub email: String,

    #[sea_orm(column_type = "Text", nullable)]
    pub display_name: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub picture: Option<String>,

    #[sea_orm(nullable)]
    pub content: Option<Json>,

    #[sea_orm(column_type = "Text", nullable)]
    pub city: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub state_province: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub user_id: Option<String>,
}

/// Profile entity relationships
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user_model::Entity",
        from = "Column::UserId",
        to = "super::user_model::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::user_model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for Profile {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            updated_at: model.updated_at,
            email: Some(model.email),
            display_name: model.display_name,
            picture: model.picture,
            content: model.content,
            city: model.city,
            state_province: model.state_province,
            user_id: model.user_id,
        }
    }
}
