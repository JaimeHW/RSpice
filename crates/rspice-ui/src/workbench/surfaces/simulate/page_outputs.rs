//! Outputs & expressions.
//!
//! The expressions this plan saves, and the semantic status each one resolves
//! to against the elaborated design. Status is the point of the page: an
//! expression that names a node the design does not have is worth catching
//! here rather than after a dispatched run.

use egui::Ui;

use crate::simulation::SavedOutputSemanticStatus;
use crate::state::workspace::SimulationPlanPayload;
use crate::workbench::RSpiceApp;

use super::page_kit::{
    Tone, card, card_body, card_note, card_row, ledger_head, ledger_row, rule_row,
};

const REGISTRY_COLUMNS: [f32; 5] = [0.20, 0.13, 0.28, 0.17, 0.22];

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let payload = plan_payload(app);
    registry(ui, app, &payload);
    card_row(
        ui,
        app,
        |ui, app| selected_record(ui, app, &payload),
        |ui, app| storage_read(ui, app, &payload),
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

fn registry(ui: &mut Ui, app: &mut RSpiceApp, payload: &SimulationPlanPayload) {
    let outputs = &payload.saved_outputs;
    let reports = app
        .simulation_controller
        .saved_outputs_preflight(&app.state, outputs);
    let invalid = reports
        .iter()
        .filter(|report| {
            matches!(
                report.semantic_status(),
                SavedOutputSemanticStatus::Invalid { .. }
            )
        })
        .count();
    let status = if invalid == 0 {
        format!("{} saved · all resolve", outputs.len())
    } else {
        format!("{invalid} of {} do not resolve", outputs.len())
    };
    let tone = if invalid == 0 { Tone::Ok } else { Tone::Error };
    let selected = app.state.workbench.selected_saved_output.clone();
    let mut pick = None;
    card(ui, "Saved outputs", Some((status.as_str(), tone)), |ui| {
        ledger_head(
            ui,
            &REGISTRY_COLUMNS,
            &["Name", "Kind", "Expression", "Save policy", "Status"],
        );
        if outputs.is_empty() {
            ledger_row(
                ui,
                &REGISTRY_COLUMNS,
                &[
                    ("No saved outputs", Tone::Neutral),
                    ("—", Tone::Neutral),
                    ("the run stores nothing", Tone::Warn),
                    ("—", Tone::Neutral),
                    ("—", Tone::Neutral),
                ],
                false,
            );
        }
        for (index, output) in outputs.iter().enumerate() {
            let (status_text, status_tone) = status_cell(reports.get(index));
            if ledger_row(
                ui,
                &REGISTRY_COLUMNS,
                &[
                    (output.name.as_str(), Tone::Accent),
                    (output.kind.label(), Tone::Neutral),
                    (output.source_expression.as_str(), Tone::Neutral),
                    (output.save_policy.label(), Tone::Neutral),
                    (status_text.as_str(), status_tone),
                ],
                selected.as_deref() == Some(output.name.as_str()),
            )
            .clicked()
            {
                pick = Some(output.name.clone());
            }
        }
        card_note(
            ui,
            "Status is resolved against the elaborated design, not against the text: an expression \
             that parses but names a node the design does not have is reported as unresolved here \
             rather than failing after dispatch.",
        );
    });
    if let Some(name) = pick {
        app.state.workbench.selected_saved_output = Some(name);
    }
}

fn status_cell(report: Option<&crate::simulation::SavedOutputPreflightReport>) -> (String, Tone) {
    match report.map(crate::simulation::SavedOutputPreflightReport::semantic_status) {
        Some(SavedOutputSemanticStatus::Valid { .. }) => ("resolves".to_owned(), Tone::Ok),
        Some(SavedOutputSemanticStatus::RuntimeBound { .. }) => {
            ("bound at run time".to_owned(), Tone::Warn)
        }
        Some(SavedOutputSemanticStatus::Invalid { reason }) => (reason.clone(), Tone::Error),
        None => ("no preflight report".to_owned(), Tone::Error),
    }
}

fn selected_record(ui: &mut Ui, app: &RSpiceApp, payload: &SimulationPlanPayload) {
    let selected = app
        .state
        .workbench
        .selected_saved_output
        .as_ref()
        .and_then(|name| {
            payload
                .saved_outputs
                .iter()
                .find(|output| &output.name == name)
        });
    let Some(output) = selected else {
        card(
            ui,
            "Selected output",
            Some(("none selected", Tone::Neutral)),
            |ui| {
                card_note(
                    ui,
                    "Select a row above to see the complete contract: which analyses it applies \
                     to, how it is stored, and whether it is streamed while the solve runs.",
                );
            },
        );
        return;
    };
    let title = format!("Selected output · {}", output.name);
    card(
        ui,
        &title,
        Some((output.kind.label(), Tone::Neutral)),
        |ui| {
            card_body(ui, |ui| {
                rule_row(ui, "Expression", &output.source_expression);
                rule_row(ui, "Applies to", output.compatible_analyses.label());
                rule_row(ui, "Save policy", output.save_policy.label());
                rule_row(ui, "Stored precision", output.stored_precision.label());
                rule_row(ui, "Streaming", output.streaming.label());
            });
        },
    );
}

/// What a run will retain, summed from the same per-output preflight reports
/// the registry shows.
///
/// Outputs whose size is indeterminate are counted separately rather than
/// folded in as zero: a total that silently omitted them would understate the
/// budget by exactly the outputs the plan knows least about.
fn storage_read(ui: &mut Ui, app: &RSpiceApp, payload: &SimulationPlanPayload) {
    use crate::simulation::SavedOutputStorageEstimate;

    let reports = app
        .simulation_controller
        .saved_outputs_preflight(&app.state, &payload.saved_outputs);
    let mut exact_bytes: u64 = 0;
    let mut indeterminate = Vec::new();
    let mut tasks = 0usize;
    for (output, report) in payload.saved_outputs.iter().zip(&reports) {
        tasks += report.compatible_analysis_count();
        match report.storage_estimate() {
            SavedOutputStorageEstimate::ExactBytes(bytes) => {
                exact_bytes = exact_bytes.saturating_add(*bytes);
            }
            SavedOutputStorageEstimate::Indeterminate { .. } => {
                indeterminate.push(output.name.clone());
            }
        }
    }
    card(
        ui,
        "What a run will retain",
        Some(("derived from the plan", Tone::Neutral)),
        |ui| {
            card_body(ui, |ui| {
                rule_row(ui, "Bounded outputs", &format_bytes(exact_bytes));
                rule_row(
                    ui,
                    "Compatible tasks",
                    &format!("{tasks} · summed over every enabled analysis"),
                );
                rule_row(
                    ui,
                    "Not yet bounded",
                    &if indeterminate.is_empty() {
                        "none".to_owned()
                    } else {
                        indeterminate.join(" · ")
                    },
                );
            });
            card_note(
                ui,
                "The total is the plan's own arithmetic over the enabled outputs and their \
                 compatible tasks, not a measurement of a previous run. An output whose size \
                 cannot be bounded before the solve is listed rather than counted as zero.",
            );
        },
    );
}

fn format_bytes(bytes: u64) -> String {
    const MIB: f64 = 1024.0 * 1024.0;
    let mib = bytes as f64 / MIB;
    if mib >= 1.0 {
        format!("{mib:.2} MiB")
    } else {
        format!("{:.1} KiB", bytes as f64 / 1024.0)
    }
}
