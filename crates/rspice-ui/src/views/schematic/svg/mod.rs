//! SVG Rendering Components for Schematic Editor
//!
//! This module provides the SVG components used to render schematic elements:
//! - `WireSvg` - Renders wire segments with selection and probe highlighting
//! - `CompSvg` - Renders components with labels, terminals, and drag support
//! - `PreviewSvg` - Ghost preview for component placement
//! - `NetLabelSvg` - Net label flag symbols
//! - `WirePreviewSvg` - Wire routing preview during drawing
//!
//! These components are pure rendering components that receive data via props
//! and use context providers for shared state like drag and label drag states.

mod component;
mod label;
mod preview;
mod wire;
mod wire_preview;

// Re-export all SVG components
pub use component::CompSvg;
pub use label::NetLabelSvg;
pub use preview::PreviewSvg;
pub use wire::WireSvg;
pub use wire_preview::WirePreviewSvg;

// Re-export utility functions
pub use component::rotate_point_by_deg;
pub use preview::symbol_path;
