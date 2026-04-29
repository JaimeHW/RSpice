use std::collections::HashMap;

use super::*;

impl OptimizerEngine {
    /// Gradient descent with line search
    pub(super) fn step_gradient_descent<F>(
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
    pub(super) fn step_pattern_search<F>(
        &mut self,
        cost_fn: &mut F,
        current_cost: f64,
    ) -> HashMap<String, f64>
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
    pub(super) fn step_simulated_annealing<F>(
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
}
