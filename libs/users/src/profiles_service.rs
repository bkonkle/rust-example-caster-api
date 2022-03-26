use anyhow::Result;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sea_orm::{entity::*, query::*, DatabaseConnection, EntityTrait};
use std::sync::Arc;

use crate::{
    profile_model::{self, Profile, ProfileList, ProfileOption},
    profile_mutations::{CreateProfileInput, UpdateProfileInput},
    profile_queries::{ProfileCondition, ProfilesOrderBy},
    user_model::{self, User},
};
use caster_utils::{ordering::Ordering, pagination::ManyResponse};

/// A ProfilesService applies business logic to a dynamic ProfilesRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait ProfilesService: Sync + Send {
    /// Get an individual `Profile` by id, returning the Model instance for updating
    async fn get_model(
        &self,
        id: &str,
        with_user: &bool,
    ) -> Result<Option<(profile_model::Model, Option<User>)>>;

    /// Get an individual `Profile` by id
    async fn get(&self, id: &str, with_user: &bool) -> Result<Option<Profile>>;

    /// Get multiple `Profile` records
    async fn get_many(
        &self,
        condition: Option<ProfileCondition>,
        order_by: Option<Vec<ProfilesOrderBy>>,
        page_size: Option<usize>,
        page: Option<usize>,
        with_user: &bool,
    ) -> Result<ManyResponse<Profile>>;

    /// Get the first `Profile` with this user_id
    async fn get_by_user_id(&self, user_id: &str, with_user: &bool) -> Result<Option<Profile>>;

    /// Get or create a `Profile`.
    async fn get_or_create(
        &self,
        user_id: &str,
        input: &CreateProfileInput,
        with_user: &bool,
    ) -> Result<Profile>;

    /// Create a `Profile` with the given input
    async fn create(&self, input: &CreateProfileInput, with_user: &bool) -> Result<Profile>;

    /// Update an existing `Profile`
    async fn update_model(
        &self,
        profile: profile_model::Model,
        input: &UpdateProfileInput,
        user: Option<User>,
    ) -> Result<Profile>;

    /// Update an existing `Profile` by id
    async fn update(
        &self,
        id: &str,
        input: &UpdateProfileInput,
        with_user: &bool,
    ) -> Result<Profile>;

    /// Delete an existing `Profile`
    async fn delete(&self, id: &str) -> Result<()>;
}

/// The default `ProfilesService` struct
pub struct DefaultProfilesService {
    /// The SeaOrm database connection
    db: Arc<DatabaseConnection>,
}

/// The default `ProfilesService` implementation
impl DefaultProfilesService {
    /// Create a new `ProfilesService` instance
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ProfilesService for DefaultProfilesService {
    async fn get_model(
        &self,
        id: &str,
        with_user: &bool,
    ) -> Result<Option<(profile_model::Model, Option<User>)>> {
        let query = profile_model::Entity::find_by_id(id.to_owned());

        let profile = if *with_user {
            query
                .find_also_related(user_model::Entity)
                .one(&*self.db)
                .await?
        } else {
            query.one(&*self.db).await?.map(|u| (u, None))
        };

        Ok(profile)
    }

    async fn get(&self, id: &str, with_user: &bool) -> Result<Option<Profile>> {
        let profile: ProfileOption = self.get_model(id, with_user).await?.into();

        Ok(profile.into())
    }

    async fn get_many(
        &self,
        condition: Option<ProfileCondition>,
        order_by: Option<Vec<ProfilesOrderBy>>,
        page: Option<usize>,
        page_size: Option<usize>,
        with_user: &bool,
    ) -> Result<ManyResponse<Profile>> {
        let page_num = page.unwrap_or(1);

        let mut query = profile_model::Entity::find();

        if let Some(condition) = condition {
            if let Some(email) = condition.email {
                query = query.filter(profile_model::Column::Email.eq(email));
            }

            if let Some(display_name) = condition.display_name {
                query = query.filter(profile_model::Column::DisplayName.eq(display_name));
            }

            if let Some(city) = condition.city {
                query = query.filter(profile_model::Column::City.eq(city));
            }

            if let Some(state_province) = condition.state_province {
                query = query.filter(profile_model::Column::StateProvince.eq(state_province));
            }

            if let Some(user_id) = condition.user_id {
                query = query.filter(profile_model::Column::UserId.eq(user_id));
            }
        }

        if let Some(order_by) = order_by {
            for order in order_by {
                let ordering: Ordering<profile_model::Column> = order.into();

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

        let (data, total) = match (page_size, with_user) {
            (Some(page_size), true) => {
                let paginator = query
                    .find_also_related(user_model::Entity)
                    .paginate(&*self.db, page_size);

                let total = paginator.num_items().await?;
                let data: ProfileList = paginator.fetch_page(page_num - 1).await?.into();

                (data, total)
            }
            (Some(page_size), false) => {
                let paginator = query.paginate(&*self.db, page_size);
                let total = paginator.num_items().await?;
                let data: ProfileList = paginator.fetch_page(page_num - 1).await?.into();

                (data, total)
            }
            (None, true) => {
                let data: ProfileList = query
                    .find_also_related(user_model::Entity)
                    .all(&*self.db)
                    .await?
                    .into();

                let total = data.len();

                (data, total)
            }
            (None, false) => {
                let data: ProfileList = query.all(&*self.db).await?.into();
                let total = data.len();

                (data, total)
            }
        };

        Ok(ManyResponse::new(data.into(), total, page_num, page_size))
    }

    async fn get_by_user_id(&self, user_id: &str, with_user: &bool) -> Result<Option<Profile>> {
        let query = profile_model::Entity::find()
            .filter(profile_model::Column::UserId.eq(user_id.to_owned()));

        let profile: ProfileOption = match with_user {
            true => query
                .find_also_related(user_model::Entity)
                .one(&*self.db)
                .await?
                .into(),
            false => query.one(&*self.db).await?.into(),
        };

        Ok(profile.into())
    }

    async fn create(&self, input: &CreateProfileInput, with_user: &bool) -> Result<Profile> {
        let profile = profile_model::ActiveModel {
            email: Set(input.email.clone()),
            display_name: Set(input.display_name.clone()),
            picture: Set(input.picture.clone()),
            content: Set(input.content.clone()),
            city: Set(input.city.clone()),
            state_province: Set(input.state_province.clone()),
            user_id: Set(Some(input.user_id.clone())),
            ..Default::default()
        }
        .insert(&*self.db)
        .await?;

        let mut created: Profile = profile.into();

        if !with_user {
            return Ok(created);
        }

        let user = user_model::Entity::find_by_id(input.user_id.clone())
            .one(&*self.db)
            .await?;

        created.user = user;

        Ok(created)
    }

    async fn get_or_create(
        &self,
        user_id: &str,
        input: &CreateProfileInput,
        with_user: &bool,
    ) -> Result<Profile> {
        let profile = self.get_by_user_id(user_id, with_user).await?;

        if let Some(profile) = profile {
            return Ok(profile);
        }

        self.create(input, with_user).await
    }

    async fn update_model(
        &self,
        profile: profile_model::Model,
        input: &UpdateProfileInput,
        user: Option<User>,
    ) -> Result<Profile> {
        let mut profile: profile_model::ActiveModel = profile.into();

        if let Some(email) = &input.email {
            profile.email = Set(email.clone());
        }

        if let Some(display_name) = &input.display_name {
            profile.display_name = Set(Some(display_name.clone()));
        }

        if let Some(picture) = &input.picture {
            profile.picture = Set(Some(picture.clone()));
        }

        if let Some(content) = &input.content {
            profile.content = Set(Some(content.clone()));
        }

        if let Some(city) = &input.city {
            profile.city = Set(Some(city.clone()));
        }

        if let Some(state_province) = &input.state_province {
            profile.state_province = Set(Some(state_province.clone()));
        }

        if let Some(user_id) = &input.user_id {
            profile.user_id = Set(Some(user_id.clone()));
        }

        let mut updated: Profile = profile.update(&*self.db).await?.into();

        // Add back the User from above
        updated.user = user;

        Ok(updated)
    }

    async fn update(
        &self,
        id: &str,
        input: &UpdateProfileInput,
        with_user: &bool,
    ) -> Result<Profile> {
        let query = profile_model::Entity::find_by_id(id.to_owned());

        // Pull out the `Profile` and the related `User`, if selected
        let (profile, user) = if *with_user {
            query
                .find_also_related(user_model::Entity)
                .one(&*self.db)
                .await?
        } else {
            // If the Profile isn't requested, just map to None
            query.one(&*self.db).await?.map(|p| (p, None))
        }
        .ok_or_else(|| anyhow!("Unable to find Profile with id: {}", id))?;

        self.update_model(profile, input, user).await
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let profile = profile_model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?
            .ok_or_else(|| anyhow!("Unable to find Profile with id: {}", id))?;

        let _result = profile.delete(&*self.db).await?;

        Ok(())
    }
}
