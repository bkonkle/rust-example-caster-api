#![allow(missing_docs)]
use async_graphql::SimpleObject;
use oso::PolarClass;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::show_model::Show;

/// The Episode GraphQL model
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, PolarClass, Serialize, SimpleObject)]
pub struct Episode {
    /// The Episode id
    #[polar(attribute)]
    pub id: String,

    /// The date the Episode was created
    pub created_at: DateTime,

    /// The date the Episode was last updated
    pub updated_at: DateTime,

    /// The Episode title
    pub title: String,

    /// An optional Episode summary
    pub summary: Option<String>,

    /// An optional Episode image
    pub picture: Option<String>,

    /// Optional Json content for a Episode
    pub content: Option<Json>,

    /// The Episode's Show id
    #[polar(attribute)]
    pub show_id: String,

    /// The associated Show
    #[polar(attribute)]
    pub show: Option<Show>,
}

/// The Profile Database model
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "episodes")]
pub struct Model {
    /// The Episode id
    #[sea_orm(primary_key, column_type = "Text")]
    #[serde(skip_deserializing)]
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
    pub show_id: String,
}

/// Episode entity relationships
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::show_model::Entity",
        from = "Column::ShowId",
        to = "super::show_model::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Show,
}

impl Related<super::show_model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Show.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
