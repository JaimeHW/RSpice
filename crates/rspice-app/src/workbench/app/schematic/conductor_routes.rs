//! Host authority for the editor's uncommitted conductor routes.

use super::edit_authority::schematic_editor_request_source;
use crate::state::{BusDeclaration, BusParseError, Point};
use crate::workbench::app_state::{AppState, Model};
use crate::workbench::state::WorkbenchSession;
use rspice_schematic_editor::session::conductor::ConductorSource;

/// Where a route is being drawn: the document, view, window and viewport it
/// started in.
fn conductor_source(
    model: &Model,
    workbench: &WorkbenchSession,
    ctx: &egui::Context,
) -> ConductorSource {
    ConductorSource {
        document: schematic_editor_request_source(model),
        view: model.workspace.content.active_view.clone(),
        window: workbench.window_session.current().value(),
        viewport: ctx.viewport_id(),
    }
}

/// Start a wire at `position` on the open schematic, bound to where it
/// started. A read-only schematic starts none.
pub(crate) fn start_canvas_wire(
    model: &mut Model,
    workbench: &WorkbenchSession,
    ctx: &egui::Context,
    position: Point,
) {
    if model.schematic_edit_read_only() {
        return;
    }
    let source = conductor_source(model, workbench, ctx);
    model.schematic.start_wire(position);
    model.schematic.session.editor.bind_conductor_source(source);
}

impl AppState {
    pub(crate) fn start_canvas_wire(&mut self, ctx: &egui::Context, position: Point) {
        start_canvas_wire(&mut self.model, &self.workbench, ctx, position);
    }

    pub(crate) fn start_canvas_bus(
        &mut self,
        ctx: &egui::Context,
        position: Point,
        declaration: Option<BusDeclaration>,
    ) -> Result<(), BusParseError> {
        if self.schematic_edit_read_only() {
            return Err(BusParseError::ReadOnly);
        }
        let source = conductor_source(&self.model, &self.workbench, ctx);
        self.model.schematic.start_bus(position, declaration)?;
        self.model
            .schematic
            .session
            .editor
            .bind_conductor_source(source);
        Ok(())
    }

    pub(crate) fn reconcile_conductor_route(&mut self, ctx: &egui::Context) -> bool {
        let editor = &self.model.schematic.session.editor;
        if !editor.wire_drawing.active && !editor.bus_drawing.active {
            return false;
        }
        let source = conductor_source(&self.model, &self.workbench, ctx);
        let can_edit = !self.schematic_edit_read_only();
        self.model
            .schematic
            .session
            .editor
            .reconcile_conductor_source(&source, can_edit)
    }
}
