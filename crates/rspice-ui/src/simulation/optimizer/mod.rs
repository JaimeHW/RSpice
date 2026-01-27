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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

// =============================================================================
// Optimization Goal
// =============================================================================

/// Strategy for meeting an optimization goal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalStrategy {
    /// Minimize value (e.g., power, area)
    Minimize,
    /// Maximize value (e.g., gain, bandwidth)
    Maximize,
    /// Hit target value precisely
    Target,
    /// Stay within range
    Range,
}

/// A target goal for optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationGoal {
    /// Name of the measurement/signal
    pub target: String,
    /// Strategy to use
    pub strategy: GoalStrategy,
    /// Target value (for Target strategy)
    pub target_val: Option<f64>,
    /// Acceptable range (min, max)
    pub range: Option<(f64, f64)>,
    /// Weight/Priority (0.0 - 1.0)
    pub weight: f32,
}

impl OptimizationGoal {
    /// Create a maximization goal
    pub fn maximize(target: impl Into<String>) -> Self {
        Self {
            target: target.into(),
            strategy: GoalStrategy::Maximize,
            target_val: None,
            range: None,
            weight: 1.0,
        }
    }

    /// Create a minimization goal
    pub fn minimize(target: impl Into<String>) -> Self {
        Self {
            target: target.into(),
            strategy: GoalStrategy::Minimize,
            target_val: None,
            range: None,
            weight: 1.0,
        }
    }

    /// Create a target goal
    pub fn hit_target(target: impl Into<String>, val: f64) -> Self {
        Self {
            target: target.into(),
            strategy: GoalStrategy::Target,
            target_val: Some(val),
            range: None,
            weight: 1.0,
        }
    }

    /// Calculate cost (error) for a given value
    /// 0.0 means perfect match, higher means worse
    pub fn calculate_cost(&self, current_val: f64) -> f64 {
        match self.strategy {
            GoalStrategy::Maximize => {
                // Return 1/val or 1-val depending on normalization
                if current_val > 0.0 {
                    1.0 / current_val
                } else {
                    f64::MAX
                }
            }
            GoalStrategy::Minimize => current_val.abs(),
            GoalStrategy::Target => {
                let target = self.target_val.unwrap_or(0.0);
                (current_val - target).powi(2)
            }
            GoalStrategy::Range => {
                if let Some((min, max)) = self.range {
                    if current_val < min {
                        (min - current_val).powi(2)
                    } else if current_val > max {
                        (current_val - max).powi(2)
                    } else {
                        0.0
                    }
                } else {
                    0.0
                }
            }
        }
    }
}

// =============================================================================
// Design Variables
// =============================================================================

/// A design variable that can be tuned by the optimizer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignVar {
    /// Variable name (must match schematic VAR)
    pub name: String,
    /// Current value
    pub value: f64,
    /// Nominal/Center value
    pub nominal: f64,
    /// Lower bound
    pub min: f64,
    /// Upper bound
    pub max: f64,
    /// Hard constraint? (cannot exceed)
    pub hard: bool,
}

impl DesignVar {
    pub fn new(name: impl Into<String>, value: f64, min: f64, max: f64) -> Self {
        Self {
            name: name.into(),
            value,
            nominal: value,
            min,
            max,
            hard: true,
        }
    }

    /// Update value with clamping
    pub fn update(&mut self, new_val: f64) {
        self.value = new_val.clamp(self.min, self.max);
    }
}

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

/// Core engine for executing optimization runs
pub struct OptimizerEngine {
    /// Design goals
    goals: Vec<OptimizationGoal>,
    /// Design variables
    variables: Vec<DesignVar>,
    /// Algorithm to use
    algorithm: OptimizerAlgo,
    /// Max iterations
    max_iterations: usize,
    /// Cost tolerance
    tolerance: f64,
}

impl Default for OptimizerEngine {
    fn default() -> Self {
        Self {
            goals: Vec::new(),
            variables: Vec::new(),
            algorithm: OptimizerAlgo::GradientDescent,
            max_iterations: 100,
            tolerance: 1e-6,
        }
    }
}

impl OptimizerEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_algorithm(mut self, algo: OptimizerAlgo) -> Self {
        self.algorithm = algo;
        self
    }

    pub fn add_goal(&mut self, goal: OptimizationGoal) {
        self.goals.push(goal);
    }

    pub fn add_var(&mut self, var: DesignVar) {
        self.variables.push(var);
    }

    /// Calculate total cost across all goals
    pub fn calculate_total_cost(&self, current_measurements: &HashMap<String, f64>) -> f64 {
        let mut total = 0.0;
        for goal in &self.goals {
            if let Some(&val) = current_measurements.get(&goal.target) {
                total += goal.calculate_cost(val) * goal.weight as f64;
            } else {
                total += 1e9; // Penalty for missing measurement
            }
        }
        total
    }

    /// Single step of the optimizer (commercial local search pattern)
    pub fn step(&mut self, current_cost: f64) -> HashMap<String, f64> {
        // In a real implementation, this would compute gradients or update pattern
        // Here we implement a simple gradient approximation placeholder
        let mut next_vars = HashMap::new();
        let step_size = 0.01;

        for var in &mut self.variables {
            // Simplified: nudge in direction that potentially reduces cost
            // (In reality, we'd run sensitivity simulations to get actual gradients)
            let nudge = (var.max - var.min) * step_size;
            var.update(var.value - (nudge * (current_cost.min(1.0))));
            next_vars.insert(var.name.clone(), var.value);
        }

        next_vars
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goal_cost_calculation() {
        let max_gain = OptimizationGoal::maximize("gain");
        assert!(max_gain.calculate_cost(100.0) < max_gain.calculate_cost(50.0));

        let target_bw = OptimizationGoal::hit_target("bw", 1e6);
        assert_eq!(target_bw.calculate_cost(1e6), 0.0);
        assert!(target_bw.calculate_cost(1.1e6) > 0.0);
    }

    #[test]
    fn test_var_clamping() {
        let mut var = DesignVar::new("width", 10e-6, 1e-6, 50e-6);
        var.update(60e-6);
        assert_eq!(var.value, 50e-6);
        var.update(0.5e-6);
        assert_eq!(var.value, 1e-6);
    }

    #[test]
    fn test_total_cost() {
        let mut engine = OptimizerEngine::new();
        engine.add_goal(OptimizationGoal::hit_target("v_out", 1.2));

        let mut measurements = HashMap::new();
        measurements.insert("v_out".to_string(), 1.2);

        assert_eq!(engine.calculate_total_cost(&measurements), 0.0);

        measurements.insert("v_out".to_string(), 1.3);
        assert!(engine.calculate_total_cost(&measurements) > 0.0);
    }
}
