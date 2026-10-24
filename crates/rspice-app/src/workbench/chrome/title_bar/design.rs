//! The title bar's Design cell: the open cell and its view.

use crate::workbench::AppState;

/// Design's title cell: the open cell and its view.
pub(in crate::workbench) fn design_title_cell(state: &AppState) -> String {
    format!(
        "{} · {}",
        state.model.workspace.content.active_view.cell,
        state.model.workspace.content.active_view.view
    )
}
