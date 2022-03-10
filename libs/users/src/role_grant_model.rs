#![allow(missing_docs)]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::user_model;

/// The `RoleGrant` GraphQL and Database Model
#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "role_grants")]
pub struct Model {
    /// The RoleGrant id
    #[sea_orm(primary_key, column_type = "Text")]
    #[serde(skip_deserializing)]
    pub id: String,

    /// The date the RoleGrant was created
    pub created_at: DateTime,

    /// The date the RoleGrant was last updated
    pub updated_at: DateTime,

    /// The key of the Role being granted to the Role
    #[sea_orm(column_type = "Text")]
    pub role_key: String,

    /// The User id that the Role is being granted to
    #[sea_orm(column_type = "Text")]
    pub user_id: String,

    /// The table of the resource that the Role is being granted for
    #[sea_orm(column_type = "Text")]
    pub resource_table: String,

    /// The id of the resource that the Role is being granted for
    #[sea_orm(column_type = "Text")]
    pub resource_id: String,
}

/// The `RoleGrant` GraphQL type is the same as the database Model
pub type RoleGrant = Model;

/// `RoleGrant` entity relationships
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "user_model::Entity",
        from = "Column::UserId",
        to = "user_model::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    User,
}

impl Related<user_model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// A struct representing a granted Role
pub struct Role {
    pub key: String,
    pub resource_table: String,
    pub resource_id: String,
}

impl From<Model> for Role {
    fn from(model: Model) -> Self {
        Self {
            key: model.role_key,
            resource_table: model.resource_table,
            resource_id: model.resource_id,
        }
    }
}

/// A wrapper around a `Vec<Role` to enable trait implementations
pub struct RoleList(Vec<Role>);

impl RoleList {
    /// Proxy to the `Vec` `len` method
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Proxy to the `Vec` `is_empty` method
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<Vec<Model>> for RoleList {
    fn from(data: Vec<Model>) -> RoleList {
        RoleList(data.into_iter().map(|p| p.into()).collect())
    }
}

#[allow(clippy::from_over_into)]
impl Into<Vec<Role>> for RoleList {
    fn into(self) -> Vec<Role> {
        self.0
    }
}
