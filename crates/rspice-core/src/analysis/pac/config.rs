//! PAC Analysis Configuration
//!
//! Provides configuration types for Periodic AC analysis including:
//! - Frequency sweep parameters (linear, decade, octave)
//! - Sideband range selection
//! - Accuracy and convergence controls

use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::frequency_grid::{
    FrequencyGridError, FrequencyGridScale, frequency_point_count, generate_frequency_grid,
    validate_generated_sweep,
};

//=============================================================================
// Frequency Sweep Type
//=============================================================================

/// Type of frequency sweep for PAC analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PacSweepType {
    /// Linear frequency sweep
    Linear,
    /// Decade (logarithmic) frequency sweep
    #[default]
    Decade,
    /// Octave frequency sweep
    Octave,
}

//=============================================================================
// PAC Configuration
//=============================================================================

/// Configuration for Periodic AC (PAC) analysis
///
/// PAC analysis performs small-signal AC analysis around a periodic steady-state
/// operating point. The result is a conversion matrix that relates input signals
/// at one frequency/sideband to output signals at other frequencies/sidebands.
///
/// # Example
///
/// ```ignore
/// let config = PacConfig::new()
///     .with_sweep(1e6, 1e9, 100)     // 1 MHz to 1 GHz, 100 points
///     .with_sidebands(-5, 5)          // Harmonics -5 to +5
///     .with_input_source("VRF")       // RF input source
///     .with_output_node("VOUT");      // Output node
/// ```
#[derive(Debug, Clone)]
pub struct PacConfig {
    /// Start frequency for input sweep (Hz)
    pub sweep_start: Value,

    /// Stop frequency for input sweep (Hz)
    pub sweep_stop: Value,

    /// Number of frequency points (or points per decade for log sweeps)
    pub num_points: usize,

    /// Sweep type (linear, decade, octave)
    pub sweep_type: PacSweepType,

    /// Minimum output sideband index relative to input (e.g., -5 for LO - 5*f₀)
    pub sideband_min: i32,

    /// Maximum output sideband index relative to input (e.g., +5 for LO + 5*f₀)  
    pub sideband_max: i32,

    /// Name of the input source (voltage or current source providing small-signal)
    pub input_source: Option<String>,

    /// Output node name for primary result extraction
    pub output_node: Option<String>,

    /// Reference node name (default: ground = "0")
    pub output_ref: Option<String>,

    /// Relative tolerance for frequency-domain solution
    pub reltol: Value,

    /// Absolute tolerance for small-signal currents (A)
    pub abstol: Value,

    /// Include DC component (sideband 0) in analysis
    pub include_dc: bool,

    /// Fundamental frequency from PSS (will be set from PSS result)
    pub fundamental_freq: Value,
}

impl Default for PacConfig {
    fn default() -> Self {
        Self {
            sweep_start: 1e3, // 1 kHz default start
            sweep_stop: 1e9,  // 1 GHz default stop
            num_points: 50,   // 50 points default
            sweep_type: PacSweepType::Decade,
            sideband_min: -5, // 5 sidebands below
            sideband_max: 5,  // 5 sidebands above
            input_source: None,
            output_node: None,
            output_ref: None,
            reltol: 1e-3,  // 0.1% relative tolerance
            abstol: 1e-12, // 1 pA absolute tolerance
            include_dc: true,
            fundamental_freq: 0.0, // Set from PSS
        }
    }
}

impl PacConfig {
    /// Create a new PAC configuration with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set frequency sweep parameters
    ///
    /// # Arguments
    /// * `start` - Start frequency in Hz
    /// * `stop` - Stop frequency in Hz  
    /// * `points` - Number of frequency points (total for linear, per decade for log)
    pub fn with_sweep(mut self, start: Value, stop: Value, points: usize) -> Self {
        self.sweep_start = start;
        self.sweep_stop = stop;
        self.num_points = points;
        self
    }

    /// Set sweep type
    pub fn with_sweep_type(mut self, sweep_type: PacSweepType) -> Self {
        self.sweep_type = sweep_type;
        self
    }

    /// Set sideband range for output
    ///
    /// # Arguments
    /// * `min` - Minimum sideband index (negative for lower sidebands)
    /// * `max` - Maximum sideband index (positive for upper sidebands)
    ///
    /// # Example
    /// ```ignore
    /// // For a mixer: input at RF (sideband +1), output at IF (sideband 0)
    /// config.with_sidebands(-3, 3);  // Analyze sidebands -3 to +3
    /// ```
    pub fn with_sidebands(mut self, min: i32, max: i32) -> Self {
        self.sideband_min = min;
        self.sideband_max = max;
        self
    }

    /// Set the input source name
    ///
    /// The input source should be a small-signal source (AC magnitude)
    /// that will be swept across the frequency range.
    pub fn with_input_source(mut self, source_name: &str) -> Self {
        self.input_source = Some(source_name.to_uppercase());
        self
    }

    /// Set the output node for primary result extraction
    pub fn with_output_node(mut self, node_name: &str) -> Self {
        self.output_node = Some(node_name.to_uppercase());
        self
    }

    /// Set the reference node (default is ground)
    pub fn with_output_ref(mut self, ref_name: &str) -> Self {
        self.output_ref = Some(ref_name.to_uppercase());
        self
    }

    /// Set convergence tolerances
    pub fn with_tolerances(mut self, reltol: Value, abstol: Value) -> Self {
        self.reltol = reltol;
        self.abstol = abstol;
        self
    }

    /// Enable or disable DC sideband (sideband 0)
    pub fn with_dc(mut self, include: bool) -> Self {
        self.include_dc = include;
        self
    }

    /// Set fundamental frequency (typically from PSS result)
    pub fn with_fundamental(mut self, freq: Value) -> Self {
        self.fundamental_freq = freq;
        self
    }

    /// Generate frequency points while preserving validation and resource failures.
    pub fn frequency_points(&self) -> Result<Vec<Value>, FrequencyGridError> {
        self.try_frequency_points()
    }

    /// Generate frequency points without a cancellation source.
    pub fn try_frequency_points(&self) -> Result<Vec<Value>, FrequencyGridError> {
        self.try_frequency_points_with_abort(&NoAbort)
    }

    /// Generate frequency points with cooperative cancellation.
    pub fn try_frequency_points_with_abort(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, FrequencyGridError> {
        generate_frequency_grid(
            self.sweep_start,
            self.sweep_stop,
            self.num_points,
            self.grid_scale(),
            false,
            1,
            abort,
        )
    }

    /// Number of points the configured sweep will generate, without
    /// allocating the frequency vector.
    pub fn frequency_point_count(&self) -> Result<usize, FrequencyGridError> {
        self.validate_frequency_sweep()?;
        frequency_point_count(
            self.sweep_start,
            self.sweep_stop,
            self.num_points,
            self.grid_scale(),
            1,
        )
    }

    /// Get the number of sidebands being analyzed
    pub fn num_sidebands(&self) -> usize {
        usize::try_from(i64::from(self.sideband_max) - i64::from(self.sideband_min) + 1)
            .unwrap_or(usize::MAX)
    }

    /// Get sideband indices as a vector
    pub fn sideband_indices(&self) -> Vec<i32> {
        (self.sideband_min..=self.sideband_max).collect()
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        self.validate_frequency_sweep()
            .map_err(|error| error.to_string())?;
        if self.sideband_min > self.sideband_max {
            return Err("Sideband min must be <= sideband max".to_string());
        }
        if !self.reltol.is_finite()
            || !self.abstol.is_finite()
            || self.reltol <= 0.0
            || self.abstol <= 0.0
        {
            return Err("Tolerances must be positive and finite".to_string());
        }
        Ok(())
    }

    fn validate_frequency_sweep(&self) -> Result<(), FrequencyGridError> {
        validate_generated_sweep(
            self.sweep_start,
            self.sweep_stop,
            self.num_points,
            self.grid_scale(),
            false,
        )
    }

    fn grid_scale(&self) -> FrequencyGridScale {
        match self.sweep_type {
            PacSweepType::Linear => FrequencyGridScale::Linear,
            PacSweepType::Decade => FrequencyGridScale::Decade,
            PacSweepType::Octave => FrequencyGridScale::Octave,
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
            PacConfig::new().with_sweep(f64::NAN, 1.0e3, 10),
            PacConfig::new().with_sweep(1.0, f64::INFINITY, 10),
            PacConfig::new().with_sweep(1.0, 1.0e3, 0),
        ] {
            assert!(
                config.validate().is_err(),
                "invalid PAC sweep config unexpectedly accepted: {config:?}"
            );
        }
    }

    #[test]
    fn validate_rejects_non_finite_tolerances() {
        for config in [
            PacConfig::new().with_tolerances(f64::NAN, 1.0e-12),
            PacConfig::new().with_tolerances(1.0e-3, f64::INFINITY),
        ] {
            assert!(
                config.validate().is_err(),
                "invalid PAC tolerance config unexpectedly accepted: {config:?}"
            );
        }
    }

    #[test]
    fn try_frequency_points_returns_validation_error_instead_of_empty_grid() {
        let config = PacConfig::new().with_sweep(1.0e6, 1.0, 10);

        let err = config
            .try_frequency_points()
            .expect_err("invalid PAC sweep should return the validation error");

        assert!(
            matches!(err, FrequencyGridError::InvalidStopFrequency),
            "unexpected PAC frequency error: {err}"
        );
        assert_eq!(
            config.frequency_points(),
            Err(FrequencyGridError::InvalidStopFrequency)
        );
    }

    #[test]
    fn frequency_grid_is_checked_fallible_and_cancellable() {
        assert_eq!(
            PacConfig::new()
                .with_sweep(1.0, 2.0, 3)
                .with_sweep_type(PacSweepType::Linear)
                .frequency_points()
                .expect("ordinary PAC grid"),
            vec![1.0, 1.5, 2.0]
        );
        assert!(matches!(
            PacConfig::new()
                .with_sweep(1.0, 2.0, usize::MAX)
                .with_sweep_type(PacSweepType::Linear)
                .frequency_points(),
            Err(FrequencyGridError::Allocation { .. })
        ));
        assert_eq!(
            PacConfig::new().try_frequency_points_with_abort(&crate::abort_signal::ImmediateAbort),
            Err(FrequencyGridError::Aborted)
        );
        assert_eq!(
            PacConfig::new()
                .with_sweep(f64::MIN_POSITIVE, f64::MAX, usize::MAX)
                .frequency_point_count(),
            Err(FrequencyGridError::PointCountOverflow)
        );
        assert_eq!(
            PacConfig::new()
                .with_sweep(1.0e3, 1.0e3, 10)
                .frequency_points()
                .expect("equal PAC endpoints remain valid"),
            vec![1.0e3]
        );
    }
}
