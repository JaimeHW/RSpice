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
//!   and final levels default to the waveform's own excursion; give them to
//!   fix the 0 % and 100 % levels absolutely, as a logic swing does.
//! - An edge is a complete transition: the most recent crossing of its first
//!   threshold, then the next crossing of its second. A runt that turns back
//!   before the second threshold is not an edge, and a second edge needs the
//!   signal to cross the first threshold again.
//! - Nothing extrapolates. A measurement that needs a crossing the window
//!   does not hold says which one is missing.

use super::evaluator::EvaluationError;
use super::functions::{extremes, forward_domain, interpolate};
use super::measures::{
    Argument, Edge, Measure, arity, edge, keyword, nth, number, ordinal, series,
};

/// The function that measures `name` (lower case), if this module owns it.
pub(super) fn lookup(name: &str) -> Option<Measure> {
    Some(match name {
        "cross" => cross,
        "rise" | "risetime" => rise,
        "fall" | "falltime" => fall,
        "slewrate" => slew_rate,
        "delay" => delay,
        "pulsewidth" => pulse_width,
        "duty" | "dutycycle" => duty_cycle,
        "period" => period,
        "freq" | "frequency" => frequency,
        "overshoot" => overshoot,
        "undershoot" => undershoot,
        "settling" | "settlingtime" => settling_time,
        _ => return None,
    })
}

fn math(message: String) -> EvaluationError {
    EvaluationError::MathError(message)
}

/// A real waveform at `index` whose x-axis runs forward.
fn timed<'a>(
    name: &str,
    args: &'a [Argument],
    index: usize,
) -> Result<(&'a [f64], &'a [f64]), EvaluationError> {
    let (x, y) = series(name, args, index)?;
    forward_domain(name, x)?;
    Ok((x, y))
}

/// Where a signal passed through a level, and which way it was going.
#[derive(Clone, Copy)]
struct Crossing {
    x: f64,
    rising: bool,
}

/// Every crossing of `level`, in domain order — which is storage order, so
/// every caller clears its grid through [`forward_domain`] first. `rising`
/// likewise means rising *in x*, not merely increasing along the samples.
///
/// A sample sitting exactly on the level is not itself a crossing — a curve
/// that touches the level and turns back has not crossed it. A run of
/// at-level samples between two opposite signs is reported at the first of
/// them, which is where the level was actually reached. A non-finite sample
/// breaks the search: a hole is not a crossing, and pretending otherwise
/// would invent an edge across a gap.
fn crossings(x: &[f64], y: &[f64], level: f64) -> Vec<Crossing> {
    let mut found = Vec::new();
    let mut previous: Option<(usize, f64)> = None;
    for i in 0..x.len().min(y.len()) {
        let delta = y[i] - level;
        if !delta.is_finite() {
            previous = None;
            continue;
        }
        if delta == 0.0 {
            continue;
        }
        let sign = if delta > 0.0 { 1.0 } else { -1.0 };
        if let Some((before, previous_sign)) = previous
            && previous_sign != sign
        {
            let at = if i == before + 1 {
                x[before] + (level - y[before]) * (x[i] - x[before]) / (y[i] - y[before])
            } else {
                x[before + 1]
            };
            found.push(Crossing {
                x: at,
                rising: sign > 0.0,
            });
        }
        previous = Some((i, sign));
    }
    found
}

/// The crossings of `level` going `edge`'s way.
fn edge_crossings(x: &[f64], y: &[f64], level: f64, edge: Edge) -> Vec<f64> {
    crossings(x, y, level)
        .into_iter()
        .filter(|crossing| edge.admits(crossing.rising))
        .map(|crossing| crossing.x)
        .collect()
}

/// How a count of crossings reads in a refusal: "rising " or nothing.
fn edge_words(edge: Edge) -> &'static str {
    match edge {
        Edge::Rising => "rising ",
        Edge::Falling => "falling ",
        Edge::Either => "",
    }
}

/// The 50 % level a periodic measurement keys off: `(min + max) / 2`.
fn mid_level(name: &str, y: &[f64]) -> Result<f64, EvaluationError> {
    let (low, high) = extremes(name, y)?;
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
    y: &[f64],
) -> Result<f64, EvaluationError> {
    match number(name, "level", args, index)? {
        Some(level) => Ok(level),
        None => mid_level(name, y),
    }
}

// =============================================================================
// Crossings and delays
// =============================================================================

/// `cross(w, level, n[, edge])` — where `w` crosses `level` for the nth
/// time, counting only crossings of the given edge type (default
/// `"either"`).
fn cross(name: &str, args: &[Argument]) -> Result<f64, EvaluationError> {
    arity(name, args, 3, 4)?;
    let (x, y) = timed(name, args, 0)?;
    let level = number(name, "level", args, 1)?.unwrap_or(f64::NAN);
    let wanted = ordinal(name, "ordinal", args, 2)?;
    let edge = edge(name, "edge type", args, 3, Edge::Either)?;
    let found = edge_crossings(x, y, level, edge);
    found.get(wanted - 1).copied().ok_or_else(|| {
        math(format!(
            "the signal crosses {level} {}{} time(s), not {wanted}",
            edge_words(edge),
            found.len()
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
fn delay(name: &str, args: &[Argument]) -> Result<f64, EvaluationError> {
    arity(name, args, 2, 8)?;
    let (from_x, from_y) = timed(name, args, 0)?;
    let (to_x, to_y) = timed(name, args, 1)?;
    let (from_level, from_edge, from_n) = match number(name, "level", args, 2)? {
        Some(level) => (
            level,
            edge(name, "edge type", args, 3, Edge::Either)?,
            ordinal(name, "edge number", args, 4)?,
        ),
        None => (mid_level(name, from_y)?, Edge::Either, 1),
    };
    let to_level = match number(name, "second level", args, 5)? {
        Some(level) => level,
        None if args.len() > 2 => from_level,
        None => mid_level(name, to_y)?,
    };
    let to_edge = edge(name, "second edge type", args, 6, from_edge)?;
    let to_n = if args.len() > 7 {
        ordinal(name, "second edge number", args, 7)?
    } else {
        from_n
    };
    let first = edge_crossings(from_x, from_y, from_level, from_edge)
        .get(from_n - 1)
        .copied()
        .ok_or_else(|| {
            math(format!(
                "the first waveform has no {} {}crossing of {from_level}",
                nth(from_n),
                edge_words(from_edge)
            ))
        })?;
    let second = edge_crossings(to_x, to_y, to_level, to_edge)
        .get(to_n - 1)
        .copied()
        .ok_or_else(|| {
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

/// `(w[, theta1, theta2[, n[, initial, final]]])`, the arguments `rise`,
/// `fall` and `slewRate` share, resolved to the nth complete edge.
fn measured_edge(
    name: &str,
    args: &[Argument],
    sense: Sense,
) -> Result<MeasuredEdge, EvaluationError> {
    arity(name, args, 1, 6)?;
    if args.len() == 2 || args.len() == 5 {
        return Err(math(format!(
            "{name} takes its two thresholds together, and its initial and final levels together"
        )));
    }
    let (x, y) = timed(name, args, 0)?;
    let theta1 = number(name, "first threshold", args, 1)?.unwrap_or(10.0);
    let theta2 = number(name, "second threshold", args, 2)?.unwrap_or(90.0);
    if !(0.0..theta2).contains(&theta1) || theta2 > 100.0 {
        return Err(math(format!(
            "{name} thresholds are percentages of the transition with 0 <= first < second <= 100; \
             got {theta1} and {theta2}"
        )));
    }
    let wanted = ordinal(name, "edge number", args, 3)?;
    let (initial, last) = match (
        number(name, "initial level", args, 4)?,
        number(name, "final level", args, 5)?,
    ) {
        (Some(initial), Some(last)) => (initial, last),
        _ => {
            let (low, high) = extremes(name, y)?;
            if sense == Sense::Fall {
                (high, low)
            } else {
                (low, high)
            }
        }
    };
    if initial == last {
        return Err(math(format!(
            "{name} needs an edge; its initial and final levels are both {initial}"
        )));
    }
    let rising = last > initial;
    match (sense, rising) {
        (Sense::Rise, false) => {
            return Err(math(format!(
                "{name} measures a rising edge, so its final level must be above its initial level"
            )));
        }
        (Sense::Fall, true) => {
            return Err(math(format!(
                "{name} measures a falling edge, so its final level must be below its initial level"
            )));
        }
        _ => {}
    }
    let from = initial + theta1 / 100.0 * (last - initial);
    let to = initial + theta2 / 100.0 * (last - initial);
    let direction = if rising { Edge::Rising } else { Edge::Falling };
    // Walk both thresholds' crossings in time order. A first-threshold
    // crossing arms an edge (the latest one wins, so a runt that turned back
    // is forgotten); the next second-threshold crossing completes it.
    let mut events: Vec<(f64, bool)> = edge_crossings(x, y, from, direction)
        .into_iter()
        .map(|at| (at, true))
        .chain(
            edge_crossings(x, y, to, direction)
                .into_iter()
                .map(|at| (at, false)),
        )
        .collect();
    // At one instant — a vertical edge on duplicated timepoints — the
    // first threshold is crossed before the second.
    events.sort_by(|a, b| a.0.total_cmp(&b.0).then(b.1.cmp(&a.1)));
    let mut armed = None;
    let mut completed = 0;
    for (at, starts) in events {
        if starts {
            armed = Some(at);
        } else if let Some(start) = armed.take() {
            completed += 1;
            if completed == wanted {
                return Ok(MeasuredEdge {
                    start,
                    end: at,
                    from,
                    to,
                });
            }
        }
    }
    Err(math(format!(
        "{name} finds {completed} complete {} edge(s) from {from} to {to}, not {wanted}",
        direction.word()
    )))
}

/// `rise(w[, theta1, theta2[, n[, initial, final]]])` — the time the nth
/// rising edge takes from `theta1` to `theta2` percent of its transition
/// (default 10 % to 90 % of the waveform's own excursion).
fn rise(name: &str, args: &[Argument]) -> Result<f64, EvaluationError> {
    let edge = measured_edge(name, args, Sense::Rise)?;
    Ok(edge.end - edge.start)
}

/// `fall(w[, theta1, theta2[, n[, initial, final]]])` — as `rise`, on the
/// nth falling edge; the initial level defaults to the maximum.
fn fall(name: &str, args: &[Argument]) -> Result<f64, EvaluationError> {
    let edge = measured_edge(name, args, Sense::Fall)?;
    Ok(edge.end - edge.start)
}

/// `slewRate(w[, theta1, theta2[, n[, initial, final]]])` — the average
/// slope of the nth edge between its two thresholds, in units per unit x.
/// The edge runs from the initial level to the final one, so it is rising
/// unless the final level is given below the initial, and a falling edge
/// reports a negative slope.
fn slew_rate(name: &str, args: &[Argument]) -> Result<f64, EvaluationError> {
    let edge = measured_edge(name, args, Sense::Either)?;
    let span = edge.end - edge.start;
    if span <= 0.0 {
        return Err(math(format!(
            "{name} finds a vertical edge: it passes {} and {} at the same instant {}",
            edge.from, edge.to, edge.start
        )));
    }
    Ok((edge.to - edge.from) / span)
}

// =============================================================================
// Pulses and periods
// =============================================================================

/// `pulseWidth(w[, level[, polarity[, n]]])` — how long the nth `"high"`
/// pulse stays above `level` (from a rising crossing to the next falling
/// one), or the nth `"low"` pulse below it. The level defaults to the mid
/// level and the polarity to `"high"`. A pulse already under way when the
/// window opens is not counted, because its start is not in it.
fn pulse_width(name: &str, args: &[Argument]) -> Result<f64, EvaluationError> {
    arity(name, args, 1, 4)?;
    let (x, y) = timed(name, args, 0)?;
    let level = level_or_mid(name, args, 1, y)?;
    let high = keyword(name, "polarity", args, 2, &["high", "low"])? != Some("low");
    let wanted = ordinal(name, "pulse number", args, 3)?;
    let found = crossings(x, y, level);
    let starts: Vec<usize> = (0..found.len())
        .filter(|&index| found[index].rising == high)
        .collect();
    let polarity = if high { "high" } else { "low" };
    let Some(&start) = starts.get(wanted - 1) else {
        return Err(math(format!(
            "{name} finds {} {polarity} pulse(s) through {level}, not {wanted}",
            starts.len()
        )));
    };
    let end = found[start + 1..]
        .iter()
        .find(|crossing| crossing.rising != high)
        .ok_or_else(|| {
            math(format!(
                "the {} {polarity} pulse through {level} starts at {} and does not end in the window",
                nth(wanted),
                found[start].x
            ))
        })?;
    Ok(end.x - found[start].x)
}

/// `dutyCycle(w[, level])` — percent of each period spent above `level`
/// (default the mid level), averaged over every whole cycle the window
/// contains.
fn duty_cycle(name: &str, args: &[Argument]) -> Result<f64, EvaluationError> {
    arity(name, args, 1, 2)?;
    let (x, y) = timed(name, args, 0)?;
    let level = level_or_mid(name, args, 1, y)?;
    let found = crossings(x, y, level);
    let mut total = 0.0;
    let mut cycles = 0usize;
    for (index, start) in found.iter().enumerate() {
        if !start.rising {
            continue;
        }
        let rest = &found[index + 1..];
        let (Some(falls), Some(repeats)) = (
            rest.iter().find(|crossing| !crossing.rising),
            rest.iter().find(|crossing| crossing.rising),
        ) else {
            continue;
        };
        let cycle = repeats.x - start.x;
        if cycle <= 0.0 {
            continue;
        }
        total += (falls.x - start.x) / cycle;
        cycles += 1;
    }
    if cycles == 0 {
        return Err(math(format!(
            "{name} needs a whole cycle: a rising crossing of {level}, a fall, and the next rise"
        )));
    }
    Ok(100.0 * total / cycles as f64)
}

/// The period between rising crossings of the level at `args[2]` (default
/// the mid level): averaged over every whole cycle in the window, or over
/// the first `args[1]` cycles when the call gives a count.
///
/// Measured across the cycles rather than from one, so jitter averages out
/// instead of deciding the answer.
fn mean_period(name: &str, args: &[Argument]) -> Result<f64, EvaluationError> {
    arity(name, args, 1, 3)?;
    let (x, y) = timed(name, args, 0)?;
    let cycles = match args.get(1) {
        Some(_) => Some(ordinal(name, "cycle count", args, 1)?),
        None => None,
    };
    let level = level_or_mid(name, args, 2, y)?;
    let rising = edge_crossings(x, y, level, Edge::Rising);
    if rising.len() < 2 {
        return Err(math(format!(
            "{name} needs at least one whole period between rising crossings of {level}"
        )));
    }
    let counted = match cycles {
        Some(wanted) if wanted < rising.len() => wanted,
        Some(wanted) => {
            return Err(math(format!(
                "{name} finds {} whole cycle(s) between rising crossings of {level}, not {wanted}",
                rising.len() - 1
            )));
        }
        None => rising.len() - 1,
    };
    let span = rising[counted] - rising[0];
    if span <= 0.0 {
        return Err(math(format!(
            "{name} found no elapsed time between crossings of {level}"
        )));
    }
    Ok(span / counted as f64)
}

/// `period(w[, n[, level]])` — the mean period, over the first `n` cycles
/// when given.
fn period(name: &str, args: &[Argument]) -> Result<f64, EvaluationError> {
    mean_period(name, args)
}

/// `frequency(w[, n[, level]])` — the reciprocal of `period`.
fn frequency(name: &str, args: &[Argument]) -> Result<f64, EvaluationError> {
    Ok(1.0 / mean_period(name, args)?)
}

// =============================================================================
// Step response
// =============================================================================

/// The first and last finite samples: where a step response starts and
/// where it ends up.
fn step_levels(name: &str, y: &[f64]) -> Result<(f64, f64), EvaluationError> {
    let no_sample = || math(format!("{name} has no finite sample"));
    let initial = y
        .iter()
        .find(|v| v.is_finite())
        .copied()
        .ok_or_else(no_sample)?;
    let settled = y
        .iter()
        .rev()
        .find(|v| v.is_finite())
        .copied()
        .ok_or_else(no_sample)?;
    Ok((initial, settled))
}

/// The initial and final levels of a step: `args[first]` and
/// `args[first + 1]` when given, else the first and last finite samples.
fn step(
    name: &str,
    args: &[Argument],
    first: usize,
    y: &[f64],
) -> Result<(f64, f64), EvaluationError> {
    let (initial, settled) = match (
        number(name, "initial level", args, first)?,
        number(name, "final level", args, first + 1)?,
    ) {
        (Some(initial), Some(settled)) => (initial, settled),
        (None, None) => step_levels(name, y)?,
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
fn overshoot(name: &str, args: &[Argument]) -> Result<f64, EvaluationError> {
    arity(name, args, 1, 3)?;
    let (_, y) = timed(name, args, 0)?;
    let (initial, settled) = step(name, args, 1, y)?;
    let (low, high) = extremes(name, y)?;
    let step = settled - initial;
    let peak = if step > 0.0 { high } else { low };
    Ok((100.0 * (peak - settled) / step).max(0.0))
}

/// `undershoot(w[, initial, final])` — percent by which the response moved
/// the wrong way, past its initial value, relative to the step: the dip a
/// non-minimum-phase response takes before it rises, as MATLAB's `stepinfo`
/// reports it. 0 when it never moved the wrong way.
fn undershoot(name: &str, args: &[Argument]) -> Result<f64, EvaluationError> {
    arity(name, args, 1, 3)?;
    let (_, y) = timed(name, args, 0)?;
    let (initial, settled) = step(name, args, 1, y)?;
    let (low, high) = extremes(name, y)?;
    let step = settled - initial;
    let trough = if step > 0.0 { low } else { high };
    Ok((100.0 * (initial - trough) / step).max(0.0))
}

/// `settlingTime(w, band[, final[, start]])` — time from `start` (default
/// the start of the window) until `w` enters and stays inside `band`
/// percent of its step, measured about the final value. The final value
/// defaults to the last sample and the initial value is the signal's value
/// at `start`.
fn settling_time(name: &str, args: &[Argument]) -> Result<f64, EvaluationError> {
    arity(name, args, 2, 4)?;
    let (x, y) = timed(name, args, 0)?;
    // NaN is rejected by `number` before it can make the tolerance NaN, every
    // in-band test below false, and a settling time that was never measured.
    let band = number(name, "band", args, 1)?.unwrap_or(f64::NAN);
    if band <= 0.0 {
        return Err(math(format!(
            "{name} band must be a positive percentage of the step"
        )));
    }
    let start = number(name, "start", args, 3)?;
    let (x, y) = match start {
        None => (x.to_vec(), y.to_vec()),
        Some(start) => {
            if !(x[0]..=x[x.len() - 1]).contains(&start) {
                return Err(math(format!(
                    "{name} start {start} lies outside the window {} to {}",
                    x[0],
                    x[x.len() - 1]
                )));
            }
            let mut clipped_x = vec![start];
            let mut clipped_y = vec![interpolate(x, y, start)];
            for (at, value) in x.iter().zip(y).filter(|(at, _)| **at > start) {
                clipped_x.push(*at);
                clipped_y.push(*value);
            }
            (clipped_x, clipped_y)
        }
    };
    let (initial, last) = step_levels(name, &y)?;
    let settled = number(name, "final value", args, 2)?.unwrap_or(last);
    if initial == settled {
        return Err(math(format!(
            "{name} needs a step; the first and last samples are both {initial}"
        )));
    }
    let tolerance = (settled - initial).abs() * band / 100.0;
    // The *last* excursion outside the band is what settling means; an early
    // dip through it says nothing about when the response finally stayed put.
    let mut last_outside = None;
    for (index, value) in y.iter().enumerate() {
        if value.is_finite() && (value - settled).abs() > tolerance {
            last_outside = Some(index);
        }
    }
    let Some(outside) = last_outside else {
        return Ok(0.0);
    };
    if outside + 1 >= y.len() {
        return Err(math(format!(
            "the signal never settles inside {band} % of its step"
        )));
    }
    let (before, after) = (outside, outside + 1);
    if !y[after].is_finite() {
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
    let departure = y[before] - settled;
    let arrival = y[after] - settled;
    let band_edge = if departure > 0.0 {
        tolerance
    } else {
        -tolerance
    };
    let travel = arrival - departure;
    // |departure| > tolerance >= |arrival|, so the two deviations differ and
    // this cannot divide by zero; a duplicated timepoint gives x[before].
    let entry = x[before] + (band_edge - departure) * (x[after] - x[before]) / travel;
    Ok(entry - x[0])
}

#[cfg(test)]
mod tests;
