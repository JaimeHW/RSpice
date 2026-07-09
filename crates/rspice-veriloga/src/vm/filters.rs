/// Circular buffer for absdelay transport delay implementation.
///
/// Stores a history of (time, value) pairs to enable looking up
/// delayed values via linear interpolation.
#[derive(Debug, Clone)]
pub struct DelayBuffer {
    /// Buffer capacity (max samples)
    pub(crate) capacity: usize,
    /// Circular buffer of (time, value) pairs
    samples: Vec<(f64, f64)>,
    /// Current write position
    write_pos: usize,
    /// Number of samples currently stored
    pub(crate) count: usize,
    /// Candidate sample produced by the current Newton evaluation.
    ///
    /// This is deliberately separate from the circular history. Repeated
    /// evaluations of one timepoint must not consume transport-delay history;
    /// the candidate enters the committed history only when the simulator
    /// accepts the timestep.
    candidate: Option<(f64, f64)>,
}

/// Evaluate a Verilog-A timer and return both its event level and the next
/// absolute event time that the transient stepper must not skip.
///
/// A non-positive period denotes a one-shot timer.  The event is level-true
/// throughout repeated Newton evaluations of the same candidate timepoint,
/// but is excluded once that timepoint becomes the lower endpoint of the next
/// step.  `time_tol` is an event-coalescing tolerance, while a small
/// scale-aware floating-point tolerance protects endpoint comparisons.
pub(crate) fn timer_event_evaluation(
    start_time: f64,
    period: f64,
    time_tol: f64,
    enable: f64,
    current_time: f64,
    timestep: f64,
) -> (f64, Option<f64>) {
    if !start_time.is_finite()
        || !period.is_finite()
        || !time_tol.is_finite()
        || !enable.is_finite()
        || !current_time.is_finite()
        || !timestep.is_finite()
        || enable == 0.0
        || period < 0.0
    {
        return (0.0, None);
    }

    let scale = start_time.abs().max(current_time.abs());
    let numeric_tol = (16.0 * f64::EPSILON * scale).max(f64::MIN_POSITIVE);
    let event_tol = time_tol.max(0.0).max(numeric_tol);
    let horizon = current_time + event_tol;
    let previous_time = current_time - timestep.max(0.0);

    let scheduled = if horizon + numeric_tol < start_time {
        None
    } else if period > 0.0 {
        let cycles = ((horizon - start_time) / period).floor().max(0.0);
        let candidate = start_time + cycles * period;
        candidate.is_finite().then_some(candidate)
    } else {
        Some(start_time)
    };

    let fired = scheduled.is_some_and(|event_time| {
        event_time <= horizon
            && if timestep > numeric_tol {
                event_time > previous_time + numeric_tol
            } else {
                (current_time - event_time).abs() <= event_tol
            }
    });

    let next_event = if horizon + numeric_tol < start_time {
        Some(start_time)
    } else if period > 0.0 {
        let cycles = ((horizon - start_time) / period).floor().max(0.0) + 1.0;
        let next = start_time + cycles * period;
        (next.is_finite() && next > current_time + numeric_tol).then_some(next)
    } else {
        None
    };

    (if fired { 1.0 } else { 0.0 }, next_event)
}

impl DelayBuffer {
    /// Create a new delay buffer with specified capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(2),
            samples: vec![(0.0, 0.0); capacity.max(2)],
            write_pos: 0,
            count: 0,
            candidate: None,
        }
    }

    /// Record a sample at the given time.
    pub fn record(&mut self, time: f64, value: f64) {
        if self.count == self.capacity {
            // Preserve the entire accepted history. A fixed sample count is
            // not a valid delay horizon because timestep control is adaptive.
            // The optional absdelay max-delay argument is applied separately
            // as a time-domain retention bound.
            let new_capacity = self.capacity.saturating_mul(2).max(self.capacity + 1);
            let mut grown = vec![(0.0, 0.0); new_capacity];
            for (index, slot) in grown.iter_mut().enumerate().take(self.count) {
                *slot = self.samples[(self.write_pos + index) % self.capacity];
            }
            self.samples = grown;
            self.capacity = new_capacity;
            self.write_pos = self.count;
        }
        self.samples[self.write_pos] = (time, value);
        self.write_pos = (self.write_pos + 1) % self.capacity;
        if self.count < self.capacity {
            self.count += 1;
        }
    }

    /// Evaluate a delayed value without mutating accepted history.
    pub fn eval(&mut self, time: f64, value: f64, delay: f64) -> f64 {
        self.candidate = Some((time, value));
        if delay <= 0.0 {
            value
        } else {
            self.get_delayed_with_candidate(time, delay)
        }
    }

    /// Commit the latest candidate after the transient step is accepted.
    pub fn commit(&mut self) {
        if let Some((time, value)) = self.candidate.take() {
            // An accepted-time reevaluation replaces the existing endpoint
            // instead of adding a duplicate timestamp.
            if self.count > 0 {
                let newest = if self.count == self.capacity {
                    (self.write_pos + self.capacity - 1) % self.capacity
                } else {
                    self.count - 1
                };
                if self.samples[newest].0 == time {
                    self.samples[newest] = (time, value);
                    return;
                }
            }
            self.record(time, value);
        }
    }

    /// Get the delayed value at (current_time - delay).
    ///
    /// Uses linear interpolation between stored samples.
    /// If delay exceeds buffer history, returns oldest value.
    /// If delay is zero or negative, returns current value (or most recent).
    pub fn get_delayed(&self, current_time: f64, delay: f64) -> f64 {
        self.lookup_delayed(current_time, delay, None)
    }

    fn get_delayed_with_candidate(&self, current_time: f64, delay: f64) -> f64 {
        self.lookup_delayed(current_time, delay, self.candidate)
    }

    fn lookup_delayed(&self, current_time: f64, delay: f64, candidate: Option<(f64, f64)>) -> f64 {
        if self.count == 0 && candidate.is_none() {
            return 0.0;
        }

        let target_time = current_time - delay;

        // Find the two samples bracketing target_time.
        // Samples are stored in circular order, so we need to search.
        let mut before: Option<(f64, f64)> = None;
        let mut after: Option<(f64, f64)> = None;

        for i in 0..self.count {
            let idx = if self.count == self.capacity {
                (self.write_pos + i) % self.capacity
            } else {
                i
            };
            let (t, v) = self.samples[idx];

            if t <= target_time {
                before = Some((t, v));
            }
            if t >= target_time && after.is_none() {
                after = Some((t, v));
            }
        }

        if let Some((t, v)) = candidate {
            if t <= target_time {
                before = Some((t, v));
            }
            if t >= target_time && after.map(|(after_time, _)| t < after_time).unwrap_or(true) {
                after = Some((t, v));
            }
        }

        match (before, after) {
            (Some((t0, v0)), Some((t1, v1))) if t0 != t1 => {
                // Linear interpolation.
                let alpha = (target_time - t0) / (t1 - t0);
                v0 + alpha * (v1 - v0)
            }
            (Some((_, v)), _) => v,
            (_, Some((_, v))) => v,
            _ => 0.0,
        }
    }

    /// Clear the buffer.
    pub fn clear(&mut self) {
        self.count = 0;
        self.write_pos = 0;
        self.candidate = None;
    }
}

#[cfg(test)]
mod tests {
    use super::{CrossDetector, DelayBuffer, SlewFilter, TransitionFilter, timer_event_evaluation};

    #[test]
    fn one_shot_timer_fires_once_and_requests_its_timepoint() {
        assert_eq!(
            timer_event_evaluation(1.0, 0.0, 0.0, 1.0, 0.0, 0.0),
            (0.0, Some(1.0))
        );
        assert_eq!(
            timer_event_evaluation(1.0, 0.0, 0.0, 1.0, 1.0, 1.0),
            (1.0, None)
        );
        assert_eq!(
            timer_event_evaluation(1.0, 0.0, 0.0, 1.0, 1.0, 1.0),
            (1.0, None),
            "repeated Newton evaluations must see the same event"
        );
        assert_eq!(
            timer_event_evaluation(1.0, 0.0, 0.0, 1.0, 1.1, 0.1),
            (0.0, None),
            "an accepted event must not fire again at the next step"
        );
    }

    #[test]
    fn periodic_timer_reports_each_next_breakpoint() {
        assert_eq!(
            timer_event_evaluation(1.0, 0.5, 0.0, 1.0, 1.0, 1.0),
            (1.0, Some(1.5))
        );
        assert_eq!(
            timer_event_evaluation(1.0, 0.5, 0.0, 1.0, 1.5, 0.5),
            (1.0, Some(2.0))
        );
    }

    #[test]
    fn disabled_and_invalid_timers_do_not_fire_or_schedule() {
        assert_eq!(
            timer_event_evaluation(1.0, 0.5, 0.0, 0.0, 0.0, 0.0),
            (0.0, None)
        );
        assert_eq!(
            timer_event_evaluation(1.0, -0.5, 0.0, 1.0, 0.0, 0.0),
            (0.0, None)
        );
    }

    #[test]
    fn delay_history_grows_without_dropping_accepted_samples() {
        let mut buffer = DelayBuffer::new(2);
        for time in 0..4096 {
            buffer.record(time as f64, time as f64);
        }

        assert_eq!(buffer.get_delayed(4095.0, 4095.0), 0.0);
        assert_eq!(buffer.get_delayed(4095.0, 2047.5), 2047.5);
    }

    #[test]
    fn stateful_filter_candidates_do_not_mutate_committed_state() {
        let mut transition = TransitionFilter::default();
        assert_eq!(transition.eval(1.0, 1.0, 0.0, 2.0, 2.0), 0.0);
        assert_eq!(transition.eval(2.0, 1.0, 0.0, 2.0, 2.0), 0.0);
        assert_eq!(transition.eval(1.0, 1.0, 0.0, 2.0, 2.0), 0.0);

        let mut slew = SlewFilter::default();
        assert_eq!(slew.eval(1.0, 1.0, 0.5, 0.5), 0.5);
        assert_eq!(slew.eval(0.0, 1.0, 0.5, 0.5), 0.0);
        assert_eq!(slew.eval(1.0, 1.0, 0.5, 0.5), 0.5);

        let mut cross = CrossDetector::default();
        assert_eq!(cross.eval(-1.0, 0.0, 1), 0.0);
        cross.commit();
        assert_eq!(cross.eval(1.0, 1.0, 1), 1.0);
        assert_eq!(cross.eval(1.0, 1.0, 1), 1.0);
    }

    #[test]
    fn cross_time_tolerance_coalesces_nearby_events() {
        let mut cross = CrossDetector::default();
        assert_eq!(cross.eval_event(-1.0, 0.0, 0, 1.0, 0.0, true), 0.0);
        cross.commit();
        assert_eq!(cross.eval_event(1.0, 1.0, 0, 1.0, 0.0, true), 1.0);
        cross.commit();

        assert_eq!(cross.eval_event(-1.0, 1.2, 0, 1.0, 0.0, true), 0.0);
        cross.commit();
        assert_eq!(cross.eval_event(1.0, 1.4, 0, 1.0, 0.0, true), 0.0);
        cross.commit();

        assert_eq!(cross.eval_event(-1.0, 2.0, 0, 1.0, 0.0, true), 1.0);
    }
}

impl Default for DelayBuffer {
    fn default() -> Self {
        Self::new(1024) // Default to 1024 samples
    }
}

#[derive(Debug, Clone, Copy)]
struct TransitionState {
    /// Current output value
    pub output: f64,
    /// Target value we're transitioning to
    pub target: f64,
    /// Time when transition started
    pub start_time: f64,
    /// Time when transition ends
    pub end_time: f64,
    /// Initial value at start of transition
    pub start_value: f64,
}

impl Default for TransitionState {
    fn default() -> Self {
        Self {
            output: 0.0,
            target: 0.0,
            start_time: 0.0,
            end_time: 0.0,
            start_value: 0.0,
        }
    }
}

/// Transition filter state for piecewise-linear signal smoothing.
#[derive(Debug, Clone, Default)]
pub struct TransitionFilter {
    committed: TransitionState,
    candidate: TransitionState,
    candidate_valid: bool,
}

impl TransitionFilter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the filter with new input and return filtered output.
    pub fn eval(
        &mut self,
        input: f64,
        time: f64,
        delay: f64,
        rise_time: f64,
        fall_time: f64,
    ) -> f64 {
        let mut state = self.committed;
        // Check if input target changed.
        if (input - state.target).abs() > 1e-15 {
            // New transition started.
            let trans_time = if input > state.output {
                rise_time
            } else {
                fall_time
            };
            state.target = input;
            state.start_value = state.output;
            state.start_time = time + delay;
            state.end_time = state.start_time + trans_time;
        }

        // Compute output based on current transition state.
        let output = if time < state.start_time {
            // Before transition starts.
            state.output
        } else if time >= state.end_time || (state.end_time - state.start_time).abs() < 1e-15 {
            // Transition complete or instantaneous.
            state.output = state.target;
            state.target
        } else {
            // In transition - linear interpolation.
            let t = (time - state.start_time) / (state.end_time - state.start_time);
            state.output = state.start_value + t * (state.target - state.start_value);
            state.output
        };
        self.candidate = state;
        self.candidate_valid = true;
        output
    }

    pub fn commit(&mut self) {
        if self.candidate_valid {
            self.committed = self.candidate;
            self.candidate_valid = false;
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct SlewState {
    output: f64,
    prev_time: f64,
}

/// Slew rate filter for limiting rate of change.
#[derive(Debug, Clone, Default)]
pub struct SlewFilter {
    committed: SlewState,
    candidate: SlewState,
    candidate_valid: bool,
}

impl SlewFilter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the filter with new input, limiting slew rate.
    pub fn eval(&mut self, input: f64, time: f64, max_pos_slew: f64, max_neg_slew: f64) -> f64 {
        let mut state = self.committed;
        let dt = time - state.prev_time;
        if dt <= 0.0 {
            self.candidate = state;
            self.candidate_valid = true;
            return state.output;
        }

        let delta = input - state.output;
        let rate = delta / dt;

        // Apply slew rate limiting.
        let limited_rate = if rate > max_pos_slew {
            max_pos_slew
        } else if rate < -max_neg_slew {
            -max_neg_slew
        } else {
            rate
        };

        state.output += limited_rate * dt;
        state.prev_time = time;
        self.candidate = state;
        self.candidate_valid = true;
        state.output
    }

    pub fn commit(&mut self) {
        if self.candidate_valid {
            self.committed = self.candidate;
            self.candidate_valid = false;
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct CrossState {
    value: f64,
    time: f64,
    side: i8,
    last_event_time: f64,
    last_crossing_time: f64,
    initialized: bool,
}

impl Default for CrossState {
    fn default() -> Self {
        Self {
            value: 0.0,
            time: 0.0,
            side: 0,
            last_event_time: f64::NEG_INFINITY,
            last_crossing_time: -1.0,
            initialized: false,
        }
    }
}

/// Cross detector for threshold crossing events.
#[derive(Debug, Clone, Default)]
pub struct CrossDetector {
    committed: CrossState,
    candidate: CrossState,
    candidate_valid: bool,
}

impl CrossDetector {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check for zero crossing, returns 1.0 if crossed, 0.0 otherwise.
    /// direction: +1 = rising only, -1 = falling only, 0 = both.
    pub fn eval(&mut self, value: f64, time: f64, direction: i32) -> f64 {
        self.eval_event(value, time, direction, 0.0, 0.0, true)
    }

    /// Evaluate `cross(expr, direction, time_tol, expr_tol, enable)`.
    pub fn eval_event(
        &mut self,
        value: f64,
        time: f64,
        direction: i32,
        time_tol: f64,
        expr_tol: f64,
        enabled: bool,
    ) -> f64 {
        self.eval_impl(value, time, direction, time_tol, expr_tol, enabled, false)
    }

    /// Evaluate `above(expr, time_tol, expr_tol, enable)`.
    ///
    /// Unlike `cross`, `above` also fires during the initial operating-point
    /// evaluation when the expression starts on the positive side.
    pub fn eval_above(
        &mut self,
        value: f64,
        time: f64,
        time_tol: f64,
        expr_tol: f64,
        enabled: bool,
    ) -> f64 {
        self.eval_impl(value, time, 1, time_tol, expr_tol, enabled, true)
    }

    /// Return the linearly interpolated time of the most recent crossing.
    /// Returns -1.0 until a crossing matching `direction` has occurred.
    pub fn eval_last_crossing(&mut self, value: f64, time: f64, direction: i32) -> f64 {
        if !self.committed.initialized {
            self.candidate = CrossState {
                value,
                time,
                side: Self::side(value, 0.0),
                initialized: true,
                ..CrossState::default()
            };
            self.candidate_valid = true;
            return -1.0;
        }

        if !value.is_finite() || !time.is_finite() || time <= self.committed.time {
            self.candidate = self.committed;
            self.candidate_valid = true;
            return self.committed.last_crossing_time;
        }

        let rising = self.committed.value < 0.0 && value >= 0.0;
        let falling = self.committed.value > 0.0 && value <= 0.0;
        let crossing_direction = if rising {
            1
        } else if falling {
            -1
        } else {
            0
        };
        let mut candidate = CrossState {
            value,
            time,
            side: Self::side(value, 0.0),
            ..self.committed
        };
        if crossing_direction != 0 && (direction == 0 || direction == crossing_direction) {
            candidate.last_crossing_time = self.estimate_crossing_time(value, time);
        }
        self.candidate = candidate;
        self.candidate_valid = true;
        candidate.last_crossing_time
    }

    fn eval_impl(
        &mut self,
        value: f64,
        time: f64,
        direction: i32,
        time_tol: f64,
        expr_tol: f64,
        enabled: bool,
        initial_above: bool,
    ) -> f64 {
        let expr_tol = if expr_tol.is_finite() {
            expr_tol.abs()
        } else {
            0.0
        };
        let time_tol = if time_tol.is_finite() {
            time_tol.max(0.0)
        } else {
            0.0
        };

        if !self.committed.initialized {
            let side = Self::side(value, expr_tol);
            let mut candidate = CrossState {
                value,
                time,
                side,
                initialized: true,
                ..CrossState::default()
            };
            let fired = initial_above && enabled && side > 0;
            if fired {
                candidate.last_event_time = time;
                candidate.last_crossing_time = time;
            }
            self.candidate = candidate;
            self.candidate_valid = true;
            return if fired { 1.0 } else { 0.0 };
        }

        if !value.is_finite() || !time.is_finite() || time <= self.committed.time {
            self.candidate = self.committed;
            self.candidate_valid = true;
            return 0.0;
        }

        let rising = self.committed.side < 0 && value >= -expr_tol;
        let falling = self.committed.side > 0 && value <= expr_tol;
        let crossing_direction = if rising {
            1
        } else if falling {
            -1
        } else {
            0
        };
        let crossing_time = if crossing_direction != 0 {
            self.estimate_crossing_time(value, time)
        } else {
            -1.0
        };
        let direction_matches = direction == 0 || direction == crossing_direction;
        let separated = crossing_time - self.committed.last_event_time > time_tol;
        let fired = enabled && crossing_direction != 0 && direction_matches && separated;

        let stable_side = Self::side(value, expr_tol);
        let side = if stable_side != 0 {
            stable_side
        } else if crossing_direction != 0 {
            crossing_direction as i8
        } else {
            self.committed.side
        };
        let mut candidate = CrossState {
            value,
            time,
            side,
            ..self.committed
        };
        if fired {
            candidate.last_event_time = crossing_time;
            candidate.last_crossing_time = crossing_time;
        }
        self.candidate = candidate;
        self.candidate_valid = true;
        if fired { 1.0 } else { 0.0 }
    }

    fn side(value: f64, tolerance: f64) -> i8 {
        if value > tolerance {
            1
        } else if value < -tolerance {
            -1
        } else {
            0
        }
    }

    fn estimate_crossing_time(&self, value: f64, time: f64) -> f64 {
        let delta = value - self.committed.value;
        if delta == 0.0 || !delta.is_finite() {
            return time;
        }
        let fraction = (-self.committed.value / delta).clamp(0.0, 1.0);
        self.committed.time + fraction * (time - self.committed.time)
    }

    pub fn commit(&mut self) {
        if self.candidate_valid {
            self.committed = self.candidate;
            self.candidate_valid = false;
        }
    }
}
