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
    /// Maximum sideband index (both positive and negative).
    ///
    /// The symmetric spelling, `MAXSIDEBAND=n`, which the card reads as
    /// `-n..=n`. Read only when neither end below is authored: the engine
    /// refuses `MAXSIDEBAND=` beside `SIDEBANDMIN=`/`SIDEBANDMAX=` outright
    /// (`ConflictingFields`), so the two spellings are alternatives and never
    /// a pair of overlapping settings.
    pub max_sideband: i32,
    /// Lowest output sideband index, when the range is stated asymmetrically.
    ///
    /// `None` is the unauthored field. Authoring either end selects the
    /// asymmetric spelling, and the end left unauthored takes the card's own
    /// default rather than anything derived from `max_sideband` — which is
    /// exactly what `parse_pac_command` does with a card stating one of them.
    pub sideband_min: Option<i32>,
    /// Highest output sideband index, when the range is stated asymmetrically.
    pub sideband_max: Option<i32>,
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
    /// Relative tolerance for the periodic solve, or `None` for plan policy.
    ///
    /// The card carries `RELTOL=`, and until now the Studio's Solver options
    /// channel owned it deck-wide: `periodic_solver_tolerances` forced
    /// `.options reltol` onto every periodic dependent, so an analysis that
    /// needed a tighter periodic solve than the rest of the deck had no way to
    /// ask for one. An unauthored field still takes the plan's policy, which
    /// is what every run before this authored.
    pub reltol: Option<f64>,
    /// Absolute tolerance for small-signal currents, or `None` for plan policy.
    pub abstol: Option<f64>,
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
            sideband_min: None,
            sideband_max: None,
            input_source: "VRF".to_string(),
            output_node: "VOUT".to_string(),
            output_ref: String::new(),
            pac_magnitude: 1.0, // 1V default
            include_dc: true,
            reltol: None,
            abstol: None,
            carrier: PeriodicCarrier::Preceding,
        }
    }
}

impl PacConfig {
    /// What the engine's `.PAC` card means by an unwritten `PACMAG=`.
    pub(crate) const CARD_DEFAULT_PAC_MAGNITUDE: f64 = 1.0;
    /// What the engine's `.PAC` card means by an unwritten `INCLUDEDC=`.
    pub(crate) const CARD_DEFAULT_INCLUDE_DC: bool = true;
    /// `PacCard::DEFAULT_SIDEBAND_MIN`: the lowest sideband a card that states
    /// one end of the range asymmetrically means at the other.
    pub(crate) const CARD_DEFAULT_SIDEBAND_MIN: i32 = -5;
    /// `PacCard::DEFAULT_SIDEBAND_MAX`, the same fact at the top end.
    pub(crate) const CARD_DEFAULT_SIDEBAND_MAX: i32 = 5;

    /// The range this configuration actually runs, in the engine's own rule.
    ///
    /// `parse_pac_command` resolves it in exactly two lines: a stated
    /// `MAXSIDEBAND=n` is `(-n, n)`, and otherwise each end is its own
    /// authored value or the card's default for that end. Asked here rather
    /// than recomputed at each caller, because the run configuration, the
    /// validator and the writer all need the same answer.
    pub(crate) fn resolved_sidebands(&self) -> (i32, i32) {
        match (self.sideband_min, self.sideband_max) {
            (None, None) => (-self.max_sideband, self.max_sideband),
            (minimum, maximum) => (
                minimum.unwrap_or(Self::CARD_DEFAULT_SIDEBAND_MIN),
                maximum.unwrap_or(Self::CARD_DEFAULT_SIDEBAND_MAX),
            ),
        }
    }

    /// Whether the symmetric field is the one that states the range.
    ///
    /// The two spellings are alternatives the card refuses together, so the
    /// form withholds the one it is not using rather than painting two
    /// controls that contradict each other.
    #[cfg(test)]
    pub(crate) fn states_symmetric_sidebands(&self) -> bool {
        self.sideband_min.is_none() && self.sideband_max.is_none()
    }

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

        // One spelling or the other, never both: the card refuses the pairing
        // where it is written, so the writer states whichever range the form
        // authored and nothing else.
        match (self.sideband_min, self.sideband_max) {
            (None, None) => cmd.push_str(&format!(" maxsideband={}", self.max_sideband)),
            (minimum, maximum) => {
                if let Some(minimum) = minimum {
                    cmd.push_str(&format!(" sidebandmin={minimum}"));
                }
                if let Some(maximum) = maximum {
                    cmd.push_str(&format!(" sidebandmax={maximum}"));
                }
            }
        }

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

        // Only an authored tolerance is written. The card's absent key means
        // the reader falls back to the deck's own `.options`, which is the
        // plan policy this field defers to when it is empty, so writing a
        // resolved number here would freeze a policy the deck states once.
        if let Some(reltol) = self.reltol {
            cmd.push_str(&format!(" reltol={reltol:e}"));
        }
        if let Some(abstol) = self.abstol {
            cmd.push_str(&format!(" abstol={abstol:e}"));
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

        // Each bound below is the engine's own, read off the `.PAC` arm that
        // parses the keyword, so the form refuses exactly what the card
        // cannot spell.
        if self.max_sideband < 0 {
            return Err("Maximum sideband must be non-negative".to_string());
        }

        let (resolved_min, resolved_max) = self.resolved_sidebands();
        if resolved_min > resolved_max {
            return Err(format!(
                "Sideband min {resolved_min} is above sideband max {resolved_max}"
            ));
        }
        // The engine refuses a card that both withholds sideband zero and
        // analyses no other sideband, whichever spelling stated the range.
        if !self.include_dc && resolved_min == 0 && resolved_max == 0 {
            return Err(
                "Withholding sideband zero leaves this analysis with nothing to publish"
                    .to_string(),
            );
        }

        for (label, tolerance) in [("Relative", self.reltol), ("Absolute", self.abstol)] {
            if let Some(tolerance) = tolerance
                && (!tolerance.is_finite() || tolerance <= 0.0)
            {
                return Err(format!("{label} tolerance must be finite and positive"));
            }
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
    /// Lowest sideband buffer; empty is the unauthored field.
    #[serde(default)]
    pub sideband_min: String,
    /// Highest sideband buffer; empty is the unauthored field.
    #[serde(default)]
    pub sideband_max: String,
    /// Relative tolerance buffer; empty defers to the plan's policy.
    #[serde(default)]
    pub reltol: String,
    /// Absolute tolerance buffer; empty defers to the plan's policy.
    #[serde(default)]
    pub abstol: String,
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
            sideband_min: optional_integer_text(config.sideband_min),
            sideband_max: optional_integer_text(config.sideband_max),
            reltol: optional_tolerance_text(config.reltol),
            abstol: optional_tolerance_text(config.abstol),
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
        let sideband_min = optional_integer(&self.sideband_min, "sideband min")?;
        let sideband_max = optional_integer(&self.sideband_max, "sideband max")?;
        let reltol = optional_tolerance(&self.reltol, "relative tolerance")?;
        let abstol = optional_tolerance(&self.abstol, "absolute tolerance")?;

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
            sideband_min,
            sideband_max,
            input_source: self.input_source.clone(),
            output_node: self.output_node.clone(),
            output_ref: self.output_ref.clone(),
            pac_magnitude: mag,
            include_dc: self.include_dc,
            reltol,
            abstol,
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

/// Read a well whose emptiness is a selection rather than an omission.
///
/// Three fields on this card work that way — the two sideband ends and the
/// two tolerances — and each means something different when it is blank: an
/// unauthored end takes the card's own default for that end, and an
/// unauthored tolerance takes the plan's policy. Neither is a parse failure,
/// so an empty well returns `None` and only a non-empty one is read.
fn optional_integer(text: &str, label: &str) -> Result<Option<i32>, String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    trimmed
        .parse()
        .map(Some)
        .map_err(|_| format!("Invalid {label}"))
}

fn optional_integer_text(value: Option<i32>) -> String {
    value.map(|value| value.to_string()).unwrap_or_default()
}

pub(super) fn optional_tolerance(text: &str, label: &str) -> Result<Option<f64>, String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    parse_si_value(trimmed)
        .map(Some)
        .map_err(|error| format!("Invalid {label}: {error}"))
}

pub(super) fn optional_tolerance_text(value: Option<f64>) -> String {
    value.map(|value| format!("{value:e}")).unwrap_or_default()
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

    /// Every carrier the engine's card accepts is a carrier this form accepts.
    ///
    /// The carrier is a *dependency*, not a field with a range: which family
    /// this request requires is `AnalysisDraft::prerequisite_roles`, and
    /// whether the bound instance can answer the measurement is
    /// `dependency_configuration_issue`. A refusal written here would have to
    /// decide without knowing the instance, which is how the harmonic-balance
    /// position came to be refused on a card the engine runs.
    #[test]
    fn every_carrier_the_engine_accepts_is_a_carrier_this_form_accepts() {
        for carrier in PeriodicCarrier::ALL {
            PacConfig {
                carrier: *carrier,
                ..PacConfig::default()
            }
            .validate()
            .unwrap_or_else(|error| panic!("{carrier:?} is a carrier the engine has: {error}"));
        }
    }

    /// The two sideband spellings are alternatives, and the card states
    /// whichever one the form authored.
    ///
    /// `parse_pac_command` refuses `MAXSIDEBAND=` beside
    /// `SIDEBANDMIN=`/`SIDEBANDMAX=` with `ConflictingFields`, so writing both
    /// would be a card the engine reads and then rejects. An end left
    /// unauthored takes the *card's* default for that end, never anything
    /// derived from the symmetric field, which is what the engine does with
    /// the same line.
    #[test]
    fn asymmetric_sidebands_reach_the_card_as_authored() {
        let asymmetric = PacConfig {
            sideband_min: Some(-2),
            sideband_max: Some(7),
            ..PacConfig::default()
        };
        assert_eq!(
            asymmetric.to_spice(),
            ".pac dec 10 1k 1G sidebandmin=-2 sidebandmax=7 input=VRF out=VOUT"
        );
        assert_eq!(asymmetric.resolved_sidebands(), (-2, 7));
        assert!(!asymmetric.states_symmetric_sidebands());

        // One end alone: the card states that end, and the other is the
        // card's own default rather than the symmetric field's.
        let one_end = PacConfig {
            max_sideband: 3,
            sideband_min: Some(-1),
            ..PacConfig::default()
        };
        assert_eq!(
            one_end.to_spice(),
            ".pac dec 10 1k 1G sidebandmin=-1 input=VRF out=VOUT"
        );
        assert_eq!(
            one_end.resolved_sidebands(),
            (-1, PacConfig::CARD_DEFAULT_SIDEBAND_MAX)
        );

        // Neither end: the symmetric spelling, exactly as before.
        assert_eq!(
            PacConfig::default().to_spice(),
            ".pac dec 10 1k 1G maxsideband=5 input=VRF out=VOUT"
        );
        assert_eq!(PacConfig::default().resolved_sidebands(), (-5, 5));
    }

    /// A range whose ends cross, and one that publishes nothing, are the
    /// engine's two refusals on this pair.
    #[test]
    fn an_empty_sideband_range_is_refused_the_way_the_engine_refuses_it() {
        let crossed = PacConfig {
            sideband_min: Some(4),
            sideband_max: Some(1),
            ..PacConfig::default()
        }
        .validate()
        .expect_err("a range whose ends cross contains no sideband");
        assert!(crossed.contains("Sideband min 4"), "{crossed}");

        let nothing_to_publish = PacConfig {
            sideband_min: Some(0),
            sideband_max: Some(0),
            include_dc: false,
            ..PacConfig::default()
        }
        .validate()
        .expect_err("withholding the only sideband leaves nothing to publish");
        assert!(
            nothing_to_publish.contains("nothing to publish"),
            "{nothing_to_publish}"
        );
    }

    /// An authored tolerance reaches the card; an empty one leaves it unsaid,
    /// which is how the plan's policy stays the single owner of the number.
    #[test]
    fn an_authored_tolerance_reaches_the_card_and_an_empty_one_leaves_it_unsaid() {
        assert!(!PacConfig::default().to_spice().contains("reltol="));
        assert!(!PacConfig::default().to_spice().contains("abstol="));
        assert_eq!(
            PacConfig {
                reltol: Some(1.0e-5),
                abstol: Some(1.0e-15),
                ..PacConfig::default()
            }
            .to_spice(),
            ".pac dec 10 1k 1G maxsideband=5 input=VRF out=VOUT reltol=1e-5 abstol=1e-15"
        );
        let refused = PacConfig {
            reltol: Some(0.0),
            ..PacConfig::default()
        }
        .validate()
        .expect_err("the card takes a positive relative tolerance");
        assert!(refused.contains("Relative tolerance"), "{refused}");
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

    /// A draft saved before the sideband ends and the tolerances existed opens
    /// as the analysis it ran: the symmetric range and the plan's policy.
    #[test]
    fn a_draft_saved_before_the_range_and_tolerance_wells_restores_unchanged() {
        let mut document = serde_json::to_value(PacDialogState::from_config(&PacConfig::default()))
            .expect("the draft serializes");
        let body = document.as_object_mut().expect("the draft is an object");
        for key in ["sideband_min", "sideband_max", "reltol", "abstol"] {
            body.remove(key)
                .unwrap_or_else(|| panic!("the key {key} this test removes must exist"));
        }
        let restored: PacDialogState =
            serde_json::from_value(document).expect("a draft written before the wells loads");
        let config = restored
            .to_config()
            .expect("the restored draft is runnable");
        assert_eq!(config.sideband_min, None);
        assert_eq!(config.sideband_max, None);
        assert_eq!(config.reltol, None);
        assert_eq!(config.abstol, None);
        assert_eq!(config.to_spice(), PacConfig::default().to_spice());
    }
}
