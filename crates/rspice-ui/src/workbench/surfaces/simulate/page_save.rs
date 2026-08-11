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
use crate::state::{RunRetention, SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming};
use crate::workbench::RSpiceApp;

use crate::ui::widgets::select;

use super::page_kit::{
    Tone, card, card_body, card_note, card_row, field_pair, ledger_group, ledger_head, ledger_row,
    rule_row,
};

const GROUP_COLUMNS: [f32; 4] = [0.34, 0.20, 0.22, 0.24];
const DATASET_COLUMNS: [f32; 3] = [0.32, 0.24, 0.44];

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let payload = plan_payload(app);
    let reports = app
        .simulation_controller
        .saved_outputs_preflight(&app.state, &payload.saved_outputs);
    capture_groups(ui, &payload, &reports);
    card_row(
        ui,
        app,
        |ui, _| streaming_contract(ui, &payload),
        |ui, app| retention_contract(ui, app),
    );
}

/// Retention depths the page offers.
///
/// A bounded set rather than free text: the number is a promise about how
/// much of the reader's work survives, and a typo in a text field is a
/// promise nobody meant to make.
const RETENTION_CHOICES: [usize; 5] = [5, 10, 20, 50, 200];

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
                              immediately, and raising it never recovers one already discarded.";

/// The rules that mean nothing until there is a dataset to act on.
const RETENTION_LIST_NOTE: &str = "The highlighted row is the active dataset. Selecting a row \
                                   pins it as a golden baseline or releases it: a baseline \
                                   survives the limit, and a released one becomes eligible for \
                                   the next run to discard. Releasing never destroys the dataset \
                                   on the spot, so a project can sit over its limit until it runs \
                                   again.";

/// One retained dataset, as the retention card states it.
struct RetentionRow {
    run_id: RunId,
    dataset: String,
    analyses: String,
    pinned: bool,
    active: bool,
}

fn plan_payload(app: &RSpiceApp) -> SimulationPlanPayload {
    app.state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())
        .and_then(|plan_id| app.state.workspace.active_plan_data(plan_id).cloned())
        .unwrap_or_default()
}

/// Outputs grouped by save policy, because the policy is what decides whether
/// a signal is retained at full rate, decimated, or not stored at all.
fn capture_groups(
    ui: &mut Ui,
    payload: &SimulationPlanPayload,
    reports: &[SavedOutputPreflightReport],
) {
    let mut groups: Vec<(SavedOutputPolicy, usize, u64, usize)> = SavedOutputPolicy::ALL
        .iter()
        .map(|policy| (*policy, 0usize, 0u64, 0usize))
        .collect();
    for (output, report) in payload.saved_outputs.iter().zip(reports) {
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
    }
    let total: u64 = groups.iter().map(|(_, _, bytes, _)| bytes).sum();
    let unbounded: usize = groups.iter().map(|(_, _, _, count)| count).sum();
    let status = if unbounded == 0 {
        format!("{} bounded", format_bytes(total))
    } else {
        format!(
            "{} bounded · {unbounded} not yet bounded",
            format_bytes(total)
        )
    };
    card(
        ui,
        "Capture groups",
        Some((
            status.as_str(),
            if unbounded == 0 { Tone::Ok } else { Tone::Warn },
        )),
        |ui| {
            ledger_head(
                ui,
                &GROUP_COLUMNS,
                &["Save policy", "Outputs", "Bounded size", "Not yet bounded"],
            );
            for (policy, count, bytes, indeterminate) in &groups {
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
            card_note(
                ui,
                "Size is summed from each output's own preflight estimate across the tasks it is \
                 compatible with. An output whose sample count cannot be known before the solve \
                 is counted as not yet bounded rather than as zero. Which group an output falls \
                 in is its save policy, set on Outputs & expressions.",
            );
        },
    );
}

fn streaming_contract(ui: &mut Ui, payload: &SimulationPlanPayload) {
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

fn retention_contract(ui: &mut Ui, app: &mut RSpiceApp) {
    let simulation = &app.state.simulation;
    let retained = simulation.runs.len();
    let limit = simulation.effective_retained_dataset_limit();
    let pinned = simulation.pinned_run_count();
    // Pinning can put the limit out of reach entirely. The card states that
    // rather than a count that reads as a policy still being enforced.
    let unenforceable = simulation.retention_limit_is_unenforceable();
    let at_limit = retained >= limit;
    let active_run_id = simulation.active_run().map(|run| run.run_id);
    let mut rows: Vec<RetentionRow> = Vec::new();
    let mut summarized = 0usize;
    for (index, run) in simulation.runs.iter().enumerate() {
        let is_pinned = run.retention().is_pinned();
        if index >= DATASETS_LISTED && !is_pinned {
            summarized += 1;
            continue;
        }
        rows.push(RetentionRow {
            run_id: run.run_id,
            dataset: format!("Run {}", run.id),
            analyses: run.analyses.len().to_string(),
            pinned: is_pinned,
            active: Some(run.run_id) == active_run_id,
        });
    }
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
            });
            if !rows.is_empty() {
                ledger_head(ui, &DATASET_COLUMNS, &["Dataset", "Analyses", "Retention"]);
                for row in &rows {
                    let response = ledger_row(
                        ui,
                        &DATASET_COLUMNS,
                        &[
                            (row.dataset.as_str(), Tone::Neutral),
                            (row.analyses.as_str(), Tone::Neutral),
                            if row.pinned {
                                ("golden baseline", Tone::Ok)
                            } else {
                                ("pruneable", Tone::Neutral)
                            },
                        ],
                        row.active,
                    );
                    if response.clicked() {
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
                    ledger_group(ui, &format!("{summarized} older · pruneable · not listed"));
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
        app.state.simulation.set_retained_dataset_limit(*depth);
    }
}

fn format_bytes(bytes: u64) -> String {
    const MIB: f64 = 1024.0 * 1024.0;
    let mib = bytes as f64 / MIB;
    if mib >= 1.0 {
        format!("{mib:.2} MiB")
    } else if bytes > 0 {
        format!("{:.1} KiB", bytes as f64 / 1024.0)
    } else {
        "0".to_owned()
    }
}
