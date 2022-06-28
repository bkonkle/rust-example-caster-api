//! # The Users Library
#![forbid(unsafe_code)]

/// The GraphQL `User` Resolver
pub mod users_resolver;

/// The `User` entity service
pub mod users_service;

/// The `User` model
pub mod user_model;

/// The `User` mutations
pub mod user_mutations;

/// The GraphQL `Profile` Resolver
pub mod profiles_resolver;

/// The `Profile` entity service
pub mod profiles_service;

/// The `Profile` model
pub mod profile_model;

/// The `Profile` queries
pub mod profile_queries;

/// The `Profile` mutations
pub mod profile_mutations;

/// `RoleGrant` models
pub mod role_grant_model;

/// `RoleGrant` entity service
pub mod role_grants_service;

/// Authorization rules
pub const AUTHORIZATION: &str = include_str!("authorization.polar");

/// User test factories
#[cfg(test)]
pub mod user_factory;

/// Profile test factories
#[cfg(test)]
pub mod profile_factory;

/// Role Grant test factories
#[cfg(test)]
pub mod role_grant_factory;

/// Error macros
#[macro_use]
extern crate anyhow;
