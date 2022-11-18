use async_graphql::{Enum, InputObject, SimpleObject};

use super::model::{self, Episode};
use caster_utils::{
    ordering::Ordering::{self, Asc, Desc},
    pagination::ManyResponse,
};

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
    count: u64,

    /// Tne total number of `Episodes` available
    total: u64,

    /// The current page
    page: u64,

    /// The number of pages available
    page_count: u64,
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

    /// The associated Show
    pub show_id: Option<String>,

    /// Filter by IDs
    pub ids_in: Option<Vec<String>>,
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

impl From<EpisodesOrderBy> for Ordering<model::Column> {
    fn from(order_by: EpisodesOrderBy) -> Ordering<model::Column> {
        match order_by {
            IdAsc => Asc(model::Column::Id),
            TitleAsc => Asc(model::Column::Title),
            ShowIdAsc => Asc(model::Column::ShowId),
            CreatedAtAsc => Asc(model::Column::CreatedAt),
            UpdatedAtAsc => Asc(model::Column::UpdatedAt),
            IdDesc => Desc(model::Column::Id),
            TitleDesc => Desc(model::Column::Title),
            ShowIdDesc => Desc(model::Column::ShowId),
            CreatedAtDesc => Desc(model::Column::CreatedAt),
            UpdatedAtDesc => Desc(model::Column::UpdatedAt),
        }
    }
}
