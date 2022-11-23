//! # A GraphQL server written in Rust
#![forbid(unsafe_code)]

use anyhow::Result;
use axum::{
    extract::Extension,
    routing::{get, IntoMakeService},
    Router, Server,
};
use graphql::create_schema;
use hyper::server::conn::AddrIncoming;
use oso::{Oso, PolarClass};
use router::{events_handler, graphiql, graphql_handler, health_handler};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tower_http::trace::{self, TraceLayer};

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

mod errors;
mod router;

/// GraphQL Schema Creation
pub mod graphql;

/// `WebSocket` Events
pub mod events;

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
pub async fn run(ctx: Arc<Context>) -> Result<Server<AddrIncoming, IntoMakeService<Router>>> {
    let port = ctx.config.port;
    let jwks = get_jwks(ctx.config).await;

    let schema = create_schema(ctx.clone())?;

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/graphql", get(graphiql).post(graphql_handler))
        .route("/events", get(events_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        .layer(Extension(jwks))
        .layer(Extension(ctx))
        .layer(Extension(schema));

    let server = Server::bind(
        &format!("0.0.0.0:{}", port)
            .parse()
            .expect("Unable to parse bind address"),
    )
    .serve(app.into_make_service());

    Ok(server)
}
