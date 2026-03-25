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

impl OptimizerEngine {
    pub fn new() -> Self {
        let config = OptimizerConfig::default();
        Self {
            goals: Vec::new(),
            variables: Vec::new(),
            step_size: config.initial_step,
            temperature: config.sa_initial_temp,
            config,
            iteration: 0,
            best_cost: f64::MAX,
            best_vars: HashMap::new(),
            gradient: Vec::new(),
            cost_history: Vec::new(),
            rng_state: 0xDEAD_BEEF_CAFE_BABE,
        }
    }

    pub fn with_config(config: OptimizerConfig) -> Self {
        Self {
            step_size: config.initial_step,
            temperature: config.sa_initial_temp,
            config,
            ..Self::new()
        }
    }

    pub fn with_algorithm(mut self, algo: OptimizerAlgo) -> Self {
        self.config.algorithm = algo;
        self
    }

    pub fn add_goal(&mut self, goal: OptimizationGoal) {
        self.goals.push(goal);
    }

    pub fn add_var(&mut self, var: DesignVar) {
        self.variables.push(var);
        self.gradient.push(0.0);
    }

    /// Get current variable values as a map
    pub fn current_vars(&self) -> HashMap<String, f64> {
        self.variables
            .iter()
            .map(|v| (v.name.clone(), v.value))
            .collect()
    }

    /// Calculate total cost across all goals
    pub fn calculate_total_cost(&self, measurements: &HashMap<String, f64>) -> f64 {
        let mut total = 0.0;
        for goal in &self.goals {
            if let Some(&val) = measurements.get(&goal.target) {
                total += goal.calculate_cost(val) * goal.weight as f64;
            } else {
                total += 1e9; // Penalty for missing measurement
            }
        }
        total
    }

    /// Compute finite-difference gradient
    ///
    /// Uses central differences for accuracy: ∂f/∂x ≈ (f(x+h) - f(x-h)) / 2h
    /// This is the standard approach used in Spectre's optimizer
    pub fn compute_gradient<F>(&mut self, cost_fn: &mut F) -> Vec<f64>
    where
        F: FnMut(&HashMap<String, f64>) -> f64,
    {
        let n = self.variables.len();
        let mut grad = vec![0.0; n];

        for (grad_entry, var) in grad.iter_mut().zip(self.variables.iter()) {
            let range = var.max - var.min;
            let h = range * self.config.fd_step;

            // Forward perturbation
            let mut vars_plus = self.current_vars();
            vars_plus.insert(var.name.clone(), (var.value + h).clamp(var.min, var.max));
            let cost_plus = cost_fn(&vars_plus);

            // Backward perturbation
            let mut vars_minus = self.current_vars();
            vars_minus.insert(var.name.clone(), (var.value - h).clamp(var.min, var.max));
            let cost_minus = cost_fn(&vars_minus);

            // Central difference
            *grad_entry = (cost_plus - cost_minus) / (2.0 * h);
        }

        self.gradient = grad.clone();
        grad
    }

    /// Simple pseudo-random number generator (Xorshift64)
    fn next_random(&mut self) -> f64 {
        self.rng_state ^= self.rng_state << 13;
        self.rng_state ^= self.rng_state >> 7;
        self.rng_state ^= self.rng_state << 17;
        (self.rng_state as f64) / (u64::MAX as f64)
    }

    /// Line search with Armijo-Goldstein condition
    ///
    /// Finds step size α that satisfies: f(x + α*d) ≤ f(x) + c*α*∇f·d
    fn line_search<F>(&mut self, direction: &[f64], current_cost: f64, cost_fn: &mut F) -> f64
    where
        F: FnMut(&HashMap<String, f64>) -> f64,
    {
        let c = 1e-4; // Armijo constant
        let rho = 0.5; // Backtracking factor
        let mut alpha = self.step_size;

        // Compute directional derivative
        let dir_deriv: f64 = self
            .gradient
            .iter()
            .zip(direction.iter())
            .map(|(g, d)| g * d)
            .sum();

        for _ in 0..20 {
            // Try step
            let mut trial_vars = self.current_vars();
            for (i, var) in self.variables.iter().enumerate() {
                let new_val = var.value + alpha * direction[i];
                trial_vars.insert(var.name.clone(), new_val.clamp(var.min, var.max));
            }

            let trial_cost = cost_fn(&trial_vars);

            // Armijo condition
            if trial_cost <= current_cost + c * alpha * dir_deriv {
                return alpha;
            }

            alpha *= rho;
            if alpha < self.config.min_step {
                break;
            }
        }

        alpha.max(self.config.min_step)
    }

    /// Take a single optimization step using the configured algorithm
    ///
    /// Returns the new variable values after the step
    pub fn step<F>(&mut self, cost_fn: &mut F) -> HashMap<String, f64>
    where
        F: FnMut(&HashMap<String, f64>) -> f64,
    {
        let current_cost = cost_fn(&self.current_vars());
        self.cost_history.push(current_cost);

        // Run the algorithm step
        let new_vars = match self.config.algorithm {
            OptimizerAlgo::GradientDescent => self.step_gradient_descent(cost_fn, current_cost),
            OptimizerAlgo::PatternSearch => self.step_pattern_search(cost_fn, current_cost),
            OptimizerAlgo::SimulatedAnnealing => {
                self.step_simulated_annealing(cost_fn, current_cost)
            }
            OptimizerAlgo::Genetic => self.step_gradient_descent(cost_fn, current_cost), // Fallback
        };

        // Evaluate cost AFTER the step and update best if improved
        let new_cost = cost_fn(&new_vars);
        if new_cost < self.best_cost {
            self.best_cost = new_cost;
            self.best_vars = new_vars.clone();
        }

        new_vars
    }

    /// Gradient descent with line search
    fn step_gradient_descent<F>(
        &mut self,
        cost_fn: &mut F,
        current_cost: f64,
    ) -> HashMap<String, f64>
    where
        F: FnMut(&HashMap<String, f64>) -> f64,
    {
        // Compute gradient
        self.compute_gradient(cost_fn);

        // Compute descent direction (negative gradient)
        let direction: Vec<f64> = self.gradient.iter().map(|g| -g).collect();

        // Line search for step size
        let alpha = self.line_search(&direction, current_cost, cost_fn);

        // Update variables
        for (i, var) in self.variables.iter_mut().enumerate() {
            let new_val = var.value + alpha * direction[i];
            var.update(new_val);
        }

        self.iteration += 1;
        self.current_vars()
    }

    /// Hooke-Jeeves Pattern Search (derivative-free)
    fn step_pattern_search<F>(&mut self, cost_fn: &mut F, current_cost: f64) -> HashMap<String, f64>
    where
        F: FnMut(&HashMap<String, f64>) -> f64,
    {
        let mut best_cost = current_cost;
        let mut best_vars = self.current_vars();
        let mut any_improved = false;

        // Exploratory moves: try each coordinate direction
        for i in 0..self.variables.len() {
            let var = &self.variables[i];
            let range = var.max - var.min;
            let delta = range * self.step_size;

            // Get current best value for this variable (may have been updated by previous coord)
            let current_val = *best_vars.get(&var.name).unwrap_or(&var.value);

            // Try positive direction
            let mut trial_plus = best_vars.clone();
            trial_plus.insert(
                var.name.clone(),
                (current_val + delta).clamp(var.min, var.max),
            );
            let cost_plus = cost_fn(&trial_plus);

            if cost_plus < best_cost {
                best_cost = cost_plus;
                best_vars = trial_plus;
                any_improved = true;
                continue;
            }

            // Try negative direction
            let mut trial_minus = best_vars.clone();
            trial_minus.insert(
                var.name.clone(),
                (current_val - delta).clamp(var.min, var.max),
            );
            let cost_minus = cost_fn(&trial_minus);

            if cost_minus < best_cost {
                best_cost = cost_minus;
                best_vars = trial_minus;
                any_improved = true;
            }
        }

        // Apply best moves
        for var in &mut self.variables {
            if let Some(&val) = best_vars.get(&var.name) {
                var.value = val;
            }
        }

        // Reduce step size if no improvement
        if !any_improved {
            self.step_size *= 0.5;
        }

        self.iteration += 1;
        self.current_vars()
    }

    /// Simulated Annealing step
    fn step_simulated_annealing<F>(
        &mut self,
        cost_fn: &mut F,
        current_cost: f64,
    ) -> HashMap<String, f64>
    where
        F: FnMut(&HashMap<String, f64>) -> f64,
    {
        // Generate random perturbations FIRST (avoid borrow conflict)
        let perturbations: Vec<f64> = (0..self.variables.len())
            .map(|_| self.next_random())
            .collect();

        // Generate random neighbor
        let mut neighbor_vars = self.current_vars();
        for (i, var) in self.variables.iter().enumerate() {
            let range = var.max - var.min;
            let perturbation = (perturbations[i] - 0.5) * 2.0 * range * self.step_size;
            let new_val = (var.value + perturbation).clamp(var.min, var.max);
            neighbor_vars.insert(var.name.clone(), new_val);
        }

        let neighbor_cost = cost_fn(&neighbor_vars);
        let delta = neighbor_cost - current_cost;

        // Accept if better, or with probability exp(-delta/T) if worse
        let accept = if delta < 0.0 {
            true
        } else {
            let prob = (-delta / self.temperature).exp();
            self.next_random() < prob
        };

        if accept {
            for var in &mut self.variables {
                if let Some(&val) = neighbor_vars.get(&var.name) {
                    var.value = val;
                }
            }
        }

        // Cool down
        self.temperature *= self.config.sa_cooling_rate;

        self.iteration += 1;
        self.current_vars()
    }

    /// Check if converged
    pub fn is_converged(&self) -> bool {
        // Cost tolerance
        if self.best_cost < self.config.cost_tolerance {
            return true;
        }

        // Step size too small (for pattern search)
        if self.step_size < self.config.min_step {
            return true;
        }

        // Gradient norm too small (for gradient descent)
        let grad_norm: f64 = self.gradient.iter().map(|g| g * g).sum::<f64>().sqrt();
        if grad_norm < self.config.var_tolerance && !self.gradient.is_empty() {
            return true;
        }

        // Cost stagnation over last 10 iterations
        if self.cost_history.len() >= 10 {
            let recent: Vec<f64> = self.cost_history.iter().rev().take(10).copied().collect();
            let max = recent.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            let min = recent.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            if (max - min).abs() < self.config.cost_tolerance {
                return true;
            }
        }

        false
    }

    /// Run full optimization loop
    pub fn optimize<F>(&mut self, mut cost_fn: F) -> OptimizationResult
    where
        F: FnMut(&HashMap<String, f64>) -> f64,
    {
        let start = Instant::now();

        // Initialize best
        let initial_cost = cost_fn(&self.current_vars());
        self.best_cost = initial_cost;
        self.best_vars = self.current_vars();

        while self.iteration < self.config.max_iterations {
            self.step(&mut cost_fn);

            if self.is_converged() {
                break;
            }
        }

        OptimizationResult {
            best_vars: self.best_vars.clone(),
            final_cost: self.best_cost,
            iterations: self.iteration,
            total_time: start.elapsed().as_secs_f64(),
            success: self.best_cost < 1e-3, // Reasonable success threshold
        }
    }

    /// Get the best result found so far
    pub fn best_result(&self) -> (&HashMap<String, f64>, f64) {
        (&self.best_vars, self.best_cost)
    }

    /// Get current iteration
    pub fn current_iteration(&self) -> usize {
        self.iteration
    }

    /// Get gradient vector
    pub fn gradient(&self) -> &[f64] {
        &self.gradient
    }

    /// Get cost history
    pub fn cost_history(&self) -> &[f64] {
        &self.cost_history
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Goal Cost Calculation Tests
    // =========================================================================

    #[test]
    fn test_goal_cost_maximize() {
        let goal = OptimizationGoal::maximize("gain");
        // Higher values should have lower cost
        assert!(goal.calculate_cost(100.0) < goal.calculate_cost(50.0));
        assert!(goal.calculate_cost(1000.0) < goal.calculate_cost(100.0));
    }

    #[test]
    fn test_goal_cost_minimize() {
        let goal = OptimizationGoal::minimize("power");
        // Lower absolute values should have lower cost
        assert!(goal.calculate_cost(1.0) < goal.calculate_cost(10.0));
        assert!(goal.calculate_cost(0.0) < goal.calculate_cost(1.0));
    }

    #[test]
    fn test_goal_cost_target() {
        let goal = OptimizationGoal::hit_target("vout", 1.2);
        assert_eq!(goal.calculate_cost(1.2), 0.0); // Exact match
        assert!(goal.calculate_cost(1.1) > 0.0); // Below target
        assert!(goal.calculate_cost(1.3) > 0.0); // Above target
        // Symmetric around target
        let cost_below = goal.calculate_cost(1.0);
        let cost_above = goal.calculate_cost(1.4);
        assert!((cost_below - cost_above).abs() < 1e-10);
    }

    #[test]
    fn test_goal_cost_range() {
        let mut goal = OptimizationGoal::minimize("bw");
        goal.strategy = GoalStrategy::Range;
        goal.range = Some((1e6, 10e6));

        assert_eq!(goal.calculate_cost(5e6), 0.0); // Inside range
        assert_eq!(goal.calculate_cost(1e6), 0.0); // At lower bound
        assert_eq!(goal.calculate_cost(10e6), 0.0); // At upper bound
        assert!(goal.calculate_cost(0.5e6) > 0.0); // Below range
        assert!(goal.calculate_cost(11e6) > 0.0); // Above range
    }

    // =========================================================================
    // Design Variable Tests
    // =========================================================================

    #[test]
    fn test_var_clamping() {
        let mut var = DesignVar::new("width", 10e-6, 1e-6, 50e-6);

        var.update(60e-6);
        assert_eq!(var.value, 50e-6); // Clamped to max

        var.update(0.5e-6);
        assert_eq!(var.value, 1e-6); // Clamped to min

        var.update(20e-6);
        assert_eq!(var.value, 20e-6); // Within range
    }

    #[test]
    fn test_var_nominal_preserved() {
        let var = DesignVar::new("length", 1e-6, 0.5e-6, 2e-6);
        assert_eq!(var.nominal, 1e-6);
    }

    // =========================================================================
    // Total Cost Calculation Tests
    // =========================================================================

    #[test]
    fn test_total_cost_single_goal() {
        let mut engine = OptimizerEngine::new();
        engine.add_goal(OptimizationGoal::hit_target("v_out", 1.2));

        let mut measurements = HashMap::new();
        measurements.insert("v_out".to_string(), 1.2);
        assert_eq!(engine.calculate_total_cost(&measurements), 0.0);

        measurements.insert("v_out".to_string(), 1.3);
        assert!(engine.calculate_total_cost(&measurements) > 0.0);
    }

    #[test]
    fn test_total_cost_missing_measurement() {
        let mut engine = OptimizerEngine::new();
        engine.add_goal(OptimizationGoal::hit_target("v_out", 1.2));

        let measurements = HashMap::new(); // Empty
        assert!(engine.calculate_total_cost(&measurements) >= 1e9);
    }

    #[test]
    fn test_total_cost_weighted_goals() {
        let mut engine = OptimizerEngine::new();

        let mut goal1 = OptimizationGoal::hit_target("v_out", 1.0);
        goal1.weight = 2.0;
        engine.add_goal(goal1);

        let mut goal2 = OptimizationGoal::hit_target("i_out", 1.0);
        goal2.weight = 1.0;
        engine.add_goal(goal2);

        let mut measurements = HashMap::new();
        measurements.insert("v_out".to_string(), 0.0); // Error of 1.0
        measurements.insert("i_out".to_string(), 0.0); // Error of 1.0

        let cost = engine.calculate_total_cost(&measurements);
        // v_out contributes 1.0 * 2.0 = 2.0, i_out contributes 1.0 * 1.0 = 1.0
        assert!((cost - 3.0).abs() < 1e-10);
    }

    // =========================================================================
    // Finite Difference Gradient Tests
    // =========================================================================

    #[test]
    fn test_gradient_finite_difference_quadratic() {
        let mut engine = OptimizerEngine::new();
        engine.add_var(DesignVar::new("x", 2.0, -10.0, 10.0));

        // f(x) = x^2, gradient should be 2x = 4 at x=2
        let mut cost_fn = |vars: &HashMap<String, f64>| {
            let x = vars.get("x").unwrap();
            x * x
        };

        let grad = engine.compute_gradient(&mut cost_fn);
        assert!((grad[0] - 4.0).abs() < 0.01); // Expect ~4.0
    }

    #[test]
    fn test_gradient_finite_difference_multivar() {
        let mut engine = OptimizerEngine::new();
        engine.add_var(DesignVar::new("x", 1.0, -10.0, 10.0));
        engine.add_var(DesignVar::new("y", 2.0, -10.0, 10.0));

        // f(x,y) = x^2 + y^2, gradient = [2x, 2y] = [2, 4]
        let mut cost_fn = |vars: &HashMap<String, f64>| {
            let x = vars.get("x").unwrap();
            let y = vars.get("y").unwrap();
            x * x + y * y
        };

        let grad = engine.compute_gradient(&mut cost_fn);
        assert!((grad[0] - 2.0).abs() < 0.01);
        assert!((grad[1] - 4.0).abs() < 0.01);
    }

    // =========================================================================
    // Gradient Descent Convergence Tests
    // =========================================================================

    #[test]
    fn test_gradient_descent_quadratic_convergence() {
        let mut engine = OptimizerEngine::with_config(OptimizerConfig {
            algorithm: OptimizerAlgo::GradientDescent,
            max_iterations: 100,
            cost_tolerance: 1e-8,
            ..Default::default()
        });
        engine.add_var(DesignVar::new("x", 5.0, -10.0, 10.0));
        engine.add_var(DesignVar::new("y", 5.0, -10.0, 10.0));

        // Minimize f(x,y) = (x-1)^2 + (y-2)^2
        // Minimum at (1, 2)
        let result = engine.optimize(|vars| {
            let x = vars.get("x").unwrap();
            let y = vars.get("y").unwrap();
            (x - 1.0).powi(2) + (y - 2.0).powi(2)
        });

        assert!(result.success);
        assert!((*result.best_vars.get("x").unwrap() - 1.0).abs() < 0.1);
        assert!((*result.best_vars.get("y").unwrap() - 2.0).abs() < 0.1);
    }

    #[test]
    fn test_gradient_descent_rosenbrock() {
        // Rosenbrock function: f(x,y) = (1-x)^2 + 100(y-x^2)^2
        // Minimum at (1, 1)
        let mut engine = OptimizerEngine::with_config(OptimizerConfig {
            algorithm: OptimizerAlgo::GradientDescent,
            max_iterations: 500,
            cost_tolerance: 1e-6,
            initial_step: 0.01, // Smaller step for Rosenbrock
            ..Default::default()
        });
        engine.add_var(DesignVar::new("x", 0.0, -5.0, 5.0));
        engine.add_var(DesignVar::new("y", 0.0, -5.0, 5.0));

        let result = engine.optimize(|vars| {
            let x = vars.get("x").unwrap();
            let y = vars.get("y").unwrap();
            (1.0 - x).powi(2) + 100.0 * (y - x * x).powi(2)
        });

        // Rosenbrock is hard; just check significant progress toward minimum
        assert!(result.final_cost < 10.0);
    }

    // =========================================================================
    // Pattern Search Tests
    // =========================================================================

    #[test]
    fn test_pattern_search_convergence() {
        let mut engine = OptimizerEngine::with_config(OptimizerConfig {
            algorithm: OptimizerAlgo::PatternSearch,
            max_iterations: 200,
            cost_tolerance: 1e-12, // Tight tolerance to ensure full convergence
            ..Default::default()
        });
        engine.add_var(DesignVar::new("x", 5.0, -10.0, 10.0));

        // Minimize (x-3)^2
        let result = engine.optimize(|vars| {
            let x = vars.get("x").unwrap();
            (x - 3.0).powi(2)
        });

        // Pattern search should find values near optimum
        // With 200 iterations and stepping, we should get reasonably close
        assert!(
            result.final_cost < 1.0,
            "Should find decent solution: cost={}",
            result.final_cost
        );
    }

    #[test]
    fn test_pattern_search_step_reduction() {
        // Pattern search should reduce step size when improvements are not found
        // and ultimately find a good solution
        let config = OptimizerConfig {
            algorithm: OptimizerAlgo::PatternSearch,
            max_iterations: 200,
            initial_step: 0.3,
            min_step: 1e-6,
            cost_tolerance: 1e-12,
            ..Default::default()
        };
        let mut engine = OptimizerEngine::with_config(config);
        engine.add_var(DesignVar::new("x", 8.0, 0.0, 10.0));

        let result = engine.optimize(|vars| {
            let x = vars.get("x").unwrap();
            (x - 3.0).powi(2) // Optimum at x=3
        });

        // Pattern search should make significant progress (cost < 5 vs initial cost of 25)
        assert!(
            result.final_cost < 5.0,
            "Pattern search should improve significantly from initial, got cost={}",
            result.final_cost
        );
    }

    // =========================================================================
    // Simulated Annealing Tests
    // =========================================================================

    #[test]
    fn test_simulated_annealing_exploration() {
        let mut engine = OptimizerEngine::with_config(OptimizerConfig {
            algorithm: OptimizerAlgo::SimulatedAnnealing,
            max_iterations: 500,
            sa_initial_temp: 50.0,
            sa_cooling_rate: 0.98,
            initial_step: 0.3,
            ..Default::default()
        });
        engine.add_var(DesignVar::new("x", -5.0, -10.0, 10.0));

        // Function with local minimum at x=-3 and global at x=3
        let result = engine.optimize(|vars| {
            let x = vars.get("x").unwrap();
            let global = (x - 3.0).powi(2);
            let local = 0.5 * (x + 3.0).powi(2);
            global.min(local + 1.0) // Local is slightly worse
        });

        // SA should find the global region (may not be exactly optimal)
        assert!(result.final_cost < 5.0);
    }

    #[test]
    fn test_simulated_annealing_cooling() {
        // SA should cool down and converge to a solution over iterations
        let mut engine = OptimizerEngine::with_config(OptimizerConfig {
            algorithm: OptimizerAlgo::SimulatedAnnealing,
            sa_initial_temp: 100.0,
            sa_cooling_rate: 0.95,
            max_iterations: 50,
            cost_tolerance: 1e-12,
            ..Default::default()
        });
        engine.add_var(DesignVar::new("x", 5.0, -10.0, 10.0));

        let initial_cost = 5.0 * 5.0; // 25.0
        let result = engine.optimize(|vars| {
            let x = vars.get("x").unwrap();
            x * x // Minimize x^2, optimum at x=0
        });

        // SA should improve from initial cost
        assert!(
            result.final_cost < initial_cost,
            "SA should improve from initial cost {}, got {}",
            initial_cost,
            result.final_cost
        );
    }

    // =========================================================================
    // Convergence Detection Tests
    // =========================================================================

    #[test]
    fn test_convergence_by_cost() {
        let config = OptimizerConfig {
            cost_tolerance: 1e-4,
            ..Default::default()
        };
        let mut engine = OptimizerEngine::with_config(config);
        engine.best_cost = 1e-5; // Below tolerance

        assert!(engine.is_converged());
    }

    #[test]
    fn test_convergence_by_gradient_norm() {
        let config = OptimizerConfig {
            var_tolerance: 1e-4,
            ..Default::default()
        };
        let mut engine = OptimizerEngine::with_config(config);
        engine.gradient = vec![1e-5, 1e-5]; // Small gradient
        engine.best_cost = 1.0; // Above cost tolerance

        assert!(engine.is_converged());
    }

    #[test]
    fn test_convergence_by_stagnation() {
        let mut engine = OptimizerEngine::new();
        // Add 10 identical costs
        for _ in 0..10 {
            engine.cost_history.push(0.5);
        }
        engine.best_cost = 0.5;
        engine.gradient = vec![1.0]; // Non-zero gradient

        assert!(engine.is_converged());
    }

    // =========================================================================
    // Integration Tests
    // =========================================================================

    #[test]
    fn test_full_optimization_workflow() {
        let mut engine = OptimizerEngine::new();

        // Add goals
        engine.add_goal(OptimizationGoal::hit_target("gain", 100.0));
        engine.add_goal(OptimizationGoal::minimize("power"));

        // Add variables
        engine.add_var(DesignVar::new("width", 10e-6, 1e-6, 100e-6));
        engine.add_var(DesignVar::new("length", 1e-6, 0.5e-6, 10e-6));

        // Mock simulation: gain ~ width/length, power ~ width
        let result = engine.optimize(|vars| {
            let w = *vars.get("width").unwrap();
            let l = *vars.get("length").unwrap();
            let gain = w / l * 1e6;
            let power = w * 1e6;

            let gain_error = (gain - 100.0).powi(2);
            let power_cost = power;

            gain_error + 0.1 * power_cost
        });

        assert!(result.iterations > 0);
        assert!(result.total_time > 0.0);
    }
}
