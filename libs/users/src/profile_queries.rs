use async_graphql::{Enum, InputObject, SimpleObject};
use caster_utils::{
    ordering::Ordering::{self, Asc, Desc},
    pagination::ManyResponse,
};

use crate::profile_model::{self, Profile};
use ProfilesOrderBy::{
    CreatedAtAsc, CreatedAtDesc, DisplayNameAsc, DisplayNameDesc, EmailAsc, EmailDesc, IdAsc,
    IdDesc, UpdatedAtAsc, UpdatedAtDesc,
};

/// The `ProfilesPage` result type
#[derive(Clone, Eq, PartialEq, SimpleObject)]
pub struct ProfilesPage {
    /// The list of `Profiles` returned for the current page
    data: Vec<Profile>,

    /// The number of `Profiles` returned for the current page
    count: usize,

    /// Tne total number of `Profiles` available
    total: usize,

    /// The current page
    page: usize,

    /// The number of pages available
    page_count: usize,
}

impl From<ManyResponse<Profile>> for ProfilesPage {
    fn from(resp: ManyResponse<Profile>) -> ProfilesPage {
        ProfilesPage {
            data: resp.data,
            count: resp.count,
            total: resp.total,
            page: resp.page,
            page_count: resp.page_count,
        }
    }
}

/// Conditions to filter Profile listings by
#[derive(Clone, Eq, PartialEq, InputObject)]
pub struct ProfileCondition {
    /// The `Profile`'s email address
    pub email: Option<String>,

    /// The `Profile`'s display name
    pub display_name: Option<String>,

    /// The `Profile`'s city
    pub city: Option<String>,

    /// The `Profile`'s state or province
    pub state_province: Option<String>,

    /// The `Profile`'s User id
    pub user_id: Option<String>,
}

/// The available ordering values
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ProfilesOrderBy {
    /// Order ascending by "id"
    IdAsc,
    /// Order descending by "id"
    IdDesc,
    /// Order ascending by "email"
    EmailAsc,
    /// Order descending by "email"
    EmailDesc,
    /// Order ascending by "displayName"
    DisplayNameAsc,
    /// Order descending by "displayName"
    DisplayNameDesc,
    /// Order ascending by "createdAt"
    CreatedAtAsc,
    /// Order descending by "createdAt"
    CreatedAtDesc,
    /// Order ascending by "updatedAt"
    UpdatedAtAsc,
    /// Order descending by "updatedAt"
    UpdatedAtDesc,
}

impl From<ProfilesOrderBy> for Ordering<profile_model::Column> {
    fn from(order_by: ProfilesOrderBy) -> Ordering<profile_model::Column> {
        match order_by {
            IdAsc => Asc(profile_model::Column::Id),
            EmailAsc => Asc(profile_model::Column::Email),
            DisplayNameAsc => Asc(profile_model::Column::DisplayName),
            CreatedAtAsc => Asc(profile_model::Column::CreatedAt),
            UpdatedAtAsc => Asc(profile_model::Column::UpdatedAt),
            IdDesc => Desc(profile_model::Column::Id),
            EmailDesc => Desc(profile_model::Column::Email),
            DisplayNameDesc => Desc(profile_model::Column::DisplayName),
            CreatedAtDesc => Desc(profile_model::Column::CreatedAt),
            UpdatedAtDesc => Desc(profile_model::Column::UpdatedAt),
        }
    }
}
