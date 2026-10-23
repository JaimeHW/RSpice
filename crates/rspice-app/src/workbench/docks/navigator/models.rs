//! The Models workspace's navigator: its pages and the loaded model libraries.

use super::*;

pub(super) fn models(ui: &mut Ui, app: &mut RSpiceApp) {
    // "Model ownership" titled a page index. Nothing under it is about who
    // owns a model.
    section_header(ui, "Pages", None);
    for page in ModelsPage::ALL {
        if nav_row(
            ui,
            WorkbenchIcon::Models,
            page.label(),
            app.state.models.models_page == page,
            None,
        ) {
            app.state.models.models_page = page;
        }
    }
    section_header(
        ui,
        "Loaded model libraries",
        Some(
            &app.state
                .model
                .model_library_manager
                .library_count()
                .to_string(),
        ),
    );
    let libraries: Vec<_> = app
        .state
        .model
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .map(|library| (library.name.clone(), library.model_count()))
        .collect();
    for (name, count) in libraries {
        let selected = app
            .state
            .model
            .model_library_manager
            .selected_library
            .as_deref()
            == Some(&name);
        if nav_row(
            ui,
            WorkbenchIcon::Models,
            &name,
            selected,
            Some(&count.to_string()),
        ) {
            app.state.select_model_library(&name);
            app.state.models.models_page = ModelsPage::Models;
            app.state.models.selected_model = None;
        }
    }
}
