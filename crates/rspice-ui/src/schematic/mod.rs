//! Schematic Editor Module
//!
//! Commercial-grade schematic capture editor components.
//! Provides the core schematic viewing, editing, and export functionality.
//!
//! - `view` - Main schematic canvas with pan/zoom and rendering
//! - `export` - Export to SVG format (and other formats)
//! - `symbols` - Component symbol library and rendering
//!
//! # Architecture
//!
//! The schematic editor follows a Model-View pattern:
//! - State is managed in `crate::state::schematic::SchematicState`
//! - This module provides the egui view layer
//! - User interactions are translated to state mutations

pub(crate) mod bus_connectivity;
pub(crate) mod bus_geometry;
mod component_palette;
pub use component_palette::{ComponentPaletteEntry, ComponentPaletteSection, component_palette};
pub mod export;
mod source_labels;
pub(crate) mod symbol_editor;
pub mod symbols;
pub mod view;

// Re-export main types
pub use export::{SvgExportConfig, export_to_svg};
pub use symbols::{Symbol, SymbolLibrary, draw_symbol};
pub use view::render_schematic_view;
