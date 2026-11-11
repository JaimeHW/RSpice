//! The Design navigator for a Verilog-A cell view.
//!
//! The panel is the bundle the view owns: its files, with the file
//! operations its lifecycle governs; what the file being edited includes and
//! declares; and where the cell is placed. It opens on the files rather than
//! on the view's path, which the document tab already names. The rail
//! resolves the same bundle as the editor, so the two can never list
//! different files.

use super::*;
use egui::Ui;

use super::source_bundle::{RailAction, SourceRail};
use crate::workbench::documents::code_workspace;

/// Draw the rail of the Verilog-A view the model has open, and return what
/// the reader asked of it.
pub(in crate::workbench) fn show(ui: &mut Ui, parts: &mut SourceRail<'_>) -> Option<RailAction> {
    if let Err(error) = code_workspace::selected_veriloga_source_in(parts.model) {
        // The editor states the same refusal; the panel must not list the
        // files of a bundle the view does not own.
        empty_navigator_row(ui, &error);
        return None;
    }
    super::source_bundle::veriloga(ui, parts)
}
