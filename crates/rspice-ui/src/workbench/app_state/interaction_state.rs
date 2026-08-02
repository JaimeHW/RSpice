//! Interaction State
//!
//! Runtime interaction state for UI operations like dragging,
//! selection, and other transient user interactions.

use serde::{Deserialize, Serialize};

// =============================================================================
// Interaction State
// =============================================================================

/// Stable canvas object currently owned by schematic keyboard traversal.
///
/// The authored document selection remains the command authority for objects
/// that already participate in editing. Probe flags deliberately remain
/// output-intent markers rather than pretending to support edit operations;
/// this transient identity gives them the same visible keyboard focus without
/// inventing probe clipboard/delete semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SchematicKeyboardFocus {
    Component(u64),
    Wire(u64),
    Bus(u64),
    BusTap(u64),
    Junction(u64),
    NetLabel(u64),
    Probe(u64),
    DesignNote(u64),
    DocumentationShape(u64),
}

/// Runtime interaction state for UI operations
///
/// This captures transient UI state that doesn't belong in persistent
/// dialog configuration (like analysis parameters) but is needed
/// for smooth user interactions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InteractionState {
    /// Current drag operation state
    pub drag: DragState,

    /// Component being hovered
    pub hover_component_id: Option<u64>,

    /// Wire being hovered
    pub hover_wire_id: Option<u64>,

    /// Last click position in grid coordinates
    pub last_click_pos: Option<(i32, i32)>,

    /// Wire vertex/junction position being dragged
    /// When dragging, ALL wires meeting at this point move together
    pub vertex_drag_pos: Option<(i32, i32)>,

    /// Whether hovering over a wire vertex (for visual feedback)
    pub hover_wire_vertex: Option<(i32, i32)>,

    /// What the open context menu is for, and the grid position of the
    /// right-click that opened it. Captured on the click frame; the menu
    /// re-renders from it for as long as egui keeps the popup open.
    #[serde(skip)]
    pub context_target: Option<(ContextTarget, (i32, i32))>,

    /// Whether the schematic selection-deletion review owns modal keyboard
    /// intent. The review payload remains in egui's temporary data, but this
    /// retained flag is available before shortcut dispatch paints the dialog.
    #[serde(skip)]
    pub schematic_delete_confirmation_open: bool,

    /// Focus identity for spatial arrow traversal on the schematic canvas.
    /// This is device-local presentation state and never project data.
    #[serde(skip)]
    pub(crate) schematic_keyboard_focus: Option<SchematicKeyboardFocus>,
}

/// What sits under a canvas right-click.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextTarget {
    /// A component instance (by id).
    Component(u64),
    /// A wire segment (by id).
    Wire(u64),
    /// Empty canvas.
    Canvas,
}

impl InteractionState {
    /// Clear all transient state
    #[cfg(test)]
    pub fn clear(&mut self) {
        self.drag.cancel();
        self.hover_component_id = None;
        self.hover_wire_id = None;
        self.last_click_pos = None;
        self.vertex_drag_pos = None;
        self.hover_wire_vertex = None;
        self.context_target = None;
        self.schematic_delete_confirmation_open = false;
        self.schematic_keyboard_focus = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_releases_schematic_delete_modal_ownership() {
        let mut state = InteractionState {
            schematic_delete_confirmation_open: true,
            schematic_keyboard_focus: Some(SchematicKeyboardFocus::Probe(17)),
            ..InteractionState::default()
        };

        state.clear();

        assert!(!state.schematic_delete_confirmation_open);
        assert_eq!(state.schematic_keyboard_focus, None);
    }
}

// =============================================================================
// Drag State
// =============================================================================

/// State for drag operations (moving selection, rubber-banding)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DragState {
    /// Starting position of the drag (grid coordinates)
    pub start_pos: Option<(i32, i32)>,

    /// Last position during drag for computing delta (grid coordinates)
    pub last_pos: Option<(i32, i32)>,

    /// Type of drag operation
    pub drag_type: DragType,
}

impl DragState {
    /// Start a new drag operation
    pub fn start(&mut self, pos: (i32, i32), drag_type: DragType) {
        self.start_pos = Some(pos);
        self.last_pos = Some(pos);
        self.drag_type = drag_type;
    }

    /// Update the drag position
    pub fn update(&mut self, pos: (i32, i32)) {
        self.last_pos = Some(pos);
    }

    /// Cancel the drag operation
    pub fn cancel(&mut self) {
        self.start_pos = None;
        self.last_pos = None;
        self.drag_type = DragType::None;
    }
}

// =============================================================================
// Drag Type Enum
// =============================================================================

/// Type of drag operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DragType {
    /// No drag operation
    #[default]
    None,
    /// Moving selected components/wires
    MoveSelection,
    /// Drawing a box selection rectangle
    BoxSelect,
    /// Panning the viewport
    Pan,
    /// Dragging a wire endpoint
    WireEndpoint,
    /// Dragging a wire vertex
    WireVertex,
}

// =============================================================================
// Tests
// =============================================================================
