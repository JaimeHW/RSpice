//! Transient Time-Domain Analysis

#![allow(clippy::type_complexity)]
use crate::Value;
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

    /// Check if timestep was rejected (too small)
    pub fn is_at_minimum(&self) -> bool {
        self.current_dt <= self.hard_min_dt * 1.001
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

/// Xyce-compatible scale for the first step after a source breakpoint.
///
/// Xyce 7.10 exposes this as `.OPTIONS TIMEINT RESTARTSTEPSCALE` and defaults
/// it to `0.005`. Keeping the restart tied to the saved attempted step prevents
/// interpolation across source derivative corners from hiding the true
/// post-breakpoint state of ideal energy-storage devices.
const BREAKPOINT_RESTART_STEP_SCALE: Value = 0.005;

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
        Self {
            breakpoints: Vec::new(),
            runtime_breakpoints: BTreeSet::new(),
            current_index: 0,
            just_passed_breakpoint: false,
            saved_delta_before_breakpoint: None,
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
                } else if current_time + BREAKPOINT_EQUALIZATION_FACTOR * proposed_dt > bp {
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
            .map(|delta| BREAKPOINT_RESTART_STEP_SCALE * delta.min(next_gap))
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

        assert_eq!(restart, BREAKPOINT_RESTART_STEP_SCALE * 6.0);
        assert!(breakpoints.should_use_minimal_step());
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
    fn predict_next_value(
        &self,
        prev: Value,
        prev_prev: Value,
        prev_prev_prev: Value,
        dt: Value,
    ) -> Value {
        if self.method_order >= 2 && self.history_count >= 3 && self.prev_prev_dt > 0.0 {
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
                    self.predict_next_value(prev, prev_prev, prev_prev_prev, dt)
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

    /// Record a solution point for history
    pub fn record(&mut self, solution: &[Value], dt: Value) {
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
        let len = prefix_len.min(current.len());
        // Need at least one previous point to estimate
        if len == 0 || self.history_count < 1 || self.prev_solution.len() < len {
            return (0.0, true); // Accept, no history yet
        }

        let mut max_lte = 0.0_f64;
        let mut excluded_pos = 0usize;

        // For trapezoidal, LTE ~ (dt^3 / 12) * d^3v/dt^3
        // We approximate by comparing predicted (linear extrapolation) vs actual
        for (i, &curr_val) in current.iter().take(len).enumerate() {
            while excluded_pos < excluded_indices.len() && excluded_indices[excluded_pos] < i {
                excluded_pos += 1;
            }
            if excluded_pos < excluded_indices.len() && excluded_indices[excluded_pos] == i {
                continue;
            }

            let prev_val = self.prev_solution[i];
            let prev_prev_val = self.prev_prev_solution.get(i).copied().unwrap_or(prev_val);
            let prev_prev_prev_val = self
                .prev_prev_prev_solution
                .get(i)
                .copied()
                .unwrap_or(prev_prev_val);
            let predicted =
                self.predict_next_value(prev_val, prev_prev_val, prev_prev_prev_val, dt);

            // LTE estimate: |actual - predicted| with weighted SPICE-like scaling.
            let lte = (curr_val - predicted).abs();
            let scale = self.lte_scale_denominator(curr_val.abs().max(predicted.abs()));
            let normalized_lte = lte / scale;

            max_lte = max_lte.max(normalized_lte);
        }

        let accept = max_lte <= self.reltol;
        (max_lte, accept)
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
        let mut max_lte = 0.0_f64;

        for (&full, &half) in x_full.iter().zip(x_half.iter()) {
            // Richardson extrapolation error estimate
            let richardson_error = (half - full).abs() / order_factor;

            let scale = self.lte_scale_denominator(half.abs().max(full.abs()));
            let normalized = richardson_error / scale;

            max_lte = max_lte.max(normalized);
        }

        let accept = max_lte <= self.reltol;
        (max_lte, accept)
    }

    /// Get recommended timestep scaling factor based on LTE
    /// Uses method order for proper scaling exponent
    pub fn recommend_scale(&self, lte: Value) -> Value {
        if !lte.is_finite() {
            0.25
        } else if lte < 1e-15 {
            2.0 // Error negligible, can increase
        } else {
            // Optimal scaling: (tol/lte)^(1/(p+1)) where p is method order
            // For order 2: exponent = 1/3, for order 1: exponent = 1/2
            let exponent = 1.0 / (self.method_order as Value + 1.0);
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

    /// Reset history (e.g., after discontinuity)
    pub fn reset(&mut self) {
        self.prev_solution.clear();
        self.prev_prev_solution.clear();
        self.prev_prev_prev_solution.clear();
        self.history_count = 0;
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
