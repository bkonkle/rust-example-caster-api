//! # The Shows Library
#![forbid(unsafe_code)]

/// The GraphQL resolver
pub mod shows_resolver;

/// The Shows service
pub mod shows_service;

/// Show models
pub mod show_model;

/// Show queries
pub mod show_queries;

/// Show mutations
pub mod show_mutations;

/// The GraphQL resolver
pub mod episodes_resolver;

/// The Episodes service
pub mod episodes_service;

/// Episode models
pub mod episode_model;

/// Episode queries
pub mod episode_queries;

/// Episode mutations
pub mod episode_mutations;

/// Authorization rules
pub const AUTHORIZATION: &str = include_str!("authorization.polar");

/// Show test factories
#[cfg(test)]
pub mod show_factory;

/// Episode test factories
#[cfg(test)]
pub mod episode_factory;

/// Error macros
#[macro_use]
extern crate anyhow;
