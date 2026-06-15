//! Periodic Noise (PNoise) Analysis Configuration
//!
//! Configuration for periodic noise analysis around a PSS operating point.
//! PNoise computes noise contributions from all components, including the
//! frequency translation effects that are critical for oscillators and mixers.
//!
//! # Commercial Features (Spectre-Compatible)
//!
//! - Sideband noise folding
//! - Spot noise and integrated noise
//! - Phase noise and jitter calculation
//! - Per-device noise contribution
//!
//! # Example SPICE Output
//!
//! ```text
//! .pnoise dec 10 1 1Meg
//! + output=VOUT maxsideband=5 noiseref=yes
//! ```

use super::options::parse_si_value;

// =============================================================================
// PNoise Sweep Type
// =============================================================================

/// Type of frequency sweep for PNoise analysis
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PnoiseSweepType {
    /// Decades (logarithmic)
    #[default]
    Decade,
    /// Octaves (logarithmic)
    Octave,
    /// Linear
    Linear,
}

impl PnoiseSweepType {
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
    pub fn all() -> &'static [PnoiseSweepType] {
        &[Self::Decade, Self::Octave, Self::Linear]
    }
}

// =============================================================================
// Noise Reference Type
// =============================================================================

/// Type of noise reference for PNoise
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NoiseReferenceType {
    /// Output-referred noise (V/√Hz or A/√Hz)
    #[default]
    Output,
    /// Input-referred noise
    Input,
    /// Phase noise (dBc/Hz)
    Phase,
}

impl NoiseReferenceType {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Output => "Output-Referred",
            Self::Input => "Input-Referred",
            Self::Phase => "Phase Noise (dBc/Hz)",
        }
    }

    /// SPICE keyword
    pub fn spice_keyword(&self) -> &'static str {
        match self {
            Self::Output => "output",
            Self::Input => "input",
            Self::Phase => "phase",
        }
    }

    /// All types
    pub fn all() -> &'static [NoiseReferenceType] {
        &[Self::Output, Self::Input, Self::Phase]
    }
}

// =============================================================================
// PNoise Configuration
// =============================================================================

/// Periodic noise (PNoise) analysis configuration
///
/// Commercial-grade configuration matching Cadence Spectre PNoise parameters.
#[derive(Debug, Clone)]
pub struct PnoiseConfig {
    /// Start frequency (Hz)
    pub start_freq: f64,
    /// Stop frequency (Hz)
    pub stop_freq: f64,
    /// Number of points (per decade for log, total for linear)
    pub num_points: u32,
    /// Sweep type
    pub sweep_type: PnoiseSweepType,
    /// Maximum sideband index for noise folding
    pub max_sideband: i32,
    /// Output node name
    pub output_node: String,
    /// Reference node (ground if empty)
    pub output_ref: String,
    /// Input source name (used for input-referred conversion)
    pub input_source: String,
    /// Noise reference type
    pub noise_ref: NoiseReferenceType,
    /// Include spot noise
    pub spot_noise: bool,
    /// Include integrated noise
    pub integrated_noise: bool,
    /// Noise summary (per-device contributions)
    pub noise_summary: bool,
    /// Fundamental frequency from PSS (set from PSS result)
    pub fundamental_freq: f64,
}

impl Default for PnoiseConfig {
    fn default() -> Self {
        Self {
            start_freq: 1.0, // 1 Hz (for phase noise)
            stop_freq: 1e6,  // 1 MHz
            num_points: 10,  // 10 per decade
            sweep_type: PnoiseSweepType::Decade,
            max_sideband: 5, // Sidebands to fold
            output_node: "VOUT".to_string(),
            output_ref: String::new(),
            input_source: "VIN".to_string(),
            noise_ref: NoiseReferenceType::Output,
            spot_noise: true,
            integrated_noise: false,
            noise_summary: true,
            fundamental_freq: 0.0, // Set from PSS
        }
    }
}

impl PnoiseConfig {
    /// Create new PNoise config
    pub fn new(start: f64, stop: f64, points: u32) -> Self {
        Self {
            start_freq: start,
            stop_freq: stop,
            num_points: points,
            ..Default::default()
        }
    }

    /// Create config for phase noise analysis
    pub fn phase_noise(offset_start: f64, offset_stop: f64) -> Self {
        Self {
            start_freq: offset_start,
            stop_freq: offset_stop,
            num_points: 10,
            sweep_type: PnoiseSweepType::Decade,
            noise_ref: NoiseReferenceType::Phase,
            spot_noise: true,
            ..Default::default()
        }
    }

    /// Set output node
    pub fn with_output(mut self, node: &str) -> Self {
        self.output_node = node.to_uppercase();
        self
    }

    /// Set noise reference type
    pub fn with_noise_ref(mut self, noise_ref: NoiseReferenceType) -> Self {
        self.noise_ref = noise_ref;
        self
    }

    /// Set input source
    pub fn with_input(mut self, source: &str) -> Self {
        self.input_source = source.to_uppercase();
        self
    }

    /// Set sideband range
    pub fn with_sidebands(mut self, max: i32) -> Self {
        self.max_sideband = max.abs();
        self
    }

    /// Set sweep type
    pub fn with_sweep_type(mut self, sweep_type: PnoiseSweepType) -> Self {
        self.sweep_type = sweep_type;
        self
    }

    /// Enable integrated noise calculation
    pub fn with_integrated_noise(mut self, enable: bool) -> Self {
        self.integrated_noise = enable;
        self
    }

    /// Set fundamental frequency
    pub fn with_fundamental(mut self, freq: f64) -> Self {
        self.fundamental_freq = freq;
        self
    }

    /// Total number of frequency points
    pub fn total_points(&self) -> u32 {
        match self.sweep_type {
            PnoiseSweepType::Decade => {
                if self.start_freq <= 0.0 || self.stop_freq <= 0.0 {
                    return self.num_points;
                }
                let decades = (self.stop_freq / self.start_freq).log10();
                (decades * self.num_points as f64).ceil() as u32 + 1
            }
            PnoiseSweepType::Octave => {
                if self.start_freq <= 0.0 || self.stop_freq <= 0.0 {
                    return self.num_points;
                }
                let octaves = (self.stop_freq / self.start_freq).log2();
                (octaves * self.num_points as f64).ceil() as u32 + 1
            }
            PnoiseSweepType::Linear => self.num_points,
        }
    }

    /// Generate SPICE directive
    pub fn to_spice(&self) -> String {
        let mut cmd = format!(
            ".pnoise {} {} {} {}",
            self.sweep_type.spice_keyword(),
            self.num_points,
            format_freq(self.start_freq),
            format_freq(self.stop_freq)
        );

        if !self.output_node.is_empty() {
            if self.output_ref.is_empty() {
                cmd.push_str(&format!(" output={}", self.output_node));
            } else {
                cmd.push_str(&format!(
                    " output=({},{})",
                    self.output_node, self.output_ref
                ));
            }
        }

        if self.noise_ref == NoiseReferenceType::Input && !self.input_source.trim().is_empty() {
            cmd.push_str(&format!(" input={}", self.input_source.trim()));
        }

        cmd.push_str(&format!(" maxsideband={}", self.max_sideband));

        if self.noise_ref != NoiseReferenceType::Output {
            cmd.push_str(&format!(" noiseref={}", self.noise_ref.spice_keyword()));
        }

        if self.integrated_noise {
            cmd.push_str(" integratedNoise=yes");
        }

        if self.noise_summary {
            cmd.push_str(" noiseSummary=yes");
        }

        cmd
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

        if self.max_sideband < 0 {
            return Err("Maximum sideband must be non-negative".to_string());
        }

        if self.output_node.is_empty() {
            return Err("Output node must be specified".to_string());
        }
        if self.noise_ref == NoiseReferenceType::Input && self.input_source.trim().is_empty() {
            return Err("Input source must be specified for input-referred noise".to_string());
        }

        Ok(())
    }

    /// Reset to defaults
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

// =============================================================================
// Dialog State
// =============================================================================

/// Dialog state with string buffers
#[derive(Debug, Clone, Default)]
pub struct PnoiseDialogState {
    /// Start frequency buffer
    pub start_freq: String,
    /// Stop frequency buffer
    pub stop_freq: String,
    /// Points buffer
    pub num_points: String,
    /// Sweep type index
    pub sweep_type_idx: usize,
    /// Max sideband buffer
    pub max_sideband: String,
    /// Output node buffer
    pub output_node: String,
    /// Output reference buffer
    pub output_ref: String,
    /// Input source buffer
    pub input_source: String,
    /// Noise reference type index
    pub noise_ref_idx: usize,
    /// Spot noise enabled
    pub spot_noise: bool,
    /// Integrated noise enabled
    pub integrated_noise: bool,
    /// Noise summary enabled
    pub noise_summary: bool,
    /// Initialized flag
    pub initialized: bool,
}

impl PnoiseDialogState {
    /// Initialize from config
    pub fn from_config(config: &PnoiseConfig) -> Self {
        Self {
            start_freq: format_freq(config.start_freq),
            stop_freq: format_freq(config.stop_freq),
            num_points: config.num_points.to_string(),
            sweep_type_idx: match config.sweep_type {
                PnoiseSweepType::Decade => 0,
                PnoiseSweepType::Octave => 1,
                PnoiseSweepType::Linear => 2,
            },
            max_sideband: config.max_sideband.to_string(),
            output_node: config.output_node.clone(),
            output_ref: config.output_ref.clone(),
            input_source: config.input_source.clone(),
            noise_ref_idx: match config.noise_ref {
                NoiseReferenceType::Output => 0,
                NoiseReferenceType::Input => 1,
                NoiseReferenceType::Phase => 2,
            },
            spot_noise: config.spot_noise,
            integrated_noise: config.integrated_noise,
            noise_summary: config.noise_summary,
            initialized: true,
        }
    }

    /// Convert to config
    pub fn to_config(&self) -> Result<PnoiseConfig, String> {
        let start = parse_si_value(&self.start_freq)
            .map_err(|e| format!("Invalid start frequency: {}", e))?;

        let stop = parse_si_value(&self.stop_freq)
            .map_err(|e| format!("Invalid stop frequency: {}", e))?;

        let points: u32 = self.num_points.parse().map_err(|_| "Invalid point count")?;

        let max_sb: i32 = self.max_sideband.parse().map_err(|_| "Invalid sideband")?;

        let sweep_type = match self.sweep_type_idx {
            0 => PnoiseSweepType::Decade,
            1 => PnoiseSweepType::Octave,
            _ => PnoiseSweepType::Linear,
        };

        let noise_ref = match self.noise_ref_idx {
            0 => NoiseReferenceType::Output,
            1 => NoiseReferenceType::Input,
            _ => NoiseReferenceType::Phase,
        };

        let config = PnoiseConfig {
            start_freq: start,
            stop_freq: stop,
            num_points: points,
            sweep_type,
            max_sideband: max_sb,
            output_node: self.output_node.clone(),
            output_ref: self.output_ref.clone(),
            input_source: self.input_source.clone(),
            noise_ref,
            spot_noise: self.spot_noise,
            integrated_noise: self.integrated_noise,
            noise_summary: self.noise_summary,
            fundamental_freq: 0.0,
        };

        config.validate()?;
        Ok(config)
    }

    /// Ensure initialized
    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&PnoiseConfig::default());
        }
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

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
