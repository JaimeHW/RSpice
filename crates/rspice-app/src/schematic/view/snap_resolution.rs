//! App-owned document/session inputs for shared schematic snap resolution.

use super::viewport::Viewport;
use crate::state::SnapResult;
use crate::workbench::app_state::AppState;
use egui::Pos2;
use rspice_schematic_editor::view::snap_resolution;

pub(super) fn resolve_grid_pointer(
    state: &AppState,
    viewport: &Viewport,
    position: Pos2,
) -> SnapResult {
    snap_resolution::resolve_grid_pointer(
        &state.schematic.session.editor.snap_engine,
        state.schematic.document().grid_size,
        viewport,
        position,
    )
}
