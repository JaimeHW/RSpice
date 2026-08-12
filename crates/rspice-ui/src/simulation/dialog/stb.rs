//! Stability (STB) Analysis Configuration
//!
//! Configuration for loop stability analysis (.stb).
//!
//! Every field here changes what the engine does. The margins themselves —
//! gain margin, phase margin, crossover frequency — are not configuration:
//! the analysis always extracts them and reports them as measurements, so a
//! control that claimed to enable them would only be able to lie.

use super::options::parse_si_value;
use serde::{Deserialize, Deserializer};

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

/// Stability analysis configuration
#[derive(Debug, Clone)]
pub struct StbConfig {
    /// Probe source element name
    pub probe_source: String,
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
