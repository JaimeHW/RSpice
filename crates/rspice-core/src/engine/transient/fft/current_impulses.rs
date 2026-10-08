//! Analytic charge contributions to the calibrated, one-sided FFT bins.
//!
//! Finite samples retain their DFT. Each singular charge contributes its exact
//! windowed exponential coefficient, before normalization or band metrics.
//! Event ownership is (START, STOP], as for current integrals and Fourier.
//! Periodic windows wrap at STOP. Symmetric tapers have compact support through
//! the last DFT sample; they are zero afterward, rather than extrapolated into
//! the unsampled final interval. Rectangular windows cover the whole interval.

use super::*;
use crate::analysis::measure_signals::impulse_observation::{
    ImpulseContribution, ImpulseObservationError,
};
use crate::numerics::{compensated_add, scaled_exp_product};

pub(super) fn add_to_bins(
    bins: &mut [Complex<Value>],
    terms: &[ImpulseContribution<'_>],
    analysis: &FftAnalysis,
    mode: XyceFftMode,
    transient_stop: Value,
    coherent_gain: Value,
    abort: &dyn AbortSignal,
) -> Result<(), ImpulseObservationError> {
    if terms.is_empty() {
        return Ok(());
    }
    let invalid = |detail: &str| ImpulseObservationError::Invalid {
        detail: detail.into(),
    };
    let start = analysis.start.unwrap_or(0.0);
    let stop = analysis.stop.unwrap_or(transient_stop);
    let duration = stop - start;
    let periodic = mode.uses_periodic_windows();
    let denominator = if periodic {
        analysis.points
    } else {
        analysis.points - 1
    } as Value;
    let last_sample = sample_time(analysis, transient_stop, analysis.points - 1);
    for (bin, coefficient) in bins.iter_mut().enumerate() {
        if abort.is_aborted() {
            return Err(ImpulseObservationError::Aborted);
        }
        let one_sided = if bin == 0 || bin == analysis.points / 2 {
            1.0
        } else {
            2.0
        };
        let mut real = coefficient.re;
        let mut imaginary = coefficient.im;
        let mut real_correction = 0.0;
        let mut imaginary_correction = 0.0;
        for term in terms {
            for (index, point) in term.trace.points().enumerate() {
                if index.is_multiple_of(64) && abort.is_aborted() {
                    return Err(ImpulseObservationError::Aborted);
                }
                if point.time <= start {
                    continue;
                }
                if point.time > stop {
                    break;
                }
                if !periodic
                    && analysis.window != FftWindow::Rectangular
                    && point.time > last_sample
                {
                    continue;
                }
                let fraction = (point.time - start) / duration;
                let position = if periodic && point.time == stop {
                    0.0
                } else {
                    fraction * analysis.points as Value
                };
                let window = window_coefficient_at_position_with_alpha(
                    analysis.window,
                    position,
                    denominator,
                    analysis.alpha,
                );
                if window == 0.0 {
                    continue;
                }
                let rate = scaled_exp_product(
                    &[point.coefficient, term.weight, window, one_sided],
                    &[duration, coherent_gain],
                    0.0,
                );
                if !rate.is_finite() || (rate == 0.0 && term.weight != 0.0) {
                    return Err(invalid("windowed impulse coefficient is not representable"));
                }
                let phase = -2.0 * PI * (bin as Value * fraction).fract();
                let (sine, cosine) = phase.sin_cos();
                compensated_add(&mut real, &mut real_correction, rate * cosine);
                compensated_add(&mut imaginary, &mut imaginary_correction, rate * sine);
            }
            for (index, point) in term.trace.derivatives().iter().enumerate() {
                if index.is_multiple_of(64) && abort.is_aborted() {
                    return Err(ImpulseObservationError::Aborted);
                }
                if analysis.window != FftWindow::Rectangular
                    || point.time <= start
                    || point.time > stop
                {
                    continue;
                }
                let rate = point
                    .periodic_rate(
                        bin as Value / duration,
                        term.weight,
                        duration,
                        coherent_gain / one_sided,
                    )
                    .map_err(|detail| ImpulseObservationError::Invalid { detail })?;
                let fraction = (point.time - start) / duration;
                let (sine, cosine) = (-2.0 * PI * (bin as Value * fraction).fract()).sin_cos();
                let (real_phase, imaginary_phase) = match point.order % 4 {
                    0 => (cosine, sine),
                    1 => (-sine, cosine),
                    2 => (-cosine, -sine),
                    _ => (sine, -cosine),
                };
                compensated_add(&mut real, &mut real_correction, rate * real_phase);
                compensated_add(
                    &mut imaginary,
                    &mut imaginary_correction,
                    rate * imaginary_phase,
                );
            }
        }
        *coefficient = Complex::new(real + real_correction, imaginary + imaginary_correction);
        if !coefficient.re.is_finite() || !coefficient.im.is_finite() {
            return Err(invalid("impulse FFT coefficient is non-finite"));
        }
    }
    Ok(())
}
