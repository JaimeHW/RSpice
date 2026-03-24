//! PAC Analysis Configuration
//!
//! Provides configuration types for Periodic AC analysis including:
//! - Frequency sweep parameters (linear, decade, octave)
//! - Sideband range selection
//! - Accuracy and convergence controls

use crate::Value;

//=============================================================================
// Frequency Sweep Type
//=============================================================================

/// Type of frequency sweep for PAC analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
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

    /// Generate frequency points based on sweep configuration
    pub fn frequency_points(&self) -> Vec<Value> {
        match self.sweep_type {
            PacSweepType::Linear => self.linear_points(),
            PacSweepType::Decade => self.decade_points(),
            PacSweepType::Octave => self.octave_points(),
        }
    }

    /// Generate linear frequency sweep points
    fn linear_points(&self) -> Vec<Value> {
        if self.num_points <= 1 {
            return vec![self.sweep_start];
        }

        let step = (self.sweep_stop - self.sweep_start) / (self.num_points - 1) as Value;
        (0..self.num_points)
            .map(|i| self.sweep_start + step * i as Value)
            .collect()
    }

    /// Generate logarithmic (decade) frequency sweep points
    fn decade_points(&self) -> Vec<Value> {
        if self.sweep_start <= 0.0 || self.sweep_stop <= 0.0 {
            return self.linear_points();
        }

        let log_start = self.sweep_start.log10();
        let log_stop = self.sweep_stop.log10();
        let num_decades = log_stop - log_start;
        let total_points = (num_decades * self.num_points as Value).ceil() as usize;

        if total_points <= 1 {
            return vec![self.sweep_start];
        }

        let log_step = (log_stop - log_start) / (total_points - 1) as Value;
        (0..total_points)
            .map(|i| 10.0_f64.powf(log_start + log_step * i as Value))
            .collect()
    }

    /// Generate octave frequency sweep points
    fn octave_points(&self) -> Vec<Value> {
        if self.sweep_start <= 0.0 || self.sweep_stop <= 0.0 {
            return self.linear_points();
        }

        let log2_start = self.sweep_start.log2();
        let log2_stop = self.sweep_stop.log2();
        let num_octaves = log2_stop - log2_start;
        let total_points = (num_octaves * self.num_points as Value).ceil() as usize;

        if total_points <= 1 {
            return vec![self.sweep_start];
        }

        let log_step = (log2_stop - log2_start) / (total_points - 1) as Value;
        (0..total_points)
            .map(|i| 2.0_f64.powf(log2_start + log_step * i as Value))
            .collect()
    }

    /// Get the number of sidebands being analyzed
    pub fn num_sidebands(&self) -> usize {
        (self.sideband_max - self.sideband_min + 1) as usize
    }

    /// Get sideband indices as a vector
    pub fn sideband_indices(&self) -> Vec<i32> {
        (self.sideband_min..=self.sideband_max).collect()
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.sweep_start <= 0.0 {
            return Err("Sweep start frequency must be positive".to_string());
        }
        if self.sweep_stop < self.sweep_start {
            return Err("Sweep stop frequency must be >= start".to_string());
        }
        if self.num_points == 0 {
            return Err("Number of frequency points must be at least 1".to_string());
        }
        if self.sideband_min > self.sideband_max {
            return Err("Sideband min must be <= sideband max".to_string());
        }
        if self.reltol <= 0.0 || self.abstol <= 0.0 {
            return Err("Tolerances must be positive".to_string());
        }
        Ok(())
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pac_config_default() {
        let config = PacConfig::new();
        assert_eq!(config.sweep_start, 1e3);
        assert_eq!(config.sweep_stop, 1e9);
        assert_eq!(config.num_points, 50);
        assert_eq!(config.sideband_min, -5);
        assert_eq!(config.sideband_max, 5);
        assert!(config.include_dc);
    }

    #[test]
    fn test_pac_config_builder() {
        let config = PacConfig::new()
            .with_sweep(1e6, 1e9, 100)
            .with_sidebands(-3, 3)
            .with_input_source("VRF")
            .with_output_node("VOUT")
            .with_sweep_type(PacSweepType::Linear);

        assert_eq!(config.sweep_start, 1e6);
        assert_eq!(config.sweep_stop, 1e9);
        assert_eq!(config.num_points, 100);
        assert_eq!(config.sideband_min, -3);
        assert_eq!(config.sideband_max, 3);
        assert_eq!(config.input_source, Some("VRF".to_string()));
        assert_eq!(config.output_node, Some("VOUT".to_string()));
        assert_eq!(config.sweep_type, PacSweepType::Linear);
    }

    #[test]
    fn test_linear_frequency_points() {
        let config = PacConfig::new()
            .with_sweep(1e6, 1e7, 10)
            .with_sweep_type(PacSweepType::Linear);

        let points = config.frequency_points();
        assert_eq!(points.len(), 10);
        assert!((points[0] - 1e6).abs() < 1.0);
        assert!((points[9] - 1e7).abs() < 1.0);

        // Check uniform spacing
        let step = (1e7 - 1e6) / 9.0;
        for i in 1..points.len() {
            let expected_step = points[i] - points[i - 1];
            assert!(
                (expected_step - step).abs() < 1.0,
                "Step at {} should be uniform",
                i
            );
        }
    }

    #[test]
    fn test_decade_frequency_points() {
        let config = PacConfig::new()
            .with_sweep(1e3, 1e6, 10) // 3 decades, 10 points/decade = 30 points
            .with_sweep_type(PacSweepType::Decade);

        let points = config.frequency_points();

        // Should have approximately 30 points (3 decades * 10 points/decade)
        assert!(
            points.len() >= 28 && points.len() <= 32,
            "Expected ~30 points, got {}",
            points.len()
        );

        // First and last should be close to sweep bounds
        assert!((points[0] - 1e3).abs() < 10.0);
        let last = points[points.len() - 1];
        assert!((last - 1e6).abs() / 1e6 < 0.01);

        // Check logarithmic spacing
        for i in 1..points.len() {
            let ratio = points[i] / points[i - 1];
            assert!(ratio > 1.0, "Frequencies should increase");
        }
    }

    #[test]
    fn test_octave_frequency_points() {
        let config = PacConfig::new()
            .with_sweep(1e3, 8e3, 3) // 3 octaves, 3 points/octave
            .with_sweep_type(PacSweepType::Octave);

        let points = config.frequency_points();
        assert!(
            points.len() >= 8 && points.len() <= 10,
            "Expected ~9 points for 3 octaves, got {}",
            points.len()
        );

        // First point should be at sweep start
        assert!((points[0] - 1e3).abs() < 1.0);
    }

    #[test]
    fn test_num_sidebands() {
        let config = PacConfig::new().with_sidebands(-5, 5);
        assert_eq!(config.num_sidebands(), 11); // -5 to +5 inclusive

        let config2 = PacConfig::new().with_sidebands(0, 3);
        assert_eq!(config2.num_sidebands(), 4); // 0, 1, 2, 3
    }

    #[test]
    fn test_sideband_indices() {
        let config = PacConfig::new().with_sidebands(-2, 2);
        let indices = config.sideband_indices();
        assert_eq!(indices, vec![-2, -1, 0, 1, 2]);
    }

    #[test]
    fn test_config_validation_valid() {
        let config = PacConfig::new().with_sweep(1e6, 1e9, 100);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_invalid_start() {
        let config = PacConfig::new().with_sweep(-1e6, 1e9, 100);
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_invalid_stop() {
        let config = PacConfig::new().with_sweep(1e9, 1e6, 100); // stop < start
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_invalid_sidebands() {
        let config = PacConfig::new().with_sidebands(5, -5); // min > max
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_zero_points() {
        let mut config = PacConfig::new();
        config.num_points = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_single_point_sweep() {
        let config = PacConfig::new()
            .with_sweep(1e6, 1e9, 1)
            .with_sweep_type(PacSweepType::Linear);

        let points = config.frequency_points();
        assert_eq!(points.len(), 1);
        assert!((points[0] - 1e6).abs() < 1.0);
    }

    #[test]
    fn test_tolerances() {
        let config = PacConfig::new().with_tolerances(1e-6, 1e-15);

        assert!((config.reltol - 1e-6).abs() < 1e-12);
        assert!((config.abstol - 1e-15).abs() < 1e-20);
    }

    #[test]
    fn test_dc_toggle() {
        let config_with_dc = PacConfig::new().with_dc(true);
        assert!(config_with_dc.include_dc);

        let config_without_dc = PacConfig::new().with_dc(false);
        assert!(!config_without_dc.include_dc);
    }

    #[test]
    fn test_fundamental_frequency() {
        let config = PacConfig::new().with_fundamental(1e9);
        assert!((config.fundamental_freq - 1e9).abs() < 1.0);
    }

    #[test]
    fn test_sweep_type_default() {
        assert_eq!(PacSweepType::default(), PacSweepType::Decade);
    }

    #[test]
    fn test_output_ref() {
        let config = PacConfig::new().with_output_ref("VSS");
        assert_eq!(config.output_ref, Some("VSS".to_string()));
    }
}
