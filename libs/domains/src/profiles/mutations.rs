use async_graphql::{InputObject, MaybeUndefined, SimpleObject};
use fake::{faker::internet::en::FreeEmail, Dummy, Fake, Faker};
use rand::Rng;

use caster_utils::graphql::dummy_maybe_undef;

use super::model::Profile;

/// The `CreateProfileInput` input type
#[derive(Clone, Default, Dummy, Eq, PartialEq, InputObject)]
pub struct CreateProfileInput {
    /// The Profile's email address
    pub email: String,

    /// The Profile's display name
    pub display_name: Option<String>,

    /// The Profile's picture
    pub picture: Option<String>,

    /// The Profile's city
    pub city: Option<String>,

    /// The Profile's state or province
    pub state_province: Option<String>,

    /// The Profile's User id
    pub user_id: String,
}

/// The `UpdateProfileInput` input type
#[derive(Clone, Default, Eq, PartialEq, InputObject)]
pub struct UpdateProfileInput {
    /// The Profile's email address
    pub email: Option<String>,

    /// The Profile's display name
    pub display_name: MaybeUndefined<String>,

    /// The Profile's picture
    pub picture: MaybeUndefined<String>,

    /// The Profile's city
    pub city: MaybeUndefined<String>,

    /// The Profile's state or province
    pub state_province: MaybeUndefined<String>,

    /// The Profile's User id
    pub user_id: Option<String>,
}

impl Dummy<Faker> for UpdateProfileInput {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        UpdateProfileInput {
            email: FreeEmail().fake(),
            display_name: dummy_maybe_undef(config, rng),
            picture: dummy_maybe_undef(config, rng),
            city: dummy_maybe_undef(config, rng),
            state_province: dummy_maybe_undef(config, rng),
            user_id: Faker.fake(),
        }
    }
}

/// The `MutateProfileResult` type
#[derive(Clone, Default, Dummy, Eq, PartialEq, SimpleObject)]
pub struct MutateProfileResult {
    /// The Profile's subscriber id
    pub profile: Option<Profile>,
}
