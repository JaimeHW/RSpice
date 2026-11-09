//! The title bar's Design cell: the open cell and its view.

use crate::workbench::AppState;

/// Design's title cell: the open cell and its view.
pub(in crate::workbench) fn design_title_cell(state: &AppState) -> String {
    let content = &state.model.workspace.content;
    format!(
        "{} · {}",
        content.active_view.cell,
        content
            .active_view_type()
            .caption_for(&content.active_view.view)
    )
}
