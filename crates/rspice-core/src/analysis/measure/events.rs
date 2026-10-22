//! `.MEASURE TRAN` on event operands.
//!
//! `D(n)` reads the digital event trace of net `n`, or the word of a declared
//! bus `n` as an unsigned integer, declared MSB first; `E(n)` reads a real
//! event trace. Both are right-continuous step functions on the instants the
//! run accepted their events at. Nothing is resampled onto the analog grid
//! and nothing is interpolated: a crossing is dated at the step that makes it,
//! and an integral is a sum of rectangles.
//!
//! X and Z are undefined values, not numbers, and a bus with any X or Z bit is
//! undefined. A crossing looks through an undefined stretch: it compares the
//! last known value before the stretch with the first known value after it,
//! and is dated at the step that leaves it, so 0→X→1 is one rise at the X→1
//! step and 0→X→0 is none. An HDL counts 0→X as a posedge (IEEE 1364-2005
//! §9.7.2); a measurement counts only a move to a known value, because it asks
//! when a net became 1. A value read inside an undefined stretch, and an
//! aggregate over a window that holds one, fails and names it.

use std::collections::HashMap;
use std::sync::Arc;

use super::{
    EdgeType, MeasureData, MeasureEngine, MeasureOperand, MeasureResult, MeasureStatement,
    MeasureType, MeasureWindow, TrigSpec, TriggerEvent, WhenCondition, delay_at_is_reached,
    delay_clause_event, delay_td_accepts_modern_event, find_at_accepted_points,
    first_measure_condition_event, lookup_signal, measure_condition_crossing,
    point_event_axis_in_window, select_measure_condition_occurrence,
};
use crate::Value;
use crate::analysis::transient::TransientResult;
use crate::netlist::canonical_symbol;
use crate::netlist::measure::ExtremaOutput;
use crate::xspice::{DigitalState, DigitalValue};

/// The widest bus whose word an `f64` holds exactly.
const MAX_EXACT_BUS_BITS: usize = 53;

/// Which event domain an operand reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Domain {
    /// `D(..)`: a digital net or a declared bus.
    Digital,
    /// `E(..)`: a real event net.
    Real,
}

/// The domain and the net or bus name of an operand spelled `D(name)` or
/// `E(name)`, or `None` for any other spelling.
fn event_operand(text: &str) -> Option<(Domain, &str)> {
    let text = text.trim();
    let domain = match text.as_bytes().first()? {
        b'D' | b'd' => Domain::Digital,
        b'E' | b'e' => Domain::Real,
        _ => return None,
    };
    let inner = text[1..]
        .trim_start()
        .strip_prefix('(')?
        .strip_suffix(')')?
        .trim();
    let simple = !inner.is_empty()
        && !inner.contains(|character: char| {
            character.is_whitespace() || "(),'\"{}".contains(character)
        });
    simple.then_some((domain, inner))
}

fn operand_key(domain: Domain, name: &str) -> String {
    let accessor = match domain {
        Domain::Digital => 'D',
        Domain::Real => 'E',
    };
    format!("{accessor}({})", canonical_symbol(name))
}

fn is_transient(statement: &MeasureStatement) -> bool {
    statement.analysis.eq_ignore_ascii_case("TRAN")
}

/// Whether `statement` reads solved points, which carry no event values.
fn reads_solved_points(statement: &MeasureStatement) -> bool {
    !statement.analysis.to_ascii_uppercase().starts_with("TRAN")
}

/// Every waveform operand a statement names, in card order.
fn waveform_operands(statement: &MeasureStatement) -> Vec<&str> {
    fn condition<'a>(condition: &'a WhenCondition, operands: &mut Vec<&'a str>) {
        operands.push(&condition.left);
        if let MeasureOperand::Waveform(right) = &condition.right {
            operands.push(right);
        }
    }
    let mut operands = Vec::new();
    match &statement.measure_type {
        MeasureType::Delay { trig, targ, .. } => {
            for clause in [trig, targ] {
                if let TriggerEvent::When(when) = &clause.event {
                    condition(when, &mut operands);
                }
            }
        }
        MeasureType::Find { signal, when, .. } | MeasureType::Derivative { signal, when, .. } => {
            operands.push(signal);
            if let Some(when) = when {
                condition(when, &mut operands);
            }
        }
        MeasureType::When {
            condition: when, ..
        } => condition(when, &mut operands),
        MeasureType::ErrorFunction {
            measured,
            comparison,
            ..
        } => {
            operands.push(measured);
            operands.push(comparison);
        }
        MeasureType::FileError { signal, .. }
        | MeasureType::Min { signal, .. }
        | MeasureType::Max { signal, .. }
        | MeasureType::PeakToPeak { signal, .. }
        | MeasureType::Avg { signal, .. }
        | MeasureType::Rms { signal, .. }
        | MeasureType::RiseTime { signal, .. }
        | MeasureType::FallTime { signal, .. }
        | MeasureType::Integ { signal, .. } => operands.push(signal),
        MeasureType::Param { .. } | MeasureType::Equation { .. } => {}
    }
    operands
}

//=============================================================================
// Step functions
//=============================================================================

/// One value of a step function.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Level {
    Known(Value),
    /// X (`'x'`) or Z (`'z'`).
    Undefined(char),
}

fn digital_level(value: DigitalValue) -> Level {
    match value.state {
        DigitalState::Zero | DigitalState::ZeroR | DigitalState::ZeroZ => Level::Known(0.0),
        DigitalState::One | DigitalState::OneR | DigitalState::OneZ => Level::Known(1.0),
        DigitalState::Unknown | DigitalState::UnknownR | DigitalState::UnknownZ => {
            Level::Undefined('x')
        }
        DigitalState::HighZ => Level::Undefined('z'),
    }
}

/// A right-continuous step function over the analysis span.
///
/// `levels[i]` holds on `[instants[i], instants[i + 1])`, and the last level
/// holds through `end`. `instants[0]` is the start of the analysis; a trace
/// that states no value until later is undefined (X) until it does.
#[derive(Debug)]
pub(crate) struct Steps {
    instants: Vec<Value>,
    levels: Vec<Level>,
    end: Value,
}

impl Steps {
    /// The step function of time-ordered `(instant, level)` points, over
    /// `[start, end]`.
    ///
    /// A run settles one value per instant, so a second point at the same
    /// instant replaces the first; a point that repeats the level is no step.
    fn new(points: impl IntoIterator<Item = (Value, Level)>, start: Value, end: Value) -> Self {
        let mut instants = vec![start];
        let mut levels = vec![Level::Undefined('x')];
        for (time, level) in points {
            if time <= start {
                levels[0] = level;
                continue;
            }
            if time > end {
                break;
            }
            if instants.last() == Some(&time) {
                *levels.last_mut().expect("one level per instant") = level;
            } else {
                instants.push(time);
                levels.push(level);
            }
        }
        let mut steps = Self {
            instants: Vec::with_capacity(levels.len()),
            levels: Vec::with_capacity(levels.len()),
            end,
        };
        for (instant, level) in instants.into_iter().zip(levels) {
            if steps.levels.last() != Some(&level) {
                steps.instants.push(instant);
                steps.levels.push(level);
            }
        }
        steps
    }

    fn start(&self) -> Value {
        self.instants[0]
    }

    /// The level at `time`: at a step instant, the new one.
    fn level_at(&self, time: Value) -> Level {
        let index = self
            .instants
            .partition_point(|instant| *instant <= time)
            .saturating_sub(1);
        self.levels[index]
    }

    /// Each step's level with the part of `[lower, upper)` it holds over, for
    /// the steps that hold over a nonempty part.
    fn pieces(&self, lower: Value, upper: Value) -> impl Iterator<Item = (Value, Value, Level)> {
        (0..self.levels.len()).filter_map(move |index| {
            let next = self.instants.get(index + 1).copied().unwrap_or(self.end);
            let from = self.instants[index].max(lower);
            let to = next.min(upper);
            (to > from).then_some((from, to, self.levels[index]))
        })
    }
}

/// The word of a bus whose members are `members`, declared MSB first.
fn bus_steps(members: &[Steps], start: Value, end: Value) -> Steps {
    let mut instants = members
        .iter()
        .flat_map(|member| member.instants.iter().copied())
        .collect::<Vec<_>>();
    instants.sort_by(Value::total_cmp);
    instants.dedup();
    let words = instants.into_iter().map(|instant| {
        let mut word = 0u64;
        let mut undefined = None;
        for member in members {
            word <<= 1;
            match member.level_at(instant) {
                Level::Known(bit) => word |= u64::from(bit != 0.0),
                // One X bit makes the word X; Z is named only when no bit is X.
                Level::Undefined(state) => {
                    undefined = Some(if undefined == Some('x') { 'x' } else { state });
                }
            }
        }
        // At most 53 bits, so the conversion is exact.
        let level = undefined.map_or(Level::Known(word as Value), Level::Undefined);
        (instant, level)
    });
    Steps::new(words, start, end)
}

//=============================================================================
// The operand table
//=============================================================================

/// The event operands a run's `.MEASURE TRAN` cards name, each resolved to
/// its step function or to the sentence that refuses it.
///
/// Built from the statements, so a run whose cards name no `D()` or `E()`
/// operand builds nothing.
#[derive(Debug)]
pub(crate) struct EventOperands {
    series: HashMap<String, Result<Steps, String>>,
}

impl EventOperands {
    /// The table for `statements` over `result`, or `None` when no
    /// transient statement names an event operand.
    pub(crate) fn new(
        statements: &[&MeasureStatement],
        result: &TransientResult,
    ) -> Option<Arc<Self>> {
        let (&start, &end) = result.time.first().zip(result.time.last())?;
        let mut series = HashMap::new();
        for statement in statements
            .iter()
            .filter(|statement| is_transient(statement))
        {
            for text in waveform_operands(statement) {
                let Some((domain, name)) = event_operand(text) else {
                    continue;
                };
                series
                    .entry(operand_key(domain, name))
                    .or_insert_with(|| resolve(domain, name, text.trim(), result, start, end));
            }
        }
        (!series.is_empty()).then(|| Arc::new(Self { series }))
    }

    fn get(&self, text: &str) -> Option<&Result<Steps, String>> {
        let (domain, name) = event_operand(text)?;
        self.series.get(&operand_key(domain, name))
    }
}

fn resolve(
    domain: Domain,
    name: &str,
    text: &str,
    result: &TransientResult,
    start: Value,
    end: Value,
) -> Result<Steps, String> {
    let wanted = canonical_symbol(name);
    let digital = |node: &str| {
        result
            .digital_traces
            .iter()
            .find(|trace| canonical_symbol(&trace.node_name) == canonical_symbol(node))
    };
    let real = result
        .real_traces
        .iter()
        .find(|trace| canonical_symbol(&trace.node_name) == wanted);
    match domain {
        Domain::Digital => {
            if let Some(trace) = digital(name) {
                let points = trace
                    .points
                    .iter()
                    .map(|point| (point.time, digital_level(point.value)));
                return Ok(Steps::new(points, start, end));
            }
            if let Some(bus) = result
                .digital_buses
                .iter()
                .find(|bus| canonical_symbol(&bus.name) == wanted)
            {
                if bus.members.len() > MAX_EXACT_BUS_BITS {
                    return Err(format!(
                        "{text} is a {}-bit bus; a measured value is a double, which holds a word \
                         of at most {MAX_EXACT_BUS_BITS} bits exactly",
                        bus.members.len()
                    ));
                }
                let members = bus
                    .members
                    .iter()
                    .map(|member| {
                        let trace = digital(member).ok_or_else(|| {
                            format!(
                                "{text} is a bus whose member '{member}' has no digital event \
                                 trace"
                            )
                        })?;
                        let points = trace
                            .points
                            .iter()
                            .map(|point| (point.time, digital_level(point.value)));
                        Ok(Steps::new(points, start, end))
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                return Ok(bus_steps(&members, start, end));
            }
            if real.is_some() {
                return Err(format!(
                    "{text} names '{name}', a real event net: its values are E({name})"
                ));
            }
        }
        Domain::Real => {
            if let Some(trace) = real {
                let points = trace
                    .points
                    .iter()
                    .map(|point| (point.time, Level::Known(point.value)));
                return Ok(Steps::new(points, start, end));
            }
            if digital(name).is_some() {
                return Err(format!(
                    "{text} names '{name}', a digital event net: its values are D({name})"
                ));
            }
        }
    }
    if let Some(refusal) = private_endpoint_refusal(name, result) {
        return Err(refusal);
    }
    let analog = result
        .node_index_named(name)
        .filter(|node| *node > 0)
        .and_then(|node| result.voltages.get(node - 1))
        .is_some_and(|waveform| !waveform.is_empty());
    Err(if analog {
        format!(
            "{text} names '{name}', an analog net with no event trace: its values are V({name})"
        )
    } else {
        match domain {
            Domain::Digital => {
                format!("{text} names no digital event net or bus that this run recorded")
            }
            Domain::Real => format!("{text} names no real event net that this run recorded"),
        }
    })
}

/// The refusal for a converter's private event endpoint (`x1.q__event_0`),
/// when `name` is one this run kept no column or trace for.
///
/// The builder names the digital side of each converted port bit
/// `<instance>.<port>__event_<bit>`. Its bit is the deck net's own event
/// trace, so the run records the endpoint only when `.ESAVE` lists it, and a
/// card that names it is sent to the deck net, which carries both the bit
/// (`D()`) and the converted voltage (`V()`).
pub(crate) fn private_endpoint_refusal(name: &str, result: &TransientResult) -> Option<String> {
    let name = name.trim();
    let marker = name.to_ascii_lowercase().rfind("__event")?;
    let (path, suffix) = (&name[..marker], &name[marker + "__event".len()..]);
    let bit = suffix.strip_prefix('_').unwrap_or(suffix);
    if !bit.chars().all(|character| character.is_ascii_digit()) {
        return None;
    }
    let (instance, port) = path.rsplit_once(['.', ':'])?;
    let node = result.node_index_named(name).filter(|node| *node > 0)?;
    if result
        .voltages
        .get(node - 1)
        .is_some_and(|waveform| !waveform.is_empty())
        || result.digital_trace_named(name).is_some()
        || result.real_trace_named(name).is_some()
    {
        return None;
    }
    let bit = if bit.is_empty() {
        String::new()
    } else {
        format!(", bit {bit}")
    };
    Some(format!(
        "'{name}' is the private event endpoint of instance {instance}'s converter on port \
         {port}{bit}, which the run records only when .ESAVE lists it. The deck net on port \
         {port} of {instance} carries the same bit: measure it as D() for the bit or V() for \
         the converted voltage."
    ))
}

/// The refusal for a transient `EQN` or `PARAM` statement whose expression
/// reads an event net of `result` through `D()` or `E()`.
///
/// An expression is evaluated on the sampled points of the analysis, and an
/// event net has no sampled column there; the step function belongs on a card
/// of its own, whose result the expression may then name.
pub(crate) fn expression_refusal(
    statement: &MeasureStatement,
    result: &TransientResult,
) -> Option<String> {
    let (measure, expression) = match &statement.measure_type {
        MeasureType::Equation { expression, .. } => ("EQN", expression),
        MeasureType::Param { expression } => ("PARAM", expression),
        _ => return None,
    };
    if !is_transient(statement) {
        return None;
    }
    let text = expression.text.trim();
    let body = text
        .strip_prefix('{')
        .and_then(|body| body.strip_suffix('}'))
        .or_else(|| {
            text.strip_prefix('\'')
                .and_then(|body| body.strip_suffix('\''))
        })
        .unwrap_or(text);
    let parsed = crate::netlist::expr::parse_expression(body).ok()?;
    let operand = expression_event_operand(&parsed, result)?;
    Some(format!(
        "{measure} cannot read {operand}, an event step function: an expression is evaluated \
         on the sampled points of the analysis, where an event net has no column. Measure \
         {operand} with a card of its own and name that result in the expression."
    ))
}

/// The first `D(name)` or `E(name)` call in `expression` that names an event
/// net or bus of `result`.
fn expression_event_operand(
    expression: &crate::netlist::expr::Expr,
    result: &TransientResult,
) -> Option<String> {
    use crate::netlist::expr::Expr;
    match expression {
        Expr::FnCall { name, args } => {
            if let [Expr::Param(net) | Expr::StringLiteral(net)] = args.as_slice() {
                let wanted = canonical_symbol(net);
                let digital = name.eq_ignore_ascii_case("D")
                    && (result
                        .digital_traces
                        .iter()
                        .any(|trace| canonical_symbol(&trace.node_name) == wanted)
                        || result
                            .digital_buses
                            .iter()
                            .any(|bus| canonical_symbol(&bus.name) == wanted));
                let real = name.eq_ignore_ascii_case("E")
                    && result
                        .real_traces
                        .iter()
                        .any(|trace| canonical_symbol(&trace.node_name) == wanted);
                if digital || real {
                    return Some(format!("{name}({net})"));
                }
            }
            args.iter()
                .find_map(|argument| expression_event_operand(argument, result))
        }
        Expr::UnaryOp { operand, .. } => expression_event_operand(operand, result),
        Expr::BinOp { left, right, .. } => expression_event_operand(left, result)
            .or_else(|| expression_event_operand(right, result)),
        Expr::Number(_) | Expr::ComplexNumber(_) | Expr::StringLiteral(_) | Expr::Param(_) => None,
    }
}

/// The refusal for an event operand on a solved-point analysis.
fn solved_point_refusal(operand: &str, analysis: &str) -> String {
    format!(
        "{operand} reads an event trace, and a .MEASURE {} point has none: a solved point \
         publishes one value per analog unknown. Event traces are measured by .MEASURE TRAN.",
        analysis.to_ascii_uppercase()
    )
}

//=============================================================================
// Evaluation
//=============================================================================

/// Evaluate `statement` in the event domain, or return `None` to leave it to
/// the sampled-waveform evaluator.
///
/// A statement is evaluated here when it names an event operand that the
/// signal table does not also hold. A transient run resolves those through
/// `events`; any other analysis refuses them, since a solved point has no
/// event values. A statement whose operands are all sampled waveforms, or
/// that has no event table to read (a remeasured file), stays with the
/// sampled evaluator, which reads an exported `D(n)` column like any other.
///
/// A statement may mix the two domains: each `TRIG`/`TARG` clause, and a
/// `FIND` operand and its `WHEN` condition, resolve in their own domain, the
/// event side exactly and the sampled side interpolated as it always is.
pub(super) fn evaluate(
    statement: &MeasureStatement,
    data: MeasureData<'_, '_>,
    events: Option<&EventOperands>,
    legacy: bool,
) -> Option<MeasureResult> {
    let operands = waveform_operands(statement);
    let unsampled =
        |text: &str| event_operand(text).is_some() && lookup_signal(data.signals, text).is_none();
    if reads_solved_points(statement) {
        let operand = operands.iter().copied().find(|text| unsampled(text))?;
        return Some(MeasureResult::failed(
            &statement.name,
            &solved_point_refusal(operand.trim(), &statement.analysis),
        ));
    }
    if !is_transient(statement) {
        return None;
    }
    let events = events?;
    let mut any_event = false;
    for &text in &operands {
        match events.get(text) {
            Some(Ok(_)) => any_event = true,
            Some(Err(refusal)) if unsampled(text) => {
                return Some(MeasureResult::failed(&statement.name, refusal));
            }
            // A sampled waveform.
            _ => {}
        }
    }
    if !any_event {
        return None;
    }
    let evaluation = Evaluation {
        statement,
        data,
        events,
    };
    evaluation.evaluate(legacy)
}

/// One statement with at least one event operand.
struct Evaluation<'a, 'd, 'b> {
    statement: &'a MeasureStatement,
    data: MeasureData<'d, 'b>,
    events: &'a EventOperands,
}

/// One known-to-known change of a condition's left operand, dated at the
/// step that makes it, with both operands' last known values before it.
#[derive(Clone, Copy)]
struct Change {
    at: Value,
    left_previous: Value,
    left: Value,
    right_previous: Value,
    right: Value,
}

impl Evaluation<'_, '_, '_> {
    fn name(&self) -> &str {
        &self.statement.name
    }

    fn failed(&self, error: &str) -> MeasureResult {
        MeasureResult::failed(self.name(), error)
    }

    fn steps(&self, text: &str) -> Result<&Steps, String> {
        match self.events.get(text) {
            Some(Ok(steps)) => Ok(steps),
            Some(Err(refusal)) => Err(refusal.clone()),
            None => Err(format!("{} is not an event operand", text.trim())),
        }
    }

    /// The value `text` reads at `time`.
    fn value_at(&self, text: &str, time: Value) -> Result<Value, String> {
        match self.steps(text)?.level_at(time) {
            Level::Known(value) => Ok(value),
            Level::Undefined(state) => Err(format!("{} is {state} at {time:e} s", text.trim())),
        }
    }

    /// Whether `text` reads an event trace rather than a sampled waveform.
    fn is_event(&self, text: &str) -> bool {
        matches!(self.events.get(text), Some(Ok(_)))
    }

    /// The first event operand the statement names.
    fn first_event_operand(&self) -> &str {
        waveform_operands(self.statement)
            .into_iter()
            .find(|text| self.is_event(text))
            .map_or("", str::trim)
    }

    /// The refusal for a measure type that has no meaning on a step function.
    fn shapeless(&self, measure: &str, reason: &str) -> MeasureResult {
        self.failed(&format!(
            "{measure} cannot read {}, an event step function: {reason}",
            self.first_event_operand()
        ))
    }

    /// The refusal for a condition that compares a sampled waveform with an
    /// event step function, or `None` when both sides are in one domain.
    fn mixed_condition(&self, condition: &WhenCondition) -> Option<String> {
        let MeasureOperand::Waveform(right) = &condition.right else {
            return None;
        };
        (self.is_event(&condition.left) != self.is_event(right)).then(|| {
            format!(
                "{} = {} compares a sampled waveform with an event step function, and no rule \
                 dates a crossing between a slope and a step. Compare each with a constant \
                 value, and take the delay between the two crossings with TRIG/TARG.",
                condition.left.trim(),
                right.trim()
            )
        })
    }

    fn evaluate(&self, legacy: bool) -> Option<MeasureResult> {
        Some(match &self.statement.measure_type {
            MeasureType::Delay {
                trig,
                targ,
                from,
                to,
                minval,
            } => self.delay(trig, targ, (*from, *to, *minval), legacy),
            MeasureType::When {
                condition,
                from,
                to,
                td,
                minval,
            } => {
                let (lower, upper) = self.point_window(*from, *to, *td);
                match self.when_instant(condition, lower, upper, *minval) {
                    Ok(Some(instant)) => {
                        MeasureResult::success(self.name(), instant).with_event_axis(instant)
                    }
                    Ok(None) => self.failed("WHEN condition not found in the measurement window"),
                    Err(error) => self.failed(&error),
                }
            }
            MeasureType::Find {
                signal,
                at,
                when,
                from,
                to,
                td,
                minval,
            } => self.find(signal, *at, when.as_ref(), (*from, *to, *td), *minval),
            MeasureType::Avg { signal, from, to } => {
                self.aggregate(signal, *from, *to, Aggregate::Avg)
            }
            MeasureType::Rms { signal, from, to } => {
                self.aggregate(signal, *from, *to, Aggregate::Rms)
            }
            MeasureType::Integ { signal, from, to } => {
                self.aggregate(signal, *from, *to, Aggregate::Integ)
            }
            MeasureType::Min {
                signal,
                from,
                to,
                output,
            } => self.extremum(signal, *from, *to, *output, false),
            MeasureType::Max {
                signal,
                from,
                to,
                output,
            } => self.extremum(signal, *from, *to, *output, true),
            MeasureType::PeakToPeak { signal, from, to } => self.peak_to_peak(signal, *from, *to),
            MeasureType::Derivative { .. } => self.shapeless("DERIV", "a step has no derivative"),
            MeasureType::RiseTime { .. } => {
                self.shapeless("RISE_TIME", "a step has no edge to time between two levels")
            }
            MeasureType::FallTime { .. } => {
                self.shapeless("FALL_TIME", "a step has no edge to time between two levels")
            }
            MeasureType::ErrorFunction { .. } | MeasureType::FileError { .. } => self.shapeless(
                "ERR",
                "the error compares sampled values point by point, and a step function has no \
                 samples",
            ),
            MeasureType::Param { .. } | MeasureType::Equation { .. } => return None,
        })
    }

    fn point_window(
        &self,
        from: Option<Value>,
        to: Option<Value>,
        td: Option<Value>,
    ) -> (Value, Value) {
        MeasureEngine::point_measurement_window_bounds(
            self.data.axis,
            &self.statement.analysis,
            from,
            to,
            td,
        )
    }

    /// Every known-to-known change of `condition`'s left operand.
    ///
    /// Both operands are step functions or a constant, so the condition can
    /// change only at their step instants. An instant at which either side is
    /// undefined is skipped: the next known instant compares with the last
    /// known one.
    fn changes(&self, condition: &WhenCondition) -> Result<Vec<Change>, String> {
        let left = self.steps(&condition.left)?;
        let (right, constant) = match &condition.right {
            MeasureOperand::Constant(value) => (None, *value),
            MeasureOperand::Waveform(text) => (Some(self.steps(text)?), Value::NAN),
        };
        let mut instants = left.instants.clone();
        if let Some(right) = right {
            instants.extend_from_slice(&right.instants);
            instants.sort_by(Value::total_cmp);
            instants.dedup();
        }
        let right_at = |instant: Value| match right {
            Some(steps) => match steps.level_at(instant) {
                Level::Known(value) => Some(value),
                Level::Undefined(_) => None,
            },
            None => Some(constant),
        };
        let mut changes = Vec::new();
        let mut previous: Option<(Value, Value)> = None;
        for instant in instants {
            let (Level::Known(left), Some(right)) = (left.level_at(instant), right_at(instant))
            else {
                continue;
            };
            if let Some((left_previous, right_previous)) = previous
                && left != left_previous
            {
                changes.push(Change {
                    at: instant,
                    left_previous,
                    left,
                    right_previous,
                    right,
                });
            }
            previous = Some((left, right));
        }
        Ok(changes)
    }

    /// The instant of a `WHEN` condition's requested occurrence inside
    /// `[lower, upper]`, counted as the sampled evaluator counts it: exact on
    /// an event condition, interpolated on a sampled one.
    fn when_instant(
        &self,
        condition: &WhenCondition,
        lower: Value,
        upper: Value,
        minval: Value,
    ) -> Result<Option<Value>, String> {
        if let Some(refusal) = self.mixed_condition(condition) {
            return Err(refusal);
        }
        if !self.is_event(&condition.left) {
            return Ok(first_measure_condition_event(
                condition,
                minval,
                self.data,
                lower,
                upper,
                self.data.segment_starts,
            )?
            .map(|(_, _, instant, _)| instant));
        }
        let changes = self.changes(condition)?;
        let candidates = changes.iter().filter_map(|change| {
            let crossing = measure_condition_crossing(
                change.left_previous,
                change.left,
                change.right_previous,
                change.right,
                minval,
            )?;
            point_event_axis_in_window(change.at, lower, upper, minval).then_some((
                0,
                0.0,
                change.at,
                crossing.current_within_minval,
                crossing.direction,
            ))
        });
        Ok(select_measure_condition_occurrence(
            candidates,
            condition.occurrence.edge,
            condition.occurrence.number,
        )
        .map(|(_, _, instant, _)| instant))
    }

    /// The instant one `TRIG` or `TARG` clause resolves to, counted as the
    /// sampled TrigTarg reading counts it, each clause from its own `TD`: an
    /// event clause at the step that crosses, a sampled clause interpolated
    /// as it always is.
    fn clause_instant(
        &self,
        clause: &TrigSpec,
        td: Option<Value>,
        (from, to, minval): (Option<Value>, Option<Value>, Value),
    ) -> Result<Option<Value>, String> {
        let condition = match &clause.event {
            TriggerEvent::At(target) => {
                let reached = target.is_finite()
                    && delay_at_is_reached(
                        self.data.axis,
                        *target,
                        self.data.segment_starts,
                        minval,
                    );
                return Ok(reached.then_some(*target));
            }
            TriggerEvent::When(condition) => condition,
        };
        if let Some(refusal) = self.mixed_condition(condition) {
            return Err(refusal);
        }
        if !self.is_event(&condition.left) {
            let window = MeasureWindow {
                from,
                to,
                td,
                minval,
            };
            return delay_clause_event(clause, window, self.data, false, None);
        }
        let crossings = self.changes(condition)?;
        let mut crossings = crossings.iter().filter_map(|change| {
            let previous = change.left_previous - change.right_previous;
            let current = change.left - change.right;
            let found = current.abs() < minval
                || previous < 0.0 && current > 0.0
                || previous > 0.0 && current < 0.0;
            if !found || !delay_td_accepts_modern_event(change.at, td, minval) {
                return None;
            }
            let rise = change.left > change.left_previous;
            let requested = !clause.occurrence_explicit
                || match condition.occurrence.edge {
                    EdgeType::Rise => rise,
                    EdgeType::Fall => !rise,
                    EdgeType::Cross => true,
                };
            requested.then_some(change.at)
        });
        let number = condition.occurrence.number;
        Ok(if number < 0 {
            crossings.rev().nth(number.unsigned_abs() - 1)
        } else {
            crossings.nth(number.max(1).unsigned_abs() - 1)
        })
    }

    fn delay(
        &self,
        trig: &TrigSpec,
        targ: &TrigSpec,
        window: (Option<Value>, Option<Value>, Value),
        legacy: bool,
    ) -> MeasureResult {
        if legacy || trig.frac_max.is_some() || targ.frac_max.is_some() {
            return self.failed(
                "the legacy TRIG/TARG reading (FRAC_MAX, or .OPTIONS MEASURE USE_LTTM) reads \
                 sampled waveforms only; an event operand is measured by the standard reading",
            );
        }
        let trigger = match self.clause_instant(trig, trig.td, window) {
            Ok(Some(instant)) => instant,
            Ok(None) => return self.failed("Trigger condition not found"),
            Err(error) => return self.failed(&error),
        };
        let target = match self.clause_instant(targ, targ.td.or(trig.td), window) {
            Ok(Some(instant)) => instant,
            Ok(None) => return self.failed("Target condition not found"),
            Err(error) => return self.failed(&error),
        };
        MeasureResult::success(self.name(), target - trigger)
    }

    fn find(
        &self,
        signal: &str,
        at: Option<Value>,
        when: Option<&WhenCondition>,
        (from, to, td): (Option<Value>, Option<Value>, Option<Value>),
        minval: Value,
    ) -> MeasureResult {
        let (lower, upper) = self.point_window(from, to, td);
        let instant = if let Some(at) = at {
            if !MeasureEngine::axis_in_measurement_window_with_minval(at, lower, upper, minval) {
                return self
                    .failed("AT point is outside the measurement window")
                    .with_event_axis(at);
            }
            let steps = match self.steps(signal) {
                Ok(steps) => steps,
                Err(error) => return self.failed(&error).with_event_axis(at),
            };
            if !(steps.start()..=steps.end).contains(&at) {
                return self
                    .failed("Time point not in simulation range")
                    .with_event_axis(at);
            }
            at
        } else if let Some(condition) = when {
            match self.when_instant(condition, lower, upper, minval) {
                Ok(Some(instant)) => instant,
                Ok(None) => {
                    return self.failed("WHEN condition not found in the measurement window");
                }
                Err(error) => return self.failed(&error),
            }
        } else {
            return self.failed("FIND requires AT= or WHEN condition");
        };
        let value = if self.is_event(signal) {
            self.value_at(signal, instant)
        } else {
            self.sampled_value_at(signal, instant, minval)
        };
        let result = match value {
            Ok(value) => MeasureResult::success(self.name(), value),
            Err(error) => self.failed(&error),
        };
        result.with_event_axis(instant)
    }

    /// A sampled waveform's value at `time`, interpolated between the
    /// accepted points around it as `FIND … AT` interpolates it.
    fn sampled_value_at(&self, signal: &str, time: Value, minval: Value) -> Result<Value, String> {
        let values = lookup_signal(self.data.signals, signal)
            .ok_or_else(|| format!("Signal '{signal}' not found"))?;
        find_at_accepted_points(
            self.data.axis,
            values,
            time,
            minval,
            self.data.segment_starts,
        )?
        .ok_or_else(|| "Time point not in simulation range".to_string())
    }

    /// The part of the analysis inside `FROM`/`TO`, or `None` when the two
    /// do not overlap.
    fn window(
        &self,
        steps: &Steps,
        from: Option<Value>,
        to: Option<Value>,
    ) -> Option<(Value, Value)> {
        let (lower, upper) = MeasureEngine::measurement_window_bounds(self.data.axis, from, to);
        let (lower, upper) = (lower.max(steps.start()), upper.min(steps.end));
        (lower <= upper).then_some((lower, upper))
    }

    /// The known pieces of `signal` over `[lower, upper)`, refusing the
    /// first undefined one.
    fn known_pieces(
        &self,
        signal: &str,
        steps: &Steps,
        lower: Value,
        upper: Value,
    ) -> Result<Vec<(Value, Value, Value)>, String> {
        steps
            .pieces(lower, upper)
            .map(|(from, to, level)| match level {
                Level::Known(value) => Ok((from, to, value)),
                Level::Undefined(state) => Err(format!(
                    "{} is {state} over [{from:e} s, {to:e} s), inside the measurement window",
                    signal.trim()
                )),
            })
            .collect()
    }

    fn aggregate(
        &self,
        signal: &str,
        from: Option<Value>,
        to: Option<Value>,
        aggregate: Aggregate,
    ) -> MeasureResult {
        if matches!((from, to), (Some(from), Some(to)) if from > to) {
            return self.failed("Empty range");
        }
        let steps = match self.steps(signal) {
            Ok(steps) => steps,
            Err(error) => return self.failed(&error),
        };
        let Some((lower, upper)) = self.window(steps, from, to) else {
            return self.failed("Empty range");
        };
        let pieces = match self.known_pieces(signal, steps, lower, upper) {
            Ok(pieces) => pieces,
            Err(error) => return self.failed(&error),
        };
        let integral = |integrand: fn(Value) -> Value| {
            pieces
                .iter()
                .map(|(from, to, value)| integrand(*value) * (to - from))
                .sum::<Value>()
        };
        let width = upper - lower;
        match aggregate {
            Aggregate::Integ => MeasureResult::success(self.name(), integral(|value| value)),
            _ if width <= 0.0 => self.failed("Empty range"),
            Aggregate::Avg => MeasureResult::success(self.name(), integral(|value| value) / width),
            Aggregate::Rms => MeasureResult::success(
                self.name(),
                (integral(|value| value * value) / width).sqrt(),
            ),
        }
    }

    /// The values `signal` takes over `[lower, upper)`, each with the instant
    /// it begins there; at a window of one instant, the value at it.
    fn window_values(
        &self,
        signal: &str,
        from: Option<Value>,
        to: Option<Value>,
    ) -> Result<Vec<(Value, Value)>, String> {
        let steps = self.steps(signal)?;
        let Some((lower, upper)) = self.window(steps, from, to) else {
            return Err("Empty range".to_string());
        };
        if lower == upper {
            return Ok(vec![(lower, self.value_at(signal, lower)?)]);
        }
        Ok(self
            .known_pieces(signal, steps, lower, upper)?
            .into_iter()
            .map(|(from, _, value)| (from, value))
            .collect())
    }

    fn extremum(
        &self,
        signal: &str,
        from: Option<Value>,
        to: Option<Value>,
        output: ExtremaOutput,
        is_max: bool,
    ) -> MeasureResult {
        let values = match self.window_values(signal, from, to) {
            Ok(values) => values,
            Err(error) => return self.failed(&error),
        };
        let mut selected: Option<(Value, Value)> = None;
        for (instant, value) in values {
            let replaces = selected.is_none_or(|(_, extreme)| {
                if is_max {
                    value > extreme
                } else {
                    value < extreme
                }
            });
            if replaces {
                selected = Some((instant, value));
            }
        }
        let Some((instant, value)) = selected else {
            return self.failed("Empty range");
        };
        let mut result = MeasureResult::success(self.name(), value);
        if output == ExtremaOutput::IndependentAxis {
            result = result.with_output_projection(instant);
        }
        result.with_event_axis(instant)
    }

    fn peak_to_peak(&self, signal: &str, from: Option<Value>, to: Option<Value>) -> MeasureResult {
        let values = match self.window_values(signal, from, to) {
            Ok(values) => values,
            Err(error) => return self.failed(&error),
        };
        if values.is_empty() {
            return self.failed("Empty range");
        }
        let minimum = values
            .iter()
            .map(|(_, value)| *value)
            .fold(Value::INFINITY, Value::min);
        let maximum = values
            .iter()
            .map(|(_, value)| *value)
            .fold(Value::NEG_INFINITY, Value::max);
        MeasureResult::success(self.name(), maximum - minimum)
    }
}

/// Which exact integral of a step function a statement reports.
#[derive(Clone, Copy)]
enum Aggregate {
    Avg,
    Rms,
    Integ,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn steps(points: &[(Value, Level)], end: Value) -> Steps {
        Steps::new(points.iter().copied(), 0.0, end)
    }

    #[test]
    fn an_operand_is_an_accessor_around_one_name() {
        assert_eq!(event_operand("D(q)"), Some((Domain::Digital, "q")));
        assert_eq!(event_operand(" e( x1.w ) "), Some((Domain::Real, "x1.w")));
        assert_eq!(event_operand("V(q)"), None);
        assert_eq!(event_operand("D(a,b)"), None);
        assert_eq!(event_operand("D()"), None);
        assert_eq!(event_operand("DERIV"), None);
    }

    #[test]
    fn a_step_function_is_right_continuous_and_collapses_repeats() {
        let one = Level::Known(1.0);
        let zero = Level::Known(0.0);
        let series = steps(
            &[
                (0.0, zero),
                (2.0, one),
                (2.0, zero),
                (3.0, zero),
                (5.0, one),
            ],
            10.0,
        );
        assert_eq!(series.instants, vec![0.0, 5.0]);
        assert_eq!(series.level_at(4.999), zero);
        assert_eq!(series.level_at(5.0), one);
        assert_eq!(series.level_at(10.0), one);
    }

    #[test]
    fn a_trace_that_starts_late_is_undefined_until_it_starts() {
        let series = steps(&[(2.0, Level::Known(1.0))], 4.0);
        assert_eq!(series.level_at(1.0), Level::Undefined('x'));
        assert_eq!(
            series.pieces(0.0, 4.0).collect::<Vec<_>>(),
            vec![
                (0.0, 2.0, Level::Undefined('x')),
                (2.0, 4.0, Level::Known(1.0))
            ]
        );
    }

    #[test]
    fn a_bus_word_is_msb_first_and_x_beats_z() {
        let bit = Level::Known;
        let msb = steps(&[(0.0, bit(1.0))], 3.0);
        let lsb = steps(
            &[
                (0.0, bit(0.0)),
                (1.0, bit(1.0)),
                (2.0, Level::Undefined('z')),
            ],
            3.0,
        );
        let word = bus_steps(&[msb, lsb], 0.0, 3.0);
        assert_eq!(word.level_at(0.5), bit(2.0));
        assert_eq!(word.level_at(1.0), bit(3.0));
        assert_eq!(word.level_at(2.5), Level::Undefined('z'));
        let x = steps(&[(0.0, Level::Undefined('x'))], 3.0);
        let z = steps(&[(0.0, Level::Undefined('z'))], 3.0);
        assert_eq!(
            bus_steps(&[z, x], 0.0, 3.0).level_at(1.0),
            Level::Undefined('x')
        );
    }
}
