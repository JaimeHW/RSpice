//! The project feature's compositions over the whole application: where
//! the shell lends the feature its context and carries out what it
//! returns, each in the order it has always run. The landing, the Library
//! page with its publication dialogs, and the launcher are children.

use egui::{ScrollArea, Ui};

use crate::workbench::project::workspace_state::ProjectPage;
use crate::workbench::surfaces::project::recovery::recovery;
use crate::workbench::surfaces::project::{configuration, dependencies, overview};
use crate::workbench::{RSpiceApp, features};

pub(crate) mod landing;
pub(crate) mod launcher;
pub(crate) mod library;
pub(crate) mod library_publication;

/// The workspace with no project behind it, which it is whenever a project
/// is closed, the launcher is dismissed with nothing open, a recovery review
/// declines to reopen, or a live session is waiting for the host's snapshot.
/// Every section of the open workspace projects an open project's state, so
/// showing them here would dress the bootstrap placeholder up as the reader's
/// own work.
pub(in crate::workbench) fn show_landing(ui: &mut Ui, app: &mut RSpiceApp) {
    landing::show(ui, app);
}

/// The open project's page in front, as the panel the chrome's tabs control.
pub(in crate::workbench) fn page(ui: &mut Ui, app: &mut RSpiceApp) {
    let page = app.state.project.project_page;
    let panel = ui.scope(|ui| match page {
        // The Overview is the page in front, so its context holds the
        // dispatcher's answers for the commands its rows offer.
        ProjectPage::Overview => {
            overview::overview(ui, &mut features::cx_with_foreign_answers(app, |_| true))
        }
        // The Library page carries out what the reader asked of it between
        // its own calls, which the shell still does over the application.
        ProjectPage::Library => library::library(ui, app),
        ProjectPage::Recovery => recovery(ui, &mut features::cx(app)),
        ProjectPage::Configuration => configuration::configuration(ui, &mut features::cx(app)),
        page => {
            ScrollArea::vertical()
                .id_salt(("workbench.project.surface", page))
                .auto_shrink([false, false])
                .show(ui, |ui| match page {
                    ProjectPage::Dependencies => {
                        dependencies::dependencies(ui, &mut features::cx(app))
                    }
                    ProjectPage::Overview
                    | ProjectPage::Library
                    | ProjectPage::Configuration
                    | ProjectPage::Recovery => {
                        unreachable!("page owns its scroll contract")
                    }
                });
        }
    });
    ui.ctx().accesskit_node_builder(panel.response.id, |node| {
        node.set_role(egui::accesskit::Role::TabPanel);
        node.set_label(format!("Project {}", page.label()));
    });
}

/// Reuse the exact project-owned Library/Cell/View browser from a specialist
/// route without introducing a second selection or mutation owner.
pub(crate) fn show_library_browser(ui: &mut Ui, app: &mut RSpiceApp) {
    library::library(ui, app);
}
