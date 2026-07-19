//! Schematic State
//!
//! Main state container for the schematic editor.

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::bus::{Bus, BusDrawing, BusTap, PendingBusTap};
use super::clipboard::ClipboardData;
use super::component::{Component, LibraryCellInstance};
use super::component_type::ComponentType;
use super::design_note::{DesignNote, PendingDesignNotePlacement};
use super::document_policy::SchematicDocumentPolicy;
use super::documentation_shape::{
    DocumentationShape, DocumentationShapeDrawing, PendingDocumentationShapePlacement,
};
use super::net_label::{Junction, NetLabel};
use super::point::Point;
use super::port::PendingPortPlacement;
use super::rotation::Rotation;
use super::selection::Selection;
use super::snap::SnapEngine;
use super::tool::Tool;
use super::wire::{Wire, WireConnection, WireDrawing, WireSegment};

mod components;
mod editor_ops;
mod identity;
mod junction_ops;
mod selection_ops;
mod undo;
mod viewport;

// =============================================================================
// Constants
// =============================================================================

/// Default zoom level for serde deserialization (prevents black screen on file load)
fn default_zoom() -> f64 {
    1.0
}

/// Snap distance in grid units for terminal connections
const SNAP_DISTANCE: i32 = 1;

/// Connectivity policy applied while translating a schematic selection.
///
/// The mode is an explicit command input rather than persistent document
/// state: the same schematic can be edited under any mode without changing
/// its serialized meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MoveSelectionMode {
    /// Move attached conductor endpoints with the selected objects. This is
    /// the historical RSpice rubber-band behavior.
    #[default]
    Connected,
    /// Translate only selected objects. Attached unselected conductors remain
    /// fixed, intentionally breaking those electrical connections.
    BreakConnections,
    /// Preserve connections by finding a deterministic, overlap-free
    /// orthogonal route for every affected unselected wire.
    Shove,
}

impl MoveSelectionMode {
    pub const ALL: [Self; 3] = [Self::Connected, Self::BreakConnections, Self::Shove];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Connected => "Connected move",
            Self::BreakConnections => "Break connections",
            Self::Shove => "Move with shove",
        }
    }
}

/// A guarded move was rejected before any document mutation occurred.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveSelectionError {
    CoordinateOverflow,
    InvalidTapAttachment { tap_id: u64 },
    NonOrthogonalWire { wire_id: u64 },
    UnsupportedInteriorConnection { wire_id: u64 },
    AttachedTapCannotBePreserved { tap_id: u64 },
    NoLegalShoveRoute { wire_id: u64 },
    GeometryOverlap { object_id: u64 },
}

impl std::fmt::Display for MoveSelectionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CoordinateOverflow => {
                formatter.write_str("The requested move exceeds the schematic coordinate range.")
            }
            Self::InvalidTapAttachment { tap_id } => write!(
                formatter,
                "Bus tap {tap_id} would no longer lie on its declared source bus."
            ),
            Self::NonOrthogonalWire { wire_id } => write!(
                formatter,
                "Wire {wire_id} is not orthogonal and cannot participate in a connectivity-preserving move."
            ),
            Self::UnsupportedInteriorConnection { wire_id } => write!(
                formatter,
                "Wire {wire_id} has an interior terminal connection that cannot be shoved safely."
            ),
            Self::AttachedTapCannotBePreserved { tap_id } => write!(
                formatter,
                "Bus tap {tap_id} cannot remain attached to the shoved conductor."
            ),
            Self::NoLegalShoveRoute { wire_id } => write!(
                formatter,
                "No bounded overlap-free orthogonal route is available for wire {wire_id}."
            ),
            Self::GeometryOverlap { object_id } => write!(
                formatter,
                "Moving object {object_id} would overlap existing routed geometry."
            ),
        }
    }
}

impl std::error::Error for MoveSelectionError {}

/// Geometry policy applied while stretching one selected schematic segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StretchOrthogonalPolicy {
    /// Retain orthogonal source and affected geometry. The requested motion
    /// must be perpendicular to the selected segment.
    #[default]
    PreserveOrthogonal,
    /// Permit diagonal adjacent segments while retaining every fixed anchor.
    AllowDiagonal,
}

impl StretchOrthogonalPolicy {
    pub const ALL: [Self; 2] = [Self::PreserveOrthogonal, Self::AllowDiagonal];

    pub const fn label(self) -> &'static str {
        match self {
            Self::PreserveOrthogonal => "Preserve orthogonal",
            Self::AllowDiagonal => "Allow diagonal",
        }
    }
}

/// Stable identity of the exact geometry handle stretched by one command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StretchTarget {
    WireSegment {
        wire_id: u64,
        segment_index: usize,
    },
    BusSegment {
        bus_id: u64,
        segment_index: usize,
    },
    /// One typed control point of a documentation/parameterized shape.
    DocumentationShapePoint {
        shape_id: u64,
        point_index: usize,
    },
}

/// A stretch was rejected before any document mutation occurred.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StretchSelectionError {
    StaleTarget,
    CoordinateOverflow,
    DegenerateGeometry { object_id: u64 },
    NonOrthogonalSource { object_id: u64 },
    PerpendicularDeltaRequired,
    FixedAnchor { point: Point },
    NetLabelAnchor { label_id: u64, point: Point },
    ConnectedTerminal { component_id: u64, point: Point },
    InvalidTapAttachment { tap_id: u64 },
    ConductorOverlap { object_id: u64, other_id: u64 },
    UnintendedConductorContact { object_id: u64, other_id: u64 },
    UnintendedTerminalContact { object_id: u64, component_id: u64 },
    ComponentBodyEntry { object_id: u64, component_id: u64 },
    InvalidDocumentationGeometry { shape_id: u64 },
}

impl std::fmt::Display for StretchSelectionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StaleTarget => formatter
                .write_str("The selected stretch handle no longer exists in the active schematic."),
            Self::CoordinateOverflow => {
                formatter.write_str("The requested stretch exceeds the schematic coordinate range.")
            }
            Self::DegenerateGeometry { object_id } => write!(
                formatter,
                "Stretching object {object_id} would create a zero-length or invalid segment."
            ),
            Self::NonOrthogonalSource { object_id } => write!(
                formatter,
                "Object {object_id} has non-orthogonal affected geometry; choose Allow diagonal to stretch it."
            ),
            Self::PerpendicularDeltaRequired => formatter.write_str(
                "Preserve orthogonal requires motion perpendicular to the selected segment.",
            ),
            Self::FixedAnchor { point } => write!(
                formatter,
                "The selected segment contains a fixed conductor or junction anchor at ({}, {}).",
                point.x, point.y
            ),
            Self::NetLabelAnchor { label_id, point } => write!(
                formatter,
                "Net label {label_id} is an electrical anchor at ({}, {}); stretch cannot move away from or under it.",
                point.x, point.y
            ),
            Self::ConnectedTerminal {
                component_id,
                point,
            } => write!(
                formatter,
                "Component {component_id} owns a connected terminal at ({}, {}); its anchor cannot be stretched.",
                point.x, point.y
            ),
            Self::InvalidTapAttachment { tap_id } => write!(
                formatter,
                "Bus tap {tap_id} cannot retain both its declared source and destination attachments."
            ),
            Self::ConductorOverlap {
                object_id,
                other_id,
            } => write!(
                formatter,
                "Stretching object {object_id} would overlap conductor {other_id}."
            ),
            Self::UnintendedConductorContact {
                object_id,
                other_id,
            } => write!(
                formatter,
                "Stretching object {object_id} would create an unintended contact with conductor {other_id}."
            ),
            Self::UnintendedTerminalContact {
                object_id,
                component_id,
            } => write!(
                formatter,
                "Stretching object {object_id} would contact a terminal on component {component_id}."
            ),
            Self::ComponentBodyEntry {
                object_id,
                component_id,
            } => write!(
                formatter,
                "Stretching object {object_id} would route through component {component_id}."
            ),
            Self::InvalidDocumentationGeometry { shape_id } => write!(
                formatter,
                "Moving that control point would make documentation shape {shape_id} invalid."
            ),
        }
    }
}

impl std::error::Error for StretchSelectionError {}

// =============================================================================
// SchematicState
// =============================================================================

/// Main schematic state
///
/// Contains all components, wires, selection state, and interaction state
/// for a single schematic document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchematicState {
    /// All placed components
    pub components: Vec<Component>,

    /// All wires
    pub wires: Vec<Wire>,

    /// Durable multi-conductor bus polylines.
    #[serde(default)]
    pub buses: Vec<Bus>,

    /// Typed scalar and slice taps attached to declared buses.
    #[serde(default)]
    pub bus_taps: Vec<BusTap>,

    /// Durable non-electrical schematic documentation objects.
    #[serde(default)]
    pub design_notes: Vec<DesignNote>,

    /// Durable non-electrical lines, boundaries, arcs, polygons, and callouts.
    #[serde(default)]
    pub documentation_shapes: Vec<DocumentationShape>,

    /// Current selection
    pub selection: Selection,

    /// Current tool (runtime state, not persisted - always starts as Select)
    #[serde(skip)]
    pub tool: Tool,

    /// Wire drawing state
    pub wire_drawing: WireDrawing,

    /// Bus drawing state (runtime only; unfinished gestures never persist).
    #[serde(skip)]
    pub bus_drawing: BusDrawing,

    /// Grid size in pixels
    pub grid_size: i32,

    /// Project-portable editor semantics resolved when this document was
    /// created. Legacy schematics receive the reviewed default policy.
    #[serde(default)]
    pub document_policy: SchematicDocumentPolicy,

    /// Zoom level (1.0 = 100%) - not part of undo history or saved files
    /// Uses default of 1.0 when deserializing to prevent black screen
    #[serde(skip, default = "default_zoom")]
    pub zoom: f64,

    /// Pan offset in pixels - not part of undo history or saved files
    #[serde(skip, default)]
    pub pan: (f64, f64),

    /// Current schematic file path (for save without dialog)
    #[serde(skip)]
    pub current_file: Option<PathBuf>,

    /// Next component ID (runtime state, not persisted)
    #[serde(skip)]
    next_id: u64,

    /// Component counters for auto-naming (runtime state, not persisted)
    #[serde(skip)]
    component_counters: HashMap<&'static str, u32>,

    /// Clipboard for copy/paste operations
    pub clipboard: ClipboardData,

    /// Net labels for naming nodes
    #[serde(default)]
    pub net_labels: Vec<NetLabel>,

    /// Explicit wire junctions for connecting crossing wires
    /// Only wires sharing an endpoint OR joined by an explicit junction are connected
    pub junctions: Vec<Junction>,

    /// Preview rotation for component placement
    pub preview_rotation: Rotation,

    /// Pending library/cell/view placement payload used with `Tool::Place(CellInstance)`.
    ///
    /// Runtime interaction state only and never persisted to schematic files.
    #[serde(skip)]
    pub pending_library_cell: Option<LibraryCellInstance>,

    /// Validated configuration used while `Tool::BusTap` is armed.
    #[serde(skip)]
    pub pending_bus_tap: Option<PendingBusTap>,

    /// Validated one-shot interface contract used while the port tool is armed.
    #[serde(skip)]
    pub pending_port: Option<PendingPortPlacement>,

    /// Validated one-shot documentation object used while the text tool is armed.
    #[serde(skip)]
    pub pending_design_note: Option<PendingDesignNotePlacement>,

    /// Validated shape kind and document authority used while the shape tool is armed.
    #[serde(skip)]
    pub pending_documentation_shape: Option<PendingDocumentationShapePlacement>,

    /// Uncommitted click sequence for the active documentation-shape gesture.
    #[serde(skip)]
    pub documentation_shape_drawing: DocumentationShapeDrawing,

    /// Wire-to-terminal connections (for rubber-banding)
    pub connections: Vec<WireConnection>,

    /// Cached point-to-net mapping from last netlist generation (for probe lookup)
    /// Updated after each simulation run
    #[serde(skip)]
    pub net_mapping: HashMap<Point, String>,

    /// Flag indicating unsaved changes (runtime state, not persisted)
    #[serde(skip)]
    pub is_dirty: bool,

    /// Flag indicating zoom_to_fit should be called after next render with actual viewport dimensions.
    /// Set to true when loading a file, cleared after the fit is performed.
    #[serde(skip)]
    pub needs_fit: bool,

    /// One-shot request to pan the view so this schematic-space point sits at
    /// the canvas center (violation cycling). Consumed on the next render,
    /// like `needs_fit`; the zoom level is left alone.
    #[serde(skip)]
    pub center_request: Option<Point>,

    /// The open view belongs to a read-only library — inspection only.
    /// Set by the workspace loader; every edit path refuses while it holds.
    #[serde(skip)]
    pub read_only: bool,

    /// Flag indicating the undo history should be reset (e.g., after loading a file).
    /// Set to true when a file is loaded, cleared after history is reset.
    #[serde(skip)]
    pub needs_history_reset: bool,

    /// Topology version counter for cache invalidation (runtime state, not persisted)
    /// Incremented on any structural change (add/remove/move component/wire/junction)
    /// Used by LabelPositionCache and JunctionCache to detect stale data
    #[serde(skip)]
    topology_version: u64,

    /// Snap engine configuration (runtime state, not persisted)
    /// Controls cursor snapping behavior during wire drawing
    #[serde(skip)]
    pub snap_engine: SnapEngine,

    /// Rubber-band box selection rectangle (runtime state, not persisted)
    /// Used for drag-to-select operations
    #[serde(skip)]
    pub selection_rect: super::selection::SelectionRect,

    /// Net highlighting state (runtime state, not persisted)
    /// Tracks which wires are part of the highlighted net
    #[serde(skip)]
    pub net_highlight: super::net_highlight::NetHighlightState,

    /// Undo/redo history (runtime state, not persisted)
    /// Manages snapshots for undo/redo operations
    #[serde(skip)]
    pub undo_history: super::undo_history::UndoHistory,

    /// Frame-coherent canvas geometry (culling bounds, hover hit-test index).
    /// Rebuilt when `topology_version` advances; resets on clone.
    #[serde(skip)]
    pub(super) canvas_cache: super::canvas_cache::CanvasCache,
}

impl Default for SchematicState {
    fn default() -> Self {
        Self {
            components: Vec::new(),
            wires: Vec::new(),
            buses: Vec::new(),
            bus_taps: Vec::new(),
            design_notes: Vec::new(),
            documentation_shapes: Vec::new(),
            selection: Selection::default(),
            tool: Tool::default(),
            wire_drawing: WireDrawing::default(),
            bus_drawing: BusDrawing::default(),
            grid_size: 10,
            document_policy: SchematicDocumentPolicy::default(),
            zoom: 1.0,
            pan: (0.0, 0.0),
            current_file: None,
            next_id: 1,
            component_counters: HashMap::new(),
            clipboard: ClipboardData::default(),
            net_labels: Vec::new(),
            junctions: Vec::new(),
            preview_rotation: Rotation::default(),
            pending_library_cell: None,
            pending_bus_tap: None,
            pending_port: None,
            pending_design_note: None,
            pending_documentation_shape: None,
            documentation_shape_drawing: DocumentationShapeDrawing::default(),
            connections: Vec::new(),
            net_mapping: HashMap::new(),
            is_dirty: false,
            needs_fit: false,
            center_request: None,
            read_only: false,
            needs_history_reset: false,
            topology_version: 0,
            snap_engine: SnapEngine::default(),
            selection_rect: super::selection::SelectionRect::default(),
            net_highlight: super::net_highlight::NetHighlightState::default(),
            undo_history: super::undo_history::UndoHistory::default(),
            canvas_cache: super::canvas_cache::CanvasCache::default(),
        }
    }
}
