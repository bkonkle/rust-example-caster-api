#![allow(missing_docs)]

use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::profile_model::{Model as ProfileModel, Profile};

/// The User GraphQL and Database Model
#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel, Deserialize, Serialize, SimpleObject)]
#[graphql(name = "User")]
#[sea_orm(table_name = "users")]
pub struct Model {
    /// The User id
    #[sea_orm(primary_key, column_type = "Text")]
    #[serde(skip_deserializing)]
    pub id: String,

    /// The date the User was created
    pub created_at: DateTime,

    /// The date the User was last updated
    pub updated_at: DateTime,

    /// The User's subscriber id
    #[sea_orm(column_type = "Text")]
    pub username: String,

    /// Whether the User is active or disabled
    pub is_active: bool,

    /// The related Profile, if one is associated
    #[sea_orm(ignore)]
    pub profile: Option<Profile>,
}

/// The User GraphQL type is the same as the database Model
pub type User = Model;

/// User entity relationships
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::profile_model::Entity")]
    Profile,
}

impl Related<super::profile_model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Profile.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// A wrapper around `Option<User>` to enable the trait implementations below
pub struct UserOption(pub Option<User>);

impl From<Option<Model>> for UserOption {
    fn from(data: Option<Model>) -> UserOption {
        UserOption(data)
    }
}

impl From<Option<(Model, Option<ProfileModel>)>> for UserOption {
    fn from(data: Option<(Model, Option<ProfileModel>)>) -> UserOption {
        UserOption(data.map(|(user, profile)| User {
            profile: profile.map(|p| p.into()),
            ..user
        }))
    }
}

#[allow(clippy::from_over_into)]
impl Into<Option<User>> for UserOption {
    fn into(self) -> Option<User> {
        self.0
    }
}
