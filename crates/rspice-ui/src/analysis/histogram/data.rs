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

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    // =========================================================================
    // HistogramBin Tests
    // =========================================================================

    #[test]
    fn test_bin_new() {
        let bin = HistogramBin::new(0.0, 1.0);
        assert_eq!(bin.lower, 0.0);
        assert_eq!(bin.upper, 1.0);
        assert_eq!(bin.count, 0);
    }

    #[test]
    fn test_bin_center() {
        let bin = HistogramBin::new(2.0, 4.0);
        assert!(approx_eq(bin.center(), 3.0));
    }

    #[test]
    fn test_bin_width() {
        let bin = HistogramBin::new(0.0, 0.5);
        assert!(approx_eq(bin.width(), 0.5));
    }

    #[test]
    fn test_bin_contains() {
        let bin = HistogramBin::new(1.0, 2.0);
        assert!(bin.contains(1.0));
        assert!(bin.contains(1.5));
        assert!(!bin.contains(2.0)); // Upper is exclusive
        assert!(!bin.contains(0.5));
    }

    #[test]
    fn test_bin_add() {
        let mut bin = HistogramBin::new(0.0, 1.0);
        bin.add(1.0);
        bin.add(2.0);
        assert_eq!(bin.count, 2);
        assert!(approx_eq(bin.weight, 3.0));
    }

    #[test]
    fn test_bin_density() {
        let mut bin = HistogramBin::new(0.0, 0.5);
        bin.count = 10;
        // Density = count / (total * width) = 10 / (100 * 0.5) = 0.2
        assert!(approx_eq(bin.density(100), 0.2));
    }

    // =========================================================================
    // Histogram Tests
    // =========================================================================

    #[test]
    fn test_histogram_new() {
        let hist = Histogram::new("test", 0.0, 10.0, 10);
        assert_eq!(hist.bin_count(), 10);
        assert_eq!(hist.range(), (0.0, 10.0));
        assert!(approx_eq(hist.bin_width(), 1.0));
    }

    #[test]
    fn test_histogram_default() {
        let hist = Histogram::default();
        assert_eq!(hist.bin_count(), 0);
        assert_eq!(hist.total_count, 0);
    }

    #[test]
    fn test_histogram_add() {
        let mut hist = Histogram::new("test", 0.0, 10.0, 10);
        hist.add(5.5);
        assert_eq!(hist.total_count, 1);
        assert_eq!(hist.bins[5].count, 1);
    }

    #[test]
    fn test_histogram_add_all() {
        let mut hist = Histogram::new("test", 0.0, 10.0, 10);
        hist.add_all(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(hist.total_count, 5);
    }

    #[test]
    fn test_histogram_underflow() {
        let mut hist = Histogram::new("test", 0.0, 10.0, 10);
        hist.add(-1.0);
        assert_eq!(hist.underflow, 1);
        assert_eq!(hist.total_count, 1);
    }

    #[test]
    fn test_histogram_overflow() {
        let mut hist = Histogram::new("test", 0.0, 10.0, 10);
        hist.add(15.0);
        assert_eq!(hist.overflow, 1);
        assert_eq!(hist.total_count, 1);
    }

    #[test]
    fn test_histogram_clear() {
        let mut hist = Histogram::new("test", 0.0, 10.0, 10);
        hist.add_all(&[1.0, 2.0, 3.0]);
        hist.clear();
        assert_eq!(hist.total_count, 0);
        assert_eq!(hist.max_count(), 0);
    }

    #[test]
    fn test_histogram_max_count() {
        let mut hist = Histogram::new("test", 0.0, 10.0, 10);
        hist.add_all(&[5.1, 5.2, 5.3, 5.4, 5.5]); // All in bin 5
        assert_eq!(hist.max_count(), 5);
    }

    #[test]
    fn test_histogram_pdf() {
        let mut hist = Histogram::new("test", 0.0, 2.0, 2);
        hist.add_all(&[0.5, 0.5, 0.5, 1.5]); // 3 in first bin, 1 in second
        let pdf = hist.pdf();
        assert_eq!(pdf.len(), 2);
        // First bin: 3 / (4 * 1.0) = 0.75
        assert!((pdf[0] - 0.75).abs() < 0.01);
    }

    #[test]
    fn test_histogram_cdf() {
        let mut hist = Histogram::new("test", 0.0, 4.0, 4);
        hist.add_all(&[0.5, 1.5, 2.5, 3.5]); // 1 in each bin
        let cdf = hist.cdf();
        assert_eq!(cdf.len(), 4);
        assert!((cdf[0] - 0.25).abs() < 0.01);
        assert!((cdf[1] - 0.50).abs() < 0.01);
        assert!((cdf[2] - 0.75).abs() < 0.01);
        assert!((cdf[3] - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_histogram_bin_centers() {
        let hist = Histogram::new("test", 0.0, 4.0, 4);
        let centers = hist.bin_centers();
        assert_eq!(centers.len(), 4);
        assert!(approx_eq(centers[0], 0.5));
        assert!(approx_eq(centers[1], 1.5));
    }

    #[test]
    fn test_histogram_percentile() {
        let mut hist = Histogram::new("test", 0.0, 100.0, 100);
        for i in 0..100 {
            hist.add(i as f64 + 0.5);
        }

        let p50 = hist.percentile(0.5).unwrap();
        assert!((p50 - 50.0).abs() < 2.0);

        let p25 = hist.percentile(0.25).unwrap();
        assert!((p25 - 25.0).abs() < 2.0);
    }

    #[test]
    fn test_histogram_percentile_empty() {
        let hist = Histogram::new("test", 0.0, 10.0, 10);
        assert!(hist.percentile(0.5).is_none());
    }

    #[test]
    fn test_histogram_nan_ignored() {
        let mut hist = Histogram::new("test", 0.0, 10.0, 10);
        hist.add(f64::NAN);
        assert_eq!(hist.total_count, 0);
    }

    // =========================================================================
    // HistogramBuilder Tests
    // =========================================================================

    #[test]
    fn test_builder_default() {
        let builder = HistogramBuilder::new();
        assert_eq!(builder.bin_count, 50);
    }

    #[test]
    fn test_builder_chain() {
        let builder = HistogramBuilder::new()
            .name("Test Histogram")
            .bin_count(20)
            .margin(0.1);

        assert_eq!(builder.name, "Test Histogram");
        assert_eq!(builder.bin_count, 20);
    }

    #[test]
    fn test_builder_with_range() {
        let hist = HistogramBuilder::new()
            .range(0.0, 100.0)
            .bin_count(10)
            .build(&[50.0]);

        assert_eq!(hist.range(), (0.0, 100.0));
        assert_eq!(hist.total_count, 1);
    }

    #[test]
    fn test_builder_auto_range() {
        let hist = HistogramBuilder::new()
            .bin_count(10)
            .build(&[10.0, 20.0, 30.0, 40.0, 50.0]);

        let (min, max) = hist.range();
        assert!(min < 10.0);
        assert!(max > 50.0);
    }

    #[test]
    fn test_builder_empty_data() {
        let hist = HistogramBuilder::new().build(&[]);
        assert_eq!(hist.total_count, 0);
    }

    #[test]
    fn test_builder_single_value() {
        let hist = HistogramBuilder::new()
            .bin_count(10)
            .build(&[5.0, 5.0, 5.0]);

        assert_eq!(hist.total_count, 3);
    }

    // =========================================================================
    // Integration Tests
    // =========================================================================

    #[test]
    fn test_gaussian_distribution() {
        // Simulate Gaussian samples
        let samples: Vec<f64> = (0..1000)
            .map(|i| (i as f64 - 500.0) / 100.0)
            .collect();

        let hist = HistogramBuilder::new()
            .name("Gaussian")
            .bin_count(20)
            .build(&samples);

        assert_eq!(hist.total_count, 1000);
        assert!(hist.max_count() > 0);
    }

    #[test]
    fn test_uniform_distribution() {
        let samples: Vec<f64> = (0..100).map(|i| i as f64 + 0.5).collect();

        // Use explicit range to ensure even distribution
        let hist = HistogramBuilder::new()
            .bin_count(10)
            .range(0.0, 100.0)
            .build(&samples);

        // Each bin should have exactly 10 counts
        assert_eq!(hist.total_count, 100);
        for bin in &hist.bins {
            assert_eq!(bin.count, 10);
        }
    }
}
