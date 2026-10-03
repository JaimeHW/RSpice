//! The studio's docked side panels.
//!
//! Each dock is a single-purpose editor over the pane that is currently
//! selected — traces, axes, markers, annotations, the report page policy —
//! and only one is open at a time. Nothing here owns state: a dock reads
//! [`VisualizationStudioState`] and writes back through it, so closing a dock
//! never discards an edit.

use super::*;
use rspice_results::waveform_comparison::{
    ComparisonExecution, ComparisonOptions, DifferenceTrace, comparison_signal_names,
    execute_comparison, matching_comparison_analysis,
};

mod comparison;
mod entities;
mod family;
mod layout;
mod pane;
mod traces;
use comparison::comparison_dock;
use entities::{
    annotation_dock, cursor_manager_dock, export_dock, measurement_dock, trace_manager_dock,
};
use family::{family_encoding_dock, family_filter_dock, family_slice_dock};
use layout::{link_groups_dock, page_editor_dock, properties_dock, reorder_panes_dock};
use pane::add_pane_dock;

pub(super) fn dock_body(ui: &mut Ui, app: &mut RSpiceApp, dock: VisualizationDock) -> bool {
    match dock {
        VisualizationDock::AddPane => add_pane_dock(ui, app),
        VisualizationDock::TraceManager => trace_manager_dock(ui, app),
        VisualizationDock::CursorManager => cursor_manager_dock(ui, app),
        VisualizationDock::DocumentProperties => properties_dock(ui, app),
        VisualizationDock::ReorderPanes => reorder_panes_dock(ui, app),
        VisualizationDock::LinkGroups => link_groups_dock(ui, app),
        VisualizationDock::PageEditor => page_editor_dock(ui, app),
        VisualizationDock::Measurement => measurement_dock(ui, app),
        VisualizationDock::Annotation => annotation_dock(ui, app),
        VisualizationDock::FamilySlice => family_slice_dock(ui, app),
        VisualizationDock::FamilyEncoding => family_encoding_dock(ui, app),
        VisualizationDock::FamilyFilter => family_filter_dock(ui, app),
        VisualizationDock::Comparison => comparison_dock(ui, app),
        VisualizationDock::Export => export_dock(ui, app),
    }
}

fn normalize_add_pane_draft(state: &mut AppState) {
    let draft_dataset = state.workbench.visualization_studio.draft_dataset_id;
    let draft_analysis = state.workbench.visualization_studio.draft_analysis_sequence;
    let draft_is_valid =
        draft_dataset
            .zip(draft_analysis)
            .is_some_and(|(dataset_id, analysis_sequence)| {
                state.simulation.runs.iter().any(|run| {
                    run.dataset_id == dataset_id
                        && run
                            .analyses
                            .iter()
                            .any(|analysis| analysis.id == analysis_sequence)
                })
            });
    if draft_is_valid {
        return;
    }
    let fallback = state
        .simulation
        .active_run()
        .and_then(|run| {
            state
                .simulation
                .active_analysis()
                .map(|analysis| (run.dataset_id, analysis.id))
        })
        .or_else(|| {
            state.simulation.runs.iter().find_map(|run| {
                run.analyses
                    .first()
                    .map(|analysis| (run.dataset_id, analysis.id))
            })
        });
    state.workbench.visualization_studio.draft_dataset_id = fallback.map(|binding| binding.0);
    state.workbench.visualization_studio.draft_analysis_sequence =
        fallback.map(|binding| binding.1);
}

fn selected_draft_analysis(state: &AppState) -> Option<&crate::state::AnalysisResult> {
    let studio = &state.workbench.visualization_studio;
    let dataset_id = studio.draft_dataset_id?;
    let analysis_sequence = studio.draft_analysis_sequence?;
    state
        .simulation
        .runs
        .iter()
        .find(|run| run.dataset_id == dataset_id)?
        .analyses
        .iter()
        .find(|analysis| analysis.id == analysis_sequence)
}

pub(super) fn save_document_properties(
    app: &mut RSpiceApp,
    significant_digits: u8,
    phase_continuous: bool,
) -> Result<(), String> {
    if active_project_visualization_document_id(&app.state).is_some() {
        transact_active_project_document(
            app,
            vec![DocumentEdit::SetPresentation(
                crate::results::visualization_document::VisualizationPresentationPolicy {
                    significant_digits,
                    phase_continuous,
                },
            )],
        )?;
    } else {
        app.state
            .workbench
            .visualization_studio
            .transact(|studio| {
                studio.significant_digits = significant_digits;
                Ok(())
            })?;
    }
    app.state.ui.results.session.phase_continuous = phase_continuous;
    app.state.workbench.visualization_studio.significant_digits = significant_digits;
    Ok(())
}

fn collect_measurement_waveforms(
    expression: &calculator::ast::CalculatorExpr,
    signals: &mut Vec<String>,
) {
    use calculator::ast::CalculatorExpr;
    match expression {
        CalculatorExpr::WaveformRef { signal, .. } => {
            if !signals.contains(signal) {
                signals.push(signal.clone());
            }
        }
        CalculatorExpr::BinaryOp { left, right, .. } => {
            collect_measurement_waveforms(left, signals);
            collect_measurement_waveforms(right, signals);
        }
        CalculatorExpr::UnaryOp { operand, .. } => {
            collect_measurement_waveforms(operand, signals);
        }
        CalculatorExpr::FunctionCall { args, .. } => {
            for argument in args {
                collect_measurement_waveforms(argument, signals);
            }
        }
        CalculatorExpr::Number(_) | CalculatorExpr::Constant(_) => {}
    }
}

fn canonical_measurement_trace_ids(
    state: &AppState,
    expression: &str,
) -> Result<
    (
        crate::results::visualization_document::PaneId,
        Vec<crate::results::visualization_document::TraceId>,
    ),
    String,
> {
    let project = &state.workspace.content;
    let (pane_id, fallback_trace) = active_project_pane_and_trace(state, None)?;
    let parsed = calculator::parser::try_parse(expression)
        .map_err(|error| format!("Parse error: {error}"))?;
    let mut signal_names = Vec::new();
    collect_measurement_waveforms(&parsed, &mut signal_names);
    if signal_names.is_empty() {
        return Ok((pane_id, vec![fallback_trace]));
    }
    let document_id = active_project_visualization_document_id(state)
        .ok_or_else(|| "Open a project-owned result document before measuring it.".to_owned())?;
    let document = project
        .visualization_document(document_id)
        .ok_or_else(|| "The active result document is no longer retained.".to_owned())?;
    let trace_ids = signal_names
        .iter()
        .map(|signal| {
            document
                .traces()
                .iter()
                .find(|trace| trace.pane_id == pane_id && trace.label == *signal)
                .map(|trace| trace.id)
                .ok_or_else(|| {
                    format!(
                        "The active result pane has no canonical trace for expression signal {signal:?}."
                    )
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok((pane_id, trace_ids))
}

pub(super) fn evaluate_scalar_measurement(
    state: &AppState,
    expression: &str,
) -> Result<(DatasetId, u64, f64), String> {
    if expression.trim().is_empty() {
        return Err("A measurement expression is required".to_owned());
    }
    let run = state
        .simulation
        .active_run()
        .ok_or_else(|| "A retained result dataset is required".to_owned())?;
    let analysis = state
        .simulation
        .active_analysis()
        .ok_or_else(|| "A retained analysis must be selected".to_owned())?;
    let parsed = calculator::parser::try_parse(expression)
        .map_err(|error| format!("Parse error: {error}"))?;
    let context = calculator::WaveformsContext::new(&analysis.waveforms);
    let value = match calculator::evaluator::evaluate(&parsed, &context)
        .and_then(calculator::CalcValue::into_real)
        .map_err(|error| error.to_string())?
    {
        calculator::RealValue::Scalar(value) => value,
        calculator::RealValue::Waveform(..) => {
            return Err(
                "The expression produces a trace; reduce it with avg(), rms(), or another scalar function"
                    .to_owned(),
            );
        }
    };
    Ok((run.dataset_id, analysis.id, value))
}

pub(super) fn commit_comparison_execution(
    app: &mut RSpiceApp,
    execution: ComparisonExecution,
) -> Result<(), String> {
    let mut projected_studio = app.state.workbench.visualization_studio.clone();
    projected_studio.transact(|studio| {
        retain_difference_trace_sets(studio, execution.difference_traces.clone())
    })?;

    if active_project_visualization_document_id(&app.state).is_some() {
        let mut edits = comparison_source_projection_edits(app, &execution.receipt)?;
        edits.push(DocumentEdit::RecordComparison(execution.receipt));
        transact_active_project_document(app, edits)?;
        reconcile_document(app);
        let studio = &mut app.state.workbench.visualization_studio.presentation;
        studio.next_identity = studio.next_identity.max(projected_studio.next_identity);
        studio.difference_trace_sets = projected_studio.presentation.difference_trace_sets;
        studio.normalize();
        return Ok(());
    }

    app.state
        .workbench
        .visualization_studio
        .transact(move |studio| {
            studio.comparison_receipts.push(execution.receipt);
            retain_difference_trace_sets(studio, execution.difference_traces)
        })
}

fn comparison_source_projection_edits(
    app: &RSpiceApp,
    receipt: &ComparisonReceipt,
) -> Result<Vec<DocumentEdit>, String> {
    let state = &app.state;
    let document_id = active_project_visualization_document_id(&state).ok_or_else(|| {
        "Open a project-owned result document before recording a comparison.".to_owned()
    })?;
    let document = state
        .workspace
        .content
        .visualization_document(document_id)
        .ok_or_else(|| "The active result document is no longer retained.".to_owned())?;
    let candidate_analysis = state
        .simulation
        .active_analysis()
        .ok_or_else(|| "No candidate analysis is selected.".to_owned())?;
    let mut edits = Vec::new();
    let mut seen = HashSet::new();
    for binding in [receipt.baseline, receipt.candidate] {
        if !seen.insert(binding.dataset_id) {
            continue;
        }
        let run = state
            .simulation
            .runs
            .iter()
            .find(|run| run.dataset_id == binding.dataset_id)
            .ok_or_else(|| "A comparison source dataset is no longer retained.".to_owned())?;
        if run.dataset_content_digest() != binding.content_digest {
            return Err("A comparison source changed after the receipt was calculated.".to_owned());
        }
        let analysis = matching_comparison_analysis(candidate_analysis, run)
            .ok_or_else(|| "A comparison source analysis is no longer retained.".to_owned())?;
        let source = result_document::visualization_source_dataset(run, analysis)?;
        let already_attached = document
            .datasets()
            .iter()
            .any(|dataset| dataset.binding().dataset_id == binding.dataset_id);
        edits.push(if already_attached {
            DocumentEdit::MergeDatasetProjection(source)
        } else {
            DocumentEdit::AttachDataset(source)
        });
    }
    Ok(edits)
}

fn comparison_signal_names_for_baseline(
    state: &AppState,
    baseline_id: DatasetId,
) -> Result<Vec<String>, String> {
    let candidate_analysis = state
        .simulation
        .active_analysis()
        .ok_or_else(|| "No candidate analysis is selected.".to_owned())?;
    let baseline_run = state
        .simulation
        .runs
        .iter()
        .find(|run| run.dataset_id == baseline_id)
        .ok_or_else(|| "The comparison dataset is no longer retained.".to_owned())?;
    let baseline_analysis = matching_comparison_analysis(candidate_analysis, baseline_run)
        .ok_or_else(|| "The comparison dataset has no unambiguous matching analysis.".to_owned())?;
    comparison_signal_names(candidate_analysis, baseline_analysis)
}

pub(super) fn compatible_comparison_dataset_ids(
    state: &AppState,
    candidate_dataset_id: DatasetId,
    candidate_analysis_sequence: u64,
) -> Vec<DatasetId> {
    let Some(candidate_run) = state
        .simulation
        .runs
        .iter()
        .find(|run| run.dataset_id == candidate_dataset_id)
    else {
        return Vec::new();
    };
    let Some(candidate_analysis) = candidate_run
        .analyses
        .iter()
        .find(|analysis| analysis.id == candidate_analysis_sequence)
    else {
        return Vec::new();
    };
    state
        .simulation
        .runs
        .iter()
        .filter(|run| run.dataset_id != candidate_dataset_id)
        .filter_map(|run| {
            let baseline_analysis = matching_comparison_analysis(candidate_analysis, run)?;
            comparison_signal_names(candidate_analysis, baseline_analysis)
                .is_ok()
                .then_some(run.dataset_id)
        })
        .collect()
}

fn active_comparison_dataset_ids(state: &AppState) -> Vec<DatasetId> {
    let Some((dataset_id, analysis_sequence)) = state
        .simulation
        .active_run()
        .zip(state.simulation.active_analysis())
        .map(|(run, analysis)| (run.dataset_id, analysis.id))
    else {
        return Vec::new();
    };
    compatible_comparison_dataset_ids(state, dataset_id, analysis_sequence)
}

pub(super) fn retain_difference_trace_sets(
    studio: &mut VisualizationStudioState,
    traces: Vec<DifferenceTrace>,
) -> Result<(), String> {
    if studio
        .difference_trace_sets
        .len()
        .saturating_add(traces.len())
        > MAX_DIFFERENCE_TRACE_SETS
    {
        return Err(format!(
            "Retaining this comparison would exceed the {MAX_DIFFERENCE_TRACE_SETS}-set difference-trace limit."
        ));
    }
    let existing_values =
        studio
            .difference_trace_sets
            .iter()
            .try_fold(0_usize, |total, trace_set| {
                total
                    .checked_add(trace_set.retained_numeric_values()?)
                    .ok_or_else(|| "Difference-trace retained-value count overflowed".to_owned())
            })?;
    let new_values =
        traces.iter().try_fold(0_usize, |total, trace| {
            total
                .checked_add(
                    trace.coordinates.len().checked_mul(4).ok_or_else(|| {
                        "Difference-trace retained-value count overflowed".to_owned()
                    })?,
                )
                .ok_or_else(|| "Difference-trace retained-value count overflowed".to_owned())
        })?;
    if existing_values.saturating_add(new_values) > MAX_DIFFERENCE_TRACE_NUMERIC_VALUES {
        return Err(format!(
            "Retaining this comparison would exceed the {MAX_DIFFERENCE_TRACE_NUMERIC_VALUES}-value difference-trace limit."
        ));
    }
    for trace in traces {
        let id = studio
            .allocate_identity()
            .ok_or_else(|| "Difference-trace identity space is exhausted.".to_owned())?;
        let absolute_id = studio
            .allocate_identity()
            .ok_or_else(|| "Difference-trace identity space is exhausted.".to_owned())?;
        let relative_id = studio
            .allocate_identity()
            .ok_or_else(|| "Difference-trace identity space is exhausted.".to_owned())?;
        let normalized_id = studio
            .allocate_identity()
            .ok_or_else(|| "Difference-trace identity space is exhausted.".to_owned())?;
        studio
            .difference_trace_sets
            .push(VisualizationDifferenceTraceSet {
                id,
                baseline: trace.baseline,
                candidate: trace.candidate,
                signal_key: trace.signal_key,
                signal_label: trace.signal_label,
                coordinate_unit: trace.coordinate_unit,
                coordinates: trace.coordinates,
                absolute: VisualizationDifferenceSeries {
                    id: absolute_id,
                    kind: VisualizationDifferenceKind::Absolute,
                    values: trace.absolute,
                },
                relative: VisualizationDifferenceSeries {
                    id: relative_id,
                    kind: VisualizationDifferenceKind::Relative,
                    values: trace.relative,
                },
                normalized: VisualizationDifferenceSeries {
                    id: normalized_id,
                    kind: VisualizationDifferenceKind::Normalized,
                    values: trace.normalized,
                },
                execution: trace.execution,
                tolerance: trace.tolerance,
            });
    }
    Ok(())
}

pub(super) fn execute_comparison_draft_with_differences(
    app: &RSpiceApp,
) -> Result<ComparisonExecution, String> {
    let active_run = app
        .state
        .simulation
        .active_run()
        .ok_or_else(|| "No candidate dataset is selected.".to_owned())?;
    let active_analysis = app
        .state
        .simulation
        .active_analysis()
        .ok_or_else(|| "No candidate analysis is selected.".to_owned())?;
    let baseline_id = app
        .state
        .workbench
        .visualization_studio
        .draft_comparison_dataset
        .ok_or_else(|| "Select an immutable comparison dataset.".to_owned())?;
    let baseline_run = app
        .state
        .simulation
        .runs
        .iter()
        .find(|run| run.dataset_id == baseline_id)
        .ok_or_else(|| "The comparison dataset is no longer retained.".to_owned())?;
    let studio = &app.state.workbench.visualization_studio;
    execute_comparison(
        active_run,
        active_analysis,
        baseline_run,
        ComparisonOptions {
            alignment: studio.draft_comparison_alignment,
            alignment_signal: &studio.draft_comparison_alignment_signal,
            threshold: studio.draft_comparison_threshold,
            maximum_lag_samples: studio.draft_comparison_maximum_lag_samples,
            absolute_tolerance: studio.draft_comparison_absolute_tolerance,
            relative_tolerance: studio.draft_comparison_relative_tolerance,
            difference_trace: studio.draft_comparison_difference_trace,
        },
    )
}

fn active_family_manifest(app: &RSpiceApp) -> Result<FamilyManifest, String> {
    let analysis = app
        .state
        .simulation
        .active_analysis()
        .ok_or_else(|| "Select an analysis with retained family data.".to_owned())?;
    FamilyManifest::from_metadata(analysis.analysis_type, analysis.family_metadata.as_ref())?
        .ok_or_else(|| "The active analysis has no retained family manifest.".to_owned())
}

pub(super) fn active_family_sample_selection(
    app: &RSpiceApp,
) -> Result<Option<SourceSampleSelection>, String> {
    let studio = &app.state.workbench.visualization_studio;
    let Some(pane) = studio.active_pane() else {
        return Ok(None);
    };
    let Some(policy) = studio.family_policies.get(&pane.id) else {
        return Ok(None);
    };
    if pane.viewer != ResultViewer::Waves {
        return Err(
            "This family policy requires the waveform renderer; choose Waveform before applying it."
                .to_owned(),
        );
    }
    let run = app
        .state
        .simulation
        .active_run()
        .ok_or_else(|| "The pane's immutable dataset is unavailable.".to_owned())?;
    let analysis = app
        .state
        .simulation
        .active_analysis()
        .ok_or_else(|| "The pane's bound analysis is unavailable.".to_owned())?;
    if run.dataset_id != pane.dataset_id || analysis.id != pane.analysis_sequence {
        return Err(
            "The active renderer binding does not match the family policy pane.".to_owned(),
        );
    }
    let manifest =
        FamilyManifest::from_metadata(analysis.analysis_type, analysis.family_metadata.as_ref())?
            .ok_or_else(|| "The pane's source no longer contains family metadata.".to_owned())?;
    let indices = manifest.matching_source_indices_for_filter(policy.filter.as_ref())?;
    for waveform in &analysis.waveforms {
        manifest.compatible_waveform_len(waveform.x.len())?;
        if waveform.x.len() != waveform.y.len() {
            return Err(format!(
                "Waveform '{}' has mismatched X and Y sample counts.",
                waveform.name
            ));
        }
    }
    SourceSampleSelection::new(run.dataset_id, analysis.id, indices)
        .and_then(|selection| selection.with_family_presentation(&manifest, policy))
        .map(Some)
}

fn document_family_dimension(
    manifest: &FamilyManifest,
    id: &str,
) -> Result<DocumentFamilyDimension, String> {
    let dimension = manifest
        .dimension(id)
        .ok_or_else(|| format!("Unknown family dimension '{id}'."))?;
    DocumentFamilyDimension::new(
        dimension.id.clone(),
        match dimension.kind {
            FamilyValueKind::Number => ValueType::Real,
            FamilyValueKind::Integer => ValueType::Integer,
            FamilyValueKind::Text | FamilyValueKind::Status => ValueType::Text,
        },
    )
    .map_err(|error| error.to_string())
}

fn build_family_policy_draft(
    app: &RSpiceApp,
    manifest: &FamilyManifest,
) -> Result<FamilyPresentationPolicy, String> {
    let studio = &app.state.workbench.visualization_studio;
    let x_dimension = document_family_dimension(manifest, &studio.draft_family_x_dimension)?;
    let mut family_dimension_ids = vec![studio.draft_family_dimension.clone()];
    for dimension in [
        &studio.draft_family_color_dimension,
        &studio.draft_family_dash_dimension,
        &studio.draft_family_marker_dimension,
    ] {
        if !dimension.is_empty()
            && *dimension != x_dimension.key
            && !family_dimension_ids.contains(dimension)
        {
            family_dimension_ids.push(dimension.clone());
        }
    }
    if studio.family_query.contains("status")
        && x_dimension.key != "status"
        && !family_dimension_ids.iter().any(|id| id == "status")
    {
        family_dimension_ids.push("status".to_owned());
    }
    family_dimension_ids.retain(|id| !id.is_empty() && *id != x_dimension.key);
    let family_dimensions = family_dimension_ids
        .iter()
        .map(|id| document_family_dimension(manifest, id))
        .collect::<Result<Vec<_>, _>>()?;
    if family_dimensions.is_empty() {
        return Err("Select at least one family dimension distinct from X.".to_owned());
    }

    let mut encodings = Vec::new();
    if !studio.draft_family_color_dimension.is_empty() {
        encodings.push(FamilyEncodingMap::Color {
            dimension: document_family_dimension(manifest, &studio.draft_family_color_dimension)?,
            palette: AccessibleColorPalette::OkabeItoCategorical,
        });
    }
    if !studio.draft_family_dash_dimension.is_empty() {
        encodings.push(FamilyEncodingMap::Dash {
            dimension: document_family_dimension(manifest, &studio.draft_family_dash_dimension)?,
        });
    }
    if !studio.draft_family_marker_dimension.is_empty() {
        encodings.push(FamilyEncodingMap::Marker {
            dimension: document_family_dimension(manifest, &studio.draft_family_marker_dimension)?,
        });
    }
    let encoded: HashSet<_> = encodings
        .iter()
        .map(|encoding| encoding.dimension().key.as_str())
        .collect();
    let unencoded: Vec<_> = family_dimensions
        .iter()
        .filter(|dimension| !encoded.contains(dimension.key.as_str()))
        .cloned()
        .collect();
    if unencoded.len() > 1 {
        return Err(format!(
            "Dimensions {} require explicit visual encodings.",
            unencoded
                .iter()
                .map(|dimension| dimension.key.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if let Some(dimension) = unencoded.into_iter().next() {
        encodings.push(FamilyEncodingMap::Label {
            dimension,
            prefix: None,
        });
    }
    let color_dimensions: Vec<_> = encodings
        .iter()
        .filter_map(|encoding| match encoding {
            FamilyEncodingMap::Color { dimension, .. } => Some(dimension.clone()),
            _ => None,
        })
        .collect();
    for color_dimension in color_dimensions {
        if !encodings.iter().any(|encoding| {
            encoding.dimension() == &color_dimension
                && matches!(
                    encoding,
                    FamilyEncodingMap::Dash { .. }
                        | FamilyEncodingMap::Marker { .. }
                        | FamilyEncodingMap::Label { .. }
                )
        }) {
            if encodings
                .iter()
                .any(|encoding| matches!(encoding, FamilyEncodingMap::Label { .. }))
            {
                return Err(format!(
                    "Color dimension '{}' requires a matching dash or marker cue.",
                    color_dimension.key
                ));
            }
            encodings.push(FamilyEncodingMap::Label {
                dimension: color_dimension,
                prefix: None,
            });
        }
    }

    let policy = FamilyPresentationPolicy {
        x_dimension: FamilyXDimension {
            dimension: x_dimension,
            ordering: FamilyXOrdering::Source,
        },
        family_dimensions,
        facet_layout: None,
        aggregation: FamilyAggregationPolicy {
            method: FamilyAggregationMethod::None,
            over_dimensions: Vec::new(),
        },
        filter: manifest.compile_filter(&studio.family_query)?,
        missing_points: if studio.draft_family_exclude_missing {
            MissingPointPolicy::ExcludeWithOmissionRecord
        } else {
            MissingPointPolicy::PreserveAsNotRun
        },
        encodings,
    };
    policy.validate().map_err(|error| error.to_string())?;
    Ok(policy)
}

fn apply_family_policy_draft(app: &mut RSpiceApp) {
    let result = (|| {
        let manifest = active_family_manifest(app)?;
        let analysis = app
            .state
            .simulation
            .active_analysis()
            .ok_or_else(|| "No active analysis is selected.".to_owned())?;
        for waveform in &analysis.waveforms {
            manifest.compatible_waveform_len(waveform.x.len())?;
            if waveform.x.len() != waveform.y.len() {
                return Err(format!(
                    "Waveform '{}' has mismatched X and Y sample counts.",
                    waveform.name
                ));
            }
        }
        let policy = build_family_policy_draft(app, &manifest)?;
        let indices = manifest.matching_source_indices_for_filter(policy.filter.as_ref())?;
        if indices.is_empty() {
            return Err("The current filter selects no retained family points.".to_owned());
        }
        SourceSampleSelection::new(DatasetId::new(), analysis.id, indices.clone())?
            .with_family_presentation(&manifest, &policy)?;
        let pane_id = app
            .state
            .workbench
            .visualization_studio
            .active_pane
            .ok_or_else(|| "No visualization pane is selected.".to_owned())?;
        if app
            .state
            .workbench
            .visualization_studio
            .active_pane()
            .is_none_or(|pane| pane.viewer != ResultViewer::Waves)
        {
            return Err(
                "Choose the Waveform viewer before applying a sample-level family policy."
                    .to_owned(),
            );
        }
        if active_project_visualization_document_id(&app.state).is_some() {
            let canonical_pane = app
                .state
                .workspace
                .content
                .visualization_document(
                    active_project_visualization_document_id(&app.state)
                        .expect("canonical branch has an active document"),
                )
                .and_then(|document| {
                    document
                        .panes()
                        .iter()
                        .find(|pane| pane.id.get() == pane_id)
                        .map(|pane| pane.id)
                })
                .ok_or_else(|| "The active project result pane no longer exists.".to_owned())?;
            transact_active_project_document(
                app,
                vec![DocumentEdit::SetPaneFamilyPresentation {
                    pane_id: canonical_pane,
                    policy: Some(policy),
                }],
            )?;
            reconcile_document(app);
        } else {
            app.state
                .workbench
                .visualization_studio
                .transact(move |studio| {
                    studio.family_policies.insert(pane_id, policy);
                    Ok(())
                })?;
        }
        Ok::<_, String>(indices.len())
    })();
    match result {
        Ok(count) => app.state.push_user_message(ConsoleMessage::info(format!(
            "Applied exact family presentation to {count} retained point(s)."
        ))),
        Err(error) => app.state.push_user_message(ConsoleMessage::error(error)),
    }
}
