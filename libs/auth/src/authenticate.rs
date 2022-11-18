//! # The Auth Library
#![forbid(unsafe_code)]

use biscuit::{jwa::SignatureAlgorithm, jws::Header, Empty, JWT};
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, Filter, Rejection,
};

use crate::errors::{
    AuthError,
    AuthError::{InvalidAuthHeaderError, JWKSError, JWTTokenError},
};
use crate::jwks::{get_secret_from_key_set, JWKS};

const BEARER: &str = "Bearer ";

/// The token's Subject claim, which corresponds with the username
#[derive(Clone)]
pub struct Subject(pub Option<String>);

/// A Warp Filter to add Authentication context
#[cfg(not(feature = "integration"))]
pub fn with_auth(
    jwks: &'static JWKS,
) -> impl Filter<Extract = (Subject,), Error = Rejection> + Clone {
    headers_cloned().and_then(move |headers: HeaderMap<HeaderValue>| authenticate(jwks, headers))
}

#[cfg(feature = "integration")]
pub use test::with_test_auth as with_auth;

/// If an authorization header is provided, make sure it's in the expected format, and
/// return it as a String.
pub fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<Option<String>, AuthError> {
    let header = if let Some(v) = headers.get(AUTHORIZATION) {
        v
    } else {
        // No Authorization header found, so return early with None
        return Ok(None);
    };

    let auth_header = if let Ok(v) = std::str::from_utf8(header.as_bytes()) {
        v
    } else {
        // Authorization header couldn't be decoded, so return early with None
        return Ok(None);
    };

    if !auth_header.starts_with(BEARER) {
        // Authorization header doesn't start with "Bearer ", so return early with an Error
        return Err(InvalidAuthHeaderError);
    }

    Ok(Some(auth_header.trim_start_matches(BEARER).to_owned()))
}

#[allow(dead_code)]
async fn authenticate(
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

#[cfg(feature = "integration")]
mod test {
    #![allow(dead_code)]

    use super::*;

    /// A Warp Filter to add Authentication context
    pub fn with_test_auth(
        _jwks: &'static JWKS,
    ) -> impl Filter<Extract = (Subject,), Error = Rejection> + Clone {
        headers_cloned().and_then(test_authenticate)
    }

    async fn test_authenticate(headers: HeaderMap<HeaderValue>) -> Result<Subject, Rejection> {
        match jwt_from_header(&headers) {
            Ok(Some(jwt)) => {
                let token = JWT::<Empty, Empty>::new_encoded(&jwt);

                let payload = token
                    .unverified_payload()
                    .map_err(|err| reject::custom(AuthError::JWTTokenError(err)))?;

                // Skip JWKS verification since this is testing

                let subject = payload.registered.subject;

                Ok(Subject(subject))
            }
            Ok(None) => Ok(Subject(None)),
            Err(e) => Err(warp::reject::custom(e)),
        }
    }
}
