//! Harmonic Balance Result Types
//!
//! Defines the output data structures for HB analysis, including spectral
//! voltages, harmonic data, and power calculations.

use crate::Value;
use num_complex::Complex64;

/// Spectral voltage data for a single node
#[derive(Debug, Clone)]
pub struct SpectralVoltage {
    /// Node name/identifier
    pub node_name: String,

    /// Complex Fourier coefficients [DC, H1, H2, ..., Hn]
    /// These are one-sided (positive frequencies only)
    pub coefficients: Vec<Complex64>,

    /// Harmonic frequencies corresponding to coefficients
    pub frequencies: Vec<Value>,
}

impl SpectralVoltage {
    /// Create new spectral voltage data
    pub fn new(node_name: impl Into<String>, num_harmonics: usize) -> Self {
        Self {
            node_name: node_name.into(),
            coefficients: vec![Complex64::new(0.0, 0.0); num_harmonics + 1],
            frequencies: Vec::new(),
        }
    }

    /// Get DC component (real value)
    pub fn dc(&self) -> Value {
        self.coefficients.first().map(|c| c.re).unwrap_or(0.0)
    }

    /// Get magnitude at harmonic k
    pub fn magnitude(&self, k: usize) -> Value {
        self.coefficients.get(k).map(|c| c.norm()).unwrap_or(0.0)
    }

    /// Get phase at harmonic k (radians)
    pub fn phase(&self, k: usize) -> Value {
        self.coefficients.get(k).map(|c| c.arg()).unwrap_or(0.0)
    }

    /// Get magnitude in dBV at harmonic k
    pub fn magnitude_dbv(&self, k: usize) -> Value {
        let mag = self.magnitude(k);
        if mag > 1e-15 {
            20.0 * mag.log10()
        } else {
            -300.0 // Floor
        }
    }

    /// Get peak-to-peak voltage (approximation from harmonics)
    pub fn peak_to_peak(&self) -> Value {
        // Sum of all harmonic amplitudes * 2 (approximate)
        let ac_sum: Value = self.coefficients.iter().skip(1).map(|c| c.norm()).sum();
        2.0 * ac_sum // Peak-to-peak of AC component
    }

    /// Get RMS voltage
    pub fn rms(&self) -> Value {
        // RMS = sqrt(DC² + Σ|Hk|²/2)
        let dc_sq = self.dc().powi(2);
        let ac_power: Value = self
            .coefficients
            .iter()
            .skip(1)
            .map(|c| c.norm_sqr() / 2.0)
            .sum();
        (dc_sq + ac_power).sqrt()
    }

    /// Get total harmonic distortion (THD) as ratio
    pub fn thd(&self) -> Value {
        let h1 = self.magnitude(1);
        if h1 < 1e-15 {
            return 0.0;
        }

        let harmonic_power: Value = self.coefficients.iter().skip(2).map(|c| c.norm_sqr()).sum();

        harmonic_power.sqrt() / h1
    }

    /// Get total harmonic distortion in percent
    pub fn thd_percent(&self) -> Value {
        self.thd() * 100.0
    }
}

/// Detailed data for a single harmonic
#[derive(Debug, Clone)]
pub struct HarmonicData {
    /// Harmonic index (0 = DC, 1 = fundamental, etc.)
    pub index: usize,

    /// Frequency in Hz
    pub frequency: Value,

    /// Complex coefficient
    pub coefficient: Complex64,

    /// Magnitude
    pub magnitude: Value,

    /// Phase in radians
    pub phase: Value,

    /// Magnitude in dBV
    pub magnitude_dbv: Value,

    /// Phase in degrees
    pub phase_degrees: Value,
}

impl HarmonicData {
    /// Create from coefficient
    pub fn from_coefficient(index: usize, frequency: Value, coeff: Complex64) -> Self {
        let magnitude = coeff.norm();
        let phase = coeff.arg();
        let magnitude_dbv = if magnitude > 1e-15 {
            20.0 * magnitude.log10()
        } else {
            -300.0
        };

        Self {
            index,
            frequency,
            coefficient: coeff,
            magnitude,
            phase,
            magnitude_dbv,
            phase_degrees: phase.to_degrees(),
        }
    }
}

/// Complete result of Harmonic Balance analysis
#[derive(Debug, Clone)]
pub struct HbResult {
    /// Convergence status
    pub converged: bool,

    /// Number of Newton iterations
    pub iterations: usize,

    /// Final residual norm
    pub residual_norm: Value,

    /// Fundamental frequency
    pub fundamental_freq: Value,

    /// Spectral voltages for each node
    pub spectral_voltages: Vec<SpectralVoltage>,

    /// Node names (for indexing)
    pub node_names: Vec<String>,

    /// Number of harmonics analyzed
    pub num_harmonics: usize,

    /// Harmonic frequencies
    pub harmonic_frequencies: Vec<Value>,

    /// Analysis time in seconds
    pub solve_time_seconds: Value,

    /// Multi-tone info (if applicable)
    pub tones: Vec<String>,
}

impl HbResult {
    /// Create a new empty result
    pub fn new(fundamental_freq: Value, num_nodes: usize, num_harmonics: usize) -> Self {
        Self {
            converged: false,
            iterations: 0,
            residual_norm: f64::INFINITY,
            fundamental_freq,
            spectral_voltages: Vec::with_capacity(num_nodes),
            node_names: Vec::new(),
            num_harmonics,
            harmonic_frequencies: (0..=num_harmonics)
                .map(|k| k as f64 * fundamental_freq)
                .collect(),
            solve_time_seconds: 0.0,
            tones: Vec::new(),
        }
    }

    /// Get number of nodes
    pub fn num_nodes(&self) -> usize {
        self.spectral_voltages.len()
    }

    /// Get harmonic data for a specific node
    pub fn get_harmonics(&self, node_index: usize) -> Vec<HarmonicData> {
        if let Some(sv) = self.spectral_voltages.get(node_index) {
            sv.coefficients
                .iter()
                .enumerate()
                .map(|(k, &coeff)| {
                    let freq = self.harmonic_frequencies.get(k).copied().unwrap_or(0.0);
                    HarmonicData::from_coefficient(k, freq, coeff)
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get voltage at a node by name
    pub fn get_node_voltage(&self, name: &str) -> Option<&SpectralVoltage> {
        let idx = self.node_names.iter().position(|n| n == name)?;
        self.spectral_voltages.get(idx)
    }

    /// Get DC operating point for all nodes
    pub fn dc_operating_point(&self) -> Vec<(String, Value)> {
        self.spectral_voltages
            .iter()
            .zip(self.node_names.iter())
            .map(|(sv, name)| (name.clone(), sv.dc()))
            .collect()
    }

    /// Get fundamental amplitude for all nodes
    pub fn fundamental_amplitudes(&self) -> Vec<(String, Value)> {
        self.spectral_voltages
            .iter()
            .zip(self.node_names.iter())
            .map(|(sv, name)| (name.clone(), sv.magnitude(1)))
            .collect()
    }

    /// Get THD for all nodes
    pub fn thd_all(&self) -> Vec<(String, Value)> {
        self.spectral_voltages
            .iter()
            .zip(self.node_names.iter())
            .map(|(sv, name)| (name.clone(), sv.thd_percent()))
            .collect()
    }

    /// Check if solution is valid (converged with reasonable values)
    pub fn is_valid(&self) -> bool {
        self.converged
            && self.residual_norm.is_finite()
            && self.spectral_voltages.iter().all(|sv| {
                sv.coefficients
                    .iter()
                    .all(|c| c.re.is_finite() && c.im.is_finite())
            })
    }
}

#[cfg(test)]
mod result_tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_spectral_voltage_dc() {
        let mut sv = SpectralVoltage::new("out", 5);
        sv.coefficients[0] = Complex64::new(1.5, 0.0);
        assert!((sv.dc() - 1.5).abs() < 1e-10);
    }

    #[test]
    fn test_spectral_voltage_magnitude() {
        let mut sv = SpectralVoltage::new("out", 5);
        sv.coefficients[1] = Complex64::new(3.0, 4.0); // |3+4j| = 5
        assert!((sv.magnitude(1) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_spectral_voltage_phase() {
        let mut sv = SpectralVoltage::new("out", 5);
        sv.coefficients[1] = Complex64::new(0.0, 1.0); // 90 degrees
        assert!((sv.phase(1) - PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_spectral_voltage_rms() {
        let mut sv = SpectralVoltage::new("out", 5);
        sv.coefficients[0] = Complex64::new(1.0, 0.0); // DC = 1
        sv.coefficients[1] = Complex64::new(1.0, 0.0); // H1 = 1
        // RMS = sqrt(1² + 1²/2) = sqrt(1.5) ≈ 1.225
        let expected_rms = (1.0 + 0.5_f64).sqrt();
        assert!((sv.rms() - expected_rms).abs() < 0.01);
    }

    #[test]
    fn test_spectral_voltage_thd() {
        let mut sv = SpectralVoltage::new("out", 5);
        sv.coefficients[1] = Complex64::new(1.0, 0.0); // H1 = 1
        sv.coefficients[2] = Complex64::new(0.1, 0.0); // H2 = 0.1
        sv.coefficients[3] = Complex64::new(0.05, 0.0); // H3 = 0.05

        // THD = sqrt(0.1² + 0.05²) / 1.0 ≈ 0.1118
        let expected_thd = (0.01 + 0.0025_f64).sqrt();
        assert!((sv.thd() - expected_thd).abs() < 0.001);
    }

    #[test]
    fn test_harmonic_data_from_coefficient() {
        let coeff = Complex64::new(1.0, 1.0); // |1+j| = sqrt(2), arg = 45°
        let hd = HarmonicData::from_coefficient(1, 1e9, coeff);

        assert_eq!(hd.index, 1);
        assert_eq!(hd.frequency, 1e9);
        assert!((hd.magnitude - 2.0_f64.sqrt()).abs() < 1e-10);
        assert!((hd.phase_degrees - 45.0).abs() < 0.01);
    }

    #[test]
    fn test_hb_result_creation() {
        let result = HbResult::new(1e9, 5, 9);
        assert_eq!(result.fundamental_freq, 1e9);
        assert_eq!(result.num_harmonics, 9);
        assert_eq!(result.harmonic_frequencies.len(), 10);
        assert!(!result.converged);
    }

    #[test]
    fn test_hb_result_dc_operating_point() {
        let mut result = HbResult::new(1e9, 2, 3);
        result.node_names = vec!["in".to_string(), "out".to_string()];

        let mut sv1 = SpectralVoltage::new("in", 3);
        sv1.coefficients[0] = Complex64::new(1.0, 0.0);

        let mut sv2 = SpectralVoltage::new("out", 3);
        sv2.coefficients[0] = Complex64::new(0.5, 0.0);

        result.spectral_voltages = vec![sv1, sv2];

        let dc = result.dc_operating_point();
        assert_eq!(dc.len(), 2);
        assert_eq!(dc[0].0, "in");
        assert!((dc[0].1 - 1.0).abs() < 1e-10);
        assert!((dc[1].1 - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_hb_result_validity() {
        let mut result = HbResult::new(1e9, 1, 3);
        result.converged = true;
        result.residual_norm = 1e-10;

        let mut sv = SpectralVoltage::new("out", 3);
        sv.coefficients = vec![Complex64::new(1.0, 0.0); 4];
        result.spectral_voltages = vec![sv];

        assert!(result.is_valid());

        // Test invalid case
        result.converged = false;
        assert!(!result.is_valid());
    }

    #[test]
    fn test_magnitude_dbv() {
        let mut sv = SpectralVoltage::new("out", 3);
        sv.coefficients[1] = Complex64::new(1.0, 0.0); // 1V = 0 dBV
        assert!((sv.magnitude_dbv(1) - 0.0).abs() < 0.01);

        sv.coefficients[2] = Complex64::new(0.1, 0.0); // 0.1V = -20 dBV
        assert!((sv.magnitude_dbv(2) - (-20.0)).abs() < 0.01);
    }
}
