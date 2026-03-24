//! Noise Analysis Module
//!
//! Computes noise spectral density as a function of frequency.

#![allow(clippy::needless_range_loop)]
//! Supports:
//! - **Thermal noise**: Johnson-Nyquist noise from resistors (4kTR)
//! - **Shot noise**: From PN junctions in diodes and BJTs (2qI)
//! - **Flicker noise**: 1/f noise in semiconductors (KF/f^AF)
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
        Self {
            device_name,
            noise_type: NoiseSourceType::Flicker,
            node_pos,
            node_neg,
            parameter: kf,
            af,
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
                // Flicker noise: Si = KF * I^AF / f (A²/Hz)
                if frequency > 0.0 {
                    self.parameter * self.current.abs().powf(self.af) / frequency
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermal_noise_resistor() {
        // 1kΩ resistor at 300K should have ~4.14e-17 V²/Hz
        let noise = thermal_voltage_noise(1000.0, 300.0);
        let expected = 4.0 * K_BOLTZMANN * 300.0 * 1000.0;
        assert!((noise - expected).abs() < 1e-25);
        assert!((noise - 1.66e-17).abs() < 1e-18);
    }

    #[test]
    fn test_shot_noise_diode() {
        // 1mA current should have shot noise of ~3.2e-22 A²/Hz
        let noise = shot_noise(1e-3);
        let expected = 2.0 * Q_ELECTRON * 1e-3;
        assert!((noise - expected).abs() < 1e-30);
    }

    #[test]
    fn test_noise_source_thermal() {
        let source = NoiseSource::thermal("R1".to_string(), 1, 0, 1000.0);
        let density = source.spectral_density(1000.0, 300.0);

        // Should be 4kT/R = 4 * 1.38e-23 * 300 / 1000 ≈ 1.66e-23 A²/Hz
        assert!((density - 1.66e-23).abs() < 1e-24);
    }

    #[test]
    fn test_noise_source_shot() {
        let source = NoiseSource::shot("D1".to_string(), 1, 0, 1e-3);
        let density = source.spectral_density(1000.0, 300.0);

        // Should be 2qI = 2 * 1.6e-19 * 1e-3 ≈ 3.2e-22 A²/Hz
        assert!((density - 3.2e-22).abs() < 1e-23);
    }

    #[test]
    fn test_noise_source_flicker() {
        let source = NoiseSource::flicker("Q1".to_string(), 1, 0, 1e-15, 1.0, 1e-3);

        // At 100Hz with KF=1e-15, AF=1, I=1mA:
        // Si = 1e-15 * 1e-3 / 100 = 1e-20 A²/Hz
        let density = source.spectral_density(100.0, 300.0);
        assert!((density - 1e-20).abs() < 1e-22);

        // At 1000Hz, should be 10x lower
        let density_1k = source.spectral_density(1000.0, 300.0);
        assert!((density / density_1k - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_decade_sweep() {
        let mut noise = NoiseAnalysis::default();
        noise.decade_sweep(1.0, 1e6, 10);

        let freqs = noise.frequencies();
        assert!(freqs[0] >= 1.0 - 1e-10);
        assert!(*freqs.last().unwrap() <= 1e6 + 1.0);
    }

    #[test]
    fn test_noise_result_rms() {
        let result = NoiseResult {
            frequency: 1000.0,
            output_noise_density: 1e-18, // V²/Hz
            input_referred_density: 1e-18,
            contributions: vec![],
        };

        // RMS should be sqrt of density
        assert!((result.output_noise_rms() - 1e-9).abs() < 1e-15);
    }

    #[test]
    fn test_integrated_noise() {
        // Create white noise results (constant spectral density)
        let results: Vec<NoiseResult> = (0..100)
            .map(|i| NoiseResult {
                frequency: (i + 1) as f64 * 100.0, // 100Hz to 10kHz
                output_noise_density: 1e-18,       // Constant V²/Hz
                input_referred_density: 1e-18,
                contributions: vec![],
            })
            .collect();

        let integrated = IntegratedNoise::new(results);
        let total = integrated.total_output_noise();

        // Total should be sqrt(density * bandwidth) = sqrt(1e-18 * 9900) ≈ 3.15e-8 V
        let expected = (1e-18_f64 * (10000.0 - 100.0)).sqrt();
        assert!((total - expected).abs() / expected < 0.02); // 2% tolerance
    }

    #[test]
    fn test_noise_analysis_temperature() {
        let mut noise = NoiseAnalysis::default();
        assert_eq!(noise.temperature(), 300.0);

        noise.set_temperature(400.0);
        assert_eq!(noise.temperature(), 400.0);
    }

    // =========================================================================
    // Commercial-Grade Comprehensive Tests
    // =========================================================================

    /// Test burst (popcorn) noise with Lorentzian spectrum.
    /// Validates the correct frequency rolloff behavior at the corner frequency.
    #[test]
    fn test_noise_source_burst_lorentzian() {
        // Create burst noise source: KB=1e-18, AB=2, I=1mA, FB=1kHz
        let source = NoiseSource::burst("Q1".to_string(), 1, 0, 1e-18, 2.0, 1e-3, 1000.0);

        // At DC (low frequency << FB), noise should be KB * I^AB = 1e-18 * (1e-3)^2 = 1e-24 A²/Hz
        let density_dc = source.spectral_density(1.0, 300.0);
        let expected_dc = 1e-18 * (1e-3_f64).powf(2.0) / (1.0_f64 + (1.0_f64 / 1000.0_f64).powi(2));
        assert!(
            (density_dc - expected_dc).abs() / expected_dc < 0.001,
            "DC burst noise mismatch: {} vs {}",
            density_dc,
            expected_dc
        );

        // At corner frequency FB=1kHz: should be half of DC value (3dB down)
        let density_corner = source.spectral_density(1000.0, 300.0);
        let expected_corner = 1e-18 * (1e-3_f64).powf(2.0) / 2.0; // 1/(1+1) = 0.5
        assert!(
            (density_corner - expected_corner).abs() / expected_corner < 0.001,
            "Corner frequency burst noise mismatch"
        );

        // At f >> FB (e.g., 10kHz): should roll off as 1/f²
        let density_high = source.spectral_density(10000.0, 300.0);
        let f_ratio_high: f64 = 10000.0 / 1000.0;
        let expected_high = 1e-18 * (1e-3_f64).powf(2.0) / (1.0_f64 + f_ratio_high.powi(2));
        assert!(
            (density_high - expected_high).abs() / expected_high < 0.001,
            "High frequency burst noise mismatch"
        );

        // Verify rolloff: ratio of DC to high freq should be ~(1 + f²/FB²)
        let rolloff_ratio = density_dc / density_high;
        let expected_ratio =
            (1.0_f64 + f_ratio_high.powi(2)) / (1.0_f64 + (1.0_f64 / 1000.0_f64).powi(2));
        assert!(
            (rolloff_ratio - expected_ratio).abs() / expected_ratio < 0.01,
            "Burst noise rolloff ratio mismatch"
        );
    }

    /// Test thermal noise temperature scaling (Johnson-Nyquist).
    /// Noise should scale linearly with absolute temperature.
    #[test]
    fn test_thermal_noise_temperature_scaling() {
        let source = NoiseSource::thermal("R1".to_string(), 1, 0, 1000.0);

        let density_300k = source.spectral_density(1000.0, 300.0);
        let density_600k = source.spectral_density(1000.0, 600.0);

        // At double temperature, thermal noise should exactly double
        let ratio = density_600k / density_300k;
        assert!(
            (ratio - 2.0).abs() < 1e-10,
            "Thermal noise should scale linearly with T: ratio = {}",
            ratio
        );

        // Test at cryogenic temperature (77K liquid nitrogen)
        let density_77k = source.spectral_density(1000.0, 77.0);
        let ratio_cryo = density_77k / density_300k;
        let expected_cryo = 77.0 / 300.0;
        assert!(
            (ratio_cryo - expected_cryo).abs() < 1e-10,
            "Cryogenic thermal noise scaling incorrect"
        );
    }

    /// Test shot noise is frequency-independent (white noise).
    #[test]
    fn test_shot_noise_is_white() {
        let source = NoiseSource::shot("D1".to_string(), 1, 0, 1e-3);

        let density_1hz = source.spectral_density(1.0, 300.0);
        let density_1khz = source.spectral_density(1000.0, 300.0);
        let density_1mhz = source.spectral_density(1e6, 300.0);
        let density_1ghz = source.spectral_density(1e9, 300.0);

        // All should be identical (white noise)
        assert!(
            (density_1hz - density_1khz).abs() < 1e-30,
            "Shot noise should be frequency-independent"
        );
        assert!(
            (density_1khz - density_1mhz).abs() < 1e-30,
            "Shot noise should be frequency-independent"
        );
        assert!(
            (density_1mhz - density_1ghz).abs() < 1e-30,
            "Shot noise should be frequency-independent"
        );
    }

    /// Test flicker noise at very low frequencies approaches infinity correctly.
    #[test]
    fn test_flicker_noise_low_frequency_behavior() {
        let source = NoiseSource::flicker("Q1".to_string(), 1, 0, 1e-15, 1.0, 1e-3);

        // At f -> 0, flicker noise -> infinity (handled by returning 0 to avoid div by zero)
        let density_zero = source.spectral_density(0.0, 300.0);
        assert_eq!(density_zero, 0.0, "Should handle f=0 gracefully");

        // At very low frequency, noise should be very large
        let density_0p01hz = source.spectral_density(0.01, 300.0);
        let density_1hz = source.spectral_density(1.0, 300.0);
        let ratio = density_0p01hz / density_1hz;
        assert!(
            (ratio - 100.0).abs() < 0.01,
            "1/f noise should scale as 1/f: ratio at 0.01Hz vs 1Hz = {}",
            ratio
        );
    }

    /// Test flicker noise exponent (AF) affects current dependence correctly.
    #[test]
    fn test_flicker_noise_exponent_af() {
        // AF = 1: noise ∝ I
        let source_af1 = NoiseSource::flicker("Q1".to_string(), 1, 0, 1e-15, 1.0, 1e-3);
        let source_af1_2x = NoiseSource::flicker("Q1".to_string(), 1, 0, 1e-15, 1.0, 2e-3);
        let ratio_af1 = source_af1_2x.spectral_density(100.0, 300.0)
            / source_af1.spectral_density(100.0, 300.0);
        assert!(
            (ratio_af1 - 2.0).abs() < 0.001,
            "AF=1 should give linear current dependence"
        );

        // AF = 2: noise ∝ I²
        let source_af2 = NoiseSource::flicker("Q1".to_string(), 1, 0, 1e-15, 2.0, 1e-3);
        let source_af2_2x = NoiseSource::flicker("Q1".to_string(), 1, 0, 1e-15, 2.0, 2e-3);
        let ratio_af2 = source_af2_2x.spectral_density(100.0, 300.0)
            / source_af2.spectral_density(100.0, 300.0);
        assert!(
            (ratio_af2 - 4.0).abs() < 0.001,
            "AF=2 should give quadratic current dependence"
        );
    }

    /// Test zero/negative resistance handling in thermal noise.
    #[test]
    fn test_thermal_noise_edge_cases() {
        // Zero resistance should give infinite noise (but we handle with 0)
        let source_zero = NoiseSource::thermal("R1".to_string(), 1, 0, 0.0);
        let density = source_zero.spectral_density(1000.0, 300.0);
        assert_eq!(density, 0.0, "Zero resistance should return 0 noise");

        // Very small resistance
        let source_small = NoiseSource::thermal("R1".to_string(), 1, 0, 1e-6);
        let density_small = source_small.spectral_density(1000.0, 300.0);
        let expected = 4.0 * K_BOLTZMANN * 300.0 / 1e-6;
        assert!(
            (density_small - expected).abs() / expected < 1e-10,
            "Small resistance thermal noise incorrect"
        );
    }

    /// Test shot noise with negative current (handled via abs()).
    #[test]
    fn test_shot_noise_negative_current() {
        let source_pos = NoiseSource::shot("D1".to_string(), 1, 0, 1e-3);
        let source_neg = NoiseSource::shot("D1".to_string(), 1, 0, -1e-3);

        let density_pos = source_pos.spectral_density(1000.0, 300.0);
        let density_neg = source_neg.spectral_density(1000.0, 300.0);

        assert!(
            (density_pos - density_neg).abs() < 1e-30,
            "Shot noise should be same magnitude for ± current"
        );
    }

    /// Test noise bandwidth formulas for standard filter responses.
    #[test]
    fn test_noise_bandwidth_formulas() {
        // First-order lowpass: NBW = π/2 * f3dB
        let f3db = 1000.0;
        let nbw_first = noise_bandwidth_first_order(f3db);
        let expected_first = std::f64::consts::FRAC_PI_2 * f3db;
        assert!(
            (nbw_first - expected_first).abs() < 1e-10,
            "First-order NBW formula incorrect"
        );

        // Second-order Butterworth: NBW ≈ 1.11 * f3dB
        let nbw_second = noise_bandwidth_second_order_butterworth(f3db);
        assert!(
            (nbw_second / f3db - 1.11).abs() < 0.001,
            "Second-order Butterworth NBW formula incorrect"
        );

        // NBW should scale linearly with f3dB
        let nbw_2x = noise_bandwidth_first_order(2.0 * f3db);
        assert!(
            (nbw_2x / nbw_first - 2.0).abs() < 1e-10,
            "NBW should scale linearly with f3dB"
        );
    }

    /// Test NoiseResult dominant source finding.
    #[test]
    fn test_noise_result_dominant_source() {
        let result = NoiseResult {
            frequency: 1000.0,
            output_noise_density: 1e-18,
            input_referred_density: 1e-18,
            contributions: vec![
                NoiseContribution {
                    device_name: "R1".to_string(),
                    noise_type: NoiseSourceType::Thermal,
                    output_contribution: 0.3e-18,
                    percentage: 30.0,
                },
                NoiseContribution {
                    device_name: "Q1_base".to_string(),
                    noise_type: NoiseSourceType::Shot,
                    output_contribution: 0.5e-18,
                    percentage: 50.0,
                },
                NoiseContribution {
                    device_name: "Q1_flicker".to_string(),
                    noise_type: NoiseSourceType::Flicker,
                    output_contribution: 0.2e-18,
                    percentage: 20.0,
                },
            ],
        };

        let dominant = result.dominant_source().expect("Should find dominant");
        assert_eq!(dominant.device_name, "Q1_base");
        assert_eq!(dominant.noise_type, NoiseSourceType::Shot);
        assert!((dominant.percentage - 50.0).abs() < 0.001);
    }

    /// Test integrated noise with 1/f spectrum (logarithmic integration).
    #[test]
    fn test_integrated_1f_noise() {
        // For 1/f noise, integrated power = KF * I^AF * ln(f_high/f_low)
        let kf: f64 = 1e-15;
        let af: f64 = 1.0;
        let current: f64 = 1e-3;
        let f_low: f64 = 1.0;
        let f_high: f64 = 10000.0;

        // Generate log-spaced frequency points for 1/f noise
        let num_points = 1000;
        let results: Vec<NoiseResult> = (0..num_points)
            .map(|i| {
                let log_f = (f_low.ln())
                    + (f_high.ln() - f_low.ln()) * (i as f64) / ((num_points - 1) as f64);
                let f = log_f.exp();
                // Flicker noise spectral density
                let s_i = kf * current.powf(af) / f;
                NoiseResult {
                    frequency: f,
                    output_noise_density: s_i, // Assuming unity transfer function
                    input_referred_density: s_i,
                    contributions: vec![],
                }
            })
            .collect();

        let integrated = IntegratedNoise::new(results);
        let total = integrated.total_output_noise();

        // Analytical result for 1/f noise: sqrt(KF * I^AF * ln(f_high/f_low))
        // But trapezoidal integration of 1/f is ∫(1/f)df which is ln(f_high/f_low)
        // With density = KF * I^AF / f, integrated power = KF * I^AF * ln(f_high/f_low)
        let expected_power = kf * current.powf(af) * (f_high / f_low).ln();
        let expected_rms = expected_power.sqrt();

        // Allow 5% tolerance due to numerical integration
        assert!(
            (total - expected_rms).abs() / expected_rms < 0.05,
            "Integrated 1/f noise mismatch: got {} expected {}",
            total,
            expected_rms
        );
    }

    /// Test linear frequency sweep generation.
    #[test]
    fn test_linear_sweep() {
        let mut noise = NoiseAnalysis::default();
        noise.linear_sweep(100.0, 10000.0, 100);

        let freqs = noise.frequencies();
        assert_eq!(freqs.len(), 100);
        assert!((freqs[0] - 100.0).abs() < 1e-10);
        assert!((freqs[99] - 10000.0).abs() < 1e-10);

        // Check uniform spacing
        let step = (10000.0 - 100.0) / 99.0;
        for i in 1..100 {
            let expected = 100.0 + step * (i as f64);
            assert!(
                (freqs[i] - expected).abs() < 1e-6,
                "Linear sweep not uniform at index {}",
                i
            );
        }
    }

    /// Test output noise in dBV/Hz conversion.
    #[test]
    fn test_noise_result_dbv() {
        // 1e-18 V²/Hz = -180 dBV²/Hz = 10*log10(1e-18) = -180
        let result = NoiseResult {
            frequency: 1000.0,
            output_noise_density: 1e-18,
            input_referred_density: 1e-18,
            contributions: vec![],
        };

        let dbv = result.output_noise_dbv();
        assert!(
            (dbv - (-180.0)).abs() < 0.01,
            "dBV conversion incorrect: {}",
            dbv
        );

        // Test very small value
        let result_small = NoiseResult {
            frequency: 1000.0,
            output_noise_density: 1e-30,
            input_referred_density: 1e-30,
            contributions: vec![],
        };
        let dbv_small = result_small.output_noise_dbv();
        assert!(
            (dbv_small - (-300.0)).abs() < 0.01,
            "Small value dBV incorrect"
        );

        // Test zero (should return -200 floor)
        let result_zero = NoiseResult {
            frequency: 1000.0,
            output_noise_density: 0.0,
            input_referred_density: 0.0,
            contributions: vec![],
        };
        let dbv_zero = result_zero.output_noise_dbv();
        assert_eq!(dbv_zero, -200.0, "Zero should return floor value");
    }

    /// Test peak noise frequency detection in IntegratedNoise.
    #[test]
    fn test_peak_noise_frequency_detection() {
        // Create noise with peak at 1kHz
        let results: Vec<NoiseResult> = vec![
            NoiseResult {
                frequency: 100.0,
                output_noise_density: 1e-18,
                input_referred_density: 1e-18,
                contributions: vec![],
            },
            NoiseResult {
                frequency: 1000.0,
                output_noise_density: 5e-18, // Peak here
                input_referred_density: 5e-18,
                contributions: vec![],
            },
            NoiseResult {
                frequency: 10000.0,
                output_noise_density: 1e-19,
                input_referred_density: 1e-19,
                contributions: vec![],
            },
        ];

        let integrated = IntegratedNoise::new(results);
        let peak_freq = integrated.peak_noise_frequency().expect("Should find peak");
        assert!(
            (peak_freq - 1000.0).abs() < 1e-10,
            "Peak frequency detection failed"
        );
    }

    /// Test physical constants are correct (IEEE/CODATA values).
    #[test]
    fn test_physical_constants_accuracy() {
        // Boltzmann constant: 1.380649×10⁻²³ J/K (exact, SI definition since 2019)
        assert!(
            (K_BOLTZMANN - 1.380649e-23).abs() < 1e-30,
            "Boltzmann constant mismatch"
        );

        // Elementary charge: 1.602176634×10⁻¹⁹ C (exact, SI definition since 2019)
        assert!(
            (Q_ELECTRON - 1.602176634e-19).abs() < 1e-30,
            "Electron charge mismatch"
        );

        // Nominal temperature: 300K (27°C)
        assert_eq!(T_NOMINAL, 300.0);
    }
}
