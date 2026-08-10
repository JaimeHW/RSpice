//! Save, streaming & retention policy.
//!
//! What a run keeps, how it is delivered while the solve is still running, and
//! what that costs. Every number here is the plan's own arithmetic over the
//! enabled outputs — the page never reports a measurement of a previous run as
//! a forecast of the next one.

use egui::Ui;

use crate::simulation::{SavedOutputStorageEstimate, output_contract::SavedOutputPreflightReport};
use crate::state::workspace::SimulationPlanPayload;
use crate::state::{SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming};
use crate::workbench::RSpiceApp;

use super::page_kit::{
    Tone, card, card_body, card_note, card_row, ledger_head, ledger_row, rule_row,
};

const GROUP_COLUMNS: [f32; 4] = [0.34, 0.20, 0.22, 0.24];

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
                 is counted as not yet bounded rather than as zero.",
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

fn retention_contract(ui: &mut Ui, app: &RSpiceApp) {
    let retained = app.state.simulation.runs.len();
    let active = app.state.simulation.active_run().map_or_else(
        || "none loaded".to_owned(),
        |run| format!("Run {} · immutable", run.id),
    );
    card(
        ui,
        "Retention",
        Some(("committed datasets are immutable", Tone::Ok)),
        |ui| {
            card_body(ui, |ui| {
                rule_row(ui, "Retained datasets", &retained.to_string());
                rule_row(ui, "Active dataset", &active);
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
            });
            card_note(
                ui,
                "A dataset is sealed against the manifest that produced it. Changing the plan and \
                 running again produces a second dataset to compare against the first, which is \
                 why a retained result can still be trusted after the plan has moved on.",
            );
        },
    );
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
