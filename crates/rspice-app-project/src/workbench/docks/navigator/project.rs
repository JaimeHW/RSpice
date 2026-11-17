//! The Project workspace's navigator: the library, configuration,
//! dependency and recovery rails.

mod configuration;
mod dependencies;
mod library;
mod recovery;

use egui::{ScrollArea, Ui};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::app::dialogs::project::ProjectCheckpointRecoveryState;
use crate::workbench::app::dialogs::project::configuration_dialogs::ConfigurationDialogState;
use crate::workbench::app_state::Model;
use crate::workbench::project::configuration_state::ConfigurationPageState;
use crate::workbench::project::workspace_state::ProjectPage;

use configuration::project_configuration_navigator;
use dependencies::project_dependency_navigator;
use library::project_library_navigator;
use recovery::project_recovery_navigator;

/// What the Project workspace's navigator reads and writes: the project
/// feature's page and the selections its rails keep, the configuration
/// page and its dialogs, the recovery catalog, the model, and the
/// navigator's filter.
pub(in crate::workbench) struct ProjectNavigator<'a> {
    pub(in crate::workbench) page: ProjectPage,
    pub(in crate::workbench) configuration: &'a mut ConfigurationPageState,
    pub(in crate::workbench) configuration_dialog: &'a mut ConfigurationDialogState,
    pub(in crate::workbench) dependency_selection: &'a mut Option<String>,
    pub(in crate::workbench) checkpoint_recovery: &'a ProjectCheckpointRecoveryState,
    pub(in crate::workbench) checkpoint_selection: &'a mut Option<String>,
    pub(in crate::workbench) model: &'a mut Model,
    pub(in crate::workbench) filter: &'a str,
}

pub(in crate::workbench) fn project(ui: &mut Ui, nav: &mut ProjectNavigator<'_>) {
    ScrollArea::vertical()
        .auto_shrink([false, true])
        .show(ui, |ui| match nav.page {
            ProjectPage::Overview | ProjectPage::Library => project_library_navigator(ui, nav),
            ProjectPage::Configuration => project_configuration_navigator(ui, nav),
            ProjectPage::Dependencies => project_dependency_navigator(ui, nav),
            ProjectPage::Recovery => project_recovery_navigator(ui, nav),
        });
}

/// The document bar's unsaved dot, set before a row's meta: the
/// configuration has edits that are not applied.
fn unsaved_mark(ui: &Ui, response: &egui::Response, meta: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    let meta_width = meta.map_or(0.0, |meta| {
        ui.painter()
            .layout_no_wrap(
                meta.to_owned(),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                t.color.text_faint,
            )
            .size()
            .x
            + 8.0
    });
    ui.painter().circle_filled(
        egui::pos2(
            response.rect.right() - 8.0 - meta_width - 3.0,
            response.rect.center().y,
        ),
        3.0,
        t.color.text_dim,
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_description("Unsaved changes.");
    });
}
