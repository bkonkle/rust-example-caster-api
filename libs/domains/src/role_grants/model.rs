#![allow(missing_docs)]

use async_graphql::SimpleObject;
use chrono::Utc;
use fake::{Dummy, Fake};
use oso::PolarClass;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::users::model as user_model;

/// The `RoleGrant` GraphQL and Database Model
#[derive(
    Clone,
    Debug,
    Dummy,
    Eq,
    PartialEq,
    DeriveEntityModel,
    Deserialize,
    Serialize,
    SimpleObject,
    PolarClass,
)]
#[sea_orm(table_name = "role_grants")]
pub struct Model {
    /// The RoleGrant id
    #[sea_orm(primary_key, column_type = "Text")]
    #[polar(attribute)]
    pub id: String,

    /// The date the RoleGrant was created
    pub created_at: DateTime,

    /// The date the RoleGrant was last updated
    pub updated_at: DateTime,

    /// The key of the Role being granted to the Role
    #[sea_orm(column_type = "Text")]
    #[polar(attribute)]
    pub role_key: String,

    /// The User id that the Role is being granted to
    #[sea_orm(column_type = "Text")]
    #[polar(attribute)]
    pub user_id: String,

    /// The table of the resource that the Role is being granted for
    #[sea_orm(column_type = "Text")]
    #[polar(attribute)]
    pub resource_table: String,

    /// The id of the resource that the Role is being granted for
    #[sea_orm(column_type = "Text")]
    #[polar(attribute)]
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

impl Default for Model {
    fn default() -> Self {
        Self {
            id: String::default(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            role_key: String::default(),
            user_id: String::default(),
            resource_table: String::default(),
            resource_id: String::default(),
        }
    }
}

/// The `CreateRoleGrantInput` type
#[derive(Clone, Eq, Dummy, PartialEq)]
pub struct CreateRoleGrantInput {
    /// The key of the role to grant
    pub role_key: String,

    /// The `User` id to grant the role to
    pub user_id: String,

    /// The table of the resource to grant the role for
    pub resource_table: String,

    /// The id of the resource to grant the role for
    pub resource_id: String,
}

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

impl From<RoleList> for Vec<Role> {
    fn from(roles: RoleList) -> Vec<Role> {
        roles.0
    }
}
