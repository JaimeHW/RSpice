//! The jitter of a clock: `abs_jitter` (the time interval error),
//! `period_jitter`, `cc_jitter` (cycle to cycle) and `longterm_jitter`.
//!
//! The edges are the clock's crossings of a threshold going one way, found
//! as ViVA finds them: the threshold defaults to the trace's mean level (its
//! integral over its x range), and a crossing counts only once the signal
//! has gone on to the threshold plus or minus a tolerance of a tenth of the
//! standard deviation of its samples, so noise that rattles across the
//! threshold on one edge makes one crossing, not several. Each reading is a
//! trace of one value per cycle against the time it completes (`"plot"`,
//! ViVA's default) or the standard deviation of those values (`"sd"`).
//!
//! - `abs_jitter`: each edge's time less the time of the same edge of an
//!   ideal clock of the nominal period, started at the first edge, as ViVA's
//!   `abs_jitter` reads it. The nominal period defaults to
//!   (last edge - first edge) / (edges - 1).
//! - `period_jitter`: each period less the mean period, as ViVA's
//!   `period_jitter` reads it with all periods averaged.
//! - `cc_jitter`: each period less the one before it.
//! - `longterm_jitter`: the time n periods take, less n mean periods.
//!
//! Option words may stand anywhere after the trace; the numbers are read in
//! their order.

use super::evaluator::EvaluationError;
use super::measures::{Argument, Edge, Reduction};
use super::series::Series;
use super::value::{CalcValue, RealValue};

/// The reduction for `name`, already lower-cased.
pub(super) fn lookup(name: &str) -> Option<Reduction> {
    Some(match name {
        "abs_jitter" => absolute,
        "period_jitter" => period,
        "cc_jitter" => cycle_to_cycle,
        "longterm_jitter" => long_term,
        _ => return None,
    })
}

/// A call's options: the edge, the output, and its numbers in order.
struct Options {
    edge: Edge,
    deviation: bool,
    numbers: Vec<f64>,
}

fn options(name: &str, rest: &[Argument]) -> Result<Options, EvaluationError> {
    let mut found = Options {
        edge: Edge::Rising,
        deviation: false,
        numbers: Vec::new(),
    };
    for argument in rest {
        match argument {
            Argument::Keyword(word) => match word.as_str() {
                "rising" => found.edge = Edge::Rising,
                "falling" => found.edge = Edge::Falling,
                "plot" => found.deviation = false,
                "sd" => found.deviation = true,
                other => {
                    return Err(EvaluationError::TypeMismatch(format!(
                        "{name} reads \"rising\" or \"falling\" and \"plot\" or \"sd\", not \"{other}\""
                    )));
                }
            },
            Argument::Value(CalcValue::Real(RealValue::Scalar(value))) if value.is_finite() => {
                found.numbers.push(*value);
            }
            Argument::Value(_) => {
                return Err(EvaluationError::TypeMismatch(format!(
                    "{name} reads finite numbers after its trace"
                )));
            }
        }
    }
    Ok(found)
}

/// The clock's edges through `threshold` (default its mean level) going
/// `edge`'s way, each confirmed by the signal reaching the threshold plus or
/// minus a tenth of its samples' standard deviation beyond it.
fn edges(
    name: &str,
    series: &Series,
    edge: Edge,
    threshold: Option<f64>,
) -> Result<Vec<f64>, EvaluationError> {
    // One pass: the axis runs forward and every sample is defined; the
    // integral for the mean level, and the samples' spread.
    let (mut count, mut mean, mut squares) = (0usize, 0.0, 0.0);
    let (mut area, mut span) = (0.0, 0.0);
    let mut previous: Option<(f64, f64)> = None;
    for (x, y) in series.pairs() {
        if !x.is_finite() || !y.is_finite() || previous.is_some_and(|(x0, _)| x < x0) {
            return Err(EvaluationError::MathError(format!(
                "{name} needs a defined trace whose time axis runs forward"
            )));
        }
        count += 1;
        let delta = y - mean;
        mean += delta / count as f64;
        squares += delta * (y - mean);
        if let Some((x0, y0)) = previous {
            area += (x - x0) * (y0 + y) / 2.0;
            span += x - x0;
        }
        previous = Some((x, y));
    }
    if span <= 0.0 {
        return Err(EvaluationError::MathError(format!(
            "{name} needs a trace that spans some time"
        )));
    }
    let threshold = threshold.unwrap_or(area / span);
    let tolerance = 0.1 * (squares / count as f64).sqrt();
    let rising = edge == Edge::Rising;
    // Signed so a rising edge goes from below to above: the distance past
    // the threshold the way the edge goes.
    let past = |y: f64| if rising { y - threshold } else { threshold - y };
    let mut found = Vec::new();
    let mut armed = false;
    let mut candidate: Option<f64> = None;
    let mut previous: Option<(f64, f64)> = None;
    for (x, y) in series.pairs() {
        if past(y) <= -tolerance {
            armed = true;
            candidate = None;
        }
        if let Some((x0, y0)) = previous
            && armed
            && past(y0) < 0.0
            && past(y) >= 0.0
        {
            candidate = Some(x0 + (threshold - y0) * (x - x0) / (y - y0));
        }
        if armed && past(y) >= tolerance && past(y) > 0.0 {
            if let Some(at) = candidate.take() {
                found.push(at);
            }
            armed = false;
        }
        previous = Some((x, y));
    }
    Ok(found)
}

/// The output: the values against the times they complete, or their
/// standard deviation.
fn output(
    name: &str,
    deviation: bool,
    at: Vec<f64>,
    values: Vec<f64>,
) -> Result<RealValue, EvaluationError> {
    if values.is_empty() {
        return Err(EvaluationError::MathError(format!(
            "{name} finds too few edges to read"
        )));
    }
    if !deviation {
        return Ok(RealValue::Waveform(at, values));
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values
        .iter()
        .map(|value| (value - mean) * (value - mean))
        .sum::<f64>()
        / values.len() as f64;
    Ok(RealValue::Scalar(variance.sqrt()))
}

fn enough(name: &str, edges: &[f64], least: usize) -> Result<(), EvaluationError> {
    if edges.len() < least {
        return Err(EvaluationError::MathError(format!(
            "{name} needs at least {least} edges; the trace has {}",
            edges.len()
        )));
    }
    Ok(())
}

/// The mean period over every edge.
fn mean_period(edges: &[f64]) -> f64 {
    (edges[edges.len() - 1] - edges[0]) / (edges.len() - 1) as f64
}

/// `abs_jitter(x, edge, output, threshold, tnom)`.
fn absolute(name: &str, series: &Series, rest: &[Argument]) -> Result<RealValue, EvaluationError> {
    let options = options(name, rest)?;
    let found = edges(name, series, options.edge, options.numbers.first().copied())?;
    enough(name, &found, 2)?;
    let nominal = match options.numbers.get(1) {
        Some(&nominal) if nominal > 0.0 => nominal,
        Some(&nominal) => {
            return Err(EvaluationError::MathError(format!(
                "{name} nominal period must be positive, not {nominal}"
            )));
        }
        None => mean_period(&found),
    };
    let values = found
        .iter()
        .enumerate()
        .map(|(k, &at)| at - (found[0] + k as f64 * nominal))
        .collect();
    output(name, options.deviation, found, values)
}

/// `period_jitter(x, edge, output, threshold)`.
fn period(name: &str, series: &Series, rest: &[Argument]) -> Result<RealValue, EvaluationError> {
    let options = options(name, rest)?;
    let found = edges(name, series, options.edge, options.numbers.first().copied())?;
    enough(name, &found, 2)?;
    let mean = mean_period(&found);
    let values = found
        .windows(2)
        .map(|pair| pair[1] - pair[0] - mean)
        .collect();
    output(name, options.deviation, found[1..].to_vec(), values)
}

/// `cc_jitter(x, edge, output, threshold)`.
fn cycle_to_cycle(
    name: &str,
    series: &Series,
    rest: &[Argument],
) -> Result<RealValue, EvaluationError> {
    let options = options(name, rest)?;
    let found = edges(name, series, options.edge, options.numbers.first().copied())?;
    enough(name, &found, 3)?;
    let values = found
        .windows(3)
        .map(|run| (run[2] - run[1]) - (run[1] - run[0]))
        .collect();
    output(name, options.deviation, found[2..].to_vec(), values)
}

/// `longterm_jitter(x, n, edge, output, threshold)`.
fn long_term(name: &str, series: &Series, rest: &[Argument]) -> Result<RealValue, EvaluationError> {
    let options = options(name, rest)?;
    let cycles = match options.numbers.first() {
        Some(&cycles) if cycles >= 1.0 && cycles.fract() == 0.0 && cycles <= u32::MAX as f64 => {
            cycles as usize
        }
        _ => {
            return Err(EvaluationError::MathError(format!(
                "{name} needs the whole number of periods, 1 or more, it reads across"
            )));
        }
    };
    let found = edges(name, series, options.edge, options.numbers.get(1).copied())?;
    enough(name, &found, cycles + 1)?;
    let mean = mean_period(&found);
    let values = found
        .windows(cycles + 1)
        .map(|run| run[cycles] - run[0] - cycles as f64 * mean)
        .collect();
    output(name, options.deviation, found[cycles..].to_vec(), values)
}

#[cfg(test)]
mod tests;
