use async_graphql::MaybeUndefined;
use fake::{Dummy, Fake, Faker};
use rand::Rng;

/// Randomly generate the `MaybeUndefined` type from the async-graphql library
pub fn dummy_maybe_undef<T, R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> MaybeUndefined<T>
where
    T: Dummy<Faker>,
{
    match (0..2).fake_with_rng(rng) {
        0 => MaybeUndefined::Undefined,
        1 => MaybeUndefined::Null,
        _ => MaybeUndefined::Value(T::dummy_with_rng(config, rng)),
    }
}
