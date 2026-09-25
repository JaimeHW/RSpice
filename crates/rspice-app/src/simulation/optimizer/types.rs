//! Optimizer configuration and state.

use serde::{Deserialize, Serialize};

// =============================================================================
// Optimizer Algorithm
// =============================================================================

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

// =============================================================================
// Optimizer State & Result
// =============================================================================

// =============================================================================
// Optimizer Engine
// =============================================================================

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
    rspice_simulation_contract::optimization_search::OptimizationSearchControls::default()
        .random_seed
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        let search =
            rspice_simulation_contract::optimization_search::OptimizationSearchControls::default();
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
