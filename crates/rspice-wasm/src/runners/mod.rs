//! Execution entry points.
//!
//! [`direct`] runs one explicitly configured analysis from JavaScript
//! arguments. [`deck`] executes an authored deck over its canonical
//! `DeckPlan` coordinate product. Both poll the same
//! [`crate::abort::ConfiguredAbort`].

pub mod deck;
pub mod direct;
