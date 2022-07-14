#![allow(missing_docs)]

use async_graphql::SimpleObject;
use chrono::Utc;
use oso::PolarClass;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::show_model::{self, Show};

/// The User GraphQL and Database Model
#[derive(
    Clone, Debug, Eq, PartialEq, DeriveEntityModel, Deserialize, Serialize, SimpleObject, PolarClass,
)]
#[graphql(complex)]
#[graphql(name = "Episode")]
#[sea_orm(table_name = "episodes")]
pub struct Model {
    /// The Episode id
    #[sea_orm(primary_key, column_type = "Text")]
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
    #[graphql(skip)]
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
        to = "show_model::Column::Id"
    )]
    Show,
}

impl Related<show_model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Show.def()
    }
}

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
            content: Option::default(),
            show_id: String::default(),
            show: Option::default(),
        }
    }
}

/// A wrapper around a `Vec<Episode` to enable trait implementations
pub struct EpisodeList(Vec<Episode>);

impl EpisodeList {
    /// Proxy to the `Vec` `len` method
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Proxy to the `Vec` `is_empty` method
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<Vec<Model>> for EpisodeList {
    fn from(data: Vec<Model>) -> EpisodeList {
        EpisodeList(data.into_iter().collect())
    }
}

impl From<Vec<(Model, Option<Show>)>> for EpisodeList {
    fn from(data: Vec<(Model, Option<Show>)>) -> EpisodeList {
        EpisodeList(
            data.into_iter()
                .map(|(episode, show)| Episode { show, ..episode })
                .collect(),
        )
    }
}

impl From<EpisodeList> for Vec<Episode> {
    fn from(episodes: EpisodeList) -> Vec<Episode> {
        episodes.0
    }
}

/// A wrapper around `Option<Episode>` to enable trait implementations
pub struct EpisodeOption(pub Option<Episode>);

impl From<Option<Model>> for EpisodeOption {
    fn from(data: Option<Model>) -> EpisodeOption {
        EpisodeOption(data)
    }
}

impl From<Option<(Model, Option<Show>)>> for EpisodeOption {
    fn from(data: Option<(Model, Option<Show>)>) -> EpisodeOption {
        EpisodeOption(data.map(|(episode, show)| Episode { show, ..episode }))
    }
}

impl From<EpisodeOption> for Option<Episode> {
    fn from(episode: EpisodeOption) -> Option<Episode> {
        episode.0
    }
}
