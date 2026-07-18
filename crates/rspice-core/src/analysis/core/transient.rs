//! Transient Time-Domain Analysis

#![allow(clippy::type_complexity)]
use crate::Value;
use crate::netlist::TransientLteReference;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Unbounded};

type ChargeLteInputs<'a> = (&'a [Value], &'a [Value], &'a [Value], &'a [Value]);
const TRAPGEAR_SIGN_CHANGE_FLOOR: Value = crate::constants::VNTOL;

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

/// Minimum timestep to use immediately after a breakpoint (for restart behavior)
const MIN_STEP_AFTER_BREAKPOINT: Value = 1e-12;

/// Fallback tolerance for detecting exact breakpoint landing.
///
/// Transient engine paths should prefer an ngspice-style tolerance derived
/// from `CKTminBreak`; this default keeps standalone manager use conservative.
const DEFAULT_BREAKPOINT_TOLERANCE: Value = 1e-15;

/// Ngspice `dctran.c` factor for equalizing the last two pre-breakpoint steps.
const BREAKPOINT_EQUALIZATION_FACTOR: Value = 1.9;

/// Established restart scale used by the default (ngspice-compatible)
/// breakpoint policy.
const DEFAULT_BREAKPOINT_RESTART_STEP_SCALE: Value = 0.005;

/// Xyce 7.10 OneStep/Gear12 scale for the first step after a breakpoint.
///
/// This is deliberately distinct from TIMEINT `RESTARTSTEPSCALE=0.005`, which
/// bounds the initial step at the beginning of an analysis. The legacy Xyce
/// integrators restart after later breakpoints at 10% of the saved, uncut
/// proposal.
const XYCE_BREAKPOINT_RESTART_STEP_SCALE: Value = 0.1;

/// Dialect-specific behavior for approaching and leaving breakpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakpointStepPolicy {
    /// Apply ngspice's 1.9-factor equalization before truncating to a corner.
    Ngspice,
    /// Match Xyce OneStep/Gear12: truncate directly to the stop time and
    /// restart at 10% of the saved pre-truncation proposal.
    Xyce,
}

#[derive(Debug, Clone, Copy)]
struct BreakpointTimeKey(Value);

impl PartialEq for BreakpointTimeKey {
    fn eq(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0) == Ordering::Equal
    }
}

impl Eq for BreakpointTimeKey {}

impl PartialOrd for BreakpointTimeKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BreakpointTimeKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

/// Breakpoint manager for handling discontinuities
///
/// Ensures solver lands exactly on breakpoints and restarts from the same
/// saved-delta scale used by Xyce's time-integration restart path.
#[derive(Debug)]
pub struct BreakpointManager {
    /// Sorted list of breakpoint times
    breakpoints: Vec<Value>,
    /// Runtime XSPICE breakpoints/events for the next transient advance.
    ///
    /// Code-model event requests are re-issued from accepted model state. They
    /// should therefore replace the prior runtime schedule instead of
    /// accumulating with source/permanent breakpoints.
    runtime_breakpoints: BTreeSet<BreakpointTimeKey>,
    /// Index of next unprocessed breakpoint (for efficiency)
    current_index: usize,
    /// Flag indicating we just passed a breakpoint
    just_passed_breakpoint: bool,
    /// Timestep that was proposed before it was cut/equalized for a breakpoint.
    saved_delta_before_breakpoint: Option<Value>,
    /// Integrator-specific fraction of the saved pre-breakpoint proposal used
    /// to initialize the first post-breakpoint step.
    restart_step_scale: Value,
    /// Whether to equalize the final two steps before a breakpoint.
    equalize_pre_breakpoint_steps: bool,
    /// Tolerance for merging and detecting breakpoint times.
    tolerance: Value,
}

impl Default for BreakpointManager {
    fn default() -> Self {
        Self::new()
    }
}

impl BreakpointManager {
    pub fn new() -> Self {
        Self::new_with_tolerance(DEFAULT_BREAKPOINT_TOLERANCE)
    }

    pub fn new_with_tolerance(tolerance: Value) -> Self {
        Self::new_with_tolerance_and_policy(tolerance, BreakpointStepPolicy::Ngspice)
    }

    pub fn new_with_tolerance_and_policy(tolerance: Value, policy: BreakpointStepPolicy) -> Self {
        let (restart_step_scale, equalize_pre_breakpoint_steps) = match policy {
            BreakpointStepPolicy::Ngspice => (DEFAULT_BREAKPOINT_RESTART_STEP_SCALE, true),
            BreakpointStepPolicy::Xyce => (XYCE_BREAKPOINT_RESTART_STEP_SCALE, false),
        };
        Self {
            breakpoints: Vec::new(),
            runtime_breakpoints: BTreeSet::new(),
            current_index: 0,
            just_passed_breakpoint: false,
            saved_delta_before_breakpoint: None,
            restart_step_scale,
            equalize_pre_breakpoint_steps,
            tolerance: if tolerance.is_finite() && tolerance > 0.0 {
                tolerance
            } else {
                DEFAULT_BREAKPOINT_TOLERANCE
            },
        }
    }

    /// Add a breakpoint time (deduplicates automatically)
    pub fn add(&mut self, time: Value) -> bool {
        Self::insert_sorted_unique(&mut self.breakpoints, time, self.tolerance)
    }

    /// Replace the dynamic runtime breakpoint schedule.
    ///
    /// XSPICE runtime requests mirror ngspice's event/temp-breakpoint machinery:
    /// they are valid for the next accepted model state, not permanent source
    /// breakpoints. Keeping them separate prevents stale predicted event times
    /// from forcing later transient steps.
    pub fn replace_runtime_breakpoints<I>(&mut self, times: I)
    where
        I: IntoIterator<Item = Value>,
    {
        self.runtime_breakpoints.clear();
        for time in times {
            if !time.is_finite() || time < 0.0 {
                continue;
            }
            if Self::contains_sorted_within_tolerance(
                &self.breakpoints[self.current_index..],
                time,
                self.tolerance,
            ) {
                continue;
            }
            Self::insert_runtime_unique(&mut self.runtime_breakpoints, time, self.tolerance);
        }
    }

    fn insert_sorted_unique(values: &mut Vec<Value>, time: Value, tolerance: Value) -> bool {
        if !time.is_finite() {
            return false;
        }
        let pos = Self::sorted_insert_position(values, time);
        if Self::neighbors_within_tolerance(values, pos, time, tolerance) {
            return false;
        }
        values.insert(pos, time);
        true
    }

    fn contains_sorted_within_tolerance(values: &[Value], time: Value, tolerance: Value) -> bool {
        if values.is_empty() {
            return false;
        }
        let pos = Self::sorted_insert_position(values, time);
        Self::neighbors_within_tolerance(values, pos, time, tolerance)
    }

    fn insert_runtime_unique(
        values: &mut BTreeSet<BreakpointTimeKey>,
        time: Value,
        tolerance: Value,
    ) -> bool {
        if !time.is_finite() || Self::contains_runtime_within_tolerance(values, time, tolerance) {
            return false;
        }
        values.insert(BreakpointTimeKey(time))
    }

    fn contains_runtime_within_tolerance(
        values: &BTreeSet<BreakpointTimeKey>,
        time: Value,
        tolerance: Value,
    ) -> bool {
        let lower = time - tolerance;
        values
            .range((Excluded(BreakpointTimeKey(lower)), Unbounded))
            .next()
            .is_some_and(|existing| (existing.0 - time).abs() < tolerance)
    }

    fn sorted_insert_position(values: &[Value], time: Value) -> usize {
        match values.binary_search_by(|existing| existing.total_cmp(&time)) {
            Ok(pos) | Err(pos) => pos,
        }
    }

    fn neighbors_within_tolerance(
        values: &[Value],
        pos: usize,
        time: Value,
        tolerance: Value,
    ) -> bool {
        values
            .get(pos)
            .is_some_and(|existing| (*existing - time).abs() < tolerance)
            || pos
                .checked_sub(1)
                .and_then(|prev| values.get(prev))
                .is_some_and(|existing| (*existing - time).abs() < tolerance)
    }

    /// Add breakpoints from a periodic source (pulse edges, clock, etc.)
    pub fn add_periodic(&mut self, start: Value, period: Value, end: Value) {
        let mut t = start;
        while t <= end {
            self.add(t);
            t += period;
        }
    }

    /// Get next breakpoint after given time
    pub fn next_after(&self, time: Value) -> Option<Value> {
        let permanent = self
            .breakpoints
            .iter()
            .skip(self.current_index)
            .copied()
            .find(|&t| t > time + self.tolerance);
        let runtime = self
            .runtime_breakpoints
            .range((
                Excluded(BreakpointTimeKey(time + self.tolerance)),
                Unbounded,
            ))
            .next()
            .map(|time| time.0);
        match (permanent, runtime) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    /// Treat breakpoints at or before `time` as already handled.
    pub fn discard_through(&mut self, time: Value) {
        while self.current_index < self.breakpoints.len()
            && self.breakpoints[self.current_index] <= time + self.tolerance
        {
            self.current_index += 1;
        }
        self.remove_runtime_through(time);
    }

    /// Check if current time is exactly at a breakpoint
    pub fn at_breakpoint(&self, time: Value) -> bool {
        self.breakpoints
            .iter()
            .skip(self.current_index)
            .any(|&bp| (time - bp).abs() < self.tolerance)
            || self
                .runtime_breakpoints
                .iter()
                .any(|bp| (time - bp.0).abs() < self.tolerance)
    }

    /// Return the exact breakpoint time when `time` is within the breakpoint
    /// tolerance. This keeps source evaluation and breakpoint bookkeeping on
    /// the same side of discontinuities after floating-point timestep cuts.
    pub fn snap_to_breakpoint(&self, time: Value) -> Value {
        if let Some(&breakpoint) = self
            .breakpoints
            .iter()
            .skip(self.current_index)
            .find(|&&bp| (time - bp).abs() < self.tolerance)
        {
            return breakpoint;
        }
        self.runtime_breakpoints
            .iter()
            .find(|breakpoint| (time - breakpoint.0).abs() < self.tolerance)
            .map(|breakpoint| breakpoint.0)
            .unwrap_or(time)
    }

    /// Limit timestep to land exactly on next breakpoint
    ///
    /// Returns (adjusted_dt, will_land_on_breakpoint)
    pub fn limit_step(&mut self, current_time: Value, proposed_dt: Value) -> (Value, bool) {
        if !current_time.is_finite() || !proposed_dt.is_finite() || proposed_dt <= 0.0 {
            return (proposed_dt, false);
        }

        match self.next_after(current_time) {
            Some(bp) => {
                let time_to_bp = bp - current_time;
                if !time_to_bp.is_finite() || time_to_bp <= 0.0 {
                    return (proposed_dt, false);
                }

                if current_time + proposed_dt >= bp {
                    self.saved_delta_before_breakpoint = Some(proposed_dt);
                    self.just_passed_breakpoint = false;
                    (time_to_bp, true)
                } else if self.equalize_pre_breakpoint_steps
                    && current_time + BREAKPOINT_EQUALIZATION_FACTOR * proposed_dt > bp
                {
                    self.saved_delta_before_breakpoint = Some(proposed_dt);
                    self.just_passed_breakpoint = false;
                    (time_to_bp / 2.0, false)
                } else {
                    (proposed_dt, false)
                }
            }
            None => (proposed_dt, false),
        }
    }

    /// Mark that we just solved at a breakpoint (call after solving at BP)
    /// Returns the Xyce-style restart timestep.
    pub fn mark_breakpoint_solved(&mut self, time: Value) -> Value {
        // Advance current_index past this breakpoint
        while self.current_index < self.breakpoints.len()
            && self.breakpoints[self.current_index] <= time + self.tolerance
        {
            self.current_index += 1;
        }
        self.remove_runtime_through(time);
        self.just_passed_breakpoint = true;
        let next_gap = self
            .next_after(time)
            .map(|next| next - time)
            .filter(|gap| gap.is_finite() && *gap > 0.0)
            .unwrap_or(Value::INFINITY);
        let saved_delta = self
            .saved_delta_before_breakpoint
            .take()
            .filter(|delta| delta.is_finite() && *delta > 0.0);

        saved_delta
            .map(|delta| self.restart_step_scale * delta.min(next_gap))
            .filter(|restart| restart.is_finite() && *restart > 0.0)
            .unwrap_or(MIN_STEP_AFTER_BREAKPOINT)
    }

    /// Check if we just passed a breakpoint and should use minimal timestep
    pub fn should_use_minimal_step(&self) -> bool {
        self.just_passed_breakpoint
    }

    /// Clear the just-passed flag (call after first step post-breakpoint)
    pub fn clear_breakpoint_flag(&mut self) {
        self.just_passed_breakpoint = false;
    }

    /// Reset the manager for a new simulation
    pub fn reset(&mut self) {
        self.current_index = 0;
        self.runtime_breakpoints.clear();
        self.just_passed_breakpoint = false;
    }

    /// Get total number of breakpoints
    pub fn len(&self) -> usize {
        self.breakpoints.len() + self.runtime_breakpoints.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.breakpoints.is_empty() && self.runtime_breakpoints.is_empty()
    }

    /// Borrow the sorted breakpoint schedule.
    pub fn times(&self) -> &[Value] {
        &self.breakpoints
    }

    fn remove_runtime_through(&mut self, time: Value) {
        let cutoff = time + self.tolerance;
        let mut retained = self
            .runtime_breakpoints
            .split_off(&BreakpointTimeKey(cutoff));
        while retained
            .first()
            .is_some_and(|breakpoint| breakpoint.0 <= cutoff)
        {
            retained.pop_first();
        }
        self.runtime_breakpoints = retained;
    }
}

#[cfg(test)]
mod breakpoint_manager_tests {
    use super::*;

    #[test]
    fn limit_step_direct_hit_lands_on_breakpoint() {
        let mut breakpoints = BreakpointManager::new();
        breakpoints.add(10.0);

        let (dt, lands_on_breakpoint) = breakpoints.limit_step(4.0, 6.0);

        assert_eq!(dt, 6.0);
        assert!(lands_on_breakpoint);
        assert!(!breakpoints.should_use_minimal_step());
    }

    #[test]
    fn breakpoint_restart_scales_saved_attempted_step() {
        let mut breakpoints = BreakpointManager::new();
        breakpoints.add(10.0);

        let (dt, lands_on_breakpoint) = breakpoints.limit_step(4.0, 6.0);
        assert_eq!(dt, 6.0);
        assert!(lands_on_breakpoint);

        let restart = breakpoints.mark_breakpoint_solved(10.0);

        assert_eq!(restart, DEFAULT_BREAKPOINT_RESTART_STEP_SCALE * 6.0);
        assert!(breakpoints.should_use_minimal_step());
    }

    #[test]
    fn xyce_breakpoint_policy_uses_the_integrator_restart_scale() {
        let mut breakpoints =
            BreakpointManager::new_with_tolerance_and_policy(1.0e-15, BreakpointStepPolicy::Xyce);
        breakpoints.add(10.0);

        let (dt, lands_on_breakpoint) = breakpoints.limit_step(4.0, 6.0);
        assert_eq!(dt, 6.0);
        assert!(lands_on_breakpoint);

        let restart = breakpoints.mark_breakpoint_solved(10.0);
        assert!((restart - 0.6).abs() <= 8.0 * Value::EPSILON);
    }

    #[test]
    fn xyce_breakpoint_policy_truncates_without_ngspice_equalization() {
        let mut breakpoints =
            BreakpointManager::new_with_tolerance_and_policy(1.0e-15, BreakpointStepPolicy::Xyce);
        breakpoints.add(10.0);

        let (dt, lands_on_breakpoint) = breakpoints.limit_step(0.0, 6.0);

        assert_eq!(dt, 6.0);
        assert!(!lands_on_breakpoint);
    }

    #[test]
    fn limit_step_equalizes_last_two_steps_without_marking_breakpoint_hit() {
        let mut breakpoints = BreakpointManager::new();
        breakpoints.add(10.0);

        let (dt, lands_on_breakpoint) = breakpoints.limit_step(0.0, 6.0);

        assert_eq!(dt, 5.0);
        assert!(!lands_on_breakpoint);
        assert!(!breakpoints.should_use_minimal_step());
    }

    #[test]
    fn limit_step_keeps_ordinary_step_before_equalization_window() {
        let mut breakpoints = BreakpointManager::new();
        breakpoints.add(10.0);

        let (dt, lands_on_breakpoint) = breakpoints.limit_step(0.0, 5.0);

        assert_eq!(dt, 5.0);
        assert!(!lands_on_breakpoint);
        assert!(!breakpoints.should_use_minimal_step());
    }

    #[test]
    fn ngspice_sized_tolerance_preserves_near_future_source_corner() {
        let mut breakpoints = BreakpointManager::new_with_tolerance(1.0e-21);
        breakpoints.add(1.0e-9);

        let current_time = 1.0e-9 - 5.0e-16;
        let (dt, lands_on_breakpoint) = breakpoints.limit_step(current_time, 1.0e-15);

        assert!((dt - 5.0e-16).abs() <= 1.0e-25, "dt={dt:.17e}");
        assert!(lands_on_breakpoint);
    }

    #[test]
    fn breakpoint_insertions_stay_sorted_and_tolerance_deduplicated() {
        let mut breakpoints = BreakpointManager::new_with_tolerance(1.0e-12);

        assert!(breakpoints.add(3.0e-9));
        assert!(breakpoints.add(1.0e-9));
        assert!(breakpoints.add(2.0e-9));
        assert!(!breakpoints.add(2.0e-9 + 0.5e-12));
        assert!(!breakpoints.add(Value::INFINITY));
        assert_eq!(breakpoints.times(), &[1.0e-9, 2.0e-9, 3.0e-9]);

        breakpoints.replace_runtime_breakpoints([
            5.0e-9,
            4.0e-9,
            4.0e-9 + 0.5e-12,
            2.0e-9 + 0.5e-12,
            Value::NAN,
            -1.0,
        ]);

        let runtime_breakpoints = breakpoints
            .runtime_breakpoints
            .iter()
            .map(|time| time.0)
            .collect::<Vec<_>>();
        assert_eq!(runtime_breakpoints, vec![4.0e-9, 5.0e-9]);
        assert_eq!(breakpoints.next_after(3.5e-9), Some(4.0e-9));

        breakpoints.discard_through(4.0e-9);
        let remaining_runtime_breakpoints = breakpoints
            .runtime_breakpoints
            .iter()
            .map(|time| time.0)
            .collect::<Vec<_>>();
        assert_eq!(remaining_runtime_breakpoints, vec![5.0e-9]);
    }

    #[test]
    fn limit_step_ignores_non_positive_or_non_finite_inputs() {
        let mut breakpoints = BreakpointManager::new();
        breakpoints.add(10.0);

        for proposed_dt in [0.0, -1.0, Value::INFINITY, Value::NAN] {
            let (dt, lands_on_breakpoint) = breakpoints.limit_step(0.0, proposed_dt);

            if proposed_dt.is_nan() {
                assert!(dt.is_nan());
            } else {
                assert_eq!(dt, proposed_dt);
            }
            assert!(!lands_on_breakpoint);
        }

        let (dt, lands_on_breakpoint) = breakpoints.limit_step(Value::INFINITY, 1.0);

        assert_eq!(dt, 1.0);
        assert!(!lands_on_breakpoint);
    }

    #[test]
    fn discard_through_prevents_t0_breakpoint_from_snapping_startup_steps_backwards() {
        let mut breakpoints = BreakpointManager::new_with_tolerance(2.0e-18);
        breakpoints.add(0.0);
        breakpoints.add(2.0e-6);

        breakpoints.discard_through(0.0);

        assert!(!breakpoints.at_breakpoint(1.0e-18));
        assert_eq!(breakpoints.snap_to_breakpoint(1.0e-18), 1.0e-18);
        assert_eq!(breakpoints.next_after(0.0), Some(2.0e-6));
    }
}

/// Local Truncation Error (LTE) estimator for adaptive timestep
///
/// Uses difference between predicted and calculated values to estimate
/// the integration error. This allows rejecting timesteps that converge
/// numerically but are physically inaccurate.
///
/// Supports both standard extrapolation and Richardson extrapolation for
/// higher accuracy LTE estimates.
#[derive(Debug)]
pub struct LteEstimator {
    /// Previous solution vector (t - dt)
    prev_solution: Vec<Value>,
    /// Solution before previous (t - 2*dt)
    prev_prev_solution: Vec<Value>,
    /// Solution from 3 steps ago (t - 3*dt) for Richardson extrapolation
    prev_prev_prev_solution: Vec<Value>,
    /// Previous timestep
    prev_dt: Value,
    /// Timestep before previous
    prev_prev_dt: Value,
    /// Relative LTE tolerance.
    reltol: Value,
    /// Absolute LTE tolerance floor for state variables.
    abstol: Value,
    /// Reference magnitude policy used by the normalized LTE weights.
    reference: TransientLteReference,
    /// Most recent accepted solution used by Xyce point-reference modes.
    accepted_reference_solution: Vec<Value>,
    /// Largest accepted state magnitude seen by signal-global weighting.
    signal_global_reference: Value,
    /// Largest accepted magnitude per state seen by signal-local weighting.
    signal_local_reference: Vec<Value>,
    /// Number of valid history entries
    history_count: usize,
    /// Current integration method (for order-aware scaling)
    method_order: u32,
}

impl LteEstimator {
    /// Create a new LTE estimator with legacy single-tolerance semantics.
    ///
    /// `tolerance` is used for both relative and absolute LTE control.
    pub fn new(tolerance: Value) -> Self {
        Self::with_tolerances(tolerance, tolerance)
    }

    /// Create a new LTE estimator with explicit relative and absolute tolerances.
    pub fn with_tolerances(reltol: Value, abstol: Value) -> Self {
        Self::with_tolerances_and_reference(reltol, abstol, TransientLteReference::PredictorLocal)
    }

    /// Create an LTE estimator with explicit tolerances and reference policy.
    pub fn with_tolerances_and_reference(
        reltol: Value,
        abstol: Value,
        reference: TransientLteReference,
    ) -> Self {
        let reltol = if reltol.is_finite() && reltol > 0.0 {
            reltol
        } else {
            1e-3
        };
        let abstol = if abstol.is_finite() && abstol > 0.0 {
            abstol
        } else {
            reltol
        };

        Self {
            prev_solution: Vec::new(),
            prev_prev_solution: Vec::new(),
            prev_prev_prev_solution: Vec::new(),
            prev_dt: 0.0,
            prev_prev_dt: 0.0,
            reltol,
            abstol,
            reference,
            accepted_reference_solution: Vec::new(),
            signal_global_reference: 0.0,
            signal_local_reference: Vec::new(),
            history_count: 0,
            method_order: 2, // Default to trapezoidal order
        }
    }

    #[inline]
    fn lte_scale_denominator(&self, magnitude: Value) -> Value {
        // Enforce SPICE-like weighted tolerance:
        // |err| <= ABSTOL + RELTOL * |x|
        // Rearranged to a normalized metric compared against RELTOL:
        // |err| / (ABSTOL/RELTOL + |x|) <= RELTOL
        let reltol = self.reltol.max(1e-30);
        (self.abstol / reltol + magnitude.abs()).max(1e-30)
    }

    #[inline]
    fn accepted_point_global_reference(&self) -> Option<Value> {
        self.accepted_reference_solution
            .iter()
            .try_fold(0.0_f64, |reference, value| {
                value.is_finite().then(|| reference.max(value.abs()))
            })
    }

    #[inline]
    fn accepted_point_global_reference_prefix_excluding(
        &self,
        prefix_len: usize,
        excluded_indices: &[usize],
    ) -> Option<Value> {
        self.accepted_reference_solution
            .iter()
            .take(prefix_len)
            .enumerate()
            .filter(|(index, _)| excluded_indices.binary_search(index).is_err())
            .try_fold(0.0_f64, |reference, (_, value)| {
                value.is_finite().then(|| reference.max(value.abs()))
            })
    }

    #[inline]
    fn accepted_reference_magnitude(&self, index: usize, accepted_point_global: Value) -> Value {
        match self.reference {
            TransientLteReference::PredictorLocal => 0.0,
            TransientLteReference::PointLocal => self.accepted_reference_solution[index].abs(),
            TransientLteReference::PointGlobal => accepted_point_global,
            TransientLteReference::SignalGlobal => {
                self.signal_global_reference.max(accepted_point_global)
            }
            TransientLteReference::SignalLocal => self
                .signal_local_reference
                .get(index)
                .copied()
                .unwrap_or_else(|| self.accepted_reference_solution[index].abs()),
        }
    }

    fn update_signal_reference_prefix(&mut self, solution: &[Value], prefix_len: usize) {
        let reference_len = prefix_len.min(solution.len());
        match self.reference {
            TransientLteReference::SignalGlobal => {
                let accepted_max = solution[..reference_len]
                    .iter()
                    .filter(|value| value.is_finite())
                    .map(|value| value.abs())
                    .fold(0.0, Value::max);
                self.signal_global_reference = self.signal_global_reference.max(accepted_max);
            }
            TransientLteReference::SignalLocal => {
                if self.signal_local_reference.len() < reference_len {
                    self.signal_local_reference.resize(reference_len, 0.0);
                }
                for (reference, value) in self
                    .signal_local_reference
                    .iter_mut()
                    .zip(&solution[..reference_len])
                {
                    if value.is_finite() {
                        *reference = (*reference).max(value.abs());
                    }
                }
            }
            TransientLteReference::PointLocal | TransientLteReference::PointGlobal => {}
            TransientLteReference::PredictorLocal => {}
        }
        if self.reference != TransientLteReference::PredictorLocal {
            self.accepted_reference_solution.clear();
            self.accepted_reference_solution
                .extend_from_slice(&solution[..reference_len]);
        }
    }

    /// Seed historical signal references without adding predictor history.
    #[cfg(test)]
    pub(crate) fn seed_reference_prefix(&mut self, solution: &[Value], prefix_len: usize) {
        self.update_signal_reference_prefix(solution, prefix_len);
    }

    /// Seed both Xyce reference state and the accepted t0 predictor history.
    pub(crate) fn seed_initial_solution(&mut self, solution: &[Value]) {
        self.update_signal_reference_prefix(solution, solution.len());
        self.restart_history_from(solution);
    }

    /// Snapshot historical signal references for transient checkpointing.
    pub(crate) fn signal_reference_snapshot(&self) -> (Value, &[Value]) {
        (self.signal_global_reference, &self.signal_local_reference)
    }

    /// Restore historical signal references from a validated checkpoint.
    pub(crate) fn restore_signal_reference_snapshot(
        &mut self,
        global: Value,
        local: &[Value],
    ) -> Result<(), String> {
        if !global.is_finite()
            || global < 0.0
            || local.iter().any(|value| !value.is_finite() || *value < 0.0)
        {
            return Err("checkpoint contains invalid transient LTE references".to_string());
        }
        match self.reference {
            TransientLteReference::SignalGlobal => {
                let accepted = self.accepted_point_global_reference().ok_or_else(|| {
                    "checkpoint accepted solution contains a non-finite LTE reference".to_string()
                })?;
                if !local.is_empty() || global < accepted {
                    return Err(
                        "checkpoint signal-global LTE state is inconsistent with the accepted solution"
                            .to_string(),
                    );
                }
            }
            TransientLteReference::SignalLocal => {
                if self
                    .accepted_reference_solution
                    .iter()
                    .any(|value| !value.is_finite())
                    || global != 0.0
                    || local.len() != self.accepted_reference_solution.len()
                    || local
                        .iter()
                        .zip(&self.accepted_reference_solution)
                        .any(|(reference, accepted)| *reference < accepted.abs())
                {
                    return Err(
                        "checkpoint signal-local LTE references do not cover the accepted solution"
                            .to_string(),
                    );
                }
            }
            TransientLteReference::PredictorLocal
            | TransientLteReference::PointLocal
            | TransientLteReference::PointGlobal => {
                if global != 0.0 || !local.is_empty() {
                    return Err(
                        "checkpoint point-reference LTE state contains signal history".to_string(),
                    );
                }
            }
        }
        self.signal_global_reference = global;
        self.signal_local_reference.clear();
        self.signal_local_reference.extend_from_slice(local);
        Ok(())
    }

    #[inline]
    fn predict_next_value(
        &self,
        prev: Value,
        prev_prev: Value,
        prev_prev_prev: Value,
        dt: Value,
        method_order: u32,
    ) -> Value {
        if method_order >= 2 && self.history_count >= 3 && self.prev_prev_dt > 0.0 {
            // Evaluate the quadratic interpolant through the previous three
            // accepted points at the proposed next time. This keeps the LTE
            // estimate order-consistent with trapezoidal/Gear2 instead of
            // falling back to a first-order predictor that can over-restrict
            // smooth waveforms.
            let h1 = self.prev_dt;
            let h2 = self.prev_prev_dt;
            let t0 = 0.0;
            let t1 = -h1;
            let t2 = -(h1 + h2);
            let t = dt;

            let l0 = (t - t1) * (t - t2) / ((t0 - t1) * (t0 - t2));
            let l1 = (t - t0) * (t - t2) / ((t1 - t0) * (t1 - t2));
            let l2 = (t - t0) * (t - t1) / ((t2 - t0) * (t2 - t1));
            prev * l0 + prev_prev * l1 + prev_prev_prev * l2
        } else if self.history_count >= 2 && self.prev_dt > 0.0 {
            let slope = (prev - prev_prev) / self.prev_dt;
            prev + slope * dt
        } else {
            prev
        }
    }

    #[inline]
    fn predict_trapezoidal_order1_value(&self, prev: Value, prev_prev: Value, dt: Value) -> Value {
        if self.history_count >= 2 && self.prev_dt > 0.0 {
            prev + dt * (prev - prev_prev) / self.prev_dt
        } else {
            prev
        }
    }

    #[inline]
    fn predict_trapezoidal_order2_value(
        &self,
        prev: Value,
        prev_prev: Value,
        prev_prev_prev: Value,
        dt: Value,
    ) -> Value {
        if self.history_count >= 3 && self.prev_dt > 0.0 && self.prev_prev_dt > 0.0 {
            let dd0 = (prev - prev_prev) / self.prev_dt;
            let dd1 = (prev_prev - prev_prev_prev) / self.prev_prev_dt;
            let b = -dt / (2.0 * self.prev_dt);
            let a = 1.0 - b;
            prev + (b * dd1 + a * dd0) * dt
        } else {
            self.predict_trapezoidal_order1_value(prev, prev_prev, dt)
        }
    }

    /// Predict the next solution vector prefix from accepted history.
    ///
    /// The predicted vector is initialized from the most recently accepted
    /// solution, then the first `predicted_len` entries are extrapolated.
    /// This mirrors ngspice's `NIpred` behavior for trapezoidal integration and
    /// falls back to order-consistent polynomial extrapolation for Gear methods.
    pub fn predict_solution_prefix(
        &self,
        dt: Value,
        method: IntegrationMethod,
        trap_order: u8,
        predicted_len: usize,
    ) -> Option<Vec<Value>> {
        if self.history_count == 0 || self.prev_solution.is_empty() || !dt.is_finite() || dt <= 0.0
        {
            return None;
        }

        let mut predicted = self.prev_solution.clone();
        let predicted_len = predicted_len.min(predicted.len());
        let method_order = Self::integration_method_order(method, trap_order);
        for (idx, value) in predicted.iter_mut().enumerate().take(predicted_len) {
            let prev = self.prev_solution[idx];
            let prev_prev = self.prev_prev_solution.get(idx).copied().unwrap_or(prev);
            let prev_prev_prev = self
                .prev_prev_prev_solution
                .get(idx)
                .copied()
                .unwrap_or(prev_prev);
            *value = match method {
                IntegrationMethod::BackwardEuler => {
                    self.predict_trapezoidal_order1_value(prev, prev_prev, dt)
                }
                IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear => {
                    if trap_order >= 2 {
                        self.predict_trapezoidal_order2_value(prev, prev_prev, prev_prev_prev, dt)
                    } else {
                        self.predict_trapezoidal_order1_value(prev, prev_prev, dt)
                    }
                }
                IntegrationMethod::Gear2 => {
                    self.predict_next_value(prev, prev_prev, prev_prev_prev, dt, method_order)
                }
            };
        }

        Some(predicted)
    }

    /// Predict the next full solution vector from accepted history.
    pub fn predict_solution(
        &self,
        dt: Value,
        method: IntegrationMethod,
        trap_order: u8,
    ) -> Option<Vec<Value>> {
        self.predict_solution_prefix(dt, method, trap_order, usize::MAX)
    }

    /// Set the integration method order for accurate timestep scaling
    /// - BackwardEuler: order 1
    /// - Trapezoidal, Gear2: order 2
    #[inline]
    pub fn set_method_order(&mut self, order: u32) {
        self.method_order = order.max(1);
    }

    #[inline]
    fn integration_method_order(method: IntegrationMethod, trap_order: u8) -> u32 {
        match method {
            IntegrationMethod::BackwardEuler => 1,
            IntegrationMethod::Trapezoidal
            | IntegrationMethod::TrapGear
            | IntegrationMethod::Gear2
                if trap_order <= 1 =>
            {
                1
            }
            IntegrationMethod::Trapezoidal
            | IntegrationMethod::Gear2
            | IntegrationMethod::TrapGear => 2,
        }
    }

    /// Whether this estimator uses Xyce accepted-solution weighting and WRMS.
    pub fn uses_accepted_solution_reference(&self) -> bool {
        self.reference != TransientLteReference::PredictorLocal
    }

    pub(crate) fn requires_signal_reference_history(&self) -> bool {
        matches!(
            self.reference,
            TransientLteReference::SignalGlobal | TransientLteReference::SignalLocal
        )
    }

    pub(crate) fn reference_mode(&self) -> TransientLteReference {
        self.reference
    }

    /// Record a solution point for history and reference weighting.
    pub fn record(&mut self, solution: &[Value], dt: Value) {
        self.record_prefix(solution, solution.len(), dt);
    }

    /// Record a solution while limiting LTE reference state to a vector prefix.
    ///
    /// Transient solution vectors append algebraic branch-current unknowns after
    /// node voltages. Callers that apply generic voltage LTE to only the nodal
    /// prefix should use the same prefix here so mixed-unit branch magnitudes do
    /// not relax voltage error weights.
    pub fn record_prefix(&mut self, solution: &[Value], prefix_len: usize, dt: Value) {
        self.update_signal_reference_prefix(solution, prefix_len);

        // Shift history: prev_prev_prev <- prev_prev <- prev <- new
        self.prev_prev_prev_solution = std::mem::take(&mut self.prev_prev_solution);
        self.prev_prev_solution = std::mem::take(&mut self.prev_solution);
        self.prev_solution = solution.to_vec();
        self.prev_prev_dt = self.prev_dt;
        self.prev_dt = dt;
        if self.history_count < 3 {
            self.history_count += 1;
        }
    }

    /// Estimate LTE using linear extrapolation vs actual value
    /// Returns (lte_estimate, should_accept)
    pub fn estimate(&self, current: &[Value], dt: Value) -> (Value, bool) {
        self.estimate_prefix(current, current.len(), dt)
    }

    /// Estimate LTE for the first `prefix_len` solution entries.
    ///
    /// Transient Newton vectors include algebraic branch-current unknowns for
    /// voltage sources and inductors. Those currents can legitimately jump to
    /// enforce KCL around capacitive edges, so generic LTE control should be
    /// applied to dynamic node-voltage state unless a device-local charge
    /// controller is being used.
    pub fn estimate_prefix(
        &self,
        current: &[Value],
        prefix_len: usize,
        dt: Value,
    ) -> (Value, bool) {
        self.estimate_prefix_excluding(current, prefix_len, dt, &[])
    }

    /// Estimate LTE for the first `prefix_len` solution entries, skipping
    /// sorted zero-based indices owned by model-specific timestep semantics.
    pub fn estimate_prefix_excluding(
        &self,
        current: &[Value],
        prefix_len: usize,
        dt: Value,
        excluded_indices: &[usize],
    ) -> (Value, bool) {
        self.estimate_prefix_excluding_with_coefficient(
            current,
            None,
            prefix_len,
            dt,
            excluded_indices,
            1.0,
            self.method_order,
        )
    }

    /// Estimate LTE using Xyce's integration-method correction coefficient.
    ///
    /// Xyce multiplies the weighted RMS predictor correction by `ck`: for
    /// one-step order 1 it is `h/(h+h_prev)`, for trapezoidal order 2 it is
    /// one third of that value, and for Gear order 2 it is
    /// `h/(h+h_prev+h_prev_prev)`. Native predictor-local control preserves its
    /// established uncorrected metric.
    pub(crate) fn estimate_prefix_excluding_for_integration(
        &self,
        current: &[Value],
        prefix_len: usize,
        dt: Value,
        excluded_indices: &[usize],
        method: IntegrationMethod,
        trap_order: u8,
    ) -> (Value, bool) {
        let coefficient = self.integration_error_coefficient(method, trap_order, dt);
        let method_order = Self::integration_method_order(method, trap_order);
        self.estimate_prefix_excluding_with_coefficient(
            current,
            None,
            prefix_len,
            dt,
            excluded_indices,
            coefficient,
            method_order,
        )
    }

    /// Estimate LTE from the exact predictor vector used to seed Newton.
    pub(crate) fn estimate_correction_prefix_excluding_for_integration(
        &self,
        current: &[Value],
        predicted: &[Value],
        prefix_len: usize,
        dt: Value,
        excluded_indices: &[usize],
        method: IntegrationMethod,
        trap_order: u8,
    ) -> (Value, bool) {
        let coefficient = self.integration_error_coefficient(method, trap_order, dt);
        let method_order = Self::integration_method_order(method, trap_order);
        self.estimate_prefix_excluding_with_coefficient(
            current,
            Some(predicted),
            prefix_len,
            dt,
            excluded_indices,
            coefficient,
            method_order,
        )
    }

    fn integration_error_coefficient(
        &self,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
    ) -> Value {
        if !self.uses_accepted_solution_reference()
            || !dt.is_finite()
            || dt <= 0.0
            || !self.prev_dt.is_finite()
            || self.prev_dt <= 0.0
        {
            return 1.0;
        }

        let one_step = dt / (dt + self.prev_dt);
        match method {
            IntegrationMethod::BackwardEuler => one_step,
            IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear => {
                if trap_order >= 2 {
                    one_step / 3.0
                } else {
                    one_step
                }
            }
            IntegrationMethod::Gear2 => {
                if trap_order >= 2
                    && self.history_count >= 2
                    && self.prev_prev_dt.is_finite()
                    && self.prev_prev_dt > 0.0
                {
                    dt / (dt + self.prev_dt + self.prev_prev_dt)
                } else {
                    one_step
                }
            }
        }
    }

    fn estimate_prefix_excluding_with_coefficient(
        &self,
        current: &[Value],
        predicted_solution: Option<&[Value]>,
        prefix_len: usize,
        dt: Value,
        excluded_indices: &[usize],
        error_coefficient: Value,
        method_order: u32,
    ) -> (Value, bool) {
        let len = prefix_len.min(current.len());
        if predicted_solution.is_some_and(|predicted| predicted.len() < len) {
            return (Value::INFINITY, false);
        }
        // Need at least one previous point to estimate
        if len == 0 || self.history_count < 1 || self.prev_solution.len() < len {
            return (0.0, true); // Accept, no history yet
        }

        let mut aggregate = 0.0_f64;
        let accepted_point_global = match self.reference {
            TransientLteReference::PredictorLocal
            | TransientLteReference::PointLocal
            | TransientLteReference::SignalLocal => 0.0,
            TransientLteReference::PointGlobal | TransientLteReference::SignalGlobal => {
                let Some(reference) =
                    self.accepted_point_global_reference_prefix_excluding(len, excluded_indices)
                else {
                    return (Value::INFINITY, false);
                };
                reference
            }
        };

        // For trapezoidal, LTE ~ (dt^3 / 12) * d^3v/dt^3
        // We approximate by comparing predicted (linear extrapolation) vs actual
        for (i, &curr_val) in current.iter().take(len).enumerate() {
            if excluded_indices.binary_search(&i).is_ok() {
                continue;
            }

            let prev_val = self.prev_solution[i];
            let prev_prev_val = self.prev_prev_solution.get(i).copied().unwrap_or(prev_val);
            let prev_prev_prev_val = self
                .prev_prev_prev_solution
                .get(i)
                .copied()
                .unwrap_or(prev_prev_val);
            let predicted = predicted_solution.map_or_else(
                || {
                    self.predict_next_value(
                        prev_val,
                        prev_prev_val,
                        prev_prev_prev_val,
                        dt,
                        method_order,
                    )
                },
                |predicted| predicted[i],
            );
            if !curr_val.is_finite() || !predicted.is_finite() {
                return (Value::INFINITY, false);
            }

            // LTE estimate: |actual - predicted| with weighted SPICE-like scaling.
            let lte = (curr_val - predicted).abs();
            let reference = if self.reference == TransientLteReference::PredictorLocal {
                curr_val.abs().max(predicted.abs())
            } else {
                self.accepted_reference_magnitude(i, accepted_point_global)
            };
            let scale = self.lte_scale_denominator(reference);
            let normalized_lte = lte / scale;

            if self.reference == TransientLteReference::PredictorLocal {
                aggregate = aggregate.max(normalized_lte);
            } else {
                aggregate += normalized_lte * normalized_lte;
            }
        }

        let raw_lte = if self.reference == TransientLteReference::PredictorLocal {
            aggregate
        } else {
            // Xyce's device-mask entries receive effectively infinite weights,
            // so they contribute zero to the sum but remain in the WRMS global
            // vector length used as the denominator.
            (aggregate / len as Value).sqrt()
        };
        let lte = raw_lte * error_coefficient;
        let accept = lte <= self.reltol;
        (lte, accept)
    }

    /// Richardson extrapolation LTE estimate (more accurate)
    ///
    /// Uses solutions computed with steps h and h/2 to estimate the true error:
    /// `LTE ≈ (x_h - x_{h/2}) / (2^p - 1)` where p is the method order.
    ///
    /// This provides a more accurate LTE estimate by exploiting the known
    /// convergence order of the integration method.
    ///
    /// # Arguments
    /// * `x_full` - Solution computed with full timestep h
    /// * `x_half` - Solution computed with two half-steps h/2
    ///
    /// # Returns
    /// (lte_estimate, should_accept)
    pub fn richardson_estimate(&self, x_full: &[Value], x_half: &[Value]) -> (Value, bool) {
        if x_full.len() != x_half.len() || x_full.is_empty() {
            return (0.0, true);
        }

        let order_factor = (1u64 << self.method_order) as Value - 1.0; // 2^p - 1
        let mut aggregate = 0.0_f64;
        let accepted_point_global = match self.reference {
            TransientLteReference::PredictorLocal
            | TransientLteReference::PointLocal
            | TransientLteReference::SignalLocal => 0.0,
            TransientLteReference::PointGlobal | TransientLteReference::SignalGlobal => {
                let Some(reference) = self.accepted_point_global_reference() else {
                    return (0.0, true);
                };
                reference
            }
        };

        for (index, (&full, &half)) in x_full.iter().zip(x_half.iter()).enumerate() {
            if !full.is_finite() || !half.is_finite() {
                return (Value::INFINITY, false);
            }
            // Richardson extrapolation error estimate
            let richardson_error = (half - full).abs() / order_factor;

            let reference = if self.reference == TransientLteReference::PredictorLocal {
                half.abs().max(full.abs())
            } else {
                self.accepted_reference_magnitude(index, accepted_point_global)
            };
            let scale = self.lte_scale_denominator(reference);
            let normalized = richardson_error / scale;

            if self.reference == TransientLteReference::PredictorLocal {
                aggregate = aggregate.max(normalized);
            } else {
                aggregate += normalized * normalized;
            }
        }

        let lte = if self.reference == TransientLteReference::PredictorLocal {
            aggregate
        } else {
            (aggregate / x_full.len() as Value).sqrt()
        };
        let accept = lte <= self.reltol;
        (lte, accept)
    }

    /// Get recommended timestep scaling factor based on LTE
    /// Uses method order for proper scaling exponent
    pub fn recommend_scale(&self, lte: Value) -> Value {
        self.recommend_scale_for_order(lte, self.method_order)
    }

    /// Return the timestep scale for the method/order of the candidate step.
    pub(crate) fn recommend_scale_for_integration(
        &self,
        lte: Value,
        method: IntegrationMethod,
        trap_order: u8,
    ) -> Value {
        self.recommend_scale_for_order(lte, Self::integration_method_order(method, trap_order))
    }

    /// Xyce's successful-step ratio policy for accepted-solution LTE modes.
    ///
    /// Ratios at or above two grow by exactly two, ratios between one and two
    /// keep the current step, and ratios at or below one shrink within Xyce's
    /// `[0.25, 0.9]` bounds.
    pub(crate) fn xyce_accepted_step_scale(
        &self,
        lte: Value,
        method: IntegrationMethod,
        trap_order: u8,
    ) -> Value {
        let ratio =
            self.xyce_raw_step_ratio(lte, Self::integration_method_order(method, trap_order));
        if ratio >= 2.0 {
            2.0
        } else if ratio <= 1.0 {
            ratio.clamp(0.25, 0.9)
        } else {
            1.0
        }
    }

    /// Xyce's failed-LTE step ratio policy.
    pub(crate) fn xyce_rejected_step_scale(
        &self,
        lte: Value,
        method: IntegrationMethod,
        trap_order: u8,
        first_failure: bool,
    ) -> Value {
        if !first_failure {
            return 0.25;
        }
        self.xyce_raw_step_ratio(lte, Self::integration_method_order(method, trap_order))
            .clamp(0.25, 0.9)
    }

    /// Whether Xyce's order-2 promotion test passes for the current error.
    pub(crate) fn xyce_should_promote_order_two(&self, lte: Value) -> bool {
        self.history_count >= 2 && self.xyce_raw_step_ratio(lte, 2) > 1.05
    }

    fn xyce_raw_step_ratio(&self, lte: Value, method_order: u32) -> Value {
        if !lte.is_finite() || lte < 0.0 {
            return 0.25;
        }
        let est_over_tol = lte / self.reltol.max(1.0e-30);
        (0.5 / (est_over_tol + 0.0001)).powf(1.0 / (method_order.max(1) as Value + 1.0))
    }

    fn recommend_scale_for_order(&self, lte: Value, method_order: u32) -> Value {
        if !lte.is_finite() {
            0.25
        } else if lte < 1e-15 {
            2.0 // Error negligible, can increase
        } else {
            // Optimal scaling: (tol/lte)^(1/(p+1)) where p is method order
            // For order 2: exponent = 1/3, for order 1: exponent = 1/2
            let exponent = 1.0 / (method_order.max(1) as Value + 1.0);
            let ratio = self.reltol / lte;
            ratio.powf(exponent).clamp(0.25, 2.0)
        }
    }

    /// Charge-based LTE estimation for capacitive elements (MOSFET gates)
    ///
    /// For charge-conserving models (BSIM4, etc.), the fundamental relationship is:
    /// ```text
    /// dQ/dt = I
    /// ```
    ///
    /// This method estimates LTE based on charge conservation errors rather than
    /// voltage errors. This is more accurate for capacitive elements because:
    /// 1. It directly tracks the physical quantity being integrated (charge)
    /// 2. It catches errors in current integration that voltage-based LTE might miss
    /// 3. It ensures charge conservation across the simulation
    ///
    /// # Arguments
    /// * `charges` - Current charge values at each node
    /// * `prev_charges` - Charge values from previous timestep
    /// * `currents` - Current values (dQ/dt) at each node
    /// * `dt` - Current timestep
    ///
    /// # Returns
    /// (max_charge_lte, should_accept)
    ///
    /// # Theory
    /// For trapezoidal integration: Q_{n+1} = Q_n + dt/2 * (I_n + I_{n+1})
    /// The LTE is estimated as: |Q_actual - Q_predicted| / |Q_actual|
    pub fn estimate_charge_lte(
        &self,
        charges: &[Value],
        prev_charges: &[Value],
        currents: &[Value],
        prev_currents: &[Value],
        dt: Value,
    ) -> (Value, bool) {
        if charges.len() != prev_charges.len() || charges.is_empty() {
            return (0.0, true);
        }

        if charges.len() != currents.len() || charges.len() != prev_currents.len() {
            return (0.0, true);
        }

        let mut max_lte = 0.0_f64;

        // For each charge node, check conservation error
        for i in 0..charges.len() {
            let q_curr = charges[i];
            let q_prev = prev_charges[i];
            let i_curr = currents[i];
            let i_prev = prev_currents[i];

            // Trapezoidal prediction: Q_n+1 = Q_n + dt/2 * (I_n + I_{n+1})
            let q_predicted = q_prev + dt * 0.5 * (i_prev + i_curr);

            // Charge conservation error
            let charge_error = (q_curr - q_predicted).abs();

            // Normalize by charge magnitude (with minimum to avoid division issues)
            let q_magnitude = q_curr.abs().max(q_prev.abs()).max(1e-18);
            let normalized_lte = charge_error / q_magnitude;

            max_lte = max_lte.max(normalized_lte);
        }

        let accept = max_lte <= self.reltol;
        (max_lte, accept)
    }

    /// Combined voltage and charge LTE estimation
    ///
    /// Uses both voltage-based and charge-based LTE to determine timestep.
    /// Takes the maximum of both to ensure both voltages and charges are
    /// accurately integrated.
    ///
    /// # Arguments
    /// * `voltages` - Current node voltages
    /// * `dt` - Current timestep
    /// * `charges` - Gate/junction charges (if available)
    /// * `prev_charges` - Previous gate/junction charges
    /// * `currents` - Node currents
    /// * `prev_currents` - Previous node currents
    pub fn estimate_combined(
        &self,
        voltages: &[Value],
        dt: Value,
        charges: Option<ChargeLteInputs<'_>>,
    ) -> (Value, bool) {
        // Standard voltage-based LTE
        let (v_lte, v_accept) = self.estimate(voltages, dt);

        // Charge-based LTE if charge data provided
        let (combined_lte, combined_accept) = if let Some((q_curr, q_prev, i_curr, i_prev)) =
            charges
        {
            let (q_lte, q_accept) = self.estimate_charge_lte(q_curr, q_prev, i_curr, i_prev, dt);
            (v_lte.max(q_lte), v_accept && q_accept)
        } else {
            (v_lte, v_accept)
        };

        (combined_lte, combined_accept)
    }

    /// Restart predictor history while retaining signal-history LTE references.
    pub fn restart_history(&mut self) {
        self.prev_solution.clear();
        self.prev_prev_solution.clear();
        self.prev_prev_prev_solution.clear();
        self.prev_dt = 0.0;
        self.prev_prev_dt = 0.0;
        self.history_count = 0;
    }

    /// Restart predictor history at an accepted breakpoint while retaining
    /// signal-history reference maxima.
    pub fn restart_history_from(&mut self, accepted_solution: &[Value]) {
        self.restart_history();
        self.prev_solution.extend_from_slice(accepted_solution);
        self.history_count = 1;
    }

    /// Seed the synthetic equal-step history used by a Xyce integration restart.
    ///
    /// OneStep/Gear12 restart with a zero first-difference vector, but initialize
    /// the previous step size to the first attempted post-breakpoint step. This
    /// makes the first predictor equal to the accepted breakpoint solution while
    /// retaining the correct `h / (h + h_prev)` LTE coefficient. Retries keep the
    /// original seed, matching Xyce's `lastTimeStep` restart state.
    pub(crate) fn seed_restart_timestep(&mut self, dt: Value) {
        if self.history_count == 1 && self.prev_dt == 0.0 && dt.is_finite() && dt > 0.0 {
            self.prev_dt = dt;
        }
    }

    /// Reset predictor history and all historical LTE references.
    pub fn reset(&mut self) {
        self.restart_history();
        self.accepted_reference_solution.clear();
        self.signal_global_reference = 0.0;
        self.signal_local_reference.clear();
    }
}

#[cfg(test)]
mod lte_estimator_tests {
    use super::*;

    fn accepted_estimator(
        reference: TransientLteReference,
        initial: &[Value],
        accepted: &[Value],
    ) -> LteEstimator {
        let mut estimator = LteEstimator::with_tolerances_and_reference(0.1, 1.0e-6, reference);
        estimator.seed_reference_prefix(initial, initial.len());
        estimator.record(accepted, 1.0);
        estimator
    }

    #[test]
    fn candidate_magnitude_cannot_relax_accepted_solution_weights() {
        let estimator = accepted_estimator(TransientLteReference::SignalGlobal, &[1.0], &[1.0]);
        let (small_lte, _) = estimator.estimate(&[2.0], 1.0);
        let (large_lte, _) = estimator.estimate(&[200.0], 1.0);

        assert!(large_lte > small_lte * 100.0);
        assert_eq!(
            estimator.signal_global_reference.to_bits(),
            1.0f64.to_bits()
        );
        assert_eq!(estimator.accepted_reference_solution, [1.0]);
    }

    #[test]
    fn signal_global_remembers_a_peak_that_point_global_discards() {
        let point = accepted_estimator(TransientLteReference::PointGlobal, &[100.0], &[1.0]);
        let signal = accepted_estimator(TransientLteReference::SignalGlobal, &[100.0], &[1.0]);

        let (_, point_accepts) = point.estimate(&[1.2], 1.0);
        let (_, signal_accepts) = signal.estimate(&[1.2], 1.0);

        assert!(!point_accepts);
        assert!(signal_accepts);
    }

    #[test]
    fn signal_local_retains_independent_per_variable_peaks() {
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::SignalLocal,
        );
        estimator.seed_reference_prefix(&[10.0, 1.0], 2);
        estimator.record(&[2.0, 20.0], 1.0);
        estimator.record(&[3.0, 4.0], 1.0);

        assert_eq!(estimator.signal_local_reference, [10.0, 20.0]);
        assert_eq!(estimator.accepted_reference_solution, [3.0, 4.0]);
    }

    #[test]
    fn point_local_uses_each_prior_accepted_state_magnitude() {
        let estimator = accepted_estimator(
            TransientLteReference::PointLocal,
            &[1.0, 100.0],
            &[1.0, 100.0],
        );
        let candidate = [1.1, 100.1];

        let (first_lte, _) = estimator.estimate_prefix_excluding(&candidate, 2, 1.0, &[1]);
        let (second_lte, _) = estimator.estimate_prefix_excluding(&candidate, 2, 1.0, &[0]);

        assert!(first_lte > second_lte * 50.0);
    }

    #[test]
    fn accepted_solution_modes_include_the_full_solution_vector_in_wrms() {
        let estimator = accepted_estimator(
            TransientLteReference::SignalGlobal,
            &[1.0, 1.0],
            &[1.0, 1.0],
        );
        let candidate = [1.0, 1.2];

        let (nodal_prefix_lte, _) = estimator.estimate_prefix(&candidate, 1, 1.0);
        let (full_lte, _) = estimator.estimate(&candidate, 1.0);

        assert_eq!(nodal_prefix_lte, 0.0);
        assert!(full_lte > 0.0);
    }

    #[test]
    fn xyce_device_mask_zeroes_error_but_keeps_global_wrms_length() {
        let estimator = accepted_estimator(
            TransientLteReference::SignalGlobal,
            &[1.0, 100.0],
            &[1.0, 100.0],
        );

        let (lte, accepts) = estimator.estimate_correction_prefix_excluding_for_integration(
            &[1.0, 200.0],
            &[1.0, 100.0],
            2,
            1.0,
            &[1],
            IntegrationMethod::Trapezoidal,
            2,
        );

        assert_eq!(lte, 0.0);
        assert!(accepts);
    }

    #[test]
    fn xyce_device_mask_excludes_algebraic_values_from_point_global_weight() {
        let estimator = accepted_estimator(
            TransientLteReference::PointGlobal,
            &[1.0, 1000.0],
            &[1.0, 1000.0],
        );

        let (masked_lte, _) = estimator.estimate_correction_prefix_excluding_for_integration(
            &[1.6, 1000.0],
            &[1.0, 1000.0],
            2,
            1.0,
            &[1],
            IntegrationMethod::Trapezoidal,
            2,
        );
        let (unmasked_lte, _) = estimator.estimate_correction_prefix_excluding_for_integration(
            &[1.6, 1000.0],
            &[1.0, 1000.0],
            2,
            1.0,
            &[],
            IntegrationMethod::Trapezoidal,
            2,
        );

        assert!(masked_lte > unmasked_lte * 100.0);
    }

    #[test]
    fn lte_uses_the_exact_predictor_vector_that_seeded_newton() {
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::PointGlobal,
        );
        estimator.seed_initial_solution(&[0.0]);
        estimator.record(&[1.0], 1.0);

        let (lte, accepts) = estimator.estimate_correction_prefix_excluding_for_integration(
            &[123.0],
            &[123.0],
            1,
            1.0,
            &[],
            IntegrationMethod::Trapezoidal,
            2,
        );

        assert_eq!(lte, 0.0);
        assert!(accepts);
    }

    #[test]
    fn initial_state_seeds_reference_and_rejection_does_not_mutate_it() {
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::SignalGlobal,
        );
        estimator.seed_reference_prefix(&[1.0, -3.0], 2);
        estimator.record(&[0.5, -2.0], 1.0);
        let before = estimator.signal_reference_snapshot().0;

        let (_, accepts) = estimator.estimate(&[Value::INFINITY, 1000.0], 1.0);

        assert!(!accepts);
        assert_eq!(before.to_bits(), 3.0f64.to_bits());
        assert_eq!(
            estimator.signal_reference_snapshot().0.to_bits(),
            before.to_bits()
        );
        assert_eq!(estimator.accepted_reference_solution, [0.5, -2.0]);
    }

    #[test]
    fn initial_state_seeds_predictor_history_for_the_second_step() {
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::PointGlobal,
        );
        estimator.seed_initial_solution(&[0.0]);
        estimator.record(&[1.0], 1.0);

        let (lte, accepts) = estimator.estimate_prefix_excluding_for_integration(
            &[2.0],
            1,
            1.0,
            &[],
            IntegrationMethod::BackwardEuler,
            1,
        );

        assert_eq!(lte, 0.0);
        assert!(accepts);
    }

    #[test]
    fn restart_preserves_signal_reference_while_reset_clears_it() {
        let mut estimator = accepted_estimator(TransientLteReference::SignalGlobal, &[5.0], &[1.0]);

        estimator.restart_history();
        assert_eq!(estimator.history_count, 0);
        assert_eq!(
            estimator.signal_global_reference.to_bits(),
            5.0f64.to_bits()
        );
        assert_eq!(estimator.accepted_reference_solution, [1.0]);

        estimator.restart_history_from(&[1.0]);
        assert_eq!(estimator.history_count, 1);
        assert_eq!(estimator.prev_solution, [1.0]);
        assert_eq!(
            estimator.signal_global_reference.to_bits(),
            5.0f64.to_bits()
        );

        estimator.reset();
        assert_eq!(estimator.signal_global_reference, 0.0);
        assert!(estimator.accepted_reference_solution.is_empty());
    }

    #[test]
    fn xyce_restart_timestep_seed_is_stable_across_retries() {
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::PointGlobal,
        );
        estimator.seed_reference_prefix(&[7.0, -2.0], 2);
        estimator.restart_history_from(&[7.0, -2.0]);

        estimator.seed_restart_timestep(0.25);
        estimator.seed_restart_timestep(0.125);

        assert_eq!(estimator.prev_dt.to_bits(), 0.25f64.to_bits());
        assert_eq!(
            estimator
                .predict_solution(0.125, IntegrationMethod::BackwardEuler, 1)
                .expect("restart history must predict from the accepted breakpoint"),
            [7.0, -2.0]
        );
        assert_eq!(
            estimator
                .integration_error_coefficient(IntegrationMethod::BackwardEuler, 1, 0.125)
                .to_bits(),
            (1.0f64 / 3.0).to_bits()
        );
    }

    #[test]
    fn predictor_local_metric_retains_legacy_candidate_scaling() {
        let mut estimator = LteEstimator::with_tolerances(0.1, 0.01);
        estimator.record(&[1.0], 1.0);

        let (lte, accepts) = estimator.estimate(&[2.0], 1.0);

        assert!((lte - 1.0 / 2.1).abs() <= 1.0e-15);
        assert!(!accepts);
    }

    #[test]
    fn non_finite_candidate_fails_closed() {
        let estimator = accepted_estimator(TransientLteReference::PointGlobal, &[1.0], &[1.0]);

        let (lte, accepts) = estimator.estimate(&[Value::NAN], 1.0);

        assert!(lte.is_infinite());
        assert!(!accepts);
    }

    #[test]
    fn timeint_absolute_tolerance_changes_the_weight_and_decision() {
        let mut strict = LteEstimator::with_tolerances_and_reference(
            0.1,
            0.001,
            TransientLteReference::PointGlobal,
        );
        let mut loose = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0,
            TransientLteReference::PointGlobal,
        );
        strict.seed_reference_prefix(&[0.0], 1);
        loose.seed_reference_prefix(&[0.0], 1);
        strict.record(&[0.0], 1.0);
        loose.record(&[0.0], 1.0);

        assert!(!strict.estimate(&[0.05], 1.0).1);
        assert!(loose.estimate(&[0.05], 1.0).1);
    }

    #[test]
    fn xyce_integration_error_coefficients_follow_method_and_step_history() {
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::PointGlobal,
        );
        estimator.seed_reference_prefix(&[0.0], 1);
        estimator.record(&[0.0], 2.0);

        assert_eq!(
            estimator
                .integration_error_coefficient(IntegrationMethod::BackwardEuler, 1, 1.0)
                .to_bits(),
            (1.0f64 / 3.0).to_bits()
        );
        assert_eq!(
            estimator
                .integration_error_coefficient(IntegrationMethod::Trapezoidal, 2, 1.0)
                .to_bits(),
            (1.0f64 / 9.0).to_bits()
        );

        estimator.record(&[0.0], 3.0);
        assert_eq!(
            estimator
                .integration_error_coefficient(IntegrationMethod::Gear2, 2, 1.0)
                .to_bits(),
            (1.0f64 / 6.0).to_bits()
        );

        let mut native = LteEstimator::with_tolerances(0.1, 1.0e-6);
        native.record(&[0.0], 2.0);
        assert_eq!(
            native.integration_error_coefficient(IntegrationMethod::Trapezoidal, 2, 1.0),
            1.0
        );
    }

    #[test]
    fn candidate_method_order_controls_prediction_and_scale_exponent() {
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::SignalGlobal,
        );
        estimator.seed_reference_prefix(&[0.0], 1);
        estimator.record(&[0.0], 1.0);
        estimator.record(&[1.0], 1.0);
        estimator.record(&[4.0], 1.0);

        let (order_one_lte, _) = estimator.estimate_prefix_excluding_for_integration(
            &[9.0],
            1,
            1.0,
            &[],
            IntegrationMethod::BackwardEuler,
            1,
        );
        let (order_two_lte, _) = estimator.estimate_prefix_excluding_for_integration(
            &[9.0],
            1,
            1.0,
            &[],
            IntegrationMethod::Trapezoidal,
            2,
        );

        assert!(order_one_lte > 0.0);
        assert_eq!(order_two_lte, 0.0);
        let order_one_scale =
            estimator.recommend_scale_for_integration(0.4, IntegrationMethod::BackwardEuler, 1);
        let order_two_scale =
            estimator.recommend_scale_for_integration(0.4, IntegrationMethod::Trapezoidal, 2);
        assert!((order_one_scale - 0.5).abs() <= 1.0e-15);
        assert!((order_two_scale - 0.25f64.powf(1.0 / 3.0)).abs() <= 1.0e-15);
    }

    #[test]
    fn xyce_step_ratio_policy_matches_success_and_failure_bounds() {
        let estimator = LteEstimator::with_tolerances_and_reference(
            0.1,
            1.0e-6,
            TransientLteReference::SignalGlobal,
        );

        assert_eq!(
            estimator.xyce_accepted_step_scale(1.0e-4, IntegrationMethod::Trapezoidal, 2),
            2.0
        );
        assert_eq!(
            estimator.xyce_accepted_step_scale(0.01, IntegrationMethod::Trapezoidal, 2),
            1.0
        );

        let boundary_scale =
            estimator.xyce_accepted_step_scale(0.1, IntegrationMethod::Trapezoidal, 2);
        let expected = (0.5f64 / 1.0001).powf(1.0 / 3.0);
        assert!((boundary_scale - expected).abs() <= 1.0e-15);

        let first_reject =
            estimator.xyce_rejected_step_scale(0.2, IntegrationMethod::Trapezoidal, 2, true);
        assert!(first_reject > 0.25 && first_reject < 0.9);
        assert_eq!(
            estimator.xyce_rejected_step_scale(0.2, IntegrationMethod::Trapezoidal, 2, false),
            0.25
        );
    }
}

//=============================================================================
// Companion Model Coefficients for Integration Methods
//=============================================================================

/// Companion model coefficients for numerical integration
///
/// Each integration method converts differential elements (C, L) into
/// equivalent conductances and current/voltage sources using these coefficients.
#[derive(Debug, Clone, Copy)]
pub struct CompanionCoefficients {
    /// Equivalent conductance coefficient: G_eq = coeff_g * C / dt
    pub coeff_g: Value,
    /// History coefficient for v_n (most recent)
    pub coeff_v_n: Value,
    /// History coefficient for v_{n-1}
    pub coeff_v_n_minus_1: Value,
    /// Whether v_{n-1} history is needed
    pub needs_two_history: bool,
    /// Whether i_n current history is needed (Trapezoidal)
    pub needs_current_history: bool,
}

impl CompanionCoefficients {
    /// Get coefficients for Backward Euler (first order, unconditionally stable)
    ///
    /// C·dv/dt = i  →  C·(v_{n+1} - v_n)/dt = i_{n+1}
    /// Companion: G_eq = C/dt, I_eq = G_eq·v_n
    #[inline]
    pub fn backward_euler() -> Self {
        Self {
            coeff_g: 1.0,
            coeff_v_n: 1.0,
            coeff_v_n_minus_1: 0.0,
            needs_two_history: false,
            needs_current_history: false,
        }
    }

    /// Get coefficients for Trapezoidal rule (second order, A-stable)
    ///
    /// Uses average of derivatives at n and n+1:
    /// C·(v_{n+1} - v_n)/dt = 0.5·(i_{n+1} + i_n)
    /// Companion: G_eq = 2C/dt, I_eq = G_eq·v_n + i_n
    #[inline]
    pub fn trapezoidal() -> Self {
        Self {
            coeff_g: 2.0,
            coeff_v_n: 2.0,
            coeff_v_n_minus_1: 0.0,
            needs_two_history: false,
            needs_current_history: true,
        }
    }

    /// Get coefficients for Gear2/BDF2 (second order, L-stable, good for stiff)
    ///
    /// Uses backward difference formula:
    /// (3·v_{n+1} - 4·v_n + v_{n-1}) / (2·dt) = f_{n+1}
    /// Companion: G_eq = 3C/(2·dt), I_eq = (4C·v_n - C·v_{n-1})/(2·dt)
    #[inline]
    pub fn gear2() -> Self {
        Self {
            coeff_g: 1.5,            // 3/2
            coeff_v_n: 2.0,          // 4/2 = 2
            coeff_v_n_minus_1: -0.5, // -1/2
            needs_two_history: true,
            needs_current_history: false,
        }
    }

    /// Get variable-step Gear2/BDF2 coefficients.
    ///
    /// For the current step `h` and the previously accepted step `h_prev`,
    /// differentiating the quadratic interpolant through the current and two
    /// preceding solution points gives
    ///
    /// `x' = (a0*x[n+1] - a1*x[n] - a2*x[n-1]) / h`,
    ///
    /// where, for `r = h / h_prev`, `a0 = (1 + 2r)/(1 + r)`,
    /// `a1 = 1 + r`, and `a2 = -r^2/(1 + r)`. The equal-step case therefore
    /// reduces exactly to the ordinary `(3/2, 2, -1/2)` BDF2 coefficients.
    #[inline]
    pub fn gear2_variable_step(dt: Value, previous_dt: Value) -> Self {
        if !dt.is_finite() || dt <= 0.0 || !previous_dt.is_finite() || previous_dt <= 0.0 {
            return Self::backward_euler();
        }

        let ratio = dt / previous_dt;
        if !ratio.is_finite() || ratio <= 0.0 {
            return Self::backward_euler();
        }
        let denominator = 1.0 + ratio;
        let coeff_g = (1.0 + 2.0 * ratio) / denominator;
        let coeff_v_n = 1.0 + ratio;
        let coeff_v_n_minus_1 = -(ratio * ratio) / denominator;
        if !coeff_g.is_finite() || !coeff_v_n.is_finite() || !coeff_v_n_minus_1.is_finite() {
            return Self::backward_euler();
        }

        Self {
            coeff_g,
            coeff_v_n,
            coeff_v_n_minus_1,
            needs_two_history: true,
            needs_current_history: false,
        }
    }

    /// Get coefficients for the specified integration method
    #[inline]
    pub fn for_method(method: IntegrationMethod) -> Self {
        match method {
            IntegrationMethod::BackwardEuler => Self::backward_euler(),
            IntegrationMethod::Trapezoidal => Self::trapezoidal(),
            IntegrationMethod::Gear2 => Self::gear2(),
            IntegrationMethod::TrapGear => Self::trapezoidal(), // Default, actual method chosen dynamically
        }
    }

    /// Get coefficients for a method using the accepted timestep history.
    #[inline]
    pub fn for_method_with_previous_step(
        method: IntegrationMethod,
        dt: Value,
        previous_dt: Value,
    ) -> Self {
        match method {
            IntegrationMethod::Gear2 => Self::gear2_variable_step(dt, previous_dt),
            _ => Self::for_method(method),
        }
    }

    /// Calculate equivalent conductance for a capacitor
    #[inline]
    pub fn capacitor_geq(&self, capacitance: Value, dt: Value) -> Value {
        self.coeff_g * capacitance / dt
    }

    /// Calculate equivalent current source for a capacitor
    /// v_n is current voltage, v_n_minus_1 is previous voltage (for Gear2)
    #[inline]
    pub fn capacitor_ieq(
        &self,
        capacitance: Value,
        dt: Value,
        v_n: Value,
        v_n_minus_1: Value,
        i_n: Value,
    ) -> Value {
        let mut ieq = self.coeff_v_n * capacitance * v_n / dt;
        if self.needs_two_history {
            ieq += self.coeff_v_n_minus_1 * capacitance * v_n_minus_1 / dt;
        }
        if self.needs_current_history {
            ieq += i_n;
        }
        ieq
    }

    /// Calculate equivalent resistance for an inductor
    #[inline]
    pub fn inductor_req(&self, inductance: Value, dt: Value) -> Value {
        self.coeff_g * inductance / dt
    }

    /// Evaluate the charge-history part of an inductor Newton correction in
    /// DAE form.
    ///
    /// Forming `Q=L*i` at each state and differencing those charges before the
    /// timestep division mirrors production SPICE time integrators. It avoids
    /// constructing and then cancelling the much larger absolute companion
    /// terms `R_eq*i`, while keeping the arithmetic order of the underlying
    /// DAE instead of relying on an algebraically equivalent fused expression.
    #[inline]
    pub fn inductor_charge_derivative_correction(
        &self,
        inductance: Value,
        dt: Value,
        current: Value,
        current_prev: Value,
        current_prev_prev: Value,
    ) -> Value {
        let charge = inductance * current;
        let charge_prev = inductance * current_prev;
        let first_difference = charge - charge_prev;
        if self.needs_two_history {
            let charge_prev_prev = inductance * current_prev_prev;
            let previous_difference = charge_prev - charge_prev_prev;
            (self.coeff_g * first_difference + self.coeff_v_n_minus_1 * previous_difference) / dt
        } else {
            self.coeff_g * (first_difference / dt)
        }
    }

    /// Calculate the equivalent voltage-source magnitude for an inductor.
    ///
    /// Exact dual of [`Self::capacitor_ieq`] (v <-> i, C <-> L): the i_n
    /// history term uses `coeff_v_n` (NOT `coeff_g` — they differ for Gear2),
    /// the i_{n-1} term applies only when the method keeps two history points,
    /// and the conjugate-variable history v_n applies only for Trapezoidal
    /// (`needs_current_history`), never for BE/Gear2.
    ///
    /// The branch row is stamped as `v(np) - v(nn) - R_eq*i_{n+1} = -V_eq`,
    /// i.e. the stamp site negates this value, yielding:
    ///   BE:   v_{n+1} = (L/dt)*(i_{n+1} - i_n)
    ///   Trap: v_{n+1} = (2L/dt)*(i_{n+1} - i_n) - v_n
    ///   BDF2: v_{n+1} = (L/dt)*(1.5*i_{n+1} - 2*i_n + 0.5*i_{n-1})
    ///
    /// The previous formulation (`coeff_g*L*i_n/dt + v_n`, stamped without
    /// negation) made the companion recursion non-contractive: on a plain RL
    /// deck the branch state alternated sign each step, the TrapGear
    /// controller read that as ringing, and the error compounded ~2x per
    /// accepted step until node voltages crossed the +-1 kV sanity clamp and
    /// the stepper death-spiraled at femtosecond dt. See the
    /// `inductor_transient` integration tests for the analytic pins.
    #[inline]
    pub fn inductor_veq(
        &self,
        inductance: Value,
        dt: Value,
        i_n: Value,
        i_n_minus_1: Value,
        v_n: Value,
    ) -> Value {
        let mut veq = self.coeff_v_n * inductance * i_n / dt;
        if self.needs_two_history {
            veq += self.coeff_v_n_minus_1 * inductance * i_n_minus_1 / dt;
        }
        if self.needs_current_history {
            veq += v_n;
        }
        veq
    }
}

#[cfg(test)]
mod companion_coefficients_tests {
    use super::*;

    #[test]
    fn variable_step_gear2_reduces_to_fixed_bdf2_for_equal_steps() {
        let variable = CompanionCoefficients::gear2_variable_step(2.0, 2.0);
        let fixed = CompanionCoefficients::gear2();

        assert_eq!(variable.coeff_g, fixed.coeff_g);
        assert_eq!(variable.coeff_v_n, fixed.coeff_v_n);
        assert_eq!(variable.coeff_v_n_minus_1, fixed.coeff_v_n_minus_1);
        assert_eq!(variable.needs_two_history, fixed.needs_two_history);
        assert_eq!(variable.needs_current_history, fixed.needs_current_history);
    }

    #[test]
    fn variable_step_gear2_differentiates_an_affine_history_exactly() {
        let dt = 2.0;
        let previous_dt = 1.0;
        let slope = 3.0;
        let offset = 7.0;
        let inductance = 0.25;
        let i_prev_prev = offset - slope * previous_dt;
        let i_prev = offset;
        let i_curr = offset + slope * dt;
        let coefficients = CompanionCoefficients::gear2_variable_step(dt, previous_dt);

        assert!((coefficients.coeff_g - 5.0 / 3.0).abs() <= Value::EPSILON);
        assert!((coefficients.coeff_v_n - 3.0).abs() <= Value::EPSILON);
        assert!((coefficients.coeff_v_n_minus_1 + 4.0 / 3.0).abs() <= Value::EPSILON);

        let voltage = coefficients.inductor_req(inductance, dt) * i_curr
            - coefficients.inductor_veq(inductance, dt, i_prev, i_prev_prev, 0.0);
        assert!((voltage - inductance * slope).abs() <= 8.0 * Value::EPSILON);
    }

    #[test]
    fn variable_step_gear2_fails_safe_without_valid_step_history() {
        let backward_euler = CompanionCoefficients::backward_euler();

        for invalid_previous_dt in [0.0, -1.0, Value::NAN, Value::INFINITY] {
            let coefficients = CompanionCoefficients::gear2_variable_step(1.0, invalid_previous_dt);
            assert_eq!(coefficients.coeff_g, backward_euler.coeff_g);
            assert_eq!(coefficients.coeff_v_n, backward_euler.coeff_v_n);
            assert_eq!(
                coefficients.coeff_v_n_minus_1,
                backward_euler.coeff_v_n_minus_1
            );
            assert_eq!(
                coefficients.needs_two_history,
                backward_euler.needs_two_history
            );
        }
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

/// Statistics from TrapGear controller for debugging/monitoring
#[derive(Debug, Clone)]
pub struct TrapGearStats {
    pub current_method: IntegrationMethod,
    pub smooth_steps: usize,
    pub max_sign_changes: usize,
}
