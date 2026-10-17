//! Context inspector with authoritative object and provenance details.

mod design;
mod executed_deck;
mod models;
mod result_authority;
/// Visible to the dock layer: the data browser opens it from a run's row, and
/// the frame draws it whichever dock is showing.
pub(super) mod run_details;
mod simulation;
/// Visible to the dock layer so the library browser's own gate can render this
/// pane beside it: the two docks state the same counts about the same design,
/// and a review that only ever opened one of them would not see them disagree.
pub(super) mod stimulus;
mod symbol;
mod veriloga;

/// The SOA verdict the Verify flow states is the one an instance's Checks row
/// states, read the same way.
pub(super) use design::analysis_soa_display;

use rspice_ui_kit::panels::inspector::muted_inspector_copy;
use rspice_ui_kit::panels::{
    PANEL_INSET, Problem, code_inspector_section, panel_body, panel_inset, section_block_gap,
    section_header,
};

use egui::{Align2, Color32, Pos2, Rect, ScrollArea, Sense, Stroke, Ui, Vec2};

use crate::state::model_library::ModelConsumerScope;
use crate::state::{CellViewRef, Component, ViewType, explicit_component_model};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::app_state::DesignCheckStatus;
use crate::workbench::documents::result_document::analysis_evidence_failure;
use crate::workbench::documents::result_document::manifest::active_manifest;
use crate::workbench::lifecycle::project_lifecycle::dirty_document_count;
use crate::workbench::{AppState, MessageId, RSpiceApp, ResultViewer};
use rspice_results_ui::session::PaneAxis;

use super::super::commands::vocabulary::Command;
use super::super::design_system::{
    PANEL_HEADER_H, WorkbenchIcon, property_row, property_row_input_action, property_row_problem,
};
use super::super::state::{ModelsPage, ProjectPage, VerificationPage, Workspace};

const INSPECTOR_SCROLL_HISTORY_LIMIT: usize = 64;

#[derive(Clone, Debug, Default)]
struct InspectorScrollMemory {
    active: Option<String>,
    /// Recent `(subject identity, vertical offset)` entries, oldest first.
    offsets: Vec<(String, f32)>,
}

impl InspectorScrollMemory {
    fn begin_subject(&mut self, identity: &str) -> Option<f32> {
        if self.active.as_deref() == Some(identity) {
            return None;
        }
        self.active = Some(identity.to_owned());
        Some(
            self.offsets
                .iter()
                .find(|(subject, _)| subject == identity)
                .map_or(0.0, |(_, offset)| *offset),
        )
    }

    fn record(&mut self, identity: String, offset: f32) {
        if let Some(index) = self
            .offsets
            .iter()
            .position(|(subject, _)| subject == &identity)
        {
            self.offsets.remove(index);
        }
        self.offsets.push((identity, offset.max(0.0)));
        if self.offsets.len() > INSPECTOR_SCROLL_HISTORY_LIMIT {
            self.offsets.remove(0);
        }
    }
}

fn inspector_scroll_memory_id() -> egui::Id {
    egui::Id::new("workbench.inspector.scroll-memory")
}

/// Validate the exact catalog binding without changing the schematic.
///
/// The Models workspace uses this to enable its action and the transaction
/// repeats it immediately before mutation, so presentation and execution
/// cannot disagree about model family, polarity, or provider authority.
pub(crate) fn validate_component_model_catalog_binding(
    state: &AppState,
    component_id: u64,
    library_name: &str,
    model_name: &str,
) -> Result<(), String> {
    let component = state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)
        .ok_or_else(|| "The selected instance no longer exists.".to_owned())?;
    let library = state
        .model_library_manager
        .get_library(library_name)
        .ok_or_else(|| format!("Model library '{library_name}' is no longer loaded."))?;
    let candidate = library
        .models
        .values()
        .find(|model| model.name.eq_ignore_ascii_case(model_name))
        .ok_or_else(|| {
            format!("Model '{model_name}' is no longer present in library '{library_name}'.")
        })?;

    if let Some(binding) = component.library_cell.as_ref() {
        if !binding.library.eq_ignore_ascii_case(library_name) {
            return Err(format!(
                "The selected cell instance is bound to library '{}'; cross-library rebinding requires the Library/Cellview Manager.",
                binding.library
            ));
        }
        let current_name = binding
            .module_name
            .as_deref()
            .unwrap_or(binding.cell.as_str());
        let current = library
            .models
            .values()
            .find(|model| model.name.eq_ignore_ascii_case(current_name))
            .ok_or_else(|| {
                format!(
                    "The selected cell's current model '{current_name}' no longer resolves in library '{library_name}'."
                )
            })?;
        if !crate::state::model_library::models_have_compatible_device_family(current, candidate) {
            return Err(format!(
                "Model '{model_name}' is incompatible with the selected cell's current model family."
            ));
        }
        return Ok(());
    }

    crate::state::model_library::validate_component_model_compatibility(component.kind, candidate)?;
    let effective = state.model_library_manager.effective_definition_provider(
        crate::state::model_library::ModelConsumerScope::PrimitiveModel,
        model_name,
    )?;
    let effective = effective
        .ok_or_else(|| format!("Model '{model_name}' has no executable catalog provider."))?;
    if !effective.library.eq_ignore_ascii_case(library_name) {
        return Err(format!(
            "Model '{model_name}' executes from project-global provider '{}', not selected library '{library_name}'. Resolve the project-global provider before binding.",
            effective.library
        ));
    }
    Ok(())
}

/// Bind one selected schematic instance from the Models & PDKs workbench
/// through the same guarded schematic transaction used by the inspector.
/// Library-cell instances retain their exact library identity; primitive
/// instances retain provider provenance as editor metadata, while the binding
/// is accepted only when it agrees with the project-global executable provider.
pub(crate) fn bind_component_model_from_catalog(
    app: &mut RSpiceApp,
    component_id: u64,
    library_name: &str,
    model_name: &str,
) -> Result<(), String> {
    app.state.commit_inline_component_edit()?;
    validate_component_model_catalog_binding(&app.state, component_id, library_name, model_name)?;
    let component = app
        .state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)
        .cloned()
        .ok_or_else(|| "The selected instance no longer exists.".to_owned())?;
    if let Some(binding) = component.library_cell.as_ref() {
        if !binding.library.eq_ignore_ascii_case(library_name) {
            return Err(format!(
                "The selected cell instance is bound to library '{}'; cross-library rebinding requires the Library/Cellview Manager.",
                binding.library
            ));
        }
        design::apply_bound_model_choice(app, component_id, model_name)?;
        return Ok(());
    }

    let effective = app
        .state
        .model_library_manager
        .effective_definition_provider(
            crate::state::model_library::ModelConsumerScope::PrimitiveModel,
            model_name,
        )?
        .ok_or_else(|| format!("Model '{model_name}' has no executable catalog provider."))?;
    if !effective.library.eq_ignore_ascii_case(library_name) {
        return Err(format!(
            "Model '{model_name}' executes from project-global provider '{}', not selected library '{library_name}'. Resolve the project-global provider before binding.",
            effective.library
        ));
    }

    let params =
        crate::state::params_string::set_parameter_value(&component.params, "model", model_name)
            .and_then(|text| {
                crate::state::params_string::set_parameter_value(
                    &text,
                    "model_library",
                    &effective.library,
                )
            })?;
    let mut candidate = component.clone();
    candidate.params = params;
    app.state
        .edit_component_transaction(&component, candidate, "bind instance model")?;
    app.invalidate_simulation_preflight();
    Ok(())
}

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    ui.spacing_mut().item_spacing.y = 0.0;
    header(ui, app);
    let scroll_identity = inspector_scroll_identity(app);
    let mut scroll_memory = ui
        .ctx()
        .data_mut(|data| data.get_temp::<InspectorScrollMemory>(inspector_scroll_memory_id()))
        .unwrap_or_default();
    let requested_offset = scroll_memory.begin_subject(&scroll_identity);
    // The ScrollArea widget ID must remain stable across egui's sizing and
    // paint passes. Subject-specific IDs can change after a pointer selection
    // inside the same frame, assigning two IDs to one scrollbar rectangle.
    // Preserve per-subject positions in the bounded cache above instead.
    let scroll_area = ScrollArea::vertical()
        .id_salt("workbench.inspector.scroll")
        .auto_shrink([false, false]);
    let scroll_area = if let Some(offset) = requested_offset {
        scroll_area.vertical_scroll_offset(offset)
    } else {
        scroll_area
    };
    let output = scroll_area.show(ui, |ui| {
        panel_body(ui, |ui| inspector_body(ui, app));
    });
    scroll_memory.record(scroll_identity, output.state.offset.y);
    ui.ctx().data_mut(|data| {
        data.insert_temp(inspector_scroll_memory_id(), scroll_memory);
    });
}

/// The inspected subject's sections, laid out on the panel body.
fn inspector_body(ui: &mut Ui, app: &mut RSpiceApp) {
    if split_selected_trace_is_inspected(app) {
        results(ui, app);
        return;
    }
    match app.state.workbench.workspace {
        Workspace::Project => project(ui, app),
        // A symbol cellview is edited against its pin contract, not against a
        // schematic selection.
        Workspace::Design
            if app.state.workspace.content.active_view_type() == crate::state::ViewType::Symbol =>
        {
            symbol::show(ui, &mut app.state);
        }
        // A Verilog-A module is inspected for what it compiles to, not for a
        // schematic selection it does not have.
        Workspace::Design
            if app.state.workspace.content.active_view_type()
                == crate::state::ViewType::VerilogA =>
        {
            veriloga::show(ui, app);
        }
        Workspace::Design => design::show(ui, app),
        Workspace::Simulate => simulation::simulate(ui, app),
        Workspace::Stimulus => stimulus::show(ui, &mut app.state),
        Workspace::Results => results(ui, app),
        Workspace::Verify => verify(ui, app),
        Workspace::Models => models::show(ui, app),
        Workspace::Netlist => code_workspace_inspector(ui, app),
    }
}

fn inspector_scroll_identity(app: &RSpiceApp) -> String {
    let route = app.state.workbench.current_route();
    if app.state.workbench.workspace != Workspace::Design {
        return route.to_string();
    }

    format!(
        "{}|{}|{:?}",
        route,
        app.state.workspace.content.active_display_path(),
        app.state.schematic.session.editor.selection
    )
}

fn split_selected_trace_is_inspected(app: &RSpiceApp) -> bool {
    app.state.workbench.workspace == Workspace::Design
        && app.state.workbench.results_split_visible(
            app.state.project_lifecycle.is_open(),
            app.state.simulation.retained.has_retained_result_dataset(),
        )
        && app
            .state
            .ui
            .results
            .valid_selected_trace(&app.state.simulation)
            .is_some()
}

fn header(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), PANEL_HEADER_H),
        egui::Sense::hover(),
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    let messages = app.state.ui.messages();
    let title = if split_selected_trace_is_inspected(app) {
        "Result details".to_owned()
    } else {
        match app.state.workbench.workspace {
            Workspace::Project => match app.state.workbench.project_page {
                ProjectPage::Overview => "Project overview".to_owned(),
                ProjectPage::Library => "Cell view".to_owned(),
                ProjectPage::Configuration => "Configuration".to_owned(),
                ProjectPage::Dependencies => "Dependency".to_owned(),
                ProjectPage::Recovery => "Checkpoint".to_owned(),
            },
            Workspace::Verify
                if app.state.workbench.verification_page == VerificationPage::Yield =>
            {
                "Yield details".to_owned()
            }
            Workspace::Verify => app.state.workbench.verification_page.label().to_owned(),
            Workspace::Netlist => match app.state.ui.code_workspace.page {
                crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist => {
                    messages.text(MessageId::CodeInspectorNetlistTitle)
                }
                crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation => {
                    messages.text(MessageId::CodeInspectorAutomationTitle)
                }
            },
            // A result sheet's context panel is named by the sheet it is
            // reading, the way every other workspace names its subject.
            Workspace::Results => {
                format!(
                    "{} details",
                    app.state.ui.results.session.viewer.tab_label()
                )
            }
            _ => app.state.workbench.workspace.inspector_title().to_owned(),
        }
    };
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &title)
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Heading);
        node.set_label(title.as_str());
        node.set_level(2);
    });
    let size = if app.state.workbench.workspace == Workspace::Design {
        tokens::FS_2
    } else {
        tokens::FS_0
    };
    ui.painter().text(
        egui::pos2(rect.left() + PANEL_INSET, rect.center().y),
        egui::Align2::LEFT_CENTER,
        &title,
        theme::sans(size, FontWeight::SemiBold),
        t.color.text,
    );
}

fn project(ui: &mut Ui, app: &mut RSpiceApp) {
    match app.state.workbench.project_page {
        ProjectPage::Overview => project_overview(ui, app),
        ProjectPage::Library => project_library(ui, app),
        ProjectPage::Configuration => project_configuration(ui, app),
        ProjectPage::Dependencies => project_dependency(ui, app),
        ProjectPage::Recovery => project_recovery(ui, app),
    }
}

fn project_overview(ui: &mut Ui, app: &mut RSpiceApp) {
    let project = &app.state.workspace.content;
    let active_configuration = project.configuration_sets.active();
    let testbench = active_configuration.map_or_else(
        || "not configured".to_owned(),
        |configuration| configuration.root().display_path(),
    );
    let configuration = active_configuration.map_or_else(
        || "not configured".to_owned(),
        |item| item.name().to_owned(),
    );
    let modified = dirty_document_count(&app.state);

    // No "CURRENT"/"REVIEW" meta: "not configured" in the Configuration row
    // is the actionable case, and it says so in the row itself. The run plan
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

    let check_status = app.state.project_root_design_check_status();
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
    if let Some(advisory) = app.state.pdk_config.scan_errors.first() {
        property_row(ui, "Model advisory", advisory);
    }

    section_header(ui, "Project actions", None);
    if inspector_action(ui, "Revision history\u{2026}") {
        Command::RevisionHistory.execute(app);
    }
    if inspector_action(ui, "Configuration sets\u{2026}") {
        Command::ConfigurationSets.execute(app);
    }
    if inspector_action(ui, "Switch project\u{2026}") {
        Command::ProjectLauncher.execute(app);
    }
}

fn project_library(ui: &mut Ui, app: &mut RSpiceApp) {
    let library = app.state.library_manager.current_library();
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
    let view = app.state.library_manager.current_view();
    let view_type = view.map_or("—", |item| item.view_type.display_name());
    let open_target = match (
        app.state.library_manager.current_library(),
        app.state.library_manager.current_cell(),
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
        .map_or_else(Vec::new, |(reference, _)| library_usage(app, reference));
    let (checks_meta, checks) = match open_target.as_ref() {
        Some((reference, ViewType::Schematic | ViewType::Testbench)) => {
            match app.state.design_check_status(reference) {
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
    if inspector_action_enabled(ui, "Open selected view", open_target.is_some())
        && let Some((reference, view_type)) = open_target.as_ref()
    {
        app.state.open_workspace_view(reference.clone());
        // Source views are cell views too: Design shows a Verilog-A module's
        // editor and states what a SPICE or Verilog view is bound to.
        let workspace = match view_type {
            ViewType::Config => {
                app.state.workbench.project_page = ProjectPage::Configuration;
                Workspace::Project
            }
            _ => Workspace::Design,
        };
        Command::OpenWorkspace(workspace).execute(app);
    }
    // No "Where used & impact…": it wrote the consumers to the console, and
    // the "Used by" row above names them in place. No "Revision history…"
    // either — it is a project-level dialog, and the Project overview
    // inspector is where the project's own actions live.
}

fn project_configuration(ui: &mut Ui, app: &mut RSpiceApp) {
    let active = app.state.workspace.content.configuration_sets.active();
    let name = active.map_or_else(
        || "No active configuration".to_owned(),
        |item| item.name().to_owned(),
    );
    let testbench = active.map_or_else(
        || "not configured".to_owned(),
        |item| item.root().display_path(),
    );
    let dut = active.map_or_else(
        || "not configured".to_owned(),
        |item| item.dut_path().to_owned(),
    );
    let overrides = active.map_or(0, |item| item.definition().overrides.len());

    section_header(ui, "Active binding", None);
    property_row(ui, "Configuration", &name);
    property_row(ui, "Testbench", &testbench);
    property_row(ui, "DUT", &dut);

    section_header(ui, "Netlisting", None);
    // "resolved" was success and "missing" restated "No active
    // configuration" two rows up; the precedence order is a constant and
    // belongs on the control that obeys it.
    property_row(ui, "Overrides", &overrides.to_string())
        .on_hover_text("Resolved in order: project, then configuration, then run.");

    section_header(ui, "Actions", None);
    if inspector_action(ui, "Validate configuration") {
        Command::PreflightChecks.execute(app);
    }
    if inspector_action(ui, "Manage configuration sets\u{2026}") {
        Command::ConfigurationSets.execute(app);
    }
}

fn project_dependency(ui: &mut Ui, app: &mut RSpiceApp) {
    let selection = app
        .state
        .workbench
        .project_dependency_selection
        .clone()
        .unwrap_or_else(|| "No dependency selected".to_owned());
    let design_library = app.state.library_manager.get_library(&selection);
    let model_library = app.state.model_library_manager.get_library(&selection);

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
        if app
            .state
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
    if inspector_action(ui, "Model & library catalog") {
        Command::ModelsPage(ModelsPage::Models).execute(app);
    }
    if inspector_action(ui, "PDK and model paths\u{2026}") {
        Command::PdkSettings.execute(app);
    }
}

fn project_recovery(ui: &mut Ui, app: &mut RSpiceApp) {
    let selected = app.state.workbench.project_checkpoint_selection.as_deref();
    let checkpoint = app
        .state
        .dialogs
        .project_checkpoint_recovery
        .checkpoints
        .iter()
        .find(|item| selected.is_some_and(|id| id == item.checkpoint_id().to_string()));
    let integrity = app
        .state
        .dialogs
        .project_checkpoint_recovery
        .error
        .is_none()
        && app
            .state
            .dialogs
            .project_checkpoint_recovery
            .quarantined
            .is_empty();

    // Only a selected checkpoint has anything to say. The revision counter,
    // the payload's byte count and the checkpoint's internal id are not
    // facts a reader restores on.
    if let Some(checkpoint) = checkpoint {
        section_header(ui, "Selected checkpoint", None);
        property_row(ui, "Reason", checkpoint.reason().label());
    }

    let quarantined = app
        .state
        .dialogs
        .project_checkpoint_recovery
        .quarantined
        .len();
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
fn library_usage(app: &RSpiceApp, reference: &CellViewRef) -> Vec<String> {
    let mut consumers = app
        .state
        .cell_instances(&reference.library, &reference.cell, Some(&reference.view))
        .into_iter()
        .map(|usage| usage.schematic.display_path())
        .collect::<Vec<_>>();
    consumers.dedup();
    consumers
}

/// The model an instance runs, and what is wrong with that binding when
/// something is. A binding that resolves says nothing beyond the model's name.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ComponentModelEvidence {
    /// The model card or module the deck gives the instance. `None` for a
    /// device that takes no model, or whose model the netlister writes itself.
    model: Option<String>,
    /// The binding's problem, worded as the problem.
    problem: Option<(Problem, String)>,
}

impl ComponentModelEvidence {
    fn resolved(model: impl Into<String>) -> Self {
        Self {
            model: Some(model.into()),
            problem: None,
        }
    }

    fn failed(model: impl Into<String>, problem: Problem, text: String) -> Self {
        Self {
            model: Some(model.into()),
            problem: Some((problem, text)),
        }
    }
}

/// What an instance of a project-owned Verilog-A view runs: the module its
/// view names, and whether its source has a compile that is still current.
/// No model catalog is involved, so none is reported as unverified.
fn project_module_evidence(
    state: &AppState,
    binding: &crate::state::LibraryCellInstance,
) -> Option<ComponentModelEvidence> {
    let reference = CellViewRef::new(&binding.library, &binding.cell, &binding.view);
    let view = state
        .library_manager
        .get_library(&reference.library)?
        .get_cell(&reference.cell)?
        .get_view(&reference.view)?;
    if view.view_type != ViewType::VerilogA {
        return None;
    }
    let model = view
        .metadata
        .get("veriloga.module")
        .cloned()
        .unwrap_or_else(|| binding.cell.clone());
    let bundle = state
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&crate::state::ProjectSourceOwner::cell_view(reference));
    Some(match bundle {
        Some(bundle) if bundle.validation_is_current() => ComponentModelEvidence::resolved(model),
        Some(_) => ComponentModelEvidence::failed(
            model,
            Problem::Warning,
            "Recompile needed: the module changed since its compile".to_owned(),
        ),
        None => ComponentModelEvidence::failed(
            model,
            Problem::Failure,
            "No Verilog-A source for this view".to_owned(),
        ),
    })
}

fn component_model_evidence(state: &AppState, component: &Component) -> ComponentModelEvidence {
    if let Some(binding) = component.library_cell.as_ref() {
        if binding.is_builtin_xspice() {
            return match crate::state::validate_builtin_xspice_binding(binding) {
                Ok(descriptor) => ComponentModelEvidence::resolved(descriptor.model_type),
                Err(error) => ComponentModelEvidence::failed(
                    binding.cell.clone(),
                    Problem::Failure,
                    format!("Model invalid: {error}"),
                ),
            };
        }
        if binding.is_generated_veriloga() {
            return match crate::state::validate_generated_veriloga_binding(binding) {
                Ok(descriptor) => ComponentModelEvidence::resolved(descriptor.model_name),
                Err(error) => ComponentModelEvidence::failed(
                    binding.cell.clone(),
                    Problem::Failure,
                    format!("Model invalid: {error}"),
                ),
            };
        }
        if let Some(evidence) = project_module_evidence(state, binding) {
            return evidence;
        }
        if let Some(library) = state.model_library_manager.get_library(&binding.library) {
            let candidates = [
                binding.module_name.as_deref(),
                Some(binding.cell.as_str()),
                (!component.value.trim().is_empty()).then_some(component.value.trim()),
            ];
            if let Some(model) = candidates.into_iter().flatten().find_map(|candidate| {
                library
                    .models
                    .values()
                    .find(|model| model.name.eq_ignore_ascii_case(candidate))
            }) {
                return catalog_model_evidence(library, model);
            }
        }

        let model = binding
            .module_name
            .clone()
            .unwrap_or_else(|| binding.cell.clone());
        if binding.source_path.is_some() {
            // The deck includes the file the binding names; no loaded library
            // vouches for what is in it.
            let text = format!("Model {model} is not in a loaded library");
            return ComponentModelEvidence::failed(model, Problem::Warning, text);
        }

        let project_view_exists = state
            .library_manager
            .get_library(&binding.library)
            .and_then(|library| library.get_cell(&binding.cell))
            .and_then(|cell| cell.get_view(&binding.view))
            .is_some();
        return if project_view_exists {
            ComponentModelEvidence::resolved(model)
        } else {
            let text = format!(
                "Library cell {}/{} ({}) not found",
                binding.library, binding.cell, binding.view
            );
            ComponentModelEvidence::failed(model, Problem::Failure, text)
        };
    }

    if let Some(model_name) = explicit_component_model(component) {
        // Resolution is a project-global decision, not a per-surface scan.
        // Counting name matches reported "ambiguous" for every contested
        // definition — including ones the project had already resolved, which
        // is what execution and the Models workspace both actually run.
        let not_found = |model_name: String| {
            let text = format!("Model {model_name} not found in the loaded libraries");
            ComponentModelEvidence::failed(model_name, Problem::Failure, text)
        };
        return match state
            .model_library_manager
            .effective_definition_provider(ModelConsumerScope::PrimitiveModel, &model_name)
        {
            Ok(Some(provider)) => state
                .model_library_manager
                .get_library(&provider.library)
                .and_then(|library| {
                    library
                        .models
                        .values()
                        .find(|model| model.name == provider.definition)
                        .map(|model| catalog_model_evidence(library, model))
                })
                .unwrap_or_else(|| not_found(model_name)),
            Ok(None) => not_found(model_name),
            Err(reason) => {
                let text = format!("Model {model_name} ambiguous: {reason}");
                ComponentModelEvidence::failed(model_name, Problem::Failure, text)
            }
        };
    }

    // A native device, or one whose model card the netlister writes from the
    // instance itself: there is no model to name.
    ComponentModelEvidence {
        model: None,
        problem: None,
    }
}

fn catalog_model_evidence(
    library: &crate::state::model_library::ModelLibrary,
    model: &crate::state::model_library::DeviceModel,
) -> ComponentModelEvidence {
    let external = library.root_path.is_some() || model.file_path.is_some();
    if !external {
        return ComponentModelEvidence::resolved(model.name.clone());
    }

    let source = model.file_path.as_deref().or(library.root_path.as_deref());
    let recorded = library.root_path.as_ref().and_then(|_| {
        source.and_then(|source| library.source_closure.iter().find(|pin| pin.path == source))
    });
    match (source, recorded) {
        (Some(_), Some(_)) => ComponentModelEvidence::resolved(model.name.clone()),
        (Some(source), None) => {
            let text = format!(
                "Model file {} is not one of library {}'s files",
                source.display(),
                library.name
            );
            ComponentModelEvidence::failed(model.name.clone(), Problem::Failure, text)
        }
        (None, _) => {
            let text = format!(
                "Library {} does not say which file holds {}",
                library.name, model.name
            );
            ComponentModelEvidence::failed(model.name.clone(), Problem::Failure, text)
        }
    }
}

fn results(ui: &mut Ui, app: &mut RSpiceApp) {
    let selected_trace = app
        .state
        .ui
        .results
        .valid_selected_trace(&app.state.simulation)
        .cloned();
    let selected_artifact = app
        .state
        .ui
        .results
        .session
        .selected_result_artifact
        .clone()
        .filter(|key| key.resolve(&app.state.simulation.retained.runs).is_some());

    // What is being read comes before where it came from: a reader adjusting
    // a pane needs its axes and bindings first, and the dataset provenance
    // below is unchanged by any of that.
    active_result_pane(ui, app, selected_trace.as_ref());
    if let Some(selected) = selected_trace.as_ref() {
        selected_result_trace(ui, app, selected);
    } else if let Some(selected) = selected_artifact.as_ref() {
        selected_result_artifact(ui, app, selected);
    }

    // The workspace's memoized projection: taking it fresh re-digested the run.
    let manifest = active_manifest(&mut app.state);
    let details = match app.state.simulation.active_run().zip(manifest.as_deref()) {
        Some((run, manifest)) => result_authority::result_run(ui, run, &manifest.model),
        None => {
            muted_inspector_copy(ui, "No run selected.");
            None
        }
    };
    if let Some(run_id) = details {
        run_details::open(&mut app.state, run_id);
    }

    // The active viewer owns its engineering readout (measurements, margins,
    // harmonics, eye metrics, distribution statistics, and so on). It follows
    // the trace and pane controls, matching the upgraded mockup's hierarchy
    // without creating a second result-data owner here.
    if app.state.ui.results.session.viewer != ResultViewer::Manifest {
        crate::workbench::documents::result_document::right_panel(ui, &mut app.state);
    }

    let Some(run) = app.state.simulation.active_run() else {
        return;
    };
    if let Some(index) = app.state.simulation.view.active_analysis_idx
        && let Some(analysis) = run.analyses.get(index)
    {
        result_authority::result_convergence(ui, analysis);
        result_authority::result_dc_sweep(ui, analysis);
    }
}

fn selected_result_artifact(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    selected: &rspice_results_ui::selection::ResultArtifactPresentationKey,
) {
    let Some((run_index, _, analysis)) = selected.resolve(&app.state.simulation.retained.runs)
    else {
        return;
    };
    let run = &app.state.simulation.retained.runs[run_index];
    // Memoized per dataset generation: the validator walks every retained
    // sample. Intact data is the normal state and says nothing.
    let corrupted = analysis_evidence_failure(&app.state, run.dataset_id, analysis);
    let analysis_label = analysis.label.clone();

    section_header(ui, "Selected result", None);
    property_row(ui, "Name", selected.canonical_name());
    property_row(ui, "Analysis", &analysis_label);
    if let Some(error) = corrupted {
        property_row_problem(ui, "Data", &error.to_string(), Some(Problem::Failure));
    }
}

fn selected_result_trace(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    selected: &rspice_results_ui::selection::SelectedResultTrace,
) {
    let Some(run) = app.state.simulation.active_run() else {
        return;
    };
    let Some((analysis_index, waveform_index, analysis, waveform)) = selected.resolve(run) else {
        return;
    };

    let waveform_name = waveform.name.clone();
    let waveform_color = waveform.color.clone();
    let waveform_visible = waveform.visible;
    let sample_count = waveform.x.len().min(waveform.y.len());
    let statistics = finite_trace_statistics(&waveform.y);
    let available_sample_count = waveform.y.iter().filter(|value| value.is_finite()).count();
    // The statistics are measurements of the trace, read in the trace's own
    // unit at the precision this panel's rows hold.
    let unit = crate::workbench::documents::result_document::browser_signal_unit(
        &waveform.name,
        waveform.unit.as_deref(),
        crate::workbench::documents::result_document::analysis_default_unit(analysis.analysis_type),
    )
    .to_owned();
    let measured = |value: f64| compact_readout(value, &unit);

    section_header(ui, "Selected trace", None);
    property_row(ui, "Name", &waveform_name);
    // Samples the solver could not give a finite value are a problem with
    // the trace; a complete trace says nothing about its sample count.
    if available_sample_count < sample_count {
        property_row_problem(
            ui,
            "Finite samples",
            &format!("{available_sample_count} of {sample_count}"),
            Some(Problem::Warning),
        );
    }
    if let Some(statistics) = statistics {
        property_row(ui, "Minimum", &measured(statistics.minimum));
        property_row(ui, "Maximum", &measured(statistics.maximum));
        property_row(ui, "Mean", &measured(statistics.mean));
        property_row(
            ui,
            "Peak-to-peak",
            &measured(statistics.maximum - statistics.minimum),
        );
    }

    let t = Tokens::get(ui.ctx());
    let mut color = rspice_results_ui::presentation::trace_color(
        &waveform_color,
        t.color.traces[waveform_index % t.color.traces.len()],
    );
    if trace_colour_row(ui, &mut color) {
        set_selected_trace_color(&mut app.state, selected, color);
    }

    section_block_gap(ui);
    ui.horizontal_wrapped(|ui| {
        if ui
            .button(if waveform_visible {
                "Hide trace"
            } else {
                "Show trace"
            })
            .clicked()
        {
            crate::workbench::documents::result_document::toggle_visibility(
                &mut app.state,
                analysis_index,
                waveform_index,
            );
        }

        if is_schematic_cross_probe_candidate(selected.source_name()) {
            let signal = selected.source_name().to_owned();
            let unavailable = schematic_cross_probe_unavailability(&app.state, selected, &signal);
            let mut response =
                ui.add_enabled(unavailable.is_none(), egui::Button::new("Cross-probe net"));
            if let Some(reason) = unavailable {
                response = response.on_hover_text(reason);
            }
            if response.clicked() {
                match cross_probe_trace_to_design(app, &signal) {
                    Ok(net) => app
                        .state
                        .prompt(format!("Selected conductor {net} from {signal}.")),
                    Err(message) => app.state.prompt(message),
                }
            }
        }
    });
}

fn active_result_pane(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    selected: Option<&rspice_results_ui::selection::SelectedResultTrace>,
) {
    let viewer = app.state.ui.results.session.viewer;
    // A pure evidence table — OP, specs, samples, events, the manifest — has
    // no drawn pane at all. Reporting one would have the inspector state a
    // view, a fit and a limit mask for a sheet that has none of them.
    if !crate::workbench::documents::result_document::viewer_draws_a_pane(viewer) {
        return;
    }
    let Some(run) = app.state.simulation.active_run() else {
        return;
    };
    let analysis_index = selected
        .and_then(|trace| {
            trace
                .resolve(run)
                .map(|(analysis_index, _, _, _)| analysis_index)
        })
        .or(app.state.simulation.view.active_analysis_idx)
        .unwrap_or(0);
    let Some(analysis) = run.analyses.get(analysis_index) else {
        return;
    };
    let pane_label = analysis.label.clone();
    // The unit names the pane in a unit-scoped stack; the viewer name is
    // already on the sheet tab above.
    let tokens = Tokens::get(ui.ctx());
    let facts =
        crate::workbench::documents::result_document::active_pane_facts(&tokens, &mut app.state);
    section_header(
        ui,
        "Active pane",
        Some(facts.unit.as_deref().unwrap_or_else(|| viewer.label())),
    );
    // The pane names the analysis it draws, which on a sheet outside the
    // waveform stack is not the one the run's analysis selector points at.
    property_row(ui, "Pane", facts.analysis.as_deref().unwrap_or(&pane_label));
    if let Some(scale) = facts.scale {
        property_row(ui, "Scale", scale);
    }
    property_row(ui, "Limit mask", facts.limit_mask);
    // Explicit limits, each with its own fit. Comparing two runs on identical
    // axes, or holding a decade while a parameter sweeps, cannot be done by
    // dragging; the fields show the viewport, so no row restates it.
    axis_limit_row(ui, app, &facts, PaneAxis::X, "X range");
    axis_limit_row(ui, app, &facts, PaneAxis::Y, "Y range");
}

/// An explicit interval for one axis of the active pane, or automatic fit.
///
/// The plot's own gestures can only ever say "a bit more than that". Holding
/// a decade while a parameter sweeps, or putting two runs on identical axes
/// so their curves can be compared at all, needs the numbers themselves.
fn axis_limit_row(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    facts: &crate::workbench::documents::result_document::ActivePaneFacts,
    axis: PaneAxis,
    label: &str,
) {
    use crate::workbench::documents::result_document::axis_range::{
        format_axis_range, parse_axis_range,
    };
    use crate::workbench::documents::result_document::{
        active_axis_is_pinned, active_axis_range, set_active_axis_range,
    };

    let viewer = app.state.ui.results.session.viewer;
    let current = active_axis_range(&app.state, facts, axis);
    let pinned = active_axis_is_pinned(&app.state, axis);
    // The unit the field writes after its numbers, and strips before reading
    // them back. Only the waveform stack knows its axes' units; every other
    // sheet's field states bare engineering numbers, as it always has.
    let unit = match axis {
        PaneAxis::X => facts.x_unit.as_deref(),
        PaneAxis::Y => facts.unit.as_deref(),
    }
    .filter(|_| crate::workbench::documents::result_document::viewer_uses_wave_stack(viewer))
    .unwrap_or_default();
    let committed = current
        .map(|range| format_axis_range(range, unit))
        .unwrap_or_default();
    let mut text = match app.state.ui.results.session.axis_limit_draft.as_ref() {
        Some((draft_viewer, draft_axis, text))
            if *draft_viewer == viewer && *draft_axis == axis =>
        {
            text.clone()
        }
        _ => committed.clone(),
    };

    let parsed = parse_axis_range(&text, unit);
    let invalid = !text.trim().is_empty() && parsed.is_none();
    let (edit, reset) = property_row_input_action(
        ui,
        label,
        &mut text,
        invalid,
        crate::workbench::design_system::PropertyRowAction {
            icon: WorkbenchIcon::ZoomFit,
            label: "Fit this axis to the retained data",
            enabled: pinned,
            disabled_reason: Some("This axis is already fitting its data."),
        },
    );

    if edit.changed() {
        app.state.ui.results.session.axis_limit_draft = Some((viewer, axis, text.clone()));
    }
    // Commit on Enter or on leaving the field, never per keystroke: "1.2" is
    // a legal prefix of "1.2m" and three orders of magnitude away from it.
    if edit.lost_focus() {
        let tokens = Tokens::get(ui.ctx());
        if text == committed {
            // The field shows the interval to six significant digits. Read
            // back unedited, that would pin an autofitting axis, or nudge a
            // pinned one, by the rounding alone.
            app.state.ui.results.session.axis_limit_draft = None;
        } else if text.trim().is_empty() {
            set_active_axis_range(&tokens, &mut app.state, axis, None);
            app.state.ui.results.session.axis_limit_draft = None;
        } else if let Some(range) = parse_axis_range(&text, unit) {
            set_active_axis_range(&tokens, &mut app.state, axis, Some(range));
            app.state.ui.results.session.axis_limit_draft = None;
        }
    }
    if reset.clicked() {
        let tokens = Tokens::get(ui.ctx());
        set_active_axis_range(&tokens, &mut app.state, axis, None);
        app.state.ui.results.session.axis_limit_draft = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TraceStatistics {
    minimum: f64,
    maximum: f64,
    mean: f64,
}

fn finite_trace_statistics(values: &[f64]) -> Option<TraceStatistics> {
    let mut minimum = f64::INFINITY;
    let mut maximum = f64::NEG_INFINITY;
    let mut sum = 0.0;
    let mut count = 0_u64;
    for value in values.iter().copied().filter(|value| value.is_finite()) {
        minimum = minimum.min(value);
        maximum = maximum.max(value);
        sum += value;
        count += 1;
    }
    (count != 0).then_some(TraceStatistics {
        minimum,
        maximum,
        mean: sum / count as f64,
    })
}

/// The significant digits a compact panel readout holds: an operating-point
/// row, a node voltage, a trace's statistics. Four, as an annotated operating
/// point reads in Spectre — the reader's own precision belongs to the result
/// tables, which have the width to spend it.
const COMPACT_SIGNIFICANT_DIGITS: usize = 4;

/// `value` in engineering notation with its unit, at the precision a compact
/// panel row reads at: `96.86 µA`, not `96.86112 µA`.
fn compact_readout(value: f64, unit: &str) -> String {
    engineering_readout(value, unit, COMPACT_SIGNIFICANT_DIGITS)
}

/// `value` in engineering notation with its unit, to the reader's chosen
/// number of significant digits, without the zeros that only pad it out:
/// `900 mV` and `4.1 V`, not `900.0000 mV` and `4.100000 V`.
fn engineering_readout(value: f64, unit: &str, significant_digits: usize) -> String {
    rspice_results_ui::waves::fmt_in_unit_readout(value, unit, significant_digits)
}

/// The trace's colour as one more property row: its name in the label column,
/// and in the value column the swatch that opens the picker beside the hex
/// value it holds. Returns whether the reader picked a different colour.
fn trace_colour_row(ui: &mut Ui, color: &mut Color32) -> bool {
    use super::super::design_system::{property_row_control_columns, property_row_height};
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let trailing = panel_inset(ui);
    let (label_left, value_left) = property_row_control_columns(ui, width);
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(width, property_row_height(ui)), Sense::hover());
    let label_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    ui.painter().text(
        Pos2::new(rect.left() + label_left, rect.center().y),
        Align2::LEFT_CENTER,
        "Colour",
        label_font,
        t.color.text_dim,
    );
    let value_rect = Rect::from_min_max(
        Pos2::new(rect.left() + value_left, rect.top()),
        Pos2::new(rect.right() - trailing, rect.bottom()),
    );
    // The swatch is the row's field: as tall as a field in any other row, a
    // pixel in from the row's top and bottom, not a free-standing control
    // that would stand out of the row.
    let field = value_rect.shrink2(Vec2::new(0.0, 1.0));
    let swatch = ui
        .scope_builder(
            egui::UiBuilder::new()
                .max_rect(field)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
            |ui| {
                ui.spacing_mut().interact_size.y = field.height();
                egui::color_picker::color_edit_button_srgba(
                    ui,
                    color,
                    egui::color_picker::Alpha::Opaque,
                )
            },
        )
        .inner;
    // Placing the swatch left the cursor under it; the row ends at its own
    // bottom.
    let _ = ui.advance_cursor_after_rect(rect);
    let hex = format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b());
    ui.painter().with_clip_rect(value_rect).text(
        Pos2::new(swatch.rect.right() + 8.0, rect.center().y),
        Align2::LEFT_CENTER,
        &hex,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text,
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label("Colour");
        node.set_value(hex.as_str());
    });
    swatch.changed()
}

fn set_selected_trace_color(
    state: &mut AppState,
    selected: &rspice_results_ui::selection::SelectedResultTrace,
    color: Color32,
) {
    let color = format!("#{:02x}{:02x}{:02x}", color.r(), color.g(), color.b());
    let Some(run_index) = state.simulation.view.active_run_idx else {
        return;
    };
    let Some((analysis_index, waveform_index)) = state
        .simulation
        .retained
        .runs
        .get(run_index)
        .and_then(|run| {
            selected
                .resolve(run)
                .map(|(analysis_index, waveform_index, _, _)| (analysis_index, waveform_index))
        })
    else {
        return;
    };
    if let Some(waveform) = state
        .simulation
        .retained
        .runs
        .get_mut(run_index)
        .and_then(|run| run.analyses.get_mut(analysis_index))
        .and_then(|analysis| analysis.waveforms.get_mut(waveform_index))
        .filter(|waveform| waveform.name == selected.source_name())
    {
        waveform.color.clone_from(&color);
    } else {
        return;
    }
    if state.simulation.view.active_analysis_idx == Some(analysis_index)
        && let Some(waveform) = state
            .simulation
            .view
            .waveforms
            .iter_mut()
            .find(|waveform| waveform.name == selected.source_name())
    {
        waveform.color = color;
    }
}

fn is_schematic_cross_probe_candidate(signal: &str) -> bool {
    (signal.starts_with("V(") || signal.starts_with("I(")) && signal.ends_with(')')
}

fn schematic_cross_probe_unavailability(
    state: &AppState,
    selected: &rspice_results_ui::selection::SelectedResultTrace,
    signal: &str,
) -> Option<String> {
    let Some(net) = rspice_schematic_editor::view::cross_probe::wrapped_signal_name(signal, 'V')
    else {
        return Some(format!(
            "{signal} is a device current or derived quantity; no single schematic net carries it."
        ));
    };
    let map = &state.simulation.source.cross_probe;
    let topology = state.schematic.topology_version();
    if !map.is_current_for(&state.workspace.content.active_view, topology) {
        return Some(
            "The schematic changed since this result was produced; run again to cross-probe it."
                .to_owned(),
        );
    }
    let Some(run) = state.simulation.active_run() else {
        return Some("The selected result dataset is no longer retained.".to_owned());
    };
    if run.dataset_id != selected.dataset_id() || selected.resolve(run).is_none() {
        return Some(
            "The selected trace does not belong to the active immutable result dataset.".to_owned(),
        );
    }
    let Some(receipt) = run.prepared_receipt() else {
        return Some(
            "This legacy result has no authenticated project revision and cannot be cross-probed."
                .to_owned(),
        );
    };
    if receipt.project_revision() != state.workspace.content.project.revision() {
        return Some(
            "The selected result was produced from a different project revision; run again before cross-probing it."
                .to_owned(),
        );
    }
    let here = state.workspace.content.occurrence_path(); // only that occurrence answers for a trace
    // A trace is scoped by written instance names, `xamp` for a cell drawn
    // `amp`, so its scope is compared with the drawn occurrence by those names
    // and named to the reader as the occurrence the run drew.
    let leaf = match crate::state::ProbeTarget::parse_legacy(net) {
        Ok(probe) if !probe.scope.same_engine_scope(&here) => {
            let deeper = probe.scope.engine_scope_starts_with(&here);
            let verb = if deeper { "Descend" } else { "Ascend" };
            let drawn = receipt
                .hierarchy_map()
                .iter()
                .filter_map(|row| crate::state::InstancePath::parse(row.occurrence()).ok())
                .collect::<Vec<_>>();
            let scope = probe.scope.resolve_drawn(&drawn).unwrap_or(&probe.scope);
            return Some(format!("{verb} to {scope} to cross-probe it."));
        }
        Ok(probe) => probe.leaf,
        Err(_) => net.to_owned(),
    };
    let mut nets = map.net_to_points.iter();
    let known = nets.any(|(name, points)| name.eq_ignore_ascii_case(&leaf) && !points.is_empty());
    (!known).then(|| format!("The open sheet has no conductor named {leaf}."))
}

/// Resolve against the current schematic map before navigation. A stale or
/// derived signal therefore leaves the engineer in Results with an exact
/// explanation. On success, the Design surface opens; a remembered split
/// remains active and projects Design + the same canonical Results document.
fn cross_probe_trace_to_design(app: &mut RSpiceApp, signal: &str) -> Result<String, String> {
    let selected = app
        .state
        .ui
        .results
        .valid_selected_trace(&app.state.simulation)
        .ok_or_else(|| "No exact retained result trace is selected.".to_owned())?;
    if let Some(reason) = schematic_cross_probe_unavailability(&app.state, selected, signal) {
        return Err(reason);
    }
    let net = crate::schematic::view::select_signal_conductor(&mut app.state, signal)
        .map_err(|error| error.message(signal))?;
    if app.state.workbench.workspace != Workspace::Design {
        Command::OpenWorkspace(Workspace::Design).execute(app);
    }
    Ok(net)
}

/// The Verify inspector states only what its page does not: the Monte Carlo
/// run's definition and worst samples, a corner run that failed, and the
/// tuning session's uncommitted or invalid values. The other pages already
/// show everything there is to say about them.
fn verify(ui: &mut Ui, app: &mut RSpiceApp) {
    match app.state.workbench.verification_page {
        VerificationPage::Yield => yield_details(ui, app),
        VerificationPage::Corners => {
            let failed = app.state.simulation.active_run().and_then(|run| {
                run.analyses.iter().find(|analysis| {
                    analysis.analysis_type == crate::state::AnalysisType::Corner
                        && !analysis.success
                })
            });
            if let Some(analysis) = failed {
                let message = analysis.error_message.as_deref().map_or_else(
                    || "The corner run failed.".to_owned(),
                    |error| format!("The corner run failed: {error}"),
                );
                crate::ui::widgets::problem_line(ui, Problem::Failure, &message);
            }
        }
        VerificationPage::Tuning => {
            let session = &app.state.workbench.verification;
            let dirty_count = session
                .tuning_variables
                .iter()
                .filter(|draft| draft.is_dirty())
                .count();
            let invalid_count = session
                .tuning_variables
                .iter()
                .filter(|draft| draft.validation_error.is_some())
                .count();
            let uncommitted = format!("{dirty_count} uncommitted");
            section_header(
                ui,
                "Tuning",
                (dirty_count > 0).then_some(uncommitted.as_str()),
            );
            if invalid_count > 0 {
                super::super::design_system::property_row_problem(
                    ui,
                    "Validation",
                    &format!("{invalid_count} invalid"),
                    Some(Problem::Failure),
                );
            }
        }
        VerificationPage::Optimization
        | VerificationPage::DeviceSafety
        | VerificationPage::Regression => {}
    }
}

fn yield_details(ui: &mut Ui, app: &RSpiceApp) {
    let run = app.state.simulation.active_run();
    let evidence = app.state.simulation.yield_results_for_active_dataset();
    let provenance = evidence.and(app.state.simulation.retained.yield_provenance());
    let results = evidence.unwrap_or_default();

    section_header(ui, "Monte Carlo", None);
    if let Some(provenance) = provenance {
        property_row(
            ui,
            "Samples",
            &format!(
                "{} / {}",
                provenance.runs_completed, provenance.runs_requested
            ),
        );
        property_row(ui, "Seed", &format!("0x{:X}", provenance.seed));
        property_row(ui, "Sampling", provenance.sampling_mode.display_name());
        let (passing, total) = joint_yield(results).unwrap_or((0, 0));
        if let Some((low, high)) = wilson_interval_95(passing, total) {
            property_row(
                ui,
                "Confidence",
                &format!("95% CI {:.2}–{:.2}%", low * 100.0, high * 100.0),
            );
        }
        if let Some(run) = run {
            property_row(ui, "Duration", &compact_readout(run.elapsed_time, "s"));
        }
    } else {
        muted_inspector_copy(ui, "No Monte Carlo run");
    }

    let failures = worst_samples(results);
    if !failures.is_empty() {
        let failed_count = joint_yield(results).map_or(0, |(passing, total)| total - passing);
        let failed_label = format!("{failed_count} fail");
        section_header(ui, "Worst samples", Some(&failed_label));
        for failure in failures.iter().take(3) {
            failure_row(ui, failure);
        }
    }

    if !results.is_empty() {
        section_header(ui, "Specification yields", None);
        for result in results {
            property_row(
                ui,
                &result.spec.target,
                &format!(
                    "{:.2}% · {} / {}",
                    result.yield_percent, result.pass_count, result.total_runs
                ),
            );
        }
    }
}

#[derive(Debug)]
struct FailedSample {
    target: String,
    unit: String,
    sample_index: usize,
    value: f64,
    normalized_margin: f64,
}

fn worst_samples(results: &[rspice_results::yield_analysis::YieldResult]) -> Vec<FailedSample> {
    let mut failures = results
        .iter()
        .flat_map(|result| {
            result
                .samples
                .iter()
                .copied()
                .enumerate()
                .filter(|(index, value)| {
                    result.trail.get(*index).is_some_and(|passes| !passes) && value.is_finite()
                })
                .filter_map(|(index, value)| {
                    normalized_yield_margin(&result.spec, value).map(|normalized_margin| {
                        FailedSample {
                            target: result.spec.target.clone(),
                            unit: result.spec.unit.clone(),
                            sample_index: index,
                            value,
                            normalized_margin,
                        }
                    })
                })
        })
        .collect::<Vec<_>>();
    failures.sort_by(|a, b| {
        a.normalized_margin
            .partial_cmp(&b.normalized_margin)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.target.cmp(&b.target))
            .then_with(|| a.sample_index.cmp(&b.sample_index))
    });
    failures
}

fn normalized_yield_margin(
    spec: &rspice_results::yield_analysis::YieldSpec,
    value: f64,
) -> Option<f64> {
    const SCALE_FLOOR: f64 = 1.0e-30;
    match (spec.min, spec.max) {
        (Some(min), Some(max)) if max > min => {
            Some((value - min).min(max - value) / (max - min).max(SCALE_FLOOR))
        }
        (Some(min), None) => Some((value - min) / min.abs().max(SCALE_FLOOR)),
        (None, Some(max)) => Some((max - value) / max.abs().max(SCALE_FLOOR)),
        (Some(_), Some(_)) | (None, None) => None,
    }
}

const FAILURE_ROW_PADDING_Y: f32 = 8.0;
const FAILURE_ROW_COLUMN_GAP: f32 = 8.0;
const FAILURE_ROW_DETAIL_GAP: f32 = 4.0;

#[derive(Debug, Clone, Copy, PartialEq)]
struct FailureRowColumns {
    target_width: f32,
    margin_width: f32,
}

/// The target and margin columns of a failure row `row_width` wide whose text
/// is inset `padding_x` from both edges: the panel's own inset, so the row's
/// text lines up with the property rows around it.
fn failure_row_columns(
    row_width: f32,
    measured_margin_width: f32,
    padding_x: f32,
) -> FailureRowColumns {
    let content_width = (row_width - padding_x * 2.0).max(1.0);
    let margin_width = measured_margin_width.max(1.0).min(content_width);
    let target_width = (content_width - margin_width - FAILURE_ROW_COLUMN_GAP).max(1.0);
    FailureRowColumns {
        target_width,
        margin_width,
    }
}

fn failure_row(ui: &mut Ui, failure: &FailedSample) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let title = format!("#{:04} · {}", failure.sample_index + 1, failure.target);
    let margin = format!("{:+.2}%", failure.normalized_margin * 100.0);
    let detail = format!(
        "observed {}",
        crate::ui::plot::fmt_si_significant(failure.value, &failure.unit, 4)
    );
    let title_font = theme::sans(tokens::FS_0, FontWeight::Medium);
    let margin_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let detail_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let measured_margin = ui
        .painter()
        .layout_no_wrap(margin, margin_font, t.color.err);
    let padding_x = panel_inset(ui);
    let columns = failure_row_columns(width, measured_margin.size().x, padding_x);
    let title_galley = ui
        .painter()
        .layout(title, title_font, t.color.text, columns.target_width);
    let content_width = (width - padding_x * 2.0).max(1.0);
    let detail_galley = ui
        .painter()
        .layout(detail, detail_font, t.color.text_faint, content_width);
    let title_height = title_galley.size().y.max(measured_margin.size().y);
    let natural_height = FAILURE_ROW_PADDING_Y * 2.0
        + title_height
        + FAILURE_ROW_DETAIL_GAP
        + detail_galley.size().y;
    let minimum_height = if t.metrics.ctl_h >= 44.0 { 58.0 } else { 47.0 };
    let height = natural_height.max(minimum_height);
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            format!(
                "Sample {}. {}. Value {} {}. Margin {:+.2}%",
                failure.sample_index + 1,
                failure.target,
                failure.value,
                failure.unit,
                failure.normalized_margin * 100.0
            ),
        )
    });
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    let title_top = rect.top() + FAILURE_ROW_PADDING_Y;
    let target_rect = Rect::from_min_size(
        Pos2::new(rect.left() + padding_x, title_top),
        Vec2::new(columns.target_width, title_height),
    );
    ui.painter().with_clip_rect(target_rect).galley(
        target_rect.left_top(),
        title_galley,
        t.color.text,
    );
    let margin_rect = Rect::from_min_size(
        Pos2::new(rect.right() - padding_x - columns.margin_width, title_top),
        Vec2::new(columns.margin_width, title_height),
    );
    ui.painter().with_clip_rect(margin_rect).galley(
        Pos2::new(
            margin_rect.right() - measured_margin.size().x,
            margin_rect.top(),
        ),
        measured_margin,
        t.color.err,
    );
    let detail_top = title_top + title_height + FAILURE_ROW_DETAIL_GAP;
    let detail_rect = Rect::from_min_max(
        Pos2::new(rect.left() + padding_x, detail_top),
        Pos2::new(rect.right() - padding_x, rect.bottom()),
    );
    ui.painter().with_clip_rect(detail_rect).galley(
        detail_rect.left_top(),
        detail_galley,
        t.color.text_faint,
    );
    response
}

fn joint_yield(results: &[rspice_results::yield_analysis::YieldResult]) -> Option<(usize, usize)> {
    let total = results.first()?.total_runs;
    if total == 0
        || results
            .iter()
            .any(|result| result.total_runs != total || result.trail.len() != total)
    {
        return None;
    }
    Some((
        (0..total)
            .filter(|index| results.iter().all(|result| result.trail[*index]))
            .count(),
        total,
    ))
}

fn wilson_interval_95(successes: usize, total: usize) -> Option<(f64, f64)> {
    if total == 0 || successes > total {
        return None;
    }
    let n = total as f64;
    let p = successes as f64 / n;
    let z = 1.959_963_984_540_054_f64;
    let z2 = z * z;
    let denominator = 1.0 + z2 / n;
    let center = (p + z2 / (2.0 * n)) / denominator;
    let half = z * ((p * (1.0 - p) / n + z2 / (4.0 * n * n)).sqrt()) / denominator;
    Some(((center - half).max(0.0), (center + half).min(1.0)))
}

fn code_workspace_inspector(ui: &mut Ui, app: &mut RSpiceApp) {
    if app.state.ui.code_workspace.page
        == crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist
    {
        netlist(ui, app);
        return;
    }

    let messages = app.state.ui.messages();
    let Some(context) = crate::workbench::commands::code_context::resolve(app) else {
        muted_inspector_copy(
            ui,
            &messages.text(MessageId::CodeInspectorContextUnavailable),
        );
        return;
    };
    let ownership = messages.text(match context.ownership {
        crate::workbench::commands::code_context::CodeDocumentOwnership::GeneratedReadOnly => {
            MessageId::CodeInspectorGeneratedReadOnly
        }
        crate::workbench::commands::code_context::CodeDocumentOwnership::ProjectOwned => {
            MessageId::CodeInspectorProjectOwned
        }
        crate::workbench::commands::code_context::CodeDocumentOwnership::ExternalReadOnly => {
            MessageId::CodeInspectorExternalReadOnly
        }
        crate::workbench::commands::code_context::CodeDocumentOwnership::ComparisonReadOnly => {
            MessageId::CodeInspectorComparisonReadOnly
        }
        crate::workbench::commands::code_context::CodeDocumentOwnership::GovernedReadOnly => {
            MessageId::CodeInspectorGovernedReadOnly
        }
    });
    // Whether this document can be edited, and nothing else. The page tab
    // names the page, the document tab names the path, and the toolbar's own
    // buttons are the one true statement of which commands are available:
    // five rows reading "available" were that statement copied out by hand.
    property_row(
        ui,
        &messages.text(MessageId::CodeInspectorOwnership),
        &ownership,
    );
}

fn netlist(ui: &mut Ui, app: &mut RSpiceApp) {
    use crate::workbench::documents::netlist_document::{
        ActiveNetlistDocument, DiagnosticSeverity,
    };

    let messages = app.state.ui.messages();
    const DIAGNOSTIC_PREVIEW_LIMIT: usize = 20;
    let diagnostics = std::sync::Arc::clone(&app.state.ui.netlist.diagnostics);
    let summary = diagnostics.summary();

    if summary.current_errors > 0 {
        code_inspector_section(
            ui,
            &messages.text(crate::workbench::MessageId::NetlistErrors),
            Some(&summary.current_errors.to_string()),
            Some(Problem::Failure),
            |ui| {
                for diagnostic in diagnostics
                    .iter()
                    .filter(|diagnostic| {
                        diagnostic.is_current() && diagnostic.severity == DiagnosticSeverity::Error
                    })
                    .take(DIAGNOSTIC_PREVIEW_LIMIT)
                {
                    diagnostic_row(ui, diagnostic);
                }
                if summary.current_errors > DIAGNOSTIC_PREVIEW_LIMIT {
                    empty_diagnostic_row(
                        ui,
                        &messages.format(
                            crate::workbench::MessageId::NetlistMoreInProblems,
                            &[(
                                "count",
                                &(summary.current_errors - DIAGNOSTIC_PREVIEW_LIMIT).to_string(),
                            )],
                        ),
                    );
                }
            },
        );
    }

    // A document with nothing to advise shows no Advisories section. A
    // section whose whole body says it is empty is a heading spent on the
    // absence of a problem.
    let advisories = summary.current_advisories();
    if advisories > 0 {
        code_inspector_section(
            ui,
            &messages.text(crate::workbench::MessageId::NetlistAdvisories),
            Some(&advisories.to_string()),
            Some(Problem::Warning),
            |ui| {
                for diagnostic in diagnostics
                    .iter()
                    .filter(|diagnostic| {
                        diagnostic.is_current() && diagnostic.severity != DiagnosticSeverity::Error
                    })
                    .take(DIAGNOSTIC_PREVIEW_LIMIT)
                {
                    diagnostic_row(ui, diagnostic);
                }
                if advisories > DIAGNOSTIC_PREVIEW_LIMIT {
                    empty_diagnostic_row(
                        ui,
                        &messages.format(
                            crate::workbench::MessageId::NetlistMoreInProblems,
                            &[(
                                "count",
                                &(advisories - DIAGNOSTIC_PREVIEW_LIMIT).to_string(),
                            )],
                        ),
                    );
                }
            },
        );
    }

    // The sandbox itself, rather than a paragraph recommending it.
    let tuning = crate::workbench::commands::vocabulary::Command::VerificationPage(
        crate::workbench::state::VerificationPage::Tuning,
    );
    if let crate::workbench::commands::CommandAvailability::Available = tuning.availability(app) {
        section_header(
            ui,
            &messages.text(crate::workbench::MessageId::NetlistParameterExploration),
            None,
        );
        if crate::ui::widgets::Button::new(
            &messages.text(crate::workbench::MessageId::NetlistTuneParameters),
        )
        .show(ui)
        .clicked()
        {
            tuning.execute(app);
        }
    }

    if crate::workbench::documents::netlist_document::active_dependency(&app.state).is_some() {
        dependency_provenance(ui, &app.state);
    } else {
        match app.state.ui.netlist.active_document {
            ActiveNetlistDocument::OwnedSource => owned_source_provenance(ui, &app.state),
            ActiveNetlistDocument::RunSnapshot => {} // its run's, stated by the strip over the deck
            _ => generated_provenance(ui, &app.state),
        }
    }
}

fn diagnostic_row(
    ui: &mut Ui,
    diagnostic: &crate::workbench::documents::netlist_document::Diagnostic,
) {
    const ICON_COLUMN_W: f32 = 14.0;
    const COLUMN_GAP: f32 = 7.0;
    const PADDING_Y: f32 = 8.0;
    const DETAIL_GAP: f32 = 3.0;

    let t = Tokens::get(ui.ctx());
    let padding_x = panel_inset(ui);
    let width = ui.available_width().max(1.0);
    let text_width = (width - padding_x * 2.0 - ICON_COLUMN_W - COLUMN_GAP).max(1.0);
    let message_galley = ui.painter().layout(
        diagnostic.message.clone(),
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
        text_width,
    );
    let location = diagnostic_location(diagnostic);
    let location_galley = ui.painter().layout(
        location.clone(),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
        text_width,
    );
    let message_height = message_galley.size().y;
    let height = PADDING_Y * 2.0 + message_height + DETAIL_GAP + location_galley.size().y;
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            format!(
                "{}: {}. {location}",
                diagnostic_severity_name(diagnostic.severity),
                diagnostic.message
            ),
        )
    });
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );

    let icon = match diagnostic.severity {
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Hint
        | crate::workbench::documents::netlist_document::DiagnosticSeverity::Info => {
            WorkbenchIcon::Info
        }
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning
        | crate::workbench::documents::netlist_document::DiagnosticSeverity::Error => {
            WorkbenchIcon::Warning
        }
    };
    icon.paint(
        ui.painter(),
        Rect::from_min_size(
            Pos2::new(rect.left() + padding_x, rect.top() + PADDING_Y),
            Vec2::splat(ICON_COLUMN_W),
        ),
        diagnostic_tone(&t, diagnostic.severity),
    );

    let text_x = rect.left() + padding_x + ICON_COLUMN_W + COLUMN_GAP;
    ui.painter().galley(
        Pos2::new(text_x, rect.top() + PADDING_Y),
        message_galley,
        t.color.text,
    );
    ui.painter().galley(
        Pos2::new(text_x, rect.top() + PADDING_Y + message_height + DETAIL_GAP),
        location_galley,
        t.color.text_faint,
    );
}

fn empty_diagnostic_row(ui: &mut Ui, message: &str) {
    let faint = Tokens::get(ui.ctx()).color.text_faint;
    rspice_ui_kit::panels::inspector::toned_inspector_copy(ui, message, faint);
}

/// Where the generated deck came from, and what is wrong with it.
///
/// The input revision, the input digest and the netlist digest were three
/// counters and hashes a designer cannot act on; the generator's healthy
/// states reported success. What is left is the cellview that produced the
/// deck, and a marked line when the deck is not the one the schematic implies.
fn generated_provenance(ui: &mut Ui, state: &AppState) {
    let source = state
        .ui
        .netlist
        .generated_document
        .as_ref()
        .and_then(|document| document.generated_artifact())
        .and_then(|artifact| artifact.source_map().first())
        .map(|entry| entry.view_identity().to_owned());
    if let Some(source) = source {
        property_row(ui, "Generated from", &source);
    }
    if let Some((label, problem)) = generated_state(state) {
        property_row_problem(ui, "State", label, Some(problem));
    }
}

/// Which file the owned deck lives in, and what is wrong with it.
///
/// The project revision, the document revision, the generated base and the
/// two digests were five counters and hashes with nothing a designer can do
/// about them; the document state reported success in catalog words. What is
/// left is the file and a marked line when the deck needs attention.
fn owned_source_provenance(ui: &mut Ui, state: &AppState) {
    if let Some(path) = state.workspace.content.netlist_source_path.as_ref() {
        property_row(ui, "File", &path.display().to_string());
    }
    let digest = crate::state::content_digest(&state.simulation.source.netlist_content);
    if let Some(label) = owned_source_state(state, digest) {
        // The same split the deck toolbar paints, and for the same reason:
        // not yet checked is where the deck is, not a verdict on it. A file
        // that changed underneath the editor is the one to act on.
        let problem = state
            .ui
            .netlist
            .external_change
            .is_some()
            .then_some(Problem::Warning);
        property_row_problem(ui, "State", label, problem);
    }
}

fn dependency_provenance(ui: &mut Ui, state: &AppState) {
    let Some(dependency) = crate::workbench::documents::netlist_document::active_dependency(state)
    else {
        return;
    };
    let messages = state.ui.messages();
    section_header(
        ui,
        &messages.text(crate::workbench::MessageId::NetlistIncludeProvenance),
        None,
    );
    property_row(
        ui,
        &messages.text(crate::workbench::MessageId::NetlistDisplayName),
        dependency.locator().display_name(),
    );
    // Only worth a row when the deck asked for something other than what it
    // got: that difference is the thing a designer chases. When they agree the
    // row restates the display name directly above it.
    if dependency.requested_locator() != dependency.locator().display_name() {
        property_row(
            ui,
            &messages.text(crate::workbench::MessageId::NetlistRequestedAs),
            dependency.requested_locator(),
        );
    }
    // Where the bytes come from, and the one failure worth a mark here: an
    // include the deck names that nothing resolved to.
    property_row_problem(
        ui,
        &messages.text(crate::workbench::MessageId::NetlistOrigin),
        &if dependency.resolution().content_digest().is_none() {
            messages.text(crate::workbench::MessageId::NetlistUnresolved)
        } else {
            dependency
                .locator()
                .native_origin()
                .map(str::to_owned)
                .unwrap_or_else(|| {
                    messages.text(crate::workbench::MessageId::NetlistRetainedAuthenticatedSource)
                })
        },
        (dependency.resolution().content_digest().is_none()).then_some(Problem::Failure),
    );
    let relationship = messages.text(if dependency.direct_include_index().is_some() {
        crate::workbench::MessageId::NetlistDirectInclude
    } else {
        crate::workbench::MessageId::NetlistTransitiveInclude
    });
    property_row(
        ui,
        &messages.text(crate::workbench::MessageId::NetlistRelationship),
        &relationship,
    );
    if let Some(parent) = dependency.parent() {
        property_row(
            ui,
            &messages.text(crate::workbench::MessageId::NetlistParent),
            parent.display_name(),
        );
    }
    let owned = state
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .and_then(|descriptor| descriptor.owned_include(dependency.locator().logical_identity()));
    property_row(
        ui,
        &messages.text(crate::workbench::MessageId::NetlistOwnership),
        &owned.map_or_else(
            || {
                messages.text(match dependency.authority() {
                    crate::state::DependencySourceAuthority::External => {
                        crate::workbench::MessageId::NetlistExternalReferenceReadOnly
                    }
                    crate::state::DependencySourceAuthority::Vendor => {
                        crate::workbench::MessageId::NetlistVendorSourceReadOnly
                    }
                    crate::state::DependencySourceAuthority::TechnologyPackage => {
                        crate::workbench::MessageId::NetlistTechnologyPackageReadOnly
                    }
                    crate::state::DependencySourceAuthority::StandardLibrary => {
                        crate::workbench::MessageId::NetlistStandardLibraryReadOnly
                    }
                })
            },
            |_| messages.text(crate::workbench::MessageId::NetlistProjectOwnedEditable),
        ),
    );
}

/// What is wrong with the generated deck, or nothing when it matches the
/// schematic. A deck that is current — validated or not — says nothing.
fn generated_state(state: &AppState) -> Option<(&'static str, Problem)> {
    let netlist = &state.ui.netlist;
    if netlist.generated_source.is_empty() {
        Some(("Not generated yet", Problem::Warning))
    } else if netlist.generation_error.is_some() {
        Some((
            "Could not generate; showing the last deck",
            Problem::Failure,
        ))
    } else if netlist.generated_input_digest != netlist.current_generation_input_digest {
        Some(("Older than the schematic", Problem::Warning))
    } else {
        None
    }
}

fn owned_source_state(
    state: &AppState,
    digest: crate::product::ContentDigest,
) -> Option<&'static str> {
    crate::workbench::documents::netlist_document::owned_netlist_publication_state(state, digest)
        .label()
}

fn diagnostic_location(
    diagnostic: &crate::workbench::documents::netlist_document::Diagnostic,
) -> String {
    let location = match (
        diagnostic.source_line.or(diagnostic.line),
        diagnostic.column,
    ) {
        (Some(line), Some(column)) => format!("line {} · column {}", line + 1, column + 1),
        (Some(line), None) => format!("line {}", line + 1),
        (None, _) => "document scope".to_owned(),
    };
    match diagnostic.source_path.as_deref() {
        Some(path) if diagnostic.source_line.is_some() => {
            format!("{} · {location}", path.display())
        }
        Some(path) => path.display().to_string(),
        None => location,
    }
}

fn diagnostic_tone(
    tokens: &Tokens,
    severity: crate::workbench::documents::netlist_document::DiagnosticSeverity,
) -> Color32 {
    match severity {
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Hint => {
            tokens.color.text_dim
        }
        // An informational diagnostic is not a problem, so it is not news:
        // it reads as the hint beside it does. Warning and error keep their
        // tones because those are the two the reader must act on.
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Info => {
            tokens.color.text_dim
        }
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning => {
            tokens.color.warn
        }
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Error => {
            tokens.color.err
        }
    }
}

fn diagnostic_severity_name(
    severity: crate::workbench::documents::netlist_document::DiagnosticSeverity,
) -> &'static str {
    match severity {
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Hint => "Hint",
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Info => "Information",
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning => "Warning",
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Error => "Error",
    }
}

#[cfg(test)]
mod tests;
