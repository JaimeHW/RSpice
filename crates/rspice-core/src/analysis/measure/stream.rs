//! `.MEASURE TRAN` statements evaluated in one pass over a run's rows, read in order from
//! wherever they are kept, with a fixed amount of state per statement.
//!
//! A transient that writes its rows to a results store measures them there instead of reading
//! the run back. The statements are planned against the store's first rows, which resolves each
//! operand through the same signal table, aliases and expressions as the evaluation in memory;
//! a statement whose operands are all the store's own columns (or the axis) is then evaluated
//! over the rows chunk by chunk. A first pass reads the axis alone, for what a window must know
//! of the whole axis before it opens (where `TD` lands, which way the axis runs); the second
//! reads the columns the statements name. Each evaluator does the arithmetic of its in-memory
//! twin in [`super`] in the same order, so the values are the same to the bit, and the results
//! then go through the same PARAM pass, contracts and units as the in-memory ones. A deck with
//! a statement of any other form is measured in memory.

use super::*;
use crate::SimulationError;
use crate::abort_signal::AbortSignal;
use crate::analysis::measure_signals::{
    evaluate_tran_measurements_directed, measurements_for_analysis,
};
use crate::analysis::transient::TransientResult;
use crate::netlist::Netlist;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// How the measure engine treats the statements of a streamed evaluation.
pub(crate) enum StreamedStatements {
    /// Record each statement's streamed form, by statement place, instead of evaluating it.
    Plan(Mutex<Vec<Option<Spec>>>),
    /// Each statement's result as the pass computed it, by statement place; `None` for one the
    /// engine evaluates itself (a PARAM, which reads the others).
    Results(Vec<Option<MeasureResult>>),
}

/// A value a streamed statement reads at each row. While the statement is planned, a waveform
/// is the address and length of the slice the signal table resolved it to; once planned, it is
/// a column's place among those the stream reads.
#[derive(Clone, Copy, Debug)]
enum Operand {
    Axis,
    Slice { address: usize, len: usize },
    Column(usize),
    Constant(Value),
}

impl Operand {
    fn slice(values: &[Value]) -> Self {
        Self::Slice {
            address: values.as_ptr() as usize,
            len: values.len(),
        }
    }

    #[inline]
    fn value(self, x: Value, row: &[Value]) -> Value {
        match self {
            Self::Axis => x,
            Self::Column(column) => row[column],
            Self::Constant(value) => value,
            // Planning maps every slice to a column before a row is read.
            Self::Slice { .. } => Value::NAN,
        }
    }
}

/// The streamed form of one statement.
#[derive(Clone)]
pub(crate) struct Spec {
    name: String,
    kind: Kind,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Accumulation {
    Average,
    RootMeanSquare,
    Integral,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Extremum {
    Minimum,
    Maximum,
    PeakToPeak,
}

/// What a `WHEN` event gives: its axis (`WHEN`), the value of a signal there (`FIND`), or the
/// slope of the signal's segment (`DERIV`).
#[derive(Clone, Copy, Debug)]
enum WhenReads {
    Axis,
    Find(Operand),
    Derivative(Operand),
}

#[derive(Clone, Copy, Debug)]
enum Clause {
    At(Value),
    When {
        left: Operand,
        right: Operand,
        occurrence: EventOccurrence,
        explicit: bool,
        td: Option<Value>,
    },
}

#[derive(Clone)]
enum Kind {
    Window {
        integral: Accumulation,
        signal: Operand,
        from: Option<Value>,
        to: Option<Value>,
        td: Option<Value>,
    },
    Extrema {
        extremum: Extremum,
        output: ExtremaOutput,
        signal: Operand,
        from: Option<Value>,
        to: Option<Value>,
        td: Option<Value>,
        interpolate_ends: bool,
    },
    At {
        derivative: bool,
        signal: Operand,
        at: Value,
        window: MeasureWindow,
    },
    When {
        reads: WhenReads,
        left: Operand,
        right: Operand,
        occurrence: EventOccurrence,
        window: MeasureWindow,
    },
    Delay {
        trig: Clause,
        targ: Clause,
        minval: Value,
    },
    OnOff {
        statistic: OnOffStatistic,
        signal: Operand,
        window: MeasureWindow,
        on: Value,
        off: Value,
    },
}

impl Spec {
    fn operands_mut(&mut self) -> Vec<&mut Operand> {
        fn clause(clause: &mut Clause) -> Vec<&mut Operand> {
            match clause {
                Clause::At(_) => Vec::new(),
                Clause::When { left, right, .. } => vec![left, right],
            }
        }
        match &mut self.kind {
            Kind::Window { signal, .. }
            | Kind::Extrema { signal, .. }
            | Kind::At { signal, .. }
            | Kind::OnOff { signal, .. } => vec![signal],
            Kind::When {
                reads, left, right, ..
            } => {
                let mut operands = vec![left, right];
                if let WhenReads::Find(signal) | WhenReads::Derivative(signal) = reads {
                    operands.push(signal);
                }
                operands
            }
            Kind::Delay { trig, targ, .. } => {
                let mut operands = clause(trig);
                operands.extend(clause(targ));
                operands
            }
        }
    }

    /// The `TD` whose first sample a window opens at.
    fn delay(&self) -> Option<Value> {
        match &self.kind {
            Kind::Window { td, .. } | Kind::Extrema { td, .. } => *td,
            _ => None,
        }
    }
}

/// The streamed form of `statement` against `data`, the table the in-memory evaluation reads it
/// from; `None` when it has none and the deck is measured in memory.
pub(super) fn plan(
    engine: &MeasureEngine,
    statement: &MeasureStatement,
    data: MeasureData<'_, '_>,
) -> Option<Spec> {
    if !statement.analysis.eq_ignore_ascii_case("TRAN")
        || events::evaluate(
            statement,
            data,
            engine.events.as_deref(),
            engine.use_legacy_tran_trig_targ,
        )
        .is_some()
    {
        return None;
    }
    let column = |name: &str| lookup_signal(data.signals, name).map(Operand::slice);
    let operand = |operand: &MeasureOperand| match operand {
        MeasureOperand::Constant(value) => Some(Operand::Constant(*value)),
        MeasureOperand::Waveform(name) => column(name),
        // An earlier result is not known until the pass has read every row.
        MeasureOperand::Result(_) => None,
    };
    let window =
        |from: Option<Value>, to: Option<Value>, td: Option<Value>, minval: Value| MeasureWindow {
            from,
            to,
            td,
            minval,
        };
    let when = |reads: WhenReads, condition: &WhenCondition, window: MeasureWindow| {
        Some(Kind::When {
            reads,
            left: column(&condition.left)?,
            right: operand(&condition.right)?,
            occurrence: condition.occurrence,
            window,
        })
    };
    let kind = match &statement.measure_type {
        MeasureType::Avg {
            signal,
            from,
            to,
            td,
        }
        | MeasureType::Rms {
            signal,
            from,
            to,
            td,
        }
        | MeasureType::Integ {
            signal,
            from,
            to,
            td,
        } => Kind::Window {
            integral: match statement.measure_type {
                MeasureType::Avg { .. } => Accumulation::Average,
                MeasureType::Rms { .. } => Accumulation::RootMeanSquare,
                _ => Accumulation::Integral,
            },
            signal: column(signal)?,
            from: *from,
            to: *to,
            td: *td,
        },
        MeasureType::Min {
            signal,
            from,
            to,
            td,
            output,
            crossing: None,
            ..
        }
        | MeasureType::Max {
            signal,
            from,
            to,
            td,
            output,
            crossing: None,
            ..
        } => Kind::Extrema {
            extremum: if matches!(statement.measure_type, MeasureType::Max { .. }) {
                Extremum::Maximum
            } else {
                Extremum::Minimum
            },
            output: *output,
            signal: column(signal)?,
            from: *from,
            to: *to,
            td: *td,
            interpolate_ends: engine.interpolate_extrema_window_ends,
        },
        MeasureType::PeakToPeak {
            signal,
            from,
            to,
            td,
            crossing: None,
            ..
        } => Kind::Extrema {
            extremum: Extremum::PeakToPeak,
            output: ExtremaOutput::Value,
            signal: column(signal)?,
            from: *from,
            to: *to,
            td: *td,
            interpolate_ends: engine.interpolate_extrema_window_ends,
        },
        MeasureType::Find {
            signal,
            at,
            when: condition,
            from,
            to,
            td,
            minval,
        }
        | MeasureType::Derivative {
            signal,
            at,
            when: condition,
            from,
            to,
            td,
            minval,
        } => {
            let derivative = matches!(statement.measure_type, MeasureType::Derivative { .. });
            let signal = column(signal)?;
            let window = window(*from, *to, *td, *minval);
            match (at, condition) {
                (Some(at), _) => Kind::At {
                    derivative,
                    signal,
                    at: *at,
                    window,
                },
                (None, Some(condition)) => {
                    let reads = if derivative {
                        WhenReads::Derivative(signal)
                    } else {
                        WhenReads::Find(signal)
                    };
                    when(reads, condition, window)?
                }
                (None, None) => return None,
            }
        }
        MeasureType::When {
            condition,
            from,
            to,
            td,
            minval,
        } => when(WhenReads::Axis, condition, window(*from, *to, *td, *minval))?,
        MeasureType::Delay {
            trig, targ, minval, ..
        } => {
            // The legacy reading gates both clauses by one window and orders the target after
            // the trigger; it is read in memory.
            if engine.use_legacy_tran_trig_targ
                || trig.frac_max.is_some()
                || targ.frac_max.is_some()
            {
                return None;
            }
            let clause = |spec: &TrigSpec, td: Option<Value>| match &spec.event {
                TriggerEvent::At(target) => Some(Clause::At(*target)),
                TriggerEvent::When(condition) => Some(Clause::When {
                    left: column(&condition.left)?,
                    right: operand(&condition.right)?,
                    occurrence: condition.occurrence,
                    explicit: spec.occurrence_explicit,
                    td,
                }),
            };
            Kind::Delay {
                trig: clause(trig, trig.td)?,
                targ: clause(targ, targ.td.or(trig.td))?,
                minval: *minval,
            }
        }
        MeasureType::OnOff {
            statistic,
            signal,
            from,
            to,
            td,
            on,
            off,
            minval,
        } => Kind::OnOff {
            statistic: *statistic,
            signal: column(signal)?,
            window: window(*from, *to, *td, *minval),
            on: *on,
            off: *off,
        },
        _ => return None,
    };
    Some(Spec {
        name: statement.name.clone(),
        kind,
    })
}

/// What a window must know of the whole axis before it opens, read in a first pass.
#[derive(Debug)]
struct AxisFacts {
    count: usize,
    first: Value,
    last: Value,
    /// Whether the axis rises at its first step that moves it; `None` when it never moves.
    first_step_rises: Option<bool>,
    nondecreasing: bool,
    nonincreasing: bool,
    minimum: Value,
    maximum: Value,
    non_finite: bool,
    /// Each `TD` a window opens at, with the first sample at or after it.
    delays: Vec<(Value, Option<Value>)>,
}

impl AxisFacts {
    fn push(&mut self, x: Value) {
        if !x.is_finite() {
            self.non_finite = true;
        }
        if self.count == 0 {
            self.first = x;
            self.minimum = x;
            self.maximum = x;
        } else {
            let previous = self.last;
            if self.first_step_rises.is_none() && previous != x {
                self.first_step_rises = Some(x > previous);
            }
            self.nondecreasing &= previous <= x;
            self.nonincreasing &= previous >= x;
            if x.total_cmp(&self.minimum).is_lt() {
                self.minimum = x;
            }
            if x.total_cmp(&self.maximum).is_gt() {
                self.maximum = x;
            }
        }
        self.last = x;
        self.count += 1;
        for (td, start) in &mut self.delays {
            if start.is_none() && delay_td_accepts_sample(x, Some(*td), 1.0e-12) {
                *start = Some(x);
            }
        }
    }

    /// [`delayed_window_start`] over the whole axis.
    fn delayed_window_start(&self, from: Option<Value>, td: Option<Value>) -> Option<Value> {
        let Some(td) = td else {
            return from;
        };
        let start = self
            .delays
            .iter()
            .find(|(delay, _)| delay.to_bits() == td.to_bits())
            .and_then(|(_, start)| *start)
            .unwrap_or(td);
        Some(from.map_or(start, |from| from.max(start)))
    }

    /// [`MeasureEngine::point_measurement_window_bounds`] of a transient's axis.
    fn point_bounds(&self, window: MeasureWindow) -> (Value, Value) {
        let (mut lower, upper) = MeasureEngine::window_bounds(
            self.first_step_rises.unwrap_or(true),
            window.from,
            window.to,
        );
        if let Some(td) = window.td {
            lower = lower.max(td);
        }
        (lower, upper)
    }

    /// [`delay_at_is_reached`] over the whole axis. Subtracting `minval` keeps the order of the
    /// samples, so the extreme sample decides what any sample would.
    fn delay_at_is_reached(&self, target: Value, minval: Value) -> bool {
        if target < self.minimum || target > self.maximum {
            return false;
        }
        if self.first_step_rises.unwrap_or(true) {
            self.maximum - minval >= target
        } else {
            self.minimum - minval <= target
        }
    }
}

/// `.MEASURE TRAN` statements evaluated over a run's rows in two passes, without the run in
/// memory: [`Self::axis`] over every row's axis value, then [`Self::rows`] over every row of
/// the columns [`Self::columns`] names, both in row order.
pub struct TranMeasureStream {
    /// Each statement's streamed form by its place among the deck's `TRAN` statements; `None`
    /// for a PARAM, which reads the others' results.
    specs: Vec<Option<Spec>>,
    /// The caller's columns the rows carry, in the order [`Self::rows`] takes them.
    columns: Vec<usize>,
    axis: AxisFacts,
    measures: Option<Vec<Option<Measure>>>,
    row: Vec<Value>,
}

impl TranMeasureStream {
    /// Plan the deck's `.MEASURE TRAN` statements against `probe`, a result holding the run's
    /// first rows, whose every column the rows could be read from is among `columns` (`None`
    /// for one that is not).
    ///
    /// `None` when a statement cannot be streamed, and the deck is measured with the run in
    /// memory: one that reads an expression, an event operand, an equation's trace, another
    /// result before the pass has finished, or a column the rows do not carry, and the forms
    /// that need the whole waveform at once.
    pub fn plan(
        netlist: &Netlist,
        probe: &TransientResult,
        columns: &[Option<&[Value]>],
        abort: &dyn AbortSignal,
    ) -> Result<Option<Self>, SimulationError> {
        let statements = measurements_for_analysis(netlist, "TRAN");
        if probe.time.is_empty()
            || probe.current_impulses.is_some()
            || probe.voltage_impulses.is_some()
            || statements
                .iter()
                .any(|statement| matches!(statement.measure_type, MeasureType::Equation { .. }))
        {
            return Ok(None);
        }
        let hook = Arc::new(StreamedStatements::Plan(Mutex::new(vec![
            None;
            statements.len()
        ])));
        evaluate_tran_measurements_directed(netlist, probe, Some(&hook), abort)?;
        let Ok(StreamedStatements::Plan(planned)) = Arc::try_unwrap(hook) else {
            return Ok(None);
        };
        let mut planned = planned
            .into_inner()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let time = (probe.time.as_ptr() as usize, probe.time.len());
        let mut needed: Vec<usize> = Vec::new();
        for (statement, spec) in statements.iter().zip(&mut planned) {
            if matches!(statement.measure_type, MeasureType::Param { .. }) {
                *spec = None;
                continue;
            }
            let Some(spec) = spec else {
                return Ok(None);
            };
            for operand in spec.operands_mut() {
                let Operand::Slice { address, len } = *operand else {
                    continue;
                };
                if (address, len) == time {
                    *operand = Operand::Axis;
                    continue;
                }
                let Some(column) = columns.iter().position(|column| {
                    column.is_some_and(|values| {
                        !values.is_empty()
                            && values.as_ptr() as usize == address
                            && values.len() == len
                    })
                }) else {
                    return Ok(None);
                };
                let place = match needed.iter().position(|&known| known == column) {
                    Some(place) => place,
                    None => {
                        needed.push(column);
                        needed.len() - 1
                    }
                };
                *operand = Operand::Column(place);
            }
        }
        let mut delays: Vec<(Value, Option<Value>)> = Vec::new();
        for td in planned.iter().flatten().filter_map(Spec::delay) {
            if !delays
                .iter()
                .any(|(known, _)| known.to_bits() == td.to_bits())
            {
                delays.push((td, None));
            }
        }
        Ok(Some(Self {
            specs: planned,
            row: vec![0.0; needed.len()],
            columns: needed,
            axis: AxisFacts {
                count: 0,
                first: 0.0,
                last: 0.0,
                first_step_rises: None,
                nondecreasing: true,
                nonincreasing: true,
                minimum: 0.0,
                maximum: 0.0,
                non_finite: false,
                delays,
            },
            measures: None,
        }))
    }

    /// The caller's columns, by their place in the `columns` [`Self::plan`] was given, that
    /// [`Self::rows`] takes, in the order it takes them.
    pub fn columns(&self) -> &[usize] {
        &self.columns
    }

    /// The first pass: the axis of the next rows.
    pub fn axis(&mut self, x: &[Value]) {
        for &x in x {
            self.axis.push(x);
        }
    }

    /// The second pass, once the first has read every row: the axis of the next rows and each of
    /// [`Self::columns`] over the same rows.
    pub fn rows(&mut self, x: &[Value], columns: &[&[Value]]) {
        let Self {
            specs,
            axis,
            measures,
            row,
            ..
        } = self;
        let measures = measures.get_or_insert_with(|| open(specs, axis));
        for (index, &x) in x.iter().enumerate() {
            for (value, column) in row.iter_mut().zip(columns) {
                *value = column[index];
            }
            for measure in measures.iter_mut().flatten() {
                measure.update(x, row);
            }
        }
    }

    /// The deck's `.MEASURE TRAN` results, as [`crate::analysis::evaluate_tran_measurements_with_abort`]
    /// gives them for the whole run, with `probe` the result [`Self::plan`] was given. `None`
    /// when the axis is one the in-memory evaluation refuses (empty or not finite), so that its
    /// refusal is the one reported.
    pub fn finish(
        self,
        netlist: &Netlist,
        probe: &TransientResult,
        abort: &dyn AbortSignal,
    ) -> Result<Option<Vec<MeasureResult>>, SimulationError> {
        let Self {
            specs,
            axis,
            measures,
            ..
        } = self;
        if axis.count == 0 || axis.non_finite {
            return Ok(None);
        }
        let measures = measures.unwrap_or_else(|| open(&specs, &axis));
        let results = specs
            .iter()
            .zip(measures)
            .map(|(spec, measure)| {
                spec.as_ref()
                    .zip(measure)
                    .map(|(spec, measure)| measure.finish(&spec.name))
            })
            .collect();
        let hook = Arc::new(StreamedStatements::Results(results));
        evaluate_tran_measurements_directed(netlist, probe, Some(&hook), abort).map(Some)
    }
}

fn open(specs: &[Option<Spec>], axis: &AxisFacts) -> Vec<Option<Measure>> {
    specs
        .iter()
        .map(|spec| spec.as_ref().map(|spec| Measure::open(spec, axis)))
        .collect()
}

/// One statement's state over the second pass.
enum Measure {
    /// Known from the axis alone.
    Done(MeasureResult),
    Window(WindowState),
    Extrema(ExtremaState),
    At(AtState),
    When(WhenState),
    Delay(Box<DelayState>),
    OnOff(OnOffState),
}

struct WindowState {
    integral: Accumulation,
    signal: Operand,
    lower: Value,
    upper: Value,
    lower_reach: Value,
    upper_reach: Value,
    direction: Value,
    previous: Option<(Value, Value)>,
    reached: bool,
    sum: Value,
    width: Value,
}

struct ExtremaState {
    extremum: Extremum,
    output: ExtremaOutput,
    signal: Operand,
    lower: Value,
    upper: Value,
    /// Whether the axis ascends, when the window's ends between samples count.
    ends: Option<bool>,
    previous: Option<(Value, Value)>,
    lower_end: Option<(Value, Value)>,
    upper_end: Option<(Value, Value)>,
    /// MIN and MAX: the selected sample's axis and value; PP: the least and greatest value.
    selected: Option<(Value, Value)>,
}

struct AtState {
    derivative: bool,
    signal: Operand,
    at: Value,
    minval: Value,
    previous: Option<(Value, Value)>,
    found: Option<Result<Value, &'static str>>,
}

/// A `WHEN` event: its axis and fraction of its segment, whether its current sample is inside
/// MINVAL's band, and the segment's ends of the signal a FIND or DERIV reads there.
#[derive(Clone, Copy)]
struct WhenEvent {
    axis: Value,
    fraction: Value,
    within: bool,
    x0: Value,
    s0: Value,
    x1: Value,
    s1: Value,
}

struct WhenState {
    reads: WhenReads,
    left: Operand,
    right: Operand,
    minval: Value,
    lower: Value,
    upper: Value,
    edge: EdgeType,
    /// The events to keep counting back from the last, for a negative occurrence (`None` when
    /// no event can be one); zero for a positive one.
    keep: Option<usize>,
    requested: usize,
    count: usize,
    previous: Option<(Value, Value, Value, Value)>,
    found: Option<WhenEvent>,
    last: VecDeque<WhenEvent>,
}

enum ClauseState {
    Known(Option<Value>),
    Tracking {
        left: Operand,
        right: Operand,
        tracker: DelayConditionTracker,
        td: Option<Value>,
        /// The events to keep counting back from the last; zero to keep the first.
        keep: usize,
        first: Option<Value>,
        last: VecDeque<Value>,
    },
}

struct DelayState {
    trig: ClauseState,
    targ: ClauseState,
}

struct OnOffState {
    statistic: OnOffStatistic,
    signal: Operand,
    window: MeasureWindow,
    on: Value,
    off: Value,
    accumulator: OnOffAccumulator,
}

impl Measure {
    fn open(spec: &Spec, axis: &AxisFacts) -> Self {
        let name = spec.name.as_str();
        match spec.kind {
            Kind::Window {
                integral,
                signal,
                from,
                to,
                td,
            } => {
                let from = axis.delayed_window_start(from, td);
                let (lower, upper, direction) = match integral {
                    // [`MeasureEngine::eval_avg`] and [`MeasureEngine::eval_rms`].
                    Accumulation::Average | Accumulation::RootMeanSquare => {
                        let (lower, upper) =
                            MeasureEngine::window_bounds(axis.last >= axis.first, from, to);
                        if lower > upper {
                            return Self::Done(MeasureResult::failed(name, "Empty range"));
                        }
                        (lower, upper, 1.0)
                    }
                    // [`MeasureEngine::eval_integ`] of a transient.
                    Accumulation::Integral => {
                        if matches!((from, to), (Some(from), Some(to)) if from > to) {
                            return Self::Done(MeasureResult::failed(name, "Empty range"));
                        }
                        let sweep = Value::signum(axis.last - axis.first);
                        match (from, to) {
                            (Some(from), Some(to)) => {
                                (from.min(to), from.max(to), Value::signum(to - from))
                            }
                            (Some(from), None) if sweep >= 0.0 => (from, Value::INFINITY, sweep),
                            (Some(from), None) => (Value::NEG_INFINITY, from, sweep),
                            (None, Some(to)) if sweep >= 0.0 => (Value::NEG_INFINITY, to, sweep),
                            (None, Some(to)) => (to, Value::INFINITY, sweep),
                            (None, None) => (Value::NEG_INFINITY, Value::INFINITY, sweep),
                        }
                    }
                };
                let tolerance = |bound: Value| {
                    if bound.is_finite() {
                        bound.abs() * 1.0e-12
                    } else {
                        0.0
                    }
                };
                Self::Window(WindowState {
                    integral,
                    signal,
                    lower,
                    upper,
                    lower_reach: lower - tolerance(lower),
                    upper_reach: upper + tolerance(upper),
                    direction,
                    previous: None,
                    reached: false,
                    sum: 0.0,
                    width: 0.0,
                })
            }
            Kind::Extrema {
                extremum,
                output,
                signal,
                from,
                to,
                td,
                interpolate_ends,
            } => {
                let from = axis.delayed_window_start(from, td);
                let (lower, upper) =
                    MeasureEngine::window_bounds(axis.first_step_rises.unwrap_or(true), from, to);
                let ends = if !interpolate_ends || lower > upper {
                    None
                } else if axis.nondecreasing {
                    Some(true)
                } else if axis.nonincreasing {
                    Some(false)
                } else {
                    None
                };
                Self::Extrema(ExtremaState {
                    extremum,
                    output,
                    signal,
                    lower,
                    upper,
                    ends,
                    previous: None,
                    lower_end: None,
                    upper_end: None,
                    selected: None,
                })
            }
            Kind::At {
                derivative,
                signal,
                at,
                window,
            } => {
                if derivative && axis.count < 2 {
                    return Self::Done(MeasureResult::failed(
                        name,
                        "Not enough points for a derivative",
                    ));
                }
                let (lower, upper) = axis.point_bounds(window);
                if !MeasureEngine::axis_in_measurement_window_with_minval(
                    at,
                    lower,
                    upper,
                    window.minval,
                ) {
                    return Self::Done(
                        MeasureResult::failed(name, "AT point is outside the measurement window")
                            .with_event_axis(at),
                    );
                }
                Self::At(AtState {
                    derivative,
                    signal,
                    at,
                    minval: window.minval,
                    previous: None,
                    found: None,
                })
            }
            Kind::When {
                reads,
                left,
                right,
                occurrence,
                window,
            } => {
                if matches!(reads, WhenReads::Derivative(_)) && axis.count < 2 {
                    return Self::Done(MeasureResult::failed(
                        name,
                        "Not enough points for a derivative",
                    ));
                }
                let (lower, upper) = axis.point_bounds(window);
                let keep = if occurrence.number < 0 {
                    occurrence
                        .number
                        .checked_abs()
                        .map(|distance| distance as usize)
                } else {
                    Some(0)
                };
                Self::When(WhenState {
                    reads,
                    left,
                    right,
                    minval: window.minval,
                    lower,
                    upper,
                    edge: occurrence.edge,
                    keep,
                    requested: occurrence.number.max(1) as usize,
                    count: 0,
                    previous: None,
                    found: None,
                    last: VecDeque::new(),
                })
            }
            Kind::Delay { trig, targ, minval } => {
                let clause = |clause: Clause| match clause {
                    Clause::At(target) => ClauseState::Known(
                        (target.is_finite() && axis.delay_at_is_reached(target, minval))
                            .then_some(target),
                    ),
                    Clause::When {
                        left,
                        right,
                        occurrence,
                        explicit,
                        td,
                    } => ClauseState::Tracking {
                        left,
                        right,
                        tracker: DelayConditionTracker::new(
                            occurrence.edge,
                            occurrence.number,
                            explicit,
                            minval,
                        ),
                        td,
                        keep: if occurrence.number < 0 {
                            occurrence.number.unsigned_abs()
                        } else {
                            0
                        },
                        first: None,
                        last: VecDeque::new(),
                    },
                };
                Self::Delay(Box::new(DelayState {
                    trig: clause(trig),
                    targ: clause(targ),
                }))
            }
            Kind::OnOff {
                statistic,
                signal,
                window,
                on,
                off,
            } => Self::OnOff(OnOffState {
                statistic,
                signal,
                window,
                on,
                off,
                accumulator: OnOffAccumulator::default(),
            }),
        }
    }

    #[inline]
    fn update(&mut self, x: Value, row: &[Value]) {
        match self {
            Self::Done(_) => {}
            Self::Window(state) => state.update(x, row),
            Self::Extrema(state) => state.update(x, row),
            Self::At(state) => state.update(x, row),
            Self::When(state) => state.update(x, row),
            Self::Delay(state) => {
                state.trig.update(x, row);
                state.targ.update(x, row);
            }
            Self::OnOff(state) => {
                let MeasureWindow {
                    from,
                    to,
                    td,
                    minval,
                } = state.window;
                if xyce_window_accepts(x, from, to, td, minval) {
                    let value = state.signal.value(x, row);
                    state
                        .accumulator
                        .push(x, value, state.on, state.off, minval);
                }
            }
        }
    }

    fn finish(self, name: &str) -> MeasureResult {
        match self {
            Self::Done(result) => result,
            Self::Window(state) => state.finish(name),
            Self::Extrema(state) => state.finish(name),
            Self::At(state) => {
                let at = state.at;
                match state.found {
                    Some(Ok(value)) => MeasureResult::success(name, value).with_event_axis(at),
                    Some(Err(error)) => MeasureResult::failed(name, error).with_event_axis(at),
                    None => MeasureResult::failed(name, "Time point not in simulation range")
                        .with_event_axis(at),
                }
            }
            Self::When(state) => state.finish(name),
            Self::Delay(state) => {
                let DelayState { trig, targ } = *state;
                let Some(trigger) = trig.event() else {
                    return MeasureResult::failed(name, "Trigger condition not found");
                };
                let Some(target) = targ.event() else {
                    return MeasureResult::failed(name, "Target condition not found");
                };
                MeasureResult::success(name, target - trigger)
            }
            Self::OnOff(state) => match state
                .accumulator
                .result(state.statistic)
                .filter(|value| value.is_finite())
            {
                Some(value) => MeasureResult::success(name, value),
                None => MeasureResult::failed(name, "Empty range"),
            },
        }
    }
}

impl WindowState {
    /// One row of [`windowed_trapezoid`].
    #[inline]
    fn update(&mut self, x: Value, row: &[Value]) {
        let y = self.signal.value(x, row);
        if x >= self.lower_reach && x <= self.upper_reach {
            self.reached = true;
        }
        if let Some(previous) = self.previous {
            let piece = if self.integral == Accumulation::RootMeanSquare {
                clipped_trapezoid(previous, (x, y), self.lower, self.upper, |v| v * v)
            } else {
                clipped_trapezoid(previous, (x, y), self.lower, self.upper, |v| v)
            };
            if let Some(piece) = piece {
                self.sum += piece.integral;
                self.width += piece.width;
                self.reached = true;
            }
        }
        self.previous = Some((x, y));
    }

    fn finish(self, name: &str) -> MeasureResult {
        let window = self.reached.then_some((self.sum, self.width));
        match self.integral {
            Accumulation::Average => match window {
                Some((integral, width)) if width > 0.0 => {
                    MeasureResult::success(name, integral / width)
                }
                _ => MeasureResult::failed(name, "Empty range"),
            },
            Accumulation::RootMeanSquare => match window {
                Some((integral, width)) if width > 0.0 => {
                    MeasureResult::success(name, (integral / width).sqrt())
                }
                _ => MeasureResult::failed(name, "Empty range"),
            },
            Accumulation::Integral => match window {
                Some((integral, _)) => MeasureResult::success(name, integral * self.direction),
                None => MeasureResult::failed(name, "Empty range"),
            },
        }
    }
}

impl ExtremaState {
    /// One row of [`MeasureEngine::eval_min_max`] or [`MeasureEngine::eval_pp`], and of
    /// [`MeasureEngine::extrema_window_ends`].
    #[inline]
    fn update(&mut self, x: Value, row: &[Value]) {
        let y = self.signal.value(x, row);
        if MeasureEngine::axis_in_measurement_window(x, self.lower, self.upper) {
            self.select(x, y);
        }
        if let (Some(ascending), Some((x0, y0))) = (self.ends, self.previous) {
            for (bound, end) in [
                (self.lower, &mut self.lower_end),
                (self.upper, &mut self.upper_end),
            ] {
                if end.is_some() || !bound.is_finite() {
                    continue;
                }
                let inside = if ascending {
                    x0 < bound && bound < x
                } else {
                    x < bound && bound < x0
                };
                if inside {
                    *end = Some((bound, y0 + (y - y0) * (bound - x0) / (x - x0)));
                }
            }
        }
        self.previous = Some((x, y));
    }

    fn select(&mut self, axis: Value, value: Value) {
        match self.extremum {
            Extremum::PeakToPeak => match &mut self.selected {
                None => self.selected = Some((value, value)),
                Some((minimum, maximum)) => {
                    if value < *minimum {
                        *minimum = value;
                    }
                    if value > *maximum {
                        *maximum = value;
                    }
                }
            },
            extremum => {
                let replaces = self.selected.is_none_or(|(_, selected)| {
                    if extremum == Extremum::Maximum {
                        value > selected
                    } else {
                        value < selected
                    }
                });
                if replaces {
                    self.selected = Some((axis, value));
                }
            }
        }
    }

    fn finish(mut self, name: &str) -> MeasureResult {
        let upper = self.upper;
        let lower_end = self.lower_end;
        let upper_end = self
            .upper_end
            .filter(|_| !lower_end.is_some_and(|(end, _)| end == upper));
        for (axis, value) in lower_end.into_iter().chain(upper_end) {
            self.select(axis, value);
        }
        let Some((first, second)) = self.selected else {
            return MeasureResult::failed(name, "Empty range");
        };
        if self.extremum == Extremum::PeakToPeak {
            return MeasureResult::success(name, second - first);
        }
        let mut result = MeasureResult::success(name, second);
        if self.output == ExtremaOutput::IndependentAxis {
            result = result.with_output_projection(first);
        }
        result.with_event_axis(first)
    }
}

impl AtState {
    /// One row of [`find_at_accepted_points`] or [`derivative_at_accepted_points`].
    #[inline]
    fn update(&mut self, x: Value, row: &[Value]) {
        if self.found.is_some() {
            return;
        }
        let y = self.signal.value(x, row);
        if self.derivative {
            if let Some((x0, y0)) = self.previous
                && accepted_row_at_match(Some(x0), x, self.at, self.minval).is_some()
            {
                self.found = Some(Ok(accepted_row_secant_slope(x0, y0, x, y)));
            }
        } else {
            match find_at_accepted_row(self.previous, (x, y), self.at, self.minval) {
                Ok(Some(value)) => self.found = Some(Ok(value)),
                Ok(None) => {}
                Err(error) => self.found = Some(Err(error)),
            }
        }
        self.previous = Some((x, y));
    }
}

impl WhenState {
    /// One row of [`first_measure_condition_event`].
    #[inline]
    fn update(&mut self, x: Value, row: &[Value]) {
        if self.found.is_some() || self.keep.is_none() {
            return;
        }
        let left = self.left.value(x, row);
        let right = self.right.value(x, row);
        let signal = match self.reads {
            WhenReads::Axis => 0.0,
            WhenReads::Find(signal) | WhenReads::Derivative(signal) => signal.value(x, row),
        };
        if let Some((x0, left0, right0, signal0)) = self.previous
            && let Some(crossing) =
                measure_condition_crossing(left0, left, right0, right, self.minval)
        {
            let axis = x0 + crossing.fraction * (x - x0);
            if point_event_axis_in_window(axis, self.lower, self.upper, self.minval) {
                self.event(
                    WhenEvent {
                        axis,
                        fraction: crossing.fraction,
                        within: crossing.current_within_minval,
                        x0,
                        s0: signal0,
                        x1: x,
                        s1: signal,
                    },
                    crossing.direction,
                );
            }
        }
        self.previous = Some((x, left, right, signal));
    }

    /// [`select_measure_condition_occurrence`], one event at a time.
    fn event(&mut self, event: WhenEvent, direction: MeasureConditionDirection) {
        match self.keep {
            Some(0) => {
                self.count += match self.edge {
                    EdgeType::Cross => 1,
                    EdgeType::Rise => usize::from(direction == MeasureConditionDirection::Rise),
                    EdgeType::Fall => usize::from(direction != MeasureConditionDirection::Rise),
                };
                if self.count >= self.requested
                    && edge_matches_measure_condition(self.edge, direction)
                {
                    self.found = Some(event);
                }
            }
            Some(keep) => {
                if edge_matches_measure_condition(self.edge, direction) {
                    self.last.push_back(event);
                    if self.last.len() > keep {
                        self.last.pop_front();
                    }
                }
            }
            None => {}
        }
    }

    fn finish(self, name: &str) -> MeasureResult {
        let event = match self.keep {
            Some(0) => self.found,
            Some(keep) if self.last.len() == keep => self.last.front().copied(),
            _ => None,
        };
        match (self.reads, event) {
            (WhenReads::Axis, Some(event)) => {
                MeasureResult::success(name, event.axis).with_event_axis(event.axis)
            }
            (WhenReads::Find(_), Some(event)) => {
                let value = if event.within {
                    event.s1
                } else {
                    interpolate_extended_real(event.s0, event.s1, event.fraction)
                };
                MeasureResult::success(name, value).with_event_axis(event.axis)
            }
            (WhenReads::Derivative(_), Some(event)) => MeasureResult::success(
                name,
                accepted_row_secant_slope(event.x0, event.s0, event.x1, event.s1),
            )
            .with_event_axis(event.axis),
            (WhenReads::Derivative(_), None) => {
                MeasureResult::failed(name, "WHEN condition never met in the measurement window")
            }
            (_, None) => {
                MeasureResult::failed(name, "WHEN condition not found in the measurement window")
            }
        }
    }
}

impl ClauseState {
    /// One row of [`delay_clause_event`] for a non-legacy clause.
    #[inline]
    fn update(&mut self, x: Value, row: &[Value]) {
        let Self::Tracking {
            left,
            right,
            tracker,
            td,
            keep,
            first,
            last,
        } = self
        else {
            return;
        };
        if *keep == 0 && first.is_some() {
            return;
        }
        let Some(event) = tracker.update_with_td(x, left.value(x, row), right.value(x, row), *td)
        else {
            return;
        };
        if *keep == 0 {
            *first = Some(event);
        } else {
            last.push_back(event);
            if last.len() > *keep {
                last.pop_front();
            }
        }
    }

    fn event(self) -> Option<Value> {
        match self {
            Self::Known(event) => event,
            Self::Tracking { keep: 0, first, .. } => first,
            Self::Tracking { keep, last, .. } => (last.len() == keep)
                .then(|| last.front().copied())
                .flatten(),
        }
    }
}

#[cfg(test)]
mod tests;
