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
        if monodromy
            .iter()
            .any(|row| row.len() != n || row.iter().any(|v| !v.is_finite()))
        {
            return Vec::new();
        }

        if n == 1 {
            return vec![num_complex::Complex64::new(monodromy[0][0], 0.0)];
        }
        if n == 2 {
            return self.eigenvalues_2x2(
                monodromy[0][0],
                monodromy[0][1],
                monodromy[1][0],
                monodromy[1][1],
            );
        }

        self.qr_eigenvalues(monodromy)
    }

    fn eigenvalues_2x2(
        &self,
        a00: Value,
        a01: Value,
        a10: Value,
        a11: Value,
    ) -> Vec<num_complex::Complex64> {
        let trace = a00 + a11;
        let det = a00 * a11 - a01 * a10;
        let discriminant = trace * trace - 4.0 * det;

        if discriminant >= 0.0 {
            let sqrt_d = discriminant.sqrt();
            vec![
                num_complex::Complex64::new((trace + sqrt_d) / 2.0, 0.0),
                num_complex::Complex64::new((trace - sqrt_d) / 2.0, 0.0),
            ]
        } else {
            let sqrt_d = (-discriminant).sqrt() / 2.0;
            vec![
                num_complex::Complex64::new(trace / 2.0, sqrt_d),
                num_complex::Complex64::new(trace / 2.0, -sqrt_d),
            ]
        }
    }

    fn qr_eigenvalues(&self, matrix: &[Vec<Value>]) -> Vec<num_complex::Complex64> {
        let n = matrix.len();
        let mut a = matrix.to_vec();
        let tol = 1e-12;
        let max_iter = self.max_iterations.max(200) * n.max(2);

        for _ in 0..max_iter {
            let mut converged = true;
            for i in 1..n {
                if a[i][i - 1].abs() > tol {
                    converged = false;
                    break;
                }
            }
            if converged {
                break;
            }

            // Basic shifted QR iteration.
            let shift = a[n - 1][n - 1];
            for (i, row) in a.iter_mut().enumerate().take(n) {
                row[i] -= shift;
            }

            let (q, r) = self.qr_decompose(&a);
            a = self.matrix_multiply(&r, &q);

            for (i, row) in a.iter_mut().enumerate().take(n) {
                row[i] += shift;
            }
        }

        let mut eigenvalues = Vec::with_capacity(n);
        let mut i = 0;
        while i < n {
            if i == n - 1 || a[i + 1][i].abs() < tol {
                eigenvalues.push(num_complex::Complex64::new(a[i][i], 0.0));
                i += 1;
            } else {
                eigenvalues.extend(self.eigenvalues_2x2(
                    a[i][i],
                    a[i][i + 1],
                    a[i + 1][i],
                    a[i + 1][i + 1],
                ));
                i += 2;
            }
        }

        eigenvalues
    }

    fn qr_decompose(&self, a: &[Vec<Value>]) -> (Vec<Vec<Value>>, Vec<Vec<Value>>) {
        let n = a.len();
        let mut q = vec![vec![0.0; n]; n];
        let mut r = vec![vec![0.0; n]; n];
        let cols: Vec<Vec<Value>> = (0..n).map(|j| (0..n).map(|i| a[i][j]).collect()).collect();

        for j in 0..n {
            let mut v = cols[j].clone();
            for i in 0..j {
                let q_col: Vec<Value> = (0..n).map(|k| q[k][i]).collect();
                let dot: Value = v.iter().zip(&q_col).map(|(x, y)| x * y).sum();
                r[i][j] = dot;
                for k in 0..n {
                    v[k] -= dot * q_col[k];
                }
            }

            let norm = v.iter().map(|x| x * x).sum::<Value>().sqrt();
            r[j][j] = norm;
            if norm > 1e-15 {
                for k in 0..n {
                    q[k][j] = v[k] / norm;
                }
            }
        }

        (q, r)
    }

    fn matrix_multiply(&self, a: &[Vec<Value>], b: &[Vec<Value>]) -> Vec<Vec<Value>> {
        let n = a.len();
        let mut out = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                let mut sum = 0.0;
                for k in 0..n {
                    sum += a[i][k] * b[k][j];
                }
                out[i][j] = sum;
            }
        }
        out
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
    fn test_floquet_multipliers_3x3_diagonal_matrix() {
        let solver = ShootingNewtonSolver::default();
        let monodromy = vec![
            vec![0.9, 0.0, 0.0],
            vec![0.0, 0.7, 0.0],
            vec![0.0, 0.0, 0.5],
        ];

        let multipliers = solver.compute_floquet_multipliers(&monodromy);
        assert_eq!(multipliers.len(), 3);

        let mut reals: Vec<Value> = multipliers.iter().map(|m| m.re).collect();
        reals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        assert!((reals[0] - 0.5).abs() < 1e-3);
        assert!((reals[1] - 0.7).abs() < 1e-3);
        assert!((reals[2] - 0.9).abs() < 1e-3);
    }

    #[test]
    fn test_floquet_multipliers_extract_complex_pair_from_larger_system() {
        let solver = ShootingNewtonSolver::default();
        let angle = std::f64::consts::PI / 6.0; // 30 degrees
        let mag = 0.8;
        let cos_a = mag * angle.cos();
        let sin_a = mag * angle.sin();

        // Block-diagonal: [rotation*mag] + [0.6] + [0.3]
        let monodromy = vec![
            vec![cos_a, -sin_a, 0.0, 0.0],
            vec![sin_a, cos_a, 0.0, 0.0],
            vec![0.0, 0.0, 0.6, 0.0],
            vec![0.0, 0.0, 0.0, 0.3],
        ];

        let multipliers = solver.compute_floquet_multipliers(&monodromy);
        assert_eq!(multipliers.len(), 4);

        let has_complex_pair = multipliers.iter().any(|m| m.im.abs() > 1e-3);
        assert!(has_complex_pair, "expected complex conjugate pair");

        let mut magnitudes: Vec<Value> = multipliers.iter().map(|m| m.norm()).collect();
        magnitudes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        assert!((magnitudes[0] - 0.3).abs() < 1e-3);
        assert!((magnitudes[1] - 0.6).abs() < 1e-3);
        assert!((magnitudes[2] - 0.8).abs() < 2e-2);
        assert!((magnitudes[3] - 0.8).abs() < 2e-2);
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
