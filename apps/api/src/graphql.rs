use anyhow::Result;
use async_graphql::{EmptySubscription, MergedObject, Schema};
use oso::{Oso, PolarClass};

use caster_shows::{
    episode_model::Episode,
    show_model::Show,
    shows_resolver::{ShowsMutation, ShowsQuery},
    AUTHORIZATION as SHOWS_AUTHZ,
};
use caster_users::{
    profile_model::Profile,
    profiles_resolver::{ProfilesMutation, ProfilesQuery},
    user_model::User,
    users_resolver::{UsersMutation, UsersQuery},
    AUTHORIZATION as PROFILES_AUTHZ,
};
use caster_utils::config::Config;

use crate::Dependencies;

#[derive(MergedObject, Default)]
pub struct Query(UsersQuery, ProfilesQuery, ShowsQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UsersMutation, ProfilesMutation, ShowsMutation);

/// The application's top-level merged GraphQL schema
pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;

/// Initialize all necessary dependencies to create a `GraphQLSchema`. Very simple dependency
/// injection based on async-graphql's `.data()` calls.
pub fn create_schema(deps: Dependencies, config: &'static Config) -> Result<GraphQLSchema> {
    let Dependencies {
        users,
        profiles,
        shows,
    } = deps;

    // Set up authorization
    let mut oso = Oso::new();

    oso.register_class(User::get_polar_class_builder().name("User").build())?;
    oso.register_class(Profile::get_polar_class_builder().name("Profile").build())?;
    oso.register_class(Show::get_polar_class_builder().name("Show").build())?;
    oso.register_class(Episode::get_polar_class_builder().name("Episode").build())?;

    oso.load_str(&format!("{}\n{}", PROFILES_AUTHZ, SHOWS_AUTHZ))?;

    // Inject the initialized services into the `Schema` instance.
    Ok(
        Schema::build(Query::default(), Mutation::default(), EmptySubscription)
            .data(config)
            .data(oso)
            .data(users)
            .data(profiles)
            .data(shows)
            .finish(),
    )
}
