use async_graphql::{InputObject, MaybeUndefined, SimpleObject};
use caster_utils::graphql::dummy_maybe_undef;
use fake::{Dummy, Fake, Faker};
use rand::Rng;

use super::model::Episode;

/// The `CreateEpisodeInput` input type
#[derive(Clone, Default, Dummy, Eq, PartialEq, InputObject)]
pub struct CreateEpisodeInput {
    /// The Episode's title
    pub title: String,

    /// The Episode's description summary
    pub summary: Option<String>,

    /// The Episode's picture
    pub picture: Option<String>,

    /// The Episode's Show id
    pub show_id: String,
}

/// The `UpdateEpisodeInput` input type
#[derive(Clone, Default, Eq, PartialEq, InputObject)]
pub struct UpdateEpisodeInput {
    /// The Episode's title
    pub title: Option<String>,

    /// The Episode's description summary
    pub summary: MaybeUndefined<String>,

    /// The Episode's picture
    pub picture: MaybeUndefined<String>,

    /// The Episode's Show id
    pub show_id: Option<String>,
}

impl Dummy<Faker> for UpdateEpisodeInput {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        UpdateEpisodeInput {
            title: Faker.fake(),
            summary: dummy_maybe_undef(config, rng),
            picture: dummy_maybe_undef(config, rng),
            show_id: Faker.fake(),
        }
    }
}

/// The `MutateEpisodeResult` type
#[derive(Clone, Default, Dummy, Eq, PartialEq, SimpleObject)]
pub struct MutateEpisodeResult {
    /// The Episode's subscriber id
    pub episode: Option<Episode>,
}
