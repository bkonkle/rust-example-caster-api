#![allow(missing_docs)]
use async_graphql::SimpleObject;
use oso::PolarClass;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::show_model::{self, Show};

/// The User GraphQL and Database Model
#[derive(
    Clone, Debug, Eq, PartialEq, DeriveEntityModel, Deserialize, Serialize, SimpleObject, PolarClass,
)]
#[graphql(name = "Episode")]
#[sea_orm(table_name = "episodes")]
pub struct Model {
    /// The Episode id
    #[sea_orm(primary_key, column_type = "Text")]
    #[serde(skip_deserializing)]
    #[polar(attribute)]
    pub id: String,

    /// The date the Episode was created
    pub created_at: DateTime,

    /// The date the Episode was last updated
    pub updated_at: DateTime,

    /// The Episode title
    #[sea_orm(column_type = "Text")]
    pub title: String,

    /// An optional Episode summary
    #[sea_orm(column_type = "Text", nullable)]
    pub summary: Option<String>,

    /// An optional Episode image
    #[sea_orm(column_type = "Text", nullable)]
    pub picture: Option<String>,

    /// Optional Json content for a Episode
    #[sea_orm(nullable)]
    pub content: Option<Json>,

    /// The Episode's Show id
    #[polar(attribute)]
    pub show_id: String,

    /// The associated Show
    #[sea_orm(ignore)]
    #[polar(attribute)]
    pub show: Option<Show>,
}

/// The Episode GraphQL type is the same as the database Model
pub type Episode = Model;

/// Episode entity relationships
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "show_model::Entity",
        from = "Column::ShowId",
        to = "show_model::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Show,
}

impl Related<show_model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Show.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
