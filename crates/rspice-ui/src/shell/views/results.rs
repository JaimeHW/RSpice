//! Results view — delegates to the results workspace
//! (`crate::shell::results`): docbar with run context + viewer tabs, and the
//! active viewer in the document well.

use egui::Ui;

use crate::common::RSpiceApp;

/// Render the results view.
pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    crate::shell::results::show(ui, app);
}
