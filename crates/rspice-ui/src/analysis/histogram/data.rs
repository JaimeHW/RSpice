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
        (self.lower + self.upper) / 2.0
    }

    /// Bin width
    pub fn width(&self) -> f64 {
        self.upper - self.lower
    }

    /// Check if value is in bin [lower, upper)
    pub fn contains(&self, value: f64) -> bool {
        value >= self.lower && value < self.upper
    }

    /// Add a sample to this bin
    pub fn add(&mut self, weight: f64) {
        self.count += 1;
        self.weight += weight;
    }

    /// Density (count / width) for PDF
    pub fn density(&self, total: usize) -> f64 {
        if total == 0 || self.width() == 0.0 {
            return 0.0;
        }
        (self.count as f64) / (total as f64 * self.width())
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
        let bin_count = bin_count.max(1);
        let width = (max - min) / bin_count as f64;

        let mut bins = Vec::with_capacity(bin_count);
        for i in 0..bin_count {
            let lower = min + i as f64 * width;
            let upper = min + (i + 1) as f64 * width;
            bins.push(HistogramBin::new(lower, upper));
        }

        Self {
            name: name.to_string(),
            bins,
            ..Default::default()
        }
    }

    /// Number of bins
    pub fn bin_count(&self) -> usize {
        self.bins.len()
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

    /// Bin width (assuming uniform bins)
    pub fn bin_width(&self) -> f64 {
        if self.bins.is_empty() {
            return 0.0;
        }
        self.bins[0].width()
    }

    /// Maximum bin count
    pub fn max_count(&self) -> usize {
        self.bins.iter().map(|b| b.count).max().unwrap_or(0)
    }

    /// Add a sample value
    pub fn add(&mut self, value: f64) {
        self.add_weighted(value, 1.0);
    }

    /// Add a weighted sample
    pub fn add_weighted(&mut self, value: f64, weight: f64) {
        if !value.is_finite() {
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

        if value >= hist_max {
            self.overflow += 1;
            return;
        }

        // Find the correct bin
        let width = self.bin_width();
        if width <= 0.0 {
            return;
        }

        let bin_idx = ((value - hist_min) / width) as usize;
        let bin_idx = bin_idx.min(self.bins.len() - 1);

        self.bins[bin_idx].add(weight);
    }

    /// Add multiple samples
    pub fn add_all(&mut self, values: &[f64]) {
        for &v in values {
            self.add(v);
        }
    }

    /// Clear all data
    pub fn clear(&mut self) {
        for bin in &mut self.bins {
            bin.count = 0;
            bin.weight = 0.0;
        }
        self.total_count = 0;
        self.total_weight = 0.0;
        self.underflow = 0;
        self.overflow = 0;
        self.data_min = f64::MAX;
        self.data_max = f64::MIN;
    }

    /// Get normalized (PDF) values
    pub fn pdf(&self) -> Vec<f64> {
        self.bins
            .iter()
            .map(|b| b.density(self.total_count))
            .collect()
    }

    /// Get cumulative distribution (CDF) values
    pub fn cdf(&self) -> Vec<f64> {
        let mut cumulative = 0.0;
        let total = self.total_count as f64;

        self.bins
            .iter()
            .map(|b| {
                cumulative += b.count as f64 / total;
                cumulative
            })
            .collect()
    }

    /// Get bin centers
    pub fn bin_centers(&self) -> Vec<f64> {
        self.bins.iter().map(|b| b.center()).collect()
    }

    /// Get percentile value (0.0 to 1.0)
    pub fn percentile(&self, p: f64) -> Option<f64> {
        if self.total_count == 0 || !(0.0..=1.0).contains(&p) {
            return None;
        }

        let target = p * self.total_count as f64;
        let mut cumulative = 0.0;

        for bin in &self.bins {
            let prev_cumulative = cumulative;
            cumulative += bin.count as f64;

            if cumulative >= target {
                // Interpolate within bin
                let fraction = if bin.count > 0 {
                    (target - prev_cumulative) / bin.count as f64
                } else {
                    0.5
                };
                return Some(bin.lower + fraction * bin.width());
            }
        }

        // Return max if we reach here
        Some(self.bins.last()?.upper)
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

    /// Set margin factor
    pub fn margin(mut self, margin: f64) -> Self {
        self.margin = margin.clamp(0.0, 0.5);
        self
    }

    /// Build histogram from data
    pub fn build(self, data: &[f64]) -> Histogram {
        let valid_data: Vec<f64> = data.iter().copied().filter(|v| v.is_finite()).collect();

        if valid_data.is_empty() {
            return Histogram::new(&self.name, 0.0, 1.0, self.bin_count);
        }

        let (min, max) = if let Some((min, max)) = self.range {
            (min, max)
        } else {
            let data_min = valid_data.iter().copied().fold(f64::MAX, f64::min);
            let data_max = valid_data.iter().copied().fold(f64::MIN, f64::max);

            // Add margin
            let range = data_max - data_min;
            if range < 1e-10 {
                (data_min - 0.5, data_max + 0.5)
            } else {
                let margin = range * self.margin;
                (data_min - margin, data_max + margin)
            }
        };

        let mut hist = Histogram::new(&self.name, min, max, self.bin_count);
        hist.add_all(&valid_data);
        hist
    }
}

// =============================================================================
// Tests
// =============================================================================

