//! Histogram Statistics
//!
//! Statistical calculations for histogram data analysis.

use super::data::Histogram;

// =============================================================================
// Histogram Statistics
// =============================================================================

/// Statistical measures from a histogram
#[derive(Debug, Clone, Default)]
pub struct HistogramStats {
    /// Sample count
    pub count: usize,
    /// Mean (first moment)
    pub mean: f64,
    /// Variance
    pub variance: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Minimum value
    pub min: f64,
    /// Maximum value
    pub max: f64,
    /// Median (50th percentile)
    pub median: f64,
    /// Skewness (third standardized moment)
    pub skewness: f64,
    /// Kurtosis (fourth standardized moment)
    pub kurtosis: f64,
    /// 1st percentile
    pub p01: f64,
    /// 5th percentile
    pub p05: f64,
    /// 25th percentile (Q1)
    pub q1: f64,
    /// 75th percentile (Q3)
    pub q3: f64,
    /// 95th percentile
    pub p95: f64,
    /// 99th percentile
    pub p99: f64,
    /// Inter-quartile range (Q3 - Q1)
    pub iqr: f64,
    /// Coefficient of variation (std_dev / mean)
    pub cv: f64,
}

impl HistogramStats {
    /// Calculate statistics from histogram
    pub fn from_histogram(hist: &Histogram) -> Self {
        if hist.total_count == 0 {
            return Self::default();
        }

        let mut stats = Self {
            count: hist.total_count,
            min: hist.data_min,
            max: hist.data_max,
            ..Default::default()
        };

        // Calculate moments from bin data
        let (mean, variance, skewness, kurtosis) = calculate_moments(hist);
        stats.mean = mean;
        stats.variance = variance;
        stats.std_dev = variance.sqrt();
        stats.skewness = skewness;
        stats.kurtosis = kurtosis;

        // Percentiles
        stats.median = hist.percentile(0.5).unwrap_or(mean);
        stats.p01 = hist.percentile(0.01).unwrap_or(stats.min);
        stats.p05 = hist.percentile(0.05).unwrap_or(stats.min);
        stats.q1 = hist.percentile(0.25).unwrap_or(stats.min);
        stats.q3 = hist.percentile(0.75).unwrap_or(stats.max);
        stats.p95 = hist.percentile(0.95).unwrap_or(stats.max);
        stats.p99 = hist.percentile(0.99).unwrap_or(stats.max);
        stats.iqr = stats.q3 - stats.q1;

        // Coefficient of variation
        if mean.abs() > 1e-15 {
            stats.cv = stats.std_dev / mean.abs();
        }

        stats
    }

}

// =============================================================================
// Moment Calculations
// =============================================================================

/// Calculate statistical moments from histogram bins
fn calculate_moments(hist: &Histogram) -> (f64, f64, f64, f64) {
    if hist.total_count == 0 {
        return (0.0, 0.0, 0.0, 0.0);
    }

    let n = hist.total_count as f64;

    // Mean (first moment)
    let mean: f64 = hist
        .bins
        .iter()
        .map(|b| b.center() * b.count as f64)
        .sum::<f64>()
        / n;

    // Variance (second central moment)
    let variance: f64 = hist
        .bins
        .iter()
        .map(|b| (b.center() - mean).powi(2) * b.count as f64)
        .sum::<f64>()
        / n;

    let std_dev = variance.sqrt();

    // Skewness (third standardized moment)
    let skewness = if std_dev > 1e-15 {
        hist.bins
            .iter()
            .map(|b| ((b.center() - mean) / std_dev).powi(3) * b.count as f64)
            .sum::<f64>()
            / n
    } else {
        0.0
    };

    // Kurtosis (fourth standardized moment)
    let kurtosis = if std_dev > 1e-15 {
        hist.bins
            .iter()
            .map(|b| ((b.center() - mean) / std_dev).powi(4) * b.count as f64)
            .sum::<f64>()
            / n
    } else {
        0.0
    };

    (mean, variance, skewness, kurtosis)
}

// =============================================================================
// Tests
// =============================================================================
