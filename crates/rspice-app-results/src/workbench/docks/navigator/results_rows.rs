//! The data browser's rows: the artifacts a retained analysis yields, the
//! virtualized quantity list, and the painting of each kind of row.

use egui::{Response, ScrollArea, Ui};

use crate::state::bus_notations;
use crate::ui::accessibility::{counted, plural_suffix};
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
    result_browser_group_expanded, result_browser_instance_expanded, results_browser_clear_filters,
    set_result_browser_dataset_expanded, set_result_browser_group_expanded,
    set_result_browser_instance_expanded,
};
use super::results_actions::{
    execute, handle_result_artifact_keyboard, handle_result_signal_keyboard,
    plot_and_inspect_signal, result_artifact_context_menu, result_signal_context_menu,
    result_signal_row_id, select_result_analysis_by_key, select_result_artifact,
    select_result_signal_by_key, set_result_quantity_ticked, update_result_browser_multi_selection,
};
use super::results_tree::{InstanceNode, SignalKind, TreeItem, split_hierarchy};
use rspice_app_workbench::workbench::docks::navigator::grouped_count;
use rspice_app_workbench::workbench::docks::navigator::muted;

use crate::workbench::documents::result_document::ResultQuantityDrag;
use crate::workbench::documents::result_document::browser_plot::{
    BUS_ROW_PREFIX, EVENT_NET_ROW_PREFIX, drag_name, last_strip_models, plotted_signal_color,
};
pub const EXPRESSION_HEADER_HEIGHT: f32 = 24.0;
pub const SIGNAL_ROW_HEIGHT: f32 = 22.0;
/// The browser's row at compact density: one register, so a dock shows half
/// again as many signals as the 36 px two-register row did. Every row of the
/// list, heads included, shares it, because the list is virtualized against
/// one stride.
pub const RESULT_QUANTITY_ROW_HEIGHT: f32 = 22.0;
/// The same row at relaxed density.
const RESULT_QUANTITY_ROW_HEIGHT_RELAXED: f32 = 24.0;
const RESULT_BROWSER_VIRTUALIZATION_THRESHOLD: usize = 250;
const RESULT_BROWSER_RENDER_WINDOW_ROWS: usize = 120;
const RESULT_BROWSER_OVERSCAN_ROWS: usize = 20;
pub const TOUCH_TARGET_HEIGHT: f32 = crate::ui::tokens::TOUCH_TARGET;

/// The height of every row the data browser lists: 22 px at compact density,
/// 24 px at relaxed, and the theme's touch target on a coarse pointer. The
/// interface scale zooms text and row together, so a row that holds its text
/// at one scale holds it at all of them.
pub(super) fn result_browser_row_height(t: &Tokens) -> f32 {
    let desktop = match t.density {
        crate::ui::tokens::Density::Compact => RESULT_QUANTITY_ROW_HEIGHT,
        crate::ui::tokens::Density::Relaxed => RESULT_QUANTITY_ROW_HEIGHT_RELAXED,
    };
    responsive_result_control_height(desktop, t.metrics.ctl_h)
}

/// A result control's height: its own desktop height, or in a touch
/// composition the theme's touch target, which Accessibility may raise.
pub fn responsive_result_control_height(desktop_height: f32, control_height: f32) -> f32 {
    if control_height >= TOUCH_TARGET_HEIGHT {
        control_height
    } else {
        desktop_height
    }
}

pub fn retained_result_artifacts(
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
        // A bus row counts its width; its members are the rows beside it.
        let noun = match kind {
            _ if canonical.starts_with(BUS_ROW_PREFIX) => "bit",
            ResultArtifactKind::Scalar => "",
            ResultArtifactKind::Array => "value",
            ResultArtifactKind::EventStream => "event",
            ResultArtifactKind::Contribution => "contribution",
        };
        let shape = if noun.is_empty() {
            String::new()
        } else {
            format!("{} {noun}{}", grouped_count(count), plural_suffix(count))
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
                    let label = format!("{}[{}:{}]", bus.name, bus.msb, bus.lsb);
                    push(
                        &format!("{BUS_ROW_PREFIX}{label}"),
                        format!("Digital bus {label}"),
                        ResultArtifactKind::EventStream,
                        bus.members.len(),
                        "",
                        Some(bus.members.join(" ")),
                        ResultViewer::Events,
                    );
                }
                // A net with no column of its own is plotted from its row
                // here, as a column is from its row among the signals.
                let nets = digital_traces
                    .iter()
                    .map(|trace| ("D", "Digital net", &trace.node_name, trace.points.len()))
                    .chain(
                        real_traces
                            .iter()
                            .map(|trace| ("E", "Real net", &trace.node_name, trace.points.len())),
                    );
                for (prefix, noun, net, points) in nets {
                    let column = format!("{prefix}({net})");
                    if analysis
                        .waveforms
                        .iter()
                        .any(|waveform| waveform.name == column)
                    {
                        continue;
                    }
                    push(
                        &format!("{EVENT_NET_ROW_PREFIX}{column}"),
                        format!("{noun} {net}"),
                        ResultArtifactKind::EventStream,
                        points,
                        "",
                        None,
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

/// Which orbit the multipliers were taken about.
fn floquet_orbit_navigator_label(orbit: crate::state::FloquetOrbitKindEvidence) -> String {
    match orbit {
        crate::state::FloquetOrbitKindEvidence::Driven => " · driven orbit".to_owned(),
        crate::state::FloquetOrbitKindEvidence::Autonomous => " · autonomous orbit".to_owned(),
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

/// An instance row of the signal list: the node, and the analysis it
/// belongs to.
pub(super) struct InstanceRow<'a> {
    pub(super) run: usize,
    pub(super) analysis: usize,
    pub(super) node: &'a InstanceNode,
}

/// The signal list as the reader sees it: one row per dataset, analysis,
/// open instance and listed quantity, in screen order.
pub(super) struct SignalRows<'a> {
    pub(super) rows: Vec<ResultSignalVirtualRow>,
    pub(super) instances: Vec<InstanceRow<'a>>,
    /// Quantities under open heads, which is what "nothing matches" counts.
    pub(super) quantity_rows: usize,
}

impl SignalRows<'_> {
    /// The quantities on screen, top to bottom: what a Shift range and
    /// "Tick visible quantities" act on.
    pub(super) fn visible_keys(&self, runs: &[ResultRun]) -> Vec<ResultBrowserSelectionKey> {
        self.rows
            .iter()
            .filter_map(|row| match *row {
                ResultSignalVirtualRow::Signal {
                    run,
                    analysis,
                    signal,
                    ..
                } => Some(ResultBrowserSelectionKey::Waveform(
                    runs[run].analyses[analysis].signals[signal]
                        .identity
                        .clone(),
                )),
                ResultSignalVirtualRow::Artifact {
                    run,
                    analysis,
                    artifact,
                    ..
                } => Some(ResultBrowserSelectionKey::Artifact(
                    runs[run].analyses[analysis].artifacts[artifact]
                        .identity
                        .clone(),
                )),
                _ => None,
            })
            .collect()
    }
}

/// Lay the filtered runs out as rows. A query opens every head and every
/// instance, since each branch the filter left holds a match.
pub(super) fn signal_rows<'a>(
    ctx: &egui::Context,
    runs: &'a [ResultRun],
    active_run: Option<usize>,
    active_analysis: Option<usize>,
    filtering: bool,
) -> SignalRows<'a> {
    let mut list = SignalRows {
        rows: Vec::new(),
        instances: Vec::new(),
        quantity_rows: 0,
    };
    for (run_position, run) in runs.iter().enumerate() {
        let run_expanded = filtering
            || result_browser_dataset_expanded(
                ctx,
                run.dataset_id,
                active_run == Some(run.run_index) || runs.len() == 1,
            );
        list.rows.push(ResultSignalVirtualRow::Dataset {
            run: run_position,
            expanded: run_expanded,
        });
        if !run_expanded {
            continue;
        }
        if run.analyses.is_empty() {
            list.rows
                .push(ResultSignalVirtualRow::DatasetOmission { run: run_position });
            continue;
        }
        for (analysis_position, analysis) in run.analyses.iter().enumerate() {
            let analysis_active = active_run == Some(run.run_index)
                && active_analysis == Some(analysis.analysis_index);
            let expanded = filtering
                || result_browser_group_expanded(ctx, analysis.presentation_key, analysis_active);
            list.rows.push(ResultSignalVirtualRow::Analysis {
                run: run_position,
                analysis: analysis_position,
                expanded,
            });
            if !expanded {
                continue;
            }
            if analysis.signals.is_empty() && analysis.artifacts.is_empty() {
                list.rows.push(ResultSignalVirtualRow::Omission {
                    run: run_position,
                    analysis: analysis_position,
                });
                continue;
            }
            list.quantity_rows += analysis.signals.len() + analysis.artifacts.len();
            match &analysis.tree {
                Some(tree) => push_instance_rows(
                    ctx,
                    &mut list,
                    (run_position, analysis_position),
                    analysis.presentation_key,
                    tree,
                    0,
                    filtering,
                ),
                None => {
                    list.rows.extend((0..analysis.signals.len()).map(|signal| {
                        ResultSignalVirtualRow::Signal {
                            run: run_position,
                            analysis: analysis_position,
                            signal,
                            depth: 0,
                        }
                    }));
                    list.rows
                        .extend((0..analysis.artifacts.len()).map(|artifact| {
                            ResultSignalVirtualRow::Artifact {
                                run: run_position,
                                analysis: analysis_position,
                                artifact,
                                depth: 0,
                            }
                        }));
                }
            }
        }
    }
    list
}

/// One level of an analysis's tree: its instances, each followed by what it
/// owns while it is open, then the quantities that sit at this level.
fn push_instance_rows<'a>(
    ctx: &egui::Context,
    list: &mut SignalRows<'a>,
    (run, analysis): (usize, usize),
    analysis_key: AnalysisPresentationKey,
    node: &'a InstanceNode,
    depth: usize,
    filtering: bool,
) {
    for child in &node.children {
        let expanded = filtering || result_browser_instance_expanded(ctx, analysis_key, &child.key);
        list.instances.push(InstanceRow {
            run,
            analysis,
            node: child,
        });
        list.rows.push(ResultSignalVirtualRow::Instance {
            instance: list.instances.len() - 1,
            depth,
            expanded,
        });
        if expanded {
            push_instance_rows(
                ctx,
                list,
                (run, analysis),
                analysis_key,
                child,
                depth + 1,
                filtering,
            );
        }
    }
    for item in &node.items {
        list.rows.push(match *item {
            TreeItem::Signal(signal) => ResultSignalVirtualRow::Signal {
                run,
                analysis,
                signal,
                depth,
            },
            TreeItem::Artifact(artifact) => ResultSignalVirtualRow::Artifact {
                run,
                analysis,
                artifact,
                depth,
            },
        });
    }
}

pub(super) fn show_virtualized_result_signals(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    runs: &[ResultRun],
    list: &SignalRows<'_>,
    active_run: Option<usize>,
    active_analysis: Option<usize>,
    selected_trace: Option<&rspice_results_ui::selection::SelectedResultTrace>,
    selected_artifact: Option<&ResultArtifactPresentationKey>,
    ordered_visible: &[ResultBrowserSelectionKey],
    ticked: &[ResultBrowserSelectionKey],
    filtering: bool,
    kind: ResultsBrowserKind,
    scope: ResultsBrowserScope,
) {
    // A row labels the conductor the design drew; its identity stays the deck
    // name the engine answered under.
    let notations = bus_notations(&state.model.workspace, &state.model.schematic);
    let rows = &list.rows;
    let signal_rows = list.quantity_rows;

    let row_height = result_browser_row_height(&Tokens::get(ui.ctx()));
    let list_height = ui
        .available_height()
        .max(row_height)
        .min(row_height * RESULT_BROWSER_RENDER_WINDOW_ROWS as f32);
    ScrollArea::vertical()
        .id_salt("workbench.results.navigator")
        .auto_shrink([false, true])
        .max_height(list_height)
        .show_viewport(ui, |ui, viewport| {
            // Rows abut, as a tree's do: the row is the stride.
            ui.spacing_mut().item_spacing.y = 0.0;
            let row_stride = row_height;
            ui.set_height(row_stride * rows.len() as f32);
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
                                if !filtering {
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
                        ResultSignalVirtualRow::Instance {
                            instance,
                            depth,
                            expanded,
                        } => {
                            let row = &list.instances[instance];
                            instance_row(ui, state, runs, row, depth, expanded);
                        }
                        ResultSignalVirtualRow::Signal {
                            run,
                            analysis,
                            signal,
                            depth,
                        } => {
                            let in_tree = runs[run].analyses[analysis].tree.is_some();
                            let analysis = &runs[run].analyses[analysis];
                            let signal = &analysis.signals[signal];
                            let shown = notations.display(&signal.name);
                            // Under an instance a row names the signal inside
                            // it; the instance rows above it carry the path.
                            let label = if in_tree {
                                split_hierarchy(&shown).leaf
                            } else {
                                shown.into_owned()
                            };
                            let color = plotted_signal_color(
                                last_strip_models(state),
                                &Tokens::get(ui.ctx()).color.traces,
                                analysis.presentation_key,
                                signal.waveform_index,
                                &signal.color,
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
                                RowLabel {
                                    name: &label,
                                    full_name: &signal.name,
                                    kind: signal.kind,
                                    indent: depth,
                                },
                                &signal.meta,
                                signal.value.as_deref(),
                                color,
                                signal_selected,
                                signal.visible,
                            );
                            if responses.selection.drag_started() {
                                start_browser_drag(
                                    ui.ctx(),
                                    ResultBrowserSelectionKey::Waveform(signal.identity.clone()),
                                    ticked,
                                );
                            }
                            if responses.check.clicked() {
                                let modifiers = ui.input(|input| input.modifiers);
                                update_result_browser_multi_selection(state,
                                    &ResultBrowserSelectionKey::Waveform(signal.identity.clone()),
                                    ordered_visible,
                                    modifiers,
                                );
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
                            if responses.selection.double_clicked() {
                                plot_and_inspect_signal(state, &signal.identity);
                            }
                            handle_result_signal_keyboard(
                                &responses.selection,
                                state,
                                &signal.identity,
                                ordered_visible,
                            );
                            result_signal_context_menu(
                                &responses.selection,
                                state,
                                &signal.identity,
                                signal.visible,
                            );
                        }
                        ResultSignalVirtualRow::Artifact {
                            run,
                            analysis,
                            artifact,
                            depth,
                        } => {
                            let in_tree = runs[run].analyses[analysis].tree.is_some();
                            let analysis = &runs[run].analyses[analysis];
                            let artifact = &analysis.artifacts[artifact];
                            let label = if in_tree && depth > 0 {
                                split_hierarchy(artifact_hierarchical_name(artifact)).leaf
                            } else {
                                artifact.name.clone()
                            };
                            let selection_key =
                                ResultBrowserSelectionKey::Artifact(artifact.identity.clone());
                            let selected = selected_artifact == Some(&artifact.identity);
                            let responses = result_artifact_row(
                                ui,
                                result_signal_row_id(&selection_key),
                                RowLabel {
                                    name: &label,
                                    full_name: &artifact.name,
                                    kind: artifact_signal_kind(artifact)
                                        .unwrap_or(SignalKind::Other(artifact.kind.glyph())),
                                    indent: depth,
                                },
                                &artifact.meta,
                                artifact.value.as_deref(),
                                selected,
                                state.results
                                    .session.is_checked_result_artifact(&artifact.identity),
                            );
                            if responses.selection.drag_started()
                                && is_plottable_artifact(artifact.identity.canonical_name())
                            {
                                start_browser_drag(ui.ctx(), selection_key.clone(), ticked);
                            }
                            if responses.check.clicked() {
                                update_result_browser_multi_selection(state,
                                    &selection_key,
                                    ordered_visible,
                                    ui.input(|input| input.modifiers),
                                );
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
                            result_artifact_context_menu(&responses.selection, state, artifact);
                        }
                        },
                    );
                }
            });
        });

    paint_browser_drag(ui.ctx());
    if signal_rows == 0 {
        muted(
            ui,
            if state.model.simulation.retained.runs.is_empty() {
                "No results yet. Run a simulation."
            } else {
                match scope {
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

/// The hierarchical name an artifact row is placed in the tree by and
/// answers a pattern with: an event net's `D(net)` or `E(net)`, a bus's
/// `name[msb:lsb]`, and any other artifact its own name.
pub(super) fn artifact_hierarchical_name(artifact: &ResultArtifact) -> &str {
    let canonical = artifact.identity.canonical_name();
    canonical
        .strip_prefix(EVENT_NET_ROW_PREFIX)
        .or_else(|| canonical.strip_prefix(BUS_ROW_PREFIX))
        .unwrap_or(&artifact.name)
}

/// Whether an artifact row is one the plot can take: an event net's or a
/// bus's, which the strip draws as a lane.
pub(super) fn is_plottable_artifact(canonical: &str) -> bool {
    canonical.starts_with(EVENT_NET_ROW_PREFIX) || canonical.starts_with(BUS_ROW_PREFIX)
}

/// The kind of an artifact row that is a net or a bus; `None` for the
/// tables and records the other artifacts are.
pub(super) fn artifact_signal_kind(artifact: &ResultArtifact) -> Option<SignalKind> {
    let canonical = artifact.identity.canonical_name();
    if canonical.starts_with(BUS_ROW_PREFIX) {
        return Some(SignalKind::Bus);
    }
    let net = canonical.strip_prefix(EVENT_NET_ROW_PREFIX)?;
    Some(if net.starts_with("E(") {
        SignalKind::Real
    } else {
        SignalKind::Digital
    })
}

/// The browser keys of every quantity an instance owns, at every depth.
pub(super) fn instance_leaf_keys(
    runs: &[ResultRun],
    row: &InstanceRow<'_>,
) -> Vec<ResultBrowserSelectionKey> {
    let analysis = &runs[row.run].analyses[row.analysis];
    row.node
        .leaves()
        .into_iter()
        .map(|leaf| match leaf {
            TreeItem::Signal(index) => {
                ResultBrowserSelectionKey::Waveform(analysis.signals[index].identity.clone())
            }
            TreeItem::Artifact(index) => {
                ResultBrowserSelectionKey::Artifact(analysis.artifacts[index].identity.clone())
            }
        })
        .collect()
}

/// An instance of the design in the tree: its disclosure caret, its name,
/// and how many quantities it holds. A click or the arrow keys fold it; its
/// menu plots or removes everything under it.
fn instance_row(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    runs: &[ResultRun],
    row: &InstanceRow<'_>,
    depth: usize,
    expanded: bool,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let analysis_key = runs[row.run].analyses[row.analysis].presentation_key;
    let node = row.node;
    let height = result_browser_row_height(&t);
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::click_and_drag(),
    );
    if response.drag_started() {
        egui::DragAndDrop::set_payload(
            ui.ctx(),
            ResultQuantityDrag {
                keys: instance_leaf_keys(runs, row),
            },
        );
    }
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::CollapsingHeader,
            ui.is_enabled(),
            format!(
                "{}, instance, {}, {}",
                node.path,
                counted(node.count, "quantity", "quantities"),
                if expanded { "expanded" } else { "collapsed" }
            ),
        )
    });
    if ui.is_rect_visible(rect) {
        if response.hovered() {
            ui.painter().rect_filled(rect, 0.0, c.bg_hover);
        }
        // The caret sits in the kind column of the rows it folds, so an
        // instance and its own quantities line up one indent apart.
        let caret_left = rect.left() + 26.0 + depth as f32 * TREE_INDENT;
        if expanded {
            WorkbenchIcon::ChevronDown
        } else {
            WorkbenchIcon::ChevronRight
        }
        .paint(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(caret_left + 6.0, rect.center().y),
                egui::vec2(11.0, 11.0),
            ),
            c.text_faint,
        );
        let count = grouped_count(node.count);
        let count_galley = ui.painter().layout_no_wrap(
            count,
            theme::mono(tokens::FS_MICRO, FontWeight::Regular),
            c.text_faint,
        );
        let count_width = count_galley.size().x;
        ui.painter().galley(
            egui::pos2(
                rect.right() - 8.0 - count_width,
                rect.center().y - count_galley.size().y * 0.5,
            ),
            count_galley,
            c.text_faint,
        );
        let name_left = caret_left + KIND_GLYPH_WIDTH;
        let font = theme::mono(tokens::FS_0, FontWeight::Medium);
        let name = super::super::super::design_system::elide_text(
            ui,
            &node.name,
            &font,
            rect.right() - 8.0 - count_width - 8.0 - name_left,
        );
        ui.painter().text(
            egui::pos2(name_left, rect.center().y),
            egui::Align2::LEFT_CENTER,
            name,
            font,
            c.text,
        );
        theme::paint_focus_ring(ui, &response, rect);
    }
    if response.clicked() {
        set_result_browser_instance_expanded(ui.ctx(), analysis_key, &node.key, !expanded);
    }
    if response.has_focus() {
        let (collapse, expand) = response.ctx.input_mut(|input| {
            (
                input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft),
                input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight),
            )
        });
        if (collapse && expanded) || (expand && !expanded) {
            set_result_browser_instance_expanded(ui.ctx(), analysis_key, &node.key, expand);
        }
    }
    let response = response.on_hover_text(format!(
        "{} · {}",
        node.path,
        counted(node.count, "quantity", "quantities")
    ));
    response.context_menu(|ui| {
        let plot = format!("Plot all {}", grouped_count(node.count));
        if ui.button(plot).clicked() {
            for key in instance_leaf_keys(runs, row) {
                set_result_quantity_ticked(state, &key, true);
            }
            ui.close();
        }
        if ui.button("Remove all from the plot").clicked() {
            for key in instance_leaf_keys(runs, row) {
                set_result_quantity_ticked(state, &key, false);
            }
            ui.close();
        }
    });
}

/// Start dragging a row: the whole ticked selection when the row is part of
/// it, the row alone otherwise, as a file manager drags.
fn start_browser_drag(
    ctx: &egui::Context,
    key: ResultBrowserSelectionKey,
    ticked: &[ResultBrowserSelectionKey],
) {
    let keys = if ticked.len() > 1 && ticked.contains(&key) {
        ticked.to_vec()
    } else {
        vec![key]
    };
    egui::DragAndDrop::set_payload(ctx, ResultQuantityDrag { keys });
}

/// While rows are being dragged, their name rides beside the pointer, so
/// the reader sees what a drop will plot before letting go.
fn paint_browser_drag(ctx: &egui::Context) {
    let Some(drag) = egui::DragAndDrop::payload::<ResultQuantityDrag>(ctx) else {
        return;
    };
    let Some(pointer) = ctx.pointer_latest_pos() else {
        return;
    };
    ctx.set_cursor_icon(egui::CursorIcon::Grabbing);
    let t = Tokens::get(ctx);
    let text = match drag.keys.as_slice() {
        [one] => drag_name(one).to_owned(),
        many => format!("{} signals", many.len()),
    };
    let painter = ctx.layer_painter(egui::LayerId::new(
        egui::Order::Tooltip,
        egui::Id::new("workbench.results.browser-drag"),
    ));
    let galley = painter.layout_no_wrap(
        text,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text,
    );
    let rect = egui::Rect::from_min_size(
        pointer + egui::vec2(14.0, 10.0),
        galley.size() + egui::vec2(12.0, 6.0),
    );
    painter.rect_filled(rect, t.radius, t.color.bg_panel);
    painter.rect_stroke(
        rect,
        t.radius,
        egui::Stroke::new(1.0, t.color.border_strong),
        egui::StrokeKind::Inside,
    );
    painter.galley(rect.min + egui::vec2(6.0, 3.0), galley, t.color.text);
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
pub enum ResultCompletenessClass {
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
pub fn result_quantity_kind_label(name: &str, unit: &str) -> &'static str {
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
    let height = result_browser_row_height(&t);
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

/// The mockup's `.result-browser-analysis-head`: a one-row group head whose
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
    let height = result_browser_row_height(&t);
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        // A head with nothing to say about its data states only its name.
        let domain = if domain.is_empty() {
            String::new()
        } else {
            format!("{domain}, ")
        };
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            format!(
                "{label}, {domain}{count} quantities, {}",
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

    // 16x16 kind glyph, inside the dense row's 22 px. It says which kind of
    // thing the group is, which is neither selection nor status, so it is
    // neutral: it used to be painted in accent on every head, and in warn ink
    // when the run had failed.
    let glyph_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 5.0 + 14.0 + 8.0, rect.center().y),
        egui::vec2(16.0, 16.0),
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
    /// Plot membership, on an expression row's swatch. A quantity row's is
    /// its tick.
    pub(super) visibility: egui::Response,
    /// The tick, on quantity rows only.
    pub(super) check: egui::Response,
}

/// One retained quantity, in one dense register: the tick column, the kind
/// mark at the row's depth in the tree, the name, and a right-aligned
/// preview value or, without one, the typed metadata.
///
/// Each mark states exactly one fact. The tick is plot membership, and so is
/// the name's ink; the kind mark is what the quantity is. A resting list is
/// pure data, with no column of controls down the panel.
fn result_quantity_row(
    ui: &mut Ui,
    id: egui::Id,
    label: RowLabel<'_>,
    meta: &str,
    value: Option<&str>,
    color: egui::Color32,
    selected: bool,
    visible: bool,
) -> SignalRowResponses {
    let name = label.full_name;
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let row_height = result_browser_row_height(&t);
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
    // The tick is the row's plot membership; a click anywhere else selects
    // the row for the Inspector and plots nothing.
    let check = ui.interact(visibility_rect, id.with("check"), egui::Sense::click());
    check.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Checkbox,
            ui.is_enabled(),
            visible,
            format!("Plot {name}"),
        )
    });
    let selection = ui.interact(
        selection_rect,
        id.with("selection"),
        egui::Sense::click_and_drag(),
    );
    // A row is spoken as its name and the kind its mark shows, then any
    // metadata it carries.
    let kind_word = label.kind.label();
    let spoken = if meta.is_empty() {
        format!("{name}, {kind_word}")
    } else {
        format!("{name}, {kind_word}, {meta}")
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

    let box_rect = egui::Rect::from_center_size(visibility_rect.center(), egui::vec2(13.0, 13.0));
    if visible {
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

    // One register: the name, and in the right column the value or, failing
    // one, the typed metadata. The name carries the trace's own colour while
    // the signal is plotted, so the list answers "which curve is this" in the
    // place the eye already is, with no swatch column. A signal that is not
    // on a pane stays neutral.
    let name_y = rect.center().y;
    let text_left = paint_kind_glyph(ui, selection_rect.left(), name_y, &label);
    let right_text = value.or((!meta.is_empty()).then_some(meta));
    paint_row_line(
        ui,
        rect,
        text_left,
        label.name,
        if visible { color } else { c.text_dim },
        right_text.map(|text| {
            (
                text,
                if visible && value.is_some() {
                    c.text_dim
                } else {
                    c.text_faint
                },
            )
        }),
    );

    theme::paint_focus_ring(ui, &check, visibility_rect);
    theme::paint_focus_ring(ui, &selection, selection_rect);
    let check = check.on_hover_text(if visible {
        format!("Remove {name} from the plot")
    } else {
        format!("Plot {name}")
    });
    let selection = selection.on_hover_text(format!("Inspect {name}"));
    SignalRowResponses {
        selection,
        visibility: check.clone(),
        check,
    }
}

/// What a dense row names and where it sits: the label it paints (the
/// signal's name inside its instance in the tree, its full name otherwise),
/// the full name it is spoken and explained by, the kind its mark shows, and
/// how many instances down the tree it is.
pub(super) struct RowLabel<'a> {
    pub(super) name: &'a str,
    pub(super) full_name: &'a str,
    pub(super) kind: SignalKind,
    pub(super) indent: usize,
}

/// How far each level of the tree steps in.
const TREE_INDENT: f32 = 12.0;
/// The kind column: wide enough for a two-character mark such as `dB`.
const KIND_GLYPH_WIDTH: f32 = 18.0;

/// Paint a row's kind mark at its indent and return where its name starts.
/// The mark is neutral ink: a kind is neither selection nor status.
fn paint_kind_glyph(ui: &Ui, left: f32, center_y: f32, label: &RowLabel<'_>) -> f32 {
    let t = Tokens::get(ui.ctx());
    let glyph_left = left + label.indent as f32 * TREE_INDENT;
    ui.painter().text(
        egui::pos2(glyph_left + KIND_GLYPH_WIDTH * 0.5 - 2.0, center_y),
        egui::Align2::CENTER_CENTER,
        label.kind.glyph(),
        theme::mono(tokens::FS_MICRO, FontWeight::SemiBold),
        t.color.text_faint,
    );
    glyph_left + KIND_GLYPH_WIDTH
}

/// One line of a dense row: `name` from `left`, elided so it stops short of
/// the right column, which holds `right` flush against the row's end.
fn paint_row_line(
    ui: &Ui,
    rect: egui::Rect,
    left: f32,
    name: &str,
    name_color: egui::Color32,
    right: Option<(&str, egui::Color32)>,
) {
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let mut name_right = rect.right() - 8.0;
    if let Some((text, color)) = right {
        let galley = ui
            .painter()
            .layout_no_wrap(text.to_owned(), font.clone(), color);
        // The value yields to the name past half the row: a long bus member
        // list must not push the identity out of its own row.
        let room = ((name_right - left) * 0.5).max(0.0);
        let shown = if galley.size().x <= room {
            text.to_owned()
        } else {
            super::super::super::design_system::elide_text(ui, text, &font, room)
        };
        let galley = ui.painter().layout_no_wrap(shown, font.clone(), color);
        let width = galley.size().x;
        ui.painter().galley(
            egui::pos2(name_right - width, rect.center().y - galley.size().y * 0.5),
            galley,
            color,
        );
        name_right -= width + 8.0;
    }
    let shown = super::super::super::design_system::elide_text(ui, name, &font, name_right - left);
    ui.painter().text(
        egui::pos2(left, rect.center().y),
        egui::Align2::LEFT_CENTER,
        shown,
        font,
        name_color,
    );
}

struct ArtifactRowResponses {
    selection: egui::Response,
    check: egui::Response,
}

fn result_artifact_row(
    ui: &mut Ui,
    id: egui::Id,
    label: RowLabel<'_>,
    meta: &str,
    value: Option<&str>,
    selected: bool,
    checked: bool,
) -> ArtifactRowResponses {
    let name = label.full_name;
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let row_height = result_browser_row_height(&t);
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
    let selection = ui.interact(
        selection_rect,
        id.with("selection"),
        egui::Sense::click_and_drag(),
    );
    selection.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            match label.kind {
                SignalKind::Other(_) => format!("{name}, {meta}"),
                kind => format!("{name}, {}, {meta}", kind.label()),
            },
        )
    });
    if !ui.is_rect_visible(rect) {
        return ArtifactRowResponses { selection, check };
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
    let name_y = rect.center().y;
    let name_left = paint_kind_glyph(ui, selection_rect.left(), name_y, &label);
    // One register, as on a signal row: the value, or failing one the shape
    // and unit, takes the right column.
    let right_text = value.or((!meta.is_empty()).then_some(meta));
    paint_row_line(
        ui,
        rect,
        name_left,
        label.name,
        c.text,
        right_text.map(|text| {
            (
                text,
                if value.is_some() {
                    c.text_dim
                } else {
                    c.text_faint
                },
            )
        }),
    );
    theme::paint_focus_ring(ui, &check, check_rect);
    theme::paint_focus_ring(ui, &selection, selection_rect);
    ArtifactRowResponses {
        selection: selection.on_hover_text(format!("Inspect {name}")),
        check: check.on_hover_text(if checked {
            format!("Deselect {name}")
        } else {
            format!("Select {name} for a batch action")
        }),
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
    }
}
