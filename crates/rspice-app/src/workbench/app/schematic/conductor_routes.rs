//! Host authority for the editor's uncommitted conductor routes.

use super::edit_authority::schematic_editor_request_source;
use crate::state::{BusDeclaration, BusParseError, Point};
use crate::workbench::app_state::AppState;
use rspice_schematic_editor::session::conductor::ConductorSource;

impl AppState {
    fn conductor_source(&self, ctx: &egui::Context) -> ConductorSource {
        ConductorSource {
            document: schematic_editor_request_source(self),
            view: self.model.workspace.content.active_view.clone(),
            window: self.workbench.window_session.current().value(),
            viewport: ctx.viewport_id(),
        }
    }

    pub(crate) fn start_canvas_wire(&mut self, ctx: &egui::Context, position: Point) {
        if self.schematic_edit_read_only() {
            return;
        }
        let source = self.conductor_source(ctx);
        self.model.schematic.start_wire(position);
        self.model
            .schematic
            .session
            .editor
            .bind_conductor_source(source);
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
        let source = self.conductor_source(ctx);
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
        let source = self.conductor_source(ctx);
        let can_edit = !self.schematic_edit_read_only();
        self.model
            .schematic
            .session
            .editor
            .reconcile_conductor_source(&source, can_edit)
    }
}
