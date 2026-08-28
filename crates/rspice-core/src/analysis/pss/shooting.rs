//! Shooting Newton Solver for PSS Analysis
//!
//! Implements the shooting method for finding periodic steady-state solutions.
//! The core algorithm solves the boundary value problem: find x(0) such that x(T) = x(0).

#![allow(clippy::needless_range_loop)]
use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::numerics::eigenspectrum::{OrdinarySpectrumError, qualified_real_eigenspectrum};
use crate::solver::{SolverError, StaticMatrix};

/// Failure to extract and strictly qualify a complete Floquet spectrum.
#[derive(Debug, Clone, thiserror::Error, PartialEq)]
#[non_exhaustive]
pub enum FloquetSpectrumError {
    /// Cooperative cancellation was requested.
    #[error("Floquet spectrum computation was aborted")]
    Aborted,
    /// The monodromy matrix or its eigenspectrum failed numerical validation.
    #[error("Floquet spectrum computation failed: {0}")]
    Numerical(String),
}

impl From<OrdinarySpectrumError> for FloquetSpectrumError {
    fn from(error: OrdinarySpectrumError) -> Self {
        match error {
            OrdinarySpectrumError::Aborted => Self::Aborted,
            error => Self::Numerical(error.to_string()),
        }
    }
}

/// State of the shooting Newton solver
#[derive(Debug, Clone)]
pub struct ShootingState {
    /// Current estimate of initial state vector
    pub x0: Vec<Value>,

    /// State at end of period (after transient integration)
    pub x_t: Vec<Value>,

    /// Residual vector: F = x(T) - x(0)
    pub residual: Vec<Value>,

    /// Current period estimate (for autonomous circuits)
    pub period: Value,

    /// Number of state variables
    n_states: usize,
}

impl ShootingState {
    /// Create new shooting state with initial guess
    pub fn new(initial_guess: Vec<Value>, period: Value) -> Self {
        let n = initial_guess.len();
        Self {
            x0: initial_guess.clone(),
            x_t: vec![0.0; n],
            residual: vec![0.0; n],
            period,
            n_states: n,
        }
    }

    /// Get dimension of state vector
    pub fn dimension(&self) -> usize {
        self.n_states
    }

    /// Compute residual after transient integration
    pub fn compute_residual(&mut self) {
        for i in 0..self.n_states {
            self.residual[i] = self.x_t[i] - self.x0[i];
        }
    }

    /// Get residual norm (L2)
    pub fn residual_norm(&self) -> Value {
        self.residual.iter().map(|r| r * r).sum::<Value>().sqrt()
    }

    /// Get relative residual norm
    pub fn relative_residual_norm(&self) -> Value {
        let x0_norm = self.x0.iter().map(|x| x * x).sum::<Value>().sqrt();
        if x0_norm > 1e-12 {
            self.residual_norm() / x0_norm
        } else {
            self.residual_norm()
        }
    }

    /// Update initial state estimate
    ///
    /// Applies Newton step: x0_new = x0 + damping * delta
    /// Note: delta is pre-computed as -J^(-1)*F by compute_newton_step
    pub fn update_x0(&mut self, delta: &[Value], damping: Value) {
        for i in 0..self.n_states {
            self.x0[i] += damping * delta[i];
        }
    }
}

/// Shooting Newton solver for PSS
///
/// Uses Newton's method to solve the nonlinear equation F(x0) = x(T) - x0 = 0
/// where x(T) is obtained by integrating the circuit equations from x0 over
/// one period T.
#[derive(Debug)]
pub struct ShootingNewtonSolver {
    /// Convergence tolerance (relative)
    tolerance: Value,

    /// Absolute tolerance for small signals
    abstol: Value,

    /// Maximum iterations
    max_iterations: usize,

    /// Damping factor for Newton updates
    pub damping: Value,

    /// Finite difference step for Jacobian computation
    fd_step: Value,

    /// Current iteration count
    iteration: usize,

    /// Has converged flag
    converged: bool,
}

impl ShootingNewtonSolver {
    /// Create a new shooting Newton solver
    pub fn new(tolerance: Value, max_iterations: usize) -> Self {
        Self {
            tolerance,
            abstol: 1e-12,
            max_iterations,
            damping: 1.0,
            fd_step: 1e-8,
            iteration: 0,
            converged: false,
        }
    }

    /// Set damping factor
    pub fn with_damping(mut self, damping: Value) -> Self {
        self.damping = damping.clamp(0.1, 1.0);
        self
    }

    /// Set absolute tolerance
    pub fn with_abstol(mut self, abstol: Value) -> Self {
        self.abstol = abstol;
        self
    }

    /// Set finite difference step size
    pub fn with_fd_step(mut self, step: Value) -> Self {
        self.fd_step = step;
        self
    }

    /// Check if solver has converged
    pub fn has_converged(&self) -> bool {
        self.converged
    }

    /// Get current iteration count
    pub fn iterations(&self) -> usize {
        self.iteration
    }

    /// Check convergence based on residual
    pub fn check_convergence(&mut self, state: &ShootingState) -> bool {
        let rel_norm = state.relative_residual_norm();
        let abs_norm = state.residual_norm();

        self.converged = rel_norm < self.tolerance || abs_norm < self.abstol;
        self.iteration += 1;

        self.converged
    }

    /// Check if maximum iterations exceeded
    pub fn is_maxed_out(&self) -> bool {
        self.iteration >= self.max_iterations
    }

    /// Compute Newton update using finite-difference Jacobian
    ///
    /// # Arguments
    /// * `state` - Current shooting state
    /// * `integrate_period` - Closure that integrates from x0 to x(T)
    ///
    /// # Returns
    /// Newton step delta_x0
    pub fn compute_newton_step<F>(
        &self,
        state: &ShootingState,
        integrate_period: F,
    ) -> Result<Vec<Value>, SolverError>
    where
        F: Fn(&[Value]) -> Vec<Value>,
    {
        let n = state.dimension();

        // Build Jacobian J = dF/dx0 = d(x(T) - x0)/dx0 = dX(T)/dx0 - I
        // where X(T) is the map from x0 to x(T)

        let mut jacobian = vec![vec![0.0; n]; n];

        // Compute Jacobian columns via finite differences
        for j in 0..n {
            let mut x0_plus = state.x0.clone();
            let mut x0_minus = state.x0.clone();

            // Perturbation size scaled by variable magnitude
            let h = self.fd_step * (state.x0[j].abs().max(1.0));

            x0_plus[j] += h;
            x0_minus[j] -= h;

            // Integrate perturbed initial conditions
            let x_t_plus = integrate_period(&x0_plus);
            let x_t_minus = integrate_period(&x0_minus);

            // Central difference for dX(T)/dx0
            for i in 0..n {
                let dx_t_dx0 = (x_t_plus[i] - x_t_minus[i]) / (2.0 * h);
                // Jacobian of F = X(T) - x0 is dX/dx0 - I
                jacobian[i][j] = dx_t_dx0 - if i == j { 1.0 } else { 0.0 };
            }
        }

        // Solve J * delta = -F for Newton step
        self.solve_linear_system(&jacobian, &state.residual)
    }

    /// Solve linear system using direct method (for small to medium systems)
    fn solve_linear_system(
        &self,
        jacobian: &[Vec<Value>],
        rhs: &[Value],
    ) -> Result<Vec<Value>, SolverError> {
        let n = rhs.len();

        if n == 0 {
            return Ok(Vec::new());
        }

        // Convert to triplets for sparse solver
        let mut triplets = Vec::with_capacity(n * n);
        for (i, row) in jacobian.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                if val.abs() > 1e-15 {
                    triplets.push((i, j, val));
                }
            }
        }

        // Add small diagonal regularization for stability
        for i in 0..n {
            triplets.push((i, i, 1e-12));
        }

        let mut matrix = StaticMatrix::from_triplets(n, n, &triplets)?;

        // Negate RHS because we solve J*delta = -F
        let neg_rhs: Vec<Value> = rhs.iter().map(|r| -r).collect();

        matrix.solve(&neg_rhs)
    }

    /// Extract Floquet multipliers from Monodromy matrix
    ///
    /// Floquet multipliers are eigenvalues of the Monodromy matrix.
    /// For a stable orbit, all multipliers should have |λ| ≤ 1.
    /// An autonomous orbit has a unit phase multiplier corresponding to
    /// perturbations along the orbit; a driven periodic solution need not.
    pub fn compute_floquet_multipliers(
        &self,
        monodromy: &[Vec<Value>],
    ) -> Result<Vec<num_complex::Complex64>, FloquetSpectrumError> {
        self.compute_floquet_multipliers_with_abort(monodromy, &NoAbort)
    }

    /// Extract and strictly residual-qualify the complete Floquet spectrum,
    /// checking for cooperative cancellation around the atomic eigensolve.
    pub fn compute_floquet_multipliers_with_abort(
        &self,
        monodromy: &[Vec<Value>],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<num_complex::Complex64>, FloquetSpectrumError> {
        let spectrum = qualified_real_eigenspectrum(monodromy, abort)?;
        Ok(spectrum.eigenvalues)
    }

    /// Reset solver state for new analysis
    pub fn reset(&mut self) {
        self.iteration = 0;
        self.converged = false;
    }
}

impl Default for ShootingNewtonSolver {
    fn default() -> Self {
        Self::new(1e-6, 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::ImmediateAbort;

    #[test]
    fn floquet_spectrum_rejects_empty_ragged_and_nonfinite_monodromy() {
        let solver = ShootingNewtonSolver::default();
        assert!(solver.compute_floquet_multipliers(&[]).is_err());
        assert!(
            solver
                .compute_floquet_multipliers(&[vec![1.0, 0.0], vec![0.0]])
                .is_err()
        );
        assert!(
            solver
                .compute_floquet_multipliers(&[vec![Value::NAN]])
                .is_err()
        );
    }

    #[test]
    fn floquet_spectrum_honors_abort() {
        let solver = ShootingNewtonSolver::default();
        let error = solver
            .compute_floquet_multipliers_with_abort(&[vec![0.5]], &ImmediateAbort)
            .unwrap_err();
        assert_eq!(error, FloquetSpectrumError::Aborted);
    }
}
