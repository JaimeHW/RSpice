//! WAVES — stacked waveform strips, one per analysis in the active run.
//!
//! Each strip carries its analysis' traces with the strip grammar (header ·
//! legend · actions over a document well). AC strips convert magnitude to dB
//! on the left axis and route phase traces, dashed, to a right axis. A/B
//! cursors live on one strip at a time; their values, deltas and windowed
//! measurements render in the right panel.

mod expressions;
mod readout;

pub(crate) use expressions::*;
pub(crate) use readout::*;

use std::collections::HashSet;
use std::sync::Arc;

use egui::Ui;

use crate::analysis::calculator;
use crate::results::visualization_document::AccessibleColorPalette;
use crate::state::{
    AnalysisResult, AnalysisType, SharedWaveformValues, SimulationRun, SimulationState,
};
use crate::ui::plot::{
    self, Axis, CursorPair, DisplayDecimation, PlotSpec, SampleInterpolation, Trace, XScale,
    fmt_si_significant, fmt_significant, sample_at_with,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{chip, section_header};
use crate::workbench::AppState;
use crate::workbench::documents::visualization_family::{
    FamilyRenderGroup, FamilyRenderPlan, FamilyTraceStyle, SourceSampleSelection,
};
use crate::workbench::{
    ComplexNumberDisplay, CursorInterpolation, LargeDatasetDisplay, ResultPresentationPolicy,
};

use super::strip::{LegendChip, StripHeader};
use super::{
    DerivedSeries, ExprEditor, ExprSeries, ExprTrace, MarkerKind, ResultMarker, ResultsState,
    SelectedResultTrace, WaveformSeriesResult, waveform_color, well_hint,
};

/// How a trace's Y values are interpreted.
#[derive(Clone, Copy, PartialEq, Eq)]
enum TraceKind {
    /// Plain values (V, A, sweep output).
    Value,
    /// dB-converted AC magnitude.
    MagnitudeDb,
    /// Phase in degrees (right axis, dashed).
    PhaseDeg,
    /// Phase in radians (right axis, dashed).
    PhaseRad,
    /// Original real component of a complex source quantity.
    Real,
    /// Original imaginary component of a complex source quantity.
    Imaginary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct FamilyTraceVisibilityKey {
    group_key: u64,
    waveform_index: usize,
    trace_kind: u8,
}

impl FamilyTraceVisibilityKey {
    const fn new(group_key: u64, waveform_index: usize, trace_kind: TraceKind) -> Self {
        Self {
            group_key,
            waveform_index,
            trace_kind: trace_kind as u8,
        }
    }
}

impl TraceKind {
    const fn is_phase(self) -> bool {
        matches!(self, Self::PhaseDeg | Self::PhaseRad)
    }
}

/// One trace of a strip, with owned `Arc` handles into the run data.
struct StripTrace {
    waveform_index: usize,
    /// Name of the source waveform in the immutable dataset. Display names
    /// may be derived representations such as `re(V(out))`.
    source_waveform_name: String,
    /// Undecorated signal name used to preserve overlay pairing when the
    /// active source is expanded into several family groups.
    base_name: String,
    name: String,
    /// Ordinary signal color retained for non-family overlay runs.
    signal_color: egui::Color32,
    color: egui::Color32,
    x: SharedWaveformValues,
    y: SharedWaveformValues,
    kind: TraceKind,
    visible: bool,
    /// The run this trace belongs to (cache-key discriminator).
    run_id: u64,
    /// Overlay traces come from a non-active run: same signal hue, reduced
    /// weight, visibility slaved to the active run's matching signal.
    overlay: bool,
    /// Stable family-group identity mixed into all derived/cache keys.
    presentation_key: u64,
    family_group_ordinal: Option<usize>,
    family_style: Option<FamilyTraceStyle>,
    family_visibility_key: Option<FamilyTraceVisibilityKey>,
}

/// One strip (== one analysis of the active run).
pub(super) struct StripModel {
    analysis_index: usize,
    analysis_type: AnalysisType,
    kind_tag: String,
    subtitle: String,
    x_scale: XScale,
    x_dimension_key: String,
    x_label: String,
    x_unit: String,
    y_unit: &'static str,
    /// Phase traces carry the unwrapped (continuous) series instead of the
    /// raw ±180°-wrapped samples. Folded into the cache keys.
    phase_continuous: bool,
    /// Number of active-run traces at the front of `traces`; everything
    /// after is overlay. The legend lists only this prefix (signal owns
    /// hue — one chip per signal, all runs).
    signal_trace_count: usize,
    traces: Vec<StripTrace>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CursorDomain {
    analysis_type: AnalysisType,
    x_scale: XScale,
    x_dimension_key: String,
    x_label: String,
    x_unit: String,
}

/// Frame cache for the strip models. Building them clones every trace name
/// and walks all overlay runs, and both the center view and the right panel
/// ask for them each frame — the fingerprint covers everything the models
/// read, so the rebuild only happens when an input actually changes.
#[derive(Default, Clone)]
pub(super) struct ModelsCache(Option<(u64, Arc<Vec<StripModel>>)>);

impl std::fmt::Debug for ModelsCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ModelsCache(..)")
    }
}

/// Everything `build_models` reads: run data version, the display-run set,
/// per-trace visibility and stored color, phase mode, and the theme palette.
fn models_fingerprint(
    simulation: &SimulationState,
    phase_continuous: bool,
    complex_display: ComplexNumberDisplay,
    selection: Option<&SourceSampleSelection>,
    hidden_family_traces: &HashSet<FamilyTraceVisibilityKey>,
    t: &Tokens,
) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    simulation.data_version.hash(&mut h);
    phase_continuous.hash(&mut h);
    complex_display.hash(&mut h);
    selection
        .map(SourceSampleSelection::fingerprint)
        .hash(&mut h);
    let mut hidden = hidden_family_traces.iter().copied().collect::<Vec<_>>();
    hidden.sort_unstable();
    hidden.hash(&mut h);
    for color in &t.color.traces {
        color.to_array().hash(&mut h);
    }
    for run in simulation.display_runs() {
        run.id.hash(&mut h);
        for analysis in &run.analyses {
            analysis.analysis_type.hash(&mut h);
            analysis.label.hash(&mut h);
            analysis
                .provenance
                .as_ref()
                .map(|provenance| provenance.source_instance_id())
                .hash(&mut h);
            analysis.waveforms.len().hash(&mut h);
            for waveform in &analysis.waveforms {
                waveform.visible.hash(&mut h);
                waveform.color.hash(&mut h);
            }
        }
    }
    h.finish()
}

/// Fingerprint-cached [`build_models`]; the returned handle is cheap to
/// clone and stays valid across later state borrows.
pub(super) fn cached_models(
    simulation: &SimulationState,
    results: &mut ResultsState,
    complex_display: ComplexNumberDisplay,
    t: &Tokens,
) -> Arc<Vec<StripModel>> {
    let fp = models_fingerprint(
        simulation,
        results.phase_continuous,
        complex_display,
        results.sample_selection.as_ref(),
        &results.hidden_family_traces,
        t,
    );
    if let Some((cached_fp, models)) = &results.models.0
        && *cached_fp == fp
    {
        return Arc::clone(models);
    }
    let models = Arc::new(build_models(
        simulation,
        &mut results.derived,
        t,
        results.phase_continuous,
        complex_display,
        results.sample_selection.as_ref(),
        &results.hidden_family_traces,
    ));
    results.models.0 = Some((fp, Arc::clone(&models)));
    models
}

/// Fold a run identity into a cache key for overlay traces. Active-run
/// keys stay unchanged so existing envelopes/ranges remain warm.
fn run_mixed_key(base: u64, run_id: u64, overlay: bool) -> u64 {
    if overlay {
        base ^ run_id.wrapping_mul(0x9E37_79B9_7F4A_7C15)
    } else {
        base
    }
}

fn unique_analysis<'a>(
    mut candidates: impl Iterator<Item = &'a AnalysisResult>,
) -> Option<&'a AnalysisResult> {
    let candidate = candidates.next()?;
    candidates.next().is_none().then_some(candidate)
}

/// Resolve the result in an overlay run that was produced by the same
/// prepared analysis instance. Kind/label inference is permitted only for
/// unambiguous legacy history, where neither side has prepared provenance.
fn matching_overlay_analysis<'a>(
    analysis: &AnalysisResult,
    overlay_run: &'a SimulationRun,
) -> Option<&'a AnalysisResult> {
    if let Some(source_instance_id) = analysis
        .provenance
        .as_ref()
        .map(|provenance| provenance.source_instance_id())
    {
        return overlay_run
            .find_analysis_by_source_instance(source_instance_id)
            .filter(|candidate| candidate.analysis_type == analysis.analysis_type);
    }

    let legacy_candidates = || {
        overlay_run.analyses.iter().filter(|candidate| {
            candidate.provenance.is_none() && candidate.analysis_type == analysis.analysis_type
        })
    };
    unique_analysis(legacy_candidates().filter(|candidate| candidate.label == analysis.label))
        .or_else(|| unique_analysis(legacy_candidates()))
}

impl StripModel {
    fn cursor_domain(&self) -> CursorDomain {
        CursorDomain {
            analysis_type: self.analysis_type,
            x_scale: self.x_scale,
            x_dimension_key: self.x_dimension_key.clone(),
            x_label: self.x_label.clone(),
            x_unit: self.x_unit.clone(),
        }
    }

    fn x_label(&self) -> &str {
        &self.x_label
    }

    fn format_x(
        &self,
        x: f64,
        significant_digits: usize,
        quantity_policy: crate::quantity::QuantityPresentationPolicy,
    ) -> String {
        if self.x_unit == "Hz" {
            return quantity_policy.format_frequency(x, significant_digits);
        }
        fmt_si_significant(
            x,
            if self.x_unit.is_empty() {
                ""
            } else {
                &self.x_unit
            },
            significant_digits,
        )
    }

    fn format_trace_value(
        &self,
        trace: &StripTrace,
        value: f64,
        significant_digits: usize,
        quantity_policy: crate::quantity::QuantityPresentationPolicy,
    ) -> String {
        match trace.kind {
            TraceKind::Value => fmt_si_significant(value, self.y_unit, significant_digits),
            TraceKind::MagnitudeDb => fmt_significant(value, significant_digits, " dB"),
            TraceKind::PhaseDeg => {
                quantity_policy.format_angle(value.to_radians(), significant_digits)
            }
            TraceKind::PhaseRad => quantity_policy.format_angle(value, significant_digits),
            TraceKind::Real | TraceKind::Imaginary => {
                fmt_si_significant(value, self.y_unit, significant_digits)
            }
        }
    }

    // -- accessors for the TABLE viewer ------------------------------------
    //
    // The table renders this same model, so its columns cannot name a trace
    // the plot does not draw or report a value in a different unit than the
    // curve. These are the only reads it needs.

    /// Index of the analysis this strip renders.
    pub(super) fn analysis_index(&self) -> usize {
        self.analysis_index
    }

    /// The unit one trace is measured in.
    fn trace_unit(&self, trace: &StripTrace) -> &'static str {
        signal_unit(&trace.base_name, trace.kind, self.y_unit)
    }

    /// The strip's panes: visible traces grouped by unit, in the order the
    /// units first appear so a strip's layout is stable across frames.
    ///
    /// Phase does not take a pane of its own while a magnitude pane exists
    /// to read it against — splitting a Bode pair across two stacked panes
    /// would break the one reading they are drawn together for.
    pub(super) fn unit_panes(&self) -> Vec<UnitPane> {
        let mut panes: Vec<UnitPane> = Vec::new();
        let mut phase: Vec<usize> = Vec::new();
        for (index, trace) in self.traces.iter().enumerate() {
            if !trace.visible {
                continue;
            }
            if trace.kind.is_phase() {
                phase.push(index);
                continue;
            }
            let unit = self.trace_unit(trace);
            match panes.iter_mut().find(|pane| pane.unit == unit) {
                Some(pane) => pane.traces.push(index),
                None => panes.push(UnitPane {
                    unit,
                    traces: vec![index],
                    right: Vec::new(),
                }),
            }
        }
        if phase.is_empty() {
            return panes;
        }
        // Attach phase to the magnitude pane it belongs to; with no
        // magnitude on screen it becomes a pane in its own right.
        let host = panes
            .iter()
            .position(|pane| pane.unit == "dB")
            .or_else(|| (!panes.is_empty()).then_some(0));
        match host.and_then(|index| panes.get_mut(index)) {
            Some(pane) => pane.right = phase,
            None => {
                let unit = self
                    .traces
                    .get(phase[0])
                    .map_or("°", |trace| self.trace_unit(trace));
                panes.push(UnitPane {
                    unit,
                    traces: phase,
                    right: Vec::new(),
                });
            }
        }
        panes
    }

    /// Total trace count, including overlay runs.
    pub(super) fn trace_count(&self) -> usize {
        self.traces.len()
    }

    /// Indices of the visible active-run signal traces, in legend order.
    pub(super) fn visible_signal_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.traces
            .iter()
            .take(self.signal_trace_count)
            .enumerate()
            .filter(|(_, trace)| trace.visible)
            .map(|(index, _)| index)
    }

    /// The analysis' retained sample grid — the X array every trace on this
    /// strip was solved against.
    pub(super) fn sample_grid(&self) -> Option<&[f64]> {
        self.traces.first().map(|trace| trace.x.as_slice())
    }

    /// Column heading for the X axis ("t · s").
    pub(super) fn x_axis_heading(&self) -> String {
        if self.x_unit.is_empty() {
            self.x_label.clone()
        } else {
            format!("{} · {}", self.x_label, self.x_unit)
        }
    }

    /// The X value at one grid index, formatted.
    pub(super) fn format_x_at(
        &self,
        index: usize,
        significant_digits: usize,
        quantity_policy: crate::quantity::QuantityPresentationPolicy,
    ) -> String {
        self.sample_grid()
            .and_then(|grid| grid.get(index).copied())
            .map_or_else(
                || "—".to_owned(),
                |x| self.format_x(x, significant_digits, quantity_policy),
            )
    }

    /// One trace's retained value at a grid index, formatted in that
    /// trace's own unit.
    ///
    /// `None` when the trace is shorter than the grid: a table reporting
    /// exact samples must say it has none rather than borrow a neighbour's.
    pub(super) fn format_sample(
        &self,
        trace_index: usize,
        sample: usize,
        significant_digits: usize,
        quantity_policy: crate::quantity::QuantityPresentationPolicy,
    ) -> Option<String> {
        let trace = self.traces.get(trace_index)?;
        let value = trace.y.as_slice().get(sample).copied()?;
        Some(self.format_trace_value(trace, value, significant_digits, quantity_policy))
    }

    /// Display name of one trace, for a column heading.
    pub(super) fn trace_heading(&self, index: usize) -> Option<(&str, egui::Color32)> {
        self.traces
            .get(index)
            .map(|trace| (trace.name.as_str(), trace.color))
    }

    /// Analysis label for the table's analysis picker.
    pub(super) fn table_label(&self) -> String {
        format!("{} · {}", self.kind_tag, self.subtitle)
    }
}

fn selected_series_pair(
    x: &SharedWaveformValues,
    y: &SharedWaveformValues,
    selection: Option<&SourceSampleSelection>,
) -> Option<(SharedWaveformValues, SharedWaveformValues)> {
    let Some(selection) = selection else {
        return Some((Arc::clone(x), Arc::clone(y)));
    };
    selected_series_pair_indices(x, y, &selection.source_indices)
}

fn selected_series_pair_indices(
    x: &SharedWaveformValues,
    y: &SharedWaveformValues,
    source_indices: &[usize],
) -> Option<(SharedWaveformValues, SharedWaveformValues)> {
    if x.len() != y.len() || source_indices.last().is_some_and(|index| *index >= x.len()) {
        return None;
    }
    let selected_x = source_indices.iter().map(|index| x[*index]).collect();
    let selected_y = source_indices.iter().map(|index| y[*index]).collect();
    Some((Arc::new(selected_x), Arc::new(selected_y)))
}

struct FamilyProjection<'a> {
    group: Option<&'a FamilyRenderGroup>,
    x: SharedWaveformValues,
    y: SharedWaveformValues,
}

fn projected_family_series<'a>(
    x: &SharedWaveformValues,
    y: &SharedWaveformValues,
    selection: Option<&'a SourceSampleSelection>,
) -> Vec<FamilyProjection<'a>> {
    let Some(selection) = selection else {
        return vec![FamilyProjection {
            group: None,
            x: Arc::clone(x),
            y: Arc::clone(y),
        }];
    };
    let groups = selection
        .family_render_plan()
        .map(|plan| plan.groups())
        .filter(|groups| !groups.is_empty());
    if let Some(groups) = groups {
        return groups
            .iter()
            .filter_map(|group| {
                selected_series_pair_indices(x, y, &group.source_indices).and_then(|(_, y)| {
                    (group.x_values.len() == y.len()).then(|| FamilyProjection {
                        group: Some(group),
                        x: Arc::new(group.x_values.clone()),
                        y,
                    })
                })
            })
            .collect();
    }
    selected_series_pair(x, y, Some(selection))
        .map(|(x, y)| vec![FamilyProjection { group: None, x, y }])
        .unwrap_or_default()
}

/// Project an already-selected derived series through the same exact source
/// row groups. Expression evaluation returns rows in `source_indices` order,
/// so this maps group source identities back to those retained positions
/// without drawing false segments between different family categories.
fn projected_selected_family_series<'a>(
    x: &SharedWaveformValues,
    y: &SharedWaveformValues,
    selection: Option<&'a SourceSampleSelection>,
) -> Option<Vec<FamilyProjection<'a>>> {
    let Some(selection) = selection else {
        return Some(vec![FamilyProjection {
            group: None,
            x: Arc::clone(x),
            y: Arc::clone(y),
        }]);
    };
    if x.len() != y.len() || x.len() != selection.source_indices.len() {
        return None;
    }
    let Some(plan) = selection.family_render_plan() else {
        return Some(vec![FamilyProjection {
            group: None,
            x: Arc::clone(x),
            y: Arc::clone(y),
        }]);
    };
    let mut projections = Vec::with_capacity(plan.groups().len());
    for group in plan.groups() {
        let positions = group
            .source_indices
            .iter()
            .map(|source_index| selection.source_indices.binary_search(source_index))
            .collect::<Result<Vec<_>, _>>()
            .ok()?;
        let projected_y = positions.iter().map(|position| y[*position]).collect();
        if group.x_values.len() != positions.len() {
            return None;
        }
        projections.push(FamilyProjection {
            group: Some(group),
            x: Arc::new(group.x_values.clone()),
            y: Arc::new(projected_y),
        });
    }
    Some(projections)
}

fn family_color(style: FamilyTraceStyle, fallback: egui::Color32) -> egui::Color32 {
    let Some(color) = style.color else {
        return fallback;
    };
    let palette: &[[u8; 3]] = match color.palette {
        AccessibleColorPalette::OkabeItoCategorical => &[
            [0x00, 0x72, 0xB2],
            [0xE6, 0x9F, 0x00],
            [0x00, 0x9E, 0x73],
            [0xD5, 0x5E, 0x00],
            [0x56, 0xB4, 0xE9],
            [0xCC, 0x79, 0xA7],
        ],
        AccessibleColorPalette::TolBrightCategorical => &[
            [0x44, 0x77, 0xAA],
            [0xEE, 0x66, 0x77],
            [0x22, 0x88, 0x33],
            [0xCC, 0xBB, 0x44],
            [0x66, 0xCC, 0xEE],
            [0xAA, 0x33, 0x77],
            [0xBB, 0xBB, 0xBB],
        ],
        AccessibleColorPalette::CividisSequential => &[
            [0x00, 0x20, 0x4C],
            [0x41, 0x4D, 0x6B],
            [0x7C, 0x7B, 0x78],
            [0xBA, 0xA8, 0x63],
            [0xFF, 0xE9, 0x45],
        ],
        AccessibleColorPalette::ViridisSequential => &[
            [0x44, 0x01, 0x54],
            [0x3B, 0x52, 0x8B],
            [0x21, 0x91, 0x8C],
            [0x5E, 0xC9, 0x62],
            [0xFD, 0xE7, 0x25],
        ],
    };
    let index = match color.palette {
        AccessibleColorPalette::CividisSequential | AccessibleColorPalette::ViridisSequential
            if color.category_count > 1 =>
        {
            color.ordinal.saturating_mul(palette.len() - 1) / (color.category_count - 1)
        }
        _ => color.ordinal % palette.len(),
    };
    let [red, green, blue] = palette[index.min(palette.len() - 1)];
    egui::Color32::from_rgb(red, green, blue)
}

fn apply_family_trace_style<'a>(
    mut trace: Trace<'a>,
    style: Option<FamilyTraceStyle>,
) -> Trace<'a> {
    let Some(style) = style else {
        return trace;
    };
    trace = trace.show_single_point();
    if let Some(ordinal) = style.dash_ordinal {
        trace = trace.dash_style(ordinal);
    }
    if let Some(ordinal) = style.marker_ordinal {
        trace = trace.marker_style(ordinal);
    }
    if let Some(width) = style.width_points {
        trace = trace.width(width);
    }
    trace
}

#[allow(clippy::too_many_arguments)]
fn append_projected_traces(
    traces: &mut Vec<StripTrace>,
    derived: &mut DerivedSeries,
    analysis_index: usize,
    waveform_index: usize,
    run_id: u64,
    selection_key: u64,
    source_waveform_name: &str,
    base_name: &str,
    signal_color: egui::Color32,
    kind: TraceKind,
    source_x: &SharedWaveformValues,
    source_y: &SharedWaveformValues,
    sample_selection: Option<&SourceSampleSelection>,
    hidden_family_traces: &HashSet<FamilyTraceVisibilityKey>,
    phase_continuous: bool,
    visible: bool,
) {
    for projection in projected_family_series(source_x, source_y, sample_selection) {
        let presentation_key = projection.group.map_or(0, |group| group.stable_key);
        let derived_key = ((analysis_index as u64) << 32 | waveform_index as u64)
            ^ selection_key
            ^ presentation_key.rotate_left(23);
        let y = match kind {
            TraceKind::MagnitudeDb => derived.db(derived_key, &projection.y),
            TraceKind::PhaseDeg | TraceKind::PhaseRad => displayed_phase_series(
                derived,
                derived_key,
                &projection.y,
                phase_continuous,
                kind == TraceKind::PhaseRad,
            ),
            _ => projection.y,
        };
        let family_style = projection.group.map(|group| group.style);
        let family_visibility_key = projection
            .group
            .map(|group| FamilyTraceVisibilityKey::new(group.stable_key, waveform_index, kind));
        let name = projection.group.map_or_else(
            || base_name.to_owned(),
            |group| format!("{base_name} · {}", group.label),
        );
        traces.push(StripTrace {
            waveform_index,
            source_waveform_name: source_waveform_name.to_owned(),
            base_name: base_name.to_owned(),
            name,
            signal_color,
            color: family_style.map_or(signal_color, |style| family_color(style, signal_color)),
            x: projection.x,
            y,
            kind,
            visible: visible
                && family_visibility_key.is_none_or(|key| !hidden_family_traces.contains(&key)),
            run_id,
            overlay: false,
            presentation_key,
            family_group_ordinal: projection.group.map(|group| group.ordinal),
            family_style,
            family_visibility_key,
        });
    }
}

/// Build strip models for every plottable analysis of the active run.
/// `phase_continuous` swaps phase traces to their unwrapped series.
pub(super) fn build_models(
    simulation: &SimulationState,
    derived: &mut DerivedSeries,
    tokens: &Tokens,
    phase_continuous: bool,
    complex_display: ComplexNumberDisplay,
    selection: Option<&SourceSampleSelection>,
    hidden_family_traces: &HashSet<FamilyTraceVisibilityKey>,
) -> Vec<StripModel> {
    let display_runs = simulation.display_runs();
    let Some((&run, overlay_runs)) = display_runs.split_first() else {
        return Vec::new();
    };
    let mut models = Vec::new();

    for (analysis_index, analysis) in run.analyses.iter().enumerate() {
        if analysis.waveforms.is_empty() {
            continue;
        }
        let displays_cartesian_complex = analysis.analysis_type == AnalysisType::Ac
            && complex_display == ComplexNumberDisplay::RealImaginary
            && analysis
                .waveforms
                .iter()
                .any(|waveform| waveform.complex.is_some());
        let sample_selection = selection.filter(|selection| {
            selection.dataset_id == run.dataset_id && selection.analysis_sequence == analysis.id
        });
        let (mut x_scale, mut x_dimension_key, mut x_label, mut x_unit, y_unit) =
            match analysis.analysis_type {
                AnalysisType::Ac if displays_cartesian_complex => {
                    (XScale::Log10, "frequency", "f", "Hz", "")
                }
                AnalysisType::Ac => (XScale::Log10, "frequency", "f", "Hz", "dB"),
                AnalysisType::Noise | AnalysisType::Pnoise => {
                    (XScale::Log10, "frequency", "f", "Hz", "V^2/Hz")
                }
                AnalysisType::Transient => (XScale::Linear, "time", "t", "s", "V"),
                AnalysisType::DcSweep => (XScale::Linear, "dc-sweep", "x", "V", "V"),
                _ => (XScale::Linear, "x", "x", "", "V"),
            };
        if let Some(axis) = sample_selection
            .and_then(SourceSampleSelection::family_render_plan)
            .map(FamilyRenderPlan::x_axis)
        {
            // Family policies currently persist no logarithmic scale. Exact
            // manifest coordinates therefore use the only truthful default.
            x_scale = XScale::Linear;
            x_dimension_key = &axis.dimension_key;
            x_label = &axis.label;
            x_unit = &axis.unit;
        }

        let mut traces = Vec::new();
        let selection_key = sample_selection
            .map(SourceSampleSelection::fingerprint)
            .unwrap_or_default()
            .rotate_left(17);
        for (waveform_index, waveform) in analysis.waveforms.iter().enumerate() {
            let color = waveform_color(waveform, waveform_index, tokens);
            let is_phase = waveform.name.starts_with("phase(");
            let is_mag = waveform.name.starts_with('|');
            if displays_cartesian_complex {
                if let Some(complex) = &waveform.complex {
                    for (kind, name, signal_color, y) in [
                        (
                            TraceKind::Real,
                            format!("re({})", complex.source_name),
                            color,
                            &complex.real,
                        ),
                        (
                            TraceKind::Imaginary,
                            format!("im({})", complex.source_name),
                            tokens.color.traces[(waveform_index + 1) % tokens.color.traces.len()],
                            &complex.imag,
                        ),
                    ] {
                        append_projected_traces(
                            &mut traces,
                            derived,
                            analysis_index,
                            waveform_index,
                            run.id,
                            selection_key,
                            &waveform.name,
                            &name,
                            signal_color,
                            kind,
                            &waveform.x,
                            y,
                            sample_selection,
                            hidden_family_traces,
                            phase_continuous,
                            waveform.visible,
                        );
                    }
                    continue;
                }
                if is_phase
                    && analysis.waveforms.iter().any(|candidate| {
                        candidate.complex.as_ref().is_some_and(|complex| {
                            waveform.name == format!("phase({})", complex.source_name)
                        })
                    })
                {
                    continue;
                }
            }
            let kind = if analysis.analysis_type == AnalysisType::Ac && is_phase {
                match complex_display {
                    ComplexNumberDisplay::MagnitudePhaseRadians => TraceKind::PhaseRad,
                    _ => TraceKind::PhaseDeg,
                }
            } else if analysis.analysis_type == AnalysisType::Ac && is_mag {
                TraceKind::MagnitudeDb
            } else {
                TraceKind::Value
            };
            append_projected_traces(
                &mut traces,
                derived,
                analysis_index,
                waveform_index,
                run.id,
                selection_key,
                &waveform.name,
                &waveform.name,
                color,
                kind,
                &waveform.x,
                &waveform.y,
                sample_selection,
                hidden_family_traces,
                phase_continuous,
                waveform.visible,
            );
        }
        let signal_trace_count = traces.len();
        let overlay_signals = traces[..signal_trace_count]
            .iter()
            .map(|trace| {
                (
                    trace.source_waveform_name.clone(),
                    trace.base_name.clone(),
                    trace.name.clone(),
                    trace.signal_color,
                    trace.color,
                    trace.kind,
                    trace.visible,
                    trace.presentation_key,
                    trace.family_group_ordinal,
                    trace.family_style,
                )
            })
            .collect::<Vec<_>>();

        // Overlay runs: match the exact prepared analysis instance and merge
        // traces by signal name. Signal owns hue —
        // overlay traces reuse the active trace's color and visibility —
        // run owns weight (applied at draw time).
        let mut overlaid_run_count = 0usize;
        let mut rejected_overlay_count = 0usize;
        for overlay_run in overlay_runs {
            let overlay_analysis = matching_overlay_analysis(analysis, overlay_run);
            let Some(overlay_analysis) = overlay_analysis else {
                continue;
            };

            let overlay_family_plan = match sample_selection
                .map(|selection| selection.overlay_render_plan(overlay_analysis))
                .transpose()
            {
                Ok(plan) => plan.flatten(),
                Err(_) => {
                    rejected_overlay_count += 1;
                    continue;
                }
            };

            let mut contributed = false;
            let mut projected_overlay_traces = Vec::new();
            let mut incompatible_x = false;
            for (
                source_name,
                base_name,
                signal_name,
                signal_color,
                display_color,
                signal_kind,
                signal_visible,
                presentation_key,
                family_group_ordinal,
                family_style,
            ) in &overlay_signals
            {
                let Some((overlay_index, overlay_waveform)) = overlay_analysis
                    .waveforms
                    .iter()
                    .enumerate()
                    .find(|(_, waveform)| waveform.name == *source_name)
                else {
                    continue;
                };

                let base_key = (analysis_index as u64) << 32 | overlay_index as u64;
                let derived_key = run_mixed_key(base_key, overlay_run.id, true);
                let source_y = match *signal_kind {
                    TraceKind::MagnitudeDb => derived.db(derived_key, &overlay_waveform.y),
                    TraceKind::PhaseDeg | TraceKind::PhaseRad => displayed_phase_series(
                        derived,
                        derived_key,
                        &overlay_waveform.y,
                        phase_continuous,
                        *signal_kind == TraceKind::PhaseRad,
                    ),
                    TraceKind::Real => {
                        let Some(complex) = &overlay_waveform.complex else {
                            continue;
                        };
                        Arc::clone(&complex.real)
                    }
                    TraceKind::Imaginary => {
                        let Some(complex) = &overlay_waveform.complex else {
                            continue;
                        };
                        Arc::clone(&complex.imag)
                    }
                    _ => Arc::clone(&overlay_waveform.y),
                };
                let projected = if let Some(plan) = overlay_family_plan.as_ref() {
                    let Some(group) = family_group_ordinal
                        .and_then(|ordinal| plan.groups().get(ordinal))
                        .filter(|group| group.stable_key == *presentation_key)
                    else {
                        incompatible_x = true;
                        break;
                    };
                    let Some((_, y)) = selected_series_pair_indices(
                        &overlay_waveform.x,
                        &source_y,
                        &group.source_indices,
                    ) else {
                        incompatible_x = true;
                        break;
                    };
                    if y.len() != group.x_values.len() {
                        incompatible_x = true;
                        break;
                    }
                    (Arc::new(group.x_values.clone()), y)
                } else if let Some(selection) = sample_selection {
                    let Some(series) =
                        selected_series_pair(&overlay_waveform.x, &source_y, Some(selection))
                    else {
                        incompatible_x = true;
                        break;
                    };
                    series
                } else {
                    (Arc::clone(&overlay_waveform.x), source_y)
                };
                projected_overlay_traces.push(StripTrace {
                    waveform_index: overlay_index,
                    source_waveform_name: source_name.clone(),
                    base_name: base_name.clone(),
                    name: signal_name.clone(),
                    signal_color: *signal_color,
                    color: *display_color,
                    x: projected.0,
                    y: projected.1,
                    kind: *signal_kind,
                    visible: *signal_visible,
                    run_id: overlay_run.id,
                    overlay: true,
                    presentation_key: *presentation_key,
                    family_group_ordinal: *family_group_ordinal,
                    family_style: *family_style,
                    family_visibility_key: None,
                });
                contributed = true;
            }
            if incompatible_x {
                rejected_overlay_count += 1;
                continue;
            }
            if contributed {
                traces.extend(projected_overlay_traces);
                overlaid_run_count += 1;
            }
        }

        let mut subtitle = analysis.label.clone();
        if overlaid_run_count > 0 {
            subtitle = format!(
                "{subtitle} · +{overlaid_run_count} run{} overlaid",
                if overlaid_run_count == 1 { "" } else { "s" }
            );
        }
        if rejected_overlay_count > 0 {
            subtitle = format!(
                "{subtitle} · {rejected_overlay_count} incompatible family overlay{} hidden",
                if rejected_overlay_count == 1 { "" } else { "s" }
            );
        }

        models.push(StripModel {
            analysis_index,
            analysis_type: analysis.analysis_type,
            kind_tag: analysis.analysis_type.short_label().to_uppercase(),
            subtitle,
            x_scale,
            x_dimension_key: x_dimension_key.to_owned(),
            x_label: x_label.to_owned(),
            x_unit: x_unit.to_owned(),
            y_unit,
            phase_continuous,
            signal_trace_count,
            traces,
        });
    }
    models
}

fn displayed_phase_series(
    derived: &mut DerivedSeries,
    key: u64,
    phase_degrees: &SharedWaveformValues,
    continuous: bool,
    radians: bool,
) -> SharedWaveformValues {
    let degrees = if continuous {
        derived.unwrapped(key, phase_degrees)
    } else {
        Arc::clone(phase_degrees)
    };
    if !radians {
        return degrees;
    }
    // Radian conversion is a cached presentation series. Stored samples stay
    // in their original degree representation for reproducibility/export.
    const RADIANS_KEY_BIT: u64 = 1 << 61;
    const CONTINUOUS_KEY_BIT: u64 = 1 << 60;
    derived.get_or(
        key ^ RADIANS_KEY_BIT ^ if continuous { CONTINUOUS_KEY_BIT } else { 0 },
        || Arc::new(degrees.iter().map(|value| value.to_radians()).collect()),
    )
}

/// Stable per-trace identity shared by the decimation, range, and
/// measurement caches. Phase traces fold in the wrapped/continuous choice
/// so a toggle never serves stale envelopes, ranges, or stats.
fn trace_key(model: &StripModel, trace: &StripTrace) -> u64 {
    run_mixed_key(anchor_key(model, trace), trace.run_id, trace.overlay)
}

/// Run-independent identity of a trace: what a marker anchors to.
///
/// Every solve of the same signal shares this key, which is exactly the
/// difference from [`trace_key`] — a decimation envelope must not outlive
/// the run that produced it, while a marker is a statement about the
/// signal and is meant to survive re-simulation.
fn anchor_key(model: &StripModel, trace: &StripTrace) -> u64 {
    let continuous = (trace.kind.is_phase() && model.phase_continuous) as u64;
    let base = (model.analysis_index as u64) << 44
        | continuous << 43
        | (trace.waveform_index as u64) << 3
        | trace.kind as u64;
    base ^ trace.presentation_key.rotate_left(11)
}

/// Case-insensitive prefix test for a signal-name accessor (`V(`, `i(`).
fn starts_with_accessor(name: &str, accessor: &str) -> bool {
    name.len() >= accessor.len() && name[..accessor.len()].eq_ignore_ascii_case(accessor)
}

/// Peel the derived-projection wrappers off a display name so the
/// underlying accessor is visible: `re(V(out))` is still volts.
fn unwrap_projection(name: &str) -> &str {
    let name = name.trim_start();
    for wrapper in ["re(", "im(", "mag(", "abs("] {
        if starts_with_accessor(name, wrapper) {
            return name[wrapper.len()..].trim_start();
        }
    }
    name.strip_prefix('|').unwrap_or(name)
}

/// The engineering unit a trace is measured in.
///
/// This is the axis model: a signal owns its unit, and the analysis default
/// applies only where the name carries no accessor to read it from. Without
/// this, a transient strip carrying `V(out)` and `I(R1)` would label both
/// against the analysis' nominal volts and quietly misreport the current.
fn signal_unit(name: &str, kind: TraceKind, analysis_unit: &'static str) -> &'static str {
    match kind {
        TraceKind::MagnitudeDb => "dB",
        TraceKind::PhaseDeg => "°",
        TraceKind::PhaseRad => "rad",
        TraceKind::Value | TraceKind::Real | TraceKind::Imaginary => {
            let name = unwrap_projection(name);
            if starts_with_accessor(name, "i(") {
                "A"
            } else if starts_with_accessor(name, "v(") {
                "V"
            } else if starts_with_accessor(name, "p(") {
                "W"
            } else {
                analysis_unit
            }
        }
    }
}

/// One Y axis of a strip: the traces measured in a single unit.
///
/// Panes of a strip always share the strip's X domain — they are one
/// measurement read against several scales, not several plots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct UnitPane {
    /// The unit every trace on this pane's left axis is measured in.
    pub unit: &'static str,
    /// Trace indices on the left axis.
    pub traces: Vec<usize>,
    /// Trace indices on this pane's right axis — phase read against the
    /// magnitude it belongs to, which is what keeps a Bode pair together.
    pub right: Vec<usize>,
}

/// Y range of the visible traces on one axis side, padded 8 %. Per-trace
/// extremes are cached on the data version — never rescanned per frame.
/// Y range of one pane's traces, padded 8 %. Per-trace extremes are cached
/// on the data version — never rescanned per frame.
///
/// The fit is per pane because each pane carries its own unit: fitting
/// volts and amps to one range would flatten whichever is smaller.
fn pane_y_range(
    derived: &mut DerivedSeries,
    model: &StripModel,
    indices: &[usize],
) -> Option<(f64, f64)> {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for trace in indices.iter().filter_map(|index| model.traces.get(*index)) {
        let extremes =
            derived.range_or(trace_key(model, trace), || super::finite_extremes(&trace.y));
        if let Some((lo, hi)) = extremes {
            min = min.min(lo);
            max = max.max(hi);
        }
    }
    if !min.is_finite() {
        return None;
    }
    if min == max {
        return Some((min - 1.0, max + 1.0));
    }
    let pad = (max - min) * 0.08;
    Some((min - pad, max + pad))
}

/// X range of a strip. Ordinary traces share one X series; family policies
/// intentionally project disjoint exact-row groups, so the range must cover
/// every visible group rather than assuming the first trace is authoritative.
fn x_range(model: &StripModel) -> Option<(f64, f64)> {
    let mut x0 = f64::INFINITY;
    let mut x1 = f64::NEG_INFINITY;
    for x in model
        .traces
        .iter()
        .filter(|trace| trace.visible)
        .flat_map(|trace| trace.x.iter().copied())
        .filter(|value| value.is_finite())
    {
        if model.x_scale == XScale::Log10 && x <= 0.0 {
            continue;
        }
        x0 = x0.min(x);
        x1 = x1.max(x);
    }
    if !x0.is_finite() || !x1.is_finite() {
        return None;
    }
    if x1 > x0 {
        return Some((x0, x1));
    }
    if model.x_scale == XScale::Log10 {
        Some((x0 / 10.0, x1 * 10.0))
    } else {
        Some((x0 - 1.0, x1 + 1.0))
    }
}

const fn cursor_interpolation(policy: CursorInterpolation) -> SampleInterpolation {
    match policy {
        CursorInterpolation::MonotoneCubicWhereValid => SampleInterpolation::MonotoneCubic,
        CursorInterpolation::Linear => SampleInterpolation::Linear,
        CursorInterpolation::NearestAcceptedPoint => SampleInterpolation::Nearest,
    }
}

const fn display_decimation(policy: LargeDatasetDisplay) -> DisplayDecimation {
    match policy {
        LargeDatasetDisplay::EnvelopeExtrema => DisplayDecimation::EnvelopeExtrema,
        LargeDatasetDisplay::UniformDisplaySampling => DisplayDecimation::Uniform,
        LargeDatasetDisplay::NoDisplayDecimation => DisplayDecimation::FullResolution,
    }
}

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the strip stack.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    show_with_pane_chrome(ui, state, true);
}

/// Render the same retained waveform strips inside the compact split-results
/// pane. Trace selection and swatch visibility remain fully interactive, while
/// the pane-only maximize/close/fit and expression controls are omitted.
pub fn show_compact(ui: &mut Ui, state: &mut AppState) {
    show_with_pane_chrome(ui, state, false);
}

fn show_with_pane_chrome(ui: &mut Ui, state: &mut AppState, pane_chrome: bool) {
    let t = Tokens::get(ui.ctx());
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &t,
    );
    if models.is_empty() {
        let hint = if state.simulation.active_run().is_none() {
            let shortcut = state.ui.preferences.shortcuts().resolved_label(
                crate::workbench::commands::vocabulary::Command::RunSimulation,
                crate::workbench::app_state::runtime_command_platform(ui.ctx()),
                ui.ctx().os(),
            );
            if shortcut.is_empty() {
                "No results yet — use the Run button to simulate".to_owned()
            } else {
                format!("No results yet — press {shortcut} or use the Run button to simulate")
            }
        } else {
            "The active run has no plottable analyses".to_owned()
        };
        well_hint(ui, &hint);
        return;
    }

    // Apply hide/maximize strip state.
    let results = &state.ui.results;
    let visible: Vec<&StripModel> = match results.maximized_strip {
        Some(max_idx) if models.iter().any(|m| m.analysis_index == max_idx) => models
            .iter()
            .filter(|m| m.analysis_index == max_idx)
            .collect(),
        _ => models
            .iter()
            .filter(|m| !results.hidden_strips.contains(&m.analysis_index))
            .collect(),
    };
    if visible.is_empty() {
        well_hint(ui, "All strips hidden — restore them from the document bar");
        return;
    }

    // Deferred state mutations (collected while iterating immutably).
    let mut toggle_trace: Option<(usize, usize)> = None;
    let mut toggle_family_trace: Option<FamilyTraceVisibilityKey> = None;
    let mut toggle_maximize: Option<usize> = None;
    let mut close_strip: Option<usize> = None;
    let mut fit_strip: Option<usize> = None;
    let mut toggle_expr: Option<(usize, usize)> = None;
    let mut remove_expr: Option<(usize, usize)> = None;
    let mut open_editor: Option<usize> = None;
    let mut select_trace: Option<SelectedResultTrace> = None;
    let active_dataset_id = state
        .simulation
        .active_run()
        .map(|run| run.dataset_id.clone());

    let avail = ui.available_rect_before_wrap();
    let n = visible.len();
    let separators = (n.saturating_sub(1)) as f32;
    let strip_height = ((avail.height() - separators) / n as f32).max(140.0);
    let maximized = state.ui.results.maximized_strip.is_some();
    let linked_cursor_domain = state
        .ui
        .results
        .cursor_strip
        .and_then(|owner| models.iter().find(|model| model.analysis_index == owner))
        .map(|model| model.cursor_domain());

    egui::ScrollArea::vertical()
        .id_salt("rspice.results.strips")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
            for (position, model) in visible.iter().enumerate() {
                if position > 0 {
                    // 1 px border seam between strips.
                    let (seam, _) = ui.allocate_exact_size(
                        egui::vec2(ui.available_width(), 1.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().rect_filled(seam, 0.0, t.color.border);
                }
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), strip_height),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.set_min_height(strip_height);
                        // Legend chips list signals only (the active-run
                        // prefix): one chip per signal toggles it across
                        // every overlaid run.
                        let legend: Vec<LegendChip<'_>> = model
                            .traces
                            .iter()
                            .take(model.signal_trace_count)
                            .map(|trace| LegendChip {
                                name: &trace.name,
                                color: trace.color,
                                on: trace.visible,
                            })
                            .collect();
                        // Expression chips follow the waveform chips; long
                        // expressions are elided so a chip never eats the row.
                        let strip_exprs: Vec<ExprTrace> = state
                            .ui
                            .results
                            .exprs
                            .get(&model.analysis_index)
                            .cloned()
                            .unwrap_or_default();
                        let expr_labels: Vec<String> =
                            strip_exprs.iter().map(|e| elide(&e.text, 24)).collect();
                        let mut legend = legend;
                        for (i, expr) in strip_exprs.iter().enumerate() {
                            legend.push(LegendChip {
                                name: &expr_labels[i],
                                color: expr_color(&t, model.signal_trace_count + i),
                                on: expr.visible,
                            });
                        }

                        let zoomed = state
                            .ui
                            .results
                            .strip_is_zoomed(super::ResultViewer::Waves, model.analysis_index);
                        let selected_legend = state
                            .ui
                            .results
                            .valid_selected_trace(&state.simulation)
                            .and_then(|selected| {
                                (selected.analysis_index == model.analysis_index).then(|| {
                                    model.traces.iter().take(model.signal_trace_count).position(
                                        |trace| {
                                            trace.waveform_index == selected.waveform_index
                                                && trace.source_waveform_name
                                                    == selected.source_name
                                        },
                                    )
                                })
                            })
                            .flatten();
                        let header = StripHeader::new(&model.kind_tag, &model.subtitle, &legend)
                            .maximized(maximized)
                            .closable(pane_chrome && !maximized && n > 1)
                            .zoomed(zoomed)
                            .expr_action(pane_chrome)
                            .removable_from(model.signal_trace_count)
                            .selected_legend(selected_legend)
                            .pane_actions(pane_chrome)
                            .show(ui);
                        if let Some(chip_index) = header.legend_clicked {
                            if chip_index < model.signal_trace_count {
                                if let (Some(trace), Some(dataset_id)) =
                                    (model.traces.get(chip_index), active_dataset_id.clone())
                                {
                                    select_trace = Some(SelectedResultTrace {
                                        dataset_id,
                                        analysis_index: model.analysis_index,
                                        waveform_index: trace.waveform_index,
                                        source_name: trace.source_waveform_name.clone(),
                                    });
                                }
                            } else {
                                toggle_expr = Some((
                                    model.analysis_index,
                                    chip_index - model.signal_trace_count,
                                ));
                            }
                        }
                        if let Some(chip_index) = header.legend_visibility_clicked {
                            if chip_index < model.signal_trace_count {
                                if let Some(trace) = model.traces.get(chip_index) {
                                    if let Some(key) = trace.family_visibility_key {
                                        toggle_family_trace = Some(key);
                                    } else {
                                        toggle_trace =
                                            Some((model.analysis_index, trace.waveform_index));
                                    }
                                }
                            } else {
                                toggle_expr = Some((
                                    model.analysis_index,
                                    chip_index - model.signal_trace_count,
                                ));
                            }
                        }
                        if let Some(chip_index) = header.legend_removed
                            && chip_index >= model.signal_trace_count
                        {
                            remove_expr =
                                Some((model.analysis_index, chip_index - model.signal_trace_count));
                        }
                        if header.maximize_clicked {
                            toggle_maximize = Some(model.analysis_index);
                        }
                        if header.close_clicked {
                            close_strip = Some(model.analysis_index);
                        }
                        if header.fit_clicked {
                            fit_strip = Some(model.analysis_index);
                        }
                        if header.add_expr_clicked {
                            open_editor = Some(model.analysis_index);
                        }

                        expr_editor_row(ui, state, model.analysis_index);

                        // Strips scrolled out of view skip the plot body
                        // entirely (range lookups, envelope mapping, shape
                        // building) — only the space is reserved.
                        let plot_rect = ui.available_rect_before_wrap();
                        if ui.is_rect_visible(plot_rect) {
                            show_strip_plot(ui, state, model, linked_cursor_domain.as_ref());
                        } else {
                            ui.allocate_exact_size(plot_rect.size(), egui::Sense::hover());
                        }
                    },
                );
            }
        });

    // Apply deferred mutations.
    if let Some((analysis_index, waveform_index)) = toggle_trace {
        toggle_visibility(state, analysis_index, waveform_index);
    }
    if let Some(key) = toggle_family_trace {
        state.ui.results.toggle_family_trace_visibility(key);
    }
    if let Some(selected) = select_trace {
        state.ui.results.selected_trace = Some(selected);
    }
    let results = &mut state.ui.results;
    if let Some(idx) = toggle_maximize {
        results.maximized_strip = (results.maximized_strip != Some(idx)).then_some(idx);
    }
    if let Some(idx) = close_strip {
        results.hidden_strips.insert(idx);
        if results.cursor_strip == Some(idx) {
            results.clear_cursors();
        }
    }
    if let Some(idx) = fit_strip {
        results.reset_plot_view(super::ResultViewer::Waves, idx);
    }
    if let Some((analysis, index)) = toggle_expr
        && let Some(expr) = results
            .exprs
            .get_mut(&analysis)
            .and_then(|list| list.get_mut(index))
    {
        expr.visible = !expr.visible;
    }
    if let Some((analysis, index)) = remove_expr
        && let Some(list) = results.exprs.get_mut(&analysis)
    {
        if index < list.len() {
            let removed = list.remove(index);
            results.expr_cache.remove(&(analysis, removed.text));
        }
        if list.is_empty() {
            results.exprs.remove(&analysis);
        }
    }
    if let Some(analysis) = open_editor {
        results.expr_editor = Some(ExprEditor {
            analysis_index: analysis,
            text: String::new(),
            error: None,
            want_focus: true,
        });
    }
}

/// Shorten a label to `max` characters with a typographic ellipsis.
fn elide(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_owned();
    }
    let mut out: String = text.chars().take(max.saturating_sub(1)).collect();
    out.push('…');
    out
}

fn append_copied_cursor(
    target: &mut String,
    cursor: &str,
    x: f64,
    model: &StripModel,
    interpolation: SampleInterpolation,
    policy: crate::quantity::QuantityPresentationPolicy,
) {
    use std::fmt::Write as _;

    let copied_x = if model.x_unit == "Hz" {
        policy.copy_frequency(x)
    } else {
        policy.copy_si_value(x, &model.x_unit)
    };
    let _ = writeln!(
        target,
        "{cursor} {} = {}",
        model.x_label(),
        copied_x.trim_end()
    );
    for trace in model.traces.iter().filter(|trace| trace.visible).take(6) {
        let value = sample_at_with(&trace.x, &trace.y, x, interpolation);
        let copied = match trace.kind {
            TraceKind::PhaseDeg => policy.copy_angle(value.to_radians()),
            TraceKind::PhaseRad => policy.copy_angle(value),
            TraceKind::MagnitudeDb => policy.copy_si_value(value, "dB"),
            TraceKind::Value | TraceKind::Real | TraceKind::Imaginary => {
                policy.copy_si_value(value, model.y_unit)
            }
        };
        let _ = writeln!(target, "{} = {}", trace.name, copied.trim_end());
    }
    while target.ends_with('\n') {
        target.pop();
    }
}

/// Kind owns the marker's colour: a spec limit reads as a bound to meet,
/// a peak as a called-out feature, a note as neutral annotation.
fn marker_color(kind: MarkerKind, t: &Tokens) -> egui::Color32 {
    match kind {
        MarkerKind::Note => t.color.text,
        MarkerKind::Peak => t.color.accent,
        MarkerKind::Spec => t.color.warn,
    }
}

/// Tag text: the id always, the note only when the user wrote one.
fn marker_label(marker: &ResultMarker) -> String {
    if marker.note.trim().is_empty() {
        format!("M{}", marker.id)
    } else {
        format!("M{} · {}", marker.id, marker.note.trim())
    }
}

/// One strip, drawn as one pane per unit.
///
/// Signals route to the pane that owns their unit; the panes stack and
/// share the strip's X domain, so a strip stays one measurement read
/// against as many scales as it genuinely needs.
fn show_strip_plot(
    ui: &mut Ui,
    state: &mut AppState,
    model: &StripModel,
    linked_cursor_domain: Option<&CursorDomain>,
) {
    let t = Tokens::get(ui.ctx());
    let Some(x_domain) = x_range(model) else {
        well_hint(ui, "No data");
        return;
    };
    let panes = model.unit_panes();
    if panes.is_empty() {
        well_hint(ui, "No visible traces — enable one in the legend");
        return;
    }

    // Expression traces participate in the first pane's automatic fit.
    let exprs = resolve_strip_exprs(state, model, &t);
    let available = ui.available_rect_before_wrap();
    let count = panes.len();
    let seams = count.saturating_sub(1) as f32;
    let pane_height = ((available.height() - seams) / count as f32).max(56.0);

    for (ordinal, pane) in panes.iter().enumerate() {
        if ordinal > 0 {
            let (seam, _) =
                ui.allocate_exact_size(egui::vec2(ui.available_width(), 1.0), egui::Sense::hover());
            ui.painter().rect_filled(seam, 0.0, t.color.canvas_grid);
        }
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), pane_height),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.set_min_height(pane_height);
                show_unit_pane(
                    ui,
                    state,
                    model,
                    pane,
                    ordinal,
                    x_domain,
                    // Only the first pane carries the strip's expressions:
                    // an expression has no declared unit, so it cannot be
                    // routed to a unit-owning pane on the evidence available.
                    if ordinal == 0 { &exprs } else { &[] },
                    linked_cursor_domain,
                );
            },
        );
    }
}

fn show_unit_pane(
    ui: &mut Ui,
    state: &mut AppState,
    model: &StripModel,
    pane: &UnitPane,
    ordinal: usize,
    x_domain: (f64, f64),
    exprs: &[ResolvedExpr],
    linked_cursor_domain: Option<&CursorDomain>,
) {
    let t = Tokens::get(ui.ctx());
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    let (x0, x1) = x_domain;

    let pane_range = pane_y_range(&mut state.ui.results.derived, model, &pane.traces);
    let auto_y = {
        let mut lo = f64::INFINITY;
        let mut hi = f64::NEG_INFINITY;
        if let Some((a, b)) = pane_range {
            lo = a;
            hi = b;
        }
        for expr in exprs {
            if let Some((a, b)) = expr.y_extremes {
                lo = lo.min(a);
                hi = hi.max(b);
            }
        }
        if !lo.is_finite() || !hi.is_finite() {
            None
        } else if lo == hi {
            Some((lo - 1.0, hi + 1.0))
        } else {
            Some((lo, hi))
        }
    };
    let Some((y0, y1)) = auto_y else {
        well_hint(ui, "No visible traces — enable one in the legend");
        return;
    };

    // User zoom/pan overrides the automatic fit per axis, per pane.
    let view =
        state
            .ui
            .results
            .plot_view_pane(super::ResultViewer::Waves, model.analysis_index, ordinal);
    let (x0, x1) = view.x.unwrap_or((x0, x1));
    let (y0, y1) = view.y.unwrap_or((y0, y1));

    let mut x_axis = match model.x_scale {
        XScale::Log10 => Axis::log_decades(x0, x1, &model.x_unit),
        XScale::Linear => Axis::linear(x0, x1, &model.x_unit),
    }
    .with_label(&model.x_label);
    if model.x_unit == "Hz" {
        let (scale, offset, unit) = quantity_policy.frequency_axis_transform();
        x_axis = x_axis.with_display_transform(scale, offset, unit);
    }
    let mut spec = PlotSpec::new(x_axis, model.x_scale, Axis::linear(y0, y1, pane.unit))
        .accessible_name("Waveform plot");
    spec.display_decimation = display_decimation(presentation.large_dataset_display());

    // Right (phase) axis when this pane hosts phase traces.
    let has_phase = !pane.right.is_empty();
    if has_phase
        && let Some((p0, p1)) = pane_y_range(&mut state.ui.results.derived, model, &pane.right)
    {
        let displays_radians = pane
            .right
            .iter()
            .filter_map(|index| model.traces.get(*index))
            .any(|trace| trace.kind == TraceKind::PhaseRad);
        let axis = match (view.y_right, displays_radians) {
            (Some((z0, z1)), true) => Axis::linear_with(z0, z1, "rad", 5),
            (None, true) => Axis::linear_with(p0, p1, "rad", 5),
            // Zoomed degree axes use plain linear ticks; the 45° lattice
            // would be too dense at arbitrary zoom depths.
            (Some((z0, z1)), false) => Axis::linear_with(z0, z1, "°", 5),
            (None, false) => {
                let p0 = (p0 / 45.0).floor() * 45.0;
                let p1 = (p1 / 45.0).ceil() * 45.0;
                let ticks: Vec<f64> = (0..=((p1 - p0) / 45.0) as i64)
                    .map(|i| p0 + i as f64 * 45.0)
                    .collect();
                Axis::with_ticks(p0, p1, "°", &ticks)
            }
        };
        let axis = if displays_radians {
            match quantity_policy.angle_display {
                crate::quantity::AngleDisplay::Degrees => {
                    axis.with_display_transform(180.0 / std::f64::consts::PI, 0.0, "°")
                }
                crate::quantity::AngleDisplay::Radians => axis,
            }
        } else {
            let (scale, offset, unit) = quantity_policy.degree_axis_transform();
            axis.with_display_transform(scale, offset, unit)
        };
        spec.y_right = Some((axis, t.color.traces[2]));
    }
    // 0 dB reference on a log-magnitude pane.
    if pane.unit == "dB" && y0 < 0.0 && y1 > 0.0 {
        spec.ref_lines.push(plot::RefLine { y: 0.0 });
    }

    // Run owns weight: overlay traces keep the signal hue at reduced alpha
    // and stroke, painted first so the active run draws at full strength
    // on top.
    let pane_indices = pane.traces.iter().chain(pane.right.iter()).copied();
    let pane_traces: Vec<(usize, &StripTrace)> = pane_indices
        .filter_map(|index| model.traces.get(index).map(|trace| (index, trace)))
        .collect();
    let draw_order = pane_traces
        .iter()
        .filter(|(_, trace)| trace.overlay)
        .chain(pane_traces.iter().filter(|(_, trace)| !trace.overlay));
    for (index, trace) in draw_order {
        let color = if trace.overlay {
            trace.color.gamma_multiply(0.40)
        } else {
            trace.color
        };
        let mut plot_trace = apply_family_trace_style(
            Trace::new(&trace.x, &trace.y, color).cache_key(trace_key(model, trace)),
            trace.family_style,
        );
        if trace.overlay {
            plot_trace = plot_trace.thin();
        }
        if pane.right.contains(index) {
            plot_trace = plot_trace.right().dashed();
        }
        spec.traces.push(plot_trace);
    }
    for expr in exprs {
        spec.traces.push(apply_family_trace_style(
            Trace::new(&expr.x, &expr.y, expr.color)
                .thin()
                .cache_key(expr.cache_key),
            expr.family_style,
        ));
    }

    // Markers ride their anchored trace: Y is resampled here rather than
    // stored, so zoom, pan, and a re-run all leave the tag on the curve.
    for marker in state.ui.results.strip_markers(model.analysis_index) {
        let color = marker_color(marker.kind, &t);
        let label = marker_label(marker);
        if marker.kind == MarkerKind::Spec {
            // A spec constrains the X position, which every pane of the
            // strip shares — so it draws on all of them.
            spec.markers
                .push(plot::Marker::limit_line(marker.x, color, label));
            continue;
        }
        // A marker belongs to the pane that owns its trace's unit; the
        // other panes are a different scale and would misplace it.
        let anchored = pane_traces
            .iter()
            .find(|(_, trace)| !trace.overlay && anchor_key(model, trace) == marker.anchor);
        let Some((index, trace)) = anchored else {
            continue;
        };
        let y = sample_at_with(&trace.x, &trace.y, marker.x, interpolation);
        if !y.is_finite() {
            continue;
        }
        let mut plot_marker = plot::Marker::point(marker.x, y, color, label);
        if pane.right.contains(index) {
            plot_marker.side = plot::YSide::Right;
        }
        spec.markers.push(plot_marker);
    }

    let model_cursor_domain = model.cursor_domain();
    let cursor_domain_matches = linked_cursor_domain == Some(&model_cursor_domain);
    let cursors = (state.ui.results.cursor_strip == Some(model.analysis_index)
        || (state.ui.results.linked_cursors && cursor_domain_matches))
        .then_some(state.ui.results.cursors);

    let readout = |x: f64| -> Vec<(String, String)> {
        let mut rows = vec![(
            model.x_label().to_owned(),
            model.format_x(x, significant_digits, quantity_policy),
        )];
        for (_, trace) in pane_traces.iter().take(6) {
            let value = sample_at_with(&trace.x, &trace.y, x, interpolation);
            rows.push((
                trace.name.clone(),
                model.format_trace_value(trace, value, significant_digits, quantity_policy),
            ));
        }
        for expr in exprs.iter().take(3) {
            let value = sample_at_with(&expr.x, &expr.y, x, interpolation);
            rows.push((
                expr.label.clone(),
                fmt_si_significant(value, "", significant_digits),
            ));
        }
        rows
    };

    let response = plot::show(
        ui,
        &spec,
        &mut state.ui.results.cache,
        cursors.as_ref(),
        Some(&readout),
    );

    // The marker tool takes the click when armed: one click cannot both
    // annotate and move a cursor, and the armed chip says which it will do.
    if let Some(clicked_x) = response.clicked_x
        && state.ui.results.marker_tool.is_armed()
    {
        let right_range = spec.y_right.as_ref().map(|(axis, _)| (axis.min, axis.max));
        let pointer_y = response.response.interact_pointer_pos().map(|pos| pos.y);
        let plot_rect = response.plot_rect;
        // "Nearest" has to mean what the eye sees, so each trace is measured
        // in screen space against the axis it actually draws on.
        let screen_y = |value: f64, right: bool| -> Option<f32> {
            let (lo, hi) = if right { right_range? } else { (y0, y1) };
            (value.is_finite() && hi > lo).then(|| {
                plot_rect.bottom() - ((value - lo) / (hi - lo)) as f32 * plot_rect.height()
            })
        };
        let nearest = pane_traces
            .iter()
            .filter(|(_, trace)| !trace.overlay)
            .filter_map(|(index, trace)| {
                let value = sample_at_with(&trace.x, &trace.y, clicked_x, interpolation);
                let y = screen_y(value, pane.right.contains(index))?;
                Some((trace, pointer_y.map_or(0.0, |pointer| (pointer - y).abs())))
            })
            .min_by(|(_, a), (_, b)| a.total_cmp(b));
        if let Some((trace, _)) = nearest {
            let anchor = anchor_key(model, trace);
            let name = trace.name.clone();
            let id = state
                .ui
                .results
                .add_marker(model.analysis_index, anchor, name, clicked_x);
            // Focus the new marker's note field: placing one is normally the
            // first half of saying what it means.
            state.ui.results.editing_marker = Some(id);
        }
    } else if let Some(clicked_x) = response.clicked_x
        && state.ui.results.cursor_tool.is_armed()
    {
        let results = &mut state.ui.results;
        if results.cursor_strip != Some(model.analysis_index)
            && (!results.linked_cursors || !cursor_domain_matches)
        {
            results.cursors = CursorPair::default();
        }
        results.cursor_strip = Some(model.analysis_index);
        results.cursors.place(clicked_x);
    }

    if response.view.any() {
        state
            .ui
            .results
            .plot_view_pane_mut(super::ResultViewer::Waves, model.analysis_index, ordinal)
            .apply(&response.view);
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// readout strip
// ---------------------------------------------------------------------------



#[cfg(test)]
mod tests;
