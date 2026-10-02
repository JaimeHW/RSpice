//! Interaction state and geometry for per-document schematic editor sessions.

mod lifecycle;

pub mod bus;
pub mod drag;
pub mod net_highlight;
pub mod selection;
pub mod snap;
pub mod stretch;
pub mod tool;
pub mod transform;
pub mod visibility;
pub mod wire;

pub mod canvas_cache;
pub mod conductor;
pub mod design_note;
pub mod documentation_shape;
pub mod placement;
pub mod placement_authority;
pub mod port;
pub mod stimulus_placement;

use self::{
    bus::{BusDrawing, PendingBusTapPlacement},
    design_note::PendingDesignNotePlacement,
    documentation_shape::{DocumentationShapeDrawing, PendingDocumentationShapePlacement},
    placement::{PendingLibraryCellPlacement, PendingPartModel},
    port::PendingPortSequence,
    snap::SnapEngine,
    stimulus_placement::PendingStimulusPlacement,
    tool::Tool,
    wire::WireDrawing,
};
use rspice_design::schematic::{
    clipboard::ClipboardData, document::SchematicDocument, rotation::Rotation, selection::Selection,
};
use rspice_design_model::Point;

/// Per-document schematic interaction and presentation state.
#[derive(Debug, Clone)]
pub struct EditorSession {
    /// Current selection (runtime state, never part of the design document).
    pub selection: Selection,

    /// Current tool (runtime state, not persisted - always starts as Select)
    pub tool: Tool,

    /// Unfinished wire gesture (runtime state, never part of the design document).
    pub wire_drawing: WireDrawing,

    /// Bus drawing state (runtime only; unfinished gestures never persist).
    pub bus_drawing: BusDrawing,

    /// Source of the current conductor draft; never persisted with the design.
    conductor_source: Option<conductor::ConductorSource>,

    /// Zoom level (1.0 = 100%) - not part of undo history or saved files
    /// Uses default of 1.0 when deserializing to prevent black screen
    pub zoom: f64,

    /// Pan offset in pixels - not part of undo history or saved files
    pub pan: (f64, f64),

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
    pub pending_library_cell: Option<PendingLibraryCellPlacement>,

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

    /// Validated tap configuration and owning document context while `Tool::BusTap` is armed.
    pub pending_bus_tap: Option<PendingBusTapPlacement>,

    /// Names still to place, and the contract they share, while the port tool
    /// is armed.
    pub pending_port_sequence: Option<PendingPortSequence>,

    /// Validated one-shot documentation object used while the text tool is armed.
    pub pending_design_note: Option<PendingDesignNotePlacement>,

    /// Validated shape kind and document authority used while the shape tool is armed.
    pub pending_documentation_shape: Option<PendingDocumentationShapePlacement>,

    /// Uncommitted click sequence for the active documentation-shape gesture.
    pub documentation_shape_drawing: DocumentationShapeDrawing,

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

    /// Snap engine configuration (runtime state, not persisted)
    /// Controls cursor snapping behavior during wire drawing
    pub snap_engine: SnapEngine,

    /// Rubber-band box selection rectangle (runtime state, not persisted)
    /// Used for drag-to-select operations
    pub selection_rect: selection::SelectionRect,

    /// Net highlighting state (runtime state, not persisted)
    /// Tracks which wires are part of the highlighted net
    pub net_highlight: net_highlight::NetHighlightState,

    /// Frame-coherent canvas geometry (culling bounds, hover hit-test index).
    /// Rebuilt when `topology_version` advances; resets on clone.
    canvas_cache: canvas_cache::CanvasCache,
}

impl Default for EditorSession {
    fn default() -> Self {
        Self {
            selection: Default::default(),
            tool: Default::default(),
            wire_drawing: Default::default(),
            bus_drawing: Default::default(),
            conductor_source: None,
            zoom: 1.0,
            pan: Default::default(),
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
            needs_fit: Default::default(),
            needs_drawing_sheet_fit: Default::default(),
            center_request: Default::default(),
            snap_engine: Default::default(),
            selection_rect: Default::default(),
            net_highlight: Default::default(),
            canvas_cache: Default::default(),
        }
    }
}

impl EditorSession {
    /// Rebuild derived geometry for this session's current design revision.
    pub fn ensure_canvas_cache(&mut self, document: &SchematicDocument, version: u64) {
        if self.canvas_cache.fresh(version).is_none() {
            let mut cache = std::mem::take(&mut self.canvas_cache);
            cache.rebuild(&document.wires, &document.junctions, version);
            self.canvas_cache = cache;
        }
    }

    pub fn canvas_cache(&self, version: u64) -> Option<&canvas_cache::CanvasCache> {
        self.canvas_cache.fresh(version)
    }
}
