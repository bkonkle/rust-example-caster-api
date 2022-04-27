use fake::{Fake, Faker};
use slug::slugify;

use crate::show_factory;
use caster_shows::{
    episode_model::Episode, episode_mutations::CreateEpisodeInput, show_model::Show,
};

/// Create a `CreateEpisodeInput`
pub fn create_episode_input(episode_title: &str, show_id: &str) -> CreateEpisodeInput {
    CreateEpisodeInput {
        title: episode_title.to_string(),
        summary: Faker.fake(),
        picture: Faker.fake(),
        content: None,
        show_id: show_id.to_string(),
    }
}

/// Create a `Episode`
pub fn create_episode_factory(title: Option<String>, show: Option<Show>) -> Episode {
    let title = title.unwrap_or_else(|| Faker.fake());
    let show = show.unwrap_or_else(show_factory::create_show);

    Episode {
        id: slugify(title.clone()),
        title,
        summary: Faker.fake(),
        picture: Faker.fake(),
        content: None,
        show_id: show.id.clone(),
        show: Some(show),
        created_at: Faker.fake(),
        updated_at: Faker.fake(),
    }
}

/// Create a `Episode` for a `Show`
pub fn create_episode_for_show(title: &str, show: Show) -> Episode {
    create_episode_factory(Some(title.to_string()), Some(show))
}

/// Create a `Episode` with a title
#[allow(dead_code)]
pub fn create_episode_with_title(title: &str) -> Episode {
    create_episode_factory(Some(title.to_string()), None)
}

/// Create a `Episode` with no input
#[allow(dead_code)]
pub fn create_episode() -> Episode {
    create_episode_factory(None, None)
}
