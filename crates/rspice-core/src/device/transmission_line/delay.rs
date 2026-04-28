use super::*;

#[derive(Debug, Clone, Copy)]
struct Sample {
    time: Value,
    value: Value,
    slope: Value, // dv/dt at this point
}

/// Circular buffer for storing time history with cubic Hermite interpolation
///
/// Uses cubic Hermite splines for smooth C1-continuous interpolation,
/// which preserves high-frequency content better than linear interpolation.
/// This is critical for transmission line simulation where linear interpolation
/// introduces artificial numerical damping.
#[derive(Debug, Clone)]
pub(in crate::device::transmission_line) struct DelayBuffer {
    /// Samples with time, value, and slope
    data: VecDeque<Sample>,
    /// Maximum storage time
    max_time: Value,
    /// Previous value for slope estimation
    prev_value: Value,
    /// Previous time for slope estimation
    prev_time: Value,
}

impl DelayBuffer {
    pub(in crate::device::transmission_line) fn new(max_time: Value) -> Self {
        Self {
            data: VecDeque::new(),
            max_time,
            prev_value: 0.0,
            prev_time: -1e30, // Very negative so first slope is ~0
        }
    }

    /// Add a new sample with automatic slope estimation
    pub(in crate::device::transmission_line) fn push(&mut self, time: Value, value: Value) {
        // Estimate slope using backward difference
        let dt = time - self.prev_time;
        let slope = if dt > 1e-18 {
            (value - self.prev_value) / dt
        } else {
            0.0
        };

        self.data.push_back(Sample { time, value, slope });

        self.prev_value = value;
        self.prev_time = time;

        // Remove old samples
        while let Some(s) = self.data.front() {
            if time - s.time > self.max_time * 1.5 {
                self.data.pop_front();
            } else {
                break;
            }
        }
    }

    /// Get interpolated value at time (time - delay) using cubic Hermite spline
    ///
    /// Cubic Hermite provides C1 continuity and better preserves:
    /// - High frequency signal content
    /// - Sharp transitions in digital signals
    /// - Phase accuracy at RF frequencies
    pub(in crate::device::transmission_line) fn get_delayed(
        &self,
        current_time: Value,
        delay: Value,
    ) -> Value {
        let target_time = current_time - delay;

        if self.data.is_empty() {
            return 0.0;
        }

        // Binary search would be faster for large buffers, but linear is fine
        // for typical transmission line delays (< 100 samples)
        let mut prev: Option<&Sample> = None;

        for s in self.data.iter() {
            if s.time >= target_time {
                if let Some(p) = prev {
                    return Self::cubic_hermite(p, s, target_time);
                }
                return s.value;
            }
            prev = Some(s);
        }

        // Target time is beyond buffer, return last value
        self.data.back().map(|s| s.value).unwrap_or(0.0)
    }

    /// Cubic Hermite spline interpolation between two samples
    ///
    /// Given points p0 and p1 with values v0, v1 and slopes m0, m1,
    /// interpolates smoothly with continuous first derivative.
    ///
    /// H(t) = (2tÂ³ - 3tÂ² + 1)v0 + (tÂ³ - 2tÂ² + t)Î”tÂ·m0
    ///      + (-2tÂ³ + 3tÂ²)v1 + (tÂ³ - tÂ²)Î”tÂ·m1
    #[inline]
    fn cubic_hermite(p0: &Sample, p1: &Sample, t: Value) -> Value {
        let dt = p1.time - p0.time;
        if dt.abs() < 1e-18 {
            return p1.value;
        }

        // Normalized parameter s âˆˆ [0, 1]
        let s = (t - p0.time) / dt;
        let s2 = s * s;
        let s3 = s2 * s;

        // Hermite basis functions
        let h00 = 2.0 * s3 - 3.0 * s2 + 1.0; // Position at p0
        let h10 = s3 - 2.0 * s2 + s; // Tangent at p0
        let h01 = -2.0 * s3 + 3.0 * s2; // Position at p1
        let h11 = s3 - s2; // Tangent at p1

        // Interpolated value
        h00 * p0.value + h10 * dt * p0.slope + h01 * p1.value + h11 * dt * p1.slope
    }

    /// Clear the buffer
    pub(in crate::device::transmission_line) fn clear(&mut self) {
        self.data.clear();
        self.prev_value = 0.0;
        self.prev_time = -1e30;
    }
}
