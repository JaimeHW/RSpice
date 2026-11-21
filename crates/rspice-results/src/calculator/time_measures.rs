//! Time-domain measurements: crossings, edges, delays, pulses, periods and
//! step-response figures.
//!
//! # Conventions
//!
//! - Every function here reads its x-axis as elapsed time, so a sweep that
//!   runs backwards is refused rather than measured in reverse.
//! - Levels a periodic measurement keys off default to the mid level,
//!   `(min + max) / 2`. Percentages are returned in percent.
//! - Edge thresholds are percentages *of the transition*, as ViVA states
//!   them: `rise(w, 10, 90)` runs from 10 % to 90 % of the way from the
//!   initial level to the final one, and so does `fall(w, 10, 90)` — on a
//!   falling edge that is the 90 % level down to the 10 % level. The initial
//!   and final levels default to the first and last samples, as ViVA's
//!   `riseTime` takes them; give them to fix the 0 % and 100 % levels
//!   absolutely, as a clock's logic swing needs.
//! - An edge is a complete transition: the most recent crossing of its first
//!   threshold, then the next crossing of its second. A runt that turns back
//!   before the second threshold is not an edge, and a second edge needs the
//!   signal to cross the first threshold again.
//! - Nothing extrapolates. A measurement that needs a crossing the window
//!   does not hold says which one is missing.
//! - `period`, `frequency`, `dutyCycle`, `rise`, `fall` and `slewRate`
//!   given the word `"plot"` anywhere after the trace read every cycle
//!   instead of one figure: a trace of one value per cycle or edge, standing
//!   at the time that cycle or edge completes, as EZwave's measurement tool
//!   plots them and ViVA's `freq` and `dutyCycle` do. A count then says
//!   where the trace starts or how many cycles it reads, as it does for the
//!   single figure.
//!
//! Every measurement reads its trace through [`Series`], in as many passes as it needs, so a
//! stored trace is read a block at a time and never held whole, and a trace in memory is read
//! by the same code.

use std::borrow::Cow;

use super::evaluator::EvaluationError;
use super::measures::{Argument, Edge, edge, keyword, nth, number, ordinal, series};
use super::series::Series;
use super::value::RealValue;

/// A single figure a measurement reads off a trace, given the arguments after it.
type Figure = fn(&str, &Series, &[Argument]) -> Result<f64, EvaluationError>;

/// The every-cycle form of a measurement: a trace of one value per cycle.
type EveryCycle = fn(&str, &Series, &[Argument]) -> Result<RealValue, EvaluationError>;

/// A time-domain measurement of one trace: how many arguments it takes, the trace included,
/// its single figure, and its every-cycle form when it has one.
#[derive(Clone, Copy)]
pub(super) struct Timed {
    least: usize,
    most: usize,
    figure: Figure,
    every_cycle: Option<EveryCycle>,
}

/// The measurement `name` (lower case), if this module owns it and it reads one trace.
pub(super) fn lookup(name: &str) -> Option<Timed> {
    let timed = |least, most, figure, every_cycle| Timed {
        least,
        most,
        figure,
        every_cycle,
    };
    Some(match name {
        "cross" => timed(3, 4, cross, None),
        "rise" | "risetime" => timed(1, 6, rise, Some(rise_trace as EveryCycle)),
        "fall" | "falltime" => timed(1, 6, fall, Some(fall_trace as EveryCycle)),
        "slewrate" => timed(1, 6, slew_rate, Some(slew_rate_trace as EveryCycle)),
        "pulsewidth" => timed(1, 4, pulse_width, None),
        "duty" | "dutycycle" => timed(1, 2, duty_cycle, Some(duty_cycle_trace as EveryCycle)),
        "period" => timed(1, 3, period, Some(period_trace as EveryCycle)),
        "freq" | "frequency" => timed(1, 3, frequency, Some(frequency_trace as EveryCycle)),
        "overshoot" => timed(1, 3, overshoot, None),
        "undershoot" => timed(1, 3, undershoot, None),
        "settling" | "settlingtime" => timed(2, 4, settling_time, None),
        _ => return None,
    })
}

/// Hold a call of `count` arguments, the trace included, to `least ..= most`.
fn arity(name: &str, count: usize, least: usize, most: usize) -> Result<(), EvaluationError> {
    if count < least || count > most {
        Err(EvaluationError::ArgCountMismatch {
            func: name.to_owned(),
            expected: if count < least { least } else { most },
            actual: count,
        })
    } else {
        Ok(())
    }
}

/// Where the word `"plot"` stands among `rest`.
fn plot_at(rest: &[Argument]) -> Option<usize> {
    rest.iter()
        .position(|argument| matches!(argument, Argument::Keyword(word) if word == "plot"))
}

impl Timed {
    /// The measurement of a call evaluated in memory: the trace is `args[0]`.
    pub(super) fn dispatch(
        self,
        name: &str,
        args: &[Argument],
    ) -> Result<RealValue, EvaluationError> {
        let plotted = args
            .get(1..)
            .and_then(plot_at)
            .filter(|_| self.every_cycle.is_some());
        arity(
            name,
            args.len() - usize::from(plotted.is_some()),
            self.least,
            self.most,
        )?;
        let (x, y) = series(name, args, 0)?;
        self.measure(name, &Series::memory(x, y), &args[1..])
    }

    /// Measure `trace`, the call's first argument, with the arguments after it: its single
    /// figure, or, with the word `"plot"` among them, its every-cycle form.
    pub(super) fn measure(
        self,
        name: &str,
        trace: &Series,
        rest: &[Argument],
    ) -> Result<RealValue, EvaluationError> {
        let plot = self
            .every_cycle
            .and_then(|form| plot_at(rest).map(|at| (form, at)));
        let rest: Cow<'_, [Argument]> = match plot {
            Some((_, at)) => {
                let mut rest = rest.to_vec();
                rest.remove(at);
                Cow::Owned(rest)
            }
            None => Cow::Borrowed(rest),
        };
        arity(name, rest.len() + 1, self.least, self.most)?;
        trace.forward(name)?;
        match plot {
            Some((form, _)) => form(name, trace, &rest),
            None => (self.figure)(name, trace, &rest).map(RealValue::Scalar),
        }
    }
}

/// `delay` of a call evaluated in memory: the traces are `args[0]` and `args[1]`.
pub(super) fn dispatch_delay(name: &str, args: &[Argument]) -> Result<f64, EvaluationError> {
    arity(name, args.len(), 2, 8)?;
    let (x, y) = series(name, args, 0)?;
    let from = Series::memory(x, y);
    from.forward(name)?;
    let (x, y) = series(name, args, 1)?;
    let to = Series::memory(x, y);
    to.forward(name)?;
    delay(name, &from, &to, &args[2..])
}

/// `delay` from `from` to `to`, the call's first two arguments, with the arguments after them.
pub(super) fn measure_delay(
    name: &str,
    from: &Series,
    to: &Series,
    rest: &[Argument],
) -> Result<f64, EvaluationError> {
    arity(name, rest.len() + 2, 2, 8)?;
    from.forward(name)?;
    to.forward(name)?;
    delay(name, from, to, rest)
}

fn math(message: String) -> EvaluationError {
    EvaluationError::MathError(message)
}

/// Where a signal passed through a level, and which way it was going.
#[derive(Clone, Copy)]
struct Crossing {
    x: f64,
    rising: bool,
}

/// Every crossing of `level`, in domain order — which is storage order, so
/// every caller clears its grid through [`Series::forward`] first. `rising`
/// likewise means rising *in x*, not merely increasing along the samples.
///
/// A sample sitting exactly on the level is not itself a crossing — a curve
/// that touches the level and turns back has not crossed it. A run of
/// at-level samples between two opposite signs is reported at the first of
/// them, which is where the level was actually reached. A non-finite sample
/// breaks the search: a hole is not a crossing, and pretending otherwise
/// would invent an edge across a gap.
struct Crossings<I> {
    pairs: I,
    level: f64,
    /// The last sample off the level: its x, its value and which side it is on.
    previous: Option<(f64, f64, bool)>,
    /// The first at-level sample since then.
    at_level: Option<f64>,
}

impl<I: Iterator<Item = (f64, f64)>> Iterator for Crossings<I> {
    type Item = Crossing;

    fn next(&mut self) -> Option<Crossing> {
        loop {
            let (x, value) = self.pairs.next()?;
            let delta = value - self.level;
            if !delta.is_finite() {
                self.previous = None;
                self.at_level = None;
                continue;
            }
            if delta == 0.0 {
                if self.previous.is_some() && self.at_level.is_none() {
                    self.at_level = Some(x);
                }
                continue;
            }
            let above = delta > 0.0;
            let crossed = match self.previous {
                Some((x0, y0, was_above)) if was_above != above => Some(
                    self.at_level
                        .unwrap_or_else(|| x0 + (self.level - y0) * (x - x0) / (value - y0)),
                ),
                _ => None,
            };
            self.previous = Some((x, value, above));
            self.at_level = None;
            if let Some(at) = crossed {
                return Some(Crossing {
                    x: at,
                    rising: above,
                });
            }
        }
    }
}

type Pairs<'a> = Box<dyn Iterator<Item = (f64, f64)> + 'a>;

fn crossings<'a>(trace: &Series<'a>, level: f64) -> Crossings<Pairs<'a>> {
    Crossings {
        pairs: trace.pairs(),
        level,
        previous: None,
        at_level: None,
    }
}

/// The crossings of `level` going `edge`'s way.
fn edge_crossings<'a>(
    trace: &Series<'a>,
    level: f64,
    edge: Edge,
) -> impl Iterator<Item = f64> + 'a {
    crossings(trace, level)
        .filter(move |crossing| edge.admits(crossing.rising))
        .map(|crossing| crossing.x)
}

/// How a count of crossings reads in a refusal: "rising " or nothing.
fn edge_words(edge: Edge) -> &'static str {
    match edge {
        Edge::Rising => "rising ",
        Edge::Falling => "falling ",
        Edge::Either => "",
    }
}

/// The lowest and highest finite samples, holes skipped, as
/// [`super::functions::extremes`] reads them in memory.
fn extremes(name: &str, trace: &Series) -> Result<(f64, f64), EvaluationError> {
    let mut low = f64::INFINITY;
    let mut high = f64::NEG_INFINITY;
    for (_, value) in trace.pairs().filter(|(_, value)| value.is_finite()) {
        low = low.min(value);
        high = high.max(value);
    }
    if low.is_finite() {
        Ok((low, high))
    } else {
        Err(math(format!("{name} has no finite sample to measure")))
    }
}

/// The 50 % level a periodic measurement keys off: `(min + max) / 2`.
fn mid_level(name: &str, trace: &Series) -> Result<f64, EvaluationError> {
    let (low, high) = extremes(name, trace)?;
    if low == high {
        return Err(math(format!(
            "{name} needs a signal that moves; every sample is {low}"
        )));
    }
    Ok((low + high) * 0.5)
}

/// The level at `index`, or the mid level when the call stops before it.
fn level_or_mid(
    name: &str,
    args: &[Argument],
    index: usize,
    trace: &Series,
) -> Result<f64, EvaluationError> {
    match number(name, "level", args, index)? {
        Some(level) => Ok(level),
        None => mid_level(name, trace),
    }
}

/// The `wanted`th of `found`, or how many there were.
fn nth_of(found: impl Iterator<Item = f64>, wanted: usize) -> Result<f64, usize> {
    let mut count = 0;
    for at in found {
        count += 1;
        if count == wanted {
            return Ok(at);
        }
    }
    Err(count)
}

// =============================================================================
// Crossings and delays
// =============================================================================

/// `cross(w, level, n[, edge])` — where `w` crosses `level` for the nth
/// time, counting only crossings of the given edge type (default
/// `"either"`).
fn cross(name: &str, trace: &Series, args: &[Argument]) -> Result<f64, EvaluationError> {
    let level = number(name, "level", args, 0)?.unwrap_or(f64::NAN);
    let wanted = ordinal(name, "ordinal", args, 1)?;
    let edge = edge(name, "edge type", args, 2, Edge::Either)?;
    nth_of(edge_crossings(trace, level, edge), wanted).map_err(|found| {
        math(format!(
            "the signal crosses {level} {}{found} time(s), not {wanted}",
            edge_words(edge),
        ))
    })
}

/// `delay(a, b[, level1[, edge1[, n1[, level2[, edge2[, n2]]]]]])` — from
/// the `n1`th `edge1` crossing of `level1` by `a` to the `n2`th `edge2`
/// crossing of `level2` by `b`, with ViVA's edge types.
///
/// `delay(a, b)` reads each signal at its own mid level, and `delay(a, b,
/// level)` reads both at one level, which is what a logic threshold needs.
/// The second signal's level, edge type and edge number default to the
/// first's, so `delay(a, b, 2.5, "rising", 3)` measures the third rising
/// edge of each. Both edges are counted from the start of the window, as
/// SPICE `.MEASURE` TRIG/TARG counts them, so a negative delay means `b`'s
/// edge came first. The two waveforms are read independently, so they need
/// not share a grid.
fn delay(
    name: &str,
    from: &Series,
    to: &Series,
    args: &[Argument],
) -> Result<f64, EvaluationError> {
    let (from_level, from_edge, from_n) = match number(name, "level", args, 0)? {
        Some(level) => (
            level,
            edge(name, "edge type", args, 1, Edge::Either)?,
            ordinal(name, "edge number", args, 2)?,
        ),
        None => (mid_level(name, from)?, Edge::Either, 1),
    };
    let to_level = match number(name, "second level", args, 3)? {
        Some(level) => level,
        None if !args.is_empty() => from_level,
        None => mid_level(name, to)?,
    };
    let to_edge = edge(name, "second edge type", args, 4, from_edge)?;
    let to_n = if args.len() > 5 {
        ordinal(name, "second edge number", args, 5)?
    } else {
        from_n
    };
    let first = nth_of(edge_crossings(from, from_level, from_edge), from_n).map_err(|_| {
        math(format!(
            "the first waveform has no {} {}crossing of {from_level}",
            nth(from_n),
            edge_words(from_edge)
        ))
    })?;
    let second = nth_of(edge_crossings(to, to_level, to_edge), to_n).map_err(|_| {
        math(format!(
            "the second waveform has no {} {}crossing of {to_level}",
            nth(to_n),
            edge_words(to_edge)
        ))
    })?;
    Ok(second - first)
}

// =============================================================================
// Edges
// =============================================================================

/// One measured edge: where it left its first threshold and reached its
/// second, and the two threshold levels.
#[derive(Clone, Copy)]
struct MeasuredEdge {
    start: f64,
    end: f64,
    from: f64,
    to: f64,
}

/// Which way an edge function measures when the caller gives no levels.
#[derive(Clone, Copy, PartialEq)]
enum Sense {
    Rise,
    Fall,
    /// From the initial level to the final one, whichever way that goes;
    /// rising when neither is given.
    Either,
}

/// The edges a call of `rise`, `fall` or `slewRate` reads: the trace, the
/// two threshold levels, the direction, and the edge number asked for.
#[derive(Clone, Copy)]
struct EdgeQuery<'q, 'a> {
    trace: &'q Series<'a>,
    from: f64,
    to: f64,
    direction: Edge,
    wanted: usize,
}

/// `[theta1, theta2[, n[, initial, final]]]` after the trace, the arguments `rise`, `fall`
/// and `slewRate` share.
fn edge_query<'q, 'a>(
    name: &str,
    trace: &'q Series<'a>,
    args: &[Argument],
    sense: Sense,
) -> Result<EdgeQuery<'q, 'a>, EvaluationError> {
    if args.len() == 1 || args.len() == 4 {
        return Err(math(format!(
            "{name} takes its two thresholds together, and its initial and final levels together"
        )));
    }
    let theta1 = number(name, "first threshold", args, 0)?.unwrap_or(10.0);
    let theta2 = number(name, "second threshold", args, 1)?.unwrap_or(90.0);
    if !(0.0..theta2).contains(&theta1) || theta2 > 100.0 {
        return Err(math(format!(
            "{name} thresholds are percentages of the transition with 0 <= first < second <= 100; \
             got {theta1} and {theta2}"
        )));
    }
    let wanted = ordinal(name, "edge number", args, 2)?;
    let given = match (
        number(name, "initial level", args, 3)?,
        number(name, "final level", args, 4)?,
    ) {
        (Some(initial), Some(last)) => Some((initial, last)),
        _ => None,
    };
    // Unless given, the 0 % and 100 % levels are where the trace starts and
    // where it ends, as ViVA's riseTime takes them, so an overshoot does not
    // move the 90 % point.
    let (initial, last) = match given {
        Some(levels) => levels,
        None => step_levels(name, trace.pairs())?,
    };
    let unstated = if given.is_none() {
        "; its first and last samples set them unless the 0 % and 100 % levels are given"
    } else {
        ""
    };
    if initial == last {
        return Err(math(format!(
            "{name} needs an edge; its initial and final levels are both {initial}{unstated}"
        )));
    }
    let rising = last > initial;
    match (sense, rising) {
        (Sense::Rise, false) => {
            return Err(math(format!(
                "{name} measures a rising edge, so its final level must be above its initial level\
                 {unstated}"
            )));
        }
        (Sense::Fall, true) => {
            return Err(math(format!(
                "{name} measures a falling edge, so its final level must be below its initial level\
                 {unstated}"
            )));
        }
        _ => {}
    }
    let from = initial + theta1 / 100.0 * (last - initial);
    let to = initial + theta2 / 100.0 * (last - initial);
    let direction = if rising { Edge::Rising } else { Edge::Falling };
    Ok(EdgeQuery {
        trace,
        from,
        to,
        direction,
        wanted,
    })
}

/// Every complete edge the query names, in time order.
fn complete_edges<'a>(query: EdgeQuery<'_, 'a>) -> impl Iterator<Item = MeasuredEdge> + 'a {
    let EdgeQuery {
        trace,
        from,
        to,
        direction,
        ..
    } = query;
    // Walk both thresholds' crossings in time order. A first-threshold
    // crossing arms an edge (the latest one wins, so a runt that turned back
    // is forgotten); the next second-threshold crossing completes it.
    let mut starts = edge_crossings(trace, from, direction).peekable();
    let mut ends = edge_crossings(trace, to, direction).peekable();
    let mut armed = None;
    std::iter::from_fn(move || {
        loop {
            // At one instant — a vertical edge on duplicated timepoints — the
            // first threshold is crossed before the second.
            let start_first = match (starts.peek(), ends.peek()) {
                (None, None) => return None,
                (Some(_), None) => true,
                (None, Some(_)) => false,
                (Some(start), Some(end)) => start.total_cmp(end).is_le(),
            };
            if start_first {
                armed = starts.next();
            } else if let (Some(end), Some(start)) = (ends.next(), armed.take()) {
                return Some(MeasuredEdge {
                    start,
                    end,
                    from,
                    to,
                });
            }
        }
    })
}

/// The nth complete edge a call of `rise`, `fall` or `slewRate` names.
fn measured_edge(
    name: &str,
    trace: &Series,
    args: &[Argument],
    sense: Sense,
) -> Result<MeasuredEdge, EvaluationError> {
    let query = edge_query(name, trace, args, sense)?;
    let mut found = 0;
    for edge in complete_edges(query) {
        found += 1;
        if found == query.wanted {
            return Ok(edge);
        }
    }
    Err(math(format!(
        "{name} finds {found} complete {} edge(s) from {} to {}, not {}",
        query.direction.word(),
        query.from,
        query.to,
        query.wanted
    )))
}

/// Every complete edge from the nth on, each read by `reading` and standing
/// at the time it completes.
fn edge_trace(
    name: &str,
    trace: &Series,
    args: &[Argument],
    sense: Sense,
    reading: fn(&str, &MeasuredEdge) -> Result<f64, EvaluationError>,
) -> Result<RealValue, EvaluationError> {
    let query = edge_query(name, trace, args, sense)?;
    let mut found = 0;
    let (mut at, mut values) = (Vec::new(), Vec::new());
    for edge in complete_edges(query) {
        found += 1;
        if found >= query.wanted {
            at.push(edge.end);
            values.push(reading(name, &edge)?);
        }
    }
    if at.is_empty() {
        return Err(math(format!(
            "{name} finds {found} complete {} edge(s) from {} to {}, none from the {} on",
            query.direction.word(),
            query.from,
            query.to,
            nth(query.wanted)
        )));
    }
    Ok(RealValue::Waveform(at, values))
}

fn duration(_: &str, edge: &MeasuredEdge) -> Result<f64, EvaluationError> {
    Ok(edge.end - edge.start)
}

/// An edge's average slope between its thresholds.
fn slope(name: &str, edge: &MeasuredEdge) -> Result<f64, EvaluationError> {
    let span = edge.end - edge.start;
    if span <= 0.0 {
        return Err(math(format!(
            "{name} finds a vertical edge: it passes {} and {} at the same instant {}",
            edge.from, edge.to, edge.start
        )));
    }
    Ok((edge.to - edge.from) / span)
}

/// `rise(w[, theta1, theta2[, n[, initial, final]]])` — the time the nth
/// rising edge takes from `theta1` to `theta2` percent of its transition
/// (default 10 % to 90 % of the way from the first sample to the last).
fn rise(name: &str, trace: &Series, args: &[Argument]) -> Result<f64, EvaluationError> {
    duration(name, &measured_edge(name, trace, args, Sense::Rise)?)
}

fn rise_trace(name: &str, trace: &Series, args: &[Argument]) -> Result<RealValue, EvaluationError> {
    edge_trace(name, trace, args, Sense::Rise, duration)
}

/// `fall(w[, theta1, theta2[, n[, initial, final]]])` — as `rise`, on the
/// nth falling edge.
fn fall(name: &str, trace: &Series, args: &[Argument]) -> Result<f64, EvaluationError> {
    duration(name, &measured_edge(name, trace, args, Sense::Fall)?)
}

fn fall_trace(name: &str, trace: &Series, args: &[Argument]) -> Result<RealValue, EvaluationError> {
    edge_trace(name, trace, args, Sense::Fall, duration)
}

/// `slewRate(w[, theta1, theta2[, n[, initial, final]]])` — the average
/// slope of the nth edge between its two thresholds, in units per unit x.
/// The edge runs from the initial level to the final one, so it is rising
/// unless the final level is given below the initial, and a falling edge
/// reports a negative slope.
fn slew_rate(name: &str, trace: &Series, args: &[Argument]) -> Result<f64, EvaluationError> {
    slope(name, &measured_edge(name, trace, args, Sense::Either)?)
}

fn slew_rate_trace(
    name: &str,
    trace: &Series,
    args: &[Argument],
) -> Result<RealValue, EvaluationError> {
    edge_trace(name, trace, args, Sense::Either, slope)
}

// =============================================================================
// Pulses and periods
// =============================================================================

/// `pulseWidth(w[, level[, polarity[, n]]])` — how long the nth `"high"`
/// pulse stays above `level` (from a rising crossing to the next falling
/// one), or the nth `"low"` pulse below it. The level defaults to the mid
/// level and the polarity to `"high"`. A pulse already under way when the
/// window opens is not counted, because its start is not in it.
fn pulse_width(name: &str, trace: &Series, args: &[Argument]) -> Result<f64, EvaluationError> {
    let level = level_or_mid(name, args, 0, trace)?;
    let high = keyword(name, "polarity", args, 1, &["high", "low"])? != Some("low");
    let wanted = ordinal(name, "pulse number", args, 2)?;
    let polarity = if high { "high" } else { "low" };
    let mut starts = 0;
    let mut start = None;
    for crossing in crossings(trace, level) {
        match start {
            None if crossing.rising == high => {
                starts += 1;
                if starts == wanted {
                    start = Some(crossing.x);
                }
            }
            Some(at) if crossing.rising != high => return Ok(crossing.x - at),
            _ => {}
        }
    }
    Err(match start {
        None => math(format!(
            "{name} finds {starts} {polarity} pulse(s) through {level}, not {wanted}"
        )),
        Some(at) => math(format!(
            "the {} {polarity} pulse through {level} starts at {at} and does not end in the window",
            nth(wanted),
        )),
    })
}

/// `dutyCycle(w[, level])` — percent of each period spent above `level`
/// (default the mid level), averaged over every whole cycle the window
/// contains.
fn duty_cycle(name: &str, trace: &Series, args: &[Argument]) -> Result<f64, EvaluationError> {
    let (mut total, mut cycles) = (0.0, 0usize);
    duty_shares(name, trace, args, |_, share| {
        total += share;
        cycles += 1;
    })?;
    Ok(100.0 * total / cycles as f64)
}

fn duty_cycle_trace(
    name: &str,
    trace: &Series,
    args: &[Argument],
) -> Result<RealValue, EvaluationError> {
    let (mut at, mut shares) = (Vec::new(), Vec::new());
    duty_shares(name, trace, args, |end, share| {
        at.push(end);
        shares.push(100.0 * share);
    })?;
    Ok(RealValue::Waveform(at, shares))
}

/// Each whole cycle's share of time above the level, handed to `cycle` with the time of the
/// rise that ends it, in time order; refused when there is no whole cycle.
///
/// A cycle starts at a rising crossing and ends at the next rising crossing; its share is the
/// time from its start to the first falling crossing after its start, over its length.
fn duty_shares(
    name: &str,
    trace: &Series,
    args: &[Argument],
    mut cycle: impl FnMut(f64, f64),
) -> Result<(), EvaluationError> {
    let level = level_or_mid(name, args, 0, trace)?;
    // The cycles started and not yet whole: each start, the rise that ends it and the first
    // fall after it, as they arrive. Only the latest start awaits its rise, and a fall reaches
    // every start still awaiting one, so the cycles become whole in the order they started.
    let mut open: std::collections::VecDeque<(f64, Option<f64>, Option<f64>)> =
        std::collections::VecDeque::new();
    let mut whole = 0usize;
    for crossing in crossings(trace, level) {
        if crossing.rising {
            if let Some(latest) = open.back_mut() {
                latest.1 = Some(crossing.x);
            }
            open.push_back((crossing.x, None, None));
        } else {
            for started in open.iter_mut().filter(|started| started.2.is_none()) {
                started.2 = Some(crossing.x);
            }
        }
        while let Some(&(start, Some(repeats), Some(falls))) = open.front() {
            open.pop_front();
            let length = repeats - start;
            if length > 0.0 {
                whole += 1;
                cycle(repeats, (falls - start) / length);
            }
        }
    }
    if whole == 0 {
        return Err(math(format!(
            "{name} needs a whole cycle: a rising crossing of {level}, a fall, and the next rise"
        )));
    }
    Ok(())
}

/// The rising crossings of the level at `args[1]` (default the mid level)
/// that bound the cycles counted — every whole cycle in the window, or the
/// first `args[0]` — handed to `bound` in time order; the level.
fn counted_crossings(
    name: &str,
    trace: &Series,
    args: &[Argument],
    mut bound: impl FnMut(f64),
) -> Result<f64, EvaluationError> {
    let cycles = match args.first() {
        Some(_) => Some(ordinal(name, "cycle count", args, 0)?),
        None => None,
    };
    let level = level_or_mid(name, args, 1, trace)?;
    let mut rising = 0usize;
    for at in edge_crossings(trace, level, Edge::Rising) {
        rising += 1;
        bound(at);
        if cycles.is_some_and(|wanted| rising > wanted) {
            break;
        }
    }
    if rising < 2 {
        return Err(math(format!(
            "{name} needs at least one whole period between rising crossings of {level}"
        )));
    }
    if let Some(wanted) = cycles
        && rising <= wanted
    {
        return Err(math(format!(
            "{name} finds {} whole cycle(s) between rising crossings of {level}, not {wanted}",
            rising - 1
        )));
    }
    Ok(level)
}

/// The period between rising crossings of the level at `args[1]` (default
/// the mid level): averaged over every whole cycle in the window, or over
/// the first `args[0]` cycles when the call gives a count.
///
/// Measured across the cycles rather than from one, so jitter averages out
/// instead of deciding the answer.
fn mean_period(name: &str, trace: &Series, args: &[Argument]) -> Result<f64, EvaluationError> {
    let (mut first, mut last, mut count) = (None, 0.0, 0usize);
    let level = counted_crossings(name, trace, args, |at| {
        first.get_or_insert(at);
        last = at;
        count += 1;
    })?;
    let counted = count - 1;
    let span = last - first.unwrap_or(last);
    if span <= 0.0 {
        return Err(math(format!(
            "{name} found no elapsed time between crossings of {level}"
        )));
    }
    Ok(span / counted as f64)
}

/// Each counted period, standing at the rising crossing that ends it.
fn periods(
    name: &str,
    trace: &Series,
    args: &[Argument],
) -> Result<(Vec<f64>, Vec<f64>), EvaluationError> {
    let mut rising = Vec::new();
    let level = counted_crossings(name, trace, args, |at| rising.push(at))?;
    let lengths: Vec<f64> = rising.windows(2).map(|pair| pair[1] - pair[0]).collect();
    if let Some(index) = lengths.iter().position(|&length| length <= 0.0) {
        return Err(math(format!(
            "{name} finds two rising crossings of {level} at the same instant {}",
            rising[index]
        )));
    }
    rising.remove(0);
    Ok((rising, lengths))
}

/// `period(w[, n[, level]])` — the mean period, over the first `n` cycles
/// when given.
fn period(name: &str, trace: &Series, args: &[Argument]) -> Result<f64, EvaluationError> {
    mean_period(name, trace, args)
}

fn period_trace(
    name: &str,
    trace: &Series,
    args: &[Argument],
) -> Result<RealValue, EvaluationError> {
    let (at, lengths) = periods(name, trace, args)?;
    Ok(RealValue::Waveform(at, lengths))
}

/// `frequency(w[, n[, level]])` — the reciprocal of `period`.
fn frequency(name: &str, trace: &Series, args: &[Argument]) -> Result<f64, EvaluationError> {
    Ok(1.0 / mean_period(name, trace, args)?)
}

fn frequency_trace(
    name: &str,
    trace: &Series,
    args: &[Argument],
) -> Result<RealValue, EvaluationError> {
    let (at, lengths) = periods(name, trace, args)?;
    Ok(RealValue::Waveform(
        at,
        lengths.into_iter().map(|length| 1.0 / length).collect(),
    ))
}

// =============================================================================
// Step response
// =============================================================================

/// The first and last finite samples of `pairs`: where a step response starts and where it
/// ends up.
fn step_levels(
    name: &str,
    pairs: impl Iterator<Item = (f64, f64)>,
) -> Result<(f64, f64), EvaluationError> {
    let mut finite = pairs
        .map(|(_, value)| value)
        .filter(|value| value.is_finite());
    let initial = finite
        .next()
        .ok_or_else(|| math(format!("{name} has no finite sample")))?;
    Ok((initial, finite.last().unwrap_or(initial)))
}

/// The initial and final levels of a step: `args[first]` and
/// `args[first + 1]` when given, else the first and last finite samples.
fn step(
    name: &str,
    args: &[Argument],
    first: usize,
    trace: &Series,
) -> Result<(f64, f64), EvaluationError> {
    let (initial, settled) = match (
        number(name, "initial level", args, first)?,
        number(name, "final level", args, first + 1)?,
    ) {
        (Some(initial), Some(settled)) => (initial, settled),
        (None, None) => step_levels(name, trace.pairs())?,
        _ => {
            return Err(math(format!(
                "{name} takes its initial and final levels together"
            )));
        }
    };
    if initial == settled {
        return Err(math(format!(
            "{name} needs a step; the first and last samples are both {initial}"
        )));
    }
    Ok((initial, settled))
}

/// `overshoot(w[, initial, final])` — percent by which the response passed
/// its final value, relative to the step it took to get there; 0 when it
/// never passed it.
fn overshoot(name: &str, trace: &Series, args: &[Argument]) -> Result<f64, EvaluationError> {
    let (initial, settled) = step(name, args, 0, trace)?;
    let (low, high) = extremes(name, trace)?;
    let step = settled - initial;
    let peak = if step > 0.0 { high } else { low };
    Ok((100.0 * (peak - settled) / step).max(0.0))
}

/// `undershoot(w[, initial, final])` — percent by which the response moved
/// the wrong way, past its initial value, relative to the step: the dip a
/// non-minimum-phase response takes before it rises, as MATLAB's `stepinfo`
/// reports it. 0 when it never moved the wrong way.
fn undershoot(name: &str, trace: &Series, args: &[Argument]) -> Result<f64, EvaluationError> {
    let (initial, settled) = step(name, args, 0, trace)?;
    let (low, high) = extremes(name, trace)?;
    let step = settled - initial;
    let trough = if step > 0.0 { low } else { high };
    Ok((100.0 * (initial - trough) / step).max(0.0))
}

/// The samples of `trace` from `opening`, a start and the trace's value there, on: the start,
/// then every sample past it; or every sample.
fn from_start<'a>(trace: &Series<'a>, opening: Option<(f64, f64)>) -> Pairs<'a> {
    match opening {
        None => trace.pairs(),
        Some((start, at)) => Box::new(
            std::iter::once((start, at)).chain(trace.pairs().filter(move |(x, _)| *x > start)),
        ),
    }
}

/// `settlingTime(w, band[, final[, start]])` — time from `start` (default
/// the start of the window) until `w` enters and stays inside `band`
/// percent of its step, measured about the final value. The final value
/// defaults to the last sample and the initial value is the signal's value
/// at `start`.
fn settling_time(name: &str, trace: &Series, args: &[Argument]) -> Result<f64, EvaluationError> {
    // NaN is rejected by `number` before it can make the tolerance NaN, every
    // in-band test below false, and a settling time that was never measured.
    let band = number(name, "band", args, 0)?.unwrap_or(f64::NAN);
    if band <= 0.0 {
        return Err(math(format!(
            "{name} band must be a positive percentage of the step"
        )));
    }
    let start = number(name, "start", args, 2)?;
    // The value at `start`, interpolated on the trace.
    let opening = match start {
        None => None,
        Some(start) => {
            let no_samples = || math(format!("{name} needs a waveform with samples"));
            let (first, last) = trace.ends().ok_or_else(no_samples)?;
            if !(first..=last).contains(&start) {
                return Err(math(format!(
                    "{name} start {start} lies outside the window {first} to {last}"
                )));
            }
            Some((start, trace.value_at(start).ok_or_else(no_samples)?))
        }
    };
    let (initial, last) = step_levels(name, from_start(trace, opening))?;
    let settled = number(name, "final value", args, 1)?.unwrap_or(last);
    if initial == settled {
        return Err(math(format!(
            "{name} needs a step; the first and last samples are both {initial}"
        )));
    }
    let tolerance = (settled - initial).abs() * band / 100.0;
    // The *last* excursion outside the band is what settling means; an early
    // dip through it says nothing about when the response finally stayed put.
    // Kept: the first sample's x, and the last sample outside the band with the one after it.
    let mut origin = None;
    type Sample = (f64, f64);
    let mut last_outside: Option<(Sample, Option<Sample>)> = None;
    for (x, value) in from_start(trace, opening) {
        origin.get_or_insert(x);
        if let Some((_, after @ None)) = &mut last_outside {
            *after = Some((x, value));
        }
        if value.is_finite() && (value - settled).abs() > tolerance {
            last_outside = Some(((x, value), None));
        }
    }
    let Some(((before_x, before_y), after)) = last_outside else {
        return Ok(0.0);
    };
    let Some((after_x, after_y)) = after else {
        return Err(math(format!(
            "the signal never settles inside {band} % of its step"
        )));
    };
    if !after_y.is_finite() {
        return Err(math(format!(
            "the signal's last excursion outside {band} % of its step is followed by a hole, \
             so there is no sample to place the band entry against"
        )));
    }
    // Interpolate on the *signed* deviation, toward the band edge the signal
    // actually crosses — the one on the side it was last outside. A response
    // that flies past the settled value between these two samples lands on
    // the far side of it, and reading unsigned magnitudes there interpolates
    // toward the wrong edge and reports the entry late.
    let departure = before_y - settled;
    let arrival = after_y - settled;
    let band_edge = if departure > 0.0 {
        tolerance
    } else {
        -tolerance
    };
    let travel = arrival - departure;
    // |departure| > tolerance >= |arrival|, so the two deviations differ and
    // this cannot divide by zero; a duplicated timepoint gives the earlier x.
    let entry = before_x + (band_edge - departure) * (after_x - before_x) / travel;
    Ok(entry - origin.unwrap_or(before_x))
}

#[cfg(test)]
mod tests;
