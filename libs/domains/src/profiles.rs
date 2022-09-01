//! # Profiles
#![forbid(unsafe_code)]

/// Service
pub mod service;

/// Model
pub mod model;

/// GraphQL Queries
pub mod queries;

/// GraphQL Mutations
pub mod mutations;

/// GraphQL Resolver
pub mod resolver;

/// Authorization rules
pub const AUTHORIZATION: &str = include_str!("profiles/authorization.polar");

/// Tests
#[cfg(test)]
mod tests;
