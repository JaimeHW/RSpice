//! Monte Carlo Analysis Configuration
//!
//! Configuration for Monte Carlo statistical analysis.
//!
//! Each trial varies one circuit and executes either the legacy operating
//! point population or an exact configured analysis selected from the plan.

use crate::mc_statistics::is_parameter_name;
use crate::{mc_checkpoint as checkpoint, mc_statistics as statistics};
use rspice_app_types::product::AnalysisInstanceId;
use rspice_results::monte_carlo::MonteCarloMeanMethod;
use serde::{Deserialize, Deserializer};
use statistics::{McCorrelationDraft, McStatisticsConfig, McVariationDraft};

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
    pub checkpoint: Option<checkpoint::McCheckpointConfig>,
    /// Zero-based index in the original seeded trial population.
    pub first_trial: u32,
    pub statistics: Option<McStatisticsConfig>,
    /// Two-sided uncertainty in the population mean, independent of yield.
    pub confidence_pct: f64,
    pub confidence_method: MonteCarloMeanMethod,
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
    /// The subset of eligible parameters a trial is allowed to vary, in
    /// authored order. Empty means every eligible parameter, which is the
    /// card's own meaning for an absent `PARAMS` list.
    ///
    /// Only [`McVariationSource::ParameterTolerance`] reads it: the deck's own
    /// `agauss`/`gauss`/`unif` expressions name what they vary themselves, and
    /// the engine refuses a generic filter alongside native Spectre statistics
    /// rather than pretending to narrow them.
    pub params: Vec<String>,
    pub base_analysis: Option<AnalysisInstanceId>,
    pub measurements: Vec<String>,
    pub histogram_bins: usize,
}

impl Default for McConfig {
    fn default() -> Self {
        Self {
            checkpoint: None,
            statistics: None,
            first_trial: 0,
            num_runs: 100,
            confidence_pct: 95.0,
            confidence_method: MonteCarloMeanMethod::StudentT,
            seed: None,
            variation_source: McVariationSource::ParameterTolerance,
            distribution: McDistribution::Gaussian,
            variation_pct: 5.0,
            params: Vec::new(),
            base_analysis: None,
            measurements: Vec::new(),
            histogram_bins: 20,
        }
    }
}

impl McConfig {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(statistics) = &self.statistics {
            if self.variation_source != McVariationSource::DeckStatistics {
                return Err("Custom statistics require the native statistics sampler".into());
            }
            statistics.parser_directive()?;
        }
        if !self.confidence_pct.is_finite()
            || self.confidence_pct <= 0.0
            || self.confidence_pct >= 100.0
        {
            return Err(
                "Mean confidence must be finite and strictly between 0 and 100 percent".into(),
            );
        }
        if matches!(self.confidence_method, MonteCarloMeanMethod::PercentileBootstrap { resamples, .. } if resamples < 2)
        {
            return Err("Bootstrap confidence requires at least two resamples".into());
        }
        if self.first_trial.checked_add(self.num_runs).is_none() {
            return Err("Trial range exceeds the supported index range".into());
        }
        if self.num_runs == 0 {
            return Err("Number of runs must be at least 1".into());
        }
        if self.base_analysis.is_some() {
            crate::study_measurement::validate_measurements(&self.measurements)?;
        }
        if self.histogram_bins == 0 {
            return Err("Histogram bins must be at least one".into());
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

/// Read the "Vary only" field as the `PARAMS` list it writes.
///
/// The card separates entries by commas or whitespace and drops a repeat, so
/// this accepts both spellings and keeps the first of each name. An all-blank
/// field is the absent list, not an empty one: `.MC ... PARAMS` with nothing
/// after it is a parse error, so a blank field must write no keyword at all.
fn parse_parameter_subset(text: &str) -> Result<Vec<String>, String> {
    let mut names: Vec<String> = Vec::new();
    for entry in text.split([',', ' ', '\t']) {
        let entry = entry.trim();
        if entry.is_empty() {
            continue;
        }
        if !is_parameter_name(entry) {
            return Err(format!(
                "Invalid .MC parameter list token {entry:?}: expected identifier"
            ));
        }
        if !names.iter().any(|name| name.eq_ignore_ascii_case(entry)) {
            names.push(entry.to_owned());
        }
    }
    Ok(names)
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct McDialogState {
    pub checkpoint: checkpoint::McCheckpointDraft,
    pub first_trial: String,
    pub variations: Vec<McVariationDraft>,
    pub correlations: Vec<McCorrelationDraft>,
    pub confidence_pct: String,
    pub confidence_method_idx: usize,
    pub bootstrap_resamples: String,
    pub bootstrap_seed: String,
    pub num_runs: String,
    #[serde(rename = "explicit_seed")]
    pub seed: String,
    pub variation_source_idx: usize,
    pub distribution_idx: usize,
    pub variation_pct: String,
    /// The "Vary only" field, as authored. Commas or spaces separate names.
    pub vary_only: String,
    pub base_analysis: Option<AnalysisInstanceId>,
    pub measurements: String,
    pub histogram_bins: String,
    #[serde(skip)]
    pub initialized: bool,
}

impl Default for McDialogState {
    fn default() -> Self {
        Self {
            checkpoint: Default::default(),
            variations: Vec::new(),
            correlations: Vec::new(),
            first_trial: default_first_trial(),
            num_runs: String::new(),
            seed: String::new(),
            variation_source_idx: 0,
            distribution_idx: 0,
            variation_pct: String::new(),
            vary_only: String::new(),
            base_analysis: None,
            measurements: String::new(),
            histogram_bins: default_histogram_bins(),
            confidence_pct: default_confidence_pct(),
            confidence_method_idx: 0,
            bootstrap_resamples: default_bootstrap_resamples(),
            bootstrap_seed: default_bootstrap_seed(),
            initialized: false,
        }
    }
}

/// Persisted editor state. New fields serialize; retired fields only decode.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PersistedMcDialogState {
    #[serde(default)]
    checkpoint: checkpoint::McCheckpointDraft,
    #[serde(default = "default_first_trial")]
    first_trial: String,
    #[serde(default)]
    variations: Vec<McVariationDraft>,
    #[serde(default)]
    correlations: Vec<McCorrelationDraft>,
    #[serde(default = "default_confidence_pct")]
    confidence_pct: String,
    #[serde(default)]
    confidence_method_idx: usize,
    #[serde(default = "default_bootstrap_resamples")]
    bootstrap_resamples: String,
    #[serde(default = "default_bootstrap_seed")]
    bootstrap_seed: String,
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
    /// A project saved before the field opened carries no subset, which is the
    /// card's own "vary everything eligible".
    #[serde(default)]
    vary_only: String,
    #[serde(default)]
    base_analysis: Option<AnalysisInstanceId>,
    #[serde(default)]
    measurements: String,
    #[serde(default = "default_histogram_bins")]
    histogram_bins: String,
    /// Retired ordinal selector; it never identified a configured analysis.
    #[serde(default, rename = "base_idx")]
    _base_idx: serde::de::IgnoredAny,
    /// Retired. Variation is applied as one spread over the eligible
    /// parameters; there is no process/mismatch split to enable.
    #[serde(default, rename = "process_variations")]
    _process_variations: serde::de::IgnoredAny,
    #[serde(default, rename = "mismatch_variations")]
    _mismatch_variations: serde::de::IgnoredAny,
    /// Retired. A trial contributes a sample, not a retained dataset, so
    /// there was nothing for this to save.
    #[serde(default, rename = "save_all_runs")]
    _save_all_runs: serde::de::IgnoredAny,
}

fn default_first_trial() -> String {
    "0".into()
}

fn default_histogram_bins() -> String {
    "20".into()
}

fn default_confidence_pct() -> String {
    "95".into()
}
fn default_bootstrap_resamples() -> String {
    "10000".into()
}
fn default_bootstrap_seed() -> String {
    "0".into()
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
            checkpoint: persisted.checkpoint,
            variations: persisted.variations,
            correlations: persisted.correlations,
            first_trial: persisted.first_trial,
            num_runs: persisted.num_runs,
            confidence_pct: persisted.confidence_pct,
            confidence_method_idx: persisted.confidence_method_idx,
            bootstrap_resamples: persisted.bootstrap_resamples,
            bootstrap_seed: persisted.bootstrap_seed,
            seed,
            variation_source_idx: persisted.variation_source_idx,
            distribution_idx: persisted.distribution_idx,
            variation_pct: persisted.variation_pct,
            vary_only: persisted.vary_only,
            base_analysis: persisted.base_analysis,
            measurements: persisted.measurements,
            histogram_bins: persisted.histogram_bins,
            initialized: false,
        })
    }
}

impl McDialogState {
    pub fn from_config(config: &McConfig) -> Self {
        Self {
            checkpoint: checkpoint::McCheckpointDraft::from_config(config.checkpoint.as_ref()),
            variations: config
                .statistics
                .as_ref()
                .map(|s| {
                    s.variations
                        .iter()
                        .map(McVariationDraft::from_config)
                        .collect()
                })
                .unwrap_or_default(),
            correlations: config
                .statistics
                .as_ref()
                .map(|s| {
                    s.correlations
                        .iter()
                        .map(McCorrelationDraft::from_config)
                        .collect()
                })
                .unwrap_or_default(),
            first_trial: config.first_trial.to_string(),
            num_runs: config.num_runs.to_string(),
            confidence_pct: config.confidence_pct.to_string(),
            confidence_method_idx: usize::from(matches!(
                config.confidence_method,
                MonteCarloMeanMethod::PercentileBootstrap { .. }
            )),
            bootstrap_resamples: match config.confidence_method {
                MonteCarloMeanMethod::StudentT => default_bootstrap_resamples(),
                MonteCarloMeanMethod::PercentileBootstrap { resamples, .. } => {
                    resamples.to_string()
                }
            },
            bootstrap_seed: match config.confidence_method {
                MonteCarloMeanMethod::StudentT => default_bootstrap_seed(),
                MonteCarloMeanMethod::PercentileBootstrap { seed, .. } => seed.to_string(),
            },
            seed: config
                .seed
                .map_or_else(String::new, |seed| seed.to_string()),
            variation_source_idx: if config.statistics.is_some() {
                2
            } else {
                McVariationSource::ALL
                    .iter()
                    .position(|source| *source == config.variation_source)
                    .unwrap_or(0)
            },
            distribution_idx: match config.distribution {
                McDistribution::Gaussian => 0,
                McDistribution::Uniform => 1,
                McDistribution::WorstCase => 2,
            },
            variation_pct: format!("{}", config.variation_pct),
            vary_only: config.params.join(", "),
            base_analysis: config.base_analysis,
            measurements: config.measurements.join("; "),
            histogram_bins: config.histogram_bins.to_string(),
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
        let variation_source = if self.variation_source_idx == 2 {
            McVariationSource::DeckStatistics
        } else {
            *McVariationSource::ALL
                .get(self.variation_source_idx)
                .ok_or("Invalid variation source")?
        };
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
        // A subset only narrows the parameter-tolerance source. Under deck
        // statistics the engine refuses a generic filter outright, so the
        // draft keeps the authored text for switching back and contributes
        // nothing to the run.
        let params = if variation_source.uses_stated_spread() {
            parse_parameter_subset(&self.vary_only)?
        } else {
            Vec::new()
        };
        let confidence_pct = self
            .confidence_pct
            .trim()
            .parse::<f64>()
            .map_err(|_| "Invalid mean confidence percentage")?;
        let confidence_method = match self.confidence_method_idx {
            0 => MonteCarloMeanMethod::StudentT,
            1 => MonteCarloMeanMethod::PercentileBootstrap {
                resamples: self
                    .bootstrap_resamples
                    .trim()
                    .parse::<usize>()
                    .map_err(|_| "Bootstrap resamples must be an integer of at least two")?,
                seed: self.bootstrap_seed.trim().parse::<u64>().map_err(
                    |_| "Bootstrap seed must be an integer from 0 to 18446744073709551615",
                )?,
            },
            _ => return Err("Invalid mean-confidence estimator".into()),
        };
        let config = McConfig {
            checkpoint: self.checkpoint.to_config()?,
            statistics: if self.variation_source_idx == 2 {
                Some(McStatisticsConfig {
                    variations: self
                        .variations
                        .iter()
                        .map(McVariationDraft::to_config)
                        .collect::<Result<_, _>>()?,
                    correlations: self
                        .correlations
                        .iter()
                        .map(McCorrelationDraft::to_config)
                        .collect::<Result<_, _>>()?,
                })
            } else {
                None
            },
            confidence_pct,
            confidence_method,
            first_trial: self
                .first_trial
                .trim()
                .parse()
                .map_err(|_| "First trial index must be a nonnegative integer")?,
            num_runs: runs,
            seed,
            variation_source,
            distribution: dist,
            variation_pct: pct,
            params,
            base_analysis: self.base_analysis,
            measurements: if self.base_analysis.is_some() {
                crate::study_measurement::parse_measurement_list(&self.measurements)?
            } else {
                Vec::new()
            },
            histogram_bins: self
                .histogram_bins
                .trim()
                .parse()
                .map_err(|_| "Histogram bins must be a positive integer")?,
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
