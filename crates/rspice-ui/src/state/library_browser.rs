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

mod cell;
mod library;
mod manager;
mod primitives;
mod view;

pub use cell::Cell;
pub use library::Library;
pub use manager::LibraryManager;
pub use view::{View, ViewType};
