//! The level measures of a stored trace, `avg`, `rms`, `min`, `max`, `pp` and `yval`, read
//! through [`Series`] a block at a time: the answers the functions give the same samples in
//! memory, from the same interval statistics and the same interpolation.

use super::evaluator::EvaluationError;
use super::measures::{Argument, Reduction};
use super::series::Series;
use super::value::RealValue;

/// The level measure `name` (lower case) of a stored trace, if it is one.
pub(super) fn lookup(name: &str) -> Option<Reduction> {
    Some(match name {
        "avg" | "average" => avg,
        "rms" => rms,
        "min" => min,
        "max" => max,
        "pp" => pp,
        "yval" => yval,
        _ => return None,
    })
}

/// A level measure's window after the trace: none reads the whole trace, `(from, to)` the part
/// of it between two points of the x-axis, its ends interpolated on the trace as drawn.
fn window(name: &str, rest: &[Argument]) -> Result<Option<(f64, f64)>, EvaluationError> {
    no_words(name, rest)?;
    match rest.len() {
        0 => Ok(None),
        2 => Ok(Some((
            scalar(name, "window start", &rest[0])?,
            scalar(name, "window end", &rest[1])?,
        ))),
        1 => Err(EvaluationError::MathError(format!(
            "{name} reads a window between two points; give both its start and its end"
        ))),
        count => Err(EvaluationError::ArgCountMismatch {
            func: name.to_owned(),
            expected: 3,
            actual: count + 1,
        }),
    }
}

/// Refuse an option word: a level measure takes none.
fn no_words(name: &str, rest: &[Argument]) -> Result<(), EvaluationError> {
    match rest.iter().find_map(|argument| match argument {
        Argument::Keyword(word) => Some(word),
        Argument::Value(_) => None,
    }) {
        Some(word) => Err(EvaluationError::TypeMismatch(format!(
            "{name} takes no option words; \"{word}\" is not one of its arguments"
        ))),
        None => Ok(()),
    }
}

fn scalar(name: &str, what: &str, argument: &Argument) -> Result<f64, EvaluationError> {
    match argument {
        Argument::Value(super::value::CalcValue::Real(RealValue::Scalar(value))) => Ok(*value),
        _ => Err(EvaluationError::TypeMismatch(format!(
            "{name} {what} must be a scalar"
        ))),
    }
}

/// The trace's statistics over `window`, as `measure_interval` takes them of the same samples
/// in memory.
fn statistics(
    name: &str,
    trace: &Series,
    window: Option<(f64, f64)>,
) -> Result<crate::measurements::IntervalStatistics, EvaluationError> {
    let measured = match trace.ends() {
        Some(ends) => crate::measurements::measure_pairs(ends, || trace.pairs(), window),
        None => Err(crate::measurements::MeasurementError::Empty),
    };
    measured.map_err(|error| EvaluationError::MathError(format!("{name}: {error}")))
}

fn avg(_: &str, trace: &Series, rest: &[Argument]) -> Result<RealValue, EvaluationError> {
    let name = "avg";
    let window = window(name, rest)?;
    Ok(RealValue::Scalar(statistics(name, trace, window)?.mean))
}

fn rms(_: &str, trace: &Series, rest: &[Argument]) -> Result<RealValue, EvaluationError> {
    let name = "rms";
    let window = window(name, rest)?;
    Ok(RealValue::Scalar(statistics(name, trace, window)?.rms))
}

/// The lowest and highest value over a level measure's window: the samples of a whole trace,
/// holes skipped, or a window's samples and its interpolated ends.
fn extremes(name: &str, trace: &Series, rest: &[Argument]) -> Result<(f64, f64), EvaluationError> {
    if let Some(window) = window(name, rest)? {
        let measured = statistics(name, trace, Some(window))?;
        return Ok((measured.min, measured.max));
    }
    let (mut low, mut high) = (f64::INFINITY, f64::NEG_INFINITY);
    for (_, value) in trace.pairs().filter(|(_, value)| value.is_finite()) {
        low = low.min(value);
        high = high.max(value);
    }
    if low.is_finite() {
        Ok((low, high))
    } else {
        Err(EvaluationError::MathError(format!(
            "{name} has no finite sample to measure"
        )))
    }
}

fn min(_: &str, trace: &Series, rest: &[Argument]) -> Result<RealValue, EvaluationError> {
    Ok(RealValue::Scalar(extremes("min", trace, rest)?.0))
}

fn max(_: &str, trace: &Series, rest: &[Argument]) -> Result<RealValue, EvaluationError> {
    Ok(RealValue::Scalar(extremes("max", trace, rest)?.1))
}

/// `pp(w)`: the peak-to-peak excursion.
fn pp(_: &str, trace: &Series, rest: &[Argument]) -> Result<RealValue, EvaluationError> {
    let (low, high) = extremes("pp", trace, rest)?;
    Ok(RealValue::Scalar(high - low))
}

/// `yval(w, at)`: the value of `w` where its x-axis reads `at`, refused outside the sweep.
fn yval(_: &str, trace: &Series, rest: &[Argument]) -> Result<RealValue, EvaluationError> {
    let name = "yval";
    no_words(name, rest)?;
    if rest.len() != 1 {
        return Err(EvaluationError::ArgCountMismatch {
            func: name.to_owned(),
            expected: 2,
            actual: rest.len() + 1,
        });
    }
    let at = scalar(name, "position", &rest[0])?;
    let (first, last) = trace.ends().ok_or_else(|| {
        EvaluationError::MathError(format!("{name} needs a waveform with samples"))
    })?;
    let (low, high) = (first.min(last), first.max(last));
    if !(at >= low && at <= high) {
        return Err(EvaluationError::MathError(format!(
            "yval position {at} lies outside the swept range {low} to {high}"
        )));
    }
    let value = trace.value_at(at).ok_or_else(|| {
        EvaluationError::MathError(format!("{name} needs a waveform with samples"))
    })?;
    Ok(RealValue::Scalar(value))
}
