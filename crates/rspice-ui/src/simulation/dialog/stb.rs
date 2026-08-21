//! Stability (STB) Analysis Configuration
//!
//! Configuration for loop stability analysis (.stb).
//!
//! Every field here changes what the engine does. The margins themselves —
//! gain margin, phase margin, crossover frequency — are not configuration:
//! the analysis always extracts them and reports them as measurements, so a
//! control that claimed to enable them would only be able to lie.

use super::options::parse_si_value;
use serde::{Deserialize, Deserializer, Serialize};

/// Frequency sweep shape for the loop-gain sweep.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StbSweepType {
    /// Points per decade (logarithmic)
    #[default]
    Decade,
    /// Points per octave (logarithmic)
    Octave,
    /// Total points, linearly spaced
    Linear,
}

impl StbSweepType {
    /// SPICE keyword, matching the `.STB` directive the parser accepts.
    pub fn spice_keyword(&self) -> &'static str {
        match self {
            Self::Decade => "dec",
            Self::Octave => "oct",
            Self::Linear => "lin",
        }
    }

    fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Decade,
            1 => Self::Octave,
            _ => Self::Linear,
        }
    }

    fn selection_index(&self) -> usize {
        match self {
            Self::Decade => 0,
            Self::Octave => 1,
            Self::Linear => 2,
        }
    }
}

/// Where the probe name in an STB configuration came from.
///
/// The distinction is not cosmetic. A name chosen from the drawing is a
/// reference to an object that can be deleted, and a run that still refers to
/// a deleted probe has to say so by name. A name typed by hand refers to
/// whatever the deck happens to contain, which this application cannot check
/// and must not pretend to.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum StbProbeReference {
    /// A loop probe placed on the schematic, referred to by its designator.
    Placed,
    /// A name entered by hand, for decks whose probe this design does not draw.
    #[default]
    Entered,
}

/// Stability analysis configuration
#[derive(Debug, Clone)]
pub struct StbConfig {
    /// Probe source element name
    pub probe_source: String,
    /// Whether `probe_source` names a placed loop probe or was typed in.
    pub probe_reference: StbProbeReference,
    /// Start frequency (Hz)
    pub start_freq: f64,
    /// Stop frequency (Hz)
    pub stop_freq: f64,
    /// Points per decade or octave; total points for a linear sweep
    pub num_points: u32,
    /// Sweep shape
    pub sweep_type: StbSweepType,
    /// Retain the Nyquist contour alongside the Bode data
    pub compute_nyquist: bool,
}

impl Default for StbConfig {
    fn default() -> Self {
        Self {
            // A fresh draft has not been shown a drawing yet, so it cannot
            // claim to refer to anything placed on one.
            probe_reference: StbProbeReference::Entered,
            probe_source: "LSTB".into(),
            start_freq: 1.0,
            stop_freq: 1e9,
            num_points: 10,
            sweep_type: StbSweepType::Decade,
            compute_nyquist: true,
        }
    }
}

impl StbConfig {
    /// The `.STB` directive, in the token order the netlist parser accepts:
    /// variation, points, start, stop, then the probe.
    pub fn to_spice(&self) -> String {
        format!(
            ".stb {} {} {} {} probe={}",
            self.sweep_type.spice_keyword(),
            self.num_points,
            format_freq(self.start_freq),
            format_freq(self.stop_freq),
            self.probe_source
        )
    }

    /// Refuse a configuration that still refers to a loop probe the drawing
    /// no longer holds.
    ///
    /// Only a placed reference can be checked. A name entered by hand is a
    /// claim about the deck, not about this design, and the engine is the
    /// only thing entitled to reject it — refusing it here would block the
    /// manual-deck workflows the entered form exists to serve.
    pub fn deleted_probe_error(&self, placed: &[String]) -> Option<String> {
        if self.probe_reference != StbProbeReference::Placed {
            return None;
        }
        let probe = self.probe_source.trim();
        if placed
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case(probe))
        {
            return None;
        }
        // The two cases need different remedies: with no probe drawn at all
        // the engineer has to place one, whereas with probes drawn the name
        // is simply stale and can be re-pointed.
        Some(if placed.is_empty() {
            format!(
                "loop probe '{probe}' is no longer on the schematic, and no loop probe is \
                 placed; draw one in series with the feedback path and select it, or enter \
                 a 0 V source name by hand"
            )
        } else {
            format!(
                "loop probe '{probe}' is no longer on the schematic; select one of {}, or \
                 enter a 0 V source name by hand",
                placed.join(", ")
            )
        })
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.probe_source.is_empty() {
            return Err("Probe source required".into());
        }
        if self.start_freq <= 0.0 {
            return Err("Start frequency must be positive".into());
        }
        if self.stop_freq <= 0.0 {
            return Err("Stop frequency must be positive".into());
        }
        if self.start_freq >= self.stop_freq {
            return Err("Start must be less than stop".into());
        }
        if self.num_points == 0 {
            return Err("Point count must be at least 1".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct StbDialogState {
    pub probe_source: String,
    pub probe_reference: StbProbeReference,
    pub start_freq: String,
    pub stop_freq: String,
    pub num_points: String,
    pub sweep_type_idx: usize,
    pub compute_nyquist: bool,
    #[serde(skip)]
    pub initialized: bool,
}

/// Persisted editor state. New fields serialize; retired fields only decode.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PersistedStbDialogState {
    #[serde(default)]
    probe_source: String,
    /// Absent in projects written before the probe could refer to a placed
    /// object. Those name whatever they name by hand, which is what the
    /// default says.
    #[serde(default)]
    probe_reference: StbProbeReference,
    #[serde(default)]
    start_freq: String,
    #[serde(default)]
    stop_freq: String,
    /// Point count. `points_per_decade` was the name before the sweep shape
    /// became configurable and the count stopped being per-decade by
    /// definition.
    #[serde(default, alias = "points_per_decade")]
    num_points: String,
    #[serde(default)]
    sweep_type_idx: usize,
    #[serde(default = "default_true")]
    compute_nyquist: bool,
    /// Retired. The analysis always extracts every margin, so these three
    /// never selected anything; they are accepted so projects written by an
    /// earlier build still open, and are never written back.
    #[serde(default)]
    #[allow(dead_code)]
    gain_margin: serde::de::IgnoredAny,
    #[serde(default)]
    #[allow(dead_code)]
    phase_margin: serde::de::IgnoredAny,
    #[serde(default)]
    #[allow(dead_code)]
    crossover_freq: serde::de::IgnoredAny,
}

fn default_true() -> bool {
    true
}

impl<'de> Deserialize<'de> for StbDialogState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let persisted = PersistedStbDialogState::deserialize(deserializer)?;
        Ok(Self {
            probe_source: persisted.probe_source,
            probe_reference: persisted.probe_reference,
            start_freq: persisted.start_freq,
            stop_freq: persisted.stop_freq,
            num_points: persisted.num_points,
            sweep_type_idx: persisted.sweep_type_idx,
            compute_nyquist: persisted.compute_nyquist,
            initialized: false,
        })
    }
}

impl StbDialogState {
    pub fn from_config(config: &StbConfig) -> Self {
        Self {
            probe_source: config.probe_source.clone(),
            probe_reference: config.probe_reference,
            start_freq: format_freq(config.start_freq),
            stop_freq: format_freq(config.stop_freq),
            num_points: config.num_points.to_string(),
            sweep_type_idx: config.sweep_type.selection_index(),
            compute_nyquist: config.compute_nyquist,
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<StbConfig, String> {
        let start = parse_si_value(&self.start_freq).map_err(|e| format!("Bad start: {}", e))?;
        let stop = parse_si_value(&self.stop_freq).map_err(|e| format!("Bad stop: {}", e))?;
        let points: u32 = self.num_points.parse().map_err(|_| "Invalid points")?;
        let config = StbConfig {
            probe_source: self.probe_source.clone(),
            probe_reference: self.probe_reference,
            start_freq: start,
            stop_freq: stop,
            num_points: points,
            sweep_type: StbSweepType::from_index(self.sweep_type_idx),
            compute_nyquist: self.compute_nyquist,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&StbConfig::default());
        }
    }
}

fn format_freq(f: f64) -> String {
    if f >= 1e9 {
        format!("{}G", f / 1e9)
    } else if f >= 1e6 {
        format!("{}Meg", f / 1e6)
    } else if f >= 1e3 {
        format!("{}k", f / 1e3)
    } else {
        format!("{}", f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directive_orders_tokens_the_way_the_parser_reads_them() {
        let config = StbConfig::default();

        assert_eq!(config.to_spice(), ".stb dec 10 1 1G probe=LSTB");
    }

    #[test]
    fn linear_sweep_reaches_the_directive() {
        let config = StbConfig {
            sweep_type: StbSweepType::Linear,
            num_points: 200,
            ..StbConfig::default()
        };

        assert!(config.to_spice().starts_with(".stb lin 200 "));
    }

    /// A reference to a probe that is no longer drawn must be refused by the
    /// name it still holds, and must say what to do instead.
    #[test]
    fn a_deleted_placed_probe_is_refused_by_name() {
        let config = StbConfig {
            probe_source: "VLOOP1".into(),
            probe_reference: StbProbeReference::Placed,
            ..StbConfig::default()
        };

        let error = config
            .deleted_probe_error(&["VLOOP2".to_owned()])
            .expect("a probe that is not placed is refused");

        assert!(error.contains("VLOOP1"), "{error}");
        // The remedy has to name what can be chosen instead, or the engineer
        // is told the run is wrong without being told what is available.
        assert!(error.contains("VLOOP2"), "{error}");
    }

    /// With nothing drawn the remedy is different: there is no probe to
    /// re-point at, so the instruction is to draw one.
    #[test]
    fn a_reference_with_no_probe_placed_says_to_draw_one() {
        let config = StbConfig {
            probe_source: "VLOOP1".into(),
            probe_reference: StbProbeReference::Placed,
            ..StbConfig::default()
        };

        let error = config
            .deleted_probe_error(&[])
            .expect("a reference with nothing placed is refused");

        assert!(error.contains("VLOOP1"), "{error}");
        assert!(error.contains("no loop probe is placed"), "{error}");
    }

    #[test]
    fn a_placed_probe_that_is_still_drawn_is_accepted() {
        let config = StbConfig {
            probe_source: "vloop1".into(),
            probe_reference: StbProbeReference::Placed,
            ..StbConfig::default()
        };

        // SPICE element names are case-insensitive, and so is the engine's own
        // probe lookup; the reference must match the same way.
        assert_eq!(config.deleted_probe_error(&["VLOOP1".to_owned()]), None);
    }

    /// An entered name is a claim about the deck, which this application
    /// cannot check. Refusing it here would break the manual-deck workflows
    /// the entered form exists to serve.
    #[test]
    fn an_entered_name_is_never_refused_for_not_being_on_the_schematic() {
        let config = StbConfig {
            probe_source: "VPRB".into(),
            probe_reference: StbProbeReference::Entered,
            ..StbConfig::default()
        };

        assert_eq!(config.deleted_probe_error(&[]), None);
    }

    /// The default draft has not been shown a drawing, so it cannot claim to
    /// refer to anything on one — and the directive ratchet depends on that
    /// default still emitting a card.
    #[test]
    fn the_default_draft_refers_to_nothing_placed() {
        let config = StbConfig::default();

        assert_eq!(config.probe_reference, StbProbeReference::Entered);
        assert_eq!(config.deleted_probe_error(&[]), None);
    }

    #[test]
    fn a_project_written_before_placed_probes_opens_as_entered() {
        let persisted = r#"{
            "probe_source": "VPRB",
            "start_freq": "1",
            "stop_freq": "1G",
            "num_points": "10"
        }"#;

        let state: StbDialogState = serde_json::from_str(persisted).expect("legacy state decodes");

        assert_eq!(state.probe_reference, StbProbeReference::Entered);
    }

    #[test]
    fn retired_margin_flags_decode_without_reappearing_on_serialize() {
        let persisted = r#"{
            "probe_source": "LP",
            "start_freq": "1",
            "stop_freq": "1G",
            "points_per_decade": "25",
            "gain_margin": true,
            "phase_margin": false,
            "crossover_freq": true
        }"#;

        let state: StbDialogState = serde_json::from_str(persisted).expect("legacy state decodes");

        assert_eq!(state.num_points, "25");
        assert!(
            state.compute_nyquist,
            "Nyquist defaults on for old projects"
        );

        let encoded = serde_json::to_value(&state).expect("state encodes");
        for retired in ["gain_margin", "phase_margin", "crossover_freq"] {
            assert!(encoded.get(retired).is_none(), "{retired}");
        }
    }
}
