//! Finite descriptive moments of retained samples.

/// Sample count, mean, sample standard deviation and finite bounds.
#[derive(Debug, Clone, Copy)]
pub struct SampleMoments {
    pub count: usize,
    pub mean: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
}

impl SampleMoments {
    /// Resolve moments with scaled, compensated accumulation.
    pub fn from_samples(samples: &[f64]) -> Option<Self> {
        if samples.is_empty() || samples.iter().any(|value| !value.is_finite()) {
            return None;
        }

        let count = samples.len();
        let min = samples.iter().copied().fold(f64::INFINITY, f64::min);
        let max = samples.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        // Center only tightly clustered values of one sign; otherwise scaling
        // about zero preserves small residuals in populations that span zero.
        let anchor = if (min > 0.0 && min >= max * 0.5) || (max < 0.0 && max <= min * 0.5) {
            samples[0]
        } else {
            0.0
        };
        let scale = (min - anchor).abs().max((max - anchor).abs());
        let (mean, std_dev) = if scale == 0.0 {
            (anchor, 0.0)
        } else {
            let normalized_mean =
                compensated_sum(samples.iter().map(|value| (value - anchor) / scale))
                    / count as f64;
            let variance = compensated_sum(
                samples
                    .iter()
                    .map(|value| ((value - anchor) / scale - normalized_mean).powi(2)),
            ) / count.saturating_sub(1).max(1) as f64;
            (anchor + normalized_mean * scale, variance.sqrt() * scale)
        };
        (mean.is_finite() && std_dev.is_finite()).then_some(Self {
            count,
            mean,
            std_dev,
            min,
            max,
        })
    }
}

fn compensated_sum(values: impl Iterator<Item = f64>) -> f64 {
    let (mut sum, mut correction) = (0.0_f64, 0.0_f64);
    for value in values {
        let next = sum + value;
        correction += if sum.abs() >= value.abs() {
            (sum - next) + value
        } else {
            (value - next) + sum
        };
        sum = next;
    }
    sum + correction
}
