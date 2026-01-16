//! Transient Time-Domain Analysis

use crate::Value;
use super::AnalysisConfig;

/// Transient Analysis engine
#[derive(Debug)]
#[allow(dead_code)] // Reserved for full integration
pub struct TransientAnalysis {
    config: AnalysisConfig,
    /// Simulation stop time
    stop_time: Value,
    /// Maximum time step
    max_step: Value,
    /// Initial time step
    initial_step: Value,
    /// Integration method
    method: IntegrationMethod,
}

/// Numerical integration methods for transient analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationMethod {
    /// Backward Euler (first order, very stable)
    BackwardEuler,
    /// Trapezoidal rule (second order, A-stable)
    Trapezoidal,
    /// Gear order 2 (BDF2, good for stiff systems)
    Gear2,
    /// TrapGear: Hybrid method that auto-switches between Trapezoidal and Gear2
    /// Uses Trapezoidal for smooth regions (better accuracy) and
    /// switches to Gear2 at discontinuities/oscillations (better stability)
    TrapGear,
}

impl TransientAnalysis {
    pub fn new(stop_time: Value, max_step: Value) -> Self {
        Self {
            config: AnalysisConfig::default(),
            stop_time,
            max_step,
            initial_step: max_step / 100.0,
            method: IntegrationMethod::Trapezoidal,
        }
    }

    /// Set integration method
    pub fn with_method(mut self, method: IntegrationMethod) -> Self {
        self.method = method;
        self
    }

    /// Set initial timestep
    pub fn with_initial_step(mut self, step: Value) -> Self {
        self.initial_step = step;
        self
    }

    /// Get stop time
    pub fn stop_time(&self) -> Value {
        self.stop_time
    }

    /// Get maximum step
    pub fn max_step(&self) -> Value {
        self.max_step
    }

    /// Get integration method
    pub fn method(&self) -> IntegrationMethod {
        self.method
    }
}

impl Default for TransientAnalysis {
    fn default() -> Self {
        Self::new(1e-3, 1e-6) // 1ms simulation, 1us max step
    }
}

/// Adaptive timestep controller
#[derive(Debug)]
pub struct TimestepController {
    /// Current timestep
    current_dt: Value,
    /// Minimum allowed timestep
    min_dt: Value,
    /// Maximum allowed timestep
    max_dt: Value,
    /// Target local truncation error
    target_lte: Value,
    /// Previous timestep (for Gear methods)
    prev_dt: Value,
}

impl TimestepController {
    pub fn new(initial_dt: Value, min_dt: Value, max_dt: Value) -> Self {
        Self {
            current_dt: initial_dt,
            min_dt,
            max_dt,
            target_lte: 1e-3,
            prev_dt: initial_dt,
        }
    }

    /// Get current timestep
    pub fn dt(&self) -> Value {
        self.current_dt
    }

    /// Adjust timestep based on local truncation error estimate
    pub fn adjust(&mut self, lte_estimate: Value) -> Value {
        // Calculate new timestep using LTE estimate
        // For trapezoidal: LTE ~ O(dt^3), so dt_new = dt * (target_lte / lte_estimate)^(1/3)
        
        if lte_estimate < 1e-15 {
            // Error too small, increase timestep
            self.prev_dt = self.current_dt;
            self.current_dt = (self.current_dt * 2.0).min(self.max_dt);
        } else {
            let ratio = self.target_lte / lte_estimate;
            let factor = ratio.powf(1.0 / 3.0);
            
            // Limit growth/shrink rate
            let factor = factor.clamp(0.5, 2.0);
            
            self.prev_dt = self.current_dt;
            self.current_dt = (self.current_dt * factor).clamp(self.min_dt, self.max_dt);
        }
        
        self.current_dt
    }

    /// Force a specific timestep (for breakpoints)
    pub fn force_step(&mut self, dt: Value) {
        self.prev_dt = self.current_dt;
        self.current_dt = dt.clamp(self.min_dt, self.max_dt);
    }

    /// Check if timestep was rejected (too small)
    pub fn is_at_minimum(&self) -> bool {
        self.current_dt <= self.min_dt * 1.001
    }
}

/// Breakpoint manager for handling discontinuities
#[derive(Debug, Default)]
#[allow(dead_code)] // Reserved for breakpoint tracking
pub struct BreakpointManager {
    /// Sorted list of breakpoint times
    breakpoints: Vec<Value>,
    /// Current index
    current_index: usize,
}

impl BreakpointManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a breakpoint time
    pub fn add(&mut self, time: Value) {
        // Insert in sorted order
        let pos = self.breakpoints.iter().position(|&t| t > time);
        match pos {
            Some(i) => self.breakpoints.insert(i, time),
            None => self.breakpoints.push(time),
        }
    }

    /// Add breakpoints from a periodic source
    pub fn add_periodic(&mut self, start: Value, period: Value, end: Value) {
        let mut t = start;
        while t <= end {
            self.add(t);
            t += period;
        }
    }

    /// Get next breakpoint after given time
    pub fn next_after(&self, time: Value) -> Option<Value> {
        self.breakpoints.iter().copied().find(|&t| t > time + 1e-15)
    }

    /// Limit timestep to not cross next breakpoint
    pub fn limit_step(&self, current_time: Value, proposed_dt: Value) -> Value {
        match self.next_after(current_time) {
            Some(bp) => {
                let time_to_bp = bp - current_time;
                if proposed_dt > time_to_bp * 0.9 {
                    time_to_bp
                } else {
                    proposed_dt
                }
            }
            None => proposed_dt,
        }
    }
}

/// Local Truncation Error (LTE) estimator for adaptive timestep
/// 
/// Uses difference between predicted and calculated values to estimate
/// the integration error. This allows rejecting timesteps that converge
/// numerically but are physically inaccurate.
#[derive(Debug)]
pub struct LteEstimator {
    /// Previous solution vector (t - dt)
    prev_solution: Vec<Value>,
    /// Solution before previous (t - 2*dt)
    prev_prev_solution: Vec<Value>,
    /// Previous timestep
    prev_dt: Value,
    /// LTE tolerance
    tolerance: Value,
    /// Number of valid history entries
    history_count: usize,
}

impl LteEstimator {
    /// Create a new LTE estimator with given tolerance
    pub fn new(tolerance: Value) -> Self {
        Self {
            prev_solution: Vec::new(),
            prev_prev_solution: Vec::new(),
            prev_dt: 0.0,
            tolerance,
            history_count: 0,
        }
    }

    /// Record a solution point for history
    pub fn record(&mut self, solution: &[Value], dt: Value) {
        self.prev_prev_solution = std::mem::take(&mut self.prev_solution);
        self.prev_solution = solution.to_vec();
        self.prev_dt = dt;
        if self.history_count < 2 {
            self.history_count += 1;
        }
    }

    /// Estimate LTE using linear extrapolation vs actual value
    /// Returns (lte_estimate, should_accept)
    pub fn estimate(&self, current: &[Value], dt: Value) -> (Value, bool) {
        // Need at least one previous point to estimate
        if self.history_count < 1 || self.prev_solution.len() != current.len() {
            return (0.0, true); // Accept, no history yet
        }

        let mut max_lte = 0.0_f64;

        // For trapezoidal, LTE ~ (dt^3 / 12) * d^3v/dt^3
        // We approximate by comparing predicted (linear extrapolation) vs actual
        for (i, &curr_val) in current.iter().enumerate() {
            let prev_val = self.prev_solution[i];
            
            // Linear prediction: v_pred = v_prev + (v_prev - v_prev_prev) * dt / prev_dt
            let predicted = if self.history_count >= 2 && self.prev_dt > 0.0 {
                let prev_prev_val = self.prev_prev_solution[i];
                let slope = (prev_val - prev_prev_val) / self.prev_dt;
                prev_val + slope * dt
            } else {
                prev_val // No second derivative, predict same value
            };

            // LTE estimate: |actual - predicted|
            let lte = (curr_val - predicted).abs();
            
            // Normalize by value magnitude to avoid small absolute errors on small values
            let normalized_lte = if curr_val.abs() > 1e-12 {
                lte / curr_val.abs().max(1.0)
            } else {
                lte
            };
            
            max_lte = max_lte.max(normalized_lte);
        }

        let accept = max_lte <= self.tolerance;
        (max_lte, accept)
    }

    /// Get recommended timestep scaling factor based on LTE
    pub fn recommend_scale(&self, lte: Value) -> Value {
        if lte < 1e-15 {
            2.0 // Error negligible, can increase
        } else {
            // For trapezoidal (order 2), optimal scaling is (tol/lte)^(1/3)
            let ratio = self.tolerance / lte;
            ratio.powf(1.0 / 3.0).clamp(0.25, 2.0)
        }
    }

    /// Reset history (e.g., after discontinuity)
    pub fn reset(&mut self) {
        self.prev_solution.clear();
        self.prev_prev_solution.clear();
        self.history_count = 0;
    }
}

/// TrapGear Controller for automatic integration method switching
/// 
/// This implements the hybrid Trapezoidal/Gear-2 method used by LTspice
/// to suppress numerical oscillation (ringing) at switching transitions.
/// 
/// # Algorithm
/// - Track solution derivative sign changes for each node
/// - When 3+ consecutive sign changes detected (oscillation), switch to Gear-2
/// - After 2-3 smooth steps without oscillation, switch back to Trapezoidal
/// - At breakpoints (source discontinuities), preemptively use Gear-2
#[derive(Debug)]
pub struct TrapGearController {
    /// Current effective method being used
    current_method: IntegrationMethod,
    /// Previous solution values for derivative calculation
    prev_values: Vec<Value>,
    /// Previous derivative signs (true = positive, false = negative)
    prev_signs: Vec<bool>,
    /// Count of consecutive sign changes per node
    sign_change_count: Vec<usize>,
    /// Steps since last oscillation detected
    smooth_steps: usize,
    /// Whether we're at or near a breakpoint
    at_breakpoint: bool,
    /// Threshold for consecutive sign changes to trigger Gear switch
    oscillation_threshold: usize,
    /// Steps of smooth behavior before returning to Trapezoidal
    recovery_steps: usize,
}

impl TrapGearController {
    /// Create a new TrapGear controller
    pub fn new() -> Self {
        Self {
            current_method: IntegrationMethod::Trapezoidal,
            prev_values: Vec::new(),
            prev_signs: Vec::new(),
            sign_change_count: Vec::new(),
            smooth_steps: 0,
            at_breakpoint: false,
            oscillation_threshold: 3,
            recovery_steps: 2,
        }
    }

    /// Get the current effective integration method
    #[inline]
    pub fn current_method(&self) -> IntegrationMethod {
        self.current_method
    }

    /// Signal that we're approaching or at a breakpoint
    pub fn set_at_breakpoint(&mut self, at_bp: bool) {
        self.at_breakpoint = at_bp;
        if at_bp {
            // Preemptively switch to Gear-2 at breakpoints
            self.current_method = IntegrationMethod::Gear2;
            self.smooth_steps = 0;
        }
    }

    /// Update the controller with new solution values
    /// Returns true if oscillation was detected
    pub fn update(&mut self, solution: &[Value], _dt: Value) -> bool {
        // Initialize on first call
        if self.prev_values.len() != solution.len() {
            self.prev_values = solution.to_vec();
            self.prev_signs = vec![true; solution.len()];
            self.sign_change_count = vec![0; solution.len()];
            return false;
        }

        let mut oscillation_detected = false;
        let mut max_sign_changes = 0;
        let mut any_sign_change = false;

        for (i, &curr) in solution.iter().enumerate() {
            let prev = self.prev_values[i];
            let derivative = curr - prev;
            let curr_sign = derivative >= 0.0;
            
            // Check for sign change in derivative
            if curr_sign != self.prev_signs[i] && derivative.abs() > 1e-12 {
                self.sign_change_count[i] += 1;
                any_sign_change = true;
            } else {
                // Reset count on smooth behavior
                self.sign_change_count[i] = 0;
            }
            
            max_sign_changes = max_sign_changes.max(self.sign_change_count[i]);
            self.prev_signs[i] = curr_sign;
            self.prev_values[i] = curr;
        }

        // Detect oscillation if any node has too many sign changes
        if max_sign_changes >= self.oscillation_threshold {
            oscillation_detected = true;
            self.smooth_steps = 0;
            self.current_method = IntegrationMethod::Gear2;
            
            // Reset sign change counts after detecting oscillation
            for count in &mut self.sign_change_count {
                *count = 0;
            }
        } else if any_sign_change {
            // Still oscillating but below threshold - stay in current method, reset smooth counter
            self.smooth_steps = 0;
        } else {
            // Truly smooth - no sign changes
            self.smooth_steps += 1;
            
            // Return to Trapezoidal after sufficient smooth steps
            if self.smooth_steps >= self.recovery_steps && !self.at_breakpoint {
                self.current_method = IntegrationMethod::Trapezoidal;
            }
        }

        oscillation_detected
    }

    /// Force switch to a specific method (for debugging or manual control)
    pub fn force_method(&mut self, method: IntegrationMethod) {
        self.current_method = method;
    }

    /// Reset the controller state
    pub fn reset(&mut self) {
        self.prev_values.clear();
        self.prev_signs.clear();
        self.sign_change_count.clear();
        self.smooth_steps = 0;
        self.at_breakpoint = false;
        self.current_method = IntegrationMethod::Trapezoidal;
    }

    /// Get statistics for debugging
    pub fn stats(&self) -> TrapGearStats {
        TrapGearStats {
            current_method: self.current_method,
            smooth_steps: self.smooth_steps,
            max_sign_changes: self.sign_change_count.iter().copied().max().unwrap_or(0),
        }
    }
}

impl Default for TrapGearController {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics from TrapGear controller for debugging/monitoring
#[derive(Debug, Clone)]
pub struct TrapGearStats {
    pub current_method: IntegrationMethod,
    pub smooth_steps: usize,
    pub max_sign_changes: usize,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestep_controller() {
        let mut ctrl = TimestepController::new(1e-6, 1e-12, 1e-3);
        
        assert_eq!(ctrl.dt(), 1e-6);
        
        // Large error - should decrease step
        ctrl.adjust(1e-1);
        assert!(ctrl.dt() < 1e-6);
        
        // Small error - should increase step
        ctrl.adjust(1e-20);
        assert!(ctrl.dt() > 1e-12);
    }

    #[test]
    fn test_breakpoint_manager() {
        let mut mgr = BreakpointManager::new();
        mgr.add(1e-3);
        mgr.add(5e-4);
        mgr.add(2e-3);
        
        assert_eq!(mgr.next_after(0.0), Some(5e-4));
        assert_eq!(mgr.next_after(6e-4), Some(1e-3));
    }

    #[test]
    fn test_trapgear_smooth_signal() {
        let mut trapgear = TrapGearController::new();
        
        // Smooth monotonic signal should stay in Trapezoidal
        for i in 0..10 {
            let value = i as f64 * 0.1;
            trapgear.update(&[value], 1e-6);
        }
        
        assert_eq!(trapgear.current_method(), IntegrationMethod::Trapezoidal);
    }

    #[test]
    fn test_trapgear_oscillation_detection() {
        let mut trapgear = TrapGearController::new();
        
        // Oscillating signal: up, down, up, down...
        let oscillating = [0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0];
        
        for &value in &oscillating {
            trapgear.update(&[value], 1e-6);
        }
        
        // Should have switched to Gear2 after detecting oscillation
        assert_eq!(trapgear.current_method(), IntegrationMethod::Gear2);
    }

    #[test]
    fn test_trapgear_recovery() {
        let mut trapgear = TrapGearController::new();
        
        // First, cause oscillation
        for &value in &[0.0, 1.0, 0.0, 1.0, 0.0, 1.0] {
            trapgear.update(&[value], 1e-6);
        }
        assert_eq!(trapgear.current_method(), IntegrationMethod::Gear2);
        
        // Now, smooth signal - should recover to Trapezoidal
        for i in 0..5 {
            trapgear.update(&[i as f64 * 0.1], 1e-6);
        }
        assert_eq!(trapgear.current_method(), IntegrationMethod::Trapezoidal);
    }

    #[test]
    fn test_trapgear_breakpoint() {
        let mut trapgear = TrapGearController::new();
        
        // At breakpoint, should switch to Gear2
        trapgear.set_at_breakpoint(true);
        assert_eq!(trapgear.current_method(), IntegrationMethod::Gear2);
        
        // Clear breakpoint and smooth signal, should return to Trapezoidal
        trapgear.set_at_breakpoint(false);
        for i in 0..5 {
            trapgear.update(&[i as f64], 1e-6);
        }
        assert_eq!(trapgear.current_method(), IntegrationMethod::Trapezoidal);
    }
}

