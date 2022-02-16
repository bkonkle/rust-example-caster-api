/// A paginated response for an entity
pub struct ManyResponse<Model> {
    /// The page of data being returned
    pub data: Vec<Model>,
    /// The number of rows returned in the current page
    pub count: usize,
    /// The total number of rows available
    pub total: usize,
    /// The current page being returned
    pub page: usize,
    /// The number of pages available
    pub page_count: usize,
}

impl<Model> ManyResponse<Model> {
    /// Given a Sea-Orm `Paginator`, create a new `ManyResponse`
    pub fn new(
        data: Vec<Model>,
        total: usize,
        page: usize,
        page_size: Option<usize>,
    ) -> ManyResponse<Model> {
        let count = data.len();
        let page_count = page_size.map(|page_size| total / page_size);

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
