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
use crate::workbench::project::DependencyLibrary;
use crate::workbench::project::configuration_state::ConfigurationPageState;
use crate::workbench::project::workspace_state::ProjectPage;
use crate::workbench::surfaces::project::library::LibraryIntent;

use configuration::project_configuration_navigator;
use dependencies::project_dependency_navigator;
use library::project_library_navigator;
use recovery::project_recovery_navigator;

/// What the Project workspace's navigator reads and writes: the project
/// feature's page and the selections its rails keep, the configuration
/// page and its dialogs, the recovery catalog, whether the Library page
/// shows the built-in library, the model, and the navigator's filter.
pub(in crate::workbench) struct ProjectNavigator<'a> {
    pub(in crate::workbench) page: ProjectPage,
    pub(in crate::workbench) configuration: &'a mut ConfigurationPageState,
    pub(in crate::workbench) configuration_dialog: &'a mut ConfigurationDialogState,
    pub(in crate::workbench) dependency_selection: &'a mut Option<DependencyLibrary>,
    pub(in crate::workbench) checkpoint_recovery: &'a ProjectCheckpointRecoveryState,
    pub(in crate::workbench) checkpoint_selection: &'a mut Option<String>,
    pub(in crate::workbench) builtin_library_open: bool,
    pub(in crate::workbench) model: &'a mut Model,
    pub(in crate::workbench) filter: &'a str,
}

/// The rail of the page in front. A library row's click is the Library
/// page's own selection, returned for the page to carry out.
pub(in crate::workbench) fn project(
    ui: &mut Ui,
    nav: &mut ProjectNavigator<'_>,
) -> Option<LibraryIntent> {
    ScrollArea::vertical()
        .auto_shrink([false, true])
        .show(ui, |ui| match nav.page {
            ProjectPage::Overview | ProjectPage::Library => project_library_navigator(ui, nav),
            ProjectPage::Configuration => {
                project_configuration_navigator(ui, nav);
                None
            }
            ProjectPage::Dependencies => {
                project_dependency_navigator(ui, nav);
                None
            }
            ProjectPage::Recovery => {
                project_recovery_navigator(ui, nav);
                None
            }
        })
        .inner
}

/// The room the unsaved dot takes at the head of a row's meta. The row
/// shortens its name to stop before the meta, so the dot never lands on
/// the name however narrow the rail is.
const DOT_ROOM: &str = "   ";

/// A row's meta with room for the unsaved dot at its head.
fn unsaved_meta(meta: Option<&str>) -> String {
    format!("{DOT_ROOM}{}", meta.unwrap_or_default())
}

/// The document bar's unsaved dot, painted in the room [`unsaved_meta`]
/// made before the row's meta: the configuration has edits that are not
/// applied. The row is still named for its label and its meta alone.
fn unsaved_mark(ui: &Ui, response: &egui::Response, label: &str, meta: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    let width = |text: &str| {
        ui.painter()
            .layout_no_wrap(
                text.to_owned(),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                t.color.text_faint,
            )
            .size()
            .x
    };
    let meta_width = meta.map_or(0.0, width);
    ui.painter().circle_filled(
        egui::pos2(
            response.rect.right() - 8.0 - meta_width - width(DOT_ROOM) / 2.0,
            response.rect.center().y,
        ),
        3.0,
        t.color.text_dim,
    );
    let name = meta.map_or_else(|| label.to_owned(), |meta| format!("{label}, {meta}"));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(name);
        node.set_description("Unsaved changes.");
    });
}
