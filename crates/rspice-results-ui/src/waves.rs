//! Waveform projection snapshots shared by plots, tables and readouts.
//!
//! The host selects retained runs, owns source invalidation and resolves evidence
//! status. These models preserve exact source identities, units and sample rows.

use rspice_results::analysis_type::AnalysisType;
use rspice_results::family_projection::FamilyTraceStyle;
use rspice_results::result_presentation::{
    AnalysisPresentationKey, TracePresentationKey, WaveformPresentationKey,
};
use rspice_results::visualization_document::{MAX_STRIP_AXES, SignalForm, Strip, StripMember};
use rspice_results::waveform::SharedWaveformValues;
use rspice_ui_kit::plot::sample::SweepShape;
use rspice_ui_kit::plot::{XScale, fmt_si_significant};
use std::sync::Arc;

mod axes;
pub mod cache;
pub mod cursor_values;
pub mod dock;
pub mod expression_editor;
pub mod extent;
pub mod header;
pub mod lanes;
pub mod logic;
pub mod marker_dialog;
pub mod navigation;
pub mod pane;
mod plot_menu;
mod policy;
mod projection;
pub mod readout;
pub mod stack;
mod strip_drag;
pub mod strip_layout;
pub mod xy;
pub use policy::{ReadoutPolicy, cursor_interpolation};
mod units;
pub use projection::{
    AcDisplay, ProjectionOptions, build_models, family_color, matching_overlay_analysis,
};
pub use rspice_ui_kit::plot::format::unit_carries_its_decade;
use units::signal_unit;
pub use units::{
    NOISE_DENSITY_UNIT, analysis_default_unit, browser_signal_is_current, browser_signal_unit,
};

/// How the host shows a result's name: the design's spelling of the name the
/// result keys it by (`I(Cout)` for the engine's `I(COUT)`). Only what a
/// surface paints passes through it; every key, request and copy keeps the
/// result's own name.
pub type DisplayName<'a> = &'a dyn for<'n> Fn(&'n str) -> std::borrow::Cow<'n, str>;

/// The name as the result spells it, for a host with no design behind it.
pub fn as_returned(name: &str) -> std::borrow::Cow<'_, str> {
    std::borrow::Cow::Borrowed(name)
}

/// How a trace's Y values are interpreted.
///
/// The discriminants are saved: a marker's trace key stores `kind as u8`, so
/// each kind keeps its number and a retired one's number is never reused.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TraceKind {
    /// Plain values (V, A, sweep output).
    Value = 0,
    /// dB-converted AC magnitude.
    MagnitudeDb = 1,
    /// Phase in degrees (dashed, own stacked pane).
    PhaseDeg = 2,
    /// Original real component of a complex source quantity.
    Real = 4,
    /// Original imaginary component of a complex source quantity.
    Imaginary = 5,
    /// nV/√Hz projection of a retained V²/Hz noise PSD.
    NoiseDensity = 6,
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
        matches!(self, Self::PhaseDeg)
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
    /// The trace's place in the canvas palette's cycle, which gives a
    /// repeated hue its own dash; `None` when a family policy colours it.
    pub palette_slot: Option<usize>,
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

impl StripTrace {
    /// The trace's samples as two slices: as they are when held in memory; a stored column once
    /// the read service has read it off the frame. `None` while a read is on its way, and for a
    /// stored column larger than the loaded results may hold, which is drawn from its tiles and
    /// read by window instead.
    pub fn held(
        &self,
    ) -> Option<(
        rspice_results::waveform::HeldColumn,
        rspice_results::waveform::HeldColumn,
    )> {
        held_pair(&self.x, &self.y)
    }
}

/// `x` and `y` as two slices, as [`StripTrace::held`] holds a trace's.
pub(crate) fn held_pair(
    x: &SharedWaveformValues,
    y: &SharedWaveformValues,
) -> Option<(
    rspice_results::waveform::HeldColumn,
    rspice_results::waveform::HeldColumn,
)> {
    Some((x.fetch_whole().ready()?, y.fetch_whole().ready()?))
}

/// What an event lane of a strip draws.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaneTraceKind {
    /// A scalar digital net, `D()`: a logic lane above the panes.
    Logic,
    /// A real net, `E()`: a step trace in the pane of its unit.
    Real,
}

/// A digital or real net of a strip's analysis with an exact event history,
/// named as its `D()` or `E()` column is, whether or not the analysis has
/// that column. It is drawn from the history, as a logic lane or a step
/// trace, and never as a column resampled onto the analog grid, so it is
/// kept out of the strip's value traces.
#[derive(Debug, Clone)]
pub struct LaneTrace {
    /// The analysis column the lane answers to; none for a net the run keeps
    /// only as event history.
    pub waveform_index: Option<usize>,
    /// The column's name, `D(net)` or `E(net)`: the lane's identity.
    pub source_waveform_name: String,
    /// The net as the event history names it.
    pub net: String,
    pub kind: LaneTraceKind,
    /// The unit the column states, for a real lane's pane.
    pub unit: Option<String>,
    pub color: egui::Color32,
    /// The column's sample grid, so the strip's extent covers the lane.
    pub x: SharedWaveformValues,
    pub visible: bool,
}

/// The search markers found on a strip: each placement's trace, X bits and kind, with where
/// the search found it, or `Err` with where it was put.
pub type MarkerPositions = std::sync::Mutex<
    Vec<(
        WaveformPresentationKey,
        u64,
        rspice_results::result_presentation::MarkerKind,
        Result<f64, f64>,
    )>,
>;

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
    /// The least and greatest finite X the visible traces hold, read once as
    /// the model is built. Whether the X axis can be logarithmic is decided
    /// from it on every frame.
    pub x_bounds: Option<(f64, f64)>,
    /// The span of time a running transient fills, from where its result starts to its stop
    /// time, while it runs. An unzoomed strip of it frames this span, so the trace grows across
    /// an axis that stands still. Set by the host, which knows the run is running.
    pub live_frame: Option<(f64, f64)>,
    /// Where each peak, valley and crossing marker on this strip stands: a search walks its
    /// trace, and the canvas and the readout ask on every frame. Kept as long as the model,
    /// which is built again when the data under it changes.
    pub marker_positions: MarkerPositions,
    /// The analysis' committed event history, as lanes, when it has one.
    pub event_lanes: Option<Arc<logic::EventLanes>>,
    /// The analysis' `D()` and `E()` columns drawn from that history.
    pub lane_traces: Vec<LaneTrace>,
    /// The hues the logic and bus rows take in turn, in their natural order:
    /// the canvas palette from the first place after the curves and real
    /// lanes the strip shows. Empty until the reader's visibility is applied,
    /// and a row then takes its lane's own colour.
    pub lane_hues: Vec<egui::Color32>,
    /// What the run's checks put on the strip's time axis: its HDL severity
    /// records and finish, its assertion violations, and its stop time when
    /// expected outputs lie beyond it. `None` when they put nothing there.
    pub event_markers: Option<Arc<logic::RunMarkers>>,
    /// Where the reader's expressions are drawn, by the unit unit algebra
    /// states for each, hidden ones with where they would be. Left empty by [`build_models`] and filled in
    /// by the host, which owns the expressions.
    pub expression_panes: Vec<ExpressionPane>,
    /// The strips the reader arranged the analysis into, top to bottom;
    /// empty, each unit has a strip of its own. Filled in by the host from
    /// the view it draws.
    pub arrangement: Vec<Strip>,
}

/// Where one expression trace is drawn.
///
/// An expression joins the pane of its unit, or opens one. One whose unit
/// cannot be stated gets an unlabeled pane of its own: it never borrows
/// another unit's axis. A difference from another run is drawn apart, on a
/// pane of its own directly below the pane of its unit.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExpressionPane {
    /// The expression's place in the strip's expression list.
    pub slot: usize,
    /// The unit as a pane names it, empty for a plain number; for an
    /// expression with no unit, the pane's identity, which nothing displays.
    pub unit: String,
    /// Why no unit can be stated, when none can.
    pub unknown: Option<String>,
    /// Whether the reader shows it. A hidden expression is drawn nowhere and
    /// waits in the legend of the pane of its unit.
    pub visible: bool,
    /// The identity of the pane of its own it is drawn on, directly below the
    /// pane of its unit, whose axis reads in `unit`: a difference from another
    /// run, whose Y axis fits the difference rather than the signals it is
    /// taken from. `None` for one that joins the pane of its unit.
    pub apart: Option<String>,
    /// The expression as typed, which a strip lists it by.
    pub text: String,
}

impl ExpressionPane {
    /// The pane of an expression whose unit cannot be stated, keyed by its
    /// text so it is the same pane from frame to frame.
    pub fn unlabeled(slot: usize, text: &str, reason: String) -> Self {
        Self {
            slot,
            unit: format!("{text} \u{2014} no unit"),
            unknown: Some(reason),
            visible: true,
            apart: None,
            text: text.to_owned(),
        }
    }
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

    /// An X value as a cursor column, a marker, a legend and a table cell
    /// read it.
    pub fn format_x(
        &self,
        x: f64,
        significant_digits: usize,
        quantity_policy: rspice_vocabulary::quantity::QuantityPresentationPolicy,
    ) -> String {
        if self.x_unit == "Hz" {
            // The digit preference counts significant digits, in whichever
            // unit Frequency display chose.
            let (scale, offset, unit) = quantity_policy.frequency_axis_transform();
            return fmt_si_significant(x * scale + offset, unit, significant_digits);
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

    /// One trace's value as a reader meets it: the legend chip beside its
    /// name, a cursor column, a measurement row, a marker, a table cell.
    pub fn format_trace_value(
        &self,
        trace: &StripTrace,
        value: f64,
        significant_digits: usize,
        quantity_policy: rspice_vocabulary::quantity::QuantityPresentationPolicy,
    ) -> String {
        match trace.kind {
            // A strip mixing voltages and currents has no single Y unit, so
            // a value is named by the unit its own signal is measured in —
            // the same unit that decided which pane draws it.
            TraceKind::Value | TraceKind::Real | TraceKind::Imaginary => {
                fmt_si_significant(value, self.trace_unit(trace), significant_digits)
            }
            TraceKind::MagnitudeDb => {
                let unit = if self.trace_unit(trace) == "dBc" {
                    "dBc"
                } else {
                    "dB"
                };
                fmt_si_significant(value, unit, significant_digits)
            }
            TraceKind::PhaseDeg => {
                phase_sample(value.to_radians(), significant_digits, quantity_policy)
            }
            TraceKind::NoiseDensity => {
                fmt_si_significant(value, NOISE_DENSITY_UNIT, significant_digits)
            }
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

    /// The unit a real lane reads in: its column's, as a value trace's is.
    pub fn lane_unit<'a>(&'a self, lane: &'a LaneTrace) -> &'a str {
        signal_unit(
            &lane.source_waveform_name,
            TraceKind::Value,
            lane.unit.as_deref(),
            self.y_unit,
        )
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
    /// visibility changes. A pane draws something or is not there: one whose
    /// signals and expressions are all hidden takes no row, as a waveform
    /// viewer shows no strip for a signal until it is plotted. Its signals
    /// wait in the data browser, whose tick brings the pane back, and its
    /// expressions in the strip header's legend.
    ///
    /// Phase owns a stacked pane of its own — the mockup's Bode instrument
    /// reads magnitude over phase as two weighted panes sharing one X
    /// domain, not a second axis on the magnitude pane. Phase panes order
    /// after the quantity panes so magnitude keeps the primary slot.
    pub fn unit_panes(&self) -> Vec<UnitPane<'_>> {
        let (panes, drawn) = self.default_panes();
        let first_unit = self
            .traces
            .first()
            .map_or("", |trace| self.trace_unit(trace));
        if !self.arrangement.is_empty() {
            return self.arranged_panes(panes, &drawn, first_unit);
        }
        let (quantity, phase): (Vec<_>, Vec<_>) = panes
            .into_iter()
            .zip(drawn)
            .filter_map(|(pane, drawn)| drawn.then_some(pane))
            .partition(|pane| !is_phase_unit(pane.axis));
        let mut panes: Vec<_> = quantity.into_iter().chain(phase).collect();
        for pane in &mut panes {
            let identity = pane.unit;
            pane.assign_slots(identity, first_unit);
        }
        panes
    }

    /// The strips as the reader arranged them. Each strip's axes draw the
    /// curves they list, a curve of another unit going to the strip's axis
    /// of its unit, or a new one while the strip has fewer than four. A
    /// curve no strip lists, or one no listing strip has room for, joins the
    /// first strip with an axis of its unit, or has a strip of its own after
    /// the arranged ones, as an unarranged unit does. A strip, or an axis,
    /// that draws nothing takes no room.
    fn arranged_panes<'s>(
        &'s self,
        defaults: Vec<UnitPane<'s>>,
        drawn: &[bool],
        first_unit: &str,
    ) -> Vec<UnitPane<'s>> {
        struct Row<'s> {
            identity: &'s str,
            slot_identity: &'s str,
            weight: f32,
            locked: bool,
            arranged: bool,
            unlabeled: bool,
            axes: Vec<(SecondaryAxis<'s>, bool)>,
            hidden_expressions: Vec<usize>,
        }
        fn axis<'s>(unit: &'s str) -> (SecondaryAxis<'s>, bool) {
            (
                SecondaryAxis {
                    unit,
                    traces: Vec::new(),
                    expressions: Vec::new(),
                    slot: SHARED_X_SLOT,
                },
                false,
            )
        }
        enum Curve {
            Trace(usize),
            Expression(usize),
        }
        struct Item<'s> {
            curve: Curve,
            unit: &'s str,
            identity: &'s str,
            unlabeled: bool,
        }
        let mut items: Vec<Item<'s>> = Vec::new();
        let mut lane_units: Vec<&'s str> = Vec::new();
        let mut hidden: Vec<(&'s str, Vec<usize>)> = Vec::new();
        for (pane, drawn) in defaults.iter().zip(drawn) {
            let unit = if pane.unlabeled { pane.unit } else { pane.axis };
            if !pane.hidden_expressions.is_empty() {
                hidden.push((unit, pane.hidden_expressions.clone()));
            }
            if *drawn && pane.traces.is_empty() && pane.expressions.is_empty() {
                lane_units.push(unit);
            }
            let item = |curve| Item {
                curve,
                unit,
                identity: pane.unit,
                unlabeled: pane.unlabeled,
            };
            items.extend(pane.traces.iter().map(|&index| item(Curve::Trace(index))));
            items.extend(
                pane.expressions
                    .iter()
                    .map(|&slot| item(Curve::Expression(slot))),
            );
        }
        let mut rows: Vec<Row<'s>> = self
            .arrangement
            .iter()
            .enumerate()
            .map(|(position, strip)| {
                let first = strip.axes.first().map_or("", |axis| axis.unit.as_str());
                let earlier = self.arrangement[..position]
                    .iter()
                    .any(|strip| strip.axes.first().is_some_and(|axis| axis.unit == first));
                Row {
                    identity: strip.key.as_str(),
                    slot_identity: if earlier { strip.key.as_str() } else { first },
                    weight: strip.weight.unwrap_or(1.0),
                    locked: strip.locked,
                    arranged: true,
                    unlabeled: false,
                    axes: strip.axes.iter().map(|held| axis(&held.unit)).collect(),
                    hidden_expressions: Vec::new(),
                }
            })
            .collect();
        let listed = |curve: &Curve, member: &StripMember| match (curve, member) {
            (Curve::Trace(index), StripMember::Signal { name, form }) => {
                self.traces.get(*index).is_some_and(|trace| {
                    trace.source_waveform_name == *name && signal_form(trace.kind) == *form
                })
            }
            (Curve::Expression(slot), StripMember::Expression { text }) => self
                .expression_panes
                .iter()
                .any(|expression| expression.slot == *slot && expression.text == *text),
            _ => false,
        };
        let push = |axis: &mut SecondaryAxis<'s>, curve: &Curve| match curve {
            Curve::Trace(index) => axis.traces.push(*index),
            Curve::Expression(slot) => axis.expressions.push(*slot),
        };
        let mut unplaced: Vec<&Item<'s>> = Vec::new();
        for item in &items {
            let mut placed = false;
            for (strip, row) in self.arrangement.iter().zip(rows.iter_mut()) {
                let Some(listing) = strip.axes.iter().position(|held| {
                    held.members
                        .iter()
                        .any(|member| listed(&item.curve, member))
                }) else {
                    continue;
                };
                let target = if row.axes[listing].0.unit == item.unit {
                    Some(listing)
                } else if let Some(other) =
                    row.axes.iter().position(|(held, _)| held.unit == item.unit)
                {
                    Some(other)
                } else if row.axes.len() < MAX_STRIP_AXES {
                    row.axes.push(axis(item.unit));
                    Some(row.axes.len() - 1)
                } else {
                    None
                };
                if let Some(target) = target {
                    push(&mut row.axes[target].0, &item.curve);
                    placed = true;
                }
            }
            if !placed {
                unplaced.push(item);
            }
        }
        for item in unplaced {
            if let Some(row) = rows
                .iter_mut()
                .find(|row| row.axes.iter().any(|(held, _)| held.unit == item.unit))
            {
                let held = row
                    .axes
                    .iter_mut()
                    .find(|(held, _)| held.unit == item.unit)
                    .expect("the row has an axis of the unit");
                push(&mut held.0, &item.curve);
                continue;
            }
            let mut held = axis(item.unit);
            push(&mut held.0, &item.curve);
            rows.push(Row {
                identity: item.identity,
                slot_identity: item.identity,
                weight: 1.0,
                locked: false,
                arranged: false,
                unlabeled: item.unlabeled,
                axes: vec![held],
                hidden_expressions: Vec::new(),
            });
        }
        for unit in lane_units {
            match rows
                .iter_mut()
                .find_map(|row| row.axes.iter_mut().find(|(held, _)| held.unit == unit))
            {
                Some(held) => held.1 = true,
                None => rows.push(Row {
                    identity: unit,
                    slot_identity: unit,
                    weight: 1.0,
                    locked: false,
                    arranged: false,
                    unlabeled: false,
                    axes: vec![(axis(unit).0, true)],
                    hidden_expressions: Vec::new(),
                }),
            }
        }
        for (unit, slots) in hidden {
            if let Some(row) = rows
                .iter_mut()
                .find(|row| row.axes.first().is_some_and(|(held, _)| held.unit == unit))
            {
                row.hidden_expressions.extend(slots);
            }
        }
        rows.into_iter()
            .filter_map(|row| {
                let mut axes = row.axes.into_iter().filter_map(|(held, lanes)| {
                    (lanes || !held.traces.is_empty() || !held.expressions.is_empty())
                        .then_some(held)
                });
                let first = axes.next()?;
                let mut pane = UnitPane {
                    traces: first.traces,
                    expressions: first.expressions,
                    hidden_expressions: row.hidden_expressions,
                    unlabeled: row.unlabeled,
                    secondary: axes.collect(),
                    weight: row.weight,
                    locked: row.locked,
                    arranged: row.arranged,
                    ..UnitPane::of(row.identity, first.unit)
                };
                pane.assign_slots(row.slot_identity, first_unit);
                Some(pane)
            })
            .collect()
    }

    /// Every unit's pane, in the order the units first appear, and whether
    /// each draws anything.
    fn default_panes(&self) -> (Vec<UnitPane<'_>>, Vec<bool>) {
        let mut panes: Vec<UnitPane<'_>> = Vec::new();
        // Whether each pane draws anything: one that draws nothing is
        // dropped at the end, once its hidden expressions have found it.
        let mut drawn: Vec<bool> = Vec::new();
        for (index, trace) in self.traces.iter().enumerate() {
            let unit = self.trace_unit(trace);
            match panes.iter().position(|pane| pane.unit == unit) {
                Some(ordinal) => {
                    if trace.visible {
                        panes[ordinal].traces.push(index);
                    }
                    drawn[ordinal] |= trace.visible;
                }
                None => {
                    panes.push(UnitPane {
                        traces: trace.visible.then_some(index).into_iter().collect(),
                        ..UnitPane::of(unit, unit)
                    });
                    drawn.push(trace.visible);
                }
            }
        }
        // A real lane is drawn in the pane of its unit, which it opens when
        // no value trace does.
        for lane in self
            .lane_traces
            .iter()
            .filter(|lane| lane.visible && lane.kind == LaneTraceKind::Real)
        {
            let unit = self.lane_unit(lane);
            match panes.iter().position(|pane| pane.unit == unit) {
                Some(ordinal) => drawn[ordinal] = true,
                None => {
                    panes.push(UnitPane::of(unit, unit));
                    drawn.push(true);
                }
            }
        }
        // An expression joins the pane whose unit is its own however the two
        // are spelled (`W` and `V·A`), else opens the pane of its unit; one
        // with no unit opens an unlabeled pane of its own. A hidden one opens
        // no pane, and waits in the legend of the pane of its unit when there
        // is one.
        let owner = |panes: &[UnitPane<'_>], expression: &ExpressionPane| {
            if expression.unknown.is_some() {
                return None;
            }
            let unit = expression.unit.as_str();
            let parsed = rspice_results::calculator::value::Unit::parse(unit);
            panes.iter().position(|pane| {
                !pane.unlabeled
                    && (pane.unit == unit
                        || parsed.is_some()
                            && rspice_results::calculator::value::Unit::parse(pane.unit) == parsed)
            })
        };
        for expression in self.expression_panes.iter().filter(|e| e.visible) {
            if let Some(identity) = &expression.apart {
                // Below the pane of its unit and any difference already
                // drawn apart under it.
                let at = owner(&panes, expression).map_or(panes.len(), |owner| {
                    let mut at = owner + 1;
                    while panes.get(at).is_some_and(|pane| pane.axis != pane.unit) {
                        at += 1;
                    }
                    at
                });
                panes.insert(
                    at,
                    UnitPane {
                        expressions: vec![expression.slot],
                        ..UnitPane::of(identity.as_str(), expression.unit.as_str())
                    },
                );
                drawn.insert(at, true);
                continue;
            }
            match owner(&panes, expression) {
                Some(owner) => {
                    panes[owner].expressions.push(expression.slot);
                    drawn[owner] = true;
                }
                None => {
                    panes.push(UnitPane {
                        expressions: vec![expression.slot],
                        unlabeled: expression.unknown.is_some(),
                        ..UnitPane::of(expression.unit.as_str(), expression.unit.as_str())
                    });
                    drawn.push(true);
                }
            }
        }
        for expression in self.expression_panes.iter().filter(|e| !e.visible) {
            if let Some(owner) = owner(&panes, expression) {
                panes[owner].hidden_expressions.push(expression.slot);
            }
        }
        (panes, drawn)
    }

    /// What a strip lists `index` by: its signal's name and the form drawn.
    pub fn trace_member(&self, index: usize) -> Option<StripMember> {
        self.traces.get(index).map(|trace| StripMember::Signal {
            name: trace.source_waveform_name.clone(),
            form: signal_form(trace.kind),
        })
    }

    /// What a strip lists the expression in `slot` by: its text.
    pub fn expression_member(&self, slot: usize) -> Option<StripMember> {
        self.expression_panes
            .iter()
            .find(|expression| expression.slot == slot)
            .map(|expression| StripMember::Expression {
                text: expression.text.clone(),
            })
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
    /// A stored grid is read off the frame and is `None` until it has arrived.
    pub fn sample_grid(&self) -> Option<rspice_results::waveform::HeldColumn> {
        self.traces
            .first()
            .and_then(|trace| trace.x.fetch_whole().ready())
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
        quantity_policy: rspice_vocabulary::quantity::QuantityPresentationPolicy,
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
        quantity_policy: rspice_vocabulary::quantity::QuantityPresentationPolicy,
    ) -> Option<String> {
        let trace = self.traces.get(trace_index)?;
        let value = trace.y.fetch_whole().ready()?.get(sample).copied()?;
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

/// A phase at the reader's significant digits, in the unit Angle display
/// chose. The digit preference counts digits, not decimal places.
fn phase_sample(
    radians: f64,
    significant_digits: usize,
    quantity_policy: rspice_vocabulary::quantity::QuantityPresentationPolicy,
) -> String {
    use rspice_vocabulary::quantity::AngleDisplay;
    match quantity_policy.angle_display {
        AngleDisplay::Degrees => fmt_si_significant(radians.to_degrees(), "°", significant_digits),
        AngleDisplay::Radians => fmt_si_significant(radians, "rad", significant_digits),
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
#[derive(Debug, Clone, PartialEq)]
pub struct UnitPane<'a> {
    /// The unit every trace on this pane's axis is measured in. Borrowed from
    /// the strip model, because a retained unit is data rather than one of a
    /// closed set of names the viewer knows in advance. On a pane apart it
    /// is only the pane's identity, and [`Self::axis`] is its unit.
    pub unit: &'a str,
    /// The unit the axis reads in: `unit` itself, except on a pane apart.
    pub axis: &'a str,
    /// Trace indices on this pane's axis.
    pub traces: Vec<usize>,
    /// The expressions drawn on this pane's axis, by their place in the
    /// strip's expression list.
    pub expressions: Vec<usize>,
    /// The hidden expressions this pane would draw, by the same places,
    /// which its legend offers to show again.
    pub hidden_expressions: Vec<usize>,
    /// Whether this is the pane of an expression whose unit cannot be
    /// stated: its axis carries no unit, and `unit` is only its identity.
    pub unlabeled: bool,
    /// The Y axes after the first, each with a gutter of its own, when the
    /// reader put curves of other units on this strip: three at most.
    pub secondary: Vec<SecondaryAxis<'a>>,
    /// The key its pinned Y range is kept by in the session's viewports:
    /// the same for the same strip and axis however many strips around it
    /// are shown or hidden. The X window of the whole analysis is kept on
    /// [`SHARED_X_SLOT`].
    pub slot: usize,
    /// Its share of the analysis' height against the other strips'.
    pub weight: f32,
    /// The reader locked its height.
    pub locked: bool,
    /// Whether the strip is one the reader arranged, rather than the strip
    /// a unit has of its own.
    pub arranged: bool,
}

/// One Y axis of a strip after its first: the curves of one unit, read
/// against a gutter of their own beside the strip's first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecondaryAxis<'a> {
    pub unit: &'a str,
    pub traces: Vec<usize>,
    pub expressions: Vec<usize>,
    /// The key its pinned Y range is kept by, as [`UnitPane::slot`].
    pub slot: usize,
}

/// The viewport slot an analysis keeps its shared X window on, which is also
/// the slot of the first unit's strip: a result document states that strip's
/// Y range and the X window in its own axes.
pub const SHARED_X_SLOT: usize = 0;

/// The form of a signal a trace of `kind` draws.
pub const fn signal_form(kind: TraceKind) -> SignalForm {
    match kind {
        TraceKind::Value => SignalForm::Value,
        TraceKind::MagnitudeDb => SignalForm::Magnitude,
        TraceKind::PhaseDeg => SignalForm::Phase,
        TraceKind::Real => SignalForm::Real,
        TraceKind::Imaginary => SignalForm::Imaginary,
        TraceKind::NoiseDensity => SignalForm::NoiseDensity,
    }
}

/// Whether a pane of `unit` reads a phase, which a unit's own strips stack
/// after every other unit's.
fn is_phase_unit(unit: &str) -> bool {
    matches!(unit, "\u{b0}" | "rad")
}

impl<'a> UnitPane<'a> {
    /// An empty pane of identity `unit` whose axis reads in `axis`.
    fn of(unit: &'a str, axis: &'a str) -> Self {
        Self {
            unit,
            axis,
            traces: Vec::new(),
            expressions: Vec::new(),
            hidden_expressions: Vec::new(),
            unlabeled: false,
            secondary: Vec::new(),
            slot: SHARED_X_SLOT,
            weight: 1.0,
            locked: false,
            arranged: false,
        }
    }

    /// The unit the strip's own curves are measured in: on a pane apart,
    /// or one the reader did not arrange, `unit`; on an arranged strip, its
    /// first axis' unit, as `unit` is then the strip's key.
    pub fn quantity_unit(&self) -> &'a str {
        if self.arranged { self.axis } else { self.unit }
    }

    /// Key this strip's axes by what they are rather than where they stand:
    /// by `identity`, the unit of a unit's first strip or a later strip's
    /// own key, and each axis' unit; the first unit's own axis on its first
    /// strip by [`SHARED_X_SLOT`].
    fn assign_slots(&mut self, identity: &str, first_unit: &str) {
        // A strip may read two curves of one unit against two axes, a
        // ripple beside its rail: the later axis is keyed by its turn too.
        let mut seen: Vec<&str> = Vec::new();
        let mut slot = |axis: &'a str| {
            let turn = seen.iter().filter(|held| **held == axis).count();
            seen.push(axis);
            if turn == 0 && identity == first_unit && axis == first_unit {
                SHARED_X_SLOT
            } else if turn == 0 {
                // Odd, so never the shared slot.
                (stable_hash(&(identity, axis)) as usize) | 1
            } else {
                (stable_hash(&(identity, axis, turn)) as usize) | 1
            }
        };
        self.slot = slot(self.axis);
        for secondary in &mut self.secondary {
            secondary.slot = slot(secondary.unit);
        }
    }

    /// The unit this pane's axis reads in: none on an unlabeled pane.
    pub fn axis_unit(&self) -> &str {
        if self.unlabeled { "" } else { self.axis }
    }

    /// Every trace the strip draws, whichever of its axes it reads against.
    pub fn all_traces(&self) -> impl Iterator<Item = usize> + '_ {
        self.traces
            .iter()
            .chain(self.secondary.iter().flat_map(|axis| axis.traces.iter()))
            .copied()
    }

    /// Every expression the strip draws, whichever of its axes it reads
    /// against.
    pub fn all_expressions(&self) -> impl Iterator<Item = usize> + '_ {
        self.expressions
            .iter()
            .chain(
                self.secondary
                    .iter()
                    .flat_map(|axis| axis.expressions.iter()),
            )
            .copied()
    }

    /// The units of the strip's axes, first to last.
    pub fn axis_units(&self) -> impl Iterator<Item = &'a str> + '_ {
        std::iter::once(self.axis).chain(self.secondary.iter().map(|axis| axis.unit))
    }
}

/// Whether a strip's sample grid is finite and non-decreasing.
///
/// Resolved with the model because it is the precondition for answering
/// "which retained sample is this cursor on" by bisection instead of by
/// reading every coordinate. A grid that fails it keeps the linear scan;
/// a parametric sweep is under no obligation to be monotonic.
pub(crate) fn grid_is_ascending(model: &StripModel) -> bool {
    // A stored run's time axis ascends, as its run checked: no need to read it.
    if model
        .traces
        .first()
        .is_some_and(|trace| trace.x.is_ascending_axis())
    {
        return true;
    }
    model.sample_grid().is_some_and(|grid| {
        let mut previous = f64::NEG_INFINITY;
        grid.iter().all(|value| {
            let ordered = value.is_finite() && *value >= previous;
            previous = *value;
            ordered
        })
    })
}

/// `trace`'s value at `x` by `interpolation`, as a cursor reads it. A trace whose samples a
/// results store keeps reads only the rows around `x`, never its whole column.
pub fn trace_value_at(
    trace: &StripTrace,
    x: f64,
    interpolation: rspice_ui_kit::plot::SampleInterpolation,
) -> f64 {
    trace
        .y
        .value_at(&trace.x, x, interpolation)
        .unwrap_or_else(|| match trace.held() {
            Some((xs, ys)) => rspice_ui_kit::plot::sample::sample_at_with_shape(
                &xs,
                &ys,
                &trace.shape,
                x,
                interpolation,
            ),
            None => f64::NAN,
        })
}

#[cfg(test)]
mod stored_tests;

#[cfg(test)]
mod phase_digit_tests {
    use super::*;
    use rspice_vocabulary::quantity::{AngleDisplay, QuantityPresentationPolicy};

    /// The digit preference counts significant digits in a phase, not
    /// places after the point: four digits of 45.678° is 45.68.
    #[test]
    fn a_phase_reads_at_significant_digits_in_the_chosen_unit() {
        let mut policy = QuantityPresentationPolicy {
            angle_display: AngleDisplay::Degrees,
            ..QuantityPresentationPolicy::default()
        };
        let degrees = phase_sample(45.678_f64.to_radians(), 4, policy);
        assert!(degrees.starts_with("45.68"), "{degrees}");
        policy.angle_display = AngleDisplay::Radians;
        let radians = phase_sample(1.234_567, 4, policy);
        assert!(
            radians.starts_with("1.235") && radians.ends_with("rad"),
            "{radians}"
        );
    }
}
