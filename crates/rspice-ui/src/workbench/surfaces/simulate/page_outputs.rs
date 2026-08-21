//! Outputs & expressions.
//!
//! The expressions this plan saves, and the semantic status each one resolves
//! to against the elaborated design. Status is the point of the page: an
//! expression that names a node the design does not have is worth catching
//! here rather than after a dispatched run.

use egui::Ui;

use crate::simulation::SavedOutputSemanticStatus;
use crate::state::workspace::SimulationPlanPayload;
use crate::state::{
    SavedOutput, SavedOutputCompatibility, SavedOutputPolicy, SavedOutputPrecision,
    SavedOutputStreaming,
};
use crate::ui::widgets::{Button, mono_input, select};
use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::Command;

use super::page_kit::{
    Tone, card, card_body, card_head_row, card_note, card_row, card_with_head, field_pair,
    filter_field, filter_row, ledger_head, ledger_row, row_matches, rule_row,
};
use super::workflows::{commit_plan_change, unique_copy_name};

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
    super::pages::plan_configuration_receipts(ui, app);
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

fn registry(ui: &mut Ui, app: &mut RSpiceApp, payload: &SimulationPlanPayload) {
    let outputs = &payload.saved_outputs;
    let reports = app
        .simulation_controller
        .saved_outputs_preflight(&app.state, outputs);
    let query = app.state.workbench.saved_output_filter.clone();
    let rows: Vec<(String, Tone, bool)> = outputs
        .iter()
        .enumerate()
        .map(|(index, output)| {
            let (text, tone) = status_cell(reports.get(index));
            let shown = row_matches(
                &query,
                &[
                    output.name.as_str(),
                    output.kind.label(),
                    output.source_expression.as_str(),
                    output.save_policy.label(),
                    text.as_str(),
                ],
            );
            (text, tone, shown)
        })
        .collect();
    let shown = rows.iter().filter(|(_, _, shown)| *shown).count();
    let (mut status, tone) = output_registry_summary(
        reports.iter().map(|report| report.semantic_status()),
        outputs.len(),
    );
    // The count of what the filter left standing belongs beside the registry's
    // own count, not instead of it: the plan still holds every output, and a
    // head that reported only the visible ones would understate what a run
    // will capture.
    if shown != outputs.len() {
        status.push_str(&format!(" · showing {shown} of {}", outputs.len()));
    }
    let selected = app.state.workbench.selected_saved_output.clone();
    let selected_output = selected
        .as_deref()
        .and_then(|name| outputs.iter().find(|output| output.name == name))
        .map(|output| (output.id, output.name.clone()));
    let mut pick = None;
    let mut remove = false;
    let mut duplicate = false;
    card_with_head(
        ui,
        |ui| {
            card_head_row(ui, "Saved outputs", Some((status.as_str(), tone)), |ui| {
                remove = Button::new("Remove")
                    .enabled(selected_output.is_some())
                    .show(ui)
                    .clicked();
                duplicate = Button::new("Duplicate")
                    .enabled(selected_output.is_some())
                    .show(ui)
                    .clicked();
            });
        },
        |ui| {
            // Withheld over an empty registry: a search box with nothing to
            // match is a control that reaches nothing.
            if !outputs.is_empty() {
                filter_row(ui, |ui| {
                    filter_field(
                        ui,
                        "simulation-plan.output-registry.filter",
                        "Filter name, expression or status…",
                        &mut app.state.workbench.saved_output_filter,
                    );
                });
            }
            ledger_head(
                ui,
                &REGISTRY_COLUMNS,
                &["Name", "Kind", "Expression", "Save policy", "Status"],
            );
            // An empty registry and a filter that matched nothing are
            // different facts with different fixes, so they cannot share a
            // row: one says the run will store nothing, the other says the
            // reader is looking through a narrowed view of outputs that are
            // still there.
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
            } else if shown == 0 {
                let held = format!("{} saved · clear the filter to see them", outputs.len());
                ledger_row(
                    ui,
                    &REGISTRY_COLUMNS,
                    &[
                        ("No output matches the filter", Tone::Warn),
                        ("—", Tone::Neutral),
                        (held.as_str(), Tone::Neutral),
                        ("—", Tone::Neutral),
                        ("—", Tone::Neutral),
                    ],
                    false,
                );
            }
            for (output, (status_text, status_tone, visible)) in outputs.iter().zip(&rows) {
                if !visible {
                    continue;
                }
                if ledger_row(
                    ui,
                    &REGISTRY_COLUMNS,
                    &[
                        (output.name.as_str(), Tone::Accent),
                        (output.kind.label(), Tone::Neutral),
                        (output.source_expression.as_str(), Tone::Neutral),
                        (output.save_policy.label(), Tone::Neutral),
                        (status_text.as_str(), *status_tone),
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
             rather than failing after dispatch. The filter is a view of this registry and nothing \
             more: it hides rows, never outputs, and a run captures every one the plan holds.",
            );
        },
    );
    if let Some((output_id, name)) = selected_output
        && let Ok(plan_id) = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .map(|plan| plan.id())
    {
        if duplicate {
            let copy = unique_copy_name(&name, |candidate| {
                outputs
                    .iter()
                    .any(|output| output.name.eq_ignore_ascii_case(candidate))
            });
            let detail = format!("Duplicated saved output {name} as {copy}.");
            let committed = copy.clone();
            if commit_plan_change(app, plan_id, &detail, move |workspace, plan_id| {
                workspace
                    .duplicate_saved_output(plan_id, output_id, copy)
                    .map(|_| ())
                    .map_err(|error| error.to_string())
            }) {
                // Selecting the copy is what makes duplicating useful: every
                // capture decision is already carried over, so the record
                // editor opens on the one field the copy exists to change.
                app.state.workbench.saved_output_name_draft = None;
                app.state.workbench.saved_output_expression_draft = None;
                app.state.workbench.selected_saved_output = Some(committed);
            }
        } else if remove {
            let detail = format!("Removed saved output {name}.");
            // A refused removal leaves the record in the registry, so the
            // selection has to survive with it — clearing regardless would
            // close the editor on a row that is still there.
            if commit_plan_change(app, plan_id, &detail, move |workspace, plan_id| {
                workspace
                    .remove_saved_output(plan_id, output_id)
                    .map(|_| ())
                    .map_err(|error| error.to_string())
            }) {
                app.state.workbench.selected_saved_output = None;
            }
        }
    }
    if let Some(name) = pick {
        // A draft belongs to the row it was typed into. Selecting another row
        // must not carry a half-typed name or expression onto it.
        if app.state.workbench.selected_saved_output.as_deref() != Some(name.as_str()) {
            app.state.workbench.saved_output_name_draft = None;
            app.state.workbench.saved_output_expression_draft = None;
        }
        app.state.workbench.selected_saved_output = Some(name);
    }
}

/// Summarize the exact semantic states represented by the registry rows.
///
/// A runtime-bound output is intentionally neither invalid nor resolved: its
/// source evidence cannot exist until execution. Keeping that third state in
/// the heading prevents a warning row from being contradicted by an "all
/// resolve" success badge. A missing report is a controller contract failure
/// and is counted as unresolved instead of disappearing from the total.
fn output_registry_summary<'a>(
    statuses: impl IntoIterator<Item = &'a SavedOutputSemanticStatus>,
    expected: usize,
) -> (String, Tone) {
    if expected == 0 {
        return ("0 saved · nothing configured".to_owned(), Tone::Neutral);
    }

    let mut valid = 0usize;
    let mut runtime_bound = 0usize;
    let mut invalid = 0usize;
    let mut seen = 0usize;
    for status in statuses {
        seen = seen.saturating_add(1);
        match status {
            SavedOutputSemanticStatus::Valid { .. } => valid = valid.saturating_add(1),
            SavedOutputSemanticStatus::RuntimeBound { .. } => {
                runtime_bound = runtime_bound.saturating_add(1);
            }
            SavedOutputSemanticStatus::Invalid { .. } => invalid = invalid.saturating_add(1),
        }
    }
    invalid = invalid.saturating_add(expected.saturating_sub(seen));

    if invalid > 0 {
        (
            format!("{invalid} of {expected} do not resolve"),
            Tone::Error,
        )
    } else if runtime_bound > 0 {
        (
            format!("{expected} saved · {valid} resolve · {runtime_bound} bind at run time"),
            Tone::Warn,
        )
    } else {
        (format!("{expected} saved · all resolve"), Tone::Ok)
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
    // Which group this output ends up in, and whether it got there by being
    // named or by matching a rule. The two read differently on purpose: an
    // engineer needs to know whether moving a rule would move this output.
    let capture_owner = crate::state::CaptureGroupMembership::resolve(
        &payload.capture_groups,
        &payload.saved_outputs,
    );
    let owner_name = payload
        .saved_outputs
        .iter()
        .position(|candidate| candidate.id == output.id)
        .map(|index| capture_owner.owner(index))
        .and_then(|id| payload.capture_groups.iter().find(|group| group.id == id))
        .map_or_else(
            || crate::state::UNGROUPED_NAME.to_owned(),
            |group| group.name.clone(),
        );
    let named_by = payload
        .capture_groups
        .iter()
        .find(|group| group.members.contains(&output.id));
    let capture_current = named_by.map_or_else(
        || format!("By rule · {owner_name}"),
        |group| group.name.clone(),
    );
    let mut capture_choices = vec![CAPTURE_BY_RULE.to_owned()];
    capture_choices.extend(
        payload
            .capture_groups
            .iter()
            .map(|group| group.name.clone()),
    );

    let mut name = app
        .state
        .workbench
        .saved_output_name_draft
        .clone()
        .unwrap_or_else(|| output.name.clone());
    let mut expression = app
        .state
        .workbench
        .saved_output_expression_draft
        .clone()
        .unwrap_or_else(|| output.source_expression.clone());

    // One `Cell` rather than four mutable captures: `field_pair` holds two
    // controls at once, and two closures cannot borrow the same slot mutably.
    let edit: std::cell::Cell<Option<OutputEdit>> = std::cell::Cell::new(None);
    // Membership is not a field of the output — it belongs to the group that
    // claims it — so it cannot ride on `OutputEdit`. `Some(None)` is the
    // request to stop naming this output and let the rules decide again.
    let membership: std::cell::Cell<Option<Option<crate::product::CaptureGroupId>>> =
        std::cell::Cell::new(None);
    let released = std::cell::Cell::new(false);
    let handoff: std::cell::Cell<Option<EditorHandoff>> = std::cell::Cell::new(None);
    let view_trace = std::cell::Cell::new(false);
    let dataset = super::output_evidence::selected_plan_dataset(app).is_some();
    let dataset_rule = if app.state.simulation.active_run().is_some() {
        "The active dataset does not belong to this simulation plan, so it cannot supply measurement or unit evidence here."
    } else {
        EditorHandoff::DATASET_RULE
    };
    // Resolved once, before the card draws: the control's label promises a
    // retained trace, so what it costs to know whether one exists is paid
    // here rather than after a click that would have to apologise.
    let trace = materialized_trace(app, &output);
    card(
        ui,
        &title,
        Some((output.kind.label(), Tone::Neutral)),
        |ui| {
            card_body(ui, |ui| {
                // A mistyped node name is the most common thing wrong with a
                // saved output, and re-authoring the whole contract to fix one
                // character is not an edit path.
                field_pair(
                    ui,
                    ("Name", &mut |ui: &mut Ui, width: f32| {
                        released
                            .set(released.get() | mono_input(ui, &mut name, width).lost_focus());
                    }),
                    Some(("Expression", &mut |ui: &mut Ui, width: f32| {
                        released.set(
                            released.get() | mono_input(ui, &mut expression, width).lost_focus(),
                        );
                    })),
                );
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
                field_pair(
                    ui,
                    ("Capture group", &mut |ui: &mut Ui, width: f32| {
                        if let Some(index) = select(
                            ui,
                            "simulation.outputs.capture-group",
                            "Capture group",
                            &capture_current,
                            &capture_choices,
                            width,
                        ) {
                            membership.set(Some(
                                index
                                    .checked_sub(1)
                                    .and_then(|position| payload.capture_groups.get(position))
                                    .map(|group| group.id),
                            ));
                        }
                    }),
                    None,
                );
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);
                    for action in EditorHandoff::ALL {
                        let enabled = dataset || !action.needs_dataset();
                        let response = Button::new(action.label()).enabled(enabled).show(ui);
                        if !enabled {
                            response.on_hover_text(dataset_rule);
                        } else if response.on_hover_text(action.hint()).clicked() {
                            handoff.set(Some(action));
                        }
                    }
                    // Not an `EditorHandoff`: those three are existing
                    // commands, and this one resolves a waveform inside a
                    // specific retained analysis, which no command names.
                    let response = Button::new("View trace").enabled(trace.is_ok()).show(ui);
                    match &trace {
                        Ok(_) => {
                            if response
                                .on_hover_text(
                                    "Open the retained waveform this output materialized, with it \
                                     selected in Results",
                                )
                                .clicked()
                            {
                                view_trace.set(true);
                            }
                        }
                        Err(reason) => {
                            response.on_hover_text(reason);
                        }
                    }
                });
            });
            card_note(
                ui,
                "Changing a capture policy is a plan-configuration change: it moves the plan \
                 revision, so a preflight taken against the previous policy no longer authorizes \
                 a dispatch. The calculator composes an expression against this plan; measurement \
                 authoring and unit diagnostics read a retained dataset, so they stay disabled \
                 until a run has been kept.",
            );
        },
    );
    // Naming an output into a group is not an edit to the output — the group
    // holds the list — so it does not go through `replace_output`. It goes
    // through the same plan transaction, which is what makes it produce a
    // receipt and re-resolve membership like every other registry edit.
    if let Some(request) = membership.take() {
        let held = named_by.map(|group| group.id);
        let target = request.or(held);
        if held != request
            && let Some(target) = target
            && let Ok(plan_id) = app
                .state
                .sim_setup
                .stable_analysis_plan()
                .map(|plan| plan.id())
        {
            let output_id = output.id;
            let named = request.is_some();
            let detail = match request
                .and_then(|id| payload.capture_groups.iter().find(|group| group.id == id))
            {
                Some(group) => format!(
                    "Named saved output {} into capture group {}.",
                    output.name, group.name
                ),
                None => format!(
                    "Released saved output {} to capture-group rules.",
                    output.name
                ),
            };
            commit_plan_change(app, plan_id, &detail, move |workspace, plan_id| {
                workspace
                    .set_capture_group_member(plan_id, target, output_id, named)
                    .map_err(|error| error.to_string())
            });
        }
    } else if let Some(edit) = edit.take() {
        let detail = edit.detail(&output.name, &scopes);
        replace_output(app, output.id, &detail, |target| edit.apply(target));
    } else if released.get() {
        let renamed = name.trim() != output.name;
        let rewritten = expression.trim() != output.source_expression;
        if renamed || rewritten {
            let detail = if renamed {
                format!("Renamed saved output {} to {}.", output.name, name.trim())
            } else {
                format!(
                    "Rewrote saved output {} as {}.",
                    output.name,
                    expression.trim()
                )
            };
            let committed = replace_output(app, output.id, &detail, |target| {
                target.name = name.trim().to_owned();
                target.source_expression = expression.trim().to_owned();
            });
            // The registry selects by name, so a rename has to carry the
            // selection with it or the record would deselect itself.
            if committed && renamed {
                app.state.workbench.selected_saved_output = Some(name.trim().to_owned());
            }
        }
        app.state.workbench.saved_output_name_draft = None;
        app.state.workbench.saved_output_expression_draft = None;
    } else {
        if name != output.name {
            app.state.workbench.saved_output_name_draft = Some(name);
        }
        if expression != output.source_expression {
            app.state.workbench.saved_output_expression_draft = Some(expression);
        }
    }
    // Dispatched after the record's own edits so that clicking a hand-off,
    // which takes focus off the field being typed into, commits that edit
    // rather than discarding it.
    if let Some(action) = handoff.take() {
        action.dispatch(app);
    } else if view_trace.get()
        && let Ok((analysis_index, waveform_index)) = trace
    {
        open_materialized_trace(app, analysis_index, waveform_index);
    }
}

/// Where the active plan dataset stored this output, or why it did not.
///
/// The receipt is the authority, not the waveform list: a name that happens
/// to match is not evidence that this contract produced it, and a contract
/// that was deferred, suppressed or refused says so in its own words.
pub(super) fn materialized_trace(
    app: &RSpiceApp,
    output: &SavedOutput,
) -> Result<(usize, usize), String> {
    let run = super::output_evidence::selected_plan_dataset(app).ok_or_else(|| {
        if app.state.simulation.active_run().is_some() {
            "The active dataset was not produced by this plan, so it holds no trace for this \
             output."
                .to_owned()
        } else {
            "No run has been retained, so this output has no stored trace.".to_owned()
        }
    })?;
    let mut refusal = None;
    for (analysis_index, analysis) in run.analyses.iter().enumerate() {
        let Some(receipt) = analysis
            .saved_output_receipts
            .iter()
            .find(|receipt| receipt.output_id == output.id)
        else {
            continue;
        };
        match &receipt.status {
            crate::state::SavedOutputMaterializationStatus::Materialized {
                waveform_name, ..
            } => {
                if let Some(waveform_index) = analysis
                    .waveforms
                    .iter()
                    .position(|waveform| &waveform.name == waveform_name)
                {
                    return Ok((analysis_index, waveform_index));
                }
                refusal.get_or_insert(format!(
                    "The receipt names waveform {waveform_name}, which the retained analysis no \
                     longer holds."
                ));
            }
            crate::state::SavedOutputMaterializationStatus::Deferred => {
                refusal.get_or_insert(
                    "This output is deferred in the retained dataset: it is evaluated on demand \
                     rather than stored, so there is no trace to open. Materialize it from the \
                     dataset manifest first."
                        .to_owned(),
                );
            }
            crate::state::SavedOutputMaterializationStatus::SuppressedOnSuccess => {
                refusal.get_or_insert(
                    "This failure-only contract was inactive on a successful analysis, so no \
                     trace was stored."
                        .to_owned(),
                );
            }
            crate::state::SavedOutputMaterializationStatus::Unavailable { reason } => {
                refusal.get_or_insert(format!("This output was not materialized: {reason}"));
            }
        }
    }
    Err(refusal.unwrap_or_else(|| {
        "The retained dataset holds no receipt for this output, so it was never stored.".to_owned()
    }))
}

/// Land on the stored trace: its analysis, its waveform, and the renderer.
///
/// The dataset is already the active one — `materialized_trace` resolves only
/// against the plan's active run — so this carries the analysis and the trace
/// and lets the viewer command own the workspace half.
pub(super) fn open_materialized_trace(
    app: &mut RSpiceApp,
    analysis_index: usize,
    waveform_index: usize,
) {
    if !app.state.simulation.select_analysis(analysis_index) {
        return;
    }
    app.state.ui.results.selected_trace = app.state.simulation.active_run().and_then(|run| {
        crate::workbench::documents::result_document::SelectedResultTrace::from_run_indices(
            run,
            analysis_index,
            waveform_index,
        )
    });
    app.state.ui.results.clear_cursors();
    Command::ResultViewer(crate::workbench::ResultViewer::Waves).execute(app);
}

/// The surfaces the expression editor hands off to.
///
/// Each is an existing workbench command. The card dispatches rather than
/// growing its own copy, so the calculator, the scalar-measurement author and
/// the diagnostics window keep one owner apiece.
#[derive(Clone, Copy)]
pub(super) enum EditorHandoff {
    Calculator,
    ScalarMeasurement,
    UnitDiagnostics,
}

impl EditorHandoff {
    pub(super) const ALL: [Self; 3] = [
        Self::Calculator,
        Self::ScalarMeasurement,
        Self::UnitDiagnostics,
    ];

    /// Why the two dataset-backed hand-offs are disabled.
    ///
    /// Stated on the control and again in the card's closing note, because a
    /// reason only a hover reveals is a reason the reader has to guess exists.
    pub(super) const DATASET_RULE: &'static str =
        "No run has been retained, so there is nothing to measure or diagnose against.";

    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::Calculator => "Open calculator…",
            // Named for what the command opens. It is an author for one
            // finite scalar measurement over the active analysis, not a
            // catalog of templates, so "insert from library" would name a
            // surface that does not exist.
            Self::ScalarMeasurement => "New scalar measurement…",
            Self::UnitDiagnostics => "Unit diagnostics…",
        }
    }

    const fn hint(self) -> &'static str {
        match self {
            Self::Calculator => "Compose and evaluate an expression without saving it to the plan.",
            Self::ScalarMeasurement => {
                "Author a retained scalar measurement over the active analysis. It opens in \
                 Visualization Studio, which owns measurement authoring."
            }
            Self::UnitDiagnostics => {
                "Report the inferred unit and dimensional check for the expressions the active \
                 dataset holds."
            }
        }
    }

    /// Whether the hand-off resolves against a retained dataset.
    ///
    /// This restates the dataset clause of the two commands' availability
    /// rules rather than calling `Command::is_enabled`, and deliberately.
    /// Their rules also require the reader to already be on a result surface,
    /// which from here is never true and which the dispatch below answers by
    /// navigating there instead. Asking the command directly would disable
    /// both hand-offs permanently and name the route as the reason.
    pub(super) const fn needs_dataset(self) -> bool {
        matches!(self, Self::ScalarMeasurement | Self::UnitDiagnostics)
    }

    const fn command(self) -> Command {
        match self {
            Self::Calculator => Command::WaveformCalculator,
            Self::ScalarMeasurement => Command::MeasurementLibrary,
            Self::UnitDiagnostics => Command::ExpressionDiagnostics,
        }
    }

    pub(super) fn dispatch(self, app: &mut RSpiceApp) {
        // The measurement author lives in Visualization Studio, and the
        // command only navigates there from a result surface. Activating
        // Results first is what makes the hand-off land somewhere the reader
        // can see, rather than arming a dock behind this page. The calculator
        // and the diagnostics window are modeless tools the frame paints over
        // whatever workspace is open, so they need no route change.
        if matches!(self, Self::ScalarMeasurement) {
            app.state
                .workbench
                .activate(crate::workbench::state::Workspace::Results);
        }
        self.command().execute(app);
    }
}

/// The option that means "no group names this output; the rules decide".
const CAPTURE_BY_RULE: &str = "By rule";

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
        options.extend(
            plan.instances()
                .iter()
                .enumerate()
                .map(|(index, instance)| {
                    (
                        format!("Only #{} · {}", index + 1, instance.kind().code()),
                        SavedOutputCompatibility::SelectedAnalysis {
                            analysis_id: instance.id(),
                        },
                    )
                }),
        );
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
///
/// Reports whether the change was adopted, so a caller that has to follow the
/// record — a rename moving the registry selection — only does so when the
/// plan actually took the edit.
fn replace_output(
    app: &mut RSpiceApp,
    output_id: crate::product::SavedOutputId,
    detail: &str,
    apply: impl FnOnce(&mut SavedOutput),
) -> bool {
    let Ok(plan_id) = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| plan.id())
    else {
        return false;
    };
    let Some(mut replacement) = app.state.workspace.plan_data(plan_id).and_then(|payload| {
        payload
            .saved_outputs
            .iter()
            .find(|output| output.id == output_id)
            .cloned()
    }) else {
        return false;
    };
    apply(&mut replacement);
    commit_plan_change(app, plan_id, detail, move |workspace, plan_id| {
        workspace
            .replace_saved_output(plan_id, output_id, replacement)
            .map(|_| ())
            .map_err(|error| error.to_string())
    })
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
    let mut retained_engine_source_analyses = std::collections::HashSet::new();
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
        retained_engine_source_analyses
            .extend(report.retained_engine_source_analysis_ids().iter().copied());
    }
    exact_bytes = exact_bytes.saturating_add(
        crate::simulation::output_contract::retained_engine_source_upper_bound_bytes(
            retained_engine_source_analyses.len(),
        ),
    );
    exact_bytes = exact_bytes.saturating_mul(
        u64::try_from(app.state.sim_setup.run_set.point_count())
            .unwrap_or(u64::MAX)
            .max(1),
    );
    card(
        ui,
        "What a run will retain",
        Some(("derived from the plan", Tone::Neutral)),
        |ui| {
            card_body(ui, |ui| {
                rule_row(
                    ui,
                    "Prepared retention forecast",
                    &format_bytes(exact_bytes),
                );
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
                "The total includes authored outputs, shared deferred source-state ceilings, and \
                 the global Run Set point count. Shared source state is counted once per prepared \
                 analysis, not once per deferred expression. An output whose size cannot be \
                 bounded before the solve is listed rather than counted as zero.",
            );
        },
    );
}

fn format_bytes(bytes: u64) -> String {
    super::workflows::format_storage_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use super::{Tone, output_registry_summary};
    use crate::simulation::SavedOutputSemanticStatus;

    #[test]
    fn registry_summary_never_calls_runtime_bound_outputs_resolved() {
        let statuses = [
            SavedOutputSemanticStatus::Valid {
                detail: "elaborated node".to_owned(),
            },
            SavedOutputSemanticStatus::RuntimeBound {
                reason: "derived result exists only after execution".to_owned(),
            },
        ];

        assert_eq!(
            output_registry_summary(statuses.iter(), statuses.len()),
            (
                "2 saved · 1 resolve · 1 bind at run time".to_owned(),
                Tone::Warn,
            ),
        );
    }

    #[test]
    fn registry_summary_fails_closed_when_a_report_is_missing() {
        let status = SavedOutputSemanticStatus::Valid {
            detail: "elaborated node".to_owned(),
        };

        assert_eq!(
            output_registry_summary(std::iter::once(&status), 2),
            ("1 of 2 do not resolve".to_owned(), Tone::Error),
        );
    }

    #[test]
    fn empty_output_registry_is_neutral() {
        assert_eq!(
            output_registry_summary(std::iter::empty(), 0),
            ("0 saved · nothing configured".to_owned(), Tone::Neutral),
        );
    }
}
