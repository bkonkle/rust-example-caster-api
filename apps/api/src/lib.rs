//! # A GraphQL server written in Rust
#![forbid(unsafe_code)]

use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{net::SocketAddr, sync::Arc};
use warp::{Filter, Future};

use caster_auth::jwks::get_jwks;
use caster_shows::{
    shows_repository::PgShowsRepository,
    shows_service::{DefaultShowsService, ShowsService},
};
use caster_users::{
    profiles_repository::PgProfilesRepository,
    profiles_service::{DefaultProfilesService, ProfilesService},
    users_repository::PgUsersRepository,
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
    /// The Users entity service
    pub users: Arc<dyn UsersService>,

    /// The Profiles entity service
    pub profiles: Arc<dyn ProfilesService>,

    /// The Shows entity service
    pub shows: Arc<dyn ShowsService>,
}

/// Intialize dependencies
impl Dependencies {
    /// Create a new set of dependencies based on the given shared resources
    pub fn new(pool: &Arc<PgPool>) -> Self {
        // Service dependencies
        let users_repo = Arc::new(PgUsersRepository::new(pool));
        let profiles_repo = Arc::new(PgProfilesRepository::new(pool));
        let shows_repo = Arc::new(PgShowsRepository::new(pool));

        // Services
        let users = Arc::new(DefaultUsersService::new(&users_repo)) as Arc<dyn UsersService>;
        let profiles =
            Arc::new(DefaultProfilesService::new(&profiles_repo)) as Arc<dyn ProfilesService>;
        let shows = Arc::new(DefaultShowsService::new(&shows_repo)) as Arc<dyn ShowsService>;

        Self {
            users,
            profiles,
            shows,
        }
    }
}

/// Start the server and return the bound address and a `Future`.
pub async fn run(config: &'static Config) -> Result<(SocketAddr, impl Future<Output = ()>)> {
    let port = config.port;
    let jwks = get_jwks(config).await;

    let pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(10)
            .connect(&config.database.url)
            .await?,
    );
    let deps = Dependencies::new(&pool);

    let router = create_routes(deps, config, jwks);

    Ok(warp::serve(
        router
            .with(warp::log("caster_api"))
            .recover(errors::handle_rejection),
    )
    .bind_ephemeral(([0, 0, 0, 0], port)))
}
