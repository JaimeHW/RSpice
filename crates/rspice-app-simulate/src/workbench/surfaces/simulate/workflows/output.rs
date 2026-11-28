//! Adding or editing a saved output: its draft, its consumers, and its unit and
//! storage summaries.

use super::*;

pub(super) fn saved_output_dialog(
    ctx: &egui::Context,
    cx: &mut Cx<'_, SimulateFeature>,
    mut draft: SavedOutputDraft,
) {
    draft.validation_error = validate_saved_output_draft(cx, &draft).err();
    let enabled = draft.validation_error.is_none();
    let candidate = saved_output_from_draft(cx, &draft).ok();
    let preflight = candidate.as_ref().map(|output| {
        cx.handles
            .run_controller
            .saved_output_preflight(cx.model, output)
    });
    let inferred_unit = candidate.as_ref().map_or_else(
        || inferred_output_unit(&draft.expression),
        |output| output.inferred_unit(),
    );
    let consumers = saved_output_consumers(cx, &draft);
    let expression_validity = preflight.as_ref().map_or_else(
        || {
            draft
                .validation_error
                .clone()
                .unwrap_or_else(|| "candidate output is incomplete".to_owned())
        },
        |report| saved_output_semantic_summary(report.semantic_status()),
    );
    let storage_increment = preflight.as_ref().map_or_else(
        || "indeterminate until the output contract is valid".to_owned(),
        |report| {
            saved_output_storage_summary(
                report.storage_estimate(),
                report.compatible_analysis_count(),
                report.retained_engine_source_analysis_ids().len(),
                report.streamed_analysis_ids().len(),
            )
        },
    );
    let choice = Dialog::new("Add saved output or derived expression", "Add output")
    .description(
        "Add a validated output contract with explicit analysis compatibility, retention, precision, and streaming behavior.",
    )
    .size(DialogSize::SimulationWorkflow)
    .flush_body()
    .ghost("Cancel")
    .primary_enabled(enabled)
    .primary_on_enter(false)
    .show(ctx, |ui| {
        workflow_split(ui, |ui| {
            workflow_select_field(
                ui,
                "saved-output-kind",
                "Output kind",
                &mut draft.kind,
                &[
                    "Raw voltage / current",
                    "Derived expression",
                    "Device operating-point quantity",
                    "Noise contributor",
                    "RF port quantity",
                ],
            );
            workflow_text_field(ui, "Name", &mut draft.name, true);
            workflow_multiline_field(ui, "Source or expression", &mut draft.expression);
            workflow_select_field(
                ui,
                "saved-output-analyses",
                "Compatible analyses",
                &mut draft.compatible_analyses,
                &["OP + TRAN + AC", "All compatible analyses", "Selected analysis only"],
            );
        }, |ui| {
            workflow_section_heading(ui, "Save, precision, and provenance");
            workflow_select_field(
                ui,
                "saved-output-save-policy",
                "Save policy",
                &mut draft.save_policy,
                &[
                    "Every accepted point",
                    "Selected + final points",
                    "On demand from retained state",
                    "Failure diagnostics only",
                ],
            );
            workflow_select_field(
                ui,
                "saved-output-precision",
                "Stored precision",
                &mut draft.precision,
                &[
                    "f64 / complex128",
                    "f32 display cache + full source precision",
                ],
            );
            workflow_select_field(
                ui,
                "saved-output-streaming",
                "Streaming",
                &mut draft.streaming,
                &["Live plot · adaptive display decimation", "Store only"],
            );
            ui.add_space(8.0);
            property_row(ui, "Inferred unit", inferred_unit);
            property_row_wrapped(ui, "Estimated increment", &storage_increment);
            property_row_wrapped(ui, "Expression validity", &expression_validity);
            property_row_wrapped(ui, "Consumers", &consumers);
        });
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    finish_workflow_choice(cx, choice, draft, commit_saved_output);
}

pub(super) fn saved_output_from_draft(
    cx: &Cx<'_, SimulateFeature>,
    draft: &SavedOutputDraft,
) -> Result<crate::state::SavedOutput, String> {
    use crate::state::{
        SavedOutput, SavedOutputKind, SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming,
    };
    let kind = SavedOutputKind::ALL
        .get(draft.kind)
        .copied()
        .ok_or_else(|| "Output kind selection is invalid.".to_owned())?;
    let compatible_analyses = saved_output_compatibility(cx, draft.compatible_analyses)?;
    let save_policy = SavedOutputPolicy::ALL
        .get(draft.save_policy)
        .copied()
        .ok_or_else(|| "Save policy selection is invalid.".to_owned())?;
    let stored_precision = SavedOutputPrecision::ALL
        .get(draft.precision)
        .copied()
        .ok_or_else(|| "Stored precision selection is invalid.".to_owned())?;
    let streaming = SavedOutputStreaming::ALL
        .get(draft.streaming)
        .copied()
        .ok_or_else(|| "Streaming selection is invalid.".to_owned())?;
    SavedOutput::new(
        kind,
        draft.name.clone(),
        draft.expression.clone(),
        compatible_analyses,
        save_policy,
        stored_precision,
        streaming,
    )
}

pub(super) fn validate_saved_output_draft(
    cx: &Cx<'_, SimulateFeature>,
    draft: &SavedOutputDraft,
) -> Result<(), String> {
    let output = saved_output_from_draft(cx, draft)?;
    let report = cx
        .handles
        .run_controller
        .saved_output_preflight(cx.model, &output);
    if let SavedOutputSemanticStatus::Invalid { reason } = report.semantic_status() {
        return Err(reason.clone());
    }
    let plan_id = cx.model.sim_setup.stable_analysis_plan()?.id();
    let mut workspace = cx.model.workspace.clone();
    workspace
        .content
        .add_saved_output(plan_id, output)
        .map_err(|error| error.to_string())?;
    workspace
        .content
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    Ok(())
}

pub(crate) fn commit_saved_output(
    cx: &mut Cx<'_, SimulateFeature>,
    draft: &SavedOutputDraft,
) -> Result<String, String> {
    validate_saved_output_draft(cx, draft)?;
    let output = saved_output_from_draft(cx, draft)?;
    let output_name = output.name.clone();
    let step = cx.model.setup_snapshot(SetupParts::PLAN_DATA);
    let mut setup = cx.model.sim_setup.clone();
    let mut workspace = cx.model.workspace.clone();
    let plan_id = setup.stable_analysis_plan()?.id();
    workspace
        .content
        .add_saved_output(plan_id, output)
        .map_err(|error| error.to_string())?;
    workspace
        .content
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    let receipt = setup
        .commit_active_plan_configuration_change(format!("Added saved output {output_name}."))
        .map_err(|error| error.to_string())?;
    cx.model.sim_setup = setup;
    cx.model.workspace = workspace;
    cx.model
        .record_setup_change(cx.state.simulation_page.label(), step);
    crate::workbench::simulate::invalidate_preflight(cx);
    cx.model.record_plan_receipt(receipt.status_line());
    Ok(format!(
        "Added saved output {output_name} to plan {}.",
        cx.model.sim_setup.active_plan_name()
    ))
}

pub(super) fn saved_output_consumers(
    cx: &Cx<'_, SimulateFeature>,
    draft: &SavedOutputDraft,
) -> String {
    match saved_output_compatibility(cx, draft.compatible_analyses) {
        Ok(compatibility) => saved_output_consumers_of(cx.model, &compatibility),
        // The draft names a scope this plan cannot supply -- an analysis-only
        // output with no analysis selected -- so nothing would write it.
        Err(_) => "no compatible enabled analyses".to_owned(),
    }
}

/// The compatibility the Add-output dialog's scope select names.
///
/// The dialog offers three positions and the output holds one of three
/// variants; this is where the two are bound together, so the control, the
/// output it commits, and the consumers either of them is previewed with
/// cannot disagree about what position two means.
fn saved_output_compatibility(
    cx: &Cx<'_, SimulateFeature>,
    selection: usize,
) -> Result<SavedOutputCompatibility, String> {
    match selection {
        0 => Ok(SavedOutputCompatibility::OpTranAc),
        1 => Ok(SavedOutputCompatibility::AllCompatibleAnalyses),
        2 => Ok(SavedOutputCompatibility::SelectedAnalysis {
            analysis_id: cx
                .state
                .active_analysis_instance
                .ok_or_else(|| "Select an analysis before using analysis-only scope.".to_owned())?,
        }),
        _ => Err("Compatible analyses selection is invalid.".to_owned()),
    }
}

/// Which enabled analyses would write an output of this compatibility.
///
/// One derivation for the Add-output dialog's preview and the registry's
/// "Consumers" column: they ask the same question of the same plan, and a
/// column deriving it separately would be free to disagree with the dialog that
/// authored the output.
pub(crate) fn saved_output_consumers_of(
    model: &Model,
    compatibility: &SavedOutputCompatibility,
) -> String {
    let Ok(plan) = model.sim_setup.stable_analysis_plan() else {
        return "plan unavailable".to_owned();
    };
    let labels = plan
        .instances()
        .iter()
        .enumerate()
        .filter(|(_, instance)| instance.enabled())
        .filter(|(_, instance)| match compatibility {
            SavedOutputCompatibility::OpTranAc => matches!(
                instance.kind(),
                AnalysisKind::OperatingPoint | AnalysisKind::Transient | AnalysisKind::Ac
            ),
            SavedOutputCompatibility::AllCompatibleAnalyses => true,
            SavedOutputCompatibility::SelectedAnalysis { analysis_id } => {
                *analysis_id == instance.id()
            }
        })
        .map(|(index, instance)| {
            plan.instance_list_label(index)
                .unwrap_or_else(|| instance.display_name().to_owned())
        })
        .collect::<Vec<_>>();
    if labels.is_empty() {
        "no compatible enabled analyses".to_owned()
    } else {
        labels.join(" · ")
    }
}

pub(super) fn inferred_output_unit(expression: &str) -> &'static str {
    let expression = expression.trim();
    if expression
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("V("))
    {
        "volts"
    } else if expression
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("I("))
    {
        "amperes"
    } else if expression
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("S("))
    {
        "dimensionless"
    } else {
        "resolved from output schema"
    }
}

pub(super) fn saved_output_semantic_summary(status: &SavedOutputSemanticStatus) -> String {
    match status {
        SavedOutputSemanticStatus::Valid { detail } => detail.clone(),
        SavedOutputSemanticStatus::RuntimeBound { reason } => reason.clone(),
        SavedOutputSemanticStatus::Invalid { reason } => reason.clone(),
    }
}

pub(crate) fn saved_output_storage_summary(
    estimate: &SavedOutputStorageEstimate,
    compatible_analyses: usize,
    retained_engine_source_analyses: usize,
    streamed_analyses: usize,
) -> String {
    let analyses = if compatible_analyses == 1 {
        "analysis"
    } else {
        "analyses"
    };
    if retained_engine_source_analyses > 0 {
        return format!(
            "deferred · shared source ceiling counted at plan level · {compatible_analyses} compatible {analyses}"
        );
    }
    if streamed_analyses > 0 {
        // The solve decides how many points it accepts; the store counts
        // them as the run writes them.
        let known = match estimate {
            SavedOutputStorageEstimate::ExactBytes(bytes) if *bytes > 0 => format!(
                "{} + ",
                rspice_simulation_contract::run_set::format_bytes(*bytes)
            ),
            _ => String::new(),
        };
        return format!(
            "{known}streamed · counted as the run writes it · {compatible_analyses} compatible {analyses}"
        );
    }
    match estimate {
        SavedOutputStorageEstimate::ExactBytes(bytes) => format!(
            "{} · {compatible_analyses} compatible {}",
            rspice_simulation_contract::run_set::format_bytes(*bytes),
            if compatible_analyses == 1 {
                "analysis"
            } else {
                "analyses"
            }
        ),
        SavedOutputStorageEstimate::Indeterminate { reason } => {
            format!("indeterminate · {reason}")
        }
    }
}
