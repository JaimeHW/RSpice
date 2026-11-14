//! The Design inspector for a Verilog-A cell view.
//!
//! The module's identity, its compile status and build targets, the
//! diagnostics of its last compile, and the ABI contract it exposes. They live
//! here so the editor takes the whole document well.

use egui::Ui;
use rspice_ui_kit::panels::inspector::muted_inspector_copy;

use crate::workbench::MessageCatalog;
use crate::workbench::documents::code_workspace::{
    self, CodeWorkspaceRuntimeState, VerilogAInspectorAction,
};
use rspice_app_state::workbench::app_state::model::Model;

/// Draw the inspector of the Verilog-A view the model has open, over the
/// Code & Automation page's state, and return what the reader asked of it.
pub(in crate::workbench) fn show(
    ui: &mut Ui,
    model: &Model,
    code_workspace: &CodeWorkspaceRuntimeState,
    messages: MessageCatalog,
) -> Option<VerilogAInspectorAction> {
    match code_workspace::selected_veriloga_source_in(model) {
        Ok(selected) => code_workspace::show_veriloga_inspector(
            ui,
            &code_workspace.veriloga,
            model.workspace.content.project.id(),
            &selected,
            messages,
        ),
        // The editor states the same refusal in full; the inspector has no
        // module to describe and says why rather than drawing empty rows.
        Err(error) => {
            muted_inspector_copy(ui, &error);
            None
        }
    }
}
