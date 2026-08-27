//! Calculator function library — the math, signal-processing and measurement
//! functions an expression can call, over scalars and real-valued series.
//!
//! # One domain policy, everywhere
//!
//! A sample outside a function's domain — `sqrt` of a negative, `ln` or
//! `log10` of a non-positive, `dB` of zero — becomes a **hole**: `NaN` at
//! that sample, with the rest of the series intact. The plot renderer breaks
//! its stroke at a hole, so the gap is *visible* rather than fabricated, and
//! one bad sample never costs the whole trace.
//!
//! The same operation on a **scalar** is an error. A scalar readout has no
//! neighbours for a hole to show up against, and a printed `NaN` reads as a
//! value; refusing is the only honest answer.
//!
//! # Measurement conventions
//!
//! - `avg` and `rms` integrate over the x-window and divide by its width, so
//!   they are correct on the non-uniform grids a transient run produces.
//!   A sample-count mean silently weights a densely-solved edge as heavily as
//!   a long quiet stretch.
//! - Percentages (`duty`, `overshoot`, `thd`) are returned in percent, and
//!   the levels a periodic measurement keys off are the mid level,
//!   `(min + max) / 2`.
//! - Nothing here extrapolates. A measurement that would need a sample
//!   outside the swept range is an error.
//!
//! # Scope
//!
//! Everything here is real-valued, because [`CalcValue`] carries a real
//! `(x, y)` pair and nothing else. Complex/AC operators — `mag`, `phase`,
//! `re`, `im` — are deliberately absent: there is no complex datum in the
//! calculator's value model to take a magnitude of, and inventing one from
//! the magnitude series a strip already holds would answer a different
//! question than the one asked.

use super::evaluator::{CalcValue, EvaluationError};
use std::f64::consts::PI;

/// Registry of built-in functions
pub struct FunctionRegistry;

impl FunctionRegistry {
    pub fn dispatch(name: &str, args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
        match name.to_lowercase().as_str() {
            // Math
            "abs" => abs(args),
            "sqrt" => sqrt(args),
            "exp" => exp(args),
            "log" | "ln" => ln(args),
            "log10" => log10(args),
            "db" => db(args),

            // Signal processing (series in, series out)
            "deriv" | "derivative" => deriv(args),
            "integ" | "integral" => integ(args),
            "clip" => clip(args),
            "unwrap" => unwrap(args),
            "xval" => xval(args),

            // Measurements (series in, scalar out)
            "avg" | "average" => avg(args),
            "rms" => rms(args),
            "min" => min(args),
            "max" => max(args),
            "pp" => pp(args),
            "yval" => yval(args),
            "cross" => cross(args),
            "freq" => freq(args),
            "period" => period(args),
            "duty" => duty(args),
            "overshoot" => overshoot(args),
            "rise" | "risetime" => rise(args),
            "fall" | "falltime" => fall(args),
            "settling" => settling(args),
            "delay" => delay(args),
            "thd" => thd(args),

            _ => Err(EvaluationError::UnknownFunction(name.to_string())),
        }
    }
}

// =============================================================================
// Implementations
// =============================================================================

fn abs(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("abs", &args, 1)?;
    match &args[0] {
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(s.abs())),
        CalcValue::Waveform(x, y) => Ok(CalcValue::create_waveform(
            x.clone(),
            y.iter().map(|v| v.abs()).collect(),
        )),
    }
}

fn sqrt(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("sqrt", &args, 1)?;
    match &args[0] {
        CalcValue::Scalar(s) if *s < 0.0 => Err(domain_error("sqrt", "a negative number")),
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(s.sqrt())),
        CalcValue::Waveform(x, y) => Ok(map_domain(x, y, f64::sqrt)),
    }
}

fn exp(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("exp", &args, 1)?;
    match &args[0] {
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(s.exp())),
        CalcValue::Waveform(x, y) => Ok(CalcValue::create_waveform(
            x.clone(),
            y.iter().map(|v| v.exp()).collect(),
        )),
    }
}

fn ln(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("ln", &args, 1)?;
    match &args[0] {
        CalcValue::Scalar(s) if *s <= 0.0 => Err(domain_error("ln", "a non-positive number")),
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(s.ln())),
        CalcValue::Waveform(x, y) => Ok(map_domain(x, y, f64::ln)),
    }
}

fn log10(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("log10", &args, 1)?;
    match &args[0] {
        CalcValue::Scalar(s) if *s <= 0.0 => Err(domain_error("log10", "a non-positive number")),
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(s.log10())),
        CalcValue::Waveform(x, y) => Ok(map_domain(x, y, f64::log10)),
    }
}

/// Voltage decibels: `20·log₁₀|x|`.
///
/// The magnitude is what is measured, so a negative sample reads as its
/// absolute value — the sign of a ratio is a phase statement, and phase is
/// not what a dB axis carries.
fn decibels(value: f64) -> f64 {
    20.0 * value.abs().log10()
}

fn db(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("dB", &args, 1)?;
    match &args[0] {
        CalcValue::Scalar(s) if *s == 0.0 => Err(domain_error("dB", "zero")),
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(decibels(*s))),
        CalcValue::Waveform(x, y) => Ok(map_domain(x, y, decibels)),
    }
}

// --- Aggregates over the x-window (Waveform -> Scalar) ---

/// The trapezoidal rule over a possibly non-uniform grid.
///
/// Exact for a piecewise-linear integrand, which is what a solver's retained
/// samples represent, and correct across the duplicated timepoints SPICE
/// emits at a vertical edge — a zero-width panel contributes zero area.
fn trapezoid(x: &[f64], y: &[f64], f: impl Fn(f64) -> f64) -> f64 {
    let mut area = 0.0;
    for i in 1..x.len() {
        area += (f(y[i]) + f(y[i - 1])) * 0.5 * (x[i] - x[i - 1]);
    }
    area
}

/// As [`trapezoid`], for an integrand that also reads the domain.
fn trapezoid_xy(x: &[f64], y: &[f64], f: impl Fn(f64, f64) -> f64) -> f64 {
    let mut area = 0.0;
    for i in 1..x.len() {
        area += (f(x[i], y[i]) + f(x[i - 1], y[i - 1])) * 0.5 * (x[i] - x[i - 1]);
    }
    area
}

/// Mean of `f(y)` over the x-window: `∫f(y)dx ÷ (x_last − x_first)`.
///
/// A degenerate window — every sample at the same x, or a single sample —
/// has no width to divide by, so it falls back to the arithmetic mean, which
/// is the limit of the window mean as the window closes.
///
/// A **hole refuses the whole measurement**. The domain policy above leaves
/// `NaN` where a sample fell outside a function's domain, and an integral
/// across a missing region is not the mean it would be printed as: the panel
/// has nowhere to show "…except between 1.2 s and 1.4 s". Skipping the holes
/// would answer a different question — the mean of what survived — and
/// letting the `NaN` through reads as a value, so the honest answer is the
/// error. Both `avg` and `rms` go through here, which is why the check lives
/// here and not in either of them.
fn window_mean(
    name: &str,
    x: &[f64],
    y: &[f64],
    f: impl Fn(f64) -> f64,
) -> Result<f64, EvaluationError> {
    if x.is_empty() || x.len() != y.len() {
        return Err(EvaluationError::MathError(format!(
            "{name} needs a waveform with samples"
        )));
    }
    if let Some(index) = y.iter().position(|v| !v.is_finite()) {
        return Err(EvaluationError::MathError(format!(
            "{name}: the series has undefined samples in the window (holes left where the \
             math went out of domain), the first at x = {}",
            x[index]
        )));
    }
    if x.iter().any(|v| !v.is_finite()) {
        return Err(EvaluationError::MathError(format!(
            "{name}: the series has a non-finite position on its x-axis, so the window it \
             would be averaged over has no width"
        )));
    }
    let span = x[x.len() - 1] - x[0];
    if span == 0.0 {
        return Ok(y.iter().map(|v| f(*v)).sum::<f64>() / y.len() as f64);
    }
    Ok(trapezoid(x, y, f) / span)
}

fn avg(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("avg", &args, 1)?;
    match &args[0] {
        // A DC value is its own mean.
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(*s)),
        CalcValue::Waveform(x, y) => Ok(CalcValue::Scalar(window_mean("avg", x, y, |v| v)?)),
    }
}

fn rms(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("rms", &args, 1)?;
    match &args[0] {
        // The RMS of a DC value is its magnitude, not the signed value.
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(s.abs())),
        // The mean of squares is non-negative by construction; the clamp
        // catches a rounding excursion just below zero on a near-degenerate
        // window and nothing else. It is deliberately *not* a NaN guard —
        // `f64::max` ignores a NaN, so using it that way would turn a hole
        // into a confident "0". `window_mean` refuses holes instead.
        CalcValue::Waveform(x, y) => Ok(CalcValue::Scalar(
            window_mean("rms", x, y, |v| v * v)?.max(0.0).sqrt(),
        )),
    }
}

// --- Transformations (Waveform -> Waveform) ---

/// One-sided difference from `at`, skipping neighbours that repeat `x[at]`.
///
/// SPICE emits a duplicated timepoint at every vertical edge; dividing by
/// that zero width is how a derivative becomes an infinity. Reaching past
/// the repeat to the nearest distinct x gives the real one-sided slope, and
/// only a wholly degenerate series — every neighbour at the same x — has no
/// slope to report and yields a hole.
fn one_sided(x: &[f64], y: &[f64], at: usize, forward: bool) -> f64 {
    let mut index = at;
    loop {
        index = if forward {
            if index + 1 < x.len() {
                index + 1
            } else {
                return f64::NAN;
            }
        } else {
            match index.checked_sub(1) {
                Some(previous) => previous,
                None => return f64::NAN,
            }
        };
        let dx = x[index] - x[at];
        if dx != 0.0 {
            return (y[index] - y[at]) / dx;
        }
    }
}

/// Central difference across `at`, or a hole where the stencil has no width.
fn central(x: &[f64], y: &[f64], at: usize) -> f64 {
    let dx = x[at + 1] - x[at - 1];
    if dx == 0.0 {
        f64::NAN
    } else {
        (y[at + 1] - y[at - 1]) / dx
    }
}

fn deriv(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("deriv", &args, 1)?;
    match &args[0] {
        // The derivative of a constant is zero.
        CalcValue::Scalar(_) => Ok(CalcValue::Scalar(0.0)),
        CalcValue::Waveform(x, y) => {
            if x.len() < 2 || x.len() != y.len() {
                return Err(EvaluationError::MathError(
                    "deriv needs at least two samples".to_owned(),
                ));
            }
            let last = x.len() - 1;
            let slopes = (0..=last)
                .map(|i| {
                    if i == 0 {
                        one_sided(x, y, 0, true)
                    } else if i == last {
                        one_sided(x, y, last, false)
                    } else {
                        central(x, y, i)
                    }
                })
                .collect();
            Ok(CalcValue::create_waveform(x.clone(), slopes))
        }
    }
}

fn integ(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("integ", &args, 1)?;
    match &args[0] {
        CalcValue::Scalar(_) => Err(EvaluationError::MathError(
            "integ needs a waveform: a scalar carries no domain to integrate over".to_owned(),
        )),
        CalcValue::Waveform(x, y) => {
            if x.len() < 2 || x.len() != y.len() {
                return Err(EvaluationError::MathError(
                    "integ needs at least two samples".to_owned(),
                ));
            }
            // Running trapezoidal integral, so the result is a series the
            // caller can plot rather than one number.
            let mut running = Vec::with_capacity(x.len());
            let mut accumulated = 0.0;
            running.push(accumulated);
            for i in 1..x.len() {
                accumulated += (y[i] + y[i - 1]) * 0.5 * (x[i] - x[i - 1]);
                running.push(accumulated);
            }
            Ok(CalcValue::create_waveform(x.clone(), running))
        }
    }
}

/// `clip(x, lo, hi)` — every sample limited to `[lo, hi]`, domain unchanged.
fn clip(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("clip", &args, 3)?;
    let low = scalar_arg("clip", "lower bound", &args[1])?;
    let high = scalar_arg("clip", "upper bound", &args[2])?;
    if low > high {
        return Err(EvaluationError::MathError(
            "clip lower bound is above its upper bound".to_owned(),
        ));
    }
    match &args[0] {
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(s.clamp(low, high))),
        CalcValue::Waveform(x, y) => Ok(CalcValue::create_waveform(
            x.clone(),
            y.iter().map(|v| v.clamp(low, high)).collect(),
        )),
    }
}

/// `xval(w)` — the domain plotted against itself, so an expression can do
/// arithmetic on time or frequency without a signal named for it.
fn xval(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("xval", &args, 1)?;
    let (x, _) = series_arg("xval", &args[0])?;
    Ok(CalcValue::create_waveform(x.to_vec(), x.to_vec()))
}

/// Unwrap a wrapped phase series given in degrees.
///
/// Standard single-pass unwrap: walk the samples and whenever the step
/// between consecutive finite samples exceeds 180° in magnitude, shift the
/// remainder of the series by the multiple of 360° that minimizes the step.
///
/// Invariants (no test module in this file, so they are stated here):
/// - The output has the same length as the input.
/// - Every output sample differs from its input sample by an exact multiple
///   of 360°, so `out[i] ≡ in[i] (mod 360°)` — the unwrapped curve never
///   changes the underlying phase, only its branch.
/// - Consecutive finite output samples differ by at most 180°, provided the
///   true signal moves less than 180° between samples (the usual sampling
///   assumption for unwrapping).
/// - Non-finite samples (NaN/±inf) pass through unchanged and are skipped
///   when measuring jumps, so a gap does not poison the running offset.
pub fn unwrap_phase_deg(phase: &[f64]) -> Vec<f64> {
    let mut out = Vec::with_capacity(phase.len());
    let mut offset = 0.0_f64;
    let mut prev: Option<f64> = None;
    for &sample in phase {
        if !sample.is_finite() {
            out.push(sample);
            continue;
        }
        if let Some(prev) = prev {
            let jump = sample - prev;
            if jump.abs() > 180.0 {
                // Nearest multiple of 360° that brings the step into ±180°.
                offset -= 360.0 * (jump / 360.0).round();
            }
        }
        prev = Some(sample);
        out.push(sample + offset);
    }
    out
}

/// `unwrap(phase_waveform)` — continuous phase from a ±180°-wrapped trace.
fn unwrap(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("unwrap", &args, 1)?;
    match &args[0] {
        // A single sample has no jumps to unwrap.
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(*s)),
        CalcValue::Waveform(x, y) => Ok(CalcValue::create_waveform(x.clone(), unwrap_phase_deg(y))),
    }
}

// =============================================================================
// Measurements (Waveform -> Scalar)
// =============================================================================

/// The lowest and highest finite samples.
///
/// Holes are skipped rather than poisoning the answer: a `NaN` left by the
/// domain policy above says "no value here", not "no value anywhere".
fn extremes(name: &str, y: &[f64]) -> Result<(f64, f64), EvaluationError> {
    let mut low = f64::INFINITY;
    let mut high = f64::NEG_INFINITY;
    for value in y.iter().filter(|v| v.is_finite()) {
        low = low.min(*value);
        high = high.max(*value);
    }
    if low.is_finite() {
        Ok((low, high))
    } else {
        Err(EvaluationError::MathError(format!(
            "{name} has no finite sample to measure"
        )))
    }
}

fn min(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("min", &args, 1)?;
    match &args[0] {
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(*s)),
        CalcValue::Waveform(_, y) => Ok(CalcValue::Scalar(extremes("min", y)?.0)),
    }
}

fn max(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("max", &args, 1)?;
    match &args[0] {
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(*s)),
        CalcValue::Waveform(_, y) => Ok(CalcValue::Scalar(extremes("max", y)?.1)),
    }
}

/// `pp(w)` — peak-to-peak excursion.
fn pp(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("pp", &args, 1)?;
    match &args[0] {
        CalcValue::Scalar(_) => Ok(CalcValue::Scalar(0.0)),
        CalcValue::Waveform(_, y) => {
            let (low, high) = extremes("pp", y)?;
            Ok(CalcValue::Scalar(high - low))
        }
    }
}

/// Linear interpolation inside the swept range. Callers check the range
/// first, because nothing in this module extrapolates.
fn interpolate(x: &[f64], y: &[f64], at: f64) -> f64 {
    if at <= x[0] {
        return y[0];
    }
    for i in 1..x.len() {
        if at <= x[i] {
            let dx = x[i] - x[i - 1];
            if dx == 0.0 {
                return y[i];
            }
            return y[i - 1] + (y[i] - y[i - 1]) * (at - x[i - 1]) / dx;
        }
    }
    y[y.len() - 1]
}

/// `yval(w, at)` — the value of `w` where its domain reads `at`.
fn yval(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("yval", &args, 2)?;
    let (x, y) = series_arg("yval", &args[0])?;
    let at = scalar_arg("yval", "position", &args[1])?;
    let (first, last) = (x[0], x[x.len() - 1]);
    let (low, high) = if first <= last {
        (first, last)
    } else {
        (last, first)
    };
    if !(at >= low && at <= high) {
        return Err(EvaluationError::MathError(format!(
            "yval position {at} lies outside the swept range {low} to {high}"
        )));
    }
    Ok(CalcValue::Scalar(interpolate(x, y, at)))
}

/// Where a signal passed through a level, and which way it was going.
#[derive(Clone, Copy)]
struct Crossing {
    x: f64,
    rising: bool,
}

/// Every crossing of `level`, in domain order.
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
        if let Some((before, previous_sign)) = previous {
            if previous_sign != sign {
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
        }
        previous = Some((i, sign));
    }
    found
}

/// The 50 % level a periodic measurement keys off: `(min + max) / 2`.
fn mid_level(name: &str, y: &[f64]) -> Result<f64, EvaluationError> {
    let (low, high) = extremes(name, y)?;
    if low == high {
        return Err(EvaluationError::MathError(format!(
            "{name} needs a signal that moves; every sample is {low}"
        )));
    }
    Ok((low + high) * 0.5)
}

/// `cross(w, level, n)` — where `w` crosses `level` for the nth time.
fn cross(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("cross", &args, 3)?;
    let (x, y) = series_arg("cross", &args[0])?;
    let level = scalar_arg("cross", "level", &args[1])?;
    let ordinal = scalar_arg("cross", "ordinal", &args[2])?;
    if !(ordinal >= 1.0) || ordinal.fract() != 0.0 {
        return Err(EvaluationError::MathError(
            "cross ordinal must be a whole number of 1 or more".to_owned(),
        ));
    }
    let wanted = ordinal as usize;
    let found = crossings(x, y, level);
    match found.get(wanted - 1) {
        Some(crossing) => Ok(CalcValue::Scalar(crossing.x)),
        None => Err(EvaluationError::MathError(format!(
            "the signal crosses {level} {} time(s), not {wanted}",
            found.len()
        ))),
    }
}

/// Repetition rate from the rising mid-level crossings.
///
/// Measured across *all* the cycles present rather than the first one, so
/// jitter averages out instead of deciding the answer.
fn fundamental(name: &str, x: &[f64], y: &[f64]) -> Result<f64, EvaluationError> {
    let mid = mid_level(name, y)?;
    let rising: Vec<f64> = crossings(x, y, mid)
        .into_iter()
        .filter(|crossing| crossing.rising)
        .map(|crossing| crossing.x)
        .collect();
    if rising.len() < 2 {
        return Err(EvaluationError::MathError(format!(
            "{name} needs at least one whole period between rising mid-level crossings"
        )));
    }
    let span = rising[rising.len() - 1] - rising[0];
    if span <= 0.0 {
        return Err(EvaluationError::MathError(format!(
            "{name} found no elapsed time between mid-level crossings"
        )));
    }
    Ok((rising.len() - 1) as f64 / span)
}

fn freq(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("freq", &args, 1)?;
    let (x, y) = series_arg("freq", &args[0])?;
    Ok(CalcValue::Scalar(fundamental("freq", x, y)?))
}

fn period(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("period", &args, 1)?;
    let (x, y) = series_arg("period", &args[0])?;
    Ok(CalcValue::Scalar(1.0 / fundamental("period", x, y)?))
}

/// `duty(w)` — percent of each period spent above the mid level, averaged
/// over every whole cycle the window contains.
fn duty(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("duty", &args, 1)?;
    let (x, y) = series_arg("duty", &args[0])?;
    let mid = mid_level("duty", y)?;
    let found = crossings(x, y, mid);
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
        return Err(EvaluationError::MathError(
            "duty needs a whole cycle: a rising mid-level crossing, a fall, and the next rise"
                .to_owned(),
        ));
    }
    Ok(CalcValue::Scalar(100.0 * total / cycles as f64))
}

/// The levels a step response starts and ends at, taken from the first and
/// last finite samples.
fn step_levels(name: &str, y: &[f64]) -> Result<(f64, f64), EvaluationError> {
    let no_sample = || EvaluationError::MathError(format!("{name} has no finite sample"));
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
    if initial == settled {
        return Err(EvaluationError::MathError(format!(
            "{name} needs a step; the first and last samples are both {initial}"
        )));
    }
    Ok((initial, settled))
}

/// `overshoot(w)` — percent by which the response passed its final value,
/// relative to the step it took to get there.
fn overshoot(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("overshoot", &args, 1)?;
    let (_, y) = series_arg("overshoot", &args[0])?;
    let (initial, settled) = step_levels("overshoot", y)?;
    let (low, high) = extremes("overshoot", y)?;
    let step = settled - initial;
    let peak = if step > 0.0 { high } else { low };
    Ok(CalcValue::Scalar(100.0 * (peak - settled) / step))
}

/// The 10 %–90 % transition time of the first edge going `rising`'s way,
/// with the two levels taken from the waveform's own excursion.
fn edge(name: &str, x: &[f64], y: &[f64], rising: bool) -> Result<f64, EvaluationError> {
    let (low, high) = extremes(name, y)?;
    if low == high {
        return Err(EvaluationError::MathError(format!(
            "{name} needs an edge; every sample is {low}"
        )));
    }
    let excursion = high - low;
    let ten = low + 0.1 * excursion;
    let ninety = low + 0.9 * excursion;
    let (from, to) = if rising { (ten, ninety) } else { (ninety, ten) };
    let start = crossings(x, y, from)
        .into_iter()
        .find(|crossing| crossing.rising == rising)
        .ok_or_else(|| {
            EvaluationError::MathError(format!("{name} finds no edge through the {from} level"))
        })?;
    let end = crossings(x, y, to)
        .into_iter()
        .find(|crossing| crossing.rising == rising && crossing.x >= start.x)
        .ok_or_else(|| {
            EvaluationError::MathError(format!("{name} finds no edge through the {to} level"))
        })?;
    Ok(end.x - start.x)
}

fn rise(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("rise", &args, 1)?;
    let (x, y) = series_arg("rise", &args[0])?;
    Ok(CalcValue::Scalar(edge("rise", x, y, true)?))
}

fn fall(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("fall", &args, 1)?;
    let (x, y) = series_arg("fall", &args[0])?;
    Ok(CalcValue::Scalar(edge("fall", x, y, false)?))
}

/// `settling(w, band)` — time from the start of the window until `w` enters
/// and stays inside `band` percent of its step, measured about the final
/// value.
fn settling(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("settling", &args, 2)?;
    let (x, y) = series_arg("settling", &args[0])?;
    let band = scalar_arg("settling", "band", &args[1])?;
    if !(band > 0.0) {
        return Err(EvaluationError::MathError(
            "settling band must be a positive percentage of the step".to_owned(),
        ));
    }
    let (initial, settled) = step_levels("settling", y)?;
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
        return Ok(CalcValue::Scalar(0.0));
    };
    if outside + 1 >= y.len() {
        return Err(EvaluationError::MathError(format!(
            "the signal never settles inside {band} % of its step"
        )));
    }
    let (before, after) = (outside, outside + 1);
    let out = (y[before] - settled).abs();
    let inside = (y[after] - settled).abs();
    let entry = if out == inside {
        x[after]
    } else {
        x[before] + (out - tolerance) * (x[after] - x[before]) / (out - inside)
    };
    Ok(CalcValue::Scalar(entry - x[0]))
}

/// `delay(a, b)` — how long after `a` first crosses its mid level `b`
/// crosses its own. `delay(a, b, level)` measures both against one level
/// instead, which is what a logic threshold needs.
///
/// The two waveforms are read independently, so they need not share a grid.
fn delay(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_range("delay", &args, 2, 3)?;
    let (from_x, from_y) = series_arg("delay", &args[0])?;
    let (to_x, to_y) = series_arg("delay", &args[1])?;
    let (from_level, to_level) = match args.get(2) {
        Some(value) => {
            let level = scalar_arg("delay", "level", value)?;
            (level, level)
        }
        None => (mid_level("delay", from_y)?, mid_level("delay", to_y)?),
    };
    let first = crossings(from_x, from_y, from_level)
        .first()
        .copied()
        .ok_or_else(|| {
            EvaluationError::MathError(format!("the first waveform never crosses {from_level}"))
        })?;
    let second = crossings(to_x, to_y, to_level)
        .first()
        .copied()
        .ok_or_else(|| {
            EvaluationError::MathError(format!("the second waveform never crosses {to_level}"))
        })?;
    Ok(CalcValue::Scalar(second.x - first.x))
}

/// How many harmonics `thd` weighs when the caller does not say.
const DEFAULT_THD_HARMONICS: usize = 10;

/// The samples inside `[from, to]`, with the two endpoints interpolated onto
/// the boundary so the window is exactly the interval asked for.
fn window(x: &[f64], y: &[f64], from: f64, to: f64) -> (Vec<f64>, Vec<f64>) {
    let mut window_x = vec![from];
    let mut window_y = vec![interpolate(x, y, from)];
    for i in 0..x.len() {
        if x[i] > from && x[i] < to {
            window_x.push(x[i]);
            window_y.push(y[i]);
        }
    }
    window_x.push(to);
    window_y.push(interpolate(x, y, to));
    (window_x, window_y)
}

/// `thd(w)` — total harmonic distortion in percent; `thd(w, n)` weighs `n`
/// harmonics instead of [`DEFAULT_THD_HARMONICS`].
///
/// The window is a whole number of periods, bounded by the first and last
/// rising crossings of the mean level, and the harmonic amplitudes come from
/// integrating `w` against a quadrature pair at each multiple of the
/// fundamental. Integrating rather than transforming is what makes this
/// correct on a non-uniform grid: an FFT would first have to resample, and
/// the resampling — not the signal — would set the noise floor.
fn thd(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_range("thd", &args, 1, 2)?;
    let (x, y) = series_arg("thd", &args[0])?;
    let harmonics = match args.get(1) {
        Some(value) => {
            let count = scalar_arg("thd", "harmonic count", value)?;
            if !(count >= 2.0) || count.fract() != 0.0 {
                return Err(EvaluationError::MathError(
                    "thd harmonic count must be a whole number of 2 or more".to_owned(),
                ));
            }
            count as usize
        }
        None => DEFAULT_THD_HARMONICS,
    };
    let mean = window_mean("thd", x, y, |v| v)?;
    let rising: Vec<f64> = crossings(x, y, mean)
        .into_iter()
        .filter(|crossing| crossing.rising)
        .map(|crossing| crossing.x)
        .collect();
    if rising.len() < 2 {
        return Err(EvaluationError::MathError(
            "thd needs at least one whole period of a periodic signal".to_owned(),
        ));
    }
    let (from, to) = (rising[0], rising[rising.len() - 1]);
    let length = to - from;
    if length <= 0.0 {
        return Err(EvaluationError::MathError(
            "thd found no elapsed time between periods".to_owned(),
        ));
    }
    let base = (rising.len() - 1) as f64 / length;
    let (window_x, window_y) = window(x, y, from, to);
    let amplitudes: Vec<f64> = (1..=harmonics)
        .map(|order| {
            let omega = 2.0 * PI * order as f64 * base;
            let cosine =
                2.0 / length * trapezoid_xy(&window_x, &window_y, |t, v| v * (omega * t).cos());
            let sine =
                2.0 / length * trapezoid_xy(&window_x, &window_y, |t, v| v * (omega * t).sin());
            cosine.hypot(sine)
        })
        .collect();
    if amplitudes[0] == 0.0 {
        return Err(EvaluationError::MathError(
            "thd found no fundamental to compare the harmonics against".to_owned(),
        ));
    }
    let harmonic_power: f64 = amplitudes[1..].iter().map(|a| a * a).sum();
    Ok(CalcValue::Scalar(
        100.0 * harmonic_power.sqrt() / amplitudes[0],
    ))
}

// =============================================================================
// Argument plumbing and the domain policy
// =============================================================================

fn check_arg_count(name: &str, args: &[CalcValue], expected: usize) -> Result<(), EvaluationError> {
    if args.len() != expected {
        Err(EvaluationError::ArgCountMismatch {
            func: name.to_string(),
            expected,
            actual: args.len(),
        })
    } else {
        Ok(())
    }
}

/// As [`check_arg_count`], for a function with an optional trailing argument.
/// The reported expectation is the bound the call actually missed.
fn check_arg_range(
    name: &str,
    args: &[CalcValue],
    least: usize,
    most: usize,
) -> Result<(), EvaluationError> {
    if args.len() < least || args.len() > most {
        Err(EvaluationError::ArgCountMismatch {
            func: name.to_string(),
            expected: if args.len() < least { least } else { most },
            actual: args.len(),
        })
    } else {
        Ok(())
    }
}

/// A waveform argument with samples, or the reason it cannot be measured.
fn series_arg<'a>(
    name: &str,
    value: &'a CalcValue,
) -> Result<(&'a [f64], &'a [f64]), EvaluationError> {
    match value {
        CalcValue::Waveform(x, y) if !x.is_empty() && x.len() == y.len() => Ok((x, y)),
        CalcValue::Waveform(..) => Err(EvaluationError::MathError(format!(
            "{name} needs a waveform with samples"
        ))),
        CalcValue::Scalar(_) => Err(EvaluationError::TypeMismatch(format!(
            "{name} measures a waveform, not a scalar"
        ))),
    }
}

fn scalar_arg(name: &str, what: &str, value: &CalcValue) -> Result<f64, EvaluationError> {
    match value {
        CalcValue::Scalar(v) => Ok(*v),
        CalcValue::Waveform(..) => Err(EvaluationError::TypeMismatch(format!(
            "{name} {what} must be a scalar"
        ))),
    }
}

fn domain_error(name: &str, what: &str) -> EvaluationError {
    EvaluationError::MathError(format!("{name} is undefined for {what}"))
}

/// The series half of the domain policy: anything that is not a finite
/// number — a `NaN` from a negative root, an infinity from a logarithmic
/// pole — becomes one hole, so the renderer breaks the stroke exactly once
/// per unusable sample.
fn hole_unless_finite(value: f64) -> f64 {
    if value.is_finite() { value } else { f64::NAN }
}

/// Apply a domain-restricted function sample-wise, holing what falls outside.
fn map_domain(x: &[f64], y: &[f64], f: impl Fn(f64) -> f64) -> CalcValue {
    CalcValue::create_waveform(
        x.to_vec(),
        y.iter().map(|v| hole_unless_finite(f(*v))).collect(),
    )
}

// =============================================================================
// Tests
// =============================================================================
//
// Every case below is an analytic oracle: a synthetic waveform on a
// deliberately NON-UNIFORM x-grid whose expected value is known in closed
// form. Nothing here is a golden capture of what the code happens to produce.

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    // -- fixtures ---------------------------------------------------------

    fn wave(x: Vec<f64>, y: Vec<f64>) -> CalcValue {
        CalcValue::create_waveform(x, y)
    }

    fn call(name: &str, args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
        FunctionRegistry::dispatch(name, args)
    }

    #[track_caller]
    fn scalar_of(name: &str, args: Vec<CalcValue>) -> f64 {
        match call(name, args) {
            Ok(CalcValue::Scalar(value)) => value,
            other => panic!("{name} did not return a scalar: {other:?}"),
        }
    }

    #[track_caller]
    fn series_of(name: &str, args: Vec<CalcValue>) -> (Vec<f64>, Vec<f64>) {
        match call(name, args) {
            Ok(CalcValue::Waveform(x, y)) => (x, y),
            other => panic!("{name} did not return a waveform: {other:?}"),
        }
    }

    #[track_caller]
    fn assert_close(actual: f64, expected: f64, tol: f64, what: &str) {
        assert!(
            (actual - expected).abs() <= tol,
            "{what}: expected {expected}, got {actual} (tolerance {tol})"
        );
    }

    /// Deterministic non-uniform grid: the step cycles 1:2:3, so the widest
    /// step is three times the narrowest and no result can pass by quietly
    /// assuming a uniform sample spacing.
    fn nonuniform(n: usize, x0: f64, x1: f64) -> Vec<f64> {
        assert!(n >= 2);
        let mut raw = Vec::with_capacity(n);
        let mut acc = 0.0;
        for i in 0..n {
            raw.push(acc);
            acc += 1.0 + (i % 3) as f64;
        }
        let span = raw[n - 1];
        raw.into_iter().map(|v| x0 + (x1 - x0) * v / span).collect()
    }

    /// Value of the piecewise-linear function through `breaks` at `x`.
    fn pwl_at(breaks: &[(f64, f64)], x: f64) -> f64 {
        if x <= breaks[0].0 {
            return breaks[0].1;
        }
        for pair in breaks.windows(2) {
            let ((x0, y0), (x1, y1)) = (pair[0], pair[1]);
            if x <= x1 {
                return y0 + (y1 - y0) * (x - x0) / (x1 - x0);
            }
        }
        breaks[breaks.len() - 1].1
    }

    /// A non-uniform grid that *contains every breakpoint*, so sampling the
    /// piecewise-linear function on it reproduces that function exactly.
    fn pwl_series(breaks: &[(f64, f64)], filler: usize) -> (Vec<f64>, Vec<f64>) {
        let (x0, x1) = (breaks[0].0, breaks[breaks.len() - 1].0);
        let mut x = nonuniform(filler, x0, x1);
        x.extend(breaks.iter().map(|(t, _)| *t));
        x.sort_by(|a, b| a.partial_cmp(b).expect("finite grid"));
        x.dedup_by(|a, b| (*a - *b).abs() < 1.0e-12);
        let y = x.iter().map(|t| pwl_at(breaks, *t)).collect();
        (x, y)
    }

    /// SPICE-shaped pulse with duplicated timepoints at the vertical edges:
    /// 2 V on [0,1], -1 V on [1,3], 2 V on [3,4].
    fn duplicated_edge_pulse() -> CalcValue {
        wave(
            vec![0.0, 1.0, 1.0, 3.0, 3.0, 4.0],
            vec![2.0, 2.0, -1.0, -1.0, 2.0, 2.0],
        )
    }

    // -- defect 2: dB ------------------------------------------------------

    #[test]
    fn db_is_twenty_log10_of_the_magnitude() {
        assert_close(
            scalar_of("db", vec![CalcValue::Scalar(100.0)]),
            40.0,
            1.0e-12,
            "dB(100)",
        );
        assert_close(
            scalar_of("dB", vec![CalcValue::Scalar(0.5)]),
            -6.020_599_913_279_624,
            1.0e-12,
            "dB(0.5)",
        );
        // Magnitude: a negative argument is its absolute value in dB.
        assert_close(
            scalar_of("dB", vec![CalcValue::Scalar(-2.0)]),
            6.020_599_913_279_624,
            1.0e-12,
            "dB(-2)",
        );

        let x = nonuniform(5, 0.0, 4.0);
        let (out_x, out_y) = series_of(
            "db",
            vec![wave(x.clone(), vec![1.0, 10.0, -100.0, 1000.0, 0.1])],
        );
        assert_eq!(out_x, x, "dB keeps the domain");
        for (got, want) in out_y.iter().zip([0.0, 20.0, 40.0, 60.0, -20.0]) {
            assert_close(*got, want, 1.0e-12, "dB series sample");
        }
    }

    #[test]
    fn db_of_zero_is_a_hole_in_a_series_and_an_error_as_a_scalar() {
        assert!(
            call("db", vec![CalcValue::Scalar(0.0)]).is_err(),
            "dB(0) is out of domain"
        );
        let (_, y) = series_of("db", vec![wave(vec![0.0, 1.0, 2.0], vec![1.0, 0.0, 10.0])]);
        assert!(
            y[0].is_finite() && y[2].is_finite(),
            "finite samples survive"
        );
        assert!(y[1].is_nan(), "dB of a zero sample is a hole, got {}", y[1]);
    }

    // -- defect 3: avg / rms integrate the window --------------------------

    #[test]
    fn avg_integrates_the_window_rather_than_averaging_samples() {
        // Ramp y = 2x + 1 over [0, 3]: (1/3)·∫(2x+1)dx = 4 exactly, and the
        // trapezoidal rule is exact for a linear integrand on any grid.
        let (x, y) = pwl_series(&[(0.0, 1.0), (3.0, 7.0)], 17);
        assert_close(
            scalar_of("avg", vec![wave(x, y)]),
            4.0,
            1.0e-12,
            "avg of a ramp",
        );

        // Duplicated-edge pulse: (2·1 + −1·2 + 2·1)/4 = 0.5. The plain sample
        // mean of the six stored samples is 1.0, so this discriminates.
        assert_close(
            scalar_of("avg", vec![duplicated_edge_pulse()]),
            0.5,
            1.0e-12,
            "avg of a pulse",
        );
    }

    #[test]
    fn rms_integrates_the_window_rather_than_averaging_samples() {
        // Pulse: sqrt((4·1 + 1·2 + 4·1)/4) = sqrt(2.5). Sample RMS is sqrt(3).
        assert_close(
            scalar_of("rms", vec![duplicated_edge_pulse()]),
            2.5_f64.sqrt(),
            1.0e-12,
            "rms of a pulse",
        );

        // Sine of amplitude 3 over exactly two periods: RMS = 3/sqrt(2).
        let x = nonuniform(4001, 0.0, 2.0);
        let y: Vec<f64> = x.iter().map(|t| 3.0 * (2.0 * PI * t).sin()).collect();
        assert_close(
            scalar_of("rms", vec![wave(x, y)]),
            3.0 / 2.0_f64.sqrt(),
            1.0e-4,
            "rms of a sine",
        );
    }

    #[test]
    fn avg_and_rms_reject_an_empty_series() {
        assert!(
            call("avg", vec![wave(vec![], vec![])]).is_err(),
            "avg of nothing is an error"
        );
        assert!(
            call("rms", vec![wave(vec![], vec![])]).is_err(),
            "rms of nothing is an error"
        );
    }

    #[test]
    fn avg_and_rms_refuse_a_series_with_holes() {
        // `sqrt` of a series that dips negative leaves exactly the hole this
        // module's domain policy promises. An integral over a window with a
        // missing region is not the mean it claims to be, so both aggregates
        // must refuse — the failure mode being guarded is silent, not loud:
        // `avg` used to print "= NaN" in the success colour, and `rms`, whose
        // `.max(0.0)` swallowed the NaN, used to print a confident "= 0".
        let x = vec![0.0, 1.0, 2.0, 3.0];
        let (hx, hy) = series_of("sqrt", vec![wave(x, vec![4.0, -1.0, 9.0, 16.0])]);
        assert!(hy[1].is_nan(), "the fixture must actually carry a hole");
        let holed = wave(hx, hy);
        for name in ["avg", "rms"] {
            let outcome = call(name, vec![holed.clone()]);
            assert!(
                outcome.is_err(),
                "{name} of a holed series must refuse, got {outcome:?}"
            );
        }
    }

    // -- defect 4: one domain policy --------------------------------------

    #[test]
    fn out_of_domain_series_samples_become_holes() {
        let x = vec![0.0, 1.0, 2.5, 4.0];
        for (name, y, hole) in [
            ("sqrt", vec![4.0, -1.0, 9.0, 1.0], 1usize),
            ("ln", vec![1.0, 0.0, PI, 2.0], 1),
            ("ln", vec![1.0, -3.0, PI, 2.0], 1),
            ("log10", vec![10.0, 100.0, 0.0, 1.0], 2),
            ("log10", vec![10.0, 100.0, -4.0, 1.0], 2),
        ] {
            let (out_x, out_y) = series_of(name, vec![wave(x.clone(), y)]);
            assert_eq!(out_x, x, "{name} keeps the domain");
            assert!(
                out_y[hole].is_nan(),
                "{name} sample {hole} must be a hole, got {}",
                out_y[hole]
            );
            for (index, value) in out_y.iter().enumerate() {
                if index != hole {
                    assert!(
                        value.is_finite(),
                        "{name} sample {index} must stay finite, got {value}"
                    );
                }
            }
        }
    }

    #[test]
    fn out_of_domain_scalars_are_errors() {
        for (name, arg) in [
            ("sqrt", -1.0),
            ("ln", 0.0),
            ("ln", -2.0),
            ("log", -2.0),
            ("log10", 0.0),
            ("log10", -5.0),
        ] {
            assert!(
                call(name, vec![CalcValue::Scalar(arg)]).is_err(),
                "{name}({arg}) is out of domain and must be an error"
            );
        }
    }

    #[test]
    fn deriv_is_exact_on_a_linear_ramp_over_a_nonuniform_grid() {
        let x = nonuniform(31, 0.0, 5.0);
        let y: Vec<f64> = x.iter().map(|t| 3.0 * t + 7.0).collect();
        let (out_x, out_y) = series_of("deriv", vec![wave(x.clone(), y)]);
        assert_eq!(out_x, x, "deriv keeps the domain");
        for (index, value) in out_y.iter().enumerate() {
            assert_close(*value, 3.0, 1.0e-9, &format!("deriv sample {index}"));
        }
    }

    #[test]
    fn deriv_guards_duplicate_timepoints() {
        // A duplicated first timepoint: the one-sided difference must reach
        // the nearest neighbour at a *distinct* x instead of dividing by zero.
        let (_, y) = series_of(
            "deriv",
            vec![wave(vec![0.0, 0.0, 1.0, 2.0], vec![0.0, 1.0, 3.0, 5.0])],
        );
        for (index, value) in y.iter().enumerate() {
            assert!(
                value.is_finite(),
                "deriv sample {index} must be finite, got {value}"
            );
        }
        assert_close(y[0], 3.0, 1.0e-12, "one-sided slope across the duplicate");
        assert_close(y[3], 2.0, 1.0e-12, "trailing backward difference");

        // A wholly degenerate stencil has no slope: a hole, never an infinity.
        let (_, y) = series_of(
            "deriv",
            vec![wave(vec![0.0, 0.0, 0.0, 1.0], vec![0.0, 1.0, 2.0, 3.0])],
        );
        assert!(
            y[1].is_nan(),
            "a zero-width central stencil is a hole, got {}",
            y[1]
        );
    }

    #[test]
    fn clip_limits_a_series_to_its_scalar_bounds() {
        let (_, y) = series_of(
            "clip",
            vec![
                wave(vec![0.0, 1.0, 2.0], vec![-5.0, 0.5, 9.0]),
                CalcValue::Scalar(-1.0),
                CalcValue::Scalar(1.0),
            ],
        );
        assert_eq!(
            y,
            vec![-1.0, 0.5, 1.0],
            "clip(x, lo, hi) takes three arguments"
        );
    }

    // -- defect 5: measurement breadth -------------------------------------

    #[test]
    fn min_max_and_pp_report_the_series_extremes() {
        let pulse = duplicated_edge_pulse();
        assert_close(scalar_of("min", vec![pulse.clone()]), -1.0, 1.0e-12, "min");
        assert_close(scalar_of("max", vec![pulse.clone()]), 2.0, 1.0e-12, "max");
        assert_close(scalar_of("pp", vec![pulse]), 3.0, 1.0e-12, "peak-to-peak");
    }

    #[test]
    fn xval_returns_the_domain_and_yval_interpolates_within_it() {
        let (x, y) = pwl_series(&[(0.0, 1.0), (3.0, 7.0)], 13);
        let (out_x, out_y) = series_of("xval", vec![wave(x.clone(), y.clone())]);
        assert_eq!(out_x, x, "xval keeps the domain");
        assert_eq!(out_y, x, "xval plots the domain against itself");

        // y = 2x + 1 at x = 1.3 is 3.6, exactly, under linear interpolation.
        assert_close(
            scalar_of(
                "yval",
                vec![wave(x.clone(), y.clone()), CalcValue::Scalar(1.3)],
            ),
            3.6,
            1.0e-12,
            "yval inside the domain",
        );
        assert!(
            call("yval", vec![wave(x, y), CalcValue::Scalar(9.0)]).is_err(),
            "yval never extrapolates"
        );
    }

    #[test]
    fn cross_finds_the_nth_crossing_of_a_level() {
        // Triangle: 0 → 2 over [0,1], 2 → −2 over [1,3], −2 → 0 over [3,4].
        // Level 1 is crossed rising at x = 0.5 and falling at x = 1.5.
        let (x, y) = pwl_series(&[(0.0, 0.0), (1.0, 2.0), (3.0, -2.0), (4.0, 0.0)], 11);
        let series = wave(x, y);
        assert_close(
            scalar_of(
                "cross",
                vec![
                    series.clone(),
                    CalcValue::Scalar(1.0),
                    CalcValue::Scalar(1.0),
                ],
            ),
            0.5,
            1.0e-12,
            "first crossing of 1",
        );
        assert_close(
            scalar_of(
                "cross",
                vec![
                    series.clone(),
                    CalcValue::Scalar(1.0),
                    CalcValue::Scalar(2.0),
                ],
            ),
            1.5,
            1.0e-12,
            "second crossing of 1",
        );
        assert!(
            call(
                "cross",
                vec![series, CalcValue::Scalar(1.0), CalcValue::Scalar(3.0)]
            )
            .is_err(),
            "there is no third crossing of 1"
        );
    }

    /// Four periods of a trapezoidal pulse train: period 1, rise 0 → 10 over
    /// [k, k+0.2], fall 10 → 0 over [k+0.5, k+0.7]. The mid level is 5, so
    /// the rising mid-crossings sit at k+0.1 and the falling ones at k+0.6:
    /// frequency 1, period 1, duty 50 %.
    fn pulse_train() -> CalcValue {
        let mut breaks = Vec::new();
        for k in 0..4 {
            let t = k as f64;
            breaks.push((t, 0.0));
            breaks.push((t + 0.2, 10.0));
            breaks.push((t + 0.5, 10.0));
            breaks.push((t + 0.7, 0.0));
        }
        breaks.push((4.0, 0.0));
        let (x, y) = pwl_series(&breaks, 61);
        wave(x, y)
    }

    #[test]
    fn freq_period_and_duty_measure_a_pulse_train() {
        assert_close(
            scalar_of("freq", vec![pulse_train()]),
            1.0,
            1.0e-9,
            "frequency",
        );
        assert_close(
            scalar_of("period", vec![pulse_train()]),
            1.0,
            1.0e-9,
            "period",
        );
        assert_close(
            scalar_of("duty", vec![pulse_train()]),
            50.0,
            1.0e-9,
            "duty cycle in percent",
        );
    }

    #[test]
    fn overshoot_is_the_percent_above_the_final_value() {
        // Step 0 → 1 that peaks at 1.2 and settles back to 1.0: 20 %.
        let (x, y) = pwl_series(
            &[(0.0, 0.0), (1.0, 1.2), (2.0, 0.9), (3.0, 1.0), (4.0, 1.0)],
            23,
        );
        assert_close(
            scalar_of("overshoot", vec![wave(x, y)]),
            20.0,
            1.0e-9,
            "overshoot",
        );
    }

    #[test]
    fn rise_and_fall_measure_the_ten_to_ninety_percent_edge() {
        // 0 → 10 over [0,1] then flat: 10 % at 0.1, 90 % at 0.9 → 0.8.
        let (x, y) = pwl_series(&[(0.0, 0.0), (1.0, 10.0), (2.0, 10.0)], 19);
        assert_close(
            scalar_of("rise", vec![wave(x, y)]),
            0.8,
            1.0e-12,
            "rise time",
        );

        // Flat 10 then 10 → 0 over [1,2]: 90 % at 1.1, 10 % at 1.9 → 0.8.
        let (x, y) = pwl_series(&[(0.0, 10.0), (1.0, 10.0), (2.0, 0.0)], 19);
        assert_close(
            scalar_of("fall", vec![wave(x, y)]),
            0.8,
            1.0e-12,
            "fall time",
        );
    }

    #[test]
    fn settling_finds_the_entry_into_the_band() {
        // Step 0 → 1. |y − 1| falls linearly from 0.05 at x = 1 to 0 at x = 2,
        // so a 2 % band (of the 1.0 step) is entered at x = 1.6.
        let (x, y) = pwl_series(&[(0.0, 0.0), (1.0, 1.05), (2.0, 1.0), (3.0, 1.0)], 17);
        assert_close(
            scalar_of("settling", vec![wave(x, y), CalcValue::Scalar(2.0)]),
            1.6,
            1.0e-12,
            "settling time to a 2 % band",
        );
    }

    #[test]
    fn delay_is_the_gap_between_two_mid_level_crossings() {
        // Both ramps run 0 → 10; the second starts 0.3 later, and the two
        // series carry different, independently non-uniform grids.
        let (ax, ay) = pwl_series(&[(0.0, 0.0), (1.0, 10.0), (2.0, 10.0)], 19);
        let (bx, by) = pwl_series(&[(0.0, 0.0), (0.3, 0.0), (1.3, 10.0), (2.0, 10.0)], 26);
        assert_close(
            scalar_of("delay", vec![wave(ax, ay), wave(bx, by)]),
            0.3,
            1.0e-12,
            "delay between mid crossings",
        );
    }

    #[test]
    fn thd_recovers_a_known_third_harmonic() {
        // sin(2πt) + 0.1·sin(6πt): a single 10 % third harmonic.
        let x = nonuniform(8001, 0.0, 4.0);
        let y: Vec<f64> = x
            .iter()
            .map(|t| (2.0 * PI * t).sin() + 0.1 * (6.0 * PI * t).sin())
            .collect();
        assert_close(
            scalar_of("thd", vec![wave(x, y)]),
            10.0,
            0.02,
            "total harmonic distortion",
        );
    }

    #[test]
    fn measurements_reject_a_series_with_nothing_to_measure() {
        for name in [
            "min", "max", "pp", "freq", "period", "duty", "rise", "fall", "thd",
        ] {
            assert!(
                call(name, vec![wave(vec![], vec![])]).is_err(),
                "{name} of an empty series must be an error"
            );
        }
    }
}
