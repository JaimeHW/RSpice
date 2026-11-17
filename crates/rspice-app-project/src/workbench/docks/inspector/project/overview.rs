//! The inspector beside the Project Overview: the simulation setup runs use,
//! what a save would write, the checks' problems and the project's own
//! actions.

use egui::Ui;
use rspice_ui_kit::panels::section_header;

use crate::workbench::app_state::DesignCheckStatus;
use crate::workbench::design_system::property_row;
use crate::workbench::lifecycle::project_lifecycle::dirty_document_count;
use crate::workbench::project::commands::{ProjectCommand, ProjectRequest};

use super::{ProjectInspector, ProjectInspectorAction, inspector_action};

pub(super) fn project_overview(
    ui: &mut Ui,
    inspector: &ProjectInspector<'_>,
) -> Option<ProjectInspectorAction> {
    let project = &inspector.model.workspace.content;
    // With no configuration used for runs, runs use Default: the project's
    // own top cell through the built-in view list.
    let testbench = project.simulation_root_reference().display_path();
    let configuration = project
        .configuration_sets
        .active()
        .map_or_else(|| "Default".to_owned(), |item| item.name().to_owned());
    let modified = dirty_document_count(&inspector.lifecycle);

    // No "CURRENT"/"REVIEW" meta: the Configuration row names what runs use,
    // Default included. The run plan
    // is on the toolbar and the execution target never varies, so neither is
    // a row here.
    section_header(ui, "Simulation setup", None);
    property_row(
        ui,
        "Top",
        &format!(
            "{}/{}",
            project.project.root_library, project.project.top_cell
        ),
    );
    property_row(ui, "Testbench", &testbench);
    property_row(ui, "Configuration", &configuration);

    // The status bar owns "Unsaved changes" and the project's revision is a
    // counter, so what is left of the working revision is how many documents
    // a save will write — and only when a save would write one.
    section_header(ui, "Working copy", None);
    if modified > 0 {
        property_row(ui, "Modified documents", &modified.to_string());
    }
    property_row(
        ui,
        "Location",
        &project.project.path.as_ref().map_or_else(
            || "Not saved yet".to_owned(),
            |path| path.display().to_string(),
        ),
    );

    let check_status = inspector
        .model
        .simulation_root_design_check_status(inspector.facts);
    let check_meta = match &check_status {
        DesignCheckStatus::Current(_) => None,
        DesignCheckStatus::NotRun => Some("not run"),
        DesignCheckStatus::Stale(_) => Some("out of date"),
        DesignCheckStatus::Unavailable { .. } => Some("unavailable"),
    };
    section_header(ui, "Problems", check_meta);
    match check_status {
        DesignCheckStatus::Current(receipt) => {
            let summary = receipt.result.summary();
            property_row(
                ui,
                "Blocking errors",
                &(summary.critical + summary.errors).to_string(),
            );
            property_row(
                ui,
                "Advisories",
                &(summary.warnings + summary.info).to_string(),
            );
            // The finding's own words, not its internal code: "CHK-001" is
            // an identifier a reader cannot do anything with.
            for violation in receipt.result.violations().iter().take(2) {
                property_row(ui, "Finding", &violation.message);
            }
        }
        // The header's meta already says which of the three states this is.
        DesignCheckStatus::NotRun | DesignCheckStatus::Stale(_) => {}
        DesignCheckStatus::Unavailable { reason, .. } => {
            property_row(ui, "Checks", &reason);
        }
    }
    if let Some(advisory) = inspector.model.pdk_config.scan_errors.first() {
        property_row(ui, "Model advisory", advisory);
    }

    section_header(ui, "Project actions", None);
    let mut action = None;
    if inspector_action(ui, "Revision history\u{2026}") {
        action = Some(ProjectInspectorAction::Execute(
            ProjectCommand::RevisionHistory.key(),
        ));
    }
    if inspector_action(ui, "Configurations") {
        action = Some(ProjectInspectorAction::Shell(
            ProjectRequest::ConfigurationSets,
        ));
    }
    if inspector_action(ui, "Switch project\u{2026}") {
        action = Some(ProjectInspectorAction::Execute(
            ProjectCommand::ProjectLauncher.key(),
        ));
    }
    action
}
