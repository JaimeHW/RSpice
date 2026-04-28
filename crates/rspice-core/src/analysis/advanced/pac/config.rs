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
