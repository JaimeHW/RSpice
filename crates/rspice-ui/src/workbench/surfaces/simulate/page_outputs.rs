//! Outputs & expressions.
//!
//! The expressions this plan saves, and the semantic status each one resolves
//! to against the elaborated design. Status is the point of the page: an
//! expression that names a node the design does not have is worth catching
//! here rather than after a dispatched run.

use egui::Ui;

use crate::product::SimulationPlanId;
use crate::simulation::SavedOutputSemanticStatus;
use crate::state::workspace::SimulationPlanPayload;
use crate::state::{
    SavedOutput, SavedOutputCompatibility, SavedOutputPolicy, SavedOutputPrecision,
    SavedOutputStreaming,
};
use crate::ui::widgets::{Button, select};
use crate::workbench::RSpiceApp;

use super::page_kit::{
    Tone, card, card_body, card_head_row, card_note, card_row, card_with_head, field_pair,
    ledger_head, ledger_row, rule_row,
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
    let selected_output = selected
        .as_deref()
        .and_then(|name| outputs.iter().find(|output| output.name == name))
        .map(|output| (output.id, output.name.clone()));
    let mut pick = None;
    let mut remove = false;
    card_with_head(
        ui,
        |ui| {
            card_head_row(ui, "Saved outputs", Some((status.as_str(), tone)), |ui| {
                remove = Button::new("Remove")
                    .enabled(selected_output.is_some())
                    .show(ui)
                    .clicked();
            });
        },
        |ui| {
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
        },
    );
    if remove
        && let Some((output_id, name)) = selected_output
        && let Ok(plan_id) = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .map(|plan| plan.id())
    {
        let detail = format!("Removed saved output {name}.");
        remove_output(app, plan_id, output_id, &detail);
        app.state.workbench.selected_saved_output = None;
    }
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

/// The selected output's complete capture contract, editable in place.
///
/// These four fields decide what a run keeps and at what cost, and the Save &
/// streaming page reports totals over exactly them. Presenting them read-only
/// would leave an engineer able to see a capture budget they had no way to
/// change short of deleting the output and authoring it again.
fn selected_record(ui: &mut Ui, app: &mut RSpiceApp, payload: &SimulationPlanPayload) {
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
        })
        .cloned();
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
    let scopes = compatibility_options(app);
    let policies: Vec<String> = SavedOutputPolicy::ALL
        .iter()
        .map(|policy| policy.label().to_owned())
        .collect();
    let precisions: Vec<String> = SavedOutputPrecision::ALL
        .iter()
        .map(|precision| precision.label().to_owned())
        .collect();
    let streamings: Vec<String> = SavedOutputStreaming::ALL
        .iter()
        .map(|streaming| streaming.label().to_owned())
        .collect();
    let current_scope = scope_label(&output.compatible_analyses, &scopes);
    let scope_labels: Vec<String> = scopes.iter().map(|(label, _)| label.clone()).collect();

    // One `Cell` rather than four mutable captures: `field_pair` holds two
    // controls at once, and two closures cannot borrow the same slot mutably.
    let edit: std::cell::Cell<Option<OutputEdit>> = std::cell::Cell::new(None);
    card(
        ui,
        &title,
        Some((output.kind.label(), Tone::Neutral)),
        |ui| {
            card_body(ui, |ui| {
                rule_row(ui, "Expression", &output.source_expression);
                field_pair(
                    ui,
                    ("Applies to", &mut |ui: &mut Ui, width: f32| {
                        if let Some(index) = select(
                            ui,
                            "simulation.outputs.scope",
                            "Applies to",
                            &current_scope,
                            &scope_labels,
                            width,
                        ) {
                            edit.set(Some(OutputEdit::Scope(scopes[index].1.clone())));
                        }
                    }),
                    Some(("Save policy", &mut |ui: &mut Ui, width: f32| {
                        if let Some(index) = select(
                            ui,
                            "simulation.outputs.policy",
                            "Save policy",
                            output.save_policy.label(),
                            &policies,
                            width,
                        ) {
                            edit.set(Some(OutputEdit::Policy(SavedOutputPolicy::ALL[index])));
                        }
                    })),
                );
                field_pair(
                    ui,
                    ("Stored precision", &mut |ui: &mut Ui, width: f32| {
                        if let Some(index) = select(
                            ui,
                            "simulation.outputs.precision",
                            "Stored precision",
                            output.stored_precision.label(),
                            &precisions,
                            width,
                        ) {
                            edit.set(Some(OutputEdit::Precision(
                                SavedOutputPrecision::ALL[index],
                            )));
                        }
                    }),
                    Some(("Streaming", &mut |ui: &mut Ui, width: f32| {
                        if let Some(index) = select(
                            ui,
                            "simulation.outputs.streaming",
                            "Streaming",
                            output.streaming.label(),
                            &streamings,
                            width,
                        ) {
                            edit.set(Some(OutputEdit::Streaming(
                                SavedOutputStreaming::ALL[index],
                            )));
                        }
                    })),
                );
            });
            card_note(
                ui,
                "Changing a capture policy is a plan-configuration change: it moves the plan \
                 revision, so a preflight taken against the previous policy no longer authorizes \
                 a dispatch.",
            );
        },
    );
    if let Some(edit) = edit.take() {
        let detail = edit.detail(&output.name, &scopes);
        replace_output(app, output.id, &detail, |target| edit.apply(target));
    }
}

/// One accepted change to a saved output's capture contract.
///
/// Kept as data rather than a closure so the four controls can write to a
/// single slot, and so the receipt wording lives next to the change it
/// describes.
enum OutputEdit {
    Scope(SavedOutputCompatibility),
    Policy(SavedOutputPolicy),
    Precision(SavedOutputPrecision),
    Streaming(SavedOutputStreaming),
}

impl OutputEdit {
    fn apply(self, target: &mut SavedOutput) {
        match self {
            Self::Scope(scope) => target.compatible_analyses = scope,
            Self::Policy(policy) => target.save_policy = policy,
            Self::Precision(precision) => target.stored_precision = precision,
            Self::Streaming(streaming) => target.streaming = streaming,
        }
    }

    fn detail(&self, name: &str, scopes: &[(String, SavedOutputCompatibility)]) -> String {
        match self {
            Self::Scope(scope) => format!(
                "Scoped saved output {name} to {}.",
                scope_label(scope, scopes)
            ),
            Self::Policy(policy) => format!(
                "Set saved output {name} to capture {}.",
                policy.label().to_lowercase()
            ),
            Self::Precision(precision) => {
                format!("Stored saved output {name} as {}.", precision.label())
            }
            Self::Streaming(streaming) => format!(
                "Set saved output {name} to {}.",
                streaming.label().to_lowercase()
            ),
        }
    }
}

/// The scopes an output may apply to: the two fixed contracts, then one entry
/// per analysis the plan actually owns.
///
/// Analyses are listed by their execution position so two instances of the
/// same kind stay distinguishable, which a bare kind label could not do.
fn compatibility_options(app: &RSpiceApp) -> Vec<(String, SavedOutputCompatibility)> {
    let mut options = vec![
        (
            SavedOutputCompatibility::AllCompatibleAnalyses
                .label()
                .to_owned(),
            SavedOutputCompatibility::AllCompatibleAnalyses,
        ),
        (
            SavedOutputCompatibility::OpTranAc.label().to_owned(),
            SavedOutputCompatibility::OpTranAc,
        ),
    ];
    if let Ok(plan) = app.state.sim_setup.stable_analysis_plan() {
        options.extend(plan.instances().iter().enumerate().map(|(index, instance)| {
            (
                format!("Only #{} · {}", index + 1, instance.kind().code()),
                SavedOutputCompatibility::SelectedAnalysis {
                    analysis_id: instance.id(),
                },
            )
        }));
    }
    options
}

/// The label for a stored scope, resolved against the offered options.
///
/// An analysis-scoped output whose analysis no longer exists says so rather
/// than falling back to a scope the record does not hold.
fn scope_label(
    scope: &SavedOutputCompatibility,
    options: &[(String, SavedOutputCompatibility)],
) -> String {
    options
        .iter()
        .find(|(_, candidate)| candidate == scope)
        .map_or_else(
            || "Selected analysis · no longer in the plan".to_owned(),
            |(label, _)| label.clone(),
        )
}

/// Replace one saved output as a single plan-configuration transaction.
fn replace_output(
    app: &mut RSpiceApp,
    output_id: crate::product::SavedOutputId,
    detail: &str,
    apply: impl FnOnce(&mut SavedOutput),
) {
    let Ok(plan_id) = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| plan.id())
    else {
        return;
    };
    let Some(mut replacement) = app
        .state
        .workspace
        .active_plan_data(plan_id)
        .and_then(|payload| {
            payload
                .saved_outputs
                .iter()
                .find(|output| output.id == output_id)
                .cloned()
        })
    else {
        return;
    };
    apply(&mut replacement);

    let mut workspace = app.state.workspace.clone();
    let mut setup = app.state.sim_setup.clone();
    let outcome = workspace
        .replace_saved_output(plan_id, output_id, replacement)
        .map_err(|error| error.to_string())
        .and_then(|_| {
            setup
                .commit_active_plan_configuration_change(detail.to_owned())
                .map_err(|error| error.to_string())
        });
    match outcome {
        Ok(receipt) => {
            app.state.workspace = workspace;
            app.state.sim_setup = setup;
            app.invalidate_simulation_preflight();
            app.state.workbench.analysis_lifecycle_status = receipt.status_line();
        }
        Err(error) => app.state.workbench.analysis_lifecycle_status = error,
    }
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

/// Remove one saved output as a single plan-configuration transaction.
fn remove_output(
    app: &mut RSpiceApp,
    plan_id: SimulationPlanId,
    output_id: crate::product::SavedOutputId,
    detail: &str,
) {
    let mut workspace = app.state.workspace.clone();
    let mut setup = app.state.sim_setup.clone();
    let outcome = workspace
        .remove_saved_output(plan_id, output_id)
        .map_err(|error| error.to_string())
        .and_then(|_| {
            setup
                .commit_active_plan_configuration_change(detail.to_owned())
                .map_err(|error| error.to_string())
        });
    match outcome {
        Ok(receipt) => {
            app.state.workspace = workspace;
            app.state.sim_setup = setup;
            app.invalidate_simulation_preflight();
            app.state.workbench.analysis_lifecycle_status = receipt.status_line();
        }
        Err(error) => app.state.workbench.analysis_lifecycle_status = error,
    }
}
