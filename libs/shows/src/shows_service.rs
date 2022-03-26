use anyhow::Result;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sea_orm::{entity::*, query::*, DatabaseConnection, EntityTrait};
use std::sync::Arc;

use crate::{
    show_model::{self, Show},
    show_mutations::{CreateShowInput, UpdateShowInput},
    show_queries::{ShowCondition, ShowsOrderBy},
};
use caster_utils::{ordering::Ordering, pagination::ManyResponse};

/// A ShowsService applies business logic to a dynamic ShowsRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait ShowsService: Sync + Send {
    /// Get an individual `Show` by id, returning the Model instance for updating
    async fn get_model(&self, id: &str) -> Result<Option<show_model::Model>>;

    /// Get an individual `Show` by id
    async fn get(&self, id: &str) -> Result<Option<Show>>;

    /// Get multiple `Show` records
    async fn get_many(
        &self,
        condition: Option<ShowCondition>,
        order_by: Option<Vec<ShowsOrderBy>>,
        page: Option<usize>,
        page_size: Option<usize>,
    ) -> Result<ManyResponse<Show>>;

    /// Create a `Show` with the given input
    async fn create(&self, input: &CreateShowInput) -> Result<Show>;

    /// Update an existing `Show` using a retrieved `Model` instance
    async fn update_model(&self, show: show_model::Model, input: &UpdateShowInput) -> Result<Show>;

    /// Update an existing `Show` by id
    async fn update(&self, id: &str, input: &UpdateShowInput) -> Result<Show>;

    /// Delete an existing `Show`
    async fn delete(&self, id: &str) -> Result<()>;
}

/// The default `ShowsService` struct.
pub struct DefaultShowsService {
    /// The SeaOrm database connection
    db: Arc<DatabaseConnection>,
}

/// The default `ShowsService` implementation
impl DefaultShowsService {
    /// Create a new `ShowsService` instance
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ShowsService for DefaultShowsService {
    async fn get_model(&self, id: &str) -> Result<Option<show_model::Model>> {
        let query = show_model::Entity::find_by_id(id.to_owned());

        let show = query.one(&*self.db).await?;

        Ok(show)
    }

    async fn get(&self, id: &str) -> Result<Option<Show>> {
        let show = self.get_model(id).await?;

        Ok(show)
    }

    async fn get_many(
        &self,
        condition: Option<ShowCondition>,
        order_by: Option<Vec<ShowsOrderBy>>,
        page: Option<usize>,
        page_size: Option<usize>,
    ) -> Result<ManyResponse<Show>> {
        let page_num = page.unwrap_or(1);

        let mut query = show_model::Entity::find();

        if let Some(condition) = condition {
            if let Some(title) = condition.title {
                query = query.filter(show_model::Column::Title.eq(title));
            }
        }

        if let Some(order_by) = order_by {
            for order in order_by {
                let ordering: Ordering<show_model::Column> = order.into();

                match ordering {
                    Ordering::Asc(column) => {
                        query = query.order_by_asc(column);
                    }
                    Ordering::Desc(column) => {
                        query = query.order_by_desc(column);
                    }
                }
            }
        }

        let (data, total) = if let Some(page_size) = page_size {
            let paginator = query.paginate(&*self.db, page_size);
            let total = paginator.num_items().await?;
            let data: Vec<Show> = paginator.fetch_page(page_num - 1).await?;

            (data, total)
        } else {
            let data: Vec<Show> = query.all(&*self.db).await?;
            let total = data.len();

            (data, total)
        };

        Ok(ManyResponse::new(data, total, page_num, page_size))
    }

    async fn create(&self, input: &CreateShowInput) -> Result<Show> {
        let show = show_model::ActiveModel {
            title: Set(input.title.clone()),
            summary: Set(input.summary.clone()),
            picture: Set(input.picture.clone()),
            content: Set(input.content.clone()),
            ..Default::default()
        }
        .insert(&*self.db)
        .await?;

        let created: Show = show;

        return Ok(created);
    }

    async fn update_model(&self, show: show_model::Model, input: &UpdateShowInput) -> Result<Show> {
        let mut show: show_model::ActiveModel = show.into();

        if let Some(title) = &input.title {
            show.title = Set(title.clone());
        }

        if let Some(summary) = &input.summary {
            show.summary = Set(Some(summary.clone()));
        }

        if let Some(picture) = &input.picture {
            show.picture = Set(Some(picture.clone()));
        }

        if let Some(content) = &input.content {
            show.content = Set(Some(content.clone()));
        }

        let updated: Show = show.update(&*self.db).await?;

        Ok(updated)
    }

    async fn update(&self, id: &str, input: &UpdateShowInput) -> Result<Show> {
        let query = show_model::Entity::find_by_id(id.to_owned());

        // Retrieve the existing Show
        let show = query
            .one(&*self.db)
            .await?
            .ok_or_else(|| anyhow!("Unable to find Show with id: {}", id))?;

        self.update_model(show, input).await
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let show = show_model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?
            .ok_or_else(|| anyhow!("Unable to find Show with id: {}", id))?;

        let _result = show.delete(&*self.db).await?;

        Ok(())
    }
}
