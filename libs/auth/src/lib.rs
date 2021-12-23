//! # The Auth Library
#![forbid(unsafe_code)]

use anyhow::Result;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, Filter, Rejection,
};

use crate::error::{reject_any, AuthError};

/// Error Cases
pub mod error;

const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"secret";

/// JWT claims retrieved from the Payload
#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

/// The token's Subject claim
pub struct Subject(String);

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String> {
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(AuthError::NoAuthHeaderError.into()),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(AuthError::NoAuthHeaderError.into()),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(AuthError::InvalidAuthHeaderError.into());
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}

async fn authorize(headers: HeaderMap<HeaderValue>) -> Result<Subject, Rejection> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(JWT_SECRET),
                &Validation::new(Algorithm::HS512),
            )
            .map_err(|err| reject::custom(AuthError::JWTTokenError(err)))?;

            Ok(Subject(decoded.claims.sub))
        }
        Err(e) => Err(reject_any(e)),
    }
}

/// A Warp Filter to add Authentication context
pub fn with_auth() -> impl Filter<Extract = (Subject,), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| (headers))
        .and_then(authorize)
}
