//! Time-domain device-noise injection: the per-source current trains a
//! transient with `.TRAN … NOISEFMAX=` stamps into the right-hand side.
//!
//! This module owns the *storage and stamping* half of transient noise. The
//! physics half — which device mechanisms exist, what their spectral
//! densities are at the present bias, and how a mechanism is rendered as a
//! train — belongs to the engine, which builds one of these and refreshes its
//! amplitudes at every accepted step. Splitting it here is what lets every
//! transient right-hand-side assembly path stamp the injection with one call
//! against state it already holds, instead of each path having to know how
//! device noise is collected.
//!
//! # The process being injected
//!
//! Each source injects a sample-and-hold current: one independent draw per
//! sample interval `NT = 1/(2·fmax)`, held constant across the interval
//! `(k·NT, (k+1)·NT]`, and zero at `t = 0` so the operating point the run
//! starts from is the deterministic one. A held sequence of independent draws
//! of variance `σ²` has autocovariance `σ²(1 − |τ|/NT)` and therefore
//! one-sided power spectral density
//!
//! ```text
//! S(f) = 2·σ²·NT·sinc²(f·NT),   sinc(x) = sin(πx)/(πx)
//! ```
//!
//! so with `NT = 1/(2·fmax)` its density at low frequency is exactly
//! `σ²/fmax` and its total mean square is exactly `σ²`. Both facts are used:
//! a source that wants a flat one-sided density `S` over the noise band draws
//! with `σ² = S·fmax`, which simultaneously makes its in-band density `S` and
//! its total power `S·fmax` — the Johnson–Nyquist power in the band.
//!
//! Holding rather than interpolating is what buys that second property.
//! Linear interpolation between the same draws shapes the density by `sinc⁴`
//! instead and keeps only two thirds of the power, so a resistor's injected
//! mean square would fall a third short of `4kTR·fmax` while its low-frequency
//! density still looked right. Ngspice's `TRNOISE` card interpolates
//! (`isrcload.c` evaluates `V1 + (V2−V1)·(t/TS − n1)`), but `NA` there is an
//! authored amplitude with no spectral claim attached, so there is nothing to
//! disagree with.
//!
//! Every sample boundary is a solver breakpoint, so the integrator never
//! steps across a hold discontinuity.

use std::sync::Arc;

use crate::CircuitData;
use crate::Value;
use crate::numerics::split_mix64_at;

/// How far, in sample intervals, a time is snapped onto a sample boundary
/// before the hold interval is selected.
///
/// The boundaries are solver breakpoints, so an accepted point lands on
/// `k·NT` up to the breakpoint manager's own rounding. Without the snap a
/// point a few ulps past the boundary would read the next interval's draw,
/// which is harmless for the statistics but makes the same run report a
/// different sample for the same accepted time depending on how `k·NT` last
/// rounded.
const SAMPLE_BOUNDARY_SNAP: Value = 1.0e-9;

/// The shape of one source's unit-density process.
#[derive(Clone)]
pub(crate) enum NoiseTrain {
    /// Frequency-flat: a standard normal draw per sample, addressed by index,
    /// so a flat source costs no memory whatever the run's length.
    Flat { stream_seed: u64 },
    /// `1/f^ef`: a precomputed Kasdin fractional-integration sequence whose
    /// unit coefficient is one at 1 Hz. It cannot be addressed without the
    /// samples before it, so it is materialized once.
    PowerLaw { samples: Arc<Vec<Value>> },
}

/// One device-noise mechanism injected between two solution rows.
#[derive(Clone)]
pub(crate) struct InjectedNoiseSource {
    /// Solution row the current is injected into, one-based; zero is ground.
    pub(crate) node_pos: usize,
    /// Solution row the current is drawn from, one-based; zero is ground.
    pub(crate) node_neg: usize,
    /// The unit-density process this source scales.
    pub(crate) train: NoiseTrain,
    /// Complete multiplier applied to the unit process, refreshed from the
    /// solution at every accepted step: `NOISESCALE · sqrt(S·fmax)` for a flat
    /// source and `NOISESCALE · sqrt(A)` for a `A/f^ef` source.
    pub(crate) amplitude: Value,
}

/// The injection plan for one transient run.
#[derive(Clone)]
pub(crate) struct TransientDeviceNoise {
    /// Sample interval in seconds, `1/(2·fmax)`.
    nt: Value,
    /// Number of held samples covering the run.
    sample_count: usize,
    /// Every injected mechanism, in the order the engine collected them.
    sources: Vec<InjectedNoiseSource>,
}

impl std::fmt::Debug for TransientDeviceNoise {
    /// Summarize rather than print every sample: the containing
    /// [`crate::CircuitData`] derives `Debug`, and a materialized `1/f` train
    /// is millions of values.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let materialized = self
            .sources
            .iter()
            .filter(|source| matches!(source.train, NoiseTrain::PowerLaw { .. }))
            .count();
        formatter
            .debug_struct("TransientDeviceNoise")
            .field("nt", &self.nt)
            .field("sample_count", &self.sample_count)
            .field("sources", &self.sources.len())
            .field("power_law_sources", &materialized)
            .finish()
    }
}

impl TransientDeviceNoise {
    /// Assemble a plan. The engine is the only caller: it owns which sources
    /// exist and what their trains are.
    pub(crate) fn new(nt: Value, sample_count: usize, sources: Vec<InjectedNoiseSource>) -> Self {
        Self {
            nt,
            sample_count,
            sources,
        }
    }

    /// Sample interval in seconds.
    pub(crate) fn sample_interval(&self) -> Value {
        self.nt
    }

    /// Number of held samples covering the run.
    pub(crate) fn sample_count(&self) -> usize {
        self.sample_count
    }

    /// Whether this run injects nothing.
    ///
    /// A deck that asked for transient noise and turned out to hold no noise
    /// mechanism runs as an ordinary deterministic transient, which means it
    /// must not pay for the sample breakpoints either.
    pub(crate) fn is_empty(&self) -> bool {
        self.sources.is_empty()
    }

    /// Mutable access to the amplitude slots, for the accepted-step refresh.
    pub(crate) fn sources_mut(&mut self) -> &mut [InjectedNoiseSource] {
        &mut self.sources
    }

    /// The multiplier one source's unit process currently carries.
    #[cfg(test)]
    pub(crate) fn amplitude(&self, index: usize) -> Value {
        self.sources[index].amplitude
    }

    /// Which held sample covers `time`, or `None` when no noise is injected
    /// there.
    ///
    /// The hold interval is `(k·NT, (k+1)·NT]`, so a point that lands exactly
    /// on a boundary belongs to the interval it has just finished crossing.
    /// `t = 0` carries no noise at all: the run's first point is the
    /// deterministic operating point.
    fn sample_index(&self, time: Value) -> Option<usize> {
        if self.sample_count == 0 || !time.is_finite() || time <= 0.0 {
            return None;
        }
        let scaled = time / self.nt;
        if !scaled.is_finite() {
            return None;
        }
        let boundary = (scaled - SAMPLE_BOUNDARY_SNAP).ceil();
        if boundary < 1.0 {
            return Some(0);
        }
        if boundary >= self.sample_count as Value {
            return Some(self.sample_count - 1);
        }
        Some(boundary as usize - 1)
    }

    /// The unit-density draw of one source at a sample index.
    fn unit_sample(source: &InjectedNoiseSource, index: usize) -> Value {
        match &source.train {
            NoiseTrain::Flat { stream_seed } => standard_normal_at(*stream_seed, index as u64),
            NoiseTrain::PowerLaw { samples } => samples.get(index).copied().unwrap_or(0.0),
        }
    }

    /// The injected current of one source at `time`, in amperes. Zero outside
    /// the run's noise window.
    ///
    /// The run itself never needs one source in isolation — it stamps them all
    /// together — so this exists for the oracle tests, which read one
    /// mechanism's train back to estimate its spectrum.
    #[cfg(test)]
    pub(crate) fn source_current(&self, index: usize, time: Value) -> Value {
        let Some(source) = self.sources.get(index) else {
            return 0.0;
        };
        let Some(sample) = self.sample_index(time) else {
            return 0.0;
        };
        source.amplitude * Self::unit_sample(source, sample)
    }

    /// Add every injected noise current to a transient right-hand side.
    ///
    /// The incidence matches an independent current source exactly, including
    /// skipping tied terminals: subtracting and re-adding at one row can erase
    /// another source's contribution.
    pub(crate) fn stamp_transient_rhs(&self, rhs: &mut [Value], time: Value) {
        let Some(sample) = self.sample_index(time) else {
            return;
        };
        for source in &self.sources {
            if source.node_pos == source.node_neg {
                continue;
            }
            let value = source.amplitude * Self::unit_sample(source, sample);
            if value == 0.0 {
                continue;
            }
            if source.node_pos > 0
                && let Some(slot) = rhs.get_mut(source.node_pos - 1)
            {
                *slot -= value;
            }
            if source.node_neg > 0
                && let Some(slot) = rhs.get_mut(source.node_neg - 1)
            {
                *slot += value;
            }
        }
    }
}

impl CircuitData {
    /// Install the injection plan for a transient-noise run.
    pub(crate) fn install_transient_device_noise(&mut self, plan: TransientDeviceNoise) {
        self.transient_device_noise = Some(Box::new(plan));
    }

    /// Borrow the installed plan, if this run injects device noise.
    pub(crate) fn transient_device_noise(&self) -> Option<&TransientDeviceNoise> {
        self.transient_device_noise.as_deref()
    }

    /// Borrow the installed plan for the accepted-step amplitude refresh.
    pub(crate) fn transient_device_noise_mut(&mut self) -> Option<&mut TransientDeviceNoise> {
        self.transient_device_noise.as_deref_mut()
    }

    /// Add the injected device-noise currents to a transient right-hand side.
    ///
    /// A no-op for every run that did not ask for transient noise, which is
    /// why each right-hand-side assembly path can call it unconditionally
    /// beside its independent-source stamp.
    pub(crate) fn stamp_transient_device_noise(&self, rhs: &mut [Value], time: Value) {
        if let Some(plan) = self.transient_device_noise.as_deref() {
            plan.stamp_transient_rhs(rhs, time);
        }
    }
}

/// The standard normal draw at `index` of the stream seeded by `seed`.
///
/// Box–Muller over two indexed SplitMix64 draws, keeping the cosine half. An
/// indexed stream cannot cache the sine half the way a sequential generator
/// does, and discarding it costs only draws — not determinism, and not the
/// distribution.
pub(crate) fn standard_normal_at(seed: u64, index: u64) -> Value {
    let first = index.wrapping_mul(2);
    let u = indexed_open_unit(seed, first);
    let v = indexed_open_unit(seed, first.wrapping_add(1));
    let radius = (-2.0 * u.ln()).sqrt();
    radius * (std::f64::consts::TAU * v).cos()
}

/// An indexed draw on the open unit interval.
///
/// Open at zero so the logarithm stays finite. A sequential generator rejects
/// and redraws; an indexed one cannot move, so it walks a fixed offset until
/// the draw is nonzero. The probability of one step is `2^-53`, and the walk
/// is deterministic, which is what the index has to be.
fn indexed_open_unit(seed: u64, index: u64) -> Value {
    const OPEN_UNIT_RETRY_STRIDE: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut probe = index;
    for _ in 0..4 {
        let bits = split_mix64_at(seed, probe) >> 11;
        if bits != 0 {
            return bits as Value / 9_007_199_254_740_992.0;
        }
        probe = probe.wrapping_add(OPEN_UNIT_RETRY_STRIDE);
    }
    // Four consecutive zero draws has probability 2^-212. Returning the
    // smallest representable draw keeps the logarithm finite rather than
    // failing a run on an outcome that cannot occur.
    1.0 / 9_007_199_254_740_992.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flat_plan(nt: Value, samples: usize, amplitude: Value) -> TransientDeviceNoise {
        TransientDeviceNoise::new(
            nt,
            samples,
            vec![InjectedNoiseSource {
                node_pos: 1,
                node_neg: 0,
                train: NoiseTrain::Flat { stream_seed: 7 },
                amplitude,
            }],
        )
    }

    #[test]
    fn the_hold_interval_is_left_open_and_right_closed() {
        let plan = flat_plan(1.0e-9, 8, 1.0);
        assert_eq!(plan.sample_index(0.0), None);
        assert_eq!(plan.sample_index(0.5e-9), Some(0));
        assert_eq!(plan.sample_index(1.0e-9), Some(0));
        assert_eq!(plan.sample_index(1.5e-9), Some(1));
        assert_eq!(plan.sample_index(2.0e-9), Some(1));
        assert_eq!(plan.sample_index(1.0e-6), Some(7));
    }

    #[test]
    fn a_boundary_a_few_ulps_late_still_reads_the_interval_it_crossed() {
        let plan = flat_plan(1.0e-9, 8, 1.0);
        let boundary = 3.0 * 1.0e-9;
        assert_eq!(plan.sample_index(boundary), Some(2));
        assert_eq!(plan.sample_index(boundary * (1.0 + 1.0e-14)), Some(2));
        assert_eq!(plan.sample_index(boundary * (1.0 - 1.0e-14)), Some(2));
    }

    #[test]
    fn a_held_standard_normal_stream_has_unit_variance() {
        let count = 1 << 16;
        let mut sum = 0.0;
        let mut square_sum = 0.0;
        for index in 0..count {
            let sample = standard_normal_at(0xABCD_1234, index);
            sum += sample;
            square_sum += sample * sample;
        }
        let mean = sum / count as Value;
        let variance = square_sum / count as Value - mean * mean;
        // 3.5 standard errors of the variance estimate at this sample count.
        assert!(mean.abs() < 0.02, "mean {mean}");
        assert!((variance - 1.0).abs() < 0.02, "variance {variance}");
    }

    #[test]
    fn a_tied_source_stamps_nothing() {
        let plan = TransientDeviceNoise::new(
            1.0e-9,
            4,
            vec![InjectedNoiseSource {
                node_pos: 1,
                node_neg: 1,
                train: NoiseTrain::Flat { stream_seed: 3 },
                amplitude: 1.0,
            }],
        );
        let mut rhs = vec![0.0; 2];
        plan.stamp_transient_rhs(&mut rhs, 1.0e-9);
        assert_eq!(rhs, vec![0.0, 0.0]);
    }

    #[test]
    fn injection_uses_the_independent_current_source_incidence() {
        let plan = TransientDeviceNoise::new(
            1.0e-9,
            4,
            vec![InjectedNoiseSource {
                node_pos: 1,
                node_neg: 2,
                train: NoiseTrain::PowerLaw {
                    samples: Arc::new(vec![2.0, 0.0, 0.0, 0.0]),
                },
                amplitude: 3.0,
            }],
        );
        let mut rhs = vec![0.0; 2];
        plan.stamp_transient_rhs(&mut rhs, 1.0e-9);
        assert_eq!(rhs, vec![-6.0, 6.0]);
    }

    #[test]
    fn no_noise_is_injected_at_the_operating_point() {
        let plan = flat_plan(1.0e-9, 4, 1.0e6);
        let mut rhs = vec![0.0; 2];
        plan.stamp_transient_rhs(&mut rhs, 0.0);
        assert_eq!(rhs, vec![0.0, 0.0]);
    }
}
