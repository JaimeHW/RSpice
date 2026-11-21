//! The statistics of a signal's values: `stddev`, `variance`, `median`,
//! `percentile`, `skewness` and `kurtosis`.
//!
//! A signal is read as the trace it draws, each value weighted by the span of
//! the x-axis it holds, as `avg` and `rms` weigh them, so a densely solved
//! edge counts for its duration and not for its sample count. The moments of
//! a straight segment are integrated exactly, so they are exact for the
//! piecewise-linear trace on any grid. `stddev` and `variance` also read the
//! samples alone, unweighted, as a statistical sample divided by N − 1, as
//! ViVA's `?type "sample"` does.
//!
//! Every pass reads the samples through [`Series`], so a stored trace is
//! read a block at a time, however long.

use super::evaluator::EvaluationError;
use super::measures::{Argument, keyword, number};
use super::series::Series;
use super::value::RealValue;

/// The reduction for `name`, already lower-cased.
pub(super) fn lookup(name: &str) -> Option<super::measures::Reduction> {
    Some(match name {
        "stddev" => stddev,
        "variance" => variance,
        "median" => median,
        "percentile" => percentile,
        "skewness" => skewness,
        "kurtosis" => kurtosis,
        _ => return None,
    })
}

/// The moments of a trace about its mean: its x-span, mean, and the second,
/// third and fourth central moments per unit of x.
struct Moments {
    mean: f64,
    second: f64,
    third: f64,
    fourth: f64,
}

/// The trace's checks, made in the first pass: an x-axis that runs one way
/// with some width, and every value defined.
fn first_pass(name: &str, series: &Series) -> Result<(f64, f64), EvaluationError> {
    let (mut span, mut area) = (0.0, 0.0);
    let (mut rises, mut falls) = (false, false);
    let mut previous: Option<(f64, f64)> = None;
    let mut count = 0usize;
    for (x, y) in series.pairs() {
        count += 1;
        if !x.is_finite() {
            return Err(EvaluationError::MathError(format!(
                "{name} reads a trace whose x-axis has an undefined sample"
            )));
        }
        if !y.is_finite() {
            return Err(EvaluationError::MathError(format!(
                "{name} reads a trace with undefined samples; clip() it to where it is defined"
            )));
        }
        if let Some((x0, y0)) = previous {
            rises |= x > x0;
            falls |= x < x0;
            let dx = (x - x0).abs();
            span += dx;
            area += dx * (y0 + y) / 2.0;
        }
        previous = Some((x, y));
    }
    if rises && falls {
        return Err(EvaluationError::MathError(format!(
            "{name} needs an x-axis that runs one way; this sweep turns back"
        )));
    }
    if count == 0 || span == 0.0 {
        return Err(EvaluationError::MathError(format!(
            "{name} needs a trace that spans some of its x-axis"
        )));
    }
    Ok((span, area / span))
}

/// The central moments of the trace, each segment integrated exactly:
/// over a segment from `a` to `b` about the mean, ∫(y − μ)ᵏ dx is
/// dx · (aᵏ + aᵏ⁻¹b + … + bᵏ) / (k + 1).
fn moments(name: &str, series: &Series) -> Result<Moments, EvaluationError> {
    let (span, mean) = first_pass(name, series)?;
    let (mut second, mut third, mut fourth) = (0.0, 0.0, 0.0);
    let mut previous: Option<(f64, f64)> = None;
    for (x, y) in series.pairs() {
        let b = y - mean;
        if let Some((x0, a)) = previous {
            let dx = (x - x0).abs();
            let (a2, b2) = (a * a, b * b);
            second += dx * (a2 + a * b + b2) / 3.0;
            third += dx * (a2 * a + a2 * b + a * b2 + b2 * b) / 4.0;
            fourth += dx * (a2 * a2 + a2 * a * b + a2 * b2 + a * b2 * b + b2 * b2) / 5.0;
        }
        previous = Some((x, b));
    }
    Ok(Moments {
        mean,
        second: second / span,
        third: third / span,
        fourth: fourth / span,
    })
}

/// Whether a `stddev` or `variance` call reads the samples as a statistical
/// sample: its second argument, `"population"` (the default) or `"sample"`.
fn reads_a_sample(name: &str, rest: &[Argument]) -> Result<bool, EvaluationError> {
    Ok(keyword(name, "type", rest, 0, &["population", "sample"])? == Some("sample"))
}

/// The unweighted variance of the samples, divided by N − 1.
fn sample_variance(name: &str, series: &Series) -> Result<f64, EvaluationError> {
    let (mut count, mut sum) = (0usize, 0.0);
    for (_, y) in series.pairs() {
        if !y.is_finite() {
            return Err(EvaluationError::MathError(format!(
                "{name} reads a trace with undefined samples; clip() it to where it is defined"
            )));
        }
        count += 1;
        sum += y;
    }
    if count < 2 {
        return Err(EvaluationError::MathError(format!(
            "{name} of a sample needs at least two samples"
        )));
    }
    let mean = sum / count as f64;
    let squares: f64 = series.pairs().map(|(_, y)| (y - mean) * (y - mean)).sum();
    Ok(squares / (count - 1) as f64)
}

fn variance_of(name: &str, series: &Series, rest: &[Argument]) -> Result<f64, EvaluationError> {
    if reads_a_sample(name, rest)? {
        sample_variance(name, series)
    } else {
        moments(name, series).map(|moments| moments.second)
    }
}

/// `stddev(x[, type])` — the standard deviation of the signal's values.
fn stddev(name: &str, series: &Series, rest: &[Argument]) -> Result<RealValue, EvaluationError> {
    variance_of(name, series, rest).map(|variance| RealValue::Scalar(variance.sqrt()))
}

/// `variance(x[, type])` — the variance of the signal's values.
fn variance(name: &str, series: &Series, rest: &[Argument]) -> Result<RealValue, EvaluationError> {
    variance_of(name, series, rest).map(RealValue::Scalar)
}

/// The standardized moment `order` (3 or 4), refused for a constant trace,
/// which has no spread to standardize by.
fn standardized(name: &str, series: &Series, order: i32) -> Result<f64, EvaluationError> {
    let moments = moments(name, series)?;
    if moments.second <= f64::EPSILON * f64::EPSILON * moments.mean.abs().max(f64::MIN_POSITIVE) {
        return Err(EvaluationError::MathError(format!(
            "{name} of a constant trace is undefined: it has no spread"
        )));
    }
    let central = if order == 3 {
        moments.third
    } else {
        moments.fourth
    };
    Ok(central / moments.second.powf(f64::from(order) / 2.0))
}

/// `skewness(x)` — the third standardized moment: 0 for a symmetric
/// signal, positive when its values spread further above the mean.
fn skewness(name: &str, series: &Series, _: &[Argument]) -> Result<RealValue, EvaluationError> {
    standardized(name, series, 3).map(RealValue::Scalar)
}

/// `kurtosis(x)` — the fourth standardized moment, Pearson's: 3 for a
/// Gaussian, 1.5 for a sine, 1.8 for a ramp.
fn kurtosis(name: &str, series: &Series, _: &[Argument]) -> Result<RealValue, EvaluationError> {
    standardized(name, series, 4).map(RealValue::Scalar)
}

/// The share of the trace's x-span over which its value is at most `level`:
/// a segment's share by where `level` cuts it, a flat one whole or not at
/// all.
fn share_at_most(series: &Series, level: f64) -> f64 {
    let (mut below, mut span) = (0.0, 0.0);
    let mut previous: Option<(f64, f64)> = None;
    for (x, y) in series.pairs() {
        if let Some((x0, y0)) = previous {
            let dx = (x - x0).abs();
            span += dx;
            let (low, high) = (y0.min(y), y0.max(y));
            below += if high == low {
                if low <= level { dx } else { 0.0 }
            } else {
                dx * ((level - low) / (high - low)).clamp(0.0, 1.0)
            };
        }
        previous = Some((x, y));
    }
    below / span
}

/// The least level the trace is at or below for `share` of its x-span,
/// found by bisection between its extremes.
fn quantile(name: &str, series: &Series, share: f64) -> Result<f64, EvaluationError> {
    first_pass(name, series)?;
    let (mut low, mut high) = series
        .pairs()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), (_, y)| {
            (low.min(y), high.max(y))
        });
    if share_at_most(series, low) >= share {
        return Ok(low);
    }
    loop {
        let middle = low + (high - low) / 2.0;
        if middle <= low || middle >= high {
            return Ok(high);
        }
        if share_at_most(series, middle) >= share {
            high = middle;
        } else {
            low = middle;
        }
    }
}

/// `median(x)` — the level the signal is at or below for half its x-span.
fn median(name: &str, series: &Series, _: &[Argument]) -> Result<RealValue, EvaluationError> {
    quantile(name, series, 0.5).map(RealValue::Scalar)
}

/// `percentile(x, p)` — the level the signal is at or below for `p` percent
/// of its x-span.
fn percentile(
    name: &str,
    series: &Series,
    rest: &[Argument],
) -> Result<RealValue, EvaluationError> {
    let Some(percent) = number(name, "percentage", rest, 0)? else {
        return Err(EvaluationError::MathError(format!(
            "{name} needs the percentage of the trace to read the level of"
        )));
    };
    if !(0.0..=100.0).contains(&percent) {
        return Err(EvaluationError::MathError(format!(
            "{name} takes a percentage from 0 to 100, not {percent}"
        )));
    }
    quantile(name, series, percent / 100.0).map(RealValue::Scalar)
}

#[cfg(test)]
mod tests;
