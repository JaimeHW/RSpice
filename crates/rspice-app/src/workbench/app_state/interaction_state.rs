//! Interaction State
//!
//! Runtime interaction state for UI operations like dragging,
//! selection, and other transient user interactions.

use serde::{Deserialize, Serialize};

// =============================================================================
// Interaction State
// =============================================================================

pub(crate) use rspice_schematic_editor::session::selection::SchematicKeyboardFocus;

/// Runtime interaction state for UI operations
///
/// This captures transient UI state that doesn't belong in persistent
/// dialog configuration (like analysis parameters) but is needed
/// for smooth user interactions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InteractionState {
    /// Current drag operation state
    #[serde(skip)]
    pub drag: DragState,

    /// Component being hovered
    pub hover_component_id: Option<u64>,

    /// Wire being hovered
    pub hover_wire_id: Option<u64>,

    /// Last click position in grid coordinates
    pub last_click_pos: Option<(i32, i32)>,

    /// Whether hovering over a wire vertex (for visual feedback)
    pub hover_wire_vertex: Option<(i32, i32)>,

    /// What the open context menu is for, and the grid position of the
    /// right-click that opened it. Captured on the click frame; the menu
    /// re-renders from it for as long as egui keeps the popup open.
    #[serde(skip)]
    pub context_target: Option<(ContextTarget, (i32, i32))>,

    /// Focus identity for spatial arrow traversal on the schematic canvas.
    /// This is device-local presentation state and never project data.
    #[serde(skip)]
    pub(crate) schematic_keyboard_focus: Option<SchematicKeyboardFocus>,
}

pub use rspice_schematic_editor::view::context_menu::ContextTarget;

impl InteractionState {
    /// Clear all transient state
    #[cfg(test)]
    pub fn clear(&mut self) {
        self.drag.cancel();
        self.hover_component_id = None;
        self.hover_wire_id = None;
        self.last_click_pos = None;
        self.hover_wire_vertex = None;
        self.context_target = None;
        self.schematic_keyboard_focus = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_releases_canvas_keyboard_focus() {
        let mut state = InteractionState {
            schematic_keyboard_focus: Some(SchematicKeyboardFocus::Probe(17)),
            ..InteractionState::default()
        };

        state.clear();

        assert_eq!(state.schematic_keyboard_focus, None);
    }
}

// =============================================================================
// Drag State
// =============================================================================

/// State for drag operations (moving selection, rubber-banding)
#[derive(Debug, Clone, Default)]
pub struct DragState {
    pub(super) owner: Option<SchematicDragOwner>,
    /// Starting position of the drag (grid coordinates)
    pub start_pos: Option<(i32, i32)>,

    /// Last position during drag for computing delta (grid coordinates)
    pub last_pos: Option<(i32, i32)>,

    /// Type of drag operation
    pub drag_type: DragType,

    /// The document's undo baseline, copied when the button went down on
    /// the canvas, for the drag that press may start. Copying it then keeps
    /// the copy off the drag's first move frame.
    pub(super) prepared: Option<rspice_design::schematic::owned::PreparedBaseline>,
}

#[derive(Debug, Clone)]
pub(super) struct SchematicDragOwner {
    pub(super) project: crate::product::ProjectId,
    pub(super) document: crate::state::CellViewRef,
    pub(super) occurrence: Option<rspice_design::occurrence::DocumentOccurrence>,
    pub(super) sheet: Option<(Option<rspice_design_model::design_management::SheetId>, u64)>,
    pub(super) window: crate::workbench::lifecycle::window_session::ApplicationWindowId,
    pub(super) viewport: egui::ViewportId,
    pub(super) operation_id: u64,
}

impl DragState {
    /// Start a new drag operation
    pub(super) fn start(
        &mut self,
        pos: (i32, i32),
        drag_type: DragType,
        owner: SchematicDragOwner,
    ) {
        self.owner = Some(owner);
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
        self.owner = None;
        self.start_pos = None;
        self.last_pos = None;
        self.drag_type = DragType::None;
    }

    /// Whether the drag owns the open schematic's pending operation.
    pub(super) fn operation_is_current(&self, model: &super::Model) -> bool {
        self.owner.as_ref().is_some_and(|owner| {
            owner.project == model.workspace.content.project.id()
                && owner.document == model.workspace.content.active_schematic_reference()
                && Some(owner.operation_id) == model.schematic.pending_operation_id()
        })
    }

    /// Let go of the schematic drag, rolling back the operation it holds in
    /// whichever buffer holds it. Answers whether there was a drag.
    pub(crate) fn cancel_schematic_drag(&mut self, model: &mut super::Model) -> bool {
        // Occurrence/sheet changes invalidate input but still roll back the
        // matching active transaction rather than an inactive buffer copy.
        let active = self.operation_is_current(model);
        let Some(owner) = self.owner.take() else {
            return false;
        };
        if active {
            model.schematic.cancel_operation();
        } else if owner.project == model.workspace.content.project.id()
            && let Some(mut schematic) = model.workspace.schematic_editor_mut(&owner.document.key())
            && schematic.editor.pending_operation_id() == Some(owner.operation_id)
        {
            schematic.editor.cancel_operation();
        }
        self.cancel();
        true
    }

    /// [`Self::cancel_schematic_drag`], for a model operation that leaves the
    /// drawing to run once it knows it will.
    pub(crate) fn let_go(&mut self) -> impl FnOnce(&mut super::Model) + '_ {
        |model| {
            self.cancel_schematic_drag(model);
        }
    }
}

// =============================================================================
// Drag Type Enum
// =============================================================================

pub use rspice_schematic_editor::session::drag::DragType;

// =============================================================================
// Tests
// =============================================================================
