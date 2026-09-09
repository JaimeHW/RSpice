//! Adaptive Damping for Newton-Raphson Convergence
//!
//! Implements multiple damping strategies to improve Newton-Raphson convergence
//! in circuit simulation. Damping limits the Newton update step to prevent
//! divergence and oscillation.
//!
//! # Strategies
//!
//! 1. **Line Search (Backtracking)**: Find α ∈ (0,1] such that ||F(x + α·δx)|| < ||F(x)||
//! 2. **Voltage Limiting**: Limit junction voltage changes to prevent exponential overflow
//! 3. **Bank-Rose Damping**: Adaptive damping based on convergence rate
//! 4. **Trust Region**: Limit step to a trusted region around current point
//!
//! # Theory
//!
//! Standard Newton update: x_{n+1} = x_n - J^{-1} · F(x_n)
//! Damped Newton update: x_{n+1} = x_n - α · J^{-1} · F(x_n)
//!
//! where α is the damping factor (0 < α ≤ 1).
//!
//! The damping factor is chosen to ensure:
//! 1. ||F(x_{n+1})|| < ||F(x_n)||  (residual decrease)
//! 2. Solution stays physically reasonable (junction voltages, currents)

use crate::Value;

//=============================================================================
// Constants
//=============================================================================

/// Maximum voltage change per Newton step for semiconductor junctions
pub const VMAX_JUNCTION: Value = 0.5; // 500 mV max per iteration
/// Critical voltage threshold for limiting (thermal voltage ~26mV at 300K)
pub const VT_THERMAL: Value = 0.02585;
/// Minimum Bank-Rose damping factor
pub const DAMP_MIN: Value = 0.1;
/// Maximum damping factor (1.0 = no damping)
pub const DAMP_MAX: Value = 1.0;
/// Initial damping factor for Bank-Rose
pub const DAMP_INITIAL: Value = 1.0;
/// Armijo condition parameter (sufficient decrease)
pub const ARMIJO_C1: Value = 1e-4;
/// Maximum line search iterations
pub const LINE_SEARCH_MAX_ITERS: usize = 10;
/// Line search backtrack factor
pub const LINE_SEARCH_FACTOR: Value = 0.5;

//=============================================================================
// Damping Strategy
//=============================================================================

/// Damping strategy enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DampingStrategy {
    /// No damping (pure Newton)
    None,
    /// Simple fixed damping factor
    Fixed,
    /// Backtracking line search
    LineSearch,
    /// Junction voltage limiting
    VoltageLimiting,
    /// Bank-Rose adaptive damping
    BankRose,
    /// Combined: voltage limiting + line search
    #[default]
    Combined,
}

//=============================================================================
// Damping Controller
//=============================================================================

/// Adaptive Damping Controller
///
/// Manages damping for Newton-Raphson iterations to improve convergence.
#[derive(Debug, Clone)]
pub struct DampingController {
    /// Active damping strategy
    strategy: DampingStrategy,

    /// Current damping factor (0 ≤ α ≤ 1); zero means no improving step.
    alpha: Value,

    /// Previous residual norm
    prev_residual: Value,

    /// Bank-Rose damping factor evolution
    bank_rose_alpha: Value,

    /// Number of residual evaluations in the latest line search
    line_search_iters: usize,

    /// Statistics: total line search iterations
    total_line_search_iters: usize,

    /// Statistics: number of limited steps
    limited_steps: usize,

    /// Voltage limiting: maximum junction voltage change
    vmax_junction: Value,

    /// Voltage limiting: thermal voltage
    vt: Value,
}

impl DampingController {
    /// Create a new damping controller with default settings
    pub fn new() -> Self {
        Self::with_strategy(DampingStrategy::Combined)
    }

    /// Create with specific strategy
    pub fn with_strategy(strategy: DampingStrategy) -> Self {
        Self {
            strategy,
            alpha: DAMP_INITIAL,
            prev_residual: f64::INFINITY,
            bank_rose_alpha: DAMP_INITIAL,
            line_search_iters: 0,
            total_line_search_iters: 0,
            limited_steps: 0,
            vmax_junction: VMAX_JUNCTION,
            vt: VT_THERMAL,
        }
    }

    /// Get current damping factor
    #[inline]
    pub fn alpha(&self) -> Value {
        self.alpha
    }

    /// Get current strategy
    #[inline]
    pub fn strategy(&self) -> DampingStrategy {
        self.strategy
    }

    /// Reset for new Newton solve
    pub fn reset(&mut self) {
        self.alpha = DAMP_INITIAL;
        self.prev_residual = f64::INFINITY;
        self.bank_rose_alpha = DAMP_INITIAL;
        self.line_search_iters = 0;
    }

    /// Apply voltage limiting to a Newton update
    ///
    /// Limits junction voltage changes to prevent exponential overflow.
    /// Uses SPICE-style Vold/Vnew limiting.
    ///
    /// # Arguments
    /// * `v_old` - Current junction voltage
    /// * `dv` - Proposed voltage change
    ///
    /// # Returns
    /// Limited voltage change
    pub fn limit_junction_voltage(&mut self, v_old: Value, dv: Value) -> Value {
        // SPICE-style junction voltage limiting
        // Based on the diode equation: I = Is * (exp(V/Vt) - 1)

        let v_new = v_old + dv;

        // For forward bias (large positive voltages)
        if dv > 0.0 && v_new > 0.0 {
            // Limit exponential region changes
            if dv > self.vmax_junction {
                self.limited_steps += 1;
                // Use SPICE-style limiting: vnew = vold + vmax*tanh((dv - vmax)/vmax)
                let excess = dv - self.vmax_junction;
                let limited =
                    self.vmax_junction + self.vmax_junction * (excess / self.vmax_junction).tanh();
                return limited;
            }
        }

        // For reverse bias moving toward forward bias
        if v_old < 0.0 && v_new > 0.5 * self.vt {
            self.limited_steps += 1;
            // Limit jump from deep reverse to forward
            return -v_old + 0.5 * self.vt;
        }

        // For large negative changes
        if dv < -self.vmax_junction {
            self.limited_steps += 1;
            return -self.vmax_junction;
        }

        dv
    }

    /// Compute damping factor using Bank-Rose algorithm
    ///
    /// Adapts damping based on convergence behavior:
    /// - Increase if residual decreased significantly
    /// - Decrease if residual increased
    ///
    /// # Arguments
    /// * `current_residual` - Current residual norm ||F(x)||
    ///
    /// # Returns
    /// Recommended damping factor
    pub fn bank_rose_damping(&mut self, current_residual: Value) -> Value {
        if self.prev_residual.is_infinite() {
            // First iteration
            self.prev_residual = current_residual;
            self.bank_rose_alpha = DAMP_INITIAL;
            self.alpha = DAMP_INITIAL;
            return DAMP_INITIAL;
        }

        let ratio = current_residual / self.prev_residual;

        if ratio < 0.5 {
            // Good decrease - allow larger steps
            self.bank_rose_alpha = (self.bank_rose_alpha * 1.5).min(DAMP_MAX);
        } else if ratio > 1.0 {
            // Residual increased - reduce step
            self.bank_rose_alpha = (self.bank_rose_alpha * 0.5).max(DAMP_MIN);
        } else if ratio > 0.9 {
            // Slow convergence - slightly reduce
            self.bank_rose_alpha = (self.bank_rose_alpha * 0.9).max(DAMP_MIN);
        }

        self.prev_residual = current_residual;
        self.alpha = self.bank_rose_alpha;

        self.bank_rose_alpha
    }

    /// Perform backtracking line search
    ///
    /// Searches for α that satisfies the scaled Armijo decrease:
    /// ||F(x + α·dx)|| ≤ ||F(x)|| - c₁·α·||dx||·||F(x)||
    ///
    /// # Arguments
    /// * `residual_current` - Finite, nonnegative ||F(x)|| at the current point
    /// * `dx_norm` - Finite, nonnegative ||dx|| Newton step norm
    /// * `residual_func` - Function to compute ||F(x + α·dx)||
    ///
    /// # Returns
    /// The first evaluated α satisfying the decrease, or the best strictly
    /// improving evaluated α if the trial budget is exhausted. Returns zero
    /// when no valid trial improves the residual, either input is invalid,
    /// or the residual or step norm is zero. Callers must treat zero as a
    /// failed or stationary search and retain the current iterate.
    /// The Bank-Rose minimum does not constrain an accepted line-search step.
    pub fn line_search<F>(
        &mut self,
        residual_current: Value,
        dx_norm: Value,
        mut residual_func: F,
    ) -> Value
    where
        F: FnMut(Value) -> Value,
    {
        self.alpha = 0.0;
        self.line_search_iters = 0;
        if !residual_current.is_finite()
            || residual_current <= 0.0
            || !dx_norm.is_finite()
            || dx_norm <= 0.0
        {
            return self.alpha;
        }

        let mut alpha = 1.0;
        let mut best_residual = residual_current;
        for _ in 0..LINE_SEARCH_MAX_ITERS {
            self.line_search_iters += 1;
            let residual_new = residual_func(alpha);
            if residual_new.is_finite() && residual_new >= 0.0 {
                if residual_new < best_residual {
                    best_residual = residual_new;
                    self.alpha = alpha;
                }
                // Normalize before comparison so a finite residual need not
                // multiply a large step norm. The trial's alpha belongs in
                // the required decrease, and roundoff must not admit no change.
                let threshold = 1.0 - (ARMIJO_C1 * alpha) * dx_norm;
                if residual_new < residual_current && residual_new / residual_current <= threshold {
                    self.alpha = alpha;
                    break;
                }
            }
            alpha *= LINE_SEARCH_FACTOR;
        }

        self.total_line_search_iters = self
            .total_line_search_iters
            .saturating_add(self.line_search_iters);
        self.alpha
    }

    /// Get statistics
    pub fn statistics(&self) -> DampingStatistics {
        DampingStatistics {
            total_line_search_iters: self.total_line_search_iters,
            limited_steps: self.limited_steps,
            final_alpha: self.alpha,
        }
    }
}

impl Default for DampingController {
    fn default() -> Self {
        Self::new()
    }
}

//=============================================================================
// Statistics
//=============================================================================

/// Damping statistics
#[derive(Debug, Clone, Default)]
pub struct DampingStatistics {
    /// Total residual evaluations across all line searches
    pub total_line_search_iters: usize,
    /// Number of steps where voltage limiting was applied
    pub limited_steps: usize,
    /// Final damping factor used
    pub final_alpha: Value,
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_bank_rose_step_updates_the_reported_factor_after_line_search() {
        let mut controller = DampingController::new();
        controller.line_search(1.0, 1.0, |_| 2.0);
        assert_eq!(controller.alpha(), 0.0);
        let alpha = controller.bank_rose_damping(1.0);
        assert_eq!(alpha, 1.0);
        assert_eq!(controller.alpha(), alpha);
        assert_eq!(controller.statistics().final_alpha, alpha);
    }

    #[test]
    fn line_search_returns_the_evaluated_step_below_the_bank_rose_floor() {
        let residual = |alpha: Value| ((1.0 - 16.0 * alpha) * (1.0 + 10.0 * alpha)).abs();
        let mut controller = DampingController::new();
        let mut trials = Vec::new();
        let alpha = controller.line_search(residual(0.0), 1.0, |alpha| {
            trials.push(alpha);
            residual(alpha)
        });
        assert_eq!(alpha, 0.0625);
        assert!(trials.contains(&alpha));
        assert_eq!(residual(alpha), 0.0);
        assert_eq!(controller.alpha(), alpha);
        assert_eq!(controller.statistics().final_alpha, alpha);
        assert_eq!(
            controller.statistics().total_line_search_iters,
            trials.len()
        );
    }

    #[test]
    fn line_search_scales_the_armijo_decrease_with_the_trial_step() {
        // The second curve has a slightly smaller full-step residual, but
        // only the half step satisfies its alpha-dependent Armijo decrease.
        for (quadratic, linear) in [(0.0005, -0.0004), (0.0001, -0.00016)] {
            let mut controller = DampingController::new();
            let alpha = controller
                .line_search(1.0, 1.0, |alpha| 1.0 + alpha * (quadratic * alpha + linear));
            assert_eq!(alpha, 0.5);
            assert_eq!(controller.statistics().total_line_search_iters, 2);
        }
    }

    #[test]
    fn exhausted_line_search_keeps_the_best_evaluated_decrease() {
        let mut controller = DampingController::new();
        let alpha = controller.line_search(1.0, 1.0, |alpha| 1.0 - 5e-5 * alpha);
        assert_eq!(alpha, 1.0);
        assert_eq!(
            controller.statistics().total_line_search_iters,
            LINE_SEARCH_MAX_ITERS
        );
    }

    #[test]
    fn failed_line_search_returns_no_step_and_counts_every_trial() {
        for residual in [1.0, 2.0, Value::INFINITY, Value::NAN, -1.0] {
            let mut controller = DampingController::new();
            let mut trials = 0;
            assert_eq!(
                controller.line_search(1.0, 1.0, |_| {
                    trials += 1;
                    residual
                }),
                0.0
            );
            assert_eq!(controller.alpha(), 0.0);
            assert_eq!(trials, LINE_SEARCH_MAX_ITERS);
            assert_eq!(controller.statistics().total_line_search_iters, trials);
            controller.reset();
            assert_eq!(controller.alpha(), 1.0);
            assert_eq!(controller.statistics().total_line_search_iters, trials);
        }
    }

    #[test]
    fn invalid_or_stationary_line_search_inputs_do_not_evaluate_trials() {
        for (residual, norm) in [
            (0.0, 1.0),
            (1.0, 0.0),
            (-1.0, 1.0),
            (1.0, -1.0),
            (Value::NAN, 1.0),
            (1.0, Value::NAN),
            (Value::INFINITY, 1.0),
            (1.0, Value::INFINITY),
        ] {
            let mut controller = DampingController::new();
            assert_eq!(
                controller.line_search(residual, norm, |_| panic!("unexpected trial")),
                0.0
            );
            assert_eq!(controller.statistics().total_line_search_iters, 0);
        }
    }
}
