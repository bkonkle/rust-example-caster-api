//! # The Utils Library
#![forbid(unsafe_code)]

/// Config utilities based on config-rs
pub mod config;

/// GraphQL utilities
pub mod graphql;

/// Utilities for working with http/https requests
pub mod http;

/// Error helpers for GraphQL
pub mod errors;

/// Pagination utils
pub mod pagination;

/// Ordering utils
pub mod ordering;

#[macro_use]
extern crate anyhow;
