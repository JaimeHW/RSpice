//! Portable model definitions, typed metadata, process corners, and correlation evidence.
//!
//! Catalog authority, source import, and further qualification contracts will join this owner
//! as their app-facing dependencies are separated.

mod authoring;
mod corner;
pub mod correlation;
mod definition_metadata;
mod model;
pub mod qualification;
mod types;

pub use authoring::*;
pub use corner::*;
pub use definition_metadata::*;
pub use model::*;
pub use types::*;
