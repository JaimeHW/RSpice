//! Streamed `.MEASURE TRAN_CONT` evaluation.
//!
//! A transient that writes its rows to a results store measures its continuous statements there
//! instead of reading the run back. The statements are planned against the store's first rows,
//! which resolves each operand through the same signal table, aliases and expressions as the
//! evaluation in memory; a deck whose statements read only the store's own columns (or the
//! axis) is then evaluated over the rows in one pass, after one over the axis for what a window
//! must know of the whole axis before it opens (which way the axis runs, how far it reaches).
//! Each statement's scan does the arithmetic of its in-memory twin in [`super`] in the same
//! order, so the records are the same to the bit, and they are then admitted, failed and
//! budgeted by the same code. A deck with a statement of any other form is measured in memory.

use super::*;
use crate::analysis::measure_signals::with_tran_continuous_signals;
use crate::analysis::transient::TransientResult;
use crate::netlist::Netlist;
use std::collections::VecDeque;

/// Where an operand's value in a row is: the axis, a column the rows carry, or a constant.
#[derive(Debug, Clone, Copy)]
enum Source {
    Axis,
    Column(usize),
    Constant(Value),
}

impl Source {
    /// The operand's value in `row`, which holds the axis and then each column.
    fn at(self, row: &[Value]) -> Value {
        match self {
            Self::Axis => row[0],
            Self::Column(column) => row[column + 1],
            Self::Constant(value) => value,
        }
    }
}

/// What a WHEN-located event records.
#[derive(Debug, Clone, Copy)]
enum EventValue {
    /// WHEN: the event's axis.
    Axis,
    /// FIND: the signal at the event.
    Find(Source),
    /// DERIV: the signal's slope over the step the event is in.
    Slope(Source),
}

/// A WHEN, FIND...WHEN or DERIV...WHEN statement's scan for condition events.
struct EventScan {
    value: EventValue,
    left: Source,
    right: Source,
    window: MeasureWindow,
    analysis: String,
    edge: EdgeType,
    number: isize,
    /// The occurrence the records start at, or for a negative occurrence the one counted from
    /// the end; `None` when no event can be recorded.
    requested: Option<usize>,
    count: usize,
    bounds: (Value, Value),
    /// For a negative occurrence, the last `requested` matching events' records.
    last: VecDeque<ContinuousMeasureRecord>,
    records: Vec<ContinuousMeasureRecord>,
}

/// A FIND or DERIV AT statement's scan for the row the point is in.
struct AtScan {
    derivative: bool,
    signal: Source,
    target: Value,
    window: MeasureWindow,
    analysis: String,
    outcome: Option<Result<Value, &'static str>>,
}

/// One side of a TRIG/TARG statement: an AT point, decided from the axis alone, or a condition
/// whose events are found row by row.
enum DelaySide {
    At {
        target: Value,
        minval: Value,
    },
    When {
        left: Source,
        right: Source,
        tracker: DelayConditionTracker,
        td: Option<Value>,
    },
}

/// A TRIG/TARG statement's scan: its trigger and target events, paired in order.
struct DelayScan {
    sides: [DelaySide; 2],
    /// Each side's events not yet paired.
    pending: [VecDeque<Value>; 2],
    /// Each side's first event.
    first: [Option<Value>; 2],
    records: Vec<ContinuousMeasureRecord>,
}

enum Scan {
    Events(EventScan),
    At(AtScan),
    Delay(DelayScan),
}

/// What the scans must know of the whole axis before they start.
struct AxisFacts {
    count: usize,
    last: Value,
    /// Whether the axis rises at its first step that moves it; `None` when it never moves.
    rises: Option<bool>,
    minimum: Value,
    maximum: Value,
    /// The first non-finite sample's index.
    non_finite: Option<usize>,
}

/// `.MEASURE TRAN_CONT` statements evaluated over a run's rows without the run in memory:
/// [`Self::axis`] over every row's axis value, then [`Self::rows`] over every row of the columns
/// [`Self::columns`] names, both in row order, then [`Self::finish`].
pub struct TranContinuousStream {
    statements: Vec<MeasureStatement>,
    scans: Vec<Scan>,
    columns: Vec<usize>,
    limit: usize,
    axis: AxisFacts,
    started: bool,
    rows: usize,
    previous: Vec<Value>,
    current: Vec<Value>,
}

impl TranContinuousStream {
    /// Plan the deck's `.MEASURE TRAN_CONT` statements against `probe`, a result holding the
    /// run's first rows, whose every column the rows could be read from is among `columns`
    /// (`None` for one that is not), budgeted by `limits` as the evaluation in memory is.
    ///
    /// `None` when the deck has none, or a statement cannot be streamed and the deck is
    /// measured with the run in memory: an operand that is no column the rows carry (an
    /// expression, a differential voltage, an alias), a form the scans do not take (FRAC_MAX, a
    /// negative TRIG/TARG occurrence, a statement the evaluation refuses), or a run with
    /// impulses, whose statements the live impulse route decides.
    pub fn plan(
        netlist: &Netlist,
        probe: &TransientResult,
        columns: &[Option<&[Value]>],
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Option<Self>, SimulationError> {
        let statements = netlist
            .measurements
            .iter()
            .filter(|statement| statement.analysis.eq_ignore_ascii_case("TRAN_CONT"))
            .cloned()
            .collect::<Vec<_>>();
        if statements.is_empty()
            || statements.len() > limits.max_result_values
            || probe.time.is_empty()
            || probe.current_impulses.is_some()
            || probe.voltage_impulses.is_some()
        {
            return Ok(None);
        }
        let time = (probe.time.as_ptr() as usize, probe.time.len());
        let resolved = with_tran_continuous_signals(netlist, probe, limits, abort, |signals| {
            let indexed = index_measure_signals_with_abort(signals, abort)?;
            Ok::<_, SimulationError>(
                statements
                    .iter()
                    .map(|statement| {
                        let mut resolve = |name: &str| {
                            let values = lookup_signal(&indexed, name)?;
                            let address = (values.as_ptr() as usize, values.len());
                            if address == time {
                                return Some(None);
                            }
                            columns
                                .iter()
                                .position(|column| {
                                    column.is_some_and(|column| {
                                        !column.is_empty()
                                            && (column.as_ptr() as usize, column.len()) == address
                                    })
                                })
                                .map(Some)
                        };
                        Scan::plan(statement, &mut resolve)
                    })
                    .collect::<Option<Vec<_>>>(),
            )
        })?
        .transpose()?
        .flatten();
        let Some(scans) = resolved else {
            log::debug!(
                "the .MEASURE TRAN_CONT statements are measured in memory: one reads a value the \
                 rows do not carry or has no streamed form"
            );
            return Ok(None);
        };
        // The caller's columns the scans read, each once, in the order the rows carry them.
        let mut needed: Vec<usize> = Vec::new();
        let mut scans = scans;
        for scan in &mut scans {
            for source in scan.sources_mut() {
                if let Source::Column(column) = *source {
                    let place = needed
                        .iter()
                        .position(|&known| known == column)
                        .unwrap_or_else(|| {
                            needed.push(column);
                            needed.len() - 1
                        });
                    *source = Source::Column(place);
                }
            }
        }
        let width = needed.len() + 1;
        Ok(Some(Self {
            statements,
            scans,
            columns: needed,
            limit: limits.max_result_values,
            axis: AxisFacts {
                count: 0,
                last: 0.0,
                rises: None,
                minimum: Value::INFINITY,
                maximum: Value::NEG_INFINITY,
                non_finite: None,
            },
            started: false,
            rows: 0,
            previous: vec![0.0; width],
            current: vec![0.0; width],
        }))
    }

    /// The caller's columns, by their place in the `columns` [`Self::plan`] was given, that
    /// [`Self::rows`] takes, in the order it takes them.
    pub fn columns(&self) -> &[usize] {
        &self.columns
    }

    /// Read the next rows' axis values.
    pub fn axis(&mut self, x: &[Value]) {
        for &value in x {
            let facts = &mut self.axis;
            if !value.is_finite() && facts.non_finite.is_none() {
                facts.non_finite = Some(facts.count);
            }
            if facts.count > 0 && facts.rises.is_none() && value != facts.last {
                facts.rises = Some(value > facts.last);
            }
            facts.minimum = facts.minimum.min(value);
            facts.maximum = facts.maximum.max(value);
            facts.last = value;
            facts.count += 1;
        }
    }

    /// Scan the next rows: their axis values `x` and, for each of [`Self::columns`], its values.
    pub fn rows(&mut self, x: &[Value], columns: &[&[Value]]) {
        if !self.started {
            self.started = true;
            let rises = self.axis.rises.unwrap_or(true);
            for scan in &mut self.scans {
                scan.start(rises, &self.axis);
            }
        }
        for (offset, &value) in x.iter().enumerate() {
            self.current[0] = value;
            for (slot, column) in self.current[1..].iter_mut().zip(columns) {
                *slot = column.get(offset).copied().unwrap_or(Value::NAN);
            }
            let previous = (self.rows > 0).then_some(self.previous.as_slice());
            for scan in &mut self.scans {
                scan.row(self.rows, previous, &self.current);
            }
            std::mem::swap(&mut self.previous, &mut self.current);
            self.rows += 1;
        }
    }

    /// The statements' results, as [`MeasureEngine::evaluate_continuous_with_limits_and_abort`]
    /// gives them for the whole run.
    pub fn finish(self) -> Result<Vec<ContinuousMeasureResult>, SimulationError> {
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            self.statements.len(),
            self.limit,
        )?;
        let invalid = if self.axis.count == 0 {
            Some("measurement axis is empty".to_owned())
        } else {
            self.axis.non_finite.map(|index| {
                format!("measurement axis contains non-finite sample at index {index}")
            })
        };
        let mut budget = RecordBudget {
            retained: 0,
            limit: self.limit,
        };
        let mut results = Vec::new();
        for (statement, scan) in self.statements.iter().zip(self.scans) {
            let before = budget.retained;
            let evaluated = match &invalid {
                Some(error) => Err(EvaluationError::Measurement(error.clone())),
                None => scan.finish(statement, &mut budget),
            };
            retain(statement, evaluated, before, &mut budget, &mut results)?;
        }
        Ok(results)
    }
}

impl Scan {
    /// `statement`'s scan, its waveform operands resolved by `resolve` to the axis (`Some(None)`)
    /// or a caller's column; `None` when it has no streamed form.
    fn plan(
        statement: &MeasureStatement,
        resolve: &mut dyn FnMut(&str) -> Option<Option<usize>>,
    ) -> Option<Self> {
        if statement.name.trim().is_empty() {
            return None;
        }
        let mut waveform =
            |name: &str| resolve(name).map(|column| column.map_or(Source::Axis, Source::Column));
        let mut operand = |operand: &MeasureOperand| match operand {
            MeasureOperand::Constant(value) => Some(Source::Constant(*value)),
            MeasureOperand::Waveform(name) => waveform(name),
            MeasureOperand::Result(_) => None,
        };
        let analysis = statement.analysis.clone();
        let events =
            |value: EventValue,
             condition: &WhenCondition,
             window: MeasureWindow,
             operand: &mut dyn FnMut(&MeasureOperand) -> Option<Source>| {
                let number = condition.occurrence.number;
                let requested = if !window.minval.is_finite() || window.minval < 0.0 {
                    None
                } else if number < 0 {
                    number.checked_abs().map(|number| number as usize)
                } else {
                    Some(number.max(1) as usize)
                };
                Some(Self::Events(EventScan {
                    value,
                    left: operand(&MeasureOperand::Waveform(condition.left.clone()))?,
                    right: operand(&condition.right)?,
                    window,
                    analysis: analysis.clone(),
                    edge: condition.occurrence.edge,
                    number,
                    requested,
                    count: 0,
                    bounds: (Value::NEG_INFINITY, Value::INFINITY),
                    last: VecDeque::new(),
                    records: Vec::new(),
                }))
            };
        match &statement.measure_type {
            MeasureType::When {
                condition,
                from,
                to,
                td,
                minval,
            } => {
                let window = MeasureWindow {
                    from: *from,
                    to: *to,
                    td: *td,
                    minval: *minval,
                };
                events(EventValue::Axis, condition, window, &mut operand)
            }
            MeasureType::Find {
                signal,
                at,
                when,
                from,
                to,
                td,
                minval,
            }
            | MeasureType::Derivative {
                signal,
                at,
                when,
                from,
                to,
                td,
                minval,
            } => {
                let derivative = matches!(statement.measure_type, MeasureType::Derivative { .. });
                let signal = operand(&MeasureOperand::Waveform(signal.clone()))?;
                let window = MeasureWindow {
                    from: *from,
                    to: *to,
                    td: *td,
                    minval: *minval,
                };
                match (at, when) {
                    (Some(target), _) => Some(Self::At(AtScan {
                        derivative,
                        signal,
                        target: *target,
                        window,
                        analysis: analysis.clone(),
                        outcome: None,
                    })),
                    (None, Some(condition)) => {
                        let value = if derivative {
                            EventValue::Slope(signal)
                        } else {
                            EventValue::Find(signal)
                        };
                        events(value, condition, window, &mut operand)
                    }
                    (None, None) => None,
                }
            }
            MeasureType::Delay {
                trig, targ, minval, ..
            } => {
                if trig.frac_max.is_some() || targ.frac_max.is_some() {
                    return None;
                }
                let mut side = |clause: &TrigSpec, td: Option<Value>| match &clause.event {
                    TriggerEvent::At(target) => Some(DelaySide::At {
                        target: *target,
                        minval: *minval,
                    }),
                    TriggerEvent::When(condition) => {
                        if condition.occurrence.number < 0 {
                            return None;
                        }
                        Some(DelaySide::When {
                            left: operand(&MeasureOperand::Waveform(condition.left.clone()))?,
                            right: operand(&condition.right)?,
                            tracker: DelayConditionTracker::new_continuous(
                                condition.occurrence.edge,
                                condition.occurrence.number,
                                clause.occurrence_explicit,
                                *minval,
                            ),
                            td,
                        })
                    }
                };
                let trigger = side(trig, trig.td)?;
                let target = side(targ, targ.td.or(trig.td))?;
                Some(Self::Delay(DelayScan {
                    sides: [trigger, target],
                    pending: [VecDeque::new(), VecDeque::new()],
                    first: [None, None],
                    records: Vec::new(),
                }))
            }
            _ => None,
        }
    }

    fn sources_mut(&mut self) -> Vec<&mut Source> {
        match self {
            Self::Events(scan) => {
                let mut sources = vec![&mut scan.left, &mut scan.right];
                if let EventValue::Find(signal) | EventValue::Slope(signal) = &mut scan.value {
                    sources.push(signal);
                }
                sources
            }
            Self::At(scan) => vec![&mut scan.signal],
            Self::Delay(scan) => scan
                .sides
                .iter_mut()
                .flat_map(|side| match side {
                    DelaySide::At { .. } => Vec::new(),
                    DelaySide::When { left, right, .. } => vec![left, right],
                })
                .collect(),
        }
    }

    /// Open the scan once the whole axis is known: its window, and an AT point it decides from
    /// the axis alone.
    fn start(&mut self, rises: bool, axis: &AxisFacts) {
        match self {
            Self::Events(scan) => scan.bounds = bounds_on(rises, &scan.analysis, scan.window),
            Self::At(scan) => {
                let (lower, upper) = bounds_on(rises, &scan.analysis, scan.window);
                if !MeasureEngine::axis_in_measurement_window_with_minval(
                    scan.target,
                    lower,
                    upper,
                    scan.window.minval,
                ) {
                    scan.outcome = Some(Err("AT point is outside the measurement window"));
                }
            }
            Self::Delay(scan) => {
                for place in 0..2 {
                    // Subtracting `minval` keeps the order of the samples, so the extreme sample
                    // decides what any sample would.
                    if let DelaySide::At { target, minval } = scan.sides[place]
                        && target.is_finite()
                        && target >= axis.minimum
                        && target <= axis.maximum
                        && if rises {
                            axis.maximum - minval >= target
                        } else {
                            axis.minimum - minval <= target
                        }
                    {
                        scan.event(place, target);
                    }
                }
            }
        }
    }

    /// Scan row `row`, `current`, after `previous`, the row before it (none for the first).
    fn row(&mut self, row: usize, previous: Option<&[Value]>, current: &[Value]) {
        match self {
            Self::Events(scan) => {
                if let Some(previous) = previous {
                    scan.step(previous, current);
                }
            }
            Self::At(scan) => {
                if scan.outcome.is_some() || (scan.derivative && row == 0) {
                    return;
                }
                let previous = previous.map(|previous| (previous[0], scan.signal.at(previous)));
                let current = (current[0], scan.signal.at(current));
                let value = if scan.derivative {
                    Ok(accepted_row_at_match(
                        previous.map(|previous| previous.0),
                        current.0,
                        scan.target,
                        scan.window.minval,
                    )
                    .map(|_| {
                        let previous = previous.unwrap_or(current);
                        accepted_row_secant_slope(previous.0, previous.1, current.0, current.1)
                    }))
                } else {
                    find_at_accepted_row(previous, current, scan.target, scan.window.minval)
                };
                match value {
                    Ok(None) => {}
                    Ok(Some(value)) => scan.outcome = Some(Ok(value)),
                    Err(error) => scan.outcome = Some(Err(error)),
                }
            }
            Self::Delay(scan) => {
                for place in 0..2 {
                    if let DelaySide::When {
                        left,
                        right,
                        tracker,
                        td,
                    } = &mut scan.sides[place]
                        && let Some(event) = tracker.update_with_td(
                            current[0],
                            left.at(current),
                            right.at(current),
                            *td,
                        )
                    {
                        scan.event(place, event);
                    }
                }
            }
        }
    }

    /// The statement's result, its records admitted to `budget` in order as the evaluation in
    /// memory admits them.
    fn finish(
        self,
        statement: &MeasureStatement,
        budget: &mut RecordBudget,
    ) -> Result<ContinuousMeasureResult, EvaluationError> {
        let mut records = Vec::new();
        let found = match self {
            Self::Events(mut scan) => {
                if scan.number < 0
                    && scan.requested == Some(scan.last.len())
                    && let Some(record) = scan.last.pop_front()
                {
                    scan.records.push(record);
                }
                scan.records
            }
            Self::At(scan) => {
                let value = match scan.outcome {
                    None => return Err("Time point not in simulation range".into()),
                    Some(outcome) => outcome?,
                };
                vec![ContinuousMeasureRecord::point(value, scan.target)]
            }
            Self::Delay(scan) => {
                if scan.records.is_empty() {
                    return Ok(ContinuousMeasureResult::failed_delay(
                        &statement.name,
                        scan.first[0],
                        scan.first[1],
                        "trigger/target event pair not found",
                    ));
                }
                scan.records
            }
        };
        for record in found {
            budget.push(&mut records, record, statement.fail_value)?;
        }
        if records.is_empty() {
            return Err(
                if matches!(statement.measure_type, MeasureType::Derivative { .. }) {
                    "WHEN condition never met in the measurement window"
                } else {
                    "WHEN condition not found in the measurement window"
                }
                .into(),
            );
        }
        Ok(ContinuousMeasureResult {
            name: statement.name.clone(),
            records,
            failure: None,
            failure_metadata: None,
        })
    }
}

impl EventScan {
    /// The condition's crossing over the step from `previous` to `current`, as
    /// `visit_condition_events` finds and counts it.
    fn step(&mut self, previous: &[Value], current: &[Value]) {
        let Some(requested) = self.requested else {
            return;
        };
        let minval = self.window.minval;
        let Some(crossing) = measure_condition_crossing(
            self.left.at(previous),
            self.left.at(current),
            self.right.at(previous),
            self.right.at(current),
            minval,
        ) else {
            return;
        };
        let event_axis = previous[0] + crossing.fraction * (current[0] - previous[0]);
        if !point_event_axis_in_window(event_axis, self.bounds.0, self.bounds.1, minval) {
            return;
        }
        let matches = edge_matches_measure_condition(self.edge, crossing.direction);
        let record = |value: EventValue| {
            let value = match value {
                EventValue::Axis => event_axis,
                EventValue::Find(signal) if crossing.current_within_minval => signal.at(current),
                EventValue::Find(signal) => interpolate_extended_real(
                    signal.at(previous),
                    signal.at(current),
                    crossing.fraction,
                ),
                EventValue::Slope(signal) => accepted_row_secant_slope(
                    previous[0],
                    signal.at(previous),
                    current[0],
                    signal.at(current),
                ),
            };
            ContinuousMeasureRecord::point(value, event_axis)
        };
        if self.number < 0 {
            // Counted from the end: the requested event is the oldest of the last ones that
            // match, once the run has ended.
            if matches {
                self.last.push_back(record(self.value));
                if self.last.len() > requested {
                    self.last.pop_front();
                }
            }
            return;
        }
        self.count += match self.edge {
            EdgeType::Cross => 1,
            EdgeType::Rise => usize::from(crossing.direction == MeasureConditionDirection::Rise),
            EdgeType::Fall => usize::from(crossing.direction != MeasureConditionDirection::Rise),
        };
        if self.count >= requested && matches {
            self.records.push(record(self.value));
        }
    }
}

impl DelayScan {
    /// Side `place`'s next event, paired with the other side's next one when it has one.
    fn event(&mut self, place: usize, axis: Value) {
        self.first[place].get_or_insert(axis);
        self.pending[place].push_back(axis);
        while !self.pending[0].is_empty() && !self.pending[1].is_empty() {
            if let (Some(trigger), Some(target)) =
                (self.pending[0].pop_front(), self.pending[1].pop_front())
            {
                self.records
                    .push(ContinuousMeasureRecord::delay(trigger, target));
            }
        }
    }
}
