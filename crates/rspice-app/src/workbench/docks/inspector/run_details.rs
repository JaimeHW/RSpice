//! Run details…: everything that proves where one retained run came from.
//!
//! The Results inspector names the run and how long it took; the identifiers,
//! digests, inputs, model sources and solver statistics behind it live here,
//! one dialog reached from the inspector's Run section and from a run's row
//! in the data browser.
//!
//! The run's manifest is projected once, when the dialog opens: the
//! projection digests every sample the run retained, which is not work for
//! every frame the dialog stays open.

use egui::{Context, Ui};

use super::super::super::design_system::{
    property_card, property_row, property_row_height, property_row_wrapped,
};
use super::{engineering_readout, executed_deck, result_authority::forced_lte_acceptances};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize};
use crate::workbench::app::RunDetailsDialogState;
use crate::workbench::{AppState, RSpiceApp};

/// At most this many significant digits for the times a convergence record
/// names.
const TIME_DIGITS: usize = 6;

/// How tall a forced-point list grows before it scrolls.
const FORCED_LIST_HEIGHT: f32 = 160.0;

/// Open Run details… on the retained run `run_id`. A run that is no longer
/// retained opens nothing.
pub(in crate::workbench::docks) fn open(state: &mut AppState, run_id: u64) {
    let Some(run) = state
        .simulation
        .retained
        .runs
        .iter()
        .find(|run| run.id == run_id)
    else {
        return;
    };
    let manifest = crate::workbench::documents::result_document::manifest::manifest_for_run(run);
    state.dialogs.run_details = Some(RunDetailsDialogState { run_id, manifest });
}

/// Draw Run details… while it is open, and act on its two routes once the
/// borrow of the run it describes is done.
pub(crate) fn show(ctx: &Context, app: &mut RSpiceApp) {
    let Some(dialog) = app.state.dialogs.run_details.as_ref() else {
        return;
    };
    let run_id = dialog.run_id;
    let Some(run) = app
        .state
        .simulation
        .retained
        .runs
        .iter()
        .find(|run| run.id == run_id)
    else {
        // The run left the history while its details were open.
        app.state.dialogs.run_details = None;
        return;
    };
    let manifest = &dialog.manifest;
    let executed = app
        .state
        .simulation
        .retained
        .executed_decks
        .get(run_id)
        .map(crate::state::ExecutedDeck::model_sources);
    // The producing-plan route resolves against the active dataset, so it is
    // offered only for the run that is active.
    let plan_block = app
        .state
        .simulation
        .active_run()
        .is_some_and(|active| active.id == run_id)
        .then(|| {
            crate::workbench::state::plan_provenance::producing_plan_block(
                &app.state.simulation,
                app.state.sim_setup.stable_analysis_plan().ok(),
            )
        });
    let digits = usize::from(
        app.state
            .ui
            .preferences
            .result_presentation_policy()
            .displayed_significant_digits()
            .get(),
    );
    let mut open_plan = false;
    let mut open_deck = None;

    let choice = Dialog::new("Run details", "Close")
        .description(
            "Where one retained run came from: its identifiers and digests, the plan and \
             inputs that produced it, the model sources it read, and the solver's statistics.",
        )
        .size(DialogSize::Transaction)
        .show(ctx, |ui| {
            property_card(ui, "Run", |ui| {
                property_row(ui, "Run", &manifest.run_label);
                property_row(ui, "Duration", &manifest.elapsed_time);
                property_row(ui, "State", &manifest.lifecycle);
                property_row(ui, "Execution target", &manifest.execution_target);
                property_row(ui, "Run ID", &manifest.run_id);
                if let Some(job) = run.job_id {
                    property_row(ui, "Job ID", &job.to_string());
                }
                property_row(ui, "Dataset ID", &manifest.dataset_id);
                property_row(ui, "Dataset digest", &manifest.dataset_digest);
                property_row(ui, "Integrity", &manifest.integrity);
            });
            match &manifest.authority {
                Some(authority) => {
                    property_card(ui, "Plan and inputs", |ui| {
                        property_row(ui, "Source", &authority.source_domain);
                        property_row(
                            ui,
                            "Simulation plan",
                            authority
                                .simulation_plan_id
                                .as_deref()
                                .unwrap_or("none, a manual deck"),
                        );
                        property_row(ui, "Project revision", &authority.project_revision);
                        property_row(
                            ui,
                            "Prepared input digest",
                            &authority.prepared_snapshot_digest,
                        );
                        property_row(
                            ui,
                            "Source content digest",
                            &authority.source_content_digest,
                        );
                        property_row(ui, "Source check", &authority.source_check);
                        property_row(ui, "Source-check digest", &authority.source_check_digest);
                        if let Some(block) = plan_block {
                            let route = Button::new("Open producing plan")
                                .enabled(block.is_none())
                                .show(ui);
                            match block {
                                None => {
                                    open_plan = route
                                        .on_hover_text(
                                            "Open the Analyses page of the plan that produced \
                                             this run, with the producing analysis selected",
                                        )
                                        .clicked();
                                }
                                Some(reason) => {
                                    route.on_hover_text(reason);
                                }
                            }
                        }
                    });
                    if !authority.model_sources.is_empty() {
                        property_card(ui, "Model source digests", |ui| {
                            for (identity, digest) in &authority.model_sources {
                                property_row(ui, identity, digest);
                            }
                        });
                    }
                }
                None => {
                    property_card(ui, "Plan and inputs", |ui| {
                        property_row_wrapped(
                            ui,
                            "Inputs",
                            "This run kept no record of the plan and inputs that produced it.",
                        );
                    });
                }
            }
            property_card(ui, "Executed deck", |ui| {
                open_deck = executed_deck::record(ui, run_id, executed.as_deref());
            });
            for analysis in &run.analyses {
                if let Some(quality) = analysis.convergence.as_deref() {
                    convergence(ui, &analysis.label, quality, digits);
                }
            }
        });

    let close = matches!(choice, DialogChoice::Primary | DialogChoice::Cancelled);
    if close || open_plan || open_deck.is_some() {
        app.state.dialogs.run_details = None;
    }
    if let Some(run_id) = open_deck {
        executed_deck::reveal(&mut app.state, run_id);
    }
    if open_plan {
        crate::workbench::commands::vocabulary::Command::OpenProducingPlan.execute(app);
    }
}

/// One analysis's solver statistics: its transient's, and for a periodic
/// analysis its initialization's.
fn convergence(
    ui: &mut Ui,
    analysis: &str,
    quality: &rspice_results::convergence_quality::TransientConvergenceEvidence,
    digits: usize,
) {
    use crate::state::PeriodicInitializationMethod;

    property_card(ui, &format!("Convergence · {analysis}"), |ui| {
        let forced = forced_lte_acceptances(quality);
        property_row(ui, "LTE acceptances", &format!("{forced} forced"))
            .on_hover_text("LTE is local truncation error.");
        convergence_report(
            ui,
            &quality.transient,
            digits,
            &format!("{analysis}/transient"),
        );
    });
    if let Some(initialization) = &quality.initialization {
        let (phase, iterations) = match initialization.method {
            PeriodicInitializationMethod::Shooting => {
                ("Shooting initialization", "Shooting corrections")
            }
            PeriodicInitializationMethod::HarmonicBalance => {
                ("Harmonic-balance initialization", "HB Newton iterations")
            }
        };
        property_card(ui, &format!("{phase} · {analysis}"), |ui| {
            property_row(
                ui,
                iterations,
                &initialization.solver_iterations.to_string(),
            );
            property_row(
                ui,
                "Final periodic residual",
                &format!("{:.3e}", initialization.final_residual),
            );
            convergence_report(
                ui,
                &initialization.report,
                digits,
                &format!("{analysis}/initialization"),
            );
        });
    }
}

/// One phase's solver counters. `salt` tells this phase's forced-point list
/// from every other one in the dialog.
fn convergence_report(
    ui: &mut Ui,
    report: &crate::state::ConvergenceReport,
    digits: usize,
    salt: &str,
) {
    for (name, count) in [
        ("Newton iterations", report.total_iterations),
        ("Gmin stepping", report.gmin_stepping_count),
        ("Source stepping", report.source_stepping_count),
        ("Step reductions", report.timestep_reductions),
        ("LTE rejections", report.lte_rejections),
        ("Device bypasses", report.bypassed_device_evaluations),
    ] {
        property_row(ui, name, &count.to_string());
    }
    property_row(
        ui,
        "Maximum residual",
        &format!("{:.3e}", report.max_residual),
    );
    property_row(
        ui,
        "Iterations per solve",
        &format!("{:.2}", report.avg_iterations_per_solve),
    );
    // Each forced point at the time it was forced on the solver's own
    // trajectory, before any cropping, envelope or Fourier step. A long run
    // can force thousands, so the list scrolls rather than the dialog.
    if let Some(basis) = &report.time_basis
        && !basis.force_accepted_times_s.is_empty()
    {
        let times = &basis.force_accepted_times_s;
        egui::ScrollArea::vertical()
            .id_salt(("run-details-forced", salt))
            .max_height(FORCED_LIST_HEIGHT)
            .auto_shrink([false, true])
            .show_rows(ui, property_row_height(ui), times.len(), |ui, range| {
                for position in range {
                    property_row(
                        ui,
                        &format!("Forced point {}", position + 1),
                        &engineering_readout(times[position], "s", digits.min(TIME_DIGITS)),
                    );
                }
            });
    }
    if let Some(diagnostic) = &report.failure_diagnostic {
        property_row_wrapped(ui, "Diagnostic", &diagnostic.summary());
        if !diagnostic.failure_message.is_empty() {
            property_row_wrapped(ui, "Solver message", &diagnostic.failure_message);
        }
    }
}
