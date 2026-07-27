//! Optimizer Engine
//!
//! Iterative optimization framework for automated circuit parameter tuning.
//! Supports local and global optimization algorithms to meet design goals.
//!
//! # Features
//!
//! - Multi-objective optimization with weighted goals
//! - Parametric design variables with range constraints
//! - Gradient-based (Local) and Heuristic (Global) algorithms
//! - Convergence tracking and history
//! - Closed-loop simulation orchestration

use std::collections::HashMap;

mod algorithms;
mod design_var;
mod engine_core;
mod goal;
mod lifecycle;
mod types;

pub use design_var::DesignVar;
pub use goal::{GoalStrategy, OptimizationGoal};
pub use types::{OptimizationResult, OptimizerAlgo, OptimizerConfig, OptimizerState};
/// Core engine for executing optimization runs
///
/// Implements Spectre-compatible optimization algorithms:
/// - Gradient Descent with finite-difference gradients
/// - Pattern Search (Hooke-Jeeves direct search)
/// - Simulated Annealing for global optimization
pub struct OptimizerEngine {
    /// Design goals
    goals: Vec<OptimizationGoal>,
    /// Design variables
    variables: Vec<DesignVar>,
    /// Configuration
    config: OptimizerConfig,
    /// Current iteration
    iteration: usize,
    /// Best cost seen
    best_cost: f64,
    /// Best variable values
    best_vars: HashMap<String, f64>,
    /// Gradient vector (for gradient-based methods)
    gradient: Vec<f64>,
    /// Current step size (adaptive)
    step_size: f64,
    /// SA temperature (for simulated annealing)
    temperature: f64,
    /// Iteration history for convergence analysis
    cost_history: Vec<f64>,
    /// RNG state for stochastic algorithms
    rng_state: u64,
}

impl Default for OptimizerEngine {
    fn default() -> Self {
        Self::new()
    }
}
