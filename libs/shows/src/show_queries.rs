use async_graphql::{Enum, InputObject, SimpleObject};
use caster_utils::{
    ordering::Ordering::{self, Asc, Desc},
    pagination::ManyResponse,
};

use crate::show_model::{self, Show};
use ShowsOrderBy::{
    CreatedAtAsc, CreatedAtDesc, IdAsc, IdDesc, TitleAsc, TitleDesc, UpdatedAtAsc, UpdatedAtDesc,
};

/// The `ShowsPage` result type
#[derive(Clone, Eq, PartialEq, SimpleObject)]
pub struct ShowsPage {
    /// The list of `Shows` returned for the current page
    data: Vec<Show>,

    /// The number of `Shows` returned for the current page
    count: usize,

    /// Tne total number of `Shows` available
    total: usize,

    /// The current page
    page: usize,

    /// The number of pages available
    page_count: usize,
}

impl From<ManyResponse<Show>> for ShowsPage {
    fn from(resp: ManyResponse<Show>) -> ShowsPage {
        ShowsPage {
            data: resp.data,
            count: resp.count,
            total: resp.total,
            page: resp.page,
            page_count: resp.page_count,
        }
    }
}

/// Conditions to filter Show listings by
#[derive(Clone, Eq, PartialEq, InputObject)]
pub struct ShowCondition {
    /// The `Show`'s title
    pub title: Option<String>,
}

/// The available ordering values
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ShowsOrderBy {
    /// Order ascending by "id"
    IdAsc,
    /// Order descending by "id"
    IdDesc,
    /// Order ascending by "displayName"
    TitleAsc,
    /// Order descending by "displayName"
    TitleDesc,
    /// Order ascending by "createdAt"
    CreatedAtAsc,
    /// Order descending by "createdAt"
    CreatedAtDesc,
    /// Order ascending by "updatedAt"
    UpdatedAtAsc,
    /// Order descending by "updatedAt"
    UpdatedAtDesc,
}

impl ShowsOrderBy {
    /// Retrieve the column name for the given ordering
    pub fn column(&self) -> show_model::Column {
        match self {
            IdAsc | IdDesc => show_model::Column::Id,
            TitleAsc | TitleDesc => show_model::Column::Title,
            CreatedAtAsc | CreatedAtDesc => show_model::Column::CreatedAt,
            UpdatedAtAsc | UpdatedAtDesc => show_model::Column::UpdatedAt,
        }
    }
}

impl From<ShowsOrderBy> for Ordering<ShowsOrderBy> {
    fn from(order_by: ShowsOrderBy) -> Ordering<ShowsOrderBy> {
        match order_by {
            IdAsc => Asc(IdAsc),
            TitleAsc => Asc(TitleAsc),
            CreatedAtAsc => Asc(CreatedAtAsc),
            UpdatedAtAsc => Asc(UpdatedAtAsc),
            IdDesc => Desc(IdDesc),
            TitleDesc => Desc(TitleDesc),
            CreatedAtDesc => Desc(CreatedAtDesc),
            UpdatedAtDesc => Desc(UpdatedAtDesc),
        }
    }
}
