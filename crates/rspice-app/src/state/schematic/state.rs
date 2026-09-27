//! Schematic State
//!
//! Main state container for the schematic editor.

use std::path::PathBuf;

use rspice_design::schematic::identity::SchematicIdentity;
use serde::{Deserialize, Serialize};

use super::bus::{Bus, BusDrawing, BusTap, PendingBusTap};
use super::clipboard::ClipboardData;
use super::component::{Component, LibraryCellInstance};
use super::component_type::ComponentType;
use super::design_note::{DesignNote, PendingDesignNotePlacement};
use super::document::SchematicDocument;
use super::documentation_shape::{
    DocumentationShape, DocumentationShapeDrawing, PendingDocumentationShapePlacement,
};
use super::net_label::{Junction, NetLabel};
use super::point::Point;
use super::port::PendingPortSequence;
use super::rotation::Rotation;
use super::selection::Selection;
use super::snap::SnapEngine;
use super::tool::Tool;
use super::wire::{Wire, WireConnection, WireDrawing, WireSegment};

mod bus_edit;
mod components;
mod editor_ops;
mod identity;
mod junction_ops;
mod selection_ops;
#[cfg(test)]
mod serialization_tests;
mod stimulus_placement;
mod undo;
mod viewport;

pub use stimulus_placement::PendingStimulusPlacement;

// =============================================================================
// Constants
// =============================================================================

/// Default zoom level for serde deserialization (prevents black screen on file load)
fn default_zoom() -> f64 {
    1.0
}

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
    ProbeSelectionUnsupported,
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
            Self::ProbeSelectionUnsupported => formatter.write_str(
                "Probe markers cannot be stretched; move the retained probe marker instead.",
            ),
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

/// A model card armed for the next placement of one exact device kind.
///
/// The armed tool is recorded beside the card so re-arming any other tool
/// retires it. Without that, arming a resistor after arming a zener would
/// place a resistor still carrying the diode's card — a component that reads
/// as valid everywhere and netlists as nonsense.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingPartModel {
    /// The exact placement tool this card belongs to.
    pub tool: Tool,
    /// Model card name, as the signed manifest publishes it. It becomes the
    /// placed instance's value, which is where every native emitter reads it.
    pub model: String,
    /// A named symbol skin the device family ships, when the part asked for
    /// one.
    pub variant: Option<String>,
}

// =============================================================================
// SchematicState
// =============================================================================

/// Schematic editor state over one persisted design document.
#[derive(Debug, Clone)]
pub struct SchematicState {
    pub document: SchematicDocument,

    /// Current selection (runtime state, never part of the design document).
    pub selection: Selection,

    /// Current tool (runtime state, not persisted - always starts as Select)
    pub tool: Tool,

    /// Unfinished wire gesture (runtime state, never part of the design document).
    pub wire_drawing: WireDrawing,

    /// Bus drawing state (runtime only; unfinished gestures never persist).
    pub bus_drawing: BusDrawing,

    /// Zoom level (1.0 = 100%) - not part of undo history or saved files
    /// Uses default of 1.0 when deserializing to prevent black screen
    pub zoom: f64,

    /// Pan offset in pixels - not part of undo history or saved files
    pub pan: (f64, f64),

    /// Current schematic file path (for save without dialog)
    pub current_file: Option<PathBuf>,

    /// Object and reference-designator allocation (runtime only).
    identity: SchematicIdentity,

    /// Clipboard for copy/paste operations.
    ///
    /// Clipboard ownership is session-local. Persisting it in a `.rsch` file
    /// made an unrelated document reopen with stale copied design objects and
    /// could retain data that the author had never committed to the drawing.
    pub clipboard: ClipboardData,

    /// Preview rotation for component placement (runtime interaction state).
    pub preview_rotation: Rotation,

    /// Horizontal mirror for component-placement previews and commits.
    ///
    /// Runtime interaction state only. It is shared by click-armed placement
    /// and component-shelf drag placement so the preview always matches the
    /// object that will be committed.
    pub preview_mirror_h: bool,

    /// Pending library/cell/view placement payload used with `Tool::Place(CellInstance)`.
    ///
    /// Runtime interaction state only and never persisted to schematic files.
    pub pending_library_cell: Option<LibraryCellInstance>,

    /// Model card armed for the next native-device placement.
    ///
    /// Runtime interaction state only. A pack publishes model cards as native
    /// devices — a zener is a diode — so the card name and the family's symbol
    /// skin ride alongside the armed device kind rather than inside a cell
    /// binding the device does not have.
    pub pending_part_model: Option<PendingPartModel>,

    /// Stimulus definition armed for the next independent-source placement.
    ///
    /// Runtime interaction state only. A definition is a placeable part, so
    /// the instance it becomes is born holding the definition's card and the
    /// receipt for it; the payload rides beside the armed tool exactly as
    /// [`PendingPartModel`] does, and for the same reason.
    pub pending_stimulus: Option<PendingStimulusPlacement>,

    /// Validated configuration used while `Tool::BusTap` is armed.
    pub pending_bus_tap: Option<PendingBusTap>,

    /// Names still to place, and the contract they share, while the port tool
    /// is armed.
    pub pending_port_sequence: Option<PendingPortSequence>,

    /// Validated one-shot documentation object used while the text tool is armed.
    pub pending_design_note: Option<PendingDesignNotePlacement>,

    /// Validated shape kind and document authority used while the shape tool is armed.
    pub pending_documentation_shape: Option<PendingDocumentationShapePlacement>,

    /// Uncommitted click sequence for the active documentation-shape gesture.
    pub documentation_shape_drawing: DocumentationShapeDrawing,

    /// Flag indicating unsaved changes (runtime state, not persisted)
    pub is_dirty: bool,

    /// Flag indicating zoom_to_fit should be called after next render with actual viewport dimensions.
    /// Set to true when loading a file, cleared after the fit is performed.
    pub needs_fit: bool,

    /// One-shot request to frame the authored drawing-sheet paper boundary.
    ///
    /// This is deliberately distinct from [`Self::needs_fit`], which frames
    /// schematic objects and may include content parked off the paper.
    pub needs_drawing_sheet_fit: bool,

    /// One-shot request to pan the view so this schematic-space point sits at
    /// the canvas center (violation cycling). Consumed on the next render,
    /// like `needs_fit`; the zoom level is left alone.
    pub center_request: Option<Point>,

    /// The open view belongs to a read-only library — inspection only.
    /// Set by the workspace loader; every edit path refuses while it holds.
    pub read_only: bool,

    /// Flag indicating the undo history should be reset (e.g., after loading a file).
    /// Set to true when a file is loaded, cleared after history is reset.
    pub needs_history_reset: bool,

    /// Topology version counter for cache invalidation (runtime state, not persisted)
    /// Incremented on any structural change (add/remove/move component/wire/junction)
    /// Used by LabelPositionCache and JunctionCache to detect stale data
    topology_version: u64,

    /// Content commit counter (runtime state, not persisted). Advances at
    /// the undo commit boundaries — `end_operation`, `undo`, `redo` — so it
    /// moves exactly when the persisted document content moves, including
    /// property edits that `topology_version` deliberately ignores. Live
    /// sessions use it to decide which buffers need rebroadcasting.
    content_version: u64,

    /// Snap engine configuration (runtime state, not persisted)
    /// Controls cursor snapping behavior during wire drawing
    pub snap_engine: SnapEngine,

    /// Rubber-band box selection rectangle (runtime state, not persisted)
    /// Used for drag-to-select operations
    pub selection_rect: super::selection::SelectionRect,

    /// Net highlighting state (runtime state, not persisted)
    /// Tracks which wires are part of the highlighted net
    pub net_highlight: super::net_highlight::NetHighlightState,

    /// Undo/redo history (runtime state, not persisted)
    /// Manages snapshots for undo/redo operations
    pub undo_history: super::undo_history::UndoHistory,

    /// Frame-coherent canvas geometry (culling bounds, hover hit-test index).
    /// Rebuilt when `topology_version` advances; resets on clone.
    pub(super) canvas_cache: super::canvas_cache::CanvasCache,
}

impl Default for SchematicState {
    fn default() -> Self {
        Self::from_document(SchematicDocument::default())
    }
}

impl SchematicState {
    /// Create fresh editor state around an owned document. Saved-file loading
    /// retains its separate legacy runtime defaults in Deserialize.
    pub(crate) fn from_document(document: SchematicDocument) -> Self {
        let snap_engine = SnapEngine {
            grid_size: document.grid_size,
            ..SnapEngine::default()
        };
        Self {
            document,
            selection: Selection::default(),
            tool: Tool::default(),
            wire_drawing: WireDrawing::default(),
            bus_drawing: BusDrawing::default(),
            zoom: 1.0,
            pan: (0.0, 0.0),
            current_file: None,
            identity: SchematicIdentity::with_cursor(1),
            clipboard: ClipboardData::default(),
            preview_rotation: Rotation::default(),
            preview_mirror_h: false,
            pending_library_cell: None,
            pending_part_model: None,
            pending_stimulus: None,
            pending_bus_tap: None,
            pending_port_sequence: None,
            pending_design_note: None,
            pending_documentation_shape: None,
            documentation_shape_drawing: DocumentationShapeDrawing::default(),
            is_dirty: false,
            needs_fit: false,
            needs_drawing_sheet_fit: false,
            center_request: None,
            read_only: false,
            needs_history_reset: false,
            topology_version: 0,
            content_version: 0,
            snap_engine,
            selection_rect: super::selection::SelectionRect::default(),
            net_highlight: super::net_highlight::NetHighlightState::default(),
            undo_history: super::undo_history::UndoHistory::default(),
            canvas_cache: super::canvas_cache::CanvasCache::default(),
        }
    }
}

// Delegate the unchanged wire layout without copying the document. Runtime
// fields retain their original serde(skip) defaults when loading saved data.
impl Serialize for SchematicState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.document.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SchematicState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            document: SchematicDocument::deserialize(deserializer)?,
            selection: Default::default(),
            tool: Default::default(),
            wire_drawing: Default::default(),
            bus_drawing: Default::default(),
            zoom: default_zoom(),
            pan: Default::default(),
            current_file: Default::default(),
            identity: Default::default(),
            clipboard: Default::default(),
            preview_rotation: Default::default(),
            preview_mirror_h: Default::default(),
            pending_library_cell: Default::default(),
            pending_part_model: Default::default(),
            pending_stimulus: Default::default(),
            pending_bus_tap: Default::default(),
            pending_port_sequence: Default::default(),
            pending_design_note: Default::default(),
            pending_documentation_shape: Default::default(),
            documentation_shape_drawing: Default::default(),
            is_dirty: Default::default(),
            needs_fit: Default::default(),
            needs_drawing_sheet_fit: Default::default(),
            center_request: Default::default(),
            read_only: Default::default(),
            needs_history_reset: Default::default(),
            topology_version: Default::default(),
            content_version: Default::default(),
            snap_engine: Default::default(),
            selection_rect: Default::default(),
            net_highlight: Default::default(),
            undo_history: Default::default(),
            canvas_cache: Default::default(),
        })
    }
}

impl SchematicState {
    /// Reconcile every in-document runtime projection of the authoritative
    /// project-portable grid pitch.
    ///
    /// Call this after restoring history or installing session-owned snap
    /// target preferences into a newly activated document.
    pub(crate) fn reconcile_grid_pitch_runtime(&mut self) {
        let grid_size = self.document.document_policy.grid_pitch.canvas_grid_size();
        self.document.grid_size = grid_size;
        self.snap_engine.grid_size = grid_size;
    }
}
