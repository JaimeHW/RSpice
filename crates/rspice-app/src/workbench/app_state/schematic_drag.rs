//! The schematic drag as the application runs it: its methods lend the
//! design slot's drag the model and the session it runs over.

#[cfg(test)]
use rspice_app_design::workbench::app_state::interaction_state::DragType;

impl super::AppState {
    /// [`DragState::begin`] for the session.
    #[cfg(test)]
    pub(crate) fn begin_schematic_drag(
        &mut self,
        position: (i32, i32),
        kind: DragType,
        ctx: &egui::Context,
    ) -> bool {
        self.design
            .interaction
            .drag
            .begin(&mut self.model, &self.workbench, position, kind, ctx)
    }

    /// [`DragState::in_progress`] for the session.
    #[cfg(test)]
    pub(crate) fn schematic_drag_in_progress(&self) -> bool {
        self.design.interaction.drag.in_progress(&self.model)
    }

    /// [`DragState::owned_by_context`] for the session.
    #[cfg(test)]
    pub(crate) fn schematic_drag_owned_by_context(&self, ctx: &egui::Context) -> bool {
        self.design
            .interaction
            .drag
            .owned_by_context(&self.model, &self.workbench, ctx)
    }

    /// Cancel only the transaction owned by this pointer gesture. A stale
    /// release must never modify a newly opened document or another operation.
    pub(crate) fn cancel_schematic_drag(&mut self) -> bool {
        self.design
            .interaction
            .drag
            .cancel_schematic_drag(&mut self.model)
    }

    pub(crate) fn cancel_schematic_drag_in_window(
        &mut self,
        window: rspice_app_workbench::workbench::lifecycle::window_session::ApplicationWindowId,
    ) {
        if self
            .design
            .interaction
            .drag
            .owner
            .as_ref()
            .is_some_and(|owner| owner.window == window)
        {
            self.cancel_schematic_drag();
        }
    }

    /// [`DragState::reconcile`] for the session.
    pub(crate) fn reconcile_schematic_drag(
        &mut self,
        ctx: &egui::Context,
        after_canvas: bool,
    ) -> bool {
        self.design
            .interaction
            .drag
            .reconcile(&mut self.model, &self.workbench, ctx, after_canvas)
    }
}
