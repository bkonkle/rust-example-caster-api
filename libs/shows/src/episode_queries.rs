use async_graphql::{Enum, InputObject, SimpleObject};
use caster_utils::{
    ordering::Ordering::{self, Asc, Desc},
    pagination::ManyResponse,
};

use crate::episode_model::{self, Episode};
use EpisodesOrderBy::{
    CreatedAtAsc, CreatedAtDesc, IdAsc, IdDesc, ShowIdAsc, ShowIdDesc, TitleAsc, TitleDesc,
    UpdatedAtAsc, UpdatedAtDesc,
};

/// The `EpisodesPage` result type
#[derive(Clone, Eq, PartialEq, SimpleObject)]
pub struct EpisodesPage {
    /// The list of `Episodes` returned for the current page
    data: Vec<Episode>,

    /// The number of `Episodes` returned for the current page
    count: usize,

    /// Tne total number of `Episodes` available
    total: usize,

    /// The current page
    page: usize,

    /// The number of pages available
    page_count: usize,
}

impl From<ManyResponse<Episode>> for EpisodesPage {
    fn from(resp: ManyResponse<Episode>) -> EpisodesPage {
        EpisodesPage {
            data: resp.data,
            count: resp.count,
            total: resp.total,
            page: resp.page,
            page_count: resp.page_count,
        }
    }
}

/// Conditions to filter Episode listings by
#[derive(Clone, Eq, PartialEq, InputObject)]
pub struct EpisodeCondition {
    /// The `Episode`'s title
    pub title: Option<String>,
}

/// The available ordering values
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum EpisodesOrderBy {
    /// Order ascending by "id"
    IdAsc,
    /// Order descending by "id"
    IdDesc,
    /// Order ascending by "displayName"
    TitleAsc,
    /// Order descending by "displayName"
    TitleDesc,
    /// Order ascending by "showId"
    ShowIdAsc,
    /// Order descending by "showId"
    ShowIdDesc,
    /// Order ascending by "createdAt"
    CreatedAtAsc,
    /// Order descending by "createdAt"
    CreatedAtDesc,
    /// Order ascending by "updatedAt"
    UpdatedAtAsc,
    /// Order descending by "updatedAt"
    UpdatedAtDesc,
}

impl From<EpisodesOrderBy> for Ordering<episode_model::Column> {
    fn from(order_by: EpisodesOrderBy) -> Ordering<episode_model::Column> {
        match order_by {
            IdAsc => Asc(episode_model::Column::Id),
            TitleAsc => Asc(episode_model::Column::Title),
            ShowIdAsc => Asc(episode_model::Column::ShowId),
            CreatedAtAsc => Asc(episode_model::Column::CreatedAt),
            UpdatedAtAsc => Asc(episode_model::Column::UpdatedAt),
            IdDesc => Desc(episode_model::Column::Id),
            TitleDesc => Desc(episode_model::Column::Title),
            ShowIdDesc => Desc(episode_model::Column::ShowId),
            CreatedAtDesc => Desc(episode_model::Column::CreatedAt),
            UpdatedAtDesc => Desc(episode_model::Column::UpdatedAt),
        }
    }
}
