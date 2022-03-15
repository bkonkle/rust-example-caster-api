use anyhow::Result;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sea_orm::{entity::*, query::*, DatabaseConnection, EntityTrait};
use std::sync::Arc;

use crate::{
    episode_model::{self, Episode},
    episode_mutations::{CreateEpisodeInput, UpdateEpisodeInput},
    episode_queries::{EpisodeCondition, EpisodesOrderBy},
};
use caster_utils::{ordering::Ordering, pagination::ManyResponse};

/// An EpisodesService applies business logic to a dynamic EpisodesRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait EpisodesService: Sync + Send {
    /// Get an individual `Episode` by id, returning the Model instance for updating
    async fn get_model(&self, id: &str) -> Result<Option<episode_model::Model>>;

    /// Get an individual `Episode` by id
    async fn get(&self, id: &str) -> Result<Option<Episode>>;

    /// Get multiple `Episode` records
    async fn get_many(
        &self,
        condition: Option<EpisodeCondition>,
        order_by: Option<Vec<EpisodesOrderBy>>,
        page_size: Option<usize>,
        page: Option<usize>,
    ) -> Result<ManyResponse<Episode>>;

    /// Create a `Episode` with the given input
    async fn create(&self, input: &CreateEpisodeInput) -> Result<Episode>;

    /// Update an existing `Episode` using a retrieved `Model` instance
    async fn update_model(
        &self,
        episode: episode_model::Model,
        input: &UpdateEpisodeInput,
    ) -> Result<Episode>;

    /// Update an existing `Episode` by id
    async fn update(&self, id: &str, input: &UpdateEpisodeInput) -> Result<Episode>;

    /// Delete an existing `Episode`
    async fn delete(&self, id: &str) -> Result<()>;
}

/// The default `EpisodesService` struct.
pub struct DefaultEpisodesService {
    /// The SeaOrm database connection
    db: Arc<DatabaseConnection>,
}

/// The default `EpisodesService` implementation
impl DefaultEpisodesService {
    /// Create a new `EpisodesService` instance
    pub fn new(db: &Arc<DatabaseConnection>) -> Self {
        Self { db: db.clone() }
    }
}

#[async_trait]
impl EpisodesService for DefaultEpisodesService {
    async fn get_model(&self, id: &str) -> Result<Option<episode_model::Model>> {
        let query = episode_model::Entity::find_by_id(id.to_owned());

        let episode = query.one(&*self.db).await?;

        Ok(episode)
    }

    async fn get(&self, id: &str) -> Result<Option<Episode>> {
        let episode = self.get_model(id).await?;

        Ok(episode)
    }

    async fn get_many(
        &self,
        condition: Option<EpisodeCondition>,
        order_by: Option<Vec<EpisodesOrderBy>>,
        page: Option<usize>,
        page_size: Option<usize>,
    ) -> Result<ManyResponse<Episode>> {
        let page_num = page.unwrap_or(1);

        let mut query = episode_model::Entity::find();

        if let Some(condition) = condition {
            if let Some(title) = condition.title {
                query = query.filter(episode_model::Column::Title.eq(title));
            }
        }

        if let Some(order_by) = order_by {
            for order in order_by {
                let ordering: Ordering<episode_model::Column> = order.into();

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
            let data: Vec<Episode> = paginator.fetch_page(page_num - 1).await?;

            (data, total)
        } else {
            let data: Vec<Episode> = query.all(&*self.db).await?;
            let total = data.len();

            (data, total)
        };

        Ok(ManyResponse::new(data, total, page_num, page_size))
    }

    async fn create(&self, input: &CreateEpisodeInput) -> Result<Episode> {
        let episode = episode_model::ActiveModel {
            title: Set(input.title.clone()),
            summary: Set(input.summary.clone()),
            picture: Set(input.picture.clone()),
            content: Set(input.content.clone()),
            show_id: Set(input.show_id.clone()),
            ..Default::default()
        }
        .insert(&*self.db)
        .await?;

        let created: Episode = episode;

        return Ok(created);
    }

    async fn update_model(
        &self,
        episode: episode_model::Model,
        input: &UpdateEpisodeInput,
    ) -> Result<Episode> {
        let mut episode: episode_model::ActiveModel = episode.into();

        if let Some(title) = &input.title {
            episode.title = Set(title.clone());
        }

        if let Some(summary) = &input.summary {
            episode.summary = Set(Some(summary.clone()));
        }

        if let Some(picture) = &input.picture {
            episode.picture = Set(Some(picture.clone()));
        }

        if let Some(content) = &input.content {
            episode.content = Set(Some(content.clone()));
        }

        if let Some(show_id) = &input.show_id {
            episode.show_id = Set(show_id.clone());
        }

        let updated: Episode = episode.update(&*self.db).await?;

        Ok(updated)
    }

    async fn update(&self, id: &str, input: &UpdateEpisodeInput) -> Result<Episode> {
        let query = episode_model::Entity::find_by_id(id.to_owned());

        // Retrieve the existing Episode
        let episode = query
            .one(&*self.db)
            .await?
            .ok_or_else(|| anyhow!("Unable to find Episode with id: {}", id))?;

        self.update_model(episode, input).await
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let episode = episode_model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?
            .ok_or_else(|| anyhow!("Unable to find Episode with id: {}", id))?;

        let _result = episode.delete(&*self.db).await?;

        Ok(())
    }
}
