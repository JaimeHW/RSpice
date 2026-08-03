//! Truthful project overview.
//!
//! This surface is a projection of authoritative project, lifecycle, design,
//! validation, simulation, and model state. It deliberately does not infer
//! autosave success, source-control history, or dependency authentication from
//! unrelated runtime flags.

use std::collections::BTreeSet;

use egui::{Align, Align2, Color32, Layout, Rect, Sense, Stroke, Ui, pos2, vec2};

use crate::services::drc::DrcSeverity;
use crate::state::netlist_document::{DiagnosticSeverity, DocumentOwnership};
use crate::state::{CellViewRef, SimulationRunLifecycle, ViewType};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::DesignCheckStatus;
use crate::workbench::commands::CommandAvailability;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::lifecycle::project_lifecycle::dirty_document_count;
use crate::workbench::state::{ModelsCatalogScope, ModelsPage, ProjectPage, Workspace};

use super::visible_workspace_width;

const IDENTITY_PANEL_HEIGHT: f32 = 176.0;
const IDENTITY_PANEL_COMPACT_HEIGHT: f32 = 210.0;
const FIRST_ROW_MIN_HEIGHT: f32 = 176.0;
const SECOND_ROW_MIN_HEIGHT: f32 = 214.0;
const SECTION_HEADER_HEIGHT: f32 = 24.0;
const STATUS_ROW_HEIGHT: f32 = 27.0;
const REGISTER_ROW_HEIGHT: f32 = 17.0;
const ACTIVITY_ROW_HEIGHT: f32 = 30.0;
const TWO_COLUMN_BREAKPOINT: f32 = 561.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tone {
    Neutral,
    Info,
    Ok,
    Warn,
    Error,
}

impl Tone {
    fn color(self, tokens: &Tokens) -> Color32 {
        match self {
            Self::Neutral => tokens.color.text_dim,
            Self::Info => tokens.color.info,
            Self::Ok => tokens.color.ok,
            Self::Warn => tokens.color.warn,
            Self::Error => tokens.color.err,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StatusSnapshot {
    area: &'static str,
    state: String,
    detail: String,
    action: &'static str,
    tone: Tone,
    intent: OverviewIntent,
    enabled: bool,
    disabled_reason: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum OverviewIntent {
    Command(Command),
    ProjectRootCommand(Command),
    OpenProjectRoot,
    OpenProjectModels,
    OpenRun(usize),
}

impl OverviewIntent {
    fn execute(self, app: &mut RSpiceApp) {
        match self {
            Self::Command(command) => command.execute(app),
            Self::ProjectRootCommand(command) => {
                open_project_root(app);
                if command.is_enabled(app) {
                    command.execute(app);
                }
            }
            Self::OpenProjectRoot => open_project_root(app),
            Self::OpenProjectModels => {
                app.state.workbench.models_view.catalog_scope = ModelsCatalogScope::Project;
                Command::ModelsPage(ModelsPage::Models).execute(app);
            }
            Self::OpenRun(index) => {
                let has_retained_dataset = app
                    .state
                    .simulation
                    .runs
                    .get(index)
                    .is_some_and(|run| !run.analyses.is_empty());
                if app.state.simulation.select_run(index) {
                    Command::OpenWorkspace(if has_retained_dataset {
                        Workspace::Results
                    } else {
                        Workspace::Simulate
                    })
                    .execute(app);
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConfigurationSnapshot {
    configured: bool,
    state: String,
    detail: String,
    root: String,
    dut: String,
    tone: Tone,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ModelSnapshot {
    state: String,
    detail: String,
    tone: Tone,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OpenDocumentSnapshot {
    reference: CellViewRef,
    view_type: ViewType,
    active: bool,
    dirty: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct OperationSnapshot {
    when: String,
    event: String,
    detail: String,
    tone: Tone,
    intent: Option<OverviewIntent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProblemSnapshot {
    code: String,
    message: String,
    path: String,
    tone: Tone,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DesignObjectSnapshot {
    key: String,
    name: String,
    kind: String,
    detail: String,
    tone: Tone,
    top: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SourceDeckSnapshot {
    name: String,
    ownership: &'static str,
    revision: u64,
    line_count: usize,
    dependency_count: usize,
    dependencies_sealed: bool,
    validation_state: String,
    validation_detail: String,
    validation_tone: Tone,
    problem: ProblemSnapshot,
}

#[derive(Debug, Clone, PartialEq)]
struct OverviewSnapshot {
    project_name: String,
    project_id: String,
    project_revision: u64,
    schema_version: u16,
    path: String,
    persistence_bound: bool,
    descriptor_root: String,
    simulation_root: String,
    technology: String,
    technology_attached: bool,
    dirty_documents: usize,
    library_count: usize,
    model_library_count: usize,
    cell_count: usize,
    view_count: usize,
    root_sheet_count: usize,
    root_instance_count: usize,
    root_net_count: usize,
    netlist_first: bool,
    source_deck: Option<SourceDeckSnapshot>,
    configuration: ConfigurationSnapshot,
    statuses: Vec<StatusSnapshot>,
    problem: ProblemSnapshot,
    open_documents: Vec<OpenDocumentSnapshot>,
    operations: Vec<OperationSnapshot>,
    run_plan_name: String,
    execution_target: String,
}

fn command_gate(app: &RSpiceApp, command: Command) -> (bool, Option<&'static str>) {
    match command.availability(app) {
        CommandAvailability::Available => (true, None),
        CommandAvailability::Disabled(reason) => (false, Some(reason)),
        CommandAvailability::Hidden => (false, Some("this action is unavailable in this context")),
    }
}

fn source_deck_snapshot(
    state: &crate::workbench::app_state::AppState,
) -> Option<SourceDeckSnapshot> {
    let document = state.workspace.netlist_document.as_ref()?;
    let name = document
        .provenance()
        .imported()
        .map(|imported| imported.origin().display_name().to_owned())
        .or_else(|| {
            document
                .save_acknowledgement()
                .map(|saved| saved.origin().display_name().to_owned())
        })
        .or_else(|| {
            state
                .workspace
                .netlist_source_path
                .as_ref()
                .and_then(|path| path.file_name())
                .map(|name| name.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| "Untitled SPICE deck".to_owned());
    let ownership = match document.ownership() {
        DocumentOwnership::Generated => "generated",
        DocumentOwnership::Imported => "imported",
        DocumentOwnership::Editable => "editable",
    };
    let line_count = document.source().lines().count();
    let dependency_count = document.dependencies().len();
    let dependencies_sealed = document.dependency_graph_is_sealed();
    let validation = document.validation();
    let validation_current =
        validation.is_some_and(|report| report.content_digest() == document.content_digest());

    let (validation_state, validation_detail, validation_tone, problem) = match validation {
        None => (
            "Not validated".to_owned(),
            "Validate the retained source before execution".to_owned(),
            Tone::Warn,
            ProblemSnapshot {
                code: "NOT RUN".to_owned(),
                message: "Validate the project source deck".to_owned(),
                path: name.clone(),
                tone: Tone::Warn,
            },
        ),
        Some(_) if !validation_current => (
            "Stale".to_owned(),
            "The source changed after the retained validation".to_owned(),
            Tone::Warn,
            ProblemSnapshot {
                code: "STALE".to_owned(),
                message: "Source changed after the retained validation".to_owned(),
                path: name.clone(),
                tone: Tone::Warn,
            },
        ),
        Some(report) => {
            let errors = report.error_count();
            let warnings = report.warning_count();
            let info = report.diagnostics().len().saturating_sub(errors + warnings);
            let tone = if errors > 0 {
                Tone::Error
            } else if warnings > 0 {
                Tone::Warn
            } else {
                Tone::Ok
            };
            let state_label = if errors > 0 {
                format!(
                    "{errors} blocking {}",
                    singular_or_plural(errors, "diagnostic", "diagnostics")
                )
            } else {
                "Validated".to_owned()
            };
            let detail = format!(
                "{warnings} {} · {info} information",
                singular_or_plural(warnings, "advisory", "advisories")
            );
            let highest = report
                .diagnostics()
                .iter()
                .max_by_key(|diagnostic| diagnostic.severity());
            let problem = highest.map_or_else(
                || ProblemSnapshot {
                    code: "SOURCE".to_owned(),
                    message: "No current source diagnostics".to_owned(),
                    path: name.clone(),
                    tone: Tone::Ok,
                },
                |diagnostic| {
                    let position = diagnostic.position();
                    ProblemSnapshot {
                        code: diagnostic.code().map_or_else(
                            || match diagnostic.severity() {
                                DiagnosticSeverity::Info => "SPICE-INFO".to_owned(),
                                DiagnosticSeverity::Warning => "SPICE-WARN".to_owned(),
                                DiagnosticSeverity::Error => "SPICE-ERROR".to_owned(),
                            },
                            str::to_owned,
                        ),
                        message: diagnostic.message().to_owned(),
                        path: format!("{}:{}:{}", name, position.line(), position.column()),
                        tone: match diagnostic.severity() {
                            DiagnosticSeverity::Info => Tone::Info,
                            DiagnosticSeverity::Warning => Tone::Warn,
                            DiagnosticSeverity::Error => Tone::Error,
                        },
                    }
                },
            );
            (state_label, detail, tone, problem)
        }
    };

    Some(SourceDeckSnapshot {
        name,
        ownership,
        revision: document.revision().get(),
        line_count,
        dependency_count,
        dependencies_sealed,
        validation_state,
        validation_detail,
        validation_tone,
        problem,
    })
}

impl OverviewSnapshot {
    fn capture(app: &RSpiceApp) -> Self {
        let state = &app.state;
        let project = &state.workspace.project;
        let dirty_documents = dirty_document_count(state);
        let netlist_first = state.is_netlist_first_without_schematic();
        let source_deck = source_deck_snapshot(state);
        let configuration = configuration_snapshot(app);
        let checks = checks_snapshot(app);
        let models = model_snapshot(app);
        let library_count = state.library_manager.library_count();
        let model_library_count = state.model_library_manager.library_count();
        let cell_count = state.library_manager.total_cell_count();
        let view_count = state.library_manager.total_view_count();
        let root_reference = CellViewRef::new(
            &project.root_library,
            &project.top_cell,
            crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
        );
        let root_schematic = if state.workspace.active_view == root_reference {
            Some(&state.schematic)
        } else {
            state.workspace.schematic_buffers.get(&root_reference.key())
        };
        let root_sheet_count = state
            .workspace
            .design_management
            .sheet_catalog(&root_reference.key())
            .map_or_else(
                || usize::from(root_schematic.is_some()),
                |catalog| catalog.sheets().len(),
            );
        let root_instance_count = root_schematic.map_or(0, |schematic| schematic.components.len());
        let root_net_count = root_schematic.map_or(0, |schematic| {
            crate::simulation::netlist_gen::design_nets(schematic).len()
        });

        let latest_run_status = state.simulation.runs.first().map_or_else(
            || {
                let command = Command::OpenWorkspace(Workspace::Simulate);
                let (enabled, disabled_reason) = command_gate(app, command);
                StatusSnapshot {
                    area: "Latest run",
                    state: "No retained run".to_owned(),
                    detail: "Run the active plan to create immutable result evidence".to_owned(),
                    action: "Open analyses",
                    tone: Tone::Neutral,
                    intent: OverviewIntent::Command(command),
                    enabled,
                    disabled_reason,
                }
            },
            |run| {
                let measurement_count: usize = run
                    .analyses
                    .iter()
                    .map(|analysis| analysis.measurements.len())
                    .sum();
                let has_dataset = !run.analyses.is_empty();
                let destination = if has_dataset {
                    Workspace::Results
                } else {
                    Workspace::Simulate
                };
                let (enabled, disabled_reason) =
                    command_gate(app, Command::OpenWorkspace(destination));
                StatusSnapshot {
                    area: "Latest run",
                    state: format!(
                        "Run {} · {}",
                        run.id,
                        lifecycle_label(run.lifecycle, run.success)
                    ),
                    detail: format!(
                        "{} {} · {measurement_count} {} · {}",
                        run.analyses.len(),
                        singular_or_plural(run.analyses.len(), "analysis", "analyses"),
                        singular_or_plural(measurement_count, "measurement", "measurements"),
                        run.execution_target.map_or_else(
                            || "execution target not retained".to_owned(),
                            |target| target.label().to_owned(),
                        )
                    ),
                    action: if has_dataset {
                        "Open results"
                    } else {
                        "Open analyses"
                    },
                    tone: lifecycle_tone(run.lifecycle, run.success),
                    intent: OverviewIntent::OpenRun(0),
                    enabled,
                    disabled_reason,
                }
            },
        );

        let (configuration_enabled, configuration_disabled_reason) =
            command_gate(app, Command::ConfigurationSets);
        let (models_enabled, models_disabled_reason) =
            command_gate(app, Command::ModelsPage(ModelsPage::Models));
        let primary_status = if netlist_first {
            let (enabled, disabled_reason) =
                command_gate(app, Command::OpenWorkspace(Workspace::Netlist));
            source_deck.as_ref().map_or_else(
                || StatusSnapshot {
                    area: "Source deck",
                    state: "Unavailable".to_owned(),
                    detail: "The project has no retained canonical source document".to_owned(),
                    action: "Open deck",
                    tone: Tone::Error,
                    intent: OverviewIntent::Command(Command::OpenWorkspace(Workspace::Netlist)),
                    enabled,
                    disabled_reason,
                },
                |deck| StatusSnapshot {
                    area: "Source deck",
                    state: deck.validation_state.clone(),
                    detail: deck.validation_detail.clone(),
                    action: "Open deck",
                    tone: deck.validation_tone,
                    intent: OverviewIntent::Command(Command::OpenWorkspace(Workspace::Netlist)),
                    enabled,
                    disabled_reason,
                },
            )
        } else {
            let enabled = state.project_lifecycle.project_open;
            StatusSnapshot {
                area: "Schematic checks",
                state: checks.state,
                detail: checks.detail,
                action: "Run checks",
                tone: checks.tone,
                intent: OverviewIntent::ProjectRootCommand(Command::RunChecks),
                enabled,
                disabled_reason: (!enabled).then_some("no project is open"),
            }
        };
        let statuses = vec![
            primary_status,
            StatusSnapshot {
                area: "Configuration",
                state: configuration.state.clone(),
                detail: configuration.detail.clone(),
                action: "Manage",
                tone: configuration.tone,
                intent: OverviewIntent::Command(Command::ConfigurationSets),
                enabled: configuration_enabled,
                disabled_reason: configuration_disabled_reason,
            },
            latest_run_status,
            StatusSnapshot {
                area: "Model closure",
                state: models.state,
                detail: models.detail,
                action: "Inspect",
                tone: models.tone,
                intent: OverviewIntent::OpenProjectModels,
                enabled: models_enabled,
                disabled_reason: models_disabled_reason,
            },
        ];
        let problem = if netlist_first {
            source_deck.as_ref().map_or_else(
                || ProblemSnapshot {
                    code: "SOURCE".to_owned(),
                    message: "The canonical source document is unavailable".to_owned(),
                    path: "Project source".to_owned(),
                    tone: Tone::Error,
                },
                |deck| deck.problem.clone(),
            )
        } else {
            problem_snapshot(app)
        };

        let open_documents = state
            .workspace
            .open_views
            .iter()
            .map(|document| OpenDocumentSnapshot {
                reference: document.reference.clone(),
                view_type: document.view_type,
                active: document.reference == state.workspace.active_view,
                dirty: document.dirty,
            })
            .collect();

        let now_ms =
            u64::try_from(crate::time_compat::unix_epoch().as_millis()).unwrap_or(u64::MAX);
        let mut operations = state
            .simulation
            .runs
            .iter()
            .enumerate()
            .take(8)
            .map(|(index, run)| {
                let measurement_count = run
                    .analyses
                    .iter()
                    .map(|analysis| analysis.measurements.len())
                    .sum();
                let timestamp_ms = unix_seconds_to_millis(run.timestamp);
                (
                    timestamp_ms.unwrap_or_default(),
                    OperationSnapshot {
                        when: timestamp_ms.map_or_else(
                            || "time n/a".to_owned(),
                            |value| relative_time(value, now_ms),
                        ),
                        event: format!(
                            "{} · {}",
                            run_title(run.id, &run.label),
                            lifecycle_label(run.lifecycle, run.success)
                        ),
                        detail: format!(
                            "{} {} · {measurement_count} {} · {} · {}",
                            run.analyses.len(),
                            singular_or_plural(run.analyses.len(), "analysis", "analyses"),
                            singular_or_plural(measurement_count, "measurement", "measurements"),
                            run.execution_target.map_or_else(
                                || "execution target not retained".to_owned(),
                                |target| target.label().to_owned(),
                            ),
                            format_duration(run.elapsed_time),
                        ),
                        tone: lifecycle_tone(run.lifecycle, run.success),
                        intent: Some(OverviewIntent::OpenRun(index)),
                    },
                )
            })
            .collect::<Vec<_>>();
        let mut retained_revision_ids = BTreeSet::new();
        for schematic in
            std::iter::once(&state.schematic).chain(state.workspace.schematic_buffers.values())
        {
            for revision in schematic.validated_revisions.records().iter().rev().take(8) {
                if !retained_revision_ids.insert(revision.id().as_uuid().to_string()) {
                    continue;
                }
                let findings = revision.finding_counts();
                let tone = if findings.blockers > 0 {
                    Tone::Error
                } else if findings.advisories > 0 {
                    Tone::Warn
                } else {
                    Tone::Ok
                };
                operations.push((
                    revision.created_unix_ms(),
                    OperationSnapshot {
                        when: relative_time(revision.created_unix_ms(), now_ms),
                        event: format!("{} validated", revision.view_identity()),
                        detail: format!(
                            "{} · project revision {} · {}",
                            revision.revision_note(),
                            revision.project_revision(),
                            revision.author()
                        ),
                        tone,
                        intent: Some(OverviewIntent::Command(Command::RevisionHistory)),
                    },
                ));
            }
        }
        operations.sort_by(|left, right| right.0.cmp(&left.0));
        let operations = operations
            .into_iter()
            .take(5)
            .map(|(_, operation)| operation)
            .collect();

        Self {
            project_name: project.display_name().to_owned(),
            project_id: project.id().to_string(),
            project_revision: project.revision().get(),
            schema_version: project.schema_version(),
            path: project_persistence_location(app),
            persistence_bound: state
                .project_lifecycle
                .accepted()
                .and_then(|accepted| accepted.binding.as_ref())
                .is_some(),
            descriptor_root: format!("{}/{}", project.root_library, project.top_cell),
            simulation_root: configuration.root.clone(),
            technology: project.technology_binding().map_or_else(
                || "Not attached".to_owned(),
                |binding| binding.display_label(),
            ),
            technology_attached: project.technology_binding().is_some(),
            dirty_documents,
            library_count,
            model_library_count,
            cell_count,
            view_count,
            root_sheet_count,
            root_instance_count,
            root_net_count,
            netlist_first,
            source_deck,
            configuration,
            statuses,
            problem,
            open_documents,
            operations,
            run_plan_name: state.sim_setup.active_plan_name().as_str().to_owned(),
            execution_target: crate::state::ExecutionTarget::current().label().to_owned(),
        }
    }
}

struct HealthCopy {
    state: String,
    detail: String,
    tone: Tone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DrcCounts {
    critical: usize,
    errors: usize,
    warnings: usize,
    info: usize,
}

/// Render the project overview from a single per-frame authoritative snapshot.
pub(super) fn overview(ui: &mut Ui, app: &mut RSpiceApp) {
    let snapshot = OverviewSnapshot::capture(app);
    let mut intent = None;
    let workspace_width = visible_workspace_width(ui);
    let paired_identity_width = overview_column_widths(workspace_width).0;
    let first_row_height =
        if workspace_width >= TWO_COLUMN_BREAKPOINT && paired_identity_width <= 500.0 {
            IDENTITY_PANEL_COMPACT_HEIGHT
        } else {
            FIRST_ROW_MIN_HEIGHT
        };
    let second_row_height = (ui.available_height() - first_row_height - 1.0).max(1.0);
    ui.scope(|ui| {
        // The reference surface is a two-by-two dashboard whose panels meet
        // on one-pixel rules. Narrow touch layouts retain the same reading
        // order as a single scrolling column.
        ui.spacing_mut().item_spacing.y = 0.0;
        if workspace_width < TWO_COLUMN_BREAKPOINT {
            egui::ScrollArea::vertical()
                .id_salt("workbench.project.overview.stacked")
                .auto_shrink([false, false])
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
                .show(ui, |ui| {
                    retain_first_intent(
                        &mut intent,
                        overview_single_panel(ui, FIRST_ROW_MIN_HEIGHT, |ui| {
                            overview_header(ui, app, &snapshot)
                        }),
                    );
                    stacked_divider(ui);
                    retain_first_intent(
                        &mut intent,
                        overview_single_panel(ui, FIRST_ROW_MIN_HEIGHT, |ui| {
                            engineering_status_register(ui, &snapshot.statuses, &snapshot.problem)
                        }),
                    );
                    stacked_divider(ui);
                    retain_first_intent(
                        &mut intent,
                        overview_single_panel(ui, SECOND_ROW_MIN_HEIGHT, |ui| {
                            project_and_document_registers(ui, app, &snapshot)
                        }),
                    );
                    stacked_divider(ui);
                    retain_first_intent(
                        &mut intent,
                        overview_single_panel(ui, SECOND_ROW_MIN_HEIGHT, |ui| {
                            run_register(ui, &snapshot)
                        }),
                    );
                });
        } else {
            retain_first_intent(
                &mut intent,
                overview_panel_pair(
                    ui,
                    "identity-status",
                    first_row_height,
                    |ui| overview_header(ui, app, &snapshot),
                    |ui| engineering_status_register(ui, &snapshot.statuses, &snapshot.problem),
                ),
            );
            let (divider_rect, _) =
                ui.allocate_exact_size(vec2(workspace_width, 1.0), Sense::hover());
            ui.painter()
                .rect_filled(divider_rect, 0.0, Tokens::get(ui.ctx()).color.border);
            retain_first_intent(
                &mut intent,
                overview_panel_pair(
                    ui,
                    "design-operations",
                    second_row_height,
                    |ui| project_and_document_registers(ui, app, &snapshot),
                    |ui| run_register(ui, &snapshot),
                ),
            );
        }
    });
    if let Some(intent) = intent {
        intent.execute(app);
    }
}

fn overview_column_widths(width: f32) -> (f32, f32) {
    let usable = (width - 1.0).max(2.0);
    let min_left = 300.0_f32.min(usable - 1.0);
    let max_left = (usable - 260.0).max(1.0);
    let left = (usable * 0.565).clamp(min_left.min(max_left), max_left.max(min_left));
    (left, (usable - left).max(1.0))
}

fn retain_first_intent(slot: &mut Option<OverviewIntent>, candidate: Option<OverviewIntent>) {
    if slot.is_none() {
        *slot = candidate;
    }
}

fn overview_single_panel(
    ui: &mut Ui,
    min_height: f32,
    content: impl FnOnce(&mut Ui) -> Option<OverviewIntent>,
) -> Option<OverviewIntent> {
    let tokens = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(tokens.color.bg_panel)
        .show(ui, |ui| {
            ui.set_width(visible_workspace_width(ui));
            ui.set_min_height(min_height);
            content(ui)
        })
        .inner
}

fn stacked_divider(ui: &mut Ui) {
    let width = visible_workspace_width(ui);
    let (rect, _) = ui.allocate_exact_size(vec2(width, 1.0), Sense::hover());
    ui.painter()
        .rect_filled(rect, 0.0, Tokens::get(ui.ctx()).color.border);
}

fn overview_panel_pair(
    ui: &mut Ui,
    id_salt: &'static str,
    min_height: f32,
    left: impl FnOnce(&mut Ui) -> Option<OverviewIntent>,
    right: impl FnOnce(&mut Ui) -> Option<OverviewIntent>,
) -> Option<OverviewIntent> {
    let tokens = Tokens::get(ui.ctx());
    let width = visible_workspace_width(ui);
    let divider = 1.0;
    let (left_width, right_width) = overview_column_widths(width);
    let mut left_intent = None;
    let mut right_intent = None;
    let shown = ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = divider;
        ui.allocate_ui_with_layout(
            vec2(left_width, min_height),
            Layout::top_down(Align::Min),
            |ui| {
                ui.set_width(left_width);
                ui.set_min_height(min_height);
                egui::Frame::new()
                    .fill(tokens.color.bg_panel)
                    .show(ui, |ui| {
                        ui.set_width(left_width);
                        egui::ScrollArea::vertical()
                            .id_salt(("workbench.project.overview", id_salt, "left"))
                            .max_height(min_height)
                            .auto_shrink([false, false])
                            .scroll_bar_visibility(
                                egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded,
                            )
                            .show(ui, |ui| {
                                ui.set_min_width(left_width);
                                left_intent = left(ui);
                            });
                    });
            },
        );
        ui.allocate_ui_with_layout(
            vec2(right_width, min_height),
            Layout::top_down(Align::Min),
            |ui| {
                ui.set_width(right_width);
                ui.set_min_height(min_height);
                egui::Frame::new()
                    .fill(tokens.color.bg_panel)
                    .show(ui, |ui| {
                        ui.set_width(right_width);
                        egui::ScrollArea::vertical()
                            .id_salt(("workbench.project.overview", id_salt, "right"))
                            .max_height(min_height)
                            .auto_shrink([false, false])
                            .scroll_bar_visibility(
                                egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded,
                            )
                            .show(ui, |ui| {
                                ui.set_min_width(right_width);
                                right_intent = right(ui);
                            });
                    });
            },
        );
    });
    ui.painter().vline(
        shown.response.rect.left() + left_width,
        shown.response.rect.y_range(),
        Stroke::new(1.0, tokens.color.border),
    );
    left_intent.or(right_intent)
}

fn overview_header(
    ui: &mut Ui,
    app: &RSpiceApp,
    snapshot: &OverviewSnapshot,
) -> Option<OverviewIntent> {
    let tokens = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let compact = width <= 500.0;
    let narrow_actions = width <= 440.0;
    let panel_height = if compact {
        IDENTITY_PANEL_COMPACT_HEIGHT
    } else {
        IDENTITY_PANEL_HEIGHT
    };
    let (rect, response) = ui.allocate_exact_size(vec2(width, panel_height), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, tokens.color.bg_panel);

    let content = rect.shrink2(vec2(14.0, 0.0));
    let text_rect = Rect::from_min_max(content.min, pos2(content.right(), rect.top() + 54.0));
    let action_rect = Rect::from_min_max(
        pos2(content.left(), rect.top() + 58.0),
        pos2(
            content.right(),
            rect.top() + if compact { 140.0 } else { 102.0 },
        ),
    );

    let project_tone = if snapshot
        .statuses
        .iter()
        .any(|status| status.tone == Tone::Error)
    {
        Tone::Error
    } else if snapshot
        .statuses
        .iter()
        .any(|status| status.tone == Tone::Warn)
    {
        Tone::Warn
    } else {
        Tone::Ok
    };
    let advisory_count = snapshot
        .statuses
        .iter()
        .filter(|status| matches!(status.tone, Tone::Warn | Tone::Error))
        .count();
    let project_status = match project_tone {
        Tone::Error => format!(
            "blocked · {} {}",
            advisory_count,
            singular_or_plural(advisory_count, "issue", "issues")
        ),
        Tone::Warn => format!(
            "ready · {} {}",
            advisory_count,
            singular_or_plural(advisory_count, "advisory", "advisories")
        ),
        _ => "current".to_owned(),
    };
    let status_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let status_width = ui
        .painter()
        .layout_no_wrap(project_status.clone(), status_font.clone(), Color32::WHITE)
        .size()
        .x
        .min(content.width() * 0.42);
    let title_font = theme::sans(tokens::FS_5, FontWeight::SemiBold);
    let title = elide_text(
        ui,
        &snapshot.project_name,
        &title_font,
        (text_rect.width() - status_width - 10.0).max(1.0),
    );
    let title_width = ui
        .painter()
        .layout_no_wrap(title.clone(), title_font.clone(), Color32::WHITE)
        .size()
        .x;
    ui.painter().text(
        pos2(text_rect.left(), rect.top() + 18.0),
        Align2::LEFT_CENTER,
        title,
        title_font,
        tokens.color.text,
    );
    ui.painter().text(
        pos2(
            (text_rect.left() + title_width + 10.0)
                .min((text_rect.right() - status_width).max(text_rect.left())),
            rect.top() + 18.0,
        ),
        Align2::LEFT_CENTER,
        &project_status,
        status_font,
        project_tone.color(&tokens),
    );
    let state_label = if !snapshot.persistence_bound {
        "no saved baseline".to_owned()
    } else if snapshot.dirty_documents == 0 {
        "working tree clean".to_owned()
    } else {
        format!(
            "{} working {}",
            snapshot.dirty_documents,
            singular_or_plural(snapshot.dirty_documents, "change", "changes")
        )
    };
    paint_elided(
        ui,
        pos2(text_rect.left(), rect.top() + 36.0),
        &format!(
            "main @ r{} · {} · {}",
            snapshot.project_revision, state_label, snapshot.path
        ),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        tokens.color.text_faint,
        text_rect.width(),
    );

    let mut intent = None;
    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(action_rect)
            .layout(Layout::top_down(Align::Min)),
        |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                let primary = if snapshot.netlist_first {
                    Button::new("Open SPICE deck").icon(Icon::File)
                } else {
                    Button::new("Open top schematic").icon(Icon::Schematic)
                };
                if primary.accent().show(ui).clicked() {
                    intent = Some(if snapshot.netlist_first {
                        OverviewIntent::Command(Command::OpenWorkspace(Workspace::Netlist))
                    } else {
                        OverviewIntent::OpenProjectRoot
                    });
                }
                if !snapshot.netlist_first {
                    let run_label = if narrow_actions {
                        "Run plan\u{2026}".to_owned()
                    } else {
                        format!("Run {}…", snapshot.run_plan_name)
                    };
                    let command = Command::OpenWorkspace(Workspace::Simulate);
                    let (enabled, disabled_reason) = command_gate(app, command);
                    let run_plan = ui
                        .add_enabled_ui(enabled, |ui| Button::new(&run_label).show(ui))
                        .inner;
                    let run_plan = disabled_reason.map_or(run_plan.clone(), |reason| {
                        run_plan.on_disabled_hover_text(reason)
                    });
                    if run_plan.clicked() {
                        intent = Some(run_plan_intent());
                    }
                }
                let (new_cell_command_enabled, new_cell_command_reason) =
                    command_gate(app, Command::NewCell);
                let has_writable_library = app
                    .state
                    .library_manager
                    .libraries_sorted()
                    .iter()
                    .any(|library| !library.read_only);
                let new_cell_allowed = new_cell_command_enabled
                    && !app.state.workbench.safe_mode.project_read_only()
                    && has_writable_library;
                let new_cell = ui
                    .add_enabled_ui(new_cell_allowed, |ui| Button::new("New cell…").show(ui))
                    .inner
                    .on_disabled_hover_text(if let Some(reason) = new_cell_command_reason {
                        reason
                    } else if app.state.workbench.safe_mode.project_read_only() {
                        "Safe mode opened this project read-only."
                    } else if !has_writable_library {
                        "Attach or create a writable design library first."
                    } else {
                        "New cell creation is unavailable."
                    });
                if new_cell.clicked() {
                    intent = Some(OverviewIntent::Command(Command::NewCell));
                }
                let open_deck_command = Command::OpenNetlist;
                let (open_deck_enabled, open_deck_disabled_reason) =
                    command_gate(app, open_deck_command);
                let import_deck = ui
                    .add_enabled_ui(open_deck_enabled, |ui| Button::new("Import deck…").show(ui))
                    .inner;
                let import_deck = open_deck_disabled_reason.map_or(import_deck.clone(), |reason| {
                    import_deck.on_disabled_hover_text(reason)
                });
                if import_deck.clicked() {
                    intent = Some(import_deck_intent());
                }
                let history_command = Command::RevisionHistory;
                let (history_enabled, history_disabled_reason) = command_gate(app, history_command);
                let history = ui
                    .add_enabled_ui(history_enabled, |ui| Button::new("History…").show(ui))
                    .inner;
                let history = history_disabled_reason.map_or(history.clone(), |reason| {
                    history.on_disabled_hover_text(reason)
                });
                if history.clicked() {
                    intent = Some(OverviewIntent::Command(Command::RevisionHistory));
                }
            });
        },
    );
    let context_top = rect.top() + if compact { 149.0 } else { 114.0 };
    let context_row_height = 25.0;
    let column_gap = 18.0;
    let column_width = ((content.width() - column_gap) * 0.5).max(1.0);
    let left = Rect::from_min_size(
        pos2(content.left(), context_top),
        vec2(column_width, context_row_height * 2.0),
    );
    let right = Rect::from_min_size(
        pos2(content.left() + column_width + column_gap, context_top),
        vec2(column_width, context_row_height * 2.0),
    );
    ui.painter().hline(
        content.x_range(),
        context_top,
        Stroke::new(1.0, tokens.color.border),
    );
    let (left_top_label, left_top_value, left_bottom_label, left_bottom_value) =
        if snapshot.netlist_first {
            let deck = snapshot.source_deck.as_ref();
            (
                "Source deck",
                deck.map_or("Unavailable", |deck| deck.name.as_str())
                    .to_owned(),
                "Validation",
                deck.map_or("Unavailable", |deck| deck.validation_state.as_str())
                    .to_owned(),
            )
        } else {
            let configuration_status = match snapshot.configuration.tone {
                Tone::Error => "blocked",
                Tone::Warn => "review required",
                _ => "resolved",
            };
            (
                "Top / DUT",
                format!(
                    "{} \u{00b7} {}",
                    snapshot.descriptor_root, snapshot.configuration.dut
                ),
                "Configuration",
                format!("{} · {configuration_status}", snapshot.configuration.state),
            )
        };
    let (right_top_label, right_top_value) = if snapshot.netlist_first {
        (
            "Ownership",
            snapshot
                .source_deck
                .as_ref()
                .map_or("Unavailable", |deck| deck.ownership)
                .to_owned(),
        )
    } else {
        ("Testbench", snapshot.configuration.root.clone())
    };
    identity_context_row(
        ui,
        Rect::from_min_size(left.min, vec2(left.width(), context_row_height)),
        left_top_label,
        &left_top_value,
        true,
    );
    identity_context_row(
        ui,
        Rect::from_min_size(
            pos2(left.left(), left.top() + context_row_height),
            vec2(left.width(), context_row_height),
        ),
        left_bottom_label,
        &left_bottom_value,
        false,
    );
    identity_context_row(
        ui,
        Rect::from_min_size(right.min, vec2(right.width(), context_row_height)),
        right_top_label,
        &right_top_value,
        !snapshot.netlist_first,
    );
    identity_context_row(
        ui,
        Rect::from_min_size(
            pos2(right.left(), right.top() + context_row_height),
            vec2(right.width(), context_row_height),
        ),
        "Execution",
        &snapshot.execution_target,
        false,
    );
    response.on_hover_text(format!(
        "Project {} · logical revision {} · schema {}",
        snapshot.project_id, snapshot.project_revision, snapshot.schema_version
    ));
    intent
}

fn run_plan_intent() -> OverviewIntent {
    OverviewIntent::Command(Command::OpenWorkspace(Workspace::Simulate))
}

fn import_deck_intent() -> OverviewIntent {
    OverviewIntent::Command(Command::OpenNetlist)
}

fn identity_context_row(ui: &Ui, rect: Rect, label: &str, value: &str, mono: bool) {
    let tokens = Tokens::get(ui.ctx());
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, tokens.color.border),
    );
    const LABEL_WIDTH: f32 = 82.0;
    const LABEL_VALUE_GAP: f32 = 8.0;
    let label_width = LABEL_WIDTH.min(rect.width() * 0.38);
    ui.painter().text(
        rect.left_center(),
        Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        tokens.color.text_faint,
    );
    let value_font = if mono {
        theme::mono(tokens::FS_0, FontWeight::Regular)
    } else {
        theme::sans(tokens::FS_0, FontWeight::Regular)
    };
    paint_elided(
        ui,
        pos2(
            rect.left() + label_width + LABEL_VALUE_GAP,
            rect.center().y - tokens::FS_0 * 0.5,
        ),
        value,
        value_font,
        tokens.color.text,
        (rect.width() - label_width - LABEL_VALUE_GAP).max(0.0),
    );
}

fn open_project_root(app: &mut RSpiceApp) {
    let reference = CellViewRef::new(
        &app.state.workspace.project.root_library,
        &app.state.workspace.project.top_cell,
        crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
    );
    app.state.open_workspace_view(reference);
    Command::OpenWorkspace(Workspace::Design).execute(app);
}

fn project_persistence_location(app: &RSpiceApp) -> String {
    let fallback = || {
        app.state.workspace.project.path.as_ref().map_or_else(
            || "Not yet saved".to_owned(),
            |path| path.display().to_string(),
        )
    };
    let Some(binding) = app
        .state
        .project_lifecycle
        .accepted()
        .and_then(|accepted| accepted.binding.as_ref())
    else {
        return fallback();
    };

    #[cfg(not(target_arch = "wasm32"))]
    {
        binding
            .canonical_path()
            .map_or_else(fallback, |path| path.display().to_string())
    }
    #[cfg(target_arch = "wasm32")]
    {
        match binding {
            crate::workbench::lifecycle::project_lifecycle::PersistenceBinding::Browser {
                display_name,
                accepted_generation,
                persisted_generation,
                ..
            } => {
                if *persisted_generation == Some(*accepted_generation) {
                    display_name.clone()
                } else {
                    format!("{display_name} · session-only browser binding")
                }
            }
        }
    }
}

fn engineering_status_register(
    ui: &mut Ui,
    statuses: &[StatusSnapshot],
    problem: &ProblemSnapshot,
) -> Option<OverviewIntent> {
    let tokens = Tokens::get(ui.ctx());
    let status_tone = if statuses.iter().any(|status| status.tone == Tone::Error) {
        Tone::Error
    } else if statuses.iter().any(|status| status.tone == Tone::Warn) {
        Tone::Warn
    } else {
        Tone::Ok
    };
    section_header(
        ui,
        "Project status",
        if status_tone == Tone::Ok {
            "current"
        } else {
            "review required"
        },
        status_tone,
    );
    ui.add_space(4.0);
    let width = ui.available_width().max(1.0);
    let mut requested = None;
    for (index, status) in statuses.iter().enumerate() {
        // The reference row is its own grid — `auto minmax(96px, auto)
        // minmax(0, 1fr)` with 9px gaps — so the detail starts just after
        // this row's name, never at a shared fraction of the panel width.
        let name_font = theme::sans(tokens::FS_0, FontWeight::Medium);
        let name_column = ui
            .painter()
            .layout_no_wrap(status.area.to_owned(), name_font.clone(), Color32::WHITE)
            .size()
            .x
            .max(96.0)
            .min(width * 0.5);
        let detail_offset = (31.0 + name_column + 9.0).min((width - 64.0).max(1.0));
        let (rect, response) = ui.allocate_exact_size(
            vec2(width, STATUS_ROW_HEIGHT),
            if status.enabled {
                Sense::click()
            } else {
                Sense::hover()
            },
        );
        if status.enabled && (response.hovered() || response.has_focus()) {
            ui.painter()
                .rect_filled(rect.shrink2(vec2(8.0, 2.0)), 4.0, tokens.color.bg_panel_2);
        }
        let primary_color = if status.enabled {
            tokens.color.text
        } else {
            tokens.color.text_faint
        };
        let detail_color = if status.enabled {
            tokens.color.text_dim
        } else {
            tokens.color.text_faint
        };
        ui.painter().circle_filled(
            pos2(rect.left() + 18.0, rect.center().y),
            4.0,
            status.tone.color(&tokens),
        );
        paint_elided(
            ui,
            pos2(rect.left() + 31.0, rect.center().y - tokens::FS_0 * 0.5),
            status.area,
            name_font,
            primary_color,
            (detail_offset - 40.0).max(1.0),
        );
        paint_elided(
            ui,
            pos2(
                rect.left() + detail_offset,
                rect.center().y - tokens::FS_0 * 0.5,
            ),
            &format!("{} \u{00b7} {}", status.state, status.detail),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            detail_color,
            (width - detail_offset - 8.0).max(1.0),
        );
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Button,
                status.enabled,
                format!(
                    "{}: {}. {}. {}",
                    status.area, status.state, status.detail, status.action
                ),
            )
        });
        theme::paint_focus_ring_outset(ui, &response, rect);
        let response = if status.enabled {
            response
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .on_hover_text(format!("{}\n{}", status.detail, status.action))
        } else {
            response.on_hover_text(status.disabled_reason.unwrap_or("Action unavailable"))
        };
        if status.enabled && response.clicked() {
            requested = Some(status.intent.clone());
        }
        if index + 1 < statuses.len() {
            ui.add_space(3.0);
        }
    }
    ui.add_space(4.0);
    let (rect, _) = ui.allocate_exact_size(vec2(width, 20.0), Sense::hover());
    ui.painter().hline(
        (rect.left() + 14.0)..=(rect.right() - 14.0),
        rect.top(),
        Stroke::new(1.0, tokens.color.border),
    );
    // The strip's cells are `auto minmax(0, 1fr) auto` with 9px gaps inside
    // the panel's 14px insets: the code takes its own width, the message the
    // rest, and the path right-aligns against the far inset.
    let code_font = theme::mono(tokens::FS_MICRO, FontWeight::Medium);
    let code_width = ui
        .painter()
        .layout_no_wrap(problem.code.clone(), code_font.clone(), Color32::WHITE)
        .size()
        .x
        .min(width * 0.22);
    let path_width = 128.0_f32.min(width * 0.34);
    paint_elided(
        ui,
        pos2(rect.left() + 14.0, rect.center().y - tokens::FS_MICRO * 0.5),
        &problem.code,
        code_font,
        problem.tone.color(&tokens),
        code_width.max(1.0),
    );
    paint_elided(
        ui,
        pos2(
            rect.left() + 14.0 + code_width + 9.0,
            rect.center().y - tokens::FS_MICRO * 0.5,
        ),
        &problem.message,
        theme::sans(tokens::FS_MICRO, FontWeight::Regular),
        tokens.color.text_dim,
        (width - 14.0 - code_width - 9.0 - path_width - 9.0 - 14.0).max(1.0),
    );
    let path_font = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    let path = elide_text(ui, &problem.path, &path_font, path_width.max(1.0));
    ui.painter().text(
        pos2(rect.right() - 14.0, rect.center().y),
        Align2::RIGHT_CENTER,
        path,
        path_font,
        tokens.color.text_faint,
    );
    requested
}

fn project_and_document_registers(
    ui: &mut Ui,
    app: &RSpiceApp,
    snapshot: &OverviewSnapshot,
) -> Option<OverviewIntent> {
    project_register(ui, app, snapshot)
}

fn project_register(
    ui: &mut Ui,
    app: &RSpiceApp,
    snapshot: &OverviewSnapshot,
) -> Option<OverviewIntent> {
    let objects = design_objects(snapshot);
    section_header(
        ui,
        "Design",
        &if snapshot.netlist_first {
            format!(
                "{} key {} · netlist-first",
                objects.len(),
                singular_or_plural(objects.len(), "source", "sources")
            )
        } else {
            format!(
                "{} key {} · {} {}",
                objects.len(),
                singular_or_plural(objects.len(), "object", "objects"),
                snapshot.cell_count,
                singular_or_plural(snapshot.cell_count, "cell", "cells")
            )
        },
        Tone::Neutral,
    );
    ui.add_space(6.0);
    let mut requested = None;
    for (index, object) in objects.iter().enumerate() {
        design_summary_row(
            ui,
            &object.name,
            &object.kind,
            &object.detail,
            object.tone,
            if object.top { 0.0 } else { 16.0 },
            false,
        );
        if index + 1 < objects.len() {
            ui.add_space(5.0);
        }
    }
    ui.add_space(5.0);
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin {
            left: 14,
            right: 14,
            top: 5,
            bottom: 0,
        })
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let library_command = Command::ProjectPage(ProjectPage::Library);
                let (library_enabled, library_disabled_reason) = command_gate(app, library_command);
                let library = ui
                    .add_enabled_ui(library_enabled, |ui| {
                        Button::new("Browse library…").show(ui)
                    })
                    .inner;
                let library = library_disabled_reason.map_or(library.clone(), |reason| {
                    library.on_disabled_hover_text(reason)
                });
                if library.clicked() {
                    requested = Some(OverviewIntent::Command(library_command));
                }
                let configuration_command = Command::ConfigurationSets;
                let (configuration_enabled, configuration_disabled_reason) =
                    command_gate(app, configuration_command);
                let configuration = ui
                    .add_enabled_ui(configuration_enabled, |ui| {
                        Button::new("Configuration sets…").show(ui)
                    })
                    .inner;
                let configuration = configuration_disabled_reason
                    .map_or(configuration.clone(), |reason| {
                        configuration.on_disabled_hover_text(reason)
                    });
                if configuration.clicked() {
                    requested = Some(OverviewIntent::Command(configuration_command));
                }
            });
        });
    requested
}

fn design_objects(snapshot: &OverviewSnapshot) -> Vec<DesignObjectSnapshot> {
    fn design_key(identity: &str) -> String {
        let identity = identity.trim().replace('\\', "/");
        for suffix in ["/schematic", "/testbench", "/config", "/symbol"] {
            if let Some(stripped) = identity.strip_suffix(suffix) {
                return stripped.to_ascii_lowercase();
            }
        }
        identity.to_ascii_lowercase()
    }

    fn push_unique(
        rows: &mut Vec<DesignObjectSnapshot>,
        keys: &mut BTreeSet<String>,
        row: DesignObjectSnapshot,
    ) {
        if keys.insert(row.key.to_ascii_lowercase()) {
            rows.push(row);
        }
    }

    let mut rows = Vec::with_capacity(5);
    let mut keys = BTreeSet::new();
    if snapshot.netlist_first {
        if let Some(deck) = &snapshot.source_deck {
            push_unique(
                &mut rows,
                &mut keys,
                DesignObjectSnapshot {
                    key: format!("source-deck:{}", deck.name.to_ascii_lowercase()),
                    name: deck.name.clone(),
                    kind: format!("SPICE deck · {}", deck.ownership),
                    detail: format!(
                        "rev {} · {} {} · {} {}{}",
                        deck.revision,
                        deck.line_count,
                        singular_or_plural(deck.line_count, "line", "lines"),
                        deck.dependency_count,
                        singular_or_plural(deck.dependency_count, "dependency", "dependencies"),
                        if deck.dependencies_sealed {
                            " · sealed"
                        } else {
                            " · unresolved"
                        }
                    ),
                    tone: deck.validation_tone,
                    top: true,
                },
            );
        }
    } else {
        push_unique(
            &mut rows,
            &mut keys,
            DesignObjectSnapshot {
                key: design_key(&snapshot.descriptor_root),
                name: snapshot.descriptor_root.clone(),
                kind: format!(
                    "schematic · {} {}",
                    snapshot.root_sheet_count,
                    singular_or_plural(snapshot.root_sheet_count, "sheet", "sheets")
                ),
                detail: format!(
                    "{} {} · {} {}",
                    snapshot.root_instance_count,
                    singular_or_plural(snapshot.root_instance_count, "instance", "instances"),
                    snapshot.root_net_count,
                    singular_or_plural(snapshot.root_net_count, "net", "nets")
                ),
                tone: Tone::Info,
                top: true,
            },
        );
        if snapshot.configuration.configured {
            push_unique(
                &mut rows,
                &mut keys,
                DesignObjectSnapshot {
                    key: design_key(&snapshot.simulation_root),
                    name: snapshot.simulation_root.clone(),
                    kind: "schematic + config".to_owned(),
                    detail: format!("active testbench · {}", snapshot.configuration.state),
                    tone: snapshot.configuration.tone,
                    top: false,
                },
            );
            push_unique(
                &mut rows,
                &mut keys,
                DesignObjectSnapshot {
                    key: design_key(&snapshot.configuration.dut),
                    name: snapshot.configuration.dut.clone(),
                    kind: "design under test".to_owned(),
                    detail: snapshot.configuration.detail.clone(),
                    tone: snapshot.configuration.tone,
                    top: false,
                },
            );
        }
    }
    for document in snapshot.open_documents.iter().filter(|document| {
        matches!(
            document.view_type,
            ViewType::Verilog | ViewType::VerilogA | ViewType::Spice
        )
    }) {
        push_unique(
            &mut rows,
            &mut keys,
            DesignObjectSnapshot {
                key: design_key(&document.reference.display_path()),
                name: document.reference.display_path(),
                kind: document.view_type.display_name().to_owned(),
                detail: document_state(document).to_owned(),
                tone: if document.dirty {
                    Tone::Warn
                } else {
                    Tone::Neutral
                },
                top: false,
            },
        );
        if rows.len() >= 4 {
            break;
        }
    }
    if snapshot.technology_attached {
        push_unique(
            &mut rows,
            &mut keys,
            DesignObjectSnapshot {
                key: format!("technology:{}", snapshot.technology),
                name: snapshot.technology.clone(),
                kind: "technology + model set".to_owned(),
                detail: format!(
                    "{} model {}",
                    snapshot.model_library_count,
                    singular_or_plural(snapshot.model_library_count, "library", "libraries")
                ),
                tone: Tone::Ok,
                top: false,
            },
        );
    }
    if rows.len() < 5 {
        push_unique(
            &mut rows,
            &mut keys,
            DesignObjectSnapshot {
                key: format!("design-library:{}", snapshot.descriptor_root),
                name: "Project design libraries".to_owned(),
                kind: "libraries · cells · views".to_owned(),
                detail: format!(
                    "{} {} · {} {} · {} {}",
                    snapshot.library_count,
                    singular_or_plural(snapshot.library_count, "library", "libraries"),
                    snapshot.cell_count,
                    singular_or_plural(snapshot.cell_count, "cell", "cells"),
                    snapshot.view_count,
                    singular_or_plural(snapshot.view_count, "view", "views")
                ),
                tone: Tone::Neutral,
                top: false,
            },
        );
    }
    for document in &snapshot.open_documents {
        if rows.len() >= 5 {
            break;
        }
        push_unique(
            &mut rows,
            &mut keys,
            DesignObjectSnapshot {
                key: design_key(&document.reference.display_path()),
                name: document.reference.display_path(),
                kind: document.view_type.display_name().to_owned(),
                detail: document_state(document).to_owned(),
                tone: if document.dirty {
                    Tone::Warn
                } else if document.active {
                    Tone::Info
                } else {
                    Tone::Neutral
                },
                top: false,
            },
        );
    }
    rows.truncate(5);
    rows
}

fn design_summary_row(
    ui: &mut Ui,
    name: &str,
    kind: &str,
    detail: &str,
    tone: Tone,
    indent: f32,
    interactive: bool,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, response) = ui.allocate_exact_size(
        vec2(width, REGISTER_ROW_HEIGHT),
        if interactive {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    if interactive && (response.hovered() || response.has_focus()) {
        ui.painter()
            .rect_filled(rect.shrink2(vec2(8.0, 1.0)), 3.0, t.color.bg_hover);
    }
    let name_width = (width * 0.30).max(110.0);
    let kind_width = (width * 0.28).max(96.0);
    paint_elided(
        ui,
        pos2(
            rect.left() + 14.0 + indent,
            rect.center().y - tokens::FS_0 * 0.5,
        ),
        name,
        theme::mono(
            tokens::FS_0,
            if indent == 0.0 {
                FontWeight::SemiBold
            } else {
                FontWeight::Regular
            },
        ),
        t.color.text,
        (name_width - 18.0 - indent).max(1.0),
    );
    // The reference kind chip is an outline only — micro type, 3px radius,
    // no fill — so it reads as an annotation, not a control.
    let kind_font = theme::sans(tokens::FS_MICRO, FontWeight::Regular);
    let kind_text = elide_text(ui, kind, &kind_font, (kind_width - 18.0).max(1.0));
    let kind_text_width = ui
        .painter()
        .layout_no_wrap(kind_text.clone(), kind_font.clone(), Color32::WHITE)
        .size()
        .x;
    let kind_rect = Rect::from_min_size(
        pos2(
            rect.left() + name_width,
            rect.center().y - (tokens::FS_MICRO + 5.0) * 0.5,
        ),
        vec2(
            (kind_text_width + 10.0).min(kind_width - 4.0),
            tokens::FS_MICRO + 5.0,
        ),
    );
    ui.painter().rect_stroke(
        kind_rect,
        3.0,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        kind_rect.center(),
        Align2::CENTER_CENTER,
        kind_text,
        kind_font,
        t.color.text_dim,
    );
    // Detail is faint annotation text in the reference; only a genuine
    // warning or error keeps its tone so real problems stay visible.
    let detail_color = match tone {
        Tone::Warn | Tone::Error => tone.color(&t),
        _ => t.color.text_faint,
    };
    paint_elided(
        ui,
        pos2(
            rect.left() + name_width + kind_width,
            rect.center().y - tokens::FS_MICRO * 0.5,
        ),
        detail,
        theme::sans(tokens::FS_MICRO, FontWeight::Regular),
        detail_color,
        (width - name_width - kind_width - 10.0).max(1.0),
    );
    theme::paint_focus_ring_outset(ui, &response, rect);
    // The row paints its three columns straight to the painter, so the label
    // has to be assembled here or a screen reader announces nothing.
    let row_label = format!("{name}, {kind}, {detail}");
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            if interactive {
                egui::WidgetType::Button
            } else {
                egui::WidgetType::Label
            },
            ui.is_enabled(),
            row_label.clone(),
        )
    });
    response.on_hover_text(format!("{name}\n{kind}\n{detail}"))
}

fn run_register(ui: &mut Ui, snapshot: &OverviewSnapshot) -> Option<OverviewIntent> {
    section_header(
        ui,
        "Recent operations",
        &format!("{} retained", snapshot.operations.len()),
        Tone::Neutral,
    );
    ui.add_space(6.0);
    let mut selected = None;
    if snapshot.operations.is_empty() {
        activity_row(
            ui,
            "now",
            "No retained project operations",
            "Validated revisions and simulation runs will appear here",
            Tone::Neutral,
            false,
        );
        ui.add_space(7.0);
        activity_row(
            ui,
            "rev",
            &format!("Working revision {}", snapshot.project_revision),
            if snapshot.dirty_documents == 0 {
                "No modified open documents"
            } else {
                "Working changes are not yet saved"
            },
            if snapshot.dirty_documents == 0 {
                Tone::Ok
            } else {
                Tone::Warn
            },
            false,
        );
    } else {
        for (index, operation) in snapshot.operations.iter().enumerate() {
            let response = activity_row(
                ui,
                &operation.when,
                &operation.event,
                &operation.detail,
                operation.tone,
                operation.intent.is_some(),
            );
            if response.clicked() {
                selected.clone_from(&operation.intent);
            }
            if index + 1 < snapshot.operations.len() {
                ui.add_space(7.0);
            }
        }
    }
    selected
}

fn activity_row(
    ui: &mut Ui,
    time: &str,
    event: &str,
    detail: &str,
    _tone: Tone,
    interactive: bool,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, response) = ui.allocate_exact_size(
        vec2(width, ACTIVITY_ROW_HEIGHT),
        if interactive {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    if interactive && (response.hovered() || response.has_focus()) {
        ui.painter()
            .rect_filled(rect.shrink2(vec2(8.0, 1.0)), 4.0, t.color.bg_panel_2);
    }
    let time_width = 54.0_f32.min(width * 0.22);
    paint_elided(
        ui,
        pos2(rect.left() + 14.0, rect.top() + 3.0),
        time,
        theme::mono(tokens::FS_MICRO, FontWeight::Regular),
        t.color.text_faint,
        (time_width - 14.0).max(1.0),
    );
    paint_elided(
        ui,
        pos2(rect.left() + time_width + 14.0, rect.top() + 2.0),
        event,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text,
        (width - time_width - 24.0).max(1.0),
    );
    paint_elided(
        ui,
        pos2(rect.left() + time_width + 14.0, rect.top() + 16.0),
        detail,
        theme::mono(tokens::FS_MICRO, FontWeight::Regular),
        t.color.text_faint,
        (width - time_width - 24.0).max(1.0),
    );
    theme::paint_focus_ring_outset(ui, &response, rect);
    // Painted columns again, so the announced label is assembled here.
    let row_label = format!("{time}, {event}, {detail}");
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            if interactive {
                egui::WidgetType::Button
            } else {
                egui::WidgetType::Label
            },
            ui.is_enabled(),
            row_label.clone(),
        )
    });
    response.on_hover_text(format!("{event}\n{detail}"))
}

fn section_header(ui: &mut Ui, title: &str, meta: &str, tone: Tone) {
    let tokens = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, response) =
        ui.allocate_exact_size(vec2(width, SECTION_HEADER_HEIGHT), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, tokens.color.bg_panel_2);
    let content = rect.shrink2(vec2(10.0, 0.0));
    let meta_width = (content.width() * 0.46).max(0.0);
    paint_elided(
        ui,
        content.left_center() - vec2(0.0, tokens::FS_0 * 0.5),
        &title.to_uppercase(),
        theme::sans(tokens::FS_2, FontWeight::SemiBold),
        tokens.color.text_dim,
        (content.width() - meta_width - 8.0).max(0.0),
    );
    let meta_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let meta = elide_text(ui, &meta.to_uppercase(), &meta_font, meta_width);
    ui.painter().text(
        content.right_center(),
        Align2::RIGHT_CENTER,
        &meta,
        meta_font,
        tone.color(&tokens),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            format!("{title}: {meta}"),
        )
    });
}

fn checks_snapshot(app: &RSpiceApp) -> HealthCopy {
    match app.state.project_root_design_check_status() {
        DesignCheckStatus::Current(receipt) => {
            let summary = receipt.result.summary();
            drc_copy(
                true,
                Some(DrcCounts {
                    critical: summary.critical,
                    errors: summary.errors,
                    warnings: summary.warnings,
                    info: summary.info,
                }),
            )
        }
        DesignCheckStatus::NotRun => HealthCopy {
            state: "Not run".to_owned(),
            detail: "Run checks for the project-root schematic".to_owned(),
            tone: Tone::Warn,
        },
        DesignCheckStatus::Stale(_) => HealthCopy {
            state: "Stale".to_owned(),
            detail: "Project-root inputs changed after the retained check".to_owned(),
            tone: Tone::Warn,
        },
        DesignCheckStatus::Unavailable { reason, .. } => HealthCopy {
            state: "Unavailable".to_owned(),
            detail: reason,
            tone: Tone::Error,
        },
    }
}

fn problem_snapshot(app: &RSpiceApp) -> ProblemSnapshot {
    let root_path = format!(
        "{}/{}",
        app.state.workspace.project.root_library, app.state.workspace.project.top_cell
    );
    let result = match app.state.project_root_design_check_status() {
        DesignCheckStatus::Current(receipt) => &receipt.result,
        DesignCheckStatus::NotRun => {
            return ProblemSnapshot {
                code: "NOT RUN".to_owned(),
                message: "Run schematic checks for the project root".to_owned(),
                path: root_path,
                tone: Tone::Warn,
            };
        }
        DesignCheckStatus::Stale(_) => {
            return ProblemSnapshot {
                code: "STALE".to_owned(),
                message: "Project-root inputs changed after the retained check".to_owned(),
                path: root_path,
                tone: Tone::Warn,
            };
        }
        DesignCheckStatus::Unavailable { reason, .. } => {
            return ProblemSnapshot {
                code: "UNAVAILABLE".to_owned(),
                message: reason,
                path: root_path,
                tone: Tone::Error,
            };
        }
    };
    let highest_priority = result
        .violations()
        .iter()
        .max_by_key(|violation| drc_severity_priority(violation.severity));
    highest_priority.map_or_else(
        || ProblemSnapshot {
            code: "CHECKS".to_owned(),
            message: "No current schematic findings".to_owned(),
            path: root_path.clone(),
            tone: Tone::Ok,
        },
        |violation| ProblemSnapshot {
            code: format!("CHK-{:03}", violation.id),
            message: violation.message.clone(),
            path: violation.location.display(),
            tone: match violation.severity {
                DrcSeverity::Critical | DrcSeverity::Error => Tone::Error,
                DrcSeverity::Warning => Tone::Warn,
                DrcSeverity::Info => Tone::Info,
            },
        },
    )
}

const fn drc_severity_priority(severity: DrcSeverity) -> u8 {
    match severity {
        DrcSeverity::Critical => 4,
        DrcSeverity::Error => 3,
        DrcSeverity::Warning => 2,
        DrcSeverity::Info => 1,
    }
}

fn drc_copy(current: bool, summary: Option<DrcCounts>) -> HealthCopy {
    if !current {
        return HealthCopy {
            state: "Not current".to_owned(),
            detail: "No check result describes the active schematic topology".to_owned(),
            tone: Tone::Warn,
        };
    }
    let Some(summary) = summary else {
        return HealthCopy {
            state: "Not run".to_owned(),
            detail: "No retained schematic-check result".to_owned(),
            tone: Tone::Warn,
        };
    };
    let blockers = summary.critical + summary.errors;
    if blockers > 0 {
        HealthCopy {
            state: format!(
                "{blockers} blocking {}",
                singular_or_plural(blockers, "finding", "findings")
            ),
            detail: format!(
                "{} critical · {} errors · {} advisories",
                summary.critical,
                summary.errors,
                summary.warnings + summary.info
            ),
            tone: Tone::Error,
        }
    } else {
        HealthCopy {
            state: "Passed".to_owned(),
            detail: format!(
                "{} {}",
                summary.warnings + summary.info,
                singular_or_plural(summary.warnings + summary.info, "advisory", "advisories")
            ),
            tone: Tone::Ok,
        }
    }
}

fn configuration_snapshot(app: &RSpiceApp) -> ConfigurationSnapshot {
    let catalog = &app.state.workspace.configuration_sets;
    if let Err(error) = catalog.validate() {
        let active = catalog.active();
        return ConfigurationSnapshot {
            configured: active.is_some(),
            state: "Invalid catalog".to_owned(),
            detail: error.to_string(),
            root: active.map_or_else(
                || "Not configured".to_owned(),
                |configuration| configuration.root().display_path(),
            ),
            dut: active.map_or_else(
                || "Not configured".to_owned(),
                |configuration| configuration.dut_path().to_owned(),
            ),
            tone: Tone::Error,
        };
    }
    let Some(active) = catalog.active() else {
        return ConfigurationSnapshot {
            configured: false,
            state: "No active configuration".to_owned(),
            detail: format!(
                "{} configuration sets in project catalog",
                catalog.configurations().len()
            ),
            root: "Not configured".to_owned(),
            dut: "Not configured".to_owned(),
            tone: Tone::Warn,
        };
    };
    let projection = app.state.workspace.configuration_execution_projection(
        &app.state.library_manager,
        &app.state.workspace.active_view,
        &app.state.schematic,
    );
    let (detail, tone) = match projection {
        Ok(projection) => (
            format!(
                "rev {} · {} bindings · {} overrides · {}",
                active.revision(),
                projection.plan().map_or(0, |plan| plan.bindings().len()),
                active.overrides().len(),
                active.owner()
            ),
            Tone::Ok,
        ),
        Err(error) => (format!("Execution blocked: {error}"), Tone::Error),
    };
    ConfigurationSnapshot {
        configured: true,
        state: active.name().to_owned(),
        detail,
        root: active.root().display_path(),
        dut: active.dut_path().to_owned(),
        tone,
    }
}

fn model_snapshot(app: &RSpiceApp) -> ModelSnapshot {
    let manager = &app.state.model_library_manager;
    let library_count = manager.library_count();
    let model_count = manager.total_model_count();
    let mut sources = BTreeSet::new();
    for library in manager.libraries_sorted() {
        for source in &library.source_closure {
            sources.insert(source.path.clone());
        }
    }
    let source_count = sources.len();
    let scan_errors = &app.state.pdk_config.scan_errors;
    let binding = app.state.workspace.project.technology_binding();
    let contract_error =
        binding.and_then(|_| app.state.validate_project_technology_contract().err());
    let source_contract_error = manager.libraries_sorted().into_iter().find_map(|library| {
        if !library.source_authority.has_execution_source() {
            return None;
        }
        if library.root_path.is_none() {
            return Some(format!(
                "Model library '{}' has no canonical root source",
                library.name
            ));
        }
        if library.source_closure.is_empty() {
            return Some(format!(
                "Model library '{}' has no authenticated source closure",
                library.name
            ));
        }
        if library.source_authority.uses_retained_bytes()
            && library.source_closure.iter().any(|source| {
                !library
                    .source_contents
                    .iter()
                    .any(|content| content.path == source.path)
            })
        {
            return Some(format!(
                "Model library '{}' does not retain every pinned source",
                library.name
            ));
        }
        None
    });

    let copy = if let Some(error) = contract_error {
        HealthCopy {
            state: "Contract stale".to_owned(),
            detail: error,
            tone: Tone::Error,
        }
    } else if let Some(error) = source_contract_error {
        HealthCopy {
            state: "Closure blocked".to_owned(),
            detail: error,
            tone: Tone::Error,
        }
    } else if let Some(first) = scan_errors.first() {
        HealthCopy {
            state: format!(
                "{} scan {}",
                scan_errors.len(),
                singular_or_plural(scan_errors.len(), "diagnostic", "diagnostics")
            ),
            detail: first.clone(),
            tone: Tone::Warn,
        }
    } else if library_count == 0 {
        HealthCopy {
            state: "No model libraries".to_owned(),
            detail: "No loaded executable model catalog".to_owned(),
            tone: Tone::Warn,
        }
    } else if binding.is_none() {
        HealthCopy {
            state: "Technology not attached".to_owned(),
            detail: format!(
                "{library_count} {} · {model_count} models · {source_count} source files",
                singular_or_plural(library_count, "library", "libraries")
            ),
            tone: Tone::Neutral,
        }
    } else {
        HealthCopy {
            state: "Contract current".to_owned(),
            detail: format!(
                "{library_count} {} · {model_count} models · {source_count} source files",
                singular_or_plural(library_count, "library", "libraries")
            ),
            tone: Tone::Ok,
        }
    };

    ModelSnapshot {
        state: copy.state,
        detail: copy.detail,
        tone: copy.tone,
    }
}

fn document_state(document: &OpenDocumentSnapshot) -> &'static str {
    if document.dirty {
        "modified"
    } else if document.active {
        "active"
    } else {
        "saved"
    }
}

const fn lifecycle_label(lifecycle: SimulationRunLifecycle, success: bool) -> &'static str {
    match lifecycle {
        SimulationRunLifecycle::LegacyUnknown => "legacy state unknown",
        SimulationRunLifecycle::Preparing => "preparing",
        SimulationRunLifecycle::Running => "running",
        SimulationRunLifecycle::Cancelling => "cancelling",
        SimulationRunLifecycle::Completed if success => "completed",
        SimulationRunLifecycle::Completed => "completed with failures",
        SimulationRunLifecycle::Failed => "failed",
        SimulationRunLifecycle::Aborted => "aborted",
        SimulationRunLifecycle::Interrupted => "interrupted",
    }
}

const fn lifecycle_tone(lifecycle: SimulationRunLifecycle, success: bool) -> Tone {
    match lifecycle {
        SimulationRunLifecycle::Completed if success => Tone::Ok,
        SimulationRunLifecycle::Preparing | SimulationRunLifecycle::Running => Tone::Info,
        SimulationRunLifecycle::Cancelling | SimulationRunLifecycle::Aborted => Tone::Warn,
        SimulationRunLifecycle::Completed
        | SimulationRunLifecycle::Failed
        | SimulationRunLifecycle::Interrupted => Tone::Error,
        SimulationRunLifecycle::LegacyUnknown => Tone::Neutral,
    }
}

fn run_title(sequence: u64, label: &str) -> String {
    let prefix = format!("Run {sequence}");
    if label.starts_with(&prefix) {
        label.to_owned()
    } else {
        format!("{prefix} · {label}")
    }
}

fn format_duration(seconds: f64) -> String {
    if !seconds.is_finite() || seconds < 0.0 {
        return "unavailable".to_owned();
    }
    if seconds >= 60.0 {
        let minutes = (seconds / 60.0).floor() as u64;
        let remainder = seconds - minutes as f64 * 60.0;
        format!("{minutes} min {remainder:.1} s")
    } else {
        format!("{seconds:.3} s")
    }
}

fn unix_seconds_to_millis(seconds: f64) -> Option<u64> {
    if !seconds.is_finite() || seconds < 0.0 || seconds > u64::MAX as f64 / 1_000.0 {
        return None;
    }
    Some((seconds * 1_000.0).round() as u64)
}

fn relative_time(timestamp_ms: u64, now_ms: u64) -> String {
    if timestamp_ms > now_ms {
        return "time n/a".to_owned();
    }
    match now_ms - timestamp_ms {
        0..=59_999 => "now".to_owned(),
        60_000..=3_599_999 => format!("{} min", (now_ms - timestamp_ms) / 60_000),
        3_600_000..=86_399_999 => format!("{} h", (now_ms - timestamp_ms) / 3_600_000),
        _ => format!("{} d", (now_ms - timestamp_ms) / 86_400_000),
    }
}

const fn singular_or_plural<'a>(count: usize, singular: &'a str, plural: &'a str) -> &'a str {
    if count == 1 { singular } else { plural }
}

fn paint_elided(
    ui: &Ui,
    position: egui::Pos2,
    text: &str,
    font: egui::FontId,
    color: Color32,
    max_width: f32,
) {
    let text = elide_text(ui, text, &font, max_width);
    ui.painter()
        .text(position, Align2::LEFT_TOP, text, font, color);
}

fn elide_text(ui: &Ui, text: &str, font: &egui::FontId, max_width: f32) -> String {
    if max_width <= 0.0 {
        return String::new();
    }
    if ui
        .painter()
        .layout_no_wrap(text.to_owned(), font.clone(), Color32::WHITE)
        .size()
        .x
        <= max_width
    {
        return text.to_owned();
    }
    let characters = text.chars().collect::<Vec<_>>();
    let mut low = 0;
    let mut high = characters.len();
    while low < high {
        let midpoint = (low + high).div_ceil(2);
        let candidate = characters[..midpoint]
            .iter()
            .copied()
            .chain(std::iter::once('…'))
            .collect::<String>();
        let width = ui
            .painter()
            .layout_no_wrap(candidate, font.clone(), Color32::WHITE)
            .size()
            .x;
        if width <= max_width {
            low = midpoint;
        } else {
            high = midpoint - 1;
        }
    }
    characters[..low]
        .iter()
        .copied()
        .chain(std::iter::once('…'))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stale_checks_never_reuse_retained_result_copy() {
        let copy = drc_copy(
            false,
            Some(DrcCounts {
                critical: 0,
                errors: 0,
                warnings: 0,
                info: 0,
            }),
        );
        assert_eq!(copy.state, "Not current");
        assert_eq!(copy.tone, Tone::Warn);
    }

    #[test]
    fn current_check_blockers_are_counted_without_losing_advisories() {
        let copy = drc_copy(
            true,
            Some(DrcCounts {
                critical: 1,
                errors: 2,
                warnings: 3,
                info: 1,
            }),
        );
        assert_eq!(copy.state, "3 blocking findings");
        assert_eq!(copy.detail, "1 critical · 2 errors · 4 advisories");
        assert_eq!(copy.tone, Tone::Error);
    }

    #[test]
    fn completed_failure_is_not_rendered_as_success() {
        assert_eq!(
            lifecycle_label(SimulationRunLifecycle::Completed, false),
            "completed with failures"
        );
        assert_eq!(
            lifecycle_tone(SimulationRunLifecycle::Completed, false),
            Tone::Error
        );
    }

    #[test]
    fn run_title_does_not_duplicate_retained_sequence_prefix() {
        assert_eq!(
            run_title(12, "Run 12 (10:31:02 AM)"),
            "Run 12 (10:31:02 AM)"
        );
        assert_eq!(run_title(12, "corner sweep"), "Run 12 · corner sweep");
    }

    #[test]
    fn recent_operation_timestamps_fail_closed_and_format_relative_age() {
        assert_eq!(unix_seconds_to_millis(f64::NAN), None);
        assert_eq!(unix_seconds_to_millis(-1.0), None);
        assert_eq!(unix_seconds_to_millis(1.25), Some(1_250));
        assert_eq!(relative_time(1_000, 999), "time n/a");
        assert_eq!(relative_time(1_000, 1_030), "now");
        assert_eq!(relative_time(1_000, 61_000), "1 min");
        assert_eq!(relative_time(1_000, 3_601_000), "1 h");
        assert_eq!(relative_time(1_000, 86_401_000), "1 d");
    }

    #[test]
    fn overview_new_cell_action_opens_the_real_creation_workflow() {
        let mut app = RSpiceApp::test_instance();
        OverviewIntent::Command(Command::NewCell).execute(&mut app);
        assert!(app.state.dialogs.new_cell_dialog);
    }

    #[test]
    fn overview_history_action_preserves_project_context() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Project;
        let active = app.state.workspace.active_view.clone();
        OverviewIntent::Command(Command::RevisionHistory).execute(&mut app);

        assert_eq!(app.state.workbench.workspace, Workspace::Project);
        assert_eq!(app.state.workspace.active_view, active);
        assert!(app.state.dialogs.project_revision_history.open);
    }

    #[test]
    fn revision_history_is_available_from_project_overview() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Project;

        assert!(Command::RevisionHistory.is_enabled(&app));
    }

    #[test]
    fn overview_run_without_a_dataset_returns_to_analyses() {
        let mut app = RSpiceApp::test_instance();
        app.state.simulation.start_run();

        OverviewIntent::OpenRun(0).execute(&mut app);

        assert_eq!(app.state.workbench.workspace, Workspace::Simulate);
    }

    #[test]
    fn overview_run_action_opens_the_plan_without_starting_execution() {
        let mut app = RSpiceApp::test_instance();
        let retained_runs = app.state.simulation.runs.len();

        run_plan_intent().execute(&mut app);

        assert_eq!(app.state.workbench.workspace, Workspace::Simulate);
        assert_eq!(app.state.simulation.runs.len(), retained_runs);
        assert!(!app.state.simulation.is_running);
    }

    #[test]
    fn overview_import_deck_uses_the_independent_netlist_project_workflow() {
        assert_eq!(
            import_deck_intent(),
            OverviewIntent::Command(Command::OpenNetlist)
        );
    }

    #[test]
    fn netlist_first_overview_never_exposes_the_pristine_bootstrap_schematic() {
        let mut app = RSpiceApp::test_instance();
        assert!(
            crate::workbench::workflows::netlist_workflow::apply_imported_netlist(
                &mut app.state,
                "V1 out 0 1\n.op\n.end\n".to_owned(),
                None,
                "front_end.sp",
            )
        );
        app.state.workbench.workspace = Workspace::Project;

        let snapshot = OverviewSnapshot::capture(&app);
        let objects = design_objects(&snapshot);

        assert!(snapshot.netlist_first);
        assert_eq!(snapshot.statuses[0].area, "Source deck");
        assert!(
            snapshot
                .statuses
                .iter()
                .all(|status| status.area != "Schematic checks")
        );
        assert_eq!(snapshot.problem.code, "NOT RUN");
        assert_eq!(snapshot.problem.path, "front_end.sp");
        assert!(objects.first().is_some_and(|object| {
            object.top && object.name == "front_end.sp" && object.kind.starts_with("SPICE deck")
        }));
        assert!(
            objects
                .iter()
                .all(|object| object.name != snapshot.descriptor_root)
        );
    }

    #[test]
    fn overview_model_closure_always_opens_the_project_catalog_scope() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.models_view.catalog_scope = ModelsCatalogScope::InstalledPacks;
        OverviewIntent::OpenProjectModels.execute(&mut app);

        assert_eq!(app.state.workbench.workspace, Workspace::Models);
        assert_eq!(
            app.state.workbench.models_view.catalog_scope,
            ModelsCatalogScope::Project
        );
    }

    #[test]
    fn current_checks_without_a_retained_result_are_never_green() {
        let mut app = RSpiceApp::test_instance();
        app.state.workspace.active_view = CellViewRef::new(
            &app.state.workspace.project.root_library,
            &app.state.workspace.project.top_cell,
            crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
        );
        let problem = problem_snapshot(&app);
        assert_eq!(problem.code, "NOT RUN");
        assert_eq!(problem.tone, Tone::Warn);
    }

    #[test]
    fn project_problem_selects_the_highest_severity_finding() {
        use crate::services::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};

        let mut app = RSpiceApp::test_instance();
        app.state.workspace.active_view = CellViewRef::new(
            &app.state.workspace.project.root_library,
            &app.state.workspace.project.top_cell,
            crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
        );
        let mut result = DrcResult::new();
        result.add_violation(
            DrcViolation::new(
                1,
                DrcViolationType::EmptyName,
                "informational finding",
                DrcLocation::Global,
            )
            .with_severity(DrcSeverity::Info),
        );
        result.add_violation(
            DrcViolation::new(
                2,
                DrcViolationType::MissingGround,
                "critical finding",
                DrcLocation::Global,
            )
            .with_severity(DrcSeverity::Critical),
        );
        result.completed = true;
        app.state
            .publish_active_design_check_result(result)
            .expect("publish project-root design-check receipt");

        let problem = problem_snapshot(&app);
        assert_eq!(problem.code, "CHK-002");
        assert_eq!(problem.message, "critical finding");
        assert_eq!(problem.tone, Tone::Error);
    }

    #[test]
    fn design_register_rows_have_unique_truthful_identities() {
        let app = RSpiceApp::test_instance();
        let snapshot = OverviewSnapshot::capture(&app);
        let rows = design_objects(&snapshot);
        let unique = rows
            .iter()
            .map(|row| row.key.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();

        assert_eq!(unique.len(), rows.len());
        assert!(rows.len() <= 5);
        assert!(rows.first().is_some_and(|row| row.top));
    }

    #[test]
    fn overview_does_not_invent_a_testbench_or_dut_without_a_configuration() {
        let app = RSpiceApp::test_instance();
        let snapshot = OverviewSnapshot::capture(&app);

        assert!(!snapshot.configuration.configured);
        assert_eq!(snapshot.configuration.root, "Not configured");
        assert_eq!(snapshot.configuration.dut, "Not configured");
        assert!(
            design_objects(&snapshot)
                .iter()
                .all(|row| !row.name.eq_ignore_ascii_case("Not configured"))
        );
    }
}
