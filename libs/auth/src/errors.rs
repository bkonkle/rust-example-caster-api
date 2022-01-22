use hyper::StatusCode;
use thiserror::Error;

/// Expected Error Cases
#[derive(Error, Debug)]
pub enum AuthError {
    /// The Authorizat ion header is not valid
    #[error("Invalid Authorization header")]
    InvalidAuthHeaderError,

    /// An error occurred while attempting to decode the token
    #[error("Invalid JWT")]
    JWTTokenError(biscuit::errors::Error),

    /// An error occured while attempting to identify the key id
    #[error("JWK verification failed")]
    JWKSError,
}

/// Get error codes and messages from `AuthError` instances
pub fn from_auth_error(err: &AuthError) -> (String, hyper::StatusCode) {
    match err {
        AuthError::JWKSError => (err.to_string(), StatusCode::UNAUTHORIZED),
        AuthError::JWTTokenError(err) => {
            (format!("JWTTokenError: {}", err), StatusCode::BAD_REQUEST)
        }
        _ => (err.to_string(), StatusCode::BAD_REQUEST),
    }
}

impl warp::reject::Reject for AuthError {}
