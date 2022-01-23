use async_graphql::{Error, ErrorExtensions};
use hyper::StatusCode;

/// A convenience function to create a GraphQL error with predictable extension props
pub fn graphql_error(message: &'static str, code: StatusCode) -> Error {
    anyhow!(message).extend_with(|_err, e| {
        e.set("code", code.as_u16());
        e.set("message", code.to_string());
    })
}

/// A convenience function to create a GraphQL error from an existing error, intended to be
/// used with `.map_err()`
pub fn as_graphql_error(
    message: &'static str,
    code: StatusCode,
) -> Box<dyn Fn(anyhow::Error) -> Error> {
    Box::new(move |err| {
        anyhow!(message).extend_with(|_err, e| {
            e.set("code", code.as_u16());
            e.set("message", code.to_string());
            e.set("reason", err.to_string());
        })
    })
}
