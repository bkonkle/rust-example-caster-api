use anyhow;
use async_graphql::{Context, Object};
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use once_cell::sync::Lazy;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::{fmt, marker::PhantomData, sync::Arc};

use crate::{user_model::User, users_service::UsersService};

/// The Query segment owned by the Users library
#[derive(Default)]
pub struct UsersQuery {}

#[Object]
impl UsersQuery {
    async fn get_user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The User id")] id: String,
    ) -> Result<Option<User>, anyhow::Error> {
        let users = ctx.data_unchecked::<Arc<dyn UsersService>>();

        Ok(users.get(id).await?)
    }

    async fn get_current_user(&self, ctx: &Context<'_>) -> Result<Option<User>, anyhow::Error> {
        Ok(None)
    }
}

static JWT_SECRET_KEY: Lazy<String> =
    Lazy::new(|| std::env::var("JWT_SECRET_KEY").expect("Can't read JWT_SECRET_KEY"));

fn string_or_seq_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrVec(PhantomData<Vec<String>>);

    impl<'de> de::Visitor<'de> for StringOrVec {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![value.to_owned()])
        }

        fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(StringOrVec(PhantomData))
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

fn decode_token(token: &str) -> TokenData<Claims> {
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET_KEY.as_ref()),
        &Validation::default(),
    )
    .expect("Can't decode token")
}

pub fn get_sub(http_request: HttpRequest) -> Option<User> {
    http_request
        .headers()
        .get("Authorization")
        .and_then(|header_value| {
            header_value.to_str().ok().map(|s| {
                let jwt_start_index = "Bearer ".len();
                let jwt = s[jwt_start_index..s.len()].to_string();
                let token_data = decode_token(&jwt);
            })
        })
}
