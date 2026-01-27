//! Schematic Editor Module
//!
//! Commercial-grade schematic capture editor components.
//! Provides the core schematic viewing, editing, and export functionality.
//!
//! - `view` - Main schematic canvas with pan/zoom and rendering
//! - `export` - Export to SVG format (and other formats)
//! - `toolbar` - Schematic editing toolbar
//! - `symbols` - Component symbol library and rendering
//!
//! # Architecture
//!
//! The schematic editor follows a Model-View pattern:
//! - State is managed in `crate::state::schematic::SchematicState`
//! - This module provides the egui view layer
//! - User interactions are translated to state mutations

pub mod export;
pub mod symbols;
pub mod toolbar;
pub mod view;

// Re-export main types
pub use export::{export_to_svg, SvgExportConfig};
pub use symbols::{draw_symbol, Symbol, SymbolLibrary};
pub use toolbar::render_toolbar;
pub use view::render_schematic_view;
