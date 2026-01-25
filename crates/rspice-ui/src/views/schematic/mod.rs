//! Schematic Editor View Module
//!
//! Interactive schematic capture with both SVG and GPU rendering options.
//! Supports pan, zoom, drag-to-move, undo/redo, and context menus.
//!
//! This module is organized into submodules:
//! - `types` - State types for drag, selection, editing, etc. (with tests)
//! - `utils` - Utility functions for geometry (with tests)
//! - `svg` - SVG rendering components (WireSvg, CompSvg, etc.) (with tests)
//! - `gpu_canvas` - GPU-accelerated canvas (event_handler, render_pass) (with tests)
//! - `handlers` - Event handling logic (keyboard bindings, etc.) (with tests)
//! - `toolbar` - Schematic toolbar component
//! - `annotation` - DC annotation layer
//! - `schematic` - Main Schematic component

mod annotation;
pub mod gpu_canvas;
pub mod handlers;
mod schematic;
pub mod svg;
mod toolbar;
pub mod types;
pub mod utils;

// Re-export the main components
pub use schematic::Schematic;
pub use toolbar::SchematicToolbar;

// Re-export GPU canvas components
pub use gpu_canvas::{GpuSchematicCanvas, GpuSchematicCanvasProps};

// Re-export SVG components for external use
pub use svg::{CompSvg, NetLabelSvg, PreviewSvg, WirePreviewSvg, WireSvg};

// Re-export handler types for external use
pub use handlers::{get_all_bindings, KeyBinding, KeyboardAction};
