//! WAVES — stacked waveform strips, one per analysis in the active run.
//!
//! Each strip carries its analysis' traces with the strip grammar (header ·
//! legend · actions over a document well). AC strips convert magnitude to dB
//! on the left axis and route phase traces, dashed, to a right axis. A/B
//! cursors live on one strip at a time; their values, deltas and windowed
//! measurements render in the right panel.

use std::collections::HashSet;
use std::sync::Arc;

use egui::Ui;

use crate::analysis::calculator;
use crate::common::AppState;
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
use crate::workbench::visualization_family::{
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
                crate::workbench::commands::Command::RunSimulation,
                crate::common::app::runtime_command_platform(ui.ctx()),
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

/// Palette color for the i-th trace slot of a strip (waveforms, then
/// expressions).
fn expr_color(tokens: &Tokens, slot: usize) -> egui::Color32 {
    tokens.color.traces[slot % tokens.color.traces.len()]
}

const EXPR_EDITOR_PADDING_X: f32 = 10.0;
const EXPR_EDITOR_PADDING_Y: f32 = 5.0;
const EXPR_EDITOR_GAP: f32 = 8.0;
const EXPR_EDITOR_COMPACT_WIDTH: f32 = 560.0;
const EXPR_EDITOR_MIN_INLINE_INPUT: f32 = 160.0;
const EXPR_EDITOR_ERROR_HEIGHT: f32 = 20.0;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct EditorSpan {
    start: f32,
    width: f32,
}

impl EditorSpan {
    fn end(self) -> f32 {
        self.start + self.width
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct ExprEditorLayout {
    label: EditorSpan,
    input: EditorSpan,
    error: EditorSpan,
    add: EditorSpan,
    stack_error: bool,
}

fn expr_editor_layout(
    available_width: f32,
    label_natural_width: f32,
    add_natural_width: f32,
    error_natural_width: Option<f32>,
) -> ExprEditorLayout {
    let available_width = available_width.max(0.0);
    // Reserve the commit action from the right edge before allocating any
    // free-form text. This is the invariant the former fixed 340 px input
    // violated on phone-sized strips.
    let add_width = add_natural_width.max(0.0).min(available_width);
    let add = EditorSpan {
        start: available_width - add_width,
        width: add_width,
    };
    let before_add = if add_width > 0.0 {
        (add.start - EXPR_EDITOR_GAP).max(0.0)
    } else {
        available_width
    };
    let label_width = label_natural_width.max(0.0).min(before_add);
    let label = EditorSpan {
        start: 0.0,
        width: label_width,
    };
    let input_start = if label_width > 0.0 {
        (label.end() + EXPR_EDITOR_GAP).min(before_add)
    } else {
        0.0
    };

    let stack_error = error_natural_width.is_some()
        && (available_width <= EXPR_EDITOR_COMPACT_WIDTH
            || before_add - input_start < EXPR_EDITOR_MIN_INLINE_INPUT + 88.0);
    let mut input_end = before_add;
    let mut error = EditorSpan::default();
    if let Some(error_natural_width) = error_natural_width.filter(|_| !stack_error) {
        let error_budget =
            (before_add - input_start - EXPR_EDITOR_MIN_INLINE_INPUT - EXPR_EDITOR_GAP).max(0.0);
        let error_width = error_natural_width
            .max(0.0)
            .min(available_width * 0.28)
            .min(error_budget);
        if error_width > 0.0 {
            error = EditorSpan {
                start: before_add - error_width,
                width: error_width,
            };
            input_end = (error.start - EXPR_EDITOR_GAP).max(input_start);
        }
    }

    ExprEditorLayout {
        label,
        input: EditorSpan {
            start: input_start,
            width: (input_end - input_start).max(0.0),
        },
        error,
        add,
        stack_error,
    }
}

/// The inline expression editor row under a strip header (when open for
/// this strip): mono input, Enter/Add commits, Esc closes, and a bounded
/// validation message that moves below the controls on compact surfaces.
fn expr_editor_row(ui: &mut Ui, state: &mut AppState, analysis_index: usize) {
    let Some(editor) = state
        .ui
        .results
        .expr_editor
        .as_mut()
        .filter(|editor| editor.analysis_index == analysis_index)
    else {
        return;
    };

    let t = Tokens::get(ui.ctx());
    let c = t.color;

    enum Action {
        None,
        Commit,
        Cancel,
    }
    let mut action = Action::None;

    let label_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let input_font = theme::mono(tokens::FS_1, FontWeight::Regular);
    let error_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let label_width = ui
        .painter()
        .layout_no_wrap("expr".to_owned(), label_font.clone(), c.text_dim)
        .size()
        .x;
    let add_width = ui
        .painter()
        .layout_no_wrap(
            "Add".to_owned(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            c.text,
        )
        .size()
        .x
        + 20.0;
    let error_width = editor.error.as_ref().map(|error| {
        ui.painter()
            .layout_no_wrap(error.clone(), error_font.clone(), c.err)
            .size()
            .x
    });
    let inner_width = (ui.available_width() - 2.0 * EXPR_EDITOR_PADDING_X).max(0.0);
    let layout = expr_editor_layout(inner_width, label_width, add_width, error_width);
    let control_row_height = t.metrics.ctl_h + 2.0 * EXPR_EDITOR_PADDING_Y;
    let total_height = control_row_height
        + if layout.stack_error {
            EXPR_EDITOR_ERROR_HEIGHT
        } else {
            0.0
        };
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), total_height),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, c.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border),
    );

    let control_rect = egui::Rect::from_min_size(
        egui::pos2(
            rect.left() + EXPR_EDITOR_PADDING_X,
            rect.top() + EXPR_EDITOR_PADDING_Y,
        ),
        egui::vec2(inner_width, t.metrics.ctl_h),
    );
    let span_rect = |span: EditorSpan| {
        egui::Rect::from_min_max(
            egui::pos2(control_rect.left() + span.start, control_rect.top()),
            egui::pos2(control_rect.left() + span.end(), control_rect.bottom()),
        )
    };
    let label_rect = span_rect(layout.label);
    ui.painter().with_clip_rect(label_rect).text(
        label_rect.left_center(),
        egui::Align2::LEFT_CENTER,
        "expr",
        label_font,
        c.text_dim,
    );

    let input_rect = span_rect(layout.input);
    let response = ui
        .scope_builder(
            egui::UiBuilder::new()
                .max_rect(input_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
            |ui| {
                ui.set_clip_rect(input_rect);
                ui.add_sized(
                    input_rect.size(),
                    egui::TextEdit::singleline(&mut editor.text)
                        .font(input_font)
                        .hint_text("V(out)/V(in) - dB(V(out)) - deriv(V(out))")
                        .desired_width(input_rect.width()),
                )
            },
        )
        .inner;
    if editor.want_focus {
        response.request_focus();
        editor.want_focus = false;
    }
    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
        action = Action::Commit;
    }
    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
        action = Action::Cancel;
    }
    let add_rect = span_rect(layout.add);
    let add_clicked = ui
        .scope_builder(
            egui::UiBuilder::new()
                .max_rect(add_rect)
                .layout(egui::Layout::right_to_left(egui::Align::Center)),
            |ui| {
                ui.set_clip_rect(add_rect);
                crate::ui::widgets::Button::new("Add")
                    .min_width(add_rect.width())
                    .max_width(add_rect.width())
                    .show(ui)
            },
        )
        .inner
        .clicked();
    if add_clicked {
        action = Action::Commit;
    }
    if let Some(error) = &editor.error {
        let error_rect = if layout.stack_error {
            egui::Rect::from_min_max(
                egui::pos2(
                    control_rect.left() + layout.input.start,
                    rect.top() + control_row_height,
                ),
                egui::pos2(control_rect.right(), rect.bottom()),
            )
        } else {
            span_rect(layout.error)
        };
        if error_rect.width() > 0.0 {
            ui.scope_builder(
                egui::UiBuilder::new()
                    .max_rect(error_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
                |ui| {
                    ui.set_clip_rect(error_rect);
                    ui.add_sized(
                        error_rect.size(),
                        egui::Label::new(egui::RichText::new(error).font(error_font).color(c.err))
                            .truncate(),
                    )
                    .on_hover_text(error);
                },
            );
        }
    }

    match action {
        Action::None => {}
        Action::Cancel => state.ui.results.expr_editor = None,
        Action::Commit => {
            let text = state
                .ui
                .results
                .expr_editor
                .as_ref()
                .map(|e| e.text.trim().to_owned())
                .unwrap_or_default();
            if text.is_empty() {
                state.ui.results.expr_editor = None;
                return;
            }
            let sample_selection = state.ui.results.sample_selection.clone();
            let (series, extremes) = evaluate_expression(
                &state.simulation,
                analysis_index,
                &text,
                sample_selection.as_ref(),
            );
            match series {
                Ok(series) => {
                    state.ui.results.expr_cache.insert(
                        (analysis_index, text.clone()),
                        ExprSeries {
                            version: expression_version(
                                state.simulation.data_version,
                                sample_selection.as_ref(),
                            ),
                            series: Ok(series),
                            y_extremes: extremes,
                        },
                    );
                    state
                        .ui
                        .results
                        .exprs
                        .entry(analysis_index)
                        .or_default()
                        .push(ExprTrace {
                            text,
                            visible: true,
                        });
                    state.ui.results.expr_editor = None;
                }
                Err(error) => {
                    if let Some(editor) = state.ui.results.expr_editor.as_mut() {
                        editor.error = Some(error);
                        editor.want_focus = true;
                    }
                }
            }
        }
    }
}

type ExpressionEvaluation = (WaveformSeriesResult, Option<(f64, f64)>);

/// Evaluate one expression against an analysis' waveforms. Scalars become a
/// constant trace across the analysis' x span.
fn evaluate_expression(
    simulation: &SimulationState,
    analysis_index: usize,
    text: &str,
    selection: Option<&SourceSampleSelection>,
) -> ExpressionEvaluation {
    let Some(run) = simulation.active_run() else {
        return (Err("analysis no longer exists".to_owned()), None);
    };
    let Some(analysis) = run.analyses.get(analysis_index) else {
        return (Err("analysis no longer exists".to_owned()), None);
    };
    let selection = selection.filter(|selection| {
        selection.dataset_id == run.dataset_id && selection.analysis_sequence == analysis.id
    });

    let ctx = calculator::WaveformsContext::new(&analysis.waveforms);
    let expr = match calculator::parser::try_parse(text) {
        Ok(expr) => expr,
        Err(error) => return (Err(format!("parse error: {error}")), None),
    };
    match calculator::evaluator::evaluate(&expr, &ctx) {
        Ok(calculator::CalcValue::Waveform(x, y)) if !x.is_empty() => {
            let (x, y) =
                match selection {
                    None => (x, y),
                    Some(selection)
                        if x.len() == y.len()
                            && selection
                                .source_indices
                                .last()
                                .is_none_or(|index| *index < x.len()) =>
                    {
                        (
                            selection
                                .source_indices
                                .iter()
                                .map(|index| x[*index])
                                .collect(),
                            selection
                                .source_indices
                                .iter()
                                .map(|index| y[*index])
                                .collect(),
                        )
                    }
                    Some(_) => {
                        return (
                        Err("expression sample count does not match the retained family manifest"
                            .to_owned()),
                        None,
                    );
                    }
                };
            let extremes = super::finite_extremes(&y);
            (Ok((x.into(), y.into())), extremes)
        }
        Ok(calculator::CalcValue::Waveform(..)) => {
            (Err("expression produced no samples".to_owned()), None)
        }
        Ok(calculator::CalcValue::Scalar(value)) => {
            if let Some(selection) = selection {
                let selected_x = analysis.waveforms.first().and_then(|waveform| {
                    selected_series_pair(&waveform.x, &waveform.y, Some(selection)).map(|(x, _)| x)
                });
                return match selected_x {
                    Some(x) if !x.is_empty() => {
                        let y = vec![value; x.len()];
                        (Ok((x, y.into())), Some((value, value)))
                    }
                    _ => (
                        Err("scalar result with no selected X rows".to_owned()),
                        None,
                    ),
                };
            }
            let span = analysis.waveforms.first().and_then(|waveform| {
                let (x, _) = selected_series_pair(&waveform.x, &waveform.y, selection)?;
                (x.len() >= 2).then(|| (x[0], x[x.len() - 1]))
            });
            match span {
                Some((x0, x1)) => (
                    Ok((vec![x0, x1].into(), vec![value, value].into())),
                    Some((value, value)),
                ),
                None => (Err("scalar result with no x span".to_owned()), None),
            }
        }
        Err(error) => (Err(error.to_string()), None),
    }
}

fn expression_version(data_version: u64, selection: Option<&SourceSampleSelection>) -> u64 {
    data_version
        ^ selection
            .map(SourceSampleSelection::fingerprint)
            .unwrap_or_default()
            .rotate_left(23)
}

/// One expression trace resolved for plotting.
struct ResolvedExpr {
    x: SharedWaveformValues,
    y: SharedWaveformValues,
    color: egui::Color32,
    cache_key: u64,
    label: String,
    y_extremes: Option<(f64, f64)>,
    family_style: Option<FamilyTraceStyle>,
}

/// Refresh the expression cache for a strip at the current data version and
/// hand back plottable series (visible expressions, successful evaluations).
fn resolve_strip_exprs(
    state: &mut AppState,
    model: &StripModel,
    tokens: &Tokens,
) -> Vec<ResolvedExpr> {
    let exprs: Vec<(usize, ExprTrace)> = state
        .ui
        .results
        .exprs
        .get(&model.analysis_index)
        .map(|list| list.iter().cloned().enumerate().collect())
        .unwrap_or_default();
    if exprs.is_empty() {
        return Vec::new();
    }

    let sample_selection = state.ui.results.sample_selection.clone();
    let version = expression_version(state.simulation.data_version, sample_selection.as_ref());
    let mut resolved = Vec::new();
    for (slot, expr) in exprs {
        let key = (model.analysis_index, expr.text.clone());
        let fresh = state
            .ui
            .results
            .expr_cache
            .get(&key)
            .is_some_and(|s| s.version == version);
        if !fresh {
            let (series, extremes) = evaluate_expression(
                &state.simulation,
                model.analysis_index,
                &expr.text,
                sample_selection.as_ref(),
            );
            if let Err(error) = &series {
                state.push_user_message(crate::common::app::ConsoleMessage::warning(format!(
                    "expression `{}`: {}",
                    expr.text, error
                )));
            }
            state.ui.results.expr_cache.insert(
                key.clone(),
                ExprSeries {
                    version,
                    series,
                    y_extremes: extremes,
                },
            );
        }
        if !expr.visible {
            continue;
        }
        let cached = state.ui.results.expr_cache.get(&key).and_then(|cached| {
            cached
                .series
                .as_ref()
                .ok()
                .map(|(x, y)| (Arc::clone(x), Arc::clone(y)))
        });
        let Some((x, y)) = cached else {
            continue;
        };
        let Some(projections) = projected_selected_family_series(&x, &y, sample_selection.as_ref())
        else {
            state.push_user_message(crate::common::app::ConsoleMessage::warning(format!(
                "expression `{}`: selected rows do not match the active family render plan",
                expr.text
            )));
            continue;
        };
        let base_color = expr_color(tokens, model.traces.len() + slot);
        let base_cache_key = expr_cache_key(model.analysis_index, &expr.text);
        let base_label = elide(&expr.text, 24);
        for projection in projections {
            let family_style = projection.group.map(|group| group.style);
            resolved.push(ResolvedExpr {
                x: projection.x,
                y_extremes: super::finite_extremes(&projection.y),
                y: projection.y,
                color: family_style.map_or(base_color, |style| family_color(style, base_color)),
                cache_key: base_cache_key
                    ^ projection
                        .group
                        .map_or(0, |group| group.stable_key.rotate_left(19)),
                label: projection.group.map_or_else(
                    || base_label.clone(),
                    |group| format!("{base_label} · {}", group.label),
                ),
                family_style,
            });
        }
    }
    resolved
}

/// Stable decimation-cache identity for an expression trace. The high bit
/// keeps it out of the waveform trace_key space.
fn expr_cache_key(analysis_index: usize, text: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    (analysis_index, text).hash(&mut hasher);
    hasher.finish() | (1 << 63)
}

/// Flip a waveform's visibility on the run, keeping the live copy in sync.
pub(crate) fn toggle_visibility(
    state: &mut AppState,
    analysis_index: usize,
    waveform_index: usize,
) {
    let Some(run_idx) = state.simulation.active_run_idx else {
        return;
    };
    let mut name: Option<String> = None;
    if let Some(waveform) = state
        .simulation
        .runs
        .get_mut(run_idx)
        .and_then(|run| run.analyses.get_mut(analysis_index))
        .and_then(|analysis| analysis.waveforms.get_mut(waveform_index))
    {
        waveform.visible = !waveform.visible;
        name = Some(waveform.name.clone());
    }
    // Mirror into the live waveform list when this is the active analysis.
    if state.simulation.active_analysis_idx == Some(analysis_index)
        && let Some(name) = name
        && let Some(live) = state
            .simulation
            .waveforms
            .iter_mut()
            .find(|w| w.name == name)
    {
        live.visible = !live.visible;
    }
}

/// Serialize the active Waves cursor readout for the platform clipboard.
/// This is the Edit → Copy consumer for the Units copied-value policy.
pub(crate) fn copy_cursor_text(state: &mut AppState) -> Option<String> {
    let x = state.ui.results.cursors.a?;
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    let sample_selection = state.ui.results.sample_selection.clone();
    let hidden_family_traces = state.ui.results.hidden_family_traces.clone();
    let models = build_models(
        &state.simulation,
        &mut state.ui.results.derived,
        &Tokens::default(),
        state.ui.results.phase_continuous,
        presentation.complex_number_display(),
        sample_selection.as_ref(),
        &hidden_family_traces,
    );
    let model = state
        .ui
        .results
        .cursor_strip
        .and_then(|index| models.iter().find(|model| model.analysis_index == index))?;

    let mut text = String::new();
    append_copied_cursor(&mut text, "A", x, model, interpolation, quantity_policy);
    if let Some(b) = state.ui.results.cursors.b {
        text.push('\n');
        append_copied_cursor(&mut text, "B", b, model, interpolation, quantity_policy);
    }
    Some(text)
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

/// Height of the cursor readout strip's header row.
const READOUT_HEADER_H: f32 = 26.0;
/// Height of one per-trace row in the readout strip.
const READOUT_ROW_H: f32 = 20.0;
/// Most trace rows the strip will show before it stops growing.
const READOUT_TRACE_LIMIT: usize = 4;
const READOUT_PAD_X: f32 = 10.0;

/// Traces the readout strip will report for the cursor's strip.
fn readout_trace_count(state: &AppState) -> usize {
    let Some(index) = state.ui.results.cursor_strip else {
        return 0;
    };
    state
        .simulation
        .active_run()
        .and_then(|run| run.analyses.get(index))
        .map_or(0, |analysis| {
            analysis
                .waveforms
                .iter()
                .filter(|waveform| waveform.visible)
                .count()
                .min(READOUT_TRACE_LIMIT)
        })
}

/// Height of one marker row.
const MARKER_ROW_H: f32 = 22.0;
/// Most marker rows the strip will show before it stops growing.
const MARKER_ROW_LIMIT: usize = 4;

/// Analysis indices whose strips are on screen right now.
///
/// A marker on a closed or un-maximized strip has nothing to point at, so
/// it must not hold the readout strip open.
fn on_screen_strips(state: &AppState) -> Vec<usize> {
    let Some(run) = state.simulation.active_run() else {
        return Vec::new();
    };
    let results = &state.ui.results;
    let present = |index: usize| index < run.analyses.len();
    match results.maximized_strip {
        Some(max_index) if present(max_index) => vec![max_index],
        _ => (0..run.analyses.len())
            .filter(|index| !results.hidden_strips.contains(index))
            .collect(),
    }
}

/// Markers the strip will list, in placement order.
fn visible_markers(state: &AppState) -> Vec<&ResultMarker> {
    let strips = on_screen_strips(state);
    state
        .ui
        .results
        .markers
        .iter()
        .filter(|marker| strips.contains(&marker.analysis_index))
        .collect()
}

/// Exact height the readout strip needs, or zero when it stands down.
///
/// The strip is content-fit by design: it is a readout, not a dock, so it
/// never reserves space for rows it has nothing to put in. Three states are
/// reachable — the full cursor readout, a markers-only strip when markers
/// exist without cursors, and no strip at all.
pub fn readout_strip_height(state: &AppState) -> f32 {
    let mut height = 0.0;
    if state.ui.results.cursor_readout_active() {
        height += READOUT_HEADER_H + readout_trace_count(state) as f32 * READOUT_ROW_H;
    }
    let markers = visible_markers(state).len();
    if markers > 0 {
        height += READOUT_HEADER_H + markers.min(MARKER_ROW_LIMIT) as f32 * MARKER_ROW_H;
    }
    height
}

/// The cursor readout: one X row naming A, B and Δ, then the value each
/// visible trace takes at those cursors.
///
/// This is the single home for the cursor readout. The inspector reports
/// window statistics the strip does not carry, and never repeats these
/// numbers one panel away.
pub fn readout_strip(ui: &mut Ui, state: &mut AppState, height: f32) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, c.bg_panel);
    ui.painter()
        .hline(rect.x_range(), rect.top(), egui::Stroke::new(1.0, c.border));

    let cursor_bottom = cursor_readout_section(ui, state, rect);
    let marker_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left(), cursor_bottom),
        egui::pos2(rect.right(), rect.bottom()),
    );
    if marker_rect.height() > 1.0 {
        if cursor_bottom > rect.top() {
            ui.painter().hline(
                rect.x_range(),
                cursor_bottom,
                egui::Stroke::new(1.0, c.border),
            );
        }
        marker_section(ui, state, marker_rect);
    }
}

/// The A/B half of the strip. Returns the Y the marker section starts at,
/// which equals the strip top when there is no cursor readout to draw.
fn cursor_readout_section(ui: &mut Ui, state: &mut AppState, rect: egui::Rect) -> f32 {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    if !state.ui.results.cursor_readout_active() {
        return rect.top();
    }
    // Read the reserved height from the same source as `readout_strip_height`
    // so the marker section can never start off the end of the strip.
    let section_bottom =
        rect.top() + READOUT_HEADER_H + readout_trace_count(state) as f32 * READOUT_ROW_H;
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &t,
    );
    let Some(model) = state
        .ui
        .results
        .cursor_strip
        .and_then(|index| models.iter().find(|model| model.analysis_index == index))
    else {
        return rect.top();
    };
    let cursors = state.ui.results.cursors;
    let Some(a) = cursors.a else {
        return rect.top();
    };

    // Header: the X positions and their separation.
    let header = egui::Rect::from_min_max(
        rect.min,
        egui::pos2(rect.right(), rect.top() + READOUT_HEADER_H),
    );
    let painter = ui.painter().with_clip_rect(header);
    let mut x = header.left() + READOUT_PAD_X;
    let mut chip = |text: String, color: egui::Color32, painter: &egui::Painter| {
        let galley =
            painter.layout_no_wrap(text, theme::mono(tokens::FS_0, FontWeight::Regular), color);
        painter.galley(
            egui::pos2(x, header.center().y - galley.size().y * 0.5),
            galley.clone(),
            color,
        );
        x += galley.size().x + 14.0;
    };
    chip("CURSORS".to_owned(), c.text_faint, &painter);
    chip(
        format!(
            "A  {} = {}",
            model.x_label(),
            model.format_x(a, significant_digits, quantity_policy)
        ),
        c.accent,
        &painter,
    );
    if let Some(b) = cursors.b {
        chip(
            format!(
                "B  {} = {}",
                model.x_label(),
                model.format_x(b, significant_digits, quantity_policy)
            ),
            c.traces[4],
            &painter,
        );
        chip(
            format!(
                "Δ  {}",
                x_separation(model, a, b, significant_digits, quantity_policy)
            ),
            c.text,
            &painter,
        );
        if let Some(slope) = slope_between(model, a, b, presentation) {
            chip(format!("slope  {slope}"), c.text_dim, &painter);
        }
    } else {
        chip("click again to place B".to_owned(), c.text_faint, &painter);
    }

    // Per-trace values at A, at B, and their difference.
    let rows = value_rows(model, a, presentation, quantity_policy);
    let b_rows = cursors
        .b
        .map(|b| value_rows(model, b, presentation, quantity_policy));
    let deltas = cursors
        .b
        .map(|b| delta_values(model, a, b, presentation, quantity_policy));
    let name_column = (rect.width() * 0.28).clamp(80.0, 220.0);
    let value_column = ((rect.width() - name_column - READOUT_PAD_X * 2.0) / 3.0).max(1.0);
    for (index, (name, a_value)) in rows.iter().take(READOUT_TRACE_LIMIT).enumerate() {
        let top = header.bottom() + index as f32 * READOUT_ROW_H;
        let row = egui::Rect::from_min_max(
            egui::pos2(rect.left(), top),
            egui::pos2(rect.right(), top + READOUT_ROW_H),
        );
        let painter = ui.painter().with_clip_rect(row);
        painter.text(
            egui::pos2(row.left() + READOUT_PAD_X, row.center().y),
            egui::Align2::LEFT_CENTER,
            name,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text_dim,
        );
        let mut column = row.left() + READOUT_PAD_X + name_column;
        let mut cell = |text: &str, color: egui::Color32| {
            painter.text(
                egui::pos2(column, row.center().y),
                egui::Align2::LEFT_CENTER,
                text,
                theme::mono(tokens::FS_0, FontWeight::Regular),
                color,
            );
            column += value_column;
        };
        cell(a_value, c.text);
        if let Some(b_rows) = b_rows.as_ref() {
            cell(
                b_rows.get(index).map_or("", |(_, value)| value.as_str()),
                c.text,
            );
        }
        if let Some(deltas) = deltas.as_ref() {
            cell(deltas.get(index).map_or("", String::as_str), c.text_dim);
        }
    }
    section_bottom
}

/// The marker half of the strip: one editable row per marker.
///
/// Markers are document content, so their row is the place they are named,
/// re-kinded and removed — there is no second marker list elsewhere to
/// disagree with this one.
fn marker_section(ui: &mut Ui, state: &mut AppState, rect: egui::Rect) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &t,
    );

    let shown: Vec<u32> = visible_markers(state)
        .into_iter()
        .take(MARKER_ROW_LIMIT)
        .map(|marker| marker.id)
        .collect();
    let total = visible_markers(state).len();
    if shown.is_empty() {
        return;
    }

    let header = egui::Rect::from_min_max(
        rect.min,
        egui::pos2(rect.right(), rect.top() + READOUT_HEADER_H),
    );
    let painter = ui.painter().with_clip_rect(header);
    painter.text(
        egui::pos2(header.left() + READOUT_PAD_X, header.center().y),
        egui::Align2::LEFT_CENTER,
        "MARKERS",
        theme::mono(tokens::FS_0, FontWeight::Regular),
        c.text_faint,
    );
    if total > shown.len() {
        painter.text(
            egui::pos2(header.right() - READOUT_PAD_X, header.center().y),
            egui::Align2::RIGHT_CENTER,
            format!("{} of {total}", shown.len()),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text_faint,
        );
    }

    let mut remove: Option<u32> = None;
    for (index, id) in shown.iter().copied().enumerate() {
        let top = header.bottom() + index as f32 * MARKER_ROW_H;
        let row = egui::Rect::from_min_max(
            egui::pos2(rect.left() + READOUT_PAD_X, top),
            egui::pos2(rect.right() - READOUT_PAD_X, top + MARKER_ROW_H),
        );
        // Everything the row reports is derived here, from the same model
        // the plot drew, so a row can never describe a marker the plot
        // placed somewhere else.
        let Some(marker) = state.ui.results.markers.iter().find(|m| m.id == id) else {
            continue;
        };
        let kind = marker.kind;
        let anchor = marker.anchor;
        let marker_x = marker.x;
        let analysis_index = marker.analysis_index;
        let trace_name = marker.trace_name.clone();
        let model = models
            .iter()
            .find(|model| model.analysis_index == analysis_index);
        let position = model.map_or_else(
            || fmt_si_significant(marker_x, "", significant_digits),
            |model| {
                format!(
                    "{} = {}",
                    model.x_label(),
                    model.format_x(marker_x, significant_digits, quantity_policy)
                )
            },
        );
        // A spec marker constrains the X position alone; reporting a curve
        // value against it would assert a reading it does not make.
        let value = kind.rides_a_trace().then(|| {
            model
                .and_then(|model| {
                    let trace = model
                        .traces
                        .iter()
                        .find(|trace| !trace.overlay && anchor_key(model, trace) == anchor)?;
                    let sampled = sample_at_with(&trace.x, &trace.y, marker_x, interpolation);
                    Some(model.format_trace_value(
                        trace,
                        sampled,
                        significant_digits,
                        quantity_policy,
                    ))
                })
                .unwrap_or_else(|| "trace unavailable".to_owned())
        });

        let mut row_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(row)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        row_ui.set_clip_rect(row);
        row_ui.spacing_mut().item_spacing.x = 8.0;
        let color = marker_color(kind, &t);
        row_ui.label(
            egui::RichText::new(format!("M{id}"))
                .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                .color(color),
        );
        if chip(&mut row_ui, kind.label(), false)
            .on_hover_text("Cycle marker kind: note → peak → spec")
            .clicked()
            && let Some(marker) = state.ui.results.marker_mut(id)
        {
            marker.kind = kind.next();
        }
        if row_ui
            .add(
                egui::Button::new(
                    egui::RichText::new("×")
                        .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                        .color(c.text_dim),
                )
                .frame(false),
            )
            .on_hover_text("Remove this marker")
            .clicked()
        {
            remove = Some(id);
        }
        row_ui.label(
            egui::RichText::new(trace_name)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(if kind.rides_a_trace() {
                    c.text_dim
                } else {
                    c.text_faint
                }),
        );
        row_ui.label(
            egui::RichText::new(position)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(c.text),
        );
        if let Some(value) = value {
            row_ui.label(
                egui::RichText::new(value)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(c.text),
            );
        }
        // The note field takes what is left of the row.
        let note_width = row_ui.available_width().max(60.0);
        let mut note = state
            .ui
            .results
            .markers
            .iter()
            .find(|m| m.id == id)
            .map_or_else(String::new, |m| m.note.clone());
        let response = row_ui.add(
            egui::TextEdit::singleline(&mut note)
                .desired_width(note_width)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .hint_text("note…"),
        );
        if state.ui.results.editing_marker == Some(id) {
            response.request_focus();
            state.ui.results.editing_marker = None;
        }
        if response.changed()
            && let Some(marker) = state.ui.results.marker_mut(id)
        {
            marker.note = note;
        }
    }
    if let Some(id) = remove {
        state.ui.results.remove_marker(id);
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Window statistics over the cursor span.
///
/// The A/B/Δ readout itself lives in the stage's readout strip; repeating it
/// one panel away is what the results de-duplication pass removed.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());

    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &t,
    );
    let cursor_model = state
        .ui
        .results
        .cursor_strip
        .and_then(|idx| models.iter().find(|m| m.analysis_index == idx));

    // Statistics over the cursor window (or the full range).
    let cursors = state.ui.results.cursors;
    let measured_model = cursor_model.or_else(|| models.first());
    if let Some(model) = measured_model {
        let window = match (cursors.a, cursors.b) {
            (Some(a), Some(b)) => Some((a.min(b), a.max(b))),
            _ => None,
        };
        let title = if window.is_some() {
            "Measurements · A–B"
        } else {
            "Measurements"
        };
        section_header(ui, title, None);
        measurement_rows(
            ui,
            &mut state.ui.results.derived,
            model,
            window,
            significant_digits,
            quantity_policy,
        );
    }
}

fn value_rows(
    model: &StripModel,
    x: f64,
    presentation: ResultPresentationPolicy,
    quantity_policy: crate::quantity::QuantityPresentationPolicy,
) -> Vec<(String, String)> {
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    model
        .traces
        .iter()
        .filter(|trace| trace.visible)
        .take(6)
        .map(|trace| {
            let value = sample_at_with(&trace.x, &trace.y, x, interpolation);
            (
                trace.name.clone(),
                model.format_trace_value(trace, value, significant_digits, quantity_policy),
            )
        })
        .collect()
}

/// Per-trace change between the cursors, in the same order and length as
/// [`value_rows`] so the readout strip's columns stay aligned row for row.
///
/// The difference is taken in value space, never between two formatted
/// readouts, so a Δ can never disagree with the values above it.
fn delta_values(
    model: &StripModel,
    a: f64,
    b: f64,
    presentation: ResultPresentationPolicy,
    quantity_policy: crate::quantity::QuantityPresentationPolicy,
) -> Vec<String> {
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    model
        .traces
        .iter()
        .filter(|trace| trace.visible)
        .take(READOUT_TRACE_LIMIT)
        .map(|trace| {
            let dv = sample_at_with(&trace.x, &trace.y, b, interpolation)
                - sample_at_with(&trace.x, &trace.y, a, interpolation);
            model.format_trace_value(trace, dv, significant_digits, quantity_policy)
        })
        .collect()
}

/// The separation between the cursors, named the way the X axis reads: a
/// time span also reports its reciprocal, because 1/Δt is the number a
/// designer is actually after when measuring a period.
fn x_separation(
    model: &StripModel,
    a: f64,
    b: f64,
    significant_digits: usize,
    quantity_policy: crate::quantity::QuantityPresentationPolicy,
) -> String {
    let dx = b - a;
    match model.x_scale {
        XScale::Linear if model.x_unit == "s" => {
            let span = fmt_si_significant(dx, "s", significant_digits);
            if dx == 0.0 {
                span
            } else {
                format!(
                    "{span}  ({})",
                    quantity_policy.format_frequency(1.0 / dx.abs(), significant_digits)
                )
            }
        }
        XScale::Log10 => quantity_policy.format_frequency(dx, significant_digits),
        _ => fmt_si_significant(dx, &model.x_unit, significant_digits),
    }
}

/// dB/decade slope of the magnitude trace between the cursors, on log-X
/// strips that carry one.
fn slope_between(
    model: &StripModel,
    a: f64,
    b: f64,
    presentation: ResultPresentationPolicy,
) -> Option<String> {
    if model.x_scale != XScale::Log10 || a <= 0.0 || b <= 0.0 {
        return None;
    }
    let dlog = (b.log10() - a.log10()).abs();
    if dlog <= 1e-12 {
        return None;
    }
    let magnitude = model
        .traces
        .iter()
        .find(|trace| trace.kind == TraceKind::MagnitudeDb && trace.visible)?;
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    let ddb = sample_at_with(&magnitude.x, &magnitude.y, b, interpolation)
        - sample_at_with(&magnitude.x, &magnitude.y, a, interpolation);
    Some(fmt_significant(
        ddb / dlog,
        usize::from(presentation.displayed_significant_digits().get()),
        " dB/dec",
    ))
}

/// min / max / rms rows per visible trace, optionally windowed to [a, b].
/// The single-pass stats are cached per (trace, window, data version), so
/// no samples are rescanned until the cursors or the data move.
fn measurement_rows(
    ui: &mut Ui,
    derived: &mut DerivedSeries,
    model: &StripModel,
    window: Option<(f64, f64)>,
    significant_digits: usize,
    quantity_policy: crate::quantity::QuantityPresentationPolicy,
) {
    use crate::analysis::measurements as basic;

    // Window identity for the cache key; u64::MAX is a NaN bit pattern no
    // finite cursor can produce, marking the full-range case.
    let (a_bits, b_bits) = match window {
        Some((a, b)) => (a.to_bits(), b.to_bits()),
        None => (u64::MAX, u64::MAX),
    };

    let mut rows: Vec<(String, String)> = Vec::new();
    for trace in model.traces.iter().filter(|t| t.visible).take(4) {
        let key = (trace_key(model, trace), a_bits, b_bits);
        let stats = derived.stats_or(key, || {
            let (start, end) = match window {
                Some((a, b)) => {
                    let start = trace.x.partition_point(|&v| v < a);
                    let end = trace.x.partition_point(|&v| v <= b);
                    (start, end.max(start))
                }
                None => (0, trace.y.len()),
            };
            basic::calculate_min_max_rms(&trace.y[start..end])
        });
        let Some((min, max, rms)) = stats else {
            continue;
        };
        let fmt = |v: f64| -> String {
            match trace.kind {
                TraceKind::Value | TraceKind::Real | TraceKind::Imaginary => {
                    fmt_si_significant(v, model.y_unit, significant_digits)
                }
                TraceKind::MagnitudeDb => fmt_significant(v, significant_digits, " dB"),
                TraceKind::PhaseDeg => {
                    quantity_policy.format_angle(v.to_radians(), significant_digits)
                }
                TraceKind::PhaseRad => quantity_policy.format_angle(v, significant_digits),
            }
        };
        rows.push((format!("{} min", trace.name), fmt(min)));
        rows.push((format!("{} max", trace.name), fmt(max)));
        if matches!(
            trace.kind,
            TraceKind::Value | TraceKind::Real | TraceKind::Imaginary
        ) {
            rows.push((format!("{} rms", trace.name), fmt(rms)));
        }
    }
    let refs: Vec<(&str, &str)> = rows.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
    crate::ui::widgets::measurement_table(ui, &refs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_readout_strip_stands_down_until_a_cursor_is_placed() {
        let mut state = AppState::default();
        assert!(state.ui.results.cursor_tool.is_armed(), "armed by default");
        assert!(!state.ui.results.cursor_readout_active());
        assert_eq!(readout_strip_height(&state), 0.0);

        state.ui.results.cursors.place(1.0e-3);
        assert!(state.ui.results.cursor_readout_active());
        // No visible traces on an empty run: the header alone, never a band
        // of blank rows.
        assert_eq!(readout_strip_height(&state), READOUT_HEADER_H);
    }

    #[test]
    fn disarming_the_cursor_tool_clears_the_pair_and_hides_the_strip() {
        let mut state = AppState::default();
        state.ui.results.cursors.place(1.0);
        state.ui.results.cursors.place(2.0);
        state.ui.results.cursor_strip = Some(0);

        state.ui.results.toggle_cursor_tool();

        assert!(!state.ui.results.cursor_tool.is_armed());
        assert!(!state.ui.results.cursors.any());
        assert_eq!(state.ui.results.cursor_strip, None);
        assert_eq!(readout_strip_height(&state), 0.0);

        state.ui.results.toggle_cursor_tool();
        assert!(state.ui.results.cursor_tool.is_armed());
        assert!(
            !state.ui.results.cursors.any(),
            "re-arming must not resurrect cleared cursors"
        );
    }

    #[test]
    fn the_strip_never_grows_past_its_trace_limit() {
        assert_eq!(READOUT_TRACE_LIMIT, 4);
        // Height is header + rows, so the limit bounds the strip exactly.
        let bounded = READOUT_HEADER_H + READOUT_TRACE_LIMIT as f32 * READOUT_ROW_H;
        assert!(bounded < 120.0, "the readout is a strip, not a dock");
    }
    use crate::product::{AnalysisInstanceId, ContentDigest, DatasetId, ObjectRevision};
    use crate::results::visualization_document::{
        FamilyAggregationMethod, FamilyAggregationPolicy, FamilyComparisonOperator,
        FamilyDimension, FamilyEncodingMap, FamilyFilterExpression, FamilyPredicate,
        FamilyPresentationPolicy, FamilyXDimension, FamilyXOrdering, MissingPointPolicy,
        TypedValue, ValueType,
    };
    use crate::state::{
        AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultProvenance, SimulationRun,
        WaveformData,
    };
    use crate::workbench::ChoicePreference;
    use crate::workbench::visualization_family::FamilyManifest;

    fn assert_editor_spans_disjoint(left: EditorSpan, right: EditorSpan) {
        assert!(
            left.end() <= right.start + f32::EPSILON,
            "editor spans overlap: {left:?} and {right:?}"
        );
    }

    #[test]
    fn compact_expression_editor_reserves_add_and_stacks_error() {
        let layout = expr_editor_layout(350.0, 28.0, 42.0, Some(420.0));

        assert!(layout.stack_error);
        assert_eq!(layout.add.end(), 350.0);
        assert_eq!(layout.error.width, 0.0);
        assert!(layout.input.width > EXPR_EDITOR_MIN_INLINE_INPUT);
        assert_editor_spans_disjoint(layout.label, layout.input);
        assert_editor_spans_disjoint(layout.input, layout.add);
    }

    #[test]
    fn wide_expression_editor_bounds_inline_error_without_starving_input() {
        let layout = expr_editor_layout(900.0, 28.0, 42.0, Some(640.0));

        assert!(!layout.stack_error);
        assert!(layout.error.width > 0.0);
        assert!(layout.error.width <= 900.0 * 0.28);
        assert!(layout.input.width >= EXPR_EDITOR_MIN_INLINE_INPUT);
        assert_editor_spans_disjoint(layout.label, layout.input);
        assert_editor_spans_disjoint(layout.input, layout.error);
        assert_editor_spans_disjoint(layout.error, layout.add);
        assert_eq!(layout.add.end(), 900.0);
    }

    #[test]
    fn expression_editor_geometry_stays_inside_pathological_widths() {
        for width in [0.0, 20.0, 64.0, 180.0, 560.0] {
            let layout = expr_editor_layout(width, 28.0, 42.0, None);
            for span in [layout.label, layout.input, layout.error, layout.add] {
                assert!(span.start >= 0.0);
                assert!(span.end() <= width + f32::EPSILON);
            }
            assert_editor_spans_disjoint(layout.label, layout.input);
            assert_editor_spans_disjoint(layout.input, layout.add);
        }
    }

    fn family_policy() -> FamilyPresentationPolicy {
        let process = FamilyDimension::new("process", ValueType::Text).unwrap();
        FamilyPresentationPolicy {
            x_dimension: FamilyXDimension {
                dimension: FamilyDimension::new("RGAIN", ValueType::Real).unwrap(),
                ordering: FamilyXOrdering::Source,
            },
            family_dimensions: vec![process.clone()],
            facet_layout: None,
            aggregation: FamilyAggregationPolicy {
                method: FamilyAggregationMethod::None,
                over_dimensions: Vec::new(),
            },
            filter: None,
            missing_points: MissingPointPolicy::ExcludeWithOmissionRecord,
            encodings: vec![
                FamilyEncodingMap::Color {
                    dimension: process.clone(),
                    palette: AccessibleColorPalette::OkabeItoCategorical,
                },
                FamilyEncodingMap::Dash {
                    dimension: process.clone(),
                },
                FamilyEncodingMap::Marker { dimension: process },
            ],
        }
    }

    fn family_analysis(values: Vec<f64>) -> AnalysisResult {
        AnalysisResult::new(41, AnalysisType::Corner, "PVT")
            .with_waveforms(vec![WaveformData::new(
                "V(out)",
                vec![101.0, 102.0, 103.0, 104.0, 105.0, 106.0],
                values,
                "#fff",
            )])
            .with_family_metadata(AnalysisResultFamilyMetadata::Corner {
                x_values: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
                x_label: "RGAIN".to_owned(),
                x_unit: "kOhm".to_owned(),
                temperatures_c: vec![27.0; 6],
                corner_labels: vec![
                    "SS".to_owned(),
                    "SS".to_owned(),
                    "TT".to_owned(),
                    "TT".to_owned(),
                    "FF".to_owned(),
                    "FF".to_owned(),
                ],
                failed_corners: 0,
            })
    }

    /// A one-analysis transient run with a single ramp on `V(out)`.
    fn marker_fixture() -> AppState {
        let mut state = AppState::default();
        state.simulation.start_run().add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 5.0], "#fff"),
            ]),
        );
        state
    }

    #[test]
    fn a_marker_reattaches_to_its_signal_after_a_re_run() {
        let waveforms = || {
            vec![WaveformData::new(
                "V(out)",
                vec![0.0, 1.0],
                vec![0.0, 5.0],
                "#fff",
            )]
        };
        let mut simulation = SimulationState::default();
        simulation.start_run().add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(waveforms()),
        );
        let mut derived = DerivedSeries::default();
        let first = build_models(
            &simulation,
            &mut derived,
            &Tokens::default(),
            false,
            ComplexNumberDisplay::MagnitudePhaseDegrees,
            None,
            &HashSet::new(),
        );
        let anchor = anchor_key(&first[0], &first[0].traces[0]);

        simulation.start_run().add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(waveforms()),
        );
        let mut derived = DerivedSeries::default();
        let second = build_models(
            &simulation,
            &mut derived,
            &Tokens::default(),
            false,
            ComplexNumberDisplay::MagnitudePhaseDegrees,
            None,
            &HashSet::new(),
        );
        let reattached = second[0]
            .traces
            .iter()
            .find(|trace| !trace.overlay && anchor_key(&second[0], trace) == anchor)
            .expect("the marker's anchor still names the signal after a re-run");
        let anchor = anchor_key(&second[0], reattached);

        // Every run of a signal shares its anchor — a marker names the
        // signal, not one solve of it — while the decimation key an overlay
        // run draws under stays separated, so two runs' envelopes can never
        // serve each other.
        assert_eq!(run_mixed_key(anchor, 7, false), anchor);
        assert_ne!(run_mixed_key(anchor, 7, true), anchor);
    }

    #[test]
    fn markers_alone_keep_a_compact_readout_strip_on_screen() {
        let mut state = marker_fixture();
        assert_eq!(readout_strip_height(&state), 0.0);

        state
            .ui
            .results
            .add_marker(0, 0x51, "V(out)".to_owned(), 0.5);
        assert_eq!(
            readout_strip_height(&state),
            READOUT_HEADER_H + MARKER_ROW_H,
            "markers-only strip: the marker header and its one row"
        );

        // A closed strip takes its markers off screen with it.
        state.ui.results.hidden_strips.insert(0);
        assert_eq!(readout_strip_height(&state), 0.0);
    }

    #[test]
    fn the_strip_carries_cursors_and_markers_together() {
        let mut state = marker_fixture();
        state.ui.results.cursors.place(0.5);
        state.ui.results.cursor_strip = Some(0);
        let cursors_only = readout_strip_height(&state);
        assert!(cursors_only > 0.0);

        state
            .ui
            .results
            .add_marker(0, 0x51, "V(out)".to_owned(), 0.5);
        assert_eq!(
            readout_strip_height(&state),
            cursors_only + READOUT_HEADER_H + MARKER_ROW_H
        );
    }

    #[test]
    fn markers_outlive_the_tool_that_placed_them() {
        let mut state = marker_fixture();
        assert!(
            !state.ui.results.marker_tool.is_armed(),
            "annotating is deliberate — the tool is off until asked for"
        );
        state.ui.results.toggle_marker_tool();
        let id = state
            .ui
            .results
            .add_marker(0, 0x51, "V(out)".to_owned(), 0.5);
        state.ui.results.toggle_marker_tool();

        assert!(!state.ui.results.marker_tool.is_armed());
        assert_eq!(state.ui.results.markers.len(), 1);

        // Cursors are a readout and clear; markers are content and do not.
        state.ui.results.clear_cursors();
        assert_eq!(state.ui.results.markers.len(), 1);
        assert_eq!(state.ui.results.markers[0].id, id);
    }

    #[test]
    fn removing_a_marker_takes_its_open_note_editor_with_it() {
        let mut state = marker_fixture();
        let first = state
            .ui
            .results
            .add_marker(0, 0x51, "V(out)".to_owned(), 0.5);
        state.ui.results.editing_marker = Some(first);

        state.ui.results.remove_marker(first);

        assert!(state.ui.results.markers.is_empty());
        assert_eq!(state.ui.results.editing_marker, None);

        // Ids are not recycled: M1 must not come back meaning something else.
        let second = state
            .ui
            .results
            .add_marker(0, 0x51, "V(out)".to_owned(), 0.9);
        assert_ne!(first, second);
    }

    #[test]
    fn only_a_spec_marker_declines_to_report_a_trace_value() {
        assert!(MarkerKind::Note.rides_a_trace());
        assert!(MarkerKind::Peak.rides_a_trace());
        assert!(
            !MarkerKind::Spec.rides_a_trace(),
            "a spec constrains the axis position, not one curve"
        );

        let mut kind = MarkerKind::Note;
        for _ in 0..MarkerKind::ALL.len() {
            kind = kind.next();
        }
        assert_eq!(kind, MarkerKind::Note, "the kind control cycles");
    }

    #[test]
    fn a_marker_tag_names_the_note_only_when_there_is_one() {
        let mut marker = ResultMarker {
            id: 3,
            analysis_index: 0,
            anchor: 0x51,
            trace_name: "V(out)".to_owned(),
            x: 0.0,
            kind: MarkerKind::Note,
            note: String::new(),
        };
        assert_eq!(marker_label(&marker), "M3");

        marker.note = "  settling  ".to_owned();
        assert_eq!(marker_label(&marker), "M3 · settling");
    }

    #[test]
    fn a_signal_owns_its_unit_rather_than_inheriting_the_analysis_default() {
        // The accessor in the name is authoritative where there is one.
        assert_eq!(signal_unit("V(out)", TraceKind::Value, "V"), "V");
        assert_eq!(signal_unit("I(R1)", TraceKind::Value, "V"), "A");
        assert_eq!(signal_unit("i(vsense)", TraceKind::Value, "V"), "A");
        assert_eq!(signal_unit("P(M1)", TraceKind::Value, "V"), "W");

        // Derived projections keep the underlying signal's unit.
        assert_eq!(signal_unit("re(V(out))", TraceKind::Real, ""), "V");
        assert_eq!(signal_unit("im(I(R1))", TraceKind::Imaginary, ""), "A");

        // The analysis default applies only where the name carries nothing
        // to read a unit from.
        assert_eq!(signal_unit("onoise", TraceKind::Value, "V^2/Hz"), "V^2/Hz");

        // Derived kinds have their own units regardless of the source.
        assert_eq!(signal_unit("V(out)", TraceKind::MagnitudeDb, "V"), "dB");
        assert_eq!(signal_unit("V(out)", TraceKind::PhaseDeg, "V"), "°");
    }

    #[test]
    fn mixed_units_on_one_analysis_become_separate_panes() {
        let mut simulation = SimulationState::default();
        simulation.start_run().add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 5.0], "#fff"),
                WaveformData::new("I(R1)", vec![0.0, 1.0], vec![0.0, 1.0e-3], "#fff"),
                WaveformData::new("V(in)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff"),
            ]),
        );
        let mut derived = DerivedSeries::default();
        let models = build_models(
            &simulation,
            &mut derived,
            &Tokens::default(),
            false,
            ComplexNumberDisplay::MagnitudePhaseDegrees,
            None,
            &HashSet::new(),
        );

        let panes = models[0].unit_panes();
        assert_eq!(panes.len(), 2, "volts and amps cannot share an axis");
        assert_eq!(panes[0].unit, "V");
        assert_eq!(
            panes[0].traces.len(),
            2,
            "both voltages belong to the volt pane"
        );
        assert_eq!(panes[1].unit, "A");
        assert_eq!(panes[1].traces.len(), 1);
    }

    #[test]
    fn one_unit_stays_one_pane() {
        let mut simulation = SimulationState::default();
        simulation.start_run().add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 5.0], "#fff"),
                WaveformData::new("V(in)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff"),
            ]),
        );
        let mut derived = DerivedSeries::default();
        let models = build_models(
            &simulation,
            &mut derived,
            &Tokens::default(),
            false,
            ComplexNumberDisplay::MagnitudePhaseDegrees,
            None,
            &HashSet::new(),
        );

        let panes = models[0].unit_panes();
        assert_eq!(panes.len(), 1, "a strip does not split without a reason to");
        assert_eq!(panes[0].unit, "V");
        assert!(panes[0].right.is_empty());
    }

    #[test]
    fn a_hidden_trace_takes_its_pane_with_it() {
        let mut simulation = SimulationState::default();
        simulation.start_run().add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 5.0], "#fff"),
                WaveformData::new("I(R1)", vec![0.0, 1.0], vec![0.0, 1.0e-3], "#fff"),
            ]),
        );
        if let Some(run) = simulation.active_run_mut()
            && let Some(analysis) = run.analyses.get_mut(0)
            && let Some(current) = analysis.waveforms.get_mut(1)
        {
            current.visible = false;
        }
        let mut derived = DerivedSeries::default();
        let models = build_models(
            &simulation,
            &mut derived,
            &Tokens::default(),
            false,
            ComplexNumberDisplay::MagnitudePhaseDegrees,
            None,
            &HashSet::new(),
        );

        let panes = models[0].unit_panes();
        assert_eq!(panes.len(), 1);
        assert_eq!(panes[0].unit, "V", "the amp axis goes with its only trace");
    }

    #[test]
    fn phase_rides_the_magnitude_pane_rather_than_taking_its_own() {
        let mut simulation = SimulationState::default();
        simulation.start_run().add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![
                WaveformData::new("|V(out)|", vec![1.0, 10.0], vec![1.0, 0.5], "#fff"),
                WaveformData::new("phase(V(out))", vec![1.0, 10.0], vec![0.0, -45.0], "#fff"),
            ]),
        );
        let mut derived = DerivedSeries::default();
        let models = build_models(
            &simulation,
            &mut derived,
            &Tokens::default(),
            false,
            ComplexNumberDisplay::MagnitudePhaseDegrees,
            None,
            &HashSet::new(),
        );

        let panes = models[0].unit_panes();
        assert_eq!(
            panes.len(),
            1,
            "a Bode pair is one reading — splitting it across stacked panes breaks it"
        );
        assert_eq!(panes[0].unit, "dB");
        assert_eq!(panes[0].right.len(), 1, "phase goes to the right axis");
    }

    #[test]
    fn fitting_a_strip_fits_every_pane_of_it() {
        let mut state = AppState::default();
        let viewer = super::super::ResultViewer::Waves;
        state.ui.results.plot_view_pane_mut(viewer, 0, 0).y = Some((0.0, 1.0));
        state.ui.results.plot_view_pane_mut(viewer, 0, 1).y = Some((0.0, 2.0));
        state.ui.results.plot_view_pane_mut(viewer, 1, 0).y = Some((0.0, 3.0));
        assert!(state.ui.results.strip_is_zoomed(viewer, 0));

        state.ui.results.reset_plot_view(viewer, 0);

        assert!(
            !state.ui.results.strip_is_zoomed(viewer, 0),
            "leaving one pane zoomed would make the strip's panes disagree"
        );
        assert!(
            state.ui.results.strip_is_zoomed(viewer, 1),
            "fitting one strip does not reach into another"
        );
    }

    #[test]
    fn each_pane_keeps_its_own_y_viewport() {
        let mut state = AppState::default();
        let viewer = super::super::ResultViewer::Waves;
        state.ui.results.plot_view_pane_mut(viewer, 0, 0).y = Some((-5.0, 5.0));
        state.ui.results.plot_view_pane_mut(viewer, 0, 1).y = Some((0.0, 1.0e-3));

        // One zoom factor across volts and amps would mean nothing, so the
        // panes never share a Y override.
        assert_eq!(
            state.ui.results.plot_view_pane(viewer, 0, 0).y,
            Some((-5.0, 5.0))
        );
        assert_eq!(
            state.ui.results.plot_view_pane(viewer, 0, 1).y,
            Some((0.0, 1.0e-3))
        );
    }

    #[test]
    fn noise_strip_uses_spectral_density_unit_without_db_conversion() {
        let mut simulation = SimulationState::default();
        simulation.start_run().add_analysis(
            AnalysisResult::new(1, AnalysisType::Noise, "Noise").with_waveforms(vec![
                WaveformData::new("onoise", vec![1.0, 10.0], vec![1.0e-18, 2.0e-18], "#fff"),
            ]),
        );
        let mut derived = DerivedSeries::default();

        let models = build_models(
            &simulation,
            &mut derived,
            &Tokens::default(),
            false,
            ComplexNumberDisplay::MagnitudePhaseDegrees,
            None,
            &HashSet::new(),
        );

        assert_eq!(models.len(), 1);
        assert_eq!(models[0].y_unit, "V^2/Hz");
        assert!(matches!(models[0].traces[0].kind, TraceKind::Value));
        assert_eq!(models[0].traces[0].y.as_slice(), &[1.0e-18, 2.0e-18]);
    }

    #[test]
    fn family_selection_projects_exact_source_rows_without_mutating_the_run() {
        let mut simulation = SimulationState::default();
        simulation.start_run().add_analysis(
            AnalysisResult::new(41, AnalysisType::Corner, "PVT").with_waveforms(vec![
                WaveformData::new(
                    "V(out)",
                    vec![1.0, 2.0, 3.0, 4.0],
                    vec![10.0, 20.0, 30.0, 40.0],
                    "#fff",
                ),
            ]),
        );
        let run = simulation.active_run().expect("active run");
        let selection = SourceSampleSelection::new(run.dataset_id, 41, vec![1, 3])
            .expect("ordered exact selection");
        let mut derived = DerivedSeries::default();

        let models = build_models(
            &simulation,
            &mut derived,
            &Tokens::default(),
            false,
            ComplexNumberDisplay::MagnitudePhaseDegrees,
            Some(&selection),
            &HashSet::new(),
        );

        assert_eq!(models[0].traces[0].x.as_slice(), &[2.0, 4.0]);
        assert_eq!(models[0].traces[0].y.as_slice(), &[20.0, 40.0]);
        let original = &simulation.active_run().unwrap().analyses[0].waveforms[0];
        assert_eq!(original.x.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(original.y.as_slice(), &[10.0, 20.0, 30.0, 40.0]);
    }

    #[test]
    fn family_policy_expands_stable_styles_and_preserves_overlay_sources() {
        let mut active = SimulationRun::new(2);
        active.add_analysis(family_analysis(vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0]));
        let active_dataset = active.dataset_id;
        let manifest = FamilyManifest::from_analysis(&active.analyses[0])
            .unwrap()
            .unwrap();
        let selection = SourceSampleSelection::new(active_dataset, 41, vec![0, 1, 2, 3, 4, 5])
            .unwrap()
            .with_family_presentation(&manifest, &family_policy())
            .unwrap();

        let mut overlay = SimulationRun::new(1);
        overlay.add_analysis(family_analysis(vec![11.0, 21.0, 31.0, 41.0, 51.0, 61.0]));
        let overlay_dataset = overlay.dataset_id;
        let simulation = SimulationState {
            runs: vec![active, overlay],
            active_run_idx: Some(0),
            active_analysis_idx: Some(0),
            overlay_dataset_ids: vec![overlay_dataset],
            ..SimulationState::default()
        };
        let mut derived = DerivedSeries::default();

        let models = build_models(
            &simulation,
            &mut derived,
            &Tokens::default(),
            false,
            ComplexNumberDisplay::MagnitudePhaseDegrees,
            Some(&selection),
            &HashSet::new(),
        );

        let model = &models[0];
        assert_eq!(model.signal_trace_count, 3);
        assert_eq!(
            model.traces.len(),
            6,
            "overlay signal must project through every exact family group"
        );
        assert_eq!(model.x_label, "RGAIN");
        assert_eq!(model.x_unit, "kOhm");
        assert_eq!(model.x_scale, XScale::Linear);
        let ss = model
            .traces
            .iter()
            .find(|trace| !trace.overlay && trace.name.contains("SS"))
            .unwrap();
        let tt = model
            .traces
            .iter()
            .find(|trace| !trace.overlay && trace.name.contains("TT"))
            .unwrap();
        let tt_visibility_key = tt.family_visibility_key.unwrap();
        assert_eq!(ss.x.as_slice(), &[1.0, 2.0]);
        assert_eq!(ss.y.as_slice(), &[10.0, 20.0]);
        assert_eq!(tt.x.as_slice(), &[3.0, 4.0]);
        assert_eq!(tt.y.as_slice(), &[30.0, 40.0]);
        assert_ne!(ss.color, tt.color);
        assert_ne!(
            ss.family_style.unwrap().marker_ordinal,
            tt.family_style.unwrap().marker_ordinal
        );
        let styled = apply_family_trace_style(Trace::new(&ss.x, &ss.y, ss.color), ss.family_style);
        assert_eq!(styled.dash_style, ss.family_style.unwrap().dash_ordinal);
        assert_eq!(styled.marker_style, ss.family_style.unwrap().marker_ordinal);
        assert!(styled.show_single_point);
        assert!(model.traces.last().unwrap().overlay);
        assert_eq!(model.traces.last().unwrap().x.as_slice(), &[3.0, 4.0]);

        let source = &simulation.runs[0].analyses[0].waveforms[0];
        assert_eq!(
            source.x.as_slice(),
            &[101.0, 102.0, 103.0, 104.0, 105.0, 106.0]
        );
        assert_eq!(source.y.as_slice(), &[10.0, 20.0, 30.0, 40.0, 50.0, 60.0]);

        let mut copied = String::new();
        append_copied_cursor(
            &mut copied,
            "A",
            3.0,
            model,
            SampleInterpolation::Linear,
            crate::quantity::QuantityPresentationPolicy::default(),
        );
        assert!(copied.contains("A RGAIN ="));
        assert!(copied.contains("kOhm"));
        let domain = model.cursor_domain();
        let mut incompatible_unit = domain.clone();
        incompatible_unit.x_unit = "Ohm".to_owned();
        assert_ne!(domain, incompatible_unit);
        let mut incompatible_label = domain.clone();
        incompatible_label.x_label = "Resistance".to_owned();
        assert_ne!(domain, incompatible_label);

        let mut results = ResultsState::default();
        results.set_sample_selection(Some(selection.clone()));
        results.toggle_family_trace_visibility(tt_visibility_key);
        let toggled = build_models(
            &simulation,
            &mut results.derived,
            &Tokens::default(),
            false,
            ComplexNumberDisplay::MagnitudePhaseDegrees,
            Some(&selection),
            &results.hidden_family_traces,
        );
        let toggled_model = &toggled[0];
        assert!(
            toggled_model
                .traces
                .iter()
                .find(|trace| !trace.overlay && trace.name.contains("SS"))
                .unwrap()
                .visible
        );
        assert!(
            !toggled_model
                .traces
                .iter()
                .find(|trace| !trace.overlay && trace.name.contains("TT"))
                .unwrap()
                .visible
        );
        for overlay in toggled_model.traces.iter().filter(|trace| trace.overlay) {
            let active = toggled_model
                .traces
                .iter()
                .find(|trace| {
                    !trace.overlay
                        && trace.presentation_key == overlay.presentation_key
                        && trace.kind == overlay.kind
                })
                .unwrap();
            assert_eq!(overlay.visible, active.visible);
        }
        assert!(
            toggled_model
                .traces
                .iter()
                .any(|trace| trace.overlay && trace.visible)
        );
        assert!(simulation.runs[0].analyses[0].waveforms[0].visible);
        results.set_sample_selection(None);
        assert!(results.hidden_family_traces.is_empty());
    }

    #[test]
    fn incompatible_family_overlay_is_visibly_rejected_without_drawing_native_x() {
        let mut active = SimulationRun::new(2);
        active.add_analysis(family_analysis(vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0]));
        let manifest = FamilyManifest::from_analysis(&active.analyses[0])
            .unwrap()
            .unwrap();
        let selection = SourceSampleSelection::new(active.dataset_id, 41, vec![0, 1, 2, 3, 4, 5])
            .unwrap()
            .with_family_presentation(&manifest, &family_policy())
            .unwrap();

        let mut incompatible = family_analysis(vec![11.0, 21.0, 31.0, 41.0, 51.0, 61.0]);
        let Some(AnalysisResultFamilyMetadata::Corner { x_unit, .. }) =
            incompatible.family_metadata.as_mut()
        else {
            panic!("corner metadata");
        };
        *x_unit = "Ohm".to_owned();
        let mut overlay = SimulationRun::new(1);
        overlay.add_analysis(incompatible);
        let overlay_dataset = overlay.dataset_id;
        let simulation = SimulationState {
            runs: vec![active, overlay],
            active_run_idx: Some(0),
            overlay_dataset_ids: vec![overlay_dataset],
            ..SimulationState::default()
        };

        let models = build_models(
            &simulation,
            &mut DerivedSeries::default(),
            &Tokens::default(),
            false,
            ComplexNumberDisplay::MagnitudePhaseDegrees,
            Some(&selection),
            &HashSet::new(),
        );
        assert_eq!(models[0].traces.len(), models[0].signal_trace_count);
        assert!(
            models[0]
                .subtitle
                .contains("incompatible family overlay hidden")
        );
        assert!(models[0].traces.iter().all(|trace| !trace.overlay));
    }

    #[test]
    fn filtered_overlay_uses_typed_ast_and_ignores_excluded_duplicate_x_rows() {
        let mut active = SimulationRun::new(2);
        active.add_analysis(family_analysis(vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0]));
        let manifest = FamilyManifest::from_analysis(&active.analyses[0])
            .unwrap()
            .unwrap();
        let process = FamilyDimension::new("process", ValueType::Text).unwrap();
        let mut policy = family_policy();
        policy.filter = Some(FamilyFilterExpression {
            // Deliberately contradictory UI source: the typed AST is the
            // persisted execution contract.
            source: "process = SS".to_owned(),
            predicate: FamilyPredicate::Compare {
                dimension: process,
                operator: FamilyComparisonOperator::Equal,
                value: TypedValue::Text("TT".to_owned()),
            },
        });
        let indices = manifest
            .matching_source_indices_for_filter(policy.filter.as_ref())
            .unwrap();
        assert_eq!(indices, [2, 3]);
        let selection = SourceSampleSelection::new(active.dataset_id, 41, indices)
            .unwrap()
            .with_family_presentation(&manifest, &policy)
            .unwrap();

        let mut overlay_analysis = family_analysis(vec![11.0, 21.0, 31.0, 41.0, 51.0, 61.0]);
        let Some(AnalysisResultFamilyMetadata::Corner { x_values, .. }) =
            overlay_analysis.family_metadata.as_mut()
        else {
            panic!("corner metadata");
        };
        // Excluded SS rows are non-monotonic. Re-evaluating all overlay rows
        // would reject this otherwise compatible filtered TT projection.
        x_values[0] = 1.0;
        x_values[1] = 1.0;
        let mut overlay = SimulationRun::new(1);
        overlay.add_analysis(overlay_analysis);
        let overlay_dataset = overlay.dataset_id;
        let simulation = SimulationState {
            runs: vec![active, overlay],
            active_run_idx: Some(0),
            overlay_dataset_ids: vec![overlay_dataset],
            ..SimulationState::default()
        };

        let models = build_models(
            &simulation,
            &mut DerivedSeries::default(),
            &Tokens::default(),
            false,
            ComplexNumberDisplay::MagnitudePhaseDegrees,
            Some(&selection),
            &HashSet::new(),
        );
        assert_eq!(models[0].signal_trace_count, 1);
        assert_eq!(models[0].traces.len(), 2);
        let overlay = models[0].traces.iter().find(|trace| trace.overlay).unwrap();
        assert_eq!(overlay.x.as_slice(), &[3.0, 4.0]);
        assert_eq!(overlay.y.as_slice(), &[31.0, 41.0]);
    }

    #[test]
    fn derived_expression_rows_are_split_by_the_exact_family_plan() {
        let analysis = family_analysis(vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0]);
        let manifest = FamilyManifest::from_analysis(&analysis).unwrap().unwrap();
        let selection = SourceSampleSelection::new(DatasetId::new(), 41, vec![0, 2, 4])
            .unwrap()
            .with_family_presentation(&manifest, &family_policy())
            .unwrap();
        // Expression evaluation has already selected exact rows 0, 2, 4.
        let x = Arc::new(vec![101.0, 103.0, 105.0]);
        let y = Arc::new(vec![100.0, 300.0, 500.0]);

        let projections = projected_selected_family_series(&x, &y, Some(&selection)).unwrap();

        assert_eq!(projections.len(), 3);
        assert!(projections.iter().all(|projection| projection.x.len() == 1));
        let tt = projections
            .iter()
            .find(|projection| projection.group.unwrap().label.contains("TT"))
            .unwrap();
        assert_eq!(tt.x.as_slice(), &[3.0]);
        assert_eq!(tt.y.as_slice(), &[300.0]);
        let styled = apply_family_trace_style(
            Trace::new(&tt.x, &tt.y, egui::Color32::WHITE),
            Some(tt.group.unwrap().style),
        );
        assert!(styled.show_single_point);
    }

    fn ac_result(
        source_id: AnalysisInstanceId,
        values: [f64; 2],
        snapshot_byte: u8,
    ) -> AnalysisResult {
        AnalysisResult::new(1, AnalysisType::Ac, "AC")
            .with_waveforms(vec![WaveformData::new(
                "V(out)",
                vec![1.0, 10.0],
                values.to_vec(),
                "#fff",
            )])
            .with_provenance(
                AnalysisResultProvenance::new(
                    source_id,
                    ObjectRevision::INITIAL,
                    ContentDigest::from_bytes([snapshot_byte; 32]),
                    Vec::new(),
                )
                .expect("valid AC provenance"),
            )
    }

    #[test]
    fn overlays_pair_two_same_kind_results_by_exact_source_instance() {
        let first_id = AnalysisInstanceId::new();
        let second_id = AnalysisInstanceId::new();

        let mut active = SimulationRun::new(2);
        active.add_analysis(ac_result(first_id, [1.0, 2.0], 0x11));
        active.add_analysis(ac_result(second_id, [3.0, 4.0], 0x11));

        let mut overlay = SimulationRun::new(1);
        // Reverse the same-kind result order: kind/label matching would alias
        // the first overlay result onto both active strips.
        overlay.add_analysis(ac_result(second_id, [201.0, 202.0], 0x22));
        overlay.add_analysis(ac_result(first_id, [101.0, 102.0], 0x22));
        let overlay_dataset_id = overlay.dataset_id;

        let mut simulation = SimulationState {
            runs: vec![active, overlay],
            active_run_idx: Some(0),
            overlay_dataset_ids: vec![overlay_dataset_id],
            ..SimulationState::default()
        };
        assert!(simulation.select_analysis(0));
        let mut derived = DerivedSeries::default();

        let models = build_models(
            &simulation,
            &mut derived,
            &Tokens::default(),
            false,
            ComplexNumberDisplay::MagnitudePhaseDegrees,
            None,
            &HashSet::new(),
        );

        assert_eq!(models.len(), 2);
        assert_eq!(models[0].signal_trace_count, 1);
        assert_eq!(models[0].traces.len(), 2);
        assert!(models[0].traces[1].overlay);
        assert_eq!(models[0].traces[1].y.as_slice(), &[101.0, 102.0]);
        assert_eq!(models[1].signal_trace_count, 1);
        assert_eq!(models[1].traces.len(), 2);
        assert!(models[1].traces[1].overlay);
        assert_eq!(models[1].traces[1].y.as_slice(), &[201.0, 202.0]);
    }

    #[test]
    fn complex_display_policy_uses_original_components_or_radian_phase() {
        let magnitude = WaveformData::new("|V(out)|", vec![1.0, 10.0], vec![1.0, 10.0], "#fff")
            .with_complex_components("V(out)", vec![0.8, 6.0], vec![0.6, 8.0]);
        let phase = WaveformData::new("phase(V(out))", vec![1.0, 10.0], vec![180.0, 90.0], "#aaa");
        let mut simulation = SimulationState::default();
        simulation.start_run().add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![magnitude, phase]),
        );

        let mut derived = DerivedSeries::default();
        let cartesian = build_models(
            &simulation,
            &mut derived,
            &Tokens::default(),
            false,
            ComplexNumberDisplay::RealImaginary,
            None,
            &HashSet::new(),
        );
        assert_eq!(cartesian[0].signal_trace_count, 2);
        assert_eq!(cartesian[0].y_unit, "");
        assert_eq!(cartesian[0].traces[0].name, "re(V(out))");
        assert_eq!(cartesian[0].traces[0].y.as_slice(), &[0.8, 6.0]);
        assert_eq!(cartesian[0].traces[1].name, "im(V(out))");
        assert_eq!(cartesian[0].traces[1].y.as_slice(), &[0.6, 8.0]);

        let radians = build_models(
            &simulation,
            &mut derived,
            &Tokens::default(),
            false,
            ComplexNumberDisplay::MagnitudePhaseRadians,
            None,
            &HashSet::new(),
        );
        assert!(matches!(radians[0].traces[0].kind, TraceKind::MagnitudeDb));
        assert_eq!(radians[0].traces[0].y.as_slice(), &[0.0, 20.0]);
        assert!(matches!(radians[0].traces[1].kind, TraceKind::PhaseRad));
        assert!((radians[0].traces[1].y[0] - std::f64::consts::PI).abs() < 1e-12);
        assert!((radians[0].traces[1].y[1] - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
    }

    #[test]
    fn cursor_copy_uses_explicit_scientific_si_policy() {
        let mut state = AppState::default();
        state.simulation.start_run().add_analysis(
            AnalysisResult::new(1, AnalysisType::Noise, "Noise").with_waveforms(vec![
                WaveformData::new("onoise", vec![1.0, 10.0], vec![2.0e-18, 4.0e-18], "#fff"),
            ]),
        );
        state.ui.results.cursor_strip = Some(0);
        state.ui.results.cursors.a = Some(10.0);
        state
            .ui
            .preferences
            .set_choice(ChoicePreference::CopiedValueFormat, 1)
            .unwrap();

        let copied = copy_cursor_text(&mut state).expect("active cursor has copy data");

        assert!(copied.contains("A f = 1.00000000000000000e1 Hz"));
        assert!(copied.contains("onoise = 4."));
        assert!(copied.contains("e-18 V^2/Hz"));
    }
}
