use anyhow::Result;
use async_graphql::{
    dataloader::Loader,
    FieldError,
    MaybeUndefined::{Null, Undefined, Value},
};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sea_orm::{entity::*, query::*, DatabaseConnection, EntityTrait};
use std::{collections::HashMap, sync::Arc};

use crate::shows::{
    model::{self, Show},
    mutations::{CreateShowInput, UpdateShowInput},
    queries::{ShowCondition, ShowsOrderBy},
};
use caster_utils::{ordering::Ordering, pagination::ManyResponse};

/// A ShowsService applies business logic to a dynamic ShowsRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait ShowsService: Sync + Send {
    /// Get an individual `Show` by id
    async fn get(&self, id: &str) -> Result<Option<Show>>;

    /// Get a list of `Show` results matching the given ids
    async fn get_by_ids(&self, ids: Vec<String>) -> Result<Vec<Show>>;

    /// Get multiple `Show` records
    async fn get_many(
        &self,
        condition: Option<ShowCondition>,
        order_by: Option<Vec<ShowsOrderBy>>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<ManyResponse<Show>>;

    /// Create a `Show` with the given input
    async fn create(&self, input: &CreateShowInput) -> Result<Show>;

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
    pub fn new(db: &Arc<DatabaseConnection>) -> Self {
        Self { db: db.clone() }
    }
}

#[async_trait]
impl ShowsService for DefaultShowsService {
    async fn get(&self, id: &str) -> Result<Option<model::Model>> {
        let query = model::Entity::find_by_id(id.to_owned());

        let show = query.one(&*self.db).await?;

        Ok(show)
    }

    async fn get_by_ids(&self, ids: Vec<String>) -> Result<Vec<Show>> {
        let mut condition = Condition::any();

        for id in ids {
            condition = condition.add(model::Column::Id.eq(id.clone()));
        }

        let shows = model::Entity::find()
            .filter(condition)
            .all(&*self.db)
            .await?;

        Ok(shows)
    }

    async fn get_many(
        &self,
        condition: Option<ShowCondition>,
        order_by: Option<Vec<ShowsOrderBy>>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<ManyResponse<Show>> {
        let page_num = page.unwrap_or(1);

        let mut query = model::Entity::find();

        if let Some(condition) = condition {
            if let Some(title) = condition.title {
                query = query.filter(model::Column::Title.eq(title));
            }
        }

        if let Some(order_by) = order_by {
            for order in order_by {
                let ordering: Ordering<model::Column> = order.into();

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
            let total = data.len().try_into().unwrap_or(0);

            (data, total)
        };

        Ok(ManyResponse::new(data, total, page_num, page_size))
    }

    async fn create(&self, input: &CreateShowInput) -> Result<Show> {
        let show = model::ActiveModel {
            title: Set(input.title.clone()),
            summary: Set(input.summary.clone()),
            picture: Set(input.picture.clone()),
            ..Default::default()
        }
        .insert(&*self.db)
        .await?;

        let created: Show = show;

        return Ok(created);
    }

    async fn update(&self, id: &str, input: &UpdateShowInput) -> Result<Show> {
        let query = model::Entity::find_by_id(id.to_owned());

        // Retrieve the existing Show
        let show = query
            .one(&*self.db)
            .await?
            .ok_or_else(|| anyhow!("Unable to find Show with id: {}", id))?;

        let mut show: model::ActiveModel = show.into();

        match &input.title {
            Undefined | Null => (),
            Value(value) => show.title = Set(value.clone()),
        };

        match &input.summary {
            Undefined => (),
            Null => show.summary = Set(None),
            Value(value) => show.summary = Set(Some(value.clone())),
        }

        match &input.picture {
            Undefined => (),
            Null => show.picture = Set(None),
            Value(value) => show.picture = Set(Some(value.clone())),
        }

        let updated: Show = show.update(&*self.db).await?;

        Ok(updated)
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let show = model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?
            .ok_or_else(|| anyhow!("Unable to find Show with id: {}", id))?;

        let _result = show.delete(&*self.db).await?;

        Ok(())
    }
}

/// A dataloader for `Show` instances
pub struct ShowLoader {
    /// The SeaOrm database connection
    shows: Arc<dyn ShowsService>,
}

/// The default implementation for the `ShowLoader`
impl ShowLoader {
    /// Create a new instance
    pub fn new(shows: &Arc<dyn ShowsService>) -> Self {
        Self {
            shows: shows.clone(),
        }
    }
}

#[async_trait]
impl Loader<String> for ShowLoader {
    type Value = Show;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let shows = self.shows.get_by_ids(keys.into()).await?;

        Ok(shows
            .into_iter()
            .map(|show| (show.id.clone(), show))
            .collect())
    }
}
