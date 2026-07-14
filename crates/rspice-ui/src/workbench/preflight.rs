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
    ConsolePage, ModelsPage, PreflightIssue, PreflightRemediation, PreflightReport, PreflightToast,
    VerificationPage, Workspace,
};

/// Run every local preflight check and retain the exact report for the current
/// project revision. A blocked report opens immediately; a clean report leaves
/// the operator on the current surface, matching the mockup workflow.
pub(crate) fn run(app: &mut RSpiceApp) {
    crate::common::menu_bar::run_design_rule_check(&mut app.state);
    let report = collect_report(&app.state);
    let blocked = !report.is_runnable();
    let message = if blocked {
        format!(
            "Preflight blocked · {} blocking issue{} · revision {} was not queued",
            report.blockers.len(),
            if report.blockers.len() == 1 { "" } else { "s" },
            report.project_revision
        )
    } else {
        format!(
            "Preflight complete · 0 blocking errors · {} advisor{} · revision {} · {} task{}",
            report.advisories.len(),
            if report.advisories.len() == 1 {
                "y"
            } else {
                "ies"
            },
            report.project_revision,
            report.task_count,
            if report.task_count == 1 { "" } else { "s" }
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

    match state.dialogs.drc_results.as_ref() {
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

    let ordered = state.sim_setup.ordered_enabled_indices();
    if ordered.is_empty() {
        blockers.push(PreflightIssue {
            check: "Run set".to_owned(),
            observed: "No analyses are enabled.".to_owned(),
            required: "At least one validated analysis".to_owned(),
            remediation: PreflightRemediation::SimulationPlan,
        });
    }
    for index in &ordered {
        if let Some(error) = state.sim_setup.validation_error(*index) {
            blockers.push(PreflightIssue {
                check: format!("{} configuration", analysis_name(*index)),
                observed: error,
                required: "A complete, numerically valid analysis configuration".to_owned(),
                remediation: PreflightRemediation::SimulationPlan,
            });
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
            remediation: PreflightRemediation::ModelBindings,
        });
    }

    let analysis_count = ordered.len();
    PreflightReport {
        project_revision: state.workspace.project.revision().get(),
        topology_revision: state.schematic.topology_version(),
        blockers,
        advisories,
        analysis_count,
        // The controller dispatches one ordered task per enabled analysis;
        // sweep/corner dimensions remain owned by that task's typed contract.
        task_count: analysis_count,
        reference_pvt: format!(
            "{} / {} °C",
            state.sim_setup.reference_pvt.process.short_name(),
            state.sim_setup.reference_pvt.temperature_celsius
        ),
        target: execution_target().to_owned(),
    }
}

fn execution_target() -> &'static str {
    #[cfg(target_arch = "wasm32")]
    {
        "Browser local WASM engine"
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        "Desktop local engine"
    }
}

fn analysis_name(index: usize) -> &'static str {
    crate::common::simulation_analysis_tabs::SIMULATION_ANALYSIS_CATEGORIES
        .iter()
        .flat_map(|(_, analyses)| analyses.iter())
        .find_map(|(candidate, name)| (*candidate == index).then_some(*name))
        .unwrap_or("Analysis")
}

/// Render the retained report above the workbench. This function also turns
/// the command's context-free notification into a real egui toast.
pub(crate) fn show(ctx: &Context, app: &mut RSpiceApp) {
    if let Some(toast) = app.state.workbench.preflight.pending_toast.take() {
        if toast.warning {
            app.state.ui.toasts.warn(ctx, toast.message);
        } else {
            app.state.ui.toasts.info(ctx, toast.message);
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
        app.state.ui.toasts.warn(
            ctx,
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
    let mut requested_fix = None;
    let choice = Dialog::new(
        "Simulation · ordered corrective action",
        "Simulation preflight",
        primary,
    )
    .size(DialogSize::Lg)
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
                egui::RichText::new(if runnable {
                    "Plan is runnable"
                } else {
                    "The run was not queued"
                })
                .font(theme::sans(tokens::FS_2, FontWeight::SemiBold)),
            );
            ui.label(
                egui::RichText::new(if runnable {
                    "Inputs, model bindings, analysis configurations, and schematic checks are ready for dispatch."
                } else {
                    "Resolve the ordered issues below, then run preflight again."
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
    if report.blockers.is_empty() {
        let t = Tokens::get(ui.ctx());
        status_dot(ui, t.color.ok, "No blocking issues");
        return;
    }

    for (index, issue) in report.blockers.iter().enumerate() {
        issue_row(ui, index, issue, requested_fix);
        if index + 1 < report.blockers.len() {
            ui.add_space(6.0);
        }
    }
}

fn issue_row(
    ui: &mut Ui,
    index: usize,
    issue: &PreflightIssue,
    requested_fix: &mut Option<PreflightRemediation>,
) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius)
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            let compact = ui.available_width() < 560.0;
            if compact {
                ui.vertical(|ui| {
                    issue_details(ui, index, issue, &t);
                    if ui.button(remediation_label(issue.remediation)).clicked() {
                        *requested_fix = Some(issue.remediation);
                    }
                });
            } else {
                ui.horizontal(|ui| {
                    issue_details(ui, index, issue, &t);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(remediation_label(issue.remediation)).clicked() {
                            *requested_fix = Some(issue.remediation);
                        }
                    });
                });
            }
        });
}

fn issue_details(ui: &mut Ui, index: usize, issue: &PreflightIssue, t: &Tokens) {
    ui.label(
        egui::RichText::new(format!("{:02}", index + 1))
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text_faint),
    );
    ui.vertical(|ui| {
        ui.label(egui::RichText::new(&issue.check).strong());
        ui.label(
            egui::RichText::new(&issue.observed)
                .color(t.color.err)
                .size(tokens::FS_1),
        );
        ui.label(
            egui::RichText::new(format!("Required · {}", issue.required))
                .color(t.color.text_dim)
                .size(tokens::FS_0),
        );
    });
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
    section_title(ui, "Frozen dispatch contract");
    property_row(ui, "Revision", &report.project_revision.to_string());
    property_row(
        ui,
        "Analysis identities",
        &format!("{} enabled definition(s)", report.analysis_count),
    );
    property_row(ui, "Reference PVT", &report.reference_pvt);
    property_row(ui, "Tasks", &report.task_count.to_string());
    property_row(ui, "Target", &report.target);
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
        PreflightRemediation::ModelBindings => "Open models",
    }
}

fn apply_remediation(app: &mut RSpiceApp, remediation: PreflightRemediation) {
    match remediation {
        PreflightRemediation::DesignChecks => {
            Command::RunChecks.execute(app);
            Command::VerificationPage(VerificationPage::Checks).execute(app);
        }
        PreflightRemediation::SimulationPlan => {
            Command::OpenWorkspace(Workspace::Simulate).execute(app);
        }
        PreflightRemediation::ModelBindings => {
            Command::ModelsPage(ModelsPage::Libraries).execute(app);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_collects_all_independent_blocker_classes() {
        let mut state = AppState::default();
        state.schematic = crate::state::SchematicState::default();
        state.sim_setup.enabled.clear();
        state.sim_setup.analysis_order.clear();
        crate::common::menu_bar::run_design_rule_check(&mut state);

        let report = collect_report(&state);

        assert!(!report.is_runnable());
        assert!(
            report
                .blockers
                .iter()
                .any(|issue| issue.check == "Design topology")
        );
        assert!(report.blockers.iter().any(|issue| issue.check == "Run set"));
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
}
