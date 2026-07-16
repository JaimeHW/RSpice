//! Revision-bound simulation-preflight workflow from the workbench mockup.
//!
//! Validation is executed once, captured as an immutable report, and then
//! rendered without recomputing beneath the operator. Every blocker has an
//! explicit destination and a validated run can be queued from the report.

use egui::{Context, Frame, Margin, Stroke, Ui};

use crate::common::RSpiceApp;
use crate::common::app::{AppState, ConsoleMessage};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};

use super::commands::Command;
use super::design_system::{property_row, status_dot};
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
        .size(DialogSize::Manager)
        .hint(&hint)
        .show(ctx, |ui| {
            report_summary(ui, &report);
            ui.add_space(14.0);
            blocker_list(ui, &report, &mut requested_fix);
            ui.add_space(14.0);
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
    Frame::new()
        .fill(if runnable {
            t.color.ok.gamma_multiply(0.08)
        } else {
            t.color.err.gamma_multiply(0.08)
        })
        .stroke(Stroke::new(
            1.0,
            if runnable { t.color.ok } else { t.color.err },
        ))
        .corner_radius(t.radius)
        .inner_margin(Margin::same(12))
        .show(ui, |ui| {
            status_dot(
                ui,
                if runnable { t.color.ok } else { t.color.err },
                if runnable { "READY" } else { "BLOCKED" },
            );
            ui.label(
                egui::RichText::new(heading).font(theme::sans(tokens::FS_2, FontWeight::SemiBold)),
            );
            ui.label(
                egui::RichText::new(if runnable {
                    RUNNABLE_SUMMARY
                } else {
                    BLOCKED_SUMMARY
                })
                .color(t.color.text_dim),
            );
        });
}

fn blocker_list(
    ui: &mut Ui,
    report: &PreflightReport,
    requested_fix: &mut Option<PreflightRemediation>,
) {
    section_title(ui, "Ordered corrective action");
    if ui.available_width() < 680.0 {
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
    Frame::new()
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius)
        .inner_margin(Margin::symmetric(10, 7))
        .show(ui, |ui| {
            ui.columns(ISSUE_TABLE_HEADERS.len(), |columns| {
                for (column, heading) in columns.iter_mut().zip(ISSUE_TABLE_HEADERS) {
                    column.label(
                        egui::RichText::new(heading.to_uppercase())
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.text_faint),
                    );
                }
            });
        });
    ui.add_space(5.0);

    if report.blockers.is_empty() {
        wide_clean_row(ui, &t);
        return;
    }

    for (index, issue) in report.blockers.iter().enumerate() {
        wide_issue_row(ui, index, issue, requested_fix, &t);
        if index + 1 < report.blockers.len() {
            ui.add_space(5.0);
        }
    }
}

fn wide_clean_row(ui: &mut Ui, t: &Tokens) {
    Frame::new()
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius)
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.columns(ISSUE_TABLE_HEADERS.len(), |columns| {
                columns[0].label(EMPTY_CELL);
                columns[1].label(egui::RichText::new(CLEAN_CHECK).strong().color(t.color.ok));
                columns[2].label(EMPTY_CELL);
                columns[3].label(EMPTY_CELL);
                columns[4].label(EMPTY_CELL);
            });
        });
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
    index: usize,
    issue: &PreflightIssue,
    requested_fix: &mut Option<PreflightRemediation>,
    t: &Tokens,
) {
    Frame::new()
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius)
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.columns(ISSUE_TABLE_HEADERS.len(), |columns| {
                columns[0].label(
                    egui::RichText::new(format!("{:02}", index + 1))
                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                        .color(t.color.text_faint),
                );
                columns[1].label(egui::RichText::new(&issue.check).strong());
                columns[2].label(
                    egui::RichText::new(&issue.observed)
                        .color(t.color.err)
                        .size(tokens::FS_1),
                );
                columns[3].label(
                    egui::RichText::new(&issue.required)
                        .color(t.color.text_dim)
                        .size(tokens::FS_1),
                );
                if columns[4]
                    .button(remediation_label(issue.remediation))
                    .clicked()
                {
                    *requested_fix = Some(issue.remediation);
                }
            });
        });
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
        Frame::new()
            .fill(t.color.bg_inset)
            .stroke(Stroke::new(1.0, t.color.border))
            .corner_radius(t.radius)
            .inner_margin(Margin::same(10))
            .show(ui, |ui| {
                compact_field(ui, ISSUE_TABLE_HEADERS[0], &format!("{:02}", index + 1), &t);
                compact_field(ui, ISSUE_TABLE_HEADERS[1], &issue.check, &t);
                compact_field(ui, ISSUE_TABLE_HEADERS[2], &issue.observed, &t);
                compact_field(ui, ISSUE_TABLE_HEADERS[3], &issue.required, &t);
                ui.label(
                    egui::RichText::new(ISSUE_TABLE_HEADERS[4].to_uppercase())
                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                        .color(t.color.text_faint),
                );
                if ui.button(remediation_label(issue.remediation)).clicked() {
                    *requested_fix = Some(issue.remediation);
                }
            });
        if index + 1 < report.blockers.len() {
            ui.add_space(6.0);
        }
    }
}

fn compact_clean_row(ui: &mut Ui, t: &Tokens) {
    Frame::new()
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius)
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            compact_field(ui, ISSUE_TABLE_HEADERS[0], EMPTY_CELL, t);
            compact_field(ui, ISSUE_TABLE_HEADERS[1], CLEAN_CHECK, t);
            compact_field(ui, ISSUE_TABLE_HEADERS[2], EMPTY_CELL, t);
            compact_field(ui, ISSUE_TABLE_HEADERS[3], EMPTY_CELL, t);
            compact_field(ui, ISSUE_TABLE_HEADERS[4], EMPTY_CELL, t);
        });
}

fn compact_field(ui: &mut Ui, label: &str, value: &str, t: &Tokens) {
    ui.label(
        egui::RichText::new(label.to_uppercase())
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text_faint),
    );
    ui.label(value);
    ui.add_space(5.0);
}

fn report_context(ui: &mut Ui, report: &PreflightReport) {
    section_title(ui, "Advisories");
    if report.advisories.is_empty() {
        ui.label("No non-blocking advisories.");
    } else {
        for advisory in &report.advisories {
            ui.label(format!("• {advisory}"));
        }
    }

    ui.add_space(12.0);
    let Some(prepared) = report.prepared.as_ref() else {
        return;
    };
    section_title(ui, "Frozen dispatch contract");
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

fn section_title(ui: &mut Ui, title: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(title.to_uppercase())
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text_faint),
    );
    ui.add_space(5.0);
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
