//! Authored optimizer algorithm and execution controls.

use serde::{Deserialize, Serialize};

/// Optimization algorithm type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum OptimizerAlgo {
    /// Standard gradient descent
    #[default]
    GradientDescent,
    /// Hooke-Jeeves pattern search (Direct search)
    PatternSearch,
    /// Simulated Annealing
    SimulatedAnnealing,
}

/// Configuration for the optimizer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizerConfig {
    #[serde(default = "default_random_seed")]
    pub random_seed: u64,
    /// Algorithm to use
    pub algorithm: OptimizerAlgo,
    /// Maximum iterations
    pub max_iterations: usize,
    /// Cost tolerance for convergence
    pub cost_tolerance: f64,
    /// Variable tolerance for convergence
    pub var_tolerance: f64,
    /// Finite difference step size (relative to variable range)
    pub fd_step: f64,
    /// Initial learning rate / step size
    pub initial_step: f64,
    /// Minimum step size before declaring convergence
    pub min_step: f64,
    /// Simulated annealing initial temperature
    pub sa_initial_temp: f64,
    /// Simulated annealing cooling rate
    pub sa_cooling_rate: f64,
}

fn default_random_seed() -> u64 {
    super::OptimizationSearchControls::default().random_seed
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        let search = super::OptimizationSearchControls::default();
        Self {
            random_seed: search.random_seed,
            algorithm: OptimizerAlgo::GradientDescent,
            max_iterations: 100,
            cost_tolerance: 1e-8,
            var_tolerance: search.var_tolerance,
            fd_step: 1e-4,     // Spectre-standard: 0.01% perturbation
            initial_step: 0.1, // 10% of range initial step
            min_step: 1e-8,
            sa_initial_temp: search.sa_initial_temp,
            sa_cooling_rate: search.sa_cooling_rate,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optimizer_config_restores_legacy_seed_and_algorithm_spelling() {
        let config = OptimizerConfig::default();
        let mut saved = serde_json::to_value(&config).unwrap();
        assert_eq!(saved["algorithm"], "GradientDescent");
        saved.as_object_mut().unwrap().remove("random_seed");

        let restored: OptimizerConfig = serde_json::from_value(saved).unwrap();
        assert_eq!(restored.random_seed, config.random_seed);
        assert_eq!(restored.algorithm, config.algorithm);
        assert_eq!(restored.var_tolerance, config.var_tolerance);
        assert_eq!(restored.sa_initial_temp, config.sa_initial_temp);
        assert_eq!(restored.sa_cooling_rate, config.sa_cooling_rate);
    }
}
