//! Breakpoint scheduling.
//!
//! Sources and switches introduce discontinuities that no integration order
//! can absorb. The manager holds the times where that happens and truncates
//! the proposed step to land on them, so the discontinuity falls between two
//! accepted points rather than inside one.

use crate::Value;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Unbounded};

/// Minimum timestep to use immediately after a breakpoint (for restart behavior)
const MIN_STEP_AFTER_BREAKPOINT: Value = 1e-12;

/// Fallback tolerance for detecting exact breakpoint landing.
///
/// Transient engine paths should prefer an ngspice-style tolerance derived
/// from `CKTminBreak`; this default keeps standalone manager use conservative.
const DEFAULT_BREAKPOINT_TOLERANCE: Value = 1e-15;

/// Xyce 7.10 `Util::BreakPoint::defaultTolerance_`.
pub(crate) const XYCE_BREAKPOINT_TOLERANCE: Value = 1.0e-20;

/// Ngspice `dctran.c` factor for equalizing the last two pre-breakpoint steps.
const BREAKPOINT_EQUALIZATION_FACTOR: Value = 1.9;

/// Ngspice `dctran.c` scale for the first step after a breakpoint.
///
/// Ngspice limits that step to 10% of the smaller of the saved, uncut
/// pre-breakpoint proposal and the distance to the next breakpoint.
const NGSPICE_BREAKPOINT_RESTART_STEP_SCALE: Value = 0.1;

/// Accuracy guard for dynamic-charge implementations whose post-breakpoint
/// LTE path has not yet demonstrated stability at the native ngspice scale.
const CONSERVATIVE_BREAKPOINT_RESTART_STEP_SCALE: Value = 0.005;

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
/// Ensures the solver lands exactly on breakpoints and restarts according to
/// the selected simulator dialect's time-integration policy.
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
    /// Xyce's breakpoint equality includes the exact tolerance boundary;
    /// ngspice-style scheduling preserves the established strict comparison.
    inclusive_tolerance: bool,
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
        let (restart_step_scale, equalize_pre_breakpoint_steps, inclusive_tolerance) = match policy
        {
            BreakpointStepPolicy::Ngspice => (NGSPICE_BREAKPOINT_RESTART_STEP_SCALE, true, false),
            BreakpointStepPolicy::Xyce => (XYCE_BREAKPOINT_RESTART_STEP_SCALE, false, true),
        };
        Self {
            breakpoints: Vec::new(),
            runtime_breakpoints: BTreeSet::new(),
            current_index: 0,
            just_passed_breakpoint: false,
            saved_delta_before_breakpoint: None,
            restart_step_scale,
            equalize_pre_breakpoint_steps,
            inclusive_tolerance,
            tolerance: if tolerance.is_finite() && tolerance > 0.0 {
                tolerance
            } else {
                DEFAULT_BREAKPOINT_TOLERANCE
            },
        }
    }

    /// Retain the established small restart for device families whose dynamic
    /// charge integration still needs the extra breakpoint warmup resolution.
    pub(crate) fn use_conservative_restart_step(&mut self) {
        self.restart_step_scale = self
            .restart_step_scale
            .min(CONSERVATIVE_BREAKPOINT_RESTART_STEP_SCALE);
    }

    /// Add a breakpoint time (deduplicates automatically)
    pub fn add(&mut self, time: Value) -> bool {
        Self::insert_sorted_unique(
            &mut self.breakpoints,
            time,
            self.tolerance,
            self.inclusive_tolerance,
        )
    }

    /// Merge a sorted or unsorted permanent schedule in linear time after one
    /// canonical sort. Non-finite values are ignored and points within the
    /// manager tolerance are coalesced exactly as repeated [`Self::add`]
    /// calls would be, without their quadratic insertion cost.
    pub fn extend<I>(&mut self, times: I)
    where
        I: IntoIterator<Item = Value>,
    {
        let processed_through = self
            .current_index
            .checked_sub(1)
            .and_then(|index| self.breakpoints.get(index))
            .copied();
        let mut incoming = times
            .into_iter()
            .filter(|time| time.is_finite())
            .collect::<Vec<_>>();
        if incoming.is_empty() {
            return;
        }
        incoming.sort_by(Value::total_cmp);

        let mut merged = Vec::with_capacity(self.breakpoints.len().saturating_add(incoming.len()));
        let mut existing_index = 0usize;
        let mut incoming_index = 0usize;
        while existing_index < self.breakpoints.len() || incoming_index < incoming.len() {
            let value = match (
                self.breakpoints.get(existing_index).copied(),
                incoming.get(incoming_index).copied(),
            ) {
                (Some(existing), Some(candidate))
                    if Self::difference_within_tolerance(
                        (existing - candidate).abs(),
                        self.tolerance,
                        self.inclusive_tolerance,
                    ) =>
                {
                    // Repeated `add` calls retain an established breakpoint
                    // when a new point falls inside its tolerance window.
                    existing_index += 1;
                    incoming_index += 1;
                    existing
                }
                (Some(existing), Some(candidate)) if existing.total_cmp(&candidate).is_le() => {
                    existing_index += 1;
                    existing
                }
                (Some(_), Some(candidate)) => {
                    incoming_index += 1;
                    candidate
                }
                (Some(existing), None) => {
                    existing_index += 1;
                    existing
                }
                (None, Some(candidate)) => {
                    incoming_index += 1;
                    candidate
                }
                (None, None) => break,
            };
            if merged.last().is_none_or(|last: &Value| {
                !Self::difference_within_tolerance(
                    (value - *last).abs(),
                    self.tolerance,
                    self.inclusive_tolerance,
                )
            }) {
                merged.push(value);
            }
        }
        self.breakpoints = merged;
        self.current_index = processed_through.map_or(0, |time| {
            self.breakpoints
                .partition_point(|breakpoint| *breakpoint <= time + self.tolerance)
        });
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
                self.inclusive_tolerance,
            ) {
                continue;
            }
            Self::insert_runtime_unique(
                &mut self.runtime_breakpoints,
                time,
                self.tolerance,
                self.inclusive_tolerance,
            );
        }
    }

    fn difference_within_tolerance(difference: Value, tolerance: Value, inclusive: bool) -> bool {
        if inclusive {
            difference <= tolerance
        } else {
            difference < tolerance
        }
    }

    fn insert_sorted_unique(
        values: &mut Vec<Value>,
        time: Value,
        tolerance: Value,
        inclusive: bool,
    ) -> bool {
        if !time.is_finite() {
            return false;
        }
        let pos = Self::sorted_insert_position(values, time);
        if Self::neighbors_within_tolerance(values, pos, time, tolerance, inclusive) {
            return false;
        }
        values.insert(pos, time);
        true
    }

    fn contains_sorted_within_tolerance(
        values: &[Value],
        time: Value,
        tolerance: Value,
        inclusive: bool,
    ) -> bool {
        if values.is_empty() {
            return false;
        }
        let pos = Self::sorted_insert_position(values, time);
        Self::neighbors_within_tolerance(values, pos, time, tolerance, inclusive)
    }

    fn insert_runtime_unique(
        values: &mut BTreeSet<BreakpointTimeKey>,
        time: Value,
        tolerance: Value,
        inclusive: bool,
    ) -> bool {
        if !time.is_finite()
            || Self::contains_runtime_within_tolerance(values, time, tolerance, inclusive)
        {
            return false;
        }
        values.insert(BreakpointTimeKey(time))
    }

    fn contains_runtime_within_tolerance(
        values: &BTreeSet<BreakpointTimeKey>,
        time: Value,
        tolerance: Value,
        inclusive: bool,
    ) -> bool {
        let lower = BreakpointTimeKey(time - tolerance);
        let candidate = if inclusive {
            values.range(lower..).next()
        } else {
            values.range((Excluded(lower), Unbounded)).next()
        };
        candidate.is_some_and(|existing| {
            Self::difference_within_tolerance((existing.0 - time).abs(), tolerance, inclusive)
        })
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
        inclusive: bool,
    ) -> bool {
        values.get(pos).is_some_and(|existing| {
            Self::difference_within_tolerance((*existing - time).abs(), tolerance, inclusive)
        }) || pos
            .checked_sub(1)
            .and_then(|prev| values.get(prev))
            .is_some_and(|existing| {
                Self::difference_within_tolerance((*existing - time).abs(), tolerance, inclusive)
            })
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

    fn permanent_breakpoint_within_tolerance(&self, time: Value) -> Option<Value> {
        if !time.is_finite() {
            return None;
        }

        let remaining = self.breakpoints.get(self.current_index..)?;
        let position = match remaining.binary_search_by(|breakpoint| breakpoint.total_cmp(&time)) {
            Ok(position) | Err(position) => position,
        };
        let first_candidate = position.saturating_sub(1);

        remaining
            .get(first_candidate..position.saturating_add(1).min(remaining.len()))?
            .iter()
            .copied()
            .find(|breakpoint| {
                Self::difference_within_tolerance(
                    (time - breakpoint).abs(),
                    self.tolerance,
                    self.inclusive_tolerance,
                )
            })
    }

    fn runtime_breakpoint_within_tolerance(&self, time: Value) -> Option<Value> {
        if !time.is_finite() {
            return None;
        }

        let key = BreakpointTimeKey(time);
        self.runtime_breakpoints
            .range(..=key)
            .next_back()
            .filter(|breakpoint| {
                Self::difference_within_tolerance(
                    (time - breakpoint.0).abs(),
                    self.tolerance,
                    self.inclusive_tolerance,
                )
            })
            .or_else(|| {
                self.runtime_breakpoints
                    .range((Excluded(key), Unbounded))
                    .next()
                    .filter(|breakpoint| {
                        Self::difference_within_tolerance(
                            (time - breakpoint.0).abs(),
                            self.tolerance,
                            self.inclusive_tolerance,
                        )
                    })
            })
            .map(|breakpoint| breakpoint.0)
    }

    /// Check if current time is exactly at a breakpoint
    pub fn at_breakpoint(&self, time: Value) -> bool {
        self.permanent_breakpoint_within_tolerance(time).is_some()
            || self.runtime_breakpoint_within_tolerance(time).is_some()
    }

    /// Return the exact breakpoint time when `time` is within the breakpoint
    /// tolerance. This keeps source evaluation and breakpoint bookkeeping on
    /// the same side of discontinuities after floating-point timestep cuts.
    pub fn snap_to_breakpoint(&self, time: Value) -> Value {
        if let Some(breakpoint) = self.permanent_breakpoint_within_tolerance(time) {
            return breakpoint;
        }
        self.runtime_breakpoint_within_tolerance(time)
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
    /// Returns the dialect-specific restart timestep.
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
    fn ngspice_breakpoint_restart_uses_ten_percent_of_saved_attempted_step() {
        let mut breakpoints = BreakpointManager::new();
        breakpoints.add(10.0);

        let (dt, lands_on_breakpoint) = breakpoints.limit_step(4.0, 6.0);
        assert_eq!(dt, 6.0);
        assert!(lands_on_breakpoint);

        let restart = breakpoints.mark_breakpoint_solved(10.0);

        assert!((restart - 0.6).abs() <= 8.0 * Value::EPSILON);
        assert!(breakpoints.should_use_minimal_step());
    }

    #[test]
    fn ngspice_breakpoint_restart_is_limited_by_next_breakpoint_gap() {
        let mut breakpoints = BreakpointManager::new();
        breakpoints.add(10.0);
        breakpoints.add(10.5);

        let (dt, lands_on_breakpoint) = breakpoints.limit_step(4.0, 6.0);
        assert_eq!(dt, 6.0);
        assert!(lands_on_breakpoint);

        let restart = breakpoints.mark_breakpoint_solved(10.0);

        assert!((restart - 0.05).abs() <= 8.0 * Value::EPSILON);
    }

    #[test]
    fn conservative_restart_guard_only_tightens_the_selected_policy() {
        let mut breakpoints = BreakpointManager::new();
        breakpoints.add(10.0);
        breakpoints.use_conservative_restart_step();

        let (dt, lands_on_breakpoint) = breakpoints.limit_step(4.0, 6.0);
        assert_eq!(dt, 6.0);
        assert!(lands_on_breakpoint);

        let restart = breakpoints.mark_breakpoint_solved(10.0);

        assert!((restart - 0.03).abs() <= 8.0 * Value::EPSILON);
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
    fn bulk_breakpoint_merge_preserves_existing_points_and_cursor() {
        let mut breakpoints = BreakpointManager::new_with_tolerance(1.0);
        breakpoints.add(10.0);
        breakpoints.add(20.0);
        breakpoints.discard_through(10.0);

        breakpoints.extend([19.5, 15.0, 11.2, 9.5, Value::NAN]);

        assert_eq!(breakpoints.times(), &[10.0, 11.2, 15.0, 20.0]);
        assert_eq!(breakpoints.next_after(10.0), Some(11.2));
        assert!(!breakpoints.at_breakpoint(10.0));
    }

    #[test]
    fn bulk_breakpoint_merge_matches_sorted_individual_insertions() {
        let incoming = [8.8, 9.5, 11.2, 12.0, 15.0, 19.5, 21.2];
        let mut individual = BreakpointManager::new_with_tolerance(1.0);
        let mut bulk = BreakpointManager::new_with_tolerance(1.0);
        for manager in [&mut individual, &mut bulk] {
            manager.add(10.0);
            manager.add(20.0);
        }
        for time in incoming {
            individual.add(time);
        }
        bulk.extend(incoming);

        assert_eq!(bulk.times(), individual.times());
    }

    #[test]
    fn xyce_breakpoint_equality_includes_the_exact_tolerance_boundary() {
        let tolerance = 1.0e-6;
        let mut xyce =
            BreakpointManager::new_with_tolerance_and_policy(tolerance, BreakpointStepPolicy::Xyce);
        let mut ngspice = BreakpointManager::new_with_tolerance_and_policy(
            tolerance,
            BreakpointStepPolicy::Ngspice,
        );
        for manager in [&mut xyce, &mut ngspice] {
            assert!(manager.add(0.0));
        }
        assert!(!xyce.add(tolerance));
        assert!(ngspice.add(tolerance));
        assert!(xyce.at_breakpoint(tolerance));

        let mut bulk =
            BreakpointManager::new_with_tolerance_and_policy(tolerance, BreakpointStepPolicy::Xyce);
        bulk.extend([0.0, tolerance]);
        assert_eq!(bulk.times(), &[0.0]);
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

    #[test]
    fn breakpoint_lookup_preserves_strict_tolerance_and_sorted_tie_breaking() {
        let mut breakpoints = BreakpointManager::new_with_tolerance(1.0);
        breakpoints.add(10.0);
        breakpoints.add(11.0);

        assert!(breakpoints.at_breakpoint(10.5));
        assert_eq!(breakpoints.snap_to_breakpoint(10.5), 10.0);
        assert!(!breakpoints.at_breakpoint(9.0));
        assert_eq!(breakpoints.snap_to_breakpoint(9.0), 9.0);
        assert!(!breakpoints.at_breakpoint(12.0));
        assert_eq!(breakpoints.snap_to_breakpoint(12.0), 12.0);
    }

    #[test]
    fn breakpoint_lookup_uses_only_the_unprocessed_permanent_schedule() {
        let mut breakpoints = BreakpointManager::new_with_tolerance(1.0e-12);
        for index in 0..=100_000 {
            breakpoints.add(index as Value * 1.0e-6);
        }

        breakpoints.discard_through(5.0e-2);

        assert!(!breakpoints.at_breakpoint(5.0e-2));
        assert_eq!(breakpoints.snap_to_breakpoint(5.0e-2), 5.0e-2);
        let near_future_knot = 7.5e-2 + 0.25e-12;
        assert!(breakpoints.at_breakpoint(near_future_knot));
        assert_eq!(breakpoints.snap_to_breakpoint(near_future_knot), 7.5e-2);
    }

    #[test]
    fn runtime_breakpoint_lookup_preserves_order_and_strict_tolerance() {
        let mut breakpoints = BreakpointManager::new_with_tolerance(1.0);
        breakpoints.replace_runtime_breakpoints([20.0, 21.0]);

        assert!(breakpoints.at_breakpoint(20.5));
        assert_eq!(breakpoints.snap_to_breakpoint(20.5), 20.0);
        assert!(!breakpoints.at_breakpoint(19.0));
        assert_eq!(breakpoints.snap_to_breakpoint(19.0), 19.0);
        assert!(!breakpoints.at_breakpoint(22.0));
        assert_eq!(breakpoints.snap_to_breakpoint(22.0), 22.0);
    }
}
