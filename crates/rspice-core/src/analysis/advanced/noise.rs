//! Noise Analysis Module
//!
//! Computes noise spectral density as a function of frequency.

#![allow(clippy::needless_range_loop)]
//! Supports:
//! - **Thermal noise**: Johnson-Nyquist noise from resistors (4kTR)
//! - **Shot noise**: From PN junctions in diodes and BJTs (2qI)
//! - **Flicker noise**: 1/f^EF noise in semiconductors (KF * I^AF / f^EF)
//!
//! # Algorithm
//! 1. Compute small-signal AC solution at each frequency
//! 2. For each noise source, compute noise current spectral density
//! 3. Use transfer function to compute output voltage noise contribution
//! 4. Sum all contributions (in mean-square) for total output noise
//!
//! # Example
//! ```ignore
//! .NOISE V(out) Vin DEC 10 1Hz 100kHz
//! ```
//! This computes noise at output node referenced to input source Vin.

use crate::Value;
use crate::analysis::AnalysisConfig;

//=============================================================================
// Constants
//=============================================================================

/// Boltzmann constant (J/K)
pub const K_BOLTZMANN: Value = 1.380649e-23;
/// Electron charge (C)
pub const Q_ELECTRON: Value = 1.602176634e-19;
/// Default temperature (K) = 300K = 27°C
pub const T_NOMINAL: Value = 300.0;

//=============================================================================
// Noise Source Types
//=============================================================================

/// Types of noise sources in the circuit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoiseSourceType {
    /// Thermal (Johnson-Nyquist) noise: 4kTR
    Thermal,
    /// Shot noise: 2qI
    Shot,
    /// Flicker (1/f) noise: KF * I^AF / f
    Flicker,
    /// Burst (popcorn) noise: KB * I^AB / (1 + (f/FB)^2)
    /// Lorentzian spectrum with corner frequency FB
    Burst,
}

/// A noise source in the circuit
#[derive(Debug, Clone)]
pub struct NoiseSource {
    /// Name of the device generating this noise
    pub device_name: String,
    /// Type of noise
    pub noise_type: NoiseSourceType,
    /// Node where noise current is injected (+)
    pub node_pos: usize,
    /// Node where noise current is injected (-)
    pub node_neg: usize,
    /// Spectral density parameter (R for thermal, I for shot, KF for flicker)
    pub parameter: Value,
    /// Flicker noise exponent (AF, typically 1.0)
    pub af: Value,
    /// Flicker frequency exponent (EF, typically 1.0)
    pub ef: Value,
    /// Current for flicker/burst noise
    pub current: Value,
    /// Corner frequency for burst noise (FB, Hz)
    pub corner_freq: Value,
}

impl NoiseSource {
    /// Create a thermal noise source (resistor)
    pub fn thermal(
        device_name: String,
        node_pos: usize,
        node_neg: usize,
        resistance: Value,
    ) -> Self {
        Self {
            device_name,
            noise_type: NoiseSourceType::Thermal,
            node_pos,
            node_neg,
            parameter: resistance,
            af: 1.0,
            ef: 1.0,
            current: 0.0,
            corner_freq: 1.0,
        }
    }

    /// Create a shot noise source (diode/BJT junction)
    pub fn shot(device_name: String, node_pos: usize, node_neg: usize, current: Value) -> Self {
        Self {
            device_name,
            noise_type: NoiseSourceType::Shot,
            node_pos,
            node_neg,
            parameter: current.abs(),
            af: 1.0,
            ef: 1.0,
            current: 0.0,
            corner_freq: 1.0,
        }
    }

    /// Create a flicker (1/f) noise source
    pub fn flicker(
        device_name: String,
        node_pos: usize,
        node_neg: usize,
        kf: Value,
        af: Value,
        current: Value,
    ) -> Self {
        Self::flicker_with_frequency_exponent(device_name, node_pos, node_neg, kf, af, 1.0, current)
    }

    /// Create a flicker (1/f^EF) noise source
    pub fn flicker_with_frequency_exponent(
        device_name: String,
        node_pos: usize,
        node_neg: usize,
        kf: Value,
        af: Value,
        ef: Value,
        current: Value,
    ) -> Self {
        Self {
            device_name,
            noise_type: NoiseSourceType::Flicker,
            node_pos,
            node_neg,
            parameter: kf,
            af,
            ef,
            current,
            corner_freq: 1.0,
        }
    }

    /// Create a burst (popcorn) noise source
    ///
    /// Burst noise has a Lorentzian spectrum: Si = KB * I^AB / (1 + (f/FB)^2)
    /// where KB is the burst noise coefficient, AB is the exponent (typically 2),
    /// and FB is the corner frequency.
    pub fn burst(
        device_name: String,
        node_pos: usize,
        node_neg: usize,
        kb: Value,
        ab: Value,
        current: Value,
        corner_freq: Value,
    ) -> Self {
        Self {
            device_name,
            noise_type: NoiseSourceType::Burst,
            node_pos,
            node_neg,
            parameter: kb,
            af: ab, // Reuse af field for AB exponent
            ef: 1.0,
            current,
            corner_freq,
        }
    }

    /// Compute current noise spectral density (A²/Hz) at given frequency
    pub fn spectral_density(&self, frequency: Value, temperature: Value) -> Value {
        match self.noise_type {
            NoiseSourceType::Thermal => {
                // Thermal noise: Si = 4kT/R (current noise spectral density)
                // For resistor R: current noise = 4kT/R A²/Hz
                if self.parameter > 0.0 {
                    4.0 * K_BOLTZMANN * temperature / self.parameter
                } else {
                    0.0
                }
            }
            NoiseSourceType::Shot => {
                // Shot noise: Si = 2qI (A²/Hz)
                2.0 * Q_ELECTRON * self.parameter
            }
            NoiseSourceType::Flicker => {
                // Flicker noise: Si = KF * I^AF / f^EF (A²/Hz)
                if frequency > 0.0 {
                    self.parameter * self.current.abs().powf(self.af) / frequency.powf(self.ef)
                } else {
                    0.0 // Avoid division by zero
                }
            }
            NoiseSourceType::Burst => {
                // Burst (popcorn) noise: Si = KB * I^AB / (1 + (f/FB)^2)
                // Lorentzian spectrum with corner frequency FB
                let kb = self.parameter;
                let ab = self.af;
                let fb = self.corner_freq;
                let f_ratio = frequency / fb;
                kb * self.current.abs().powf(ab) / (1.0 + f_ratio * f_ratio)
            }
        }
    }
}

//=============================================================================
// Noise Analysis Engine
//=============================================================================

/// Noise Analysis engine
#[derive(Debug)]
pub struct NoiseAnalysis {
    config: AnalysisConfig,
    /// Frequency points for analysis
    frequencies: Vec<Value>,
    /// Temperature for noise calculations (K)
    temperature: Value,
    /// Output node for noise measurement
    output_node: Option<usize>,
    /// Reference node (usually ground)
    reference_node: usize,
    /// Input source name for input-referred noise
    input_source: Option<String>,
}

impl NoiseAnalysis {
    /// Create a new noise analysis
    pub fn new(config: AnalysisConfig) -> Self {
        Self {
            config,
            frequencies: Vec::new(),
            temperature: T_NOMINAL,
            output_node: None,
            reference_node: 0,
            input_source: None,
        }
    }

    /// Set output node for noise measurement
    pub fn set_output(&mut self, node: usize) {
        self.output_node = Some(node);
    }

    /// Set reference node (default is ground = 0)
    pub fn set_reference(&mut self, node: usize) {
        self.reference_node = node;
    }

    /// Set input source for input-referred noise calculation
    pub fn set_input_source(&mut self, source_name: String) {
        self.input_source = Some(source_name);
    }

    /// Set temperature for noise calculations
    pub fn set_temperature(&mut self, temp_kelvin: Value) {
        self.temperature = temp_kelvin;
    }

    /// Set up decade frequency sweep
    pub fn decade_sweep(&mut self, start: Value, stop: Value, points_per_decade: usize) {
        let start_log = start.log10();
        let stop_log = stop.log10();
        let num_decades = stop_log - start_log;
        let total_points = (num_decades * points_per_decade as f64).ceil() as usize;

        self.frequencies = (0..=total_points)
            .map(|i| {
                let log_f = start_log + (stop_log - start_log) * (i as f64) / (total_points as f64);
                10.0_f64.powf(log_f)
            })
            .collect();
    }

    /// Set up linear frequency sweep
    pub fn linear_sweep(&mut self, start: Value, stop: Value, points: usize) {
        self.frequencies = (0..points)
            .map(|i| start + (stop - start) * (i as f64) / ((points - 1).max(1) as f64))
            .collect();
    }

    /// Get frequency points
    pub fn frequencies(&self) -> &[Value] {
        &self.frequencies
    }

    /// Get temperature
    pub fn temperature(&self) -> Value {
        self.temperature
    }

    /// Get config
    pub fn config(&self) -> &AnalysisConfig {
        &self.config
    }
}

impl Default for NoiseAnalysis {
    fn default() -> Self {
        Self::new(AnalysisConfig::default())
    }
}

//=============================================================================
// Noise Result
//=============================================================================

/// Noise analysis result at a single frequency
#[derive(Debug, Clone)]
pub struct NoiseResult {
    /// Frequency (Hz)
    pub frequency: Value,
    /// Total output voltage noise spectral density (V²/Hz)
    pub output_noise_density: Value,
    /// Input-referred noise spectral density (V²/Hz)
    pub input_referred_density: Value,
    /// Individual noise contributions from each source
    pub contributions: Vec<NoiseContribution>,
}

/// Contribution from a single noise source
#[derive(Debug, Clone)]
pub struct NoiseContribution {
    /// Device name
    pub device_name: String,
    /// Noise type
    pub noise_type: NoiseSourceType,
    /// Contribution to output noise (V²/Hz)
    pub output_contribution: Value,
    /// Percentage of total noise
    pub percentage: Value,
}

impl NoiseResult {
    /// Get total output noise in V/√Hz (RMS voltage noise density)
    pub fn output_noise_rms(&self) -> Value {
        self.output_noise_density.sqrt()
    }

    /// Get input-referred noise in V/√Hz
    pub fn input_referred_rms(&self) -> Value {
        self.input_referred_density.sqrt()
    }

    /// Get total output noise in dBV/Hz
    pub fn output_noise_dbv(&self) -> Value {
        if self.output_noise_density > 0.0 {
            10.0 * self.output_noise_density.log10()
        } else {
            -200.0
        }
    }

    /// Get the dominant noise source
    pub fn dominant_source(&self) -> Option<&NoiseContribution> {
        self.contributions.iter().max_by(|a, b| {
            a.output_contribution
                .partial_cmp(&b.output_contribution)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}

//=============================================================================
// Integrated Noise Calculator
//=============================================================================

/// Calculator for integrated noise over a frequency band
#[derive(Debug)]
pub struct IntegratedNoise {
    /// Results at each frequency point
    results: Vec<NoiseResult>,
}

impl IntegratedNoise {
    /// Create from a vector of noise results
    pub fn new(results: Vec<NoiseResult>) -> Self {
        Self { results }
    }

    /// Calculate total integrated output noise over the frequency band (V RMS)
    /// Uses trapezoidal integration
    pub fn total_output_noise(&self) -> Value {
        if self.results.len() < 2 {
            return 0.0;
        }

        let mut total = 0.0;
        for i in 1..self.results.len() {
            let f1 = self.results[i - 1].frequency;
            let f2 = self.results[i].frequency;
            let s1 = self.results[i - 1].output_noise_density;
            let s2 = self.results[i].output_noise_density;

            // Trapezoidal integration
            total += 0.5 * (s1 + s2) * (f2 - f1);
        }

        total.sqrt() // Return RMS voltage
    }

    /// Calculate integrated input-referred noise (V RMS)
    pub fn total_input_referred_noise(&self) -> Value {
        if self.results.len() < 2 {
            return 0.0;
        }

        let mut total = 0.0;
        for i in 1..self.results.len() {
            let f1 = self.results[i - 1].frequency;
            let f2 = self.results[i].frequency;
            let s1 = self.results[i - 1].input_referred_density;
            let s2 = self.results[i].input_referred_density;

            total += 0.5 * (s1 + s2) * (f2 - f1);
        }

        total.sqrt()
    }

    /// Get the frequency with maximum noise density
    pub fn peak_noise_frequency(&self) -> Option<Value> {
        self.results
            .iter()
            .max_by(|a, b| {
                a.output_noise_density
                    .partial_cmp(&b.output_noise_density)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|r| r.frequency)
    }

    /// Get all results
    pub fn results(&self) -> &[NoiseResult] {
        &self.results
    }
}

//=============================================================================
// Helper Functions
//=============================================================================

/// Calculate thermal noise voltage spectral density for a resistor (V²/Hz)
#[inline]
pub fn thermal_voltage_noise(resistance: Value, temperature: Value) -> Value {
    4.0 * K_BOLTZMANN * temperature * resistance
}

/// Calculate thermal noise current spectral density for a resistor (A²/Hz)
#[inline]
pub fn thermal_current_noise(resistance: Value, temperature: Value) -> Value {
    if resistance > 0.0 {
        4.0 * K_BOLTZMANN * temperature / resistance
    } else {
        0.0
    }
}

/// Calculate shot noise current spectral density (A²/Hz)
#[inline]
pub fn shot_noise(current: Value) -> Value {
    2.0 * Q_ELECTRON * current.abs()
}

/// Calculate equivalent noise bandwidth for a first-order lowpass filter
#[inline]
pub fn noise_bandwidth_first_order(f3db: Value) -> Value {
    std::f64::consts::FRAC_PI_2 * f3db
}

/// Calculate equivalent noise bandwidth for a second-order lowpass filter (Q=0.707)
#[inline]
pub fn noise_bandwidth_second_order_butterworth(f3db: Value) -> Value {
    1.11 * f3db
}

//=============================================================================
// Tests
//=============================================================================
