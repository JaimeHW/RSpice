//! Schematic presentation over canonical design data and explicit view settings.

pub mod annotations;
pub mod bus_geometry;
pub mod bus_tap_placement;
mod component_palette;
pub mod export;
pub mod net_label_placement;
pub mod pin_placement;
pub mod port_overlay;
pub mod selection_forms;
pub mod session;
pub mod source_labels;
pub mod symbol_editor;
pub mod symbols;
pub mod vector_preview;
pub mod view;
pub mod xspice_placement;

pub use component_palette::{ComponentPaletteEntry, ComponentPaletteSection, component_palette};
pub use symbols::SymbolLibrary;

pub mod requests;
