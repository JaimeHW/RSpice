//! Arc-Length Continuation for Difficult DC Convergence
//!
//! Arc-length continuation (also called homotopy or path-following) is a
//! sophisticated technique for solving DC operating points in circuits with
//! multiple solutions or near turning points (folds).
//!
//! # Theory
//!
//! Standard Newton-Raphson fails when the Jacobian becomes singular, which
//! occurs at:
//! - Turning points (folds) in the solution curve
//! - Bifurcation points
//! - Circuits with multiple operating points (bistable circuits)
//!
//! Arc-length continuation adds a constraint equation that parameterizes
//! both the solution variables AND a continuation parameter (e.g., source
//! scaling) by an arc-length parameter `s`:
//!
//! ```text
//!   F(x, λ) = 0              (circuit equations)
//!   N(x, λ, s) = 0           (arc-length constraint)
//! ```
//!
//! where:
//! - `x` = node voltages
//! - `λ` = continuation parameter (0 → 1)  
//! - `s` = arc-length parameter
//!
//! The arc-length constraint prevents step size collapse near singular points.
//!
//! # Algorithm: Pseudo-Arc-Length Continuation
//!
//! 1. **Predictor Step**: Linear extrapolation along tangent to curve
//! 2. **Corrector Step**: Newton iteration orthogonal to predictor direction
//! 3. **Step Size Control**: Adapt based on convergence behavior
//!
//! # References
//!
//! - Keller, H.B., "Numerical Solution of Bifurcation and Nonlinear
//!   Eigenvalue Problems," Applications of Bifurcation Theory, 1977
//! - Kundert, K.S., et al., "A sparse linear equation solver for circuit
//!   simulation," ICCAD, 1986

use crate::Value;

//=============================================================================
// Constants
//=============================================================================

/// Initial arc-length step size
pub const ARC_STEP_INITIAL: Value = 0.1;
/// Minimum arc-length step size
pub const ARC_STEP_MIN: Value = 1e-6;
/// Maximum arc-length step size
pub const ARC_STEP_MAX: Value = 0.5;
/// Arc step growth factor on success
pub const ARC_STEP_GROW: Value = 1.5;
/// Arc step shrink factor on failure
pub const ARC_STEP_SHRINK: Value = 0.5;
/// Maximum continuation iterations
pub const ARC_MAX_ITERATIONS: usize = 500;
/// Target Newton iterations per continuation step
pub const ARC_TARGET_NEWTON_ITERS: usize = 4;
/// Maximum Newton iterations per corrector
pub const ARC_MAX_NEWTON_ITERS: usize = 15;

//=============================================================================
// Arc-Length State
//=============================================================================

/// State of the arc-length continuation solver
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArcLengthState {
    /// Ready to start (λ = 0)
    Ready,
    /// In progress (0 < λ < 1)
    InProgress,
    /// Reached target (λ = 1)
    Complete,
    /// Failed to converge
    Failed,
}

impl Default for ArcLengthState {
    fn default() -> Self {
        Self::Ready
    }
}

//=============================================================================
// Arc-Length Continuation Controller
//=============================================================================

/// Arc-Length Continuation Solver
///
/// Implements pseudo-arc-length continuation for robust DC convergence.
/// This handles circuits with multiple solutions and near-singular Jacobians.
#[derive(Debug, Clone)]
pub struct ArcLengthContinuation {
    /// Current continuation parameter (0 to 1)
    lambda: Value,

    /// Current arc-length step size
    arc_step: Value,

    /// Previous solution vector (for tangent calculation)
    x_prev: Vec<Value>,

    /// Previous continuation parameter
    lambda_prev: Value,

    /// Tangent direction for predictor
    tangent_x: Vec<Value>,
    tangent_lambda: Value,

    /// Number of continuation steps taken
    steps: usize,

    /// Total Newton iterations across all steps
    total_newton_iters: usize,

    /// Current state
    state: ArcLengthState,

    /// Solver configuration
    config: ArcLengthConfig,
}

/// Configuration for arc-length continuation
#[derive(Debug, Clone)]
pub struct ArcLengthConfig {
    /// Initial step size
    pub initial_step: Value,
    /// Minimum step size (give up below this)
    pub min_step: Value,
    /// Maximum step size
    pub max_step: Value,
    /// Growth factor on successful step
    pub grow_factor: Value,
    /// Shrink factor on failed step
    pub shrink_factor: Value,
    /// Maximum total continuation steps
    pub max_steps: usize,
    /// Maximum Newton iterations per corrector
    pub max_newton_iters: usize,
    /// Target Newton iterations for optimal step sizing
    pub target_newton_iters: usize,
    /// Convergence tolerance for corrector
    pub tolerance: Value,
}

impl Default for ArcLengthConfig {
    fn default() -> Self {
        Self {
            initial_step: ARC_STEP_INITIAL,
            min_step: ARC_STEP_MIN,
            max_step: ARC_STEP_MAX,
            grow_factor: ARC_STEP_GROW,
            shrink_factor: ARC_STEP_SHRINK,
            max_steps: ARC_MAX_ITERATIONS,
            max_newton_iters: ARC_MAX_NEWTON_ITERS,
            target_newton_iters: ARC_TARGET_NEWTON_ITERS,
            tolerance: 1e-9,
        }
    }
}

impl ArcLengthContinuation {
    /// Create a new arc-length continuation solver
    pub fn new(num_nodes: usize) -> Self {
        Self::with_config(num_nodes, ArcLengthConfig::default())
    }

    /// Create with custom configuration
    pub fn with_config(num_nodes: usize, config: ArcLengthConfig) -> Self {
        Self {
            lambda: 0.0,
            arc_step: config.initial_step,
            x_prev: vec![0.0; num_nodes],
            lambda_prev: 0.0,
            tangent_x: vec![0.0; num_nodes],
            tangent_lambda: 1.0, // Initially move in λ direction only
            steps: 0,
            total_newton_iters: 0,
            state: ArcLengthState::Ready,
            config,
        }
    }

    /// Get current continuation parameter λ (0 to 1)
    #[inline]
    pub fn lambda(&self) -> Value {
        self.lambda
    }

    /// Get current arc-length step size
    #[inline]
    pub fn arc_step(&self) -> Value {
        self.arc_step
    }

    /// Get current state
    #[inline]
    pub fn state(&self) -> ArcLengthState {
        self.state
    }

    /// Get number of continuation steps taken
    #[inline]
    pub fn steps(&self) -> usize {
        self.steps
    }

    /// Get total Newton iterations
    #[inline]
    pub fn total_newton_iters(&self) -> usize {
        self.total_newton_iters
    }

    /// Check if continuation is complete (λ reached 1)
    #[inline]
    pub fn is_complete(&self) -> bool {
        self.state == ArcLengthState::Complete
    }

    /// Check if continuation failed
    #[inline]
    pub fn is_failed(&self) -> bool {
        self.state == ArcLengthState::Failed
    }

    /// Initialize with starting solution at λ = 0
    ///
    /// # Arguments
    /// * `x0` - Initial solution vector (typically zero or previous DC solution)
    pub fn initialize(&mut self, x0: &[Value]) {
        self.lambda = 0.0;
        self.arc_step = self.config.initial_step;
        self.x_prev.clear();
        self.x_prev.extend_from_slice(x0);
        self.lambda_prev = 0.0;
        self.tangent_x = vec![0.0; x0.len()];
        self.tangent_lambda = 1.0;
        self.steps = 0;
        self.total_newton_iters = 0;
        self.state = ArcLengthState::Ready;
    }

    /// Compute predictor step: linear extrapolation along tangent
    ///
    /// Returns the predicted (x, λ) for the next point on the curve.
    ///
    /// # Arguments
    /// * `x_current` - Current solution vector
    ///
    /// # Returns
    /// * `(x_predicted, lambda_predicted)` - Predicted next point
    pub fn predict(&mut self, x_current: &[Value]) -> (Vec<Value>, Value) {
        // Save current as previous
        self.x_prev.clear();
        self.x_prev.extend_from_slice(x_current);
        self.lambda_prev = self.lambda;

        // Normalize tangent
        let norm = self.tangent_norm();
        if norm < 1e-15 {
            // First step: move purely in λ direction
            self.tangent_lambda = 1.0;
            for t in &mut self.tangent_x {
                *t = 0.0;
            }
        }

        let norm = self.tangent_norm().max(1e-15);

        // Predicted point = current + arc_step * tangent / ||tangent||
        let scale = self.arc_step / norm;

        let x_pred: Vec<Value> = x_current
            .iter()
            .zip(self.tangent_x.iter())
            .map(|(&x, &t)| x + scale * t)
            .collect();

        let lambda_pred = self.lambda + scale * self.tangent_lambda;

        (x_pred, lambda_pred.clamp(0.0, 1.0))
    }

    /// Compute the arc-length constraint residual
    ///
    /// The constraint is: ||x - x_prev||² + (λ - λ_prev)² - Δs² = 0
    ///
    /// Using a normalized form orthogonal to the tangent for better conditioning.
    pub fn constraint_residual(&self, x: &[Value], lambda: Value) -> Value {
        // Arc-length constraint using tangent projection
        // N = (x - x_prev) · tangent_x + (λ - λ_prev) · tangent_λ - Δs = 0
        let mut proj = 0.0;

        for ((&xi, &xi_prev), &ti) in x.iter().zip(self.x_prev.iter()).zip(self.tangent_x.iter()) {
            proj += (xi - xi_prev) * ti;
        }
        proj += (lambda - self.lambda_prev) * self.tangent_lambda;

        proj - self.arc_step
    }

    /// Compute constraint Jacobian entries
    ///
    /// Returns (dN/dx, dN/dλ) for use in augmented Newton system
    pub fn constraint_jacobian(&self) -> (&[Value], Value) {
        // dN/dx_i = tangent_x_i
        // dN/dλ = tangent_λ
        (&self.tangent_x, self.tangent_lambda)
    }

    /// Update tangent direction from converged corrector
    ///
    /// The tangent is computed from the solution increment normalized
    /// to point in the direction of increasing λ when possible.
    ///
    /// # Arguments
    /// * `x` - Converged solution vector
    /// * `lambda` - Converged λ value
    pub fn update_tangent(&mut self, x: &[Value], lambda: Value) {
        // Compute secant direction
        for (i, (&xi, &xi_prev)) in x.iter().zip(self.x_prev.iter()).enumerate() {
            if i < self.tangent_x.len() {
                self.tangent_x[i] = xi - xi_prev;
            }
        }
        self.tangent_lambda = lambda - self.lambda_prev;

        // Ensure tangent points forward (λ increasing for most cases)
        if self.tangent_lambda < 0.0 && lambda < 0.99 {
            // We're going backward - might be approaching a turning point
            // Keep direction but note this
        }

        // Normalize
        let norm = self.tangent_norm();
        if norm > 1e-15 {
            for t in &mut self.tangent_x {
                *t /= norm;
            }
            self.tangent_lambda /= norm;
        }
    }

    /// Called when corrector converged successfully
    ///
    /// # Arguments
    /// * `x` - Converged solution
    /// * `lambda` - Converged continuation parameter
    /// * `newton_iters` - Number of Newton iterations used
    pub fn accept_step(&mut self, x: &[Value], lambda: Value, newton_iters: usize) {
        // Update tangent
        self.update_tangent(x, lambda);

        // Update state
        self.lambda = lambda;
        self.steps += 1;
        self.total_newton_iters += newton_iters;

        // Check if we've reached the target
        if lambda >= 1.0 - 1e-10 {
            self.lambda = 1.0;
            self.state = ArcLengthState::Complete;
            return;
        }

        self.state = ArcLengthState::InProgress;

        // Adapt step size based on Newton convergence
        // Fewer iterations = can increase step, more = decrease
        if newton_iters <= self.config.target_newton_iters {
            self.arc_step = (self.arc_step * self.config.grow_factor).min(self.config.max_step);
        } else if newton_iters > self.config.target_newton_iters * 2 {
            self.arc_step = (self.arc_step * self.config.shrink_factor).max(self.config.min_step);
        }

        // Ensure we don't overshoot λ = 1
        if self.lambda + self.arc_step * self.tangent_lambda.abs() > 1.2 {
            // Approaching target, reduce step to hit exactly
            let remaining = 1.0 - self.lambda;
            if self.tangent_lambda.abs() > 1e-10 {
                self.arc_step = (remaining / self.tangent_lambda.abs()).min(self.arc_step);
            }
        }
    }

    /// Called when corrector failed to converge
    ///
    /// Reduces step size and allows retry. Returns false if step is too small.
    pub fn reject_step(&mut self) -> bool {
        self.arc_step *= self.config.shrink_factor;

        if self.arc_step < self.config.min_step {
            self.state = ArcLengthState::Failed;
            return false;
        }

        if self.steps >= self.config.max_steps {
            self.state = ArcLengthState::Failed;
            return false;
        }

        true
    }

    /// Reset the solver for a new continuation
    pub fn reset(&mut self, num_nodes: usize) {
        self.lambda = 0.0;
        self.arc_step = self.config.initial_step;
        self.x_prev = vec![0.0; num_nodes];
        self.lambda_prev = 0.0;
        self.tangent_x = vec![0.0; num_nodes];
        self.tangent_lambda = 1.0;
        self.steps = 0;
        self.total_newton_iters = 0;
        self.state = ArcLengthState::Ready;
    }

    /// Compute norm of tangent vector
    fn tangent_norm(&self) -> Value {
        let sum_sq: Value = self.tangent_x.iter().map(|&t| t * t).sum();
        (sum_sq + self.tangent_lambda * self.tangent_lambda).sqrt()
    }

    /// Scale a source value by current continuation parameter
    #[inline]
    pub fn scale_source(&self, value: Value) -> Value {
        value * self.lambda
    }
}

//=============================================================================
// Arc-Length Continuation Result
//=============================================================================

/// Result of arc-length continuation
#[derive(Debug, Clone)]
pub struct ArcLengthResult {
    /// Final continuation parameter (should be 1.0 for success)
    pub final_lambda: Value,
    /// Number of continuation steps
    pub continuation_steps: usize,
    /// Total Newton iterations
    pub total_newton_iters: usize,
    /// Whether continuation succeeded
    pub success: bool,
    /// Minimum step size reached
    pub min_step_reached: Value,
    /// Solution path (x, λ) pairs if tracking enabled
    pub path: Option<Vec<(Vec<Value>, Value)>>,
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arc_length_creation() {
        let arc = ArcLengthContinuation::new(5);
        assert_eq!(arc.lambda(), 0.0);
        assert_eq!(arc.state(), ArcLengthState::Ready);
        assert_eq!(arc.steps(), 0);
    }

    #[test]
    fn test_arc_length_initialize() {
        let mut arc = ArcLengthContinuation::new(3);
        let x0 = vec![0.0, 1.0, 2.0];

        arc.initialize(&x0);

        assert_eq!(arc.lambda(), 0.0);
        assert_eq!(arc.x_prev, x0);
        assert_eq!(arc.state(), ArcLengthState::Ready);
    }

    #[test]
    fn test_arc_length_predict() {
        let mut arc = ArcLengthContinuation::new(3);
        let x0 = vec![1.0, 2.0, 3.0];
        arc.initialize(&x0);

        let (x_pred, lambda_pred) = arc.predict(&x0);

        // Should predict in λ direction initially
        assert!(lambda_pred > 0.0);
        assert!(lambda_pred <= 1.0);

        // x should not change much on first step
        // (tangent_x is zero initially)
        for (xp, x0i) in x_pred.iter().zip(x0.iter()) {
            assert!((xp - x0i).abs() < 0.01);
        }
    }

    #[test]
    fn test_arc_length_accept_step() {
        let mut arc = ArcLengthContinuation::new(2);
        let x0 = vec![0.0, 0.0];
        arc.initialize(&x0);

        let (x_pred, lambda_pred) = arc.predict(&x0);

        // Simulate successful corrector
        arc.accept_step(&x_pred, lambda_pred, 3);

        assert!(arc.lambda() > 0.0);
        assert_eq!(arc.steps(), 1);
        assert_eq!(arc.state(), ArcLengthState::InProgress);
    }

    #[test]
    fn test_arc_length_complete() {
        let mut arc = ArcLengthContinuation::new(2);
        let x = vec![1.0, 2.0];
        arc.initialize(&x);
        arc.lambda = 0.99;

        // Accept step that reaches λ = 1
        arc.accept_step(&x, 1.0, 3);

        assert!(arc.is_complete());
        assert_eq!(arc.lambda(), 1.0);
    }

    #[test]
    fn test_arc_length_reject() {
        let mut arc = ArcLengthContinuation::new(2);
        arc.initialize(&[0.0, 0.0]);

        let initial_step = arc.arc_step();

        // Reject step
        let can_continue = arc.reject_step();

        assert!(can_continue);
        assert!(arc.arc_step() < initial_step);
    }

    #[test]
    fn test_arc_length_min_step_failure() {
        let config = ArcLengthConfig {
            min_step: 0.01,
            initial_step: 0.02,
            shrink_factor: 0.5,
            ..Default::default()
        };

        let mut arc = ArcLengthContinuation::with_config(2, config);
        arc.initialize(&[0.0, 0.0]);

        // Multiple rejections should fail at min step
        arc.reject_step();
        let can_continue = arc.reject_step();

        assert!(!can_continue);
        assert!(arc.is_failed());
    }

    #[test]
    fn test_constraint_residual() {
        let mut arc = ArcLengthContinuation::new(2);
        arc.initialize(&[0.0, 0.0]);

        // At starting point, residual should equal -arc_step
        // (since we haven't moved yet)
        let residual = arc.constraint_residual(&[0.0, 0.0], 0.0);
        assert!((residual + arc.arc_step()).abs() < 1e-10);
    }

    #[test]
    fn test_tangent_update() {
        let mut arc = ArcLengthContinuation::new(2);
        let x0 = vec![0.0, 0.0];
        arc.initialize(&x0);
        arc.lambda = 0.0;

        // Move to a new point
        let x1 = vec![1.0, 1.0];
        arc.update_tangent(&x1, 0.5);

        // Tangent should point toward the new point
        let norm = arc.tangent_norm();
        assert!(norm > 0.0);
        assert!((norm - 1.0).abs() < 1e-10); // Should be normalized
    }

    #[test]
    fn test_scale_source() {
        let mut arc = ArcLengthContinuation::new(2);
        arc.initialize(&[0.0, 0.0]);

        // At λ = 0, should scale to 0
        assert_eq!(arc.scale_source(5.0), 0.0);

        // Move to λ = 0.5
        arc.lambda = 0.5;
        assert!((arc.scale_source(10.0) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_step_size_adaptation() {
        let mut arc = ArcLengthContinuation::new(2);
        arc.initialize(&[0.0, 0.0]);

        let initial_step = arc.arc_step();

        // Good convergence (few iterations) should grow step
        arc.accept_step(&[0.1, 0.1], 0.1, 2);
        let step_after_good = arc.arc_step();
        assert!(step_after_good > initial_step);

        // Poor convergence (many iterations) should shrink step
        arc.accept_step(&[0.2, 0.2], 0.2, 12);
        let step_after_poor = arc.arc_step();
        assert!(step_after_poor < step_after_good);
    }

    #[test]
    fn test_reset() {
        let mut arc = ArcLengthContinuation::new(3);
        arc.initialize(&[0.0, 0.0, 0.0]);
        arc.accept_step(&[1.0, 1.0, 1.0], 0.5, 4);

        arc.reset(3);

        assert_eq!(arc.lambda(), 0.0);
        assert_eq!(arc.steps(), 0);
        assert_eq!(arc.state(), ArcLengthState::Ready);
    }

    #[test]
    fn test_constraint_jacobian() {
        let mut arc = ArcLengthContinuation::new(2);
        arc.initialize(&[0.0, 0.0]);
        arc.tangent_x = vec![0.6, 0.8];
        arc.tangent_lambda = 0.0;

        let (dx, dlambda) = arc.constraint_jacobian();

        assert_eq!(dx.len(), 2);
        assert!((dx[0] - 0.6).abs() < 1e-10);
        assert!((dx[1] - 0.8).abs() < 1e-10);
        assert!((dlambda - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_approaching_target() {
        let mut arc = ArcLengthContinuation::new(2);
        arc.initialize(&[0.0, 0.0]);
        arc.lambda = 0.95;
        arc.tangent_lambda = 1.0;
        arc.arc_step = 0.5;

        // Accept step near target
        arc.accept_step(&[0.0, 0.0], 0.98, 3);

        // Step should be reduced to not overshoot
        assert!(arc.arc_step() < 0.5);
    }

    #[test]
    fn test_arc_length_config_default() {
        let config = ArcLengthConfig::default();

        assert!(config.initial_step > 0.0);
        assert!(config.min_step < config.max_step);
        assert!(config.grow_factor > 1.0);
        assert!(config.shrink_factor < 1.0);
    }

    #[test]
    fn test_arc_length_state_default() {
        assert_eq!(ArcLengthState::default(), ArcLengthState::Ready);
    }

    #[test]
    fn test_full_continuation_simulation() {
        let mut arc = ArcLengthContinuation::new(2);
        arc.initialize(&[0.0, 0.0]);

        // Simulate a simple continuation from λ=0 to λ=1
        let mut x = vec![0.0, 0.0];

        for _ in 0..100 {
            if arc.is_complete() {
                break;
            }

            let (x_pred, lambda_pred) = arc.predict(&x);

            // Simulate successful corrector
            // In real use, Newton iteration would modify x_pred
            x = x_pred;
            arc.accept_step(&x, lambda_pred, 4);
        }

        assert!(
            arc.is_complete() || arc.lambda() > 0.9,
            "Should reach near target, got λ = {}",
            arc.lambda()
        );
    }
}
