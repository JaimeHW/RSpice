//! Durable design-management model and the drawing-sheet package contract
//! built on top of it.
//!
//! Everything here is data and rules over data: identities, revisions,
//! semantic digests, sheet geometry, and the canonical preset package. No
//! rendering, no widgets, no engine. `rspice-app` owns the editing surfaces
//! and re-exports this crate's types from the modules that used to define
//! them, so application paths are unchanged.

pub mod bus;
pub mod cell_view;
pub mod design_management;
pub mod port;
pub mod primitives;
pub mod sheet_authoring;
pub mod sheet_package;
pub mod symbol_geometry;
pub mod symbol_pin;
#[cfg(feature = "work-observation")]
pub mod work_tally;

pub use design_management::*;
pub use primitives::*;
pub use sheet_authoring::{StartingFrame, custom_format, validate_dimensions};
