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

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use egui::Ui;

use crate::analysis::calculator;
use crate::results::visualization_document::AccessibleColorPalette;
use crate::state::{
    AnalysisResult, AnalysisType, SharedWaveformValues, SimulationRun, SimulationState,
};
use crate::ui::icons::Icon;
use crate::ui::plot::{
    self, Axis, CursorPair, DisplayDecimation, PlotSpec, SampleInterpolation, Trace, XScale,
    fmt_si_significant, fmt_significant, sample_at_with,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{IconButton, chip, section_header};
use crate::workbench::AppState;
use crate::workbench::documents::visualization_family::{
    FamilyRenderGroup, FamilyRenderPlan, FamilyTraceStyle, SourceSampleSelection,
};
use crate::workbench::{
    ComplexNumberDisplay, CursorInterpolation, LargeDatasetDisplay, ResultPresentationPolicy,
};

use super::strip::{LegendChip, StripHeader};
use super::{
    AnalysisPresentationKey, DerivedSeries, ExprEditor, ExprSeries, ExprTrace,
    HorizontalWaveCursor, MarkerKind, ResultMarker, ResultsState, SelectedResultTrace,
    TracePresentationKey, WavePanePresentationKey, WaveformPresentationKey, WaveformSeriesResult,
    waveform_color, well_hint,
};

const WAVE_SHARED_X_HEIGHT: f32 = 57.0;

/// The shared-X strip's three bands, offset from the strip's top edge.
///
/// They are ordered by what each one describes. Tick labels and the A/B
/// flags both read the panes' zoomed viewport, so they sit directly against
/// the panes; the full-range overview lane is the outermost band, and
/// nothing viewport-scaled is ever drawn across it.
const SHARED_X_LABEL_TOP: f32 = 3.0;
const SHARED_X_FLAG_TOP: f32 = 17.0;
const SHARED_X_FLAG_HEIGHT: f32 = 13.0;
const SHARED_X_LANE_TOP: f32 = 35.0;
const SHARED_X_LANE_HEIGHT: f32 = 14.0;

/// The narrowest viewport an overview-handle drag can produce, as a fraction
/// of the full retained sweep. It is the reciprocal of the zoom ceiling, so
/// dragging a handle cannot reach a magnification the zoom controls refuse.
const SHARED_X_MIN_WINDOW: f64 = 1.0 / 200.0;
// Mockup gutter geometry: a 64 px left gutter carries the Y ticks and the
// X-strip's band labels; the right edge keeps only a 14 px breathing strip
// now that no pane owns a secondary axis.
const WAVE_SHARED_LEFT_MARGIN: f32 = 64.0;
const WAVE_SHARED_RIGHT_MARGIN: f32 = 14.0;

/// The mockup's per-sheet left gutter: the noise sheet's nV/√Hz tick
/// labels need 88 px where the shared 64 px gutter suffices elsewhere.
fn wave_left_margin(results: &ResultsState) -> f32 {
    match results.viewer {
        super::ResultViewer::NoiseContrib => 88.0,
        _ => WAVE_SHARED_LEFT_MARGIN,
    }
}
const WAVE_PANE_HEADER_HEIGHT: f32 = 25.0;
const WAVE_MIN_PLOT_HEIGHT: f32 = 24.0;

/// How a trace's Y values are interpreted.
#[derive(Clone, Copy, PartialEq, Eq)]
enum TraceKind {
    /// Plain values (V, A, sweep output).
    Value,
    /// dB-converted AC magnitude.
    MagnitudeDb,
    /// Phase in degrees (dashed, own stacked pane).
    PhaseDeg,
    /// Phase in radians (dashed, own stacked pane).
    PhaseRad,
    /// Original real component of a complex source quantity.
    Real,
    /// Original imaginary component of a complex source quantity.
    Imaginary,
    /// nV/√Hz projection of a retained V²/Hz noise PSD.
    NoiseDensity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct FamilyTraceVisibilityKey {
    analysis: AnalysisPresentationKey,
    group_key: u64,
    source_name_hash: u64,
    trace_kind: u8,
}

impl FamilyTraceVisibilityKey {
    fn new(
        analysis: AnalysisPresentationKey,
        group_key: u64,
        source_name: &str,
        trace_kind: TraceKind,
    ) -> Self {
        Self {
            analysis,
            group_key,
            source_name_hash: stable_hash(&source_name),
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
    analysis_key: AnalysisPresentationKey,
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

impl StripModel {
    #[cfg(test)]
    pub(super) const fn analysis_type(&self) -> AnalysisType {
        self.analysis_type
    }
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
    viewer: super::ResultViewer,
    phase_continuous: bool,
    complex_display: ComplexNumberDisplay,
    selection: Option<&SourceSampleSelection>,
    hidden_family_traces: &HashSet<FamilyTraceVisibilityKey>,
    t: &Tokens,
) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    simulation.data_version.hash(&mut h);
    viewer.hash(&mut h);
    phase_continuous.hash(&mut h);
    complex_display.hash(&mut h);
    selection
        .map(SourceSampleSelection::fingerprint)
        .hash(&mut h);
    let mut hidden = hidden_family_traces.iter().copied().collect::<Vec<_>>();
    hidden.sort_unstable_by_key(stable_hash);
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
    results.reconcile_expression_projection(simulation);
    let fp = models_fingerprint(
        simulation,
        results.viewer,
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
    let mut built = build_models(
        simulation,
        &mut results.derived,
        t,
        results.phase_continuous,
        complex_display,
        results.sample_selection.as_ref(),
        &results.hidden_family_traces,
    );
    built.retain(|model| match results.viewer {
        super::ResultViewer::DcSweep => model.analysis_type == AnalysisType::DcSweep,
        super::ResultViewer::Waves => model.analysis_type == AnalysisType::Transient,
        super::ResultViewer::Bode => model.analysis_type == AnalysisType::Ac,
        super::ResultViewer::NoiseContrib => model.analysis_type == AnalysisType::Noise,
        _ => true,
    });
    let models = Arc::new(built);
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

fn stable_hash(value: &impl std::hash::Hash) -> u64 {
    use std::hash::Hasher;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
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
            // A strip mixing voltages and currents has no single Y unit, so
            // a value is named by the unit its own signal is measured in —
            // the same unit that decided which pane draws it.
            TraceKind::Value | TraceKind::Real | TraceKind::Imaginary => {
                fmt_in_unit(value, self.trace_unit(trace), significant_digits)
            }
            TraceKind::MagnitudeDb => fmt_significant(value, significant_digits, " dB"),
            TraceKind::PhaseDeg => {
                quantity_policy.format_angle(value.to_radians(), significant_digits)
            }
            TraceKind::PhaseRad => quantity_policy.format_angle(value, significant_digits),
            TraceKind::NoiseDensity => fmt_in_unit(value, NOISE_DENSITY_UNIT, significant_digits),
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

    pub(super) fn analysis_key(&self) -> AnalysisPresentationKey {
        self.analysis_key
    }

    pub(super) fn trace_presentation_key(&self, index: usize) -> Option<TracePresentationKey> {
        self.traces.get(index).map(trace_presentation_key)
    }

    pub(super) fn trace_index_for_key(&self, key: &TracePresentationKey) -> Option<usize> {
        let mut matching = self
            .traces
            .iter()
            .take(self.signal_trace_count)
            .enumerate()
            .filter(|(_, trace)| trace_presentation_key(trace) == *key);
        let (index, _) = matching.next()?;
        matching.next().is_none().then_some(index)
    }

    /// The unit one trace is measured in.
    fn trace_unit(&self, trace: &StripTrace) -> &'static str {
        signal_unit(&trace.base_name, trace.kind, self.y_unit)
    }

    /// The strip's panes: retained signal identities grouped by unit, in the
    /// order the units first appear so a strip's layout is stable across
    /// visibility changes. A pane with no visible trace remains as the exact
    /// owner of its hidden signals so its Add signal control can restore one.
    ///
    /// Phase owns a stacked pane of its own — the mockup's Bode instrument
    /// reads magnitude over phase as two weighted panes sharing one X
    /// domain, not a second axis on the magnitude pane. Phase panes order
    /// after the quantity panes so magnitude keeps the primary slot.
    pub(super) fn unit_panes(&self) -> Vec<UnitPane> {
        let mut panes: Vec<UnitPane> = Vec::new();
        for (index, trace) in self.traces.iter().enumerate() {
            let unit = self.trace_unit(trace);
            match panes.iter_mut().find(|pane| pane.unit == unit) {
                Some(pane) => {
                    if trace.visible {
                        pane.traces.push(index);
                    }
                }
                None => panes.push(UnitPane {
                    unit,
                    traces: trace.visible.then_some(index).into_iter().collect(),
                }),
            }
        }
        let (quantity, phase): (Vec<_>, Vec<_>) = panes
            .into_iter()
            .partition(|pane| !matches!(pane.unit, "°" | "rad"));
        quantity.into_iter().chain(phase).collect()
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

fn append_projected_traces(
    traces: &mut Vec<StripTrace>,
    derived: &mut DerivedSeries,
    analysis_key: AnalysisPresentationKey,
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
        let derived_key = stable_hash(&(analysis_key, source_waveform_name, kind as u8))
            ^ selection_key
            ^ presentation_key.rotate_left(23);
        let y = match kind {
            TraceKind::MagnitudeDb => derived.db(derived_key, &projection.y),
            TraceKind::NoiseDensity => derived.noise_density_nv(derived_key, &projection.y),
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
        let family_visibility_key = projection.group.map(|group| {
            FamilyTraceVisibilityKey::new(
                analysis_key,
                group.stable_key,
                source_waveform_name,
                kind,
            )
        });
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
        let analysis_key = AnalysisPresentationKey::new(run.dataset_id, analysis);
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
                            analysis_key,
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
            } else if analysis.analysis_type == AnalysisType::Noise {
                // Retained noise PSDs are V²/Hz; the pane reads nV/√Hz like
                // the mockup's noise instrument.
                TraceKind::NoiseDensity
            } else {
                TraceKind::Value
            };
            append_projected_traces(
                &mut traces,
                derived,
                analysis_key,
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
                    TraceKind::NoiseDensity => {
                        derived.noise_density_nv(derived_key, &overlay_waveform.y)
                    }
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
            analysis_key,
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
    let continuous = trace.kind.is_phase() && model.phase_continuous;
    run_mixed_key(
        stable_hash(&(anchor_key(model, trace), continuous)),
        trace.run_id,
        trace.overlay,
    )
}

/// Dataset-bound identity of a trace: what a marker anchors to.
///
/// The key follows its retained source through analysis/waveform reordering,
/// but includes the immutable dataset so an annotation can never silently
/// migrate to a later solve that happens to reuse the same signal name.
fn trace_presentation_key(trace: &StripTrace) -> TracePresentationKey {
    TracePresentationKey {
        source_name: trace.source_waveform_name.clone(),
        kind: trace.kind as u8,
        family_group: trace.presentation_key,
    }
}

fn anchor_key(model: &StripModel, trace: &StripTrace) -> WaveformPresentationKey {
    WaveformPresentationKey {
        analysis: model.analysis_key,
        trace: trace_presentation_key(trace),
    }
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
/// The noise sheet reports amplitude density in a unit that already carries
/// its own SI prefix.
const NOISE_DENSITY_UNIT: &str = "nV/√Hz";

/// Format a value in a pane's unit.
///
/// A unit that already carries an SI prefix takes no second one: 1.79 µV/√Hz
/// of output noise reads as `1786.13 nV/√Hz`, never as `1.78613 knV/√Hz`.
fn fmt_in_unit(value: f64, unit: &str, significant_digits: usize) -> String {
    if unit == NOISE_DENSITY_UNIT {
        return fmt_significant(value, significant_digits, " nV/√Hz");
    }
    fmt_si_significant(value, unit, significant_digits)
}

fn signal_unit(name: &str, kind: TraceKind, analysis_unit: &'static str) -> &'static str {
    match kind {
        TraceKind::MagnitudeDb => "dB",
        TraceKind::PhaseDeg => "°",
        TraceKind::PhaseRad => "rad",
        TraceKind::NoiseDensity => "nV/√Hz",
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

/// Unit for a raw dataset waveform name in the results data browser, which
/// carries no trace-kind projection: the name's accessor decides, however
/// wrapped, and the analysis default applies only where there is none.
pub(crate) fn browser_signal_unit(name: &str, analysis_unit: &'static str) -> &'static str {
    let name = name.trim_start();
    if starts_with_accessor(name, "phase(") {
        return "°";
    }
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

/// Whether a raw dataset waveform name reads a current, however wrapped.
pub(crate) fn browser_signal_is_current(name: &str) -> bool {
    let name = name.trim_start();
    let name = if starts_with_accessor(name, "phase(") {
        name["phase(".len()..].trim_start()
    } else {
        name
    };
    starts_with_accessor(unwrap_projection(name), "i(")
}

/// One Y axis of a strip: the traces measured in a single unit.
///
/// Panes of a strip always share the strip's X domain — they are one
/// measurement read against several scales, not several plots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct UnitPane {
    /// The unit every trace on this pane's axis is measured in.
    pub unit: &'static str,
    /// Trace indices on this pane's axis.
    pub traces: Vec<usize>,
}

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

fn model_is_visible(model: &StripModel, models: &[StripModel], results: &ResultsState) -> bool {
    match results.maximized_strip {
        Some(maximized) if models.iter().any(|item| item.analysis_key == maximized) => {
            model.analysis_key == maximized
        }
        _ => !results.hidden_strips.contains(&model.analysis_key),
    }
}

fn active_pane<'a>(
    models: &'a [StripModel],
    results: &ResultsState,
) -> Option<(&'a StripModel, usize, UnitPane)> {
    let active = results.active_wave_pane.as_ref()?;
    let model = models
        .iter()
        .find(|model| model.analysis_key == active.analysis)
        .filter(|model| model_is_visible(model, models, results))?;
    model
        .unit_panes()
        .into_iter()
        .enumerate()
        .find(|(_, pane)| pane.unit == active.unit)
        .map(|(ordinal, pane)| (model, ordinal, pane))
}

/// Keep the instrument strip bound to an exact, currently visible waveform
/// pane. Stable analysis identity and engineering unit are used instead of
/// transient pane ordinals, so hiding or reordering a strip cannot redirect an
/// action to unrelated data.
pub(super) fn reconcile_active_pane(state: &mut AppState, t: &Tokens) {
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        t,
    );
    if active_pane(&models, &state.ui.results).is_some() {
        return;
    }

    let preferred_analysis = state
        .ui
        .results
        .valid_selected_trace(&state.simulation)
        .map(SelectedResultTrace::analysis_key)
        .or_else(|| {
            state.ui.results.cursor_strip.and_then(|index| {
                models
                    .iter()
                    .find(|model| model.analysis_index == index)
                    .map(|model| model.analysis_key)
            })
        });
    let next = preferred_analysis
        .and_then(|analysis| {
            models.iter().find(|model| {
                model.analysis_key == analysis
                    && model_is_visible(model, &models, &state.ui.results)
            })
        })
        .or_else(|| {
            models
                .iter()
                .find(|model| model_is_visible(model, &models, &state.ui.results))
        })
        .and_then(|model| {
            model
                .unit_panes()
                .first()
                .map(|pane| WavePanePresentationKey {
                    analysis: model.analysis_key,
                    unit: pane.unit.to_owned(),
                })
        });
    state.ui.results.active_wave_pane = next;
}

fn matching_spec_limits(
    state: &AppState,
    model: &StripModel,
    pane: &UnitPane,
    t: &Tokens,
) -> Vec<plot::LimitLine> {
    let mut seen = HashSet::new();
    let mut limits = Vec::new();
    for trace in pane
        .traces
        .iter()
        .filter_map(|index| model.traces.get(*index))
        .filter(|trace| trace.visible && !trace.overlay)
    {
        for specification in state.workspace.specs.iter().filter(|specification| {
            specification
                .measurement
                .eq_ignore_ascii_case(&trace.source_waveform_name)
        }) {
            if let Some(minimum) = specification.min.filter(|value| value.is_finite())
                && seen.insert((
                    specification.measurement.to_ascii_lowercase(),
                    0_u8,
                    minimum.to_bits(),
                ))
            {
                limits.push(plot::LimitLine {
                    y: minimum,
                    color: t.color.warn,
                    label: format!(
                        "{} \u{2265} {}",
                        specification.measurement,
                        fmt_si_significant(minimum, &specification.unit, 6)
                    ),
                });
            }
            if let Some(maximum) = specification.max.filter(|value| value.is_finite())
                && seen.insert((
                    specification.measurement.to_ascii_lowercase(),
                    1_u8,
                    maximum.to_bits(),
                ))
            {
                limits.push(plot::LimitLine {
                    y: maximum,
                    color: t.color.warn,
                    label: format!(
                        "{} \u{2264} {}",
                        specification.measurement,
                        fmt_si_significant(maximum, &specification.unit, 6)
                    ),
                });
            }
        }
    }
    limits
}

pub(super) fn spec_limits_available(state: &mut AppState, t: &Tokens) -> bool {
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        t,
    );
    active_pane(&models, &state.ui.results)
        .is_some_and(|(model, _, pane)| !matching_spec_limits(state, model, &pane, t).is_empty())
}

#[derive(Debug)]
struct FamilyEnvelopeSeries {
    x: Vec<f64>,
    minimum: Vec<f64>,
    maximum: Vec<f64>,
    color: egui::Color32,
    minimum_cache_key: u64,
    maximum_cache_key: u64,
}

fn family_envelope_series(model: &StripModel, pane: &UnitPane) -> Vec<FamilyEnvelopeSeries> {
    let mut groups = HashMap::<(String, u8), Vec<&StripTrace>>::new();
    for trace in pane
        .traces
        .iter()
        .filter_map(|index| model.traces.get(*index))
        .filter(|trace| trace.visible && !trace.overlay && trace.family_group_ordinal.is_some())
    {
        groups
            .entry((trace.source_waveform_name.clone(), trace.kind as u8))
            .or_default()
            .push(trace);
    }

    let mut envelopes = Vec::new();
    for ((source_name, kind), traces) in groups {
        let family_groups = traces
            .iter()
            .map(|trace| trace.presentation_key)
            .collect::<HashSet<_>>();
        if family_groups.len() < 2 {
            continue;
        }

        let mut points = HashMap::<u64, (f64, f64, f64, usize)>::new();
        for trace in &traces {
            for (&x, &y) in trace.x.iter().zip(trace.y.iter()) {
                if !x.is_finite() || !y.is_finite() {
                    continue;
                }
                points
                    .entry(x.to_bits())
                    .and_modify(|(_, minimum, maximum, count)| {
                        *minimum = minimum.min(y);
                        *maximum = maximum.max(y);
                        *count += 1;
                    })
                    .or_insert((x, y, y, 1));
            }
        }
        let mut points = points
            .into_values()
            .filter(|(_, _, _, count)| *count >= 2)
            .collect::<Vec<_>>();
        points.sort_by(|left, right| left.0.total_cmp(&right.0));
        if points.is_empty() {
            continue;
        }

        let presentation_keys = traces
            .iter()
            .map(|trace| trace.presentation_key)
            .collect::<Vec<_>>();
        let identity = stable_hash(&(model.analysis_key, source_name, kind, presentation_keys));
        envelopes.push(FamilyEnvelopeSeries {
            x: points.iter().map(|point| point.0).collect(),
            minimum: points.iter().map(|point| point.1).collect(),
            maximum: points.iter().map(|point| point.2).collect(),
            color: traces[0].signal_color.gamma_multiply(0.78),
            minimum_cache_key: identity ^ 0x1357_9BDF_2468_ACE0,
            maximum_cache_key: identity ^ 0x0246_8ACE_1357_9BDF,
        });
    }
    envelopes
}

pub(super) fn family_envelope_available(state: &mut AppState, t: &Tokens) -> bool {
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        t,
    );
    active_pane(&models, &state.ui.results)
        .is_some_and(|(model, _, pane)| !family_envelope_series(model, &pane).is_empty())
}

fn cursor_marker_target(
    state: &AppState,
    models: &[StripModel],
) -> Option<(
    AnalysisPresentationKey,
    WaveformPresentationKey,
    String,
    f64,
)> {
    let cursor_x = state
        .ui
        .results
        .cursors
        .a
        .filter(|value| value.is_finite())?;
    let (model, _, pane) = active_pane(models, &state.ui.results)?;
    let pane_traces = pane
        .traces
        .iter()
        .filter_map(|index| model.traces.get(*index))
        .filter(|trace| trace.visible && !trace.overlay)
        .collect::<Vec<_>>();
    let selected_source = state
        .ui
        .results
        .valid_selected_trace(&state.simulation)
        .filter(|selected| selected.analysis_key() == model.analysis_key)
        .map(SelectedResultTrace::source_name);
    let trace = state
        .ui
        .results
        .cursor_a_anchor
        .as_ref()
        .filter(|anchor| anchor.analysis == model.analysis_key)
        .and_then(|anchor| {
            pane_traces
                .iter()
                .copied()
                .find(|trace| anchor_key(model, trace) == *anchor)
        })
        .or_else(|| {
            selected_source.and_then(|source| {
                pane_traces
                    .iter()
                    .copied()
                    .find(|trace| trace.source_waveform_name == source)
            })
        })
        .or_else(|| pane_traces.first().copied())?;
    let sampled = sample_at_with(
        &trace.x,
        &trace.y,
        cursor_x,
        cursor_interpolation(
            state
                .ui
                .preferences
                .result_presentation_policy()
                .cursor_interpolation(),
        ),
    );
    sampled.is_finite().then(|| {
        (
            model.analysis_key,
            anchor_key(model, trace),
            trace.name.clone(),
            cursor_x,
        )
    })
}

pub(super) fn marker_at_cursor_a_available(state: &mut AppState, t: &Tokens) -> bool {
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        t,
    );
    cursor_marker_target(state, &models).is_some()
}

pub(super) fn drop_marker_at_cursor_a(state: &mut AppState, t: &Tokens) {
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        t,
    );
    let Some((analysis, anchor, trace_name, x)) = cursor_marker_target(state, &models) else {
        return;
    };
    let marker = state.ui.results.add_marker(analysis, anchor, trace_name, x);
    state.ui.results.editing_marker = Some(marker);
}

fn scaled_range(range: (f64, f64), factor: f64, logarithmic: bool) -> Option<(f64, f64)> {
    if !factor.is_finite() || factor <= 0.0 || !range.0.is_finite() || !range.1.is_finite() {
        return None;
    }
    if logarithmic {
        if range.0 <= 0.0 || range.1 <= range.0 {
            return None;
        }
        let low = range.0.log10();
        let high = range.1.log10();
        let center = (low + high) * 0.5;
        let half_span = (high - low) * 0.5 * factor;
        return Some((
            10.0_f64.powf(center - half_span),
            10.0_f64.powf(center + half_span),
        ));
    }
    if range.1 <= range.0 {
        return None;
    }
    let center = (range.0 + range.1) * 0.5;
    let half_span = (range.1 - range.0) * 0.5 * factor;
    Some((center - half_span, center + half_span))
}

pub(super) fn zoom_active_pane(state: &mut AppState, t: &Tokens, factor: f64) {
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        t,
    );
    let Some((model, ordinal, pane)) = active_pane(&models, &state.ui.results) else {
        return;
    };
    let current = state.ui.results.analysis_plot_view_pane(
        super::ResultViewer::Waves,
        model.analysis_key,
        ordinal,
    );
    let x = current
        .x
        .or_else(|| x_range(model))
        .and_then(|range| scaled_range(range, factor, model.x_scale == XScale::Log10));
    let y = current
        .y
        .or_else(|| pane_y_range(&mut state.ui.results.derived, model, &pane.traces))
        .and_then(|range| scaled_range(range, factor, false));
    let view = state.ui.results.analysis_plot_view_pane_mut(
        super::ResultViewer::Waves,
        model.analysis_key,
        ordinal,
    );
    if let Some(x) = x {
        view.x = Some(x);
    }
    if let Some(y) = y {
        view.y = Some(y);
    }
}

pub(super) fn fit_active_pane(state: &mut AppState, t: &Tokens) {
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        t,
    );
    let Some((model, ordinal, _)) = active_pane(&models, &state.ui.results) else {
        return;
    };
    state.ui.results.reset_analysis_plot_view_pane(
        super::ResultViewer::Waves,
        model.analysis_key,
        ordinal,
    );
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

/// The Bode sheet: the run's AC response through the same pane-stack
/// instrument, which [`StripModel::unit_panes`] reads as the mockup's two
/// weighted panes — magnitude over phase on one shared X domain. The AC
/// scope rides the viewer-aware model cache; stability margins stay in the
/// right panel's inspector card.
pub fn show_bode(ui: &mut Ui, state: &mut AppState) {
    show_with_pane_chrome(ui, state, true);
}

/// The Noise sheet: retained noise PSDs as an nV/√Hz pane through the same
/// instrument, scoped to Noise analyses by the viewer-aware model cache.
/// The spectrum summary card stays in the right panel.
pub fn show_noise(ui: &mut Ui, state: &mut AppState) {
    show_with_pane_chrome(ui, state, true);
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
        Some(max_key) if models.iter().any(|m| m.analysis_key == max_key) => models
            .iter()
            .filter(|m| m.analysis_key == max_key)
            .collect(),
        _ => models
            .iter()
            .filter(|m| !results.hidden_strips.contains(&m.analysis_key))
            .collect(),
    };
    if visible.is_empty() {
        well_hint(ui, "All strips hidden — restore them from the document bar");
        return;
    }

    // Deferred state mutations (collected while iterating immutably).
    let mut toggle_maximize: Option<AnalysisPresentationKey> = None;
    let mut close_strip: Option<AnalysisPresentationKey> = None;
    let mut fit_strip: Option<AnalysisPresentationKey> = None;
    let mut toggle_expr: Option<(AnalysisPresentationKey, usize)> = None;
    let mut remove_expr: Option<(AnalysisPresentationKey, usize)> = None;
    let mut open_editor: Option<AnalysisPresentationKey> = None;
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
                        // Analysis identity remains in the strip header.
                        // Unit-owned signal legends live in the pane headers;
                        // only unitless expressions remain alongside the
                        // analysis identity here.
                        let strip_exprs: Vec<ExprTrace> = state
                            .ui
                            .results
                            .analysis_exprs
                            .get(&model.analysis_key)
                            .cloned()
                            .unwrap_or_default();
                        let expr_labels: Vec<String> =
                            strip_exprs.iter().map(|e| elide(&e.text, 24)).collect();
                        let legend: Vec<LegendChip<'_>> = strip_exprs
                            .iter()
                            .enumerate()
                            .map(|(i, expr)| LegendChip {
                                name: &expr_labels[i],
                                color: expr_color(&t, model.signal_trace_count + i),
                                on: expr.visible,
                            })
                            .collect();

                        let zoomed = state.ui.results.analysis_strip_is_zoomed(
                            super::ResultViewer::Waves,
                            model.analysis_key,
                        );
                        let header = StripHeader::new(&model.kind_tag, &model.subtitle, &legend)
                            .maximized(maximized)
                            .closable(pane_chrome && !maximized && n > 1)
                            .zoomed(zoomed)
                            .expr_action(pane_chrome)
                            .removable_from(0)
                            .pane_actions(pane_chrome)
                            .show(ui);
                        if let Some(chip_index) = header.legend_clicked {
                            toggle_expr = Some((model.analysis_key, chip_index));
                        }
                        if let Some(chip_index) = header.legend_visibility_clicked {
                            toggle_expr = Some((model.analysis_key, chip_index));
                        }
                        if let Some(chip_index) = header.legend_removed {
                            remove_expr = Some((model.analysis_key, chip_index));
                        }
                        if header.maximize_clicked {
                            toggle_maximize = Some(model.analysis_key);
                        }
                        if header.close_clicked {
                            close_strip = Some(model.analysis_key);
                        }
                        if header.fit_clicked {
                            fit_strip = Some(model.analysis_key);
                        }
                        if header.add_expr_clicked {
                            open_editor = Some(model.analysis_key);
                        }

                        expr_editor_row(ui, state, model.analysis_key, model.analysis_index);

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
    let results = &mut state.ui.results;
    if let Some(idx) = toggle_maximize {
        results.maximized_strip = (results.maximized_strip != Some(idx)).then_some(idx);
    }
    if let Some(idx) = close_strip {
        results.hidden_strips.insert(idx);
        if models
            .iter()
            .find(|model| model.analysis_key == idx)
            .is_some_and(|model| results.cursor_strip == Some(model.analysis_index))
        {
            results.clear_cursors();
        }
    }
    if let Some(key) = fit_strip {
        results.reset_analysis_plot_view(super::ResultViewer::Waves, key);
    }
    if let Some((analysis, index)) = toggle_expr
        && let Some(expr) = results
            .analysis_exprs
            .get_mut(&analysis)
            .and_then(|list| list.get_mut(index))
    {
        expr.visible = !expr.visible;
        if let Some(model) = models.iter().find(|model| model.analysis_key == analysis) {
            results.sync_expression_projection(analysis, model.analysis_index);
        }
    }
    if let Some((analysis, index)) = remove_expr
        && let Some(list) = results.analysis_exprs.get_mut(&analysis)
    {
        if index < list.len() {
            let removed = list.remove(index);
            results
                .analysis_expr_cache
                .remove(&(analysis, removed.text));
        }
        if list.is_empty() {
            results.analysis_exprs.remove(&analysis);
        }
        if let Some(model) = models.iter().find(|model| model.analysis_key == analysis) {
            results.sync_expression_projection(analysis, model.analysis_index);
        }
    }
    if let Some(analysis) = open_editor {
        results.expr_editor = Some(ExprEditor {
            analysis,
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
            TraceKind::NoiseDensity => policy.copy_si_value(value, "nV/√Hz"),
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

fn model_x_axis(
    model: &StripModel,
    x0: f64,
    x1: f64,
    quantity_policy: crate::quantity::QuantityPresentationPolicy,
) -> Axis {
    let axis = match model.x_scale {
        XScale::Log10 => Axis::log_decades(x0, x1, &model.x_unit),
        XScale::Linear => Axis::linear(x0, x1, &model.x_unit),
    }
    .with_label(&model.x_label);
    if model.x_unit == "Hz" {
        let (scale, offset, unit) = quantity_policy.frequency_axis_transform();
        axis.with_display_transform(scale, offset, unit)
    } else {
        axis
    }
}

fn shared_x_view(
    results: &ResultsState,
    analysis: AnalysisPresentationKey,
    pane_count: usize,
) -> Option<(f64, f64)> {
    (0..pane_count).find_map(|ordinal| {
        results
            .analysis_plot_view_pane(super::ResultViewer::Waves, analysis, ordinal)
            .x
    })
}

fn set_shared_x_view(
    results: &mut ResultsState,
    analysis: AnalysisPresentationKey,
    pane_count: usize,
    range: Option<(f64, f64)>,
) {
    for ordinal in 0..pane_count {
        results
            .analysis_plot_view_pane_mut(super::ResultViewer::Waves, analysis, ordinal)
            .x = range;
    }
}

fn shared_axis_viewport_fraction(scale: XScale, full: (f64, f64), view: (f64, f64)) -> (f64, f64) {
    let start = scale.normalize(view.0, full.0, full.1).clamp(0.0, 1.0);
    let end = scale.normalize(view.1, full.0, full.1).clamp(0.0, 1.0);
    (start.min(end), start.max(end))
}

fn panned_shared_x_view(
    scale: XScale,
    full: (f64, f64),
    view: (f64, f64),
    fraction_delta: f64,
) -> Option<(f64, f64)> {
    if !fraction_delta.is_finite() {
        return None;
    }
    let (start, end) = shared_axis_viewport_fraction(scale, full, view);
    let width = end - start;
    if !(width > 0.0 && width < 1.0) {
        return None;
    }
    let next_start = (start + fraction_delta).clamp(0.0, 1.0 - width);
    let next_end = next_start + width;
    Some((
        scale.denormalize(next_start, full.0, full.1),
        scale.denormalize(next_end, full.0, full.1),
    ))
}

fn zoomed_shared_x_view(
    scale: XScale,
    full: (f64, f64),
    view: (f64, f64),
    anchor_fraction: f64,
    factor: f64,
) -> Option<(f64, f64)> {
    if !anchor_fraction.is_finite() || !factor.is_finite() || factor <= 0.0 {
        return None;
    }
    let (start, end) = shared_axis_viewport_fraction(scale, full, view);
    let width = end - start;
    if width <= 0.0 {
        return None;
    }
    let anchor = anchor_fraction.clamp(0.0, 1.0);
    let relative = ((anchor - start) / width).clamp(0.0, 1.0);
    let next_width = (width * factor).clamp(1.0e-6, 1.0);
    let next_start = (anchor - relative * next_width).clamp(0.0, 1.0 - next_width);
    let next_end = next_start + next_width;
    Some((
        scale.denormalize(next_start, full.0, full.1),
        scale.denormalize(next_end, full.0, full.1),
    ))
}

/// Resize the shared viewport by dragging one edge of the overview window.
///
/// The dragged edge follows the pointer and the opposite edge stays fixed,
/// so the gesture zooms and pans in one motion the way pulling a scrollbar
/// handle does.
fn resized_shared_x_view(
    scale: XScale,
    full: (f64, f64),
    view: (f64, f64),
    move_start: bool,
    edge_fraction: f64,
) -> Option<(f64, f64)> {
    if !edge_fraction.is_finite() {
        return None;
    }
    let (start, end) = shared_axis_viewport_fraction(scale, full, view);
    let edge = edge_fraction.clamp(0.0, 1.0);
    let (next_start, next_end) = if move_start {
        (edge.min(end - SHARED_X_MIN_WINDOW), end)
    } else {
        (start, edge.max(start + SHARED_X_MIN_WINDOW))
    };
    let next_start = next_start.clamp(0.0, 1.0 - SHARED_X_MIN_WINDOW);
    let next_end = next_end.clamp(next_start + SHARED_X_MIN_WINDOW, 1.0);
    Some((
        scale.denormalize(next_start, full.0, full.1),
        scale.denormalize(next_end, full.0, full.1),
    ))
}

/// The name the strip prints into the panes' left gutter beside the tick
/// values, following the mockup's axis vocabulary.
///
/// The tick values themselves are SI-prefixed bare numbers, so the gutter is
/// where the axis states its unit — once, for the whole row.
fn shared_x_gutter_label(unit: &str) -> String {
    let name = match unit {
        "s" => "TIME",
        "Hz" => "FREQ",
        _ => "X",
    };
    if unit.is_empty() {
        name.to_owned()
    } else {
        format!("{name} · {unit}")
    }
}

fn recentered_shared_x_view(
    scale: XScale,
    full: (f64, f64),
    view: (f64, f64),
    centre_fraction: f64,
) -> Option<(f64, f64)> {
    let (start, end) = shared_axis_viewport_fraction(scale, full, view);
    let width = end - start;
    if !(width > 0.0 && width < 1.0) || !centre_fraction.is_finite() {
        return None;
    }
    let next_start = (centre_fraction.clamp(0.0, 1.0) - width * 0.5).clamp(0.0, 1.0 - width);
    Some((
        scale.denormalize(next_start, full.0, full.1),
        scale.denormalize(next_start + width, full.0, full.1),
    ))
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct WaveStackGeometry {
    /// Height of one weight unit, before a pane's own weight is applied.
    pane_unit_height: f32,
    total_weight: f32,
    shared_x_height: f32,
    seam_height: f32,
}

/// The mockup's pane weights: the sheet's primary quantity takes three parts
/// and every companion pane two, so a supply-current or phase pane reads as
/// the secondary evidence it is instead of splitting the stack evenly.
fn pane_weight(ordinal: usize, pane_count: usize) -> f32 {
    if pane_count <= 1 {
        1.0
    } else if ordinal == 0 {
        3.0
    } else {
        2.0
    }
}

impl WaveStackGeometry {
    fn pane_height(&self, ordinal: usize, pane_count: usize) -> f32 {
        (self.pane_unit_height * pane_weight(ordinal, pane_count)).max(0.0)
    }
}

fn wave_stack_geometry(available_height: f32, pane_count: usize) -> WaveStackGeometry {
    if pane_count == 0 || !available_height.is_finite() {
        return WaveStackGeometry {
            pane_unit_height: 0.0,
            total_weight: 1.0,
            shared_x_height: 0.0,
            seam_height: 0.0,
        };
    }
    let available = available_height.max(0.0);
    let seam_count = pane_count.saturating_sub(1) as f32;
    let seam_height = if seam_count > 0.0 {
        (available / seam_count).min(1.0)
    } else {
        0.0
    };
    let content = (available - seam_height * seam_count).max(0.0);
    // The normal 50 px navigator is retained when space permits and shrinks
    // proportionally in constrained multi-pane/multi-strip arrangements.
    // The sum is exact: this function never asks the parent to grow.
    let shared_x_height = (content * 0.28).min(WAVE_SHARED_X_HEIGHT);
    let total_weight: f32 = (0..pane_count)
        .map(|ordinal| pane_weight(ordinal, pane_count))
        .sum();
    let pane_unit_height = ((content - shared_x_height) / total_weight).max(0.0);
    WaveStackGeometry {
        pane_unit_height,
        total_weight,
        shared_x_height,
        seam_height,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SharedXDrag {
    Viewport,
    ResizeStart,
    ResizeEnd,
    CursorA,
    CursorB,
}

fn shared_x_drag_id(model: &StripModel) -> egui::Id {
    egui::Id::new(("rspice.results.shared-x-drag", model.analysis_key))
}

/// What the inspector's Active pane section reports about the pane the
/// instrument is acting on.
///
/// The pane's identity, scale and limit binding live behind this module's
/// privacy, so the inspector asks for them rather than reaching in — and the
/// scale reader sits beside the header toggle that writes it.
pub(crate) struct ActivePaneFacts {
    /// The unit that names the pane in a unit-scoped stack.
    pub unit: Option<String>,
    /// The analysis the pane belongs to. A sheet that is not the waveform
    /// stack still draws exactly one analysis, and it is not necessarily the
    /// one the run's analysis selector points at.
    pub analysis: Option<String>,
    /// Visible and bound trace counts on this pane alone.
    pub traces: Option<(usize, usize)>,
    pub scale: Option<&'static str>,
    pub limit_mask: &'static str,
    pub x_viewport: Option<String>,
    pub y_viewport: Option<String>,
}

pub(crate) fn active_pane_facts(
    ctx: &egui::Context,
    tokens: &Tokens,
    state: &mut AppState,
) -> ActivePaneFacts {
    // The viewport the pane is actually showing, whether the user pinned it
    // or it is fitting the retained data. The mockup states the interval
    // either way: "automatic" alone does not tell a reader what they see.
    let (x_viewport, y_viewport) = active_pane_viewports(tokens, state);
    let (analysis, traces) = active_pane_identity(tokens, state);
    let key = state.ui.results.active_wave_pane.as_ref();
    let scale = key.map(|key| {
        let log = ctx
            .data_mut(|data| {
                data.get_persisted::<bool>(egui::Id::new((
                    "rspice.results.pane-log-y",
                    key.analysis,
                    key.unit.as_str(),
                )))
            })
            .unwrap_or(false);
        if log { "logarithmic" } else { "linear" }
    });
    ActivePaneFacts {
        unit: key.map(|key| key.unit.clone()),
        analysis,
        traces,
        scale,
        limit_mask: if state.ui.results.show_spec_limits {
            "project specification limits"
        } else {
            "none bound"
        },
        x_viewport,
        y_viewport,
    }
}

/// What the status bar reports about the sheet on the Results workspace.
pub(crate) struct SharedXStatus {
    /// The X interval the panes are showing.
    pub span: String,
    /// How far that interval is magnified from the full retained sweep.
    pub zoom: f64,
}

/// The visible X interval and its magnification.
///
/// The instrument bar owns the A/B cursor readout, so the status bar's
/// coordinate segment states the view rather than triplicating the cursors —
/// and its zoom chip reports this magnification instead of a canvas scale no
/// waveform sheet has.
pub(crate) fn active_shared_x_status(
    tokens: &Tokens,
    state: &mut AppState,
) -> Option<SharedXStatus> {
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let digits = usize::from(presentation.displayed_significant_digits().get());
    let active = state
        .ui
        .results
        .active_wave_pane
        .as_ref()
        .map(|key| key.analysis);
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        tokens,
    );
    let model = match active {
        Some(key) => models.iter().find(|model| model.analysis_key == key),
        None => models.first(),
    }?;
    let full = x_range(model)?;
    let panes = model.unit_panes().len();
    let view = shared_x_view(&state.ui.results, model.analysis_key, panes).unwrap_or(full);
    let (start, end) = shared_axis_viewport_fraction(model.x_scale, full, view);
    let width = end - start;
    Some(SharedXStatus {
        span: format!(
            "{} … {}",
            model.format_x(view.0, digits, quantity_policy),
            model.format_x(view.1, digits, quantity_policy)
        ),
        zoom: if width > 0.0 { 1.0 / width } else { 1.0 },
    })
}

/// The analysis the active pane belongs to and how many of its traces the
/// pane carries.
///
/// A sheet that is not the waveform stack picks its own analysis, so the
/// run's analysis selector is not the authority here: the pane is.
fn active_pane_identity(
    tokens: &Tokens,
    state: &mut AppState,
) -> (Option<String>, Option<(usize, usize)>) {
    let Some(key) = state.ui.results.active_wave_pane.clone() else {
        return (None, None);
    };
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        tokens,
    );
    let Some(model) = models
        .iter()
        .find(|model| model.analysis_key == key.analysis)
    else {
        return (None, None);
    };
    let bound = model
        .traces
        .iter()
        .filter(|trace| model.trace_unit(trace) == key.unit)
        .count();
    let visible = model
        .traces
        .iter()
        .filter(|trace| trace.visible && model.trace_unit(trace) == key.unit)
        .count();
    let label = state
        .simulation
        .active_run()
        .and_then(|run| run.analyses.get(model.analysis_index))
        .map(|analysis| analysis.label.clone());
    (label, Some((visible, bound)))
}

/// The active pane's X and Y intervals, formatted through the strip's own
/// formatter so they read like every other number on the sheet.
fn active_pane_viewports(
    tokens: &Tokens,
    state: &mut AppState,
) -> (Option<String>, Option<String>) {
    let Some(key) = state.ui.results.active_wave_pane.clone() else {
        return (None, None);
    };
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let digits = usize::from(presentation.displayed_significant_digits().get());
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        tokens,
    );
    let Some(model) = models
        .iter()
        .find(|model| model.analysis_key == key.analysis)
    else {
        return (None, None);
    };
    let panes = model.unit_panes();
    let Some((ordinal, pane)) = panes
        .iter()
        .enumerate()
        .find(|(_, pane)| pane.unit == key.unit)
    else {
        return (None, None);
    };
    let x = x_range(model).map(|full| {
        let (x0, x1) = shared_x_view(&state.ui.results, key.analysis, panes.len()).unwrap_or(full);
        format!(
            "{} … {}",
            model.format_x(x0, digits, quantity_policy),
            model.format_x(x1, digits, quantity_policy)
        )
    });
    let pinned = state
        .ui
        .results
        .analysis_plot_view_pane(super::ResultViewer::Waves, key.analysis, ordinal)
        .y;
    let y = pinned
        .or_else(|| pane_y_range(&mut state.ui.results.derived, model, &pane.traces))
        .map(|(y0, y1)| {
            format!(
                "{} … {}",
                fmt_in_unit(y0, pane.unit, digits),
                fmt_in_unit(y1, pane.unit, digits)
            )
        });
    (x, y)
}

fn pane_log_y_id(model: &StripModel, pane: &UnitPane) -> egui::Id {
    egui::Id::new(("rspice.results.pane-log-y", model.analysis_key, pane.unit))
}

fn pane_log_y(ui: &Ui, model: &StripModel, pane: &UnitPane) -> bool {
    ui.ctx()
        .data_mut(|data| data.get_persisted::<bool>(pane_log_y_id(model, pane)))
        .unwrap_or(false)
}

fn set_pane_log_y(ui: &Ui, model: &StripModel, pane: &UnitPane, enabled: bool) {
    ui.ctx()
        .data_mut(|data| data.insert_persisted(pane_log_y_id(model, pane), enabled));
}

fn trace_belongs_to_pane(
    model: &StripModel,
    pane: &UnitPane,
    ordinal: usize,
    trace: &StripTrace,
) -> bool {
    if !trace.kind.is_phase() {
        return model.trace_unit(trace) == pane.unit;
    }
    let has_magnitude = model
        .traces
        .iter()
        .any(|candidate| !candidate.kind.is_phase() && model.trace_unit(candidate) == "dB");
    if has_magnitude {
        pane.unit == "dB"
    } else {
        ordinal == 0
    }
}

#[derive(Default)]
struct UnitPaneHeaderResponse {
    autoscale_y: bool,
    toggle_log_y: bool,
}

fn show_unit_pane_header(
    ui: &mut Ui,
    state: &mut AppState,
    model: &StripModel,
    pane: &UnitPane,
    ordinal: usize,
    log_y: bool,
    log_y_available: bool,
    height: f32,
) -> UnitPaneHeaderResponse {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let height = height.min(WAVE_PANE_HEADER_HEIGHT).max(0.0);
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, c.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border),
    );
    if height < 18.0 {
        return UnitPaneHeaderResponse::default();
    }

    let pane_key = WavePanePresentationKey {
        analysis: model.analysis_key,
        unit: pane.unit.to_owned(),
    };
    let active = state.ui.results.active_wave_pane.as_ref() == Some(&pane_key);
    let action_width = 58.0;
    // Fit the pane's actual unit tag: nV/√Hz is wider than the quantity tags
    // the old fixed 46 px assumed, and a clipped unit misreads as a new unit.
    let unit_label_width = ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap(
                pane.unit.to_owned(),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                c.text,
            )
            .size()
            .x
    });
    let unit_width = (unit_label_width + 28.0).max(46.0);
    let unit_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 5.0, rect.top() + 1.5),
        egui::pos2(
            (rect.left() + unit_width).min(rect.right()),
            rect.bottom() - 1.5,
        ),
    );
    let actions_rect = egui::Rect::from_min_max(
        egui::pos2(
            (rect.right() - action_width).max(unit_rect.right()),
            rect.top(),
        ),
        rect.right_bottom(),
    );
    let legend_rect = egui::Rect::from_min_max(
        egui::pos2(unit_rect.right() + 3.0, rect.top()),
        egui::pos2(
            (actions_rect.left() - 3.0).max(unit_rect.right() + 3.0),
            rect.bottom(),
        ),
    );

    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(unit_rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
        |ui| {
            ui.set_clip_rect(unit_rect);
            let response = chip(ui, pane.unit, active)
                .on_hover_text(format!("{} unit-scoped Y axis", pane.unit));
            if response.clicked() {
                state.ui.results.active_wave_pane = Some(pane_key.clone());
            }
        },
    );

    // Cursor A only reads out on the strip it was placed on; another strip's
    // chips must not imply a value at an X they never sampled.
    let cursor_a_value = (state.ui.results.cursor_readout_active()
        && state.ui.results.cursor_strip == Some(model.analysis_index))
    .then(|| {
        state.ui.results.cursors.a.map(|x| {
            (
                x,
                state.ui.preferences.result_presentation_policy(),
                state.ui.preferences.quantity_presentation_policy(),
            )
        })
    })
    .flatten();

    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(legend_rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
        |ui| {
            ui.set_clip_rect(legend_rect);
            egui::ScrollArea::horizontal()
                .id_salt(("rspice.results.pane-legend", model.analysis_key, pane.unit))
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.x = 3.0;
                    let traces = model
                        .traces
                        .iter()
                        .take(model.signal_trace_count)
                        .enumerate()
                        .filter(|(_, trace)| trace_belongs_to_pane(model, pane, ordinal, trace));
                    for (_, trace) in traces {
                        let selected = state
                            .ui
                            .results
                            .valid_selected_trace(&state.simulation)
                            .is_some_and(|selected| {
                                selected.analysis_key() == model.analysis_key
                                    && selected.source_name() == trace.source_waveform_name
                            });
                        let (swatch, swatch_response) =
                            ui.allocate_exact_size(egui::vec2(13.0, 19.0), egui::Sense::click());
                        ui.painter().hline(
                            egui::Rangef::new(swatch.left() + 1.0, swatch.right() - 1.0),
                            swatch.center().y,
                            egui::Stroke::new(2.0, trace.color),
                        );
                        swatch_response.widget_info(|| {
                            egui::WidgetInfo::selected(
                                egui::WidgetType::Button,
                                true,
                                trace.visible,
                                format!("Toggle {} visibility", trace.name),
                            )
                        });
                        if swatch_response.clicked() {
                            if let Some(key) = trace.family_visibility_key {
                                state.ui.results.toggle_family_trace_visibility(key);
                            } else {
                                toggle_visibility(
                                    state,
                                    model.analysis_index,
                                    trace.waveform_index,
                                );
                            }
                        }
                        // The instrument idiom: a trace states its own value
                        // at cursor A right where its name is, so reading one
                        // curve never costs a trip to the readout register.
                        // A corner family draws one chip for the whole group,
                        // so the chip states how many traces it stands for.
                        let family = trace.family_group_ordinal.map(|_| {
                            model
                                .traces
                                .iter()
                                .filter(|candidate| {
                                    candidate.presentation_key == trace.presentation_key
                                        && candidate.family_group_ordinal.is_some()
                                })
                                .count()
                        });
                        let name = elide(&trace.name, 20);
                        let label = match &cursor_a_value {
                            Some((x, presentation, quantity_policy)) if trace.visible => {
                                let value = sample_at_with(
                                    &trace.x,
                                    &trace.y,
                                    *x,
                                    cursor_interpolation(presentation.cursor_interpolation()),
                                );
                                format!(
                                    "{}  {}",
                                    elide(&trace.name, 16),
                                    model.format_trace_value(
                                        trace,
                                        value,
                                        usize::from(
                                            presentation.displayed_significant_digits().get()
                                        ),
                                        *quantity_policy,
                                    )
                                )
                            }
                            _ => name,
                        };
                        let label = match family {
                            Some(count) if count > 1 => format!("{label}  ×{count}"),
                            _ => label,
                        };
                        // A hidden trace keeps its chip so it can be brought
                        // back, but must not read as a curve on the canvas.
                        let label = if trace.visible {
                            egui::RichText::new(label)
                        } else {
                            egui::RichText::new(label)
                                .strikethrough()
                                .color(c.text_faint)
                        };
                        if ui
                            .selectable_label(selected, label)
                            .on_hover_text(&trace.name)
                            .clicked()
                        {
                            state.ui.results.selected_trace =
                                Some(SelectedResultTrace::from_identity(
                                    model.analysis_key,
                                    trace.source_waveform_name.clone(),
                                ));
                            state.ui.results.active_wave_pane = Some(pane_key.clone());
                        }
                    }
                    ui.menu_button("+", |ui| {
                        let mut any = false;
                        for trace in
                            model
                                .traces
                                .iter()
                                .take(model.signal_trace_count)
                                .filter(|trace| {
                                    !trace.visible
                                        && trace_belongs_to_pane(model, pane, ordinal, trace)
                                })
                        {
                            any = true;
                            if ui.button(&trace.name).clicked() {
                                if let Some(key) = trace.family_visibility_key {
                                    state.ui.results.toggle_family_trace_visibility(key);
                                } else {
                                    toggle_visibility(
                                        state,
                                        model.analysis_index,
                                        trace.waveform_index,
                                    );
                                }
                                ui.close();
                            }
                        }
                        if !any {
                            ui.label("All compatible signals are already shown");
                        }
                    })
                    .response
                    .on_hover_text(format!("Add a signal to the {} pane", pane.unit));
                });
        },
    );

    let mut output = UnitPaneHeaderResponse::default();
    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(actions_rect)
            .layout(egui::Layout::right_to_left(egui::Align::Center)),
        |ui| {
            ui.set_clip_rect(actions_rect);
            let log = ui
                .add_enabled_ui(log_y_available, |ui| chip(ui, "log", log_y))
                .inner
                .on_hover_text(if log_y_available {
                    "Toggle logarithmic Y axis"
                } else {
                    "Logarithmic Y requires strictly positive visible values"
                });
            if log.clicked() {
                output.toggle_log_y = true;
            }
            if IconButton::new(Icon::ZoomFit)
                .side(19.0)
                .tooltip("Autoscale Y to visible traces")
                .show(ui)
                .clicked()
            {
                output.autoscale_y = true;
            }
        },
    );
    output
}

/// One shared X-axis and viewport navigator for every unit pane in an
/// analysis strip. The plots retain X grid/interactions but never repeat the
/// labeled axis, so vertical grids remain aligned and readable.
fn show_shared_x_axis(
    ui: &mut Ui,
    state: &mut AppState,
    model: &StripModel,
    full_domain: (f64, f64),
    pane_count: usize,
    height: f32,
    linked_cursor_domain: Option<&CursorDomain>,
) {
    if height <= 1.0 {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let current =
        shared_x_view(&state.ui.results, model.analysis_key, pane_count).unwrap_or(full_domain);
    let axis = model_x_axis(
        model,
        current.0,
        current.1,
        state.ui.preferences.quantity_presentation_policy(),
    );
    let cursor_owner = state.ui.results.cursor_strip == Some(model.analysis_index);
    let linked_cursor =
        state.ui.results.linked_cursors && linked_cursor_domain == Some(&model.cursor_domain());
    let cursor_values = (cursor_owner || linked_cursor).then_some(state.ui.results.cursors);
    let cursor_summary = cursor_values.map_or_else(
        || "No A/B cursors on this axis".to_owned(),
        |cursors| {
            let a = cursors.a.map_or_else(
                || "not placed".to_owned(),
                |value| axis.format_display_value(value),
            );
            let b = cursors.b.map_or_else(
                || "not placed".to_owned(),
                |value| axis.format_display_value(value),
            );
            format!("Cursor A {a}; cursor B {b}")
        },
    );
    let accessibility_label = format!(
        "Shared {} axis. Current range {} to {}. Full retained range {} to {}. {}. Drag the viewport to pan, click the overview to recenter, use the wheel to zoom at the pointer, drag A or B to move a cursor, and press F to fit.",
        model.x_label(),
        axis.format_display_value(current.0),
        axis.format_display_value(current.1),
        axis.format_display_value(full_domain.0),
        axis.format_display_value(full_domain.1),
        cursor_summary,
    );
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::click_and_drag(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Image, true, accessibility_label.clone())
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::GraphicsDocument);
        node.set_label(accessibility_label.clone());
    });
    let painter = ui.painter().with_clip_rect(rect);
    painter.rect_filled(rect, 0.0, c.bg_panel);
    painter.hline(rect.x_range(), rect.top(), egui::Stroke::new(1.0, c.border));

    let plot_left = (rect.left() + wave_left_margin(&state.ui.results)).min(rect.right());
    let plot_right = (rect.right() - WAVE_SHARED_RIGHT_MARGIN).max(plot_left);
    let label_top = rect.top() + SHARED_X_LABEL_TOP;
    let flag_top = rect.top() + SHARED_X_FLAG_TOP;
    let flag_bottom = flag_top + SHARED_X_FLAG_HEIGHT;
    // The lane spans exactly the panes' plot area, so the overview sits
    // under the traces it mirrors and its viewport window maps 1:1 to what
    // the panes show. Both rows name themselves into the panes' own left
    // gutter, which states the thing two stacked X scales otherwise hide:
    // the ticks are the zoomed viewport, the bar below is the full sweep.
    let track = egui::Rect::from_min_max(
        egui::pos2(plot_left, rect.top() + SHARED_X_LANE_TOP),
        egui::pos2(
            plot_right,
            (rect.top() + SHARED_X_LANE_TOP + SHARED_X_LANE_HEIGHT).min(rect.bottom()),
        ),
    );
    let mut viewport = egui::Rect::NOTHING;
    if track.width() > 1.0 {
        painter.rect_filled(track, 0.0, c.canvas_bg);
        painter.rect_stroke(
            track,
            0.0,
            egui::Stroke::new(1.0, c.border),
            egui::StrokeKind::Inside,
        );
        if let Some(trace) = model
            .traces
            .iter()
            .find(|trace| trace.visible && !trace.overlay)
        {
            let (minimum, maximum) = trace
                .y
                .iter()
                .copied()
                .filter(|value| value.is_finite())
                .fold(
                    (f64::INFINITY, f64::NEG_INFINITY),
                    |(minimum, maximum), value| (minimum.min(value), maximum.max(value)),
                );
            let (minimum, maximum) = if minimum.is_finite() && maximum.is_finite() {
                (minimum, maximum)
            } else {
                (0.0, 1.0)
            };
            let span = (maximum - minimum).max(f64::EPSILON);
            let points = trace
                .x
                .iter()
                .zip(trace.y.iter())
                .step_by((trace.x.len() / 160).max(1))
                .filter_map(|(&x, &y)| {
                    let fraction = model.x_scale.normalize(x, full_domain.0, full_domain.1);
                    (fraction.is_finite() && y.is_finite()).then(|| {
                        egui::pos2(
                            track.left() + track.width() * fraction as f32,
                            track.bottom()
                                - 2.0
                                - ((y - minimum) / span) as f32 * (track.height() - 4.0),
                        )
                    })
                })
                .collect::<Vec<_>>();
            if points.len() >= 2 {
                painter.add(egui::Shape::line(
                    points,
                    egui::Stroke::new(1.0, trace.color.gamma_multiply(0.75)),
                ));
            }
        }

        let (start, end) = shared_axis_viewport_fraction(model.x_scale, full_domain, current);
        let window_left = track.left() + track.width() * start as f32;
        viewport = egui::Rect::from_min_max(
            egui::pos2(window_left, track.top()),
            egui::pos2(
                (track.left() + track.width() * end as f32).max(window_left + 6.0),
                track.bottom(),
            ),
        );
        painter.rect_filled(viewport, 0.0, c.accent.gamma_multiply(0.14));
        painter.rect_stroke(
            viewport,
            0.0,
            egui::Stroke::new(1.0, c.accent),
            egui::StrokeKind::Inside,
        );
        // Grab handles at both edges are the always-visible affordance for
        // resizing the visible span in place: drag an edge in from the full
        // range to zoom, and the opposite edge holds.
        for edge_x in [viewport.left(), viewport.right()] {
            painter.rect_filled(
                egui::Rect::from_min_max(
                    egui::pos2(edge_x - 3.0, track.top() - 1.0),
                    egui::pos2(edge_x + 3.0, track.bottom() + 1.0),
                ),
                2.0,
                c.accent,
            );
            painter.vline(
                edge_x,
                egui::Rangef::new(track.top() + 3.0, track.bottom() - 3.0),
                egui::Stroke::new(1.0, c.canvas_bg),
            );
        }
    }

    // Tick values for the zoomed viewport sit at the top of the strip, right
    // against the panes they describe, and their stubs point up toward the
    // plot.
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let mut last_right = f32::NEG_INFINITY;
    for (value, label) in &axis.ticks {
        let fraction = model.x_scale.normalize(*value, axis.min, axis.max);
        if !fraction.is_finite() || !(0.0..=1.0).contains(&fraction) {
            continue;
        }
        let x = plot_left + (plot_right - plot_left) * fraction as f32;
        painter.vline(
            x,
            egui::Rangef::new(rect.top(), rect.top() + 3.0),
            egui::Stroke::new(1.0, c.border_strong),
        );
        let galley = painter.layout_no_wrap(label.clone(), font.clone(), c.text_dim);
        // The stub marks the true position; the label itself is held inside
        // the plot area so an edge value never reaches into the gutter
        // column and collides with the row's own name.
        let half = galley.size().x * 0.5 + 2.0;
        let centre = x.clamp(plot_left + half, (plot_right - half).max(plot_left + half));
        let left = centre - galley.size().x * 0.5;
        if left >= last_right + 6.0 {
            last_right = left + galley.size().x;
            painter.galley(egui::pos2(left, label_top), galley, c.text_dim);
        }
    }
    let gutter_font = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    painter.text(
        egui::pos2(plot_left - 8.0, label_top + font.size * 0.5),
        egui::Align2::RIGHT_CENTER,
        shared_x_gutter_label(&model.x_unit),
        gutter_font.clone(),
        c.text_faint,
    );
    painter.text(
        egui::pos2(plot_left - 8.0, track.center().y),
        egui::Align2::RIGHT_CENTER,
        "FULL",
        gutter_font,
        c.text_faint,
    );

    let flag_centre = |value: f64| {
        // Flags track the viewport like the pane cursor lines above them,
        // so a cursor zoomed past simply leaves the strip.
        let fraction = model.x_scale.normalize(value, current.0, current.1);
        (fraction.is_finite() && (-0.001..=1.001).contains(&fraction))
            .then(|| plot_left + (plot_right - plot_left) * fraction.clamp(0.0, 1.0) as f32)
    };
    if let Some(cursors) = cursor_values {
        for (label, value, color) in [("A", cursors.a, c.traces[1]), ("B", cursors.b, c.accent)] {
            let Some(value) = value else { continue };
            if let Some(px) = flag_centre(value) {
                let flag = egui::Rect::from_min_max(
                    egui::pos2(px - 8.0, flag_top),
                    egui::pos2(px + 8.0, flag_bottom),
                );
                painter.vline(
                    px,
                    egui::Rangef::new(rect.top(), flag_top),
                    egui::Stroke::new(1.0, color),
                );
                painter.rect_filled(flag, 2.0, color);
                painter.text(
                    flag.center(),
                    egui::Align2::CENTER_CENTER,
                    label,
                    theme::mono(tokens::FS_0, FontWeight::SemiBold),
                    c.canvas_bg,
                );
            }
            // The overview keeps its own notch at the cursor's absolute
            // source position, so it still locates a cursor the viewport has
            // left behind.
            if track.width() > 1.0 {
                let fraction = model.x_scale.normalize(value, full_domain.0, full_domain.1);
                if fraction.is_finite() {
                    let notch = track.left() + track.width() * fraction.clamp(0.0, 1.0) as f32;
                    painter.vline(
                        notch,
                        egui::Rangef::new(track.top() + 1.0, track.top() + 5.0),
                        egui::Stroke::new(2.0, color),
                    );
                }
            }
        }
    }

    if response.clicked() {
        response.request_focus();
    }
    // The lane band owns the overview gestures and everything above it
    // belongs to the flags, so a flag grab never fights a resize handle.
    let lane_band = egui::Rangef::new(track.top() - 4.0, track.bottom() + 4.0);
    let handle_at = |pointer: egui::Pos2| {
        if track.width() <= 1.0 || !lane_band.contains(pointer.y) {
            return None;
        }
        if (pointer.x - viewport.left()).abs() <= 5.0 {
            Some(SharedXDrag::ResizeStart)
        } else if (pointer.x - viewport.right()).abs() <= 5.0 {
            Some(SharedXDrag::ResizeEnd)
        } else {
            None
        }
    };
    let flag_at = |pointer: egui::Pos2| {
        if pointer.y >= track.top() - 4.0 {
            return None;
        }
        let cursors = cursor_values?;
        [
            (SharedXDrag::CursorA, cursors.a),
            (SharedXDrag::CursorB, cursors.b),
        ]
        .into_iter()
        .find_map(|(drag, value)| {
            let px = flag_centre(value?)?;
            ((pointer.x - px).abs() <= 9.0).then_some(drag)
        })
    };
    let drag_id = shared_x_drag_id(model);
    if response.drag_started()
        && let Some(pointer) = response.interact_pointer_pos()
    {
        let drag = handle_at(pointer)
            .or_else(|| flag_at(pointer))
            .unwrap_or(SharedXDrag::Viewport);
        ui.memory_mut(|memory| memory.data.insert_temp(drag_id, drag));
    }
    let drag = ui.memory(|memory| memory.data.get_temp::<SharedXDrag>(drag_id));
    if response.dragged()
        && let Some(pointer) = response.interact_pointer_pos()
        && track.width() > 1.0
        && let Some(drag) = drag
    {
        let fraction = f64::from(((pointer.x - track.left()) / track.width()).clamp(0.0, 1.0));
        match drag {
            SharedXDrag::Viewport => {
                let delta = ui.ctx().input(|input| input.pointer.delta().x);
                if let Some(range) = panned_shared_x_view(
                    model.x_scale,
                    full_domain,
                    current,
                    f64::from(delta / track.width()),
                ) {
                    set_shared_x_view(
                        &mut state.ui.results,
                        model.analysis_key,
                        pane_count,
                        Some(range),
                    );
                }
            }
            SharedXDrag::ResizeStart | SharedXDrag::ResizeEnd => {
                if let Some(range) = resized_shared_x_view(
                    model.x_scale,
                    full_domain,
                    current,
                    drag == SharedXDrag::ResizeStart,
                    fraction,
                ) {
                    set_shared_x_view(
                        &mut state.ui.results,
                        model.analysis_key,
                        pane_count,
                        Some(range),
                    );
                }
            }
            SharedXDrag::CursorA | SharedXDrag::CursorB => {
                // A flag is anchored to what the panes show, so its drag
                // converts through the viewport rather than the full sweep.
                let view_fraction = f64::from(
                    ((pointer.x - plot_left) / (plot_right - plot_left).max(1.0)).clamp(0.0, 1.0),
                );
                let x = model
                    .x_scale
                    .denormalize(view_fraction, current.0, current.1);
                if !cursor_owner && !linked_cursor {
                    state.ui.results.clear_cursors();
                    state.ui.results.cursor_strip = Some(model.analysis_index);
                }
                match drag {
                    SharedXDrag::CursorA => {
                        state.ui.results.cursors.a = Some(x);
                        state.ui.results.cursor_a_anchor = None;
                    }
                    SharedXDrag::CursorB => state.ui.results.cursors.b = Some(x),
                    SharedXDrag::Viewport | SharedXDrag::ResizeStart | SharedXDrag::ResizeEnd => {}
                }
            }
        }
    }
    if response.drag_stopped() {
        ui.memory_mut(|memory| memory.data.remove::<SharedXDrag>(drag_id));
    }

    if response.hovered()
        && let Some(pointer) = response.hover_pos()
    {
        ui.ctx().set_cursor_icon(
            if handle_at(pointer).is_some() || flag_at(pointer).is_some() {
                egui::CursorIcon::ResizeHorizontal
            } else if lane_band.contains(pointer.y) {
                egui::CursorIcon::Grab
            } else {
                egui::CursorIcon::Default
            },
        );
    }

    if response.clicked()
        && let Some(pointer) = response.interact_pointer_pos()
        && track.contains(pointer)
        && !viewport.contains(pointer)
        && let Some(range) = recentered_shared_x_view(
            model.x_scale,
            full_domain,
            current,
            f64::from((pointer.x - track.left()) / track.width()),
        )
    {
        set_shared_x_view(
            &mut state.ui.results,
            model.analysis_key,
            pane_count,
            Some(range),
        );
    }

    if response.hovered() && track.width() > 1.0 {
        let scroll = ui.input(|input| input.smooth_scroll_delta.y);
        if scroll != 0.0
            && let Some(pointer) = response.hover_pos()
            && let Some(range) = zoomed_shared_x_view(
                model.x_scale,
                full_domain,
                current,
                f64::from((pointer.x - track.left()) / track.width()),
                (f64::from(-scroll) * 0.002).exp(),
            )
        {
            ui.input_mut(|input| input.smooth_scroll_delta = egui::Vec2::ZERO);
            set_shared_x_view(
                &mut state.ui.results,
                model.analysis_key,
                pane_count,
                Some(range),
            );
        }
    }

    if response.has_focus() {
        let fit_key =
            ui.input(|input| input.key_pressed(egui::Key::F) || input.key_pressed(egui::Key::Home));
        if fit_key {
            set_shared_x_view(&mut state.ui.results, model.analysis_key, pane_count, None);
        }
        let zoom_in = ui.input(|input| input.key_pressed(egui::Key::Plus));
        let zoom_out = ui.input(|input| input.key_pressed(egui::Key::Minus));
        if (zoom_in || zoom_out)
            && let Some(range) = zoomed_shared_x_view(
                model.x_scale,
                full_domain,
                current,
                0.5,
                if zoom_in { 0.8 } else { 1.25 },
            )
        {
            set_shared_x_view(
                &mut state.ui.results,
                model.analysis_key,
                pane_count,
                Some(range),
            );
        }
        let direction = ui.input(|input| {
            if input.key_pressed(egui::Key::ArrowLeft) {
                -1.0
            } else if input.key_pressed(egui::Key::ArrowRight) {
                1.0
            } else {
                0.0
            }
        });
        if direction != 0.0
            && let Some(range) =
                panned_shared_x_view(model.x_scale, full_domain, current, direction * 0.05)
        {
            set_shared_x_view(
                &mut state.ui.results,
                model.analysis_key,
                pane_count,
                Some(range),
            );
        }
    }

    if response.double_clicked() {
        set_shared_x_view(&mut state.ui.results, model.analysis_key, pane_count, None);
    }
    response.on_hover_text(
        "Shared X overview — drag the window or its edges, click to recenter, wheel to zoom, drag A/B, F to fit",
    );
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
    let geometry = wave_stack_geometry(available.height(), count);

    for (ordinal, pane) in panes.iter().enumerate() {
        if ordinal > 0 {
            let (seam, _) = ui.allocate_exact_size(
                egui::vec2(ui.available_width(), geometry.seam_height),
                egui::Sense::hover(),
            );
            ui.painter().rect_filled(seam, 0.0, t.color.canvas_grid);
        }
        let pane_height = geometry.pane_height(ordinal, count);
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), pane_height),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.set_height(pane_height);
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
                    count,
                );
            },
        );
    }
    show_shared_x_axis(
        ui,
        state,
        model,
        x_domain,
        count,
        geometry.shared_x_height,
        linked_cursor_domain,
    );
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
    pane_count: usize,
) {
    let t = Tokens::get(ui.ctx());
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    let (x0, x1) = x_domain;
    // The pane's own top edge, kept so the active rail can span header and
    // canvas together once the pane's full height is known.
    let pane_top = ui.available_rect_before_wrap().top();

    let pane_range = pane_y_range(&mut state.ui.results.derived, model, &pane.traces);
    let specification_limits = if state.ui.results.show_spec_limits {
        matching_spec_limits(state, model, pane, &t)
    } else {
        Vec::new()
    };
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
        for limit in &specification_limits {
            lo = lo.min(limit.y);
            hi = hi.max(limit.y);
        }
        if !lo.is_finite() || !hi.is_finite() {
            None
        } else if lo == hi && lo > 0.0 {
            Some((lo / 1.1, hi * 1.1))
        } else if lo == hi {
            Some((lo - 1.0, hi + 1.0))
        } else {
            Some((lo, hi))
        }
    };
    let log_y_available = auto_y.is_some_and(|(minimum, maximum)| minimum > 0.0 && maximum > 0.0);
    let mut log_y = pane_log_y(ui, model, pane) && log_y_available;
    if !log_y_available && pane_log_y(ui, model, pane) {
        set_pane_log_y(ui, model, pane, false);
    }
    let header = show_unit_pane_header(
        ui,
        state,
        model,
        pane,
        ordinal,
        log_y,
        log_y_available,
        WAVE_PANE_HEADER_HEIGHT.min(ui.available_height()),
    );
    if header.autoscale_y || header.toggle_log_y {
        let view = state.ui.results.analysis_plot_view_pane_mut(
            super::ResultViewer::Waves,
            model.analysis_key,
            ordinal,
        );
        view.y = None;
    }
    if header.toggle_log_y {
        log_y = !log_y;
        set_pane_log_y(ui, model, pane, log_y);
    }

    let Some((auto_y0, auto_y1)) = auto_y else {
        well_hint(ui, "No visible traces — enable one in the legend");
        return;
    };
    if ui.available_height() < WAVE_MIN_PLOT_HEIGHT {
        return;
    }

    // User zoom/pan overrides the automatic fit per axis, per pane.
    let pane_view = state.ui.results.analysis_plot_view_pane(
        super::ResultViewer::Waves,
        model.analysis_key,
        ordinal,
    );
    let (x0, x1) =
        shared_x_view(&state.ui.results, model.analysis_key, pane_count).unwrap_or((x0, x1));
    let (mut y0, mut y1) = pane_view
        .y
        .filter(|(minimum, maximum)| !log_y || (*minimum > 0.0 && *maximum > 0.0))
        .unwrap_or((auto_y0, auto_y1));

    let x_axis = model_x_axis(model, x0, x1, quantity_policy);
    let family_envelopes = if state.ui.results.show_family_envelope {
        family_envelope_series(model, pane)
    } else {
        Vec::new()
    };
    let y_axis = if log_y {
        Axis::log_decades(y0, y1, pane.unit)
    } else if pane.unit == "°" && pane_view.y.is_none() {
        // An unzoomed degree pane keeps the 45° lattice a Bode phase
        // reading expects; arbitrary zoom depths fall back to plain linear
        // ticks, which stay legible where the lattice would crowd.
        y0 = (y0 / 45.0).floor() * 45.0;
        y1 = (y1 / 45.0).ceil() * 45.0;
        let ticks: Vec<f64> = (0..=((y1 - y0) / 45.0) as i64)
            .map(|i| y0 + i as f64 * 45.0)
            .collect();
        Axis::with_ticks(y0, y1, "°", &ticks)
    } else {
        Axis::linear(y0, y1, pane.unit)
    };
    let y_axis = if pane.unit == "rad" {
        match quantity_policy.angle_display {
            crate::quantity::AngleDisplay::Degrees => {
                y_axis.with_display_transform(180.0 / std::f64::consts::PI, 0.0, "°")
            }
            crate::quantity::AngleDisplay::Radians => y_axis,
        }
    } else if pane.unit == "°" {
        let (scale, offset, unit) = quantity_policy.degree_axis_transform();
        y_axis.with_display_transform(scale, offset, unit)
    } else {
        y_axis
    };
    let mut spec = PlotSpec::new(x_axis, model.x_scale, y_axis)
        .accessible_name("Waveform plot")
        .without_x_axis_chrome()
        .with_right_margin(WAVE_SHARED_RIGHT_MARGIN);
    spec.left_margin = wave_left_margin(&state.ui.results);
    if log_y {
        spec = spec.with_log_y();
    }
    spec.display_decimation = display_decimation(presentation.large_dataset_display());
    spec.limit_lines = specification_limits;
    spec.minor_grid = state.ui.results.show_minor_grid;
    let pane_key = WavePanePresentationKey {
        analysis: model.analysis_key,
        unit: pane.unit.to_owned(),
    };
    spec.horizontal_cursor = state
        .ui
        .results
        .horizontal_cursor
        .as_ref()
        .filter(|cursor| cursor.pane == pane_key)
        .map(|cursor| cursor.y);
    spec.horizontal_cursor_interactive = state.ui.results.horizontal_cursor_placement_enabled();

    // 0 dB reference on a log-magnitude pane.
    if pane.unit == "dB" && y0 < 0.0 && y1 > 0.0 {
        spec.ref_lines.push(plot::RefLine { y: 0.0 });
    }

    // Family envelopes are derived only from exact shared X coordinates.
    // They draw behind source curves and never interpolate missing family
    // samples into evidence that was not retained.
    for envelope in &family_envelopes {
        let mut minimum = Trace::new(&envelope.x, &envelope.minimum, envelope.color)
            .thin()
            .dashed()
            .cache_key(envelope.minimum_cache_key);
        let mut maximum = Trace::new(&envelope.x, &envelope.maximum, envelope.color)
            .thin()
            .dashed()
            .cache_key(envelope.maximum_cache_key);
        if envelope.x.len() == 1 {
            minimum = minimum.show_single_point();
            maximum = maximum.show_single_point();
        }
        spec.traces.push(minimum);
        spec.traces.push(maximum);
    }

    // Run owns weight: overlay traces keep the signal hue at reduced alpha
    // and stroke, painted first so the active run draws at full strength
    // on top.
    let pane_traces: Vec<(usize, &StripTrace)> = pane
        .traces
        .iter()
        .copied()
        .filter_map(|index| model.traces.get(index).map(|trace| (index, trace)))
        .collect();
    let draw_order = pane_traces
        .iter()
        .filter(|(_, trace)| trace.overlay)
        .chain(pane_traces.iter().filter(|(_, trace)| !trace.overlay));
    for (_, trace) in draw_order {
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
    for marker in state.ui.results.strip_markers(model.analysis_key) {
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
        let Some((_, trace)) = anchored else {
            continue;
        };
        let y = sample_at_with(&trace.x, &trace.y, marker.x, interpolation);
        if !y.is_finite() {
            continue;
        }
        spec.markers
            .push(plot::Marker::point(marker.x, y, color, label));
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
    if response.response.hovered()
        || response.response.dragged()
        || response.clicked_x.is_some()
        || response.horizontal_cursor_y.is_some()
    {
        state.ui.results.active_wave_pane = Some(pane_key.clone());
    }
    if let Some(y) = response.horizontal_cursor_y {
        state.ui.results.horizontal_cursor = Some(HorizontalWaveCursor { pane: pane_key, y });
    }

    // The marker tool takes the click when armed: one click cannot both
    // annotate and move a cursor, and the armed chip says which it will do.
    if let Some(clicked_x) = response.clicked_x
        && state.ui.results.marker_tool.is_armed()
    {
        let pointer_y = response.response.interact_pointer_pos().map(|pos| pos.y);
        let plot_rect = response.plot_rect;
        // "Nearest" has to mean what the eye sees, so traces are measured
        // in screen space against the pane's drawn range.
        let screen_y = |value: f64| -> Option<f32> {
            (value.is_finite() && y1 > y0).then(|| {
                plot_rect.bottom() - ((value - y0) / (y1 - y0)) as f32 * plot_rect.height()
            })
        };
        let nearest = pane_traces
            .iter()
            .filter(|(_, trace)| !trace.overlay)
            .filter_map(|(_, trace)| {
                let value = sample_at_with(&trace.x, &trace.y, clicked_x, interpolation);
                let y = screen_y(value)?;
                Some((trace, pointer_y.map_or(0.0, |pointer| (pointer - y).abs())))
            })
            .min_by(|(_, a), (_, b)| a.total_cmp(b));
        if let Some((trace, _)) = nearest {
            let anchor = anchor_key(model, trace);
            let name = trace.name.clone();
            let id = state
                .ui
                .results
                .add_marker(model.analysis_key, anchor, name, clicked_x);
            // Focus the new marker's note field: placing one is normally the
            // first half of saying what it means.
            state.ui.results.editing_marker = Some(id);
        }
    } else if let Some(clicked_x) = response.clicked_x
        && state.ui.results.cursor_placement_enabled()
    {
        let placing_cursor_a = state.ui.results.cursor_a_is_next();
        let pointer_y = response
            .response
            .interact_pointer_pos()
            .map(|position| position.y);
        let nearest_anchor = placing_cursor_a.then(|| {
            pane_traces
                .iter()
                .filter(|(_, trace)| !trace.overlay)
                .filter_map(|(_, trace)| {
                    let value = sample_at_with(&trace.x, &trace.y, clicked_x, interpolation);
                    if !value.is_finite() || y1 <= y0 {
                        return None;
                    }
                    let screen_y = response.plot_rect.bottom()
                        - ((value - y0) / (y1 - y0)) as f32 * response.plot_rect.height();
                    Some((
                        anchor_key(model, trace),
                        pointer_y.map_or(0.0, |pointer| (pointer - screen_y).abs()),
                    ))
                })
                .min_by(|(_, left), (_, right)| left.total_cmp(right))
                .map(|(anchor, _)| anchor)
        });
        let results = &mut state.ui.results;
        if results.cursor_strip != Some(model.analysis_index)
            && (!results.linked_cursors || !cursor_domain_matches)
        {
            results.cursors = CursorPair::default();
        }
        results.cursor_strip = Some(model.analysis_index);
        if placing_cursor_a {
            results.cursor_a_anchor = nearest_anchor.flatten();
        }
        results.cursors.place(clicked_x);
    }

    if response.view.reset {
        set_shared_x_view(&mut state.ui.results, model.analysis_key, pane_count, None);
        let view = state.ui.results.analysis_plot_view_pane_mut(
            super::ResultViewer::Waves,
            model.analysis_key,
            ordinal,
        );
        view.y = None;
    } else if response.view.any() {
        if let Some(x) = response.view.x {
            set_shared_x_view(
                &mut state.ui.results,
                model.analysis_key,
                pane_count,
                Some(x),
            );
        }
        let view = state.ui.results.analysis_plot_view_pane_mut(
            super::ResultViewer::Waves,
            model.analysis_key,
            ordinal,
        );
        if let Some(y) = response.view.y {
            view.y = Some(y);
        }
    }

    // The mockup's `.plot-pane.active::before`: a 2 px rail down the pane
    // that received the instrument's actions. Painted last so it reads over
    // the header fill and the canvas alike.
    let pane_active = state.ui.results.active_wave_pane.as_ref()
        == Some(&WavePanePresentationKey {
            analysis: model.analysis_key,
            unit: pane.unit.to_owned(),
        });
    if pane_active {
        let bottom = ui.min_rect().bottom().max(pane_top);
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                egui::pos2(ui.min_rect().left(), pane_top),
                egui::pos2(ui.min_rect().left() + 2.0, bottom),
            ),
            0.0,
            t.color.accent,
        );
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
