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

/// The Profile mutations
pub mod profiles_repository;

/// The Profiles entity service
pub mod profiles_service;

/// The Profile model
pub mod profile_model;

/// The Profile mutations
pub mod profile_mutations;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate log;
