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
//! + out=VOUT maxsideband=5
//! ```

use crate::services::simulation_runner::PeriodicCarrier;

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

// =============================================================================
// PNoise Configuration
// =============================================================================

/// Periodic noise (PNoise) analysis configuration
///
/// Commercial-grade configuration matching Cadence Spectre PNoise parameters.
#[derive(Debug, Clone)]
pub struct PnoiseConfig {
    pub input_sideband: i32,
    pub output_sideband: i32,
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
    /// Which periodic solve this analysis folds noise around; `FROM=`.
    pub carrier: PeriodicCarrier,
}

impl Default for PnoiseConfig {
    fn default() -> Self {
        Self {
            input_sideband: 0,
            output_sideband: 0,
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
            carrier: PeriodicCarrier::Preceding,
        }
    }
}

impl PnoiseConfig {
    /// What the engine's `.PNOISE` card means by an unwritten
    /// `INTEGRATEDNOISE=`.
    pub(crate) const CARD_DEFAULT_INTEGRATED_NOISE: bool = false;
    /// What the engine's `.PNOISE` card means by an unwritten
    /// `NOISESUMMARY=`.
    pub(crate) const CARD_DEFAULT_NOISE_SUMMARY: bool = true;

    /// The `.PNOISE` card the engine reads, in the engine's own grammar.
    ///
    /// `rspice-core/src/netlist/parser/periodic_cards.rs::parse_pnoise_command`
    /// reads `.PNOISE DEC|LIN|OCT np fstart fstop KEY=VALUE ...` and refuses
    /// any keyword outside `OUT`, `INPUT`, `MAXSIDEBAND`, `INPUTSIDEBAND`, `OUTSIDEBAND`, `NOISEREF`,
    /// `INTEGRATEDNOISE`, `NOISESUMMARY` and `FROM`. `OUT=` is the output
    /// probe, spelled `V(node)`, `V(node,ref)` or a bare node name.
    ///
    /// `INPUT=` alone means input-referred noise, so `noiseref=input` beside
    /// it would restate what the source already said and the card refuses the
    /// pair as redundant only when they disagree; the writer states the source
    /// and stops. `noiseref=phase` has no such shorthand and is written
    /// outright. `integratednoise=` and `noisesummary=` appear only where they
    /// differ from what the card means without them.
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
                cmd.push_str(&format!(" out={}", self.output_node));
            } else {
                cmd.push_str(&format!(" out=V({},{})", self.output_node, self.output_ref));
            }
        }

        if self.noise_ref == NoiseReferenceType::Input && !self.input_source.trim().is_empty() {
            cmd.push_str(&format!(" input={}", self.input_source.trim()));
        }

        if self.noise_ref == NoiseReferenceType::Phase {
            cmd.push_str(" noiseref=phase");
        }

        cmd.push_str(&format!(" maxsideband={}", self.max_sideband));
        if self.noise_ref == NoiseReferenceType::Input && self.input_sideband != 0 {
            cmd.push_str(&format!(" inputsideband={}", self.input_sideband));
        }
        if self.noise_ref != NoiseReferenceType::Phase && self.output_sideband != 0 {
            cmd.push_str(&format!(" outsideband={}", self.output_sideband));
        }

        if self.integrated_noise != Self::CARD_DEFAULT_INTEGRATED_NOISE {
            cmd.push_str(" integratednoise=yes");
        }

        if self.noise_summary != Self::CARD_DEFAULT_NOISE_SUMMARY {
            cmd.push_str(" noisesummary=no");
        }

        // The absent key is the "preceding periodic solve" position, so an
        // untouched form writes the card it always wrote.
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
        if self.integrated_noise
            && self.sweep_type == PnoiseSweepType::Linear
            && self.num_points == 1
        {
            return Err("Integrated noise requires at least two distinct frequency points".into());
        }

        crate::services::simulation_runner::validate_noise_sidebands(
            self.input_sideband,
            self.output_sideband,
            usize::try_from(self.max_sideband)
                .map_err(|_| "Maximum sideband must be nonnegative")?,
        )?;
        if (self.noise_ref == NoiseReferenceType::Phase && self.output_sideband != 0)
            || (self.noise_ref != NoiseReferenceType::Input && self.input_sideband != 0)
        {
            return Err("Conversion sidebands apply to driven noise; input sideband requires input-referred noise".into());
        }

        if self.output_node.is_empty() {
            return Err("Output node must be specified".to_string());
        }
        if self.noise_ref == NoiseReferenceType::Input && self.input_source.trim().is_empty() {
            return Err("Input source must be specified for input-referred noise".to_string());
        }
        // The carrier is not validated here: it is a *dependency*, and the
        // plan owns it. `AnalysisDraft::prerequisite_roles` turns this
        // position into the prerequisite family this request requires, and
        // `dependency_configuration_issue` asks whether the bound carrier can
        // answer the measurement — which is the question a form field cannot
        // ask, because the answer depends on the instance it is bound to.
        Ok(())
    }
}

// =============================================================================
// Dialog State
// =============================================================================

/// Dialog state with string buffers
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct PnoiseDialogState {
    pub input_sideband: String,
    pub output_sideband: String,
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
    /// Carrier chooser position, into [`PeriodicCarrier::ALL`].
    pub carrier_idx: usize,
    /// Initialized flag
    #[serde(skip)]
    pub initialized: bool,
}

/// Persisted editor state. New fields serialize; retired fields only decode.
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct PersistedPnoiseDialogState {
    #[serde(default = "default_sideband")]
    input_sideband: String,
    #[serde(default = "default_sideband")]
    output_sideband: String,
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
    /// Absent means position zero, the preceding periodic solve: the card
    /// every project written before this row existed wrote, and the binding it
    /// ran.
    #[serde(default)]
    carrier_idx: usize,
    /// Retired. The sweep always produces the per-frequency spectrum, so this
    /// selected nothing. Accepted so earlier projects still open.
    #[serde(default)]
    #[allow(dead_code)]
    spot_noise: serde::de::IgnoredAny,
}

fn default_sideband() -> String {
    "0".into()
}

impl<'de> serde::Deserialize<'de> for PnoiseDialogState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let persisted = PersistedPnoiseDialogState::deserialize(deserializer)?;
        Ok(Self {
            input_sideband: persisted.input_sideband,
            output_sideband: persisted.output_sideband,
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
            carrier_idx: persisted.carrier_idx,
            initialized: false,
        })
    }
}

impl PnoiseDialogState {
    /// Initialize from config
    pub fn from_config(config: &PnoiseConfig) -> Self {
        Self {
            input_sideband: config.input_sideband.to_string(),
            output_sideband: config.output_sideband.to_string(),
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
            carrier_idx: config.carrier.index(),
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
            input_sideband: if noise_ref == NoiseReferenceType::Input {
                self.input_sideband
                    .trim()
                    .parse()
                    .map_err(|_| "Input sideband must be a signed integer")?
            } else {
                0
            },
            output_sideband: if noise_ref != NoiseReferenceType::Phase {
                self.output_sideband
                    .trim()
                    .parse()
                    .map_err(|_| "Output sideband must be a signed integer")?
            } else {
                0
            },
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
            carrier: PeriodicCarrier::at(self.carrier_idx),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pnoise_sidebands_round_trip_and_inactive_controls_do_not_change_phase_noise() {
        let config = PnoiseConfig {
            input_sideband: -1,
            output_sideband: 2,
            noise_ref: NoiseReferenceType::Input,
            ..Default::default()
        };
        config.validate().unwrap();
        let draft = PnoiseDialogState::from_config(&config);
        let encoded = serde_json::to_string(&draft).unwrap();
        let mut restored: PnoiseDialogState = serde_json::from_str(&encoded).unwrap();
        let round_trip = restored.to_config().unwrap();
        assert_eq!(
            (round_trip.input_sideband, round_trip.output_sideband),
            (-1, 2)
        );
        let deck = format!(
            "PNOISE card\nV1 out 0 0\nR1 out 0 1k\n.HB 1Meg\n{}\n.end\n",
            round_trip.to_spice()
        );
        let parsed = rspice_core::Netlist::parse(&deck).unwrap();
        let rspice_core::netlist::AnalysisCommand::Pnoise(card) = &parsed.analyses[1] else {
            panic!("PNOISE card");
        };
        assert_eq!((card.input_sideband, card.output_sideband), (-1, 2));
        restored.input_sideband = "-2147483648".into();
        assert!(restored.to_config().is_err());
        restored.noise_ref_idx = 2;
        let phase = restored.to_config().unwrap();
        assert_eq!((phase.input_sideband, phase.output_sideband), (0, 0));
        assert!(!phase.to_spice().contains("inputsideband"));
        assert!(!phase.to_spice().contains("outsideband"));
        let mut old: serde_json::Value = serde_json::from_str(&encoded).unwrap();
        old.as_object_mut().unwrap().remove("input_sideband");
        old.as_object_mut().unwrap().remove("output_sideband");
        let legacy: PnoiseDialogState = serde_json::from_value(old).unwrap();
        let legacy = legacy.to_config().unwrap();
        assert_eq!((legacy.input_sideband, legacy.output_sideband), (0, 0));
        assert!(
            PnoiseConfig {
                max_sideband: 0,
                ..Default::default()
            }
            .validate()
            .is_ok()
        );
        assert!(
            PnoiseConfig {
                num_points: 1,
                sweep_type: PnoiseSweepType::Linear,
                integrated_noise: true,
                ..Default::default()
            }
            .validate()
            .is_err()
        );
    }

    /// The exact card, in the engine's spelling.
    ///
    /// The output probe is `out=`, and `INPUT=` appears only for the
    /// input-referred selection it means on the engine's card.
    #[test]
    fn the_card_states_only_what_the_engine_reads_from_it() {
        assert_eq!(
            PnoiseConfig::default().to_spice(),
            ".pnoise dec 10 1 1Meg out=VOUT maxsideband=5"
        );
        assert_eq!(
            PnoiseConfig {
                noise_ref: NoiseReferenceType::Input,
                ..PnoiseConfig::default()
            }
            .to_spice(),
            ".pnoise dec 10 1 1Meg out=VOUT input=VIN maxsideband=5"
        );
    }

    /// Phase noise is a different measurement from output-referred noise, and
    /// the card now says which one it is instead of leaving the difference to
    /// a typed option the deck cannot carry.
    #[test]
    fn the_card_states_phase_noise_outright() {
        assert_eq!(
            PnoiseConfig {
                noise_ref: NoiseReferenceType::Phase,
                ..PnoiseConfig::default()
            }
            .to_spice(),
            ".pnoise dec 10 1 1Meg out=VOUT noiseref=phase maxsideband=5"
        );
    }

    /// The reporting switches appear only when they differ from what the card
    /// means without them.
    #[test]
    fn the_card_states_the_reporting_switches_only_when_they_differ() {
        assert_eq!(
            PnoiseConfig {
                integrated_noise: true,
                noise_summary: false,
                ..PnoiseConfig::default()
            }
            .to_spice(),
            ".pnoise dec 10 1 1Meg out=VOUT maxsideband=5 integratednoise=yes noisesummary=no"
        );
        assert_eq!(
            PnoiseConfig {
                integrated_noise: PnoiseConfig::CARD_DEFAULT_INTEGRATED_NOISE,
                noise_summary: PnoiseConfig::CARD_DEFAULT_NOISE_SUMMARY,
                ..PnoiseConfig::default()
            }
            .to_spice(),
            PnoiseConfig::default().to_spice()
        );
    }

    /// The carrier appears on the card only when it is named, and in the
    /// engine's keyword when it is.
    #[test]
    fn the_card_names_its_carrier_only_when_one_is_named() {
        assert!(!PnoiseConfig::default().to_spice().contains("from="));
        assert_eq!(
            PnoiseConfig {
                carrier: PeriodicCarrier::Pss,
                ..PnoiseConfig::default()
            }
            .to_spice(),
            ".pnoise dec 10 1 1Meg out=VOUT maxsideband=5 from=pss"
        );
    }

    /// Every carrier the engine's card accepts is a carrier this form accepts;
    /// which family it requires is the plan's declared prerequisite role, and
    /// whether that carrier can answer phase noise is the plan's dependency
    /// contract rather than a field check that cannot see the instance.
    #[test]
    fn every_carrier_the_engine_accepts_is_a_carrier_this_form_accepts() {
        for carrier in PeriodicCarrier::ALL {
            PnoiseConfig {
                carrier: *carrier,
                ..PnoiseConfig::default()
            }
            .validate()
            .unwrap_or_else(|error| panic!("{carrier:?} is a carrier the engine has: {error}"));
        }
    }

    /// A draft saved before the carrier row existed opens as the analysis it
    /// ran.
    #[test]
    fn a_draft_saved_before_the_carrier_selector_restores_as_preceding() {
        let mut document =
            serde_json::to_value(PnoiseDialogState::from_config(&PnoiseConfig::default()))
                .expect("the draft serializes");
        document
            .as_object_mut()
            .expect("the draft is an object")
            .remove("carrier_idx")
            .expect("the key this test removes must exist");
        let restored: PnoiseDialogState =
            serde_json::from_value(document).expect("a draft written before the carrier row loads");
        assert_eq!(restored.carrier_idx, 0);
        assert_eq!(
            restored
                .to_config()
                .expect("the restored draft is runnable")
                .carrier,
            PeriodicCarrier::Preceding
        );
    }
}
