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
mod hierarchy;
mod net_highlight;
mod net_label;
mod point;
mod port;
mod probe;
mod replacement;
mod rotation;
mod selection;
mod snap;
mod state;
mod symbol_gen;
mod tool;
mod undo_history;
mod validated_revision;
mod visibility;
mod wire;

// Re-exports. This block used to say "re-export all public types for backwards
// compatibility", which an application crate has no one to keep compatibility
// with -- nothing outside `rspice-ui` consumes it. A name earns a line here by
// having a caller; the rest are reachable through their own module.
pub use array::{
    SchematicArrayCount, SchematicArrayError, SchematicArrayImpact, SchematicArrayKind,
    SchematicArrayNameAtom, SchematicArrayNaming, SchematicArrayPlacement, SchematicArrayPlan,
    SchematicArrayPreview,
};
pub(crate) use bus::nearest_lattice_point_on_segment;
pub use bus::{
    Bus, BusDeclaration, BusDirection, BusNotation, BusParseError, BusPropertyImpact, BusSlice,
    BusTap, BusTapOrientation, BusTargetKind, MAX_BUS_MEMBER_INDEX, PendingBusTap,
};
pub use clipboard::ClipboardData;
pub use component::{
    Component, ComponentDisplayMode, LibraryCellInstance, validate_library_netlist_template,
};
pub use component_type::ComponentType;
pub use design_note::{
    DesignNote, DesignNoteKind, DesignNoteLayer, DesignNoteRenderContext, DesignReviewMutation,
    DesignReviewState, PendingDesignNotePlacement, RequirementTarget,
};
pub use document_policy::{
    NetNamingPolicy, OperatingPointAnnotationPolicy, PropertyCommitPolicy, SchematicDocumentPolicy,
    SchematicGridPitch, SelectionCrossingPolicy, WireJunctionPolicy,
};
pub(crate) use documentation_shape::clamped_documentation_shape_translation;
pub use documentation_shape::{
    DocumentationShape, DocumentationShapeError, DocumentationShapeGeometry,
    DocumentationShapeKind, DocumentationShapeLayer, PendingDocumentationShapePlacement,
    arc_parameters, geometry_from_points,
};
pub(crate) use hierarchy::SheetMoveConnectivityPlan;
pub use hierarchy::{
    HierarchyExtractionPlan, HierarchyExtractionTerminal, HierarchyNetConnectivity,
    hierarchy_terminal_direction, hierarchy_terminal_discipline,
};
pub use net_highlight::{NetGraph, NetHighlightState};
pub use net_label::{Junction, NetLabel};
pub use point::Point;
pub use port::{
    PendingPortPlacement, PortContract, PortDirection, PortDirectionType, PortDiscipline,
    PortSignalType, PortSpec,
};
pub use probe::SchematicProbe;
pub use replacement::{
    SchematicReplacementAuthority, SchematicReplacementCompatibility, SchematicReplacementError,
    SchematicReplacementImpact, SchematicReplacementMappingStatus, SchematicReplacementParameter,
    SchematicReplacementParameterMapping, SchematicReplacementPreview,
    SchematicReplacementSemanticStatus, SchematicReplacementSourceSpec,
    SchematicReplacementTargetSpec, SchematicReplacementTerminal,
    SchematicReplacementTerminalMapping, SchematicReplacementValuePolicy,
    SchematicReplacementWireEdit,
};
pub(crate) use replacement::{
    format_replacement_parameters, parse_replacement_parameters_strict,
    valid_replacement_parameter_name,
};
pub use rotation::Rotation;
pub use selection::{JunctionSelection, SchematicSelectionFilter, Selection};
pub use snap::SnapEngine;
pub use state::{
    MoveSelectionMode, SchematicState, StretchOrthogonalPolicy, StretchTarget,
};
pub use symbol_gen::generate_symbol;
pub use tool::Tool;
pub use undo_history::SchematicSnapshot;
pub use validated_revision::{
    AdvisoryDisposition, MAX_VALIDATED_REVISION_NOTE_LEN, ValidatedRevisionDependency,
    ValidatedRevisionJournal,
    ValidatedRevisionObjectDelta, ValidatedRevisionRequest, ValidatedRevisionSemanticDelta,
    ValidatedSchematicRevision, ValidatedSchematicRevisionId, ValidationFindingCounts,
};
pub use visibility::{
    GridStyle, SchematicAnnotationVisibility, SchematicBackAnnotationContent,
    SchematicHierarchyVisibility, SchematicNetHighlighting, SchematicParameterLabelVisibility,
    SchematicReviewMarkerVisibility, SchematicVisibilityPolicy, SchematicWireRoutingStyle,
};
pub use wire::{Wire, WireConnection, WireRoutingMode, WireSegment};
