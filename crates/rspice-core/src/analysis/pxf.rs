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
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::frequency_grid::{
    FrequencyGridError, FrequencyGridScale, frequency_point_count, generate_frequency_grid,
    validate_generated_sweep,
};
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

    /// Generate frequency points while preserving validation and resource failures.
    pub fn frequency_points(&self) -> Result<Vec<Value>, PxfError> {
        self.try_frequency_points()
    }

    /// Generate frequency points, preserving validation failures for callers
    /// that need to distinguish invalid input from a deliberately empty grid.
    pub fn try_frequency_points(&self) -> Result<Vec<Value>, PxfError> {
        self.try_frequency_points_with_abort(&NoAbort)
    }

    /// Generate frequency points with cooperative cancellation.
    pub(crate) fn try_frequency_points_with_abort(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, PxfError> {
        generate_frequency_grid(
            self.freq_start,
            self.freq_stop,
            self.num_points,
            self.grid_scale(),
            false,
            1,
            abort,
        )
        .map_err(PxfError::FrequencyGrid)
    }

    /// Number of points the configured sweep will retain without allocating it.
    pub fn frequency_point_count(&self) -> Result<usize, PxfError> {
        self.validate_frequency_sweep()?;
        frequency_point_count(
            self.freq_start,
            self.freq_stop,
            self.num_points,
            self.grid_scale(),
            1,
        )
        .map_err(PxfError::FrequencyGrid)
    }

    fn grid_scale(&self) -> FrequencyGridScale {
        match self.sweep_type {
            PxfSweepType::Linear => FrequencyGridScale::Linear,
            PxfSweepType::Decade => FrequencyGridScale::Decade,
            PxfSweepType::Octave => FrequencyGridScale::Octave,
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), PxfError> {
        self.validate_frequency_sweep()
    }

    fn validate_frequency_sweep(&self) -> Result<(), PxfError> {
        validate_generated_sweep(
            self.freq_start,
            self.freq_stop,
            self.num_points,
            self.grid_scale(),
            false,
        )
        .map_err(PxfError::FrequencyGrid)
    }
}

//=============================================================================
// PXF Error
//=============================================================================

/// Errors during PXF analysis
#[derive(Debug, Clone)]
pub enum PxfError {
    /// Frequency-grid validation, capacity, allocation, or cancellation failure.
    FrequencyGrid(FrequencyGridError),
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
            PxfError::FrequencyGrid(error) => write!(f, "PXF frequency grid: {error}"),
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
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_rejects_non_finite_sweep_values() {
        for config in [
            PxfConfig::new().with_sweep(f64::NAN, 1.0e3, 10),
            PxfConfig::new().with_sweep(1.0, f64::INFINITY, 10),
            PxfConfig::new().with_sweep(1.0, 1.0e3, 0),
        ] {
            assert!(
                config.validate().is_err(),
                "invalid PXF sweep config unexpectedly accepted: {config:?}"
            );
        }
    }

    #[test]
    fn try_frequency_points_preserves_validation_error() {
        let config = PxfConfig::new().with_sweep(1.0e6, 1.0, 10);

        let err = config
            .try_frequency_points()
            .expect_err("invalid PXF sweep should return the validation error");

        assert!(matches!(
            err,
            PxfError::FrequencyGrid(FrequencyGridError::InvalidStopFrequency)
        ));
        assert!(matches!(
            config.frequency_points(),
            Err(PxfError::FrequencyGrid(
                FrequencyGridError::InvalidStopFrequency
            ))
        ));
    }

    #[test]
    fn frequency_grid_is_checked_fallible_and_cancellable() {
        assert_eq!(
            PxfConfig::new()
                .with_sweep(1.0, 2.0, 3)
                .with_sweep_type(PxfSweepType::Linear)
                .frequency_points()
                .expect("ordinary PXF grid"),
            vec![1.0, 1.5, 2.0]
        );
        assert!(matches!(
            PxfConfig::new()
                .with_sweep(1.0, 2.0, usize::MAX)
                .with_sweep_type(PxfSweepType::Linear)
                .frequency_points(),
            Err(PxfError::FrequencyGrid(
                FrequencyGridError::Allocation { .. }
            ))
        ));
        assert!(matches!(
            PxfConfig::new().try_frequency_points_with_abort(&crate::abort_signal::ImmediateAbort),
            Err(PxfError::FrequencyGrid(FrequencyGridError::Aborted))
        ));
        assert!(matches!(
            PxfConfig::new()
                .with_sweep(f64::MIN_POSITIVE, f64::MAX, usize::MAX)
                .frequency_point_count(),
            Err(PxfError::FrequencyGrid(
                FrequencyGridError::PointCountOverflow
            ))
        ));
        assert_eq!(
            PxfConfig::new()
                .with_sweep(1.0e3, 1.0e3, 10)
                .frequency_points()
                .expect("equal PXF endpoints remain valid"),
            vec![1.0e3]
        );
    }
}
