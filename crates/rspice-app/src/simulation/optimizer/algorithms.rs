//! Optimization algorithms.
//!
//! The search strategies the engine can drive: gradient, pattern, and
//! population methods, each stepping the design variables toward the goal.

use std::collections::HashMap;

use super::*;

impl OptimizerEngine {
    /// Gradient descent with line search
    pub(super) fn step_gradient_descent<F, S>(
        &mut self,
        cost_fn: &mut F,
        current_cost: OptimizationScore,
    ) -> HashMap<String, f64>
    where
        F: FnMut(&HashMap<String, f64>) -> S,
        S: Into<OptimizationScore>,
    {
        // Compute gradient
        self.compute_gradient_for_phase(cost_fn, current_cost.violation > 0.0);
        // Failed neighboring circuit evaluations have infinite cost. A
        // derivative across that boundary is undefined; use bounded
        // exploratory moves without propagating NaN into design variables.
        if self.gradient.iter().any(|value| !value.is_finite()) {
            self.gradient.clear();
            return self.step_pattern_search(cost_fn, current_cost);
        }

        // Compute descent direction (negative gradient)
        let direction: Vec<f64> = self.gradient.iter().map(|g| -g).collect();

        // Line search for step size
        let alpha = self.line_search(&direction, current_cost, cost_fn);
        if alpha == 0.0 {
            // A bounded line search may stall against a hard limit or an
            // unusually scaled cost. Try bounded coordinate moves before
            // treating repeated identical iterates as convergence.
            self.gradient.clear();
            return self.step_pattern_search(cost_fn, current_cost);
        }

        // Update variables
        for (i, var) in self.variables.iter_mut().enumerate() {
            let new_val = var.value + alpha * direction[i];
            var.update(new_val);
        }

        self.iteration += 1;
        self.current_vars()
    }

    /// Hooke-Jeeves Pattern Search (derivative-free)
    pub(super) fn step_pattern_search<F, S>(
        &mut self,
        cost_fn: &mut F,
        current_cost: OptimizationScore,
    ) -> HashMap<String, f64>
    where
        F: FnMut(&HashMap<String, f64>) -> S,
        S: Into<OptimizationScore>,
    {
        let mut best_cost = current_cost;
        let mut best_vars = self.current_vars();
        let mut any_improved = false;

        // Exploratory moves: try each coordinate direction
        for i in 0..self.variables.len() {
            let var = &self.variables[i];
            let delta = var.displacement(self.step_size);

            // Get current best value for this variable (may have been updated by previous coord)
            let current_val = *best_vars.get(&var.name).unwrap_or(&var.value);

            // Try positive direction
            let mut trial_plus = best_vars.clone();
            trial_plus.insert(var.name.clone(), var.proposal(current_val + delta));
            let cost_plus = cost_fn(&trial_plus).into();

            if cost_plus.better_than(best_cost) {
                best_cost = cost_plus;
                best_vars = trial_plus;
                any_improved = true;
                continue;
            }

            // Try negative direction
            let mut trial_minus = best_vars.clone();
            trial_minus.insert(var.name.clone(), var.proposal(current_val - delta));
            let cost_minus = cost_fn(&trial_minus).into();

            if cost_minus.better_than(best_cost) {
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
    pub(super) fn step_simulated_annealing<F, S>(
        &mut self,
        cost_fn: &mut F,
        current_cost: OptimizationScore,
    ) -> HashMap<String, f64>
    where
        F: FnMut(&HashMap<String, f64>) -> S,
        S: Into<OptimizationScore>,
    {
        // Generate random perturbations FIRST (avoid borrow conflict)
        let perturbations: Vec<f64> = (0..self.variables.len())
            .map(|_| self.next_random())
            .collect();

        // Generate random neighbor
        let mut neighbor_vars = self.current_vars();
        for (i, var) in self.variables.iter().enumerate() {
            let perturbation = if var.quantum.is_some() {
                (perturbations[i] - 0.5) * 2.0 * var.displacement(self.step_size)
            } else {
                // Preserve the continuous search's arithmetic and seeded sequence.
                (perturbations[i] - 0.5) * 2.0 * (var.max - var.min) * self.step_size
            };
            let new_val = var.proposal(var.value + perturbation);
            neighbor_vars.insert(var.name.clone(), new_val);
        }

        let neighbor_cost = cost_fn(&neighbor_vars).into();
        let delta = if neighbor_cost.violation == current_cost.violation {
            neighbor_cost.cost - current_cost.cost
        } else {
            neighbor_cost.violation - current_cost.violation
        };
        // Annealing may explore worse objectives/violations but cannot leave
        // the feasible region once it has found a feasible design.
        let accept = if neighbor_cost.better_than(current_cost) {
            true
        } else if neighbor_cost.is_valid()
            && (current_cost.violation > 0.0 || neighbor_cost.violation == 0.0)
        {
            let prob = (-delta / self.temperature).exp();
            self.next_random() < prob
        } else {
            false
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
}
