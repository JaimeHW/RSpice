//! Schematic presentation over canonical design data and explicit view settings.

pub mod annotations;
pub mod bus_geometry;
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
pub mod view;

pub use component_palette::{ComponentPaletteEntry, ComponentPaletteSection, component_palette};
pub use symbols::SymbolLibrary;

pub mod requests;
