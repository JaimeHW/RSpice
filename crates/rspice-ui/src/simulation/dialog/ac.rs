//! AC Analysis Configuration
//!
//! Configuration for small-signal AC analysis (.ac).


// =============================================================================
// Frequency Sweep Type
// =============================================================================

/// Type of frequency sweep
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FrequencySweep {
    /// Decades (logarithmic)
    #[default]
    Decade,
    /// Octaves (logarithmic)
    Octave,
    /// Linear
    Linear,
}

impl FrequencySweep {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Decade => "Decade",
            Self::Octave => "Octave",
            Self::Linear => "Linear",
        }
    }

    /// SPICE keyword
    pub fn spice_keyword(&self) -> &'static str {
        match self {
            Self::Decade => "dec",
            Self::Octave => "oct",
            Self::Linear => "lin",
        }
    }

    /// All types
    pub fn all() -> &'static [FrequencySweep] {
        &[Self::Decade, Self::Octave, Self::Linear]
    }
}

// =============================================================================
// AC Config
// =============================================================================

/// AC analysis configuration
#[derive(Debug, Clone)]
pub struct AcConfig {
    /// Sweep type
    pub sweep_type: FrequencySweep,
    /// Number of points (per decade/octave for log, total for linear)
    pub num_points: u32,
    /// Start frequency (Hz)
    pub start_freq: f64,
    /// Stop frequency (Hz)
    pub stop_freq: f64,
    /// Save all nodes
    pub save_all: bool,
}

impl Default for AcConfig {
    fn default() -> Self {
        Self {
            sweep_type: FrequencySweep::Decade,
            num_points: 10,
            start_freq: 1.0,
            stop_freq: 1e9,
            save_all: true,
        }
    }
}

impl AcConfig {
    /// Create new config
    pub fn new(start: f64, stop: f64, points: u32) -> Self {
        Self {
            start_freq: start,
            stop_freq: stop,
            num_points: points,
            ..Default::default()
        }
    }

    /// Set sweep type
    pub fn with_sweep(mut self, sweep: FrequencySweep) -> Self {
        self.sweep_type = sweep;
        self
    }

    /// Generate SPICE directive
    pub fn to_spice(&self) -> String {
        format!(
            ".ac {} {} {} {}",
            self.sweep_type.spice_keyword(),
            self.num_points,
            format_freq(self.start_freq),
            format_freq(self.stop_freq)
        )
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.start_freq <= 0.0 {
            return Err("Start frequency must be positive".to_string());
        }

        if self.stop_freq <= 0.0 {
            return Err("Stop frequency must be positive".to_string());
        }

        if self.start_freq >= self.stop_freq {
            return Err("Start frequency must be less than stop frequency".to_string());
        }

        if self.num_points == 0 {
            return Err("Number of points must be at least 1".to_string());
        }

        Ok(())
    }

    /// Total number of frequency points
    pub fn total_points(&self) -> u32 {
        match self.sweep_type {
            FrequencySweep::Decade => {
                let decades = (self.stop_freq / self.start_freq).log10();
                (decades * self.num_points as f64).ceil() as u32 + 1
            }
            FrequencySweep::Octave => {
                let octaves = (self.stop_freq / self.start_freq).log2();
                (octaves * self.num_points as f64).ceil() as u32 + 1
            }
            FrequencySweep::Linear => self.num_points,
        }
    }
}


fn format_freq(freq: f64) -> String {
    if freq >= 1e9 {
        format!("{}G", freq / 1e9)
    } else if freq >= 1e6 {
        format!("{}Meg", freq / 1e6)
    } else if freq >= 1e3 {
        format!("{}k", freq / 1e3)
    } else {
        format!("{}", freq)
    }
}

// =============================================================================
// Tests
// =============================================================================
