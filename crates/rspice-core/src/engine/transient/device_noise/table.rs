//! Unit trains for tabulated noise densities.
//!
//! Verilog-A `noise_table` and `noise_table_log`, and the native EKV3 VANOISE
//! export, state a density as a table of `(f, S)` points. A table is neither
//! flat nor a power law, so it is rendered by shaping unit white noise with a
//! filter whose magnitude follows the table.
//!
//! # The filter
//!
//! The band `[0, fmax]` of the `NT = 1/(2·fmax)` grid is sampled at `L + 1`
//! frequencies `f_k = k·fmax/L`, `L = ceil(fmax/fmin)`, so that `fmin` is the
//! table's frequency resolution. The magnitude there is
//!
//! ```text
//! H_k = sqrt(T(f_k) · fmax)
//! ```
//!
//! with `T` the table's shape interpolated as the language defines it (linear
//! for `noise_table`, log-log for `noise_table_log`) and clamped to its end
//! values outside the points. A zero-phase frequency-sampling design gives the
//! real, even impulse response `h[m]`, `m = -L..=L`, whose response at every
//! `f_k` is exactly `H_k`; the two `±L` taps carry half of the one periodic
//! sample each, which keeps the filter symmetric. Applied to unit-variance
//! white noise, a held sequence of discrete density `|H|²` has continuous
//! one-sided density `|H|²/fmax · sinc²(f·NT)`, which is `T(f)` across the
//! band: the same hold every flat and flicker train carries.
//!
//! The filter runs causally through the same dyadic convolution the flicker
//! trains use, delayed by `L` samples. The first `2L` outputs see an
//! incomplete window and are discarded, so the retained record is
//! stationary from its first sample and depends only on the seed and the
//! sample index — a prefix of a longer record is the shorter record.

use super::super::noise::{MAX_NOISE_SAMPLES, SplitMix64, filtered_white_noise};
use crate::Value;
use crate::abort_signal::AbortSignal;
use crate::analysis::noise::NoiseSource;
use crate::engine::SimulationError;

/// The shape of a tabulated density: its points and interpolation, at unit
/// scale.
pub(super) struct TableShape {
    unit: NoiseSource,
}

impl TableShape {
    pub(super) fn new(points: &[(Value, Value)], log_interp: bool) -> Self {
        Self {
            unit: NoiseSource::tabulated(String::new(), 0, 0, 1.0, points.to_vec(), log_interp),
        }
    }

    /// The table's value at `frequency`, interpolated and clamped exactly as
    /// `.NOISE` evaluates it.
    fn at(&self, frequency: Value) -> Value {
        self.unit.spectral_density(frequency, 0.0)
    }
}

/// How many frequency bins the filter resolves, before the sample cap.
fn bin_count(fmax: Value, fmin: Value, sample_count: usize) -> Option<usize> {
    let bins = (fmax / fmin).ceil();
    if !(bins.is_finite() && bins >= 1.0) {
        return None;
    }
    // A table resolved more finely than the record itself spans adds taps
    // nothing in the run can observe, which is the same cap the flicker
    // integrator applies to its kernel.
    Some((bins as usize).clamp(1, sample_count.max(1)))
}

/// The causal, `L`-delayed taps of the zero-phase table filter.
fn table_filter_taps(
    shape: &TableShape,
    fmax: Value,
    bins: usize,
    abort: &dyn AbortSignal,
) -> Result<Vec<Value>, SimulationError> {
    use rustfft::{FftPlanner, num_complex::Complex};
    let length = 2 * bins;
    let mut spectrum = vec![Complex::new(0.0, 0.0); length];
    for k in 0..=bins {
        if k.is_multiple_of(512) && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let frequency = k as Value * fmax / bins as Value;
        let magnitude = (shape.at(frequency) * fmax).sqrt();
        spectrum[k] = Complex::new(magnitude, 0.0);
        if k != 0 && k != bins {
            spectrum[length - k] = Complex::new(magnitude, 0.0);
        }
    }
    let mut planner = FftPlanner::new();
    let inverse = planner.plan_fft_inverse(length);
    inverse.process(&mut spectrum);
    let scale = 1.0 / length as Value;
    // h[m] for m in -L..=L, read from the periodic response; the shared
    // sample at m = ±L is split between its two ends.
    let mut taps = Vec::with_capacity(length + 1);
    for tap in 0..=length {
        let lag = tap as isize - bins as isize;
        let periodic = lag.rem_euclid(length as isize) as usize;
        let weight = if tap == 0 || tap == length { 0.5 } else { 1.0 };
        taps.push(weight * spectrum[periodic].re * scale);
    }
    Ok(taps)
}

/// One process's unit train for a tabulated density: one-sided density
/// `T(f)` per unit of the table's scale once held on the sample grid.
///
/// `budget` is the run's remaining materialized-sample allowance; the train
/// and its warm-up are charged to it.
#[allow(clippy::too_many_arguments)]
pub(super) fn tabulated_unit_train(
    shape: &TableShape,
    nt: Value,
    sample_count: usize,
    fmin: Value,
    seed: u64,
    budget: &mut usize,
    describe: &str,
    abort: &dyn AbortSignal,
) -> Result<Vec<Value>, SimulationError> {
    let fmax = 1.0 / (2.0 * nt);
    let bins = bin_count(fmax, fmin, sample_count).ok_or_else(|| {
        SimulationError::Circuit(format!(
            "NOISEFMIN={fmin} does not resolve the {nt} s noise sample grid for the tabulated \
             density of '{describe}'"
        ))
    })?;
    let warm_up = 2 * bins;
    let total = sample_count
        .checked_add(warm_up)
        .filter(|total| *total <= MAX_NOISE_SAMPLES)
        .ok_or_else(|| {
            SimulationError::Circuit(format!(
                "transient noise source '{describe}' needs {sample_count} samples plus a \
                 {warm_up}-sample warm-up, past the {MAX_NOISE_SAMPLES} limit. Raise NOISEFMIN, \
                 lower NOISEFMAX, or shorten the transient."
            ))
        })?;
    super::charge_materialized_budget(budget, total, describe)?;

    let taps = table_filter_taps(shape, fmax, bins, abort)?;
    if taps.iter().any(|tap| !tap.is_finite()) {
        return Err(SimulationError::Circuit(format!(
            "transient noise source '{describe}' has a tabulated density with no finite \
             time-domain filter"
        )));
    }
    let mut rng = SplitMix64::new(seed);
    let mut train = filtered_white_noise(total, &taps, 1.0, &mut rng, abort)?;
    train.drain(..warm_up);
    if train.iter().any(|value| !value.is_finite()) {
        return Err(SimulationError::Circuit(format!(
            "transient noise source '{describe}' generated a non-finite tabulated sample"
        )));
    }
    Ok(train)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::NoAbort;

    /// The zero-phase design reproduces the table at every sampled
    /// frequency, which is the property the whole rendering rests on.
    #[test]
    fn the_filter_response_equals_the_table_on_its_frequency_grid() {
        let shape = TableShape::new(
            &[(1.0e3, 4.0e-18), (1.0e5, 1.0e-20), (1.0e6, 9.0e-20)],
            true,
        );
        let fmax = 5.0e6;
        let bins = 64;
        let taps = table_filter_taps(&shape, fmax, bins, &NoAbort).expect("taps");
        assert_eq!(taps.len(), 2 * bins + 1);
        for (offset, tap) in taps.iter().enumerate() {
            let mirrored = taps[taps.len() - 1 - offset];
            assert!(
                (tap - mirrored).abs() <= 1.0e-12 * tap.abs().max(mirrored.abs()).max(1e-30),
                "the filter must be symmetric about its delay"
            );
        }
        for k in 0..=bins {
            let omega = std::f64::consts::PI * k as Value / bins as Value;
            // Remove the L-sample delay: a symmetric filter is real there.
            let response: Value = taps
                .iter()
                .enumerate()
                .map(|(tap, value)| value * (omega * (tap as Value - bins as Value)).cos())
                .sum();
            let expected = (shape.at(k as Value * fmax / bins as Value) * fmax).sqrt();
            assert!(
                (response - expected).abs() <= 1.0e-9 * expected,
                "bin {k}: response {response:e} against table magnitude {expected:e}"
            );
        }
    }

    /// A longer record of the same stream begins with the shorter one, so a
    /// train never depends on how long the run happens to be.
    #[test]
    fn a_tabulated_train_is_a_prefix_of_its_longer_records() {
        let shape = TableShape::new(&[(1.0e3, 1.0e-18), (1.0e6, 1.0e-20)], false);
        let nt = 1.0e-7;
        let mut budget = usize::MAX;
        let short = tabulated_unit_train(&shape, nt, 300, 1.0e5, 9, &mut budget, "X1", &NoAbort)
            .expect("short train");
        let long = tabulated_unit_train(&shape, nt, 900, 1.0e5, 9, &mut budget, "X1", &NoAbort)
            .expect("long train");
        assert_eq!(short.len(), 300);
        assert_eq!(&long[..300], short.as_slice());
    }
}
