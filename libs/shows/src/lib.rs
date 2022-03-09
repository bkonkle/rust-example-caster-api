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

/// Episode models
pub mod episode_model;

/// Authorization rules
pub const AUTHORIZATION: &str = include_str!("authorization.polar");

/// Show tests
#[cfg(test)]
mod tests;

/// Error macros
#[macro_use]
extern crate anyhow;
