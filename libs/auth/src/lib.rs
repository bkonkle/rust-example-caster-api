//! # The Auth Library
#![forbid(unsafe_code)]

use anyhow::Result;
use biscuit::{jwa::SignatureAlgorithm, jws::Header, Empty, JWT};
use jwks::JWKS;
use serde::{Deserialize, Serialize};
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, Filter, Rejection,
};

use crate::error::{reject_any, AuthError};
use crate::jwks::get_secret_from_key_set;

/// Error Cases
pub mod error;

/// JWKS well-known key set retrieval
pub mod jwks;

const BEARER: &str = "Bearer ";

/// JWT claims retrieved from the Payload
#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

/// The token's Subject claim
#[derive(Clone)]
pub struct Subject(String);

impl Subject {
    /// Retrieve the wrapped username
    pub fn username(&self) -> &String {
        let Subject(username) = self;

        username
    }
}

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

async fn authorize(
    jwks: &'static JWKS,
    headers: HeaderMap<HeaderValue>,
) -> Result<Subject, Rejection> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            // First extract without verifying the header to locate the key-id (kid)
            let token = JWT::<Claims, Empty>::new_encoded(&jwt);

            let header: Header<Empty> = token
                .unverified_header()
                .map_err(|err| reject::custom(AuthError::JWTTokenError(err)))?;

            let key_id = header
                .registered
                .key_id
                .ok_or_else(|| reject::custom(AuthError::JWKSError))?;

            // Now that we have the key, construct our RSA public key secret
            let secret = get_secret_from_key_set(jwks, &key_id)
                .map_err(|_err| reject::custom(AuthError::JWKSError))?;

            // Not fully verify and extract the token with verification
            let token = token
                .into_decoded(&secret, SignatureAlgorithm::RS256)
                .map_err(|err| reject::custom(AuthError::JWTTokenError(err)))?;

            let payload = token
                .payload()
                .map_err(|err| reject::custom(AuthError::JWTTokenError(err)))?;

            Ok(Subject(payload.private.sub.clone()))
        }
        Err(e) => Err(reject_any(e)),
    }
}

/// A Warp Filter to add Authentication context
pub fn with_auth(
    jwks: &'static JWKS,
) -> impl Filter<Extract = (Subject,), Error = Rejection> + Clone {
    headers_cloned().and_then(move |headers: HeaderMap<HeaderValue>| authorize(jwks, headers))
}
