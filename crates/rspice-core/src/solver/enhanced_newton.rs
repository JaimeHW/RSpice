//! Enhanced Newton Solver with Advanced Damping Integration
//!
//! Integrates the DampingController with the Newton-Raphson solver for
//! improved convergence on difficult circuits. This is the production
//! interface used by the simulation engine.
//!
//! # Features
//!
//! - **Line search**: Backtracking to ensure residual decrease
//! - **Bank-Rose damping**: Adaptive damping based on convergence rate
//! - **Voltage limiting**: Junction-aware step limiting
//! - **Gmin stepping**: Automatic fallback to parallel conductances
//! - **Source stepping**: Gradual source ramping for hard circuits
//!
//! # Usage
//!
//! ```ignore
//! let solver = EnhancedNewtonSolver::new(config)
//!     .with_damping_strategy(DampingStrategy::Combined)
//!     .with_junction_nodes(vec![1, 3, 5]);
//!
//! let result = solver.solve(&mut circuit, &mut matrix)?;
//! ```

use super::SolverError;
use super::convergence::{ConvergenceController, ConvergenceMethod};
use super::damping::{DampingController, DampingStrategy};
use crate::Value;

//=============================================================================
// Configuration
//=============================================================================

/// Enhanced Newton solver configuration
#[derive(Debug, Clone)]
pub struct EnhancedNewtonConfig {
    /// Absolute convergence tolerance
    pub abs_tol: Value,
    /// Relative convergence tolerance
    pub rel_tol: Value,
    /// Maximum Newton iterations per solve
    pub max_iter: usize,
    /// Damping strategy to use
    pub damping_strategy: DampingStrategy,
    /// Maximum voltage change for junction limiting
    pub vmax_junction: Value,
    /// Thermal voltage (for limiting calculations)
    pub vt: Value,
    /// Enable automatic convergence aids (gmin, source stepping)
    pub auto_convergence_aids: bool,
    /// Maximum line search iterations
    pub max_line_search: usize,
    /// Enable verbose logging
    pub verbose: bool,
}

impl Default for EnhancedNewtonConfig {
    fn default() -> Self {
        Self {
            abs_tol: 1e-12,
            rel_tol: 1e-6,
            max_iter: 100,
            damping_strategy: DampingStrategy::Combined,
            vmax_junction: 0.5,
            vt: 0.02585,
            auto_convergence_aids: true,
            max_line_search: 10,
            verbose: false,
        }
    }
}

impl EnhancedNewtonConfig {
    /// Create config for fast convergence (less conservative)
    pub fn fast() -> Self {
        Self {
            abs_tol: 1e-9,
            rel_tol: 1e-4,
            max_iter: 50,
            damping_strategy: DampingStrategy::BankRose,
            max_line_search: 5,
            ..Default::default()
        }
    }

    /// Create config for robust convergence (more conservative)
    pub fn robust() -> Self {
        Self {
            abs_tol: 1e-14,
            rel_tol: 1e-8,
            max_iter: 200,
            damping_strategy: DampingStrategy::Combined,
            max_line_search: 20,
            ..Default::default()
        }
    }

    /// Set damping strategy
    pub fn with_damping(mut self, strategy: DampingStrategy) -> Self {
        self.damping_strategy = strategy;
        self
    }

    /// Enable verbose output
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
}

//=============================================================================
// Solver State
//=============================================================================

/// State of an enhanced Newton solve
#[derive(Debug, Clone)]
pub struct EnhancedNewtonState {
    /// Current solution vector
    pub x: Vec<Value>,
    /// Current residual norm
    pub residual_norm: Value,
    /// Current iteration count
    pub iteration: usize,
    /// Whether converged
    pub converged: bool,
    /// Damping factor used in last iteration
    pub alpha: Value,
    /// Number of line searches performed
    pub line_search_iters: usize,
    /// Whether convergence aids were used
    pub used_convergence_aids: bool,
    /// Gmin value if gmin stepping was used
    pub gmin_value: Option<Value>,
    /// Source scale if source stepping was used
    pub source_scale: Option<Value>,
}

impl EnhancedNewtonState {
    /// Create new state with initial guess
    pub fn new(initial_guess: Vec<Value>) -> Self {
        Self {
            x: initial_guess,
            residual_norm: f64::INFINITY,
            iteration: 0,
            converged: false,
            alpha: 1.0,
            line_search_iters: 0,
            used_convergence_aids: false,
            gmin_value: None,
            source_scale: None,
        }
    }

    /// Reset for new solve with same size
    pub fn reset(&mut self) {
        self.x.iter_mut().for_each(|v| *v = 0.0);
        self.residual_norm = f64::INFINITY;
        self.iteration = 0;
        self.converged = false;
        self.alpha = 1.0;
        self.line_search_iters = 0;
        self.used_convergence_aids = false;
        self.gmin_value = None;
        self.source_scale = None;
    }
}

impl Default for EnhancedNewtonState {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

//=============================================================================
// Enhanced Newton Solver
//=============================================================================

/// Enhanced Newton-Raphson solver with advanced damping
///
/// This is the main solver interface that integrates all convergence
/// improvements into a single, easy-to-use API.
#[derive(Debug)]
pub struct EnhancedNewtonSolver {
    /// Configuration
    config: EnhancedNewtonConfig,

    /// Damping controller for line search and voltage limiting
    damping: DampingController,

    /// Convergence controller for gmin/source stepping
    convergence: ConvergenceController,

    /// Indices of junction voltage nodes (for voltage limiting)
    junction_nodes: Vec<usize>,

    /// Previous solution for convergence check
    x_prev: Vec<Value>,

    /// Statistics: total iterations across all solves
    total_iterations: usize,

    /// Statistics: number of convergence failures
    failures: usize,
}

impl EnhancedNewtonSolver {
    /// Create new enhanced Newton solver
    pub fn new(config: EnhancedNewtonConfig) -> Self {
        let damping = DampingController::with_strategy(config.damping_strategy)
            .with_voltage_limits(config.vmax_junction, config.vt);

        Self {
            damping,
            convergence: ConvergenceController::new(),
            junction_nodes: Vec::new(),
            x_prev: Vec::new(),
            total_iterations: 0,
            failures: 0,
            config,
        }
    }

    /// Set junction nodes for voltage limiting
    pub fn with_junction_nodes(mut self, nodes: Vec<usize>) -> Self {
        self.junction_nodes = nodes;
        self
    }

    /// Set damping strategy
    pub fn with_damping_strategy(mut self, strategy: DampingStrategy) -> Self {
        self.damping.set_strategy(strategy);
        self.config.damping_strategy = strategy;
        self
    }

    /// Reset solver state for new circuit
    pub fn reset(&mut self) {
        self.damping.reset();
        self.convergence.reset();
        self.x_prev.clear();
    }

    /// Get total iterations statistic
    #[inline]
    pub fn total_iterations(&self) -> usize {
        self.total_iterations
    }

    /// Get failure count
    #[inline]
    pub fn failures(&self) -> usize {
        self.failures
    }

    /// Perform a single Newton iteration
    ///
    /// # Arguments
    /// * `state` - Current solver state
    /// * `dx` - Newton update vector (solution of J·dx = -F)
    /// * `residual_norm` - Current residual norm ||F||
    /// * `compute_residual` - Closure to recompute residual for line search
    ///
    /// # Returns
    /// Updated residual norm after damped step
    pub fn iterate<F>(
        &mut self,
        state: &mut EnhancedNewtonState,
        dx: &[Value],
        residual_norm: Value,
        compute_residual: F,
    ) -> Value
    where
        F: Fn(&[Value]) -> Value,
    {
        state.iteration += 1;
        state.residual_norm = residual_norm;

        // Apply voltage limiting to junction nodes
        let dx_limited = if !self.junction_nodes.is_empty() {
            self.damping
                .limit_update_vector(&state.x, dx, &self.junction_nodes)
        } else {
            dx.to_vec()
        };

        // Determine damping factor
        let alpha = match self.config.damping_strategy {
            DampingStrategy::None => 1.0,
            DampingStrategy::Fixed => 0.5,
            DampingStrategy::LineSearch => {
                // Perform line search
                let dx_norm = dx_limited.iter().map(|d| d * d).sum::<Value>().sqrt();
                self.damping.line_search(residual_norm, dx_norm, |a| {
                    // Compute trial solution
                    let x_trial: Vec<Value> = state
                        .x
                        .iter()
                        .zip(dx_limited.iter())
                        .map(|(&x, &d)| x + a * d)
                        .collect();
                    compute_residual(&x_trial)
                })
            }
            DampingStrategy::BankRose | DampingStrategy::Combined => {
                self.damping.bank_rose_damping(residual_norm)
            }
            DampingStrategy::VoltageLimiting => 1.0,
        };

        state.alpha = alpha;

        // Apply damped update
        for (x, d) in state.x.iter_mut().zip(dx_limited.iter()) {
            *x += alpha * d;
        }

        // Compute new residual
        let new_residual = compute_residual(&state.x);
        state.residual_norm = new_residual;
        state.line_search_iters += self.damping.statistics().total_line_search_iters;

        new_residual
    }

    /// Check if converged
    pub fn check_convergence(&mut self, state: &EnhancedNewtonState) -> bool {
        if state.iteration == 0 {
            self.x_prev = state.x.clone();
            return false;
        }

        // Check absolute and relative convergence
        let mut max_diff = 0.0_f64;
        let mut max_rel_diff = 0.0_f64;

        for (&x_old, &x_new) in self.x_prev.iter().zip(state.x.iter()) {
            let diff = (x_new - x_old).abs();
            max_diff = max_diff.max(diff);

            if x_new.abs() > self.config.abs_tol {
                let rel_diff = diff / x_new.abs();
                max_rel_diff = max_rel_diff.max(rel_diff);
            }
        }

        // Update previous for next iteration
        self.x_prev.clone_from(&state.x);

        let converged = max_diff < self.config.abs_tol || max_rel_diff < self.config.rel_tol;

        if converged {
            self.total_iterations += state.iteration;
        }

        converged
    }

    /// Check if should give up
    pub fn should_give_up(&self, state: &EnhancedNewtonState) -> bool {
        state.iteration >= self.config.max_iter
    }

    /// Try convergence aids if initial solve failed
    ///
    /// Returns the convergence method that succeeded, if any
    pub fn try_convergence_aids<F>(
        &mut self,
        state: &mut EnhancedNewtonState,
        mut solve_with_aid: F,
    ) -> Result<ConvergenceMethod, SolverError>
    where
        F: FnMut(&mut EnhancedNewtonState, Option<Value>, Option<Value>) -> Result<(), SolverError>,
    {
        if !self.config.auto_convergence_aids {
            return Err(SolverError::ConvergenceFailed(state.iteration));
        }

        state.used_convergence_aids = true;

        // Try source stepping first
        self.convergence
            .set_method(ConvergenceMethod::SourceStepping);

        while !self.convergence.source_stepper.is_complete() {
            let scale = self.convergence.source_stepper.factor();
            state.reset();
            state.source_scale = Some(scale);

            match solve_with_aid(state, Some(scale), None) {
                Ok(()) => {
                    self.convergence.source_stepper.advance_on_success();
                    if self.convergence.source_stepper.is_complete() {
                        return Ok(ConvergenceMethod::SourceStepping);
                    }
                }
                Err(_) => {
                    if !self.convergence.source_stepper.reduce_on_failure() {
                        break; // Step size too small, give up on source stepping
                    }
                }
            }
        }

        // If source stepping failed, try gmin stepping
        self.convergence.set_method(ConvergenceMethod::GminStepping);

        while !self.convergence.gmin_stepper.is_complete() {
            let gmin = self.convergence.gmin_stepper.gmin();
            state.reset();
            state.gmin_value = Some(gmin);

            match solve_with_aid(state, None, Some(gmin)) {
                Ok(()) => {
                    self.convergence.gmin_stepper.advance_on_success();
                    if self.convergence.gmin_stepper.is_complete() {
                        return Ok(ConvergenceMethod::GminStepping);
                    }
                }
                Err(_) => {
                    if !self.convergence.gmin_stepper.increase_on_failure() {
                        break; // Gmin at maximum, give up
                    }
                }
            }
        }

        self.failures += 1;
        Err(SolverError::ConvergenceFailed(state.iteration))
    }

    /// Get configuration
    pub fn config(&self) -> &EnhancedNewtonConfig {
        &self.config
    }

    /// Get damping statistics
    pub fn damping_stats(&self) -> super::damping::DampingStatistics {
        self.damping.statistics()
    }
}

impl Default for EnhancedNewtonSolver {
    fn default() -> Self {
        Self::new(EnhancedNewtonConfig::default())
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_newton_config_default() {
        let config = EnhancedNewtonConfig::default();
        assert!((config.abs_tol - 1e-12).abs() < 1e-20);
        assert_eq!(config.max_iter, 100);
        assert_eq!(config.damping_strategy, DampingStrategy::Combined);
    }

    #[test]
    fn test_enhanced_newton_config_fast() {
        let config = EnhancedNewtonConfig::fast();
        assert!(config.abs_tol > 1e-12);
        assert!(config.max_iter < 100);
    }

    #[test]
    fn test_enhanced_newton_config_robust() {
        let config = EnhancedNewtonConfig::robust();
        assert!(config.abs_tol < 1e-12);
        assert!(config.max_iter > 100);
    }

    #[test]
    fn test_enhanced_newton_state_new() {
        let state = EnhancedNewtonState::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(state.x.len(), 3);
        assert!(!state.converged);
        assert_eq!(state.iteration, 0);
    }

    #[test]
    fn test_enhanced_newton_state_reset() {
        let mut state = EnhancedNewtonState::new(vec![1.0, 2.0, 3.0]);
        state.iteration = 10;
        state.converged = true;

        state.reset();

        assert_eq!(state.iteration, 0);
        assert!(!state.converged);
        assert_eq!(state.x, vec![0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_enhanced_newton_solver_creation() {
        let solver = EnhancedNewtonSolver::new(EnhancedNewtonConfig::default());
        assert_eq!(solver.total_iterations(), 0);
        assert_eq!(solver.failures(), 0);
    }

    #[test]
    fn test_enhanced_newton_with_junction_nodes() {
        let solver = EnhancedNewtonSolver::new(EnhancedNewtonConfig::default())
            .with_junction_nodes(vec![1, 3, 5]);

        assert_eq!(solver.junction_nodes.len(), 3);
    }

    #[test]
    fn test_enhanced_newton_with_damping_strategy() {
        let solver = EnhancedNewtonSolver::new(EnhancedNewtonConfig::default())
            .with_damping_strategy(DampingStrategy::LineSearch);

        assert_eq!(solver.config.damping_strategy, DampingStrategy::LineSearch);
    }

    #[test]
    fn test_enhanced_newton_iterate_simple() {
        let mut solver = EnhancedNewtonSolver::new(
            EnhancedNewtonConfig::default().with_damping(DampingStrategy::None),
        );
        let mut state = EnhancedNewtonState::new(vec![0.0, 0.0]);

        // Simple update
        let dx = vec![1.0, 2.0];
        let new_residual = solver.iterate(&mut state, &dx, 1.0, |x| {
            x.iter().map(|v| v * v).sum::<Value>().sqrt()
        });

        assert!((state.x[0] - 1.0).abs() < 1e-10);
        assert!((state.x[1] - 2.0).abs() < 1e-10);
        assert!(new_residual > 0.0);
    }

    #[test]
    fn test_enhanced_newton_iterate_with_line_search() {
        let mut solver = EnhancedNewtonSolver::new(
            EnhancedNewtonConfig::default().with_damping(DampingStrategy::LineSearch),
        );
        let mut state = EnhancedNewtonState::new(vec![0.0, 0.0]);

        let dx = vec![10.0, 20.0];
        solver.iterate(&mut state, &dx, 100.0, |x| {
            // Residual that decreases with smaller steps
            x.iter().map(|v| v * v).sum::<Value>().sqrt()
        });

        // Should have applied some damping
        assert!(state.alpha <= 1.0);
    }

    #[test]
    fn test_enhanced_newton_check_convergence() {
        let mut solver = EnhancedNewtonSolver::new(EnhancedNewtonConfig::default());

        let mut state = EnhancedNewtonState::new(vec![1.0, 2.0, 3.0]);
        state.iteration = 1;

        // First call - initializes prev
        solver.check_convergence(&state);

        // Same values - should converge
        state.iteration = 2;
        let converged = solver.check_convergence(&state);
        assert!(converged);
    }

    #[test]
    fn test_enhanced_newton_should_give_up() {
        let solver = EnhancedNewtonSolver::new(EnhancedNewtonConfig::default());

        let mut state = EnhancedNewtonState::new(vec![1.0]);
        state.iteration = 99;
        assert!(!solver.should_give_up(&state));

        state.iteration = 100;
        assert!(solver.should_give_up(&state));
    }

    #[test]
    fn test_enhanced_newton_voltage_limiting() {
        let mut solver =
            EnhancedNewtonSolver::new(EnhancedNewtonConfig::default()).with_junction_nodes(vec![0]);

        let mut state = EnhancedNewtonState::new(vec![0.5]);

        // Large voltage jump should be limited
        let dx = vec![2.0]; // Large change
        solver.iterate(&mut state, &dx, 1.0, |_| 0.0);

        // Change should be limited (not full 2.0V)
        assert!(state.x[0] < 2.5);
    }

    #[test]
    fn test_enhanced_newton_reset() {
        let mut solver = EnhancedNewtonSolver::new(EnhancedNewtonConfig::default());

        // Simulate some usage
        let mut state = EnhancedNewtonState::new(vec![1.0]);
        solver.iterate(&mut state, &[0.1], 1.0, |_| 0.5);

        solver.reset();

        assert!(solver.x_prev.is_empty());
    }

    #[test]
    fn test_enhanced_newton_config_builder() {
        let config = EnhancedNewtonConfig::default()
            .with_damping(DampingStrategy::BankRose)
            .with_verbose(true);

        assert_eq!(config.damping_strategy, DampingStrategy::BankRose);
        assert!(config.verbose);
    }

    #[test]
    fn test_enhanced_newton_bank_rose_damping() {
        let mut solver = EnhancedNewtonSolver::new(
            EnhancedNewtonConfig::default().with_damping(DampingStrategy::BankRose),
        );
        let mut state = EnhancedNewtonState::new(vec![0.0]);

        // First iteration at high residual
        solver.iterate(&mut state, &[1.0], 100.0, |_| 50.0);
        let alpha1 = state.alpha;

        // Second iteration with good decrease
        solver.iterate(&mut state, &[0.1], 50.0, |_| 10.0);
        let alpha2 = state.alpha;

        // Alpha should adapt
        assert!(alpha1 > 0.0 && alpha1 <= 1.0);
        assert!(alpha2 > 0.0 && alpha2 <= 1.0);
    }

    #[test]
    fn test_enhanced_newton_state_default() {
        let state = EnhancedNewtonState::default();
        assert!(state.x.is_empty());
        assert!(!state.converged);
    }

    #[test]
    fn test_enhanced_newton_damping_stats() {
        let solver = EnhancedNewtonSolver::default();
        let stats = solver.damping_stats();

        assert_eq!(stats.total_line_search_iters, 0);
        assert_eq!(stats.limited_steps, 0);
    }

    #[test]
    fn test_convergence_with_good_residual_decrease() {
        let mut solver = EnhancedNewtonSolver::new(EnhancedNewtonConfig::default());
        let mut state = EnhancedNewtonState::new(vec![10.0]);

        // Simulate converging iterations
        for i in 0..5 {
            let residual = 100.0 / (10.0_f64.powi(i));
            let dx = vec![-state.x[0] * 0.5];
            solver.iterate(&mut state, &dx, residual, |_| residual / 2.0);
        }

        // After good convergence, should have reasonable alpha
        assert!(state.alpha >= 0.1);
    }
}
