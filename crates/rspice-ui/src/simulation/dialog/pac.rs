//! Periodic AC (PAC) Analysis Configuration
//!
//! Configuration for periodic AC analysis around a PSS operating point.
//! PAC computes the frequency conversion (mixing) behavior of periodically
//! operating circuits like mixers, samplers, and switched-capacitor filters.
//!
//! # Commercial Features (Spectre-Compatible)
//!
//! - Sideband range specification
//! - Input source and output node configuration
//! - Frequency sweep with decade/linear options
//! - Conversion matrix results
//!
//! # Example SPICE Output
//!
//! ```text
//! .pac dec 10 1k 1G
//! + maxsideband=5 input=VRF out=VOUT
//! ```

use crate::services::simulation_runner::PeriodicCarrier;

use super::options::parse_si_value;

// =============================================================================
// PAC Sweep Type
// =============================================================================

/// Type of frequency sweep for PAC analysis
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PacSweepType {
    /// Decades (logarithmic)
    #[default]
    Decade,
    /// Octaves (logarithmic)
    Octave,
    /// Linear
    Linear,
}

impl PacSweepType {
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
// PAC Configuration
// =============================================================================

/// Periodic AC (PAC) analysis configuration
///
/// Commercial-grade configuration matching Cadence Spectre PAC parameters.
#[derive(Debug, Clone)]
pub struct PacConfig {
    /// Start frequency (Hz)
    pub start_freq: f64,
    /// Stop frequency (Hz)
    pub stop_freq: f64,
    /// Number of points (per decade for log, total for linear)
    pub num_points: u32,
    /// Sweep type
    pub sweep_type: PacSweepType,
    /// Maximum sideband index (both positive and negative)
    pub max_sideband: i32,
    /// Input source name (AC stimulus)
    pub input_source: String,
    /// Output node name
    pub output_node: String,
    /// Reference node (ground if empty)
    pub output_ref: String,
    /// PAC magnitude (input signal magnitude)
    pub pac_magnitude: f64,
    /// Include DC sideband (band 0)
    pub include_dc: bool,
    /// Which periodic solve this analysis linearizes around.
    ///
    /// The card's `FROM=` keyword, and the engine's own three positions: the
    /// preceding periodic solve of either family, which is what an absent key
    /// means, the preceding `.PSS`, or the preceding `.HB`.
    pub carrier: PeriodicCarrier,
}

impl Default for PacConfig {
    fn default() -> Self {
        Self {
            start_freq: 1e3, // 1 kHz
            stop_freq: 1e9,  // 1 GHz
            num_points: 10,  // 10 per decade
            sweep_type: PacSweepType::Decade,
            max_sideband: 5, // Sidebands -5 to +5
            input_source: "VRF".to_string(),
            output_node: "VOUT".to_string(),
            output_ref: String::new(),
            pac_magnitude: 1.0, // 1V default
            include_dc: true,
            carrier: PeriodicCarrier::Preceding,
        }
    }
}

impl PacConfig {
    /// What the engine's `.PAC` card means by an unwritten `PACMAG=`.
    pub(crate) const CARD_DEFAULT_PAC_MAGNITUDE: f64 = 1.0;
    /// What the engine's `.PAC` card means by an unwritten `INCLUDEDC=`.
    pub(crate) const CARD_DEFAULT_INCLUDE_DC: bool = true;

    /// The `.PAC` card the engine reads, in the engine's own grammar.
    ///
    /// `rspice-core/src/netlist/parser/periodic_cards.rs::parse_pac_command`
    /// reads `.PAC DEC|LIN|OCT np fstart fstop KEY=VALUE ...` and refuses any
    /// keyword outside `INPUT`, `OUT`, `MAXSIDEBAND`, `SIDEBANDMIN`,
    /// `SIDEBANDMAX`, `RELTOL`, `ABSTOL`, `PACMAG`, `INCLUDEDC` and `FROM`.
    /// The output probe is `OUT=`, spelled `V(node)` or `V(node,ref)` or a
    /// bare node name.
    ///
    /// `pacmag=` and `includedc=` are written only where they differ from the
    /// card's own defaults, which are these defaults: a statement carries what
    /// the run does differently, not a restatement of every default.
    pub fn to_spice(&self) -> String {
        let mut cmd = format!(
            ".pac {} {} {} {}",
            self.sweep_type.spice_keyword(),
            self.num_points,
            format_freq(self.start_freq),
            format_freq(self.stop_freq)
        );

        cmd.push_str(&format!(" maxsideband={}", self.max_sideband));

        if !self.input_source.is_empty() {
            cmd.push_str(&format!(" input={}", self.input_source));
        }

        if !self.output_node.is_empty() {
            if self.output_ref.is_empty() {
                cmd.push_str(&format!(" out={}", self.output_node));
            } else {
                cmd.push_str(&format!(" out=V({},{})", self.output_node, self.output_ref));
            }
        }

        if self.pac_magnitude != Self::CARD_DEFAULT_PAC_MAGNITUDE {
            cmd.push_str(&format!(" pacmag={}", self.pac_magnitude));
        }

        if self.include_dc != Self::CARD_DEFAULT_INCLUDE_DC {
            cmd.push_str(" includedc=no");
        }

        // `FROM=` has no spelling for "the preceding periodic solve": the
        // card's absent key *is* that selection, which is why the carrier is
        // held as a three-position choice and not as a pair of names. An
        // untouched form therefore writes exactly the card it always wrote.
        if let Some(carrier) = self.carrier.spice_name() {
            cmd.push_str(&format!(" from={carrier}"));
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

        if self.input_source.is_empty() {
            return Err("Input source must be specified".to_string());
        }

        if self.output_node.is_empty() {
            return Err("Output node must be specified".to_string());
        }

        if self.pac_magnitude <= 0.0 {
            return Err("PAC magnitude must be positive".to_string());
        }

        // The engine accepts all three carriers on this card and the Studio
        // runs two of them, so the third is refused here with the reason and
        // the place it does run. The chooser paints it disabled for the same
        // reason; this is what answers a project or a deck that names it.
        if let Some(reason) = self.carrier.unroutable_reason(".PAC") {
            return Err(reason);
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
pub struct PacDialogState {
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
    /// Input source buffer
    pub input_source: String,
    /// Output node buffer
    pub output_node: String,
    /// Output reference buffer
    pub output_ref: String,
    /// PAC magnitude buffer
    pub pac_magnitude: String,
    /// Include DC
    pub include_dc: bool,
    /// Carrier chooser position, into [`PeriodicCarrier::ALL`].
    ///
    /// Absent in every project saved before the carrier was authorable, and
    /// absent is position zero — the preceding periodic solve, which is the
    /// card those projects wrote and the binding they ran.
    #[serde(default)]
    pub carrier_idx: usize,
    /// Initialized flag
    #[serde(skip)]
    pub initialized: bool,
}

impl PacDialogState {
    /// Initialize from config
    pub fn from_config(config: &PacConfig) -> Self {
        Self {
            start_freq: format_freq(config.start_freq),
            stop_freq: format_freq(config.stop_freq),
            num_points: config.num_points.to_string(),
            sweep_type_idx: match config.sweep_type {
                PacSweepType::Decade => 0,
                PacSweepType::Octave => 1,
                PacSweepType::Linear => 2,
            },
            max_sideband: config.max_sideband.to_string(),
            input_source: config.input_source.clone(),
            output_node: config.output_node.clone(),
            output_ref: config.output_ref.clone(),
            pac_magnitude: config.pac_magnitude.to_string(),
            include_dc: config.include_dc,
            carrier_idx: config.carrier.index(),
            initialized: true,
        }
    }

    /// Convert to config
    pub fn to_config(&self) -> Result<PacConfig, String> {
        let start = parse_si_value(&self.start_freq)
            .map_err(|e| format!("Invalid start frequency: {}", e))?;

        let stop = parse_si_value(&self.stop_freq)
            .map_err(|e| format!("Invalid stop frequency: {}", e))?;

        let points: u32 = self.num_points.parse().map_err(|_| "Invalid point count")?;

        let max_sb: i32 = self.max_sideband.parse().map_err(|_| "Invalid sideband")?;

        let mag: f64 = self
            .pac_magnitude
            .parse()
            .map_err(|_| "Invalid magnitude")?;

        let sweep_type = match self.sweep_type_idx {
            0 => PacSweepType::Decade,
            1 => PacSweepType::Octave,
            _ => PacSweepType::Linear,
        };

        let config = PacConfig {
            start_freq: start,
            stop_freq: stop,
            num_points: points,
            sweep_type,
            max_sideband: max_sb,
            input_source: self.input_source.clone(),
            output_node: self.output_node.clone(),
            output_ref: self.output_ref.clone(),
            pac_magnitude: mag,
            include_dc: self.include_dc,
            carrier: PeriodicCarrier::at(self.carrier_idx),
        };

        config.validate()?;
        Ok(config)
    }

    /// Ensure initialized
    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&PacConfig::default());
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The exact card, in the engine's spelling.
    ///
    /// `directive_parse_ratchet` proves the engine reads whatever this writes.
    /// This pins which card was written — in particular `out=`, which is the
    /// keyword the engine's `.PAC` arm knows. The `output=` this used to write
    /// parsed for months as a card no parser owned, and then as one the parser
    /// refused outright.
    #[test]
    fn the_card_spells_its_output_probe_the_way_the_engine_reads_it() {
        assert_eq!(
            PacConfig::default().to_spice(),
            ".pac dec 10 1k 1G maxsideband=5 input=VRF out=VOUT"
        );
        assert_eq!(
            PacConfig {
                output_ref: "REF".to_owned(),
                ..PacConfig::default()
            }
            .to_spice(),
            ".pac dec 10 1k 1G maxsideband=5 input=VRF out=V(VOUT,REF)"
        );
    }

    /// The drive amplitude and the sideband-zero choice appear only when they
    /// differ from what the card means without them.
    #[test]
    fn the_card_states_the_drive_and_the_sideband_zero_choice_only_when_they_differ() {
        assert_eq!(
            PacConfig {
                pac_magnitude: 0.05,
                include_dc: false,
                ..PacConfig::default()
            }
            .to_spice(),
            ".pac dec 10 1k 1G maxsideband=5 input=VRF out=VOUT pacmag=0.05 includedc=no"
        );
        assert_eq!(
            PacConfig {
                pac_magnitude: PacConfig::CARD_DEFAULT_PAC_MAGNITUDE,
                include_dc: PacConfig::CARD_DEFAULT_INCLUDE_DC,
                ..PacConfig::default()
            }
            .to_spice(),
            PacConfig::default().to_spice()
        );
    }

    /// The carrier the analysis linearizes around appears on the card, in the
    /// engine's own keyword, and only when it is named.
    ///
    /// The absent `FROM=` is not a missing setting: `resolve_periodic_source`
    /// binds a card without it to the nearest preceding `.PSS` *or* `.HB`,
    /// which is a third answer rather than a default spelling of either. So an
    /// untouched form writes the card it has always written, and the two named
    /// positions write the engine's two spellings.
    #[test]
    fn the_card_names_its_carrier_only_when_one_is_named() {
        assert!(
            !PacConfig::default().to_spice().contains("from="),
            "the preceding periodic solve is the absent keyword, not a written one"
        );
        assert_eq!(
            PacConfig {
                carrier: PeriodicCarrier::Pss,
                ..PacConfig::default()
            }
            .to_spice(),
            ".pac dec 10 1k 1G maxsideband=5 input=VRF out=VOUT from=pss"
        );
        assert_eq!(
            PacConfig {
                carrier: PeriodicCarrier::Hb,
                ..PacConfig::default()
            }
            .to_spice(),
            ".pac dec 10 1k 1G maxsideband=5 input=VRF out=VOUT from=hb"
        );
    }

    /// A carrier the Studio cannot run is refused by name, with the place it
    /// does run, rather than bound to whichever periodic state is at hand.
    #[test]
    fn a_carrier_without_a_studio_route_is_refused_by_name() {
        let error = PacConfig {
            carrier: PeriodicCarrier::Hb,
            ..PacConfig::default()
        }
        .validate()
        .expect_err("a harmonic-balance carrier has no PAC runner in this crate");
        assert!(
            error.contains("from=hb") && error.contains("command line"),
            "the refusal must name the carrier and where it runs: {error}"
        );
        for carrier in [PeriodicCarrier::Preceding, PeriodicCarrier::Pss] {
            PacConfig {
                carrier,
                ..PacConfig::default()
            }
            .validate()
            .unwrap_or_else(|error| panic!("{carrier:?} is routable here: {error}"));
        }
    }

    /// A draft saved before the carrier row existed opens as the analysis it
    /// ran: the preceding periodic solve, writing no `FROM=` at all.
    #[test]
    fn a_draft_saved_before_the_carrier_selector_restores_as_preceding() {
        let mut document = serde_json::to_value(PacDialogState::from_config(&PacConfig::default()))
            .expect("the draft serializes");
        document
            .as_object_mut()
            .expect("the draft is an object")
            .remove("carrier_idx")
            .expect("the key this test removes must exist");
        let restored: PacDialogState =
            serde_json::from_value(document).expect("a draft written before the carrier row loads");
        assert_eq!(restored.carrier_idx, 0);
        let config = restored
            .to_config()
            .expect("the restored draft is runnable");
        assert_eq!(config.carrier, PeriodicCarrier::Preceding);
        assert!(!config.to_spice().contains("from="));
    }
}
