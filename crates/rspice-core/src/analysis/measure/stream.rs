//! `.MEASURE TRAN` statements evaluated in passes over a run's rows, read in order from
//! wherever they are kept, with a fixed amount of state per statement.
//!
//! A transient that writes its rows to a results store measures them there instead of reading
//! the run back. The statements are planned against the store's first rows, which resolves each
//! operand through the same signal table, aliases and expressions as the evaluation in memory;
//! a statement whose operands are the store's own columns (or the axis) is then evaluated over
//! the rows chunk by chunk, and one that reads an expression, a differential voltage or an
//! alias, over the values the same evaluation derives from each chunk's rows. A first pass
//! reads the axis alone, for what a window must know of the whole axis before it opens (where
//! `TD` lands, which way the axis runs); the next reads the columns the statements name, and
//! one more follows for a statement that reads an earlier one's result, and for a legacy
//! TRIG/TARG's target. Each evaluator does the arithmetic of its in-memory twin in [`super`] in
//! the same order, so the values are the same to the bit, and the results then go through the
//! same PARAM pass, contracts and units as the in-memory ones. A deck with a statement of any
//! other form is measured in memory.

use super::*;
use crate::SimulationError;
use crate::abort_signal::AbortSignal;
use crate::analysis::measure_signals::{
    evaluate_tran_measurements_directed, measurements_for_analysis, statement_live_dependencies,
};
use crate::analysis::transient::TransientResult;
use crate::netlist::Netlist;
use std::collections::{HashSet, VecDeque};
use std::sync::{Arc, Mutex};

/// How the measure engine treats the statements of a streamed evaluation.
pub(crate) enum StreamedStatements {
    /// Record each statement's streamed form, by statement place, instead of evaluating it.
    Plan(Mutex<Vec<Option<Spec>>>),
    /// Copy out the values of a chunk of rows that the rows themselves do not carry.
    Chunk(Mutex<ChunkReads>),
    /// Each statement's result as the pass computed it, by statement place; `None` for one the
    /// engine evaluates itself (a PARAM, which reads the others).
    Results(Vec<Option<MeasureResult>>),
}

/// A value a streamed statement reads at each row. While the statement is planned, a waveform
/// is the address and length of the slice the signal table resolved it to, with the place of
/// its name among the statement's; once planned, it is a column's place among those the stream
/// reads, the rows' own columns first and then the values each chunk derives.
#[derive(Clone, Copy, Debug)]
enum Operand {
    Axis,
    Slice {
        address: usize,
        len: usize,
        name: usize,
    },
    /// While planned: a value each chunk derives, by its place among those.
    Derived(usize),
    Column(usize),
    Constant(Value),
}

impl Operand {
    #[inline]
    fn value(self, x: Value, row: &[Value]) -> Value {
        match self {
            Self::Axis => x,
            Self::Column(column) => row[column],
            Self::Constant(value) => value,
            // Planning maps every slice to a column before a row is read.
            Self::Slice { .. } | Self::Derived(_) => Value::NAN,
        }
    }
}

/// The values of one chunk of rows a streamed evaluation reads but the rows do not carry (an
/// expression, a differential voltage, an alias), copied out of the signal table the measure
/// engine builds over the chunk.
pub(crate) struct ChunkReads {
    /// Each value's statement place and the name the statement reads it by.
    wanted: Vec<(usize, String)>,
    values: Vec<Vec<Value>>,
    found: Vec<bool>,
}

/// Copy out of `data` the values of the chunk that statement `index` reads.
pub(super) fn read_chunk(reads: &Mutex<ChunkReads>, index: usize, data: MeasureData<'_, '_>) {
    let mut reads = reads
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let ChunkReads {
        wanted,
        values,
        found,
    } = &mut *reads;
    for ((statement, name), (values, found)) in wanted.iter().zip(values.iter_mut().zip(found)) {
        if *statement != index {
            continue;
        }
        if let Some(column) =
            lookup_signal(data.signals, name).filter(|column| column.len() == data.axis.len())
        {
            values.clear();
            values.extend_from_slice(column);
            *found = true;
        }
    }
}

/// Whether `name`, a value a chunk derives, reads rows other than its own: a time integral
/// (`sdt`) carries its sum from row to row, so a chunk cannot give it alone.
pub(crate) fn reads_other_rows(name: &str) -> bool {
    let name = name.to_ascii_lowercase();
    let bytes = name.as_bytes();
    name.match_indices("sdt").any(|(at, _)| {
        let starts = at == 0 || !(bytes[at - 1].is_ascii_alphanumeric() || bytes[at - 1] == b'_');
        starts && name[at + 3..].trim_start().starts_with('(')
    })
}

/// The streamed form of one statement.
#[derive(Clone)]
pub(crate) struct Spec {
    name: String,
    kind: Kind,
    /// The names of the waveforms the statement reads, by the place a slice records.
    names: Vec<String>,
    /// The earlier result a FIND or DERIV's `WHEN` compares with, which a later pass reads
    /// once the earlier statement's pass is over.
    earlier: Option<String>,
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
        /// RISE, FALL or CROSS with its level: only the samples of that step count.
        crossing: Option<(EventOccurrence, Option<Value>)>,
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
    /// Xyce's legacy `RiseFallDelay`: one window gates both clauses, and the target counts
    /// only after the trigger, so the target takes a second pass.
    LegacyDelay {
        trig: Clause,
        targ: Clause,
        window: MeasureWindow,
    },
    /// A statement that reads only event operands: its event traces are in memory, and it
    /// reads of the sampled run only what the run's first and last rows give, so it is
    /// evaluated on those two rows once the passes are over.
    Span,
    /// A result the axis cannot change.
    Fixed(MeasureResult),
    OnOff {
        statistic: OnOffStatistic,
        signal: Operand,
        window: MeasureWindow,
        on: Value,
        off: Value,
    },
}

impl Kind {
    fn operands_mut(&mut self) -> Vec<&mut Operand> {
        fn clause(clause: &mut Clause) -> Vec<&mut Operand> {
            match clause {
                Clause::At(_) => Vec::new(),
                Clause::When { left, right, .. } => vec![left, right],
            }
        }
        match self {
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
            Kind::Delay { trig, targ, .. } | Kind::LegacyDelay { trig, targ, .. } => {
                let mut operands = clause(trig);
                operands.extend(clause(targ));
                operands
            }
            Kind::Span | Kind::Fixed(_) => Vec::new(),
        }
    }
}

impl Spec {
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
    if !statement.analysis.eq_ignore_ascii_case("TRAN") {
        return None;
    }
    if events::evaluate(
        statement,
        data,
        engine.events.as_deref(),
        engine.use_legacy_tran_trig_targ,
    )
    .is_some()
    {
        return events::reads_only_events(statement, engine.events.as_deref()).then(|| Spec {
            name: statement.name.clone(),
            kind: Kind::Span,
            earlier: None,
            names: Vec::new(),
        });
    }
    let mut earlier = None;
    let names = std::cell::RefCell::new(Vec::<String>::new());
    let column = |name: &str| {
        let values = lookup_signal(data.signals, name)?;
        let mut names = names.borrow_mut();
        let place = match names.iter().position(|known| known == name) {
            Some(place) => place,
            None => {
                names.push(name.to_owned());
                names.len() - 1
            }
        };
        Some(Operand::Slice {
            address: values.as_ptr() as usize,
            len: values.len(),
            name: place,
        })
    };
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
    // A FIND or DERIV at an earlier result reads it as a constant once its pass is over.
    let at_earlier = |reads: WhenReads, condition: &WhenCondition, window: MeasureWindow| {
        Some(Kind::When {
            reads,
            left: column(&condition.left)?,
            right: Operand::Constant(Value::NAN),
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
            crossing,
            rfc_level,
        }
        | MeasureType::Max {
            signal,
            from,
            to,
            td,
            output,
            crossing,
            rfc_level,
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
            crossing: crossing.map(|crossing| (crossing, *rfc_level)),
        },
        MeasureType::PeakToPeak {
            signal,
            from,
            to,
            td,
            crossing,
            rfc_level,
        } => Kind::Extrema {
            extremum: Extremum::PeakToPeak,
            output: ExtremaOutput::Value,
            signal: column(signal)?,
            from: *from,
            to: *to,
            td: *td,
            interpolate_ends: engine.interpolate_extrema_window_ends,
            crossing: crossing.map(|crossing| (crossing, *rfc_level)),
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
                    match &condition.right {
                        MeasureOperand::Result(name) => {
                            earlier = Some(name.clone());
                            at_earlier(reads, condition, window)?
                        }
                        _ => when(reads, condition, window)?,
                    }
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
            trig,
            targ,
            from,
            to,
            minval,
        } => {
            // FRAC_MAX moves its level with the running maximum, and Xyce replays the history it
            // keeps whenever the maximum grows; it is read in memory.
            if trig.frac_max.is_some() || targ.frac_max.is_some() {
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
            if engine.use_legacy_tran_trig_targ {
                if matches!(targ.event, TriggerEvent::At(_)) {
                    Kind::Fixed(MeasureResult::failed(
                        &statement.name,
                        "AT keyword not allowed in legacy TARG block",
                    ))
                } else {
                    let td = targ.td.or(trig.td);
                    Kind::LegacyDelay {
                        trig: clause(trig, td)?,
                        targ: clause(targ, td)?,
                        window: window(*from, *to, td, *minval),
                    }
                }
            } else {
                Kind::Delay {
                    trig: clause(trig, trig.td)?,
                    targ: clause(targ, targ.td.or(trig.td))?,
                    minval: *minval,
                }
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
        earlier,
        names: names.into_inner(),
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

/// `.MEASURE TRAN` statements evaluated over a run's rows without the run in memory:
/// [`Self::axis`] over every row's axis value, then [`Self::rows`] over every row of the columns
/// [`Self::columns`] names, both in row order, and again from the first row while
/// [`Self::next_pass`] asks for another pass (for a FIND at an earlier result, and for the
/// legacy TRIG/TARG target, which counts only after its trigger).
pub struct TranMeasureStream {
    /// Each statement's streamed form by its place among the deck's `TRAN` statements; `None`
    /// for a PARAM, which reads the others' results.
    specs: Vec<Option<Spec>>,
    /// The earlier statement each FIND or DERIV at an earlier result reads.
    earlier: Vec<Option<usize>>,
    /// The caller's columns the rows carry, in the order [`Self::rows`] takes them.
    columns: Vec<usize>,
    /// The values each chunk derives, for the rows [`Self::derive`] last read.
    reads: Option<ChunkReads>,
    axis: AxisFacts,
    slots: Option<Vec<Slot>>,
    row: Vec<Value>,
}

/// Where one statement is in the passes.
enum Slot {
    /// A PARAM, or a statement the engine evaluates on the run's span.
    Engine,
    /// Waiting for the earlier result it reads.
    Waiting,
    Active(Measure),
    Done(MeasureResult),
}

impl TranMeasureStream {
    /// Plan the deck's `.MEASURE TRAN` statements against `probe`, a result holding the run's
    /// first rows, whose every column the rows could be read from is among `columns` (`None`
    /// for one that is not). A waveform none of them is (an expression, a differential
    /// voltage, an alias, a column the rows do not carry) is derived from each chunk's rows of
    /// every column by [`Self::derive`].
    ///
    /// `None` when a statement cannot be streamed, and the deck is measured with the run in
    /// memory:
    /// - an expression that reads other rows (`sdt`) or another statement's running result,
    ///   which a chunk alone does not give;
    /// - an event operand beside a sampled one, whose sampled values are read at the events;
    /// - FRAC_MAX, whose level moves with the running maximum while Xyce replays its history;
    /// - an equation, an error function or a FOUR statement (forms with no pass of their own);
    /// - a run with impulses, whose integrals add them.
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
        let mut earlier = vec![None; planned.len()];
        let time = (probe.time.as_ptr() as usize, probe.time.len());
        let mut needed: Vec<usize> = Vec::new();
        let mut derived: Vec<(usize, String)> = Vec::new();
        let measured = statements
            .iter()
            .map(|statement| statement.name.to_ascii_uppercase())
            .collect::<HashSet<_>>();
        for (index, (statement, spec)) in statements.iter().zip(&mut planned).enumerate() {
            if matches!(statement.measure_type, MeasureType::Param { .. }) {
                *spec = None;
                continue;
            }
            let Some(spec) = spec else {
                return Ok(None);
            };
            if let Some(name) = &spec.earlier {
                // The last statement before this one with that name, as the engine finds it.
                earlier[index] = statements[..index]
                    .iter()
                    .rposition(|statement| statement.name.eq_ignore_ascii_case(name));
            }
            let mut derivable = None;
            for operand in spec.kind.operands_mut() {
                let Operand::Slice { address, len, name } = *operand else {
                    continue;
                };
                if (address, len) == time {
                    *operand = Operand::Axis;
                    continue;
                }
                if let Some(column) = columns.iter().position(|column| {
                    column.is_some_and(|values| {
                        !values.is_empty()
                            && values.as_ptr() as usize == address
                            && values.len() == len
                    })
                }) {
                    let place = match needed.iter().position(|&known| known == column) {
                        Some(place) => place,
                        None => {
                            needed.push(column);
                            needed.len() - 1
                        }
                    };
                    *operand = Operand::Column(place);
                    continue;
                }
                // A value the rows do not carry (an expression, a differential voltage, an
                // alias) is derived from each chunk's rows alone, which gives the same values
                // as the whole run when the value reads no other row and no other statement's
                // running result.
                let name = &spec.names[name];
                let derivable = *derivable.get_or_insert_with(|| {
                    statement_live_dependencies(statement, &netlist.params).is_disjoint(&measured)
                });
                if !derivable || reads_other_rows(name) {
                    return Ok(None);
                }
                let place = match derived.iter().position(|(_, known)| known == name) {
                    Some(place) => place,
                    None => {
                        derived.push((index, name.clone()));
                        derived.len() - 1
                    }
                };
                *operand = Operand::Derived(place);
            }
        }
        for spec in planned.iter_mut().flatten() {
            for operand in spec.kind.operands_mut() {
                if let Operand::Derived(place) = *operand {
                    *operand = Operand::Column(needed.len() + place);
                }
            }
        }
        // A result read during the passes must come from them, not from the span.
        if earlier.iter().flatten().any(|&found| {
            matches!(
                planned[found].as_ref().map(|spec| &spec.kind),
                Some(Kind::Span)
            )
        }) {
            return Ok(None);
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
            earlier,
            row: vec![0.0; needed.len() + derived.len()],
            columns: needed,
            reads: Some(ChunkReads {
                values: vec![Vec::new(); derived.len()],
                found: vec![false; derived.len()],
                wanted: derived,
            }),
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
            slots: None,
        }))
    }

    /// The caller's columns, by their place in the `columns` [`Self::plan`] was given, that
    /// [`Self::rows`] takes, in the order it takes them.
    pub fn columns(&self) -> &[usize] {
        &self.columns
    }

    /// Whether the statements read values the rows do not carry, which [`Self::derive`] gives
    /// before each chunk's [`Self::rows`].
    pub fn derives(&self) -> bool {
        self.reads
            .as_ref()
            .is_some_and(|reads| !reads.wanted.is_empty())
    }

    /// Derive the values the next rows do not carry from `chunk`, a result holding those rows
    /// of every column the run saved; `false` when one has no value there (an expression that
    /// fails on one of the rows), and the run is measured in memory.
    pub fn derive(
        &mut self,
        netlist: &Netlist,
        chunk: &TransientResult,
        abort: &dyn AbortSignal,
    ) -> Result<bool, SimulationError> {
        let Some(mut reads) = self.reads.take() else {
            return Ok(false);
        };
        reads.found.fill(false);
        let hook = Arc::new(StreamedStatements::Chunk(Mutex::new(reads)));
        evaluate_tran_measurements_directed(netlist, chunk, Some(&hook), abort)?;
        let Ok(StreamedStatements::Chunk(reads)) = Arc::try_unwrap(hook) else {
            return Ok(false);
        };
        let reads = reads
            .into_inner()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let complete = reads.found.iter().all(|&found| found);
        self.reads = Some(reads);
        Ok(complete)
    }

    /// The first pass: the axis of the next rows.
    pub fn axis(&mut self, x: &[Value]) {
        for &x in x {
            self.axis.push(x);
        }
    }

    /// A pass over the values, once the axis pass has read every row: the axis of the next
    /// rows and each of [`Self::columns`] over the same rows.
    pub fn rows(&mut self, x: &[Value], columns: &[&[Value]]) {
        self.open();
        let Self {
            slots, row, reads, ..
        } = self;
        let Some(slots) = slots else {
            return;
        };
        let derived = reads.as_ref().map_or(&[][..], |reads| &reads.values[..]);
        let stored = columns.len().min(row.len());
        for (index, &x) in x.iter().enumerate() {
            for (value, column) in row[..stored].iter_mut().zip(columns) {
                *value = column[index];
            }
            for (value, column) in row[stored..].iter_mut().zip(derived) {
                *value = column[index];
            }
            for slot in slots.iter_mut() {
                if let Slot::Active(measure) = slot {
                    measure.update(x, row);
                }
            }
        }
    }

    /// End a pass over the values; `true` when another pass from the first row is needed.
    pub fn next_pass(&mut self) -> bool {
        self.open();
        let Self {
            specs,
            earlier,
            axis,
            slots,
            ..
        } = self;
        let Some(slots) = slots else {
            return false;
        };
        for (slot, spec) in slots.iter_mut().zip(specs.iter()) {
            if !matches!(slot, Slot::Active(_)) {
                continue;
            }
            let Slot::Active(measure) = std::mem::replace(slot, Slot::Engine) else {
                continue;
            };
            let name = spec.as_ref().map_or("", |spec| spec.name.as_str());
            *slot = measure.end_pass(name);
        }
        for index in 0..slots.len() {
            if !matches!(slots[index], Slot::Waiting) {
                continue;
            }
            let (Some(found), Some(spec)) = (earlier[index], specs[index].as_ref()) else {
                continue;
            };
            let value = match &slots[found] {
                Slot::Done(result) => result.value,
                // A PARAM has no result while the others are evaluated; the engine refuses
                // the read itself.
                Slot::Engine => None,
                Slot::Waiting | Slot::Active(_) => continue,
            };
            slots[index] = match value {
                Some(value) => Slot::Active(Measure::open(spec, axis, Some(value))),
                None => Slot::Done(MeasureResult::failed(&spec.name, "no earlier result")),
            };
        }
        slots.iter().any(|slot| matches!(slot, Slot::Active(_)))
    }

    fn open(&mut self) {
        if self.slots.is_some() {
            return;
        }
        let slots = self
            .specs
            .iter()
            .zip(&self.earlier)
            .map(|(spec, earlier)| match (spec, earlier) {
                (None, _) => Slot::Engine,
                (Some(spec), _) if matches!(spec.kind, Kind::Span) => Slot::Engine,
                (
                    Some(Spec {
                        kind: Kind::Fixed(result),
                        ..
                    }),
                    _,
                ) => Slot::Done(result.clone()),
                // The engine refuses a read of a result no earlier statement has.
                (
                    Some(
                        spec @ Spec {
                            earlier: Some(_), ..
                        },
                    ),
                    None,
                ) => Slot::Done(MeasureResult::failed(&spec.name, "no earlier result")),
                (Some(_), Some(_)) => Slot::Waiting,
                (Some(spec), None) => Slot::Active(Measure::open(spec, &self.axis, None)),
            })
            .collect();
        self.slots = Some(slots);
    }

    /// The deck's `.MEASURE TRAN` results, as
    /// [`crate::analysis::evaluate_tran_measurements_with_abort`] gives them for the whole run,
    /// once [`Self::next_pass`] has asked for no more passes. `span` is the result
    /// [`Self::plan`] was given, holding instead the run's first and last rows: a statement
    /// that reads only event operands is evaluated on it. `None` when the axis is one the
    /// in-memory evaluation refuses (empty or not finite) or one the span does not stand for,
    /// so that the run is measured in memory.
    pub fn finish(
        mut self,
        netlist: &Netlist,
        span: &TransientResult,
        abort: &dyn AbortSignal,
    ) -> Result<Option<Vec<MeasureResult>>, SimulationError> {
        if self.axis.count == 0 || self.axis.non_finite {
            return Ok(None);
        }
        let spans = self
            .specs
            .iter()
            .flatten()
            .any(|spec| matches!(spec.kind, Kind::Span));
        if spans && !self.axis.nondecreasing {
            return Ok(None);
        }
        self.open();
        let Self { slots, .. } = self;
        let mut results = Vec::new();
        for slot in slots.unwrap_or_default() {
            results.push(match slot {
                Slot::Engine => None,
                Slot::Done(result) => Some(result),
                // A caller that stopped before the last pass left these unread.
                Slot::Active(_) | Slot::Waiting => return Ok(None),
            });
        }
        let hook = Arc::new(StreamedStatements::Results(results));
        evaluate_tran_measurements_directed(netlist, span, Some(&hook), abort).map(Some)
    }
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
    LegacyDelay(Box<LegacyDelayState>),
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
    /// RISE, FALL or CROSS: which of the window's samples are in the step it names.
    crossing: Option<RiseFallCrossTracker>,
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

/// [`delay_clause_event`] of a legacy clause: the trigger's pass, then the target's, which
/// counts only the events after the trigger.
struct LegacyDelayState {
    window: MeasureWindow,
    /// The trigger's event once its pass is over.
    trigger: Option<Value>,
    targ: Clause,
    clause: LegacyClauseState,
}

enum LegacyClauseState {
    /// The first accepted sample at or after the axis value; `None` for one that is not
    /// finite, which no sample is at or after.
    At {
        target: Option<Value>,
        found: Option<Value>,
    },
    When {
        left: Operand,
        right: Operand,
        tracker: DelayConditionTracker,
        /// Every negative occurrence reads as LAST.
        last: bool,
        event: Option<Value>,
    },
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
    /// `earlier` is the earlier result a FIND or DERIV at one compares with.
    fn open(spec: &Spec, axis: &AxisFacts, earlier: Option<Value>) -> Self {
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
                crossing,
            } => {
                let from = axis.delayed_window_start(from, td);
                let (lower, upper) =
                    MeasureEngine::window_bounds(axis.first_step_rises.unwrap_or(true), from, to);
                let crossing = crossing.and_then(|(occurrence, level)| {
                    RiseFallCrossTracker::of(Some(occurrence), level)
                });
                // A step's window takes no ends between samples.
                let ends = if !interpolate_ends || lower > upper || crossing.is_some() {
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
                    crossing,
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
                mut right,
                occurrence,
                window,
            } => {
                if let Some(earlier) = earlier {
                    right = Operand::Constant(earlier);
                }
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
            Kind::LegacyDelay { trig, targ, window } => {
                Self::LegacyDelay(Box::new(LegacyDelayState {
                    window,
                    trigger: None,
                    targ,
                    clause: LegacyClauseState::open(trig, window.minval),
                }))
            }
            Kind::Fixed(ref result) => Self::Done(result.clone()),
            // The engine evaluates it on the run's span; the stream opens no state for it.
            Kind::Span => Self::Done(MeasureResult::failed(name, "read from the run's span")),
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
            Self::LegacyDelay(state) => state.update(x, row),
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

    /// End a pass: the result, or the state for one more pass.
    fn end_pass(self, name: &str) -> Slot {
        Slot::Done(match self {
            Self::LegacyDelay(state) => {
                return match state.end_pass(name) {
                    Ok(result) => Slot::Done(result),
                    Err(state) => Slot::Active(Self::LegacyDelay(state)),
                };
            }
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
                    return Slot::Done(MeasureResult::failed(name, "Trigger condition not found"));
                };
                let Some(target) = targ.event() else {
                    return Slot::Done(MeasureResult::failed(name, "Target condition not found"));
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
        })
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
            // [`RiseFallCrossTracker::select`]: a LAST window restarts at each new step.
            let step = match &mut self.crossing {
                Some(tracker) => tracker.step(y),
                None => Some(false),
            };
            if let Some(restart) = step {
                if restart {
                    self.selected = None;
                }
                self.select(x, y);
            }
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
            return match self.crossing {
                Some(tracker) => MeasureResult::failed(name, &tracker.absent()),
                None => MeasureResult::failed(name, "Empty range"),
            };
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

impl LegacyDelayState {
    #[inline]
    fn update(&mut self, x: Value, row: &[Value]) {
        if !legacy_delay_accepts_sample(
            x,
            self.window.td,
            MeasureWindow {
                td: None,
                ..self.window
            },
        ) {
            return;
        }
        match &mut self.clause {
            LegacyClauseState::At { target, found } => {
                if found.is_none()
                    && let Some(target) = *target
                    && x >= target
                {
                    *found = Some(x);
                }
            }
            LegacyClauseState::When {
                left,
                right,
                tracker,
                last,
                event,
            } => {
                if event.is_some() && !*last {
                    return;
                }
                if let Some(axis) =
                    tracker.update_with_td(x, left.value(x, row), right.value(x, row), None)
                    && self.trigger.is_none_or(|trigger| x > trigger)
                {
                    *event = Some(axis);
                }
            }
        }
    }

    /// End the trigger's pass, which opens the target's, or the target's, which gives the delay.
    fn end_pass(mut self: Box<Self>, name: &str) -> Result<MeasureResult, Box<Self>> {
        let event = match &self.clause {
            LegacyClauseState::At { found, .. } => *found,
            LegacyClauseState::When { event, .. } => *event,
        };
        match (self.trigger, event) {
            (None, None) => Ok(MeasureResult::failed(name, "Trigger condition not found")),
            (None, Some(trigger)) => {
                self.trigger = Some(trigger);
                self.clause = LegacyClauseState::open(self.targ, self.window.minval);
                Err(self)
            }
            (Some(_), None) => Ok(MeasureResult::failed(name, "Target condition not found")),
            (Some(trigger), Some(target)) => Ok(MeasureResult::success(name, target - trigger)),
        }
    }
}

impl LegacyClauseState {
    fn open(clause: Clause, minval: Value) -> Self {
        match clause {
            Clause::At(target) => Self::At {
                target: target.is_finite().then_some(target),
                found: None,
            },
            Clause::When {
                left,
                right,
                occurrence,
                explicit,
                ..
            } => Self::When {
                left,
                right,
                tracker: DelayConditionTracker::new_legacy(
                    occurrence.edge,
                    occurrence.number,
                    explicit,
                    minval,
                ),
                last: occurrence.number < 0,
                event: None,
            },
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
