#![allow(missing_docs)]

use async_graphql::SimpleObject;
use chrono::Utc;
use fake::{Dummy, Fake};
use oso::PolarClass;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// The Show GraphQL and Database Model
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
#[graphql(name = "Show")]
#[sea_orm(table_name = "shows")]
pub struct Model {
    /// The Show id
    #[sea_orm(primary_key, column_type = "Text")]
    #[polar(attribute)]
    pub id: String,

    /// The date the Show was created
    pub created_at: DateTime,

    /// The date the Show was last updated
    pub updated_at: DateTime,

    /// The Show title
    #[sea_orm(column_type = "Text")]
    pub title: String,

    /// An optional Show summary
    #[sea_orm(column_type = "Text", nullable)]
    pub summary: Option<String>,

    /// An optional Show image
    #[sea_orm(column_type = "Text", nullable)]
    pub picture: Option<String>,
}

/// The Show GraphQL type is the same as the database Model
pub type Show = Model;

/// Show entity relationships
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Default for Model {
    fn default() -> Self {
        Self {
            id: String::default(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            title: String::default(),
            summary: Option::default(),
            picture: Option::default(),
        }
    }
}
