//! Portable identities and shared value contracts for the RSpice application.
//!
//! This crate contains no GUI or simulation runtime. Domain-specific models
//! remain with their owners; only values crossing those boundaries live here.

pub mod canonical;
pub mod hierarchy_path;
pub mod product;
pub mod quantity;
pub mod source_revision;
