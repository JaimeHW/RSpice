//! Portable model definitions, typed metadata, and process corners.
//!
//! Catalog authority, source import, and qualification will join this owner
//! as their app-facing dependencies are separated.

mod authoring;
mod corner;
mod definition_metadata;
mod model;
mod types;

pub use authoring::*;
pub use corner::*;
pub use definition_metadata::*;
pub use model::*;
pub use types::*;
