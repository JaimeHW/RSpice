//! What the inspector says about the model library a reader has selected.
//!
//! Two sections, in the dock's usual order: the library — which one is
//! current, and what process it declares — and the one model inside it the
//! catalog is on. Both are read-only reports over the model library manager;
//! authoring a model is the model editor's, not this dock's.
//!
//! Every row here is a fact a designer checks before trusting a run. A row
//! that would read "Unspecified" is a field the library never declared, so it
//! is not drawn at all, and the counts the catalog beside this panel already
//! lists — models, corners, parameters — are not restated as rows.

use egui::Ui;

use crate::quantity::format_engineering_value;
use crate::workbench::RSpiceApp;

use super::super::super::design_system::property_row;
use super::section_header;

/// A library field the reader can act on, or nothing when it is blank.
fn declared(ui: &mut Ui, label: &str, value: &str) {
    if value.is_empty() {
        return;
    }
    property_row(ui, label, value);
}

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let Some(library) = app.state.model_library_manager.current_library().cloned() else {
        return;
    };
    section_header(ui, "Library", None);
    property_row(
        ui,
        "Library",
        app.state
            .model_library_manager
            .selected_library
            .as_deref()
            .unwrap_or_default(),
    );
    declared(ui, "PDK", &library.pdk_name);
    declared(ui, "Technology", &library.technology_node);
    declared(ui, "Version", &library.version);
    if let Some(corner) = library.selected_corner.as_deref() {
        property_row(ui, "Selected corner", corner);
    }
    let Some(model) = app
        .state
        .workbench
        .selected_model
        .as_ref()
        .and_then(|name| library.models.get(name))
    else {
        return;
    };
    // The model's own name is the section's title: a "Name" row under a
    // heading reading "Selected model" states the catalog's selection twice.
    section_header(ui, &model.name, None);
    property_row(ui, "Type", model.model_type.display_name());
    property_row(ui, "Level", model.level.display_name());
    if let Some(vdd) = model.vdd {
        property_row(
            ui,
            "Nominal VDD",
            &format!("{}V", format_engineering_value(vdd)),
        );
    }
}
