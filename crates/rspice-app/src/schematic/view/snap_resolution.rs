//! App-owned document/session inputs for shared schematic snap resolution.

use super::{SchematicSymbolContext, viewport::Viewport};
use crate::state::SnapResult;
use crate::workbench::app_state::AppState;
use egui::Pos2;
use rspice_schematic_editor::view::snap_resolution;
pub(super) use snap_resolution::target_acquisition_radius;

pub(super) fn conductor_attachment_pitch(state: &AppState) -> Option<i32> {
    snap_resolution::conductor_attachment_pitch(
        &state.schematic.session.editor.snap_engine,
        state.schematic.document().grid_size,
    )
}
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
pub(super) fn resolve_target_pointer(
    state: &AppState,
    symbols: &SchematicSymbolContext,
    viewport: &Viewport,
    position: Pos2,
) -> SnapResult {
    snap_resolution::resolve_target_pointer(
        &super::schematic_design_view(state),
        &state.schematic.session.editor.snap_engine,
        symbols,
        viewport,
        position,
    )
}
