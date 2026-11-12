//! The Simulate workspace as the shell draws it on the stage, for the
//! cases that drive it through a whole application.

use egui::Ui;

use crate::workbench::RSpiceApp;
use crate::workbench::state::Workspace;

/// The Simulate workspace as the shell draws it on the stage.
pub(super) fn show_on_stage(ui: &mut Ui, app: &mut RSpiceApp) {
    use crate::workbench::framework::host::{Host, Point};
    app.contribute(Point::Surface(Workspace::Simulate), ui);
}
