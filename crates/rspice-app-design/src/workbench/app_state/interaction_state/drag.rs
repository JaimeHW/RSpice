//! The schematic drag: a press prepares the drag's undo baseline, a move
//! past the threshold opens its operation on the open schematic, and
//! anything that leaves the gesture's window, sheet or tool lets it go.
//!
//! The drag keeps its owner in the design slot's interaction state and runs
//! over the model and the session it is lent.

use super::{DragState, DragType, SchematicDragOwner};
use crate::workbench::app_state::Model;
use crate::workbench::state::WorkbenchSession;

impl DragState {
    /// A press on the canvas copies the undo baseline of the drag it may
    /// start, so that drag's first move frame does not pay for the copy. A
    /// drag that begins once the document has changed captures its own.
    pub(crate) fn prepare(&mut self, model: &Model) {
        if model.schematic.has_pending_operation() || model.schematic_edit_read_only() {
            return;
        }
        self.prepared = Some(model.schematic.prepare_operation());
    }

    pub fn begin(
        &mut self,
        model: &mut Model,
        workbench: &WorkbenchSession,
        position: (i32, i32),
        kind: DragType,
        ctx: &egui::Context,
    ) -> bool {
        if model.schematic.has_pending_operation() || model.schematic_edit_read_only() {
            return false;
        }
        let description = match kind {
            DragType::MoveSelection => "move selection",
            DragType::WireVertex => "drag wire vertex",
            _ => return false,
        };
        let prepared = self.prepared.take();
        // A new pointer edit cannot leave another window's transaction
        // orphaned when it takes over the shared gesture owner.
        self.cancel_schematic_drag(model);
        model
            .schematic
            .begin_prepared_operation(prepared, description);
        let owner = SchematicDragOwner {
            project: model.workspace.content.project.id(),
            document: model.workspace.content.active_schematic_reference(),
            occurrence: model.workspace.content.active_occurrence().cloned(),
            sheet: drag_sheet(model),
            window: workbench.window_session.current(),
            viewport: ctx.viewport_id(),
            operation_id: model
                .schematic
                .pending_operation_id()
                .expect("new drag operation"),
        };
        self.start(position, kind, owner);
        true
    }

    pub fn in_progress(&self, model: &Model) -> bool {
        self.operation_is_current(model)
            && self.owner.as_ref().is_some_and(|owner| {
                owner.occurrence.as_ref() == model.workspace.content.active_occurrence()
                    && owner.sheet == drag_sheet(model)
            })
    }

    pub(crate) fn snapshot(
        &self,
        model: &Model,
        workbench: &WorkbenchSession,
        ctx: &egui::Context,
    ) -> Option<rspice_schematic_editor::view::selection_drag::ActiveSelectionDrag> {
        if !self.owned_by_context(model, workbench, ctx) {
            return None;
        }
        Some(
            rspice_schematic_editor::view::selection_drag::ActiveSelectionDrag {
                kind: self.drag_type,
                last_position: self.last_pos.map(|(x, y)| crate::state::Point::new(x, y)),
                operation_id: self.owner.as_ref()?.operation_id,
            },
        )
    }

    pub fn owned_by_context(
        &self,
        model: &Model,
        workbench: &WorkbenchSession,
        ctx: &egui::Context,
    ) -> bool {
        self.in_progress(model)
            && self.owner.as_ref().is_some_and(|owner| {
                owner.window == workbench.window_session.current()
                    && owner.viewport == ctx.viewport_id()
            })
    }

    /// Called before input dispatch and after the canvas has handled release.
    /// An explicit touch cancellation always wins over a synthetic release.
    pub fn reconcile(
        &mut self,
        model: &mut Model,
        workbench: &WorkbenchSession,
        ctx: &egui::Context,
        after_canvas: bool,
    ) -> bool {
        // A press let go without starting a drag lets its baseline go.
        if !ctx.input(|input| input.pointer.primary_down()) {
            self.prepared = None;
        }
        let Some(owner) = &self.owner else {
            return false;
        };
        if owner.window != workbench.window_session.current() || owner.viewport != ctx.viewport_id()
        {
            return false;
        }
        let invalid_context = !self.in_progress(model)
            || model.schematic.session.editor.tool != crate::state::Tool::Select
            || model.schematic_edit_read_only()
            || workbench.current_route().surface_id()
                != crate::workbench::routing::surface_catalog::SurfaceId::Design;
        let interrupted = ctx.input(|input| {
            let released = input.pointer.button_released(egui::PointerButton::Primary);
            !input.focused
                || input.events.iter().any(|event| {
                    matches!(
                        event,
                        egui::Event::Touch {
                            phase: egui::TouchPhase::Cancel,
                            ..
                        }
                    )
                })
                || (!released
                    && input
                        .events
                        .iter()
                        .any(|event| matches!(event, egui::Event::PointerGone)))
                || (!input.pointer.primary_down() && (after_canvas || !released))
        });
        if invalid_context || interrupted {
            self.cancel_schematic_drag(model);
            return true;
        }
        false
    }
}

/// The active sheet and its catalog revision, as a drag's owner records the
/// sheet it began on.
fn drag_sheet(
    model: &Model,
) -> Option<(Option<rspice_design_model::design_management::SheetId>, u64)> {
    model
        .workspace
        .content
        .design_management
        .sheet_catalog(&model.workspace.content.active_schematic_reference().key())
        .map(|catalog| (catalog.active_sheet_id(), catalog.revision()))
}
