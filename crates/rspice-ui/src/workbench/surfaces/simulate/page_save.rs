//! Save, streaming & retention policy.
//!
//! What a run keeps, how it is delivered while the solve is still running, and
//! what that costs. Every number here is the plan's own arithmetic over the
//! enabled outputs — the page never reports a measurement of a previous run as
//! a forecast of the next one.

mod groups;

use egui::Ui;

use crate::product::RunId;
use crate::simulation::capture_ledger::CaptureLedger;
use crate::state::workspace::SimulationPlanPayload;
use crate::state::{
    CaptureGroupMembership, RunRetention, SavedOutputPrecision, SavedOutputStreaming,
};
use crate::workbench::{AppState, RSpiceApp};

use crate::ui::widgets::select;

use super::page_kit::{
    Tone, card, card_body, card_note, card_row, field_pair, ledger_group, ledger_head, ledger_row,
    rule_row,
};

const DATASET_COLUMNS: [f32; 4] = [0.26, 0.14, 0.20, 0.40];

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let payload = plan_payload(app);
    let selection = app.simulation_controller.effective_saved_outputs_preflight(
        &app.state,
        &payload.saved_outputs,
        &payload.capture_groups,
    );
    let (effective_outputs, reports, automatic_fallback, membership, selection_error) =
        match selection {
            Ok((outputs, reports, automatic_fallback, membership)) => (
                outputs,
                reports,
                automatic_fallback,
                membership,
                None::<String>,
            ),
            Err(error) => (
                Vec::new(),
                Vec::new(),
                false,
                CaptureGroupMembership::default(),
                Some(error.to_string()),
            ),
        };
    // The page states one forecast, and this is where it comes from: the same
    // fold `validate_plan_saved_output_budget` prices the run with. The page
    // adds nothing to it.
    let ledger = CaptureLedger::resolve(
        &payload.capture_groups,
        &effective_outputs,
        &reports,
        &membership,
        app.state.sim_setup.save_policy.output_selection_mode,
        app.state.sim_setup.enabled_analysis_instance_count(),
        u64::try_from(app.state.sim_setup.run_set.point_count()).unwrap_or(u64::MAX),
    );
    // The card reports what was asked of it and the page acts on it, so the
    // card needs only the state it draws from — every command it can raise is
    // a plan transaction, and those belong to the frame that owns preflight.
    let command = groups::capture_groups(
        ui,
        &mut app.state,
        &effective_outputs,
        &ledger,
        &membership,
        automatic_fallback,
        selection_error.as_deref(),
    );
    if let Some(command) = command {
        groups::apply_group_command(app, &payload.capture_groups, command);
    }
    card_row(
        ui,
        &mut app.state,
        |ui, state| streaming_contract(ui, state, &payload),
        retention_contract,
    );
    super::pages::plan_configuration_receipts(ui, app);
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

fn streaming_contract(ui: &mut Ui, state: &mut AppState, payload: &SimulationPlanPayload) {
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
                let mut policy = state.sim_setup.save_policy;
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
                    commit_save_policy(state, policy, "Save policy · streaming and diagnostics");
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

fn retention_contract(ui: &mut Ui, state: &mut AppState) {
    let simulation = &state.simulation;
    let Some(plan_id) = state
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
    let limit = state.sim_setup.save_policy.retained_dataset_limit;
    let pinned = simulation.pinned_plan_run_count(plan_id);
    // Pinning can put the limit out of reach entirely. The card states that
    // rather than a count that reads as a policy still being enforced.
    let unenforceable = pinned >= limit;
    let at_limit = retained >= limit;
    let active_run_id = simulation.active_run().map(|run| run.run_id);
    let regression_baseline = state
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
        state.simulation.set_run_retention(run_id, retention);
    }
    if let Some(index) = picked
        && let Some(depth) = RETENTION_CHOICES.get(index)
    {
        let mut policy = state.sim_setup.save_policy;
        policy.retained_dataset_limit = *depth;
        if commit_save_policy(state, policy, "Save policy · dataset retention") {
            state.simulation.prune_plan_runs(plan_id, *depth);
        }
    }
}

fn commit_save_policy(
    state: &mut AppState,
    policy: crate::workbench::app_state::SimulationSavePolicy,
    detail: &str,
) -> bool {
    if policy == state.sim_setup.save_policy {
        return false;
    }
    if let Err(error) = policy.validate() {
        state
            .workbench
            .analysis_lifecycle_status
            .record_refusal(error);
        return false;
    }
    let previous = state.sim_setup.save_policy;
    state.sim_setup.save_policy = policy;
    match state
        .sim_setup
        .commit_active_plan_configuration_change(detail)
    {
        Ok(receipt) => {
            state
                .workbench
                .analysis_lifecycle_status
                .record_receipt(receipt.status_line());
            state.workbench.preflight.invalidate();
            true
        }
        Err(error) => {
            state.sim_setup.save_policy = previous;
            state
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
    use super::{DATASETS_LISTED, commit_save_policy, format_bytes, retention_ledger};
    use crate::workbench::app::RSpiceApp;

    fn active_plan_revision(app: &RSpiceApp) -> crate::product::ObjectRevision {
        app.state
            .sim_setup
            .analysis_plan
            .as_ref()
            .expect("test instance has an active plan")
            .revision()
    }

    /// One sealed receipt for `plan`, distinguished only by `byte`.
    fn plan_receipt(
        plan_id: crate::product::SimulationPlanId,
        byte: u8,
    ) -> crate::state::PreparedRunReceipt {
        let digest = |value: u8| crate::product::ContentDigest::from_bytes([value; 32]);
        crate::state::PreparedRunReceipt::new(
            crate::state::AnalysisResultSourceDomain::SimulationPlan,
            Some(plan_id),
            crate::product::ObjectRevision::INITIAL,
            digest(byte),
            digest(byte.wrapping_add(1)),
            crate::state::PreparedSourceCheckReceipt::SchematicDrc(digest(byte.wrapping_add(2))),
            vec![
                crate::state::PreparedRunTaskReceipt::new(
                    crate::product::AnalysisInstanceId::new(),
                    crate::product::ObjectRevision::INITIAL,
                    Vec::new(),
                    0,
                    digest(byte.wrapping_add(3)),
                )
                .expect("task receipt"),
            ],
        )
        .expect("plan receipt")
    }

    /// The ledger's own arithmetic is the whole claim the card makes about
    /// storage, so it is checked against a history whose deck bytes are known
    /// exactly and which straddles the listing cutoff in both directions.
    #[test]
    fn the_retention_ledger_accounts_for_every_retained_deck_byte_exactly() {
        let plan = crate::product::SimulationPlanId::new();
        let mut simulation = crate::state::SimulationState::default();
        let mut sequences = Vec::new();
        for byte in 0..8_u8 {
            sequences.push(
                simulation
                    .start_prepared_run(plan_receipt(plan, byte * 16))
                    .id,
            );
        }
        // Two of the oldest and two of the newest, which is both halves of the
        // ledger and exactly the four runs the archive can hold.
        let retained: Vec<(u64, usize)> = vec![
            (sequences[0], 100),
            (sequences[1], 250),
            (sequences[6], 1_000),
            (sequences[7], 4_096),
        ];
        for (sequence, bytes) in &retained {
            let deck: std::sync::Arc<str> = std::sync::Arc::from("d".repeat(*bytes));
            simulation
                .executed_decks
                .retain(crate::state::ExecutedDeck {
                    run_id: *sequence,
                    // Two points over one shared source: what the run costs is the
                    // source once, never once per point.
                    points: (0..2)
                        .map(|index| crate::state::ExecutedDeckPoint {
                            label: format!("point {index}"),
                            model_sources: Vec::new(),
                            deck: std::sync::Arc::clone(&deck),
                        })
                        .collect(),
                });
        }

        let (rows, summarized, decks) = retention_ledger(&simulation, plan, None, None);

        assert_eq!(rows.len(), DATASETS_LISTED);
        assert_eq!(summarized, 8 - DATASETS_LISTED);
        assert_eq!(decks.held, 4, "four runs' decks are held, not eight");
        assert_eq!(
            decks.total(),
            retained.iter().map(|(_, bytes)| *bytes as u64).sum::<u64>(),
            "the total is the shared sources once each"
        );
        assert_eq!(
            decks.listed.saturating_add(decks.summarized),
            decks.total(),
            "the printed rows and the printed tail partition the printed total"
        );
        assert_eq!(decks.listed, 1_000 + 4_096);
        assert_eq!(decks.summarized, 100 + 250);
        assert_eq!(
            rows.iter()
                .filter(|row| row.decks == "not retained")
                .count(),
            DATASETS_LISTED - 2,
            "a run whose decks are gone says so rather than reading as zero bytes"
        );
        assert_eq!(
            rows.iter()
                .map(|row| row.decks.as_str())
                .filter(|cell| *cell != "not retained")
                .collect::<Vec<_>>(),
            vec![format_bytes(4_096), format_bytes(1_000)],
            "and a run whose decks are held prints their exact size"
        );
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
            &mut app.state,
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
            &mut app.state,
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
            &mut app.state,
            candidate,
            "Save policy rollback test"
        ));

        assert_eq!(app.state.sim_setup.save_policy, before_policy);
        assert!(app.state.workbench.analysis_lifecycle_status.is_refusal());
    }
}
