//! Run details…: everything that proves where one retained run came from.
//!
//! The Results inspector names the run and how long it took; the identifiers,
//! digests, the inventory of its analyses, inputs, model sources and solver
//! statistics behind it live here, one dialog reached from the inspector's
//! Run section and from a run's row in the data browser.
//!
//! The run's manifest is projected once, when the dialog opens. Its dataset
//! digest reads every sample the run holds in memory (a run kept in a results
//! store is named by the store's own digest), so it is computed on a Results
//! job and its row fills in when it is known: the dialog shows at once.

mod executed_deck;

use egui::{Context, Ui};

use super::super::super::design_system::{
    property_card, property_row, property_row_height, property_row_wrapped,
};
use super::result_authority::forced_lte_acceptances;
use crate::state::SimulationState;
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize};
use crate::workbench::app_state::Model;
use rspice_results::number_format::fmt_si_significant;

/// The run Run details… is open on, with its manifest taken once when the
/// dialog opened, and its dataset digest once a Results job has computed it.
#[derive(Debug, Clone)]
pub struct RunDetailsDialogState {
    pub run_id: u64,
    pub manifest: rspice_results::manifest::ManifestViewModel,
    dataset_digest: std::sync::Arc<std::sync::OnceLock<String>>,
}

impl RunDetailsDialogState {
    /// Run details… on `run`: its manifest at once, and its dataset digest
    /// computed on a Results job.
    pub fn new(run: &crate::state::SimulationRun) -> Self {
        use crate::workbench::documents::result_document::{frame_work, manifest};

        let dataset_digest = std::sync::Arc::<std::sync::OnceLock<String>>::default();
        let (slot, retained) = (std::sync::Arc::clone(&dataset_digest), run.data.clone());
        let _ = crate::workbench::results_job::ResultsJob::start("run-digest", move |_| {
            frame_work::note(frame_work::DatasetWalk::DatasetDigest);
            let digest = crate::state::run_store::dataset_digest(&retained);
            let _ = slot.set(digest.to_string());
            Ok(())
        });
        Self {
            run_id: run.id,
            manifest: manifest::manifest_without_digest(run),
            dataset_digest,
        }
    }

    /// The run's dataset digest; `None` while it is still being computed.
    pub fn dataset_digest(&self) -> Option<&str> {
        self.dataset_digest.get().map(String::as_str)
    }
}

/// At most this many significant digits for the times a convergence record
/// names.
const TIME_DIGITS: usize = 6;

/// How tall a forced-point list, or the inventory, grows before it scrolls.
const FORCED_LIST_HEIGHT: f32 = 160.0;

/// Open Run details… on the retained run `run_id`. A run that is no longer
/// retained opens nothing.
pub fn open(
    run_details: &mut Option<Box<RunDetailsDialogState>>,
    simulation: &SimulationState,
    run_id: u64,
) {
    let Some(run) = simulation.retained.runs.iter().find(|run| run.id == run_id) else {
        return;
    };
    *run_details = Some(Box::new(RunDetailsDialogState::new(run)));
}

/// The routes out of Run details… the reader took this frame. Each is a
/// button of the dialog, and taking either closes it.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct RunDetailsRoutes {
    /// Open the producing plan's Analyses page.
    pub(crate) open_plan: bool,
    /// Open the deck the engine executed for this run.
    pub(crate) open_deck: Option<u64>,
}

/// Draw Run details… while it is open, and return the routes the reader
/// took, for the caller to follow once the borrow of the run it describes
/// is done.
pub fn show(
    ctx: &Context,
    run_details: &mut Option<Box<RunDetailsDialogState>>,
    model: &Model,
) -> RunDetailsRoutes {
    let Some(dialog) = run_details.as_ref() else {
        return RunDetailsRoutes::default();
    };
    let run_id = dialog.run_id;
    let Some(run) = model
        .simulation
        .retained
        .runs
        .iter()
        .find(|run| run.id == run_id)
    else {
        // The run left the history while its details were open.
        *run_details = None;
        return RunDetailsRoutes::default();
    };
    let manifest = &dialog.manifest;
    let executed = model
        .simulation
        .retained
        .executed_decks
        .get(run_id)
        .map(crate::state::ExecutedDeck::model_sources);
    // The producing-plan route resolves against the active dataset, so it is
    // offered only for the run that is active.
    let plan_block = model
        .simulation
        .active_run()
        .is_some_and(|active| active.id == run_id)
        .then(|| {
            crate::workbench::state::plan_provenance::producing_plan_block(
                &model.simulation,
                model.sim_setup.stable_analysis_plan().ok(),
            )
        });
    let digits = usize::from(
        model
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
                property_row(
                    ui,
                    "Duration",
                    &super::result_authority::run_duration(ui, run, manifest),
                );
                property_row(ui, "State", &manifest.lifecycle);
                // Stated only for a run whose checks reported something: a
                // run that checked nothing has not passed anything.
                if let Some(verdict) = run.check_verdict() {
                    use rspice_results::analysis_result::CheckVerdict;
                    let verdict = match verdict {
                        CheckVerdict::Passed => "Passed",
                        CheckVerdict::Warned => "Warned",
                        CheckVerdict::Failed => "Failed",
                    };
                    property_row(ui, "Checks", verdict);
                }
                property_row(ui, "Execution target", &manifest.execution_target);
                property_row(ui, "Run ID", &manifest.run_id);
                if let Some(job) = run.job_id {
                    property_row(ui, "Job ID", &job.to_string());
                }
                property_row(ui, "Dataset ID", &manifest.dataset_id);
                let digest = dialog.dataset_digest().unwrap_or_else(|| {
                    // The thread computing it cannot wake the frame, so the
                    // dialog looks again shortly.
                    ui.ctx()
                        .request_repaint_after(std::time::Duration::from_millis(100));
                    "Computing\u{2026}"
                });
                property_row(ui, "Dataset digest", digest);
                property_row(ui, "Integrity", &manifest.integrity);
            });
            inventory(ui, manifest);
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
                        // The design the run elaborated: by the name it ran
                        // under, and by identity and revision, which is what
                        // tells it apart from the configuration as it stands.
                        if let Some(recorded) = run
                            .prepared_receipt()
                            .and_then(|receipt| receipt.configuration())
                        {
                            property_row(
                                ui,
                                "Configuration",
                                &crate::state::workspace::recorded_configuration_words(
                                    &model.workspace.content.configuration_sets,
                                    recorded,
                                ),
                            );
                            if let Some(id) = recorded.id {
                                property_row(
                                    ui,
                                    "Configuration ID",
                                    &format!("{id} \u{b7} revision {}", recorded.revision),
                                );
                            }
                        }
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
        *run_details = None;
    }
    RunDetailsRoutes {
        open_plan,
        open_deck,
    }
}

/// The run's analyses as its manifest lists them, one row per task: what
/// each kept and whether it may be cited. The rest of a row (its domain,
/// precision, expansion and identities) is on hover. A save-all run lists
/// thousands, so the list scrolls rather than the dialog.
fn inventory(ui: &mut Ui, manifest: &rspice_results::manifest::ManifestViewModel) {
    property_card(ui, "Analyses", |ui| {
        property_row(
            ui,
            "Retained",
            &format!(
                "{} of {} tasks \u{b7} {}",
                manifest.retained_result_count, manifest.task_count, manifest.inventory_status
            ),
        );
        egui::ScrollArea::vertical()
            .id_salt("run-details-inventory")
            .max_height(FORCED_LIST_HEIGHT)
            .auto_shrink([false, true])
            .show_rows(
                ui,
                property_row_height(ui),
                manifest.rows.len(),
                |ui, range| {
                    for row in &manifest.rows[range] {
                        let mut details = vec![
                            format!("Domain axis {}", row.domain_axis),
                            format!("Precision {}", row.precision),
                            format!("Expansion {}", row.expansion),
                        ];
                        details.extend(row.task_identity.iter().map(|id| format!("Task {id}")));
                        details.extend(
                            row.config_digest
                                .iter()
                                .map(|digest| format!("Configuration digest {digest}")),
                        );
                        property_row(
                            ui,
                            &row.analysis,
                            &format!("{} \u{b7} {}", row.stored_values, row.eligibility),
                        )
                        .on_hover_text(details.join("\n"));
                    }
                },
            );
    });
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
                        &fmt_si_significant(times[position], "s", digits.min(TIME_DIGITS)),
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
