//! Host authority for the editor's uncommitted conductor routes.

use super::edit_authority::schematic_editor_request_source;
use crate::state::{BusDeclaration, BusParseError, Point};
use crate::workbench::app_state::Model;
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

/// Start a bus route at `position`, bound to the document, view, window and
/// viewport it is drawn in.
pub(crate) fn start_canvas_bus(
    model: &mut Model,
    workbench: &WorkbenchSession,
    ctx: &egui::Context,
    position: Point,
    declaration: Option<BusDeclaration>,
) -> Result<(), BusParseError> {
    if model.schematic_edit_read_only() {
        return Err(BusParseError::ReadOnly);
    }
    let source = conductor_source(model, workbench, ctx);
    model.schematic.start_bus(position, declaration)?;
    model.schematic.session.editor.bind_conductor_source(source);
    Ok(())
}

/// Let an uncommitted route go once the document, view, window or viewport
/// it was drawn in is no longer in front, or the schematic became read-only.
/// Returns whether it was let go.
pub(crate) fn reconcile_conductor_route(
    model: &mut Model,
    workbench: &WorkbenchSession,
    ctx: &egui::Context,
) -> bool {
    let editor = &model.schematic.session.editor;
    if !editor.wire_drawing.active && !editor.bus_drawing.active {
        return false;
    }
    let source = conductor_source(model, workbench, ctx);
    let can_edit = !model.schematic_edit_read_only();
    model
        .schematic
        .session
        .editor
        .reconcile_conductor_source(&source, can_edit)
}
