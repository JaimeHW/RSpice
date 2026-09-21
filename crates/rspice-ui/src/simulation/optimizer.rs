//! Optimizer Engine
//!
//! Iterative optimization framework for automated circuit parameter tuning.
//! Supports local and global optimization algorithms over a caller-provided
//! scalar cost function.
//!
//! # Features
//!
//! - Parametric design variables with range constraints
//! - Gradient-based (Local) and Heuristic (Global) algorithms
//! - Convergence tracking and history
//! - Closed-loop simulation orchestration

use std::collections::HashMap;

mod algorithms;
mod constraint;
pub use constraint::{
    OptimizationConstraint, OptimizationConstraintObservation, OptimizationScore,
    validate_optimization_constraint_result, validate_optimization_constraints,
};
mod design_var;
mod domain;
pub use domain::OptimizationVariableDomain;
mod engine_core;
mod lifecycle;
mod objective;
pub use objective::{
    OptimizationObjectiveGoal, OptimizationObjectiveObservation, OptimizationObjectiveTerm,
    validate_optimization_objectives,
};
mod types;

/// Blank retains the original measurement scale.
pub(crate) fn validate_requested_unit(unit: &str) -> Result<(), String> {
    if unit.chars().any(char::is_control) {
        return Err("Optimization units must not contain control characters".into());
    }
    if !unit.trim().is_empty() {
        rspice_core::analysis::MeasurementUnit::known(unit)?;
    }
    Ok(())
}

pub use design_var::DesignVar;
pub use types::{OptimizerAlgo, OptimizerConfig};
/// Core engine for executing optimization runs
///
/// Implements Spectre-compatible optimization algorithms:
/// - Gradient Descent with finite-difference gradients
/// - Pattern Search (Hooke-Jeeves direct search)
/// - Simulated Annealing for global optimization
pub struct OptimizerEngine {
    /// Design variables
    variables: Vec<DesignVar>,
    /// Configuration
    config: OptimizerConfig,
    /// Current iteration
    iteration: usize,
    /// Best cost seen
    best_cost: f64,
    best_violation: f64,
    /// Best variable values
    best_vars: HashMap<String, f64>,
    /// Gradient vector (for gradient-based methods)
    gradient: Vec<f64>,
    /// Current step size (adaptive)
    step_size: f64,
    /// SA temperature (for simulated annealing)
    temperature: f64,
    /// Iteration history for convergence analysis
    cost_history: Vec<OptimizationScore>,
    /// RNG state for stochastic algorithms
    rng_state: u64,
}

impl Default for OptimizerEngine {
    fn default() -> Self {
        Self::new()
    }
}
