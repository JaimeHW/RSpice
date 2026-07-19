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
//! - `bus` - Typed buses and bus taps
//! - `net_label` - Net labels and junctions
//! - `state` - Main SchematicState

mod array;
mod bus;
mod canvas_cache;
mod clipboard;
mod component;
mod component_type;
mod design_note;
mod document_policy;
mod documentation_shape;
mod net_highlight;
mod net_label;
mod point;
mod port;
mod replacement;
mod rotation;
mod selection;
mod snap;
mod state;
mod symbol_gen;
mod tool;
mod undo_history;
mod wire;

// Re-export all public types for backwards compatibility
pub use array::{
    MAX_SCHEMATIC_ARRAY_MEMBERS, SchematicArrayCount, SchematicArrayError, SchematicArrayImpact,
    SchematicArrayKind, SchematicArrayNameAtom, SchematicArrayNameRange, SchematicArrayNaming,
    SchematicArrayPlacement, SchematicArrayPlan, SchematicArrayPreview,
};
pub(crate) use bus::nearest_lattice_point_on_segment;
pub use bus::{
    Bus, BusDeclaration, BusDirection, BusDrawing, BusMember, BusNotation, BusParseError,
    BusPropertyImpact, BusSlice, BusTap, BusTapOrientation, BusTargetKind, MAX_BUS_MEMBER_INDEX,
    PendingBusTap,
};
pub use canvas_cache::CanvasCache;
pub use clipboard::ClipboardData;
pub use component::{Component, LibraryCellInstance};
pub use component_type::ComponentType;
pub use design_note::{
    DesignNote, DesignNoteError, DesignNoteKind, DesignNoteLayer, DesignNotePlacementAuthority,
    DesignNoteRenderContext, DesignReviewRecord, DesignReviewState, MAX_DESIGN_NOTE_TEXT_LEN,
    PendingDesignNotePlacement, RequirementTarget,
};
pub use document_policy::{
    NetNamingPolicy, OperatingPointAnnotationPolicy, PropertyCommitPolicy, SchematicDocumentPolicy,
    SchematicGridPitch, SelectionCrossingPolicy, WireJunctionPolicy,
};
pub(crate) use documentation_shape::clamped_documentation_shape_translation;
pub use documentation_shape::{
    DocumentationShape, DocumentationShapeDrawing, DocumentationShapeError,
    DocumentationShapeGeometry, DocumentationShapeKind, DocumentationShapeLayer,
    DocumentationShapePlacementAuthority, MAX_DOCUMENTATION_POLYGON_POINTS,
    PendingDocumentationShapePlacement, arc_parameters, geometry_from_points,
};
pub use net_highlight::{NetGraph, NetHighlightState};
pub use net_label::{Junction, NetLabel};
pub use point::{LabelPosition, Point};
pub use port::{
    PendingPortPlacement, PortContract, PortDirection, PortDirectionType, PortDiscipline,
    PortPlacementAuthority, PortPlacementError, PortSignalType, PortSpec,
};
pub use replacement::{
    SchematicReplacementAuthority, SchematicReplacementCompatibility, SchematicReplacementError,
    SchematicReplacementImpact, SchematicReplacementMappingStatus, SchematicReplacementParameter,
    SchematicReplacementParameterMapping, SchematicReplacementPreview,
    SchematicReplacementSemanticStatus, SchematicReplacementSourceSpec,
    SchematicReplacementTargetSpec, SchematicReplacementTerminal,
    SchematicReplacementTerminalMapping, SchematicReplacementValuePolicy,
    SchematicReplacementWireEdit,
};
pub use rotation::Rotation;
pub use selection::{JunctionSelection, Selection, WireSegmentSelection, WireVertexSelection};
pub use snap::{SnapEngine, SnapResult, SnapTarget, SnapTargetType};
pub use state::{
    MoveSelectionError, MoveSelectionMode, SchematicState, StretchOrthogonalPolicy,
    StretchSelectionError, StretchTarget,
};
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
