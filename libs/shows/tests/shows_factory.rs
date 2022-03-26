use fake::{Fake, Faker};
use slug::slugify;

use caster_shows::{episode_model::Episode, show_model::Show};

pub fn create_show(title: &str) -> Show {
    Show {
        id: slugify(title),
        title: title.to_string(),
        summary: Faker.fake(),
        picture: Faker.fake(),
        content: None,
        created_at: Faker.fake(),
        updated_at: Faker.fake(),
    }
}

#[allow(dead_code)]
pub fn create_episode_for_show_option(title: &str, show: Option<Show>) -> Episode {
    let show = show.unwrap_or_else(|| create_show(title));

    Episode {
        id: slugify(title),
        title: title.to_string(),
        summary: Faker.fake(),
        picture: Faker.fake(),
        content: None,
        show_id: show.id.clone(),
        show: Some(show),
        created_at: Faker.fake(),
        updated_at: Faker.fake(),
    }
}

#[allow(dead_code)]
pub fn create_episode_for_show(title: &str, show: Show) -> Episode {
    create_episode_for_show_option(title, Some(show))
}

#[allow(dead_code)]
pub fn create_episode(title: &str) -> Episode {
    create_episode_for_show_option(title, None)
}
