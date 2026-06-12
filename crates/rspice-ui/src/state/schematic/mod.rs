//! Schematic State Module
//!
//! Data structures for the schematic capture editor.
//! Manages components, wires, selection, and interaction state.
//!
//! This module is split into focused submodules for maintainability:
//! - `point` - Grid-aligned coordinates
//! - `rotation` - Component rotation
//! - `component_type` - Component type enumeration
//! - `component` - Component struct
//! - `wire` - Wire and wire drawing state
//! - `selection` - Selection management
//! - `tool` - Current interaction tool
//! - `clipboard` - Copy/paste support
//! - `net_label` - Net labels and junctions
//! - `state` - Main SchematicState

mod canvas_cache;
mod clipboard;
mod component;
mod component_type;
mod net_highlight;
mod net_label;
mod point;
mod port;
mod rotation;
mod selection;
mod snap;
mod state;
mod symbol_gen;
mod tool;
mod undo_history;
mod wire;

// Re-export all public types for backwards compatibility
pub use canvas_cache::CanvasCache;
pub use clipboard::ClipboardData;
pub use component::{Component, LibraryCellInstance};
pub use component_type::ComponentType;
pub use net_highlight::{NetGraph, NetHighlightState};
pub use net_label::{Junction, NetLabel};
pub use point::{LabelPosition, Point};
pub use port::{PortDirection, PortSpec};
pub use rotation::Rotation;
pub use selection::{JunctionSelection, Selection, WireSegmentSelection, WireVertexSelection};
pub use snap::{SnapEngine, SnapResult, SnapTarget, SnapTargetType};
pub use state::SchematicState;
pub use symbol_gen::{GeneratedPin, GeneratedSymbol, generate_symbol};
pub use tool::Tool;
pub use undo_history::{MAX_UNDO_STEPS, SchematicSnapshot, UndoHistory};
pub use wire::{
    // Connection types
    ConnectionSet,
    DragConstraint,
    JunctionType,
    // Core types
    Wire,
    WireConnection,
    WireDragContext,
    // Drag types
    WireDragMode,
    WireDrawing,
    WireHitResult,
    WireRoutingMode,
    // Segment types
    WireSegment,
    convert_to_orthogonal,
    count_bends,
    // Convenience functions
    create_wire,
    find_wire_intersections,
    is_valid_route,
    // Routing utilities
    optimize_route,
    route_length,
    wires_connected,
};
