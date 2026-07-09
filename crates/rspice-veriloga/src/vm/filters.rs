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
    use super::{CrossDetector, DelayBuffer, SlewFilter, TransitionFilter};

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

#[derive(Debug, Clone, Copy, Default)]
struct CrossState {
    value: f64,
    time: f64,
    initialized: bool,
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
        let result = if !self.committed.initialized || time <= self.committed.time {
            0.0
        } else {
            let crossed = match direction {
                1 => self.committed.value < 0.0 && value >= 0.0, // Rising
                -1 => self.committed.value > 0.0 && value <= 0.0, // Falling
                _ => {
                    (self.committed.value < 0.0 && value >= 0.0)
                        || (self.committed.value > 0.0 && value <= 0.0)
                }
            };
            if crossed { 1.0 } else { 0.0 }
        };

        self.candidate = CrossState {
            value,
            time,
            initialized: true,
        };
        self.candidate_valid = true;
        result
    }

    pub fn commit(&mut self) {
        if self.candidate_valid {
            self.committed = self.candidate;
            self.candidate_valid = false;
        }
    }
}
