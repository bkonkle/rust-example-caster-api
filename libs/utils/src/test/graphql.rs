use anyhow::Result;
use hyper::{Body, Method, Request};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct GraphQLRequest {
    query: String,
}

/// Utilities for testing graphql endpoints
pub struct GraphQL {
    url: String,
}

impl GraphQL {
    /// Construct a new GraphQL helper with a path to the endpoint
    pub fn new(url: String) -> Self {
        GraphQL { url }
    }

    /// Create a GraphQL query request for Hyper
    pub fn query(&self, query: String, token: Option<String>) -> Result<Request<Body>> {
        let mut req = Request::builder().method(Method::POST).uri(&self.url);

        if let Some(token) = token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }

        let body = serde_json::to_string(&GraphQLRequest { query })?;

        req.body(Body::from(body)).map_err(|err| err.into())
    }

    /// An alias for "query"
    pub fn mutation(&self, query: String, token: Option<String>) -> Result<Request<Body>> {
        self.query(query, token)
    }
}
