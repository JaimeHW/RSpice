//! Revision-bound simulation-preflight workflow from the workbench mockup.
//!
//! Validation is executed once, captured as an immutable report, and then
//! rendered without recomputing beneath the operator. Every blocker has an
//! explicit destination and a validated run can be queued from the report.

use egui::{Align2, Color32, Context, Frame, Margin, Pos2, Rect, Sense, Stroke, Ui, Vec2};

use crate::common::RSpiceApp;
use crate::common::app::{AppState, ConsoleMessage};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize};

use super::commands::Command;
use super::design_system::property_row;
use super::state::{
    ConsolePage, PreflightIssue, PreflightRemediation, PreflightReport, PreflightToast,
    PreparedPreflightContract, VerificationPage, Workspace,
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
    crate::common::menu_bar::run_design_rule_check(&mut app.state);
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
    let blocked = !report.is_runnable();
    let message = if blocked {
        format!(
            "Preflight blocked · {} blocking issue{} · revision {} was not queued",
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
            "Preflight complete · 0 blocking errors · {} advisor{} · revision {} · {} task{}",
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

fn collect_report(state: &AppState) -> PreflightReport {
    let mut blockers = Vec::new();
    let mut advisories = Vec::new();

    if state.schematic.components.is_empty() {
        blockers.push(PreflightIssue {
            check: "Design topology".to_owned(),
            observed: "The active schematic contains no components.".to_owned(),
            required: "A non-empty circuit topology".to_owned(),
            remediation: PreflightRemediation::DesignChecks,
        });
    }

    let hierarchy = state.workspace.resolve_hierarchy_with_active(
        &state.library_manager,
        &state.workspace.active_view,
        &state.schematic,
    );
    for binding in hierarchy
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

    match state.dialogs.drc_results.as_ref() {
        Some(_) if state.dialogs.drc_checked_version != state.schematic.topology_version() => {
            blockers.push(PreflightIssue {
                check: "Schematic validation".to_owned(),
                observed: "The retained source-check receipt belongs to an earlier topology."
                    .to_owned(),
                required: "A completed validation result for this topology revision".to_owned(),
                remediation: PreflightRemediation::DesignChecks,
            });
        }
        Some(result) if result.completed => {
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
        }
        Some(_) => blockers.push(PreflightIssue {
            check: "Schematic validation".to_owned(),
            observed: "The schematic validator did not complete.".to_owned(),
            required: "A completed validation result for this topology revision".to_owned(),
            remediation: PreflightRemediation::DesignChecks,
        }),
        None => blockers.push(PreflightIssue {
            check: "Schematic validation".to_owned(),
            observed: "No schematic validation result was returned.".to_owned(),
            required: "A completed validation result for this topology revision".to_owned(),
            remediation: PreflightRemediation::DesignChecks,
        }),
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

    if let Err(error) = state
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

    PreflightReport {
        project_revision: state.workspace.project.revision().get(),
        topology_revision: state.schematic.topology_version(),
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

    // A report is valid only for the exact source revision it inspected.
    if report.project_revision != app.state.workspace.project.revision().get()
        || report.topology_revision != app.state.schematic.topology_version()
    {
        app.state.workbench.preflight.open = false;
        app.state.ui.toasts.warn_with_title(
            ctx,
            "Preflight report expired",
            "Preflight report expired because the design revision changed",
        );
        return;
    }

    let runnable = report.is_runnable();
    let primary = if runnable {
        "Queue validated run"
    } else {
        "Close"
    };
    let hint = format!(
        "Revision {} · {} blocking · {} advisory",
        report.project_revision,
        report.blockers.len(),
        report.advisories.len()
    );
    let kicker = format!(
        "SIMULATION · REVISION {} · ORDERED CORRECTIVE ACTION",
        report.project_revision
    );
    let mut requested_fix = None;
    let choice = Dialog::new(&kicker, "Simulation preflight", primary)
        .description(
            "Review ordered blockers, advisories, and frozen run inputs before closing or queuing this validated simulation revision.",
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
            app.state.workbench.preflight.open = false;
            Command::RunSimulation.execute(app);
        }
        DialogChoice::Primary | DialogChoice::Cancelled => {
            app.state.workbench.preflight.open = false;
        }
        DialogChoice::None | DialogChoice::Secondary | DialogChoice::Ghost => {}
    }
}

fn report_summary(ui: &mut Ui, report: &PreflightReport) {
    let t = Tokens::get(ui.ctx());
    let runnable = report.is_runnable();
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
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            let (header, response) =
                ui.allocate_exact_size(Vec2::new(ui.available_width(), 29.0), Sense::hover());
            response.widget_info(|| {
                egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), title)
            });
            ui.painter().rect_filled(header, 0.0, t.color.bg_panel_2);
            ui.painter().text(
                header.left_center() + Vec2::new(10.0, 0.0),
                Align2::LEFT_CENTER,
                title.to_uppercase(),
                theme::mono(tokens::FS_0, FontWeight::SemiBold),
                t.color.text_dim,
            );
            body(ui);
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn blocker_report(observed: &str) -> PreflightReport {
        PreflightReport {
            project_revision: 7,
            topology_revision: 11,
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
        let output = ctx.run(input, |ctx| {
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
        state.schematic = crate::state::SchematicState::default();
        let plan = state
            .sim_setup
            .analysis_plan
            .as_mut()
            .expect("current project owns a stable plan");
        let transient_id = plan.instances()[0].id();
        plan.set_enabled(transient_id, false)
            .expect("the sole analysis disables");
        crate::common::menu_bar::run_design_rule_check(&mut state);

        let report = collect_report(&state);

        assert!(!report.is_runnable());
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
    }

    #[test]
    fn report_is_bound_to_the_exact_project_and_topology_revision() {
        let mut state = AppState::default();
        crate::common::menu_bar::run_design_rule_check(&mut state);

        let report = collect_report(&state);

        assert_eq!(
            report.project_revision,
            state.workspace.project.revision().get()
        );
        assert_eq!(report.topology_revision, state.schematic.topology_version());
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
        crate::common::menu_bar::run_design_rule_check(&mut state);

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
    }

    #[test]
    fn summary_copy_and_blocker_action_mapping_match_the_mockup_contract() {
        let runnable = PreflightReport {
            project_revision: 7,
            topology_revision: 11,
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
}
