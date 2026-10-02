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

use rspice_design::schematic::array;
use rspice_design::schematic::array_edit;
pub(crate) mod bulk_edit;
mod bus;
mod canvas_cache;
use rspice_design::schematic::clipboard;
use rspice_design::schematic::clipboard_edit;
use rspice_design::schematic::history as committed_history;
mod component;
mod component_references;
#[cfg(test)]
use rspice_design::schematic::component_references as reference_edit;
use rspice_design::schematic::component_type;
use rspice_design::schematic::deletion;
mod design_note;
mod design_projection;
use rspice_design::schematic::device_catalog;
use rspice_design::schematic::document;
use rspice_design::schematic::document_policy;
mod documentation_shape;
use rspice_design::schematic::generated_veriloga_catalog;
mod ground_names;
mod hierarchy;
use rspice_design::schematic::hierarchy as hierarchy_edit;
pub(crate) mod named_net;
use rspice_design::schematic::net_label;
use rspice_schematic_editor::session::net_highlight;
use rspice_schematic_editor::session::placement_authority;
mod point;
mod port;
use rspice_design::schematic::probe;
use rspice_design::schematic::replacement;
use rspice_design::schematic::rotation;
use rspice_schematic_editor::session::selection;
use rspice_schematic_editor::session::snap;
mod state;
use rspice_schematic_editor::session::tool;
mod undo_history;
mod validated_revision;
use rspice_schematic_editor::session::visibility;
use rspice_schematic_editor::session::wire;

// Re-exports. This block used to say "re-export all public types for backwards
// compatibility", which an application crate has no one to keep compatibility
// with -- nothing outside `rspice-ui` consumes it. A name earns a line here by
// having a caller; the rest are reachable through their own module.
pub use array::{
    SchematicArrayCount, SchematicArrayError, SchematicArrayImpact, SchematicArrayKind,
    SchematicArrayNaming, SchematicArrayPlacement, SchematicArrayPlan, SchematicArrayPreview,
};
pub use bus::{
    Bus, BusDeclaration, BusDirection, BusNotation, BusParseError, BusPropertyImpact, BusSlice,
    BusTap, BusTapOrientation, BusTargetKind, PendingBusTap, declared_vector, declared_width,
};
pub use component::{
    Component, ComponentDisplayMode, InstanceMultiplicity, LibraryCellInstance,
    explicit_component_model, validate_library_netlist_template,
};
pub use component_type::ComponentType;
pub use design_note::{
    DesignNote, DesignNoteKind, DesignNoteRenderContext, DesignReviewMutation, DesignReviewState,
    PendingDesignNotePlacement, RequirementTarget,
};
pub use device_catalog::{
    CatalogXspiceVectorPort, builtin_xspice_library_binding,
    builtin_xspice_library_binding_with_vector_widths, builtin_xspice_vector_ports,
    engine_only_xspice_devices, validate_builtin_xspice_binding,
};
pub use document_policy::{
    NetNamingPolicy, OperatingPointAnnotationPolicy, PropertyCommitPolicy, SchematicDocumentPolicy,
    SchematicGridPitch, SchematicPageOrientation, SchematicPageSize, SelectionCrossingPolicy,
    WireJunctionPolicy,
};
pub use documentation_shape::{
    DocumentationShape, DocumentationShapeError, DocumentationShapeGeometry,
    DocumentationShapeKind, PendingDocumentationShapePlacement, geometry_from_points,
};
pub use generated_veriloga_catalog::{
    generated_veriloga_devices, generated_veriloga_library_binding,
    validate_generated_veriloga_binding,
};
pub use ground_names::is_ground_reference;
pub(crate) use hierarchy::SheetMoveConnectivityPlan;
pub use hierarchy::{
    HierarchyExtractionPlan, HierarchyExtractionTerminal, HierarchyNetConnectivity,
    hierarchy_terminal_direction, hierarchy_terminal_discipline,
};
pub use net_highlight::NetHighlightState;
#[cfg(test)]
pub use net_label::Junction;
pub use net_label::{NetLabel, NetLabelKind};
pub use placement_authority::PlacementAuthority;
pub use point::Point;
#[cfg(test)]
pub use port::{PendingPortPlacement, PortDirectionType, PortSignalType};
pub use port::{PendingPortSequence, PortContract, PortDirection, PortDiscipline, PortSpec};
pub use probe::SchematicProbe;
pub(crate) use replacement::parse_replacement_parameters_strict;
pub use replacement::{
    SchematicReplacementAuthority, SchematicReplacementError, SchematicReplacementParameter,
    SchematicReplacementPreview, SchematicReplacementSourceSpec, SchematicReplacementTargetSpec,
    SchematicReplacementTerminal,
};
pub use rotation::Rotation;
#[cfg(test)]
pub use rspice_design::schematic::wire::WireConnection;
pub use selection::{
    DuplicateExternalNets, JunctionSelection, SchematicSelectionFilter, Selection,
};
pub use snap::{SnapEngine, SnapResult};
pub use state::{
    MoveSelectionMode, PendingStimulusPlacement, SchematicState, StretchOrthogonalPolicy,
    StretchTarget,
};
pub(crate) use state::{SchematicEditorMut, SchematicEditorRef, SchematicSession};
pub use tool::Tool;
pub use undo_history::{SchematicSnapshot, UndoSequence, next_undo_sequence};
pub use validated_revision::{
    AdvisoryDisposition, MAX_VALIDATED_REVISION_NOTE_LEN, ValidatedRevisionDependency,
    ValidatedRevisionJournal, ValidatedRevisionObjectDelta, ValidatedRevisionRequest,
    ValidatedRevisionSemanticDelta, ValidatedSchematicRevision, ValidatedSchematicRevisionId,
    ValidationFindingCounts,
};
pub use visibility::{
    DrawingSheetLayerVisibility, GridStyle, SchematicAnnotationVisibility,
    SchematicBackAnnotationContent, SchematicHierarchyVisibility, SchematicNetHighlighting,
    SchematicParameterLabelVisibility, SchematicReviewMarkerVisibility, SchematicVisibilityPolicy,
    SchematicWireRoutingStyle,
};
pub use wire::{Wire, WireRoutingMode, WireSegment};
