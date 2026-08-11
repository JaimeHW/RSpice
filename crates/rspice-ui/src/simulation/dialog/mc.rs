//! Monte Carlo Analysis Configuration
//!
//! Configuration for Monte Carlo statistical analysis.
//!
//! Each trial perturbs the eligible netlist parameters and solves an
//! operating point, and the result is the distribution of the node voltages
//! across trials. That is the whole contract: there is no per-trial base
//! analysis to choose, and no per-trial dataset to retain, so this
//! configuration offers neither.

use serde::{Deserialize, Deserializer};

/// Random distribution type
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum McDistribution {
    #[default]
    Gaussian,
    Uniform,
    WorstCase,
}

/// Where a trial's variation comes from.
///
/// The two sources are not two spellings of one thing. One states the spread
/// here and applies it to the deck's eligible parameters; the other takes the
/// spread from the deck, which is how a PDK expresses it.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum McVariationSource {
    /// Perturb the deck's eligible `.param` values by the spread stated below.
    /// Model cards are not touched.
    #[default]
    ParameterTolerance,
    /// Redraw the deck's own `agauss`/`gauss`/`unif` expressions from a fresh
    /// seed each trial, model cards included. The deck states the spread, so
    /// the distribution and spread controls below do not apply.
    DeckStatistics,
}

impl McVariationSource {
    pub const ALL: [Self; 2] = [Self::ParameterTolerance, Self::DeckStatistics];

    pub const fn display_name(self) -> &'static str {
        match self {
            Self::ParameterTolerance => "Parameter tolerance",
            Self::DeckStatistics => "Deck statistics",
        }
    }

    /// Whether the distribution and spread controls reach this trial's engine.
    pub const fn uses_stated_spread(self) -> bool {
        matches!(self, Self::ParameterTolerance)
    }
}

/// Monte Carlo analysis configuration
#[derive(Debug, Clone)]
pub struct McConfig {
    /// Number of runs
    pub num_runs: u32,
    /// Random seed (0 = auto)
    pub seed: u32,
    /// Where a trial's variation comes from
    pub variation_source: McVariationSource,
    /// Distribution type. Applies to [`McVariationSource::ParameterTolerance`].
    pub distribution: McDistribution,
    /// Variation percentage (sigma for Gaussian, ± for Uniform). Applies to
    /// [`McVariationSource::ParameterTolerance`].
    pub variation_pct: f64,
}

impl Default for McConfig {
    fn default() -> Self {
        Self {
            num_runs: 100,
            seed: 0,
            variation_source: McVariationSource::ParameterTolerance,
            distribution: McDistribution::Gaussian,
            variation_pct: 5.0,
        }
    }
}

impl McConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.num_runs == 0 {
            return Err("Number of runs must be at least 1".into());
        }
        // The spread is only asked for by one variation source. Rejecting it
        // under the other would reject a value that never reaches a solve.
        if self.variation_source.uses_stated_spread() {
            if self.variation_pct <= 0.0 {
                return Err("Variation percentage must be positive".into());
            }
            if self.variation_pct > 100.0 {
                return Err("Variation percentage cannot exceed 100%".into());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct McDialogState {
    pub num_runs: String,
    pub seed: String,
    pub variation_source_idx: usize,
    pub distribution_idx: usize,
    pub variation_pct: String,
    #[serde(skip)]
    pub initialized: bool,
}

/// Persisted editor state. New fields serialize; retired fields only decode.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PersistedMcDialogState {
    #[serde(default)]
    num_runs: String,
    #[serde(default)]
    seed: String,
    #[serde(default)]
    variation_source_idx: usize,
    #[serde(default)]
    distribution_idx: usize,
    #[serde(default)]
    variation_pct: String,
    /// Retired. Every trial is an operating point; the choice named base
    /// analyses that were never dispatched.
    #[serde(default)]
    #[allow(dead_code)]
    base_idx: serde::de::IgnoredAny,
    /// Retired. Variation is applied as one spread over the eligible
    /// parameters; there is no process/mismatch split to enable.
    #[serde(default)]
    #[allow(dead_code)]
    process_variations: serde::de::IgnoredAny,
    #[serde(default)]
    #[allow(dead_code)]
    mismatch_variations: serde::de::IgnoredAny,
    /// Retired. A trial contributes a sample, not a retained dataset, so
    /// there was nothing for this to save.
    #[serde(default)]
    #[allow(dead_code)]
    save_all_runs: serde::de::IgnoredAny,
}

impl<'de> Deserialize<'de> for McDialogState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let persisted = PersistedMcDialogState::deserialize(deserializer)?;
        Ok(Self {
            num_runs: persisted.num_runs,
            seed: persisted.seed,
            variation_source_idx: persisted.variation_source_idx,
            distribution_idx: persisted.distribution_idx,
            variation_pct: persisted.variation_pct,
            initialized: false,
        })
    }
}

impl McDialogState {
    pub fn from_config(config: &McConfig) -> Self {
        Self {
            num_runs: config.num_runs.to_string(),
            seed: config.seed.to_string(),
            variation_source_idx: McVariationSource::ALL
                .iter()
                .position(|source| *source == config.variation_source)
                .unwrap_or(0),
            distribution_idx: match config.distribution {
                McDistribution::Gaussian => 0,
                McDistribution::Uniform => 1,
                McDistribution::WorstCase => 2,
            },
            variation_pct: format!("{}", config.variation_pct),
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<McConfig, String> {
        let runs: u32 = self.num_runs.parse().map_err(|_| "Invalid runs")?;
        let seed: u32 = self.seed.parse().map_err(|_| "Invalid seed")?;
        let variation_source = *McVariationSource::ALL
            .get(self.variation_source_idx)
            .ok_or("Invalid variation source")?;
        // The spread buffer is still parsed under either source so an
        // unreadable value is reported rather than silently carried, but it
        // only has to be present for the source that reads it.
        let pct: f64 = if variation_source.uses_stated_spread() {
            self.variation_pct
                .parse()
                .map_err(|_| "Invalid variation")?
        } else {
            self.variation_pct
                .parse()
                .unwrap_or(McConfig::default().variation_pct)
        };
        let dist = match self.distribution_idx {
            0 => McDistribution::Gaussian,
            1 => McDistribution::Uniform,
            _ => McDistribution::WorstCase,
        };
        let config = McConfig {
            num_runs: runs,
            seed,
            variation_source,
            distribution: dist,
            variation_pct: pct,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&McConfig::default());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retired_controls_decode_without_reappearing_on_serialize() {
        let persisted = r#"{
            "num_runs": "250",
            "seed": "7",
            "distribution_idx": 1,
            "variation_pct": "2.5",
            "base_idx": 0,
            "process_variations": true,
            "mismatch_variations": false,
            "save_all_runs": true
        }"#;

        let state: McDialogState = serde_json::from_str(persisted).expect("legacy state decodes");

        assert_eq!(state.num_runs, "250");
        assert_eq!(state.distribution_idx, 1);

        let encoded = serde_json::to_value(&state).expect("state encodes");
        for retired in [
            "base_idx",
            "process_variations",
            "mismatch_variations",
            "save_all_runs",
        ] {
            assert!(encoded.get(retired).is_none(), "{retired}");
        }
    }
}
