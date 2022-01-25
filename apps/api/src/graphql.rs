use async_graphql::{EmptySubscription, MergedObject, Schema};
use sqlx::PgPool;
use std::sync::Arc;

use caster_shows::{
    shows_repository::PgShowsRepository,
    shows_resolver::ShowsQuery,
    shows_service::{DefaultShowsService, ShowsService},
};
use caster_users::{
    users_repository::PgUsersRepository,
    users_resolver::{UsersMutation, UsersQuery},
    users_service::{DefaultUsersService, UsersService},
};
use caster_utils::config::Config;

#[derive(MergedObject, Default)]
pub struct Query(UsersQuery, ShowsQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UsersMutation);

/// The application's top-level merged GraphQL schema
pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;

/// Initialize all necessary dependencies to create a `GraphQLSchema`. Very simple dependency
/// injection based on async-graphql's `.data()` calls.
pub fn create_schema(pool: &Arc<PgPool>, config: &'static Config) -> GraphQLSchema {
    // Service dependencies
    let shows_repo = Arc::new(PgShowsRepository::new(pool));
    let users_repo = Arc::new(PgUsersRepository::new(pool));

    // Services
    let shows = Arc::new(DefaultShowsService::new(&shows_repo)) as Arc<dyn ShowsService>;
    let users = Arc::new(DefaultUsersService::new(&users_repo)) as Arc<dyn UsersService>;

    // Inject the initialized services into the `Schema` instance.
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(config)
        .data(shows)
        .data(users)
        .finish()
}
