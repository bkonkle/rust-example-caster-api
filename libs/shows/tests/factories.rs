use fake::{Fake, Faker};

use caster_shows::{episode_model::Episode, show_model::Show};

pub fn create_show() -> Show {
    Show {
        id: String::from("test-show"),
        title: String::from("Test Show"),
        summary: Faker.fake(),
        picture: Faker.fake(),
        content: None,
        created_at: Faker.fake(),
        updated_at: Faker.fake(),
    }
}

#[allow(dead_code)]
pub fn create_episode_from_show_option(show: Option<Show>) -> Episode {
    let show = show.unwrap_or_else(create_show);

    Episode {
        id: String::from("test-episode"),
        title: String::from("Test Episode"),
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
pub fn create_episode_from_show(show: Show) -> Episode {
    create_episode_from_show_option(Some(show))
}

#[allow(dead_code)]
pub fn create_episode() -> Episode {
    create_episode_from_show_option(None)
}
