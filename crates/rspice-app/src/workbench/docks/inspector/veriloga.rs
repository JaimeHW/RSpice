//! The Design inspector for a Verilog-A cell view.
//!
//! The module's identity, its compile status and build targets, the
//! diagnostics of its last compile, and the ABI contract it exposes. They live
//! here so the editor takes the whole document well.

use egui::Ui;
use rspice_ui_kit::panels::inspector::muted_inspector_copy;

use crate::workbench::RSpiceApp;
use crate::workbench::documents::code_workspace;

pub(super) fn show(ui: &mut Ui, app: &RSpiceApp) {
    match code_workspace::selected_veriloga_source(app) {
        Ok(selected) => code_workspace::show_veriloga_inspector(
            ui,
            &app.state.ui.code_workspace.veriloga,
            app.state.workspace.content.project.id(),
            &selected,
            app.state.ui.messages(),
        ),
        // The editor states the same refusal in full; the inspector has no
        // module to describe and says why rather than drawing empty rows.
        Err(error) => muted_inspector_copy(ui, &error),
    }
}
