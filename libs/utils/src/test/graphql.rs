use anyhow::Result;
use hyper::{Body, Method, Request};
use serde_json::json;

/// Utilities for testing graphql endpoints
pub struct GraphQL {
    url: String,
}

impl GraphQL {
    /// Construct a new GraphQL helper with a path to the endpoint
    pub fn new(url: String) -> Self {
        GraphQL { url }
    }

    /// Create a GraphQL query request for Hyper with an optional auth token
    pub fn create_query(&self, query: &str, token: Option<&str>) -> Result<Request<Body>> {
        let mut req = Request::builder().method(Method::POST).uri(&self.url);

        if let Some(token) = token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }

        let body = serde_json::to_string(&json!({ "query": query }))?;

        req.body(Body::from(body)).map_err(|err| err.into())
    }

    /// Create a query request with an auth token
    pub fn query(&self, query: &str, token: &str) -> Result<Request<Body>> {
        self.create_query(query, Some(token))
    }

    /// Create a query request without an auth token
    pub fn anon_query(&self, query: &str) -> Result<Request<Body>> {
        self.create_query(query, None)
    }

    /// Create a mutation request with an auth token
    pub fn mutation(&self, query: &str, token: &str) -> Result<Request<Body>> {
        self.create_query(query, Some(token))
    }

    /// Create a mutation request without an auth token
    pub fn anon_mutation(&self, query: &str) -> Result<Request<Body>> {
        self.create_query(query, None)
    }
}
