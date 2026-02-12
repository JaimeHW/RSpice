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
}

impl DelayBuffer {
    /// Create a new delay buffer with specified capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(2),
            samples: vec![(0.0, 0.0); capacity.max(2)],
            write_pos: 0,
            count: 0,
        }
    }

    /// Record a sample at the given time.
    pub fn record(&mut self, time: f64, value: f64) {
        self.samples[self.write_pos] = (time, value);
        self.write_pos = (self.write_pos + 1) % self.capacity;
        if self.count < self.capacity {
            self.count += 1;
        }
    }

    /// Get the delayed value at (current_time - delay).
    ///
    /// Uses linear interpolation between stored samples.
    /// If delay exceeds buffer history, returns oldest value.
    /// If delay is zero or negative, returns current value (or most recent).
    pub fn get_delayed(&self, current_time: f64, delay: f64) -> f64 {
        if self.count == 0 {
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
    }
}

impl Default for DelayBuffer {
    fn default() -> Self {
        Self::new(1024) // Default to 1024 samples
    }
}

/// Transition filter state for piecewise-linear signal smoothing.
#[derive(Debug, Clone)]
pub struct TransitionFilter {
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

impl TransitionFilter {
    pub fn new() -> Self {
        Self {
            output: 0.0,
            target: 0.0,
            start_time: 0.0,
            end_time: 0.0,
            start_value: 0.0,
        }
    }

    /// Update the filter with new input and return filtered output.
    pub fn update(
        &mut self,
        input: f64,
        time: f64,
        delay: f64,
        rise_time: f64,
        fall_time: f64,
    ) -> f64 {
        // Check if input target changed.
        if (input - self.target).abs() > 1e-15 {
            // New transition started.
            let trans_time = if input > self.output {
                rise_time
            } else {
                fall_time
            };
            self.target = input;
            self.start_value = self.output;
            self.start_time = time + delay;
            self.end_time = self.start_time + trans_time;
        }

        // Compute output based on current transition state.
        if time < self.start_time {
            // Before transition starts.
            self.output
        } else if time >= self.end_time || (self.end_time - self.start_time).abs() < 1e-15 {
            // Transition complete or instantaneous.
            self.output = self.target;
            self.target
        } else {
            // In transition - linear interpolation.
            let t = (time - self.start_time) / (self.end_time - self.start_time);
            self.output = self.start_value + t * (self.target - self.start_value);
            self.output
        }
    }
}

impl Default for TransitionFilter {
    fn default() -> Self {
        Self::new()
    }
}

/// Slew rate filter for limiting rate of change.
#[derive(Debug, Clone)]
pub struct SlewFilter {
    /// Current output value
    pub output: f64,
    /// Previous time
    pub prev_time: f64,
}

impl SlewFilter {
    pub fn new() -> Self {
        Self {
            output: 0.0,
            prev_time: 0.0,
        }
    }

    /// Update the filter with new input, limiting slew rate.
    pub fn update(&mut self, input: f64, time: f64, max_pos_slew: f64, max_neg_slew: f64) -> f64 {
        let dt = time - self.prev_time;
        if dt <= 0.0 {
            return self.output;
        }

        let delta = input - self.output;
        let rate = delta / dt;

        // Apply slew rate limiting.
        let limited_rate = if rate > max_pos_slew {
            max_pos_slew
        } else if rate < -max_neg_slew {
            -max_neg_slew
        } else {
            rate
        };

        self.output += limited_rate * dt;
        self.prev_time = time;
        self.output
    }
}

impl Default for SlewFilter {
    fn default() -> Self {
        Self::new()
    }
}

/// Cross detector for threshold crossing events.
#[derive(Debug, Clone)]
pub struct CrossDetector {
    /// Previous value
    pub prev_value: f64,
    /// Previous time
    pub prev_time: f64,
}

impl CrossDetector {
    pub fn new() -> Self {
        Self {
            prev_value: 0.0,
            prev_time: 0.0,
        }
    }

    /// Check for zero crossing, returns 1.0 if crossed, 0.0 otherwise.
    /// direction: +1 = rising only, -1 = falling only, 0 = both.
    pub fn update(&mut self, value: f64, time: f64, direction: i32) -> f64 {
        let result = if time == self.prev_time {
            0.0 // First call
        } else {
            let crossed = match direction {
                1 => self.prev_value < 0.0 && value >= 0.0,  // Rising
                -1 => self.prev_value > 0.0 && value <= 0.0, // Falling
                _ => {
                    (self.prev_value < 0.0 && value >= 0.0)
                        || (self.prev_value > 0.0 && value <= 0.0)
                }
            };
            if crossed { 1.0 } else { 0.0 }
        };

        self.prev_value = value;
        self.prev_time = time;
        result
    }
}

impl Default for CrossDetector {
    fn default() -> Self {
        Self::new()
    }
}
