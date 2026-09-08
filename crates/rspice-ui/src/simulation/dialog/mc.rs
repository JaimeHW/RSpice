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
    /// Explicit random seed; absence selects the runner's repeatable default.
    pub seed: Option<u64>,
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
            seed: None,
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
            if !self.variation_pct.is_finite() {
                return Err("Variation percentage must be finite".into());
            }
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
    #[serde(rename = "explicit_seed")]
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
    // Deserialize a present string directly, including in RON. An Option
    // decoder would instead require RON's Some(...) representation.
    #[serde(default, deserialize_with = "deserialize_explicit_seed")]
    explicit_seed: Option<String>,
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

fn deserialize_explicit_seed<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    String::deserialize(deserializer).map(Some)
}

impl<'de> Deserialize<'de> for McDialogState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let persisted = PersistedMcDialogState::deserialize(deserializer)?;
        let seed = match persisted.explicit_seed {
            Some(seed) if persisted.seed.is_empty() => seed,
            Some(_) => {
                return Err(serde::de::Error::custom(
                    "Monte Carlo draft contains both legacy and explicit seed fields",
                ));
            }
            // The old editor reserved zero for the default stream. Do not
            // change an existing project's random sequence during migration.
            None if persisted.seed.trim().parse::<u64>() == Ok(0) => String::new(),
            None => persisted.seed,
        };
        Ok(Self {
            num_runs: persisted.num_runs,
            seed,
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
            seed: config
                .seed
                .map_or_else(String::new, |seed| seed.to_string()),
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
        let runs: u32 = self.num_runs.trim().parse().map_err(|_| "Invalid runs")?;
        let seed = if self.seed.trim().is_empty() {
            None
        } else {
            Some(self.seed.trim().parse::<u64>().map_err(|_| "Seed must be an integer from 0 to 18446744073709551615, or blank for the default")?)
        };
        let variation_source = *McVariationSource::ALL
            .get(self.variation_source_idx)
            .ok_or("Invalid variation source")?;
        // Inactive buffers remain in the draft for switching back. They do
        // not contribute values to a run whose variation comes from the deck.
        let pct: f64 = if variation_source.uses_stated_spread() {
            self.variation_pct
                .parse()
                .map_err(|_| "Invalid variation")?
        } else {
            McConfig::default().variation_pct
        };
        let dist = match self.distribution_idx {
            0 => McDistribution::Gaussian,
            1 => McDistribution::Uniform,
            2 => McDistribution::WorstCase,
            _ if !variation_source.uses_stated_spread() => McDistribution::default(),
            _ => return Err("Invalid distribution".to_owned()),
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
    fn monte_carlo_seed_migration_preserves_legacy_default_and_editor_buffers() {
        for (legacy, expected) in [
            ("0", ""),
            (" 000 ", ""),
            ("7", "7"),
            ("unfinished(", "unfinished("),
        ] {
            let json = serde_json::json!({ "seed": legacy });
            let ron = format!("(seed:{legacy:?})");
            for state in [
                serde_json::from_value::<McDialogState>(json).unwrap(),
                ron::from_str::<McDialogState>(&ron).unwrap(),
            ] {
                assert_eq!(state.seed, expected);
                let encoded = serde_json::to_value(&state).unwrap();
                assert!(encoded.get("seed").is_none());
                assert_eq!(encoded["explicit_seed"], expected);
            }
        }
    }

    #[test]
    fn monte_carlo_seed_round_trips_json_and_ron_through_draft_restore() {
        use crate::simulation::plan::AnalysisDraft;
        for seed in [None, Some(0), Some((1_u64 << 53) + 1), Some(u64::MAX)] {
            let config = McConfig {
                seed,
                ..Default::default()
            };
            let draft = AnalysisDraft::MonteCarlo(McDialogState::from_config(&config));
            let json = serde_json::to_string(&draft).unwrap();
            let ron = ron::to_string(&draft).unwrap();
            for mut restored in [
                serde_json::from_str::<AnalysisDraft>(&json).unwrap(),
                ron::from_str::<AnalysisDraft>(&ron).unwrap(),
            ] {
                restored.prepare_after_restore();
                let AnalysisDraft::MonteCarlo(mut state) = restored else {
                    panic!("wrong restored draft");
                };
                state.ensure_initialized();
                assert_eq!(state.to_config().unwrap().seed, seed);
            }
        }
    }

    #[test]
    fn monte_carlo_seed_corrupt_wire_values_are_rejected() {
        for source in [
            r#"{"explicit_seed":null}"#,
            r#"{"explicit_seed":0}"#,
            r#"{"seed":"7","explicit_seed":"0"}"#,
        ] {
            assert!(
                serde_json::from_str::<McDialogState>(source).is_err(),
                "{source}"
            );
        }
        for source in [
            "(explicit_seed:None)",
            "(explicit_seed:0)",
            "(seed:\"7\",explicit_seed:\"0\")",
        ] {
            assert!(ron::from_str::<McDialogState>(source).is_err(), "{source}");
        }
    }

    #[test]
    fn invalid_active_spread_and_distribution_are_refused() {
        let mut draft = McDialogState::from_config(&McConfig::default());
        draft.variation_pct = "NaN".to_owned();
        assert!(
            draft.to_config().is_err(),
            "NaN spread is not a runnable configuration"
        );
        draft.variation_pct = "5".to_owned();
        draft.distribution_idx = usize::MAX;
        assert!(
            draft.to_config().is_err(),
            "a missing distribution is not Worst Case"
        );
    }

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
