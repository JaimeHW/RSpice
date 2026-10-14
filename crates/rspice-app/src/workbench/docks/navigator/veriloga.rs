//! The Design navigator for a Verilog-A cell view.
//!
//! The panel is the bundle the view owns: its files, with the file
//! operations its lifecycle governs; what the file being edited includes and
//! declares; and where the cell is placed. It opens on the files rather than
//! on the view's path, which the document tab already names. The rail
//! resolves the same bundle as the editor, so the two can never list
//! different files.

use super::*;

use crate::workbench::documents::code_workspace;

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    if let Err(error) = code_workspace::selected_veriloga_source(app) {
        // The editor states the same refusal; the panel must not list the
        // files of a bundle the view does not own.
        empty_navigator_row(ui, &error);
        return;
    }
    super::source_bundle::veriloga(ui, app);
}
