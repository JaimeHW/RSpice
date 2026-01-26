//! Histogram Statistics
//!
//! Statistical calculations for histogram data analysis.

use super::data::Histogram;
use std::f64::consts::PI;

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

    /// Format mean with appropriate precision
    pub fn format_mean(&self) -> String {
        format_si_value(self.mean)
    }

    /// Format std dev with appropriate precision
    pub fn format_std_dev(&self) -> String {
        format_si_value(self.std_dev)
    }

    /// Format range
    pub fn format_range(&self) -> String {
        format!(
            "{} to {}",
            format_si_value(self.min),
            format_si_value(self.max)
        )
    }

    /// Check if distribution is approximately normal
    pub fn is_approximately_normal(&self) -> bool {
        // Normal distribution has skewness ≈ 0 and kurtosis ≈ 3
        self.skewness.abs() < 0.5 && (self.kurtosis - 3.0).abs() < 1.0
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
// Distribution Fitting
// =============================================================================

/// Normal distribution parameters
#[derive(Debug, Clone, Copy, Default)]
pub struct NormalParams {
    pub mean: f64,
    pub std_dev: f64,
}

impl NormalParams {
    /// Fit normal distribution to histogram
    pub fn fit(hist: &Histogram) -> Self {
        let stats = HistogramStats::from_histogram(hist);
        Self {
            mean: stats.mean,
            std_dev: stats.std_dev,
        }
    }

    /// PDF value at x
    pub fn pdf(&self, x: f64) -> f64 {
        if self.std_dev <= 0.0 {
            return 0.0;
        }
        let z = (x - self.mean) / self.std_dev;
        (-0.5 * z * z).exp() / (self.std_dev * (2.0 * PI).sqrt())
    }

    /// CDF value at x (using error function approximation)
    pub fn cdf(&self, x: f64) -> f64 {
        if self.std_dev <= 0.0 {
            return if x >= self.mean { 1.0 } else { 0.0 };
        }
        let z = (x - self.mean) / (self.std_dev * 2.0_f64.sqrt());
        0.5 * (1.0 + erf_approx(z))
    }
}

/// Error function approximation
fn erf_approx(x: f64) -> f64 {
    // Abramowitz and Stegun approximation
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

    sign * y
}

/// Log-normal distribution parameters
#[derive(Debug, Clone, Copy, Default)]
pub struct LogNormalParams {
    pub mu: f64,    // log-space mean
    pub sigma: f64, // log-space std dev
}

impl LogNormalParams {
    /// Fit log-normal distribution to histogram (x > 0 data)
    pub fn fit(hist: &Histogram) -> Option<Self> {
        if hist.total_count == 0 {
            return None;
        }

        // Check all data is positive
        if hist.data_min <= 0.0 {
            return None;
        }

        let n = hist.total_count as f64;

        // Calculate log-space moments
        let mu: f64 = hist
            .bins
            .iter()
            .filter(|b| b.center() > 0.0)
            .map(|b| b.center().ln() * b.count as f64)
            .sum::<f64>()
            / n;

        let sigma_sq: f64 = hist
            .bins
            .iter()
            .filter(|b| b.center() > 0.0)
            .map(|b| (b.center().ln() - mu).powi(2) * b.count as f64)
            .sum::<f64>()
            / n;

        Some(Self {
            mu,
            sigma: sigma_sq.sqrt(),
        })
    }

    /// PDF value at x (x > 0)
    pub fn pdf(&self, x: f64) -> f64 {
        if x <= 0.0 || self.sigma <= 0.0 {
            return 0.0;
        }
        let z = (x.ln() - self.mu) / self.sigma;
        (-0.5 * z * z).exp() / (x * self.sigma * (2.0 * PI).sqrt())
    }
}

// =============================================================================
// Utility Functions
// =============================================================================

/// Format value with SI prefix
fn format_si_value(value: f64) -> String {
    let abs_val = value.abs();
    if abs_val >= 1e12 {
        format!("{:.3} T", value / 1e12)
    } else if abs_val >= 1e9 {
        format!("{:.3} G", value / 1e9)
    } else if abs_val >= 1e6 {
        format!("{:.3} M", value / 1e6)
    } else if abs_val >= 1e3 {
        format!("{:.3} k", value / 1e3)
    } else if abs_val >= 1.0 {
        format!("{:.3}", value)
    } else if abs_val >= 1e-3 {
        format!("{:.3} m", value * 1e3)
    } else if abs_val >= 1e-6 {
        format!("{:.3} µ", value * 1e6)
    } else if abs_val >= 1e-9 {
        format!("{:.3} n", value * 1e9)
    } else if abs_val >= 1e-12 {
        format!("{:.3} p", value * 1e12)
    } else if abs_val >= 1e-15 {
        format!("{:.3} f", value * 1e15)
    } else {
        format!("{:.3e}", value)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::super::data::HistogramBuilder;
    use super::*;

    const EPSILON: f64 = 1e-6;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    fn approx_eq_rel(a: f64, b: f64, rel_tol: f64) -> bool {
        if b.abs() < EPSILON {
            a.abs() < EPSILON
        } else {
            ((a - b) / b).abs() < rel_tol
        }
    }

    // =========================================================================
    // HistogramStats Tests
    // =========================================================================

    #[test]
    fn test_stats_from_empty() {
        let hist = Histogram::default();
        let stats = HistogramStats::from_histogram(&hist);
        assert_eq!(stats.count, 0);
    }

    #[test]
    fn test_stats_uniform() {
        // Uniform distribution from 0 to 100
        let data: Vec<f64> = (0..100).map(|i| i as f64 + 0.5).collect();
        let hist = HistogramBuilder::new()
            .bin_count(10)
            .range(0.0, 100.0)
            .build(&data);

        let stats = HistogramStats::from_histogram(&hist);

        assert_eq!(stats.count, 100);
        // Mean should be roughly 50
        assert!((stats.mean - 50.0).abs() < 5.0);
        // Min/max
        assert!(approx_eq(stats.min, 0.5));
        assert!(approx_eq(stats.max, 99.5));
    }

    #[test]
    fn test_stats_percentiles() {
        let data: Vec<f64> = (1..=100).map(|i| i as f64).collect();
        let hist = HistogramBuilder::new()
            .bin_count(100)
            .range(0.0, 101.0)
            .build(&data);

        let stats = HistogramStats::from_histogram(&hist);

        // Median should be ~50
        assert!((stats.median - 50.0).abs() < 5.0);
        // Q1 should be ~25
        assert!((stats.q1 - 25.0).abs() < 5.0);
        // Q3 should be ~75
        assert!((stats.q3 - 75.0).abs() < 5.0);
    }

    #[test]
    fn test_stats_iqr() {
        let data: Vec<f64> = (0..100).map(|i| i as f64).collect();
        let hist = HistogramBuilder::new().bin_count(20).build(&data);

        let stats = HistogramStats::from_histogram(&hist);

        // IQR should be roughly Q3 - Q1 ≈ 50
        assert!((stats.iqr - 50.0).abs() < 10.0);
    }

    #[test]
    fn test_stats_cv() {
        let data: Vec<f64> = (50..150).map(|i| i as f64).collect();
        let hist = HistogramBuilder::new().build(&data);

        let stats = HistogramStats::from_histogram(&hist);

        // CV = std_dev / mean, should be reasonable
        assert!(stats.cv > 0.0 && stats.cv < 1.0);
    }

    #[test]
    fn test_is_approximately_normal() {
        // Symmetric data should have low skewness
        let data: Vec<f64> = (-50..50).map(|i| i as f64).collect();
        let hist = HistogramBuilder::new().build(&data);

        let stats = HistogramStats::from_histogram(&hist);

        // Should have low skewness for symmetric data
        assert!(stats.skewness.abs() < 0.5);
    }

    // =========================================================================
    // NormalParams Tests
    // =========================================================================

    #[test]
    fn test_normal_fit() {
        let data: Vec<f64> = (-50..50).map(|i| i as f64).collect();
        let hist = HistogramBuilder::new().build(&data);

        let params = NormalParams::fit(&hist);

        // Mean should be close to 0
        assert!(params.mean.abs() < 5.0);
        assert!(params.std_dev > 0.0);
    }

    #[test]
    fn test_normal_pdf() {
        let params = NormalParams {
            mean: 0.0,
            std_dev: 1.0,
        };

        // PDF at mean should be 1/sqrt(2π)
        let pdf_at_mean = params.pdf(0.0);
        let expected = 1.0 / (2.0 * PI).sqrt();
        assert!(approx_eq_rel(pdf_at_mean, expected, 0.01));
    }

    #[test]
    fn test_normal_cdf() {
        let params = NormalParams {
            mean: 0.0,
            std_dev: 1.0,
        };

        // CDF at mean should be 0.5
        assert!((params.cdf(0.0) - 0.5).abs() < 0.01);

        // CDF at -inf should be ~0, at +inf should be ~1
        assert!(params.cdf(-5.0) < 0.001);
        assert!(params.cdf(5.0) > 0.999);
    }

    // =========================================================================
    // LogNormalParams Tests
    // =========================================================================

    #[test]
    fn test_lognormal_fit() {
        // Positive data
        let data: Vec<f64> = (1..100).map(|i| i as f64).collect();
        let hist = HistogramBuilder::new().build(&data);

        let params = LogNormalParams::fit(&hist);
        assert!(params.is_some());
    }

    #[test]
    fn test_lognormal_fit_negative_data() {
        // Data with negative values should fail
        let data: Vec<f64> = (-10..10).map(|i| i as f64).collect();
        let hist = HistogramBuilder::new().build(&data);

        let params = LogNormalParams::fit(&hist);
        assert!(params.is_none());
    }

    #[test]
    fn test_lognormal_pdf() {
        let params = LogNormalParams {
            mu: 0.0,
            sigma: 1.0,
        };

        // PDF should be positive for x > 0
        assert!(params.pdf(1.0) > 0.0);
        assert!(params.pdf(0.5) > 0.0);

        // PDF should be 0 for x <= 0
        assert_eq!(params.pdf(0.0), 0.0);
        assert_eq!(params.pdf(-1.0), 0.0);
    }

    // =========================================================================
    // ERF Tests
    // =========================================================================

    #[test]
    fn test_erf_zero() {
        assert!(approx_eq(erf_approx(0.0), 0.0));
    }

    #[test]
    fn test_erf_large() {
        assert!(erf_approx(3.0) > 0.99);
        assert!(erf_approx(-3.0) < -0.99);
    }

    #[test]
    fn test_erf_symmetry() {
        let x = 1.5;
        assert!(approx_eq(erf_approx(x), -erf_approx(-x)));
    }

    // =========================================================================
    // Format Tests
    // =========================================================================

    #[test]
    fn test_format_si_value() {
        assert!(format_si_value(1500.0).contains("k"));
        assert!(format_si_value(0.001).contains("m"));
        assert!(format_si_value(1e-9).contains("n"));
    }
}
