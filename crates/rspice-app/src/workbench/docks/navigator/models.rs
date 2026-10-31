//! The Models workspace's navigator: its pages and the loaded model libraries.

use super::*;
use egui::Ui;

use crate::state::ModelsPage;
use crate::ui::panels::{WorkbenchIcon, section_header};
use crate::workbench::app_state::Model;
use crate::workbench::state::WorkbenchSession;

pub(in crate::workbench) fn models(
    ui: &mut Ui,
    models_page: &mut ModelsPage,
    selected_model: &mut Option<String>,
    model: &mut Model,
    workbench: &mut WorkbenchSession,
) {
    // "Model ownership" titled a page index. Nothing under it is about who
    // owns a model.
    section_header(ui, "Pages", None);
    for page in ModelsPage::ALL {
        if nav_row(
            ui,
            WorkbenchIcon::Models,
            page.label(),
            *models_page == page,
            None,
        ) {
            *models_page = page;
        }
    }
    section_header(
        ui,
        "Loaded model libraries",
        Some(&model.model_library_manager.library_count().to_string()),
    );
    let libraries: Vec<_> = model
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .map(|library| (library.name.clone(), library.model_count()))
        .collect();
    for (name, count) in libraries {
        let selected = model.model_library_manager.selected_library.as_deref() == Some(&name);
        if nav_row(
            ui,
            WorkbenchIcon::Models,
            &name,
            selected,
            Some(&count.to_string()),
        ) {
            crate::workbench::models::workspace_state::select_library_or_prompt(
                model, workbench, &name,
            );
            *models_page = ModelsPage::Models;
            *selected_model = None;
        }
    }
}
