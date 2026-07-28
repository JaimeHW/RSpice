//! Optimizer run lifecycle.
//!
//! Starting, pausing, resuming, and cancelling a search, and the state that
//! has to be consistent at each boundary for a resumed run to continue
//! rather than restart.

use std::collections::HashMap;

use super::*;

impl OptimizerEngine {
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

    /// Get the best result found so far
    pub fn best_result(&self) -> (&HashMap<String, f64>, f64) {
        (&self.best_vars, self.best_cost)
    }

    /// Get current iteration
    pub fn current_iteration(&self) -> usize {
        self.iteration
    }
}
