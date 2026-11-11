//! The title bar's Netlist cell: the document the workspace's page shows.

use crate::workbench::MessageCatalog;
use crate::workbench::documents::code_workspace::CodeWorkspaceRuntimeState;
use rspice_app_state::workbench::app_state::model::Model;

/// The Netlist workspace's title cell: the document its page shows.
pub(in crate::workbench) fn code_title_cell(
    model: &Model,
    code_workspace: &CodeWorkspaceRuntimeState,
    messages: MessageCatalog,
) -> String {
    crate::workbench::documents::code_workspace::active_document_label(
        &model.workspace,
        code_workspace,
        messages,
    )
}
