//! # A GraphQL server written in Rust
#![forbid(unsafe_code)]

use anyhow::Result;
use graphql::create_schema;
use sea_orm::DatabaseConnection;
use std::{net::SocketAddr, sync::Arc};
use warp::{Filter, Future};

use caster_auth::jwks::get_jwks;
use caster_shows::{
    episodes_service::{DefaultEpisodesService, EpisodesService},
    shows_service::{DefaultShowsService, ShowsService},
};
use caster_users::{
    profiles_service::{DefaultProfilesService, ProfilesService},
    role_grants_service::{DefaultRoleGrantsService, RoleGrantsService},
    users_service::{DefaultUsersService, UsersService},
};
use caster_utils::config::Config;
use router::create_routes;

mod errors;
mod graphql;
mod router;

#[macro_use]
extern crate log;

/// Dependencies needed by the resolvers
pub struct Dependencies {
    /// The `User` entity service
    pub users: Arc<dyn UsersService>,

    /// The `Profile` entity service
    pub profiles: Arc<dyn ProfilesService>,

    /// The `RoleGrant` entity service
    pub role_grants: Arc<dyn RoleGrantsService>,

    /// The `Show` entity service
    pub shows: Arc<dyn ShowsService>,

    /// The `Episode` entity service
    pub episodes: Arc<dyn EpisodesService>,
}

/// Intialize dependencies
impl Dependencies {
    /// Create a new set of dependencies based on the given shared resources
    pub fn new(db: &Arc<DatabaseConnection>) -> Self {
        // Services
        let users = Arc::new(DefaultUsersService::new(db)) as Arc<dyn UsersService>;
        let profiles = Arc::new(DefaultProfilesService::new(db)) as Arc<dyn ProfilesService>;
        let role_grants = Arc::new(DefaultRoleGrantsService::new(db)) as Arc<dyn RoleGrantsService>;
        let shows = Arc::new(DefaultShowsService::new(db)) as Arc<dyn ShowsService>;
        let episodes = Arc::new(DefaultEpisodesService::new(db)) as Arc<dyn EpisodesService>;

        Self {
            users,
            profiles,
            role_grants,
            shows,
            episodes,
        }
    }
}

/// Start the server and return the bound address and a `Future`.
pub async fn run(config: &'static Config) -> Result<(SocketAddr, impl Future<Output = ()>)> {
    let port = config.port;
    let jwks = get_jwks(config).await;

    let db = Arc::new(sea_orm::Database::connect(&config.database.url).await?);
    let deps = Dependencies::new(&db);
    let users = deps.users.clone();

    let schema = create_schema(deps, config)?;
    let router = create_routes(users, schema, jwks);

    Ok(warp::serve(
        router
            .with(warp::log("caster_api"))
            .recover(errors::handle_rejection),
    )
    .bind_ephemeral(([0, 0, 0, 0], port)))
}
