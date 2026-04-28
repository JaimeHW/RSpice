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
        if let Some(first) = self.points.first()
            && first.freq_in < 100.0
        {
            self.dc_gain = Some(first.transfer);
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

