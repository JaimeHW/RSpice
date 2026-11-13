//! App-owned document/session inputs for shared schematic snap resolution.

use super::viewport::Viewport;
use crate::state::SnapResult;
use crate::workbench::app_state::Model;
use egui::Pos2;
use rspice_schematic_editor::view::snap_resolution;

pub(super) fn resolve_grid_pointer(
    model: &Model,
    viewport: &Viewport,
    position: Pos2,
) -> SnapResult {
    snap_resolution::resolve_grid_pointer(
        &model.schematic.session.editor.snap_engine,
        model.schematic.document().grid_size,
        viewport,
        position,
    )
}
