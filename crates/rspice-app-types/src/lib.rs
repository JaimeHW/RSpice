//! Portable identities and shared value contracts for the RSpice application.
//!
//! This crate contains no GUI or simulation runtime. Domain-specific models
//! remain with their owners; only values crossing those boundaries live here.

pub mod canonical;
pub mod hierarchy_path;
pub mod product;
pub mod property;
pub mod quantity;
pub mod raw_probe;
pub mod source_revision;
pub mod text_validation;
