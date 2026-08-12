//! Temperature Sweep Analysis Configuration
//!
//! Configuration for temperature sweep analysis.
//! Runs a base analysis across a range of temperatures.
//!
//! The dimension is authored either as a range or as an explicit list. The
//! list is the user's, not a fixed preset: a design qualified at four named
//! temperatures is qualified at the four its own programme names, and a
//! sweep that can only offer -40/25/85/125 forces the reader to fake the
//! rest with a range that hits them by accident.

use serde::{Deserialize, Deserializer};

/// Base analysis for temperature sweep
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TempBaseAnalysis {
    #[default]
    Op,
    Transient,
    Ac,
    Dc,
}

/// Temperature sweep configuration
#[derive(Debug, Clone)]
pub struct TempConfig {
    /// Start temperature (Celsius)
    pub temp_start: f64,
    /// Stop temperature (Celsius)
    pub temp_stop: f64,
    /// Temperature step (Celsius)
    pub temp_step: f64,
    /// Base analysis type
    pub base_analysis: TempBaseAnalysis,
    /// Explicit temperatures (Celsius). When non-empty this is the dimension,
    /// and the range is not consulted.
    pub specific_temps: Vec<f64>,
}

impl Default for TempConfig {
    fn default() -> Self {
        Self {
            temp_start: -40.0,
            temp_stop: 125.0,
            temp_step: 25.0,
            base_analysis: TempBaseAnalysis::Op,
            specific_temps: Vec::new(),
        }
    }
}

impl TempConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.temp_start < -273.15 || self.temp_stop < -273.15 {
            return Err("Temperature cannot be below absolute zero (-273.15°C)".into());
        }
        for temperature in &self.specific_temps {
            if !temperature.is_finite() {
                return Err("Temperature values must be finite".into());
            }
            if *temperature < -273.15 {
                return Err("Temperature cannot be below absolute zero (-273.15°C)".into());
            }
        }
        if self.specific_temps.is_empty() {
            if self.temp_step.abs() < 0.01 {
                return Err("Temperature step too small".into());
            }
            if (self.temp_stop > self.temp_start && self.temp_step < 0.0)
                || (self.temp_stop < self.temp_start && self.temp_step > 0.0)
            {
                return Err("Step sign must match sweep direction".into());
            }
        }
        Ok(())
    }

    pub fn num_temps(&self) -> usize {
        if !self.specific_temps.is_empty() {
            return self.specific_temps.len();
        }
        if self.temp_step.abs() < 0.01 {
            return 1;
        }
        ((self.temp_stop - self.temp_start) / self.temp_step)
            .abs()
            .ceil() as usize
            + 1
    }
}

/// The temperatures the retired `corner_temps` preset stood for.
const RETIRED_CORNER_TEMPERATURES: &str = "-40, 25, 85, 125";

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct TempDialogState {
    pub temp_start: String,
    pub temp_stop: String,
    pub temp_step: String,
    pub base_idx: usize,
    /// Explicit temperatures, comma or space separated. Empty means the range
    /// above is the dimension.
    pub specific_temps: String,
    #[serde(skip)]
    pub initialized: bool,
}

/// Persisted editor state. New fields serialize; retired fields only decode.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PersistedTempDialogState {
    #[serde(default)]
    temp_start: String,
    #[serde(default)]
    temp_stop: String,
    #[serde(default)]
    temp_step: String,
    #[serde(default)]
    base_idx: usize,
    #[serde(default)]
    specific_temps: Option<String>,
    /// Retired preset for the four qualification temperatures. A project that
    /// had it on migrates to those exact values as an authored list, so the
    /// dimension it ran on is preserved rather than silently reverting to the
    /// range.
    #[serde(default)]
    corner_temps: Option<bool>,
}

impl<'de> Deserialize<'de> for TempDialogState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let persisted = PersistedTempDialogState::deserialize(deserializer)?;
        let specific_temps = persisted.specific_temps.unwrap_or_default();
        let specific_temps =
            if specific_temps.trim().is_empty() && persisted.corner_temps == Some(true) {
                RETIRED_CORNER_TEMPERATURES.to_owned()
            } else {
                specific_temps
            };
        Ok(Self {
            temp_start: persisted.temp_start,
            temp_stop: persisted.temp_stop,
            temp_step: persisted.temp_step,
            base_idx: persisted.base_idx,
            specific_temps,
            initialized: false,
        })
    }
}

impl TempDialogState {
    pub fn from_config(config: &TempConfig) -> Self {
        Self {
            temp_start: format!("{}", config.temp_start),
            temp_stop: format!("{}", config.temp_stop),
            temp_step: format!("{}", config.temp_step),
            base_idx: match config.base_analysis {
                TempBaseAnalysis::Op => 0,
                TempBaseAnalysis::Transient => 1,
                TempBaseAnalysis::Ac => 2,
                TempBaseAnalysis::Dc => 3,
            },
            specific_temps: config
                .specific_temps
                .iter()
                .map(|temperature| format!("{temperature}"))
                .collect::<Vec<_>>()
                .join(", "),
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<TempConfig, String> {
        let start: f64 = self.temp_start.parse().map_err(|_| "Invalid start temp")?;
        let stop: f64 = self.temp_stop.parse().map_err(|_| "Invalid stop temp")?;
        let step: f64 = self.temp_step.parse().map_err(|_| "Invalid step")?;
        let base = match self.base_idx {
            0 => TempBaseAnalysis::Op,
            1 => TempBaseAnalysis::Transient,
            2 => TempBaseAnalysis::Ac,
            _ => TempBaseAnalysis::Dc,
        };
        let config = TempConfig {
            temp_start: start,
            temp_stop: stop,
            temp_step: step,
            base_analysis: base,
            specific_temps: parse_temperature_list(&self.specific_temps)?,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&TempConfig::default());
        }
    }
}

/// Parse an authored temperature list.
///
/// Rejects rather than repairs: a value that is not a number is a typo the
/// reader has to see, not a point to quietly drop from a qualification sweep.
fn parse_temperature_list(text: &str) -> Result<Vec<f64>, String> {
    let mut temperatures = Vec::new();
    for token in text
        .split([',', ';', ' ', '\t', '\n'])
        .map(str::trim)
        .filter(|token| !token.is_empty())
    {
        let temperature: f64 = token
            .parse()
            .map_err(|_| format!("'{token}' is not a temperature"))?;
        temperatures.push(temperature);
    }
    Ok(temperatures)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_authored_list_replaces_the_range() {
        let mut state = TempDialogState::from_config(&TempConfig::default());
        state.specific_temps = "-40, 27, 125".to_owned();

        let config = state.to_config().expect("list parses");

        assert_eq!(config.specific_temps, vec![-40.0, 27.0, 125.0]);
        assert_eq!(config.num_temps(), 3);
    }

    #[test]
    fn an_empty_list_leaves_the_range_in_charge() {
        let state = TempDialogState::from_config(&TempConfig::default());

        let config = state.to_config().expect("range parses");

        assert!(config.specific_temps.is_empty());
        assert_eq!(config.num_temps(), 8);
    }

    #[test]
    fn a_malformed_value_is_rejected_rather_than_dropped() {
        let mut state = TempDialogState::from_config(&TempConfig::default());
        state.specific_temps = "-40, warm, 125".to_owned();

        let error = state.to_config().expect_err("a typo must not be skipped");

        assert!(error.contains("warm"), "{error}");
    }

    #[test]
    fn the_retired_corner_preset_migrates_to_its_exact_temperatures() {
        let persisted = r#"{
            "temp_start": "-40",
            "temp_stop": "125",
            "temp_step": "25",
            "base_idx": 0,
            "corner_temps": true
        }"#;

        let state: TempDialogState = serde_json::from_str(persisted).expect("legacy state decodes");

        assert_eq!(state.specific_temps, "-40, 25, 85, 125");

        let encoded = serde_json::to_value(&state).expect("state encodes");
        assert!(encoded.get("corner_temps").is_none());
    }

    #[test]
    fn absolute_zero_is_rejected_in_an_authored_list() {
        let mut state = TempDialogState::from_config(&TempConfig::default());
        state.specific_temps = "-300".to_owned();

        assert!(state.to_config().is_err());
    }
}
