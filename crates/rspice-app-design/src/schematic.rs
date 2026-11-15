//! Schematic Editor Module
//!
//! Commercial-grade schematic capture editor components.
//! Provides the core schematic viewing, editing, and export functionality.
//!
//! - `view` - Main schematic canvas with pan/zoom and rendering
//! - `symbols` - Component symbol library and rendering
//!
//! # Architecture
//!
//! The schematic editor follows a Model-View pattern:
//! - State is managed in `crate::state::schematic::SchematicState`
//! - This module provides the egui view layer
//! - User interactions are translated to state mutations

pub use rspice_schematic_editor::symbols;
pub use rspice_schematic_editor::{
    ComponentCategory, ComponentPaletteEntry, PaletteRow, SupplyPaletteEntry, component_palette,
    device_class_keywords, palette_rows, supply_palette,
};
pub mod symbol_editor;
pub mod view;
