//! The viewer stage, its exact-data dock, and the inspector beside it.
//!
//! The stage draws only what the active binding actually resolves to: when a
//! retained result does not satisfy a viewer's contract the stage says so and
//! draws nothing, rather than rendering a partial or substituted view. The
//! exact-data dock is the paired numeric view — it lists the retained values a
//! curve was drawn from, never resampled or interpolated ones.

use super::*;
use rspice_results_ui::studio::{inspector, stage as presentation};

pub(super) fn viewer_stage(ui: &mut Ui, app: &mut RSpiceApp) {
    let selected = app
        .state
        .workbench
        .visualization_studio
        .selected_viewer_document
        .clone();
    let definition = viewer_document(&selected);
    let analysis_ids = available_analysis_ids(&app.state);
    let capabilities = ViewerCapabilities {
        analysis_ids: &analysis_ids,
        external_capabilities: &[],
    };
    let family_selection = active_family_sample_selection(app);
    let availability = active_binding_error(app).map_or_else(
        || {
            definition
                .ok_or_else(|| "Viewer identity is not registered".to_owned())
                .and_then(|definition| {
                    resolved_viewer_availability(&app.state, definition, capabilities)
                })
                .and_then(|viewer| {
                    family_selection
                        .as_ref()
                        .map_or_else(|error| Err(error.clone()), |_| Ok(viewer))
                })
        },
        Err,
    );
    presentation::viewer_stage(
        ui,
        definition,
        availability,
        &mut StageHost {
            app,
            family_selection: &family_selection,
        },
    );
}

struct StageHost<'a> {
    app: &'a mut RSpiceApp,
    family_selection: &'a Result<Option<SourceSampleSelection>, String>,
}

impl presentation::StageHost for StageHost<'_> {
    fn source_label(&self) -> String {
        self.app.state.simulation.active_run().map_or_else(
            || "No active dataset".to_owned(),
            |run| format!("{} · {}", run.label, short_dataset(run.dataset_id)),
        )
    }
    fn render(&mut self, ui: &mut Ui, viewer: ResultViewer) {
        let app = &mut *self.app;
        let family_selection = self.family_selection;
        app.state.ui.results.viewer = viewer;
        let interaction = match app.state.workbench.visualization_studio.tool {
            ViewerTool::Select => crate::ui::plot::InteractionMode::Select,
            ViewerTool::Pan => crate::ui::plot::InteractionMode::Pan,
            ViewerTool::Zoom => crate::ui::plot::InteractionMode::Zoom,
        };
        crate::ui::plot::set_interaction_mode(ui.ctx(), interaction);
        result_document::show_embedded_with_sample_selection(
            ui,
            app,
            family_selection.as_ref().ok().cloned().flatten(),
        );
        capture_active_link_state(ui.ctx(), app);
        paint_visualization_markers(ui, app);
        crate::ui::plot::set_interaction_mode(ui.ctx(), crate::ui::plot::InteractionMode::All);
    }
    fn revision(&self) -> u64 {
        self.app.state.workbench.visualization_studio.revision
    }
    fn exact_rows(&self) -> Vec<ExactSourceRow> {
        exact_source_rows(&self.app.state)
    }
}

/// Whether the sheet on screen draws the analysis's own retained sweep on its
/// horizontal axis.
///
/// A marker or annotation is anchored by an X coordinate taken from a
/// retained waveform — seconds on a transient, hertz on a noise sweep. The
/// overlay maps that coordinate onto whatever the well is currently showing,
/// so it must only draw where the well's horizontal axis is that same sweep.
/// The unit-pane stack is exactly that set; the derived sheets — the folded
/// eye, the binned histogram, the spectrum, the Smith and Nyquist charts —
/// compute their own abscissa, and a time marker placed on one of those is
/// drawn at a position that means nothing.
pub(super) fn marker_domain_matches_the_pane(viewer: ResultViewer) -> bool {
    result_document::viewer_uses_wave_stack(viewer)
}

pub(super) fn paint_visualization_markers(ui: &Ui, app: &mut RSpiceApp) {
    if !marker_domain_matches_the_pane(app.state.ui.results.viewer) {
        return;
    }
    let Some(well) = app.state.ui.results.well_rect else {
        return;
    };
    let Some(analysis_index) = app.state.simulation.active_analysis_idx else {
        return;
    };
    let source = app.state.simulation.active_run().and_then(|run| {
        let analysis = run.analyses.get(analysis_index)?;
        let waveform = analysis
            .waveforms
            .iter()
            .find(|waveform| waveform.visible)?;
        let (source_min, source_max) = waveform.x_range();
        Some((run.dataset_id, analysis.id, source_min, source_max))
    });
    let Some((dataset_id, analysis_sequence, source_min, source_max)) = source else {
        return;
    };
    let (x_min, x_max) = result_document::active_renderer_axis_range(
        ui.ctx(),
        &mut app.state,
        result_document::PaneAxis::X,
    )
    .unwrap_or((source_min, source_max));
    presentation::paint_markers(
        ui,
        well,
        (x_min, x_max),
        app.state
            .workbench
            .visualization_studio
            .markers
            .iter()
            .filter(|marker| {
                marker.dataset_id == dataset_id && marker.analysis_sequence == analysis_sequence
            }),
        app.state
            .workbench
            .visualization_studio
            .annotations
            .iter()
            .filter(|annotation| {
                annotation.dataset_id == dataset_id
                    && annotation.analysis_sequence == analysis_sequence
            }),
    );
}

pub(super) fn active_binding_error(app: &RSpiceApp) -> Option<String> {
    let studio = &app.state.workbench.visualization_studio;
    let active = studio.active_pane?;
    let pane = studio.panes.iter().find(|pane| pane.id == active)?;
    let Some(run) = app
        .state
        .simulation
        .runs
        .iter()
        .find(|run| run.dataset_id == pane.dataset_id)
    else {
        return Some(format!(
            "Bound dataset {} is no longer retained; the pane was not retargeted",
            short_dataset(pane.dataset_id)
        ));
    };
    (!run
        .analyses
        .iter()
        .any(|analysis| analysis.id == pane.analysis_sequence))
    .then(|| {
        format!(
            "Bound analysis {} is no longer retained in dataset {}; the pane was not retargeted",
            pane.analysis_sequence,
            short_dataset(pane.dataset_id)
        )
    })
}

pub(super) struct RetainedPoleZeroPayload<'a> {
    pub(super) poles: &'a [crate::state::ComplexResultValue],
    pub(super) zeros: &'a [crate::state::ComplexResultValue],
    pub(super) pole_evidence: &'a crate::state::PoleZeroRootSetEvidence,
    pub(super) zero_evidence: &'a crate::state::PoleZeroRootSetEvidence,
    pub(super) gain: Option<f64>,
}

pub(super) fn exact_source_rows(state: &AppState) -> Vec<ExactSourceRow> {
    let Some(run) = state.simulation.active_run() else {
        return Vec::new();
    };
    let Some(analysis_index) = state.simulation.active_analysis_idx else {
        return Vec::new();
    };
    let Some(analysis) = run.analyses.get(analysis_index) else {
        return Vec::new();
    };
    if let Some(payload) = retained_pole_zero_payload(analysis) {
        return exact_pole_zero_rows(
            run,
            analysis,
            payload.poles,
            payload.zeros,
            payload.pole_evidence,
            payload.zero_evidence,
            payload.gain,
        );
    }
    if let Some(evidence) = retained_sensitivity_study(analysis) {
        return exact_sensitivity_study_rows(run, analysis, evidence);
    }
    if let Some((output, result_mode, rows)) = retained_sensitivity_payload(analysis) {
        return exact_sensitivity_rows(run, analysis, output, result_mode, rows);
    }
    let Some(waveform) = analysis.waveforms.iter().find(|waveform| waveform.visible) else {
        return Vec::new();
    };
    let count = waveform.x.len().min(waveform.y.len());
    if count == 0 {
        return Vec::new();
    }
    let mut indices = vec![0, count / 2, count - 1];
    for cursor in [state.ui.results.cursors.a, state.ui.results.cursors.b]
        .into_iter()
        .flatten()
    {
        if let Some(index) = waveform.x[..count]
            .iter()
            .enumerate()
            .filter(|(_, value)| value.is_finite())
            .min_by(|(_, left), (_, right)| {
                (*left - cursor).abs().total_cmp(&(*right - cursor).abs())
            })
            .map(|(index, _)| index)
        {
            indices.push(index);
        }
    }
    indices.sort_unstable();
    indices.dedup();
    indices
        .into_iter()
        .take(5)
        .map(|index| ExactSourceRow {
            binding: short_dataset(run.dataset_id),
            stable_row: format!("{}:{index}", analysis.id),
            coordinate: format!("x={:.17e}", waveform.x[index]),
            value: format!("{:.17e}", waveform.y[index]),
            origin: waveform.name.clone(),
        })
        .collect()
}

pub(super) fn retained_pole_zero_payload(
    analysis: &AnalysisResult,
) -> Option<RetainedPoleZeroPayload<'_>> {
    if !analysis.success || analysis.analysis_type != AnalysisType::PoleZero {
        return None;
    }
    let payload = analysis.result_payload.as_ref()?;
    if payload.validate_for(analysis.analysis_type).is_err() {
        return None;
    }
    match payload {
        AnalysisResultPayload::PoleZero {
            poles,
            zeros,
            pole_evidence,
            zero_evidence,
            gain,
        } => Some(RetainedPoleZeroPayload {
            poles: poles.as_slice(),
            zeros: zeros.as_slice(),
            pole_evidence,
            zero_evidence,
            gain: *gain,
        }),
        AnalysisResultPayload::DcSweep { .. }
        | AnalysisResultPayload::OperatingPoint { .. }
        | AnalysisResultPayload::PssFloquet { .. }
        | AnalysisResultPayload::Pstb { .. }
        | AnalysisResultPayload::Sensitivity { .. }
        | AnalysisResultPayload::SensitivityStudy { .. }
        | AnalysisResultPayload::DcMismatch { .. }
        | AnalysisResultPayload::TransferFunction { .. }
        | AnalysisResultPayload::ScalarMeasurements { .. }
        | AnalysisResultPayload::Soa { .. }
        | AnalysisResultPayload::TransientEvents { .. }
        | AnalysisResultPayload::FftSpectrum { .. }
        | AnalysisResultPayload::Qpac { .. }
        | AnalysisResultPayload::Qpxf { .. }
        | AnalysisResultPayload::Qpnoise { .. }
        | AnalysisResultPayload::Qpss { .. } => None,
    }
}

/// The study a validated analysis retains, if that is the family it carries.
pub(super) fn retained_sensitivity_study(
    analysis: &AnalysisResult,
) -> Option<&crate::state::SensitivityStudyEvidence> {
    if !analysis.success || analysis.analysis_type != AnalysisType::Sensitivity {
        return None;
    }
    let payload = analysis.result_payload.as_ref()?;
    if payload.validate_for(analysis.analysis_type).is_err() {
        return None;
    }
    match payload {
        AnalysisResultPayload::SensitivityStudy { evidence } => Some(evidence.as_ref()),
        _ => None,
    }
}

/// One exact row per variable, per point, per quantity.
///
/// The coordinate names the point as well as the variable, because a study of
/// a sweep answers the same question sixty-one times and a row that named
/// only the variable would be sixty-one rows wearing one address.
pub(super) fn exact_sensitivity_study_rows(
    run: &SimulationRun,
    analysis: &AnalysisResult,
    evidence: &crate::state::SensitivityStudyEvidence,
) -> Vec<ExactSourceRow> {
    let points = evidence.point_count();
    let mut exact = Vec::with_capacity(evidence.rows.len() * points * 3);
    for (index, row) in evidence.rows.iter().enumerate() {
        for point in 0..points {
            let basis = evidence.frequency_at(point).map_or_else(
                || "dc".to_owned(),
                |frequency| format!("ac@{frequency:.17e}Hz"),
            );
            for (quantity, column) in [
                ("raw", &row.raw),
                ("normalized", &row.normalized),
                ("phase", &row.phase),
            ] {
                let Some(value) = column.get(point) else {
                    continue;
                };
                exact.push(ExactSourceRow {
                    binding: short_dataset(run.dataset_id),
                    stable_row: format!("{}:sensitivity[{index}].{quantity}[{point}]", analysis.id),
                    coordinate: format!("parameter={};basis={basis}", row.parameter),
                    value: match value {
                        rspice_core::analysis::sensitivity::SensitivityValue::Available(value) => {
                            format!("{value:.17e}")
                        }
                        rspice_core::analysis::sensitivity::SensitivityValue::Unavailable {
                            unavailable,
                        } => format!("unavailable:{}", unavailable.as_str()),
                    },
                    origin: evidence.output.clone(),
                });
            }
        }
    }
    exact
}

pub(super) fn retained_sensitivity_payload(
    analysis: &AnalysisResult,
) -> Option<(&str, SensitivityResultMode, &[SensitivityResultRow])> {
    if !analysis.success || analysis.analysis_type != AnalysisType::Sensitivity {
        return None;
    }
    let payload = analysis.result_payload.as_ref()?;
    if payload.validate_for(analysis.analysis_type).is_err() {
        return None;
    }
    match payload {
        AnalysisResultPayload::Sensitivity {
            output,
            result_mode,
            rows,
        } => Some((output.as_str(), *result_mode, rows.as_slice())),
        // The Studio pane's exact-row machinery is sensitivity-only and says
        // so through its own refusal; a DC mismatch payload is declined here
        // rather than half-rendered.
        AnalysisResultPayload::DcSweep { .. }
        | AnalysisResultPayload::OperatingPoint { .. }
        | AnalysisResultPayload::PoleZero { .. }
        | AnalysisResultPayload::PssFloquet { .. }
        | AnalysisResultPayload::Pstb { .. }
        | AnalysisResultPayload::SensitivityStudy { .. }
        | AnalysisResultPayload::DcMismatch { .. }
        | AnalysisResultPayload::TransferFunction { .. }
        | AnalysisResultPayload::ScalarMeasurements { .. }
        | AnalysisResultPayload::Soa { .. }
        | AnalysisResultPayload::TransientEvents { .. }
        | AnalysisResultPayload::FftSpectrum { .. }
        | AnalysisResultPayload::Qpac { .. }
        | AnalysisResultPayload::Qpxf { .. }
        | AnalysisResultPayload::Qpnoise { .. }
        | AnalysisResultPayload::Qpss { .. } => None,
    }
}

pub(super) fn exact_sensitivity_rows(
    run: &SimulationRun,
    analysis: &AnalysisResult,
    output: &str,
    result_mode: SensitivityResultMode,
    rows: &[SensitivityResultRow],
) -> Vec<ExactSourceRow> {
    let basis = match result_mode {
        SensitivityResultMode::Dc => "dc".to_owned(),
        SensitivityResultMode::Ac { frequency_hz } => {
            format!("ac@{frequency_hz:.17e}Hz")
        }
    };
    let mut exact = Vec::with_capacity(rows.len() * 2);
    for (index, row) in rows.iter().enumerate() {
        for (quantity, value) in [("raw", row.raw), ("normalized", row.normalized)] {
            exact.push(ExactSourceRow {
                binding: short_dataset(run.dataset_id),
                stable_row: format!("{}:sensitivity[{index}].{quantity}", analysis.id),
                coordinate: format!("parameter={};basis={basis}", row.parameter),
                value: match value {
                    rspice_core::analysis::sensitivity::SensitivityValue::Available(value) => {
                        format!("{value:.17e}")
                    }
                    rspice_core::analysis::sensitivity::SensitivityValue::Unavailable {
                        unavailable,
                    } => format!("unavailable:{}", unavailable.as_str()),
                },
                origin: output.to_owned(),
            });
        }
    }
    exact
}

pub(super) fn exact_pole_zero_rows(
    run: &SimulationRun,
    analysis: &AnalysisResult,
    poles: &[crate::state::ComplexResultValue],
    zeros: &[crate::state::ComplexResultValue],
    pole_evidence: &crate::state::PoleZeroRootSetEvidence,
    zero_evidence: &crate::state::PoleZeroRootSetEvidence,
    gain: Option<f64>,
) -> Vec<ExactSourceRow> {
    let mut rows = Vec::with_capacity(11 + (poles.len() + zeros.len()) * 2);
    rows.push(ExactSourceRow {
        binding: short_dataset(run.dataset_id),
        stable_row: format!("{}:gain", analysis.id),
        coordinate: "scalar".to_owned(),
        value: gain
            .map(|gain| format!("{gain:.17e}"))
            .unwrap_or_else(|| "unavailable".to_owned()),
        origin: "DC transfer gain".to_owned(),
    });
    append_root_evidence_rows(&mut rows, run, analysis, "pole", pole_evidence);
    append_root_evidence_rows(&mut rows, run, analysis, "zero", zero_evidence);
    for (kind, roots) in [("pole", poles), ("zero", zeros)] {
        for (index, root) in roots.iter().enumerate() {
            for (component, value) in [("real", root.real), ("imaginary", root.imaginary)] {
                rows.push(ExactSourceRow {
                    binding: short_dataset(run.dataset_id),
                    stable_row: format!("{}:{kind}[{index}].{component}", analysis.id),
                    coordinate: format!("{kind}[{index}].{component}"),
                    value: format!("{value:.17e}"),
                    origin: format!("ordered {kind} root"),
                });
            }
        }
    }
    rows
}

fn append_root_evidence_rows(
    rows: &mut Vec<ExactSourceRow>,
    run: &SimulationRun,
    analysis: &AnalysisResult,
    kind: &str,
    evidence: &crate::state::PoleZeroRootSetEvidence,
) {
    rows.push(ExactSourceRow {
        binding: short_dataset(run.dataset_id),
        stable_row: format!("{}:{kind}_evidence.status", analysis.id),
        coordinate: format!("{kind}_evidence.status"),
        value: evidence.label().to_owned(),
        origin: "root-set qualification".to_owned(),
    });
    let Some(certificate) = evidence.certificate() else {
        return;
    };
    for (field, value) in [
        ("problem_order", certificate.problem_order.to_string()),
        ("infinite_count", certificate.infinite_count.to_string()),
        (
            "max_backward_error",
            format!("{:.17e}", certificate.max_backward_error),
        ),
        (
            "qualification_tolerance",
            format!("{:.17e}", certificate.qualification_tolerance),
        ),
    ] {
        rows.push(ExactSourceRow {
            binding: short_dataset(run.dataset_id),
            stable_row: format!("{}:{kind}_evidence.{field}", analysis.id),
            coordinate: format!("{kind}_evidence.{field}"),
            value,
            origin: "root-set qualification certificate".to_owned(),
        });
    }
}

/// A dataset identity as a stage label spells it, elided by
/// [`crate::product::short_identity`] so it reads as the same prefix the
/// navigator and the specification band show for the same dataset.
pub(super) fn short_dataset(id: DatasetId) -> String {
    crate::product::short_identity(id)
}

pub(super) fn result_entity_rows(state: &AppState) -> Vec<ResultEntityRow> {
    let mut rows = Vec::new();
    if let (Some(run), Some(analysis)) = (
        state.simulation.active_run(),
        state.simulation.active_analysis(),
    ) {
        let dataset = short_dataset(run.dataset_id);
        if let Some(waveform) = analysis.waveforms.first() {
            rows.push(ResultEntityRow {
                identity: format!("axis:{}:x", analysis.id),
                kind: "axis",
                binding: format!("{} · source X coordinate", dataset),
                state: format!("{} exact rows", waveform.x.len()),
            });
        }
        for (index, waveform) in analysis.waveforms.iter().enumerate() {
            rows.push(ResultEntityRow {
                identity: format!("trace:{}:{index}", analysis.id),
                kind: "trace",
                binding: format!("{} · {}", dataset, waveform.name),
                state: if waveform.visible {
                    "visible · source exact".to_owned()
                } else {
                    "hidden · retained".to_owned()
                },
            });
        }
    }
    for (label, coordinate) in [
        ("A", state.ui.results.cursors.a),
        ("B", state.ui.results.cursors.b),
    ] {
        if let Some(coordinate) = coordinate {
            rows.push(ResultEntityRow {
                identity: format!("cursor:{label}"),
                kind: "cursor",
                binding: format!("x={coordinate:.17e}"),
                state: "nearest source row".to_owned(),
            });
        }
    }
    for marker in &state.workbench.visualization_studio.markers {
        rows.push(ResultEntityRow {
            identity: format!("marker:{}", marker.id),
            kind: "marker",
            binding: format!(
                "{} · {}[{}]",
                short_dataset(marker.dataset_id),
                marker.waveform_name,
                marker.sample_index
            ),
            state: marker.label.clone(),
        });
    }
    for measurement in &state.workbench.visualization_studio.measurements {
        rows.push(ResultEntityRow {
            identity: format!("measurement:{}", measurement.id),
            kind: "measurement",
            binding: measurement.expression.clone(),
            state: format!("{:.17e}", measurement.value),
        });
    }
    for annotation in &state.workbench.visualization_studio.annotations {
        rows.push(ResultEntityRow {
            identity: format!("annotation:{}", annotation.id),
            kind: "annotation",
            binding: format!(
                "{} · analysis {} · x={:.9e}",
                short_dataset(annotation.dataset_id),
                annotation.analysis_sequence,
                annotation.x
            ),
            state: annotation.text.clone(),
        });
    }
    rows
}

pub(super) fn viewer_inspector(ui: &mut Ui, app: &mut RSpiceApp, compact: bool) {
    inspector::viewer_inspector(ui, &mut InspectorHost(app), compact);
}

struct InspectorHost<'a>(&'a mut RSpiceApp);

impl inspector::InspectorHost for InspectorHost<'_> {
    fn entities(&self) -> Vec<ResultEntityRow> {
        result_entity_rows(&self.0.state)
    }
    fn latest_comparison(&self) -> Option<&ComparisonReceipt> {
        self.0
            .state
            .workbench
            .visualization_studio
            .comparison_receipts
            .last()
    }
    fn operation(&self) -> inspector::OperationProgress {
        let studio = &self.0.state.workbench.visualization_studio;
        inspector::OperationProgress {
            state: studio.operation_state,
            processed: studio.operation_processed,
            total: studio.operation_total,
            checksum: studio.operation_checksum,
        }
    }
    fn source_available(&self) -> bool {
        source_integrity_scan_binding(&self.0.state).is_some()
    }
    fn request(&mut self, action: inspector::InspectorAction) {
        use inspector::InspectorAction;
        match action {
            InspectorAction::OpenComparison => open_dock(self.0, VisualizationDock::Comparison),
            InspectorAction::Start => start_source_integrity_scan(self.0),
            InspectorAction::Advance => {
                if let Err(error) = advance_source_integrity_scan(self.0) {
                    self.0.state.push_user_message(ConsoleMessage::error(error));
                }
            }
            InspectorAction::Cancel => {
                self.0.state.workbench.visualization_studio.operation_state =
                    OperationState::Cancelled
            }
            InspectorAction::Recover => {
                if let Err(error) = recover_source_integrity_scan(self.0) {
                    self.0.state.push_user_message(ConsoleMessage::error(error));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Markers overlay only panes whose horizontal axis is the sweep the
    /// marker's coordinate was taken in.
    ///
    /// The overlay maps a retained X coordinate onto whatever the well is
    /// showing. On the folded eye, the binned histogram, the spectrum or a
    /// Smith chart the abscissa is computed by the sheet, so a marker placed
    /// in seconds was drawn at a position that means nothing.
    #[test]
    fn markers_overlay_only_panes_that_draw_the_retained_sweep() {
        for viewer in [
            ResultViewer::Waves,
            ResultViewer::DcSweep,
            ResultViewer::Bode,
            ResultViewer::NoiseContrib,
        ] {
            assert!(
                marker_domain_matches_the_pane(viewer),
                "{viewer:?} draws the retained sweep"
            );
        }
        for viewer in [
            ResultViewer::Fft,
            ResultViewer::Eye,
            ResultViewer::Hist,
            ResultViewer::Smith,
            ResultViewer::Nyquist,
            ResultViewer::PhaseNoise,
            ResultViewer::HarmonicBalance,
        ] {
            assert!(
                !marker_domain_matches_the_pane(viewer),
                "{viewer:?} computes its own abscissa"
            );
        }
    }
}
