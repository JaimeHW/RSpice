//! What the inspector says about the model library a reader has selected.
//!
//! Two sections, in the dock's usual order: the binding — which library is
//! current, and what technology it declares — and the one model inside it the
//! catalog is on. Both are read-only reports over the model library manager;
//! authoring a model is the model editor's, not this dock's.

use egui::Ui;

use crate::workbench::RSpiceApp;

use super::super::super::design_system::property_row;
use super::section_header;

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    section_header(ui, "Model binding", None);
    property_row(
        ui,
        "Library",
        app.state
            .model_library_manager
            .selected_library
            .as_deref()
            .unwrap_or("None"),
    );
    let selected_library = app.state.model_library_manager.current_library().cloned();
    if let Some(library) = selected_library {
        property_row(
            ui,
            "PDK",
            if library.pdk_name.is_empty() {
                "Unspecified"
            } else {
                &library.pdk_name
            },
        );
        property_row(
            ui,
            "Technology",
            if library.technology_node.is_empty() {
                "Unspecified"
            } else {
                &library.technology_node
            },
        );
        property_row(
            ui,
            "Version",
            if library.version.is_empty() {
                "Unspecified"
            } else {
                &library.version
            },
        );
        property_row(ui, "Models", &library.model_count().to_string());
        property_row(ui, "Corners", &library.corner_count().to_string());
        property_row(
            ui,
            "Selected corner",
            library.selected_corner.as_deref().unwrap_or("None"),
        );
        if let Some(model_name) = &app.state.workbench.selected_model
            && let Some(model) = library.models.get(model_name)
        {
            section_header(ui, "Selected model", None);
            property_row(ui, "Name", &model.name);
            property_row(ui, "Type", &format!("{:?}", model.model_type));
            property_row(ui, "Level", &format!("{:?}", model.level));
            property_row(ui, "Parameters", &model.parameters.len().to_string());
            if let Some(vdd) = model.vdd {
                property_row(ui, "Nominal VDD", &format!("{vdd:.6} V"));
            }
        }
    }
}
