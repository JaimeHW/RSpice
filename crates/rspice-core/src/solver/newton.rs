//! Newton-Raphson iteration for nonlinear circuit solving

use super::SolverError;
use crate::Value;

/// Newton-Raphson solver configuration
#[derive(Debug, Clone)]
pub struct NewtonConfig {
    /// Absolute tolerance for convergence
    pub abs_tol: Value,
    /// Relative tolerance for convergence
    pub rel_tol: Value,
    /// Maximum number of iterations
    pub max_iter: usize,
    /// Enable damping for improved convergence
    pub damping: bool,
    /// Initial damping factor (0 < alpha <= 1)
    pub damping_factor: Value,
}

impl Default for NewtonConfig {
    fn default() -> Self {
        Self {
            abs_tol: 1e-12,
            rel_tol: 1e-6,
            max_iter: 50,
            damping: true,
            damping_factor: 1.0,
        }
    }
}

/// Newton-Raphson update-convergence tracker.
///
/// The caller owns the linear solve, damping and equation-residual checks.
/// This tracker retains one previous iterate and allocates only when its size changes.
pub struct NewtonSolver {
    config: NewtonConfig,
    /// Previous solution vector
    x_prev: Vec<Value>,
    /// Current iteration count
    iteration: usize,
    /// Has converged flag
    converged: bool,
}

impl NewtonSolver {
    pub fn new(config: NewtonConfig) -> Self {
        Self {
            config,
            x_prev: Vec::new(),
            iteration: 0,
            converged: false,
        }
    }

    /// Initialize for a new solve
    ///
    /// Retains storage of the correct size to avoid allocation during iteration.
    pub fn init(&mut self, initial_guess: Vec<Value>) {
        let n = initial_guess.len();

        // Reuse the retained vector when the dimension is unchanged.
        if self.x_prev.len() != n {
            self.x_prev = initial_guess;
        } else {
            // Reuse existing allocation - just copy data
            self.x_prev.copy_from_slice(&initial_guess);
        }

        self.iteration = 0;
        self.converged = false;
    }

    /// Check whether every coordinate satisfies its absolute-plus-relative update tolerance.
    ///
    /// Empty or nonfinite states, invalid tolerances and exhausted iteration budgets
    /// cannot converge. A size change starts a new sequence with the supplied baseline.
    /// Equal-sized checks retain the new iterate without allocating; SIMD builds use
    /// vectorized comparisons with the same criterion as scalar builds.
    pub fn check_convergence(&mut self, x_new: &[Value]) -> bool {
        self.converged = false;
        if x_new.is_empty() || self.is_maxed_out() {
            return false;
        }
        if self.x_prev.len() != x_new.len() {
            self.init(x_new.to_vec());
            return false;
        }

        self.converged = crate::numerics::solution_update_converged(
            &self.x_prev,
            x_new,
            self.config.abs_tol,
            self.config.rel_tol,
        );
        self.x_prev.copy_from_slice(x_new);
        self.iteration = self.iteration.saturating_add(1);
        self.converged
    }

    /// Get current iteration count
    pub fn iterations(&self) -> usize {
        self.iteration
    }

    /// Check if max iterations reached
    pub fn is_maxed_out(&self) -> bool {
        self.iteration >= self.config.max_iter
    }

    /// Return error if not converged
    pub fn result(&self) -> Result<(), SolverError> {
        if self.converged {
            Ok(())
        } else {
            Err(SolverError::ConvergenceFailed(self.iteration))
        }
    }
}

/// Limit voltage changes to prevent numerical issues
pub fn limit_voltage_step(v_old: Value, v_new: Value, v_crit: Value) -> Value {
    let v_diff = v_new - v_old;

    if v_diff.abs() > v_crit {
        // Limit the step
        v_old + v_diff.signum() * v_crit
    } else {
        v_new
    }
}

/// Limit junction voltage for PN devices (diodes, BJTs)
pub fn limit_pn_voltage(v_old: Value, v_new: Value, vt: Value) -> Value {
    // Critical voltage is typically 10 * thermal voltage
    let v_crit = 10.0 * vt;

    if v_new > v_crit && (v_new - v_old).abs() > 2.0 * vt {
        // Use logarithmic limiting
        if v_old > 0.0 {
            let arg = 1.0 + (v_new - v_old) / vt;
            if arg > 0.0 {
                return v_old + vt * arg.ln();
            }
        }
        vt * (1.0 + (v_new / vt).ln())
    } else {
        limit_voltage_step(v_old, v_new, v_crit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonfinite_coordinates_never_converge_at_any_vector_position() {
        for length in [1, 15, 16, 17, 32, 33] {
            for lane in 0..length {
                for invalid in [Value::NAN, Value::INFINITY, Value::NEG_INFINITY] {
                    let mut solver = NewtonSolver::new(NewtonConfig::default());
                    let finite = vec![1.0; length];
                    let mut invalid_state = finite.clone();
                    invalid_state[lane] = invalid;
                    solver.init(finite.clone());
                    assert!(!solver.check_convergence(&invalid_state));
                    assert!(solver.result().is_err());
                    assert!(!solver.check_convergence(&finite));
                    assert!(solver.check_convergence(&finite));
                    solver.init(invalid_state);
                    assert!(!solver.check_convergence(&finite));
                }
            }
        }
    }

    #[test]
    fn dimension_change_and_exhausted_budget_clear_prior_success() {
        let mut solver = NewtonSolver::new(NewtonConfig::default());
        solver.init(vec![1.0]);
        assert!(solver.check_convergence(&[1.0]));
        assert!(!solver.check_convergence(&[1.0, 2.0]));
        assert!(solver.result().is_err());
        assert_eq!(solver.iterations(), 0);
        assert!(solver.check_convergence(&[1.0, 2.0]));
        assert!(!solver.check_convergence(&[]));
        assert!(solver.result().is_err());

        let mut solver = NewtonSolver::new(NewtonConfig {
            max_iter: 1,
            ..NewtonConfig::default()
        });
        solver.init(vec![1.0]);
        assert!(solver.check_convergence(&[1.0]));
        assert!(solver.is_maxed_out());
        assert!(!solver.check_convergence(&[1.0]));
        assert!(solver.result().is_err());
        assert_eq!(solver.iterations(), 1);
    }

    #[test]
    fn coordinate_tolerances_cover_zero_crossings_and_mixed_scales() {
        let mut solver = NewtonSolver::new(NewtonConfig::default());
        solver.init(vec![1.0]);
        assert!(!solver.check_convergence(&[0.0]));
        assert!(solver.check_convergence(&[0.0]));
        solver.init(vec![0.0, 1e6]);
        assert!(solver.check_convergence(&[5e-13, 1e6 + 0.5]));
        solver.init(vec![0.0, 1e6]);
        assert!(!solver.check_convergence(&[2e-12, 1e6 + 0.5]));
    }

    #[test]
    fn invalid_tolerances_and_empty_states_cannot_report_success() {
        for (abs_tol, rel_tol) in [
            (Value::NAN, 1e-6),
            (1e-12, Value::NAN),
            (Value::INFINITY, 1e-6),
            (1e-12, Value::INFINITY),
            (-1.0, 1e-6),
            (1e-12, -1.0),
        ] {
            let mut solver = NewtonSolver::new(NewtonConfig {
                abs_tol,
                rel_tol,
                ..NewtonConfig::default()
            });
            solver.init(vec![1.0]);
            assert!(!solver.check_convergence(&[1.0]));
            assert!(solver.result().is_err());
        }
        let mut solver = NewtonSolver::new(NewtonConfig::default());
        assert!(!solver.check_convergence(&[]));
        assert!(solver.result().is_err());
    }
}
