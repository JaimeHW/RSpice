use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    /// Genetic algorithm (Global)
    Genetic,
    /// Simulated Annealing
    SimulatedAnnealing,
}

// =============================================================================
// Optimizer State & Result
// =============================================================================

/// Current state of the optimization process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizerState {
    pub iteration: usize,
    pub current_cost: f64,
    pub best_cost: f64,
    pub elapsed_secs: f64,
    pub converged: bool,
    pub var_values: HashMap<String, f64>,
}

/// Final result of optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub best_vars: HashMap<String, f64>,
    pub final_cost: f64,
    pub iterations: usize,
    pub total_time: f64,
    pub success: bool,
}

// =============================================================================
// Optimizer Engine
// =============================================================================

/// Configuration for the optimizer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizerConfig {
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

impl Default for OptimizerConfig {
    fn default() -> Self {
        Self {
            algorithm: OptimizerAlgo::GradientDescent,
            max_iterations: 100,
            cost_tolerance: 1e-8,
            var_tolerance: 1e-6,
            fd_step: 1e-4,     // Spectre-standard: 0.01% perturbation
            initial_step: 0.1, // 10% of range initial step
            min_step: 1e-8,
            sa_initial_temp: 100.0,
            sa_cooling_rate: 0.95,
        }
    }
}
