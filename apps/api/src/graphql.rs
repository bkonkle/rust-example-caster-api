use async_graphql::{EmptySubscription, MergedObject, Schema};

use caster_shows::shows_resolver::ShowsQuery;
use caster_users::{
    profiles_resolver::{ProfilesMutation, ProfilesQuery},
    users_resolver::{UsersMutation, UsersQuery},
};
use caster_utils::config::Config;

use crate::Dependencies;

#[derive(MergedObject, Default)]
pub struct Query(UsersQuery, ProfilesQuery, ShowsQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UsersMutation, ProfilesMutation);

/// The application's top-level merged GraphQL schema
pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;

/// Initialize all necessary dependencies to create a `GraphQLSchema`. Very simple dependency
/// injection based on async-graphql's `.data()` calls.
pub fn create_schema(deps: Dependencies, config: &'static Config) -> GraphQLSchema {
    let Dependencies {
        users,
        profiles,
        shows,
    } = deps;

    // Inject the initialized services into the `Schema` instance.
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(config)
        .data(users)
        .data(profiles)
        .data(shows)
        .finish()
}
