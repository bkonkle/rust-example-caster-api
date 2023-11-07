use async_graphql::{InputObject, MaybeUndefined, SimpleObject};
use caster_utils::graphql::dummy_maybe_undef;
use fake::{Dummy, Faker};
use rand::Rng;

use super::model::Show;

/// The `CreateShowInput` input type
#[derive(Clone, Default, Dummy, Eq, PartialEq, InputObject)]
pub struct CreateShowInput {
    /// The Show's title
    pub title: String,

    /// The Show's description summary
    pub summary: Option<String>,

    /// The Show's picture
    pub picture: Option<String>,
}

/// The `UpdateShowInput` input type
#[derive(Clone, Default, Eq, PartialEq, InputObject)]
pub struct UpdateShowInput {
    /// The Show's title
    pub title: MaybeUndefined<String>,

    /// The Show's description summary
    pub summary: MaybeUndefined<String>,

    /// The Show's picture
    pub picture: MaybeUndefined<String>,
}

impl Dummy<Faker> for UpdateShowInput {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        UpdateShowInput {
            title: dummy_maybe_undef(config, rng),
            summary: dummy_maybe_undef(config, rng),
            picture: dummy_maybe_undef(config, rng),
        }
    }
}

/// The `MutateShowResult` type
#[derive(Clone, Default, Dummy, Eq, PartialEq, SimpleObject)]
pub struct MutateShowResult {
    /// The Show's subscriber id
    pub show: Option<Show>,
}
