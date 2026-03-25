//! Periodic Transfer Function (PXF) Analysis Module
//!
//! PXF analysis computes the frequency-domain transfer function from a small-signal
//! input to circuit outputs around a periodic steady-state operating point. This is
//! essential for:
//!
//! - **Mixer noise figure**: Transfer of noise sources to output sidebands
//! - **LNA gain compression**: Small-signal gain vs. large-signal interference
//! - **Frequency converter sensitivity**: Input-to-output sideband mapping
//! - **Oscillator injection locking**: Response to external perturbations
//!
//! # Theory
//!
//! PXF analysis extends standard AC transfer function analysis to time-varying
//! systems. For a periodic operating point with period T, the transfer function
//! H(s, t) is also periodic in t. Using Floquet/LPTV theory:
//!
//! H(s, t) = Σₖ Hₖ(s) · exp(jk·ω₀·t)
//!
//! where ω₀ = 2π/T is the fundamental frequency and Hₖ(s) are the harmonic
//! transfer functions (conversion gains).
//!
//! # Relationship to PAC
//!
//! PXF and PAC are complementary:
//! - **PAC**: Sweeps input frequency, observes response at all sidebands
//! - **PXF**: Fixes sideband relationship, sweeps both input/output frequency
//!
//! PXF provides the complete transfer function matrix H(f_in, f_out) for any
//! frequency pair related by the LO frequency.

use crate::Value;
use num_complex::Complex64;
use std::f64::consts::PI;

//=============================================================================
// PXF Configuration
//=============================================================================

/// Configuration for Periodic Transfer Function (PXF) analysis
#[derive(Debug, Clone)]
pub struct PxfConfig {
    /// Start frequency for sweep (Hz)
    pub freq_start: Value,

    /// Stop frequency for sweep (Hz)
    pub freq_stop: Value,

    /// Number of frequency points
    pub num_points: usize,

    /// Sweep type
    pub sweep_type: PxfSweepType,

    /// Input sideband index (relative to LO)
    pub input_sideband: i32,

    /// Output sideband index (relative to LO)
    pub output_sideband: i32,

    /// Maximum number of sidebands to include in computation
    pub max_sidebands: usize,

    /// Input source name
    pub input_source: Option<String>,

    /// Output node name
    pub output_node: Option<String>,

    /// Reference node (usually ground)
    pub ref_node: String,

    /// Fundamental (LO) frequency from PSS
    pub fundamental_freq: Value,

    /// Include noise transfer (for noise figure computation)
    pub include_noise: bool,
}

/// Sweep type for PXF frequency sweep
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PxfSweepType {
    /// Linear frequency sweep
    Linear,
    /// Decade (logarithmic) sweep
    #[default]
    Decade,
    /// Octave sweep
    Octave,
}

impl Default for PxfConfig {
    fn default() -> Self {
        Self {
            freq_start: 1e3,
            freq_stop: 1e9,
            num_points: 50,
            sweep_type: PxfSweepType::Decade,
            input_sideband: 1,  // Default: RF input (LO + IF)
            output_sideband: 0, // Default: IF output (baseband)
            max_sidebands: 5,
            input_source: None,
            output_node: None,
            ref_node: "0".to_string(),
            fundamental_freq: 0.0,
            include_noise: false,
        }
    }
}

impl PxfConfig {
    /// Create new PXF configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set frequency sweep range
    pub fn with_sweep(mut self, start: Value, stop: Value, points: usize) -> Self {
        self.freq_start = start;
        self.freq_stop = stop;
        self.num_points = points;
        self
    }

    /// Set sweep type
    pub fn with_sweep_type(mut self, sweep_type: PxfSweepType) -> Self {
        self.sweep_type = sweep_type;
        self
    }

    /// Set input/output sideband pair
    pub fn with_sidebands(mut self, input: i32, output: i32) -> Self {
        self.input_sideband = input;
        self.output_sideband = output;
        self
    }

    /// Set input source
    pub fn with_input(mut self, source: &str) -> Self {
        self.input_source = Some(source.to_uppercase());
        self
    }

    /// Set output node
    pub fn with_output(mut self, node: &str) -> Self {
        self.output_node = Some(node.to_uppercase());
        self
    }

    /// Set fundamental frequency
    pub fn with_fundamental(mut self, freq: Value) -> Self {
        self.fundamental_freq = freq;
        self
    }

    /// Enable noise transfer computation
    pub fn with_noise(mut self, enable: bool) -> Self {
        self.include_noise = enable;
        self
    }

    /// Generate frequency points based on sweep type
    pub fn frequency_points(&self) -> Vec<Value> {
        match self.sweep_type {
            PxfSweepType::Linear => self.linear_points(),
            PxfSweepType::Decade => self.decade_points(),
            PxfSweepType::Octave => self.octave_points(),
        }
    }

    fn linear_points(&self) -> Vec<Value> {
        if self.num_points <= 1 {
            return vec![self.freq_start];
        }
        let step = (self.freq_stop - self.freq_start) / (self.num_points - 1) as Value;
        (0..self.num_points)
            .map(|i| self.freq_start + i as Value * step)
            .collect()
    }

    fn decade_points(&self) -> Vec<Value> {
        if self.freq_start <= 0.0 || self.freq_stop <= 0.0 {
            return vec![self.freq_start.max(1e-15)];
        }
        let log_start = self.freq_start.log10();
        let log_stop = self.freq_stop.log10();
        let num_decades = log_stop - log_start;
        let total_points = (num_decades * self.num_points as f64).ceil() as usize;
        let total_points = total_points.max(1);

        (0..total_points)
            .map(|i| {
                let log_f = log_start
                    + (log_stop - log_start) * i as f64 / (total_points - 1).max(1) as f64;
                10.0_f64.powf(log_f)
            })
            .collect()
    }

    fn octave_points(&self) -> Vec<Value> {
        if self.freq_start <= 0.0 || self.freq_stop <= 0.0 {
            return vec![self.freq_start.max(1e-15)];
        }
        let log2_start = self.freq_start.log2();
        let log2_stop = self.freq_stop.log2();
        let num_octaves = log2_stop - log2_start;
        let total_points = (num_octaves * self.num_points as f64).ceil() as usize;
        let total_points = total_points.max(1);

        (0..total_points)
            .map(|i| {
                let log2_f = log2_start
                    + (log2_stop - log2_start) * i as f64 / (total_points - 1).max(1) as f64;
                2.0_f64.powf(log2_f)
            })
            .collect()
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), PxfError> {
        if self.freq_start <= 0.0 {
            return Err(PxfError::InvalidFrequency(
                "Start frequency must be positive".into(),
            ));
        }
        if self.freq_stop < self.freq_start {
            return Err(PxfError::InvalidFrequency(
                "Stop frequency must be >= start".into(),
            ));
        }
        if self.num_points == 0 {
            return Err(PxfError::InvalidConfiguration(
                "Must have at least one point".into(),
            ));
        }
        Ok(())
    }
}

//=============================================================================
// PXF Error
//=============================================================================

/// Errors during PXF analysis
#[derive(Debug, Clone)]
pub enum PxfError {
    /// Invalid frequency specification
    InvalidFrequency(String),
    /// Invalid configuration
    InvalidConfiguration(String),
    /// Missing PSS solution
    MissingPssSolution(String),
    /// Computation error
    ComputationError(String),
}

impl std::fmt::Display for PxfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PxfError::InvalidFrequency(s) => write!(f, "Invalid frequency: {}", s),
            PxfError::InvalidConfiguration(s) => write!(f, "Invalid configuration: {}", s),
            PxfError::MissingPssSolution(s) => write!(f, "Missing PSS solution: {}", s),
            PxfError::ComputationError(s) => write!(f, "Computation error: {}", s),
        }
    }
}

impl std::error::Error for PxfError {}

//=============================================================================
// Transfer Function Point
//=============================================================================

/// A single transfer function point in PXF analysis
#[derive(Debug, Clone)]
pub struct TransferPoint {
    /// Input frequency (at input sideband)
    pub freq_in: Value,

    /// Output frequency (at output sideband)
    pub freq_out: Value,

    /// Complex transfer function H(f_in → f_out)
    pub transfer: Complex64,

    /// Input sideband index
    pub sideband_in: i32,

    /// Output sideband index
    pub sideband_out: i32,
}

impl TransferPoint {
    /// Get magnitude in linear scale
    pub fn magnitude(&self) -> Value {
        self.transfer.norm()
    }

    /// Get magnitude in dB
    pub fn magnitude_db(&self) -> Value {
        20.0 * self.transfer.norm().log10()
    }

    /// Get phase in radians
    pub fn phase(&self) -> Value {
        self.transfer.arg()
    }

    /// Get phase in degrees
    pub fn phase_degrees(&self) -> Value {
        self.transfer.arg() * 180.0 / PI
    }

    /// Get group delay at this point (requires adjacent points)
    pub fn group_delay(&self, next: &TransferPoint) -> Value {
        let df = next.freq_in - self.freq_in;
        if df.abs() < 1e-15 {
            return 0.0;
        }
        let dphi = next.phase() - self.phase();
        // Unwrap phase if needed
        let dphi_unwrapped = if dphi > PI {
            dphi - 2.0 * PI
        } else if dphi < -PI {
            dphi + 2.0 * PI
        } else {
            dphi
        };
        -dphi_unwrapped / (2.0 * PI * df)
    }
}

//=============================================================================
// PXF Result
//=============================================================================

/// Result of PXF analysis
#[derive(Debug, Clone)]
pub struct PxfResult {
    /// Fundamental (LO) frequency
    pub fundamental_freq: Value,

    /// Input sideband index
    pub input_sideband: i32,

    /// Output sideband index
    pub output_sideband: i32,

    /// Transfer function points
    pub points: Vec<TransferPoint>,

    /// Node names
    pub node_names: Vec<String>,

    /// DC gain (transfer at f = 0)
    pub dc_gain: Option<Complex64>,

    /// Peak gain and frequency
    pub peak_gain: Option<(Value, Value)>, // (frequency, gain_db)

    /// 3dB bandwidth (if applicable)
    pub bandwidth_3db: Option<Value>,

    /// Unity gain frequency
    pub unity_gain_freq: Option<Value>,
}

impl PxfResult {
    /// Create new PXF result
    pub fn new(fundamental_freq: Value, input_sb: i32, output_sb: i32) -> Self {
        Self {
            fundamental_freq,
            input_sideband: input_sb,
            output_sideband: output_sb,
            points: Vec::new(),
            node_names: Vec::new(),
            dc_gain: None,
            peak_gain: None,
            bandwidth_3db: None,
            unity_gain_freq: None,
        }
    }

    /// Add a transfer point
    pub fn add_point(&mut self, point: TransferPoint) {
        self.points.push(point);
    }

    /// Get number of frequency points
    pub fn num_points(&self) -> usize {
        self.points.len()
    }

    /// Get input frequencies
    pub fn input_frequencies(&self) -> Vec<Value> {
        self.points.iter().map(|p| p.freq_in).collect()
    }

    /// Get magnitude curve (frequency, magnitude_db)
    pub fn magnitude_curve(&self) -> Vec<(Value, Value)> {
        self.points
            .iter()
            .map(|p| (p.freq_in, p.magnitude_db()))
            .collect()
    }

    /// Get phase curve (frequency, phase_degrees)
    pub fn phase_curve(&self) -> Vec<(Value, Value)> {
        self.points
            .iter()
            .map(|p| (p.freq_in, p.phase_degrees()))
            .collect()
    }

    /// Get group delay curve (frequency, delay in seconds)
    pub fn group_delay_curve(&self) -> Vec<(Value, Value)> {
        if self.points.len() < 2 {
            return Vec::new();
        }

        self.points
            .windows(2)
            .map(|w| {
                let gd = w[0].group_delay(&w[1]);
                ((w[0].freq_in + w[1].freq_in) / 2.0, gd)
            })
            .collect()
    }

    /// Find peak gain
    pub fn find_peak_gain(&self) -> Option<(Value, Value)> {
        self.points
            .iter()
            .map(|p| (p.freq_in, p.magnitude_db()))
            .filter(|(_, db)| db.is_finite())
            .max_by(|a, b| a.1.total_cmp(&b.1))
    }

    /// Find 3dB bandwidth below peak
    pub fn find_bandwidth_3db(&self) -> Option<Value> {
        let (peak_freq, peak_db) = self.find_peak_gain()?;
        let threshold = peak_db - 3.0;

        // Find lower -3dB point
        let lower = self
            .points
            .iter()
            .filter(|p| p.freq_in < peak_freq)
            .filter(|p| p.magnitude_db() <= threshold)
            .map(|p| p.freq_in)
            .next_back();

        // Find upper -3dB point
        let upper = self
            .points
            .iter()
            .filter(|p| p.freq_in > peak_freq)
            .filter(|p| p.magnitude_db() <= threshold)
            .map(|p| p.freq_in)
            .next();

        match (lower, upper) {
            (Some(l), Some(u)) => Some(u - l),
            (None, Some(u)) => Some(u - self.points.first()?.freq_in),
            (Some(l), None) => Some(self.points.last()?.freq_in - l),
            _ => None,
        }
    }

    /// Find unity gain frequency (0 dB crossing)
    pub fn find_unity_gain_freq(&self) -> Option<Value> {
        if self.points.len() < 2 {
            return None;
        }

        for window in self.points.windows(2) {
            let db0 = window[0].magnitude_db();
            let db1 = window[1].magnitude_db();

            // Look for 0dB crossing
            if (db0 >= 0.0 && db1 < 0.0) || (db0 < 0.0 && db1 >= 0.0) {
                // Linear interpolation
                let f0 = window[0].freq_in;
                let f1 = window[1].freq_in;
                let alpha = (0.0 - db0) / (db1 - db0);
                return Some(f0 + alpha * (f1 - f0));
            }
        }

        None
    }

    /// Compute derived metrics
    pub fn compute_metrics(&mut self) {
        self.peak_gain = self.find_peak_gain();
        self.bandwidth_3db = self.find_bandwidth_3db();
        self.unity_gain_freq = self.find_unity_gain_freq();

        // DC gain from lowest frequency point
        if let Some(first) = self.points.first() {
            if first.freq_in < 100.0 {
                self.dc_gain = Some(first.transfer);
            }
        }
    }
}

//=============================================================================
// PXF Analyzer
//=============================================================================

/// Periodic Transfer Function Analyzer
#[derive(Debug)]
pub struct PxfAnalyzer {
    /// Configuration
    config: PxfConfig,
}

impl PxfAnalyzer {
    /// Create new PXF analyzer
    pub fn new(config: PxfConfig) -> Self {
        Self { config }
    }

    /// Analyze transfer function using conversion matrix from PAC
    ///
    /// The conversion matrix H[n,m] gives the transfer from input sideband m
    /// to output sideband n.
    pub fn analyze_from_conversion_matrix(
        &self,
        frequencies: &[Value],
        conversion_matrix: &[Vec<Vec<Complex64>>], // [freq_idx][output_sb][input_sb]
        fundamental_freq: Value,
    ) -> Result<PxfResult, PxfError> {
        if frequencies.is_empty() {
            return Err(PxfError::InvalidConfiguration("No frequency points".into()));
        }

        if conversion_matrix.len() != frequencies.len() {
            return Err(PxfError::InvalidConfiguration(
                "Frequency/matrix size mismatch".into(),
            ));
        }

        let mut result = PxfResult::new(
            fundamental_freq,
            self.config.input_sideband,
            self.config.output_sideband,
        );

        for (i, &freq) in frequencies.iter().enumerate() {
            let matrix = &conversion_matrix[i];

            // Map sideband indices to matrix indices
            // Assuming matrix is indexed from 0 with offset
            let num_sidebands = matrix.len();
            let offset = (num_sidebands as i32 - 1) / 2;

            let out_idx = (self.config.output_sideband + offset) as usize;
            let in_idx = (self.config.input_sideband + offset) as usize;

            if out_idx >= num_sidebands || in_idx >= matrix[0].len() {
                continue; // Skip if sideband out of range
            }

            let transfer = matrix[out_idx][in_idx];

            // Compute output frequency based on sideband relationship
            let freq_out = freq
                + (self.config.output_sideband - self.config.input_sideband) as f64
                    * fundamental_freq;

            let point = TransferPoint {
                freq_in: freq,
                freq_out,
                transfer,
                sideband_in: self.config.input_sideband,
                sideband_out: self.config.output_sideband,
            };

            result.add_point(point);
        }

        result.compute_metrics();
        Ok(result)
    }

    /// Create a simple test transfer function (for testing)
    pub fn create_test_transfer(&self, gain_db: Value, pole_freq: Value) -> PxfResult {
        let frequencies = self.config.frequency_points();
        let mut result = PxfResult::new(
            self.config.fundamental_freq,
            self.config.input_sideband,
            self.config.output_sideband,
        );

        let gain_lin = 10.0_f64.powf(gain_db / 20.0);

        for freq in frequencies {
            // Single-pole transfer function: H(s) = gain / (1 + s/ω_p)
            let s = Complex64::new(0.0, 2.0 * PI * freq);
            let wp = 2.0 * PI * pole_freq;
            let transfer = Complex64::new(gain_lin, 0.0) / (1.0 + s / wp);

            let point = TransferPoint {
                freq_in: freq,
                freq_out: freq, // Same frequency for this test
                transfer,
                sideband_in: self.config.input_sideband,
                sideband_out: self.config.output_sideband,
            };

            result.add_point(point);
        }

        result.compute_metrics();
        result
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Configuration Tests
    // =========================================================================

    #[test]
    fn test_pxf_config_default() {
        let config = PxfConfig::default();
        assert!(config.freq_start > 0.0);
        assert!(config.freq_stop > config.freq_start);
        assert!(config.num_points > 0);
        assert_eq!(config.input_sideband, 1);
        assert_eq!(config.output_sideband, 0);
    }

    #[test]
    fn test_pxf_config_builder() {
        let config = PxfConfig::new()
            .with_sweep(1e6, 1e9, 100)
            .with_sweep_type(PxfSweepType::Linear)
            .with_sidebands(2, -1)
            .with_input("VRF")
            .with_output("VOUT")
            .with_fundamental(1e9)
            .with_noise(true);

        assert_eq!(config.freq_start, 1e6);
        assert_eq!(config.freq_stop, 1e9);
        assert_eq!(config.num_points, 100);
        assert_eq!(config.sweep_type, PxfSweepType::Linear);
        assert_eq!(config.input_sideband, 2);
        assert_eq!(config.output_sideband, -1);
        assert_eq!(config.input_source, Some("VRF".to_string()));
        assert_eq!(config.output_node, Some("VOUT".to_string()));
        assert!(config.include_noise);
    }

    #[test]
    fn test_pxf_config_validation() {
        let valid = PxfConfig::new().with_sweep(1e6, 1e9, 50);
        assert!(valid.validate().is_ok());

        let invalid_start = PxfConfig::new().with_sweep(-1e6, 1e9, 50);
        assert!(invalid_start.validate().is_err());

        let invalid_stop = PxfConfig::new().with_sweep(1e9, 1e6, 50);
        assert!(invalid_stop.validate().is_err());

        let zero_points = PxfConfig::new().with_sweep(1e6, 1e9, 0);
        assert!(zero_points.validate().is_err());
    }

    #[test]
    fn test_linear_frequency_points() {
        let config = PxfConfig::new()
            .with_sweep(100.0, 200.0, 11)
            .with_sweep_type(PxfSweepType::Linear);

        let points = config.frequency_points();
        assert_eq!(points.len(), 11);
        assert!((points[0] - 100.0).abs() < 1e-10);
        assert!((points[10] - 200.0).abs() < 1e-10);

        // Check uniform spacing
        let step = points[1] - points[0];
        for i in 2..points.len() {
            assert!((points[i] - points[i - 1] - step).abs() < 1e-10);
        }
    }

    #[test]
    fn test_decade_frequency_points() {
        let config = PxfConfig::new()
            .with_sweep(1e3, 1e6, 10) // 3 decades
            .with_sweep_type(PxfSweepType::Decade);

        let points = config.frequency_points();
        assert!(points.len() >= 25); // ~30 points for 3 decades

        // Check first and last
        assert!((points[0] - 1e3).abs() / 1e3 < 0.01);
        assert!((points[points.len() - 1] - 1e6).abs() / 1e6 < 0.01);

        // Check log spacing
        for i in 1..points.len() {
            assert!(points[i] > points[i - 1]);
        }
    }

    #[test]
    fn test_single_point_sweep() {
        let config = PxfConfig::new()
            .with_sweep(1e6, 1e9, 1)
            .with_sweep_type(PxfSweepType::Linear);

        let points = config.frequency_points();
        assert_eq!(points.len(), 1);
        assert!((points[0] - 1e6).abs() < 1.0);
    }

    // =========================================================================
    // Transfer Point Tests
    // =========================================================================

    #[test]
    fn test_transfer_point_magnitude() {
        let point = TransferPoint {
            freq_in: 1e6,
            freq_out: 1e6,
            transfer: Complex64::new(3.0, 4.0), // |H| = 5
            sideband_in: 0,
            sideband_out: 0,
        };

        assert!((point.magnitude() - 5.0).abs() < 1e-10);
        assert!((point.magnitude_db() - 20.0 * 5.0_f64.log10()).abs() < 0.01);
    }

    #[test]
    fn test_transfer_point_phase() {
        // Pure imaginary = 90 degrees
        let point = TransferPoint {
            freq_in: 1e6,
            freq_out: 1e6,
            transfer: Complex64::new(0.0, 1.0),
            sideband_in: 0,
            sideband_out: 0,
        };

        assert!((point.phase() - PI / 2.0).abs() < 1e-10);
        assert!((point.phase_degrees() - 90.0).abs() < 0.01);
    }

    #[test]
    fn test_transfer_point_group_delay() {
        let point1 = TransferPoint {
            freq_in: 1e6,
            freq_out: 1e6,
            transfer: Complex64::from_polar(1.0, 0.0),
            sideband_in: 0,
            sideband_out: 0,
        };

        let point2 = TransferPoint {
            freq_in: 2e6,
            freq_out: 2e6,
            transfer: Complex64::from_polar(1.0, -PI / 2.0), // 90 degree lag
            sideband_in: 0,
            sideband_out: 0,
        };

        let gd = point1.group_delay(&point2);
        // dφ/df = (-π/2) / (1e6) => τ = -dφ/(2πdf) = 0.5/(2*1e6) = 0.25 μs
        assert!(gd > 0.0); // Positive delay
    }

    // =========================================================================
    // Result Tests
    // =========================================================================

    #[test]
    fn test_pxf_result_creation() {
        let result = PxfResult::new(1e9, 1, 0);
        assert_eq!(result.fundamental_freq, 1e9);
        assert_eq!(result.input_sideband, 1);
        assert_eq!(result.output_sideband, 0);
        assert!(result.points.is_empty());
    }

    #[test]
    fn test_pxf_result_add_points() {
        let mut result = PxfResult::new(1e9, 1, 0);

        for i in 0..10 {
            let freq = 1e6 * (i + 1) as f64;
            result.add_point(TransferPoint {
                freq_in: freq,
                freq_out: freq,
                transfer: Complex64::new(1.0, 0.0),
                sideband_in: 1,
                sideband_out: 0,
            });
        }

        assert_eq!(result.num_points(), 10);
    }

    #[test]
    fn test_pxf_result_magnitude_curve() {
        let mut result = PxfResult::new(1e9, 1, 0);

        // Add points with decreasing gain
        for i in 0..5 {
            let freq = 10.0_f64.powi(i + 3); // 1k, 10k, 100k, 1M, 10M
            let gain = 10.0 / (1.0 + freq / 1e6); // Pole at 1 MHz
            result.add_point(TransferPoint {
                freq_in: freq,
                freq_out: freq,
                transfer: Complex64::new(gain, 0.0),
                sideband_in: 1,
                sideband_out: 0,
            });
        }

        let curve = result.magnitude_curve();
        assert_eq!(curve.len(), 5);

        // Gain should decrease with frequency
        for i in 1..curve.len() {
            assert!(curve[i].1 <= curve[i - 1].1);
        }
    }

    #[test]
    fn test_pxf_result_peak_gain() {
        let mut result = PxfResult::new(1e9, 1, 0);

        // Bandpass response - peak in middle
        let gains = [0.1, 0.5, 1.0, 0.5, 0.1];
        let freqs = [1e3, 1e4, 1e5, 1e6, 1e7];

        for (i, &gain) in gains.iter().enumerate() {
            result.add_point(TransferPoint {
                freq_in: freqs[i],
                freq_out: freqs[i],
                transfer: Complex64::new(gain, 0.0),
                sideband_in: 1,
                sideband_out: 0,
            });
        }

        let peak = result.find_peak_gain();
        assert!(peak.is_some());
        let (peak_freq, peak_db) = peak.unwrap();
        assert!((peak_freq - 1e5).abs() < 1.0);
        assert!((peak_db - 0.0).abs() < 0.01); // 1.0 linear = 0 dB
    }

    #[test]
    fn test_pxf_result_peak_gain_ignores_non_finite_points() {
        let mut result = PxfResult::new(1e9, 1, 0);
        result.add_point(TransferPoint {
            freq_in: 1e3,
            freq_out: 1e3,
            transfer: Complex64::new(0.1, 0.0),
            sideband_in: 1,
            sideband_out: 0,
        });
        result.add_point(TransferPoint {
            freq_in: 1e4,
            freq_out: 1e4,
            transfer: Complex64::new(f64::NAN, 0.0),
            sideband_in: 1,
            sideband_out: 0,
        });
        result.add_point(TransferPoint {
            freq_in: 1e5,
            freq_out: 1e5,
            transfer: Complex64::new(1.0, 0.0),
            sideband_in: 1,
            sideband_out: 0,
        });

        let (peak_freq, peak_db) = result.find_peak_gain().expect("peak should exist");
        assert!((peak_freq - 1e5).abs() < 1.0);
        assert!((peak_db - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_pxf_result_peak_gain_all_non_finite_returns_none() {
        let mut result = PxfResult::new(1e9, 1, 0);
        result.add_point(TransferPoint {
            freq_in: 1e3,
            freq_out: 1e3,
            transfer: Complex64::new(f64::NAN, 0.0),
            sideband_in: 1,
            sideband_out: 0,
        });
        result.add_point(TransferPoint {
            freq_in: 1e4,
            freq_out: 1e4,
            transfer: Complex64::new(f64::INFINITY, 0.0),
            sideband_in: 1,
            sideband_out: 0,
        });

        assert!(result.find_peak_gain().is_none());
    }

    #[test]
    fn test_pxf_result_unity_gain_freq() {
        let mut result = PxfResult::new(1e9, 1, 0);

        // Lowpass with unity gain at 1 MHz
        let freqs = [1e4, 1e5, 5e5, 1e6, 2e6, 1e7];
        let gains = [10.0, 3.16, 1.25, 1.0, 0.5, 0.1]; // Decreasing

        for (i, &gain) in gains.iter().enumerate() {
            result.add_point(TransferPoint {
                freq_in: freqs[i],
                freq_out: freqs[i],
                transfer: Complex64::new(gain, 0.0),
                sideband_in: 1,
                sideband_out: 0,
            });
        }

        let ugf = result.find_unity_gain_freq();
        // Should find crossing between 5e5 (1.25) and 1e6 (1.0)
        // or between 1e6 (1.0) and 2e6 (0.5)
        assert!(ugf.is_some());
    }

    // =========================================================================
    // Analyzer Tests
    // =========================================================================

    #[test]
    fn test_pxf_analyzer_creation() {
        let config = PxfConfig::default();
        let analyzer = PxfAnalyzer::new(config);
        assert_eq!(analyzer.config.input_sideband, 1);
    }

    #[test]
    fn test_pxf_analyzer_test_transfer() {
        let config = PxfConfig::new()
            .with_sweep(100.0, 1e6, 10)
            .with_sweep_type(PxfSweepType::Decade);

        let analyzer = PxfAnalyzer::new(config);
        let result = analyzer.create_test_transfer(20.0, 1e4); // 20dB gain, 10kHz pole

        assert!(result.num_points() > 0);

        // At low frequencies, gain should be ~20dB
        let low_freq_db = result.points.first().unwrap().magnitude_db();
        assert!((low_freq_db - 20.0).abs() < 1.0);

        // At high frequencies (above pole), gain should roll off
        let high_freq_db = result.points.last().unwrap().magnitude_db();
        assert!(high_freq_db < low_freq_db);
    }

    #[test]
    fn test_pxf_analyzer_from_conversion_matrix() {
        let config = PxfConfig::new()
            .with_sweep(1e6, 1e7, 5)
            .with_sidebands(1, 0)
            .with_fundamental(1e9);

        let analyzer = PxfAnalyzer::new(config);

        // Create test conversion matrix
        // 3x3 matrix: sidebands -1, 0, +1
        let frequencies = vec![1e6, 3e6, 5e6, 7e6, 1e7];
        let mut conversion_matrix = Vec::new();

        for _ in &frequencies {
            // Matrix: output_sideband x input_sideband
            let matrix = vec![
                vec![
                    Complex64::new(0.1, 0.0),
                    Complex64::new(0.0, 0.0),
                    Complex64::new(0.1, 0.0),
                ], // sb=-1
                vec![
                    Complex64::new(0.5, 0.0),
                    Complex64::new(1.0, 0.0),
                    Complex64::new(0.5, 0.0),
                ], // sb=0
                vec![
                    Complex64::new(0.1, 0.0),
                    Complex64::new(0.0, 0.0),
                    Complex64::new(0.1, 0.0),
                ], // sb=+1
            ];
            conversion_matrix.push(matrix);
        }

        let result = analyzer
            .analyze_from_conversion_matrix(&frequencies, &conversion_matrix, 1e9)
            .unwrap();

        assert_eq!(result.num_points(), 5);

        // Transfer from sideband +1 to sideband 0 should be 0.5
        for point in &result.points {
            assert!((point.magnitude() - 0.5).abs() < 0.01);
        }
    }

    #[test]
    fn test_pxf_empty_frequency_error() {
        let config = PxfConfig::new().with_sidebands(1, 0);
        let analyzer = PxfAnalyzer::new(config);

        let result = analyzer.analyze_from_conversion_matrix(&[], &[], 1e9);
        assert!(result.is_err());
    }

    // =========================================================================
    // Sweep Type Tests
    // =========================================================================

    #[test]
    fn test_sweep_type_default() {
        assert_eq!(PxfSweepType::default(), PxfSweepType::Decade);
    }

    #[test]
    fn test_octave_sweep() {
        let config = PxfConfig::new()
            .with_sweep(1e3, 8e3, 3) // 3 octaves
            .with_sweep_type(PxfSweepType::Octave);

        let points = config.frequency_points();
        assert!(points.len() >= 6); // ~9 points for 3 octaves at 3 pts/octave

        // Check log2 spacing
        for i in 1..points.len() {
            assert!(points[i] > points[i - 1]);
        }
    }

    // =========================================================================
    // Metrics Tests
    // =========================================================================

    #[test]
    fn test_compute_metrics() {
        let config = PxfConfig::new().with_sweep(1e3, 1e6, 10);
        let analyzer = PxfAnalyzer::new(config);

        let result = analyzer.create_test_transfer(26.0, 1e4); // ~20x gain, 10kHz pole

        // Metrics should be computed
        assert!(result.peak_gain.is_some());
    }

    #[test]
    fn test_group_delay_curve() {
        let config = PxfConfig::new()
            .with_sweep(1e3, 1e6, 50)
            .with_sweep_type(PxfSweepType::Decade);
        let analyzer = PxfAnalyzer::new(config);

        let result = analyzer.create_test_transfer(20.0, 1e4);
        let gd_curve = result.group_delay_curve();

        assert!(!gd_curve.is_empty());
        assert_eq!(gd_curve.len(), result.num_points() - 1);
    }
}
