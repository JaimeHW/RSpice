//! The inspector beside the Project Overview: the simulation setup runs use,
//! where the project file is, the checks' findings and the project's own
//! actions.

use egui::Ui;
use rspice_ui_kit::panels::section_header;

use crate::workbench::app_state::DesignCheckStatus;
use crate::workbench::design_system::{property_row, property_row_problem, property_row_wrapped};
use crate::workbench::project::commands::{ProjectCommand, ProjectRequest};
use crate::workbench::project::health::{self, DesignSubject};

use super::{ProjectInspector, ProjectInspectorAction, inspector_action};

pub(super) fn project_overview(
    ui: &mut Ui,
    inspector: &ProjectInspector<'_>,
) -> Option<ProjectInspectorAction> {
    let model = inspector.model;
    let project = &model.workspace.content;

    // The setup a run uses. A netlist-first project runs its deck, so the
    // schematic it leaves pristine is no part of its setup, as on the page.
    section_header(ui, "Simulation setup", None);
    match DesignSubject::of(model) {
        DesignSubject::SourceDeck => {
            let deck = health::source_deck(model);
            property_row(
                ui,
                "Source deck",
                &health::deck_name(model).unwrap_or_else(|| "Unavailable".to_owned()),
            );
            property_row_problem(ui, "Validation", &deck.state, deck.tone.problem());
        }
        DesignSubject::SchematicChecks => {
            let configuration = health::configuration(ui.ctx(), model);
            property_row(
                ui,
                "Top",
                &format!(
                    "{}/{}",
                    project.project.root_library, project.project.top_cell
                ),
            );
            // With no configuration used for runs, runs use Default: the
            // project's own top cell through the built-in view list.
            property_row(
                ui,
                "Testbench",
                &project.simulation_root_reference().display_path(),
            );
            property_row_problem(
                ui,
                "Configuration",
                &configuration.state,
                configuration.tone.problem(),
            );
        }
    }

    section_header(ui, "Project file", None);
    property_row(
        ui,
        "Location",
        &health::location(model, inspector.lifecycle.platform)
            .unwrap_or_else(|| "Not saved".to_owned()),
    );

    // The findings themselves, each whole: the page's status row states the
    // checks' counts and its strip the worst finding by code.
    if DesignSubject::of(model) == DesignSubject::SchematicChecks {
        let checks = health::schematic_checks(model, inspector.facts);
        section_header(ui, "Problems", Some(&checks.state));
        match model.simulation_root_design_check_status(inspector.facts) {
            DesignCheckStatus::Current(receipt) => {
                for violation in receipt.result.violations().iter().take(3) {
                    property_row_wrapped(ui, "Finding", &violation.message);
                }
            }
            DesignCheckStatus::Unavailable { reason, .. } => {
                property_row_wrapped(ui, "Checks", &reason);
            }
            DesignCheckStatus::NotRun | DesignCheckStatus::Stale(_) => {}
        }
    }

    section_header(ui, "Project actions", None);
    let mut action = None;
    if inspector_action(ui, "Project properties\u{2026}") {
        action = Some(ProjectInspectorAction::Execute(
            ProjectCommand::Properties.key(),
        ));
    }
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
