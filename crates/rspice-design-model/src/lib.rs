//! Durable design-management model and the signed drawing-sheet package
//! contract built on top of it.
//!
//! Everything here is data and rules over data: identities, revisions,
//! semantic digests, sheet geometry, and the canonical package a publisher
//! signs. No rendering, no widgets, no engine. `rspice-ui` owns the editing
//! surfaces and re-exports this crate's types from the modules that used to
//! define them, so application paths are unchanged.
//!
//! The crate exists because two very different programs must agree on these
//! bytes exactly. `rspice-ui` imports a signed drawing-sheet package; the
//! offline `rspice-sheet-publisher` ceremony signs one. Sharing one
//! implementation is what makes "the publisher signed it" and "the importer
//! will accept it" the same statement — and it keeps the publisher's trusted
//! computing base down to serde, SHA-256, and Ed25519 rather than the whole
//! desktop application.

pub mod design_management;
pub mod primitives;
pub mod sheet_authoring;
pub mod sheet_package;

pub use design_management::*;
pub use primitives::*;
pub use sheet_authoring::{StartingFrame, custom_format, validate_dimensions};
