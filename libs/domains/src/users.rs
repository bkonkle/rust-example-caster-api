//! # Users

/// Service
pub mod service;

/// Model
pub mod model;

/// GraphQL Mutations
pub mod mutations;

/// GraphQL Resolver
pub mod resolver;

/// Authorization rules
pub const AUTHORIZATION: &str = include_str!("users/authorization.polar");

/// Tests
#[cfg(test)]
mod tests;
