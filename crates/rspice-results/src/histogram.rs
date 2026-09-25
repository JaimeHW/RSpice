//! Histogram Data Structures
//!
//! Core data types for histogram construction and analysis.

// =============================================================================
// Histogram Bin
// =============================================================================

/// A single histogram bin
#[derive(Debug, Clone, PartialEq)]
pub struct HistogramBin {
    /// Lower edge of bin
    pub lower: f64,
    /// Upper edge of bin
    pub upper: f64,
    /// Count of samples in bin
    pub count: usize,
    /// Accumulated weight (for weighted histograms)
    pub weight: f64,
}

impl HistogramBin {
    /// Create new bin
    pub fn new(lower: f64, upper: f64) -> Self {
        Self {
            lower,
            upper,
            count: 0,
            weight: 0.0,
        }
    }

    /// Bin center
    pub fn center(&self) -> f64 {
        let sum = self.lower + self.upper;
        if sum.is_finite() {
            sum * 0.5
        } else {
            self.lower * 0.5 + self.upper * 0.5
        }
    }

    /// Bin width
    pub fn width(&self) -> f64 {
        self.upper - self.lower
    }

    /// Add a sample to this bin
    pub fn add(&mut self, weight: f64) {
        self.count += 1;
        self.weight += weight;
    }
}

// =============================================================================
// Histogram
// =============================================================================

/// Complete histogram data
#[derive(Debug, Clone)]
pub struct Histogram {
    /// Name/label for the histogram
    pub name: String,
    /// All bins
    pub bins: Vec<HistogramBin>,
    /// Total sample count
    pub total_count: usize,
    /// Total weight
    pub total_weight: f64,
    /// Underflow count (samples below min)
    pub underflow: usize,
    /// Overflow count (samples above max)
    pub overflow: usize,
    /// Minimum value in data
    pub data_min: f64,
    /// Maximum value in data
    pub data_max: f64,
}

impl Default for Histogram {
    fn default() -> Self {
        Self {
            name: String::new(),
            bins: Vec::new(),
            total_count: 0,
            total_weight: 0.0,
            underflow: 0,
            overflow: 0,
            data_min: f64::MAX,
            data_max: f64::MIN,
        }
    }
}

impl Histogram {
    /// Create new empty histogram with specified range
    pub fn new(name: &str, min: f64, max: f64, bin_count: usize) -> Self {
        if !min.is_finite() || !max.is_finite() || min > max {
            return Self {
                name: name.to_owned(),
                ..Self::default()
            };
        }
        let bin_count = if min == max { 1 } else { bin_count.max(1) };

        let mut bins = Vec::with_capacity(bin_count);
        let edge = |index: usize| {
            if index == 0 {
                return min;
            }
            if index == bin_count {
                return max;
            }
            let fraction = index as f64 / bin_count as f64;
            if min.signum() == max.signum() {
                min + (max - min) * fraction
            } else {
                min * (1.0 - fraction) + max * fraction
            }
        };
        for i in 0..bin_count {
            let lower = edge(i);
            let upper = edge(i + 1);
            // More requested bins than representable edges must not create
            // duplicate zero-width intervals. A point population keeps one.
            if upper > lower || min == max {
                bins.push(HistogramBin::new(lower, upper));
            }
        }

        Self {
            name: name.to_string(),
            bins,
            ..Default::default()
        }
    }

    /// Histogram range (min, max)
    pub fn range(&self) -> (f64, f64) {
        if self.bins.is_empty() {
            return (0.0, 0.0);
        }
        (
            self.bins.first().unwrap().lower,
            self.bins.last().unwrap().upper,
        )
    }

    /// Add a sample value
    pub fn add(&mut self, value: f64) {
        self.add_weighted(value, 1.0);
    }

    /// Add a weighted sample
    pub fn add_weighted(&mut self, value: f64, weight: f64) {
        if !value.is_finite() || self.bins.is_empty() {
            return;
        }

        self.total_count += 1;
        self.total_weight += weight;
        self.data_min = self.data_min.min(value);
        self.data_max = self.data_max.max(value);

        let (hist_min, hist_max) = self.range();

        if value < hist_min {
            self.underflow += 1;
            return;
        }

        if value > hist_max {
            self.overflow += 1;
            return;
        }

        // Half-open bins, with the final upper edge included. Searching the
        // actual edges also handles point populations and rounded tiny bins.
        let bin_idx = self
            .bins
            .partition_point(|bin| bin.upper <= value)
            .min(self.bins.len() - 1);

        self.bins[bin_idx].add(weight);
    }

    /// Add multiple samples
    pub fn add_all(&mut self, values: &[f64]) {
        for &v in values {
            self.add(v);
        }
    }
}

// =============================================================================
// Histogram Builder
// =============================================================================

/// Builder for creating histograms from data
#[derive(Debug, Clone)]
pub struct HistogramBuilder {
    /// Name for the histogram
    name: String,
    /// Number of bins
    bin_count: usize,
    /// Optional explicit range
    range: Option<(f64, f64)>,
    /// Extended range factor (margin around data)
    margin: f64,
}

impl Default for HistogramBuilder {
    fn default() -> Self {
        Self {
            name: String::new(),
            bin_count: 50,
            range: None,
            margin: 0.05,
        }
    }
}

impl HistogramBuilder {
    /// Create new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set histogram name
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    /// Set bin count
    pub fn bin_count(mut self, count: usize) -> Self {
        self.bin_count = count.max(1);
        self
    }

    /// Set explicit range
    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.range = Some((min, max));
        self
    }

    /// Build histogram from data
    pub fn build(self, data: &[f64]) -> Histogram {
        let mut valid_data = data.iter().copied().filter(|v| v.is_finite());
        let Some(first) = valid_data.next() else {
            return Histogram::new(&self.name, 0.0, 1.0, self.bin_count);
        };
        let (data_min, data_max) = valid_data.fold((first, first), |(min, max), value| {
            (min.min(value), max.max(value))
        });

        let (min, max) = if let Some((min, max)) = self.range {
            (min, max)
        } else {
            // Relative padding preserves pico/nano-scale populations. Equal
            // samples remain a single point bin; extreme padding saturates.
            let margin = data_max * self.margin - data_min * self.margin;
            (
                (data_min - margin).max(-f64::MAX),
                (data_max + margin).min(f64::MAX),
            )
        };

        let mut hist = Histogram::new(&self.name, min, max, self.bin_count);
        hist.add_all(data);
        hist
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiny_and_constant_populations_preserve_their_scale_and_counts() {
        let tiny = HistogramBuilder::new()
            .bin_count(3)
            .build(&[1e-15, 2e-15, 3e-15]);
        assert!(tiny.range().0 > 0.0 && tiny.range().1 < 4e-15);
        assert_eq!(
            tiny.bins.iter().map(|bin| bin.count).collect::<Vec<_>>(),
            vec![1, 1, 1]
        );
        for value in [0.0, f64::from_bits(1), 1e-15, 1.5, f64::MAX] {
            let point = HistogramBuilder::new().build(&[value; 9]);
            assert_eq!(point.range(), (value, value));
            assert_eq!(point.bins.len(), 1);
            assert_eq!(point.bins[0].count, 9);
            assert_eq!(point.bins[0].center(), value);
            assert_eq!((point.underflow, point.overflow), (0, 0));
        }
    }

    #[test]
    fn explicit_range_includes_both_endpoints_and_accounts_for_excluded_samples() {
        let hist = HistogramBuilder::new()
            .bin_count(2)
            .range(0.0, 2.0)
            .build(&[-1.0, 0.0, 1.0, 2.0, 3.0]);
        assert_eq!((hist.underflow, hist.overflow), (1, 1));
        assert_eq!(
            hist.bins.iter().map(|bin| bin.count).collect::<Vec<_>>(),
            vec![1, 2]
        );
        assert_eq!(hist.total_count, 5);
    }

    #[test]
    fn finite_extremes_and_adjacent_edges_do_not_overflow_or_lose_samples() {
        for samples in [vec![-f64::MAX, 0.0, f64::MAX], vec![1.0, 1.0_f64.next_up()]] {
            let hist = HistogramBuilder::new().bin_count(100).build(&samples);
            assert_eq!(
                hist.bins.iter().map(|bin| bin.count).sum::<usize>(),
                samples.len()
            );
            assert!(hist.bins.iter().all(|bin| bin.lower.is_finite()
                && bin.upper.is_finite()
                && bin.upper > bin.lower));
        }
    }

    #[test]
    fn nonfinite_samples_do_not_create_observations() {
        let hist = HistogramBuilder::new().name("empty").bin_count(4).build(&[
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
        ]);

        assert_eq!(hist.total_count, 0);
        assert_eq!(hist.bins.len(), 4);
        assert!(hist.bins.iter().all(|bin| bin.count == 0));
    }
}
