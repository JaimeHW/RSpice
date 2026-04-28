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
