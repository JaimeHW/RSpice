//! Shooting Newton Solver for PSS Analysis
//!
//! Implements the shooting method for finding periodic steady-state solutions.
//! The core algorithm solves the boundary value problem: find x(0) such that x(T) = x(0).

use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::{FloquetSpectrumCertificate, FloquetSpectrumEvidence};
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
        self.residual.iter().fold(0.0, |norm, &r| norm.hypot(r))
    }

    /// Get the relative L2 residual. A zero initial state has zero relative
    /// error only for an exact zero residual; otherwise the absolute
    /// convergence criterion must decide whether the orbit has closed.
    pub fn relative_residual_norm(&self) -> Value {
        let Some(scale) = self
            .x0
            .iter()
            .chain(&self.residual)
            .try_fold(0.0_f64, |scale, &x| {
                x.is_finite().then_some(scale.max(x.abs()))
            })
        else {
            return Value::NAN;
        };
        if scale == 0.0 {
            return 0.0;
        }
        // Normalize both vectors by the same maximum component before
        // taking either norm. Either L2 norm may exceed f64 while the ratio
        // fits; every normalized component is bounded by one.
        let norm = |values: &[Value]| {
            values
                .iter()
                .fold(0.0_f64, |norm, &x| norm.hypot(x / scale))
        };
        norm(&self.residual) / norm(&self.x0)
    }

    /// Update initial state estimate
    ///
    /// Applies Newton step: x0_new = x0 + damping * delta
    /// Note: delta is pre-computed as -J^(-1)*F by compute_newton_step
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

    /// Solve and certify `J * delta = -F` without changing the Jacobian.
    fn solve_linear_system(
        &self,
        jacobian: &[Vec<Value>],
        rhs: &[Value],
    ) -> Result<Vec<Value>, SolverError> {
        let n = rhs.len();

        if n == 0 {
            return if jacobian.is_empty() {
                Ok(Vec::new())
            } else {
                Err(SolverError::InvalidCircuit(format!(
                    "Shooting Newton system has {} matrix rows but an empty RHS",
                    jacobian.len()
                )))
            };
        }
        if jacobian.len() != n {
            return Err(SolverError::InvalidCircuit(format!(
                "Shooting Newton dimension mismatch: matrix has {} rows, RHS has {n}",
                jacobian.len()
            )));
        }
        if rhs.iter().any(|value| !value.is_finite()) {
            return Err(SolverError::Overflow);
        }

        // Retain exact-zero diagonal entries structurally so factorization,
        // rather than sparse assembly, owns the singularity diagnosis.
        let mut triplets = Vec::with_capacity(n.saturating_mul(n));
        for (row_index, row) in jacobian.iter().enumerate() {
            if row.len() != n {
                return Err(SolverError::InvalidCircuit(format!(
                    "Shooting Newton row {row_index} has {} columns; expected {n}",
                    row.len()
                )));
            }
            for (col_index, &value) in row.iter().enumerate() {
                if !value.is_finite() {
                    return Err(SolverError::Overflow);
                }
                if row_index == col_index || value != 0.0 {
                    triplets.push((row_index, col_index, value));
                }
            }
        }

        let mut matrix = StaticMatrix::from_triplets(n, n, &triplets)?;
        let neg_rhs: Vec<Value> = rhs.iter().map(|r| -r).collect();
        match matrix.solve(&neg_rhs) {
            Ok(solution) => Ok(solution),
            Err(SolverError::InaccurateSolution(_)) if n <= 64 => {
                matrix.solve_dense_extended(&neg_rhs)
            }
            Err(error) => Err(error),
        }
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
        let (multipliers, _) = self.compute_floquet_spectrum_with_abort(monodromy, abort)?;
        Ok(multipliers)
    }

    /// Extract a complete Floquet spectrum together with strict evidence.
    pub fn compute_floquet_spectrum(
        &self,
        monodromy: &[Vec<Value>],
    ) -> Result<(Vec<num_complex::Complex64>, FloquetSpectrumEvidence), FloquetSpectrumError> {
        self.compute_floquet_spectrum_with_abort(monodromy, &NoAbort)
    }

    /// Extract a complete Floquet spectrum and retain its canonical residual
    /// certificate, checking cancellation around the atomic eigensolve.
    pub fn compute_floquet_spectrum_with_abort(
        &self,
        monodromy: &[Vec<Value>],
        abort: &dyn AbortSignal,
    ) -> Result<(Vec<num_complex::Complex64>, FloquetSpectrumEvidence), FloquetSpectrumError> {
        let spectrum = qualified_real_eigenspectrum(monodromy, abort)?;
        let certificate: FloquetSpectrumCertificate = spectrum.certificate;
        let evidence = FloquetSpectrumEvidence::qualified(certificate).ok_or_else(|| {
            FloquetSpectrumError::Numerical(
                "qualified eigensolver returned an invalid certificate".to_owned(),
            )
        })?;
        Ok((spectrum.eigenvalues, evidence))
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
    fn shooting_relative_tolerance_never_becomes_an_absolute_small_signal_floor() {
        let mut state = ShootingState::new(vec![0.0], 1.0);
        state.x_t[0] = 5e-8;
        state.compute_residual();
        let mut solver = ShootingNewtonSolver::default();
        assert!(!solver.check_convergence(&state));
        state.x_t[0] = 5e-13;
        state.compute_residual();
        assert!(solver.check_convergence(&state));
        state.x_t[0] = 0.0;
        state.compute_residual();
        assert_eq!(state.relative_residual_norm(), 0.0);
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
                let expected =
                    state.relative_residual_norm() < 1e-6 || state.residual_norm() < 1e-12;
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
            assert!((state.relative_residual_norm() / 1e-5 - 1.0).abs() < 1e-14);
        }
        // The initial norm itself exceeds f64, although the state entries,
        // residual norm and relative error are all representable.
        let mut state = ShootingState::new(vec![1e308; 4], 1.0);
        state.residual = vec![1e298; 4];
        assert!((state.relative_residual_norm() / 1e-10 - 1.0).abs() < 1e-14);
        let mut state = ShootingState::new(vec![1.0; 4], 1.0);
        state.residual = vec![1e308; 4];
        assert!((state.relative_residual_norm() / 1e308 - 1.0).abs() < 1e-14);
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
    fn shooting_newton_linear_solve_fails_closed_on_singular_system() {
        let solver = ShootingNewtonSolver::default();
        assert!(matches!(
            solver.solve_linear_system(&[vec![0.0]], &[1.0]),
            Err(SolverError::SingularMatrix)
        ));
    }

    #[test]
    fn shooting_newton_linear_solve_preserves_tiny_physical_scale() {
        let solver = ShootingNewtonSolver::default();
        let solution = solver
            .solve_linear_system(&[vec![1.0e-18]], &[-1.0])
            .expect("a finite tiny coefficient is nonsingular");
        assert!((solution[0] / 1.0e18 - 1.0).abs() <= 4.0 * Value::EPSILON);
    }

    #[test]
    fn shooting_newton_linear_solve_rejects_malformed_and_nonfinite_inputs() {
        let solver = ShootingNewtonSolver::default();
        assert!(matches!(
            solver.solve_linear_system(&[vec![1.0], vec![2.0]], &[1.0]),
            Err(SolverError::InvalidCircuit(_))
        ));
        assert!(matches!(
            solver.solve_linear_system(&[vec![Value::NAN]], &[1.0]),
            Err(SolverError::Overflow)
        ));
        assert!(matches!(
            solver.solve_linear_system(&[vec![1.0]], &[Value::INFINITY]),
            Err(SolverError::Overflow)
        ));
    }

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

    #[test]
    fn floquet_spectrum_retains_canonical_qualification_evidence() {
        let solver = ShootingNewtonSolver::default();
        let (multipliers, evidence) = solver.compute_floquet_spectrum(&[vec![0.5]]).unwrap();
        assert_eq!(multipliers, vec![num_complex::Complex64::new(0.5, 0.0)]);
        let certificate = evidence.certificate().unwrap();
        assert_eq!(certificate.problem_order, 1);
        assert_eq!(
            certificate.qualification_tolerance,
            FloquetSpectrumCertificate::canonical_qualification_tolerance(1)
        );
    }
}
