//! Monte Carlo Analysis for Statistical Circuit Simulation
//!
//! Provides statistical analysis with component tolerances, supporting:
//! - Gaussian and uniform distributions
//! - Lot and device tolerances
//! - Statistical output (mean, std, min, max, histograms)
//! - Reproducible runs via seeding
//!
//! # Example
//! ```ignore
//! // Component definitions with tolerances:
//! // R1 1 0 1k tol=5%     - 5% uniform tolerance
//! // R2 2 0 1k lot=2% dev=1%  - lot-to-lot + device-to-device variation
//!
//! let config = MonteCarloConfig::new(1000).with_seed(42);
//! let runner = MonteCarloRunner::new(config);
//! let results = runner.run(&netlist, |n| engine.run_tran(n, 1e-3, 100e-6));
//! println!("Mean output: {} ± {}", results.mean("V(out)"), results.std_dev("V(out)"));
//! ```

use crate::Value;
use std::collections::HashMap;

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

    /// Sample from this distribution using a random number generator
    pub fn sample(&self, rng: &mut Xorshift128Plus, nominal: Value) -> Value {
        match *self {
            Distribution::Gaussian { sigma } => {
                let std_dev = nominal * sigma;
                let z = rng.next_gaussian();
                nominal + z * std_dev
            }
            Distribution::Uniform { tolerance } => {
                let delta = nominal * tolerance;
                let u = rng.next_f64();
                nominal + (2.0 * u - 1.0) * delta
            }
            Distribution::WorstCase { tolerance } => {
                let delta = nominal * tolerance;
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

    /// Create a tolerance with lot-to-lot variation only
    pub fn lot_only(pct: Value) -> Self {
        Self {
            lot: Some(Distribution::Uniform {
                tolerance: pct / 100.0,
            }),
            dev: None,
        }
    }

    /// Create lot + dev tolerance
    pub fn lot_and_dev(lot_pct: Value, dev_pct: Value) -> Self {
        Self {
            lot: Some(Distribution::Uniform {
                tolerance: lot_pct / 100.0,
            }),
            dev: Some(Distribution::Uniform {
                tolerance: dev_pct / 100.0,
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

    /// Random seed for reproducibility (None = random each time)
    pub seed: Option<u64>,

    /// Number of histogram bins for output distribution
    pub histogram_bins: usize,

    /// Confidence interval percentage (95 = 95%)
    pub confidence_pct: Value,
}

impl MonteCarloConfig {
    /// Create new configuration with specified number of runs
    pub fn new(num_runs: usize) -> Self {
        Self {
            num_runs,
            seed: None,
            histogram_bins: 50,
            confidence_pct: 95.0,
        }
    }

    /// Set random seed for reproducibility
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set number of histogram bins
    pub fn with_bins(mut self, bins: usize) -> Self {
        self.histogram_bins = bins;
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
}

impl VariableStatistics {
    /// Create from samples
    pub fn from_samples(name: &str, samples: Vec<Value>, num_bins: usize) -> Self {
        let n = samples.len() as Value;

        // Mean
        let mean = samples.iter().sum::<Value>() / n;

        // Standard deviation
        let variance =
            samples.iter().map(|x| (x - mean).powi(2)).sum::<Value>() / (n - 1.0).max(1.0);
        let std_dev = variance.sqrt();

        // Min/max
        let min = samples.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = samples.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

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
        }
    }

    fn compute_histogram(
        samples: &[Value],
        num_bins: usize,
        min: Value,
        max: Value,
    ) -> (Vec<usize>, Vec<Value>) {
        let range = max - min;
        if range <= 0.0 || num_bins == 0 {
            return (vec![samples.len()], vec![min, max]);
        }

        let bin_width = range / num_bins as Value;
        let mut histogram = vec![0usize; num_bins];
        let bin_edges: Vec<Value> = (0..=num_bins)
            .map(|i| min + (i as Value) * bin_width)
            .collect();

        for &sample in samples {
            let bin = ((sample - min) / bin_width).floor() as usize;
            let bin = bin.min(num_bins - 1);
            histogram[bin] += 1;
        }

        (histogram, bin_edges)
    }

    /// Get percentile value (0-100)
    pub fn percentile(&self, pct: Value) -> Value {
        if self.samples.is_empty() {
            return 0.0;
        }

        let mut sorted = self.samples.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

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

/// Results from a complete Monte Carlo analysis
#[derive(Debug, Clone)]
pub struct MonteCarloResult {
    /// Number of runs completed
    pub num_runs: usize,
    /// Statistics for each output variable
    pub variables: HashMap<String, VariableStatistics>,
    /// Whether all runs converged
    pub all_converged: bool,
    /// Number of failed runs
    pub num_failures: usize,
}

impl MonteCarloResult {
    pub fn new() -> Self {
        Self {
            num_runs: 0,
            variables: HashMap::new(),
            all_converged: true,
            num_failures: 0,
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
// Random Number Generator (Xorshift128+)
//=============================================================================

/// Fast PRNG suitable for Monte Carlo simulation
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
        let u1 = self.next_f64();
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
    tolerances: HashMap<String, (Value, Tolerance)>, // name -> (nominal, tolerance)
}

impl MonteCarloRunner {
    /// Create runner with configuration
    pub fn new(config: MonteCarloConfig) -> Self {
        Self {
            config,
            tolerances: HashMap::new(),
        }
    }

    /// Register a component with tolerance
    pub fn add_component(&mut self, name: &str, nominal: Value, tolerance: Tolerance) {
        self.tolerances
            .insert(name.to_string(), (nominal, tolerance));
    }

    /// Generate variation set for one run
    pub fn generate_variations(
        &self,
        rng: &mut Xorshift128Plus,
        lot_values: &HashMap<String, Value>,
    ) -> VariationSet {
        let mut variations = VariationSet::new();

        for (name, (nominal, tol)) in &self.tolerances {
            let mut value = *nominal;

            // Apply lot variation (same for all instances in this run)
            if let Some(_lot) = &tol.lot {
                if let Some(&lot_factor) = lot_values.get(name) {
                    value = lot_factor;
                }
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
    pub fn generate_lot_variations(&self, rng: &mut Xorshift128Plus) -> HashMap<String, Value> {
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
    /// `MonteCarloResult` with statistics for all output variables
    pub fn run<F, E>(&self, mut run_simulation: F) -> MonteCarloResult
    where
        F: FnMut(&VariationSet) -> Result<HashMap<String, Value>, E>,
    {
        let seed = self.config.seed.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(12345)
        });

        let mut rng = Xorshift128Plus::new(seed);
        let mut all_outputs: HashMap<String, Vec<Value>> = HashMap::new();
        let mut num_failures = 0;

        for _run in 0..self.config.num_runs {
            // Generate lot-level variations for this run
            let lot_values = self.generate_lot_variations(&mut rng);

            // Generate device-level variations
            let variations = self.generate_variations(&mut rng, &lot_values);

            // Run simulation
            match run_simulation(&variations) {
                Ok(outputs) => {
                    for (name, value) in outputs {
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
                let stats =
                    VariableStatistics::from_samples(&name, samples, self.config.histogram_bins);
                (name, stats)
            })
            .collect();

        MonteCarloResult {
            num_runs: self.config.num_runs,
            variables,
            all_converged: num_failures == 0,
            num_failures,
        }
    }

    /// Run Monte Carlo analysis in parallel using rayon
    ///
    /// This method distributes simulation runs across threads for better performance
    /// on multi-core systems. Each thread gets its own RNG state derived from the
    /// base seed to ensure reproducibility.
    ///
    /// # Arguments
    /// * `run_simulation` - Thread-safe closure that runs one simulation
    ///
    /// # Returns
    /// `MonteCarloResult` with statistics for all output variables
    #[cfg(feature = "parallel")]
    pub fn run_parallel<F, E>(&self, run_simulation: F) -> MonteCarloResult
    where
        F: Fn(&VariationSet) -> Result<HashMap<String, Value>, E> + Sync,
    {
        use rayon::prelude::*;
        use std::sync::atomic::{AtomicUsize, Ordering};

        let base_seed = self.config.seed.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(12345)
        });

        let num_failures = AtomicUsize::new(0);

        // Pre-generate all variation sets (required for parallel iteration)
        let variation_sets: Vec<_> = (0..self.config.num_runs)
            .map(|run_idx| {
                // Create per-run RNG with unique seed
                let run_seed = base_seed.wrapping_add(run_idx as u64 * 0x9E3779B97F4A7C15);
                let mut rng = Xorshift128Plus::new(run_seed);

                // Generate lot-level variations for this run
                let lot_values = self.generate_lot_variations(&mut rng);

                // Generate device-level variations
                self.generate_variations(&mut rng, &lot_values)
            })
            .collect();

        // Run simulations in parallel and collect results
        let results: Vec<_> = variation_sets
            .par_iter()
            .filter_map(|variations| match run_simulation(variations) {
                Ok(outputs) => Some(outputs),
                Err(_) => {
                    num_failures.fetch_add(1, Ordering::Relaxed);
                    None
                }
            })
            .collect();

        // Aggregate results
        let mut all_outputs: HashMap<String, Vec<Value>> = HashMap::new();
        for outputs in results {
            for (name, value) in outputs {
                all_outputs.entry(name).or_default().push(value);
            }
        }

        // Compute statistics
        let variables: HashMap<String, VariableStatistics> = all_outputs
            .into_iter()
            .map(|(name, samples)| {
                let stats =
                    VariableStatistics::from_samples(&name, samples, self.config.histogram_bins);
                (name, stats)
            })
            .collect();

        let failures = num_failures.load(Ordering::Relaxed);

        MonteCarloResult {
            num_runs: self.config.num_runs,
            variables,
            all_converged: failures == 0,
            num_failures: failures,
        }
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xorshift_basics() {
        let mut rng = Xorshift128Plus::new(12345);

        // Should produce different values
        let v1 = rng.next_f64();
        let v2 = rng.next_f64();
        assert!(v1 != v2);

        // Values should be in [0, 1)
        for _ in 0..100 {
            let v = rng.next_f64();
            assert!(v >= 0.0 && v < 1.0);
        }
    }

    #[test]
    fn test_xorshift_reproducibility() {
        let mut rng1 = Xorshift128Plus::new(42);
        let mut rng2 = Xorshift128Plus::new(42);

        for _ in 0..10 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn test_gaussian_distribution() {
        let mut rng = Xorshift128Plus::new(12345);
        let dist = Distribution::Gaussian { sigma: 0.1 };

        let mut samples = Vec::new();
        let nominal = 1000.0;

        for _ in 0..10000 {
            samples.push(dist.sample(&mut rng, nominal));
        }

        let mean: Value = samples.iter().sum::<Value>() / samples.len() as Value;
        let variance: Value =
            samples.iter().map(|x| (x - mean).powi(2)).sum::<Value>() / samples.len() as Value;
        let std_dev = variance.sqrt();

        // Mean should be close to nominal
        assert!((mean - nominal).abs() < 10.0);

        // Std dev should be close to 10% of 1000 = 100
        assert!((std_dev - 100.0).abs() < 10.0);
    }

    #[test]
    fn test_uniform_distribution() {
        let mut rng = Xorshift128Plus::new(12345);
        let dist = Distribution::Uniform { tolerance: 0.05 };

        let mut samples = Vec::new();
        let nominal = 1000.0;

        for _ in 0..10000 {
            samples.push(dist.sample(&mut rng, nominal));
        }

        let min = samples.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = samples.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        // All values should be within ±5% (950 to 1050)
        assert!(min >= 950.0 - 0.1);
        assert!(max <= 1050.0 + 0.1);
    }

    #[test]
    fn test_worst_case_distribution_samples_extremes_only() {
        let mut rng = Xorshift128Plus::new(12345);
        let dist = Distribution::WorstCase { tolerance: 0.05 };
        let nominal = 1000.0;
        let low = nominal * 0.95;
        let high = nominal * 1.05;

        let mut saw_low = false;
        let mut saw_high = false;
        for _ in 0..200 {
            let sample = dist.sample(&mut rng, nominal);
            assert!(
                (sample - low).abs() < 1e-12 || (sample - high).abs() < 1e-12,
                "worst-case sample must be at tolerance edge"
            );
            if (sample - low).abs() < 1e-12 {
                saw_low = true;
            }
            if (sample - high).abs() < 1e-12 {
                saw_high = true;
            }
        }

        assert!(saw_low, "expected to observe lower worst-case sample");
        assert!(saw_high, "expected to observe upper worst-case sample");
    }

    #[test]
    fn test_variable_statistics() {
        let samples: Vec<Value> = (0..100).map(|i| i as Value).collect();
        let stats = VariableStatistics::from_samples("test", samples, 10);

        // Mean of 0..100 = 49.5
        assert!((stats.mean - 49.5).abs() < 0.1);

        // Min = 0, Max = 99
        assert!((stats.min - 0.0).abs() < 0.1);
        assert!((stats.max - 99.0).abs() < 0.1);

        // Histogram should have 10 bins
        assert_eq!(stats.histogram.len(), 10);

        // Each bin should have ~10 values
        for count in &stats.histogram {
            assert!(*count >= 8 && *count <= 12);
        }
    }

    #[test]
    fn test_monte_carlo_runner() {
        let config = MonteCarloConfig::new(100).with_seed(42);
        let mut runner = MonteCarloRunner::new(config);

        // Register components
        runner.add_component("R1", 1000.0, Tolerance::uniform(5.0));
        runner.add_component("R2", 2000.0, Tolerance::uniform(10.0));

        // Simple simulation: output = R1 + R2
        let result = runner.run(|variations| {
            let r1 = variations.get("R1", 1000.0);
            let r2 = variations.get("R2", 2000.0);
            let mut outputs = HashMap::new();
            outputs.insert("sum".to_string(), r1 + r2);
            Ok::<_, ()>(outputs)
        });

        assert_eq!(result.num_runs, 100);
        assert!(result.all_converged);

        // Mean should be close to 3000
        let mean = result.mean("sum").unwrap();
        assert!((mean - 3000.0).abs() < 100.0);
    }

    #[test]
    fn test_lot_and_device_tolerance() {
        let config = MonteCarloConfig::new(50).with_seed(123);
        let mut runner = MonteCarloRunner::new(config);

        // 2% lot, 1% device tolerance
        runner.add_component("R1", 1000.0, Tolerance::lot_and_dev(2.0, 1.0));

        let result = runner.run(|variations| {
            let r1 = variations.get("R1", 1000.0);
            let mut outputs = HashMap::new();
            outputs.insert("R1".to_string(), r1);
            Ok::<_, ()>(outputs)
        });

        // All values should be within ~5% of nominal (lot + dev)
        let range = result.range("R1").unwrap();
        assert!(range.0 > 900.0);
        assert!(range.1 < 1100.0);
    }

    #[test]
    fn test_percentile() {
        let samples: Vec<Value> = (0..100).map(|i| i as Value).collect();
        let stats = VariableStatistics::from_samples("test", samples, 10);

        // 50th percentile should be ~50
        let p50 = stats.percentile(50.0);
        assert!((p50 - 49.0).abs() < 2.0);

        // 90th percentile should be ~90
        let p90 = stats.percentile(90.0);
        assert!((p90 - 89.0).abs() < 2.0);
    }

    // =========================================================================
    // Commercial-Grade Comprehensive Tests
    // =========================================================================

    /// Test 3-sigma range calculation for Gaussian distribution.
    /// 99.7% of samples should fall within mean ± 3*sigma.
    #[test]
    fn test_three_sigma_range() {
        let mut rng = Xorshift128Plus::new(54321);
        let dist = Distribution::Gaussian { sigma: 0.1 }; // 10% sigma
        let nominal: f64 = 1000.0;

        let mut samples = Vec::new();
        for _ in 0..10000 {
            samples.push(dist.sample(&mut rng, nominal));
        }

        let stats = VariableStatistics::from_samples("test", samples.clone(), 50);
        let (low, high) = stats.three_sigma_range();

        // Count how many samples fall outside 3-sigma
        let outliers = samples.iter().filter(|&&x| x < low || x > high).count();

        // Should be < 0.3% outliers for true Gaussian
        let outlier_pct = (outliers as f64) / (samples.len() as f64) * 100.0;
        assert!(
            outlier_pct < 1.0,
            "Too many outliers outside 3-sigma: {}%",
            outlier_pct
        );
    }

    /// Test Gaussian distribution mean converges to nominal.
    #[test]
    fn test_gaussian_mean_convergence() {
        let mut rng = Xorshift128Plus::new(99999);
        let dist = Distribution::Gaussian { sigma: 0.05 };
        let nominal: f64 = 500.0;

        let mut samples: Vec<f64> = Vec::new();
        for _ in 0..5000 {
            samples.push(dist.sample(&mut rng, nominal));
        }

        let mean: f64 = samples.iter().sum::<f64>() / samples.len() as f64;

        // Mean should be within 1% of nominal for 5000 samples
        let error_pct = ((mean - nominal) / nominal).abs() * 100.0;
        assert!(
            error_pct < 1.0,
            "Mean {} too far from nominal {}: {}%",
            mean,
            nominal,
            error_pct
        );
    }

    /// Test histogram bin count accuracy.
    #[test]
    fn test_histogram_accuracy() {
        // Linear samples 0-99 with 10 bins should have exactly 10 per bin
        let samples: Vec<Value> = (0..100).map(|i| i as Value).collect();
        let stats = VariableStatistics::from_samples("linear", samples, 10);

        assert_eq!(stats.histogram.len(), 10);
        assert_eq!(stats.bin_edges.len(), 11); // n+1 edges for n bins

        // Total should equal sample count
        let total: usize = stats.histogram.iter().sum();
        assert_eq!(total, 100);
    }

    /// Test simulation failure tracking.
    #[test]
    fn test_failure_tracking() {
        let config = MonteCarloConfig::new(100).with_seed(42);
        let runner = MonteCarloRunner::new(config);

        let mut fail_count = 0;
        let result = runner.run(|_| {
            fail_count += 1;
            if fail_count % 3 == 0 {
                Err("simulated failure")
            } else {
                let mut outputs = HashMap::new();
                outputs.insert("x".to_string(), 1.0);
                Ok(outputs)
            }
        });

        // Every 3rd run fails = 33 failures out of 100
        assert_eq!(result.num_failures, 33);
        assert!(!result.all_converged);
    }

    /// Test no-tolerance (nominal only) handling.
    #[test]
    fn test_no_tolerance() {
        let config = MonteCarloConfig::new(50).with_seed(42);
        let mut runner = MonteCarloRunner::new(config);

        runner.add_component("R1", 1000.0, Tolerance::none());

        let result = runner.run(|variations| {
            let r1 = variations.get("R1", 1000.0);
            let mut outputs = HashMap::new();
            outputs.insert("R1".to_string(), r1);
            Ok::<_, ()>(outputs)
        });

        // All samples should be exactly nominal
        let stats = result.variables.get("R1").unwrap();
        assert!(
            (stats.min - 1000.0).abs() < 1e-10,
            "Min should be exactly nominal"
        );
        assert!(
            (stats.max - 1000.0).abs() < 1e-10,
            "Max should be exactly nominal"
        );
        assert!(
            stats.std_dev.abs() < 1e-10,
            "Std dev should be zero for no tolerance"
        );
    }

    /// Test lot-only tolerance (no device variation).
    #[test]
    fn test_lot_only_tolerance() {
        let config = MonteCarloConfig::new(100).with_seed(42);
        let mut runner = MonteCarloRunner::new(config);

        runner.add_component("R1", 1000.0, Tolerance::lot_only(5.0));

        let result = runner.run(|variations| {
            let r1 = variations.get("R1", 1000.0);
            let mut outputs = HashMap::new();
            outputs.insert("R1".to_string(), r1);
            Ok::<_, ()>(outputs)
        });

        // All samples should be within 5% of nominal
        let range = result.range("R1").unwrap();
        assert!(range.0 >= 950.0 - 1.0, "Min should be >= 95%");
        assert!(range.1 <= 1050.0 + 1.0, "Max should be <= 105%");
    }

    /// Test empty samples handling.
    #[test]
    fn test_empty_samples_percentile() {
        let samples: Vec<Value> = vec![];
        let stats = VariableStatistics::from_samples("empty", samples, 10);

        // Should return 0 for empty samples
        assert_eq!(stats.percentile(50.0), 0.0);
    }

    /// Test single sample statistics.
    #[test]
    fn test_single_sample() {
        let samples: Vec<Value> = vec![42.0];
        let stats = VariableStatistics::from_samples("single", samples, 1);

        assert!((stats.mean - 42.0).abs() < 1e-10);
        assert!((stats.min - 42.0).abs() < 1e-10);
        assert!((stats.max - 42.0).abs() < 1e-10);
        // Std dev of single sample should be 0 (or handled gracefully)
    }

    /// Test Monte Carlo result accessors with missing variable.
    #[test]
    fn test_result_accessors_missing() {
        let result = MonteCarloResult::new();

        assert!(result.mean("nonexistent").is_none());
        assert!(result.std_dev("nonexistent").is_none());
        assert!(result.range("nonexistent").is_none());
    }

    /// Test seed reproducibility across runner instances.
    #[test]
    fn test_seed_reproducibility_across_runners() {
        let config1 = MonteCarloConfig::new(50).with_seed(12345);
        let mut runner1 = MonteCarloRunner::new(config1);
        runner1.add_component("R1", 1000.0, Tolerance::uniform(10.0));

        let config2 = MonteCarloConfig::new(50).with_seed(12345);
        let mut runner2 = MonteCarloRunner::new(config2);
        runner2.add_component("R1", 1000.0, Tolerance::uniform(10.0));

        let result1 = runner1.run(|v| {
            let mut out = HashMap::new();
            out.insert("x".to_string(), v.get("R1", 1000.0));
            Ok::<_, ()>(out)
        });

        let result2 = runner2.run(|v| {
            let mut out = HashMap::new();
            out.insert("x".to_string(), v.get("R1", 1000.0));
            Ok::<_, ()>(out)
        });

        // Results should be identical with same seed
        let mean1 = result1.mean("x").unwrap();
        let mean2 = result2.mean("x").unwrap();
        assert!(
            (mean1 - mean2).abs() < 1e-10,
            "Same seed should give same results"
        );
    }

    /// Test uniform distribution bounds are respected.
    #[test]
    fn test_uniform_bounds_strict() {
        let mut rng = Xorshift128Plus::new(77777);
        let tolerance: f64 = 0.1; // 10%
        let dist = Distribution::Uniform { tolerance };
        let nominal: f64 = 100.0;

        for _ in 0..10000 {
            let sample = dist.sample(&mut rng, nominal);
            assert!(
                sample >= nominal * (1.0 - tolerance),
                "Sample {} below lower bound",
                sample
            );
            assert!(
                sample <= nominal * (1.0 + tolerance),
                "Sample {} above upper bound",
                sample
            );
        }
    }

    /// Test ConfigVariation default and builder pattern.
    #[test]
    fn test_config_builder_pattern() {
        let config = MonteCarloConfig::new(500).with_seed(42).with_bins(100);

        assert_eq!(config.num_runs, 500);
        assert_eq!(config.seed, Some(42));
        assert_eq!(config.histogram_bins, 100);
        assert!((config.confidence_pct - 95.0).abs() < 0.01);
    }

    /// Test VariationSet get with fallback.
    #[test]
    fn test_variation_set_fallback() {
        let mut vs = VariationSet::new();
        vs.set("R1", 1234.0);

        assert!((vs.get("R1", 1000.0) - 1234.0).abs() < 1e-10);
        assert!((vs.get("R2", 2000.0) - 2000.0).abs() < 1e-10); // Falls back to nominal
    }

    /// Test default trait implementations.
    #[test]
    fn test_default_implementations() {
        let tol = Tolerance::default();
        assert!(tol.lot.is_none());
        assert!(tol.dev.is_none());

        let config = MonteCarloConfig::default();
        assert_eq!(config.num_runs, 100);

        let vs = VariationSet::default();
        assert!(vs.values.is_empty());

        let result = MonteCarloResult::default();
        assert_eq!(result.num_runs, 0);
        assert!(result.all_converged);
    }
}
