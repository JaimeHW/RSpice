//! Library/Cell/View Browser
//!
//! Cadence-style hierarchical design library management system.
//!
//! # Architecture
//!
//! The Library/Cell/View (LCV) hierarchy mirrors Cadence Virtuoso:
//! - **Library**: A collection of design cells (e.g., `my_designs`)
//! - **Cell**: A single design unit (e.g., `opamp`, `bandgap`)
//! - **View**: A particular representation (e.g., `schematic`, `symbol`, `layout`)
//!
//! # Example Hierarchy
//!
//! ```text
//! my_library/
//! ├── opamp/
//! │   ├── schematic
//! │   ├── symbol
//! │   └── testbench
//! ├── bandgap/
//! │   ├── schematic
//! │   └── symbol
//! └── resistor/
//!     └── symbol
//! ```

mod placement;

pub use placement::{LibraryCellPlacementCandidate, library_cell_placement_candidates};
pub use rspice_design::library::{
    Cell, Library, ProjectLibraryEditLock, ProjectLibraryEditLockScope,
    ProjectLibraryLockAuthority, ProjectLibraryLockSnapshot, View, ViewType,
};
pub use rspice_project::ProjectLibraries as LibraryManager;
