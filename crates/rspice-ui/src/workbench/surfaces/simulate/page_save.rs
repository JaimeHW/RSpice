//! Save, streaming & retention policy.
//!
//! What a run keeps, how it is delivered while the solve is still running, and
//! what that costs. Every number here is the plan's own arithmetic over the
//! enabled outputs — the page never reports a measurement of a previous run as
//! a forecast of the next one.

use egui::Ui;

use crate::product::RunId;
use crate::simulation::{SavedOutputStorageEstimate, output_contract::SavedOutputPreflightReport};
use crate::state::workspace::SimulationPlanPayload;
use crate::state::{
    OutputSelectionMode, RunRetention, SavedOutputPolicy, SavedOutputPrecision,
    SavedOutputStreaming,
};
use crate::workbench::RSpiceApp;

use crate::ui::widgets::select;

use super::page_kit::{
    Tone, card, card_body, card_note, card_row, field_pair, ledger_group, ledger_head, ledger_row,
    rule_row,
};

const GROUP_COLUMNS: [f32; 4] = [0.34, 0.20, 0.22, 0.24];
const DATASET_COLUMNS: [f32; 4] = [0.26, 0.14, 0.20, 0.40];

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let payload = plan_payload(app);
    let selection = app
        .simulation_controller
        .effective_saved_outputs_preflight(&app.state, &payload.saved_outputs);
    let (effective_outputs, reports, automatic_fallback, selection_error) = match selection {
        Ok((outputs, reports, automatic_fallback)) => (outputs, reports, automatic_fallback, None),
        Err(error) => (Vec::new(), Vec::new(), false, Some(error.to_string())),
    };
    capture_groups(
        ui,
        app,
        &effective_outputs,
        &reports,
        automatic_fallback,
        selection_error.as_deref(),
    );
    card_row(
        ui,
        app,
        |ui, app| streaming_contract(ui, app, &payload),
        retention_contract,
    );
}

/// Retention depths the page offers.
///
/// A bounded set rather than free text: the number is a promise about how
/// much of the reader's work survives, and a typo in a text field is a
/// promise nobody meant to make.
const RETENTION_CHOICES: [usize; 5] = [5, 10, 20, 50, 200];
const STORAGE_BUDGET_CHOICES: [u64; 5] = [
    256 * 1024 * 1024,
    1024 * 1024 * 1024,
    10 * 1024 * 1024 * 1024,
    50 * 1024 * 1024 * 1024,
    200 * 1024 * 1024 * 1024,
];

/// How many datasets the retention list names before it summarizes the rest.
///
/// A baseline past that point is still listed: one the reader cannot see is
/// one they cannot release, and a project whose baselines have all scrolled
/// out of reach can never get back under its limit.
const DATASETS_LISTED: usize = 6;

/// The retention rules the controls cannot state on their own.
const RETENTION_NOTE: &str = "A dataset is sealed against the manifest that produced it, so a \
                              retained result can still be trusted after the plan has moved on. \
                              Lowering the limit discards the oldest unpinned datasets \
                              immediately, and raising it never recovers one already discarded. \
                              A discarded dataset takes the exact deck its engine read with it.";

/// The rules that mean nothing until there is a dataset to act on.
const RETENTION_LIST_NOTE: &str = "The highlighted row is the active dataset. Selecting a row \
                                   pins it as a golden baseline or releases it: a baseline \
                                   survives the limit, and a released one becomes eligible for \
                                   the next run to discard. Releasing never destroys the dataset \
                                   on the spot, so a project can sit over its limit until it runs \
                                   again. The baseline selected by Golden Regression is locked \
                                   here until a different baseline is selected there.";

/// One retained dataset, as the retention card states it.
struct RetentionRow {
    run_id: RunId,
    dataset: String,
    analyses: String,
    /// What this dataset's retained executed decks cost, formatted, or the
    /// statement that none are held. A run whose decks were evicted, or which
    /// executed before the project was written, costs nothing and says so —
    /// it never reads as zero bytes of a deck that exists.
    decks: String,
    pinned: bool,
    regression_baseline: bool,
    active: bool,
}

/// What one plan's retained executed decks cost, split the way the ledger
/// prints it so the printed rows and the printed total cannot disagree.
///
/// `listed` and `summarized` are exactly the two things the card draws, and
/// `total` is their sum by construction rather than by a second pass over the
/// history that could count a different set of runs.
struct ExecutedDeckStorage {
    listed: u64,
    summarized: u64,
    /// How many of the plan's retained datasets still have their decks.
    held: usize,
}

impl ExecutedDeckStorage {
    const fn total(&self) -> u64 {
        self.listed.saturating_add(self.summarized)
    }
}

fn plan_payload(app: &RSpiceApp) -> SimulationPlanPayload {
    app.state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())
        .and_then(|plan_id| app.state.workspace.plan_data(plan_id).cloned())
        .unwrap_or_default()
}

/// Outputs grouped by save policy, because the policy is what decides whether
/// a signal is retained at full rate, decimated, or not stored at all.
fn capture_groups(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    outputs: &[crate::state::SavedOutput],
    reports: &[SavedOutputPreflightReport],
    automatic_fallback: bool,
    selection_error: Option<&str>,
) {
    let selection_mode = app.state.sim_setup.save_policy.output_selection_mode;
    let mut groups: Vec<(SavedOutputPolicy, usize, u64, usize)> = SavedOutputPolicy::ALL
        .iter()
        .map(|policy| (*policy, 0usize, 0u64, 0usize))
        .collect();
    let mut retained_engine_source_analyses = std::collections::HashSet::new();
    for (output, report) in outputs.iter().zip(reports) {
        let Some(entry) = groups
            .iter_mut()
            .find(|(policy, _, _, _)| *policy == output.save_policy)
        else {
            continue;
        };
        entry.1 += 1;
        match report.storage_estimate() {
            SavedOutputStorageEstimate::ExactBytes(bytes) => {
                entry.2 = entry.2.saturating_add(*bytes)
            }
            SavedOutputStorageEstimate::Indeterminate { .. } => entry.3 += 1,
        }
        retained_engine_source_analyses
            .extend(report.retained_engine_source_analysis_ids().iter().copied());
    }
    if selection_mode != OutputSelectionMode::SaveAll
        && let Some((_, _, bytes, _)) = groups
            .iter_mut()
            .find(|(policy, _, _, _)| *policy == SavedOutputPolicy::OnDemandFromRetainedState)
    {
        *bytes = bytes.saturating_add(
            crate::simulation::output_contract::retained_engine_source_upper_bound_bytes(
                retained_engine_source_analyses.len(),
            ),
        );
    }
    let run_set_points = u64::try_from(app.state.sim_setup.run_set.point_count())
        .unwrap_or(u64::MAX)
        .max(1);
    for (_, _, bytes, _) in &mut groups {
        *bytes = bytes.saturating_mul(run_set_points);
    }
    let save_all_engine_ceiling = (selection_mode == OutputSelectionMode::SaveAll).then(|| {
        crate::simulation::output_contract::retained_engine_source_upper_bound_bytes(
            app.state.sim_setup.enabled_analysis_instance_count(),
        )
        .saturating_mul(run_set_points)
    });
    let authored_total = groups.iter().map(|(_, _, bytes, _)| bytes).sum::<u64>();
    let total = save_all_engine_ceiling.map_or(authored_total, |engine| {
        engine.saturating_add(authored_total)
    });
    let indeterminate: usize = groups.iter().map(|(_, _, _, count)| count).sum();
    let budget = app.state.sim_setup.save_policy.maximum_storage_bytes;
    let over_budget = total > budget;
    let status = if selection_error.is_some() {
        "Selection preflight unavailable".to_owned()
    } else if over_budget {
        format!(
            "{} forecast exceeds {} ceiling",
            format_bytes(total),
            format_bytes(budget)
        )
    } else if selection_mode == OutputSelectionMode::SaveAll {
        format!("{} Save All ceiling", format_bytes(total))
    } else if automatic_fallback {
        format!("{} automatic forecast", format_bytes(total))
    } else if indeterminate == 0 {
        format!("{} explicit forecast", format_bytes(total))
    } else {
        format!(
            "{} minimum · {indeterminate} indeterminate",
            format_bytes(total)
        )
    };
    card(
        ui,
        "Capture groups",
        Some((
            status.as_str(),
            if selection_error.is_some() || over_budget {
                Tone::Error
            } else if indeterminate == 0 {
                Tone::Ok
            } else {
                Tone::Warn
            },
        )),
        |ui| {
            let budget_choices = STORAGE_BUDGET_CHOICES
                .iter()
                .map(|bytes| format_bytes(*bytes))
                .collect::<Vec<_>>();
            let selected_budget = format_bytes(budget);
            let selected_mode = selection_mode;
            let mode_choices = OutputSelectionMode::ALL
                .iter()
                .map(|mode| mode.label().to_owned())
                .collect::<Vec<_>>();
            let mut picked_budget = None;
            let mut picked_mode = None;
            card_body(ui, |ui| {
                field_pair(
                    ui,
                    ("Output selection", &mut |ui: &mut Ui, width: f32| {
                        picked_mode = select(
                            ui,
                            "simulation.save.output-selection",
                            "Plan output selection mode",
                            selected_mode.label(),
                            &mode_choices,
                            width,
                        );
                    }),
                    Some((
                        "Retained-evidence ceiling",
                        &mut |ui: &mut Ui, width: f32| {
                            picked_budget = select(
                                ui,
                                "simulation.save.storage-budget",
                                "Plan retained-evidence storage ceiling",
                                &selected_budget,
                                &budget_choices,
                                width,
                            );
                        },
                    )),
                );
                rule_row(ui, "Selection behavior", selected_mode.description());
            });
            ledger_head(
                ui,
                &GROUP_COLUMNS,
                &["Save policy", "Outputs", "Forecast size", "Indeterminate"],
            );
            if let Some(engine_ceiling) = save_all_engine_ceiling {
                ledger_row(
                    ui,
                    &GROUP_COLUMNS,
                    &[
                        ("Engine result set", Tone::Neutral),
                        ("all", Tone::Accent),
                        (format_bytes(engine_ceiling).as_str(), Tone::Neutral),
                        ("—", Tone::Neutral),
                    ],
                    false,
                );
            }
            for (policy, count, bytes, indeterminate) in &groups {
                if selected_mode == OutputSelectionMode::SaveAll && *count == 0 {
                    continue;
                }
                let tone = if *count == 0 {
                    Tone::Neutral
                } else {
                    Tone::Accent
                };
                ledger_row(
                    ui,
                    &GROUP_COLUMNS,
                    &[
                        (policy.label(), Tone::Neutral),
                        (count.to_string().as_str(), tone),
                        (format_bytes(*bytes).as_str(), Tone::Neutral),
                        (
                            if *indeterminate == 0 {
                                "—".to_owned()
                            } else {
                                indeterminate.to_string()
                            }
                            .as_str(),
                            if *indeterminate == 0 {
                                Tone::Neutral
                            } else {
                                Tone::Warn
                            },
                        ),
                    ],
                    false,
                );
            }
            if let Some(error) = selection_error {
                card_note(
                    ui,
                    &format!("Output selection cannot be forecast yet: {error}"),
                );
            }
            card_note(
                ui,
                if selected_mode == OutputSelectionMode::SaveAll {
                    "Save All reserves the engine's complete bounded result-value allowance for \
                     every enabled analysis and Run Set point. Authored outputs and probes still \
                     control initial plotting and any derived-output storage is budgeted in its \
                     policy row. The authenticated ceiling is rechecked before each terminal \
                     analysis is adopted."
                } else {
                    "Size is summed from the effective output set across the tasks each output is \
                     compatible with, then multiplied by the global Run Set point count. \
                     Automatic fallback is resolved from the configured simulation root, not \
                     whichever child sheet is open. An output whose retained source or sample \
                     count cannot be proven before the solve is indeterminate rather than zero."
                },
            );
            let mut policy = app.state.sim_setup.save_policy;
            if let Some(index) = picked_mode
                && let Some(mode) = OutputSelectionMode::ALL.get(index).copied()
            {
                policy.output_selection_mode = mode;
            }
            if let Some(index) = picked_budget
                && let Some(bytes) = STORAGE_BUDGET_CHOICES.get(index).copied()
            {
                policy.maximum_storage_bytes = bytes;
            }
            if policy != app.state.sim_setup.save_policy {
                commit_save_policy(app, policy, "Save policy · output selection and storage");
            }
        },
    );
}

fn streaming_contract(ui: &mut Ui, app: &mut RSpiceApp, payload: &SimulationPlanPayload) {
    let streamed = payload
        .saved_outputs
        .iter()
        .filter(|output| {
            matches!(
                output.streaming,
                SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation
            )
        })
        .count();
    let precisions = SavedOutputPrecision::ALL
        .iter()
        .map(|precision| {
            (
                precision.label(),
                payload
                    .saved_outputs
                    .iter()
                    .filter(|output| output.stored_precision == *precision)
                    .count(),
            )
        })
        .filter(|(_, count)| *count > 0)
        .map(|(label, count)| format!("{count} × {label}"))
        .collect::<Vec<_>>();
    card(
        ui,
        "Streaming & precision",
        Some((
            format!("{streamed} streamed").as_str(),
            if streamed == 0 {
                Tone::Neutral
            } else {
                Tone::Ok
            },
        )),
        |ui| {
            card_body(ui, |ui| {
                let mut policy = app.state.sim_setup.save_policy;
                let mut changed = false;
                changed |= ui
                    .checkbox(
                        &mut policy.live_streaming_enabled,
                        "Permit live delivery for outputs that request it",
                    )
                    .changed();
                changed |= ui
                    .checkbox(
                        &mut policy.retain_failure_diagnostics,
                        "Retain accepted transient prefixes after failure or abort",
                    )
                    .changed();
                rule_row(
                    ui,
                    "Streamed while solving",
                    &format!("{streamed} of {}", payload.saved_outputs.len()),
                );
                rule_row(
                    ui,
                    "Stored precision",
                    &if precisions.is_empty() {
                        "no outputs configured".to_owned()
                    } else {
                        precisions.join(" · ")
                    },
                );
                rule_row(
                    ui,
                    "Display decimation",
                    "never changes what is stored · the dataset keeps every accepted point",
                );
                if changed {
                    commit_save_policy(app, policy, "Save policy · streaming and diagnostics");
                }
            });
            card_note(
                ui,
                "Streaming decides when a signal becomes visible, not what is retained. A stream \
                 that cannot keep up is decimated for display only; the committed dataset is \
                 identical either way.",
            );
        },
    );
}

/// The retention ledger's rows, its unlisted tail, and what both cost.
///
/// One pass, so the printed rows and the printed total are the same
/// measurement partitioned rather than two walks of the history that could
/// disagree about which runs belong to the plan. Separated from the painter so
/// the arithmetic can be checked without one.
fn retention_ledger(
    simulation: &crate::state::SimulationState,
    plan_id: crate::product::SimulationPlanId,
    regression_baseline: Option<RunId>,
    active_run_id: Option<RunId>,
) -> (Vec<RetentionRow>, usize, ExecutedDeckStorage) {
    let mut rows: Vec<RetentionRow> = Vec::new();
    let mut summarized = 0usize;
    let mut decks = ExecutedDeckStorage {
        listed: 0,
        summarized: 0,
        held: 0,
    };
    for (index, run) in simulation
        .runs
        .iter()
        .filter(|run| {
            run.prepared_receipt()
                .and_then(crate::state::PreparedRunReceipt::simulation_plan_id)
                == Some(plan_id)
        })
        .enumerate()
    {
        let is_pinned = run.retention().is_pinned();
        // The deck bytes are read once per run and then attributed to
        // whichever half of the ledger prints that run, so the two printed
        // figures partition the same measurement instead of sampling it twice.
        let deck_bytes = simulation
            .executed_decks
            .run_bytes(run.id)
            .map(|bytes| bytes as u64);
        if deck_bytes.is_some() {
            decks.held += 1;
        }
        if index >= DATASETS_LISTED && !is_pinned {
            summarized += 1;
            decks.summarized = decks.summarized.saturating_add(deck_bytes.unwrap_or(0));
            continue;
        }
        decks.listed = decks.listed.saturating_add(deck_bytes.unwrap_or(0));
        rows.push(RetentionRow {
            run_id: run.run_id,
            dataset: format!("Run {}", run.id),
            analyses: run.analyses.len().to_string(),
            decks: deck_bytes.map_or_else(|| "not retained".to_owned(), format_bytes),
            pinned: is_pinned,
            regression_baseline: Some(run.run_id) == regression_baseline,
            active: Some(run.run_id) == active_run_id,
        });
    }
    (rows, summarized, decks)
}

fn retention_contract(ui: &mut Ui, app: &mut RSpiceApp) {
    let simulation = &app.state.simulation;
    let Some(plan_id) = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())
    else {
        card(
            ui,
            "Retention",
            Some(("plan unavailable", Tone::Error)),
            |ui| {
                card_note(
                    ui,
                    "A stable active simulation plan is required to own dataset retention.",
                );
            },
        );
        return;
    };
    let retained = simulation.retained_plan_dataset_count(plan_id);
    let limit = app.state.sim_setup.save_policy.retained_dataset_limit;
    let pinned = simulation.pinned_plan_run_count(plan_id);
    // Pinning can put the limit out of reach entirely. The card states that
    // rather than a count that reads as a policy still being enforced.
    let unenforceable = pinned >= limit;
    let at_limit = retained >= limit;
    let active_run_id = simulation.active_run().map(|run| run.run_id);
    let regression_baseline = app
        .state
        .workspace
        .plan_data(plan_id)
        .and_then(|payload| payload.regression_baseline_run);
    let (rows, summarized, decks) =
        retention_ledger(simulation, plan_id, regression_baseline, active_run_id);
    let status = if retained > limit {
        format!("{retained} of {limit} · {pinned} pinned · over the limit")
    } else if unenforceable {
        format!("{retained} of {limit} · {pinned} pinned · the limit no longer holds")
    } else if pinned > 0 {
        format!("{retained} of {limit} · {pinned} pinned · never pruned")
    } else if at_limit {
        format!("{retained} of {limit} · the next run discards the oldest")
    } else {
        format!("{retained} of {limit} retained")
    };
    let beyond_limit = if unenforceable {
        format!("{pinned} pinned · nothing left to discard within the limit of {limit}")
    } else {
        "the oldest unpinned dataset is discarded when a new run exceeds it".to_owned()
    };
    // The deck sentence names both halves of what is drawn below it, so the
    // ledger can be added up against it rather than believed.
    let deck_storage = if retained == 0 {
        "no dataset retained yet · a run's decks are kept with its dataset".to_owned()
    } else if decks.held == 0 {
        format!("none held · this project holds no deck for its {retained} retained datasets")
    } else {
        format!(
            "{} across {} of {retained} datasets · {} listed · {} summarized",
            format_bytes(decks.total()),
            decks.held,
            format_bytes(decks.listed),
            format_bytes(decks.summarized)
        )
    };
    let note = if rows.is_empty() {
        RETENTION_NOTE.to_owned()
    } else {
        format!("{RETENTION_NOTE} {RETENTION_LIST_NOTE}")
    };
    let choices: Vec<String> = RETENTION_CHOICES
        .iter()
        .map(|depth| format!("{depth} datasets"))
        .collect();
    let selected = format!("{limit} datasets");
    let mut picked = None;
    let mut reclassified = None;
    card(
        ui,
        "Retention",
        Some((
            status.as_str(),
            if at_limit || unenforceable {
                Tone::Warn
            } else {
                Tone::Ok
            },
        )),
        |ui| {
            card_body(ui, |ui| {
                field_pair(
                    ui,
                    ("Datasets kept", &mut |ui: &mut Ui, width: f32| {
                        picked = select(
                            ui,
                            "simulation.save.retention",
                            "Datasets kept",
                            &selected,
                            &choices,
                            width,
                        );
                    }),
                    None,
                );
                rule_row(
                    ui,
                    "Commit granularity",
                    "transactional per point · a cancelled run keeps the points it completed",
                );
                rule_row(
                    ui,
                    "Configuration change",
                    "produces a new dataset · it never rewrites a retained one",
                );
                rule_row(ui, "Beyond the limit", &beyond_limit);
                rule_row(ui, "Executed decks", &deck_storage);
            });
            if !rows.is_empty() {
                ledger_head(
                    ui,
                    &DATASET_COLUMNS,
                    &["Dataset", "Analyses", "Deck", "Retention"],
                );
                for row in &rows {
                    let response = ledger_row(
                        ui,
                        &DATASET_COLUMNS,
                        &[
                            (row.dataset.as_str(), Tone::Neutral),
                            (row.analyses.as_str(), Tone::Neutral),
                            (row.decks.as_str(), Tone::Neutral),
                            if row.regression_baseline {
                                ("active regression baseline · pinned", Tone::Ok)
                            } else if row.pinned {
                                ("golden baseline", Tone::Ok)
                            } else {
                                ("pruneable", Tone::Neutral)
                            },
                        ],
                        row.active,
                    );
                    if row.regression_baseline {
                        response.on_hover_text(
                            "This run is the plan's active Golden Regression baseline. Select a different baseline in Verification before releasing it.",
                        );
                    } else if response.clicked() {
                        reclassified = Some((
                            row.run_id,
                            if row.pinned {
                                RunRetention::Pruneable
                            } else {
                                RunRetention::GoldenBaseline
                            },
                        ));
                    }
                }
                if summarized > 0 {
                    ledger_group(
                        ui,
                        &format!(
                            "{summarized} older · pruneable · not listed · {} of decks",
                            format_bytes(decks.summarized)
                        ),
                    );
                }
            }
            card_note(ui, &note);
        },
    );
    if let Some((run_id, retention)) = reclassified {
        app.state.simulation.set_run_retention(run_id, retention);
    }
    if let Some(index) = picked
        && let Some(depth) = RETENTION_CHOICES.get(index)
    {
        let mut policy = app.state.sim_setup.save_policy;
        policy.retained_dataset_limit = *depth;
        if commit_save_policy(app, policy, "Save policy · dataset retention") {
            app.state.simulation.prune_plan_runs(plan_id, *depth);
        }
    }
}

fn commit_save_policy(
    app: &mut RSpiceApp,
    policy: crate::workbench::app_state::SimulationSavePolicy,
    detail: &str,
) -> bool {
    if policy == app.state.sim_setup.save_policy {
        return false;
    }
    if let Err(error) = policy.validate() {
        app.state
            .workbench
            .analysis_lifecycle_status
            .record_refusal(error);
        return false;
    }
    let previous = app.state.sim_setup.save_policy;
    app.state.sim_setup.save_policy = policy;
    match app
        .state
        .sim_setup
        .commit_active_plan_configuration_change(detail)
    {
        Ok(receipt) => {
            app.state
                .workbench
                .analysis_lifecycle_status
                .record_receipt(receipt.status_line());
            app.state.workbench.preflight.invalidate();
            true
        }
        Err(error) => {
            app.state.sim_setup.save_policy = previous;
            app.state
                .workbench
                .analysis_lifecycle_status
                .record_refusal(error.to_string());
            false
        }
    }
}

fn format_bytes(bytes: u64) -> String {
    super::workflows::format_storage_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use super::{commit_save_policy, format_bytes};
    use crate::workbench::app::RSpiceApp;

    fn active_plan_revision(app: &RSpiceApp) -> crate::product::ObjectRevision {
        app.state
            .sim_setup
            .analysis_plan
            .as_ref()
            .expect("test instance has an active plan")
            .revision()
    }

    #[test]
    fn storage_formatter_selects_the_largest_exact_binary_unit() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(17), "17 B");
        assert_eq!(format_bytes(1024), "1.00 KiB");
        assert_eq!(format_bytes(256 * 1024 * 1024), "256.00 MiB");
        assert_eq!(format_bytes(10 * 1024 * 1024 * 1024), "10.00 GiB");
        assert_eq!(format_bytes(3 * 1024 * 1024 * 1024 * 1024), "3.00 TiB");
    }

    #[test]
    fn storage_formatter_preserves_fractional_engineering_scale() {
        assert_eq!(format_bytes(1536), "1.50 KiB");
        assert_eq!(format_bytes(3 * 1024 * 1024 / 2), "1.50 MiB");
        assert_eq!(format_bytes(5 * 1024 * 1024 * 1024 / 2), "2.50 GiB");
    }

    #[test]
    fn valid_save_policy_commit_is_revisioned_and_invalidates_preflight() {
        let mut app = RSpiceApp::test_instance();
        let before_revision = active_plan_revision(&app);
        app.state.workbench.preflight.open = true;
        let mut policy = app.state.sim_setup.save_policy;
        policy.maximum_storage_bytes /= 2;

        assert!(commit_save_policy(
            &mut app,
            policy,
            "Save policy test commit"
        ));

        assert_eq!(app.state.sim_setup.save_policy, policy);
        assert_ne!(active_plan_revision(&app), before_revision);
        assert!(!app.state.workbench.preflight.open);
        assert!(!app.state.workbench.analysis_lifecycle_status.is_refusal());
    }

    #[test]
    fn invalid_save_policy_is_refused_without_mutating_plan_or_preflight() {
        let mut app = RSpiceApp::test_instance();
        let before_policy = app.state.sim_setup.save_policy;
        let before_revision = active_plan_revision(&app);
        app.state.workbench.preflight.open = true;
        let mut invalid = before_policy;
        invalid.maximum_storage_bytes = 0;

        assert!(!commit_save_policy(
            &mut app,
            invalid,
            "Invalid save policy test"
        ));

        assert_eq!(app.state.sim_setup.save_policy, before_policy);
        assert_eq!(active_plan_revision(&app), before_revision);
        assert!(app.state.workbench.preflight.open);
        assert!(app.state.workbench.analysis_lifecycle_status.is_refusal());
    }

    #[test]
    fn failed_plan_commit_rolls_back_the_candidate_save_policy() {
        let mut app = RSpiceApp::test_instance();
        let before_policy = app.state.sim_setup.save_policy;
        let mut candidate = before_policy;
        candidate.retained_dataset_limit += 1;
        app.state.sim_setup.analysis_plan = None;

        assert!(!commit_save_policy(
            &mut app,
            candidate,
            "Save policy rollback test"
        ));

        assert_eq!(app.state.sim_setup.save_policy, before_policy);
        assert!(app.state.workbench.analysis_lifecycle_status.is_refusal());
    }
}
