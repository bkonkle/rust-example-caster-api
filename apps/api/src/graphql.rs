use anyhow::Result;
use async_graphql::{dataloader::DataLoader, EmptySubscription, MergedObject, Schema};
use std::sync::Arc;

use caster_shows::{
    episodes_resolver::{EpisodesMutation, EpisodesQuery},
    episodes_service::EpisodeLoader,
    shows_resolver::{ShowsMutation, ShowsQuery},
    shows_service::ShowLoader,
};
use caster_users::{
    profiles_resolver::{ProfilesMutation, ProfilesQuery},
    profiles_service::ProfileLoader,
    role_grants_service::RoleGrantLoader,
    users_resolver::{UsersMutation, UsersQuery},
    users_service::UserLoader,
};

use crate::Context;

/// The GraphQL top-level Query type
#[derive(MergedObject, Default)]
pub struct Query(UsersQuery, ProfilesQuery, ShowsQuery, EpisodesQuery);

/// The GraphQL top-level Mutation type
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
    // Instantiate loaders
    let user_loader = UserLoader::new(ctx.users.clone());
    let profile_loader = ProfileLoader::new(ctx.profiles.clone());
    let role_grant_loader = RoleGrantLoader::new(ctx.role_grants.clone());
    let show_loader = ShowLoader::new(ctx.shows.clone());
    let episode_loader = EpisodeLoader::new(ctx.episodes.clone());

    // Inject the initialized services into the `Schema` instance.
    Ok(
        Schema::build(Query::default(), Mutation::default(), EmptySubscription)
            .data(ctx.config)
            .data(ctx.oso.clone())
            .data(ctx.users.clone())
            .data(DataLoader::new(user_loader, tokio::spawn))
            .data(DataLoader::new(profile_loader, tokio::spawn))
            .data(DataLoader::new(role_grant_loader, tokio::spawn))
            .data(ctx.profiles.clone())
            .data(ctx.role_grants.clone())
            .data(ctx.shows.clone())
            .data(ctx.episodes.clone())
            .data(DataLoader::new(show_loader, tokio::spawn))
            .data(DataLoader::new(episode_loader, tokio::spawn))
            .finish(),
    )
}
