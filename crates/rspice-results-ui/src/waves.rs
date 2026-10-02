//! Waveform projection snapshots shared by plots, tables and readouts.
//!
//! The host selects retained runs, owns source invalidation and resolves evidence
//! status. These models preserve exact source identities, units and sample rows.

use rspice_results::analysis_type::AnalysisType;
use rspice_results::family_projection::FamilyTraceStyle;
use rspice_results::result_presentation::{
    AnalysisPresentationKey, TracePresentationKey, WaveformPresentationKey,
};
use rspice_results::waveform::SharedWaveformValues;
use rspice_ui_kit::plot::sample::SweepShape;
use rspice_ui_kit::plot::{XScale, fmt_si_significant, fmt_significant};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod navigation;
pub mod pane;
mod policy;
mod projection;
pub mod readout;
pub use policy::{
    CursorInterpolation, DisplayedSignificantDigits, ReadoutPolicy, cursor_interpolation,
};
mod units;
pub use projection::{ProjectionOptions, build_models, family_color};
use units::signal_unit;
pub use units::{
    NOISE_DENSITY_UNIT, analysis_default_unit, browser_signal_is_current, browser_signal_unit,
    fmt_in_unit,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComplexNumberDisplay {
    #[default]
    MagnitudePhaseDegrees,
    RealImaginary,
    MagnitudePhaseRadians,
}

impl ComplexNumberDisplay {
    pub const fn index(self) -> usize {
        match self {
            Self::MagnitudePhaseDegrees => 0,
            Self::RealImaginary => 1,
            Self::MagnitudePhaseRadians => 2,
        }
    }

    pub fn from_index(index: usize) -> Result<Self, &'static str> {
        match index {
            0 => Ok(Self::MagnitudePhaseDegrees),
            1 => Ok(Self::RealImaginary),
            2 => Ok(Self::MagnitudePhaseRadians),
            _ => Err("complex-number display index is outside its domain"),
        }
    }
}

/// How a trace's Y values are interpreted.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TraceKind {
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
pub struct FamilyTraceVisibilityKey {
    analysis: AnalysisPresentationKey,
    group_key: u64,
    source_name_hash: u64,
    trace_kind: u8,
}

impl FamilyTraceVisibilityKey {
    pub fn new(
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

    /// The retained analysis this visibility override is a decision about.
    pub const fn analysis(self) -> AnalysisPresentationKey {
        self.analysis
    }
}

impl TraceKind {
    pub const fn is_phase(self) -> bool {
        matches!(self, Self::PhaseDeg | Self::PhaseRad)
    }
}

/// One trace of a strip, with owned `Arc` handles into the run data.
pub struct StripTrace {
    pub waveform_index: usize,
    /// Name of the source waveform in the immutable dataset. Display names
    /// may be derived representations such as `re(V(out))`.
    pub source_waveform_name: String,
    /// Undecorated signal name used to preserve overlay pairing when the
    /// active source is expanded into several family groups.
    pub base_name: String,
    pub name: String,
    /// The unit the source waveform states for its retained samples, when it
    /// states one. Overlay traces copy their active counterpart's so a run
    /// overlay always lands on the pane it is being compared against.
    pub unit: Option<String>,
    /// Ordinary signal color retained for non-family overlay runs.
    pub signal_color: egui::Color32,
    pub color: egui::Color32,
    pub x: SharedWaveformValues,
    pub y: SharedWaveformValues,
    /// What this trace's abscissa actually is: one ascending sweep, a reverse
    /// sweep, or a loop that turns around partway.
    ///
    /// Carried on the trace rather than derived where it is needed, because
    /// every consumer needs it on every frame and one of them — the hover
    /// readout — is a `Fn` closure the plot holds while it already owns the
    /// decimation cache mutably, so it has no route back to the memo.
    pub shape: Arc<SweepShape>,
    pub kind: TraceKind,
    pub visible: bool,
    /// The run this trace belongs to (cache-key discriminator).
    pub run_id: u64,
    /// Overlay traces come from a non-active run: same signal hue, reduced
    /// weight, visibility slaved to the active run's matching signal.
    pub overlay: bool,
    /// Stable family-group identity mixed into all derived/cache keys.
    pub presentation_key: u64,
    pub family_group_ordinal: Option<usize>,
    pub family_style: Option<FamilyTraceStyle>,
    pub family_visibility_key: Option<FamilyTraceVisibilityKey>,
}

/// One strip (== one analysis of the active run).
pub struct StripModel {
    pub analysis_index: usize,
    pub analysis_key: AnalysisPresentationKey,
    pub analysis_type: AnalysisType,
    pub kind_tag: String,
    pub subtitle: String,
    /// Why this strip's analysis is less than a complete run, if it is.
    /// Carried on the model rather than re-derived at paint time so the
    /// strip and the sheet bar cannot disagree about the same run.
    pub incomplete: Option<&'static str>,
    pub x_scale: XScale,
    pub x_dimension_key: String,
    pub x_label: String,
    pub x_unit: String,
    pub y_unit: &'static str,
    /// Phase traces carry the unwrapped (continuous) series instead of the
    /// raw ±180°-wrapped samples. Folded into the cache keys.
    pub phase_continuous: bool,
    /// Number of active-run traces at the front of `traces`; everything
    /// after is overlay. The legend lists only this prefix (signal owns
    /// hue — one chip per signal, all runs).
    pub signal_trace_count: usize,
    pub traces: Vec<StripTrace>,
    /// Whether the sample grid is finite and non-decreasing, which is what
    /// lets a cursor be mapped to a retained sample by bisection.
    pub grid_ascending: bool,
    /// The strip's shared X extent over its visible traces.
    ///
    /// Resolved while the model is built rather than on demand: the axis,
    /// the overview lane, the viewport gestures and the fit control all ask
    /// for it, and each answer walked every visible sample.
    ///
    /// Left `None` by [`build_models`] and filled in by
    /// the host's X-range resolution once the reader's visibility overrides have
    /// been applied — which traces are visible is not known until then.
    pub x_range: Option<(f64, f64)>,
}

impl StripModel {
    #[cfg(any(test, feature = "test-support"))]
    pub const fn analysis_type(&self) -> AnalysisType {
        self.analysis_type
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorDomain {
    pub analysis_type: AnalysisType,
    pub x_scale: XScale,
    pub x_dimension_key: String,
    pub x_label: String,
    pub x_unit: String,
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

pub fn stable_hash(value: &impl std::hash::Hash) -> u64 {
    use std::hash::Hasher;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

impl StripModel {
    pub fn cursor_domain(&self) -> CursorDomain {
        CursorDomain {
            analysis_type: self.analysis_type,
            x_scale: self.x_scale,
            x_dimension_key: self.x_dimension_key.clone(),
            x_label: self.x_label.clone(),
            x_unit: self.x_unit.clone(),
        }
    }

    pub fn x_label(&self) -> &str {
        &self.x_label
    }

    pub fn format_x(
        &self,
        x: f64,
        significant_digits: usize,
        quantity_policy: rspice_app_types::quantity::QuantityPresentationPolicy,
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

    pub fn format_trace_value(
        &self,
        trace: &StripTrace,
        value: f64,
        significant_digits: usize,
        quantity_policy: rspice_app_types::quantity::QuantityPresentationPolicy,
    ) -> String {
        match trace.kind {
            // A strip mixing voltages and currents has no single Y unit, so
            // a value is named by the unit its own signal is measured in —
            // the same unit that decided which pane draws it.
            TraceKind::Value | TraceKind::Real | TraceKind::Imaginary => {
                fmt_in_unit(value, self.trace_unit(trace), significant_digits)
            }
            TraceKind::MagnitudeDb => {
                let suffix = if self.trace_unit(trace) == "dBc" {
                    " dBc"
                } else {
                    " dB"
                };
                fmt_significant(value, significant_digits, suffix)
            }
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
    pub fn analysis_index(&self) -> usize {
        self.analysis_index
    }

    pub fn analysis_key(&self) -> AnalysisPresentationKey {
        self.analysis_key
    }

    pub fn trace_presentation_key(&self, index: usize) -> Option<TracePresentationKey> {
        self.traces.get(index).map(trace_presentation_key)
    }

    pub fn trace_index_for_key(&self, key: &TracePresentationKey) -> Option<usize> {
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
    pub fn trace_unit<'a>(&'a self, trace: &'a StripTrace) -> &'a str {
        signal_unit(
            &trace.base_name,
            trace.kind,
            trace.unit.as_deref(),
            self.y_unit,
        )
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
    pub fn unit_panes(&self) -> Vec<UnitPane<'_>> {
        let mut panes: Vec<UnitPane<'_>> = Vec::new();
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
    pub fn visible_signal_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.traces
            .iter()
            .take(self.signal_trace_count)
            .enumerate()
            .filter(|(_, trace)| trace.visible)
            .map(|(index, _)| index)
    }

    /// The analysis' retained sample grid — the X array every trace on this
    /// strip was solved against.
    pub fn sample_grid(&self) -> Option<&[f64]> {
        self.traces.first().map(|trace| trace.x.as_slice())
    }

    /// Whether [`Self::sample_grid`] can be bisected; see
    /// [`Self::grid_is_ascending`].
    pub const fn grid_is_ascending(&self) -> bool {
        self.grid_ascending
    }

    /// Column heading for the X axis ("t · s").
    pub fn x_axis_heading(&self) -> String {
        if self.x_unit.is_empty() {
            self.x_label.clone()
        } else {
            format!("{} · {}", self.x_label, self.x_unit)
        }
    }

    /// The X value at one grid index, formatted.
    pub fn format_x_at(
        &self,
        index: usize,
        significant_digits: usize,
        quantity_policy: rspice_app_types::quantity::QuantityPresentationPolicy,
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
    pub fn format_sample(
        &self,
        trace_index: usize,
        sample: usize,
        significant_digits: usize,
        quantity_policy: rspice_app_types::quantity::QuantityPresentationPolicy,
    ) -> Option<String> {
        let trace = self.traces.get(trace_index)?;
        let value = trace.y.as_slice().get(sample).copied()?;
        Some(self.format_trace_value(trace, value, significant_digits, quantity_policy))
    }

    /// Display name of one trace, for a column heading.
    pub fn trace_heading(&self, index: usize) -> Option<(&str, egui::Color32)> {
        self.traces
            .get(index)
            .map(|trace| (trace.name.as_str(), trace.color))
    }

    /// Analysis label for the table's analysis picker.
    pub fn table_label(&self) -> String {
        format!("{} · {}", self.kind_tag, self.subtitle)
    }
}

/// Stable per-trace identity shared by the decimation, range, and
/// measurement caches. Phase traces fold in the wrapped/continuous choice
/// so a toggle never serves stale envelopes, ranges, or stats.
pub fn trace_key(model: &StripModel, trace: &StripTrace) -> u64 {
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
pub fn trace_presentation_key(trace: &StripTrace) -> TracePresentationKey {
    TracePresentationKey {
        source_name: trace.source_waveform_name.clone(),
        kind: trace.kind as u8,
        family_group: trace.presentation_key,
    }
}

pub fn anchor_key(model: &StripModel, trace: &StripTrace) -> WaveformPresentationKey {
    WaveformPresentationKey {
        analysis: model.analysis_key,
        trace: trace_presentation_key(trace),
    }
}

/// One Y axis of a strip: the traces measured in a single unit.
///
/// Panes of a strip always share the strip's X domain — they are one
/// measurement read against several scales, not several plots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitPane<'a> {
    /// The unit every trace on this pane's axis is measured in. Borrowed from
    /// the strip model, because a retained unit is data rather than one of a
    /// closed set of names the viewer knows in advance.
    pub unit: &'a str,
    /// Trace indices on this pane's axis.
    pub traces: Vec<usize>,
}

/// Whether a strip's sample grid is finite and non-decreasing.
///
/// Resolved with the model because it is the precondition for answering
/// "which retained sample is this cursor on" by bisection instead of by
/// reading every coordinate. A grid that fails it keeps the linear scan;
/// a parametric sweep is under no obligation to be monotonic.
pub(crate) fn grid_is_ascending(model: &StripModel) -> bool {
    model.sample_grid().is_some_and(|grid| {
        let mut previous = f64::NEG_INFINITY;
        grid.iter().all(|value| {
            let ordered = value.is_finite() && *value >= previous;
            previous = *value;
            ordered
        })
    })
}
