//! Cursor values, branch readings and retained-sample measurements.

use super::cursor_values::slope_text;
use super::pane::SwatchStyle;
use super::{
    DisplayName, NOISE_DENSITY_UNIT, ReadoutPolicy, StripModel, StripTrace, TraceKind,
    cursor_interpolation, trace_key,
};
use crate::derived::{DerivedSeries, WindowStats};
use egui::Ui;
use rspice_results::waveform::{SharedWaveformValues, WaveformValues};
use rspice_ui_kit::panels::section_header;
use rspice_ui_kit::plot::sample::{
    SweepClass, SweepShape, XOrientation, nearest_sample, sample_branches_into,
};
use rspice_ui_kit::plot::{CursorPair, SampleInterpolation, XScale};
use rspice_ui_kit::widgets::table::{Cell, Column, DataTable};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};

pub const READOUT_ROW_H: f32 = 20.0;
pub const READOUT_PAD_X: f32 = 10.0;

/// How many branches of one trace the readout will name separately.
///
/// Two is the hysteresis loop this exists for and six covers a retrace with
/// a few passes. Past that the rows stop being a measurement and become a
/// listing, so the readout says how many branches there are instead of
/// pretending to report each one.
pub const MAX_READOUT_BRANCHES: usize = 6;

/// How one branch of a multi-branch sweep names itself.
///
/// Two branches are the ordinary loop and read as the direction they travel.
/// Past two the direction alone no longer identifies a branch, so its ordinal
/// leads. Plain ASCII throughout: the bundled faces carry no arrow glyph, and
/// a tofu box beside a measured value is worse than a word.
pub fn branch_tag(shape: &SweepShape, run: usize) -> String {
    let direction = match shape.runs().get(run).map(|run| run.orientation) {
        Some(XOrientation::Descending) => "rev",
        _ => "fwd",
    };
    if shape.branch_count() <= 2 {
        direction.to_owned()
    } else {
        format!("r{} {direction}", run + 1)
    }
}

/// How the Measurements table words what it measured.
#[derive(Clone, Copy)]
pub struct MeasurementWording<'a> {
    pub significant_digits: usize,
    pub quantity_policy: QuantityPresentationPolicy,
    pub display_name: DisplayName<'a>,
}

/// Paint retained-sample measurements for the selected strip and cursor span.
///
/// The strip's plotted traces are measured once for each build of the strip
/// models (`generation`) and window, off the frame, and the table lays out
/// only the rows it shows, each named and worded as it is shown: a strip of a
/// hundred thousand ticked signals costs a frame what a strip of ten does.
pub fn measurement_panel(
    ui: &mut Ui,
    derived: &mut DerivedSeries,
    models: &Arc<Vec<StripModel>>,
    measured: usize,
    generation: u64,
    window: Option<(f64, f64)>,
    wording: MeasurementWording<'_>,
) {
    let Some(model) = models.get(measured) else {
        return;
    };
    let title = if window.is_some() {
        "Measurements · A–B"
    } else {
        "Measurements"
    };
    // Linear interpolation between retained samples is how every measurement
    // in the product is taken; stating it on the header of every strip is a
    // constant, not a qualification.
    section_header(ui, title, None);
    let of = MeasuredOf {
        generation,
        strip: super::stable_hash(&model.analysis_key),
        window: window.map(|(a, b)| (a.min(b).to_bits(), a.max(b).to_bits())),
    };
    let shown = derived.measurements_mut().shown(of, ui.ctx(), || PlanOf {
        models: Arc::clone(models),
        measured,
        window,
        signals: model.traces.iter().filter(|trace| trace.visible).count(),
    });
    let columns = [
        Column::name("Measurement", MEASUREMENT_COLUMN_LEAST_W).unsortable(),
        Column::name("Value", MEASUREMENT_COLUMN_LEAST_W).unsortable(),
    ];
    match shown {
        Shown::Measured(measured) => {
            let _ = DataTable::new("results.measurements", &columns, measured.rows)
                .list()
                .show(ui, |row| {
                    let (name, value) = measured.row(model, row, wording);
                    vec![Cell::faint(name), Cell::number(value)]
                });
        }
        Shown::Measuring(signals) => {
            let _ = DataTable::new("results.measurements", &columns[..1], 1)
                .list()
                .show(ui, |_| {
                    vec![Cell::faint(format!(
                        "Measuring {signals} signal{}…",
                        if signals == 1 { "" } else { "s" }
                    ))]
                });
        }
        Shown::Nothing => {}
    }
}

/// The least width of each of the Measurements table's two columns, which
/// share the inspector's width between them.
const MEASUREMENT_COLUMN_LEAST_W: f32 = 64.0;

/// What a strip's measurements were taken of: a build of its models, the
/// strip, and the window's bounds' bits, or none for the whole record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MeasuredOf {
    generation: u64,
    strip: u64,
    window: Option<(u64, u64)>,
}

impl MeasuredOf {
    /// Whether measurements taken of `self` name the same traces as `other`
    /// does: the same build of the same strip, whatever the window.
    fn same_traces(self, other: Self) -> bool {
        self.generation == other.generation && self.strip == other.strip
    }
}

/// One plotted trace, or one branch of it, as measured, and the first of its
/// rows in the table.
#[derive(Debug, Clone, Copy)]
struct MeasuredTrace {
    /// Its index in the strip's traces.
    trace: usize,
    branch: Option<usize>,
    stats: WindowStats,
    first_row: usize,
}

/// A strip's plotted traces measured over one window, in row order.
#[derive(Debug, Default)]
struct Measurements {
    traces: Vec<MeasuredTrace>,
    rows: usize,
}

impl Measurements {
    /// Row `row` of the table, named and worded as `model` names and words
    /// its traces.
    fn row(
        &self,
        model: &StripModel,
        row: usize,
        wording: MeasurementWording<'_>,
    ) -> (String, String) {
        let at = self
            .traces
            .partition_point(|measured| measured.first_row <= row)
            .saturating_sub(1);
        let Some(measured) = self.traces.get(at) else {
            return (String::new(), READOUT_ABSENT.to_owned());
        };
        let Some(trace) = model.traces.get(measured.trace) else {
            return (String::new(), READOUT_ABSENT.to_owned());
        };
        let mut name = (wording.display_name)(&trace.name).into_owned();
        if let Some(branch) = measured.branch {
            name.push(' ');
            name.push_str(&branch_tag(&trace.shape, branch));
        }
        if let Some(tag) = run_tag(model, trace) {
            name.push_str(" · ");
            name.push_str(&tag);
        }
        let stats = match measured.stats {
            Ok(stats) => stats,
            Err(error) => return (name, format!("Unavailable: {error}")),
        };
        let (statistic, value) = match row - measured.first_row {
            0 => ("min", stats.min),
            1 => ("max", stats.max),
            _ => ("rms", stats.rms),
        };
        (
            format!("{name} {statistic}"),
            model.format_trace_value(
                trace,
                value,
                wording.significant_digits,
                wording.quantity_policy,
            ),
        )
    }
}

/// How many rows a measured trace takes: one saying why it was not measured,
/// or its min and max, and its RMS where its kind has one (`rms`).
fn rows_of(stats: &WindowStats, rms: bool) -> usize {
    match stats {
        Err(_) => 1,
        Ok(_) if rms => 3,
        Ok(_) => 2,
    }
}

fn has_rms(kind: TraceKind) -> bool {
    matches!(
        kind,
        TraceKind::Value | TraceKind::Real | TraceKind::Imaginary
    )
}

/// The branches of `trace` the table names, each measured on its own, or the
/// whole trace.
fn measured_branches(trace: &StripTrace) -> Vec<Option<usize>> {
    if trace.shape.class() == SweepClass::MultiBranch
        && trace.shape.branch_count() <= MAX_READOUT_BRANCHES
    {
        (0..trace.shape.branch_count()).map(Some).collect()
    } else {
        vec![None]
    }
}

/// What a measuring measures: the plotted traces of one strip model of a
/// build, planned off the frame with the measuring itself.
struct PlanOf {
    models: Arc<Vec<StripModel>>,
    measured: usize,
    window: Option<(f64, f64)>,
    /// The plotted signals.
    signals: usize,
}

impl PlanOf {
    fn plan(&self) -> MeasurementPlan {
        self.models
            .get(self.measured)
            .map_or_else(MeasurementPlan::default, |model| {
                MeasurementPlan::of(model, self.window)
            })
    }
}

/// A strip's plotted traces, owned whole so they can be measured off the
/// frame.
#[derive(Default)]
struct MeasurementPlan {
    window: Option<(f64, f64)>,
    traces: Vec<PlannedTrace>,
}

struct PlannedTrace {
    trace: usize,
    branch: Option<usize>,
    x: SharedWaveformValues,
    y: SharedWaveformValues,
    shape: Arc<SweepShape>,
    rms: bool,
}

impl MeasurementPlan {
    fn of(model: &StripModel, window: Option<(f64, f64)>) -> Self {
        let mut traces = Vec::new();
        for (index, trace) in model.traces.iter().enumerate() {
            if !trace.visible {
                continue;
            }
            for branch in measured_branches(trace) {
                traces.push(PlannedTrace {
                    trace: index,
                    branch,
                    x: trace.x.clone(),
                    y: trace.y.clone(),
                    shape: Arc::clone(&trace.shape),
                    rms: has_rms(trace.kind),
                });
            }
        }
        Self { window, traces }
    }

    /// Measure every planned trace, or stop when `stop` is raised.
    fn measure(self, stop: &AtomicBool) -> Option<Measurements> {
        let mut rows = 0;
        let mut traces = Vec::with_capacity(self.traces.len());
        for planned in self.traces {
            if stop.load(Ordering::Relaxed) {
                return None;
            }
            let stats = interval_statistics(
                &planned.x,
                &planned.y,
                &planned.shape,
                self.window,
                planned.branch,
            );
            traces.push(MeasuredTrace {
                trace: planned.trace,
                branch: planned.branch,
                stats,
                first_row: rows,
            });
            rows += rows_of(&stats, planned.rms);
        }
        Some(Measurements { traces, rows })
    }
}

/// A measuring under way off the frame: what it measures, its answer once it
/// has one (`None` if it was stopped), and the flag that stops it.
#[derive(Debug, Clone)]
struct Measuring {
    of: MeasuredOf,
    signals: usize,
    answer: Arc<OnceLock<Option<Arc<Measurements>>>>,
    stop: Arc<AtomicBool>,
}

impl Measuring {
    fn start(of: MeasuredOf, plan: PlanOf, ctx: &egui::Context) -> Self {
        let signals = plan.signals;
        let answer = Arc::new(OnceLock::new());
        let stop = Arc::new(AtomicBool::new(false));
        #[cfg(not(target_arch = "wasm32"))]
        {
            let (held, halt, ctx) = (Arc::clone(&answer), Arc::clone(&stop), ctx.clone());
            let under_way = under_way(&ctx);
            under_way.fetch_add(1, Ordering::SeqCst);
            let ending = Arc::clone(&under_way);
            let spawned = std::thread::Builder::new()
                .name("results-measurements".to_owned())
                .spawn(move || {
                    // The traces are taken from the strips before they are
                    // measured, so the strips are not held while they are:
                    // a tick meanwhile changes them in place.
                    let measured = plan.plan();
                    drop(plan);
                    let _ = held.set(measured.measure(&halt).map(Arc::new));
                    ending.fetch_sub(1, Ordering::SeqCst);
                    ctx.request_repaint();
                });
            // With no thread to be had, the table says nothing rather than
            // measure on the frame.
            if spawned.is_err() {
                under_way.fetch_sub(1, Ordering::SeqCst);
                let _ = answer.set(None);
            }
        }
        // The page has one thread: it measures as it starts.
        #[cfg(target_arch = "wasm32")]
        {
            let _ = ctx;
            let _ = answer.set(plan.plan().measure(&stop).map(Arc::new));
        }
        Self {
            of,
            signals,
            answer,
            stop,
        }
    }
}

/// How many measurings `ctx` has under way off the frame.
#[cfg(not(target_arch = "wasm32"))]
fn under_way(ctx: &egui::Context) -> Arc<AtomicUsize> {
    ctx.data_mut(|data| {
        data.get_temp_mut_or_default::<Arc<AtomicUsize>>(egui::Id::new(
            "rspice measurements under way",
        ))
        .clone()
    })
}

/// Whether a Measurements table drawn in `ctx` is measuring off the frame:
/// what a frame-cost test waits out before it counts idle frames.
#[cfg(any(test, feature = "test-support"))]
pub fn measuring_off_frame(ctx: &egui::Context) -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    {
        under_way(ctx).load(Ordering::SeqCst) > 0
    }
    // The page measures as it starts.
    #[cfg(target_arch = "wasm32")]
    {
        let _ = ctx;
        false
    }
}

/// The Measurements table's measurements between frames: the last finished,
/// and the one under way. At most one measures at a time; one asked for
/// while another runs starts once it has stopped.
#[derive(Debug, Clone, Default)]
pub(crate) struct MeasurementsMemo {
    done: Option<(MeasuredOf, Arc<Measurements>)>,
    running: Option<Measuring>,
}

/// What the table shows this frame.
enum Shown {
    Measured(Arc<Measurements>),
    /// Measuring this many signals, with nothing of these traces to show yet.
    Measuring(usize),
    Nothing,
}

impl MeasurementsMemo {
    /// Keep the answer of a measuring that has ended.
    fn collect(&mut self) {
        if let Some(running) = &self.running
            && let Some(answer) = running.answer.get()
        {
            if let Some(measured) = answer {
                self.done = Some((running.of, Arc::clone(measured)));
            }
            self.running = None;
        }
    }

    /// What to show of the measurements of `of`, measuring `plan` when none
    /// is under way: those of `of`, or while they are measured the last of
    /// the same traces over another window.
    fn shown(
        &mut self,
        of: MeasuredOf,
        ctx: &egui::Context,
        plan: impl FnOnce() -> PlanOf,
    ) -> Shown {
        self.collect();
        let current = self.done.as_ref().is_some_and(|(done, _)| *done == of);
        let start = match &self.running {
            Some(running) => {
                if running.of != of {
                    running.stop.store(true, Ordering::Relaxed);
                }
                false
            }
            None => !current,
        };
        if start {
            let plan = plan();
            if plan.signals == 0 {
                self.done = Some((of, Arc::new(Measurements::default())));
            } else {
                self.running = Some(Measuring::start(of, plan, ctx));
                self.collect();
            }
        }
        match (&self.done, &self.running) {
            (Some((done, measured)), _) if done.same_traces(of) => {
                if measured.rows == 0 {
                    Shown::Nothing
                } else {
                    Shown::Measured(Arc::clone(measured))
                }
            }
            (_, Some(running)) => Shown::Measuring(running.signals),
            _ => Shown::Nothing,
        }
    }
}
/// The traces the readout lists, in row order.
///
/// Every per-row producer reads this one predicate, so the value, delta,
/// slope and swatch columns cannot fall out of step with each other.
pub fn readout_traces(model: &StripModel) -> impl Iterator<Item = &StripTrace> {
    model.traces.iter().filter(|trace| trace.visible)
}

/// Nothing was measured here.
pub const READOUT_ABSENT: &str = "—";

/// One row of the cursor readout: what a trace — or one branch of it — says
/// at A, at B, and between them.
///
/// A row is a branch rather than a trace because a sweep that turns around
/// has two answers at the same abscissa, and one composite cell holding both
/// is not a column of measurements any more. The overlay run a row belongs to
/// is named on the row for the same reason: two runs of the same signal
/// otherwise arrive as two rows with one name.
pub struct ReadoutRow {
    pub name: String,
    pub swatch: egui::Color32,
    /// The trace's dash and marker, for a swatch that is its curve; `None`
    /// for an event lane, whose swatch is its colour.
    pub swatch_style: Option<SwatchStyle>,
    pub a: String,
    pub b: String,
    pub delta: String,
    pub slope: String,
}

/// Every row the readout will draw for one strip, in draw order.
///
/// One producer, because the value, difference and slope columns were three
/// independent walks of the same trace list and only stayed aligned by
/// producing equal-length vectors — which stopped being true the moment one
/// trace owed the reader more than one row.
pub fn readout_rows(
    model: &StripModel,
    cursors: CursorPair,
    presentation: ReadoutPolicy,
    quantity_policy: QuantityPresentationPolicy,
    display_name: DisplayName<'_>,
) -> Vec<ReadoutRow> {
    readout_rows_within(
        model,
        cursors,
        (presentation, quantity_policy),
        display_name,
        &|_| None,
    )
}

/// [`readout_rows`], each trace's readings settled against `extent`, the
/// lowest and highest the trace reaches over its record: a reading, a
/// difference or a slope finer than the trace's resolution reads 0.
pub fn readout_rows_within(
    model: &StripModel,
    cursors: CursorPair,
    (presentation, quantity_policy): (ReadoutPolicy, QuantityPresentationPolicy),
    display_name: DisplayName<'_>,
    extent: &dyn Fn(&StripTrace) -> Option<(f64, f64)>,
) -> Vec<ReadoutRow> {
    let positions = [cursors.a, cursors.b];
    let layout = ReadingLayout {
        positions: &positions,
        base: 0,
        deltas: &[1],
        pair: true,
    };
    readout_traces(model)
        .flat_map(|trace| {
            trace_table_rows(
                model,
                trace,
                &layout,
                (presentation, quantity_policy, extent(trace)),
                display_name,
            )
        })
        .map(|row| {
            let mut cells = row.cells.into_iter();
            let mut next = || cells.next().unwrap_or_else(|| READOUT_ABSENT.to_owned());
            ReadoutRow {
                name: row.name,
                swatch: row.swatch,
                swatch_style: row.swatch_style,
                a: next(),
                b: next(),
                delta: next(),
                slope: next(),
            }
        })
        .collect()
}

/// Where a table reads and what it reads against.
pub struct ReadingLayout<'a> {
    /// Each column's X, `None` for a column that stands nowhere: a cursor
    /// not placed, or a search that finds nothing.
    pub positions: &'a [Option<f64>],
    /// The column every difference is taken from.
    pub base: usize,
    /// The columns read against the base, in the order their differences
    /// are listed.
    pub deltas: &'a [usize],
    /// Exactly two columns: their difference has a slope as well.
    pub pair: bool,
}

/// One row of a cursor table: what a trace, or one branch of it, reads at
/// every column, then each difference against the base, then the pair's
/// slope.
pub struct TableRow {
    pub name: String,
    pub swatch: egui::Color32,
    /// The trace's dash and marker, for a swatch that is its curve; `None`
    /// for an event lane, whose swatch is its colour.
    pub swatch_style: Option<SwatchStyle>,
    pub cells: Vec<String>,
}

/// The finest reading a trace reaching from `low` to `high` states at
/// `significant_digits`: its span times the printed significance. A flat
/// trace is resolved against its level instead, so that the rounding noise
/// between two equal readings of it is not a difference.
#[must_use]
pub fn reading_resolution((low, high): (f64, f64), significant_digits: usize) -> f64 {
    let significance = 10f64.powi(-i32::try_from(significant_digits).unwrap_or(i32::MAX));
    let level = low.abs().max(high.abs());
    let scale = (high - low).abs().max(level * significance);
    if scale.is_finite() {
        scale * significance
    } else {
        0.0
    }
}

/// `value`, or 0 when it is finer than `resolution`: what is left of a zero
/// crossing, or of two equal readings, after the floating point.
#[must_use]
pub fn settled(value: f64, resolution: f64) -> f64 {
    if value.abs() < resolution { 0.0 } else { value }
}

/// The suffix that says which run a row's numbers came from.
///
/// The active run owns the signal's name, so only an overlay adds one. Two
/// runs of the same signal are otherwise two rows spelled identically, with
/// nothing in the table to say which is being compared against which.
fn run_tag(model: &StripModel, trace: &StripTrace) -> Option<String> {
    trace
        .overlay
        .then(|| format!("run {}", run_ordinal(model, trace)))
}

/// Which overlay run of the strip this trace belongs to, counting from one in
/// the order the strip drew them.
fn run_ordinal(model: &StripModel, trace: &StripTrace) -> usize {
    model
        .traces
        .iter()
        .filter(|candidate| candidate.overlay)
        .map(|candidate| candidate.run_id)
        .collect::<std::collections::BTreeSet<_>>()
        .iter()
        .position(|run_id| *run_id == trace.run_id)
        .map_or(1, |index| index + 1)
}

/// Every row a trace puts on a cursor table laid out as `layout`: one, or
/// one per branch of a sweep that turns around. Each reading, difference
/// and slope is settled against the trace's resolution over `extent`.
pub fn trace_table_rows(
    model: &StripModel,
    trace: &StripTrace,
    layout: &ReadingLayout<'_>,
    (presentation, quantity_policy, extent): (
        ReadoutPolicy,
        QuantityPresentationPolicy,
        Option<(f64, f64)>,
    ),
    display_name: DisplayName<'_>,
) -> Vec<TableRow> {
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let resolution = extent.map_or(0.0, |extent| reading_resolution(extent, significant_digits));
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    let shape = trace.shape.as_ref();
    let branches = shape.branch_count();
    let qualifiers = |branch: Option<usize>| -> String {
        let mut name = display_name(&trace.name).into_owned();
        if let Some(branch) = branch {
            name.push(' ');
            name.push_str(&branch_tag(shape, branch));
        }
        if let Some(tag) = run_tag(model, trace) {
            name.push_str(" · ");
            name.push_str(&tag);
        }
        name
    };
    let format =
        |value: f64| model.format_trace_value(trace, value, significant_digits, quantity_policy);
    let absent = || READOUT_ABSENT.to_owned();
    let differences = layout.deltas.len() + usize::from(layout.pair);

    // Past the branch budget — or with no branch model at all — the honest
    // answer is the nearest retained sample, and the row says so rather than
    // presenting an interpolation of a curve that has no single value here.
    if branches > MAX_READOUT_BRANCHES || shape.class() == SweepClass::NonSweep {
        let approximate = |x: Option<f64>| {
            x.map_or_else(
                || READOUT_ABSENT.to_owned(),
                |x| {
                    let nearest = trace
                        .held()
                        .map_or(f64::NAN, |(xs, ys)| nearest_sample(&xs, &ys, x));
                    format!("\u{2248} {}", format(nearest))
                },
            )
        };
        let mut cells: Vec<String> = layout.positions.iter().map(|x| approximate(*x)).collect();
        cells.extend(std::iter::repeat_with(absent).take(differences));
        return vec![TableRow {
            name: qualifiers(None),
            swatch: trace.color,
            swatch_style: Some(SwatchStyle::of(trace)),
            cells,
        }];
    }

    let sample = |x: f64, branch: Option<usize>| -> Option<f64> {
        match branch {
            None => Some(model.trace_value(trace, x, interpolation)),
            Some(branch) => {
                let (xs, ys) = trace.held()?;
                let mut out = Vec::new();
                sample_branches_into(&xs, &ys, shape, x, interpolation, &mut out);
                out.iter()
                    .find(|found| found.run == branch)
                    .map(|found| found.value)
            }
        }
    };
    // The pair's run, from the base to the other column.
    let position = |column: usize| layout.positions.get(column).copied().flatten();
    let other = layout.deltas.first().copied();
    let denominator = match (position(layout.base), other.and_then(position)) {
        (Some(a), Some(b)) if layout.pair => match model.x_scale {
            XScale::Log10 if a > 0.0 && b > 0.0 => b.log10() - a.log10(),
            XScale::Log10 => f64::NAN,
            XScale::Linear => b - a,
        },
        _ => f64::NAN,
    };

    let branch_list: Vec<Option<usize>> = if shape.is_monotone() || branches == 0 {
        vec![None]
    } else {
        (0..branches).map(Some).collect()
    };
    branch_list
        .into_iter()
        .map(|branch| {
            let values: Vec<Option<f64>> = layout
                .positions
                .iter()
                .map(|cursor| {
                    cursor
                        .and_then(|x| sample(x, branch))
                        .map(|value| settled(value, resolution))
                })
                .collect();
            let mut cells: Vec<String> = values
                .iter()
                .map(|value| value.map_or_else(absent, &format))
                .collect();
            let base = values.get(layout.base).copied().flatten();
            // A slope is taken from the settled difference, so two equal
            // readings have none rather than one in fV/s.
            let mut first_difference = None;
            for (order, &column) in layout.deltas.iter().enumerate() {
                let difference = match (base, values.get(column).copied().flatten()) {
                    (Some(base), Some(value)) => Some(settled(value - base, resolution)),
                    _ => None,
                };
                if order == 0 {
                    first_difference = difference;
                }
                cells.push(difference.map_or_else(absent, &format));
            }
            if layout.pair {
                let slope = first_difference
                    .filter(|_| denominator.is_finite() && denominator != 0.0)
                    .map(|difference| difference / denominator)
                    .filter(|slope| slope.is_finite());
                cells.push(slope.map_or_else(absent, |slope| {
                    // A trace names its slope in its own unit, never the
                    // strip's: a current sharing a sheet with voltages must
                    // not report mA/ms of rise as volts. `trace_unit` is the
                    // one owner of that mapping.
                    let x_unit = match model.x_scale {
                        XScale::Log10 => "dec",
                        XScale::Linear if model.x_unit.is_empty() => "x",
                        XScale::Linear => model.x_unit.as_str(),
                    };
                    slope_text(slope, model.trace_unit(trace), x_unit, significant_digits)
                }));
            }
            TableRow {
                name: qualifiers(branch),
                swatch: trace.color,
                swatch_style: Some(SwatchStyle::of(trace)),
                cells,
            }
        })
        .collect()
}

/// What the X row says about the shape of the sweep it is reporting, when
/// that shape is not one pass.
pub fn readout_branch_note(model: &StripModel) -> Option<String> {
    let mut branches = 1usize;
    for trace in readout_traces(model) {
        if trace.shape.class() == SweepClass::NonSweep
            || trace.shape.branch_count() > MAX_READOUT_BRANCHES
        {
            return Some("multi-branch sweep".to_owned());
        }
        branches = branches.max(trace.shape.branch_count());
    }
    (branches > 1).then(|| format!("{branches} branches"))
}

/// Interval statistics are cached by source, cursor window, and sweep branch.
pub fn trace_interval_statistics(
    derived: &mut DerivedSeries,
    model: &StripModel,
    trace: &StripTrace,
    window: Option<(f64, f64)>,
    branch: Option<usize>,
) -> WindowStats {
    if window.is_some_and(|(a, b)| !a.is_finite() || !b.is_finite()) {
        return Err(rspice_results::measurements::MeasurementError::NonFiniteWindow);
    }
    let (a_bits, b_bits) = window.map_or((u64::MAX, u64::MAX), |(a, b)| {
        (a.min(b).to_bits(), a.max(b).to_bits())
    });
    let key = (
        trace_key(model, trace),
        a_bits,
        b_bits,
        branch.unwrap_or(usize::MAX),
    );
    derived.stats_or(key, || {
        interval_statistics(&trace.x, &trace.y, &trace.shape, window, branch)
    })
}

/// Min, max and RMS of the samples `x` and `y` hold over `window`, or over
/// the whole of them, of one branch of `shape` or of all of it.
fn interval_statistics(
    x: &WaveformValues,
    y: &WaveformValues,
    shape: &SweepShape,
    window: Option<(f64, f64)>,
    branch: Option<usize>,
) -> WindowStats {
    use rspice_results::measurements::{MeasurementError, measure_interval};

    if window.is_some_and(|(a, b)| !a.is_finite() || !b.is_finite()) {
        return Err(MeasurementError::NonFiniteWindow);
    }
    if x.len() != y.len() {
        return Err(MeasurementError::LengthMismatch);
    }
    let (range, ascending) = match branch {
        Some(index) => {
            let run = shape
                .runs()
                .get(index)
                .ok_or(MeasurementError::NonMonotoneAxis)?;
            (
                run.start..run.end,
                run.orientation == rspice_results::sampling::XOrientation::Ascending,
            )
        }
        None => (0..x.len(), shape.is_single_ascending()),
    };
    // A window over an ascending trace measures only the samples in it and
    // one either side, found by bisection; a stored trace reads just those
    // rows and keeps none of them.
    let range = match window {
        Some((a, b)) if ascending => window_rows(x, range, a.min(b), a.max(b)),
        _ => range,
    };
    // A stored window that cannot be read has no samples to measure.
    let unread = |_| MeasurementError::IncompleteCoverage { fraction: 0.0 };
    let xs = x.read_range(range.clone()).map_err(unread)?;
    if xs.iter().any(|v| !v.is_finite()) {
        return Err(MeasurementError::NonFiniteAxis);
    }
    let ys = y.read_range(range).map_err(unread)?;
    measure_interval(&xs, &ys, window)
}

/// The samples of `range` of the ascending axis `x` that a measurement over
/// `low ..= high` reads: those inside it and the nearest one either side,
/// found by bisection over single samples.
fn window_rows(
    x: &WaveformValues,
    range: std::ops::Range<usize>,
    low: f64,
    high: f64,
) -> std::ops::Range<usize> {
    let sample = |index: usize| {
        x.read_range(index..index + 1)
            .ok()
            .and_then(|value| value.first().copied())
            .unwrap_or(f64::NAN)
    };
    // The first index of `range` whose sample is not `before` the bound.
    let first_not = |before: &dyn Fn(f64) -> bool| {
        let (mut lo, mut hi) = (range.start, range.end);
        while lo < hi {
            let middle = lo + (hi - lo) / 2;
            if before(sample(middle)) {
                lo = middle + 1;
            } else {
                hi = middle;
            }
        }
        lo
    };
    let start = first_not(&|value| value < low)
        .saturating_sub(1)
        .max(range.start);
    let end = (first_not(&|value| value <= high) + 1).min(range.end);
    start..end.max(start)
}

pub fn measurement_values(
    derived: &mut DerivedSeries,
    model: &StripModel,
    window: Option<(f64, f64)>,
    significant_digits: usize,
    quantity_policy: QuantityPresentationPolicy,
    display_name: DisplayName<'_>,
) -> Vec<(String, String)> {
    let mut rows = 0;
    let mut traces = Vec::new();
    for (index, trace) in model.traces.iter().enumerate() {
        if !trace.visible {
            continue;
        }
        for branch in measured_branches(trace) {
            let stats = trace_interval_statistics(derived, model, trace, window, branch);
            traces.push(MeasuredTrace {
                trace: index,
                branch,
                stats,
                first_row: rows,
            });
            rows += rows_of(&stats, has_rms(trace.kind));
        }
    }
    let measured = Measurements { traces, rows };
    let wording = MeasurementWording {
        significant_digits,
        quantity_policy,
        display_name,
    };
    // An arbitrary cap the reader had to work around by hiding traces is not
    // a measurement, so the row that announced it goes.
    (0..measured.rows)
        .map(|row| measured.row(model, row, wording))
        .collect()
}

/// Append the exact cursor values using the explicit copied-value policy.
pub fn append_copied_cursor(
    target: &mut String,
    cursor: &str,
    x: f64,
    model: &StripModel,
    interpolation: SampleInterpolation,
    policy: QuantityPresentationPolicy,
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
    for trace in readout_traces(model) {
        // The copy has to say what the table says. Read unshaped, a loop's
        // line pasted a value off the far side of its turnaround while the
        // register on screen reported each branch.
        let value = model.trace_value(trace, x, interpolation);
        let copied = copied_trace_value(policy, trace.kind, value, model.trace_unit(trace));
        let _ = writeln!(target, "{} = {}", trace.name, copied.trim_end());
    }
    while target.ends_with('\n') {
        target.pop();
    }
}

/// One trace's value as Copied values writes it. The unit is the trace's
/// own, not the strip's: copying a supply current off a sheet it shares
/// with node voltages must not paste milliamps as millivolts.
fn copied_trace_value(
    policy: QuantityPresentationPolicy,
    kind: TraceKind,
    value: f64,
    unit: &str,
) -> String {
    match kind {
        TraceKind::PhaseDeg => policy.copy_angle_degrees(value),
        TraceKind::NoiseDensity => policy.copy_scaled_unit_value(value, NOISE_DENSITY_UNIT),
        TraceKind::MagnitudeDb
        | TraceKind::Value
        | TraceKind::Real
        | TraceKind::Imaginary
        | TraceKind::GroupDelay => policy.copy_si_value(value, unit),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_vocabulary::quantity::{AngleDisplay, CopiedValueFormat};

    /// A phase trace holds degrees. Its copy went through radians and back,
    /// which landed a unit in the last place off for about one degree value
    /// in eight, so the pasted phase was not the one on the trace.
    #[test]
    fn a_copied_phase_reads_back_as_the_degrees_on_the_trace() {
        let policy = QuantityPresentationPolicy {
            copied_value_format: CopiedValueFormat::EngineeringNotationWithUnit,
            angle_display: AngleDisplay::Degrees,
            ..QuantityPresentationPolicy::default()
        };
        let mut degrees = (-72_000..=72_000)
            .map(|step| f64::from(step) * 0.01)
            .collect::<Vec<_>>();
        degrees.extend([1.0 / 3.0, 0.1, 5e-324, f64::MIN_POSITIVE, 1e300]);
        for exponent in -300..=300 {
            for mantissa in [1.0, 1.234_567_890_123_456_7, 3.2, 9.999_999_999_999_998] {
                let value = mantissa * 10_f64.powi(exponent);
                degrees.extend([value, -value]);
            }
        }
        for value in degrees {
            let copied = copied_trace_value(policy, TraceKind::PhaseDeg, value, "°");
            let (number, unit) = copied.split_once(' ').expect("a value and its unit");
            assert_eq!(unit, "°");
            let read_back: f64 = number.parse().expect("the copy parses as a number");
            assert_eq!(
                read_back.to_bits(),
                value.to_bits(),
                "{value:e} copied as {copied}"
            );
        }
    }
}
