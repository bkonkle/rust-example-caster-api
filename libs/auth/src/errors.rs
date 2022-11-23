use axum::response::{IntoResponse, Response};
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

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::JWKSError => (StatusCode::UNAUTHORIZED, self.to_string()).into_response(),
            AuthError::JWTTokenError(err) => {
                (StatusCode::BAD_REQUEST, format!("JWTTokenError: {}", err)).into_response()
            }
            AuthError::InvalidAuthHeaderError => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
        }
    }
}
