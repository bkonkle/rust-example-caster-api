use thiserror::Error;

/// Expected Error Cases
#[derive(Error, Debug)]
pub enum AuthError {
    /// The Authorization header is missing
    #[error("no auth header")]
    NoAuthHeaderError,

    /// The Authorization header is not valid
    #[error("invalid auth header")]
    InvalidAuthHeaderError,

    /// An error occurrend while attempting to decode the token
    #[error("jwt token not valid")]
    JWTTokenError(jsonwebtoken::errors::Error),
}

/// A wrapper for `anyhow::Error` to make it compatible with `warp::Rejection`
#[derive(Debug)]
struct AnyReject(anyhow::Error);

impl warp::reject::Reject for AuthError {}

impl warp::reject::Reject for AnyReject {}

pub(crate) fn reject_any(error: impl Into<anyhow::Error>) -> warp::Rejection {
    warp::reject::custom(AnyReject(error.into()))
}
