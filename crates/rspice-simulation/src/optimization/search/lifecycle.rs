//! Optimizer run lifecycle.
//!
//! Starting, pausing, resuming, and cancelling a search, and the state that
//! has to be consistent at each boundary for a resumed run to continue
//! rather than restart.

use std::collections::HashMap;

use super::*;

impl OptimizerEngine {
    /// Check if converged
    pub fn is_converged(&self, target_cost: Option<f64>) -> bool {
        self.best_violation == 0.0 && self.search_finished(target_cost)
    }

    /// A stalled infeasible search finishes without claiming convergence.
    pub fn search_finished(&self, target_cost: Option<f64>) -> bool {
        // Absolute cost tolerance is meaningful only when the scoring
        // contract declares a target cost (for example, zero squared error).
        // A signed minimize/maximize objective must not converge merely
        // because its cost is negative or happens to cross zero.
        if let Some(target) = target_cost
            && self.best_violation == 0.0
            && (self.best_cost - target).abs() < self.config.cost_tolerance
        {
            return true;
        }

        // Step-size convergence belongs to pattern search. Gradient descent
        // has its own norm criterion, while annealing cools independently.
        if self.config.algorithm == OptimizerAlgo::PatternSearch
            && self.step_size < self.config.min_step
        {
            return true;
        }

        // A zero-initialized gradient is not evidence of convergence for the
        // derivative-free algorithms.
        let grad_norm: f64 = self.gradient.iter().map(|g| g * g).sum::<f64>().sqrt();
        if self.config.algorithm == OptimizerAlgo::GradientDescent
            && self.iteration > 0
            && grad_norm < self.config.var_tolerance
            && !self.gradient.is_empty()
        {
            return true;
        }

        // Cost stagnation over last 10 iterations
        if self.cost_history.len() >= 10 {
            let recent_scores: Vec<_> = self.cost_history.iter().rev().take(10).copied().collect();
            // Crossing into feasibility is progress even if the raw objective
            // is constant; never compare costs across different search phases.
            let feasibility = self.best_violation > 0.0;
            if recent_scores
                .iter()
                .any(|score| (score.violation > 0.0) != feasibility)
            {
                return false;
            }
            let recent: Vec<f64> = recent_scores
                .iter()
                .map(|score| score.phase_value(feasibility))
                .collect();
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
