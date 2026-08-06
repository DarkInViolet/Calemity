//! Transport-neutral API contracts for Calemity clients and hosts.
//!
//! This crate contains request, response, error, and event payload types.
//! It must not contain application, database, or transport-specific logic.

pub mod conversations;
pub mod error;
pub mod messages;
