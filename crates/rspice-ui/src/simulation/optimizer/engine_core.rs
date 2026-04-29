use std::collections::HashMap;

use super::*;

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
}
