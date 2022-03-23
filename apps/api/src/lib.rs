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
pub struct Context {
    /// The app config
    pub config: &'static Config,

    /// The database connections
    pub db: Arc<DatabaseConnection>,

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
impl Context {
    /// Create a new set of dependencies based on the given shared resources
    pub async fn init(config: &'static Config) -> Result<Self> {
        let db = Arc::new(sea_orm::Database::connect(&config.database.url).await?);

        Ok(Self {
            config,
            users: Arc::new(DefaultUsersService::new(db.clone())),
            profiles: Arc::new(DefaultProfilesService::new(db.clone())),
            role_grants: Arc::new(DefaultRoleGrantsService::new(db.clone())),
            shows: Arc::new(DefaultShowsService::new(db.clone())),
            episodes: Arc::new(DefaultEpisodesService::new(db.clone())),
            db,
        })
    }
}

/// Start the server and return the bound address and a `Future`.
pub async fn run(context: Arc<Context>) -> Result<(SocketAddr, impl Future<Output = ()>)> {
    let port = context.config.port;
    let jwks = get_jwks(context.config).await;

    let schema = create_schema(context.clone())?;
    let router = create_routes(context, schema, jwks);

    Ok(warp::serve(
        router
            .with(warp::log("caster_api"))
            .recover(errors::handle_rejection),
    )
    .bind_ephemeral(([0, 0, 0, 0], port)))
}
