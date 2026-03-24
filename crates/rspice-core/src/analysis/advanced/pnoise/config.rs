//! PNoise Configuration Types
//!
//! Configuration for phase noise analysis with support for:
//! - Offset frequency sweeps (logarithmic or linear)
//! - Multiple sideband analysis (upper, lower, or both)
//! - Output node specification
//! - Integration bandwidth for RMS jitter calculation

use crate::Value;

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

    /// Integration limits for RMS jitter calculation [Hz]
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

    /// Set maximum number of sidebands to analyze
    pub fn with_max_sidebands(mut self, n: usize) -> Self {
        self.max_sidebands = n;
        self
    }

    /// Set carrier/reference frequency explicitly
    pub fn with_reference_freq(mut self, freq: Value) -> Self {
        self.reference_freq = Some(freq);
        self
    }

    /// Set jitter integration bandwidth
    pub fn with_jitter_integration(mut self, start: Value, stop: Value) -> Self {
        self.jitter_integration = Some((start, stop));
        self
    }

    /// Set points per decade for log sweep
    pub fn with_points_per_decade(mut self, ppd: usize) -> Self {
        self.points_per_decade = ppd.max(1);
        self
    }

    /// Set relative tolerance
    pub fn with_reltol(mut self, tol: Value) -> Self {
        self.reltol = tol;
        self
    }

    /// Set absolute tolerance
    pub fn with_abstol(mut self, tol: Value) -> Self {
        self.abstol = tol;
        self
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), PnoiseConfigError> {
        if !self.sweep.is_valid() {
            return Err(PnoiseConfigError::Sweep);
        }
        if self.max_sidebands == 0 {
            return Err(PnoiseConfigError::Sidebands);
        }
        if self.reltol <= 0.0 || self.abstol < 0.0 {
            return Err(PnoiseConfigError::Tolerance);
        }
        Ok(())
    }

    /// Generate offset frequency points for the sweep
    pub fn offset_frequencies(&self) -> Vec<Value> {
        self.sweep.generate_points(self.points_per_decade)
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
        match self {
            Self::Log {
                start,
                stop,
                points_per_decade,
            } => *start > 0.0 && *stop > *start && *points_per_decade > 0,
            Self::Linear {
                start,
                stop,
                num_points,
            } => *start >= 0.0 && *stop > *start && *num_points > 0,
            Self::List(freqs) => !freqs.is_empty() && freqs.iter().all(|&f| f > 0.0),
        }
    }

    /// Generate frequency points
    pub fn generate_points(&self, default_ppd: usize) -> Vec<Value> {
        match self {
            Self::Log {
                start,
                stop,
                points_per_decade,
            } => {
                let ppd = if *points_per_decade > 0 {
                    *points_per_decade
                } else {
                    default_ppd
                };
                let log_start = start.log10();
                let log_stop = stop.log10();
                let decades = log_stop - log_start;
                let total_points = ((decades * ppd as f64).ceil() as usize).max(2);

                (0..total_points)
                    .map(|i| {
                        let t = i as f64 / (total_points - 1) as f64;
                        10.0_f64.powf(log_start + t * decades)
                    })
                    .collect()
            }
            Self::Linear {
                start,
                stop,
                num_points,
            } => {
                let n = (*num_points).max(2);
                (0..n)
                    .map(|i| {
                        let t = i as f64 / (n - 1) as f64;
                        start + t * (stop - start)
                    })
                    .collect()
            }
            Self::List(freqs) => freqs.clone(),
        }
    }

    /// Get start frequency
    pub fn start_freq(&self) -> Value {
        match self {
            Self::Log { start, .. } => *start,
            Self::Linear { start, .. } => *start,
            Self::List(freqs) => freqs.first().copied().unwrap_or(1.0),
        }
    }

    /// Get stop frequency
    pub fn stop_freq(&self) -> Value {
        match self {
            Self::Log { stop, .. } => *stop,
            Self::Linear { stop, .. } => *stop,
            Self::List(freqs) => freqs.last().copied().unwrap_or(1e6),
        }
    }
}

/// Sideband analysis mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
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
mod config_tests {
    use super::*;

    #[test]
    fn test_pnoise_config_default() {
        let config = PnoiseConfig::default();
        assert_eq!(config.max_sidebands, 10);
        assert_eq!(config.sidebands, PnoiseSideband::Both);
        assert_eq!(config.points_per_decade, 10);
    }

    #[test]
    fn test_pnoise_config_new() {
        let config = PnoiseConfig::new("out", 1.0, 1e6);
        assert_eq!(config.output_node.positive, "out");
        assert!(config.output_node.negative.is_none());
    }

    #[test]
    fn test_pnoise_config_builder() {
        let config = PnoiseConfig::new("vco_out", 10.0, 10e6)
            .with_max_sidebands(5)
            .with_sidebands(PnoiseSideband::Upper)
            .with_reference_freq(1e9)
            .with_points_per_decade(20)
            .with_jitter_integration(1e3, 1e7);

        assert_eq!(config.max_sidebands, 5);
        assert_eq!(config.sidebands, PnoiseSideband::Upper);
        assert_eq!(config.reference_freq, Some(1e9));
        assert_eq!(config.points_per_decade, 20);
        assert!(config.jitter_integration.is_some());
    }

    #[test]
    fn test_pnoise_config_validate_valid() {
        let config = PnoiseConfig::new("out", 1.0, 1e6);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_pnoise_config_validate_invalid_sidebands() {
        let config = PnoiseConfig::new("out", 1.0, 1e6).with_max_sidebands(0);
        assert!(matches!(
            config.validate(),
            Err(PnoiseConfigError::Sidebands)
        ));
    }

    #[test]
    fn test_pnoise_config_validate_invalid_tolerance() {
        let config = PnoiseConfig::new("out", 1.0, 1e6).with_reltol(-1.0);
        assert!(matches!(
            config.validate(),
            Err(PnoiseConfigError::Tolerance)
        ));
    }

    #[test]
    fn test_output_node_single() {
        let node = NoiseOutputNode::single("vco_out");
        assert_eq!(node.positive, "vco_out");
        assert!(node.negative.is_none());
    }

    #[test]
    fn test_output_node_differential() {
        let node = NoiseOutputNode::differential("outp", "outn");
        assert_eq!(node.positive, "outp");
        assert_eq!(node.negative, Some("outn".to_string()));
    }

    #[test]
    fn test_sweep_log_default() {
        let sweep = PnoiseSweep::default();
        assert!(sweep.is_valid());
        assert_eq!(sweep.start_freq(), 1.0);
        assert_eq!(sweep.stop_freq(), 10e6);
    }

    #[test]
    fn test_sweep_log_generate() {
        let sweep = PnoiseSweep::log(1.0, 1000.0, 3); // 3 decades, 3 ppd = 9 points
        let points = sweep.generate_points(10);

        assert!(points.len() >= 2);
        assert!((points[0] - 1.0).abs() < 0.1);
        assert!((points.last().unwrap() - 1000.0).abs() < 1.0);

        // Check logarithmic spacing
        for i in 1..points.len() {
            assert!(points[i] > points[i - 1]);
        }
    }

    #[test]
    fn test_sweep_linear_generate() {
        let sweep = PnoiseSweep::linear(0.0, 100.0, 11);
        let points = sweep.generate_points(10);

        assert_eq!(points.len(), 11);
        assert_eq!(points[0], 0.0);
        assert_eq!(points[10], 100.0);

        // Check linear spacing
        for i in 1..points.len() {
            let expected_step = 10.0;
            assert!((points[i] - points[i - 1] - expected_step).abs() < 0.01);
        }
    }

    #[test]
    fn test_sweep_list() {
        let freqs = vec![10.0, 100.0, 1000.0, 10000.0];
        let sweep = PnoiseSweep::list(freqs.clone());
        let points = sweep.generate_points(10);

        assert_eq!(points, freqs);
    }

    #[test]
    fn test_sweep_invalid() {
        let sweep_neg = PnoiseSweep::log(-1.0, 100.0, 10);
        assert!(!sweep_neg.is_valid());

        let sweep_reversed = PnoiseSweep::log(100.0, 1.0, 10);
        assert!(!sweep_reversed.is_valid());

        let sweep_empty = PnoiseSweep::List(vec![]);
        assert!(!sweep_empty.is_valid());
    }

    #[test]
    fn test_sideband_default() {
        assert_eq!(PnoiseSideband::default(), PnoiseSideband::Both);
    }

    #[test]
    fn test_config_error_display() {
        assert!(
            PnoiseConfigError::Sweep
                .to_string()
                .contains("sweep")
        );
        assert!(
            PnoiseConfigError::Sidebands
                .to_string()
                .contains("sideband")
        );
        assert!(
            PnoiseConfigError::Tolerance
                .to_string()
                .contains("tolerance")
        );
    }

    #[test]
    fn test_offset_frequencies_generation() {
        let config =
            PnoiseConfig::new("out", 1.0, 1e6).with_sweep(PnoiseSweep::log(10.0, 10000.0, 3));

        let freqs = config.offset_frequencies();
        assert!(!freqs.is_empty());
        assert!((freqs[0] - 10.0).abs() < 1.0);
    }
}
