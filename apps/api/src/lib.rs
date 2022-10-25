//! # A GraphQL server written in Rust
#![forbid(unsafe_code)]

use anyhow::Result;
use graphql::create_schema;
use oso::{Oso, PolarClass};
use sea_orm::DatabaseConnection;
use std::{net::SocketAddr, sync::Arc};
use warp::{Filter, Future};

use caster_auth::jwks::get_jwks;
use caster_domains::{
    episodes::{
        model::Episode,
        service::{DefaultEpisodesService, EpisodesService},
        AUTHORIZATION as EPISODES_AUTHZ,
    },
    profiles::{
        model::Profile,
        service::{DefaultProfilesService, ProfilesService},
        AUTHORIZATION as PROFILES_AUTHZ,
    },
    role_grants::service::{DefaultRoleGrantsService, RoleGrantsService},
    shows::{
        model::Show,
        service::{DefaultShowsService, ShowsService},
        AUTHORIZATION as SHOWS_AUTHZ,
    },
    users::{
        model::User,
        service::{UsersService, UsersServiceTrait},
        AUTHORIZATION as USERS_AUTHZ,
    },
};
use caster_utils::config::Config;
use events::connections::Connections;
use router::create_routes;

mod errors;
mod router;

/// GraphQL Schema Creation
pub mod graphql;

/// `WebSocket` Events
pub mod events;

#[macro_use]
extern crate log;

/// Dependencies needed by the resolvers
pub struct Context {
    /// The app config
    pub config: &'static Config,

    /// The database connections
    pub db: Arc<DatabaseConnection>,

    /// The `Oso` authorization library
    pub oso: Oso,

    /// The `User` entity service
    pub users: Arc<dyn UsersServiceTrait>,

    /// The `Profile` entity service
    pub profiles: Arc<dyn ProfilesService>,

    /// The `RoleGrant` entity service
    pub role_grants: Arc<dyn RoleGrantsService>,

    /// The `Show` entity service
    pub shows: Arc<dyn ShowsService>,

    /// The `Episode` entity service
    pub episodes: Arc<dyn EpisodesService>,

    /// WebSockets connections currently active on this server
    pub connections: Connections,
}

/// Intialize dependencies
impl Context {
    /// Create a new set of dependencies based on the given shared resources
    pub async fn init(config: &'static Config) -> Result<Self> {
        let db = Arc::new(sea_orm::Database::connect(&config.database.url).await?);

        // Set up authorization
        let mut oso = Oso::new();

        let connections = Connections::default();

        oso.register_class(User::get_polar_class_builder().name("User").build())?;
        oso.register_class(Profile::get_polar_class_builder().name("Profile").build())?;
        oso.register_class(Show::get_polar_class_builder().name("Show").build())?;
        oso.register_class(Episode::get_polar_class_builder().name("Episode").build())?;

        oso.load_str(&[USERS_AUTHZ, PROFILES_AUTHZ, SHOWS_AUTHZ, EPISODES_AUTHZ].join("\n"))?;

        Ok(Self {
            config,
            users: Arc::new(UsersService::new(&db)),
            profiles: Arc::new(DefaultProfilesService::new(&db)),
            role_grants: Arc::new(DefaultRoleGrantsService::new(&db)),
            shows: Arc::new(DefaultShowsService::new(&db)),
            episodes: Arc::new(DefaultEpisodesService::new(&db)),
            oso,
            db,
            connections,
        })
    }
}

/// Start the server and return the bound address and a `Future`.
pub async fn run(ctx: Arc<Context>) -> Result<(SocketAddr, impl Future<Output = ()>)> {
    let port = ctx.config.port;
    let jwks = get_jwks(ctx.config).await;

    let schema = create_schema(ctx.clone())?;
    let router = create_routes(&ctx, schema, jwks);

    Ok(warp::serve(
        router
            .with(warp::log("caster_api"))
            .recover(errors::handle_rejection),
    )
    .bind_ephemeral(([0, 0, 0, 0], port)))
}
