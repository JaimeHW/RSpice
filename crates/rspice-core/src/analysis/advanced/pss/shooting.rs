//! Shooting Newton Solver for PSS Analysis
//!
//! Implements the shooting method for finding periodic steady-state solutions.
//! The core algorithm solves the boundary value problem: find x(0) such that x(T) = x(0).

use crate::Value;
use crate::solver::{SolverError, StaticMatrix};

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

    /// Monodromy matrix (Jacobian of x(T) w.r.t. x(0))
    /// This is the state transition matrix over one period
    monodromy: Option<Vec<Vec<Value>>>,

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
            monodromy: None,
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

    /// Use direct Jacobian computation if available
    use_direct_jacobian: bool,
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
            use_direct_jacobian: false,
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

    /// Enable direct Jacobian computation (faster if available)
    pub fn with_direct_jacobian(mut self, enable: bool) -> Self {
        self.use_direct_jacobian = enable;
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

    /// Compute Monodromy matrix (state transition matrix over one period)
    ///
    /// This is the Jacobian dX(T)/dX(0), which is used for:
    /// - Stability analysis (Floquet multipliers are eigenvalues)
    /// - PNoise analysis (periodic transfer functions)
    pub fn compute_monodromy<F>(
        &self,
        state: &ShootingState,
        integrate_period: F,
    ) -> Vec<Vec<Value>>
    where
        F: Fn(&[Value]) -> Vec<Value>,
    {
        let n = state.dimension();
        let mut monodromy = vec![vec![0.0; n]; n];

        for j in 0..n {
            let mut x0_plus = state.x0.clone();
            let mut x0_minus = state.x0.clone();

            let h = self.fd_step * (state.x0[j].abs().max(1.0));

            x0_plus[j] += h;
            x0_minus[j] -= h;

            let x_t_plus = integrate_period(&x0_plus);
            let x_t_minus = integrate_period(&x0_minus);

            for i in 0..n {
                monodromy[i][j] = (x_t_plus[i] - x_t_minus[i]) / (2.0 * h);
            }
        }

        monodromy
    }

    /// Extract Floquet multipliers from Monodromy matrix
    ///
    /// Floquet multipliers are eigenvalues of the Monodromy matrix.
    /// For a stable orbit, all multipliers should have |λ| ≤ 1.
    /// One multiplier is always 1 (corresponding to perturbations along the orbit).
    pub fn compute_floquet_multipliers(
        &self,
        monodromy: &[Vec<Value>],
    ) -> Vec<num_complex::Complex64> {
        let n = monodromy.len();
        if n == 0 {
            return Vec::new();
        }

        // For small systems, use power iteration for dominant eigenvalue
        // For production, would use LAPACK/nalgebra eigenvalue decomposition

        // Simplified: return approximation based on trace and determinant
        // (only valid for 2x2, placeholder for larger systems)

        if n == 2 {
            let trace = monodromy[0][0] + monodromy[1][1];
            let det = monodromy[0][0] * monodromy[1][1] - monodromy[0][1] * monodromy[1][0];

            let discriminant = trace * trace - 4.0 * det;

            if discriminant >= 0.0 {
                let sqrt_d = discriminant.sqrt();
                vec![
                    num_complex::Complex64::new((trace + sqrt_d) / 2.0, 0.0),
                    num_complex::Complex64::new((trace - sqrt_d) / 2.0, 0.0),
                ]
            } else {
                let sqrt_d = (-discriminant).sqrt();
                vec![
                    num_complex::Complex64::new(trace / 2.0, sqrt_d / 2.0),
                    num_complex::Complex64::new(trace / 2.0, -sqrt_d / 2.0),
                ]
            }
        } else {
            // For larger systems, use iterative methods
            // Return empty for now - full implementation needs eigenvalue solver
            self.power_iteration_eigenvalues(monodromy, 3)
        }
    }

    /// Power iteration for finding dominant eigenvalues
    fn power_iteration_eigenvalues(
        &self,
        matrix: &[Vec<Value>],
        num_eigenvalues: usize,
    ) -> Vec<num_complex::Complex64> {
        let n = matrix.len();
        if n == 0 {
            return Vec::new();
        }

        let mut eigenvalues = Vec::with_capacity(num_eigenvalues);

        // Simple power iteration for dominant eigenvalue
        let mut v: Vec<Value> = (0..n).map(|i| if i == 0 { 1.0 } else { 0.0 }).collect();
        let mut eigenvalue = 0.0;

        for _ in 0..100 {
            // Matrix-vector multiply
            let mut w: Vec<Value> = vec![0.0; n];
            for i in 0..n {
                for j in 0..n {
                    w[i] += matrix[i][j] * v[j];
                }
            }

            // Compute norm
            let norm: Value = w.iter().map(|x| x * x).sum::<Value>().sqrt();
            if norm < 1e-15 {
                break;
            }

            // Estimate eigenvalue from Rayleigh quotient
            let new_eigenvalue: Value = v.iter().zip(w.iter()).map(|(vi, wi)| vi * wi).sum();

            // Normalize
            for x in w.iter_mut() {
                *x /= norm;
            }
            v = w;

            if (new_eigenvalue - eigenvalue).abs() < 1e-10 {
                eigenvalue = new_eigenvalue;
                break;
            }
            eigenvalue = new_eigenvalue;
        }

        eigenvalues.push(num_complex::Complex64::new(eigenvalue, 0.0));

        // For simplicity, only return dominant eigenvalue
        // Full implementation would use deflation or QR iteration
        eigenvalues
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

/// Result of a single shooting iteration
#[derive(Debug, Clone)]
pub struct ShootingIterationResult {
    /// Residual norm before this iteration
    pub residual_before: Value,

    /// Residual norm after this iteration
    pub residual_after: Value,

    /// Newton step norm
    pub step_norm: Value,

    /// Effective damping used
    pub damping_used: Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shooting_state_creation() {
        let x0 = vec![1.0, 2.0, 3.0];
        let state = ShootingState::new(x0.clone(), 1e-9);

        assert_eq!(state.dimension(), 3);
        assert_eq!(state.x0, x0);
        assert_eq!(state.period, 1e-9);
    }

    #[test]
    fn test_residual_computation() {
        let mut state = ShootingState::new(vec![1.0, 2.0], 1e-9);
        state.x_t = vec![1.1, 1.9];
        state.compute_residual();

        assert!((state.residual[0] - 0.1).abs() < 1e-10);
        assert!((state.residual[1] - (-0.1)).abs() < 1e-10);
    }

    #[test]
    fn test_residual_norm() {
        let mut state = ShootingState::new(vec![1.0, 0.0], 1e-9);
        state.residual = vec![0.3, 0.4]; // 3-4-5 triangle

        let norm = state.residual_norm();
        assert!((norm - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_convergence_check() {
        let mut solver = ShootingNewtonSolver::new(1e-6, 100);

        // Large residual - not converged
        let mut state = ShootingState::new(vec![1.0, 1.0], 1e-9);
        state.residual = vec![0.1, 0.1];
        assert!(!solver.check_convergence(&state));

        // Small residual - converged
        solver.reset();
        state.residual = vec![1e-8, 1e-8];
        assert!(solver.check_convergence(&state));
    }

    #[test]
    fn test_linear_system_solve() {
        let solver = ShootingNewtonSolver::default();

        // Simple 2x2 system: [[2, 1], [1, 3]] * x = [5, 5]
        // Solution: x = [2, 1]
        let jacobian = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let rhs = vec![-5.0, -5.0]; // Negated because solve_linear_system negates

        let result = solver.solve_linear_system(&jacobian, &rhs).unwrap();

        assert!((result[0] - 2.0).abs() < 0.01);
        assert!((result[1] - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_newton_step_simple_map() {
        let solver = ShootingNewtonSolver::new(1e-8, 50);

        // Test with a simple contractive map: x(T) = 0.5 * x(0)
        // Fixed point is x = 0
        // From x0 = 2: x(T) = 1, F = x(T) - x(0) = 1 - 2 = -1
        // Jacobian J = dF/dx0 = d(0.5*x0 - x0)/dx0 = -0.5
        // Newton step: delta = -J^{-1} * F
        // We solve J * delta = -F, so delta = J^{-1} * (-F) = (-2) * (1) = -2
        // Update: x0_new = x0 + delta = 2 + (-2) = 0 (the fixed point!)

        let integrate = |x0: &[Value]| -> Vec<Value> { vec![0.5 * x0[0]] };

        let mut state = ShootingState::new(vec![2.0], 1.0);

        // IMPORTANT: Must compute x_t and residual before Newton step
        state.x_t = integrate(&state.x0);
        state.compute_residual();

        // Verify residual is correct: F = x(T) - x(0) = 1 - 2 = -1
        assert!(
            (state.residual[0] - (-1.0)).abs() < 1e-10,
            "Residual should be -1.0, got {}",
            state.residual[0]
        );

        let delta = solver.compute_newton_step(&state, integrate).unwrap();

        // Delta should be -2.0 (to move from x0=2 to x0=0)
        // The exact value depends on finite difference accuracy
        assert!(
            (delta[0] - (-2.0)).abs() < 0.1,
            "Newton delta should be ~-2.0, got {}",
            delta[0]
        );

        // Verify that applying the delta moves us toward the fixed point
        state.update_x0(&delta, 1.0);
        assert!(
            state.x0[0].abs() < 0.1,
            "After Newton step, should be near fixed point x=0, got {}",
            state.x0[0]
        );
    }

    #[test]
    fn test_monodromy_computation() {
        let solver = ShootingNewtonSolver::new(1e-8, 50);

        // Linear map: x' = A * x where A = [[0.9, 0.1], [0, 0.8]]
        let state = ShootingState::new(vec![1.0, 1.0], 1.0);

        let integrate =
            |x0: &[Value]| -> Vec<Value> { vec![0.9 * x0[0] + 0.1 * x0[1], 0.8 * x0[1]] };

        let monodromy = solver.compute_monodromy(&state, integrate);

        // Should recover the matrix A approximately
        assert!((monodromy[0][0] - 0.9).abs() < 0.01);
        assert!((monodromy[0][1] - 0.1).abs() < 0.01);
        assert!((monodromy[1][0] - 0.0).abs() < 0.01);
        assert!((monodromy[1][1] - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_floquet_multipliers_2x2() {
        let solver = ShootingNewtonSolver::default();

        // Monodromy with known eigenvalues λ = 0.9, 0.8
        let monodromy = vec![vec![0.9, 0.0], vec![0.0, 0.8]];

        let multipliers = solver.compute_floquet_multipliers(&monodromy);

        assert_eq!(multipliers.len(), 2);
        // Check eigenvalues (order may vary)
        let mags: Vec<Value> = multipliers.iter().map(|m| m.re).collect();
        assert!(mags.iter().any(|&m| (m - 0.9).abs() < 0.01));
        assert!(mags.iter().any(|&m| (m - 0.8).abs() < 0.01));
    }

    #[test]
    fn test_complex_floquet_multipliers() {
        let solver = ShootingNewtonSolver::default();

        // Rotation matrix: should have complex eigenvalues on unit circle
        let angle = std::f64::consts::PI / 4.0; // 45 degrees
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let monodromy = vec![vec![cos_a, -sin_a], vec![sin_a, cos_a]];

        let multipliers = solver.compute_floquet_multipliers(&monodromy);

        assert_eq!(multipliers.len(), 2);
        // Both should have magnitude 1 (on unit circle)
        for m in &multipliers {
            assert!((m.norm() - 1.0).abs() < 0.01);
        }
    }

    #[test]
    fn test_solver_reset() {
        let mut solver = ShootingNewtonSolver::new(1e-6, 100);
        solver.iteration = 50;
        solver.converged = true;

        solver.reset();

        assert_eq!(solver.iterations(), 0);
        assert!(!solver.has_converged());
    }

    #[test]
    fn test_damping_clamp() {
        let solver = ShootingNewtonSolver::default().with_damping(2.0);
        assert_eq!(solver.damping, 1.0);

        let solver2 = ShootingNewtonSolver::default().with_damping(0.01);
        assert_eq!(solver2.damping, 0.1);
    }
}
