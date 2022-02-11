use async_graphql::{Context, Object, Result};
use caster_auth::Subject;
use caster_utils::errors::{as_graphql_error, graphql_error};
use hyper::StatusCode;
use std::sync::Arc;

use crate::{
    profile_model::Profile, profiles_service::ProfilesService, users_service::UsersService,
};

/// The Query segment for Profiles
#[derive(Default)]
pub struct ProfilesQuery {}

/// Queries for the User model
#[Object]
impl ProfilesQuery {
    async fn get_profile(&self, ctx: &Context<'_>, id: String) -> Result<Option<Profile>> {
        let users = ctx.data_unchecked::<Arc<dyn UsersService>>();
        let profiles = ctx.data_unchecked::<Arc<dyn ProfilesService>>();
        let subject = ctx.data_unchecked::<Subject>();

        let profile = profiles.get(&id, &false).await?;

        // Retrieve the User for authorization
        let user = match subject {
            Subject(Some(username)) => {
                users
                    .get_by_username(username, &false)
                    .await
                    .map_err(as_graphql_error(
                        "Error while retrieving User",
                        StatusCode::INTERNAL_SERVER_ERROR,
                    ))
            }
            _ => Err(graphql_error(
                "A valid JWT token is required",
                StatusCode::UNAUTHORIZED,
            )),
        }?;

        let censored = match user {
            Some(user) => {
                let user_id = user.id.clone();

                // If the User and Profile are present, censor the Profile based on the User id
                profile.map(|p| {
                    Profile {
                        user: Box::new(Some(user)),
                        ..p
                    }
                    .censor(Some(user_id))
                })
            }
            // If the User is absent, always censor the Profile
            None => profile.map(|p| p.censor(None)),
        };

        Ok(censored)
    }
}
