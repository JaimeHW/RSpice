//! The schematic drag as the application runs it: a press prepares the
//! drag's undo baseline, a move past the threshold opens its operation on
//! the open schematic, and anything that leaves the gesture's window,
//! sheet or tool lets it go.

use super::interaction_state::{DragType, SchematicDragOwner};

impl super::AppState {
    /// A press on the canvas copies the undo baseline of the drag it may
    /// start, so that drag's first move frame does not pay for the copy. A
    /// drag that begins once the document has changed captures its own.
    pub(crate) fn prepare_schematic_drag(&mut self) {
        if self.model.schematic.has_pending_operation() || self.schematic_edit_read_only() {
            return;
        }
        self.workbench.interaction.drag.prepared = Some(self.model.schematic.prepare_operation());
    }

    pub(crate) fn begin_schematic_drag(
        &mut self,
        position: (i32, i32),
        kind: DragType,
        ctx: &egui::Context,
    ) -> bool {
        if self.model.schematic.has_pending_operation() || self.schematic_edit_read_only() {
            return false;
        }
        let description = match kind {
            DragType::MoveSelection => "move selection",
            DragType::WireVertex => "drag wire vertex",
            _ => return false,
        };
        let prepared = self.workbench.interaction.drag.prepared.take();
        // A new pointer edit cannot leave another window's transaction
        // orphaned when it takes over the shared gesture owner.
        self.cancel_schematic_drag();
        self.model
            .schematic
            .begin_prepared_operation(prepared, description);
        let owner = SchematicDragOwner {
            project: self.model.workspace.content.project.id(),
            document: self.model.workspace.content.active_schematic_reference(),
            occurrence: self.model.workspace.content.active_occurrence().cloned(),
            sheet: self.schematic_drag_sheet(),
            window: self.workbench.window_session.current(),
            viewport: ctx.viewport_id(),
            operation_id: self
                .model
                .schematic
                .pending_operation_id()
                .expect("new drag operation"),
        };
        self.workbench.interaction.drag.start(position, kind, owner);
        true
    }

    fn schematic_drag_operation_is_current(&self) -> bool {
        self.workbench
            .interaction
            .drag
            .operation_is_current(&self.model)
    }

    fn schematic_drag_sheet(
        &self,
    ) -> Option<(Option<rspice_design_model::design_management::SheetId>, u64)> {
        self.model
            .workspace
            .content
            .design_management
            .sheet_catalog(
                &self
                    .model
                    .workspace
                    .content
                    .active_schematic_reference()
                    .key(),
            )
            .map(|catalog| (catalog.active_sheet_id(), catalog.revision()))
    }

    pub(crate) fn schematic_drag_in_progress(&self) -> bool {
        self.schematic_drag_operation_is_current()
            && self
                .workbench
                .interaction
                .drag
                .owner
                .as_ref()
                .is_some_and(|owner| {
                    owner.occurrence.as_ref() == self.model.workspace.content.active_occurrence()
                        && owner.sheet == self.schematic_drag_sheet()
                })
    }

    pub(crate) fn schematic_drag_snapshot(
        &self,
        ctx: &egui::Context,
    ) -> Option<rspice_schematic_editor::view::selection_drag::ActiveSelectionDrag> {
        if !self.schematic_drag_owned_by_context(ctx) {
            return None;
        }
        let drag = &self.workbench.interaction.drag;
        Some(
            rspice_schematic_editor::view::selection_drag::ActiveSelectionDrag {
                kind: drag.drag_type,
                last_position: drag.last_pos.map(|(x, y)| crate::state::Point::new(x, y)),
                operation_id: drag.owner.as_ref()?.operation_id,
            },
        )
    }

    pub(crate) fn schematic_drag_owned_by_context(&self, ctx: &egui::Context) -> bool {
        self.schematic_drag_in_progress()
            && self
                .workbench
                .interaction
                .drag
                .owner
                .as_ref()
                .is_some_and(|owner| {
                    owner.window == self.workbench.window_session.current()
                        && owner.viewport == ctx.viewport_id()
                })
    }

    /// Cancel only the transaction owned by this pointer gesture. A stale
    /// release must never modify a newly opened document or another operation.
    pub(crate) fn cancel_schematic_drag(&mut self) -> bool {
        self.workbench
            .interaction
            .drag
            .cancel_schematic_drag(&mut self.model)
    }

    pub(crate) fn cancel_schematic_drag_in_window(
        &mut self,
        window: crate::workbench::lifecycle::window_session::ApplicationWindowId,
    ) {
        if self
            .workbench
            .interaction
            .drag
            .owner
            .as_ref()
            .is_some_and(|owner| owner.window == window)
        {
            self.cancel_schematic_drag();
        }
    }

    /// Called before input dispatch and after the canvas has handled release.
    /// An explicit touch cancellation always wins over a synthetic release.
    pub(crate) fn reconcile_schematic_drag(
        &mut self,
        ctx: &egui::Context,
        after_canvas: bool,
    ) -> bool {
        // A press let go without starting a drag lets its baseline go.
        if !ctx.input(|input| input.pointer.primary_down()) {
            self.workbench.interaction.drag.prepared = None;
        }
        let Some(owner) = &self.workbench.interaction.drag.owner else {
            return false;
        };
        if owner.window != self.workbench.window_session.current()
            || owner.viewport != ctx.viewport_id()
        {
            return false;
        }
        let invalid_context = !self.schematic_drag_in_progress()
            || self.model.schematic.session.editor.tool != crate::state::Tool::Select
            || self.schematic_edit_read_only()
            || self.workbench.current_route().surface_id()
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
            self.cancel_schematic_drag();
            return true;
        }
        false
    }
}
