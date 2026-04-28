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

