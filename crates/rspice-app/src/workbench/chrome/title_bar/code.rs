//! The title bar's Netlist cell: the document the workspace's page shows.

use crate::workbench::AppState;

/// The Netlist workspace's title cell: the document its page shows.
pub(super) fn code_title_cell(state: &AppState) -> String {
    crate::workbench::documents::code_workspace::active_document_label(
        &state.model.workspace,
        &state.code.code_workspace,
        state.workbench.messages(),
    )
}
