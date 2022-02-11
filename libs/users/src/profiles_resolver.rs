use std::sync::Arc;

use anyhow::Result;
use async_graphql::{Context, Object};
use caster_auth::Subject;

use crate::{profile_model::Profile, profiles_service::ProfilesService};

/// The Query segment for Profiles
#[derive(Default)]
pub struct ProfilesQuery {}

/// Queries for the User model
#[Object]
impl ProfilesQuery {
    async fn get_profile(&self, ctx: &Context<'_>, id: String) -> Result<Option<Profile>> {
        let profiles = ctx.data_unchecked::<Arc<dyn ProfilesService>>();
        let _subject = ctx.data_unchecked::<Subject>();

        let _profile = profiles.get(&id).await?;

        // match subject {
        //     Subject(Some(username)) => {
        //         users
        //             .get_by_username(username)
        //             .await
        //             .map_err(as_graphql_error(
        //                 "Error while retrieving User",
        //                 StatusCode::INTERNAL_SERVER_ERROR,
        //             ))
        //     }
        //     _ => Err(graphql_error(
        //         "A valid JWT token is required",
        //         StatusCode::UNAUTHORIZED,
        //     )),
        // }

        Ok(None)
    }
}
