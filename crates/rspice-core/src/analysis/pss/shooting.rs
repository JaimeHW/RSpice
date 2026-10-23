//! Shooting Newton Solver for PSS Analysis
//!
//! Implements the shooting method for finding periodic steady-state solutions.
//! The core algorithm solves the boundary value problem: find x(0) such that x(T) = x(0).

use crate::Value;
use crate::abort_signal::AbortSignal;
use crate::analysis::{FloquetSpectrumCertificate, FloquetSpectrumEvidence};
use crate::numerics::eigenspectrum::{OrdinarySpectrumError, qualified_real_eigenspectrum};

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
        self.residual.iter().fold(0.0, |norm, &r| norm.hypot(r))
    }

    /// Update initial state estimate
    ///
    /// Applies Newton step: x0_new = x0 + damping * delta, where delta solves
    /// J * delta = -F
    pub fn update_x0(&mut self, delta: &[Value], damping: Value) {
        for (i, &entry) in delta.iter().enumerate().take(self.n_states) {
            self.x0[i] += damping * entry;
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

    /// Get current iteration count
    pub fn iterations(&self) -> usize {
        self.iteration
    }

    /// Require every shooting coordinate to meet its relative or absolute
    /// tolerance. A voltage magnitude must not scale a winding-current error.
    pub fn check_convergence(&mut self, state: &ShootingState) -> bool {
        self.converged = self.tolerance.is_finite()
            && self.tolerance > 0.0
            && self.abstol.is_finite()
            && self.abstol > 0.0
            && state.x0.len() == state.n_states
            && state.x_t.len() == state.n_states
            && state.residual.len() == state.n_states
            && state
                .x0
                .iter()
                .chain(&state.x_t)
                .chain(&state.residual)
                .all(|x| x.is_finite())
            && state
                .x0
                .iter()
                .zip(&state.residual)
                .all(|(&initial, &residual)| {
                    // Normalize before division, as in the diagnostic L2 ratio,
                    // preserving the single-coordinate contract at extreme scales.
                    let scale = initial.abs().max(residual.abs());
                    residual.abs() < self.abstol
                        || (residual / scale).abs() / (initial / scale).abs() < self.tolerance
                });
        self.iteration += 1;

        self.converged
    }

    /// Check if maximum iterations exceeded
    pub fn is_maxed_out(&self) -> bool {
        self.iteration >= self.max_iterations
    }

    /// Extract and strictly residual-qualify the complete Floquet spectrum,
    /// checking for cooperative cancellation around the atomic eigensolve.
    ///
    /// Floquet multipliers are eigenvalues of the Monodromy matrix.
    /// For a stable orbit, all multipliers should have |λ| ≤ 1.
    /// An autonomous orbit has a unit phase multiplier corresponding to
    /// perturbations along the orbit; a driven periodic solution need not.
    pub fn compute_floquet_multipliers_with_abort(
        &self,
        monodromy: &[Vec<Value>],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<num_complex::Complex64>, FloquetSpectrumError> {
        let (multipliers, _) = self.compute_floquet_spectrum_with_abort(monodromy, abort)?;
        Ok(multipliers)
    }

    /// Extract a complete Floquet spectrum and retain its canonical residual
    /// certificate, checking cancellation around the atomic eigensolve.
    pub fn compute_floquet_spectrum_with_abort(
        &self,
        monodromy: &[Vec<Value>],
        abort: &dyn AbortSignal,
    ) -> Result<(Vec<num_complex::Complex64>, FloquetSpectrumEvidence), FloquetSpectrumError> {
        let spectrum = qualified_real_eigenspectrum(monodromy, abort)?;
        let certificate: FloquetSpectrumCertificate = spectrum.certificate();
        let evidence = FloquetSpectrumEvidence::qualified(certificate).ok_or_else(|| {
            FloquetSpectrumError::Numerical(
                "qualified eigensolver returned an invalid certificate".to_owned(),
            )
        })?;
        Ok((spectrum.into_eigenvalues(), evidence))
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
    use crate::abort_signal::{ImmediateAbort, NoAbort};

    #[test]
    fn shooting_relative_tolerance_never_becomes_an_absolute_small_signal_floor() {
        let mut state = ShootingState::new(vec![0.0], 1.0);
        state.x_t[0] = 5e-8;
        state.compute_residual();
        let mut solver = ShootingNewtonSolver::default();
        assert!(!solver.check_convergence(&state));
        state.x_t[0] = 5e-13;
        state.compute_residual();
        assert!(solver.check_convergence(&state));
    }

    #[test]
    fn shooting_coordinates_cannot_mask_each_others_closure_errors() {
        for current_scale in [0.001, 1.0, 1000.0] {
            let mut state = ShootingState::new(vec![0.1, 3e-5 / current_scale], 1.0);
            state.residual = vec![0.0, 5e-9 / current_scale];
            state.x_t = state
                .x0
                .iter()
                .zip(&state.residual)
                .map(|(a, b)| a + b)
                .collect();
            assert!(!ShootingNewtonSolver::default().check_convergence(&state));
            state.residual[1] = 1e-11 / current_scale;
            state.x_t[1] = state.x0[1] + state.residual[1];
            assert!(ShootingNewtonSolver::default().check_convergence(&state));
        }
        // Different coordinates may legitimately use different criteria.
        let mut state = ShootingState::new(vec![1e6, 0.0], 1.0);
        state.x_t = vec![1e6 + 0.5, 0.5e-12];
        state.compute_residual();
        assert!(ShootingNewtonSolver::default().check_convergence(&state));
    }

    #[test]
    fn component_tolerances_preserve_single_state_convergence() {
        for initial in [-1e300, -1.0, -1e-300, 0.0, 1e-300, 1.0, 1e300] {
            for residual in [
                0.0,
                Value::from_bits(1),
                1e-300,
                5e-13,
                1e-12,
                1e-6,
                1.0,
                1e298,
                1e308,
            ] {
                let mut state = ShootingState::new(vec![initial], 1.0);
                state.residual[0] = residual;
                state.x_t[0] = initial + residual;
                // A single coordinate closes below the absolute tolerance or
                // below the relative tolerance of its initial value.
                let expected =
                    residual.abs() / initial.abs() < 1e-6 || state.residual_norm() < 1e-12;
                assert_eq!(
                    ShootingNewtonSolver::default().check_convergence(&state),
                    expected,
                    "initial={initial:e}, residual={residual:e}"
                );
            }
        }
    }

    #[test]
    fn shooting_norms_preserve_small_and_large_finite_residuals() {
        for scale in [1e-250, 1e-100, 1.0, 1e100, 1e250] {
            let mut state = ShootingState::new(vec![3.0 * scale, 4.0 * scale], 1.0);
            state.residual = vec![3e-5 * scale, 4e-5 * scale];
            let expected = 5e-5 * scale;
            assert!((state.residual_norm() / expected - 1.0).abs() < 1e-14);
        }
    }

    #[test]
    fn shooting_convergence_refuses_nonfinite_or_incomplete_state() {
        let mut solver = ShootingNewtonSolver::default();
        for value in [Value::NAN, Value::INFINITY, Value::NEG_INFINITY] {
            for lane in 0..3 {
                let mut state = ShootingState::new(vec![0.0], 1.0);
                match lane {
                    0 => state.x0[0] = value,
                    1 => state.x_t[0] = value,
                    _ => state.residual[0] = value,
                }
                assert!(!solver.check_convergence(&state), "lane={lane}, {value}");
            }
        }
        let mut state = ShootingState::new(vec![0.0], 1.0);
        state.residual.clear();
        assert!(!solver.check_convergence(&state));
        let state = ShootingState::new(vec![0.0], 1.0);
        assert!(!ShootingNewtonSolver::new(Value::INFINITY, 10).check_convergence(&state));
    }

    #[test]
    fn floquet_spectrum_rejects_empty_ragged_and_nonfinite_monodromy() {
        let solver = ShootingNewtonSolver::default();
        assert!(
            solver
                .compute_floquet_multipliers_with_abort(&[], &NoAbort)
                .is_err()
        );
        assert!(
            solver
                .compute_floquet_multipliers_with_abort(&[vec![1.0, 0.0], vec![0.0]], &NoAbort)
                .is_err()
        );
        assert!(
            solver
                .compute_floquet_multipliers_with_abort(&[vec![Value::NAN]], &NoAbort)
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

    #[test]
    fn floquet_spectrum_retains_canonical_qualification_evidence() {
        let solver = ShootingNewtonSolver::default();
        let (multipliers, evidence) = solver
            .compute_floquet_spectrum_with_abort(&[vec![0.5]], &NoAbort)
            .unwrap();
        assert_eq!(multipliers, vec![num_complex::Complex64::new(0.5, 0.0)]);
        let certificate = evidence.certificate().unwrap();
        assert_eq!(certificate.problem_order, 1);
        assert_eq!(
            certificate.qualification_tolerance,
            FloquetSpectrumCertificate::canonical_qualification_tolerance(1)
        );
    }
}
