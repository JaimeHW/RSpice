//! Page-setup contracts and pagination.
//!
//! This module is the bottom of the hardcopy stack: what a project persists
//! about printing, and what pagination means. It deliberately contains no
//! egui, printer, filesystem, or browser integration. It turns an exact
//! document snapshot and a validated page setup into an immutable render
//! plan, deterministic preview pages, and a sealed execution receipt.
//! Physical geometry is represented in integer micrometres; neither
//! pagination nor content identities depend on host floating-point behavior.
//!
//! # Why hardcopy lives in two places
//!
//! Turning documents into artifacts needs the schematic symbol library, the
//! analysis viewers, the platform print APIs, and egui — so it sits near the
//! top of the crate. What a project *stores* about page setup needs none of
//! those, and `state` has to be able to persist it. Those are different
//! layers, so they are different modules:
//!
//! - `crate::hardcopy` (here) — the persisted contract and the pagination
//!   plan. Depends on `product` and nothing else.
//! - `crate::workbench::hardcopy` — document adapters, the deterministic
//!   scene renderer, the platform print boundary, and the dialogs.
//!
//! Splitting on persistence rather than on feature keeps `state` from having
//! to reach up into application chrome to describe its own saved data.

pub(crate) mod contract;
pub(crate) mod mappings;
pub(crate) mod sources;

pub use contract::*;
pub use mappings::{
    PrintMappingCatalogOwner, PrintMappingPersistenceError, PrintMappingPresetCatalog,
    PrintMappingSaveDisposition, PrintMappingSaveReceipt,
};
