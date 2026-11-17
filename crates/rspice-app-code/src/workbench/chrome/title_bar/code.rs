//! The title bar's Netlist cell: the document the workspace shows.

use rspice_app_state::workbench::app_state::model::Model;

/// The Netlist workspace's title cell: the document it shows.
pub fn code_title_cell(model: &Model) -> String {
    crate::workbench::documents::code_workspace::active_document_label(&model.workspace)
}
