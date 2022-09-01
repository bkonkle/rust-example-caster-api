use anyhow::Result;
use async_graphql::{dataloader::DataLoader, EmptySubscription, MergedObject, Schema};
use std::sync::Arc;

use crate::Context;
use caster_domains::{
    episodes::{
        resolver::{EpisodesMutation, EpisodesQuery},
        service::EpisodeLoader,
    },
    profiles::{
        resolver::{ProfilesMutation, ProfilesQuery},
        service::ProfileLoader,
    },
    role_grants::service::RoleGrantLoader,
    shows::{
        resolver::{ShowsMutation, ShowsQuery},
        service::ShowLoader,
    },
    users::{
        resolver::{UsersMutation, UsersQuery},
        service::UserLoader,
    },
};

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
    let user_loader = UserLoader::new(&ctx.users);
    let profile_loader = ProfileLoader::new(&ctx.profiles);
    let role_grant_loader = RoleGrantLoader::new(&ctx.role_grants);
    let show_loader = ShowLoader::new(&ctx.shows);
    let episode_loader = EpisodeLoader::new(&ctx.episodes);

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
