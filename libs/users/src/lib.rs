//! # The Users Library
#![forbid(unsafe_code)]

/// The GraphQL Resolver
pub mod users_resolver;

/// The Users entity repository
pub mod users_repository;

/// The Users entity service
pub mod users_service;

/// The User model
pub mod user_model;

/// The User mutations
pub mod user_mutations;

/// The Profile model
pub mod profile_model;

/// Error macros
#[macro_use]
extern crate anyhow;
