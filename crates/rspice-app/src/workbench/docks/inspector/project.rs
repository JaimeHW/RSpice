//! The Project workspace's inspector: the overview, library, configuration,
//! dependency and recovery pages.

pub(crate) mod configuration;

use egui::Ui;
use rspice_app_models::veriloga_library::BuiltinVerilogAModel;
use rspice_ui_kit::panels::{Problem, section_header};

use crate::state::command_catalog::CommandKey;
use crate::state::{CellViewRef, ViewType};
use crate::workbench::app::dialogs::project::ProjectCheckpointRecoveryState;
use crate::workbench::app_state::{DesignCheckStatus, Model, SlotFacts};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::lifecycle::project_lifecycle::{LifecycleView, dirty_document_count};

use super::super::super::design_system::{property_row, property_row_problem};
use crate::workbench::project::commands::{ProjectCommand, ProjectRequest};
use crate::workbench::project::configuration_state::{
    ConfigurationPageState, ConfigurationSelection,
};
use crate::workbench::project::workspace_state::ProjectPage;

/// What the Project workspace's inspector reads and writes: the project
/// feature's page and the selections its rails keep, the configuration
/// page, the recovery catalog and the model, with what the design checks
/// read of the other features' state and the project as its lifecycle reads
/// it.
pub(in crate::workbench) struct ProjectInspector<'a> {
    pub(in crate::workbench) page: ProjectPage,
    pub(in crate::workbench) configuration: &'a mut ConfigurationPageState,
    pub(in crate::workbench) dependency_selection: &'a Option<String>,
    pub(in crate::workbench) checkpoint_recovery: &'a ProjectCheckpointRecoveryState,
    pub(in crate::workbench) checkpoint_selection: &'a Option<String>,
    /// Whether the Library page shows the built-in Verilog-A library rather
    /// than one of the project's.
    pub(in crate::workbench) builtin_library_open: bool,
    /// The model selected in that library.
    pub(in crate::workbench) builtin_model: Option<&'static BuiltinVerilogAModel>,
    pub(in crate::workbench) model: &'a Model,
    pub(in crate::workbench) facts: SlotFacts,
    /// Read for how many documents a save would write.
    pub(in crate::workbench) lifecycle: LifecycleView<'a>,
}

/// What the reader asked of the inspector this frame, raised once it has
/// drawn.
pub(in crate::workbench) enum ProjectInspectorAction {
    /// Run the command this key names.
    Execute(CommandKey),
    /// Use this configuration for runs. The feature commits it as the
    /// inspector returns, and its refusal is stated on the Configuration
    /// page.
    UseForRuns(ConfigurationSelection),
    /// Hand the shell what only it carries out.
    Shell(ProjectRequest),
}

pub(in crate::workbench) fn project(
    ui: &mut Ui,
    inspector: &mut ProjectInspector<'_>,
) -> Option<ProjectInspectorAction> {
    match inspector.page {
        ProjectPage::Overview => project_overview(ui, inspector),
        ProjectPage::Library => project_library(ui, inspector),
        ProjectPage::Configuration => {
            configuration::project_configuration(ui, inspector.configuration, inspector.model)
                .map(ProjectInspectorAction::UseForRuns)
        }
        ProjectPage::Dependencies => project_dependency(ui, inspector),
        ProjectPage::Recovery => {
            project_recovery(ui, inspector);
            None
        }
    }
}

fn project_overview(
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

/// The selection in the built-in Verilog-A library. Nothing here acts: the
/// page's band owns what can be done with a model.
fn builtin_model_selection(ui: &mut Ui, model: Option<&BuiltinVerilogAModel>) {
    section_header(ui, "Selection", None);
    if let Some(model) = model {
        property_row(ui, "Module", model.module);
        property_row(ui, "Version", model.version);
    }
    property_row(ui, "Access", "read only");
}

fn project_library(
    ui: &mut Ui,
    inspector: &ProjectInspector<'_>,
) -> Option<ProjectInspectorAction> {
    if inspector.builtin_library_open {
        builtin_model_selection(ui, inspector.builtin_model);
        return None;
    }
    let model = inspector.model;
    let library = model.library_manager.current_library();
    let library_path = library
        .and_then(|item| item.path.as_ref())
        .map_or_else(|| "not bound".to_owned(), |path| path.display().to_string());
    let technology = library.map_or("unbound", |item| {
        if item.technology.trim().is_empty() {
            "unbound"
        } else {
            item.technology.as_str()
        }
    });
    let view = model.library_manager.current_view();
    let view_type = view.map_or("—", |item| item.view_type.caption());
    let open_target = match (
        model.library_manager.current_library(),
        model.library_manager.current_cell(),
        view,
    ) {
        (Some(library), Some(cell), Some(view)) => Some((
            CellViewRef::new(&library.name, &cell.name, &view.name),
            view.view_type,
        )),
        _ => None,
    };
    let usage = open_target
        .as_ref()
        .map_or_else(Vec::new, |(reference, _)| library_usage(model, reference));
    let (checks_meta, checks) = match open_target.as_ref() {
        Some((reference, ViewType::Schematic | ViewType::Testbench)) => {
            match model.design_check_status(reference, inspector.facts) {
                DesignCheckStatus::Current(receipt) => {
                    let summary = receipt.result.summary();
                    (
                        None,
                        Some(format!(
                            "{} errors \u{00b7} {} advisories",
                            summary.critical + summary.errors,
                            summary.warnings + summary.info
                        )),
                    )
                }
                DesignCheckStatus::NotRun => (Some("not run"), None),
                DesignCheckStatus::Stale(_) => (Some("out of date"), None),
                DesignCheckStatus::Unavailable { reason, .. } => {
                    (Some("unavailable"), Some(reason))
                }
            }
        }
        // A symbol or a source view has no schematic checks to report, and a
        // row saying so is a row about nothing.
        Some(_) | None => (None, None),
    };

    // The library browser beside this panel shows the library, the cell and
    // the view; what the panel adds is what opening the selection does, and
    // where the library lives.
    section_header(
        ui,
        "Selection",
        library
            .is_some_and(|item| item.read_only)
            .then_some("read-only"),
    );
    property_row(ui, "View type", view_type);

    section_header(ui, "Library", None);
    if technology != "unbound" {
        property_row(ui, "Technology", technology);
    }
    property_row(ui, "Source", &library_path);
    // A view nobody has edited is in its normal state.
    if view.is_some_and(|item| item.modified) {
        property_row(ui, "State", "modified");
    }

    section_header(ui, "Validation", checks_meta);
    if let Some(checks) = &checks {
        property_row(ui, "Checks", checks);
    }
    // One row, naming the consumers rather than counting them and then
    // naming the first of them underneath.
    if !usage.is_empty() {
        property_row(ui, "Used by", &usage.join(", "));
    }

    section_header(ui, "Actions", None);
    let open = inspector_action_enabled(ui, "Open selected view", open_target.is_some());
    // No "Where used & impact…": it wrote the consumers to the console, and
    // the "Used by" row above names them in place. No "Revision history…"
    // either — it is a project-level dialog, and the Project overview
    // inspector is where the project's own actions live.
    open_target.filter(|_| open).map(|(reference, view_type)| {
        ProjectInspectorAction::Shell(ProjectRequest::OpenView(reference, view_type))
    })
}

fn project_dependency(
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

fn project_recovery(ui: &mut Ui, inspector: &ProjectInspector<'_>) {
    let recovery = inspector.checkpoint_recovery;
    let selected = inspector.checkpoint_selection.as_deref();
    let checkpoint = recovery
        .checkpoints
        .iter()
        .find(|item| selected.is_some_and(|id| id == item.checkpoint_id().to_string()));
    let integrity = recovery.error.is_none() && recovery.quarantined.is_empty();

    // Only a selected checkpoint has anything to say. The revision counter,
    // the payload's byte count and the checkpoint's internal id are not
    // facts a reader restores on.
    if let Some(checkpoint) = checkpoint {
        section_header(ui, "Selected checkpoint", None);
        property_row(ui, "Reason", checkpoint.reason().label());
    }

    let quarantined = recovery.quarantined.len();
    // The store is in order or it is not: "verified" is success, and the
    // restore mode and what it leaves alone are constants that belong on the
    // Restore control. Damaged checkpoints are the one thing to report.
    if !integrity || quarantined > 0 {
        section_header(ui, "Recovery store", None);
        if !integrity {
            property_row_problem(ui, "Integrity", "Review required", Some(Problem::Warning));
        }
        if quarantined > 0 {
            property_row_problem(
                ui,
                "Quarantined",
                &quarantined.to_string(),
                Some(Problem::Warning),
            );
        }
    }
}

fn inspector_action(ui: &mut Ui, label: &str) -> bool {
    inspector_action_enabled(ui, label, true)
}

fn inspector_action_enabled(ui: &mut Ui, label: &str, enabled: bool) -> bool {
    let width = ui.available_width().max(1.0);
    let response = ui
        .add_enabled_ui(enabled, |ui| {
            ui.add_sized([width, 28.0], egui::Button::new(label))
        })
        .inner;
    ui.add_space(4.0);
    response.clicked()
}

/// The schematics that place `reference`, each once.
fn library_usage(model: &Model, reference: &CellViewRef) -> Vec<String> {
    let mut consumers = model
        .cell_instances(&reference.library, &reference.cell, Some(&reference.view))
        .into_iter()
        .map(|usage| usage.schematic.display_path())
        .collect::<Vec<_>>();
    consumers.dedup();
    consumers
}

#[cfg(test)]
mod tests {
    use rspice_app_models::veriloga_library::BUILTIN_VERILOGA_MODELS;

    use super::*;
    use crate::workbench::app_state::AppState;

    /// Everything the Library page's inspector announces for `state`.
    fn announced(
        state: &mut AppState,
        builtin_library_open: bool,
        builtin_model: Option<&'static BuiltinVerilogAModel>,
    ) -> Vec<String> {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let input = || egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(300.0, 640.0),
            )),
            ..Default::default()
        };
        let mut pass = |ctx: &mut Ui| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let inspector = ProjectInspector {
                    page: ProjectPage::Library,
                    configuration: &mut state.project.configuration,
                    dependency_selection: &state.project.project_dependency_selection,
                    checkpoint_recovery: &state.project.project_checkpoint_recovery,
                    checkpoint_selection: &state.project.project_checkpoint_selection,
                    builtin_library_open,
                    builtin_model,
                    model: &state.model,
                    facts: SlotFacts {
                        preflight_report_retained: false,
                        review_markers: state.design.schematic_visibility.review_markers,
                    },
                    lifecycle: LifecycleView {
                        model: &state.model,
                        platform: &state.platform,
                        workspace: state.workbench.workspace,
                        presentation: &state.results.results,
                    },
                };
                let _ = project_library(ui, &inspector);
            });
        };
        // Two passes: the first builds the fonts the rows are measured in.
        let _ = ctx.run_ui(input(), &mut pass);
        ctx.run_ui(input(), &mut pass)
            .platform_output
            .accesskit_update
            .expect("AccessKit tree")
            .nodes
            .into_iter()
            .flat_map(|(_, node)| {
                [
                    node.label().map(str::to_owned),
                    node.value().map(str::to_owned),
                ]
            })
            .flatten()
            .collect()
    }

    #[test]
    fn the_open_built_in_library_is_what_the_inspector_describes() {
        let model = &BUILTIN_VERILOGA_MODELS[0];
        let mut state = AppState::default();

        let said = announced(&mut state, true, Some(model));

        for row in [
            "Module",
            model.module,
            "Version",
            model.version,
            "Access",
            "read only",
        ] {
            assert!(said.iter().any(|text| text == row), "{row:?} in {said:?}");
        }
        for row in [
            "View type",
            "Source",
            "State",
            "Checks",
            "Used by",
            "Open selected view",
        ] {
            assert!(!said.iter().any(|text| text == row), "{row:?} in {said:?}");
        }
    }

    #[test]
    fn a_verilog_a_view_reads_as_verilog_a() {
        let mut state = AppState::default();
        let mut cell = crate::state::Cell::new("amp");
        cell.add_view(crate::state::View::new("veriloga", ViewType::VerilogA));
        let mut library = crate::state::Library::new("analog");
        library.add_cell(cell);
        let libraries = &mut state.model.library_manager;
        libraries.add_library(library);
        libraries.select_view("analog", "amp", "veriloga");

        let said = announced(&mut state, false, None);

        assert!(said.iter().any(|text| text == "Verilog-A"), "{said:?}");
        assert!(!said.iter().any(|text| text == "veriloga"), "{said:?}");
    }
}
