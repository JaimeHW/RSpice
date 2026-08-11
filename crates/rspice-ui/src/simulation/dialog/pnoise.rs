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
    /// SPICE keyword
    pub fn spice_keyword(&self) -> &'static str {
        match self {
            Self::Decade => "dec",
            Self::Octave => "oct",
            Self::Linear => "lin",
        }
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
    /// SPICE keyword
    pub fn spice_keyword(&self) -> &'static str {
        match self {
            Self::Output => "output",
            Self::Input => "input",
            Self::Phase => "phase",
        }
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
    /// Include integrated noise
    pub integrated_noise: bool,
    /// Noise summary (per-device contributions)
    pub noise_summary: bool,
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
            integrated_noise: false,
            noise_summary: true,
        }
    }
}

impl PnoiseConfig {
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
}

// =============================================================================
// Dialog State
// =============================================================================

/// Dialog state with string buffers
#[derive(Debug, Clone, Default, serde::Serialize)]
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
    /// Integrated noise enabled
    pub integrated_noise: bool,
    /// Noise summary enabled
    pub noise_summary: bool,
    /// Initialized flag
    #[serde(skip)]
    pub initialized: bool,
}

/// Persisted editor state. New fields serialize; retired fields only decode.
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct PersistedPnoiseDialogState {
    #[serde(default)]
    start_freq: String,
    #[serde(default)]
    stop_freq: String,
    #[serde(default)]
    num_points: String,
    #[serde(default)]
    sweep_type_idx: usize,
    #[serde(default)]
    max_sideband: String,
    #[serde(default)]
    output_node: String,
    #[serde(default)]
    output_ref: String,
    #[serde(default)]
    input_source: String,
    #[serde(default)]
    noise_ref_idx: usize,
    #[serde(default)]
    integrated_noise: bool,
    #[serde(default)]
    noise_summary: bool,
    /// Retired. The sweep always produces the per-frequency spectrum, so this
    /// selected nothing. Accepted so earlier projects still open.
    #[serde(default)]
    #[allow(dead_code)]
    spot_noise: serde::de::IgnoredAny,
}

impl<'de> serde::Deserialize<'de> for PnoiseDialogState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let persisted = PersistedPnoiseDialogState::deserialize(deserializer)?;
        Ok(Self {
            start_freq: persisted.start_freq,
            stop_freq: persisted.stop_freq,
            num_points: persisted.num_points,
            sweep_type_idx: persisted.sweep_type_idx,
            max_sideband: persisted.max_sideband,
            output_node: persisted.output_node,
            output_ref: persisted.output_ref,
            input_source: persisted.input_source,
            noise_ref_idx: persisted.noise_ref_idx,
            integrated_noise: persisted.integrated_noise,
            noise_summary: persisted.noise_summary,
            initialized: false,
        })
    }
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
            integrated_noise: self.integrated_noise,
            noise_summary: self.noise_summary,
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
