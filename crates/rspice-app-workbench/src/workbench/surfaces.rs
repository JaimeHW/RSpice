//! Canonical task-owner surfaces.

pub mod memory_bands;

use egui::Ui;

use crate::workbench::framework::host::Point;
use crate::workbench::framework::requests::Drain;
use crate::workbench::framework::workbench_host::WorkbenchHost;

pub fn show(ui: &mut Ui, app: &mut impl WorkbenchHost) {
    // The page that draws a source editor publishes it again below; whatever
    // else is on screen has none for the Edit menu to act on.
    app.workbench_mut().active_source_editor = None;
    show_primary(ui, app);
    if app.workbench().outbox.waits_at(Drain::SourceEditor) {
        if app.workbench().active_source_editor.is_some() {
            // Asked after the editor was drawn: it takes it next frame.
            ui.ctx().request_repaint();
        } else {
            // No editor is on screen to take it.
            app.workbench_mut().outbox.decline_at(Drain::SourceEditor);
        }
    }
}

/// The stage of the workspace that is showing, drawn by whoever owns it.
fn show_primary(ui: &mut Ui, app: &mut impl WorkbenchHost) {
    let workspace = app.workbench().workspace;
    app.contribute(Point::Surface(workspace), ui);
}
