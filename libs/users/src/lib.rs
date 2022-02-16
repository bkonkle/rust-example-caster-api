//! # The Users Library
#![forbid(unsafe_code)]

/// The GraphQL Users Resolver
pub mod users_resolver;

/// The Users entity service
pub mod users_service;

/// The User model
pub mod user_model;

/// The User mutations
pub mod user_mutations;

/// The GraphQL Profiles Resolver
pub mod profiles_resolver;

/// The Profiles entity service
pub mod profiles_service;

/// The Profile model
pub mod profile_model;

/// The Profile queries
pub mod profile_queries;

/// The Profile mutations
pub mod profile_mutations;

/// Profile utilities
pub mod profile_utils;

/// Error macros
#[macro_use]
extern crate anyhow;
