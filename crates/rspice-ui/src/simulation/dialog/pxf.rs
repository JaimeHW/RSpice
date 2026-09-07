//! Periodic Transfer Function (PXF) Analysis Configuration
//!
//! Configuration for periodic transfer function analysis around a PSS operating point.
//! PXF computes the transfer function from input to output including frequency
//! conversion effects.
//!
//! # Commercial Features (Spectre-Compatible)
//!
//! - Sideband-to-sideband transfer functions
//! - Input/output node specification
//! - Frequency sweep with multiple sweep types
//!
//! # Example SPICE Output
//!
//! ```text
//! .pxf dec 10 1k 1G
//! + out=VOUT outsideband=1 input=VIN
//! ```
//!
//! `.PXF` is a card the engine's netlist parser owns
//! (`rspice-core/src/netlist/parser/periodic_cards.rs`), and the grammar below
//! is that card's grammar. Its full key set is
//! `INPUT= OUT= [INPUTSIDEBAND=1] [OUTSIDEBAND=1] [MAXSIDEBAND=5] [RELTOL=1e-3]
//! [ABSTOL=1e-12] [FROM=PSS|HB]`; this form writes the first five and
//! deliberately writes none of the last three:
//!
//! - `RELTOL=`/`ABSTOL=` are the deck's numerical contract, and the Solver
//!   options channel ([`super::options`]) owns it deck-wide through
//!   `.options`. `PacConfig`, whose card carries the identical pair at the
//!   identical defaults, does not offer them either, and the manual-deck
//!   reader falls back to the deck's own `.options` for both — a sharper
//!   answer than a per-card constant. Two places to set one tolerance is a
//!   contradiction waiting to be authored.
//! - `FROM=` selects the carrier to linearize about. No periodic form offers
//!   it, because the Studio binds a dependent analysis to the one PSS it runs
//!   for it; `FROM=HB` names a binding this pipeline does not have, so a
//!   control for it would write a card the Studio's own reader refuses.
//!
//! The output probe is spelled `out=` because the engine spells it that way,
//! so the whole periodic family reads one way.

use super::options::parse_si_value;

// =============================================================================
// PXF Sweep Type
// =============================================================================

/// Type of frequency sweep for PXF analysis
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PxfSweepType {
    /// Decades (logarithmic)
    #[default]
    Decade,
    /// Octaves (logarithmic)
    Octave,
    /// Linear
    Linear,
}

impl PxfSweepType {
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
// PXF Configuration
// =============================================================================

/// Periodic transfer function (PXF) analysis configuration
#[derive(Debug, Clone)]
pub struct PxfConfig {
    /// Start frequency (Hz)
    pub start_freq: f64,
    /// Stop frequency (Hz)
    pub stop_freq: f64,
    /// Number of points
    pub num_points: u32,
    /// Sweep type
    pub sweep_type: PxfSweepType,
    /// Output node name
    pub output_node: String,
    /// Output reference node
    pub output_ref: String,
    /// Output sideband index
    pub output_sideband: i32,
    /// Input source name
    pub input_source: String,
    /// Input sideband index.
    ///
    /// A PXF measures a transfer between a *pair* of sidebands — H(m -> n) and
    /// H(n -> m) are different mixer paths, not one number read two ways — and
    /// this is the half the form used to leave unsaid. It was pinned to a
    /// literal `1` where the run configuration was built, so a mixer's
    /// down-conversion path H(0 -> n) was not authorable from the Studio at
    /// all while its up-conversion path was.
    pub input_sideband: i32,
    /// Maximum sideband index
    pub max_sideband: i32,
}

impl Default for PxfConfig {
    fn default() -> Self {
        Self {
            start_freq: 1e3,
            stop_freq: 1e9,
            num_points: 10,
            sweep_type: PxfSweepType::Decade,
            output_node: "VOUT".to_string(),
            output_ref: String::new(),
            output_sideband: 1,
            input_source: "VIN".to_string(),
            // `PxfCard::DEFAULT_INPUT_SIDEBAND`, and the value the manual-deck
            // reader has applied to a line that omits the key since it became
            // authorable, so a form that has never been touched writes the
            // analysis it always ran.
            input_sideband: 1,
            max_sideband: 5,
        }
    }
}

impl PxfConfig {
    /// Generate SPICE directive
    pub fn to_spice(&self) -> String {
        let mut cmd = format!(
            ".pxf {} {} {} {}",
            self.sweep_type.spice_keyword(),
            self.num_points,
            format_freq(self.start_freq),
            format_freq(self.stop_freq)
        );

        if !self.output_node.is_empty() {
            if self.output_ref.is_empty() {
                cmd.push_str(&format!(" out={}", self.output_node));
            } else {
                cmd.push_str(&format!(" out=V({},{})", self.output_node, self.output_ref));
            }
        }

        cmd.push_str(&format!(" outsideband={}", self.output_sideband));

        if !self.input_source.is_empty() {
            cmd.push_str(&format!(" input={}", self.input_source));
        }

        // Beside the source it belongs to, so the line names each end of the
        // transfer with the sideband it is measured at.
        cmd.push_str(&format!(" inputsideband={}", self.input_sideband));

        cmd.push_str(&format!(" maxsideband={}", self.max_sideband));

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
        if self.output_node.is_empty() {
            return Err("Output node must be specified".to_string());
        }
        if self.input_source.is_empty() {
            return Err("Input source must be specified".to_string());
        }
        if self.max_sideband < 0 {
            return Err("Maximum sideband must be non-negative".to_string());
        }
        // The card refuses a conversion depth it then measures outside of, and
        // so does the manual-deck reader. Without this the form could write a
        // line the engine will not read: a transfer between two sidebands the
        // solve it asked for does not contain.
        for (label, sideband) in [
            ("Input", self.input_sideband),
            ("Output", self.output_sideband),
        ] {
            if sideband.saturating_abs() > self.max_sideband {
                return Err(format!(
                    "{label} sideband {sideband} lies outside the maximum sideband {}",
                    self.max_sideband
                ));
            }
        }
        Ok(())
    }
}

// =============================================================================
// Dialog State
// =============================================================================

/// Dialog state with string buffers
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PxfDialogState {
    pub start_freq: String,
    pub stop_freq: String,
    pub num_points: String,
    pub sweep_type_idx: usize,
    pub output_node: String,
    pub output_ref: String,
    pub output_sideband: String,
    pub input_source: String,
    /// Absent in every project saved before the input side of the transfer was
    /// authorable, and absent means `1` — the value those projects ran at,
    /// pinned in the run-configuration builder and defaulted by both readers.
    /// Reopening one must not silently move it to sideband zero.
    #[serde(default = "default_input_sideband_text")]
    pub input_sideband: String,
    pub max_sideband: String,
    #[serde(skip)]
    pub initialized: bool,
}

fn default_input_sideband_text() -> String {
    PxfConfig::default().input_sideband.to_string()
}

impl PxfDialogState {
    /// Initialize from config
    pub fn from_config(config: &PxfConfig) -> Self {
        Self {
            start_freq: format_freq(config.start_freq),
            stop_freq: format_freq(config.stop_freq),
            num_points: config.num_points.to_string(),
            sweep_type_idx: match config.sweep_type {
                PxfSweepType::Decade => 0,
                PxfSweepType::Octave => 1,
                PxfSweepType::Linear => 2,
            },
            output_node: config.output_node.clone(),
            output_ref: config.output_ref.clone(),
            output_sideband: config.output_sideband.to_string(),
            input_source: config.input_source.clone(),
            input_sideband: config.input_sideband.to_string(),
            max_sideband: config.max_sideband.to_string(),
            initialized: true,
        }
    }

    /// Convert to config
    pub fn to_config(&self) -> Result<PxfConfig, String> {
        let start = parse_si_value(&self.start_freq)
            .map_err(|e| format!("Invalid start frequency: {}", e))?;
        let stop = parse_si_value(&self.stop_freq)
            .map_err(|e| format!("Invalid stop frequency: {}", e))?;
        let points: u32 = self.num_points.parse().map_err(|_| "Invalid point count")?;
        let out_sb: i32 = self
            .output_sideband
            .parse()
            .map_err(|_| "Invalid output sideband")?;
        let in_sb: i32 = self
            .input_sideband
            .parse()
            .map_err(|_| "Invalid input sideband")?;
        let max_sb: i32 = self
            .max_sideband
            .parse()
            .map_err(|_| "Invalid max sideband")?;

        let sweep_type = match self.sweep_type_idx {
            0 => PxfSweepType::Decade,
            1 => PxfSweepType::Octave,
            _ => PxfSweepType::Linear,
        };

        let config = PxfConfig {
            start_freq: start,
            stop_freq: stop,
            num_points: points,
            sweep_type,
            output_node: self.output_node.clone(),
            output_ref: self.output_ref.clone(),
            output_sideband: out_sb,
            input_source: self.input_source.clone(),
            input_sideband: in_sb,
            max_sideband: max_sb,
        };

        config.validate()?;
        Ok(config)
    }

    /// Ensure initialized
    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&PxfConfig::default());
        }
    }
}

// =============================================================================
// Helper
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
