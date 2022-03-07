//! # The Shows Library
#![forbid(unsafe_code)]

/// The GraphQL resolver
pub mod shows_resolver;

/// The Shows service
pub mod shows_service;

/// The Show models
pub mod show_model;

/// The Show queries
pub mod show_queries;

/// The Show mutations
pub mod show_mutations;

/// Show tests
#[cfg(test)]
mod tests;

/// Error macros
#[macro_use]
extern crate anyhow;
