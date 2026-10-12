//! The Design navigator for a Verilog-A cell view.
//!
//! A module is a view of a cell, so the panel opens with the path that names
//! it, the way a symbol's does. Below that is the bundle the view owns: its
//! module sources, with the file operations its lifecycle governs, its build
//! policy, and the outline of the file being edited. The rail resolves the
//! same bundle as the editor, so the two can never list different files.

use super::*;

use crate::workbench::documents::code_workspace;

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let reference = app.state.workspace.content.active_view.clone();
    super::symbol::hierarchy_path(ui, &reference);
    if let Err(error) = code_workspace::selected_veriloga_source(app) {
        // The editor states the same refusal; the panel must not list the
        // files of a bundle the view does not own.
        empty_navigator_row(ui, &error);
        return;
    }
    super::source_bundle::veriloga(ui, app);
}
