use anyhow::Result;
use hyper::{client::HttpConnector, Client};
use hyper_tls::HttpsConnector;
use once_cell::sync::{Lazy, OnceCell};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::time::sleep;

use caster_api::{run, Context};
use caster_shows::{
    episode_model::Episode, episode_mutations::CreateEpisodeInput, show_model::Show,
    show_mutations::CreateShowInput,
};
use caster_test::{graphql::GraphQL, oauth2::OAuth2Utils};
use caster_users::{
    profile_model::Profile, profile_mutations::CreateProfileInput, user_model::User,
};
use caster_utils::{config::get_config, http::http_client};

static HTTP_CLIENT: Lazy<Client<HttpsConnector<HttpConnector>>> = Lazy::new(http_client);
static OAUTH: OnceCell<OAuth2Utils> = OnceCell::new();

pub async fn run_server(context: Arc<Context>) -> Result<SocketAddr> {
    let (addr, server) = run(context).await?;

    // Spawn the server in the background
    tokio::spawn(server);

    // Wait for it to initialize
    sleep(Duration::from_millis(200)).await;

    // Return the bound address
    Ok(addr)
}

/// Common test utils
pub struct TestUtils {
    pub http_client: &'static Client<HttpsConnector<HttpConnector>>,
    pub oauth: &'static OAuth2Utils,
    pub graphql: GraphQL,
    pub ctx: Arc<Context>,
}

impl TestUtils {
    /// Initialize a new set of utils
    pub async fn init() -> Result<Self> {
        let _ = pretty_env_logger::try_init();

        let config = get_config();

        // This needs to be created anew each time because the database connection can't be shared
        // when the Tokio runtime is being stopped and re-started between tests
        let ctx = Arc::new(Context::init(config).await?);

        let oauth = OAUTH.get_or_init(|| OAuth2Utils::new(config));

        let http_client = &HTTP_CLIENT;
        let addr = run_server(ctx.clone()).await?;

        let graphql = GraphQL::new(format!(
            "http://localhost:{port}/graphql",
            port = addr.port()
        ));

        Ok(TestUtils {
            http_client,
            oauth,
            graphql,
            ctx,
        })
    }

    /// Create a User and Profile together
    #[allow(dead_code)] // Since each test is an independent module, this is necessary
    pub async fn create_user_and_profile(
        &self,
        username: &str,
        email: &str,
    ) -> Result<(User, Profile)> {
        let user = self.ctx.users.create(username).await?;

        let profile = self
            .ctx
            .profiles
            .create(
                &CreateProfileInput {
                    email: email.to_string(),
                    user_id: user.id.clone(),
                    display_name: None,
                    picture: None,
                    content: None,
                    city: None,
                    state_province: None,
                },
                &false,
            )
            .await?;

        Ok((user, profile))
    }

    /// Create a Show and Episode together
    #[allow(dead_code)] // Since each test is an independent module, this is necessary
    pub async fn create_show_and_episode(
        &self,
        show_title: &str,
        episode_title: &str,
    ) -> Result<(Show, Episode)> {
        let show = self
            .ctx
            .shows
            .create(&CreateShowInput {
                title: show_title.to_string(),
                summary: None,
                picture: None,
                content: None,
            })
            .await?;

        let episode = self
            .ctx
            .episodes
            .create(
                &CreateEpisodeInput {
                    title: episode_title.to_string(),
                    summary: None,
                    picture: None,
                    content: None,
                    show_id: show.id.clone(),
                },
                &false,
            )
            .await?;

        Ok((show, episode))
    }
}
