#![allow(dead_code)] // Since each test is an independent module, this is needed

use anyhow::Result;
use biscuit::{
    jwa::SignatureAlgorithm,
    jws::{RegisteredHeader, Secret},
    ClaimsSet, Empty, RegisteredClaims, SingleOrMultiple, JWT,
};
use fake::{Fake, Faker};
use futures_util::{stream::SplitStream, Future, SinkExt, StreamExt};
use hyper::{client::HttpConnector, Client};
use hyper_tls::HttpsConnector;
use once_cell::sync::Lazy;
use std::default::Default;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::{
    net::TcpStream,
    time::{sleep, timeout},
};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use caster_api::{run, Context};
use caster_domains::{
    episodes::{model::Episode, mutations::CreateEpisodeInput},
    profiles::{model::Profile, mutations::CreateProfileInput},
    shows::{model::Show, mutations::CreateShowInput},
    users::model::User,
};
use caster_testing::graphql::GraphQL;
use caster_utils::{config::get_config, http::http_client};

static HTTP_CLIENT: Lazy<Client<HttpsConnector<HttpConnector>>> = Lazy::new(http_client);

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
    pub graphql: GraphQL,
    pub ctx: Arc<Context>,
    pub addr: SocketAddr,
}

impl TestUtils {
    /// Initialize a new set of utils
    pub async fn init() -> Result<Self> {
        pretty_env_logger::try_init()?;

        let config = get_config();

        // This needs to be created anew each time because the database connection can't be shared
        // when the Tokio runtime is being stopped and re-started between tests
        let ctx = Arc::new(Context::init(config).await?);

        let http_client = &HTTP_CLIENT;
        let addr = run_server(ctx.clone()).await?;

        let graphql = GraphQL::new(format!(
            "http://localhost:{port}/graphql",
            port = addr.port()
        ));

        Ok(TestUtils {
            http_client,
            graphql,
            ctx,
            addr,
        })
    }

    /// Create a test JWT token with a dummy secret
    pub fn create_jwt(&self, username: &str) -> String {
        let auth = self.ctx.config.auth.clone();

        let expected_claims = ClaimsSet::<Empty> {
            registered: RegisteredClaims {
                issuer: Some(auth.url),
                subject: Some(username.to_string()),
                audience: Some(SingleOrMultiple::Single(auth.audience)),
                ..Default::default()
            },
            private: Default::default(),
        };

        let jwt = JWT::new_decoded(
            From::from(RegisteredHeader {
                algorithm: SignatureAlgorithm::HS256,
                ..Default::default()
            }),
            expected_claims,
        );

        let token = jwt
            .into_encoded(&Secret::Bytes("test-jwt-secret".into()))
            .unwrap();

        token.unwrap_encoded().to_string()
    }

    /// Create a User and Profile together
    #[allow(dead_code)] // Since each test is an independent module, this is necessary
    pub async fn create_user_and_profile(
        &self,
        username: &str,
        email: &str,
    ) -> Result<(User, Profile)> {
        let user = self.ctx.users.create(username).await?;

        let mut profile_input: CreateProfileInput = Faker.fake();
        profile_input.user_id = user.id.clone();
        profile_input.email = email.to_string();

        let profile = self.ctx.profiles.create(&profile_input, &false).await?;

        Ok((user, profile))
    }

    /// Create a Show and Episode together
    #[allow(dead_code)] // Since each test is an independent module, this is necessary
    pub async fn create_show_and_episode(
        &self,
        show_title: &str,
        episode_title: &str,
    ) -> Result<(Show, Episode)> {
        let show_input = CreateShowInput {
            title: show_title.to_string(),
            ..Default::default()
        };

        let show = self.ctx.shows.create(&show_input).await?;

        let episode_input = CreateEpisodeInput {
            title: episode_title.to_string(),
            show_id: show.id.clone(),
            ..Default::default()
        };

        let episode = self.ctx.episodes.create(&episode_input, &false).await?;

        Ok((show, episode))
    }

    /// Send a message with the default timeout
    pub async fn send_message<T>(
        &self,
        message: Message,
        to_future: fn(SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) -> T,
    ) -> Result<()>
    where
        T: Future,
    {
        self.send_to_websocket(message, to_future, None).await
    }

    /// Send a message with a custom timeout
    pub async fn send_message_with_timeout<T>(
        &self,
        message: Message,
        to_future: fn(SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) -> T,
        timer: u64,
    ) -> Result<()>
    where
        T: Future,
    {
        self.send_to_websocket(message, to_future, Some(timer))
            .await
    }

    async fn send_to_websocket<T>(
        &self,
        message: Message,
        to_future: fn(SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) -> T,
        time: Option<u64>,
    ) -> Result<()>
    where
        T: Future,
    {
        let url = url::Url::parse(&format!(
            "ws://localhost:{port}/events",
            port = self.addr.port()
        ))?;

        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        let (mut write, read) = ws_stream.split();

        write.send(message).await.unwrap();

        if timeout(Duration::from_millis(time.unwrap_or(1000)), to_future(read))
            .await
            .is_err()
        {
            panic!("Error: future timed out")
        }

        Ok(())
    }
}
