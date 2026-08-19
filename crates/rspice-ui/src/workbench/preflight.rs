//! Revision-bound simulation-preflight workflow from the workbench mockup.
//!
//! Validation is executed once, captured as an immutable report, and then
//! rendered without recomputing beneath the operator. Every blocker has an
//! explicit destination and a validated run can be queued from the report.

use egui::{Align2, Color32, Context, Frame, Margin, Pos2, Rect, Sense, Stroke, Ui, Vec2};

use crate::diagnostics::ConsoleMessage;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;

use super::commands::vocabulary::Command;
use super::design_system::property_row;
use super::state::{
    ConsolePage, PreflightIssue, PreflightRemediation, PreflightReport, PreflightToast,
    PreparedPreflightContract, ProjectPage, VerificationPage, Workspace,
};

const ISSUE_TABLE_HEADERS: [&str; 5] = ["Order", "Check", "Observed", "Required", "Action"];
const CLEAN_CHECK: &str = "No blocking issues.";
const EMPTY_CELL: &str = "—";
const FROZEN_DISPATCH_ROWS: [&str; 5] = [
    "Revision",
    "Analysis identities",
    "PVT points",
    "Tasks",
    "Target",
];
const RUNNABLE_HEADING: &str = "Plan is runnable";
const RUNNABLE_SUMMARY: &str =
    "Immutable inputs, target, task graph, and save policy are ready for dispatch.";
const BLOCKED_SUMMARY: &str =
    "The run was not queued. Resolve the ordered issues below, then rerun preflight.";
const TECHNOLOGY_NOT_REQUIRED_ADVISORY: &str = "Technology: not attached — not required by this plan (built-in device models, TT reference process)";
const PREFLIGHT_DIALOG_SIZE: DialogSize = DialogSize::SimulationWorkflow;
const ISSUE_TABLE_COMPACT_BREAKPOINT: f32 = 680.0;
const CONTEXT_STACK_BREAKPOINT: f32 = 760.0;
const TABLE_HEADER_HEIGHT: f32 = 27.0;
const TABLE_CELL_PADDING_X: f32 = 8.0;
const TABLE_CELL_PADDING_Y: f32 = 6.0;
const TABLE_BUTTON_HORIZONTAL_CHROME: f32 = 20.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IssueLayout {
    Table,
    Records,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContextLayout {
    Split,
    Stacked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PreflightBodyLayout {
    issues: IssueLayout,
    context: ContextLayout,
}

impl PreflightBodyLayout {
    fn resolve(local_width: f32, viewport_width: f32) -> Self {
        Self {
            issues: if local_width < ISSUE_TABLE_COMPACT_BREAKPOINT {
                IssueLayout::Records
            } else {
                IssueLayout::Table
            },
            context: if viewport_width <= CONTEXT_STACK_BREAKPOINT {
                ContextLayout::Stacked
            } else {
                ContextLayout::Split
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct IssueTableGeometry {
    column_edges: [f32; 6],
}

impl IssueTableGeometry {
    fn resolve(width: f32, measured_action_width: f32) -> Self {
        let width = width.max(1.0);
        let order_width = 54.0_f32.min(width * 0.12).max(1.0);
        let action_width =
            (measured_action_width + TABLE_CELL_PADDING_X * 2.0 + TABLE_BUTTON_HORIZONTAL_CHROME)
                .clamp(132.0, 160.0)
                .min((width - order_width).max(1.0));
        let middle = (width - order_width - action_width).max(1.0);
        let check_width = middle * 0.23;
        let observed_width = middle * 0.39;
        let required_width = (middle - check_width - observed_width).max(1.0);
        let mut column_edges = [0.0; 6];
        column_edges[1] = order_width;
        column_edges[2] = column_edges[1] + check_width;
        column_edges[3] = column_edges[2] + observed_width;
        column_edges[4] = column_edges[3] + required_width;
        column_edges[5] = width;
        Self { column_edges }
    }

    fn cell(self, row: Rect, index: usize) -> Rect {
        Rect::from_min_max(
            Pos2::new(row.left() + self.column_edges[index], row.top()),
            Pos2::new(row.left() + self.column_edges[index + 1], row.bottom()),
        )
    }
}

/// Run every local preflight check and retain the exact report for the current
/// project revision. A blocked report opens immediately; a clean report leaves
/// the operator on the current surface, matching the mockup workflow.
pub(crate) fn run(app: &mut RSpiceApp) {
    app.state.sync_active_schematic_to_workspace();
    crate::workbench::menu_bar::run_design_rule_check(&mut app.state);
    let mut report = collect_report(&app.state);
    if report.blockers.is_empty() {
        match app
            .simulation_controller
            .prepare_run_set_for_preflight(&app.state)
        {
            Ok(metadata) => {
                report.advisories.extend(metadata.advisories);
                report.prepared = Some(PreparedPreflightContract {
                    snapshot_digest: metadata.snapshot_digest,
                    source_digest: metadata.source_digest,
                    receipt_digest: metadata.receipt_digest,
                    receipt_label: metadata.receipt_label,
                    analysis_ids: metadata.analysis_ids,
                    task_count: metadata.task_count,
                    saved_output_contract_count: metadata.saved_output_contract_count,
                    pvt_point_count: metadata.pvt_point_count,
                    target: metadata.target,
                    save_policy: metadata.save_policy,
                    model_identity_count: metadata.model_identity_count,
                });
            }
            Err(error) => report.blockers.push(PreflightIssue {
                check: preparation_check_label(error.stage()).to_owned(),
                observed: error.message().to_owned(),
                required: preparation_requirement(error.stage()).to_owned(),
                remediation: preparation_remediation(error.stage()),
            }),
        }
    }
    if report.prepared.is_none() {
        app.simulation_controller.clear_prepared_run();
    }
    let current_plan = app.state.active_plan_revision();
    let (topology_root, topology_revision, topology_closure) =
        app.state.configured_topology_revision();
    let blocked = !report.is_runnable_for(
        app.state.workspace.project.revision().get(),
        &topology_root,
        topology_revision,
        &topology_closure,
        current_plan,
    );
    let message = if blocked {
        format!(
            "Preflight blocked · {} blocking issue{} · design revision {} was not queued",
            report.blockers.len(),
            if report.blockers.len() == 1 { "" } else { "s" },
            report.project_revision
        )
    } else {
        let task_count = report
            .prepared
            .as_ref()
            .map_or(0, |prepared| prepared.task_count);
        format!(
            "Preflight complete · 0 blocking errors · {} advisor{} · design revision {} · {} task{}",
            report.advisories.len(),
            if report.advisories.len() == 1 {
                "y"
            } else {
                "ies"
            },
            report.project_revision,
            task_count,
            if task_count == 1 { "" } else { "s" }
        )
    };

    app.state.workbench.preflight.report = Some(report);
    app.state.workbench.preflight.open = blocked;
    app.state.workbench.preflight.pending_toast = Some(PreflightToast {
        message: message.clone(),
        warning: blocked,
    });

    if blocked {
        app.state
            .push_user_message(ConsoleMessage::warning(message));
        app.state.workbench.console_visible = true;
        app.state.workbench.console_page = ConsolePage::Problems;
    } else {
        app.state.push_user_message(ConsoleMessage::info(message));
    }
}

/// Run the same governed preflight used by the explicit Preflight command and
/// queue only the exact immutable snapshot it authorizes. A retained current
/// report is reusable; the controller still rebuilds and digest-compares the
/// live contract when it consumes the one-shot permit.
pub(crate) fn run_and_queue(app: &mut RSpiceApp) {
    if app.state.simulation.has_active_execution() || app.state.simulation.trigger_simulation {
        app.state.push_user_message(ConsoleMessage::warning(
            "A simulation is already running or waiting to start".to_owned(),
        ));
        return;
    }

    if queue_retained_run(app) {
        return;
    }

    run(app);
    let _ = queue_retained_run(app);
}

/// Queue a retained authorized report only while every revision it names is
/// still current. This is also the dialog's primary action, avoiding a second
/// preflight pass between review and dispatch.
fn queue_retained_run(app: &mut RSpiceApp) -> bool {
    let project_revision = app.state.workspace.project.revision().get();
    let (topology_root, topology_revision, topology_closure) =
        app.state.configured_topology_revision();
    let current_plan = app.state.active_plan_revision();
    let runnable = app
        .state
        .workbench
        .preflight
        .report
        .as_ref()
        .is_some_and(|report| {
            report.is_runnable_for(
                project_revision,
                &topology_root,
                topology_revision,
                &topology_closure,
                current_plan,
            )
        });
    if !runnable {
        return false;
    }

    app.state.workbench.preflight.open = false;
    app.state.request_run_set_simulation();
    app.state.workbench.activate(Workspace::Simulate);
    true
}

fn collect_report(state: &AppState) -> PreflightReport {
    let mut blockers = Vec::new();
    let mut advisories = Vec::new();
    let root_reference = state.workspace.simulation_root_reference();
    let execution_projection = state.workspace.configuration_execution_projection(
        &state.library_manager,
        &state.workspace.active_view,
        &state.schematic,
    );

    match &execution_projection {
        Err(error) => blockers.push(PreflightIssue {
            check: "Design topology".to_owned(),
            observed: error.to_string(),
            required: "An existing, fully resolved executable configuration root".to_owned(),
            remediation: PreflightRemediation::DesignChecks,
        }),
        Ok(projection)
            if projection
                .root_schematic()
                .is_some_and(|root| root.components.is_empty()) =>
        {
            blockers.push(PreflightIssue {
                check: "Design topology".to_owned(),
                observed: format!(
                    "Configured simulation root '{}' contains no components.",
                    root_reference.display_path()
                ),
                required: "A non-empty circuit topology".to_owned(),
                remediation: PreflightRemediation::DesignChecks,
            });
        }
        Ok(_) => {}
    }

    let hierarchy_resolution = state.workspace.resolve_hierarchy_with_active(
        &state.library_manager,
        &state.workspace.active_view,
        &state.schematic,
    );
    for binding in hierarchy_resolution
        .bindings
        .iter()
        .filter(|binding| !binding.status.is_resolved())
    {
        let instance_scope = if binding.instance_count == 1 {
            String::new()
        } else {
            format!(" · {} instances", binding.instance_count)
        };
        let diagnostic = binding.diagnostic.clone().unwrap_or_else(|| {
            format!(
                "{}/{} is {}",
                binding.reference.library,
                binding.reference.cell,
                binding.status.label()
            )
        });
        blockers.push(PreflightIssue {
            check: "Hierarchy binding".to_owned(),
            observed: format!("{diagnostic}{instance_scope}"),
            required: "A finite executable master for every hierarchical instance".to_owned(),
            remediation: PreflightRemediation::DesignChecks,
        });
    }
    for binding in hierarchy_resolution
        .bindings
        .iter()
        .filter(|binding| binding.used_review_fallback)
    {
        advisories.push(format!(
            "Configuration fallback reviewed for {} at {}",
            binding.reference.display_path(),
            binding.instance_paths.join(", ")
        ));
    }
    // A warning is a resolved binding the configuration honours differently
    // than it reads. It blocks nothing — the run will proceed — but starting
    // one without saying so leaves the author reading a policy the executed
    // hierarchy does not follow.
    for binding in &hierarchy_resolution.bindings {
        for warning in &binding.warnings {
            advisories.push(format!(
                "Configuration warning for {}: {warning}",
                binding.reference.display_path()
            ));
        }
    }

    if let Ok(execution_projection) = &execution_projection {
        let root_schematic = execution_projection
            .root_schematic()
            .expect("a successful execution projection has a materialized root");
        let hierarchy_source =
            crate::simulation::netlist_gen::HierarchySource::from_execution_projection(
                &state.library_manager,
                execution_projection,
            );
        let result = crate::services::drc::run_drc_check_with_hierarchy_and_config(
            root_schematic,
            &hierarchy_source,
            crate::services::drc::DrcConfig {
                check_missing_ground: true,
                ..crate::services::drc::DrcConfig::default()
            },
        );
        if result.completed {
            for violation in result.errors() {
                blockers.push(PreflightIssue {
                    check: "Schematic validation".to_owned(),
                    observed: format!("{} · {}", violation.message, violation.location.display()),
                    required: violation.violation_type.suggested_fix().to_owned(),
                    remediation: PreflightRemediation::DesignChecks,
                });
            }
            for violation in result.warnings() {
                advisories.push(format!(
                    "{} · {}",
                    violation.message,
                    violation.location.display()
                ));
            }
        } else {
            blockers.push(PreflightIssue {
                check: "Schematic validation".to_owned(),
                observed: "The configured-root schematic validator did not complete.".to_owned(),
                required: "A completed validation result for this topology revision".to_owned(),
                remediation: PreflightRemediation::DesignChecks,
            });
        }
    }

    match state.sim_setup.stable_analysis_plan() {
        Err(error) => blockers.push(PreflightIssue {
            check: "Analysis-instance plan".to_owned(),
            observed: error,
            required: "A migrated, structurally valid simulation plan".to_owned(),
            remediation: PreflightRemediation::SimulationPlan,
        }),
        Ok(plan) => {
            for issue in plan.validation_issues() {
                blockers.push(PreflightIssue {
                    check: "Analysis-instance graph".to_owned(),
                    observed: issue.to_string(),
                    required: "Stable enabled identities with resolved, ordered prerequisites"
                        .to_owned(),
                    remediation: PreflightRemediation::SimulationPlan,
                });
            }
            for instance in plan
                .instances()
                .iter()
                .filter(|instance| instance.enabled())
            {
                if let Some(error) = state
                    .sim_setup
                    .analysis_draft_validation_error(instance.draft())
                {
                    blockers.push(PreflightIssue {
                        check: format!("{} configuration", instance.kind().label()),
                        observed: format!("{}: {error}", instance.id()),
                        required: "A complete, numerically valid analysis configuration".to_owned(),
                        remediation: PreflightRemediation::SimulationPlan,
                    });
                }
            }
        }
    }

    // A project need not have a technology; if it has one it must be valid,
    // and if the plan needs one it must have one. Each demanding entity is
    // reported as its own row so the operator sees what to change.
    let demand = state.technology_demand();
    match state.workspace.project.technology_binding() {
        None if demand.is_empty() => advisories.push(TECHNOLOGY_NOT_REQUIRED_ADVISORY.to_owned()),
        None => blockers.extend(demand.reasons().iter().map(|reason| PreflightIssue {
            check: "Project technology contract".to_owned(),
            observed: reason.observed(),
            required: reason.required(),
            remediation: PreflightRemediation::ProjectTechnology,
        })),
        Some(binding) => {
            if state.workspace.project.technology_change_audit().is_empty() {
                blockers.push(PreflightIssue {
                    check: "Project technology contract".to_owned(),
                    observed:
                        "The attached technology binding predates checkpoint-backed authority receipts"
                            .to_owned(),
                    required: "Reattach the technology to record an audited change receipt"
                        .to_owned(),
                    remediation: PreflightRemediation::ProjectTechnology,
                });
            }
            if let Err(error) = state
                .model_library_manager
                .validate_attached_technology(Some(binding))
            {
                blockers.push(PreflightIssue {
                    check: "Project model technology".to_owned(),
                    observed: error,
                    required:
                        "The exact attached model-source closure must resolve without catalog drift"
                            .to_owned(),
                    remediation: PreflightRemediation::ProjectTechnology,
                });
            }
            if let Err(error) =
                binding.validate_signed_package(&state.pdk_config.technology_registry)
            {
                blockers.push(PreflightIssue {
                    check: "Project signed PDK".to_owned(),
                    observed: error.to_string(),
                    required:
                        "The exact attached package revision must remain signature-trusted and compatible with this runtime"
                            .to_owned(),
                    remediation: PreflightRemediation::ProjectTechnology,
                });
            }
        }
    }

    // An unmet demand already owns the missing reference section and carries the
    // only remediation that resolves it; a second row would contradict it.
    if (state.project_technology_in_effect() || demand.is_empty())
        && let Err(error) = state
            .model_library_manager
            .reference_process_model_cards(state.sim_setup.reference_pvt.process)
    {
        blockers.push(PreflightIssue {
            check: "Reference model binding".to_owned(),
            observed: error,
            required: format!(
                "A resolved {} model section",
                state.sim_setup.reference_pvt.process.short_name()
            ),
            remediation: PreflightRemediation::DesignChecks,
        });
    }

    let simulation_plan = state.active_plan_revision();
    let (topology_root, topology_revision, topology_closure) = state.configured_topology_revision();
    PreflightReport {
        project_revision: state.workspace.project.revision().get(),
        topology_root,
        topology_revision,
        topology_closure,
        simulation_plan_id: simulation_plan.map(|(id, _)| id),
        simulation_plan_revision: simulation_plan.map(|(_, revision)| revision),
        blockers,
        advisories,
        prepared: None,
    }
}

fn preparation_check_label(stage: crate::simulation::execution::PreparationStage) -> &'static str {
    use crate::simulation::execution::PreparationStage;
    match stage {
        PreparationStage::DesignChecks => "Schematic validation",
        PreparationStage::SourceChecks => "Source closure",
        PreparationStage::AnalysisPlan => "Run set",
        PreparationStage::ModelBindings => "Model bindings",
        PreparationStage::Netlist => "Executable netlist",
        PreparationStage::Authorization => "Dispatch authorization",
    }
}

fn preparation_requirement(stage: crate::simulation::execution::PreparationStage) -> &'static str {
    use crate::simulation::execution::PreparationStage;
    match stage {
        PreparationStage::DesignChecks => "A current completed source-check receipt",
        PreparationStage::SourceChecks => "A closed, parseable source dependency set",
        PreparationStage::AnalysisPlan => "A complete executable analysis configuration",
        PreparationStage::ModelBindings => "Authenticated materialized model sources",
        PreparationStage::Netlist => "A deterministic self-contained executable netlist",
        PreparationStage::Authorization => "A one-use generation-safe dispatch authorization",
    }
}

fn preparation_remediation(
    stage: crate::simulation::execution::PreparationStage,
) -> PreflightRemediation {
    use crate::simulation::execution::PreparationStage;
    match stage {
        PreparationStage::AnalysisPlan | PreparationStage::Authorization => {
            PreflightRemediation::SimulationPlan
        }
        PreparationStage::ModelBindings => PreflightRemediation::DesignChecks,
        PreparationStage::DesignChecks
        | PreparationStage::SourceChecks
        | PreparationStage::Netlist => PreflightRemediation::DesignChecks,
    }
}

/// Render the retained report above the workbench. This function also turns
/// the command's context-free notification into a real egui toast.
pub(crate) fn show(ctx: &Context, app: &mut RSpiceApp) {
    if let Some(toast) = app.state.workbench.preflight.pending_toast.take() {
        if toast.warning {
            app.state
                .ui
                .toasts
                .warn_with_title(ctx, "Simulation preflight blocked", toast.message);
        } else {
            app.state
                .ui
                .toasts
                .success(ctx, "Simulation preflight passed", toast.message);
        }
    }

    if !app.state.workbench.preflight.open {
        return;
    }
    let Some(report) = app.state.workbench.preflight.report.clone() else {
        app.state.workbench.preflight.open = false;
        return;
    };

    // A report is valid only for the exact project, topology, and active-plan
    // revision it inspected. Drop the controller permit with the report so no
    // hidden stale authorization survives an out-of-band plan mutation.
    let project_revision = app.state.workspace.project.revision().get();
    let (topology_root, topology_revision, topology_closure) =
        app.state.configured_topology_revision();
    let current_plan = app.state.active_plan_revision();
    if !report.is_current_for(
        project_revision,
        &topology_root,
        topology_revision,
        &topology_closure,
        current_plan,
    ) {
        app.invalidate_simulation_preflight();
        app.state.ui.toasts.warn_with_title(
            ctx,
            "Preflight report expired",
            "Preflight report expired because the design or simulation-plan revision changed",
        );
        return;
    }

    let runnable = report.is_runnable_for(
        project_revision,
        &topology_root,
        topology_revision,
        &topology_closure,
        current_plan,
    );
    let primary = if runnable {
        "Queue validated run"
    } else {
        "Close"
    };
    let hint = format!(
        "Design revision {} · {} blocking · {} advisory",
        report.project_revision,
        report.blockers.len(),
        report.advisories.len()
    );
    let kicker = format!(
        "SIMULATION · DESIGN REVISION {} · ORDERED CORRECTIVE ACTION",
        report.project_revision
    );
    let mut requested_fix = None;
    let choice = Dialog::new(&kicker, "Simulation preflight", primary)
        .description(
            "Review ordered blockers, advisories, and frozen run inputs before closing or queuing this validated set of simulation inputs.",
        )
        .size(PREFLIGHT_DIALOG_SIZE)
        .flush_body()
        .hint(&hint)
        .show(ctx, |ui| {
            report_summary(ui, &report);
            blocker_list(ui, &report, &mut requested_fix);
            report_context(ui, &report);
        });

    if let Some(remediation) = requested_fix {
        app.state.workbench.preflight.open = false;
        apply_remediation(app, remediation);
        return;
    }

    match choice {
        DialogChoice::Primary if runnable => {
            if !queue_retained_run(app) {
                app.invalidate_simulation_preflight();
                app.state.ui.toasts.warn_with_title(
                    ctx,
                    "Preflight report expired",
                    "Run inputs changed before the validated snapshot could be queued",
                );
            }
        }
        DialogChoice::Primary | DialogChoice::Cancelled => {
            app.state.workbench.preflight.open = false;
        }
        DialogChoice::None | DialogChoice::Secondary | DialogChoice::Ghost => {}
    }
}

fn report_summary(ui: &mut Ui, report: &PreflightReport) {
    let t = Tokens::get(ui.ctx());
    // `show` rejects stale reports before rendering this immutable snapshot.
    let runnable = report.blockers.is_empty() && report.prepared.is_some();
    let heading = summary_heading(report);
    let tone = if runnable { t.color.ok } else { t.color.err };
    let width = ui.available_width().max(1.0);
    let band = Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(Margin {
            left: 12,
            right: 12,
            top: 10,
            bottom: 10,
        })
        .show(ui, |ui| {
            ui.set_width((width - 24.0).max(1.0));
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                let (mark_rect, response) =
                    ui.allocate_exact_size(Vec2::new(38.0, 38.0), Sense::hover());
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Label,
                        ui.is_enabled(),
                        if runnable { "Ready" } else { "Blocked" },
                    )
                });
                paint_summary_mark(ui, mark_rect, tone, runnable);
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 2.0;
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(heading)
                                .font(theme::sans(tokens::FS_2, FontWeight::SemiBold)),
                        )
                        .wrap(),
                    );
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(if runnable {
                                RUNNABLE_SUMMARY
                            } else {
                                BLOCKED_SUMMARY
                            })
                            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                            .color(t.color.text_dim),
                        )
                        .wrap(),
                    );
                });
            });
        });
    ui.painter().rect_filled(
        Rect::from_min_max(
            band.response.rect.left_top(),
            Pos2::new(band.response.rect.left() + 3.0, band.response.rect.bottom()),
        ),
        0.0,
        tone,
    );
    ui.painter().hline(
        band.response.rect.x_range(),
        band.response.rect.bottom() - 0.5,
        Stroke::new(1.0, t.color.border_strong),
    );
}

fn paint_summary_mark(ui: &Ui, rect: Rect, tone: Color32, runnable: bool) {
    let center = rect.center();
    let stroke = Stroke::new(1.5, tone);
    if runnable {
        ui.painter().circle_stroke(center, 9.0, stroke);
        ui.painter().line_segment(
            [center + Vec2::new(-4.0, 0.0), center + Vec2::new(-1.0, 3.5)],
            stroke,
        );
        ui.painter().line_segment(
            [center + Vec2::new(-1.0, 3.5), center + Vec2::new(5.0, -4.0)],
            stroke,
        );
    } else {
        let points = [
            center + Vec2::new(0.0, -10.0),
            center + Vec2::new(10.0, 8.0),
            center + Vec2::new(-10.0, 8.0),
        ];
        ui.painter()
            .add(egui::Shape::closed_line(points.to_vec(), stroke));
        ui.painter().line_segment(
            [center + Vec2::new(0.0, -4.0), center + Vec2::new(0.0, 2.0)],
            stroke,
        );
        ui.painter()
            .circle_filled(center + Vec2::new(0.0, 5.0), 1.2, tone);
    }
}

fn blocker_list(
    ui: &mut Ui,
    report: &PreflightReport,
    requested_fix: &mut Option<PreflightRemediation>,
) {
    if PreflightBodyLayout::resolve(ui.available_width(), ui.ctx().content_rect().width()).issues
        == IssueLayout::Records
    {
        compact_issue_list(ui, report, requested_fix);
    } else {
        wide_issue_table(ui, report, requested_fix);
    }
}

fn wide_issue_table(
    ui: &mut Ui,
    report: &PreflightReport,
    requested_fix: &mut Option<PreflightRemediation>,
) {
    let t = Tokens::get(ui.ctx());
    let action_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let measured_action_width = [
        remediation_label(PreflightRemediation::DesignChecks),
        remediation_label(PreflightRemediation::SimulationPlan),
        remediation_label(PreflightRemediation::ProjectTechnology),
    ]
    .into_iter()
    .map(|label| {
        ui.painter()
            .layout_no_wrap(label.to_owned(), action_font.clone(), t.color.text)
            .size()
            .x
    })
    .fold(0.0_f32, f32::max);
    let geometry = IssueTableGeometry::resolve(ui.available_width(), measured_action_width);
    issue_table_header(ui, geometry, &t);

    if report.blockers.is_empty() {
        wide_clean_row(ui, geometry, &t);
        return;
    }

    for (index, issue) in report.blockers.iter().enumerate() {
        wide_issue_row(ui, geometry, index, issue, requested_fix, &t);
    }
}

fn issue_table_header(ui: &mut Ui, geometry: IssueTableGeometry, t: &Tokens) {
    let width = ui.available_width().max(1.0);
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(width, TABLE_HEADER_HEIGHT), Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            "Preflight issue table columns",
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
    });
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );
    let font = theme::mono(tokens::FS_0, FontWeight::Medium);
    for (index, heading) in ISSUE_TABLE_HEADERS.into_iter().enumerate() {
        let cell = geometry.cell(rect, index);
        let clip = cell.shrink2(Vec2::new(TABLE_CELL_PADDING_X, 0.0));
        let galley =
            ui.painter()
                .layout_no_wrap(heading.to_uppercase(), font.clone(), t.color.text_faint);
        ui.painter().with_clip_rect(clip).galley(
            Pos2::new(clip.left(), rect.center().y - galley.size().y * 0.5),
            galley,
            t.color.text_faint,
        );
    }
}

fn wide_clean_row(ui: &mut Ui, geometry: IssueTableGeometry, t: &Tokens) {
    let values = [EMPTY_CELL, CLEAN_CHECK, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL];
    let tones = [
        t.color.text_dim,
        t.color.ok,
        t.color.text_dim,
        t.color.text_dim,
        t.color.text_dim,
    ];
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let galleys = std::array::from_fn::<_, 5, _>(|index| {
        let cell_width = (geometry.column_edges[index + 1]
            - geometry.column_edges[index]
            - TABLE_CELL_PADDING_X * 2.0)
            .max(1.0);
        ui.painter().layout(
            values[index].to_owned(),
            font.clone(),
            tones[index],
            cell_width,
        )
    });
    let height = (galleys
        .iter()
        .map(|galley| galley.size().y)
        .fold(0.0_f32, f32::max)
        + TABLE_CELL_PADDING_Y * 2.0)
        .max(t.metrics.row_h);
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(1.0), height),
        Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), CLEAN_CHECK)
    });
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );
    for (index, galley) in galleys.into_iter().enumerate() {
        paint_table_galley(ui, geometry.cell(rect, index), galley, tones[index]);
    }
}

fn summary_heading(report: &PreflightReport) -> String {
    if report.is_runnable() {
        RUNNABLE_HEADING.to_owned()
    } else {
        format!(
            "{} blocking issue{}",
            report.blockers.len(),
            if report.blockers.len() == 1 { "" } else { "s" }
        )
    }
}

fn wide_issue_row(
    ui: &mut Ui,
    geometry: IssueTableGeometry,
    index: usize,
    issue: &PreflightIssue,
    requested_fix: &mut Option<PreflightRemediation>,
    t: &Tokens,
) {
    let widths = std::array::from_fn::<_, 5, _>(|column| {
        (geometry.column_edges[column + 1]
            - geometry.column_edges[column]
            - TABLE_CELL_PADDING_X * 2.0)
            .max(1.0)
    });
    let order = format!("{:02}", index + 1);
    let order_galley = ui.painter().layout(
        order,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_faint,
        widths[0],
    );
    let check_galley = ui.painter().layout(
        issue.check.clone(),
        theme::sans(tokens::FS_1, FontWeight::SemiBold),
        t.color.text,
        widths[1],
    );
    let observed_galley = ui.painter().layout(
        issue.observed.clone(),
        theme::sans(tokens::FS_1, FontWeight::Regular),
        t.color.err,
        widths[2],
    );
    let required_galley = ui.painter().layout(
        issue.required.clone(),
        theme::sans(tokens::FS_1, FontWeight::Regular),
        t.color.text_dim,
        widths[3],
    );
    let action_label = remediation_label(issue.remediation);
    let action_galley = ui.painter().layout(
        action_label.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text,
        (widths[4] - 20.0).max(1.0),
    );
    let text_height = [
        order_galley.size().y,
        check_galley.size().y,
        observed_galley.size().y,
        required_galley.size().y,
    ]
    .into_iter()
    .fold(0.0_f32, f32::max);
    let button_height = t.metrics.ctl_h.max(action_galley.size().y + 8.0);
    let height = (text_height + TABLE_CELL_PADDING_Y * 2.0)
        .max(button_height + 8.0)
        .max(t.metrics.row_h);
    let width = ui.available_width().max(1.0);
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            format!(
                "Issue {}. {}. Observed: {}. Required: {}.",
                index + 1,
                issue.check,
                issue.observed,
                issue.required
            ),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
    });
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );
    paint_table_galley(ui, geometry.cell(rect, 0), order_galley, t.color.text_faint);
    paint_table_galley(ui, geometry.cell(rect, 1), check_galley, t.color.text);
    paint_table_galley(ui, geometry.cell(rect, 2), observed_galley, t.color.err);
    paint_table_galley(
        ui,
        geometry.cell(rect, 3),
        required_galley,
        t.color.text_dim,
    );

    let action_cell = geometry
        .cell(rect, 4)
        .shrink2(Vec2::new(TABLE_CELL_PADDING_X, 4.0));
    let button_rect = Rect::from_min_size(
        Pos2::new(
            action_cell.left(),
            action_cell.center().y - button_height * 0.5,
        ),
        Vec2::new(action_cell.width(), button_height),
    );
    let mut action_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(button_rect)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    let accessible = format!("{action_label} for blocker: {}", issue.observed);
    if Button::new(action_label)
        .max_width(action_cell.width())
        .accessible_label(&accessible)
        .show(&mut action_ui)
        .clicked()
    {
        *requested_fix = Some(issue.remediation);
    }
}

fn paint_table_galley(ui: &Ui, cell: Rect, galley: std::sync::Arc<egui::Galley>, color: Color32) {
    let clip = cell.shrink2(Vec2::new(TABLE_CELL_PADDING_X, 0.0));
    ui.painter().with_clip_rect(clip).galley(
        Pos2::new(clip.left(), cell.center().y - galley.size().y * 0.5),
        galley,
        color,
    );
}

fn compact_issue_list(
    ui: &mut Ui,
    report: &PreflightReport,
    requested_fix: &mut Option<PreflightRemediation>,
) {
    let t = Tokens::get(ui.ctx());
    if report.blockers.is_empty() {
        compact_clean_row(ui, &t);
        return;
    }

    for (index, issue) in report.blockers.iter().enumerate() {
        let width = ui.available_width().max(1.0);
        let record = Frame::new()
            .fill(t.color.bg_app)
            .inner_margin(Margin::same(10))
            .show(ui, |ui| {
                ui.set_width((width - 20.0).max(1.0));
                compact_field(ui, ISSUE_TABLE_HEADERS[0], &format!("{:02}", index + 1), &t);
                compact_field(ui, ISSUE_TABLE_HEADERS[1], &issue.check, &t);
                compact_field_with_tone(
                    ui,
                    ISSUE_TABLE_HEADERS[2],
                    &issue.observed,
                    t.color.err,
                    &t,
                );
                compact_field(ui, ISSUE_TABLE_HEADERS[3], &issue.required, &t);
                ui.label(
                    egui::RichText::new(ISSUE_TABLE_HEADERS[4].to_uppercase())
                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                        .color(t.color.text_faint),
                );
                let action_label = remediation_label(issue.remediation);
                let accessible = format!("{action_label} for blocker: {}", issue.observed);
                if Button::new(action_label)
                    .max_width(ui.available_width())
                    .accessible_label(&accessible)
                    .show(ui)
                    .clicked()
                {
                    *requested_fix = Some(issue.remediation);
                }
            });
        ui.painter().hline(
            record.response.rect.x_range(),
            record.response.rect.bottom() - 0.5,
            Stroke::new(1.0, t.color.border),
        );
    }
}

fn compact_clean_row(ui: &mut Ui, t: &Tokens) {
    let width = ui.available_width().max(1.0);
    let record = Frame::new()
        .fill(t.color.bg_app)
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.set_width((width - 20.0).max(1.0));
            compact_field(ui, ISSUE_TABLE_HEADERS[0], EMPTY_CELL, t);
            compact_field_with_tone(ui, ISSUE_TABLE_HEADERS[1], CLEAN_CHECK, t.color.ok, t);
            compact_field(ui, ISSUE_TABLE_HEADERS[2], EMPTY_CELL, t);
            compact_field(ui, ISSUE_TABLE_HEADERS[3], EMPTY_CELL, t);
            compact_field(ui, ISSUE_TABLE_HEADERS[4], EMPTY_CELL, t);
        });
    ui.painter().hline(
        record.response.rect.x_range(),
        record.response.rect.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );
}

fn compact_field(ui: &mut Ui, label: &str, value: &str, t: &Tokens) {
    compact_field_with_tone(ui, label, value, t.color.text_dim, t);
}

fn compact_field_with_tone(ui: &mut Ui, label: &str, value: &str, tone: Color32, t: &Tokens) {
    ui.label(
        egui::RichText::new(label.to_uppercase())
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text_faint),
    );
    ui.add(
        egui::Label::new(
            egui::RichText::new(value)
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(tone),
        )
        .wrap(),
    );
    ui.add_space(5.0);
}

fn report_context(ui: &mut Ui, report: &PreflightReport) {
    let t = Tokens::get(ui.ctx());
    let Some(prepared) = report.prepared.as_ref() else {
        let panel = context_panel(ui, "Advisories", |ui| advisory_rows(ui, report));
        ui.painter().hline(
            panel.x_range(),
            panel.bottom() - 0.5,
            Stroke::new(1.0, t.color.border_strong),
        );
        return;
    };
    let layout =
        PreflightBodyLayout::resolve(ui.available_width(), ui.ctx().content_rect().width()).context;
    match layout {
        ContextLayout::Split => {
            let width = ui.available_width().max(1.0);
            let panel_width = width * 0.5;
            let split = ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                let left = ui.vertical(|ui| {
                    ui.set_width(panel_width);
                    context_panel(ui, "Advisories", |ui| advisory_rows(ui, report))
                });
                let right = ui.vertical(|ui| {
                    ui.set_width((width - panel_width).max(1.0));
                    context_panel(ui, "Frozen dispatch contract", |ui| {
                        frozen_dispatch_rows(ui, report, prepared)
                    })
                });
                (left.response.rect, right.response.rect)
            });
            let divider_x = split.inner.0.right() - 0.5;
            ui.painter().vline(
                divider_x,
                split.response.rect.y_range(),
                Stroke::new(1.0, t.color.border_strong),
            );
            ui.painter().hline(
                split.response.rect.x_range(),
                split.response.rect.bottom() - 0.5,
                Stroke::new(1.0, t.color.border_strong),
            );
        }
        ContextLayout::Stacked => {
            let advisories = context_panel(ui, "Advisories", |ui| advisory_rows(ui, report));
            ui.painter().hline(
                advisories.x_range(),
                advisories.bottom() - 0.5,
                Stroke::new(1.0, t.color.border_strong),
            );
            let contract = context_panel(ui, "Frozen dispatch contract", |ui| {
                frozen_dispatch_rows(ui, report, prepared)
            });
            ui.painter().hline(
                contract.x_range(),
                contract.bottom() - 0.5,
                Stroke::new(1.0, t.color.border_strong),
            );
        }
    }
}

fn context_panel(ui: &mut Ui, title: &str, body: impl FnOnce(&mut Ui)) -> Rect {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .fill(t.color.bg_app)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            let (header, response) =
                ui.allocate_exact_size(Vec2::new(ui.available_width(), 29.0), Sense::hover());
            response.widget_info(|| {
                egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), title)
            });
            ui.ctx().accesskit_node_builder(response.id, |node| {
                node.set_role(egui::accesskit::Role::Label);
                node.set_label(title);
            });
            ui.painter().rect_filled(header, 0.0, t.color.bg_panel_2);
            ui.painter().text(
                header.left_center() + Vec2::new(10.0, 0.0),
                Align2::LEFT_CENTER,
                title.to_uppercase(),
                theme::mono(tokens::FS_0, FontWeight::SemiBold),
                t.color.text_dim,
            );
            Frame::new().inner_margin(Margin::same(10)).show(ui, body);
        })
        .response
        .rect
}

fn advisory_rows(ui: &mut Ui, report: &PreflightReport) {
    let t = Tokens::get(ui.ctx());
    if report.advisories.is_empty() {
        ui.add_space(6.0);
        ui.add(
            egui::Label::new(
                egui::RichText::new("No non-blocking advisories.")
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.text_dim),
            )
            .wrap(),
        );
        return;
    }
    for advisory in &report.advisories {
        let width = ui.available_width().max(1.0);
        let text_width = (width - 23.0).max(1.0);
        let galley = ui.painter().layout(
            advisory.clone(),
            theme::sans(tokens::FS_1, FontWeight::Regular),
            t.color.text_dim,
            text_width,
        );
        let height = (galley.size().y + 12.0).max(29.0);
        let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), advisory)
        });
        ui.painter().circle_filled(
            Pos2::new(rect.left() + 6.0, rect.top() + 12.0),
            1.5,
            t.color.text_dim,
        );
        let text_rect = Rect::from_min_max(
            Pos2::new(rect.left() + 17.0, rect.top() + 6.0),
            Pos2::new(rect.right() - 6.0, rect.bottom()),
        );
        ui.painter().with_clip_rect(text_rect).galley(
            text_rect.left_top(),
            galley,
            t.color.text_dim,
        );
        ui.painter().hline(
            rect.x_range(),
            rect.bottom() - 0.5,
            Stroke::new(1.0, t.color.border),
        );
    }
}

fn frozen_dispatch_rows(
    ui: &mut Ui,
    report: &PreflightReport,
    prepared: &PreparedPreflightContract,
) {
    property_row(
        ui,
        FROZEN_DISPATCH_ROWS[0],
        &report.project_revision.to_string(),
    );
    property_row(
        ui,
        FROZEN_DISPATCH_ROWS[1],
        &format!(
            "{} stable instance{}",
            prepared.analysis_ids.len(),
            if prepared.analysis_ids.len() == 1 {
                ""
            } else {
                "s"
            }
        ),
    );
    property_row(
        ui,
        FROZEN_DISPATCH_ROWS[2],
        &prepared.pvt_point_count.to_string(),
    );
    property_row(
        ui,
        FROZEN_DISPATCH_ROWS[3],
        &prepared.task_count.to_string(),
    );
    property_row(ui, FROZEN_DISPATCH_ROWS[4], prepared.target);
}

fn remediation_label(remediation: PreflightRemediation) -> &'static str {
    match remediation {
        PreflightRemediation::DesignChecks => "Run source checks",
        PreflightRemediation::SimulationPlan => "Open plan",
        PreflightRemediation::ProjectTechnology => "Attach technology",
    }
}

fn apply_remediation(app: &mut RSpiceApp, remediation: PreflightRemediation) {
    match remediation {
        PreflightRemediation::DesignChecks => {
            Command::RunChecks.execute(app);
            Command::VerificationPage(VerificationPage::Yield).execute(app);
        }
        PreflightRemediation::SimulationPlan => {
            Command::OpenWorkspace(Workspace::Simulate).execute(app);
        }
        PreflightRemediation::ProjectTechnology => {
            Command::ProjectPage(ProjectPage::Dependencies).execute(app);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn blocker_report(observed: &str) -> PreflightReport {
        PreflightReport {
            project_revision: 7,
            topology_root: "user/top/schematic".to_owned(),
            topology_revision: 11,
            topology_closure: vec![("user/top/schematic".to_owned(), 11)],
            simulation_plan_id: None,
            simulation_plan_revision: None,
            blockers: vec![PreflightIssue {
                check: "Source and netlist currentness".to_owned(),
                observed: observed.to_owned(),
                required: "A current validated input with an exact source closure".to_owned(),
                remediation: PreflightRemediation::DesignChecks,
            }],
            advisories: Vec::new(),
            prepared: None,
        }
    }

    fn action_bounds(width: f32) -> egui::accesskit::Rect {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(width, 900.0))),
            ..egui::RawInput::default()
        };
        let report = blocker_report(
            "Generated input revision differs from the validated dependency closure and must wrap safely.",
        );
        let output = ctx.run_ui(input, |ctx| {
            egui::CentralPanel::default()
                .frame(Frame::new())
                .show(ctx, |ui| {
                    let mut requested_fix = None;
                    blocker_list(ui, &report, &mut requested_fix);
                });
        });
        output
            .platform_output
            .accesskit_update
            .expect("AccessKit update")
            .nodes
            .iter()
            .find_map(|(_, node)| {
                (node.role() == egui::accesskit::Role::Button
                    && node
                        .label()
                        .is_some_and(|label| label.starts_with("Run source checks for blocker:")))
                .then(|| node.bounds())
                .flatten()
            })
            .expect("preflight remediation button bounds")
    }

    fn prepared_contract() -> PreparedPreflightContract {
        PreparedPreflightContract {
            snapshot_digest: crate::product::ContentDigest::from_bytes([1; 32]),
            source_digest: crate::product::ContentDigest::from_bytes([2; 32]),
            receipt_digest: crate::product::ContentDigest::from_bytes([3; 32]),
            receipt_label: "receipt",
            analysis_ids: vec![crate::product::ContentDigest::from_bytes([4; 32])],
            task_count: 1,
            saved_output_contract_count: 0,
            pvt_point_count: 1,
            target: "Desktop background thread",
            save_policy: "Retain engine-produced results",
            model_identity_count: 1,
        }
    }

    fn disable_global_process_axis(state: &mut AppState) {
        for dimension in &mut state.sim_setup.run_set.dimensions {
            if dimension.kind == crate::simulation::run_set::RunSetDimensionKind::ProcessSection {
                dimension.enabled = false;
            }
        }
    }

    #[test]
    fn run_command_remains_actionable_and_surfaces_preflight_blockers() {
        let mut app = RSpiceApp::test_instance();
        app.state.schematic = crate::state::SchematicState::default();

        assert!(Command::RunSimulation.is_enabled(&app));
        Command::RunSimulation.execute(&mut app);

        // Run records the request rather than calling the workflow, so that
        // the command vocabulary does not have to name a module above it.
        // `frame` serves it in the same pass; this is that hop.
        assert!(
            app.state.workbench.preflight.take_run_and_queue_request(),
            "Run must record a preflight-and-queue request for the frame loop"
        );
        run_and_queue(&mut app);

        assert!(!app.state.simulation.trigger_simulation);
        let report = app
            .state
            .workbench
            .preflight
            .report
            .as_ref()
            .expect("Run retains the same blocked report as explicit preflight");
        assert!(!report.blockers.is_empty());
        assert!(app.state.workbench.preflight.open);
        let toast = app
            .state
            .workbench
            .preflight
            .pending_toast
            .as_ref()
            .expect("blocked preflight retains its notification");
        assert!(
            toast.message.contains("design revision"),
            "preflight must distinguish the input-design revision from the simulation-plan revision: {}",
            toast.message
        );
    }

    #[test]
    fn a_current_retained_report_queues_without_reauthoring_preflight() {
        let mut app = RSpiceApp::test_instance();
        let (topology_root, topology_revision, topology_closure) =
            app.state.configured_topology_revision();
        let current_plan = app.state.active_plan_revision();
        app.state.workbench.preflight.report = Some(PreflightReport {
            project_revision: app.state.workspace.project.revision().get(),
            topology_root,
            topology_revision,
            topology_closure,
            simulation_plan_id: current_plan.map(|(id, _)| id),
            simulation_plan_revision: current_plan.map(|(_, revision)| revision),
            blockers: Vec::new(),
            advisories: Vec::new(),
            prepared: Some(prepared_contract()),
        });

        assert!(queue_retained_run(&mut app));
        assert!(app.state.simulation.trigger_simulation);
        assert_eq!(app.state.workbench.workspace, Workspace::Simulate);
    }

    #[test]
    fn preflight_uses_the_mockup_workflow_geometry_and_local_breakpoints() {
        assert_eq!(PREFLIGHT_DIALOG_SIZE, DialogSize::SimulationWorkflow);
        assert_eq!(
            PreflightBodyLayout::resolve(760.0, 1_440.0),
            PreflightBodyLayout {
                issues: IssueLayout::Table,
                context: ContextLayout::Split,
            }
        );
        assert_eq!(
            PreflightBodyLayout::resolve(760.0, 760.0),
            PreflightBodyLayout {
                issues: IssueLayout::Table,
                context: ContextLayout::Stacked,
            }
        );
        assert_eq!(
            PreflightBodyLayout::resolve(390.0, 390.0),
            PreflightBodyLayout {
                issues: IssueLayout::Records,
                context: ContextLayout::Stacked,
            }
        );
    }

    #[test]
    fn advisories_header_is_full_bleed_while_only_its_body_is_inset() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(400.0, 160.0))),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(Frame::NONE)
                    .show(ctx, |ui| {
                        context_panel(ui, "Advisories", |ui| {
                            ui.label("Body content");
                        });
                    });
            },
        );
        let header = output
            .platform_output
            .accesskit_update
            .expect("accessibility update")
            .nodes
            .iter()
            .find_map(|(_, node)| {
                (node.role() == egui::accesskit::Role::Label && node.label() == Some("Advisories"))
                    .then(|| node.bounds())
                    .flatten()
            })
            .expect("advisories header bounds");

        assert!(header.x0 <= 0.5, "header left edge was inset: {header:?}");
        assert!(
            header.x1 >= 399.5,
            "header right edge was inset: {header:?}"
        );
    }

    #[test]
    fn issue_table_regions_are_contiguous_and_non_overlapping_at_the_boundary() {
        let geometry = IssueTableGeometry::resolve(680.0, 104.0);
        assert_eq!(geometry.column_edges[0], 0.0);
        assert!((geometry.column_edges[5] - 680.0).abs() < f32::EPSILON);
        assert!(
            geometry
                .column_edges
                .windows(2)
                .all(|edges| edges[0] < edges[1])
        );
        for cells in geometry.column_edges.windows(3) {
            assert!(cells[1] >= cells[0] && cells[2] >= cells[1]);
        }
    }

    #[test]
    fn remediation_action_stays_inside_wide_table_and_phone_record_surfaces() {
        for width in [680.0_f32, 390.0] {
            let bounds = action_bounds(width);
            assert!(
                bounds.x0 >= 0.0,
                "button started outside {width}: {bounds:?}"
            );
            assert!(
                bounds.x1 <= f64::from(width) + 0.5,
                "button overflowed {width}: {bounds:?}"
            );
            assert!(bounds.x1 > bounds.x0 && bounds.y1 > bounds.y0);
        }
    }

    #[test]
    fn report_collects_all_independent_blocker_classes() {
        let mut state = AppState::default();
        disable_global_process_axis(&mut state);
        state.schematic = crate::state::SchematicState::default();
        let plan = state
            .sim_setup
            .analysis_plan
            .as_mut()
            .expect("current project owns a stable plan");
        let transient_id = plan.instances()[0].id();
        plan.set_enabled(transient_id, false)
            .expect("the sole analysis disables");
        crate::workbench::menu_bar::run_design_rule_check(&mut state);

        let report = collect_report(&state);
        let (topology_root, topology_revision, topology_closure) =
            state.configured_topology_revision();

        assert!(!report.is_runnable_for(
            state.workspace.project.revision().get(),
            &topology_root,
            topology_revision,
            &topology_closure,
            state.active_plan_revision(),
        ));
        assert!(
            report
                .blockers
                .iter()
                .any(|issue| issue.check == "Design topology")
        );
        assert!(report.blockers.iter().any(|issue| {
            issue.check == "Analysis-instance graph"
                && issue.observed == "Enable at least one analysis instance."
                && issue.remediation == PreflightRemediation::SimulationPlan
        }));
        assert!(
            !report
                .blockers
                .iter()
                .any(|issue| issue.check == "Project technology contract"),
            "a typical plan demands no technology: {:?}",
            report.blockers
        );
        assert!(
            report
                .advisories
                .iter()
                .any(|advisory| advisory == TECHNOLOGY_NOT_REQUIRED_ADVISORY),
            "{:?}",
            report.advisories
        );
    }

    #[test]
    fn a_non_typical_reference_process_is_the_only_row_that_owns_the_missing_section() {
        let mut state = AppState::default();
        disable_global_process_axis(&mut state);
        state
            .sim_setup
            .set_reference_pvt(crate::product::ProcessCorner::SS, 27.0)
            .expect("the reference point is valid");

        let report = collect_report(&state);

        let technology: Vec<&PreflightIssue> = report
            .blockers
            .iter()
            .filter(|issue| issue.check == "Project technology contract")
            .collect();
        assert_eq!(technology.len(), 1, "{technology:?}");
        assert!(technology[0].observed.contains("SS"), "{technology:?}");
        assert_eq!(
            technology[0].remediation,
            PreflightRemediation::ProjectTechnology
        );
        assert!(
            !report
                .blockers
                .iter()
                .any(|issue| issue.check == "Reference model binding"),
            "the demand row already owns the missing reference section: {:?}",
            report.blockers
        );
    }

    #[test]
    fn an_enabled_corner_analysis_names_its_instance_and_sections_in_its_own_row() {
        let mut state = AppState::default();
        disable_global_process_axis(&mut state);
        let mut corner = crate::simulation::dialog::corner::CornerDialogState::default();
        for dimension in &mut corner.run_set.dimensions {
            if dimension.kind == crate::simulation::run_set::RunSetDimensionKind::Supply {
                dimension.source = format!(
                    "{}VDD",
                    crate::simulation::run_set::NETLIST_SUPPLY_SOURCE_PREFIX
                );
            }
        }
        let plan = state
            .sim_setup
            .stable_analysis_plan_mut()
            .expect("a default state owns a stable plan");
        let position = plan.instances().len();
        let (instance, _) = plan
            .insert_draft_with_id(
                crate::product::AnalysisInstanceId::new(),
                crate::simulation::plan::AnalysisDraft::Corner(corner),
                true,
                position,
            )
            .expect("a corner analysis has no prerequisites");

        let report = collect_report(&state);

        let technology = report
            .blockers
            .iter()
            .find(|issue| issue.check == "Project technology contract")
            .expect("an enabled corner analysis demands its non-typical sections");
        assert!(technology.observed.contains("SS, FF"), "{technology:?}");
        assert!(
            technology.observed.contains(&instance.to_string()),
            "{technology:?}"
        );
        assert_eq!(
            technology.remediation,
            PreflightRemediation::ProjectTechnology
        );
    }

    #[test]
    fn an_unaudited_technology_binding_blocks_on_reattachment_without_faking_drift() {
        let mut state = AppState::default();
        state.provision_test_project_technology_contract();
        // A project copy retains the exact binding and starts a fresh audit
        // history, which is precisely the binding-without-receipts state.
        state.workspace.project = state
            .workspace
            .project
            .fork_copy_at(std::path::PathBuf::from("preflight_copy.rspiceproj"));
        assert!(state.workspace.project.technology_binding().is_some());
        assert!(state.workspace.project.technology_change_audit().is_empty());

        let report = collect_report(&state);

        let technology = report
            .blockers
            .iter()
            .find(|issue| issue.check == "Project technology contract")
            .expect("a binding without receipts blocks preflight");
        assert_eq!(
            technology.observed,
            "The attached technology binding predates checkpoint-backed authority receipts"
        );
        assert_eq!(
            technology.required,
            "Reattach the technology to record an audited change receipt"
        );
        assert_eq!(
            technology.remediation,
            PreflightRemediation::ProjectTechnology
        );
        assert!(
            !report.blockers.iter().any(|issue| {
                issue.check == "Project model technology" || issue.check == "Project signed PDK"
            }),
            "the retained binding still resolves exactly: {:?}",
            report.blockers
        );
    }

    #[test]
    fn a_valid_attached_technology_reports_no_technology_row_at_all() {
        let mut state = AppState::default();
        state.provision_test_project_technology_contract();

        let report = collect_report(&state);

        assert!(
            !report.blockers.iter().any(|issue| {
                issue.check == "Project technology contract"
                    || issue.check == "Project model technology"
                    || issue.check == "Project signed PDK"
            }),
            "a valid contract is silent: {:?}",
            report.blockers
        );
        assert!(
            !report
                .advisories
                .iter()
                .any(|advisory| advisory == TECHNOLOGY_NOT_REQUIRED_ADVISORY),
            "{:?}",
            report.advisories
        );
    }

    #[test]
    fn report_is_bound_to_the_exact_project_topology_and_plan_revision() {
        let mut state = AppState::default();
        crate::workbench::menu_bar::run_design_rule_check(&mut state);

        let report = collect_report(&state);

        assert_eq!(
            report.project_revision,
            state.workspace.project.revision().get()
        );
        let (plan_id, plan_revision) = state.active_plan_revision().expect("active plan");
        let (topology_root, topology_revision, topology_closure) =
            state.configured_topology_revision();
        assert_eq!(report.topology_root, topology_root);
        assert_eq!(report.topology_revision, topology_revision);
        assert_eq!(report.topology_closure, topology_closure);
        assert_eq!(report.simulation_plan_id, Some(plan_id));
        assert_eq!(report.simulation_plan_revision, Some(plan_revision));
        assert!(report.is_current_for(
            state.workspace.project.revision().get(),
            &topology_root,
            topology_revision,
            &topology_closure,
            Some((plan_id, plan_revision)),
        ));

        let transient_id = state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .instances()[0]
            .id();
        state
            .sim_setup
            .stable_analysis_plan_mut()
            .expect("stable plan")
            .edit(transient_id, |_| ())
            .expect("analysis edit advances the plan revision");
        assert!(!report.is_current_for(
            state.workspace.project.revision().get(),
            &topology_root,
            topology_revision,
            &topology_closure,
            state.active_plan_revision(),
        ));
    }

    #[test]
    fn report_currentness_tracks_the_configured_root_not_an_unrelated_active_editor() {
        let mut state = AppState::default();
        let configured_root = state.workspace.simulation_root_reference();
        let configured_root_key = configured_root.key();
        let configured_schematic = state
            .workspace
            .schematic_buffers
            .get_mut(&configured_root_key)
            .expect("default configured root buffer");
        configured_schematic.add_component(
            crate::state::ComponentType::Ground,
            crate::state::Point::new(40, 40),
        );
        let configured_revision = configured_schematic.topology_version();

        state.workspace.active_view =
            crate::state::CellViewRef::new("user", "unrelated_editor", "schematic");
        state.schematic = crate::state::SchematicState::default();
        state.schematic.add_component(
            crate::state::ComponentType::Resistor,
            crate::state::Point::new(80, 80),
        );

        let report = collect_report(&state);
        assert_eq!(report.topology_root, configured_root_key);
        assert_eq!(report.topology_revision, configured_revision);

        state.schematic.add_component(
            crate::state::ComponentType::Capacitor,
            crate::state::Point::new(120, 80),
        );
        let (live_root, live_revision, live_closure) = state.configured_topology_revision();
        assert!(report.is_current_for(
            state.workspace.project.revision().get(),
            &live_root,
            live_revision,
            &live_closure,
            state.active_plan_revision(),
        ));
    }

    #[test]
    fn report_currentness_expires_when_a_referenced_child_topology_changes() {
        let mut state = AppState::default();
        let child = crate::state::CellViewRef::new("user", "child", "schematic");
        state.schematic.add_library_cell_component(
            crate::state::Point::new(40, 40),
            crate::state::LibraryCellInstance::new("user", "child", "schematic"),
        );
        let mut child_schematic = crate::state::SchematicState::default();
        child_schematic.add_component(
            crate::state::ComponentType::Ground,
            crate::state::Point::new(20, 20),
        );
        state
            .workspace
            .schematic_buffers
            .insert(child.key(), child_schematic.clone());

        let report = collect_report(&state);
        let root_revision = report.topology_revision;
        assert!(
            report
                .topology_closure
                .iter()
                .any(|(key, _)| key.eq_ignore_ascii_case(&child.key()))
        );

        // A real editor transition persists the departing active schematic
        // before activating the child view. Mirror that contract explicitly;
        // otherwise this synthetic state would replace the edited root with
        // the stale default buffer and test a transition the UI cannot make.
        state.workspace.schematic_buffers.insert(
            crate::state::CellViewRef::default_top().key(),
            state.schematic.clone(),
        );
        state.workspace.active_view = child;
        state.schematic = child_schematic;
        state.schematic.add_component(
            crate::state::ComponentType::Resistor,
            crate::state::Point::new(80, 20),
        );
        let (live_root, live_revision, live_closure) = state.configured_topology_revision();
        assert_eq!(live_revision, root_revision);
        assert!(!report.is_current_for(
            state.workspace.project.revision().get(),
            &live_root,
            live_revision,
            &live_closure,
            state.active_plan_revision(),
        ));
    }

    #[test]
    fn unresolved_hierarchy_is_an_ordered_preflight_blocker() {
        let mut state = AppState::default();
        state.schematic.add_library_cell_component(
            crate::state::Point::new(20, 20),
            crate::state::LibraryCellInstance::new(
                "missing_library",
                "missing_master",
                "schematic",
            ),
        );
        assert!(
            state
                .workspace
                .schematic_buffers
                .get(&crate::state::CellViewRef::default_top().key())
                .expect("persisted root schematic")
                .components
                .is_empty(),
            "fixture must remain unsynchronized to exercise the live overlay"
        );
        crate::workbench::menu_bar::run_design_rule_check(&mut state);

        let report = collect_report(&state);

        let hierarchy = report
            .blockers
            .iter()
            .find(|issue| issue.check == "Hierarchy binding")
            .expect("unbound master blocks preflight");
        assert!(
            hierarchy
                .observed
                .contains("missing_library/missing_master")
        );
        assert_eq!(hierarchy.remediation, PreflightRemediation::DesignChecks);
    }

    #[test]
    fn visible_mockup_labels_are_exact_and_do_not_expose_internal_contract_rows() {
        assert_eq!(
            ISSUE_TABLE_HEADERS,
            ["Order", "Check", "Observed", "Required", "Action"]
        );
        assert_eq!(CLEAN_CHECK, "No blocking issues.");
        assert_eq!(EMPTY_CELL, "—");
        assert_eq!(
            FROZEN_DISPATCH_ROWS,
            [
                "Revision",
                "Analysis identities",
                "PVT points",
                "Tasks",
                "Target"
            ]
        );
        assert_eq!(
            remediation_label(PreflightRemediation::DesignChecks),
            "Run source checks"
        );
        assert_eq!(
            remediation_label(PreflightRemediation::SimulationPlan),
            "Open plan"
        );
        assert_eq!(
            remediation_label(PreflightRemediation::ProjectTechnology),
            "Attach technology"
        );
    }

    #[test]
    fn summary_copy_and_blocker_action_mapping_match_the_mockup_contract() {
        let runnable = PreflightReport {
            project_revision: 7,
            topology_root: "user/top/schematic".to_owned(),
            topology_revision: 11,
            topology_closure: vec![("user/top/schematic".to_owned(), 11)],
            simulation_plan_id: None,
            simulation_plan_revision: None,
            blockers: Vec::new(),
            advisories: Vec::new(),
            prepared: Some(prepared_contract()),
        };
        assert_eq!(summary_heading(&runnable), RUNNABLE_HEADING);
        assert_eq!(
            RUNNABLE_SUMMARY,
            "Immutable inputs, target, task graph, and save policy are ready for dispatch."
        );
        assert_eq!(
            BLOCKED_SUMMARY,
            "The run was not queued. Resolve the ordered issues below, then rerun preflight."
        );

        let issue = |check: &str| PreflightIssue {
            check: check.to_owned(),
            observed: "blocked".to_owned(),
            required: "fixed".to_owned(),
            remediation: PreflightRemediation::DesignChecks,
        };
        let one = PreflightReport {
            blockers: vec![issue("Source closure")],
            prepared: None,
            ..runnable.clone()
        };
        assert_eq!(summary_heading(&one), "1 blocking issue");
        let two = PreflightReport {
            blockers: vec![issue("Model bindings"), issue("Executable netlist")],
            prepared: None,
            ..runnable
        };
        assert_eq!(summary_heading(&two), "2 blocking issues");

        use crate::simulation::execution::PreparationStage;
        for stage in [
            PreparationStage::DesignChecks,
            PreparationStage::SourceChecks,
            PreparationStage::ModelBindings,
            PreparationStage::Netlist,
        ] {
            assert_eq!(
                preparation_remediation(stage),
                PreflightRemediation::DesignChecks
            );
        }
        for stage in [
            PreparationStage::AnalysisPlan,
            PreparationStage::Authorization,
        ] {
            assert_eq!(
                preparation_remediation(stage),
                PreflightRemediation::SimulationPlan
            );
        }
    }

    /// A configuration whose stop view names a schematic.
    ///
    /// A schematic is not a terminal implementation, so the resolver descends
    /// into it and records a warning on the otherwise resolved binding: the
    /// configuration is honoured differently than it reads.
    fn state_with_a_stop_view_warning() -> AppState {
        let mut state = AppState::default();
        let mut work = crate::state::Library::new("work");
        let mut amp = crate::state::Cell::new("amp");
        amp.add_view(crate::state::View::new(
            "schematic",
            crate::state::ViewType::Schematic,
        ));
        work.add_cell(amp);
        state.library_manager.add_library(work);

        let mut master = crate::state::SchematicState::default();
        master.add_component(
            crate::state::ComponentType::Resistor,
            crate::state::Point::new(30, 0),
        );
        state
            .workspace
            .schematic_buffers
            .insert("work/amp/schematic".to_owned(), master);

        let binding = crate::state::LibraryCellInstance::new("work", "amp", "schematic");
        let instance = state
            .schematic
            .add_library_cell_component(crate::state::Point::new(100, 0), binding);
        state
            .schematic
            .components
            .iter_mut()
            .find(|component| component.id == instance)
            .expect("the placed instance is retained")
            .name = "X1".to_owned();
        state.sync_active_schematic_to_workspace();

        let root = state.workspace.active_view.clone();
        state
            .workspace
            .configuration_sets
            .create(crate::state::ConfigurationSetDefinition {
                name: "Stop at a schematic".to_owned(),
                root,
                dut_path: "/X1".to_owned(),
                executable_view_policy: vec!["schematic".to_owned()],
                stop_views: vec!["schematic".to_owned()],
                unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "projection consumer test".to_owned(),
            })
            .expect("the fixture configuration is well formed");
        state
    }

    #[test]
    fn a_configuration_honoured_differently_than_it_reads_is_a_preflight_advisory() {
        let state = state_with_a_stop_view_warning();

        let report = collect_report(&state);

        assert!(
            report
                .advisories
                .iter()
                .any(|advisory| advisory.contains("stop view")),
            "preflight states the configuration warning before the run: {:?}",
            report.advisories
        );
    }
}
