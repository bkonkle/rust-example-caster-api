//! # The Auth Library
#![forbid(unsafe_code)]

use biscuit::{jwa::SignatureAlgorithm, jws::Header, Empty, JWT};
use jwks::JWKS;
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, Filter, Rejection,
};

use crate::errors::{
    AuthError,
    AuthError::{InvalidAuthHeaderError, JWKSError, JWTTokenError},
};
use crate::jwks::get_secret_from_key_set;

#[macro_use]
extern crate log;

/// Error Cases
pub mod errors;

/// JWKS well-known key set retrieval
pub mod jwks;

const BEARER: &str = "Bearer ";

/// The token's Subject claim
#[derive(Clone)]
pub struct Subject(pub Option<String>);

/// If an authorization header is provided, make sure it's in the expected format, and
/// return it as a String.
fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<Option<String>, AuthError> {
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Ok(None),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(InvalidAuthHeaderError);
    }

    Ok(Some(auth_header.trim_start_matches(BEARER).to_owned()))
}

async fn authorize(
    jwks: &'static JWKS,
    headers: HeaderMap<HeaderValue>,
) -> Result<Subject, Rejection> {
    match jwt_from_header(&headers) {
        Ok(Some(jwt)) => {
            // First extract without verifying the header to locate the key-id (kid)
            let token = JWT::<Empty, Empty>::new_encoded(&jwt);

            let header: Header<Empty> = token
                .unverified_header()
                .map_err(|err| reject::custom(JWTTokenError(err)))?;

            let key_id = header
                .registered
                .key_id
                .ok_or_else(|| reject::custom(JWKSError))?;

            debug!("Fetching signing key for '{:?}'", key_id);

            // Now that we have the key, construct our RSA public key secret
            let secret =
                get_secret_from_key_set(jwks, &key_id).map_err(|_err| reject::custom(JWKSError))?;

            // Now fully verify and extract the token
            let token = token
                .into_decoded(&secret, SignatureAlgorithm::RS256)
                .map_err(|err| reject::custom(JWTTokenError(err)))?;

            let payload = token
                .payload()
                .map_err(|err| reject::custom(JWTTokenError(err)))?;
            let subject = payload.registered.subject.clone();

            debug!("Successfully verified token with subject: {:?}", subject);

            Ok(Subject(subject))
        }
        Ok(None) => Ok(Subject(None)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// pub(crate) fn reject_any(error: impl Into<anyhow::Error>) -> warp::Rejection {
//     warp::reject::custom(AnyReject(error.into()))
// }

/// A Warp Filter to add Authentication context
pub fn with_auth(
    jwks: &'static JWKS,
) -> impl Filter<Extract = (Subject,), Error = Rejection> + Clone {
    headers_cloned().and_then(move |headers: HeaderMap<HeaderValue>| authorize(jwks, headers))
}
