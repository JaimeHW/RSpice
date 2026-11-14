//! The Library page's composition over the whole application.

use crate::workbench::RSpiceApp;
use crate::workbench::features::{self, render_dialog};
use crate::workbench::framework::dialogs::DialogKey;
use crate::workbench::surfaces::project::library::page;
use egui::Ui;

/// The Library page as the shell composes it: the page in the project
/// feature's context, with what it raised answered as it returns, then its
/// dialogs.
pub(super) fn library(ui: &mut Ui, app: &mut RSpiceApp) {
    let ctx = ui.ctx().clone();
    page(ui, &mut features::cx(app));
    app.after_contribution();
    dialogs(&ctx, app);
}

/// The dialogs of a page that draws the Library browser, once what the page
/// raised has been answered: the publication dialogs, which write their
/// file through the application, then the membership dialogs. The Project
/// workspace's Library page and the Library and cellview manager's
/// Libraries page draw them.
pub(in crate::workbench) fn dialogs(ctx: &egui::Context, app: &mut RSpiceApp) {
    super::library_publication::show(ctx, app);
    // The library and view membership modals are reachable only from the
    // pages that draw the Library browser, so those pages own their frame.
    // Each draws through the feature registry, in the project feature's
    // context, and what one raised is answered as it returns.
    render_dialog(app, ctx, DialogKey("project.new_library"));
    render_dialog(app, ctx, DialogKey("project.rename_library"));
    render_dialog(app, ctx, DialogKey("project.delete_library"));
    render_dialog(app, ctx, DialogKey("project.rename_view"));
}
