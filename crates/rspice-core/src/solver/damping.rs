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
/// Minimum damping factor
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

    /// Current damping factor (0 < α ≤ 1)
    alpha: Value,

    /// Previous residual norm
    prev_residual: Value,

    /// Bank-Rose damping factor evolution
    bank_rose_alpha: Value,

    /// Number of line search iterations this step
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

    /// Create with custom voltage limits
    pub fn with_voltage_limits(mut self, vmax: Value, vt: Value) -> Self {
        self.vmax_junction = vmax;
        self.vt = vt;
        self
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

    /// Set strategy
    pub fn set_strategy(&mut self, strategy: DampingStrategy) {
        self.strategy = strategy;
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

    /// Apply voltage limiting to a full update vector
    ///
    /// # Arguments
    /// * `x_old` - Current solution vector
    /// * `dx` - Proposed update vector
    /// * `junction_nodes` - Indices of nodes that are junction voltages
    ///
    /// # Returns
    /// Modified update vector with limited junction voltages
    pub fn limit_update_vector(
        &mut self,
        x_old: &[Value],
        dx: &[Value],
        junction_nodes: &[usize],
    ) -> Vec<Value> {
        let mut dx_limited = dx.to_vec();

        for &node in junction_nodes {
            if node < x_old.len() && node < dx_limited.len() {
                dx_limited[node] = self.limit_junction_voltage(x_old[node], dx[node]);
            }
        }

        dx_limited
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
    /// Finds α that satisfies Armijo condition:
    /// ||F(x + α·dx)|| ≤ ||F(x)|| - c₁·α·||dx||·||F(x)||
    ///
    /// # Arguments
    /// * `residual_current` - ||F(x)|| at current point
    /// * `dx_norm` - ||dx|| Newton step norm
    /// * `residual_func` - Function to compute ||F(x + α·dx)||
    ///
    /// # Returns
    /// Optimal α satisfying Armijo condition
    pub fn line_search<F>(
        &mut self,
        residual_current: Value,
        dx_norm: Value,
        mut residual_func: F,
    ) -> Value
    where
        F: FnMut(Value) -> Value,
    {
        let mut alpha = 1.0;
        let mut iters = 0;

        // Armijo threshold
        let threshold = residual_current - ARMIJO_C1 * dx_norm * residual_current;

        while iters < LINE_SEARCH_MAX_ITERS {
            let residual_new = residual_func(alpha);

            if residual_new <= threshold || residual_new < residual_current * 0.99 {
                // Sufficient decrease achieved
                break;
            }

            // Backtrack
            alpha *= LINE_SEARCH_FACTOR;
            iters += 1;
        }

        self.line_search_iters = iters;
        self.total_line_search_iters += iters;
        self.alpha = alpha.max(DAMP_MIN);

        self.alpha
    }

    /// Get optimal damping factor based on current strategy
    ///
    /// This is the main entry point for getting a damping factor.
    ///
    /// # Arguments
    /// * `residual_norm` - Current residual norm
    ///
    /// # Returns
    /// Recommended damping factor
    pub fn get_damping_factor(&mut self, residual_norm: Value) -> Value {
        match self.strategy {
            DampingStrategy::None => 1.0,
            DampingStrategy::Fixed => 0.5,
            DampingStrategy::BankRose | DampingStrategy::Combined => {
                self.bank_rose_damping(residual_norm)
            }
            DampingStrategy::LineSearch => {
                // Line search is handled separately via line_search()
                self.alpha
            }
            DampingStrategy::VoltageLimiting => {
                // Voltage limiting doesn't affect alpha directly
                1.0
            }
        }
    }

    /// Apply damping to Newton update
    #[inline]
    pub fn damp_update(&self, dx: &[Value]) -> Vec<Value> {
        dx.iter().map(|&d| d * self.alpha).collect()
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
    /// Total line search iterations across all Newton steps
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
    fn test_damping_controller_creation() {
        let ctrl = DampingController::new();
        assert_eq!(ctrl.strategy(), DampingStrategy::Combined);
        assert!((ctrl.alpha() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_damping_with_strategy() {
        let ctrl = DampingController::with_strategy(DampingStrategy::LineSearch);
        assert_eq!(ctrl.strategy(), DampingStrategy::LineSearch);
    }

    #[test]
    fn test_junction_voltage_limiting_large_positive() {
        let mut ctrl = DampingController::new();

        // Large positive change should be limited
        let limited = ctrl.limit_junction_voltage(0.5, 1.0);
        assert!(limited < 1.0, "Should limit large positive change");
        assert!(limited > 0.0, "Should still be positive");
    }

    #[test]
    fn test_junction_voltage_limiting_small_change() {
        let mut ctrl = DampingController::new();

        // Small change should not be limited
        let dv = 0.1;
        let limited = ctrl.limit_junction_voltage(0.5, dv);
        assert!(
            (limited - dv).abs() < 1e-10,
            "Small change should not be limited"
        );
    }

    #[test]
    fn test_junction_voltage_limiting_reverse_to_forward() {
        let mut ctrl = DampingController::new();

        // Large jump from reverse to forward should be limited
        let limited = ctrl.limit_junction_voltage(-5.0, 6.0);
        assert!(limited < 6.0, "Reverse to forward jump should be limited");
    }

    #[test]
    fn test_junction_voltage_limiting_large_negative() {
        let mut ctrl = DampingController::new();

        // Large negative change should be limited
        let limited = ctrl.limit_junction_voltage(0.0, -1.0);
        assert!(limited >= -VMAX_JUNCTION);
    }

    #[test]
    fn test_bank_rose_initial() {
        let mut ctrl = DampingController::new();

        let alpha = ctrl.bank_rose_damping(1.0);
        assert!((alpha - DAMP_INITIAL).abs() < 1e-10);
    }

    #[test]
    fn test_bank_rose_good_decrease() {
        let mut ctrl = DampingController::new();

        // First call sets reference
        ctrl.bank_rose_damping(1.0);

        // Good decrease (ratio < 0.5) should increase alpha
        let alpha = ctrl.bank_rose_damping(0.3);
        assert!(alpha >= DAMP_INITIAL);
    }

    #[test]
    fn test_bank_rose_residual_increase() {
        let mut ctrl = DampingController::new();

        // First call
        ctrl.bank_rose_damping(1.0);

        // Residual increased - should decrease alpha
        let alpha = ctrl.bank_rose_damping(1.5);
        assert!(alpha < DAMP_INITIAL);
    }

    #[test]
    fn test_line_search() {
        let mut ctrl = DampingController::new();

        // Simple line search test
        let residual_current = 1.0;
        let dx_norm = 1.0;

        // Residual function that decreases with smaller alpha
        let alpha = ctrl.line_search(residual_current, dx_norm, |a| 1.0 - 0.5 * a + 0.1 * a * a);

        assert!(alpha > 0.0);
        assert!(alpha <= 1.0);
    }

    #[test]
    fn test_line_search_no_improvement() {
        let mut ctrl = DampingController::new();

        // Residual function that never decreases enough
        let alpha = ctrl.line_search(1.0, 1.0, |_a| {
            1.0 // Constant residual
        });

        // Should backtrack to minimum
        assert!(alpha >= DAMP_MIN);
        assert!(ctrl.line_search_iters > 0);
    }

    #[test]
    fn test_damp_update() {
        let mut ctrl = DampingController::new();
        ctrl.alpha = 0.5;

        let dx = vec![1.0, 2.0, 3.0];
        let damped = ctrl.damp_update(&dx);

        assert!((damped[0] - 0.5).abs() < 1e-10);
        assert!((damped[1] - 1.0).abs() < 1e-10);
        assert!((damped[2] - 1.5).abs() < 1e-10);
    }

    #[test]
    fn test_limit_update_vector() {
        let mut ctrl = DampingController::new();

        let x_old = vec![0.5, 0.0, -1.0];
        let dx = vec![1.0, 0.1, 2.0]; // First and third are large
        let junction_nodes = vec![0, 2]; // Limit nodes 0 and 2

        let dx_limited = ctrl.limit_update_vector(&x_old, &dx, &junction_nodes);

        // Non-junction nodes should be unchanged
        assert!((dx_limited[1] - 0.1).abs() < 1e-10);

        // Junction nodes may be limited
        assert!(dx_limited[0] <= dx[0] || dx[0] <= VMAX_JUNCTION);
    }

    #[test]
    fn test_get_damping_factor_none() {
        let mut ctrl = DampingController::with_strategy(DampingStrategy::None);
        let alpha = ctrl.get_damping_factor(1.0);
        assert!((alpha - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_get_damping_factor_fixed() {
        let mut ctrl = DampingController::with_strategy(DampingStrategy::Fixed);
        let alpha = ctrl.get_damping_factor(1.0);
        assert!((alpha - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_reset() {
        let mut ctrl = DampingController::new();
        ctrl.alpha = 0.3;
        ctrl.line_search_iters = 5;

        ctrl.reset();

        assert!((ctrl.alpha() - DAMP_INITIAL).abs() < 1e-10);
        assert_eq!(ctrl.line_search_iters, 0);
    }

    #[test]
    fn test_statistics() {
        let mut ctrl = DampingController::new();
        ctrl.total_line_search_iters = 10;
        ctrl.limited_steps = 3;
        ctrl.alpha = 0.7;

        let stats = ctrl.statistics();

        assert_eq!(stats.total_line_search_iters, 10);
        assert_eq!(stats.limited_steps, 3);
        assert!((stats.final_alpha - 0.7).abs() < 1e-10);
    }

    #[test]
    fn test_with_voltage_limits() {
        let ctrl = DampingController::new().with_voltage_limits(0.3, 0.03);

        assert!((ctrl.vmax_junction - 0.3).abs() < 1e-10);
        assert!((ctrl.vt - 0.03).abs() < 1e-10);
    }

    #[test]
    fn test_damping_strategy_default() {
        assert_eq!(DampingStrategy::default(), DampingStrategy::Combined);
    }

    #[test]
    fn test_set_strategy() {
        let mut ctrl = DampingController::new();
        ctrl.set_strategy(DampingStrategy::BankRose);
        assert_eq!(ctrl.strategy(), DampingStrategy::BankRose);
    }

    #[test]
    fn test_damping_statistics_default() {
        let stats = DampingStatistics::default();
        assert_eq!(stats.total_line_search_iters, 0);
        assert_eq!(stats.limited_steps, 0);
        assert!((stats.final_alpha - 0.0).abs() < 1e-10);
    }
}
