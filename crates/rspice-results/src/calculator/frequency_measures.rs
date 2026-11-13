//! Frequency-response measurements: bandwidth and the loop-gain margins.
//!
//! Each reads a response over a positive, increasing frequency axis — an AC,
//! STB or similar sweep — and interpolates in log-frequency, as the Bode
//! sheet does. `gainMargin` and `phaseMargin` are the Bode sheet's own
//! margins ([`crate::bode::metrics_from_curves`]), so a margin read from the
//! calculator can never disagree with the one the sheet prints for the same
//! response.

use num_complex::Complex64;

use super::evaluator::EvaluationError;
use super::measures::{Argument, Measure, arity, keyword, number};
use super::value::{CalcValue, ComplexValue, RealValue};
use crate::bode;

/// The function that measures `name` (lower case), if this module owns it.
pub(super) fn lookup(name: &str) -> Option<Measure> {
    Some(match name {
        "bandwidth" => bandwidth,
        "gainmargin" => gain_margin,
        "phasemargin" => phase_margin,
        _ => return None,
    })
}

fn math(message: String) -> EvaluationError {
    EvaluationError::MathError(message)
}

/// A frequency response: its axis, its gain in dB, and its phase in degrees
/// when the response is complex.
struct Response {
    frequency: Vec<f64>,
    gain_db: Vec<f64>,
    phase_deg: Option<Vec<f64>>,
}

fn response(name: &str, args: &[Argument]) -> Result<Response, EvaluationError> {
    let (frequency, samples): (&[f64], Vec<Complex64>) = match &args[0] {
        Argument::Value(CalcValue::Complex(ComplexValue::Waveform(x, y))) => (x, y.clone()),
        Argument::Value(CalcValue::Real(RealValue::Waveform(x, y))) => (
            x,
            y.iter().map(|value| Complex64::new(*value, 0.0)).collect(),
        ),
        Argument::Value(_) => {
            return Err(EvaluationError::TypeMismatch(format!(
                "{name} measures a frequency response, not a scalar"
            )));
        }
        Argument::Keyword(word) => {
            return Err(EvaluationError::TypeMismatch(format!(
                "{name} reads a response where \"{word}\" stands"
            )));
        }
    };
    if frequency.len() < 2 || frequency.len() != samples.len() {
        return Err(math(format!(
            "{name} needs a frequency response with at least two samples"
        )));
    }
    if frequency[0].is_nan()
        || frequency[0] <= 0.0
        || frequency
            .windows(2)
            .any(|pair| pair[1].is_nan() || pair[1] <= pair[0])
    {
        return Err(math(format!(
            "{name} reads a frequency response: its x-axis must be positive and increasing"
        )));
    }
    let complex = matches!(&args[0], Argument::Value(CalcValue::Complex(_)));
    let gain_db = samples
        .iter()
        .map(|value| 20.0 * value.norm().log10())
        .collect();
    let phase_deg = complex.then(|| {
        samples
            .iter()
            .map(|value| value.arg().to_degrees())
            .collect()
    });
    Ok(Response {
        frequency: frequency.to_vec(),
        gain_db,
        phase_deg,
    })
}

/// `bandwidth(w[, dB[, shape]])` — the band over which the response stays
/// within `dB` (default 3) of its reference gain:
///
/// - `"low"` (default): a low-pass response's corner, the first frequency
///   at which the gain falls `dB` below its value at the lowest frequency —
///   the Bode sheet's f₋₃dB;
/// - `"high"`: a high-pass response's corner, the last frequency at which
///   the gain is still `dB` below its value at the highest frequency;
/// - `"band"`: the width between the two frequencies either side of the
///   peak at which the gain is `dB` below the peak.
///
/// Read `w` as the response itself, not its dB: a real waveform is taken as
/// a linear amplitude.
fn bandwidth(name: &str, args: &[Argument]) -> Result<f64, EvaluationError> {
    arity(name, args, 1, 3)?;
    let response = response(name, args)?;
    let drop = number(name, "dB", args, 1)?.unwrap_or(3.0);
    if drop <= 0.0 {
        return Err(math(format!(
            "{name} dB must be a positive drop below the reference gain"
        )));
    }
    let shape = keyword(name, "shape", args, 2, &["low", "high", "band"])?.unwrap_or("low");
    let (f, gain) = (&response.frequency, &response.gain_db);
    if gain.iter().any(|value| !value.is_finite()) {
        return Err(math(format!(
            "{name} needs a finite, non-zero response at every frequency"
        )));
    }
    let edge = |reference: f64| bode::log_frequency_crossings(f, gain, reference - drop);
    match shape {
        "low" => edge(gain[0]).first().copied().ok_or_else(|| {
            math(format!(
                "{name} finds no frequency at which the gain falls {drop} dB below its \
                 low-frequency value of {} dB",
                gain[0]
            ))
        }),
        "high" => {
            let reference = gain[gain.len() - 1];
            edge(reference).last().copied().ok_or_else(|| {
                math(format!(
                    "{name} finds no frequency at which the gain is {drop} dB below its \
                     high-frequency value of {reference} dB"
                ))
            })
        }
        _ => {
            let peak = (0..gain.len())
                .max_by(|&a, &b| gain[a].total_cmp(&gain[b]))
                .expect("a response has samples");
            let crossings = edge(gain[peak]);
            let below = crossings.iter().rev().find(|&&at| at < f[peak]);
            let above = crossings.iter().find(|&&at| at > f[peak]);
            match (below, above) {
                (Some(low), Some(high)) => Ok(high - low),
                _ => Err(math(format!(
                    "{name} needs the gain {drop} dB below its {} dB peak on both sides of {}; \
                     the sweep {}",
                    gain[peak],
                    f[peak],
                    if below.is_none() {
                        "does not fall that far below the peak"
                    } else {
                        "does not fall that far above the peak"
                    }
                ))),
            }
        }
    }
}

/// The Bode sheet's metrics for the loop gain `w`, which must be complex.
fn loop_metrics(name: &str, args: &[Argument]) -> Result<bode::AcBodeMetrics, EvaluationError> {
    arity(name, args, 1, 1)?;
    let response = response(name, args)?;
    let phase = response.phase_deg.ok_or_else(|| {
        EvaluationError::TypeMismatch(format!(
            "{name} needs the complex loop gain; a real response carries no phase"
        ))
    })?;
    Ok(bode::metrics_from_curves(
        &response.frequency,
        &response.gain_db,
        Some(&phase),
    ))
}

/// `gainMargin(w)` — the gain margin in dB of the loop gain `w`: how far
/// below 0 dB the gain is where the phase reaches -180°. Where the phase
/// passes -180° more than once, the binding crossing — the smallest margin
/// — is the one reported, signed, as the Bode sheet reports it.
fn gain_margin(name: &str, args: &[Argument]) -> Result<f64, EvaluationError> {
    loop_metrics(name, args)?.gm_db.ok_or_else(|| {
        math(format!(
            "{name} finds no frequency in the sweep at which the loop phase reaches -180°"
        ))
    })
}

/// `phaseMargin(w)` — the phase margin in degrees of the loop gain `w`:
/// `180° + ∠w` where its gain crosses 0 dB, folded into one turn. Where the
/// gain crosses 0 dB more than once, the binding crossing is reported, as
/// the Bode sheet reports it.
fn phase_margin(name: &str, args: &[Argument]) -> Result<f64, EvaluationError> {
    loop_metrics(name, args)?.pm_deg.ok_or_else(|| {
        math(format!(
            "{name} finds no frequency in the sweep at which the loop gain crosses 0 dB"
        ))
    })
}

#[cfg(test)]
mod tests;
