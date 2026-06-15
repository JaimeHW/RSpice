//! Calculator Functions Library
//!
//! Implementations of mathematical and signal processing functions.
//! Designed to handle vector (waveform) and scalar inputs.

use super::evaluator::{CalcValue, EvaluationError};

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

            // Signal Processing
            "avg" | "average" => avg(args),
            "rms" => rms(args),
            "deriv" | "derivative" => deriv(args),
            "integ" | "integral" => integ(args),
            "clip" => clip(args),
            "unwrap" => unwrap(args),

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
        CalcValue::Waveform(x, y) => {
            let new_y = y.iter().map(|v| v.abs()).collect();
            Ok(CalcValue::create_waveform(x.clone(), new_y))
        }
    }
}

fn sqrt(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("sqrt", &args, 1)?;
    match &args[0] {
        CalcValue::Scalar(s) => {
            if *s < 0.0 {
                return Err(EvaluationError::MathError("Sqrt of negative number".into()));
            }
            Ok(CalcValue::Scalar(s.sqrt()))
        }
        CalcValue::Waveform(x, y) => {
            let new_y: Result<Vec<f64>, _> = y
                .iter()
                .map(|v| {
                    if *v < 0.0 {
                        Err(EvaluationError::MathError(
                            "Sqrt of negative number in waveform".into(),
                        ))
                    } else {
                        Ok(v.sqrt())
                    }
                })
                .collect();
            Ok(CalcValue::create_waveform(x.clone(), new_y?))
        }
    }
}

fn exp(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("exp", &args, 1)?;
    match &args[0] {
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(s.exp())),
        CalcValue::Waveform(x, y) => {
            let new_y = y.iter().map(|v| v.exp()).collect();
            Ok(CalcValue::create_waveform(x.clone(), new_y))
        }
    }
}

fn ln(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("ln", &args, 1)?;
    match &args[0] {
        CalcValue::Scalar(s) => {
            if *s <= 0.0 {
                return Err(EvaluationError::MathError(
                    "Ln of non-positive number".into(),
                ));
            }
            Ok(CalcValue::Scalar(s.ln()))
        }
        CalcValue::Waveform(x, y) => {
            // Avoid evaluating invalid domains
            // In commercial tools, this often produces NaN or holes. here we error.
            let new_y = y.iter().map(|v| v.ln()).collect();
            Ok(CalcValue::create_waveform(x.clone(), new_y))
        }
    }
}

fn log10(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("log10", &args, 1)?;
    match &args[0] {
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(s.log10())),
        CalcValue::Waveform(x, y) => {
            let new_y = y.iter().map(|v| v.log10()).collect();
            Ok(CalcValue::create_waveform(x.clone(), new_y))
        }
    }
}

// --- Aggregate Functions (Waveform -> Scalar) ---

fn avg(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("avg", &args, 1)?;
    match &args[0] {
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(*s)),
        CalcValue::Waveform(_x, y) => {
            if y.is_empty() {
                return Ok(CalcValue::Scalar(0.0));
            }
            // Simple average
            let sum: f64 = y.iter().sum();
            Ok(CalcValue::Scalar(sum / y.len() as f64))
        }
    }
}

fn rms(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("rms", &args, 1)?;
    match &args[0] {
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(*s)), // RMS of DC is DC
        CalcValue::Waveform(_x, y) => {
            if y.is_empty() {
                return Ok(CalcValue::Scalar(0.0));
            }
            let square_sum: f64 = y.iter().map(|v| v * v).sum();
            Ok(CalcValue::Scalar((square_sum / y.len() as f64).sqrt()))
        }
    }
}

// --- Transformation Functions (Waveform -> Waveform) ---

fn deriv(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("deriv", &args, 1)?;
    match &args[0] {
        CalcValue::Scalar(_) => Ok(CalcValue::Scalar(0.0)), // Derivative of constant is 0
        CalcValue::Waveform(x, y) => {
            if x.len() < 2 {
                return Ok(CalcValue::Scalar(0.0));
            } // Not enough points

            let mut new_y = Vec::with_capacity(x.len());
            // Simple forward/backward difference or central?
            // Commercial tools use central difference for interior points

            // Point 0 (forward)
            new_y.push((y[1] - y[0]) / (x[1] - x[0]));

            // Interior points (central)
            for i in 1..x.len() - 1 {
                let dy = y[i + 1] - y[i - 1];
                let dx = x[i + 1] - x[i - 1];
                new_y.push(dy / dx);
            }

            // Last point (backward)
            let last = x.len() - 1;
            new_y.push((y[last] - y[last - 1]) / (x[last] - x[last - 1]));

            Ok(CalcValue::create_waveform(x.clone(), new_y))
        }
    }
}

fn integ(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    check_arg_count("integ", &args, 1)?;
    match &args[0] {
        CalcValue::Scalar(_s) => Err(EvaluationError::MathError(
            "Cannot integrate scalar without domain".into(),
        )),
        CalcValue::Waveform(x, y) => {
            if x.len() < 2 {
                return Ok(CalcValue::Scalar(0.0));
            }

            // Trapezoidal rule cumulative integration
            let mut new_y = Vec::with_capacity(x.len());
            let mut accum = 0.0;
            new_y.push(accum);

            for i in 1..x.len() {
                let dt = x[i] - x[i - 1];
                let avg_val = (y[i] + y[i - 1]) * 0.5;
                accum += avg_val * dt;
                new_y.push(accum);
            }

            Ok(CalcValue::create_waveform(x.clone(), new_y))
        }
    }
}

fn clip(args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    // clip(signal, result, min, max)
    check_arg_count("clip", &args, 3)?;

    let min_val = match args[1] {
        CalcValue::Scalar(v) => v,
        _ => {
            return Err(EvaluationError::TypeMismatch(
                "clip min must be scalar".into(),
            ));
        }
    };

    let max_val = match args[2] {
        CalcValue::Scalar(v) => v,
        _ => {
            return Err(EvaluationError::TypeMismatch(
                "clip max must be scalar".into(),
            ));
        }
    };

    match &args[0] {
        CalcValue::Scalar(s) => Ok(CalcValue::Scalar(s.clamp(min_val, max_val))),
        CalcValue::Waveform(x, y) => {
            let new_y = y.iter().map(|v| v.clamp(min_val, max_val)).collect();
            Ok(CalcValue::create_waveform(x.clone(), new_y))
        }
    }
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
