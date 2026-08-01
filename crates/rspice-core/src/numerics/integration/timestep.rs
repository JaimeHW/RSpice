//! Step-size selection and integration-order control.
//!
//! Two controllers that both answer "what should the next step look like":
//! [`TimestepController`] sizes it from the LTE verdict, and
//! [`TrapGearController`] picks the method, dropping trapezoidal for Gear-2
//! when the solution starts ringing.

use crate::Value;
use crate::numerics::integration::IntegrationMethod;

const TRAPGEAR_SIGN_CHANGE_FLOOR: Value = crate::constants::VNTOL;

/// Adaptive timestep controller
#[derive(Debug)]
pub struct TimestepController {
    /// Current timestep
    current_dt: Value,
    /// Hard minimum allowed timestep.
    ///
    /// This is the true convergence-recovery floor. Retry paths may shrink all
    /// the way to this bound before the transient solver gives up on a timepoint.
    hard_min_dt: Value,
    /// Preferred minimum timestep for normal forward stepping.
    ///
    /// This is a soft floor used to help the controller climb back out of
    /// recovery-sized timesteps once LTE indicates the system is smooth again.
    preferred_min_dt: Value,
    /// Maximum allowed timestep
    max_dt: Value,
    /// Target local truncation error
    target_lte: Value,
    /// Previous timestep (for Gear methods)
    prev_dt: Value,
}

impl TimestepController {
    pub fn new(initial_dt: Value, min_dt: Value, max_dt: Value) -> Self {
        Self::new_with_preferred_min(initial_dt, min_dt, min_dt, max_dt)
    }

    pub fn new_with_preferred_min(
        initial_dt: Value,
        hard_min_dt: Value,
        preferred_min_dt: Value,
        max_dt: Value,
    ) -> Self {
        let hard_min_dt = hard_min_dt.max(1e-30).min(max_dt);
        let preferred_min_dt = preferred_min_dt.max(hard_min_dt).min(max_dt);
        Self {
            current_dt: initial_dt.clamp(hard_min_dt, max_dt),
            hard_min_dt,
            preferred_min_dt,
            max_dt,
            target_lte: 1e-3,
            prev_dt: initial_dt.clamp(hard_min_dt, max_dt),
        }
    }

    /// Get current timestep
    pub fn dt(&self) -> Value {
        self.current_dt
    }

    /// Get the hard minimum timestep.
    pub fn hard_min_dt(&self) -> Value {
        self.hard_min_dt
    }

    /// Get the preferred minimum timestep.
    pub fn preferred_min_dt(&self) -> Value {
        self.preferred_min_dt
    }

    /// Update the hard recovery floor while preserving the controller's
    /// current proposal and soft-floor invariants.
    ///
    /// Xyce derives this floor from the current accepted time and machine
    /// precision, so it changes as a transient advances. Dialects with a
    /// fixed floor simply never call this method after construction.
    pub fn set_hard_min_dt(&mut self, hard_min_dt: Value) {
        let hard_min_dt = if hard_min_dt.is_finite() && hard_min_dt >= 0.0 {
            hard_min_dt.max(1e-30).min(self.max_dt)
        } else {
            1e-30_f64.min(self.max_dt)
        };
        self.hard_min_dt = hard_min_dt;
        self.preferred_min_dt = self.preferred_min_dt.max(hard_min_dt).min(self.max_dt);
        self.current_dt = self.current_dt.clamp(hard_min_dt, self.max_dt);
        self.prev_dt = self.prev_dt.clamp(hard_min_dt, self.max_dt);
    }

    /// Adjust timestep based on local truncation error estimate
    pub fn adjust(&mut self, lte_estimate: Value) -> Value {
        // Calculate new timestep using LTE estimate
        // For trapezoidal: LTE ~ O(dt^3), so dt_new = dt * (target_lte / lte_estimate)^(1/3)

        if lte_estimate < 1e-15 {
            // Error too small, increase timestep
            self.prev_dt = self.current_dt;
            self.current_dt = (self.current_dt * 2.0).clamp(self.hard_min_dt, self.max_dt);
        } else {
            let ratio = self.target_lte / lte_estimate;
            let factor = ratio.powf(1.0 / 3.0);

            // Limit growth/shrink rate
            let factor = factor.clamp(0.5, 2.0);

            self.prev_dt = self.current_dt;
            self.current_dt = (self.current_dt * factor).clamp(self.hard_min_dt, self.max_dt);
        }

        if self.current_dt > self.hard_min_dt && self.current_dt < self.preferred_min_dt {
            let grew = self.current_dt > self.prev_dt;
            if grew {
                self.current_dt = self.preferred_min_dt.min(self.max_dt);
            }
        }

        self.current_dt
    }

    /// Force a specific timestep (for breakpoints)
    pub fn force_step(&mut self, dt: Value) {
        self.prev_dt = self.current_dt;
        self.current_dt = dt.clamp(self.hard_min_dt, self.max_dt);
    }

    /// Update the maximum allowed timestep.
    pub fn set_max_dt(&mut self, max_dt: Value) {
        let max_dt = max_dt.max(self.hard_min_dt);
        self.max_dt = max_dt;
        self.preferred_min_dt = self.preferred_min_dt.clamp(self.hard_min_dt, self.max_dt);
        self.current_dt = self.current_dt.clamp(self.hard_min_dt, self.max_dt);
        self.prev_dt = self.prev_dt.clamp(self.hard_min_dt, self.max_dt);
    }

    /// Check if timestep was rejected (too small)
    pub fn is_at_minimum(&self) -> bool {
        self.current_dt <= self.hard_min_dt * 1.001
    }
}

#[cfg(test)]
mod timestep_controller_tests {
    use super::*;

    #[test]
    fn dynamic_hard_minimum_clamps_existing_and_future_proposals() {
        let mut controller =
            TimestepController::new_with_preferred_min(1.0e-20, 1.0e-30, 1.0e-12, 1.0);

        controller.set_hard_min_dt(2.0e-18);
        assert_eq!(controller.hard_min_dt(), 2.0e-18);
        assert_eq!(controller.dt(), 2.0e-18);
        assert!(controller.is_at_minimum());

        controller.set_hard_min_dt(1.0e-19);
        controller.force_step(5.0e-20);
        assert_eq!(controller.dt(), 1.0e-19);
    }
}

/// TrapGear Controller for automatic integration method switching
///
/// This implements the hybrid Trapezoidal/Gear-2 method
/// to suppress numerical oscillation (ringing) at switching transitions.
///
/// # Algorithm
/// - Track solution derivative sign changes for each node
/// - When 3+ consecutive sign changes detected (oscillation), switch to Gear-2
/// - After 2-3 smooth steps without oscillation, switch back to Trapezoidal
/// - At breakpoints (source discontinuities), restart with first-order
///   trapezoidal history; switch to Gear-2 only for detected oscillation.
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
            // The engine restarts trapezoidal history with a backward-Euler
            // companion on breakpoint steps. Forcing Gear-2 here would apply
            // equal-step BDF2 history across a source corner, producing
            // spurious voltage impulses for ideal L/C state.
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
            if curr_sign != self.prev_signs[i] && derivative.abs() > TRAPGEAR_SIGN_CHANGE_FLOOR {
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
}

impl Default for TrapGearController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod trapgear_controller_tests {
    use super::*;

    #[test]
    fn ignores_voltage_tolerance_scale_sign_noise() {
        let mut controller = TrapGearController::new();
        assert!(!controller.update(&[-50.0], 1.0e-6));

        for sample in [
            -50.0 - 0.1 * TRAPGEAR_SIGN_CHANGE_FLOOR,
            -50.0 + 0.1 * TRAPGEAR_SIGN_CHANGE_FLOOR,
            -50.0 - 0.2 * TRAPGEAR_SIGN_CHANGE_FLOOR,
            -50.0 + 0.2 * TRAPGEAR_SIGN_CHANGE_FLOOR,
        ] {
            assert!(!controller.update(&[sample], 1.0e-6));
            assert_eq!(controller.current_method(), IntegrationMethod::Trapezoidal);
        }
    }

    #[test]
    fn breakpoint_restart_does_not_force_gear2() {
        let mut controller = TrapGearController::new();

        controller.set_at_breakpoint(true);

        assert_eq!(controller.current_method(), IntegrationMethod::Trapezoidal);
    }
}
