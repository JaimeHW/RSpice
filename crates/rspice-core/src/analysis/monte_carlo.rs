//! Monte Carlo Analysis for Statistical Circuit Simulation
//!
//! Provides statistical analysis with component tolerances, supporting:
//! - Gaussian and uniform distributions
//! - Lot and device tolerances
//! - Statistical output (mean, std, min, max, histograms)
//! - Reproducible runs via seeding
//!
//! # Example
//! ```
//! use rspice_core::analysis::{MonteCarloConfig, MonteCarloRunner, Tolerance};
//! use std::collections::HashMap;
//!
//! let config = MonteCarloConfig::new(1000).with_seed(42);
//! let mut runner = MonteCarloRunner::new(config);
//! runner.add_component("R1", 1000.0, Tolerance::uniform(5.0));
//! // A callback may invoke a solver or, here, evaluate a divider analytically.
//! let results = runner.run(|values| {
//!     let output = 1000.0 / (values.get("R1", 1000.0) + 1000.0);
//!     Ok::<_, std::convert::Infallible>(HashMap::from([("V(out)".to_owned(), output)]))
//! }).unwrap();
//! assert_eq!(results.sampling.unwrap().seed, 42);
//! ```

use super::error::SimulationError;
use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::config::SimulationConfigError;
use crate::resource::{ResourceKind, ResourceLimitError, ResourceLimits};
use std::collections::{BTreeMap, HashMap};

mod confidence;
pub use confidence::{MeanConfidenceInterval, MeanConfidenceMethod, MonteCarloConfidence};

#[derive(Default)]
struct CompensatedSum {
    sum: Value,
    correction: Value,
}

impl CompensatedSum {
    fn add(&mut self, value: Value) {
        let next = self.sum + value;
        self.correction += if self.sum.abs() >= value.abs() {
            (self.sum - next) + value
        } else {
            (value - next) + self.sum
        };
        self.sum = next;
    }

    fn total(self) -> Value {
        self.sum + self.correction
    }
}

fn statistical_location_scale(samples: &[Value], min: Value, max: Value) -> (Value, Value) {
    // Center only a tightly clustered, single-sign population. Those
    // differences are exact by Sterbenz's lemma; centering a population that
    // spans zero can erase small values before compensation can recover them.
    let anchor = if (min > 0.0 && min >= max * 0.5) || (max < 0.0 && max <= min * 0.5) {
        samples[0]
    } else {
        0.0
    };
    (anchor, (min - anchor).abs().max((max - anchor).abs()))
}

//=============================================================================
// Distribution Types
//=============================================================================

/// Statistical distribution for component variation
#[derive(Debug, Clone, Copy)]
pub enum Distribution {
    /// Gaussian (normal) distribution: mean ± sigma
    /// 3-sigma corresponds to 99.7% of values
    Gaussian {
        /// Relative standard deviation (e.g., 0.01 for 1%)
        sigma: Value,
    },

    /// Uniform distribution: value ± tolerance
    Uniform {
        /// Relative half-width (e.g., 0.05 for ±5%)
        tolerance: Value,
    },

    /// Worst-case two-point distribution: value is always nominal ± tolerance.
    WorstCase {
        /// Relative half-width (e.g., 0.05 for ±5%)
        tolerance: Value,
    },
}

impl Distribution {
    /// Create a Gaussian distribution with given sigma
    pub fn gaussian(sigma: Value) -> Self {
        Distribution::Gaussian { sigma }
    }

    /// Create a uniform distribution with given tolerance
    pub fn uniform(tolerance: Value) -> Self {
        Distribution::Uniform { tolerance }
    }

    /// Create a worst-case distribution with given tolerance
    pub fn worst_case(tolerance: Value) -> Self {
        Distribution::WorstCase { tolerance }
    }

    pub(crate) fn validate(self) -> Result<(), SimulationConfigError> {
        let spread = match self {
            Self::Gaussian { sigma } => sigma,
            Self::Uniform { tolerance } | Self::WorstCase { tolerance } => tolerance,
        };
        if !spread.is_finite() || spread < 0.0 {
            return Err(SimulationConfigError::InvalidValue {
                field: "monte_carlo.spread",
                value: spread,
                requirement: "finite and non-negative",
            });
        }
        Ok(())
    }

    /// Sample a validated distribution about a finite nominal value.
    /// Relative spread scales with the nominal magnitude, including negative
    /// parameters. Batch entry points validate inputs before calling this.
    pub fn sample(&self, rng: &mut Xorshift128Plus, nominal: Value) -> Value {
        let magnitude = nominal.abs();
        match *self {
            Distribution::Gaussian { sigma } => nominal + rng.next_gaussian() * magnitude * sigma,
            Distribution::Uniform { tolerance } => {
                let delta = magnitude * tolerance;
                let u = rng.next_f64();
                nominal + (2.0 * u - 1.0) * delta
            }
            Distribution::WorstCase { tolerance } => {
                let delta = magnitude * tolerance;
                let sign = if (rng.next_u64() & 1) == 0 { -1.0 } else { 1.0 };
                nominal + sign * delta
            }
        }
    }
}

//=============================================================================
// Tolerance Specification
//=============================================================================

/// Component tolerance specification
#[derive(Debug, Clone)]
pub struct Tolerance {
    /// Lot-to-lot variation (same per Monte Carlo run)
    /// Models manufacturing batch differences
    pub lot: Option<Distribution>,

    /// Device-to-device variation (different for each component instance)
    /// Models variation within a single production batch
    pub dev: Option<Distribution>,
}

impl Tolerance {
    /// Create a simple uniform tolerance
    pub fn uniform(pct: Value) -> Self {
        Self {
            lot: None,
            dev: Some(Distribution::Uniform {
                tolerance: pct / 100.0,
            }),
        }
    }

    /// No tolerance (use nominal value)
    pub fn none() -> Self {
        Self {
            lot: None,
            dev: None,
        }
    }
}

impl Default for Tolerance {
    fn default() -> Self {
        Self::none()
    }
}

//=============================================================================
// Monte Carlo Configuration
//=============================================================================

/// Configuration for Monte Carlo analysis
#[derive(Debug, Clone)]
pub struct MonteCarloConfig {
    /// Number of simulation runs
    pub num_runs: usize,

    /// Random seed for reproducibility. None requests OS/browser host entropy;
    /// the resolved seed is retained in the result for replay.
    pub seed: Option<u64>,

    /// Number of histogram bins for output distribution
    pub histogram_bins: usize,

    /// Confidence interval percentage (95 = 95%)
    pub confidence_pct: Value,
    /// Mean-uncertainty estimator. Each method has explicit distributional
    /// assumptions; see `MeanConfidenceMethod`.
    pub confidence_method: MeanConfidenceMethod,
    /// Limits enforced before simulation and before retaining outputs.
    pub resource_limits: ResourceLimits,
}

impl MonteCarloConfig {
    /// Create new configuration with specified number of runs
    pub fn new(num_runs: usize) -> Self {
        Self {
            num_runs,
            seed: None,
            histogram_bins: 50,
            confidence_pct: 95.0,
            confidence_method: MeanConfidenceMethod::StudentT,
            resource_limits: ResourceLimits::default(),
        }
    }

    /// Set random seed for reproducibility
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }
}

impl Default for MonteCarloConfig {
    fn default() -> Self {
        Self::new(100)
    }
}

//=============================================================================
// Monte Carlo Results
//=============================================================================

/// Statistics for a single output variable
#[derive(Debug, Clone)]
pub struct VariableStatistics {
    /// Variable name
    pub name: String,
    /// All sampled values
    pub samples: Vec<Value>,
    /// Computed mean
    pub mean: Value,
    /// Computed standard deviation
    pub std_dev: Value,
    /// Minimum value
    pub min: Value,
    /// Maximum value
    pub max: Value,
    /// Histogram bin counts
    pub histogram: Vec<usize>,
    /// Histogram bin edges
    pub bin_edges: Vec<Value>,
    /// None when no estimator was requested; otherwise finite limits or an
    /// explicit reason they are unavailable.
    pub mean_confidence: Option<MeanConfidenceInterval>,
}

impl VariableStatistics {
    /// Create from samples
    pub fn from_samples(name: &str, samples: Vec<Value>, num_bins: usize) -> Self {
        if samples.is_empty() || samples.iter().any(|value| !value.is_finite()) {
            return Self {
                name: name.to_string(),
                samples,
                mean: Value::NAN,
                std_dev: Value::NAN,
                min: Value::NAN,
                max: Value::NAN,
                histogram: Vec::new(),
                bin_edges: Vec::new(),
                mean_confidence: None,
            };
        }

        let n = samples.len() as Value;

        // Min/max
        let min = samples.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = samples.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        // Center before scaling so tightly clustered large values retain
        // their low bits; scale before squaring to protect extreme moments.
        let (anchor, scale) = statistical_location_scale(&samples, min, max);
        let (mean, std_dev) = if scale == 0.0 {
            (anchor, 0.0)
        } else {
            let mut sum = CompensatedSum::default();
            for value in &samples {
                sum.add((value - anchor) / scale);
            }
            let normalized_mean = sum.total() / n;
            let mut sum = CompensatedSum::default();
            for value in &samples {
                sum.add(((value - anchor) / scale - normalized_mean).powi(2));
            }
            let normalized_variance = sum.total() / (n - 1.0).max(1.0);
            (
                anchor + normalized_mean * scale,
                normalized_variance.sqrt() * scale,
            )
        };

        // Histogram
        let (histogram, bin_edges) = Self::compute_histogram(&samples, num_bins, min, max);

        Self {
            name: name.to_string(),
            samples,
            mean,
            std_dev,
            min,
            max,
            histogram,
            bin_edges,
            mean_confidence: None,
        }
    }

    fn compute_histogram(
        samples: &[Value],
        num_bins: usize,
        min: Value,
        max: Value,
    ) -> (Vec<usize>, Vec<Value>) {
        if num_bins == 0 {
            return (vec![samples.len()], vec![min, max]);
        }

        let range = max - min;
        if !range.is_finite() || range <= 0.0 || !min.is_finite() || !max.is_finite() {
            return (vec![samples.len()], vec![min, max]);
        }

        let bin_width = range / num_bins as Value;
        if !bin_width.is_finite() || bin_width <= 0.0 {
            return (vec![samples.len()], vec![min, max]);
        }
        let mut histogram = vec![0usize; num_bins];
        let bin_edges: Vec<Value> = (0..=num_bins)
            .map(|i| min + (i as Value) * bin_width)
            .collect();

        for &sample in samples {
            let bin_position = ((sample - min) / bin_width).floor();
            if !bin_position.is_finite() {
                continue;
            }
            let bin = bin_position.max(0.0) as usize;
            let bin = bin.min(num_bins - 1);
            histogram[bin] += 1;
        }

        (histogram, bin_edges)
    }

    /// Get percentile value (0-100)
    pub fn percentile(&self, pct: Value) -> Value {
        if !pct.is_finite()
            || self.samples.is_empty()
            || self.samples.iter().any(|value| !value.is_finite())
        {
            return Value::NAN;
        }

        let mut sorted = self.samples.clone();
        sorted.sort_by(f64::total_cmp);

        let pct = pct.clamp(0.0, 100.0);
        let idx = ((pct / 100.0) * (sorted.len() - 1) as Value).floor() as usize;
        sorted[idx.min(sorted.len() - 1)]
    }

    /// Get 3-sigma range (mean ± 3*std_dev)
    pub fn three_sigma_range(&self) -> (Value, Value) {
        (
            self.mean - 3.0 * self.std_dev,
            self.mean + 3.0 * self.std_dev,
        )
    }
}

/// Seed and versioned sampling policy needed to reproduce a batch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonteCarloSampling {
    pub seed: u64,
    pub policy: &'static str,
}

/// Results from a complete Monte Carlo analysis
#[derive(Debug, Clone)]
pub struct MonteCarloResult {
    /// Total number of attempted runs, including failed runs.
    /// Every variable contains `num_runs - num_failures` successful samples.
    pub num_runs: usize,
    /// Statistics for each output variable
    pub variables: HashMap<String, VariableStatistics>,
    /// Whether all runs converged
    pub all_converged: bool,
    /// Number of failed runs
    pub num_failures: usize,
    /// Absent only for externally aggregated trials with no supplied provenance.
    pub sampling: Option<MonteCarloSampling>,
    /// Method, sample population, and assumptions for the mean intervals.
    pub confidence: Option<MonteCarloConfidence>,
}

impl MonteCarloResult {
    pub fn new() -> Self {
        Self {
            num_runs: 0,
            variables: HashMap::new(),
            all_converged: true,
            num_failures: 0,
            sampling: None,
            confidence: None,
        }
    }

    /// Get mean for a variable
    pub fn mean(&self, name: &str) -> Option<Value> {
        self.variables.get(name).map(|v| v.mean)
    }

    /// Get standard deviation for a variable
    pub fn std_dev(&self, name: &str) -> Option<Value> {
        self.variables.get(name).map(|v| v.std_dev)
    }

    /// Get min/max range for a variable
    pub fn range(&self, name: &str) -> Option<(Value, Value)> {
        self.variables.get(name).map(|v| (v.min, v.max))
    }
}

impl Default for MonteCarloResult {
    fn default() -> Self {
        Self::new()
    }
}

//=============================================================================
// Random Number Generator (xoroshiro128+)
//=============================================================================

/// xoroshiro128+ (2018 transition) seeded by SplitMix64.
///
/// The historical public type name is retained for SDK compatibility.
pub struct Xorshift128Plus {
    s0: u64,
    s1: u64,
}

impl Xorshift128Plus {
    /// Create with seed
    pub fn new(seed: u64) -> Self {
        // Use splitmix64 to initialize state from single seed
        let mut state = seed;
        let s0 = Self::splitmix64(&mut state);
        let s1 = Self::splitmix64(&mut state);
        Self { s0, s1 }
    }

    fn splitmix64(state: &mut u64) -> u64 {
        *state = state.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = *state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }

    /// Generate next u64
    pub fn next_u64(&mut self) -> u64 {
        let s0 = self.s0;
        let mut s1 = self.s1;
        let result = s0.wrapping_add(s1);

        s1 ^= s0;
        self.s0 = s0.rotate_left(24) ^ s1 ^ (s1 << 16);
        self.s1 = s1.rotate_left(37);

        result
    }

    /// Generate uniform f64 in [0, 1)
    pub fn next_f64(&mut self) -> f64 {
        let u = self.next_u64();
        (u >> 11) as f64 / (1_u64 << 53) as f64
    }

    /// Generate standard normal using Box-Muller transform
    pub fn next_gaussian(&mut self) -> f64 {
        // Box-Muller requires an open lower endpoint: ln(0) would turn a
        // valid random draw into an infinite parameter perturbation.
        let u1 = loop {
            let value = self.next_f64();
            if value > 0.0 {
                break value;
            }
        };
        let u2 = self.next_f64();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
}

//=============================================================================
// Variation Set (parameters for one run)
//=============================================================================

/// A set of varied parameter values for one Monte Carlo run
#[derive(Debug, Clone)]
pub struct VariationSet {
    /// Map of component name to varied value
    pub values: HashMap<String, Value>,
}

impl VariationSet {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    /// Set a varied value
    pub fn set(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_string(), value);
    }

    /// Get a varied value, falling back to nominal
    pub fn get(&self, name: &str, nominal: Value) -> Value {
        *self.values.get(name).unwrap_or(&nominal)
    }
}

impl Default for VariationSet {
    fn default() -> Self {
        Self::new()
    }
}

//=============================================================================
// Monte Carlo Runner
//=============================================================================

/// Runner for Monte Carlo analysis
pub struct MonteCarloRunner {
    config: MonteCarloConfig,
    tolerances: BTreeMap<String, (Value, Tolerance)>, // stable name -> (nominal, tolerance)
}

impl MonteCarloRunner {
    /// Create runner with configuration
    pub fn new(config: MonteCarloConfig) -> Self {
        Self {
            config,
            tolerances: BTreeMap::new(),
        }
    }

    /// Register a component with tolerance
    pub fn add_component(&mut self, name: &str, nominal: Value, tolerance: Tolerance) {
        self.tolerances
            .insert(name.to_string(), (nominal, tolerance));
    }

    /// Generate variation set for one run
    fn generate_variations(
        &self,
        rng: &mut Xorshift128Plus,
        lot_values: &HashMap<String, Value>,
    ) -> VariationSet {
        let mut variations = VariationSet::new();

        for (name, (nominal, tol)) in &self.tolerances {
            let mut value = *nominal;

            // Apply lot variation (same for all instances in this run)
            if let Some(_lot) = &tol.lot
                && let Some(&lot_factor) = lot_values.get(name)
            {
                value = lot_factor;
            }

            // Apply device variation (different for each instance)
            if let Some(dev) = &tol.dev {
                value = dev.sample(rng, value);
            }

            variations.set(name, value);
        }

        variations
    }

    /// Generate lot-level variations (once per run)
    fn generate_lot_variations(&self, rng: &mut Xorshift128Plus) -> HashMap<String, Value> {
        let mut lot_values = HashMap::new();

        for (name, (nominal, tol)) in &self.tolerances {
            if let Some(lot) = &tol.lot {
                let varied = lot.sample(rng, *nominal);
                lot_values.insert(name.clone(), varied);
            }
        }

        lot_values
    }

    /// Run Monte Carlo analysis
    ///
    /// # Arguments
    /// * `run_simulation` - Closure that runs one simulation and returns output value
    ///   Takes a `&VariationSet` and returns `Result<HashMap<String, Value>, E>`
    ///
    /// # Returns
    /// Statistics, or a configuration, resource, or host-entropy error.
    pub fn run<F, E>(&self, run_simulation: F) -> Result<MonteCarloResult, SimulationError>
    where
        F: FnMut(&VariationSet) -> Result<HashMap<String, Value>, E>,
    {
        self.run_with_abort(run_simulation, &NoAbort)
    }

    /// Run with cooperative cancellation between sampling and callback steps.
    /// The callback must poll the same signal during long-running solves.
    pub fn run_with_abort<F, E>(
        &self,
        mut run_simulation: F,
        abort: &dyn AbortSignal,
    ) -> Result<MonteCarloResult, SimulationError>
    where
        F: FnMut(&VariationSet) -> Result<HashMap<String, Value>, E>,
    {
        if abort.is_aborted() {
            return Err(SimulationError::from_abort(abort));
        }
        for (field, value) in [
            ("monte_carlo.num_runs", self.config.num_runs),
            ("monte_carlo.histogram_bins", self.config.histogram_bins),
        ] {
            if value == 0 {
                return Err(SimulationConfigError::InvalidCount { field, value }.into());
            }
        }
        ResourceLimitError::ensure(
            ResourceKind::BatchRuns,
            self.config.num_runs,
            self.config.resource_limits.max_batch_runs,
        )?;
        let histogram_values = self
            .config
            .histogram_bins
            .checked_mul(2)
            .and_then(|count| count.checked_add(1))
            .ok_or_else(|| {
                SimulationError::Circuit(
                    "Monte Carlo histogram bins overflow result cardinality".to_owned(),
                )
            })?;
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            histogram_values,
            self.config.resource_limits.max_result_values,
        )?;
        confidence::validate_request(
            self.config.confidence_pct,
            self.config.confidence_method,
            self.config.num_runs,
            1,
            self.config.resource_limits,
        )?;
        for (name, (nominal, tolerance)) in &self.tolerances {
            if name.trim().is_empty() || !nominal.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "Monte Carlo component '{name}' requires a nonempty name and finite nominal value",
                )));
            }
            for distribution in [tolerance.lot, tolerance.dev].into_iter().flatten() {
                distribution.validate()?;
            }
        }
        let seed = match self.config.seed {
            Some(seed) => seed,
            None => {
                let mut entropy = [0; 8];
                getrandom::fill(&mut entropy).map_err(|error| {
                    SimulationError::Circuit(format!(
                        "Monte Carlo could not obtain a seed from the host: {error}",
                    ))
                })?;
                u64::from_le_bytes(entropy)
            }
        };
        let mut rng = Xorshift128Plus::new(seed);
        let mut all_outputs: HashMap<String, Vec<Value>> = HashMap::new();
        let mut output_schema: Option<Vec<String>> = None;
        let mut num_failures = 0;
        let mut sample_values = 0usize;

        for _run in 0..self.config.num_runs {
            if abort.is_aborted() {
                return Err(SimulationError::from_abort(abort));
            }
            // Generate lot-level variations for this run
            let lot_values = self.generate_lot_variations(&mut rng);

            // Generate device-level variations
            let variations = self.generate_variations(&mut rng, &lot_values);
            if variations.values.values().any(|value| !value.is_finite()) {
                return Err(SimulationError::Circuit(
                    "Monte Carlo variation overflowed a finite parameter value".to_owned(),
                ));
            }

            // Run simulation
            let outcome = run_simulation(&variations);
            if abort.is_aborted() {
                return Err(SimulationError::from_abort(abort));
            }
            match outcome {
                Ok(outputs) => {
                    let mut names = outputs.keys().cloned().collect::<Vec<_>>();
                    names.sort();
                    let valid = !names.is_empty()
                        && outputs.values().all(|value| value.is_finite())
                        && output_schema
                            .as_ref()
                            .is_none_or(|expected| expected == &names);
                    if !valid {
                        num_failures += 1;
                        continue;
                    }
                    if output_schema.is_none() {
                        confidence::validate_request(
                            self.config.confidence_pct,
                            self.config.confidence_method,
                            self.config.num_runs,
                            names.len(),
                            self.config.resource_limits,
                        )?;
                    }
                    sample_values = sample_values.saturating_add(names.len());
                    ResourceLimitError::ensure(
                        ResourceKind::ResultValues,
                        sample_values.saturating_add(
                            names
                                .len()
                                .saturating_mul(histogram_values.saturating_add(7)),
                        ),
                        self.config.resource_limits.max_result_values,
                    )?;
                    if output_schema.is_none() {
                        output_schema = Some(names.clone());
                    }
                    for name in names {
                        // The schema came directly from this map, so lookup
                        // cannot fail. Keeping insertion after whole-run
                        // validation preserves equal sample cardinality.
                        let value = outputs[&name];
                        all_outputs.entry(name).or_default().push(value);
                    }
                }
                Err(_) => {
                    num_failures += 1;
                }
            }
        }

        // Compute statistics
        let variables: HashMap<String, VariableStatistics> = all_outputs
            .into_iter()
            .map(|(name, samples)| {
                if abort.is_aborted() {
                    return Err(SimulationError::from_abort(abort));
                }
                let stats =
                    VariableStatistics::from_samples(&name, samples, self.config.histogram_bins);
                Ok((name, stats))
            })
            .collect::<Result<_, SimulationError>>()?;

        if abort.is_aborted() {
            return Err(SimulationError::from_abort(abort));
        }
        let mut result = MonteCarloResult {
            num_runs: self.config.num_runs,
            variables,
            all_converged: num_failures == 0,
            num_failures,
            sampling: Some(MonteCarloSampling {
                seed,
                policy: "component-xoroshiro128plus-2018-v2",
            }),
            confidence: None,
        };
        result.compute_mean_confidence(
            self.config.confidence_pct,
            self.config.confidence_method,
            self.config.resource_limits,
            abort,
        )?;
        Ok(result)
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gaussian_redraws_a_zero_uniform_sample() {
        let mut rng = Xorshift128Plus {
            s0: 1,
            s1: u64::MAX,
        };
        assert!(rng.next_gaussian().is_finite());
    }

    #[test]
    fn statistics_preserve_representable_moments_at_extreme_scales() {
        let cancellation =
            VariableStatistics::from_samples("cancellation", vec![1e16, 1.0, -1e16], 10);
        assert!((cancellation.mean - 1.0 / 3.0).abs() < 1e-15);
        let constant = VariableStatistics::from_samples("constant", vec![f64::MAX; 4], 10);
        assert_eq!(constant.mean, f64::MAX);
        assert_eq!(constant.std_dev, 0.0);
        let close = VariableStatistics::from_samples("close", vec![1e16, 1e16 + 2.0], 2);
        assert_eq!(close.mean, 1e16);
        assert!((close.std_dev - 2.0_f64.sqrt()).abs() < 1e-15);
        for scale in [1e-300, 1e300] {
            let statistics =
                VariableStatistics::from_samples("scaled", vec![-scale, 0.0, scale], 10);
            assert_eq!(statistics.mean, 0.0);
            assert!((statistics.std_dev / scale - 1.0).abs() < 1e-15);
            assert_eq!(statistics.histogram.iter().sum::<usize>(), 3);
        }
    }

    #[test]
    fn variable_statistics_preserve_invalid_evidence_as_invalid() {
        let statistics = VariableStatistics::from_samples("gain", vec![1.0, Value::NAN], 10);
        assert_eq!(statistics.samples.len(), 2);
        assert!(statistics.mean.is_nan());
        assert!(statistics.std_dev.is_nan());
        assert!(statistics.min.is_nan());
        assert!(statistics.max.is_nan());
        assert!(statistics.histogram.is_empty());
        assert!(statistics.bin_edges.is_empty());
        assert!(statistics.percentile(50.0).is_nan());
    }

    #[test]
    fn monte_carlo_rejects_a_whole_nonfinite_trial_without_shortening_variables() {
        let runner = MonteCarloRunner::new(MonteCarloConfig::new(3).with_seed(7));
        let mut run = 0;
        let result = runner
            .run::<_, ()>(|_| {
                let outputs = match run {
                    0 => HashMap::from([("gain".to_string(), 1.0), ("offset".to_string(), 2.0)]),
                    1 => HashMap::from([
                        ("gain".to_string(), Value::NAN),
                        ("offset".to_string(), 99.0),
                    ]),
                    _ => HashMap::from([("gain".to_string(), 3.0), ("offset".to_string(), 4.0)]),
                };
                run += 1;
                Ok(outputs)
            })
            .unwrap();

        assert_eq!(result.num_failures, 1);
        assert!(!result.all_converged);
        assert_eq!(result.variables["gain"].samples, vec![1.0, 3.0]);
        assert_eq!(result.variables["offset"].samples, vec![2.0, 4.0]);
    }

    #[test]
    fn monte_carlo_rejects_output_schema_drift_as_a_failed_trial() {
        let runner = MonteCarloRunner::new(MonteCarloConfig::new(2).with_seed(9));
        let mut run = 0;
        let result = runner
            .run::<_, ()>(|_| {
                run += 1;
                Ok(if run == 1 {
                    HashMap::from([("gain".to_string(), 1.0), ("offset".to_string(), 2.0)])
                } else {
                    HashMap::from([("gain".to_string(), 3.0)])
                })
            })
            .unwrap();

        assert_eq!(result.num_failures, 1);
        assert_eq!(result.variables["gain"].samples, vec![1.0]);
        assert_eq!(result.variables["offset"].samples, vec![2.0]);
    }
}
