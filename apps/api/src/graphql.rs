use anyhow::Result;
use async_graphql::{EmptySubscription, MergedObject, Schema};
use std::sync::Arc;

use caster_shows::{
    episodes_resolver::{EpisodesMutation, EpisodesQuery},
    shows_resolver::{ShowsMutation, ShowsQuery},
};
use caster_users::{
    profiles_resolver::{ProfilesMutation, ProfilesQuery},
    users_resolver::{UsersMutation, UsersQuery},
};

use crate::Context;

#[derive(MergedObject, Default)]
pub struct Query(UsersQuery, ProfilesQuery, ShowsQuery, EpisodesQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(
    UsersMutation,
    ProfilesMutation,
    ShowsMutation,
    EpisodesMutation,
);

/// The application's top-level merged GraphQL schema
pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;

/// Initialize all necessary dependencies to create a `GraphQLSchema`. Very simple dependency
/// injection based on async-graphql's `.data()` calls.
pub fn create_schema(ctx: Arc<Context>) -> Result<GraphQLSchema> {
    // Inject the initialized services into the `Schema` instance.
    Ok(
        Schema::build(Query::default(), Mutation::default(), EmptySubscription)
            .data(ctx.config)
            .data(ctx.oso.clone())
            .data(ctx.users.clone())
            .data(ctx.profiles.clone())
            .data(ctx.role_grants.clone())
            .data(ctx.shows.clone())
            .data(ctx.episodes.clone())
            .finish(),
    )
}
