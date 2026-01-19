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

/// Newton-Raphson iteration state
///
/// Uses double-buffering to avoid allocation during convergence checks.
pub struct NewtonSolver {
    config: NewtonConfig,
    /// Previous solution vector (buffer A)
    x_prev: Vec<Value>,
    /// Temporary buffer for swap operations (buffer B)
    x_temp: Vec<Value>,
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
            x_temp: Vec::new(),
            iteration: 0,
            converged: false,
        }
    }

    /// Initialize for a new solve
    ///
    /// Pre-allocates buffers to the correct size to avoid allocation during iteration.
    pub fn init(&mut self, initial_guess: Vec<Value>) {
        let n = initial_guess.len();

        // Resize buffers if needed (only allocates if growing)
        if self.x_prev.len() != n {
            self.x_prev = initial_guess;
            self.x_temp = vec![0.0; n];
        } else {
            // Reuse existing allocation - just copy data
            self.x_prev.copy_from_slice(&initial_guess);
            // x_temp already has correct size
        }

        self.iteration = 0;
        self.converged = false;
    }

    /// Check if solution has converged
    ///
    /// Uses double-buffering with O(1) swap instead of allocation.
    /// When compiled with the `simd` feature, uses SIMD-accelerated
    /// max-difference calculations for 2-3x faster convergence checks.
    pub fn check_convergence(&mut self, x_new: &[Value]) -> bool {
        // Handle size mismatch (should only happen on first call if init wasn't called properly)
        if self.x_prev.len() != x_new.len() {
            self.x_prev = x_new.to_vec();
            self.x_temp = vec![0.0; x_new.len()];
            return false;
        }

        // Compute max differences using SIMD when available
        #[cfg(feature = "simd")]
        let (max_diff, max_rel_diff) = {
            let max_diff = crate::simd::max_abs_diff(&self.x_prev, x_new);
            let max_rel_diff = crate::simd::max_rel_diff(&self.x_prev, x_new, self.config.abs_tol);
            (max_diff, max_rel_diff)
        };

        #[cfg(not(feature = "simd"))]
        let (max_diff, max_rel_diff) = {
            let mut max_diff = 0.0_f64;
            let mut max_rel_diff = 0.0_f64;

            for (&x_old, &x_curr) in self.x_prev.iter().zip(x_new.iter()) {
                let diff = (x_curr - x_old).abs();
                max_diff = max_diff.max(diff);

                if x_curr.abs() > self.config.abs_tol {
                    let rel_diff = diff / x_curr.abs();
                    max_rel_diff = max_rel_diff.max(rel_diff);
                }
            }
            (max_diff, max_rel_diff)
        };

        // Copy new values and swap buffers
        #[cfg(feature = "simd")]
        crate::simd::copy(&mut self.x_temp, x_new);

        #[cfg(not(feature = "simd"))]
        self.x_temp.copy_from_slice(x_new);

        // O(1) swap of buffer pointers instead of allocation
        std::mem::swap(&mut self.x_prev, &mut self.x_temp);

        self.iteration += 1;

        self.converged = max_diff < self.config.abs_tol || max_rel_diff < self.config.rel_tol;
        self.converged
    }

    /// Apply damping to the Newton update
    ///
    /// Computes `x_new = x_old + alpha * delta` where alpha is adaptively
    /// reduced for large updates to improve convergence stability.
    ///
    /// When compiled with the `simd` feature, uses SIMD acceleration.
    pub fn apply_damping(&self, x_old: &[Value], delta: &[Value]) -> Vec<Value> {
        if !self.config.damping {
            let mut result = x_old.to_vec();
            #[cfg(feature = "simd")]
            crate::simd::add(&mut result, delta);
            #[cfg(not(feature = "simd"))]
            for (r, d) in result.iter_mut().zip(delta.iter()) {
                *r += *d;
            }
            return result;
        }

        let mut alpha = self.config.damping_factor;

        // Reduce damping factor if update is very large
        #[cfg(feature = "simd")]
        let max_delta = crate::simd::max_abs(delta);
        #[cfg(not(feature = "simd"))]
        let max_delta = delta.iter().map(|d| d.abs()).fold(0.0, f64::max);

        if max_delta > 1.0 {
            alpha = (1.0 / max_delta).max(0.1);
        }

        let mut result = x_old.to_vec();
        #[cfg(feature = "simd")]
        crate::simd::axpy(&mut result, delta, alpha);
        #[cfg(not(feature = "simd"))]
        for (r, d) in result.iter_mut().zip(delta.iter()) {
            *r += alpha * *d;
        }
        result
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
    fn test_convergence_check() {
        let config = NewtonConfig::default();
        let mut solver = NewtonSolver::new(config);

        solver.init(vec![1.0, 2.0, 3.0]);

        // First iteration - not converged
        assert!(!solver.check_convergence(&[1.1, 2.1, 3.1]));

        // Close enough - should converge
        assert!(solver.check_convergence(&[1.1, 2.1, 3.1]));
    }

    #[test]
    fn test_pn_voltage_limit() {
        let vt = 0.026; // Thermal voltage at room temp

        // Normal update - no limiting
        let v = limit_pn_voltage(0.6, 0.65, vt);
        assert!((v - 0.65).abs() < 0.01);

        // Large jump - should be limited
        let v = limit_pn_voltage(0.0, 10.0, vt);
        assert!(v < 10.0);
    }
}
