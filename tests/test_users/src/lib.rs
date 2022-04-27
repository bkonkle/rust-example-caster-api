//! User and Profile tests

/// `User` factories
pub mod user_factory;

/// `Profile` factories
pub mod profile_factory;

/// `RoleGrant` factories
pub mod role_grant_factory;

#[cfg(test)]
mod test_profiles_service;

#[cfg(test)]
mod test_role_grants_service;

#[cfg(test)]
mod test_users_service;
