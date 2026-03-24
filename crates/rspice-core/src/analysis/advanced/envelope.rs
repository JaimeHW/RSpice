//! Envelope Transient (envlp) Analysis
//!
//! Time-domain simulation of modulated signals using envelope following.

#![cfg_attr(test, allow(clippy::field_reassign_with_default))]
//! This is critical for RF/wireless circuits with amplitude or frequency
//! modulated signals (AM, FM, QAM, etc.).
//!
//! # Theory
//!
//! Instead of simulating every carrier cycle, envelope analysis:
//! 1. Uses a slowly-varying envelope representation
//! 2. Solves for amplitude and phase at each envelope timestep
//! 3. Reconstructs full waveform when needed
//!
//! For a modulated signal: x(t) = A(t) × cos(ωc × t + φ(t))
//! - A(t) is the slowly-varying envelope (amplitude)
//! - φ(t) is the slowly-varying phase modulation
//! - ωc is the carrier frequency
//!
//! # Envelope Representation
//!
//! Complex envelope: x̃(t) = A(t) × e^(jφ(t))
//! Full signal: x(t) = Re{x̃(t) × e^(jωc×t)}
//!
//! # Benefits
//!
//! - **Speed**: Steps at envelope rate, not carrier rate
//! - **Accuracy**: Captures modulation dynamics precisely
//! - **Memory**: Much smaller data storage
//!
//! # Use Cases
//!
//! - Power amplifier AM-AM/AM-PM distortion
//! - Mixer transient response to modulation
//! - PLL lock acquisition with frequency steps
//! - AGC loop dynamics
//!
//! # Algorithm
//!
//! 1. Initial PSS to find carrier steady-state
//! 2. Extract envelope from PSS solution
//! 3. Time-step envelope using modified transient
//! 4. Apply carrier modulation if output needed

use crate::Value;
use num_complex::Complex64;
use std::f64::consts::PI;

//=============================================================================
// Constants
//=============================================================================

/// Default envelope timestep factor (relative to envelope bandwidth)
pub const ENVELOPE_TIMESTEP_FACTOR: Value = 0.1;
/// Minimum samples per envelope period
pub const MIN_ENVELOPE_SAMPLES: usize = 20;
/// Maximum envelope timestep ratio to carrier period
pub const MAX_TIMESTEP_RATIO: Value = 100.0;

//=============================================================================
// Envelope Signal
//=============================================================================

/// Complex envelope representation of a modulated signal
#[derive(Debug, Clone)]
pub struct EnvelopeSignal {
    /// Time points (at envelope rate)
    pub time: Vec<Value>,
    /// Complex envelope values at each time point
    pub envelope: Vec<Complex64>,
    /// Carrier frequency (Hz)
    pub carrier_freq: Value,
    /// Initial phase offset (radians)
    pub initial_phase: Value,
}

impl EnvelopeSignal {
    /// Create new envelope signal
    pub fn new(carrier_freq: Value) -> Self {
        Self {
            time: Vec::new(),
            envelope: Vec::new(),
            carrier_freq,
            initial_phase: 0.0,
        }
    }

    /// Create from time and envelope vectors
    pub fn from_data(time: Vec<Value>, envelope: Vec<Complex64>, carrier_freq: Value) -> Self {
        Self {
            time,
            envelope,
            carrier_freq,
            initial_phase: 0.0,
        }
    }

    /// Set initial phase
    pub fn with_initial_phase(mut self, phase: Value) -> Self {
        self.initial_phase = phase;
        self
    }

    /// Get number of samples
    #[inline]
    pub fn len(&self) -> usize {
        self.time.len()
    }

    /// Check if empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.time.is_empty()
    }

    /// Get envelope duration
    pub fn duration(&self) -> Value {
        if self.time.len() < 2 {
            return 0.0;
        }
        self.time.last().unwrap() - self.time[0]
    }

    /// Get envelope at specific time (with interpolation)
    pub fn at(&self, t: Value) -> Option<Complex64> {
        if self.time.is_empty() || t < self.time[0] || t > *self.time.last()? {
            return None;
        }

        // Binary search for interval
        let idx = self.time.partition_point(|&x| x < t);

        if idx == 0 {
            return Some(self.envelope[0]);
        }
        if idx >= self.time.len() {
            return Some(*self.envelope.last()?);
        }

        // Linear interpolation of complex envelope
        let t0 = self.time[idx - 1];
        let t1 = self.time[idx];
        let e0 = self.envelope[idx - 1];
        let e1 = self.envelope[idx];

        let alpha = (t - t0) / (t1 - t0);
        Some(e0 * (1.0 - alpha) + e1 * alpha)
    }

    /// Reconstruct full time-domain signal at given time
    ///
    /// x(t) = Re{envelope(t) × e^(j(ωc×t + φ₀))}
    pub fn reconstruct_at(&self, t: Value) -> Option<Value> {
        let env = self.at(t)?;
        let carrier_phase = 2.0 * PI * self.carrier_freq * t + self.initial_phase;
        let carrier = Complex64::new(carrier_phase.cos(), carrier_phase.sin());
        Some((env * carrier).re)
    }

    /// Reconstruct full waveform at carrier sample rate
    ///
    /// # Arguments
    /// * `samples_per_carrier_cycle` - How many samples per carrier period
    ///
    /// # Returns
    /// (time, values) tuple with full waveform
    pub fn reconstruct_waveform(
        &self,
        samples_per_carrier_cycle: usize,
    ) -> (Vec<Value>, Vec<Value>) {
        if self.time.is_empty() {
            return (Vec::new(), Vec::new());
        }
        if samples_per_carrier_cycle == 0 {
            return (Vec::new(), Vec::new());
        }
        if !self.carrier_freq.is_finite() || self.carrier_freq <= 0.0 {
            return (Vec::new(), Vec::new());
        }

        let duration = self.duration();
        if !duration.is_finite() || duration < 0.0 {
            return (Vec::new(), Vec::new());
        }

        let carrier_period = 1.0 / self.carrier_freq;
        if !carrier_period.is_finite() || carrier_period <= 0.0 {
            return (Vec::new(), Vec::new());
        }
        let dt = carrier_period / samples_per_carrier_cycle as Value;
        if !dt.is_finite() || dt <= 0.0 {
            return (Vec::new(), Vec::new());
        }

        let samples = duration / dt;
        if !samples.is_finite() || samples < 0.0 || samples > (usize::MAX - 1) as Value {
            return (Vec::new(), Vec::new());
        }
        let num_samples = samples as usize + 1;

        let mut time = Vec::with_capacity(num_samples);
        let mut values = Vec::with_capacity(num_samples);

        let t_start = self.time[0];

        for i in 0..num_samples {
            let t = t_start + i as Value * dt;
            time.push(t);
            values.push(self.reconstruct_at(t).unwrap_or(0.0));
        }

        (time, values)
    }

    /// Get amplitude envelope (magnitude)
    pub fn amplitude(&self) -> Vec<Value> {
        self.envelope.iter().map(|e| e.norm()).collect()
    }

    /// Get phase envelope (angle in radians)
    pub fn phase(&self) -> Vec<Value> {
        self.envelope.iter().map(|e| e.arg()).collect()
    }

    /// Get instantaneous frequency deviation from carrier
    ///
    /// f_inst(t) = (1/2π) × dφ/dt
    pub fn instantaneous_frequency(&self) -> Vec<Value> {
        if self.time.len() < 2 {
            return Vec::new();
        }

        let phase = self.phase();
        let mut freq = Vec::with_capacity(phase.len());

        // Unwrap phase first
        let mut unwrapped = vec![phase[0]];
        for i in 1..phase.len() {
            let mut diff = phase[i] - unwrapped[i - 1];
            // Unwrap
            while diff > PI {
                diff -= 2.0 * PI;
            }
            while diff < -PI {
                diff += 2.0 * PI;
            }
            unwrapped.push(unwrapped[i - 1] + diff);
        }

        // Compute derivative
        for i in 0..unwrapped.len() {
            let dphi_dt = if i == 0 {
                (unwrapped[1] - unwrapped[0]) / (self.time[1] - self.time[0])
            } else if i == unwrapped.len() - 1 {
                (unwrapped[i] - unwrapped[i - 1]) / (self.time[i] - self.time[i - 1])
            } else {
                (unwrapped[i + 1] - unwrapped[i - 1]) / (self.time[i + 1] - self.time[i - 1])
            };
            freq.push(dphi_dt / (2.0 * PI));
        }

        freq
    }

    /// Compute AM-AM (amplitude-to-amplitude) distortion
    ///
    /// Returns (input_amplitude, output_amplitude) pairs
    pub fn am_am(&self, reference: &EnvelopeSignal) -> Vec<(Value, Value)> {
        let mut result = Vec::new();

        for (i, &t) in self.time.iter().enumerate() {
            if let Some(ref_env) = reference.at(t) {
                let input_amp = ref_env.norm();
                let output_amp = self.envelope[i].norm();
                result.push((input_amp, output_amp));
            }
        }

        result
    }

    /// Compute AM-PM (amplitude-to-phase) distortion
    ///
    /// Returns (input_amplitude, output_phase) pairs in degrees
    pub fn am_pm(&self, reference: &EnvelopeSignal) -> Vec<(Value, Value)> {
        let mut result = Vec::new();

        for (i, &t) in self.time.iter().enumerate() {
            if let Some(ref_env) = reference.at(t) {
                let input_amp = ref_env.norm();
                let output_phase = self.envelope[i].arg() * 180.0 / PI;
                result.push((input_amp, output_phase));
            }
        }

        result
    }
}

//=============================================================================
// Envelope Transient Configuration
//=============================================================================

/// Configuration for envelope transient analysis
#[derive(Debug, Clone)]
pub struct EnvelopeTransientConfig {
    /// Carrier frequency (Hz)
    pub carrier_freq: Value,
    /// Start time
    pub t_start: Value,
    /// Stop time
    pub t_stop: Value,
    /// Maximum envelope timestep (seconds)
    pub max_timestep: Value,
    /// Minimum envelope timestep (seconds)
    pub min_timestep: Value,
    /// Relative tolerance for envelope timestep control
    pub rel_tol: Value,
    /// Absolute tolerance for envelope amplitude
    pub abs_tol: Value,
    /// Number of harmonics to include (0 = envelope only)
    pub num_harmonics: usize,
    /// Initial PSS solution (complex amplitudes at carrier harmonics)
    pub initial_pss: Option<Vec<Complex64>>,
}

impl Default for EnvelopeTransientConfig {
    fn default() -> Self {
        Self {
            carrier_freq: 1e9, // 1 GHz
            t_start: 0.0,
            t_stop: 1e-6,        // 1 μs
            max_timestep: 1e-9,  // 1 ns
            min_timestep: 1e-12, // 1 ps
            rel_tol: 1e-3,
            abs_tol: 1e-9,
            num_harmonics: 0,
            initial_pss: None,
        }
    }
}

impl EnvelopeTransientConfig {
    /// Create config for given carrier and modulation bandwidth
    pub fn new(carrier_freq: Value, modulation_bandwidth: Value) -> Self {
        let max_timestep = ENVELOPE_TIMESTEP_FACTOR / modulation_bandwidth;
        let carrier_period = 1.0 / carrier_freq;

        // Ensure timestep is much larger than carrier period
        let max_timestep = max_timestep.max(carrier_period * MAX_TIMESTEP_RATIO);

        Self {
            carrier_freq,
            max_timestep,
            min_timestep: max_timestep / 1000.0,
            ..Default::default()
        }
    }

    /// Set simulation time range
    pub fn with_time_range(mut self, t_start: Value, t_stop: Value) -> Self {
        self.t_start = t_start;
        self.t_stop = t_stop;
        self
    }

    /// Set number of harmonics
    pub fn with_harmonics(mut self, n: usize) -> Self {
        self.num_harmonics = n;
        self
    }

    /// Set initial PSS solution
    pub fn with_pss(mut self, pss_solution: Vec<Complex64>) -> Self {
        self.initial_pss = Some(pss_solution);
        self
    }

    /// Set tolerances
    pub fn with_tolerances(mut self, rel_tol: Value, abs_tol: Value) -> Self {
        self.rel_tol = rel_tol;
        self.abs_tol = abs_tol;
        self
    }

    /// Get carrier period
    #[inline]
    pub fn carrier_period(&self) -> Value {
        1.0 / self.carrier_freq
    }
}

//=============================================================================
// Envelope Transient State
//=============================================================================

/// State during envelope transient simulation
#[derive(Debug, Clone)]
pub struct EnvelopeTransientState {
    /// Current simulation time
    pub time: Value,
    /// Current envelope values for each node
    pub envelopes: Vec<Complex64>,
    /// Current timestep
    pub timestep: Value,
    /// Previous envelopes (for LTE estimation)
    pub prev_envelopes: Vec<Complex64>,
    /// Previous timestep
    pub prev_timestep: Value,
    /// Number of steps taken
    pub steps: usize,
    /// Number of rejected steps
    pub rejected_steps: usize,
    /// Total equation evaluations
    pub evaluations: usize,
}

impl EnvelopeTransientState {
    /// Create new state with initial envelopes
    pub fn new(initial_envelopes: Vec<Complex64>) -> Self {
        Self {
            time: 0.0,
            envelopes: initial_envelopes.clone(),
            timestep: 1e-9,
            prev_envelopes: initial_envelopes,
            prev_timestep: 1e-9,
            steps: 0,
            rejected_steps: 0,
            evaluations: 0,
        }
    }

    /// Accept current step and advance
    pub fn accept_step(&mut self, new_envelopes: Vec<Complex64>, dt: Value) {
        self.prev_envelopes = std::mem::replace(&mut self.envelopes, new_envelopes);
        self.prev_timestep = self.timestep;
        self.timestep = dt;
        self.time += dt;
        self.steps += 1;
    }

    /// Reject step and reduce timestep
    pub fn reject_step(&mut self, new_dt: Value) {
        self.timestep = new_dt;
        self.rejected_steps += 1;
    }

    /// Number of envelope nodes
    #[inline]
    pub fn num_nodes(&self) -> usize {
        self.envelopes.len()
    }
}

//=============================================================================
// Envelope Transient Result
//=============================================================================

/// Result of envelope transient analysis
#[derive(Debug, Clone)]
pub struct EnvelopeTransientResult {
    /// Configuration used
    pub config: EnvelopeTransientConfig,
    /// Envelope signals for each node
    pub node_envelopes: Vec<EnvelopeSignal>,
    /// Node names
    pub node_names: Vec<String>,
    /// Total simulation time (wall clock)
    pub simulation_time_seconds: Value,
    /// Number of envelope timesteps
    pub num_steps: usize,
    /// Number of rejected steps
    pub rejected_steps: usize,
    /// Speedup vs full transient (estimated)
    pub speedup_factor: Value,
}

impl EnvelopeTransientResult {
    /// Create new result
    pub fn new(config: EnvelopeTransientConfig) -> Self {
        Self {
            config,
            node_envelopes: Vec::new(),
            node_names: Vec::new(),
            simulation_time_seconds: 0.0,
            num_steps: 0,
            rejected_steps: 0,
            speedup_factor: 1.0,
        }
    }

    /// Add node envelope result
    pub fn add_node(&mut self, name: String, signal: EnvelopeSignal) {
        self.node_names.push(name);
        self.node_envelopes.push(signal);
    }

    /// Get envelope for node by name
    pub fn get_node(&self, name: &str) -> Option<&EnvelopeSignal> {
        self.node_names
            .iter()
            .position(|n| n == name)
            .map(|i| &self.node_envelopes[i])
    }

    /// Compute estimated speedup vs full transient
    pub fn compute_speedup(&mut self) {
        let carrier_period = self.config.carrier_period();
        let duration = self.config.t_stop - self.config.t_start;

        // Estimate: full transient needs ~10 points per carrier cycle
        let full_transient_steps = (duration / carrier_period * 10.0) as usize;

        if self.num_steps > 0 {
            self.speedup_factor = full_transient_steps as Value / self.num_steps as Value;
        }
    }
}

//=============================================================================
// Envelope Transient Solver
//=============================================================================

/// Envelope transient solver
///
/// Uses modified transient analysis that works on complex envelope
/// rather than instantaneous voltage.
#[derive(Debug)]
pub struct EnvelopeTransientSolver {
    /// Configuration
    config: EnvelopeTransientConfig,
    /// Current state
    state: EnvelopeTransientState,
    /// Recorded time points
    recorded_time: Vec<Value>,
    /// Recorded envelopes per node
    recorded_envelopes: Vec<Vec<Complex64>>,
}

impl EnvelopeTransientSolver {
    /// Create new envelope transient solver
    pub fn new(config: EnvelopeTransientConfig, num_nodes: usize) -> Self {
        let initial = config
            .initial_pss
            .clone()
            .unwrap_or_else(|| vec![Complex64::new(0.01, 0.0); num_nodes]);

        Self {
            state: EnvelopeTransientState::new(initial),
            recorded_time: Vec::new(),
            recorded_envelopes: vec![Vec::new(); num_nodes],
            config,
        }
    }

    /// Initialize from PSS solution
    pub fn from_pss(config: EnvelopeTransientConfig, pss_envelopes: Vec<Complex64>) -> Self {
        let num_nodes = pss_envelopes.len();
        Self {
            state: EnvelopeTransientState::new(pss_envelopes),
            recorded_time: Vec::new(),
            recorded_envelopes: vec![Vec::new(); num_nodes],
            config,
        }
    }

    /// Get current time
    #[inline]
    pub fn current_time(&self) -> Value {
        self.state.time
    }

    /// Check if simulation is complete
    #[inline]
    pub fn is_complete(&self) -> bool {
        self.state.time >= self.config.t_stop
    }

    /// Take a single envelope timestep
    ///
    /// # Arguments
    /// * `compute_envelope_derivative` - Closure that computes dE/dt for each node
    ///
    /// # Returns
    /// Whether the step was accepted
    pub fn step<F>(&mut self, compute_envelope_derivative: F) -> bool
    where
        F: Fn(&[Complex64], Value) -> Vec<Complex64>,
    {
        let dt = self
            .state
            .timestep
            .min(self.config.t_stop - self.state.time);

        if dt <= 0.0 {
            return false;
        }

        self.state.evaluations += 1;

        // Compute derivative at current point
        let k1 = compute_envelope_derivative(&self.state.envelopes, self.state.time);

        // Forward Euler prediction
        let e_predict: Vec<Complex64> = self
            .state
            .envelopes
            .iter()
            .zip(k1.iter())
            .map(|(&e, &de)| e + de * dt)
            .collect();

        self.state.evaluations += 1;

        // Compute derivative at predicted point (for error estimate)
        let k2 = compute_envelope_derivative(&e_predict, self.state.time + dt);

        // Trapezoidal corrector
        let e_new: Vec<Complex64> = self
            .state
            .envelopes
            .iter()
            .zip(k1.iter())
            .zip(k2.iter())
            .map(|((&e, &de1), &de2)| e + (de1 + de2) * 0.5 * dt)
            .collect();

        // Error estimate (difference between predictor and corrector)
        let mut max_error = 0.0_f64;
        for (&e_p, &e_c) in e_predict.iter().zip(e_new.iter()) {
            let err = (e_c - e_p).norm();
            let scale = e_c.norm().max(self.config.abs_tol);
            let rel_err = err / scale;
            max_error = max_error.max(rel_err);

            // Also check absolute error
            let abs_err = err;
            if abs_err > self.config.abs_tol && e_c.norm() < self.config.abs_tol * 10.0 {
                max_error = max_error.max(1.0);
            }
        }

        // Decide whether to accept
        let accept = max_error <= self.config.rel_tol;

        if accept {
            self.state.accept_step(e_new, dt);

            // Record results
            self.recorded_time.push(self.state.time);
            for (i, e) in self.state.envelopes.iter().enumerate() {
                self.recorded_envelopes[i].push(*e);
            }
        } else {
            // Reduce timestep
            let factor = 0.9 * (self.config.rel_tol / max_error).powf(0.5);
            let new_dt = (dt * factor.clamp(0.1, 0.9)).max(self.config.min_timestep);
            self.state.reject_step(new_dt);
        }

        // Adjust timestep for next step
        if accept && max_error > 0.0 {
            let factor = 0.9 * (self.config.rel_tol / max_error).powf(0.5);
            self.state.timestep = (self.state.timestep * factor.clamp(0.5, 2.0))
                .clamp(self.config.min_timestep, self.config.max_timestep);
        }

        accept
    }

    /// Run full envelope transient simulation
    ///
    /// # Arguments
    /// * `compute_envelope_derivative` - Closure computing dE/dt
    ///
    /// # Returns
    /// Envelope transient result
    pub fn run<F>(&mut self, compute_envelope_derivative: F) -> EnvelopeTransientResult
    where
        F: Fn(&[Complex64], Value) -> Vec<Complex64>,
    {
        // Record initial point
        self.recorded_time.push(self.state.time);
        for (i, e) in self.state.envelopes.iter().enumerate() {
            self.recorded_envelopes[i].push(*e);
        }

        // Main simulation loop
        while !self.is_complete() {
            self.step(&compute_envelope_derivative);

            // Safety limit
            if self.state.steps > 1_000_000 {
                break;
            }
        }

        self.build_result()
    }

    /// Build result from recorded data
    fn build_result(&self) -> EnvelopeTransientResult {
        let mut result = EnvelopeTransientResult::new(self.config.clone());

        for (i, envelopes) in self.recorded_envelopes.iter().enumerate() {
            let signal = EnvelopeSignal::from_data(
                self.recorded_time.clone(),
                envelopes.clone(),
                self.config.carrier_freq,
            );
            result.add_node(format!("node_{}", i), signal);
        }

        result.num_steps = self.state.steps;
        result.rejected_steps = self.state.rejected_steps;
        result.compute_speedup();

        result
    }

    /// Get current state
    pub fn state(&self) -> &EnvelopeTransientState {
        &self.state
    }
}

//=============================================================================
// Modulation Sources
//=============================================================================

/// AM (Amplitude Modulation) envelope source
#[derive(Debug, Clone)]
pub struct AmSource {
    /// Carrier amplitude
    pub carrier_amplitude: Value,
    /// Modulation depth (0 to 1)
    pub modulation_depth: Value,
    /// Modulation frequency (Hz)
    pub modulation_freq: Value,
}

impl AmSource {
    /// Create new AM source
    pub fn new(carrier_amp: Value, mod_depth: Value, mod_freq: Value) -> Self {
        Self {
            carrier_amplitude: carrier_amp,
            modulation_depth: mod_depth,
            modulation_freq: mod_freq,
        }
    }

    /// Compute envelope at time t
    pub fn envelope_at(&self, t: Value) -> Complex64 {
        let amp = self.carrier_amplitude
            * (1.0 + self.modulation_depth * (2.0 * PI * self.modulation_freq * t).cos());
        Complex64::new(amp, 0.0)
    }
}

/// FM (Frequency Modulation) envelope source
#[derive(Debug, Clone)]
pub struct FmSource {
    /// Carrier amplitude
    pub carrier_amplitude: Value,
    /// Frequency deviation (Hz)
    pub frequency_deviation: Value,
    /// Modulation frequency (Hz)
    pub modulation_freq: Value,
}

impl FmSource {
    /// Create new FM source
    pub fn new(carrier_amp: Value, freq_dev: Value, mod_freq: Value) -> Self {
        Self {
            carrier_amplitude: carrier_amp,
            frequency_deviation: freq_dev,
            modulation_freq: mod_freq,
        }
    }

    /// Compute envelope at time t
    pub fn envelope_at(&self, t: Value) -> Complex64 {
        // FM: constant amplitude, varying phase
        // Phase: φ(t) = β × sin(ωm × t), where β = Δf/fm
        let beta = self.frequency_deviation / self.modulation_freq;
        let phase = beta * (2.0 * PI * self.modulation_freq * t).sin();

        self.carrier_amplitude * Complex64::new(phase.cos(), phase.sin())
    }
}

/// Digital modulation (generic IQ) source
#[derive(Debug, Clone)]
pub struct IqSource {
    /// I/Q symbol sequence
    pub symbols: Vec<Complex64>,
    /// Symbol rate (symbols/second)
    pub symbol_rate: Value,
    /// Pulse shaping filter type
    pub pulse_shape: PulseShape,
    /// Roll-off factor for raised cosine
    pub roll_off: Value,
}

/// Pulse shaping filter types
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Default)]
pub enum PulseShape {
    /// Rectangular (no shaping)
    Rectangular,
    /// Raised cosine
    #[default]
    RaisedCosine,
    /// Root raised cosine
    RootRaisedCosine,
    /// Gaussian
    Gaussian,
}


impl IqSource {
    /// Create new IQ source
    pub fn new(symbols: Vec<Complex64>, symbol_rate: Value) -> Self {
        Self {
            symbols,
            symbol_rate,
            pulse_shape: PulseShape::RaisedCosine,
            roll_off: 0.35,
        }
    }

    /// Compute envelope at time t (with pulse shaping)
    pub fn envelope_at(&self, t: Value) -> Complex64 {
        if self.symbols.is_empty() {
            return Complex64::new(0.0, 0.0);
        }

        let symbol_period = 1.0 / self.symbol_rate;

        // Find relevant symbols
        let symbol_idx = (t / symbol_period) as usize;
        let symbol_phase = (t / symbol_period).fract();

        match self.pulse_shape {
            PulseShape::Rectangular => {
                if symbol_idx < self.symbols.len() {
                    self.symbols[symbol_idx]
                } else {
                    *self.symbols.last().unwrap_or(&Complex64::new(0.0, 0.0))
                }
            }
            PulseShape::RaisedCosine => {
                // Interpolate between adjacent symbols
                let s0 = self
                    .symbols
                    .get(symbol_idx)
                    .copied()
                    .unwrap_or(Complex64::new(0.0, 0.0));
                let s1 = self.symbols.get(symbol_idx + 1).copied().unwrap_or(s0);

                // Raised cosine interpolation
                let alpha = (1.0 - (PI * symbol_phase).cos()) * 0.5;
                s0 * (1.0 - alpha) + s1 * alpha
            }
            _ => {
                // Default to rectangular for other shapes
                if symbol_idx < self.symbols.len() {
                    self.symbols[symbol_idx]
                } else {
                    Complex64::new(0.0, 0.0)
                }
            }
        }
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_envelope_signal_creation() {
        let carrier_freq = 1e9;
        let signal = EnvelopeSignal::new(carrier_freq);

        assert!(signal.is_empty());
        assert_eq!(signal.carrier_freq, carrier_freq);
    }

    #[test]
    fn test_envelope_signal_from_data() {
        let time = vec![0.0, 1e-9, 2e-9];
        let envelope = vec![
            Complex64::new(1.0, 0.0),
            Complex64::new(0.8, 0.2),
            Complex64::new(0.6, 0.4),
        ];

        let signal = EnvelopeSignal::from_data(time, envelope, 1e9);

        assert_eq!(signal.len(), 3);
        assert!((signal.duration() - 2e-9).abs() < 1e-15);
    }

    #[test]
    fn test_envelope_interpolation() {
        let time = vec![0.0, 1.0, 2.0];
        let envelope = vec![
            Complex64::new(0.0, 0.0),
            Complex64::new(1.0, 0.0),
            Complex64::new(2.0, 0.0),
        ];

        let signal = EnvelopeSignal::from_data(time, envelope, 1e9);

        // Interpolate at midpoint
        let interp = signal.at(0.5).unwrap();
        assert!((interp.re - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_envelope_reconstruction() {
        let carrier_freq = 1e6; // 1 MHz for easy calculation
        let time = vec![0.0, 1e-6, 2e-6];
        let envelope = vec![
            Complex64::new(1.0, 0.0),
            Complex64::new(1.0, 0.0),
            Complex64::new(1.0, 0.0),
        ];

        let signal = EnvelopeSignal::from_data(time, envelope, carrier_freq);

        // At t=0, with unity envelope, the reconstructed value should be 1.0
        let val = signal.reconstruct_at(0.0).unwrap();
        assert!((val - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_envelope_amplitude_phase() {
        let time = vec![0.0, 1.0];
        let envelope = vec![Complex64::new(1.0, 0.0), Complex64::new(0.0, 1.0)];

        let signal = EnvelopeSignal::from_data(time, envelope, 1e9);

        let amp = signal.amplitude();
        assert!((amp[0] - 1.0).abs() < 1e-10);
        assert!((amp[1] - 1.0).abs() < 1e-10);

        let phase = signal.phase();
        assert!(phase[0].abs() < 1e-10);
        assert!((phase[1] - PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_envelope_transient_config_default() {
        let config = EnvelopeTransientConfig::default();

        assert!((config.carrier_freq - 1e9).abs() < 1.0);
        assert!(config.max_timestep > config.min_timestep);
    }

    #[test]
    fn test_envelope_transient_config_from_bandwidth() {
        let carrier = 2.4e9;
        let bandwidth = 20e6;

        let config = EnvelopeTransientConfig::new(carrier, bandwidth);

        assert_eq!(config.carrier_freq, carrier);
        // Timestep should be related to envelope bandwidth
        assert!(config.max_timestep > 0.0);
    }

    #[test]
    fn test_envelope_transient_state() {
        let initial = vec![Complex64::new(1.0, 0.0), Complex64::new(0.5, 0.5)];
        let state = EnvelopeTransientState::new(initial);

        assert_eq!(state.num_nodes(), 2);
        assert_eq!(state.steps, 0);
    }

    #[test]
    fn test_envelope_transient_solver_creation() {
        let config = EnvelopeTransientConfig::default();
        let solver = EnvelopeTransientSolver::new(config, 2);

        assert_eq!(solver.current_time(), 0.0);
        assert!(!solver.is_complete());
    }

    #[test]
    fn test_envelope_transient_single_step() {
        let mut config = EnvelopeTransientConfig::default();
        config.t_stop = 10e-9;
        config.max_timestep = 1e-9;

        let mut solver = EnvelopeTransientSolver::new(config, 1);

        // Simple decay: dE/dt = -E
        let _accepted = solver.step(|e, _t| e.iter().map(|&env| -env * 1e8).collect());

        // Should take at least one step
        assert!(solver.state.steps > 0 || solver.state.rejected_steps > 0);
    }

    #[test]
    fn test_am_source() {
        let source = AmSource::new(1.0, 0.5, 1e3);

        // At t=0, should have maximum amplitude (1 + 0.5)
        let env = source.envelope_at(0.0);
        assert!((env.norm() - 1.5).abs() < 1e-10);

        // At t=0.5/f_m, should have minimum (1 - 0.5)
        let t_half_period = 0.5 / 1e3;
        let env_min = source.envelope_at(t_half_period);
        assert!((env_min.norm() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_fm_source() {
        let source = FmSource::new(1.0, 75e3, 15e3);

        // Amplitude should be constant
        let env1 = source.envelope_at(0.0);
        let env2 = source.envelope_at(1e-5);

        assert!((env1.norm() - 1.0).abs() < 1e-10);
        assert!((env2.norm() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_iq_source_rectangular() {
        let symbols = vec![Complex64::new(1.0, 1.0), Complex64::new(-1.0, 1.0)];

        let mut source = IqSource::new(symbols, 1e6);
        source.pulse_shape = PulseShape::Rectangular;

        // First symbol period
        let env = source.envelope_at(0.5e-6);
        assert!((env.re - 1.0).abs() < 1e-10);
        assert!((env.im - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_envelope_result() {
        let config = EnvelopeTransientConfig::default();
        let mut result = EnvelopeTransientResult::new(config);

        let signal = EnvelopeSignal::from_data(
            vec![0.0, 1.0],
            vec![Complex64::new(1.0, 0.0), Complex64::new(0.5, 0.0)],
            1e9,
        );

        result.add_node("out".to_string(), signal);

        assert!(result.get_node("out").is_some());
        assert!(result.get_node("missing").is_none());
    }

    #[test]
    fn test_pulse_shape_default() {
        assert_eq!(PulseShape::default(), PulseShape::RaisedCosine);
    }

    #[test]
    fn test_reconstruct_waveform() {
        let carrier_freq = 1e6;
        let time = vec![0.0, 1e-6];
        let envelope = vec![Complex64::new(1.0, 0.0), Complex64::new(1.0, 0.0)];

        let signal = EnvelopeSignal::from_data(time, envelope, carrier_freq);
        let (t_out, v_out) = signal.reconstruct_waveform(10);

        assert!(!t_out.is_empty());
        assert_eq!(t_out.len(), v_out.len());
    }

    #[test]
    fn test_reconstruct_waveform_rejects_zero_samples_per_cycle() {
        let signal = EnvelopeSignal::from_data(
            vec![0.0, 1e-6],
            vec![Complex64::new(1.0, 0.0), Complex64::new(1.0, 0.0)],
            1e6,
        );

        let (t_out, v_out) = signal.reconstruct_waveform(0);
        assert!(t_out.is_empty());
        assert!(v_out.is_empty());
    }

    #[test]
    fn test_reconstruct_waveform_rejects_non_positive_carrier_frequency() {
        let signal = EnvelopeSignal::from_data(
            vec![0.0, 1e-6],
            vec![Complex64::new(1.0, 0.0), Complex64::new(1.0, 0.0)],
            0.0,
        );
        let (t_out, v_out) = signal.reconstruct_waveform(16);
        assert!(t_out.is_empty());
        assert!(v_out.is_empty());

        let signal_neg = EnvelopeSignal::from_data(
            vec![0.0, 1e-6],
            vec![Complex64::new(1.0, 0.0), Complex64::new(1.0, 0.0)],
            -1e6,
        );
        let (t_out_neg, v_out_neg) = signal_neg.reconstruct_waveform(16);
        assert!(t_out_neg.is_empty());
        assert!(v_out_neg.is_empty());
    }

    #[test]
    fn test_reconstruct_waveform_rejects_non_finite_carrier_frequency() {
        let signal = EnvelopeSignal::from_data(
            vec![0.0, 1e-6],
            vec![Complex64::new(1.0, 0.0), Complex64::new(1.0, 0.0)],
            f64::INFINITY,
        );

        let (t_out, v_out) = signal.reconstruct_waveform(16);
        assert!(t_out.is_empty());
        assert!(v_out.is_empty());
    }

    #[test]
    fn test_reconstruct_waveform_rejects_non_finite_duration() {
        let signal = EnvelopeSignal::from_data(
            vec![0.0, f64::INFINITY],
            vec![Complex64::new(1.0, 0.0), Complex64::new(1.0, 0.0)],
            1e6,
        );

        let (t_out, v_out) = signal.reconstruct_waveform(16);
        assert!(t_out.is_empty());
        assert!(v_out.is_empty());
    }

    #[test]
    fn test_am_am_computation() {
        let time = vec![0.0, 1.0, 2.0];

        let ref_signal = EnvelopeSignal::from_data(
            time.clone(),
            vec![
                Complex64::new(0.5, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(1.5, 0.0),
            ],
            1e9,
        );

        let out_signal = EnvelopeSignal::from_data(
            time.clone(),
            vec![
                Complex64::new(0.4, 0.0),
                Complex64::new(0.9, 0.0),
                Complex64::new(1.2, 0.0),
            ],
            1e9,
        );

        let am_am = out_signal.am_am(&ref_signal);
        assert_eq!(am_am.len(), 3);

        // Output should show compression at high power
        assert!(am_am[2].1 < am_am[2].0); // Compressed at highest input
    }

    #[test]
    fn test_instantaneous_frequency() {
        // Signal with linear phase ramp = constant frequency offset
        let time: Vec<Value> = (0..100).map(|i| i as Value * 1e-9).collect();
        let freq_offset = 1e6; // 1 MHz offset
        let envelope: Vec<Complex64> = time
            .iter()
            .map(|&t| {
                let phase = 2.0 * PI * freq_offset * t;
                Complex64::new(phase.cos(), phase.sin())
            })
            .collect();

        let signal = EnvelopeSignal::from_data(time, envelope, 1e9);
        let inst_freq = signal.instantaneous_frequency();

        // Should be approximately constant at freq_offset
        for (i, &f) in inst_freq.iter().enumerate().skip(5).take(90) {
            assert!(
                (f - freq_offset).abs() < freq_offset * 0.1,
                "Freq at {} = {} (expected {})",
                i,
                f,
                freq_offset
            );
        }
    }

    #[test]
    fn test_envelope_with_initial_phase() {
        let signal = EnvelopeSignal::new(1e9).with_initial_phase(PI / 4.0);

        assert!((signal.initial_phase - PI / 4.0).abs() < 1e-10);
    }
}
