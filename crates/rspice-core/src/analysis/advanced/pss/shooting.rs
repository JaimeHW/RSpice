//! Shooting Newton Solver for PSS Analysis
//!
//! Implements the shooting method for finding periodic steady-state solutions.
//! The core algorithm solves the boundary value problem: find x(0) such that x(T) = x(0).

#![allow(clippy::needless_range_loop)]
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
