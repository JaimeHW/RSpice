//! The navigator's configuration rail: the project's configurations, Default
//! first, each with its actions menu.

use egui::Ui;

use crate::workbench::app::dialogs::project::configuration_dialogs::{
    configuration_actions_menu, open_configuration_dialog,
};
use crate::workbench::design_system::{WorkbenchIcon, section_header};
use crate::workbench::project::configuration_state::{ConfigurationSelection, used_for_runs};

use super::super::{nav_row_response, row_context_menu};
use super::{ProjectNavigator, unsaved_mark, unsaved_meta};

/// The project's configurations, Default first. A row puts its configuration
/// in front of the reader; which one runs use changes only on the page.
pub(super) fn project_configuration_navigator(ui: &mut Ui, nav: &mut ProjectNavigator<'_>) {
    let query = nav.filter.trim().to_ascii_lowercase();
    let catalog = &nav.model.workspace.content.configuration_sets;
    let selection = nav.configuration.selection(catalog);
    let project = &nav.model.workspace.content.project;
    let default_top = format!("{}/{}", project.root_library, project.top_cell);
    let rows = std::iter::once((
        ConfigurationSelection::Default,
        "Default".to_owned(),
        default_top,
    ))
    .chain(catalog.configurations().iter().map(|configuration| {
        (
            ConfigurationSelection::Configuration(configuration.id()),
            configuration.name().to_owned(),
            configuration.root().display_path(),
        )
    }))
    .filter(|(_, name, top)| {
        query.is_empty()
            || name.to_ascii_lowercase().contains(&query)
            || top.to_ascii_lowercase().contains(&query)
    })
    .map(|(target, name, _)| (target, name, used_for_runs(catalog, target)))
    .collect::<Vec<_>>();
    let total = catalog.configurations().len() + 1;
    section_header(
        ui,
        "Configurations",
        (!query.is_empty())
            .then(|| format!("{} / {total}", rows.len()))
            .as_deref(),
    );
    let page = &nav.configuration;
    let edited = page
        .is_dirty(catalog)
        .then(|| page.draft().map(|draft| draft.id))
        .flatten();
    let write_allowed = !nav.model.safe_mode.project_read_only();
    let mut picked = None;
    let mut dialog = None;
    for (target, name, runs) in rows {
        let meta = runs.then_some("used for runs");
        let unsaved = edited.is_some_and(|id| target == ConfigurationSelection::Configuration(id));
        let shown_meta = unsaved.then(|| unsaved_meta(meta));
        let response = nav_row_response(
            ui,
            WorkbenchIcon::Sliders,
            &name,
            selection == target,
            shown_meta.as_deref().or(meta),
        );
        if unsaved {
            unsaved_mark(ui, &response, &name, meta);
        }
        if response.clicked() {
            picked = Some(target);
        }
        let (popup, _) = row_context_menu(&response);
        popup.show(|ui| {
            if let Some(request) = configuration_actions_menu(ui, target, write_allowed) {
                dialog = Some(request);
                ui.close();
            }
        });
    }
    if let Some(target) = picked {
        nav.configuration
            .request_selection(&nav.model.workspace.content.configuration_sets, target);
    }
    if let Some(request) = dialog {
        open_configuration_dialog(nav.configuration_dialog, nav.model, request);
    }
}
