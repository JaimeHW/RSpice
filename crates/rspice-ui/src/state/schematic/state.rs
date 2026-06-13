//! Schematic State
//!
//! Main state container for the schematic editor.

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::clipboard::ClipboardData;
use super::component::{Component, LibraryCellInstance};
use super::component_type::ComponentType;
use super::net_label::{Junction, NetLabel};
use super::point::Point;
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

    /// Current selection
    pub selection: Selection,

    /// Current tool (runtime state, not persisted - always starts as Select)
    #[serde(skip)]
    pub tool: Tool,

    /// Wire drawing state
    pub wire_drawing: WireDrawing,

    /// Grid size in pixels
    pub grid_size: i32,

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
            selection: Selection::default(),
            tool: Tool::default(),
            wire_drawing: WireDrawing::default(),
            grid_size: 10,
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
