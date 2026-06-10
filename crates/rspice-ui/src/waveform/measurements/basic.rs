#[derive(Debug, Clone, Copy, Default)]
pub(super) struct BasicStats {
    pub(super) count: usize,
    pub(super) min: f64,
    pub(super) max: f64,
    pub(super) mean: f64,
    pub(super) sum_squares: f64,
    pub(super) m2: f64,
}

impl BasicStats {
    pub(super) fn from_samples(samples: &[f64]) -> Option<Self> {
        let mut stats = Self {
            count: 0,
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            mean: 0.0,
            sum_squares: 0.0,
            m2: 0.0,
        };

        for &value in samples {
            if !value.is_finite() {
                continue;
            }

            stats.count += 1;
            stats.min = stats.min.min(value);
            stats.max = stats.max.max(value);
            stats.sum_squares += value * value;

            let delta = value - stats.mean;
            stats.mean += delta / stats.count as f64;
            let delta2 = value - stats.mean;
            stats.m2 += delta * delta2;
        }

        (stats.count > 0).then_some(stats)
    }

    pub(super) fn pk_pk(self) -> f64 {
        self.max - self.min
    }

    pub(super) fn rms(self) -> f64 {
        (self.sum_squares / self.count as f64).sqrt()
    }

    pub(super) fn std_dev(self) -> Option<f64> {
        (self.count > 1).then(|| (self.m2 / (self.count - 1) as f64).sqrt())
    }
}

// =============================================================================

// Basic Amplitude Measurements
// =============================================================================

/// Calculate minimum value in a region
pub fn calculate_min(y_data: &[f64]) -> Option<f64> {
    BasicStats::from_samples(y_data).map(|stats| stats.min)
}

/// Calculate maximum value in a region
pub fn calculate_max(y_data: &[f64]) -> Option<f64> {
    BasicStats::from_samples(y_data).map(|stats| stats.max)
}

/// Calculate peak-to-peak amplitude
pub fn calculate_pk_pk(y_data: &[f64]) -> Option<f64> {
    BasicStats::from_samples(y_data).map(BasicStats::pk_pk)
}

/// Calculate mean (average) value
pub fn calculate_mean(y_data: &[f64]) -> Option<f64> {
    BasicStats::from_samples(y_data).map(|stats| stats.mean)
}

/// Calculate RMS (root mean square) value
pub fn calculate_rms(y_data: &[f64]) -> Option<f64> {
    BasicStats::from_samples(y_data).map(BasicStats::rms)
}

/// Calculate minimum, maximum, and RMS in a single pass
pub fn calculate_min_max_rms(y_data: &[f64]) -> Option<(f64, f64, f64)> {
    BasicStats::from_samples(y_data).map(|stats| (stats.min, stats.max, stats.rms()))
}

/// Calculate standard deviation
pub fn calculate_std_dev(y_data: &[f64]) -> Option<f64> {
    BasicStats::from_samples(y_data).and_then(BasicStats::std_dev)
}

// =============================================================================
