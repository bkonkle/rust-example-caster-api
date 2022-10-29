use serde::{Deserialize, Serialize};

/// A paginated response for an entity
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ManyResponse<Model> {
    /// The page of data being returned
    pub data: Vec<Model>,
    /// The number of rows returned in the current page
    pub count: u64,
    /// The total number of rows available
    pub total: u64,
    /// The current page being returned
    pub page: u64,
    /// The number of pages available
    pub page_count: u64,
}

impl<Model> ManyResponse<Model> {
    /// Given a Sea-Orm `Paginator`, create a new `ManyResponse`
    pub fn new(
        data: Vec<Model>,
        total: u64,
        page: u64,
        page_size: Option<u64>,
    ) -> ManyResponse<Model> {
        let count = data.len().try_into().unwrap_or(0);
        let page_count = page_size
            .map(|page_size| total / page_size + if total % page_size != 0 { 1 } else { 0 });

        Self {
            data,
            count,
            total,
            page,
            page_count: page_count.unwrap_or(page),
        }
    }

    /// Transform the data contained in the `ManyResponse`
    pub fn map<B, F>(self, func: F) -> ManyResponse<B>
    where
        F: Fn(Model) -> B,
    {
        ManyResponse {
            data: self.data.into_iter().map(func).collect(),
            count: self.count,
            total: self.total,
            page: self.page,
            page_count: self.page_count,
        }
    }
}
