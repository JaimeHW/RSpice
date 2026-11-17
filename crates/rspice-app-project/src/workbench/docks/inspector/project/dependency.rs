//! The inspector beside the Dependencies page: the selected dependency as it
//! resolved, and the project's technology.

use egui::Ui;
use rspice_ui_kit::panels::{Problem, section_header};

use crate::workbench::design_system::{property_row, property_row_problem};
use crate::workbench::framework::command_registry::ids;

use super::{ProjectInspector, ProjectInspectorAction, inspector_action};

pub(super) fn project_dependency(
    ui: &mut Ui,
    inspector: &ProjectInspector<'_>,
) -> Option<ProjectInspectorAction> {
    let model = inspector.model;
    let selection = inspector
        .dependency_selection
        .clone()
        .unwrap_or_else(|| "No dependency selected".to_owned());
    let design_library = model.library_manager.get_library(&selection);
    let model_library = model.model_library_manager.get_library(&selection);

    section_header(ui, "Resolved dependency", None);
    property_row(ui, "Name", &selection);
    if let Some(library) = design_library {
        property_row(ui, "Type", "Design library");
        property_row(
            ui,
            "Access",
            if library.read_only {
                "read only"
            } else {
                "writable"
            },
        );
        property_row(
            ui,
            "Source",
            &library.path.as_ref().map_or_else(
                || "in project".to_owned(),
                |path| path.display().to_string(),
            ),
        );
    } else if let Some(library) = model_library {
        property_row(ui, "Type", "Model library");
        property_row(ui, "Version", &library.version);
        // A library whose model files resolve is the normal case and says
        // nothing; one whose do not cannot simulate, which is the fact.
        if !library.source_authority.has_execution_source() || library.source_closure.is_empty() {
            property_row_problem(
                ui,
                "Model files",
                "Not found: cannot simulate",
                Some(Problem::Failure),
            );
        }
    }

    section_header(ui, "Technology", None);
    property_row(
        ui,
        "Technology",
        if model
            .workspace
            .content
            .project
            .technology_binding()
            .is_some()
        {
            "attached"
        } else {
            "not attached"
        },
    );

    section_header(ui, "Actions", None);
    let mut action = None;
    if inspector_action(ui, "Library manager") {
        action = Some(ProjectInspectorAction::Execute(ids::MODELS_CATALOG));
    }
    if inspector_action(ui, "PDK and model paths\u{2026}") {
        action = Some(ProjectInspectorAction::Execute(ids::PDK_SETTINGS));
    }
    action
}
