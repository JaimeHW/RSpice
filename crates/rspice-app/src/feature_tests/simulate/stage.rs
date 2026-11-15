//! The Simulate workspace as the shell draws it on the stage, for the
//! cases that drive it through a whole application.

use egui::Ui;

use crate::workbench::RSpiceApp;
use rspice_app_state::workbench::workspace::Workspace;

/// The Simulate workspace as the shell draws it on the stage.
pub(super) fn show_on_stage(ui: &mut Ui, app: &mut RSpiceApp) {
    use rspice_app_workbench::workbench::framework::host::{Host, Point};
    app.contribute(Point::Surface(Workspace::Simulate), ui);
}
