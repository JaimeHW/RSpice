//! Step-size selection and integration-order control.
//!
//! Two controllers that both answer "what should the next step look like":
//! [`TimestepController`] sizes it from the LTE verdict, and
//! [`TrapGearController`] picks the method, dropping trapezoidal for Gear-2
//! when the solution starts ringing.

use crate::Value;
use crate::numerics::integration::IntegrationMethod;

const TRAPGEAR_SIGN_CHANGE_FLOOR: Value = crate::constants::VNTOL;
const TRAPGEAR_OSCILLATION_THRESHOLD: usize = 3;
const TRAPGEAR_RECOVERY_STEPS: usize = 2;

/// Xyce transient-step acceptance policy selected by `TIMEINT ERROPTION`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TransientErrorControl {
    /// `ERROPTION=0`: local-truncation error controls acceptance and resizing.
    #[default]
    LocalTruncation,
    /// `ERROPTION=1`: nonlinear iteration counts control resizing and LTE does
    /// not reject an otherwise converged step.
    NonlinearIterations,
}

impl TransientErrorControl {
    /// Decode Xyce's exact integer selector domain.
    pub const fn from_xyce_selector(selector: usize) -> Option<Self> {
        match selector {
            0 => Some(Self::LocalTruncation),
            1 => Some(Self::NonlinearIterations),
            _ => None,
        }
    }

    /// Return the Xyce integer spelling used by checkpoint identities and
    /// diagnostics.
    pub const fn xyce_selector(self) -> usize {
        match self {
            Self::LocalTruncation => 0,
            Self::NonlinearIterations => 1,
        }
    }
}

/// Xyce 7.10 `TIMEINT` defaults used by `ERROPTION=1` step control.
pub(crate) const XYCE_DEFAULT_MIN_TIME_STEPS_BREAKPOINT: usize = 10;
pub(crate) const XYCE_DEFAULT_NLMIN: usize = 3;
pub(crate) const XYCE_DEFAULT_NLMAX: usize = 8;

/// Fixed ceiling for one Xyce breakpoint span.
///
/// Xyce computes `(next stop - interval start) / MINTIMESTEPSBP` once when a
/// new integration interval is established. Recomputing from the remaining
/// distance on every step would shrink geometrically and never reproduce the
/// source algorithm.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct XyceBreakpointSpanCeiling {
    min_steps: Option<usize>,
    max_dt: Option<Value>,
}

impl XyceBreakpointSpanCeiling {
    pub(crate) fn new(min_steps: Option<usize>) -> Self {
        Self {
            min_steps: min_steps.filter(|steps| *steps > 0),
            max_dt: None,
        }
    }

    /// Anchor a new ceiling at startup/resume or after actually landing on a
    /// breakpoint. Merely discovering a runtime stop must not change the
    /// current span: the breakpoint manager first limits the step to that stop,
    /// then the accepted landing anchors the following span.
    pub(crate) fn anchor(
        &mut self,
        current_time: Value,
        next_breakpoint: Option<Value>,
        final_time: Value,
    ) -> Option<Value> {
        let min_steps = self.min_steps?;
        let span_end = next_breakpoint
            .filter(|time| time.is_finite() && *time > current_time)
            .map_or(final_time, |time| time.min(final_time));
        if !current_time.is_finite()
            || !span_end.is_finite()
            || span_end <= current_time
            || final_time <= current_time
        {
            self.max_dt = None;
            return None;
        }
        self.max_dt = Some((span_end - current_time) / min_steps as Value);
        self.max_dt
    }

    pub(crate) const fn ceiling(&self) -> Option<Value> {
        self.max_dt
    }

    /// Restore the ceiling of an already-active span from an authenticated
    /// in-flight checkpoint. Re-anchoring at the checkpoint time would create
    /// a shorter artificial span and could clamp the restored next proposal.
    pub(crate) fn restore_active_ceiling(
        &mut self,
        max_dt: Option<Value>,
    ) -> Result<Option<Value>, String> {
        if max_dt.is_some_and(|value| !value.is_finite() || value <= 0.0) {
            return Err(
                "checkpoint Xyce breakpoint-span ceiling must be finite and positive".to_string(),
            );
        }
        if self.min_steps.is_none() != max_dt.is_none() {
            return Err(match self.min_steps {
                None => {
                    "checkpoint carries a Xyce breakpoint-span ceiling while the policy is disabled"
                        .to_string()
                }
                Some(_) => {
                    "checkpoint omits the active Xyce breakpoint-span ceiling while the policy is enabled"
                        .to_string()
                }
            });
        }
        self.max_dt = max_dt;
        Ok(self.max_dt)
    }
}

/// Xyce `ERROPTION=1` accepted-step multiplier.
#[inline]
pub(crate) const fn xyce_iteration_step_scale(
    iterations: usize,
    nlmin: usize,
    nlmax: usize,
) -> Value {
    if iterations <= nlmin {
        2.0
    } else if iterations > nlmax {
        0.125
    } else {
        1.0
    }
}

/// Whether an otherwise converged `ERROPTION=1` attempt passes Xyce's
/// optional timestep-reversal test.
#[inline]
pub(crate) const fn xyce_iteration_step_accepts(
    iterations: usize,
    nlmax: usize,
    timesteps_reversal: bool,
) -> bool {
    !timesteps_reversal || iterations <= nlmax
}

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

    /// Get the controller's effective maximum timestep.
    ///
    /// This can be greater than a raw device or breakpoint-span limit when
    /// that limit lies below the hard convergence-recovery floor.
    pub fn max_dt(&self) -> Value {
        self.max_dt
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
    use crate::numerics::integration::{BreakpointManager, BreakpointStepPolicy};

    #[test]
    fn maximum_step_respects_the_existing_hard_floor() {
        let mut timestep =
            TimestepController::new_with_preferred_min(2.0e-12, 1.0e-12, 1.0e-9, 1.0e-6);
        timestep.set_max_dt(5.0e-13);

        assert_eq!(timestep.hard_min_dt().to_bits(), 1.0e-12_f64.to_bits());
        assert_eq!(timestep.max_dt().to_bits(), 1.0e-12_f64.to_bits());
        assert_eq!(timestep.dt().to_bits(), 1.0e-12_f64.to_bits());
    }

    #[test]
    fn restored_effective_maximum_preserves_floor_before_new_raw_cap() {
        let saved_proposal = 5.0e-13;
        let saved_effective_max = 5.0e-13;
        let resumed_time_floor = 1.0e-12;
        let resumed_raw_cap = 1.0e-13;

        let mut restored = TimestepController::new_with_preferred_min(
            saved_proposal,
            resumed_time_floor,
            1.0e-9,
            saved_effective_max,
        );
        restored.set_max_dt(resumed_raw_cap);

        assert_eq!(
            restored.hard_min_dt().to_bits(),
            saved_effective_max.to_bits()
        );
        assert_eq!(restored.max_dt().to_bits(), saved_effective_max.to_bits());
        assert_eq!(restored.dt().to_bits(), saved_proposal.to_bits());
    }

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

    #[test]
    fn xyce_iteration_policy_uses_inclusive_nlmin_and_exclusive_nlmax() {
        assert_eq!(xyce_iteration_step_scale(2, 3, 8), 2.0);
        assert_eq!(xyce_iteration_step_scale(3, 3, 8), 2.0);
        assert_eq!(xyce_iteration_step_scale(4, 3, 8), 1.0);
        assert_eq!(xyce_iteration_step_scale(8, 3, 8), 1.0);
        assert_eq!(xyce_iteration_step_scale(9, 3, 8), 0.125);
        assert!(xyce_iteration_step_accepts(9, 8, false));
        assert!(!xyce_iteration_step_accepts(9, 8, true));
        assert!(xyce_iteration_step_accepts(8, 8, true));
    }

    #[test]
    fn xyce_breakpoint_ceiling_reanchors_only_after_a_landing() {
        let mut ceiling = XyceBreakpointSpanCeiling::new(Some(10));
        assert_eq!(ceiling.anchor(0.0, Some(10.0), 20.0), Some(1.0));
        assert_eq!(ceiling.ceiling(), Some(1.0));

        // A runtime stop discovered mid-span limits the landing elsewhere but
        // does not geometrically or identity-refresh the established ceiling.
        let newly_discovered_runtime_stop = 4.0;
        assert_eq!(newly_discovered_runtime_stop, 4.0);
        assert_eq!(ceiling.ceiling(), Some(1.0));

        // Only after landing does the following interval get a new fixed cap.
        assert_eq!(ceiling.anchor(4.0, Some(10.0), 20.0), Some(0.6));
        assert_eq!(ceiling.ceiling(), Some(0.6));
    }

    #[test]
    fn xyce_breakpoint_ceiling_restores_an_interior_span_without_reanchoring() {
        let mut ceiling = XyceBreakpointSpanCeiling::new(Some(10));
        ceiling
            .restore_active_ceiling(Some(0.75))
            .expect("active ceiling restores");
        assert_eq!(ceiling.ceiling(), Some(0.75));

        let mut disabled = XyceBreakpointSpanCeiling::new(None);
        assert!(
            disabled
                .restore_active_ceiling(Some(0.75))
                .expect_err("disabled policy rejects invented ceiling")
                .contains("disabled")
        );
        assert_eq!(
            disabled
                .restore_active_ceiling(None)
                .expect("disabled policy restores absent ceiling"),
            None
        );

        let mut enabled = XyceBreakpointSpanCeiling::new(Some(10));
        assert!(
            enabled
                .restore_active_ceiling(None)
                .expect_err("enabled policy rejects missing active ceiling")
                .contains("omits")
        );
    }

    #[test]
    fn runtime_stop_limits_landing_without_reanchoring_the_active_span() {
        let mut breakpoints =
            BreakpointManager::new_with_tolerance_and_policy(1.0e-12, BreakpointStepPolicy::Xyce);
        breakpoints.add(10.0);
        let mut ceiling = XyceBreakpointSpanCeiling::new(Some(10));
        assert_eq!(
            ceiling.anchor(0.0, breakpoints.next_after(0.0), 10.0),
            Some(1.0)
        );

        // The runtime stop appears after the interval has begun. It constrains
        // the landing, but the original 1.0 cap remains active until then.
        breakpoints.replace_runtime_breakpoints([4.0]);
        assert_eq!(breakpoints.next_after(1.0), Some(4.0));
        assert_eq!(ceiling.ceiling(), Some(1.0));
        let (dt, lands) = breakpoints.limit_step(3.5, 1.0);
        assert_eq!(dt, 0.5);
        assert!(lands);
        assert_eq!(ceiling.ceiling(), Some(1.0));

        breakpoints.mark_breakpoint_solved(4.0);
        assert_eq!(
            ceiling.anchor(4.0, breakpoints.next_after(4.0), 10.0),
            Some(0.6)
        );
    }

    #[test]
    fn xyce_breakpoint_ceiling_uses_final_horizon_and_can_be_disabled() {
        let mut ceiling = XyceBreakpointSpanCeiling::new(Some(10));
        assert_eq!(ceiling.anchor(2.0, None, 7.0), Some(0.5));
        assert_eq!(ceiling.ceiling(), Some(0.5));

        let mut disabled = XyceBreakpointSpanCeiling::new(None);
        assert_eq!(disabled.anchor(0.0, Some(1.0), 2.0), None);
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
    /// Whether each previous derivative sign came from a real interval.
    prev_sign_valid: Vec<bool>,
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

/// Version-neutral state projection of the Trap/Gear hysteresis detector.
///
/// Checkpoint encoders own their wire version and transpose these parallel
/// vectors as needed. The detector's thresholds are deliberately absent:
/// they are implementation policy covered by the simulation identity, not
/// mutable trajectory state.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TrapGearControllerSnapshot {
    pub(crate) current_method: IntegrationMethod,
    pub(crate) prev_values: Vec<Value>,
    pub(crate) prev_signs: Vec<bool>,
    pub(crate) prev_sign_valid: Vec<bool>,
    pub(crate) sign_change_count: Vec<usize>,
    pub(crate) smooth_steps: usize,
    pub(crate) at_breakpoint: bool,
}

impl TrapGearController {
    /// Create a new TrapGear controller
    pub fn new() -> Self {
        Self {
            current_method: IntegrationMethod::Trapezoidal,
            prev_values: Vec::new(),
            prev_signs: Vec::new(),
            prev_sign_valid: Vec::new(),
            sign_change_count: Vec::new(),
            smooth_steps: 0,
            at_breakpoint: false,
            oscillation_threshold: TRAPGEAR_OSCILLATION_THRESHOLD,
            recovery_steps: TRAPGEAR_RECOVERY_STEPS,
        }
    }

    /// Capture every mutable field that can affect the next method decision.
    pub(crate) fn capture_snapshot(&self) -> TrapGearControllerSnapshot {
        TrapGearControllerSnapshot {
            current_method: self.current_method,
            prev_values: self.prev_values.clone(),
            prev_signs: self.prev_signs.clone(),
            prev_sign_valid: self.prev_sign_valid.clone(),
            sign_change_count: self.sign_change_count.clone(),
            smooth_steps: self.smooth_steps,
            at_breakpoint: self.at_breakpoint,
        }
    }

    /// Validate an untrusted controller snapshot without mutating live state.
    ///
    /// An empty detector is the canonical pre-first-update state and may sit
    /// beside a nonempty accepted solution. Once active, every lane is updated
    /// together and `prev_values` must be the exact accepted boundary used to
    /// seed the next derivative decision.
    pub(crate) fn validate_snapshot(
        snapshot: &TrapGearControllerSnapshot,
        accepted_solution: &[Value],
    ) -> Result<(), String> {
        if snapshot.current_method == IntegrationMethod::TrapGear {
            return Err(
                "Trap/Gear controller snapshot uses hybrid TrapGear as an effective method"
                    .to_string(),
            );
        }

        let lane_count = snapshot.prev_values.len();
        for (label, actual) in [
            ("previous derivative signs", snapshot.prev_signs.len()),
            (
                "previous derivative-sign validity",
                snapshot.prev_sign_valid.len(),
            ),
            (
                "consecutive sign-change counts",
                snapshot.sign_change_count.len(),
            ),
        ] {
            if actual != lane_count {
                return Err(format!(
                    "Trap/Gear controller snapshot {label} length {actual} does not match previous-value length {lane_count}"
                ));
            }
        }

        if snapshot.smooth_steps > TRAPGEAR_RECOVERY_STEPS {
            return Err(format!(
                "Trap/Gear controller snapshot smooth-step count {} exceeds canonical recovery bound {}",
                snapshot.smooth_steps, TRAPGEAR_RECOVERY_STEPS
            ));
        }

        if lane_count > 0 {
            if accepted_solution.len() != lane_count {
                return Err(format!(
                    "active Trap/Gear controller snapshot lane count {lane_count} does not match accepted solution length {}",
                    accepted_solution.len()
                ));
            }
            for (index, (&previous, &accepted)) in snapshot
                .prev_values
                .iter()
                .zip(accepted_solution)
                .enumerate()
            {
                if !previous.is_finite() {
                    return Err(format!(
                        "Trap/Gear controller snapshot previous value {index} is not finite"
                    ));
                }
                if !accepted.is_finite() {
                    return Err(format!(
                        "Trap/Gear controller accepted solution value {index} is not finite"
                    ));
                }
                if previous.to_bits() != accepted.to_bits() {
                    return Err(format!(
                        "Trap/Gear controller snapshot previous value {index} does not exactly match the accepted solution"
                    ));
                }
            }
        }

        let all_signs_valid = snapshot.prev_sign_valid.iter().all(|valid| *valid);
        let no_signs_valid = snapshot.prev_sign_valid.iter().all(|valid| !*valid);
        if !all_signs_valid && !no_signs_valid {
            return Err(
                "Trap/Gear controller snapshot mixes initialized and uninitialized derivative-sign lanes"
                    .to_string(),
            );
        }
        for (index, &count) in snapshot.sign_change_count.iter().enumerate() {
            if count >= TRAPGEAR_OSCILLATION_THRESHOLD {
                return Err(format!(
                    "Trap/Gear controller snapshot sign-change count {count} at lane {index} reaches detector threshold {TRAPGEAR_OSCILLATION_THRESHOLD}"
                ));
            }
            if !snapshot.prev_sign_valid[index] {
                if count != 0 {
                    return Err(format!(
                        "Trap/Gear controller snapshot uninitialized lane {index} has sign-change count {count}"
                    ));
                }
                if !snapshot.prev_signs[index] {
                    return Err(format!(
                        "Trap/Gear controller snapshot uninitialized lane {index} does not carry the canonical positive sign"
                    ));
                }
            }
        }
        Ok(())
    }

    /// Restore a validated snapshot atomically with respect to validation.
    pub(crate) fn restore_snapshot(
        &mut self,
        snapshot: &TrapGearControllerSnapshot,
        accepted_solution: &[Value],
    ) -> Result<(), String> {
        Self::validate_snapshot(snapshot, accepted_solution)?;
        self.current_method = snapshot.current_method;
        self.prev_values.clone_from(&snapshot.prev_values);
        self.prev_signs.clone_from(&snapshot.prev_signs);
        self.prev_sign_valid.clone_from(&snapshot.prev_sign_valid);
        self.sign_change_count
            .clone_from(&snapshot.sign_change_count);
        self.smooth_steps = snapshot.smooth_steps;
        self.at_breakpoint = snapshot.at_breakpoint;
        Ok(())
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
            self.prev_sign_valid = vec![false; solution.len()];
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
            if !self.prev_sign_valid[i] {
                self.sign_change_count[i] = 0;
                self.prev_sign_valid[i] = true;
            } else if curr_sign != self.prev_signs[i]
                && derivative.abs() > TRAPGEAR_SIGN_CHANGE_FLOOR
            {
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
            self.smooth_steps = self.smooth_steps.saturating_add(1).min(self.recovery_steps);

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

    /// Restart the hybrid detector from an accepted breakpoint solution.
    ///
    /// A source discontinuity invalidates derivative-sign evidence collected
    /// on the preceding smooth span. Retain the accepted breakpoint value as
    /// the derivative origin for the first departure, while returning the
    /// effective method and hysteresis counters to their canonical startup
    /// state.
    pub(crate) fn restart_from(&mut self, accepted_solution: &[Value]) {
        self.current_method = IntegrationMethod::Trapezoidal;
        self.prev_values.clear();
        self.prev_values.extend_from_slice(accepted_solution);
        self.prev_signs.clear();
        self.prev_signs.resize(accepted_solution.len(), true);
        self.prev_sign_valid.clear();
        self.prev_sign_valid.resize(accepted_solution.len(), false);
        self.sign_change_count.clear();
        self.sign_change_count.resize(accepted_solution.len(), 0);
        self.smooth_steps = 0;
        self.at_breakpoint = true;
    }

    /// Reset the controller state
    pub fn reset(&mut self) {
        self.prev_values.clear();
        self.prev_signs.clear();
        self.prev_sign_valid.clear();
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
    fn snapshot_round_trip_preserves_active_detector_phase_exactly() {
        let mut source = TrapGearController::new();
        for sample in [0.0, 1.0, 0.0, 1.0, 0.0, 1.0] {
            source.update(&[sample], 1.0e-9);
        }
        source.set_at_breakpoint(true);
        assert_eq!(source.current_method(), IntegrationMethod::Gear2);

        let snapshot = source.capture_snapshot();
        TrapGearController::validate_snapshot(&snapshot, &[1.0])
            .expect("captured active detector validates");
        assert_eq!(snapshot.sign_change_count, [1]);
        assert!(snapshot.prev_sign_valid[0]);
        assert!(snapshot.at_breakpoint);

        let mut restored = TrapGearController::new();
        restored.force_method(IntegrationMethod::BackwardEuler);
        restored.update(&[99.0], 2.0e-9);
        restored
            .restore_snapshot(&snapshot, &[1.0])
            .expect("active detector restores");
        assert_eq!(restored.capture_snapshot(), snapshot);

        assert_eq!(
            source.update(&[0.0], 1.0e-9),
            restored.update(&[0.0], 1.0e-9)
        );
        assert_eq!(source.capture_snapshot(), restored.capture_snapshot());
        assert_eq!(
            source.update(&[1.0], 1.0e-9),
            restored.update(&[1.0], 1.0e-9)
        );
        assert_eq!(source.capture_snapshot(), restored.capture_snapshot());
    }

    #[test]
    fn snapshot_validation_rejects_corruption_before_restore_mutates() {
        let mut source = TrapGearController::new();
        source.restart_from(&[1.0, 2.0]);
        let valid = source.capture_snapshot();
        TrapGearController::validate_snapshot(&valid, &[1.0, 2.0])
            .expect("canonical restart detector validates");

        let mut destination = TrapGearController::new();
        destination.update(&[7.0], 1.0e-9);
        let destination_before = destination.capture_snapshot();

        let mut bad_shape = valid.clone();
        bad_shape.prev_signs.pop();
        assert!(
            destination
                .restore_snapshot(&bad_shape, &[1.0, 2.0])
                .expect_err("shape mismatch is rejected")
                .contains("length")
        );
        assert_eq!(destination.capture_snapshot(), destination_before);

        let mut bad_method = valid.clone();
        bad_method.current_method = IntegrationMethod::TrapGear;
        assert!(
            TrapGearController::validate_snapshot(&bad_method, &[1.0, 2.0])
                .expect_err("hybrid effective method is rejected")
                .contains("effective method")
        );

        let mut bad_value = valid.clone();
        bad_value.prev_values[0] = Value::NAN;
        assert!(
            TrapGearController::validate_snapshot(&bad_value, &[Value::NAN, 2.0])
                .expect_err("non-finite previous value is rejected")
                .contains("not finite")
        );

        let mut bad_boundary = valid.clone();
        bad_boundary.prev_values[0] = -0.0;
        assert!(
            TrapGearController::validate_snapshot(&bad_boundary, &[0.0, 2.0])
                .expect_err("non-exact accepted boundary is rejected")
                .contains("exactly match")
        );

        let mut bad_count = valid.clone();
        bad_count.prev_sign_valid.fill(true);
        bad_count.sign_change_count[0] = TRAPGEAR_OSCILLATION_THRESHOLD;
        assert!(
            TrapGearController::validate_snapshot(&bad_count, &[1.0, 2.0])
                .expect_err("threshold-reaching count is rejected")
                .contains("threshold")
        );

        let mut bad_uninitialized_lane = valid.clone();
        bad_uninitialized_lane.sign_change_count[0] = 1;
        assert!(
            TrapGearController::validate_snapshot(&bad_uninitialized_lane, &[1.0, 2.0])
                .expect_err("uninitialized count is rejected")
                .contains("uninitialized lane")
        );

        let mut bad_mixed_validity = valid.clone();
        bad_mixed_validity.prev_sign_valid[0] = true;
        assert!(
            TrapGearController::validate_snapshot(&bad_mixed_validity, &[1.0, 2.0])
                .expect_err("mixed validity is rejected")
                .contains("mixes")
        );

        let mut bad_smooth = valid;
        bad_smooth.smooth_steps = TRAPGEAR_RECOVERY_STEPS + 1;
        assert!(
            TrapGearController::validate_snapshot(&bad_smooth, &[1.0, 2.0])
                .expect_err("oversized smooth count is rejected")
                .contains("recovery bound")
        );
    }

    #[test]
    fn snapshot_allows_canonical_inactive_state_and_caps_smooth_provenance() {
        let inactive = TrapGearController::new().capture_snapshot();
        TrapGearController::validate_snapshot(&inactive, &[4.0, 5.0])
            .expect("pre-first-update detector may accompany an accepted solution");

        let mut smooth = TrapGearController::new();
        smooth.update(&[1.0], 1.0e-9);
        for _ in 0..16 {
            smooth.update(&[1.0], 1.0e-9);
        }
        let snapshot = smooth.capture_snapshot();
        assert_eq!(snapshot.smooth_steps, TRAPGEAR_RECOVERY_STEPS);
        TrapGearController::validate_snapshot(&snapshot, &[1.0])
            .expect("saturated smooth detector validates");
    }

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

    #[test]
    fn breakpoint_restart_discards_detector_phase_and_seeds_the_accepted_solution() {
        let mut controller = TrapGearController::new();
        controller.force_method(IntegrationMethod::Gear2);
        controller.update(&[100.0], 1.0e-9);
        controller.update(&[-100.0], 1.0e-9);

        controller.restart_from(&[3.0]);
        assert_eq!(controller.current_method(), IntegrationMethod::Trapezoidal);

        assert!(!controller.update(&[2.0], 1.0e-9));
        assert!(!controller.update(&[3.0], 1.0e-9));
        assert!(!controller.update(&[2.0], 1.0e-9));
        assert!(
            controller.update(&[3.0], 1.0e-9),
            "the accepted breakpoint value must be the first derivative origin"
        );
        assert_eq!(controller.current_method(), IntegrationMethod::Gear2);
    }
}
