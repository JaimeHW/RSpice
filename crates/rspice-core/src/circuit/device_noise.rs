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
    /// A shaped unit process — a `1/f^ef` Kasdin fractional-integration
    /// sequence, or a filtered train following a tabulated density. It cannot
    /// be addressed without the samples before it, so it is materialized
    /// once, and it is never changed afterwards.
    Materialized { samples: Arc<Vec<Value>> },
}

/// One place a source's current enters the circuit.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct NoiseInjection {
    /// Solution row the current is injected into, one-based; zero is ground.
    pub(crate) node_pos: usize,
    /// Solution row the current is drawn from, one-based; zero is ground.
    pub(crate) node_neg: usize,
    /// The share of the source's current this injection carries.
    pub(crate) gain: Value,
}

impl NoiseInjection {
    /// A whole current between two rows: the shape of every native device
    /// mechanism.
    pub(crate) fn unit(node_pos: usize, node_neg: usize) -> Self {
        Self {
            node_pos,
            node_neg,
            gain: 1.0,
        }
    }
}

/// One noise process: a unit train, scaled by one amplitude and injected at
/// every place its value reaches the circuit.
///
/// A native mechanism is one current between two rows. A Verilog-A noise
/// function whose value reaches several equations is one process with several
/// injections, all driven by the same train, which is what makes them
/// coherent.
#[derive(Clone)]
pub(crate) struct InjectedNoiseSource {
    /// The unit-density process this source scales.
    pub(crate) train: NoiseTrain,
    /// Complete multiplier applied to the unit process, refreshed from the
    /// solution at every accepted step: `NOISESCALE · sqrt(S·fmax)` for a flat
    /// source, `NOISESCALE · sqrt(A)` for a `A/f^ef` source and
    /// `NOISESCALE · sqrt(scale)` for a tabulated one.
    pub(crate) amplitude: Value,
    /// Where the scaled process is injected, each with its own gain.
    pub(crate) injections: Vec<NoiseInjection>,
    /// The filter a module shapes the scaled process through before it
    /// reaches the circuit, or `None` when every injection is a plain gain.
    /// A shaped source's injections carry unit gain: injection `i` carries
    /// the filter's output `i`.
    pub(crate) shaping: Option<ShapedNoise>,
}

/// One output of a [`ShapedNoise`] filter, as the circuit injection it
/// drives sees it:
///
/// ```text
/// y_n = state·z_n + prior_state·z_{n-1} + input·u_n + prior_input·u_{n-1}
/// ```
///
/// where `u` is the scaled input train and `z` the filter state.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ShapedOutput {
    pub(crate) state: Vec<Value>,
    pub(crate) prior_state: Vec<Value>,
    pub(crate) input: Value,
    pub(crate) prior_input: Value,
}

/// The state step of a [`ShapedNoise`] filter:
///
/// ```text
/// z_n = transition·z_{n-1} + drive·u_n + carry·u_{n-1}
/// ```
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ShapedStep {
    /// Row-major `order × order`.
    pub(crate) transition: Vec<Value>,
    pub(crate) drive: Vec<Value>,
    pub(crate) carry: Vec<Value>,
}

impl ShapedStep {
    fn order(&self) -> usize {
        self.drive.len()
    }
}

/// The filter state after a number of absorbed samples.
#[derive(Clone, Debug, PartialEq)]
struct ShapedState {
    /// How many train samples the state has absorbed: the next train index.
    absorbed: usize,
    z: Vec<Value>,
    prior_z: Vec<Value>,
    u: Value,
    prior_u: Value,
}

/// A process the module shapes through `ddt`, `idt` or `laplace_*`: its
/// scaled unit train is the input of a finite-state linear filter, already
/// discretized on the sample grid, and each injection carries one output.
///
/// The state is *accepted* state. It advances only when the engine commits
/// an accepted point, and a stamp at any later time computes forward from it
/// without changing it, so a rejected step restamps exactly the samples the
/// next attempt will see and nothing needs restoring. The filter starts from
/// rest `warmup` train samples before the run's first sample, so the record
/// opens stationary down to the lowest represented frequency instead of with
/// the filter's own start-up transient.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ShapedNoise {
    step: ShapedStep,
    outputs: Vec<ShapedOutput>,
    /// Train index of the run's first sample.
    warmup: usize,
    committed: ShapedState,
}

impl ShapedNoise {
    /// A filter at rest, advanced through its warm-up at `amplitude`.
    pub(crate) fn warmed(
        step: ShapedStep,
        outputs: Vec<ShapedOutput>,
        warmup: usize,
        train: &NoiseTrain,
        amplitude: Value,
    ) -> Self {
        let order = step.order();
        let mut shaped = Self {
            step,
            outputs,
            warmup,
            committed: ShapedState {
                absorbed: 0,
                z: vec![0.0; order],
                prior_z: vec![0.0; order],
                u: 0.0,
                prior_u: 0.0,
            },
        };
        shaped.committed = shaped.advanced(&shaped.committed, warmup, train, amplitude);
        shaped
    }

    /// Replace the outputs: the gains a refresh re-derived from the bias.
    pub(crate) fn set_outputs(&mut self, outputs: Vec<ShapedOutput>) {
        self.outputs = outputs;
    }

    /// `state` advanced until it has absorbed `absorbed` train samples.
    fn advanced(
        &self,
        state: &ShapedState,
        absorbed: usize,
        train: &NoiseTrain,
        amplitude: Value,
    ) -> ShapedState {
        let mut state = state.clone();
        let order = self.step.order();
        let mut next = vec![0.0; order];
        while state.absorbed < absorbed {
            let u = amplitude * train_sample(train, state.absorbed);
            for (row, slot) in next.iter_mut().enumerate() {
                let transition = &self.step.transition[row * order..(row + 1) * order];
                *slot = transition
                    .iter()
                    .zip(&state.z)
                    .map(|(a, z)| a * z)
                    .sum::<Value>()
                    + self.step.drive[row] * u
                    + self.step.carry[row] * state.u;
            }
            std::mem::swap(&mut state.prior_z, &mut state.z);
            state.z.copy_from_slice(&next);
            state.prior_u = state.u;
            state.u = u;
            state.absorbed += 1;
        }
        state
    }

    /// Commit the samples through run sample `sample`: called at accepted
    /// points only.
    fn commit_through(&mut self, sample: usize, train: &NoiseTrain, amplitude: Value) {
        let absorbed = self.warmup.saturating_add(sample).saturating_add(1);
        if absorbed > self.committed.absorbed {
            self.committed = self.advanced(&self.committed, absorbed, train, amplitude);
        }
    }

    /// Every output's value over run sample `sample`, without changing the
    /// committed state.
    fn outputs_at(
        &self,
        sample: usize,
        train: &NoiseTrain,
        amplitude: Value,
        mut visit: impl FnMut(usize, Value),
    ) {
        let absorbed = self.warmup.saturating_add(sample).saturating_add(1);
        // A time before the committed point is never stamped by the run; it
        // reads the committed sample rather than recomputing the past.
        let ahead;
        let state = if absorbed > self.committed.absorbed {
            ahead = self.advanced(&self.committed, absorbed, train, amplitude);
            &ahead
        } else {
            &self.committed
        };
        for (index, output) in self.outputs.iter().enumerate() {
            let value = output
                .state
                .iter()
                .zip(&state.z)
                .chain(output.prior_state.iter().zip(&state.prior_z))
                .map(|(coefficient, z)| coefficient * z)
                .sum::<Value>()
                + output.input * state.u
                + output.prior_input * state.prior_u;
            visit(index, value);
        }
    }
}

/// The unit-density draw of a train at a train index.
fn train_sample(train: &NoiseTrain, index: usize) -> Value {
    match train {
        NoiseTrain::Flat { stream_seed } => standard_normal_at(*stream_seed, index as u64),
        NoiseTrain::Materialized { samples } => samples.get(index).copied().unwrap_or(0.0),
    }
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
    /// Whether sources may join the plan during the run: a circuit whose
    /// noise processes are all dormant at the operating point still needs
    /// its sample grid.
    dormant_sources: bool,
}

impl std::fmt::Debug for TransientDeviceNoise {
    /// Summarize rather than print every sample: the containing
    /// [`crate::CircuitData`] derives `Debug`, and a materialized `1/f` train
    /// is millions of values.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let materialized = self
            .sources
            .iter()
            .filter(|source| matches!(source.train, NoiseTrain::Materialized { .. }))
            .count();
        formatter
            .debug_struct("TransientDeviceNoise")
            .field("nt", &self.nt)
            .field("sample_count", &self.sample_count)
            .field("sources", &self.sources.len())
            .field("materialized_sources", &materialized)
            .field("dormant_sources", &self.dormant_sources)
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
            dormant_sources: false,
        }
    }

    /// Declare that sources may become active after the operating point.
    #[must_use]
    pub(crate) fn with_dormant_sources(mut self, dormant: bool) -> Self {
        self.dormant_sources = dormant;
        self
    }

    /// Start injecting a source that became active during the run, returning
    /// its position.
    pub(crate) fn push_source(&mut self, source: InjectedNoiseSource) -> usize {
        self.sources.push(source);
        self.sources.len() - 1
    }

    /// Sample interval in seconds.
    pub(crate) fn sample_interval(&self) -> Value {
        self.nt
    }

    /// Number of held samples covering the run.
    pub(crate) fn sample_count(&self) -> usize {
        self.sample_count
    }

    /// Whether this run injects nothing, now or later.
    ///
    /// A deck that asked for transient noise and turned out to hold no noise
    /// mechanism runs as an ordinary deterministic transient, which means it
    /// must not pay for the sample breakpoints either.
    pub(crate) fn is_empty(&self) -> bool {
        self.sources.is_empty() && !self.dormant_sources
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
        train_sample(&source.train, index)
    }

    /// The scaled current of one source at `time`, in amperes, before any
    /// injection gain — for a shaped source, its first output. Zero outside
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
        if let Some(shaping) = &source.shaping {
            let mut first = 0.0;
            shaping.outputs_at(sample, &source.train, source.amplitude, |output, value| {
                if output == 0 {
                    first = value;
                }
            });
            return first;
        }
        source.amplitude * Self::unit_sample(source, sample)
    }

    /// Advance every shaped source's filter through the sample covering an
    /// accepted `time`, at the amplitude its stamps used.
    ///
    /// Called at accepted points only, before the refresh re-derives the
    /// amplitudes from that point's bias.
    pub(crate) fn commit_accepted(&mut self, time: Value) {
        let Some(sample) = self.sample_index(time) else {
            return;
        };
        for source in &mut self.sources {
            let InjectedNoiseSource {
                train,
                amplitude,
                shaping,
                ..
            } = source;
            if let Some(shaping) = shaping {
                shaping.commit_through(sample, train, *amplitude);
            }
        }
    }

    /// Every source's amplitude, in plan order, for the oracle tests.
    #[cfg(test)]
    pub(crate) fn amplitudes(&self) -> Vec<Value> {
        self.sources.iter().map(|source| source.amplitude).collect()
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
            if let Some(shaping) = &source.shaping {
                shaping.outputs_at(sample, &source.train, source.amplitude, |output, value| {
                    if let Some(injection) = source.injections.get(output) {
                        Self::stamp_injection(rhs, injection, value);
                    }
                });
                continue;
            }
            let value = source.amplitude * Self::unit_sample(source, sample);
            if value == 0.0 {
                continue;
            }
            for injection in &source.injections {
                Self::stamp_injection(rhs, injection, value);
            }
        }
    }

    /// Stamp one injection carrying `value` before its gain.
    fn stamp_injection(rhs: &mut [Value], injection: &NoiseInjection, value: Value) {
        if injection.node_pos == injection.node_neg {
            return;
        }
        let current = value * injection.gain;
        if current == 0.0 {
            return;
        }
        if injection.node_pos > 0
            && let Some(slot) = rhs.get_mut(injection.node_pos - 1)
        {
            *slot -= current;
        }
        if injection.node_neg > 0
            && let Some(slot) = rhs.get_mut(injection.node_neg - 1)
        {
            *slot += current;
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
                train: NoiseTrain::Flat { stream_seed: 7 },
                amplitude,
                injections: vec![NoiseInjection::unit(1, 0)],
                shaping: None,
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
                train: NoiseTrain::Flat { stream_seed: 3 },
                amplitude: 1.0,
                injections: vec![NoiseInjection::unit(1, 1)],
                shaping: None,
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
                train: NoiseTrain::Materialized {
                    samples: Arc::new(vec![2.0, 0.0, 0.0, 0.0]),
                },
                amplitude: 3.0,
                injections: vec![NoiseInjection::unit(1, 2)],
                shaping: None,
            }],
        );
        let mut rhs = vec![0.0; 2];
        plan.stamp_transient_rhs(&mut rhs, 1.0e-9);
        assert_eq!(rhs, vec![-6.0, 6.0]);
    }

    /// One process reaching two equations drives both from one draw, each
    /// scaled by its own gain: the injections are coherent by construction.
    #[test]
    fn a_process_drives_every_injection_from_one_draw() {
        let plan = TransientDeviceNoise::new(
            1.0e-9,
            4,
            vec![InjectedNoiseSource {
                train: NoiseTrain::Materialized {
                    samples: Arc::new(vec![0.5, 0.0, 0.0, 0.0]),
                },
                amplitude: 2.0,
                injections: vec![
                    NoiseInjection {
                        node_pos: 1,
                        node_neg: 0,
                        gain: 1.0,
                    },
                    NoiseInjection {
                        node_pos: 2,
                        node_neg: 3,
                        gain: -2.0,
                    },
                ],
                shaping: None,
            }],
        );
        let mut rhs = vec![0.0; 3];
        plan.stamp_transient_rhs(&mut rhs, 1.0e-9);
        assert_eq!(rhs, vec![-1.0, 2.0, -2.0]);
    }

    /// A plan whose sources may join later keeps its sample grid even while
    /// nothing is injected yet.
    #[test]
    fn a_plan_with_dormant_sources_is_not_empty() {
        let plan = TransientDeviceNoise::new(1.0e-9, 4, Vec::new());
        assert!(plan.is_empty());
        let mut dormant = plan.with_dormant_sources(true);
        assert!(!dormant.is_empty());
        let position = dormant.push_source(InjectedNoiseSource {
            train: NoiseTrain::Flat { stream_seed: 1 },
            amplitude: 1.0,
            injections: vec![NoiseInjection::unit(1, 0)],
            shaping: None,
        });
        assert_eq!(position, 0);
        assert_eq!(dormant.amplitudes(), vec![1.0]);
    }

    /// A backward-difference integrator `z_n = z_{n−1} + NT·u_n` driving
    /// one injection, fed by a known train.
    fn integrating_plan(train: Vec<Value>, warmup: usize) -> TransientDeviceNoise {
        let nt = 0.5;
        let train = NoiseTrain::Materialized {
            samples: Arc::new(train),
        };
        let shaping = ShapedNoise::warmed(
            ShapedStep {
                transition: vec![1.0],
                drive: vec![nt],
                carry: vec![0.0],
            },
            vec![ShapedOutput {
                state: vec![1.0],
                prior_state: vec![0.0],
                input: 0.0,
                prior_input: 0.0,
            }],
            warmup,
            &train,
            2.0,
        );
        TransientDeviceNoise::new(
            nt,
            4,
            vec![InjectedNoiseSource {
                train,
                amplitude: 2.0,
                injections: vec![NoiseInjection::unit(1, 0)],
                shaping: Some(shaping),
            }],
        )
    }

    fn stamped(plan: &TransientDeviceNoise, time: Value) -> Value {
        let mut rhs = vec![0.0];
        plan.stamp_transient_rhs(&mut rhs, time);
        -rhs[0]
    }

    /// A shaped source's filter output over each sample is the recurrence
    /// applied to the scaled train, after the warm-up samples.
    #[test]
    fn a_shaped_source_injects_its_filter_output() {
        // Warm-up absorbs 1.0; the run then sees 2.0, 3.0, 4.0, 5.0.
        let plan = integrating_plan(vec![1.0, 2.0, 3.0, 4.0, 5.0], 1);
        // u = 2·train, z_n = z_{n−1} + 0.5·u_n = the running sum of the
        // train; the hold interval of sample k is (k·NT, (k+1)·NT].
        assert_eq!(stamped(&plan, 0.25), 1.0 + 2.0);
        assert_eq!(stamped(&plan, 0.5), 1.0 + 2.0);
        assert_eq!(stamped(&plan, 1.0), 1.0 + 2.0 + 3.0);
        assert_eq!(stamped(&plan, 1.25), 1.0 + 2.0 + 3.0 + 4.0);
        assert_eq!(stamped(&plan, 2.0), 1.0 + 2.0 + 3.0 + 4.0 + 5.0);
    }

    /// Stamping never moves the state: a rejected step restamps the same
    /// value, and committing an accepted point leaves every later stamp
    /// unchanged.
    #[test]
    fn a_shaped_source_moves_only_at_committed_points() {
        let mut plan = integrating_plan(vec![1.0, -2.0, 0.5, 4.0, 3.0], 0);
        let ahead: Vec<Value> = [0.25, 0.75, 1.5, 2.0]
            .iter()
            .map(|time| stamped(&plan, *time))
            .collect();
        assert_eq!(stamped(&plan, 1.5), ahead[2], "a restamp is identical");
        plan.commit_accepted(0.75);
        plan.commit_accepted(0.6);
        let after: Vec<Value> = [0.75, 1.5, 2.0]
            .iter()
            .map(|time| stamped(&plan, *time))
            .collect();
        assert_eq!(after, ahead[1..].to_vec());
        plan.commit_accepted(2.0);
        assert_eq!(stamped(&plan, 2.0), ahead[3]);
    }

    #[test]
    fn no_noise_is_injected_at_the_operating_point() {
        let plan = flat_plan(1.0e-9, 4, 1.0e6);
        let mut rhs = vec![0.0; 2];
        plan.stamp_transient_rhs(&mut rhs, 0.0);
        assert_eq!(rhs, vec![0.0, 0.0]);
    }
}
