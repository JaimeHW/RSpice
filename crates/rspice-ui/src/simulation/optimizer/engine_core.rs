//! The optimizer loop.
//!
//! Proposes a point, runs the analyses, scores the result against the goals,
//! and decides whether to continue. Every iteration is recorded so the run
//! can be inspected after the fact.

use std::collections::HashMap;

use super::*;

impl OptimizerEngine {
    pub fn new() -> Self {
        let config = OptimizerConfig::default();
        Self {
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

    /// Register an evaluated candidate with the best-so-far state.
    ///
    /// Callers evaluate the initial design before taking the first step; that
    /// point must participate in the final optimum just like every generated
    /// candidate.
    pub fn observe_candidate(&mut self, vars: &HashMap<String, f64>, cost: f64) {
        if cost.is_finite() && cost < self.best_cost {
            self.best_cost = cost;
            self.best_vars = vars.clone();
        }
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
            let plus_value = (var.value + h).clamp(var.min, var.max);
            vars_plus.insert(var.name.clone(), plus_value);
            let cost_plus = cost_fn(&vars_plus);

            // Backward perturbation
            let mut vars_minus = self.current_vars();
            let minus_value = (var.value - h).clamp(var.min, var.max);
            vars_minus.insert(var.name.clone(), minus_value);
            let cost_minus = cost_fn(&vars_minus);

            // Use the realized bounded displacement. At a variable limit the
            // nominal 2h denominator would understate a one-sided derivative.
            let displacement = plus_value - minus_value;
            *grad_entry = if displacement > 0.0 {
                (cost_plus - cost_minus) / displacement
            } else {
                0.0
            };
        }

        self.gradient = grad.clone();
        grad
    }

    /// Simple pseudo-random number generator (Xorshift64)
    pub(super) fn next_random(&mut self) -> f64 {
        self.rng_state ^= self.rng_state << 13;
        self.rng_state ^= self.rng_state >> 7;
        self.rng_state ^= self.rng_state << 17;
        (self.rng_state as f64) / (u64::MAX as f64)
    }

    /// Line search with Armijo-Goldstein condition
    ///
    /// Finds step size α that satisfies: f(x + α*d) ≤ f(x) + c*α*∇f·d
    pub(super) fn line_search<F>(
        &mut self,
        direction: &[f64],
        current_cost: f64,
        cost_fn: &mut F,
    ) -> f64
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

        0.0
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
        };

        // Evaluate cost AFTER the step and update best if improved
        let new_cost = cost_fn(&new_vars);
        self.observe_candidate(&new_vars, new_cost);

        new_vars
    }
}
