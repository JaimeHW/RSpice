//! PNoise Configuration Types
//!
//! Configuration for phase noise analysis with support for:
//! - Offset frequency sweeps (logarithmic or linear)
//! - Multiple sideband analysis (upper, lower, or both)
//! - Output node specification
//! - Integration bandwidth for RMS jitter calculation

use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::frequency_grid::{
    FrequencyGridError, FrequencyGridScale, copy_explicit_frequency_grid, frequency_point_count,
    generate_frequency_grid, validate_generated_sweep,
};

/// Phase noise analysis configuration
#[derive(Debug, Clone)]
pub struct PnoiseConfig {
    /// Reference node for phase noise measurement (typically oscillator output)
    pub output_node: NoiseOutputNode,

    /// Offset frequency sweep specification
    pub sweep: PnoiseSweep,

    /// Which sidebands to analyze
    pub sidebands: PnoiseSideband,

    /// Maximum number of sidebands (harmonics) to include
    pub max_sidebands: usize,

    /// Reference frequency for phase noise (carrier frequency)
    /// If None, uses fundamental from PSS/HB
    pub reference_freq: Option<Value>,

    /// Integration limits for RMS jitter calculation `Hz`
    pub jitter_integration: Option<(Value, Value)>,

    /// Number of points per decade for log sweep
    pub points_per_decade: usize,

    /// Relative tolerance for noise calculations
    pub reltol: Value,

    /// Absolute tolerance for noise calculations
    pub abstol: Value,
}

impl Default for PnoiseConfig {
    fn default() -> Self {
        Self {
            output_node: NoiseOutputNode::default(),
            sweep: PnoiseSweep::default(),
            sidebands: PnoiseSideband::Both,
            max_sidebands: 10,
            reference_freq: None,
            jitter_integration: None,
            points_per_decade: 10,
            reltol: 1e-3,
            abstol: 1e-18, // Very small for noise floor
        }
    }
}

impl PnoiseConfig {
    /// Create new PNoise config with specified output node and sweep
    pub fn new(output_node: &str, start_freq: Value, stop_freq: Value) -> Self {
        Self {
            output_node: NoiseOutputNode::single(output_node),
            sweep: PnoiseSweep::log(start_freq, stop_freq, 10),
            ..Default::default()
        }
    }

    /// Set the offset frequency sweep
    pub fn with_sweep(mut self, sweep: PnoiseSweep) -> Self {
        self.sweep = sweep;
        self
    }

    /// Set sideband analysis mode
    pub fn with_sidebands(mut self, sidebands: PnoiseSideband) -> Self {
        self.sidebands = sidebands;
        self
    }

    /// Set absolute tolerance
    pub fn with_abstol(mut self, tol: Value) -> Self {
        self.abstol = tol;
        self
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), PnoiseConfigError> {
        self.sweep
            .validate()
            .map_err(PnoiseConfigError::FrequencyGrid)?;
        if self.max_sidebands == 0 {
            return Err(PnoiseConfigError::Sidebands);
        }
        if !self.reltol.is_finite()
            || !self.abstol.is_finite()
            || self.reltol <= 0.0
            || self.abstol < 0.0
        {
            return Err(PnoiseConfigError::Tolerance);
        }
        Ok(())
    }

    /// Generate offset frequency points for the sweep
    pub fn offset_frequencies(&self) -> Result<Vec<Value>, FrequencyGridError> {
        self.try_offset_frequencies_with_abort(&NoAbort)
    }

    /// Generate offset frequencies with cooperative cancellation.
    pub fn try_offset_frequencies_with_abort(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, FrequencyGridError> {
        self.sweep.generate_points_with_abort(abort)
    }
}

/// Output node specification for phase noise measurement
#[derive(Debug, Clone, Default)]
pub struct NoiseOutputNode {
    /// Positive node name
    pub positive: String,
    /// Negative node name (ground if empty)
    pub negative: Option<String>,
}

impl NoiseOutputNode {
    /// Create single-ended output node (referenced to ground)
    pub fn single(node: &str) -> Self {
        Self {
            positive: node.to_string(),
            negative: None,
        }
    }

    /// Create differential output node
    pub fn differential(pos: &str, neg: &str) -> Self {
        Self {
            positive: pos.to_string(),
            negative: Some(neg.to_string()),
        }
    }
}

/// Offset frequency sweep specification
#[derive(Debug, Clone)]
pub enum PnoiseSweep {
    /// Logarithmic sweep (typical for phase noise)
    Log {
        start: Value,
        stop: Value,
        points_per_decade: usize,
    },
    /// Linear sweep
    Linear {
        start: Value,
        stop: Value,
        num_points: usize,
    },
    /// Explicit list of offset frequencies
    List(Vec<Value>),
}

impl Default for PnoiseSweep {
    fn default() -> Self {
        // Default: 1 Hz to 10 MHz offset, typical for VCO characterization
        Self::Log {
            start: 1.0,
            stop: 10e6,
            points_per_decade: 10,
        }
    }
}

impl PnoiseSweep {
    /// Create logarithmic sweep
    pub fn log(start: Value, stop: Value, points_per_decade: usize) -> Self {
        Self::Log {
            start,
            stop,
            points_per_decade,
        }
    }

    /// Create linear sweep
    pub fn linear(start: Value, stop: Value, num_points: usize) -> Self {
        Self::Linear {
            start,
            stop,
            num_points,
        }
    }

    /// Create sweep from explicit frequency list
    pub fn list(frequencies: Vec<Value>) -> Self {
        Self::List(frequencies)
    }

    /// Check if sweep is valid
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    /// Validate the authored sweep without allocating its retained grid.
    pub fn validate(&self) -> Result<(), FrequencyGridError> {
        match self {
            Self::Log {
                start,
                stop,
                points_per_decade,
            } => {
                validate_generated_sweep(
                    *start,
                    *stop,
                    *points_per_decade,
                    FrequencyGridScale::Decade,
                    false,
                )?;
                if stop == start {
                    return Err(FrequencyGridError::NonIncreasingSweep);
                }
                Ok(())
            }
            Self::Linear {
                start,
                stop,
                num_points,
            } => {
                validate_generated_sweep(
                    *start,
                    *stop,
                    *num_points,
                    FrequencyGridScale::Linear,
                    true,
                )?;
                if stop == start {
                    return Err(FrequencyGridError::NonIncreasingSweep);
                }
                Ok(())
            }
            Self::List(frequencies) => {
                if frequencies.is_empty() {
                    return Err(FrequencyGridError::EmptySweep);
                }
                if let Some(index) = frequencies
                    .iter()
                    .position(|frequency| !frequency.is_finite() || *frequency <= 0.0)
                {
                    return Err(FrequencyGridError::InvalidExplicitFrequency { index });
                }
                Ok(())
            }
        }
    }

    /// Generate frequency points while preserving validation and resource failures.
    pub fn generate_points(&self) -> Result<Vec<Value>, FrequencyGridError> {
        self.generate_points_with_abort(&NoAbort)
    }

    /// Generate frequency points with cooperative cancellation.
    pub fn generate_points_with_abort(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, FrequencyGridError> {
        self.validate()?;
        match self {
            Self::Log {
                start,
                stop,
                points_per_decade,
            } => generate_frequency_grid(
                *start,
                *stop,
                *points_per_decade,
                FrequencyGridScale::Decade,
                false,
                2,
                abort,
            ),
            Self::Linear {
                start,
                stop,
                num_points,
            } => generate_frequency_grid(
                *start,
                *stop,
                (*num_points).max(2),
                FrequencyGridScale::Linear,
                true,
                2,
                abort,
            ),
            Self::List(frequencies) => copy_explicit_frequency_grid(frequencies, abort),
        }
    }

    /// Return the generated point count without allocating the grid.
    pub fn point_count(&self) -> Result<usize, FrequencyGridError> {
        self.validate()?;
        match self {
            Self::Log {
                start,
                stop,
                points_per_decade,
            } => frequency_point_count(
                *start,
                *stop,
                *points_per_decade,
                FrequencyGridScale::Decade,
                2,
            ),
            Self::Linear { num_points, .. } => Ok((*num_points).max(2)),
            Self::List(frequencies) => Ok(frequencies.len()),
        }
    }

    /// Get start frequency
    pub fn start_freq(&self) -> Result<Value, FrequencyGridError> {
        self.validate()?;
        match self {
            Self::Log { start, .. } | Self::Linear { start, .. } => Ok(*start),
            Self::List(frequencies) => frequencies
                .first()
                .copied()
                .ok_or(FrequencyGridError::EmptySweep),
        }
    }

    /// Get stop frequency
    pub fn stop_freq(&self) -> Result<Value, FrequencyGridError> {
        self.validate()?;
        match self {
            Self::Log { stop, .. } | Self::Linear { stop, .. } => Ok(*stop),
            Self::List(frequencies) => frequencies
                .last()
                .copied()
                .ok_or(FrequencyGridError::EmptySweep),
        }
    }
}

/// Sideband analysis mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PnoiseSideband {
    /// Analyze only upper sidebands (f0 + offset)
    Upper,
    /// Analyze only lower sidebands (f0 - offset)
    Lower,
    /// Analyze both sidebands and combine (typical)
    #[default]
    Both,
}

/// Configuration errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PnoiseConfigError {
    /// Frequency-grid validation failure.
    FrequencyGrid(FrequencyGridError),
    /// Invalid sweep specification
    Sweep,
    /// Invalid sideband count
    Sidebands,
    /// Invalid tolerance values
    Tolerance,
}

impl std::fmt::Display for PnoiseConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FrequencyGrid(error) => write!(f, "Invalid offset frequency sweep: {error}"),
            Self::Sweep => write!(f, "Invalid offset frequency sweep"),
            Self::Sidebands => write!(f, "Invalid sideband configuration"),
            Self::Tolerance => write!(f, "Invalid tolerance values"),
        }
    }
}

impl std::error::Error for PnoiseConfigError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_offset_grids_preserve_ordinary_sweeps() {
        let logarithmic = PnoiseSweep::log(1.0, 100.0, 10)
            .generate_points()
            .expect("ordinary PNoise log grid");
        assert_eq!(logarithmic.len(), 20);
        assert_eq!(logarithmic.first(), Some(&1.0));
        assert_eq!(logarithmic.last(), Some(&100.0));

        assert_eq!(
            PnoiseSweep::linear(0.0, 1.0, 1)
                .generate_points()
                .expect("one authored point preserves legacy two-point minimum"),
            vec![0.0, 1.0]
        );
    }

    #[test]
    fn checked_offset_grids_reject_invalid_overflow_and_allocation_cases() {
        assert_eq!(
            PnoiseSweep::log(0.0, 1.0, 10).generate_points(),
            Err(FrequencyGridError::InvalidStartFrequency)
        );
        assert_eq!(
            PnoiseSweep::list(vec![1.0, f64::NAN]).generate_points(),
            Err(FrequencyGridError::InvalidExplicitFrequency { index: 1 })
        );
        for sweep in [
            PnoiseSweep::log(1.0, 1.0, 10),
            PnoiseSweep::linear(1.0, 1.0, 10),
        ] {
            assert_eq!(
                sweep.generate_points(),
                Err(FrequencyGridError::NonIncreasingSweep)
            );
        }
        assert_eq!(
            PnoiseSweep::log(f64::MIN_POSITIVE, f64::MAX, usize::MAX).point_count(),
            Err(FrequencyGridError::PointCountOverflow)
        );
        assert!(matches!(
            PnoiseSweep::linear(0.0, 1.0, usize::MAX / 2).generate_points(),
            Err(FrequencyGridError::Allocation { .. })
        ));
    }

    #[test]
    fn checked_offset_grids_propagate_cancellation() {
        assert_eq!(
            PnoiseSweep::log(1.0, 100.0, 10)
                .generate_points_with_abort(&crate::abort_signal::ImmediateAbort),
            Err(FrequencyGridError::Aborted)
        );
    }

    #[test]
    fn config_validation_rejects_non_finite_tolerances() {
        for config in [
            PnoiseConfig {
                reltol: f64::NAN,
                ..PnoiseConfig::default()
            },
            PnoiseConfig {
                abstol: f64::INFINITY,
                ..PnoiseConfig::default()
            },
        ] {
            assert_eq!(config.validate(), Err(PnoiseConfigError::Tolerance));
        }
    }
}
