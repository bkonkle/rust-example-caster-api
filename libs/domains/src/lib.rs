//! # Domains
#![forbid(unsafe_code)]

/// Users
pub mod users;

/// Profiles
pub mod profiles;

/// Role Grants
pub mod role_grants;

/// Shows
pub mod shows;

/// Episodes
pub mod episodes;

/// Error macros
#[macro_use]
extern crate anyhow;
