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
//! This library implements real-valued operations. Complex-aware dispatch
//! routes rectangular inputs to their own arithmetic and explicit projections;
//! it calls these kernels only when an operation has defined real semantics.

use super::evaluator::EvaluationError;
use super::measures::Argument;
use super::value::RealValue as CalcValue;

mod conversion;
mod trigonometry;
pub use conversion::{EventWord, analog_staircase, is_logic_expression, measured_steps};

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
            "db" | "db20" => db(args, Decibels::Amplitude),
            "db10" => db(args, Decibels::Power),
            "limit" => limit(args),
            "if" => choose(args),

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

            // Analog-to-digital and digital-to-analog conversion
            "digital" => conversion::digital(args),
            "analog" => conversion::analog(args),

            name if trigonometry::NAMES.contains(&name) => trigonometry::apply(name, args),

            // Edge, pulse and step measurements take option words, so they live with the
            // argument handling that reads them.
            _ => {
                let lower = name.to_lowercase();
                let args: Vec<Argument> = args.into_iter().map(Argument::from).collect();
                if lower == "delay" {
                    return super::time_measures::dispatch_delay(name, &args)
                        .map(CalcValue::Scalar);
                }
                match super::time_measures::lookup(&lower) {
                    Some(timed) => timed.dispatch(name, &args),
                    None => Err(EvaluationError::UnknownFunction(name.to_string())),
                }
            }
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

/// Which decibel a quantity is read in, as ViVA's `dB20` and `dB10` define
/// them: `20·log₁₀(mag(x))` for an amplitude (a voltage, a current, a ratio
/// of them), `10·log₁₀(mag(x))` for a power.
#[derive(Clone, Copy)]
pub(super) enum Decibels {
    Amplitude,
    Power,
}

impl Decibels {
    pub(super) fn factor(self) -> f64 {
        match self {
            Decibels::Amplitude => 20.0,
            Decibels::Power => 10.0,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Decibels::Amplitude => "dB20",
            Decibels::Power => "dB10",
        }
    }
}

/// `dB(x)` and `dB20(x)` are `20·log₁₀|x|`; `dB10(x)` is `10·log₁₀|x|`.
///
/// The magnitude is what is measured, so a negative sample reads as its
/// absolute value — the sign of a ratio is a phase statement, and phase is
/// not what a dB axis carries.
fn db(args: Vec<CalcValue>, decibels: Decibels) -> Result<CalcValue, EvaluationError> {
    let name = decibels.name();
    check_arg_count(name, &args, 1)?;
    let factor = decibels.factor();
    let level = |value: f64| factor * value.abs().log10();
    match &args[0] {
        CalcValue::Scalar(s) if *s == 0.0 => Err(domain_error(name, "zero")),
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(level(*s))),
        CalcValue::Waveform(x, y) => Ok(map_domain(x, y, level)),
    }
}

/// `limit(x, lo, hi)` — the middle value of the three, sample by sample: `x`
/// held between the two bounds, whichever order they are given in.
fn limit(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("limit", &args, 3)?;
    let first = scalar_arg("limit", "first bound", &args[1])?;
    let second = scalar_arg("limit", "second bound", &args[2])?;
    if !first.is_finite() || !second.is_finite() {
        return Err(EvaluationError::MathError(
            "limit bounds must be finite numbers".to_owned(),
        ));
    }
    let (low, high) = (first.min(second), first.max(second));
    match &args[0] {
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(s.clamp(low, high))),
        CalcValue::Waveform(x, y) => Ok(CalcValue::create_waveform(
            x.clone(),
            y.iter().map(|v| v.clamp(low, high)).collect(),
        )),
    }
}

/// `if(cond, a, b)`: `a` where `cond` is true and `b` where it is not, a
/// condition being true above 0.5 as LTspice's waveform arithmetic reads it,
/// so a comparison's 1 and 0 choose as they read. A series condition
/// chooses sample by sample, and a hole in it is
/// a hole in the result; series on different x grids are read on the span
/// and samples they share, as arithmetic reads them.
fn choose(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("if", &args, 3)?;
    let mut args = args.into_iter();
    let (condition, chosen, otherwise) = (
        args.next().unwrap(),
        args.next().unwrap(),
        args.next().unwrap(),
    );
    let (mut x, mut holds) = match condition {
        CalcValue::Scalar(value) if value.is_nan() => {
            return Err(EvaluationError::MathError(
                "if has no condition to read: it is not a number".to_owned(),
            ));
        }
        CalcValue::Scalar(value) => return Ok(if value > 0.5 { chosen } else { otherwise }),
        CalcValue::Waveform(x, y) => (x, y),
    };
    let mismatch = |error: crate::interpolation::InterpolationError| {
        EvaluationError::WaveformMismatch(format!("if: {error}"))
    };
    for branch in [&chosen, &otherwise] {
        if let CalcValue::Waveform(bx, by) = branch
            && *bx != x
        {
            let (shared, condition, _) = crate::interpolation::align_waveforms(
                &x,
                &holds,
                bx,
                by,
                crate::interpolation::InterpolationMethod::Linear,
            )
            .map_err(mismatch)?;
            (x, holds) = (shared, condition);
        }
    }
    let on_grid = |branch: CalcValue| -> Result<Vec<f64>, EvaluationError> {
        match branch {
            CalcValue::Scalar(value) => Ok(vec![value; x.len()]),
            CalcValue::Waveform(bx, by) if bx == x => Ok(by),
            CalcValue::Waveform(bx, by) => {
                crate::interpolation::WaveformInterpolator::new(&bx, &by)
                    .and_then(|reader| reader.resample(&x))
                    .map_err(mismatch)
            }
        }
    };
    let (chosen, otherwise) = (on_grid(chosen)?, on_grid(otherwise)?);
    let y = holds
        .iter()
        .zip(chosen.iter().zip(&otherwise))
        .map(|(&holds, (&chosen, &otherwise))| {
            if holds.is_nan() {
                f64::NAN
            } else if holds > 0.5 {
                chosen
            } else {
                otherwise
            }
        })
        .collect();
    Ok(CalcValue::create_waveform(x, y))
}

// --- Aggregates over the x-window (Waveform -> Scalar) ---

/// A level measure's window: `(x)` reads the whole trace, `(x, from, to)`
/// the part of it between two points of the x-axis, its ends interpolated
/// on the trace as drawn.
fn level_window(name: &str, args: &[CalcValue]) -> Result<Option<(f64, f64)>, EvaluationError> {
    match args.len() {
        1 => Ok(None),
        3 => Ok(Some((
            scalar_arg(name, "window start", &args[1])?,
            scalar_arg(name, "window end", &args[2])?,
        ))),
        2 => Err(EvaluationError::MathError(format!(
            "{name} reads a window between two points; give both its start and its end"
        ))),
        actual => Err(EvaluationError::ArgCountMismatch {
            func: name.to_owned(),
            expected: 3,
            actual,
        }),
    }
}

fn avg(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    let window = level_window("avg", &args)?;
    match &args[0] {
        CalcValue::Scalar(s) if !s.is_finite() => Err(domain_error("avg", "non-finite value")),
        // A DC value is its own mean.
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(*s)),
        CalcValue::Waveform(x, y) => {
            interval_statistics("avg", x, y, window).map(|stats| CalcValue::Scalar(stats.mean))
        }
    }
}

fn rms(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    let window = level_window("rms", &args)?;
    match &args[0] {
        CalcValue::Scalar(s) if !s.is_finite() => Err(domain_error("rms", "non-finite value")),
        // The RMS of a DC value is its magnitude, not the signed value.
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(s.abs())),
        CalcValue::Waveform(x, y) => {
            interval_statistics("rms", x, y, window).map(|stats| CalcValue::Scalar(stats.rms))
        }
    }
}

fn interval_statistics(
    name: &str,
    x: &[f64],
    y: &[f64],
    window: Option<(f64, f64)>,
) -> Result<crate::measurements::IntervalStatistics, EvaluationError> {
    crate::measurements::measure_interval(x, y, window)
        .map_err(|error| EvaluationError::MathError(format!("{name}: {error}")))
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

/// The slope at `at` of the parabola through samples `a`, `at` and `b`, the
/// three-point formula for uneven steps: exact for a quadratic, and second
/// order on a transient's grid where the centred secant is only first order.
/// `None` when two of the three share an x, so they span no parabola.
fn parabola_slope(x: &[f64], y: &[f64], [a, b, c]: [usize; 3], at: usize) -> Option<f64> {
    let (xa, xb, xc) = (x[a], x[b], x[c]);
    if xa == xb || xb == xc || xa == xc {
        return None;
    }
    // The derivative of the Lagrange interpolant through the three points.
    let t = x[at];
    let slope = y[a] * ((t - xb) + (t - xc)) / ((xa - xb) * (xa - xc))
        + y[b] * ((t - xa) + (t - xc)) / ((xb - xa) * (xb - xc))
        + y[c] * ((t - xa) + (t - xb)) / ((xc - xa) * (xc - xb));
    Some(slope)
}

/// `deriv(x)` — the slope at every sample, from the parabola through it and
/// its two neighbours (at an end, its two nearest). Each side of a vertical
/// edge, where SPICE writes the same timepoint twice, keeps the one-sided
/// slope of its own side.
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
                    let stencil = match i {
                        _ if last < 2 => None,
                        0 => Some([0, 1, 2]),
                        _ if i == last => Some([last - 2, last - 1, last]),
                        _ => Some([i - 1, i, i + 1]),
                    };
                    stencil
                        .and_then(|stencil| parabola_slope(x, y, stencil, i))
                        .unwrap_or_else(|| {
                            if i == 0 || i == last {
                                return one_sided(x, y, i, i == 0);
                            }
                            // The right side of a repeated timepoint reads
                            // forward, its left side backward; a sample with
                            // a repeat on both sides is inside the edge and
                            // has no slope.
                            match (x[i - 1] == x[i], x[i + 1] == x[i]) {
                                (true, false) => one_sided(x, y, i, true),
                                (false, true) => one_sided(x, y, i, false),
                                _ => f64::NAN,
                            }
                        })
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

/// `clip(x, from, to)` — the part of the trace between two points of its
/// x-axis, as ViVA's `clip` returns it: every sample inside, and the trace's
/// value interpolated at each end that falls between samples. It restricts
/// what a measurement around it reads: `avg(clip(V(out), 1m, 2m))` is the
/// mean from 1 ms to 2 ms. A DC value is the same everywhere, so it clips to
/// itself.
fn clip(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("clip", &args, 3)?;
    let from = scalar_arg("clip", "start", &args[1])?;
    let to = scalar_arg("clip", "end", &args[2])?;
    match &args[0] {
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(*s)),
        CalcValue::Waveform(x, y) => {
            let (x, y) = clip_x(x, y, from, to)?;
            Ok(CalcValue::create_waveform(x, y))
        }
    }
}

/// The samples of `(x, y)` between `from` and `to` on the x-axis, either
/// order, with interpolated ends: the trace as drawn over that interval, in
/// the order it was swept. Refused on an axis that turns back, where one x
/// has several values, and for an interval that misses the trace.
pub(super) fn clip_x(
    x: &[f64],
    y: &[f64],
    from: f64,
    to: f64,
) -> Result<(Vec<f64>, Vec<f64>), EvaluationError> {
    if !from.is_finite() || !to.is_finite() {
        return Err(EvaluationError::MathError(
            "clip start and end must be finite numbers".to_owned(),
        ));
    }
    if x.is_empty() || x.len() != y.len() {
        return Err(EvaluationError::MathError(
            "clip needs a waveform with samples".to_owned(),
        ));
    }
    let rises = x.windows(2).any(|pair| pair[1] > pair[0]);
    let falls = x.windows(2).any(|pair| pair[1] < pair[0]);
    if rises && falls {
        return Err(EvaluationError::MathError(
            "clip needs an x-axis that runs one way; this sweep turns back".to_owned(),
        ));
    }
    let (low, high) = (from.min(to), from.max(to));
    let (first, last) = (x[0], x[x.len() - 1]);
    if high < first.min(last) || low > first.max(last) {
        return Err(EvaluationError::MathError(format!(
            "clip from {from} to {to} lies outside the trace, which runs from {first} to {last}"
        )));
    }
    Ok(super::series::Clipped::new(x.iter().copied().zip(y.iter().copied()), low, high).unzip())
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
/// - Non-finite samples pass through unchanged. Unwrapping restarts after
///   a gap because the missing interval's number of phase turns is unknown.
pub fn unwrap_phase_deg(phase: &[f64]) -> Vec<f64> {
    let mut out = Vec::with_capacity(phase.len());
    let mut offset = 0.0_f64;
    let mut prev: Option<f64> = None;
    for &sample in phase {
        if !sample.is_finite() {
            out.push(sample);
            prev = None;
            offset = 0.0;
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
pub(super) fn extremes(name: &str, y: &[f64]) -> Result<(f64, f64), EvaluationError> {
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

/// The lowest and highest value over a level measure's window: the
/// retained samples of a whole trace, or a window's samples and its
/// interpolated ends.
fn windowed_extremes(
    name: &str,
    args: &[CalcValue],
) -> Result<Option<(f64, f64)>, EvaluationError> {
    let window = level_window(name, args)?;
    match (&args[0], window) {
        (CalcValue::Scalar(_), _) => Ok(None),
        (CalcValue::Waveform(_, y), None) => extremes(name, y).map(Some),
        (CalcValue::Waveform(x, y), Some(window)) => {
            interval_statistics(name, x, y, Some(window)).map(|stats| Some((stats.min, stats.max)))
        }
    }
}

fn min(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    Ok(CalcValue::Scalar(match windowed_extremes("min", &args)? {
        Some((low, _)) => low,
        None => scalar_arg("min", "value", &args[0])?,
    }))
}

fn max(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    Ok(CalcValue::Scalar(match windowed_extremes("max", &args)? {
        Some((_, high)) => high,
        None => scalar_arg("max", "value", &args[0])?,
    }))
}

/// `pp(w)` — peak-to-peak excursion.
fn pp(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    Ok(CalcValue::Scalar(
        windowed_extremes("pp", &args)?.map_or(0.0, |(low, high)| high - low),
    ))
}

/// Linear interpolation inside the swept range. Callers check the range
/// first, because nothing in this module extrapolates.
///
/// The bracket search runs whichever way the grid does. A `.dc V1 5 0 -0.1`
/// sweep stores its samples with a *descending* x, and an ascending-only
/// search short-circuits on the very first comparison there, answering every
/// in-range query with the first stored sample. The arithmetic itself needs
/// no mirroring — `dx` is signed — so only the comparisons flip.
pub(super) fn interpolate(x: &[f64], y: &[f64], at: f64) -> f64 {
    let descending = x.len() > 1 && x[0] > x[x.len() - 1];
    let before = |a: f64, b: f64| if descending { a >= b } else { a <= b };
    if before(at, x[0]) {
        return y[0];
    }
    for i in 1..x.len() {
        if before(at, x[i]) {
            let dx = x[i] - x[i - 1];
            if dx == 0.0 {
                return y[i];
            }
            return y[i - 1] + (y[i] - y[i - 1]) * (at - x[i - 1]) / dx;
        }
    }
    y[y.len() - 1]
}

/// Refuse a domain that runs backwards.
///
/// Every measurement that reaches for this reads its x-axis as elapsed time:
/// "the first crossing", "the last excursion outside the band", "rising".
/// Run those over a reversed sweep — the grid a `.dc V1 5 0 -0.1` produces —
/// and each one answers in reverse: a rise reads as a fall, a delay comes
/// back negative, settling measures from the end. None of that is visible in
/// the single number the panel prints, so the sweep is refused instead.
///
/// Duplicated timepoints are not a reversal, so the test is `<`, not `<=`;
/// the aggregates (`avg`, `rms`) and the sample-wise transforms are absent
/// here because a signed width and a signed `dx` already carry the direction.
pub(super) fn forward_domain(name: &str, x: &[f64]) -> Result<(), EvaluationError> {
    match (1..x.len()).find(|&i| x[i] < x[i - 1]) {
        Some(index) => Err(EvaluationError::MathError(format!(
            "{name} reads its x-axis as elapsed time, but this sweep runs backwards \
             (x steps from {} to {}); reverse it before measuring",
            x[index - 1],
            x[index]
        ))),
        None => Ok(()),
    }
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

    /// The same curve swept the other way: a `.dc V1 5 0 -0.1` grid, whose
    /// domain steps backwards while the samples stay paired with their x.
    fn reversed(value: &CalcValue) -> CalcValue {
        match value {
            CalcValue::Waveform(x, y) => {
                let mut x = x.clone();
                let mut y = y.clone();
                x.reverse();
                y.reverse();
                wave(x, y)
            }
            other => other.clone(),
        }
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
    fn interval_rms_is_exact_for_piecewise_linear_retained_samples() {
        for (x, y, expected) in [
            (vec![0.0, 1.0], vec![0.0, 1.0], (1.0_f64 / 3.0).sqrt()),
            (
                vec![0.0, 0.99, 1.0],
                vec![1.0, 1.0, 0.0],
                (149.0_f64 / 150.0).sqrt(),
            ),
            (
                vec![1.0, 0.99, 0.0],
                vec![0.0, 1.0, 1.0],
                (149.0_f64 / 150.0).sqrt(),
            ),
        ] {
            assert_close(
                scalar_of("rms", vec![wave(x, y)]),
                expected,
                1.0e-14,
                "PWL RMS",
            );
        }
    }

    #[test]
    fn interval_aggregates_preserve_extreme_finite_signal_scales() {
        for value in [1.0e-300, 1.0e300, f64::MAX] {
            for name in ["avg", "rms"] {
                let result = scalar_of(name, vec![wave(vec![-f64::MAX, f64::MAX], vec![value; 2])]);
                assert!(result.is_finite());
                assert_close(result / value, 1.0, 1.0e-14, name);
            }
        }
    }

    #[test]
    fn interval_aggregates_refuse_to_merge_opposing_sweep_branches() {
        for name in ["avg", "rms"] {
            assert!(call(name, vec![wave(vec![0.0, 1.0, 0.0], vec![0.0, 1.0, 2.0])]).is_err());
        }
    }

    #[test]
    fn interval_aggregates_refuse_nonfinite_scalars() {
        for name in ["avg", "rms"] {
            for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                assert!(call(name, vec![CalcValue::Scalar(value)]).is_err());
            }
        }
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
    fn deriv_of_a_quadratic_is_exact_on_a_random_grid() {
        // A reproducible uneven grid: steps from a linear congruential
        // sequence, between 0.1 and 2.1 units.
        let mut state: u64 = 0x2545_f491_4f6c_dd1d;
        let mut t = 0.0;
        let mut x = Vec::new();
        for _ in 0..202 {
            x.push(t);
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            t += 0.1 + 2.0 * (state >> 11) as f64 / (1u64 << 53) as f64;
        }
        let y = x.iter().map(|t| t * t).collect();
        let (_, slope) = series_of("deriv", vec![wave(x.clone(), y)]);
        for (at, value) in x.iter().zip(slope) {
            let expected = 2.0 * at;
            // Exact up to rounding: the centred secant is off by the
            // difference of the two steps, up to two units here.
            assert_close(value, expected, 1.0e-9 * expected.max(1.0), "d(t²)/dt = 2t");
        }
    }

    #[test]
    fn deriv_of_an_rc_step_follows_its_closed_form_on_a_transient_grid() {
        // v = 1 − e^(−t/τ), sampled with steps that grow and shrink by up
        // to a factor of three, as a transient's do.
        let tau = 1.0e-3;
        let x = nonuniform(1_049, 0.0, 5.0e-3);
        let y = x.iter().map(|t| 1.0 - (-t / tau).exp()).collect();
        let (_, slope) = series_of("deriv", vec![wave(x.clone(), y)]);
        for (at, value) in x.iter().zip(slope) {
            let expected = (-at / tau).exp() / tau;
            assert_close(value, expected, 2.0e-5 * expected, "dv/dt = e^(−t/τ)/τ");
        }
    }

    #[test]
    fn clip_keeps_the_part_of_the_trace_between_two_x_points() {
        let trace = wave(vec![0.0, 1.0, 2.0, 3.0], vec![0.0, 10.0, 20.0, 30.0]);
        let clip = |from: f64, to: f64| {
            series_of(
                "clip",
                vec![
                    trace.clone(),
                    CalcValue::Scalar(from),
                    CalcValue::Scalar(to),
                ],
            )
        };
        // Ends between samples are interpolated on the trace.
        assert_eq!(
            clip(0.5, 2.25),
            (vec![0.5, 1.0, 2.0, 2.25], vec![5.0, 10.0, 20.0, 22.5])
        );
        // An end on a sample keeps that sample once; the order of the ends
        // does not matter.
        assert_eq!(clip(2.0, 1.0), (vec![1.0, 2.0], vec![10.0, 20.0]));
        // A backwards sweep keeps its direction.
        let (x, y) = series_of(
            "clip",
            vec![
                reversed(&trace),
                CalcValue::Scalar(0.5),
                CalcValue::Scalar(1.5),
            ],
        );
        assert_eq!((x, y), (vec![1.5, 1.0, 0.5], vec![15.0, 10.0, 5.0]));
        // A window beside the trace has nothing to keep.
        assert!(
            call(
                "clip",
                vec![
                    trace.clone(),
                    CalcValue::Scalar(4.0),
                    CalcValue::Scalar(5.0)
                ]
            )
            .is_err()
        );
        // A sweep that turns back has no single x range to keep.
        assert!(
            call(
                "clip",
                vec![
                    wave(vec![0.0, 1.0, 0.5], vec![0.0, 1.0, 2.0]),
                    CalcValue::Scalar(0.0),
                    CalcValue::Scalar(1.0)
                ]
            )
            .is_err()
        );
    }

    #[test]
    fn the_mean_of_a_clipped_rc_step_is_the_closed_form_mean_over_that_interval() {
        // RC step, τ = 1 ms: v = 1 − e^(−t/τ). Its mean from 1 ms to 2 ms is
        // 1 − τ (e^(−1) − e^(−2)) / 1 ms.
        let tau = 1.0e-3;
        let x = nonuniform(50_001, 0.0, 5.0e-3);
        let y = x.iter().map(|t| 1.0 - (-t / tau).exp()).collect();
        let clipped = call(
            "clip",
            vec![
                wave(x, y),
                CalcValue::Scalar(1.0e-3),
                CalcValue::Scalar(2.0e-3),
            ],
        )
        .unwrap();
        let expected = 1.0 - tau * ((-1.0f64).exp() - (-2.0f64).exp()) / 1.0e-3;
        assert!((expected - 0.767).abs() < 1.0e-3, "closed form {expected}");
        assert_close(
            scalar_of("avg", vec![clipped]),
            expected,
            1.0e-9,
            "avg(clip(v, 1m, 2m))",
        );
    }

    #[test]
    fn limit_holds_each_sample_between_two_bounds_in_either_order() {
        for (low, high) in [(-1.0, 1.0), (1.0, -1.0)] {
            let (_, y) = series_of(
                "limit",
                vec![
                    wave(vec![0.0, 1.0, 2.0], vec![-5.0, 0.5, 9.0]),
                    CalcValue::Scalar(low),
                    CalcValue::Scalar(high),
                ],
            );
            assert_eq!(y, vec![-1.0, 0.5, 1.0], "limit(x, {low}, {high})");
        }
    }

    #[test]
    fn db10_reads_a_power_and_db20_an_amplitude() {
        // ViVA's own example: 371.875 mV reads −4.296 dB10 and −8.5921 dB20.
        let sample = 0.371_875;
        assert_close(
            scalar_of("dB10", vec![CalcValue::Scalar(sample)]),
            10.0 * sample.log10(),
            1.0e-12,
            "dB10",
        );
        assert_close(
            scalar_of("dB10", vec![CalcValue::Scalar(sample)]),
            -4.296,
            5.0e-4,
            "dB10 of ViVA's sample",
        );
        assert_close(
            scalar_of("dB20", vec![CalcValue::Scalar(-sample)]),
            -8.5921,
            5.0e-5,
            "dB20 of ViVA's sample",
        );
        // A 1 mW power is −30 dB10 (dBW); dB() stays the amplitude form.
        assert_close(
            scalar_of("dB10", vec![CalcValue::Scalar(1.0e-3)]),
            -30.0,
            1.0e-12,
            "dBW",
        );
        assert_close(
            scalar_of("dB", vec![CalcValue::Scalar(1.0e-3)]),
            -60.0,
            1.0e-12,
            "dB",
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
    fn yval_reads_a_backwards_sweep_off_the_same_curve() {
        // A `.dc V1 5 0 -0.1` grid: x descends. `yval` already normalizes its
        // range check for this, so an in-range query must land on the curve
        // rather than short-circuiting to the first stored sample.
        let x = vec![5.0, 4.0, 3.0, 2.0, 1.0, 0.0];
        let y: Vec<f64> = x.iter().map(|v| 10.0 * v).collect();
        for (at, want) in [(2.5, 25.0), (4.25, 42.5), (5.0, 50.0), (0.0, 0.0)] {
            assert_close(
                scalar_of(
                    "yval",
                    vec![wave(x.clone(), y.clone()), CalcValue::Scalar(at)],
                ),
                want,
                1.0e-12,
                &format!("yval at {at} on a descending sweep"),
            );
        }
        assert!(
            call(
                "yval",
                vec![wave(x.clone(), y.clone()), CalcValue::Scalar(6.0)]
            )
            .is_err(),
            "yval never extrapolates, whichever way the sweep runs"
        );

        // The window aggregates divide a signed integral by a signed width,
        // so a reversed sweep is the same mean rather than an error.
        assert_close(
            scalar_of("avg", vec![wave(x, y)]),
            25.0,
            1.0e-12,
            "avg of a descending sweep",
        );
    }

    #[test]
    fn time_ordered_measurements_refuse_a_backwards_sweep() {
        // Everything below reads the domain as elapsed time — "the first
        // crossing", "the last excursion", "rising". On a reversed grid each
        // of those answers comes out backwards, so the measurement must
        // refuse rather than report a plausible wrong number.
        let train = pulse_train();
        let (rx, ry) = pwl_series(&[(0.0, 0.0), (1.0, 10.0), (2.0, 10.0)], 19);
        let rising_ramp = wave(rx, ry);
        let (fx, fy) = pwl_series(&[(0.0, 10.0), (1.0, 10.0), (2.0, 0.0)], 19);
        let falling_ramp = wave(fx, fy);
        let (sx, sy) = pwl_series(
            &[(0.0, 0.0), (1.0, 1.2), (2.0, 0.9), (3.0, 1.0), (4.0, 1.0)],
            23,
        );
        let step = wave(sx, sy);

        let cases: Vec<(&str, Vec<CalcValue>)> = vec![
            (
                "cross",
                vec![
                    train.clone(),
                    CalcValue::Scalar(5.0),
                    CalcValue::Scalar(1.0),
                ],
            ),
            ("freq", vec![train.clone()]),
            ("period", vec![train.clone()]),
            ("duty", vec![train]),
            ("rise", vec![rising_ramp.clone()]),
            ("fall", vec![falling_ramp]),
            ("overshoot", vec![step.clone()]),
            ("settling", vec![step, CalcValue::Scalar(5.0)]),
            ("delay", vec![rising_ramp.clone(), rising_ramp]),
        ];
        for (name, args) in cases {
            assert!(
                call(name, args.clone()).is_ok(),
                "{name} must answer on the forward sweep, or the reversal below proves nothing"
            );
            let backwards: Vec<CalcValue> = args.iter().map(reversed).collect();
            let outcome = call(name, backwards);
            assert!(
                outcome.is_err(),
                "{name} must refuse a reversed sweep, got {outcome:?}"
            );
        }
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
    fn settling_reports_the_last_entry_into_the_band() {
        // Step 0 → 1. |y − 1| falls linearly from 0.05 at x = 1 to 0 at x = 2,
        // so a 2 % band (of the 1.0 step) is entered at x = 1.6 and never
        // left again — the monotone case, where first and last entry agree.
        let (x, y) = pwl_series(&[(0.0, 0.0), (1.0, 1.05), (2.0, 1.0), (3.0, 1.0)], 17);
        assert_close(
            scalar_of("settling", vec![wave(x, y), CalcValue::Scalar(2.0)]),
            1.6,
            1.0e-12,
            "settling time to a 2 % band",
        );
    }

    #[test]
    fn settling_ignores_an_early_pass_through_the_band_of_a_ringing_step() {
        // Step 0 → 1 that overshoots to 1.10, falls back *through* the 2 %
        // band (the sample at x = 1.5 sits exactly on 1.00), re-exits below
        // it to 0.94, and only then approaches. Settling is the LAST entry:
        // the deviation runs −0.06 at x = 2 to −0.01 at x = 3, reaching the
        // −0.02 band edge at x = 2 + 0.04/0.05 = 2.8. A first-entry reading
        // answers 1.4 — the fall back through the band — instead, and the
        // true entry falls strictly between two retained samples, so
        // snapping to the first inside sample (x = 3) is wrong too.
        let ringing = wave(
            vec![0.0, 0.4, 1.0, 1.5, 2.0, 3.0, 4.0],
            vec![0.0, 0.6, 1.10, 1.00, 0.94, 0.99, 1.00],
        );
        assert_close(
            scalar_of("settling", vec![ringing, CalcValue::Scalar(2.0)]),
            2.8,
            1.0e-12,
            "settling of a ringing step",
        );
    }

    #[test]
    fn settling_interpolates_toward_the_band_edge_it_actually_crosses() {
        // The last excursion, at x = 2, sits 0.05 *above* the settled 1.0;
        // the next sample, at x = 3, sits 0.01 *below* it. The band edge the
        // signal crosses is therefore the upper one, reached where the signed
        // deviation passes +0.02: x = 2 + (0.05 − 0.02)/(0.05 + 0.01) = 2.5.
        // Interpolating on unsigned magnitudes instead — 0.05 down to 0.01 —
        // reports the entry late, at 2.75.
        let flyback = wave(
            vec![0.0, 0.5, 1.25, 2.0, 3.0, 4.0],
            vec![0.0, 0.6, 1.10, 1.05, 0.99, 1.00],
        );
        assert_close(
            scalar_of("settling", vec![flyback, CalcValue::Scalar(2.0)]),
            2.5,
            1.0e-12,
            "settling across the band edge",
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

    /// A NaN parameter is refused by the guard that owns it, not absorbed.
    ///
    /// These two guards are written `x.is_nan() || x < bound` rather than
    /// `!(x >= bound)`. The two forms agree exactly, and this pins the part
    /// that is easy to lose: it is the NaN arm, not the comparison, that
    /// rejects NaN. Dropping it — writing a bare `x < bound` — lets NaN
    /// through every one of them, and none fails loudly afterwards. `cross`
    /// would cast NaN to `0usize`, and `settling` would measure
    /// against a NaN tolerance that no sample can be inside and report a
    /// settling time it never found. Asserting on the message rather than on
    /// `is_err` keeps a later guard from passing this test by accident.
    #[test]
    fn a_nan_parameter_is_refused_by_its_own_guard() {
        let (x, y) = pwl_series(&[(0.0, 0.0), (1.0, 2.0), (3.0, -2.0), (4.0, 0.0)], 11);
        let series = wave(x, y);
        for (name, args, expected) in [
            (
                "cross",
                vec![
                    series.clone(),
                    CalcValue::Scalar(1.0),
                    CalcValue::Scalar(f64::NAN),
                ],
                "cross ordinal must be",
            ),
            (
                "settling",
                vec![series.clone(), CalcValue::Scalar(f64::NAN)],
                "settling band must be",
            ),
        ] {
            let outcome = call(name, args);
            let message = match &outcome {
                Err(error) => error.to_string(),
                Ok(value) => panic!("{name} measured a NaN parameter and returned {value:?}"),
            };
            assert!(
                message.contains(expected),
                "{name} refused a NaN parameter, but not at its own guard: {message}"
            );
        }
    }

    #[test]
    fn measurements_reject_a_series_with_nothing_to_measure() {
        for name in ["min", "max", "pp", "freq", "period", "duty", "rise", "fall"] {
            assert!(
                call(name, vec![wave(vec![], vec![])]).is_err(),
                "{name} of an empty series must be an error"
            );
        }
    }

    /// A level measure reads a window between two points of the x-axis,
    /// its ends interpolated: a ramp 0..10 over 0..10 s averages 5 between
    /// 4 and 6 s, peaks at 6, bottoms at 4 and spans 2, whatever samples
    /// fall at the ends.
    #[test]
    fn level_measures_read_a_window_with_interpolated_ends() {
        let ramp = || wave(vec![0.0, 3.0, 7.0, 10.0], vec![0.0, 3.0, 7.0, 10.0]);
        let window = |name: &str| {
            scalar_of(
                name,
                vec![ramp(), CalcValue::Scalar(4.0), CalcValue::Scalar(6.0)],
            )
        };
        assert!((window("avg") - 5.0).abs() < 1e-12);
        assert!((window("min") - 4.0).abs() < 1e-12);
        assert!((window("max") - 6.0).abs() < 1e-12);
        assert!((window("pp") - 2.0).abs() < 1e-12);
        // sqrt of the mean of t^2 over [4, 6]: (216 - 64) / 3 / 2.
        assert!((window("rms") - (152.0_f64 / 6.0).sqrt()).abs() < 1e-12);
        // Without a window the whole trace is read.
        assert!((scalar_of("avg", vec![ramp()]) - 5.0).abs() < 1e-12);
        assert!((scalar_of("pp", vec![ramp()]) - 10.0).abs() < 1e-12);
        // A window needs both its ends.
        assert!(call("avg", vec![ramp(), CalcValue::Scalar(4.0)]).is_err());
    }
}
