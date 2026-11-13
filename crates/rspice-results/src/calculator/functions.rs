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
use std::f64::consts::PI;

mod conversion;
pub use conversion::{EventWord, analog_staircase};

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
            "thd" => thd(args),

            // Analog-to-digital and digital-to-analog conversion
            "digital" => conversion::digital(args),
            "analog" => conversion::analog(args),

            _ => match super::time_measures::lookup(&name.to_lowercase()) {
                // Edge, pulse and step measurements take option words, so
                // they live with the argument handling that reads them.
                Some(measure) => {
                    let args: Vec<Argument> = args.into_iter().map(Argument::from).collect();
                    measure(name, &args).map(CalcValue::Scalar)
                }
                None => Err(EvaluationError::UnknownFunction(name.to_string())),
            },
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

/// How many harmonics `thd` weighs when the caller does not say.
const DEFAULT_THD_HARMONICS: usize = 10;

/// The auto detector needs enough cycles to separate a fundamental from its
/// neighbours under its Hann discovery window. Calls with an explicit `f0`
/// need only one complete cycle and do not pass through the detector.
const MIN_AUTO_THD_CYCLES: f64 = 3.0;
const THD_SEARCH_START_CYCLES: f64 = MIN_AUTO_THD_CYCLES - 0.5;

/// Eight trial frequencies per `1 / record_length` resolution interval keeps
/// the parabolic peak refinement comfortably inside its three-point bracket.
const THD_SEARCH_OVERSAMPLING: f64 = 8.0;

/// A Hann-windowed sinusoid's first sidelobe is about 2.7 % in amplitude. A
/// one-percent floor therefore needs the main-lobe exclusion below before it
/// can be treated as independent evidence of another periodicity.
const THD_AMBIGUITY_FLOOR: f64 = 0.01;
const THD_STRONG_PEAK_FLOOR: f64 = 0.05;
const THD_HANN_MAIN_LOBE_BINS: f64 = 3.0;
const THD_HARMONIC_ALIGNMENT_BINS: f64 = 0.35;

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

#[derive(Clone, Copy)]
struct SpectralPeak {
    /// Frequency multiplied by the complete input-record duration. Expressing
    /// it in resolution bins avoids repeatedly multiplying and dividing by a
    /// possibly very large or very small physical time unit.
    cycles: f64,
    amplitude: f64,
}

/// Add one value with Neumaier compensation. The projections below can span
/// thousands of panels whose positive and negative contributions nearly
/// cancel, exactly the case a plain left-to-right sum handles least well.
fn compensated_add(sum: &mut f64, correction: &mut f64, value: f64) {
    let next = *sum + value;
    if sum.abs() >= value.abs() {
        *correction += (*sum - next) + value;
    } else {
        *correction += (value - next) + *sum;
    }
    *sum = next;
}

/// The two basis integrals needed to integrate a line segment against a
/// complex exponential. Series expansions avoid cancellation when a solver
/// has placed two timepoints very close together.
fn linear_basis_integrals(phase_span: f64) -> (f64, f64) {
    let z2 = phase_span * phase_span;
    if phase_span.abs() < 1.0e-3 {
        let z4 = z2 * z2;
        (
            1.0 - z2 / 24.0 + z4 / 1_920.0,
            phase_span / 12.0 - phase_span * z2 / 480.0 + phase_span * z4 / 53_760.0,
        )
    } else {
        let half = phase_span * 0.5;
        (
            half.sin() / half,
            (2.0 * half.sin() - phase_span * half.cos()) / z2,
        )
    }
}

/// Exact Fourier projection of the piecewise-linear waveform represented by
/// `values` on the normalized `[0, 1]` domain. The result is the integral,
/// not an amplitude; callers apply the factor of two where appropriate.
fn pwl_projection(unit_x: &[f64], values: &[f64], cycles: f64) -> (f64, f64) {
    let angular_cycles = 2.0 * PI * cycles;
    let mut real = 0.0;
    let mut real_correction = 0.0;
    let mut imag = 0.0;
    let mut imag_correction = 0.0;
    for index in 1..unit_x.len() {
        let from = unit_x[index - 1];
        let to = unit_x[index];
        let width = to - from;
        let phase_span = angular_cycles * width;
        let (constant_basis, slope_basis) = linear_basis_integrals(phase_span);
        let average = (values[index - 1] + values[index]) * 0.5;
        let difference = values[index] - values[index - 1];
        let constant = average * constant_basis;
        let slope = difference * slope_basis;
        let phase = angular_cycles * (from + to) * 0.5;
        let (sine, cosine) = phase.sin_cos();
        compensated_add(
            &mut real,
            &mut real_correction,
            width * (constant * cosine - slope * sine),
        );
        compensated_add(
            &mut imag,
            &mut imag_correction,
            width * (constant * sine + slope * cosine),
        );
    }
    (real + real_correction, imag + imag_correction)
}

/// Normalize time and signal magnitude before spectral arithmetic. THD is
/// scale invariant, so this prevents a representable input near `f64::MAX`
/// from overflowing a projection and preserves tiny harmonics beside it.
fn normalized_thd_signal(x: &[f64], y: &[f64]) -> Result<(Vec<f64>, Vec<f64>), EvaluationError> {
    let span = x[x.len() - 1] - x[0];
    if !span.is_finite() || span <= 0.0 {
        return Err(EvaluationError::MathError(
            "thd needs a positive, representable observation interval".to_owned(),
        ));
    }
    let scale = y
        .iter()
        .fold(0.0_f64, |largest, value| largest.max(value.abs()));
    if scale == 0.0 {
        return Err(EvaluationError::MathError(
            "thd found no AC signal to measure".to_owned(),
        ));
    }
    let unit_x: Vec<f64> = x.iter().map(|time| (*time - x[0]) / span).collect();
    let mut normalized: Vec<f64> = y.iter().map(|value| *value / scale).collect();
    let mut mean = 0.0;
    let mut correction = 0.0;
    for index in 1..unit_x.len() {
        compensated_add(
            &mut mean,
            &mut correction,
            (normalized[index - 1] + normalized[index]) * 0.5 * (unit_x[index] - unit_x[index - 1]),
        );
    }
    let mean = mean + correction;
    normalized.iter_mut().for_each(|value| *value -= mean);
    Ok((unit_x, normalized))
}

/// Validate the sampling contract and return the record span and the most
/// conservative Nyquist bound for a non-uniform grid. The largest gap is the
/// interval over which the retained PWL signal carries the least information.
fn thd_sampling_limits(x: &[f64], y: &[f64]) -> Result<(f64, f64), EvaluationError> {
    if x.len() < 3 {
        return Err(EvaluationError::MathError(
            "thd needs at least three time samples".to_owned(),
        ));
    }
    if let Some(index) = x.iter().position(|value| !value.is_finite()) {
        return Err(EvaluationError::MathError(format!(
            "thd time sample {index} is not finite"
        )));
    }
    if let Some(index) = y.iter().position(|value| !value.is_finite()) {
        return Err(EvaluationError::MathError(format!(
            "thd waveform sample {index} is not finite"
        )));
    }
    let mut largest_step = 0.0_f64;
    for index in 1..x.len() {
        let step = x[index] - x[index - 1];
        if !step.is_finite() || step <= 0.0 {
            return Err(EvaluationError::MathError(format!(
                "thd needs strictly increasing finite time samples; samples {} and {index} are {} and {}",
                index - 1,
                x[index - 1],
                x[index]
            )));
        }
        largest_step = largest_step.max(step);
    }
    let span = x[x.len() - 1] - x[0];
    if !span.is_finite() || span <= 0.0 {
        return Err(EvaluationError::MathError(
            "thd needs a positive, representable observation interval".to_owned(),
        ));
    }
    let nyquist = 0.5 / largest_step;
    if !nyquist.is_finite() || nyquist <= 0.0 {
        return Err(EvaluationError::MathError(
            "thd could not represent the sampling-grid Nyquist frequency".to_owned(),
        ));
    }
    Ok((span, nyquist))
}

fn parse_thd_harmonics(value: Option<&CalcValue>) -> Result<usize, EvaluationError> {
    let Some(value) = value else {
        return Ok(DEFAULT_THD_HARMONICS);
    };
    let count = scalar_arg("thd", "harmonic count", value)?;
    if !count.is_finite() || count < 2.0 || count.fract() != 0.0 || count >= usize::MAX as f64 {
        return Err(EvaluationError::MathError(
            "thd harmonic count must be a finite whole number of 2 or more".to_owned(),
        ));
    }
    Ok(count as usize)
}

fn spectral_amplitude(unit_x: &[f64], windowed: &[f64], cycles: f64) -> f64 {
    let (real, imag) = pwl_projection(unit_x, windowed, cycles);
    real.hypot(imag)
}

/// Refine a bracketed local maximum. The coarse grid exists to find the right
/// lobe; repeated ternary subdivision then avoids biasing the integer-cycle
/// measurement window with a resolution-bin-rounded `f0`.
fn refine_spectral_peak(unit_x: &[f64], windowed: &[f64], centre_cycles: f64) -> SpectralPeak {
    let half_step = 1.0 / THD_SEARCH_OVERSAMPLING;
    let mut low = centre_cycles - half_step;
    let mut high = centre_cycles + half_step;
    for _ in 0..32 {
        let third = (high - low) / 3.0;
        let left = low + third;
        let right = high - third;
        if spectral_amplitude(unit_x, windowed, left) < spectral_amplitude(unit_x, windowed, right)
        {
            low = left;
        } else {
            high = right;
        }
    }
    let cycles = (low + high) * 0.5;
    SpectralPeak {
        cycles,
        amplitude: spectral_amplitude(unit_x, windowed, cycles),
    }
}

fn harmonically_aligned(base: f64, candidate: f64) -> bool {
    let order = (candidate / base).round().max(1.0);
    (candidate - order * base).abs() <= THD_HARMONIC_ALIGNMENT_BINS
}

fn normalized_value_at(unit_x: &[f64], values: &[f64], at: f64) -> f64 {
    let upper = unit_x.partition_point(|position| *position <= at);
    if upper == 0 {
        return values[0];
    }
    if upper == unit_x.len() {
        return values[values.len() - 1];
    }
    let lower = upper - 1;
    let fraction = (at - unit_x[lower]) / (unit_x[upper] - unit_x[lower]);
    values[lower] + (values[upper] - values[lower]) * fraction
}

/// Integral of `(w(t + period) - w(t))²` across their common domain. The
/// integration grid is the union of both PWL knot sets, so this is exact for
/// the retained waveform rather than a sample-count autocorrelation.
fn periodic_mismatch(unit_x: &[f64], values: &[f64], cycles: f64) -> f64 {
    let period = cycles.recip();
    let end = 1.0 - period;
    let mut left_knot = 1usize;
    let mut shifted_knot = unit_x.partition_point(|position| *position <= period);
    let mut at = 0.0;
    let mut area = 0.0;
    let mut correction = 0.0;
    while at < end {
        while left_knot < unit_x.len() && unit_x[left_knot] <= at {
            left_knot += 1;
        }
        while shifted_knot < unit_x.len() && unit_x[shifted_knot] - period <= at {
            shifted_knot += 1;
        }
        let left_boundary = unit_x.get(left_knot).copied().unwrap_or(end);
        let shifted_boundary = unit_x
            .get(shifted_knot)
            .map(|position| *position - period)
            .unwrap_or(end);
        let next = end.min(left_boundary).min(shifted_boundary);
        if next <= at {
            // The strict input grid makes this reachable only through a
            // rounded shifted knot coinciding with `at`; advance its owner.
            if left_knot < unit_x.len() && left_boundary <= at {
                left_knot += 1;
            }
            if shifted_knot < unit_x.len() && shifted_boundary <= at {
                shifted_knot += 1;
            }
            continue;
        }
        let difference_at = normalized_value_at(unit_x, values, at + period)
            - normalized_value_at(unit_x, values, at);
        let difference_next = normalized_value_at(unit_x, values, next + period)
            - normalized_value_at(unit_x, values, next);
        let panel = (next - at)
            * (difference_at * difference_at
                + difference_at * difference_next
                + difference_next * difference_next)
            / 3.0;
        compensated_add(&mut area, &mut correction, panel);
        at = next;
    }
    area + correction
}

/// The Hann spectrum identifies the correct lobe and harmonic family. A
/// shift-consistency fit then removes the small finite-record frequency bias
/// of any one-sided spectral peak (including its negative-frequency image).
fn refine_periodicity(unit_x: &[f64], values: &[f64], centre_cycles: f64) -> f64 {
    let mut low = centre_cycles - 2.0 / THD_SEARCH_OVERSAMPLING;
    let mut high = centre_cycles + 2.0 / THD_SEARCH_OVERSAMPLING;
    for _ in 0..36 {
        let third = (high - low) / 3.0;
        let left = low + third;
        let right = high - third;
        if periodic_mismatch(unit_x, values, left) < periodic_mismatch(unit_x, values, right) {
            high = right;
        } else {
            low = left;
        }
    }
    (low + high) * 0.5
}

/// Estimate a fundamental from a Hann-windowed, DC-removed spectrum. The
/// detector accepts only a single harmonic family. Independent subharmonic,
/// intermodulation, or incommensurate peaks are an ambiguity error; callers
/// can resolve that intentionally with `thd(w, n, f0)`.
fn estimate_thd_fundamental(
    x: &[f64],
    y: &[f64],
    span: f64,
    nyquist: f64,
    harmonics: usize,
) -> Result<f64, EvaluationError> {
    let (unit_x, normalized) = normalized_thd_signal(x, y)?;
    let windowed: Vec<f64> = unit_x
        .iter()
        .zip(normalized.iter().copied())
        .map(|(position, value)| value * (PI * position).sin().powi(2))
        .collect();
    let maximum_cycles = nyquist * span / harmonics as f64;
    if !maximum_cycles.is_finite() || maximum_cycles <= MIN_AUTO_THD_CYCLES {
        return Err(EvaluationError::MathError(format!(
            "thd cannot auto-estimate a fundamental with {harmonics} harmonics: the record and sampling grid provide insufficient frequency resolution"
        )));
    }
    let scan_intervals =
        ((maximum_cycles - THD_SEARCH_START_CYCLES) * THD_SEARCH_OVERSAMPLING).floor();
    if !scan_intervals.is_finite() || scan_intervals < 2.0 || scan_intervals >= usize::MAX as f64 {
        return Err(EvaluationError::MathError(
            "thd automatic frequency search is not representable".to_owned(),
        ));
    }
    let sample_count = scan_intervals as usize + 1;
    let mut spectrum = Vec::new();
    spectrum.try_reserve_exact(sample_count).map_err(|_| {
        EvaluationError::MathError("thd could not allocate its frequency search".to_owned())
    })?;
    for index in 0..sample_count {
        let cycles = THD_SEARCH_START_CYCLES + index as f64 / THD_SEARCH_OVERSAMPLING;
        let amplitude = spectral_amplitude(&unit_x, &windowed, cycles);
        if !amplitude.is_finite() {
            return Err(EvaluationError::MathError(
                "thd automatic frequency search produced a non-finite spectrum".to_owned(),
            ));
        }
        spectrum.push(amplitude);
    }
    let maximum = spectrum.iter().copied().fold(0.0_f64, f64::max);
    if maximum == 0.0 {
        return Err(EvaluationError::MathError(
            "thd found no resolvable periodic component".to_owned(),
        ));
    }
    let mut peaks = Vec::new();
    peaks.try_reserve_exact(spectrum.len() / 2).map_err(|_| {
        EvaluationError::MathError("thd could not allocate its spectral peaks".to_owned())
    })?;
    for index in 1..spectrum.len() - 1 {
        if spectrum[index] >= spectrum[index - 1]
            && spectrum[index] > spectrum[index + 1]
            && spectrum[index] >= maximum * THD_AMBIGUITY_FLOOR
        {
            peaks.push(refine_spectral_peak(
                &unit_x,
                &windowed,
                THD_SEARCH_START_CYCLES + index as f64 / THD_SEARCH_OVERSAMPLING,
            ));
        }
    }
    let strong: Vec<SpectralPeak> = peaks
        .iter()
        .copied()
        .filter(|peak| peak.amplitude >= maximum * THD_STRONG_PEAK_FLOOR)
        .collect();
    let Some(base) = strong.iter().min_by(|a, b| a.cycles.total_cmp(&b.cycles)) else {
        return Err(EvaluationError::MathError(
            "thd could not resolve an interior fundamental peak; provide f0 explicitly".to_owned(),
        ));
    };
    if base.cycles < MIN_AUTO_THD_CYCLES {
        return Err(EvaluationError::MathError(
            "thd automatic fundamental needs at least three observed cycles; provide f0 explicitly"
                .to_owned(),
        ));
    }
    if strong
        .iter()
        .any(|peak| !harmonically_aligned(base.cycles, peak.cycles))
    {
        return Err(EvaluationError::MathError(
            "thd automatic fundamental is ambiguous: significant peaks do not form one harmonic family; provide f0 explicitly"
                .to_owned(),
        ));
    }
    // Reject an independent weaker peak too. Peaks inside a strong peak's
    // three-bin Hann main lobe are spectral leakage, not a second periodicity.
    if peaks.iter().any(|peak| {
        let belongs_to_main_lobe = strong
            .iter()
            .any(|strong_peak| (peak.cycles - strong_peak.cycles).abs() <= THD_HANN_MAIN_LOBE_BINS);
        !belongs_to_main_lobe && !harmonically_aligned(base.cycles, peak.cycles)
    }) {
        return Err(EvaluationError::MathError(
            "thd automatic fundamental is ambiguous: a subharmonic or unrelated spectral peak is present; provide f0 explicitly"
                .to_owned(),
        ));
    }
    let frequency = refine_periodicity(&unit_x, &normalized, base.cycles) / span;
    if !frequency.is_finite() || frequency <= 0.0 {
        return Err(EvaluationError::MathError(
            "thd automatic fundamental is not representable; provide f0 explicitly".to_owned(),
        ));
    }
    Ok(frequency)
}

fn measure_thd(
    x: &[f64],
    y: &[f64],
    harmonics: usize,
    fundamental: f64,
    span: f64,
    nyquist: f64,
) -> Result<f64, EvaluationError> {
    if !fundamental.is_finite() || fundamental <= 0.0 {
        return Err(EvaluationError::MathError(
            "thd fundamental frequency f0 must be finite and positive".to_owned(),
        ));
    }
    let observed_cycles = fundamental * span;
    if !observed_cycles.is_finite() || observed_cycles < 1.0 {
        return Err(EvaluationError::MathError(format!(
            "thd needs at least one complete cycle at f0 = {fundamental} within the observation interval"
        )));
    }
    let highest_harmonic = fundamental * harmonics as f64;
    if !highest_harmonic.is_finite() || highest_harmonic >= nyquist {
        return Err(EvaluationError::MathError(format!(
            "thd harmonic {harmonics} at {highest_harmonic} is not below the conservative sampling-grid Nyquist frequency {nyquist}"
        )));
    }
    let complete_cycles = observed_cycles.floor();
    let mut length = complete_cycles / fundamental;
    let span_tolerance = 4.0 * f64::EPSILON * span;
    if length > span && length - span <= span_tolerance {
        length = span;
    }
    if !length.is_finite() || length <= 0.0 || length > span {
        return Err(EvaluationError::MathError(
            "thd complete-cycle measurement window is not representable".to_owned(),
        ));
    }
    let from = x[0];
    let mut to = from + length;
    let axis_scale = from.abs().max(x[x.len() - 1].abs()).max(span);
    let endpoint_tolerance = 4.0 * f64::EPSILON * axis_scale;
    if to > x[x.len() - 1] && to - x[x.len() - 1] <= endpoint_tolerance {
        to = x[x.len() - 1];
    }
    if !to.is_finite() || to <= from || to > x[x.len() - 1] {
        return Err(EvaluationError::MathError(
            "thd complete-cycle measurement window cannot be represented on this time axis"
                .to_owned(),
        ));
    }
    let (window_x, window_y) = window(x, y, from, to);
    let (unit_x, normalized) = normalized_thd_signal(&window_x, &window_y)?;
    let mut harmonic_rss = 0.0_f64;
    let mut fundamental_amplitude = 0.0;
    for order in 1..=harmonics {
        let cycles = complete_cycles * order as f64;
        if !cycles.is_finite() {
            return Err(EvaluationError::MathError(
                "thd harmonic phase is not representable".to_owned(),
            ));
        }
        let (cosine, sine) = pwl_projection(&unit_x, &normalized, cycles);
        let amplitude = 2.0 * cosine.hypot(sine);
        if !amplitude.is_finite() {
            return Err(EvaluationError::MathError(format!(
                "thd harmonic {order} amplitude is not representable"
            )));
        }
        if order == 1 {
            fundamental_amplitude = amplitude;
        } else {
            harmonic_rss = harmonic_rss.hypot(amplitude);
        }
    }
    if fundamental_amplitude == 0.0 {
        return Err(EvaluationError::MathError(
            "thd found no fundamental to compare the harmonics against".to_owned(),
        ));
    }
    let percent = if harmonic_rss < fundamental_amplitude {
        // Multiplication first preserves a subnormal ratio whose percentage
        // is representable. Normalized amplitudes are at most order unity,
        // so multiplying the smaller operand by 100 cannot overflow.
        (harmonic_rss * 100.0) / fundamental_amplitude
    } else {
        // Division first avoids overflowing the numerator when distortion is
        // large; a non-representable final percentage is rejected below.
        (harmonic_rss / fundamental_amplitude) * 100.0
    };
    if !percent.is_finite() {
        return Err(EvaluationError::MathError(
            "thd percentage is not representable".to_owned(),
        ));
    }
    Ok(percent)
}

/// `thd(w)` — total harmonic distortion in percent; `thd(w, n)` weighs `n`
/// harmonics instead of [`DEFAULT_THD_HARMONICS`], and `thd(w, n, f0)` uses
/// the caller's explicit fundamental frequency.
///
/// All forms require finite waveform values on a finite, strictly increasing
/// (but not necessarily uniform) time grid. The requested top harmonic must
/// be below the conservative Nyquist limit implied by the grid's largest
/// sample gap, and the record must hold at least one complete `f0` cycle.
///
/// The one- and two-argument forms estimate `f0` from a DC-removed Hann
/// spectrum and refuse records with an unresolved, subharmonic, or unrelated
/// competing peak. The Hann window is used only to identify `f0`. The final
/// amplitudes use exact piecewise-linear integration over a rectangular,
/// integer-cycle window beginning at the first sample. Its time-weighted DC
/// mean is removed before projection, so DC offset does not leak into the
/// reported harmonics. Supply `f0` when the record is intentionally multitone
/// or when its fundamental is known more accurately than the record permits.
fn thd(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_range("thd", &args, 1, 3)?;
    let (x, y) = series_arg("thd", &args[0])?;
    let harmonics = parse_thd_harmonics(args.get(1))?;
    let (span, nyquist) = thd_sampling_limits(x, y)?;
    let fundamental = match args.get(2) {
        Some(value) => scalar_arg("thd", "fundamental frequency f0", value)?,
        None => estimate_thd_fundamental(x, y, span, nyquist, harmonics)?,
    };
    Ok(CalcValue::Scalar(measure_thd(
        x,
        y,
        harmonics,
        fundamental,
        span,
        nyquist,
    )?))
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
            ("duty", vec![train.clone()]),
            // This fixture's largest retained gap cannot resolve the default
            // ten harmonics below Nyquist; two are enough for this test's
            // forward-versus-reversed domain assertion.
            ("thd", vec![train, CalcValue::Scalar(2.0)]),
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
    fn thd_explicit_fundamental_handles_dc_scale_and_a_strong_second_harmonic() {
        // The non-uniform grid, huge signal scale, and DC offset exercise the
        // documented normalization and leakage policy. The AC signal is
        // sin(2πt) + 0.8 cos(4πt), so THD relative to f0 = 1 Hz is 80 %.
        let x = nonuniform(6001, 0.0, 4.0);
        let y: Vec<f64> = x
            .iter()
            .map(|time| 1.0e300 * (0.25 + (2.0 * PI * time).sin() + 0.8 * (4.0 * PI * time).cos()))
            .collect();
        assert_close(
            scalar_of(
                "thd",
                vec![wave(x, y), CalcValue::Scalar(5.0), CalcValue::Scalar(1.0)],
            ),
            80.0,
            1.0e-3,
            "explicit-f0 THD with DC offset and a large signal scale",
        );
    }

    #[test]
    fn thd_auto_estimator_keeps_a_strong_second_harmonic_out_of_the_fundamental() {
        // Mean-level crossing counts see extra crossings once the second
        // harmonic dominates. The spectral estimator must still select the
        // lowest member of the one coherent harmonic family.
        let x = nonuniform(6001, 0.0, 6.0);
        let y: Vec<f64> = x
            .iter()
            .map(|time| (2.0 * PI * time).sin() + 1.5 * (4.0 * PI * time).sin())
            .collect();
        assert_close(
            scalar_of("thd", vec![wave(x, y), CalcValue::Scalar(4.0)]),
            150.0,
            0.02,
            "auto-estimated THD with a dominant second harmonic",
        );
    }

    #[test]
    fn thd_auto_estimator_refuses_an_ambiguous_missing_subharmonic() {
        // Peaks at 1 Hz and 1.5 Hz could be the second and third harmonics of
        // an absent 0.5 Hz component, or two intentional tones. Choosing 1 Hz
        // would print a plausible but unjustified answer, so auto mode must
        // ask for the explicit-f0 form instead.
        let x = nonuniform(8001, 0.0, 8.0);
        let y: Vec<f64> = x
            .iter()
            .map(|time| (2.0 * PI * time).sin() + 0.7 * (2.0 * PI * 1.5 * time).sin())
            .collect();
        let error = call("thd", vec![wave(x, y), CalcValue::Scalar(4.0)])
            .expect_err("an absent subharmonic makes automatic f0 ambiguous")
            .to_string();
        assert!(
            error.contains("ambiguous") && error.contains("provide f0 explicitly"),
            "unexpected ambiguity diagnostic: {error}"
        );
    }

    #[test]
    fn thd_accepts_irregular_sampling_but_rejects_invalid_samples() {
        let x = nonuniform(2001, 0.0, 4.0);
        let y: Vec<f64> = x
            .iter()
            .map(|time| (2.0 * PI * time).sin() + 0.2 * (4.0 * PI * time).sin())
            .collect();
        assert_close(
            scalar_of(
                "thd",
                vec![
                    wave(x.clone(), y.clone()),
                    CalcValue::Scalar(3.0),
                    CalcValue::Scalar(1.0),
                ],
            ),
            20.0,
            2.0e-3,
            "explicit-f0 THD on an irregular grid",
        );

        let mut duplicate_x = x.clone();
        duplicate_x[100] = duplicate_x[99];
        let duplicate_error = call(
            "thd",
            vec![
                wave(duplicate_x, y.clone()),
                CalcValue::Scalar(3.0),
                CalcValue::Scalar(1.0),
            ],
        )
        .expect_err("THD cannot assign a Nyquist limit to duplicate timepoints")
        .to_string();
        assert!(
            duplicate_error.contains("strictly increasing"),
            "unexpected duplicate-time diagnostic: {duplicate_error}"
        );

        let mut undefined_y = y;
        undefined_y[200] = f64::NAN;
        let undefined_error = call(
            "thd",
            vec![
                wave(x, undefined_y),
                CalcValue::Scalar(3.0),
                CalcValue::Scalar(1.0),
            ],
        )
        .expect_err("THD cannot integrate through a waveform hole")
        .to_string();
        assert!(
            undefined_error.contains("waveform sample 200 is not finite"),
            "unexpected non-finite-sample diagnostic: {undefined_error}"
        );
    }

    #[test]
    fn thd_enforces_fundamental_resolution_and_nyquist_bounds() {
        let x: Vec<f64> = (0..=400).map(|index| index as f64 * 0.01).collect();
        let y: Vec<f64> = x.iter().map(|time| (2.0 * PI * time).sin()).collect();
        for (fundamental, harmonics, expected) in [
            (0.2, 2.0, "at least one complete cycle"),
            (0.0, 2.0, "finite and positive"),
            (f64::NAN, 2.0, "finite and positive"),
            // Largest gap is 0.01 s, so the conservative Nyquist limit is
            // 50 Hz and the fifth harmonic of 10 Hz lies on, not below, it.
            (10.0, 5.0, "not below"),
        ] {
            let error = call(
                "thd",
                vec![
                    wave(x.clone(), y.clone()),
                    CalcValue::Scalar(harmonics),
                    CalcValue::Scalar(fundamental),
                ],
            )
            .expect_err("invalid explicit-f0 measurement must fail closed")
            .to_string();
            assert!(
                error.contains(expected),
                "f0={fundamental:?}, n={harmonics}: expected {expected:?} in {error:?}"
            );
        }
    }

    /// A NaN parameter is refused by the guard that owns it, not absorbed.
    ///
    /// These three guards are written `x.is_nan() || x < bound` rather than
    /// `!(x >= bound)`. The two forms agree exactly, and this pins the part
    /// that is easy to lose: it is the NaN arm, not the comparison, that
    /// rejects NaN. Dropping it — writing a bare `x < bound` — lets NaN
    /// through every one of them, and none fails loudly afterwards. `cross`
    /// and `thd` would cast NaN to `0usize`, and `settling` would measure
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
            (
                "thd",
                vec![series.clone(), CalcValue::Scalar(f64::NAN)],
                "thd harmonic count must be",
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
        for name in [
            "min", "max", "pp", "freq", "period", "duty", "rise", "fall", "thd",
        ] {
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
