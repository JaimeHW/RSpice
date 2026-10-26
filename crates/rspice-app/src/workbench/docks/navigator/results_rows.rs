//! The data browser's rows: the artifacts a retained analysis yields, the
//! virtualized quantity list, and the painting of each kind of row.

use egui::{Response, ScrollArea, Ui};

use crate::state::bus_notations;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::documents::result_document::{AnalysisPresentationKey, ResultsParts};
use crate::workbench::results::commands::ResultsCommand;
use rspice_results::result_presentation::ResultArtifactPresentationKey;
use rspice_results_ui::selection::ResultBrowserSelectionKey;

use super::super::super::design_system::WorkbenchIcon;
use super::results::{
    ResultArtifact, ResultArtifactKind, ResultRun, ResultSignalVirtualRow, ResultsBrowserKind,
    ResultsBrowserScope, data_state_meta, result_browser_dataset_expanded,
    result_browser_group_expanded, results_browser_clear_filters,
    set_result_browser_dataset_expanded, set_result_browser_group_expanded,
};
use super::results_actions::{
    execute, handle_result_artifact_keyboard, handle_result_signal_keyboard,
    result_artifact_context_menu, result_signal_context_menu, result_signal_row_id,
    select_result_analysis_by_key, select_result_artifact, select_result_signal_by_key,
    toggle_result_signal_visibility, update_result_browser_multi_selection,
};
use super::{grouped_count, muted};

pub(super) const EXPRESSION_HEADER_HEIGHT: f32 = 28.0;
pub(super) const SIGNAL_ROW_HEIGHT: f32 = 30.0;
// The results browser's two-register quantity row and its group head, from
// the mockup's `.result-browser-quantity` / `.result-browser-analysis-head`.
pub(super) const RESULT_QUANTITY_ROW_HEIGHT: f32 = 36.0;
const RESULT_ANALYSIS_HEAD_HEIGHT: f32 = 31.0;
const RESULT_BROWSER_VIRTUALIZATION_THRESHOLD: usize = 250;
const RESULT_BROWSER_RENDER_WINDOW_ROWS: usize = 120;
const RESULT_BROWSER_OVERSCAN_ROWS: usize = 20;
const RESULT_MANIFEST_ROW_HEIGHT: f32 = 26.0;
pub(super) const TOUCH_TARGET_HEIGHT: f32 = crate::ui::tokens::TOUCH_TARGET;

/// A result control's height: its own desktop height, or in a touch
/// composition the theme's touch target, which Accessibility may raise.
pub(super) fn responsive_result_control_height(desktop_height: f32, control_height: f32) -> f32 {
    if control_height >= TOUCH_TARGET_HEIGHT {
        control_height
    } else {
        desktop_height
    }
}

pub(super) fn retained_result_artifacts(
    analysis: &crate::state::AnalysisResult,
    presentation_key: AnalysisPresentationKey,
) -> Vec<ResultArtifact> {
    use crate::state::{AnalysisResultFamilyMetadata, AnalysisResultPayload};
    use crate::workbench::ResultViewer;

    let mut artifacts = Vec::new();
    let mut push = |canonical: &str,
                    name: String,
                    kind: ResultArtifactKind,
                    count: usize,
                    unit: &str,
                    value: Option<String>,
                    viewer: ResultViewer| {
        // How much is in it and what it is in. The kind word used to be
        // printed twice over — "Array / array[12] / V" — and the row's own
        // glyph column already says which kind it is.
        let shape = match kind {
            ResultArtifactKind::Scalar => String::new(),
            ResultArtifactKind::Array => format!("{} values", grouped_count(count)),
            ResultArtifactKind::EventStream => format!("{} events", grouped_count(count)),
            ResultArtifactKind::Contribution => {
                format!("{} contributions", grouped_count(count))
            }
        };
        let unit_dimension = unit.to_owned();
        let meta = match (shape.is_empty(), unit.is_empty()) {
            (true, true) => String::new(),
            (true, false) => unit.to_owned(),
            (false, true) => shape,
            (false, false) => format!("{shape} · {unit}"),
        };
        artifacts.push(ResultArtifact {
            identity: ResultArtifactPresentationKey::new(presentation_key, canonical),
            name,
            kind,
            unit: unit_dimension,
            meta,
            value,
            viewer,
        });
    };

    if let Some(dc) = &analysis.dc_op {
        for (canonical, name, values, unit) in [
            (
                "dc-op/node-voltages",
                "Node voltages",
                dc.node_voltages.as_slice(),
                "V",
            ),
            (
                "dc-op/branch-currents",
                "Branch currents",
                dc.branch_currents.as_slice(),
                "A",
            ),
            (
                "dc-op/power-dissipation",
                "Device power dissipation",
                dc.power_dissipation.as_slice(),
                "W",
            ),
        ] {
            if !values.is_empty() {
                push(
                    canonical,
                    name.to_owned(),
                    ResultArtifactKind::Array,
                    values.len(),
                    unit,
                    None,
                    ResultViewer::Op,
                );
            }
        }
    }
    if let Some(report) = &analysis.device_op
        && !report.entries.is_empty()
    {
        push(
            "dc-op/device-report",
            "Device operating points".to_owned(),
            ResultArtifactKind::Array,
            report.entries.len(),
            "",
            None,
            ResultViewer::Op,
        );
    }
    if let Some(noise) = &analysis.noise_summary {
        if !noise.rows.is_empty() {
            push(
                "noise/contributions",
                "Integrated noise contributors".to_owned(),
                ResultArtifactKind::Contribution,
                noise.rows.len(),
                "V^2",
                None,
                ResultViewer::NoiseContrib,
            );
        }
        for (canonical, name, value) in [
            ("noise/output-rms", "Output noise RMS", noise.total_rms),
            (
                "noise/input-rms",
                "Input-referred noise RMS",
                noise.input_rms,
            ),
        ] {
            if let Some(value) = value {
                push(
                    canonical,
                    name.to_owned(),
                    ResultArtifactKind::Scalar,
                    1,
                    "V",
                    Some(crate::ui::plot::fmt_si(value, "V", 3)),
                    ResultViewer::NoiseContrib,
                );
            }
        }
    }
    if let Some(payload) = &analysis.result_payload {
        let (canonical, name, kind, count, value, viewer) = match payload {
            AnalysisResultPayload::Stb { response } => (
                "payload/stb",
                "Loop-gain samples, margins and diagnostics",
                ResultArtifactKind::Array,
                response.bode_points.len(),
                Some(response.margin_assessment().to_owned()),
                ResultViewer::Bode,
            ),
            AnalysisResultPayload::Qpnoise { response } => (
                "payload/qpnoise",
                "QPNOISE spectra, covariance and measurements",
                ResultArtifactKind::Array,
                response
                    .points
                    .iter()
                    .flat_map(|p| &p.adjoints)
                    .flat_map(|a| &a.sensitivities)
                    .map(Vec::len)
                    .sum::<usize>(),
                Some(format!(
                    "{} frequencies · {} outputs · {} noise mechanisms",
                    response.points.len(),
                    response.outputs.len(),
                    response.sources.len()
                )),
                ResultViewer::NoiseContrib,
            ),
            AnalysisResultPayload::Qpxf { response } => (
                "payload/qpxf",
                "QPXF unit transfers and adjoints",
                ResultArtifactKind::Array,
                response
                    .solutions
                    .iter()
                    .flat_map(|s| &s.sensitivities)
                    .map(Vec::len)
                    .sum::<usize>()
                    + response
                        .transfers
                        .iter()
                        .map(|t| t.values.len())
                        .sum::<usize>(),
                Some(format!(
                    "{} output frequencies · {} sources × {} input tuples · output {}",
                    response.solutions.len(),
                    response.metadata.input_sources.len(),
                    response.metadata.input_lattices.len(),
                    harmonic_tuple(&response.metadata.request.output_lattice)
                )),
                ResultViewer::Table,
            ),
            AnalysisResultPayload::Qpac { response } => (
                "payload/qpac",
                "QPAC complete complex response",
                ResultArtifactKind::Array,
                response
                    .unit_solutions
                    .iter()
                    .flat_map(|s| &s.spectra)
                    .map(Vec::len)
                    .sum(),
                Some(format!(
                    "{} offsets · {} signed tuples · input {} → output {}",
                    response.unit_solutions.len(),
                    response.metadata.tuples.len(),
                    harmonic_tuple(&response.metadata.request.input_lattice),
                    harmonic_tuple(&response.metadata.request.output_lattice)
                )),
                ResultViewer::Table,
            ),
            AnalysisResultPayload::Qpss { operating_point } => (
                "payload/qpss",
                "QPSS spectrum",
                ResultArtifactKind::Array,
                operating_point.spectra().iter().map(Vec::len).sum(),
                Some(format!(
                    "{} independent tones · {} nodes · {} branch currents",
                    operating_point.config().grid.frequencies_hz.len(),
                    operating_point.node_names().len(),
                    operating_point.branch_names().len()
                )),
                ResultViewer::HarmonicBalance,
            ),
            AnalysisResultPayload::DcSweep { evidence } => (
                "payload/dc-sweep",
                "DC sweep curves",
                ResultArtifactKind::Array,
                analysis.waveforms.len(),
                Some(format!(
                    "{} · {} · {} members",
                    evidence.source,
                    evidence.direction.label(),
                    evidence.member_count()
                )),
                ResultViewer::Table,
            ),
            AnalysisResultPayload::OperatingPoint { mna_solution, .. } => (
                "payload/operating-point",
                "Operating-point solution",
                ResultArtifactKind::Array,
                mna_solution.len(),
                None,
                ResultViewer::Op,
            ),
            AnalysisResultPayload::PoleZero {
                poles,
                zeros,
                pole_evidence,
                zero_evidence,
                gain,
            } => (
                "payload/pole-zero",
                "Poles, zeros, gain, and qualification",
                ResultArtifactKind::Array,
                poles.len() + zeros.len() + 3,
                Some(format!(
                    "{} · poles {} · zeros {}",
                    gain.map(|gain| format!("gain {}", crate::ui::plot::fmt_si(gain, "", 4)))
                        .unwrap_or_else(|| "gain unavailable".to_owned()),
                    pole_evidence.label(),
                    zero_evidence.label()
                )),
                ResultViewer::PoleZero,
            ),
            AnalysisResultPayload::PssFloquet {
                multipliers,
                orbit_kind,
                stability_verdict,
                ..
            } => (
                "payload/pss-floquet",
                "Floquet multipliers",
                ResultArtifactKind::Array,
                multipliers.len(),
                Some(format!(
                    "{} multipliers{} · {}",
                    multipliers.len(),
                    floquet_orbit_navigator_label(*orbit_kind),
                    floquet_verdict_navigator_label(*stability_verdict),
                )),
                ResultViewer::Table,
            ),
            AnalysisResultPayload::Pstb {
                modes,
                orbit_kind,
                stability_verdict,
                stability_classification,
                num_unstable,
                ..
            } => (
                "payload/pstb-floquet",
                "Floquet modes",
                ResultArtifactKind::Array,
                modes.len(),
                Some(format!(
                    "{} modes{}{} · {} ({})",
                    modes.len(),
                    num_unstable
                        .map(|count| format!(", {count} unstable"))
                        .unwrap_or_default(),
                    floquet_orbit_navigator_label(*orbit_kind),
                    floquet_verdict_navigator_label(*stability_verdict),
                    pstb_classification_navigator_label(*stability_classification),
                )),
                ResultViewer::Table,
            ),
            AnalysisResultPayload::Sensitivity { rows, .. } => (
                "payload/sensitivity",
                "Sensitivity coefficients",
                ResultArtifactKind::Array,
                rows.len(),
                None,
                ResultViewer::Contribution,
            ),
            // The count is the rows the study holds, not the rows times the
            // points: one variable is one artifact of the study whether it
            // was solved at one frequency or sixty-one.
            AnalysisResultPayload::SensitivityStudy { evidence } => (
                "payload/sensitivity",
                "Sensitivity coefficients",
                ResultArtifactKind::Array,
                evidence.rows.len(),
                None,
                ResultViewer::Contribution,
            ),
            AnalysisResultPayload::DcMismatch { evidence } => (
                "payload/dc-mismatch",
                "DC mismatch contributors",
                ResultArtifactKind::Contribution,
                evidence.contributors.len(),
                None,
                ResultViewer::Contribution,
            ),
            AnalysisResultPayload::ScalarMeasurements { values } => (
                "payload/scalar-measurements",
                "Scalar result values",
                ResultArtifactKind::Scalar,
                values.len(),
                None,
                ResultViewer::Table,
            ),
            AnalysisResultPayload::TransferFunction {
                gain,
                input_resistance,
                output_resistance,
                ..
            } => (
                "payload/transfer-function",
                "Transfer-function scalars",
                ResultArtifactKind::Scalar,
                [gain, input_resistance, output_resistance]
                    .into_iter()
                    .filter(|value| value.is_some())
                    .count(),
                None,
                ResultViewer::TransferFunction,
            ),
            AnalysisResultPayload::Soa {
                source_history: _,
                evaluations,
                violations,
            } => (
                "payload/safe-operating-area",
                "Safe-operating-area checks",
                ResultArtifactKind::Contribution,
                evaluations.len() + violations.len(),
                None,
                ResultViewer::Soa,
            ),
            AnalysisResultPayload::FftSpectrum { spectrum } => (
                "payload/recorded-fft",
                "Recorded FFT spectrum",
                ResultArtifactKind::Array,
                spectrum.bin_count(),
                Some(format!(
                    "{} · {} points · {}",
                    spectrum.output, spectrum.point_count, spectrum.window
                )),
                ResultViewer::HarmonicBalance,
            ),
            AnalysisResultPayload::TransientEvents {
                digital_traces,
                real_traces,
                digital_buses,
                current_impulses,
                ..
            } => {
                // One child row per declaration, under the artifact that
                // carries them: a bus is a node of the browser in its own
                // right — it is what the Events sheet lists and what a dump
                // writes as one vector — and its members are the traces
                // already listed beside it, so the row states the range and
                // the width rather than repeating them.
                for bus in digital_buses {
                    push(
                        "payload/transient-events/bus",
                        format!("Digital bus {}[{}:{}]", bus.name, bus.msb, bus.lsb),
                        ResultArtifactKind::EventStream,
                        bus.members.len(),
                        "",
                        Some(bus.members.join(" ")),
                        ResultViewer::Events,
                    );
                }
                (
                    "payload/transient-events",
                    "Committed transient events",
                    ResultArtifactKind::EventStream,
                    digital_traces
                        .iter()
                        .map(|trace| trace.points.len())
                        .sum::<usize>()
                        + real_traces
                            .iter()
                            .map(|trace| trace.points.len())
                            .sum::<usize>()
                        + current_impulses.as_ref().map_or(0, |history| {
                            history
                                .traces
                                .iter()
                                .map(|trace| trace.points.len())
                                .sum::<usize>()
                        }),
                    None,
                    ResultViewer::Events,
                )
            }
        };
        // DC metadata alone lives in the inspector; this table route requires
        // retained curves to display. The raw MNA solution is the node
        // voltages and branch currents listed above it, so it has no row of
        // its own.
        let duplicates_a_row = matches!(payload, AnalysisResultPayload::OperatingPoint { .. });
        let needs_curves = matches!(payload, AnalysisResultPayload::DcSweep { .. }) && count == 0;
        if !duplicates_a_row && !needs_curves {
            push(canonical, name.to_owned(), kind, count, "", value, viewer);
        }
    }
    if let Some(metadata) = &analysis.family_metadata {
        let (name, count, viewer) = match metadata {
            AnalysisResultFamilyMetadata::Parametric { sweep_values, .. } => {
                ("Sweep values", sweep_values.len(), ResultViewer::Waves)
            }
            AnalysisResultFamilyMetadata::Corner { x_values, .. } => {
                ("Corner list", x_values.len(), ResultViewer::Waves)
            }
            AnalysisResultFamilyMetadata::MonteCarlo { variables, .. } => (
                "Monte Carlo source samples",
                variables
                    .iter()
                    .map(|variable| variable.samples.len())
                    .sum(),
                ResultViewer::Hist,
            ),
            AnalysisResultFamilyMetadata::Optimization { iterations, .. } => (
                "Optimization iterations",
                iterations.len(),
                ResultViewer::Optimization,
            ),
            AnalysisResultFamilyMetadata::Soa { time } => {
                ("SOA time coordinates", time.len(), ResultViewer::Soa)
            }
            AnalysisResultFamilyMetadata::PeriodicNoise { .. } => {
                ("Periodic noise", 1, ResultViewer::PhaseNoise)
            }
            AnalysisResultFamilyMetadata::SParameter {
                reference_impedances_ohm,
                ..
            } => (
                "Port reference impedances",
                reference_impedances_ohm.len(),
                ResultViewer::Smith,
            ),
        };
        push(
            "family/metadata",
            name.to_owned(),
            ResultArtifactKind::Array,
            count,
            "",
            None,
            viewer,
        );
    }
    for measurement in &analysis.measurements {
        push(
            &format!("measurement/{}", measurement.name),
            format!(".MEAS {}", measurement.name),
            ResultArtifactKind::Scalar,
            1,
            "",
            measurement
                .value
                .map(|value| crate::ui::plot::fmt_si(value, "", 4)),
            ResultViewer::Specs,
        );
    }
    let mut counts = std::collections::HashMap::with_capacity(artifacts.len());
    for artifact in &artifacts {
        *counts.entry(artifact.identity.clone()).or_insert(0_usize) += 1;
    }
    artifacts.retain(|artifact| counts.get(&artifact.identity) == Some(&1));
    artifacts
}

/// A mixing-product index as the reader writes it: `(1, -1)`.
fn harmonic_tuple(indices: &[i32]) -> String {
    let indices: Vec<String> = indices.iter().map(i32::to_string).collect();
    format!("({})", indices.join(", "))
}

/// Which orbit the multipliers were taken about, when the run recorded it.
/// A run that did not is not a fact about the circuit, so it says nothing.
fn floquet_orbit_navigator_label(orbit: crate::state::FloquetOrbitKindEvidence) -> String {
    match orbit {
        crate::state::FloquetOrbitKindEvidence::Driven => " · driven orbit".to_owned(),
        crate::state::FloquetOrbitKindEvidence::Autonomous => " · autonomous orbit".to_owned(),
        crate::state::FloquetOrbitKindEvidence::LegacyUnknown => String::new(),
    }
}

const fn floquet_verdict_navigator_label(
    verdict: crate::state::FloquetStabilityVerdictEvidence,
) -> &'static str {
    match verdict {
        crate::state::FloquetStabilityVerdictEvidence::Stable => "stable",
        crate::state::FloquetStabilityVerdictEvidence::Unstable => "unstable",
        crate::state::FloquetStabilityVerdictEvidence::Marginal => "marginal",
        crate::state::FloquetStabilityVerdictEvidence::Indeterminate => "indeterminate",
    }
}

const fn pstb_classification_navigator_label(
    classification: crate::state::PstbStabilityClassificationEvidence,
) -> &'static str {
    use crate::state::PstbStabilityClassificationEvidence as Classification;
    match classification {
        Classification::Stable => "stable",
        Classification::UnstableReal => "unstable real",
        Classification::UnstableComplex => "unstable complex",
        Classification::PeriodDoubling => "period doubling",
        Classification::NeimarkSacker => "Neimark-Sacker",
        Classification::SaddleNode => "saddle-node",
        Classification::Marginal => "marginal",
        Classification::Indeterminate => "indeterminate",
    }
}

pub(super) fn show_virtualized_result_signals(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    runs: &[ResultRun],
    active_run: Option<usize>,
    active_analysis: Option<usize>,
    selected_trace: Option<&rspice_results_ui::selection::SelectedResultTrace>,
    selected_artifact: Option<&ResultArtifactPresentationKey>,
    ordered_visible: &[ResultBrowserSelectionKey],
    query: &str,
    kind: ResultsBrowserKind,
    scope: ResultsBrowserScope,
) {
    // A row labels the conductor the design drew; its identity stays the deck
    // name the engine answered under.
    let notations = bus_notations(&state.model.workspace, &state.model.schematic);
    let mut rows = Vec::new();
    let mut signal_rows = 0_usize;
    for (run_position, run) in runs.iter().enumerate() {
        let run_expanded = !query.is_empty()
            || result_browser_dataset_expanded(
                ui.ctx(),
                run.dataset_id,
                active_run == Some(run.run_index) || runs.len() == 1,
            );
        rows.push(ResultSignalVirtualRow::Dataset {
            run: run_position,
            expanded: run_expanded,
        });
        if !run_expanded {
            continue;
        }
        if run.analyses.is_empty() {
            rows.push(ResultSignalVirtualRow::DatasetOmission { run: run_position });
            continue;
        }
        for (analysis_position, analysis) in run.analyses.iter().enumerate() {
            let analysis_active = active_run == Some(run.run_index)
                && active_analysis == Some(analysis.analysis_index);
            let expanded = !query.is_empty()
                || result_browser_group_expanded(
                    ui.ctx(),
                    analysis.presentation_key,
                    analysis_active,
                );
            rows.push(ResultSignalVirtualRow::Analysis {
                run: run_position,
                analysis: analysis_position,
                expanded,
            });
            if !expanded {
                continue;
            }
            if analysis.signals.is_empty() && analysis.artifacts.is_empty() {
                rows.push(ResultSignalVirtualRow::Omission {
                    run: run_position,
                    analysis: analysis_position,
                });
            } else {
                signal_rows += analysis.signals.len() + analysis.artifacts.len();
                rows.extend((0..analysis.signals.len()).map(|signal| {
                    ResultSignalVirtualRow::Signal {
                        run: run_position,
                        analysis: analysis_position,
                        signal,
                    }
                }));
                rows.extend((0..analysis.artifacts.len()).map(|artifact| {
                    ResultSignalVirtualRow::Artifact {
                        run: run_position,
                        analysis: analysis_position,
                        artifact,
                    }
                }));
            }
        }
    }

    let row_height = responsive_result_control_height(
        RESULT_QUANTITY_ROW_HEIGHT.max(RESULT_ANALYSIS_HEAD_HEIGHT),
        Tokens::get(ui.ctx()).metrics.ctl_h,
    );
    let list_height = ui
        .available_height()
        .max(row_height)
        .min(row_height * RESULT_BROWSER_RENDER_WINDOW_ROWS as f32);
    ScrollArea::vertical()
        .id_salt("workbench.results.navigator")
        .auto_shrink([false, true])
        .max_height(list_height)
        .show_viewport(ui, |ui, viewport| {
            let spacing = ui.spacing().item_spacing.y;
            let row_stride = row_height + spacing;
            ui.set_height((row_stride * rows.len() as f32 - spacing).max(0.0));
            let mut visible_start = (viewport.min.y / row_stride).floor().max(0.0) as usize;
            let visible_end =
                ((viewport.max.y / row_stride).ceil() as usize + 1).min(rows.len());
            if visible_end == rows.len() {
                let visible_count = visible_end.saturating_sub(visible_start);
                visible_start = rows.len().saturating_sub(visible_count);
            }
            let (render_start, render_end) = if rows.len()
                >= RESULT_BROWSER_VIRTUALIZATION_THRESHOLD
            {
                (
                    visible_start.saturating_sub(RESULT_BROWSER_OVERSCAN_ROWS),
                    visible_end
                        .saturating_add(RESULT_BROWSER_OVERSCAN_ROWS)
                        .min(rows.len()),
                )
            } else {
                (0, rows.len())
            };
            let y_min = ui.max_rect().top() + render_start as f32 * row_stride;
            let y_max = ui.max_rect().top() + render_end as f32 * row_stride;
            let render_rect =
                egui::Rect::from_x_y_ranges(ui.max_rect().x_range(), y_min..=y_max);
            ui.scope_builder(egui::UiBuilder::new().max_rect(render_rect), |ui| {
                ui.skip_ahead_auto_ids(render_start);
                for row in rows
                    .iter()
                    .take(render_end)
                    .skip(render_start)
                    .copied()
                {
                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width(), row_height),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| match row {
                        ResultSignalVirtualRow::Dataset { run, expanded } => {
                            let run = &runs[run];
                            let count = run
                                .analyses
                                .iter()
                                .map(|analysis| {
                                    analysis.signals.len() + analysis.artifacts.len()
                                })
                                .sum::<usize>();
                            let domain = data_state_meta(run.data_state);
                            let head = result_browser_analysis_head(
                                ui,
                                "DS",
                                &run.label,
                                domain,
                                count,
                                count,
                                active_run == Some(run.run_index),
                                run.success,
                                expanded,
                            );
                            if head.clicked() {
                                set_result_browser_dataset_expanded(
                                    ui.ctx(),
                                    run.dataset_id,
                                    !expanded,
                                );
                            }
                            if head.has_focus() {
                                let collapse = head.ctx.input_mut(|input| {
                                    input.consume_key(
                                        egui::Modifiers::NONE,
                                        egui::Key::ArrowLeft,
                                    )
                                });
                                let expand = head.ctx.input_mut(|input| {
                                    input.consume_key(
                                        egui::Modifiers::NONE,
                                        egui::Key::ArrowRight,
                                    )
                                });
                                if (collapse && expanded) || (expand && !expanded) {
                                    set_result_browser_dataset_expanded(
                                        ui.ctx(),
                                        run.dataset_id,
                                        expand,
                                    );
                                }
                            }
                        }
                        ResultSignalVirtualRow::DatasetOmission { run } => {
                            let run = &runs[run];
                            result_browser_virtual_omission(
                                ui,
                                &format!("No matches in {}", run.label),
                                row_height,
                            );
                        }
                        ResultSignalVirtualRow::Analysis {
                            run,
                            analysis,
                            expanded,
                        } => {
                            let run = &runs[run];
                            let analysis = &run.analyses[analysis];
                            let analysis_active = active_run == Some(run.run_index)
                                && active_analysis == Some(analysis.analysis_index);
                            let shown = analysis.signals.len() + analysis.artifacts.len();
                            let head = result_browser_analysis_head(
                                ui,
                                analysis.short_label,
                                &analysis.label,
                                &analysis.domain,
                                shown,
                                analysis.total_signals,
                                analysis_active,
                                analysis.success,
                                expanded,
                            );
                            // The retained fraction is the head's whole
                            // statement about scope; its reason rides the
                            // tooltip rather than a word beside the number.
                            let head = if shown == analysis.total_signals {
                                head
                            } else {
                                head.on_hover_text(format!(
                                    "{shown} listed of {} retained · {}",
                                    analysis.total_signals, analysis.data_state.note
                                ))
                            };
                            if head.clicked() {
                                if query.is_empty() {
                                    set_result_browser_group_expanded(
                                        ui.ctx(),
                                        analysis.presentation_key,
                                        !expanded,
                                    );
                                }
                                select_result_analysis_by_key(state, analysis.presentation_key);
                            }
                            if head.has_focus() {
                                let collapse = head.ctx.input_mut(|input| {
                                    input.consume_key(
                                        egui::Modifiers::NONE,
                                        egui::Key::ArrowLeft,
                                    )
                                });
                                let expand = head.ctx.input_mut(|input| {
                                    input.consume_key(
                                        egui::Modifiers::NONE,
                                        egui::Key::ArrowRight,
                                    )
                                });
                                if (collapse && expanded) || (expand && !expanded) {
                                    set_result_browser_group_expanded(
                                        ui.ctx(),
                                        analysis.presentation_key,
                                        expand,
                                    );
                                }
                            }
                        }
                        ResultSignalVirtualRow::Omission { run, analysis } => {
                            let analysis = &runs[run].analyses[analysis];
                            let reason = if analysis.total_signals == 0
                                && analysis.domain == "scalar solution"
                            {
                                "Values are in the OP sheet"
                            } else if analysis.total_signals == 0 {
                                "No waveform quantities were retained for this swept analysis. Place a probe or change Output selection, then run again."
                            } else {
                                "Hidden by the filter"
                            };
                            result_browser_virtual_omission(ui, reason, row_height);
                        }
                        ResultSignalVirtualRow::Signal {
                            run,
                            analysis,
                            signal,
                        } => {
                            let analysis = &runs[run].analyses[analysis];
                            let signal = &analysis.signals[signal];
                            let t = Tokens::get(ui.ctx());
                            let color = rspice_results_ui::presentation::trace_color(
                                &signal.color,
                                t.color.traces[signal.waveform_index % t.color.traces.len()],
                            );
                            let signal_selected = selected_trace.is_some_and(|selected| {
                                selected.analysis_key() == analysis.presentation_key
                                    && selected.source_name() == signal.name
                            });
                            let responses = result_quantity_row(
                                ui,
                                result_signal_row_id(&ResultBrowserSelectionKey::Waveform(
                                    signal.identity.clone(),
                                )),
                                &notations.display(&signal.name),
                                &signal.meta,
                                signal.value.as_deref(),
                                color,
                                signal_selected,
                                signal.visible,
                                state.results.session.is_favorite_signal(&signal.identity),
                                state.results.session.is_checked_signal(&signal.identity),
                            );
                            if responses.check.clicked() {
                                let modifiers = ui.input(|input| input.modifiers);
                                update_result_browser_multi_selection(state,
                                    &ResultBrowserSelectionKey::Waveform(signal.identity.clone()),
                                    ordered_visible,
                                    modifiers,
                                );
                            }
                            if responses.visibility.clicked() {
                                toggle_result_signal_visibility(state, &signal.identity);
                            }
                            if responses
                                .favorite
                                .as_ref()
                                .is_some_and(|star| star.clicked())
                            {
                                state.results
                                    .session.toggle_favorite_signal(signal.identity.clone());
                            }
                            if responses.selection.clicked() {
                                let modifiers = ui.input(|input| input.modifiers);
                                if modifiers.shift || modifiers.command || modifiers.ctrl {
                                    update_result_browser_multi_selection(state,
                                        &ResultBrowserSelectionKey::Waveform(
                                            signal.identity.clone(),
                                        ),
                                        ordered_visible,
                                        modifiers,
                                    );
                                } else {
                                    state.results
                                        .session.set_browser_range_anchor(
                                            ResultBrowserSelectionKey::Waveform(
                                                signal.identity.clone(),
                                            ),
                                        );
                                }
                                select_result_signal_by_key(state, &signal.identity);
                            }
                            handle_result_signal_keyboard(
                                &responses.selection,
                                state,
                                &signal.identity,
                                ordered_visible,
                            );
                            let favorite = state.results
                                .session.is_favorite_signal(&signal.identity);
                            result_signal_context_menu(
                                &responses.selection,
                                state,
                                &signal.identity,
                                signal.visible,
                                favorite,
                            );
                        }
                        ResultSignalVirtualRow::Artifact {
                            run,
                            analysis,
                            artifact,
                        } => {
                            let analysis = &runs[run].analyses[analysis];
                            let artifact = &analysis.artifacts[artifact];
                            let selection_key =
                                ResultBrowserSelectionKey::Artifact(artifact.identity.clone());
                            let selected = selected_artifact == Some(&artifact.identity);
                            let favorite = state.results
                                .session.is_favorite_result_artifact(&artifact.identity);
                            let responses = result_artifact_row(
                                ui,
                                result_signal_row_id(&selection_key),
                                &artifact.name,
                                &artifact.meta,
                                artifact.value.as_deref(),
                                artifact.kind,
                                selected,
                                favorite,
                                state.results
                                    .session.is_checked_result_artifact(&artifact.identity),
                            );
                            if responses.check.clicked() {
                                update_result_browser_multi_selection(state,
                                    &selection_key,
                                    ordered_visible,
                                    ui.input(|input| input.modifiers),
                                );
                            }
                            if responses
                                .favorite
                                .as_ref()
                                .is_some_and(|star| star.clicked())
                            {
                                state.results
                                    .session.toggle_favorite_result_artifact(artifact.identity.clone());
                            }
                            if responses.selection.clicked() {
                                let modifiers = ui.input(|input| input.modifiers);
                                if modifiers.shift || modifiers.command || modifiers.ctrl {
                                    update_result_browser_multi_selection(state,
                                        &selection_key,
                                        ordered_visible,
                                        modifiers,
                                    );
                                } else {
                                    state.results
                                        .session.set_browser_range_anchor(selection_key.clone());
                                }
                                select_result_artifact(state, &artifact.identity);
                            }
                            handle_result_artifact_keyboard(
                                &responses.selection,
                                state,
                                artifact,
                                ordered_visible,
                            );
                            result_artifact_context_menu(
                                &responses.selection,
                                state,
                                artifact,
                                favorite,
                            );
                        }
                        },
                    );
                }
            });
        });

    if signal_rows == 0 {
        muted(
            ui,
            if state.model.simulation.retained.runs.is_empty() {
                "No results yet. Run a simulation."
            } else {
                match scope {
                    ResultsBrowserScope::Favorites => "Star a signal to build the Favorites scope.",
                    ResultsBrowserScope::Recent => "Signals you select or show collect here.",
                    ResultsBrowserScope::All => {
                        "No signal of the active dataset matches this filter."
                    }
                }
            },
        );
        results_browser_clear_filters(ui, state, kind, scope);
    }
}

fn result_browser_virtual_omission(ui: &mut Ui, text: &str, row_height: f32) {
    let t = Tokens::get(ui.ctx());
    let response = ui.add_sized(
        [ui.available_width(), row_height],
        egui::Label::new(
            egui::RichText::new(text)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        )
        .truncate(),
    );
    response.on_hover_text(text);
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ResultCompletenessClass {
    Complete,
    Partial,
    Loading,
    Failed,
    Cancelled,
}

pub(super) fn result_run_operational_label(run: &crate::state::SimulationRun) -> &'static str {
    use crate::state::SimulationRunLifecycle;

    match run.lifecycle {
        SimulationRunLifecycle::LegacyUnknown => "legacy / unknown",
        SimulationRunLifecycle::Preparing => "loading / preparing",
        SimulationRunLifecycle::Running => "loading",
        SimulationRunLifecycle::Cancelling => "loading / cancelling",
        SimulationRunLifecycle::Completed if run.success => "ready",
        SimulationRunLifecycle::Completed | SimulationRunLifecycle::Failed => "failed",
        SimulationRunLifecycle::Aborted => "cancelled",
        SimulationRunLifecycle::Interrupted => "interrupted",
    }
}

impl ResultCompletenessClass {
    const fn label(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Loading => "loading",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }

    /// Worst first, so a rollup over several analyses can take the minimum.
    const fn severity_rank(self) -> u8 {
        match self {
            Self::Failed => 0,
            Self::Cancelled => 1,
            Self::Loading => 2,
            Self::Partial => 3,
            Self::Complete => 4,
        }
    }
}

pub(super) fn result_analysis_completeness(
    run: &crate::state::SimulationRun,
    analysis: &crate::state::AnalysisResult,
) -> ResultCompletenessClass {
    use crate::state::SimulationRunLifecycle;

    if analysis.is_live_partial() {
        return ResultCompletenessClass::Partial;
    }
    match run.lifecycle {
        SimulationRunLifecycle::Preparing
        | SimulationRunLifecycle::Running
        | SimulationRunLifecycle::Cancelling => ResultCompletenessClass::Loading,
        SimulationRunLifecycle::Completed if analysis.success => ResultCompletenessClass::Complete,
        SimulationRunLifecycle::Aborted => ResultCompletenessClass::Cancelled,
        SimulationRunLifecycle::LegacyUnknown if analysis.success => {
            ResultCompletenessClass::Partial
        }
        SimulationRunLifecycle::Completed
        | SimulationRunLifecycle::Failed
        | SimulationRunLifecycle::Interrupted
        | SimulationRunLifecycle::LegacyUnknown => ResultCompletenessClass::Failed,
    }
}

/// The binding's completeness: the worst reading among the analyses it retains,
/// so a dataset row never reads complete over a failed or still-loading one.
pub(super) fn result_run_completeness(
    run: &crate::state::SimulationRun,
) -> ResultCompletenessClass {
    run.analyses
        .iter()
        .map(|analysis| result_analysis_completeness(run, analysis))
        .min_by_key(|class| class.severity_rank())
        .unwrap_or(ResultCompletenessClass::Complete)
}

pub(super) fn result_analysis_browser_status(
    run: &crate::state::SimulationRun,
    analysis: &crate::state::AnalysisResult,
    evidence_valid: bool,
) -> (&'static str, &'static str) {
    let integrity = if evidence_valid {
        "integrity verified"
    } else {
        "corrupted"
    };
    (
        integrity,
        result_analysis_completeness(run, analysis).label(),
    )
}

/// The caption under an analysis head: what it swept, over what interval.
///
/// The interval comes from the retained abscissa rather than the requested
/// setup, so a run that stopped early states the range it actually solved.
pub(super) fn result_analysis_domain(
    analysis_type: crate::state::AnalysisType,
    first_waveform: Option<&crate::state::RetainedTrace>,
) -> String {
    let (sweep, unit, ..) = analysis_type.axis_info();
    if sweep.is_empty() {
        return "scalar solution".to_owned();
    }
    let sweep = sweep.to_lowercase();
    let span = first_waveform.and_then(|waveform| {
        let start = *waveform.x.first()?;
        let end = *waveform.x.last()?;
        (start.is_finite() && end.is_finite()).then(|| {
            format!(
                "{} … {}",
                sweep_endpoint(start, unit),
                sweep_endpoint(end, unit)
            )
        })
    });
    match span {
        Some(span) => format!("{sweep} · {span}"),
        None if unit.is_empty() => sweep,
        None => format!("{sweep} · {unit}"),
    }
}

/// One end of a swept interval. Zero stays bare: "0.00 s" spends three
/// characters restating that the sweep starts where sweeps start.
fn sweep_endpoint(value: f64, unit: &str) -> String {
    if value == 0.0 {
        "0".to_owned()
    } else {
        crate::ui::plot::fmt_si_significant(value, unit, 3)
    }
}

/// Human name for a retained quantity's kind, read from the same accessor
/// authority the browser's units come from. The metadata line states it once
/// per row so no row needs a repeated kind chip.
pub(super) fn result_quantity_kind_label(name: &str, unit: &str) -> &'static str {
    if crate::workbench::documents::result_document::browser_signal_is_current(name) {
        return if unit == "°" {
            "Current phase"
        } else {
            "Current"
        };
    }
    match unit {
        "V" => "Voltage",
        "°" => "Phase",
        "W" => "Power",
        "dB" => "Magnitude",
        "A" => "Current",
        "V^2/Hz" => "Noise PSD",
        _ => "Quantity",
    }
}

/// The mockup's `.result-browser-manifest-row`: one retained analysis of a
/// dataset, as its kind glyph, code, and the domain it solved. It is an
/// inventory line rather than a tree row — the dataset above it is the
/// subject, and these state what that dataset contains.
pub(super) fn result_browser_manifest_row(
    ui: &mut Ui,
    glyph: &str,
    code: &str,
    domain: &str,
    selected: bool,
    ok: bool,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let height = responsive_result_control_height(RESULT_MANIFEST_ROW_HEIGHT, t.metrics.ctl_h);
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            format!("{code}, {domain}"),
        )
    });
    if !ui.is_rect_visible(rect) {
        return response;
    }
    if selected {
        ui.painter().rect_filled(rect, 0.0, c.accent_dim);
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, c.bg_hover);
    }
    let glyph_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 28.0 + 8.0, rect.center().y),
        egui::vec2(16.0, 16.0),
    );
    let glyph_color = if ok { c.accent } else { c.warn };
    ui.painter().rect_filled(glyph_rect, 3.0, c.accent_dim);
    ui.painter().text(
        glyph_rect.center(),
        egui::Align2::CENTER_CENTER,
        glyph,
        theme::mono(tokens::FS_MICRO, FontWeight::SemiBold),
        glyph_color,
    );
    let code_end = ui
        .painter()
        .text(
            egui::pos2(glyph_rect.right() + 6.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            code,
            theme::mono(tokens::FS_0, FontWeight::Medium),
            if selected { c.text } else { c.text_dim },
        )
        .right();
    // The domain caption keeps under 42% of the row, as the mockup bounds it.
    let domain_left = code_end + 6.0;
    let available = (rect.right() - 8.0 - domain_left).min(rect.width() * 0.42);
    if available > 24.0 {
        let galley = ui.fonts_mut(|fonts| {
            fonts.layout(
                domain.to_owned(),
                theme::sans(tokens::FS_MICRO, FontWeight::Medium),
                c.text_faint,
                available,
            )
        });
        if galley.rows.len() == 1 {
            ui.painter().galley(
                egui::pos2(
                    rect.right() - 8.0 - galley.size().x,
                    rect.center().y - galley.size().y / 2.0,
                ),
                galley,
                c.text_faint,
            );
        }
    }
    theme::paint_focus_ring(ui, &response, rect);
    response
}

/// The mockup's `.result-browser-analysis-head`: a 31 px group head whose
/// kind glyph, name, domain caption, and quantity count identify the retained
/// analysis a quantity belongs to, so no row has to repeat any of it.
fn result_browser_analysis_head(
    ui: &mut Ui,
    glyph: &str,
    label: &str,
    domain: &str,
    count: usize,
    retained: usize,
    selected: bool,
    ok: bool,
    expanded: bool,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let height = responsive_result_control_height(RESULT_ANALYSIS_HEAD_HEIGHT, t.metrics.ctl_h);
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            format!(
                "{label}, {domain}, {count} quantities, {}",
                if expanded { "expanded" } else { "collapsed" }
            ),
        )
    });
    if !ui.is_rect_visible(rect) {
        return response;
    }
    ui.painter().rect_filled(
        rect,
        0.0,
        if response.hovered() {
            c.bg_hover
        } else {
            c.bg_panel
        },
    );
    // `box-shadow: 0 1px var(--border)` — the head keeps its own hairline so
    // it reads as pinned chrome above the quantities it owns.
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border),
    );

    // The disclosure caret: a group with many quantities is closed by
    // default, so the panel opens as an index of the run's analyses. It is a
    // stroked chevron rather than a text glyph — the bundled faces carry no
    // U+2304, so an expanded head used to paint a missing-glyph box.
    if expanded {
        WorkbenchIcon::ChevronDown
    } else {
        WorkbenchIcon::ChevronRight
    }
    .paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(rect.left() + 5.0 + 7.0, rect.center().y),
            egui::vec2(12.0, 12.0),
        ),
        c.text_faint,
    );

    // 18x18 kind glyph. It says which kind of thing the group is, which is
    // neither selection nor status, so it is neutral: it used to be painted
    // in accent on every head, and in warn ink when the run had failed.
    let glyph_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 5.0 + 14.0 + 9.0, rect.center().y),
        egui::vec2(18.0, 18.0),
    );
    ui.painter().rect_filled(glyph_rect, 4.0, c.bg_inset);
    ui.painter().rect_stroke(
        glyph_rect,
        4.0,
        egui::Stroke::new(1.0, c.border_strong),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        glyph_rect.center(),
        egui::Align2::CENTER_CENTER,
        glyph,
        theme::mono(tokens::FS_MICRO, FontWeight::SemiBold),
        c.text_dim,
    );

    // `shown / retained` while a filter hides part of the group, so the head
    // never implies the analysis solved fewer quantities than it did.
    let count_text = if count == retained {
        count.to_string()
    } else {
        format!("{count} / {retained}")
    };
    let count_width = ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap(
                count_text.clone(),
                theme::mono(tokens::FS_MICRO, FontWeight::Regular),
                c.text_faint,
            )
            .size()
            .x
    });
    ui.painter().text(
        egui::pos2(rect.right() - 8.0, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        &count_text,
        theme::mono(tokens::FS_MICRO, FontWeight::Regular),
        c.text_faint,
    );

    // Label and domain share one baseline; the domain elides first because the
    // label is the identity the head exists to state. A run that did not
    // finish wears the mark that owns that colour, in front of its name.
    let mut text_left = glyph_rect.right() + 5.0;
    if !ok {
        use crate::ui::widgets::{PROBLEM_MARK_ADVANCE, PROBLEM_MARK_SIDE};
        crate::ui::widgets::paint_problem_mark(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(text_left + PROBLEM_MARK_SIDE * 0.5, rect.center().y),
                egui::Vec2::splat(PROBLEM_MARK_SIDE),
            ),
            crate::ui::widgets::Problem::Failure,
        );
        text_left += PROBLEM_MARK_ADVANCE;
    }
    let text_right = rect.right() - 8.0 - count_width - 8.0;
    let label_end = ui
        .painter()
        .text(
            egui::pos2(text_left, rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            theme::mono(tokens::FS_0, FontWeight::SemiBold),
            if selected { c.accent } else { c.text },
        )
        .right();
    let domain_left = label_end + 6.0;
    if domain_left < text_right {
        let available = text_right - domain_left;
        let galley = ui.fonts_mut(|fonts| {
            fonts.layout(
                domain.to_owned(),
                theme::sans(tokens::FS_MICRO, FontWeight::Medium),
                c.text_faint,
                available,
            )
        });
        if galley.rows.len() == 1 {
            ui.painter().galley(
                egui::pos2(domain_left, rect.center().y - galley.size().y / 2.0),
                galley,
                c.text_faint,
            );
        }
    }
    theme::paint_focus_ring(ui, &response, rect);
    response
}

pub(super) fn expression_header(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let t = Tokens::get(ui.ctx());
    let header_height = responsive_result_control_height(EXPRESSION_HEADER_HEIGHT, t.metrics.ctl_h);
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), header_height),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    // No caption: the tab above it says Expressions. The band is here for
    // the calculator at its trailing edge.
    let add_size = responsive_result_control_height(EXPRESSION_HEADER_HEIGHT, t.metrics.ctl_h);
    let add_center_x = if add_size >= TOUCH_TARGET_HEIGHT {
        rect.right() - 10.0 - add_size * 0.5
    } else {
        rect.right() - 16.0
    };
    let add_rect = egui::Rect::from_center_size(
        egui::pos2(add_center_x, rect.center().y),
        egui::vec2(add_size, add_size),
    );
    let response = ui.interact(
        add_rect,
        ui.id().with("add-result-expression"),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            "Open waveform calculator",
        )
    });
    if response.hovered() {
        ui.painter()
            .rect_filled(add_rect, t.radius, t.color.bg_hover);
    }
    WorkbenchIcon::Add.paint(
        ui.painter(),
        egui::Rect::from_center_size(add_rect.center(), egui::vec2(14.0, 14.0)),
        t.color.text_dim,
    );
    theme::paint_focus_ring(ui, &response, add_rect);
    if response.on_hover_text("Open calculator").clicked() {
        execute(state, ResultsCommand::WaveformCalculator.key());
    }
}

pub(super) struct SignalRowResponses {
    pub(super) selection: egui::Response,
    /// Plot membership. On a quantity row this is the hover-revealed ±
    /// action; expression rows keep it on their swatch.
    pub(super) visibility: egui::Response,
    /// The batch-selection checkbox, on quantity rows only.
    pub(super) check: egui::Response,
    /// Hover-revealed star toggle; present only while the row shows it.
    pub(super) favorite: Option<egui::Response>,
}

/// One retained quantity, in the mockup's two-register anatomy: a selection
/// checkbox column, the name with its persistent favorite flag and a
/// right-aligned preview value, and the typed metadata line beneath.
///
/// Each mark states exactly one fact. The checkbox is batch membership; the
/// name's ink is plot membership; the favorite flag is a saved preference.
/// The flag and the favorite *action* are deliberately different marks — the
/// flag states membership on an idle row, while the action cluster appears on
/// hover and takes the value column, so a resting list stays pure data
/// instead of carrying a column of controls down the panel.
fn result_quantity_row(
    ui: &mut Ui,
    id: egui::Id,
    name: &str,
    meta: &str,
    value: Option<&str>,
    color: egui::Color32,
    selected: bool,
    visible: bool,
    favorite: bool,
    checked: bool,
) -> SignalRowResponses {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let row_height = responsive_result_control_height(RESULT_QUANTITY_ROW_HEIGHT, t.metrics.ctl_h);
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_height),
        egui::Sense::hover(),
    );
    // On a coarse pointer there is no hover to reveal anything with, so the
    // row's actions are simply present. Hiding them behind a state a finger
    // cannot produce would put plot membership and favouriting out of reach
    // on the tablet composition entirely.
    let hovered = ui.rect_contains_pointer(rect) || t.metrics.is_touch();
    let visibility_rect = egui::Rect::from_min_max(
        rect.left_top(),
        egui::pos2((rect.left() + 26.0).min(rect.right()), rect.bottom()),
    );
    let selection_rect = egui::Rect::from_min_max(
        egui::pos2(visibility_rect.right(), rect.top()),
        rect.right_bottom(),
    );
    // The checkbox marks the row for a batch action; plotting is the row's
    // own affordance in the action cluster, as the mockup separates them.
    let check = ui.interact(visibility_rect, id.with("check"), egui::Sense::click());
    check.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Checkbox,
            ui.is_enabled(),
            checked,
            format!("Select {name}"),
        )
    });
    let selection = ui.interact(selection_rect, id.with("selection"), egui::Sense::click());
    // A row with nothing in its second register is spoken as its name.
    let spoken = if meta.is_empty() {
        name.to_owned()
    } else {
        format!("{name}, {meta}")
    };
    selection.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            spoken.clone(),
        )
    });
    if !ui.is_rect_visible(rect) {
        return SignalRowResponses {
            selection,
            visibility: check.clone(),
            check,
            favorite: None,
        };
    }

    if selected {
        ui.painter().rect_filled(rect, 0.0, c.accent_dim);
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + 2.0, rect.bottom()),
            ),
            0.0,
            c.accent,
        );
    } else if hovered {
        ui.painter().rect_filled(rect, 0.0, c.bg_hover);
    }

    // The checkbox states one thing: whether this row is in the batch. It
    // used to double as the plot-membership light, tinted in the trace's
    // colour, which left an unselected-but-plotted row looking checked.
    // Plot membership moved to the name below.
    let box_rect = egui::Rect::from_center_size(visibility_rect.center(), egui::vec2(13.0, 13.0));
    if checked {
        ui.painter().rect_stroke(
            box_rect,
            3.0,
            egui::Stroke::new(1.0, c.accent),
            egui::StrokeKind::Inside,
        );
        ui.painter()
            .rect_filled(box_rect.shrink(3.0), 1.0, c.accent);
    } else {
        ui.painter().rect_filled(box_rect, 3.0, c.bg_panel);
        ui.painter().rect_stroke(
            box_rect,
            3.0,
            egui::Stroke::new(1.0, c.border_strong),
            egui::StrokeKind::Inside,
        );
    }

    // Two baselines inside the row: identity above, typed metadata below.
    // A row with nothing to say in the second register keeps the row's
    // height — the list is virtualized against one stride — and centres the
    // name in it rather than leaving a blank line under the words.
    //
    // The name carries the trace's own colour while the signal is plotted,
    // so the list answers "which curve is this" in the place the eye already
    // is — no swatch column, and no second meaning loaded onto the checkbox.
    // A signal that is not on a pane stays neutral.
    let (name_y, meta_y) = if meta.is_empty() {
        (rect.center().y, rect.center().y)
    } else {
        (rect.top() + 11.0, rect.top() + 25.0)
    };
    let text_left = selection_rect.left();
    let name_end = ui
        .painter()
        .text(
            egui::pos2(text_left, name_y),
            egui::Align2::LEFT_CENTER,
            name,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            if visible { color } else { c.text_dim },
        )
        .right();
    if favorite {
        WorkbenchIcon::StarFilled.paint(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(name_end + 5.0 + 5.0, name_y),
                egui::vec2(10.0, 10.0),
            ),
            c.warn.gamma_multiply(0.72),
        );
    }

    // The action cluster and the preview value share the right column: on an
    // idle row the value owns it, on intent the actions take it.
    // The action cluster: plot membership, then the favorite mark.
    let plot_response = hovered.then(|| {
        let plot_rect = egui::Rect::from_center_size(
            egui::pos2(rect.right() - 3.0 - 22.0 - 11.0, rect.center().y),
            egui::vec2(22.0, 24.0),
        );
        let response = ui.interact(plot_rect, id.with("plot"), egui::Sense::click());
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::Button,
                ui.is_enabled(),
                visible,
                format!("{name} plotted"),
            )
        });
        if response.hovered() {
            ui.painter().rect_filled(plot_rect, 3.0, c.bg_active);
        }
        ui.painter().text(
            plot_rect.center(),
            egui::Align2::CENTER_CENTER,
            if visible { "−" } else { "+" },
            theme::mono(tokens::FS_1, FontWeight::Medium),
            if visible { color } else { c.text_faint },
        );
        theme::paint_focus_ring(ui, &response, plot_rect);
        response.on_hover_text(if visible {
            format!("Remove {name} from its pane")
        } else {
            format!("Add {name} to its pane")
        })
    });
    let favorite_response = if hovered {
        let star_rect = egui::Rect::from_center_size(
            egui::pos2(rect.right() - 3.0 - 11.0, rect.center().y),
            egui::vec2(22.0, 24.0),
        );
        let response = ui.interact(star_rect, id.with("favorite"), egui::Sense::click());
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::Button,
                ui.is_enabled(),
                favorite,
                format!("{name} favorite"),
            )
        });
        if response.hovered() {
            ui.painter().rect_filled(star_rect, 3.0, c.bg_active);
        }
        if favorite {
            WorkbenchIcon::StarFilled
        } else {
            WorkbenchIcon::Star
        }
        .paint(
            ui.painter(),
            egui::Rect::from_center_size(star_rect.center(), egui::vec2(13.0, 13.0)),
            if favorite { c.warn } else { c.text_faint },
        );
        theme::paint_focus_ring(ui, &response, star_rect);
        Some(response.on_hover_text(if favorite {
            format!("Remove {name} from favorites")
        } else {
            format!("Add {name} to favorites")
        }))
    } else {
        if let Some(value) = value {
            ui.painter().text(
                egui::pos2(rect.right() - 8.0, name_y),
                egui::Align2::RIGHT_CENTER,
                value,
                theme::mono(tokens::FS_0, FontWeight::Regular),
                if visible { c.text_dim } else { c.text_faint },
            );
        }
        None
    };

    if !meta.is_empty() {
        ui.painter().text(
            egui::pos2(text_left, meta_y),
            egui::Align2::LEFT_CENTER,
            meta,
            theme::sans(tokens::FS_MICRO, FontWeight::Medium),
            c.text_faint,
        );
    }

    theme::paint_focus_ring(ui, &check, visibility_rect);
    theme::paint_focus_ring(ui, &selection, selection_rect);
    let check = check.on_hover_text(if checked {
        format!("Deselect {name}")
    } else {
        format!("Select {name} for a batch action")
    });
    let selection = selection.on_hover_text(format!("Inspect {name}"));
    SignalRowResponses {
        selection,
        visibility: plot_response.unwrap_or_else(|| check.clone()),
        check,
        favorite: favorite_response,
    }
}

struct ArtifactRowResponses {
    selection: egui::Response,
    check: egui::Response,
    favorite: Option<egui::Response>,
}

fn result_artifact_row(
    ui: &mut Ui,
    id: egui::Id,
    name: &str,
    meta: &str,
    value: Option<&str>,
    kind: ResultArtifactKind,
    selected: bool,
    favorite: bool,
    checked: bool,
) -> ArtifactRowResponses {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let row_height = responsive_result_control_height(RESULT_QUANTITY_ROW_HEIGHT, t.metrics.ctl_h);
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_height),
        egui::Sense::hover(),
    );
    let hovered = ui.rect_contains_pointer(rect) || t.metrics.is_touch();
    let check_rect = egui::Rect::from_min_max(
        rect.left_top(),
        egui::pos2((rect.left() + 26.0).min(rect.right()), rect.bottom()),
    );
    let selection_rect = egui::Rect::from_min_max(
        egui::pos2(check_rect.right(), rect.top()),
        rect.right_bottom(),
    );
    let check = ui.interact(check_rect, id.with("check"), egui::Sense::click());
    check.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Checkbox,
            ui.is_enabled(),
            checked,
            format!("Select {name}"),
        )
    });
    let selection = ui.interact(selection_rect, id.with("selection"), egui::Sense::click());
    selection.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            format!("{name}, {meta}"),
        )
    });
    if !ui.is_rect_visible(rect) {
        return ArtifactRowResponses {
            selection,
            check,
            favorite: None,
        };
    }
    if selected {
        ui.painter().rect_filled(rect, 0.0, c.accent_dim);
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + 2.0, rect.bottom()),
            ),
            0.0,
            c.accent,
        );
    } else if hovered {
        ui.painter().rect_filled(rect, 0.0, c.bg_hover);
    }
    let box_rect = egui::Rect::from_center_size(check_rect.center(), egui::vec2(13.0, 13.0));
    ui.painter().rect_filled(box_rect, 3.0, c.bg_panel);
    ui.painter().rect_stroke(
        box_rect,
        3.0,
        egui::Stroke::new(1.0, if checked { c.accent } else { c.border_strong }),
        egui::StrokeKind::Inside,
    );
    if checked {
        ui.painter()
            .rect_filled(box_rect.shrink(3.0), 1.0, c.accent);
    }
    let (name_y, meta_y) = if meta.is_empty() {
        (rect.center().y, rect.center().y)
    } else {
        (rect.top() + 11.0, rect.top() + 25.0)
    };
    let glyph = match kind {
        ResultArtifactKind::Scalar => "#",
        ResultArtifactKind::Array => "[]",
        ResultArtifactKind::EventStream => ">",
        ResultArtifactKind::Contribution => "S",
    };
    ui.painter().text(
        egui::pos2(selection_rect.left(), name_y),
        egui::Align2::LEFT_CENTER,
        glyph,
        theme::mono(tokens::FS_MICRO, FontWeight::SemiBold),
        c.accent,
    );
    let name_left = selection_rect.left() + 18.0;
    let name_end = ui
        .painter()
        .text(
            egui::pos2(name_left, name_y),
            egui::Align2::LEFT_CENTER,
            name,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text,
        )
        .right();
    if favorite {
        ui.painter().text(
            egui::pos2(name_end + 5.0, name_y),
            egui::Align2::LEFT_CENTER,
            "*",
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.warn.gamma_multiply(0.72),
        );
    }
    let favorite_response = hovered.then(|| {
        let star_rect = egui::Rect::from_center_size(
            egui::pos2(rect.right() - 14.0, rect.center().y),
            egui::vec2(26.0, 26.0),
        );
        let response = ui.interact(star_rect, id.with("favorite"), egui::Sense::click());
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::Button,
                ui.is_enabled(),
                favorite,
                format!("{name} favorite"),
            )
        });
        ui.painter().text(
            star_rect.center(),
            egui::Align2::CENTER_CENTER,
            if favorite { "*" } else { "+" },
            theme::mono(tokens::FS_1, FontWeight::Regular),
            if favorite { c.warn } else { c.text_faint },
        );
        theme::paint_focus_ring(ui, &response, star_rect);
        response
    });
    if !hovered && let Some(value) = value {
        ui.painter().text(
            egui::pos2(rect.right() - 8.0, name_y),
            egui::Align2::RIGHT_CENTER,
            value,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text_dim,
        );
    }
    if !meta.is_empty() {
        ui.painter().text(
            egui::pos2(name_left, meta_y),
            egui::Align2::LEFT_CENTER,
            meta,
            theme::sans(tokens::FS_MICRO, FontWeight::Medium),
            c.text_faint,
        );
    }
    theme::paint_focus_ring(ui, &check, check_rect);
    theme::paint_focus_ring(ui, &selection, selection_rect);
    ArtifactRowResponses {
        selection: selection.on_hover_text(format!("Inspect {name}")),
        check: check.on_hover_text(if checked {
            format!("Deselect {name}")
        } else {
            format!("Select {name} for a batch action")
        }),
        favorite: favorite_response,
    }
}

pub(super) fn signal_row(
    ui: &mut Ui,
    id: egui::Id,
    name: &str,
    value: Option<&str>,
    color: egui::Color32,
    selected: bool,
    visible: bool,
) -> SignalRowResponses {
    let t = Tokens::get(ui.ctx());
    let row_height = responsive_result_control_height(SIGNAL_ROW_HEIGHT, t.metrics.ctl_h);
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_height),
        egui::Sense::hover(),
    );
    let visibility_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 19.0, rect.center().y),
        egui::vec2(26.0, row_height),
    )
    .intersect(rect);
    let selection_rect = egui::Rect::from_min_max(
        egui::pos2(visibility_rect.right(), rect.top()),
        rect.right_bottom(),
    );
    let visibility = ui.interact(visibility_rect, id.with("visibility"), egui::Sense::click());
    visibility.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Button,
            ui.is_enabled(),
            visible,
            format!("{name} trace visibility"),
        )
    });
    let selection = ui.interact(selection_rect, id.with("selection"), egui::Sense::click());
    selection.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            name,
        )
    });
    if selected || selection.hovered() || visibility.hovered() {
        ui.painter().rect_filled(
            rect,
            0.0,
            if selected {
                t.color.accent_dim
            } else {
                t.color.bg_hover
            },
        );
    }
    if selected {
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + 2.0, rect.bottom()),
            ),
            0.0,
            t.color.accent,
        );
    }
    let swatch = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 19.0, rect.center().y),
        egui::vec2(12.0, 3.0),
    );
    ui.painter().rect_filled(
        swatch,
        1.0,
        if visible { color } else { t.color.text_faint },
    );
    ui.painter().text(
        egui::pos2(rect.left() + 31.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        name,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        if visible {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    if let Some(value) = value {
        ui.painter().text(
            egui::pos2(rect.right() - 8.0, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            value,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
    theme::paint_focus_ring(ui, &visibility, visibility_rect);
    theme::paint_focus_ring(ui, &selection, selection_rect);
    let visibility = visibility.on_hover_text(if visible {
        format!("Hide {name}")
    } else {
        format!("Show {name}")
    });
    let selection = selection.on_hover_text(format!("Select {name}"));
    SignalRowResponses {
        // Expression rows carry no batch selection: the swatch is their only
        // control, so the check response mirrors it rather than inventing one.
        check: visibility.clone(),
        selection,
        visibility,
        favorite: None,
    }
}
