//! Histogram Viewer for Monte Carlo Analysis
//!
//! Commercial-grade statistical visualization for Monte Carlo simulation results.
//! Features:
//!
//! - Histogram with adjustable bin count
//! - Normal/Gaussian fit overlay
//! - Statistical metrics (mean, std, min, max, percentiles)
//! - CDF (Cumulative Distribution Function) view
//! - PDF (Probability Density Function) normalization
//! - Yield/pass-fail analysis
//! - Multiple distribution overlays
//! - Export statistics

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

// =============================================================================
// Statistical Distribution Types
// =============================================================================

/// Distribution type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DistributionType {
    /// Normal/Gaussian distribution
    #[default]
    Normal,
    /// Uniform distribution
    Uniform,
    /// Log-normal distribution
    LogNormal,
    /// Unknown/empirical
    Empirical,
}

impl DistributionType {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Normal => "Normal (Gaussian)",
            Self::Uniform => "Uniform",
            Self::LogNormal => "Log-Normal",
            Self::Empirical => "Empirical",
        }
    }

    pub fn all() -> &'static [DistributionType] {
        &[
            DistributionType::Normal,
            DistributionType::Uniform,
            DistributionType::LogNormal,
            DistributionType::Empirical,
        ]
    }
}

/// Histogram display mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum HistogramMode {
    /// Raw counts
    #[default]
    Count,
    /// Probability Density Function (normalized, area = 1)
    PDF,
    /// Cumulative Distribution Function
    CDF,
    /// Complementary CDF (1 - CDF)
    CCDF,
}

impl HistogramMode {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Count => "Count",
            Self::PDF => "PDF",
            Self::CDF => "CDF",
            Self::CCDF => "CCDF (1-CDF)",
        }
    }

    pub fn all() -> &'static [HistogramMode] {
        &[
            HistogramMode::Count,
            HistogramMode::PDF,
            HistogramMode::CDF,
            HistogramMode::CCDF,
        ]
    }

    pub fn y_axis_label(&self) -> &'static str {
        match self {
            Self::Count => "Count",
            Self::PDF => "Probability Density",
            Self::CDF => "Cumulative Probability",
            Self::CCDF => "Complementary Probability",
        }
    }
}

// =============================================================================
// Statistical Metrics
// =============================================================================

/// Comprehensive statistics for a data set
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Statistics {
    /// Number of samples
    pub count: usize,

    /// Mean (average)
    pub mean: f64,

    /// Median (50th percentile)
    pub median: f64,

    /// Mode (most frequent value bin center)
    pub mode: f64,

    /// Standard deviation
    pub std_dev: f64,

    /// Variance
    pub variance: f64,

    /// Minimum value
    pub min: f64,

    /// Maximum value
    pub max: f64,

    /// Range (max - min)
    pub range: f64,

    /// 1st percentile
    pub p01: f64,

    /// 5th percentile
    pub p05: f64,

    /// 10th percentile
    pub p10: f64,

    /// 25th percentile (Q1)
    pub p25: f64,

    /// 75th percentile (Q3)
    pub p75: f64,

    /// 90th percentile
    pub p90: f64,

    /// 95th percentile
    pub p95: f64,

    /// 99th percentile
    pub p99: f64,

    /// Interquartile range (Q3 - Q1)
    pub iqr: f64,

    /// Skewness
    pub skewness: f64,

    /// Kurtosis (excess)
    pub kurtosis: f64,

    /// Coefficient of variation (std/mean)
    pub cv: f64,

    /// Standard error of mean
    pub sem: f64,

    /// 3-sigma lower bound
    pub sigma3_low: f64,

    /// 3-sigma upper bound
    pub sigma3_high: f64,

    /// 6-sigma lower bound
    pub sigma6_low: f64,

    /// 6-sigma upper bound
    pub sigma6_high: f64,
}

impl Statistics {
    /// Calculate statistics from data
    pub fn from_data(data: &[f64]) -> Self {
        if data.is_empty() {
            return Self::default();
        }

        let count = data.len();
        let n = count as f64;

        // Sort for percentiles
        let mut sorted = data.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Basic stats
        let sum: f64 = data.iter().sum();
        let mean = sum / n;

        let min = sorted[0];
        let max = sorted[count - 1];
        let range = max - min;

        let median = percentile(&sorted, 0.5);

        // Variance and std dev
        let sum_sq_diff: f64 = data.iter().map(|x| (x - mean).powi(2)).sum();
        let variance = if count > 1 {
            sum_sq_diff / (n - 1.0)
        } else {
            0.0
        };
        let std_dev = variance.sqrt();

        // Percentiles
        let p01 = percentile(&sorted, 0.01);
        let p05 = percentile(&sorted, 0.05);
        let p10 = percentile(&sorted, 0.10);
        let p25 = percentile(&sorted, 0.25);
        let p75 = percentile(&sorted, 0.75);
        let p90 = percentile(&sorted, 0.90);
        let p95 = percentile(&sorted, 0.95);
        let p99 = percentile(&sorted, 0.99);

        let iqr = p75 - p25;

        // Skewness
        let skewness = if std_dev > 1e-15 && count > 2 {
            let m3: f64 = data.iter().map(|x| ((x - mean) / std_dev).powi(3)).sum();
            m3 / n
        } else {
            0.0
        };

        // Kurtosis (excess)
        let kurtosis = if std_dev > 1e-15 && count > 3 {
            let m4: f64 = data.iter().map(|x| ((x - mean) / std_dev).powi(4)).sum();
            m4 / n - 3.0
        } else {
            0.0
        };

        // Mode (from histogram peak)
        let mode = find_mode(&sorted, 20);

        // Derived metrics
        let cv = if mean.abs() > 1e-15 {
            (std_dev / mean.abs()) * 100.0
        } else {
            0.0
        };

        let sem = std_dev / n.sqrt();

        let sigma3_low = mean - 3.0 * std_dev;
        let sigma3_high = mean + 3.0 * std_dev;
        let sigma6_low = mean - 6.0 * std_dev;
        let sigma6_high = mean + 6.0 * std_dev;

        Self {
            count,
            mean,
            median,
            mode,
            std_dev,
            variance,
            min,
            max,
            range,
            p01,
            p05,
            p10,
            p25,
            p75,
            p90,
            p95,
            p99,
            iqr,
            skewness,
            kurtosis,
            cv,
            sem,
            sigma3_low,
            sigma3_high,
            sigma6_low,
            sigma6_high,
        }
    }

    /// Check if values are normally distributed (rough test)
    pub fn is_approximately_normal(&self) -> bool {
        // Check skewness and kurtosis
        self.skewness.abs() < 2.0 && self.kurtosis.abs() < 7.0
    }

    /// Calculate Cpk (process capability) given spec limits
    pub fn cpk(&self, lower_spec: f64, upper_spec: f64) -> f64 {
        if self.std_dev < 1e-15 {
            return f64::INFINITY;
        }
        let cpu = (upper_spec - self.mean) / (3.0 * self.std_dev);
        let cpl = (self.mean - lower_spec) / (3.0 * self.std_dev);
        cpu.min(cpl)
    }

    /// Calculate Cp (process capability)
    pub fn cp(&self, lower_spec: f64, upper_spec: f64) -> f64 {
        if self.std_dev < 1e-15 {
            return f64::INFINITY;
        }
        (upper_spec - lower_spec) / (6.0 * self.std_dev)
    }

    /// Calculate yield given spec limits
    pub fn yield_percent(&self, lower_spec: f64, upper_spec: f64, data: &[f64]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        let pass_count = data
            .iter()
            .filter(|&&x| x >= lower_spec && x <= upper_spec)
            .count();
        (pass_count as f64 / data.len() as f64) * 100.0
    }
}

/// Calculate percentile from sorted data
fn percentile(sorted_data: &[f64], p: f64) -> f64 {
    if sorted_data.is_empty() {
        return 0.0;
    }
    if p <= 0.0 {
        return sorted_data[0];
    }
    if p >= 1.0 {
        return sorted_data[sorted_data.len() - 1];
    }

    let n = sorted_data.len() as f64;
    let idx = (n - 1.0) * p;
    let low = idx.floor() as usize;
    let high = idx.ceil() as usize;
    let frac = idx - low as f64;

    if low >= sorted_data.len() {
        sorted_data[sorted_data.len() - 1]
    } else if high >= sorted_data.len() {
        sorted_data[low]
    } else {
        sorted_data[low] * (1.0 - frac) + sorted_data[high] * frac
    }
}

/// Find mode using histogram approach
fn find_mode(sorted_data: &[f64], num_bins: usize) -> f64 {
    if sorted_data.is_empty() {
        return 0.0;
    }

    let min = sorted_data[0];
    let max = sorted_data[sorted_data.len() - 1];
    if (max - min).abs() < 1e-15 {
        return min;
    }

    let bin_width = (max - min) / num_bins as f64;
    let mut counts = vec![0usize; num_bins];

    for &val in sorted_data {
        let bin = ((val - min) / bin_width) as usize;
        let bin = bin.min(num_bins - 1);
        counts[bin] += 1;
    }

    let max_bin = counts
        .iter()
        .enumerate()
        .max_by_key(|(_, &c)| c)
        .map(|(i, _)| i)
        .unwrap_or(0);

    min + (max_bin as f64 + 0.5) * bin_width
}

// =============================================================================
// Histogram Data
// =============================================================================

/// Histogram bin
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistogramBin {
    /// Bin left edge
    pub left: f64,
    /// Bin right edge
    pub right: f64,
    /// Bin center
    pub center: f64,
    /// Count in this bin
    pub count: usize,
    /// Normalized count (for PDF)
    pub density: f64,
    /// Cumulative count
    pub cumulative: usize,
    /// Cumulative fraction (for CDF)
    pub cdf: f64,
}

/// Complete histogram data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistogramData {
    /// Variable name
    pub name: String,

    /// Unit (if any)
    pub unit: Option<String>,

    /// Raw data values
    pub values: Vec<f64>,

    /// Computed bins
    pub bins: Vec<HistogramBin>,

    /// Number of bins
    pub num_bins: usize,

    /// Computed statistics
    pub stats: Statistics,
}

impl HistogramData {
    /// Create histogram from data
    pub fn from_data(name: &str, data: Vec<f64>, num_bins: usize) -> Self {
        let stats = Statistics::from_data(&data);
        let bins = compute_histogram_bins(&data, num_bins);

        Self {
            name: name.to_string(),
            unit: None,
            values: data,
            bins,
            num_bins,
            stats,
        }
    }

    /// Set unit
    pub fn with_unit(mut self, unit: &str) -> Self {
        self.unit = Some(unit.to_string());
        self
    }

    /// Recompute with different bin count
    pub fn rebin(&mut self, num_bins: usize) {
        self.num_bins = num_bins;
        self.bins = compute_histogram_bins(&self.values, num_bins);
    }

    /// Get bin values for display in given mode
    pub fn get_display_values(&self, mode: HistogramMode) -> Vec<f64> {
        match mode {
            HistogramMode::Count => self.bins.iter().map(|b| b.count as f64).collect(),
            HistogramMode::PDF => self.bins.iter().map(|b| b.density).collect(),
            HistogramMode::CDF => self.bins.iter().map(|b| b.cdf).collect(),
            HistogramMode::CCDF => self.bins.iter().map(|b| 1.0 - b.cdf).collect(),
        }
    }

    /// Get Gaussian fit values at bin centers
    pub fn gaussian_fit_values(&self) -> Vec<f64> {
        let mu = self.stats.mean;
        let sigma = self.stats.std_dev;

        if sigma < 1e-15 {
            return vec![0.0; self.bins.len()];
        }

        self.bins
            .iter()
            .map(|b| gaussian_pdf(b.center, mu, sigma))
            .collect()
    }

    /// Get max display value for given mode
    pub fn max_display_value(&self, mode: HistogramMode) -> f64 {
        let values = self.get_display_values(mode);
        values.iter().cloned().fold(0.0_f64, f64::max)
    }

    /// Calculate yield given specification limits
    pub fn yield_analysis(&self, lower: Option<f64>, upper: Option<f64>) -> YieldAnalysis {
        let lower = lower.unwrap_or(f64::NEG_INFINITY);
        let upper = upper.unwrap_or(f64::INFINITY);

        let mut pass = 0;
        let mut low_fail = 0;
        let mut high_fail = 0;

        for &v in &self.values {
            if v < lower {
                low_fail += 1;
            } else if v > upper {
                high_fail += 1;
            } else {
                pass += 1;
            }
        }

        let total = self.values.len();
        let yield_pct = if total > 0 {
            (pass as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        YieldAnalysis {
            total,
            pass,
            fail: low_fail + high_fail,
            low_fail,
            high_fail,
            yield_percent: yield_pct,
            lower_spec: if lower == f64::NEG_INFINITY {
                None
            } else {
                Some(lower)
            },
            upper_spec: if upper == f64::INFINITY {
                None
            } else {
                Some(upper)
            },
        }
    }
}

/// Compute histogram bins from data
fn compute_histogram_bins(data: &[f64], num_bins: usize) -> Vec<HistogramBin> {
    if data.is_empty() || num_bins == 0 {
        return Vec::new();
    }

    let min = data.iter().cloned().fold(f64::MAX, f64::min);
    let max = data.iter().cloned().fold(f64::MIN, f64::max);
    let range = max - min;

    // Handle constant data
    if range < 1e-15 {
        return vec![HistogramBin {
            left: min - 0.5,
            right: min + 0.5,
            center: min,
            count: data.len(),
            density: 1.0,
            cumulative: data.len(),
            cdf: 1.0,
        }];
    }

    let bin_width = range / num_bins as f64;
    let n = data.len() as f64;

    // Count values in each bin
    let mut counts = vec![0usize; num_bins];
    for &val in data {
        let bin = ((val - min) / bin_width) as usize;
        let bin = bin.min(num_bins - 1);
        counts[bin] += 1;
    }

    // Compute cumulative and normalized values
    let mut cumulative = 0usize;
    let mut bins = Vec::with_capacity(num_bins);

    for i in 0..num_bins {
        let left = min + i as f64 * bin_width;
        let right = left + bin_width;
        let center = left + bin_width / 2.0;
        let count = counts[i];

        cumulative += count;
        let density = (count as f64) / (n * bin_width);
        let cdf = cumulative as f64 / n;

        bins.push(HistogramBin {
            left,
            right,
            center,
            count,
            density,
            cumulative,
            cdf,
        });
    }

    bins
}

/// Gaussian PDF value
fn gaussian_pdf(x: f64, mu: f64, sigma: f64) -> f64 {
    if sigma < 1e-15 {
        return 0.0;
    }
    let z = (x - mu) / sigma;
    (1.0 / (sigma * (2.0 * PI).sqrt())) * (-0.5 * z * z).exp()
}

// =============================================================================
// Yield Analysis
// =============================================================================

/// Yield analysis results
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YieldAnalysis {
    /// Total samples
    pub total: usize,
    /// Passing samples
    pub pass: usize,
    /// Failing samples
    pub fail: usize,
    /// Failing below lower spec
    pub low_fail: usize,
    /// Failing above upper spec
    pub high_fail: usize,
    /// Yield percentage
    pub yield_percent: f64,
    /// Lower specification (if any)
    pub lower_spec: Option<f64>,
    /// Upper specification (if any)
    pub upper_spec: Option<f64>,
}

impl YieldAnalysis {
    /// Get defect rate (ppm)
    pub fn defect_ppm(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        (self.fail as f64 / self.total as f64) * 1_000_000.0
    }

    /// Get sigma level (approximate)
    pub fn sigma_level(&self) -> f64 {
        let dpmo = self.defect_ppm();
        if dpmo < 1.0 {
            return 6.0; // Cap at 6 sigma
        }
        if dpmo >= 1_000_000.0 {
            return 0.0;
        }

        // Approximate inverse normal CDF
        let p = 1.0 - (dpmo / 1_000_000.0);
        inverse_normal_cdf(p)
    }
}

/// Approximate inverse normal CDF
fn inverse_normal_cdf(p: f64) -> f64 {
    if p <= 0.0 {
        return f64::NEG_INFINITY;
    }
    if p >= 1.0 {
        return f64::INFINITY;
    }

    // Rational approximation
    let a = [
        -3.969683028665376e1,
        2.209460984245205e2,
        -2.759285104469687e2,
        1.383577518672690e2,
        -3.066479806614716e1,
        2.506628277459239e0,
    ];
    let b = [
        -5.447609879822406e1,
        1.615858368580409e2,
        -1.556989798598866e2,
        6.680131188771972e1,
        -1.328068155288572e1,
    ];
    let c = [
        -7.784894002430293e-3,
        -3.223964580411365e-1,
        -2.400758277161838e0,
        -2.549732539343734e0,
        4.374664141464968e0,
        2.938163982698783e0,
    ];
    let d = [
        7.784695709041462e-3,
        3.224671290700398e-1,
        2.445134137142996e0,
        3.754408661907416e0,
    ];

    let p_low = 0.02425;
    let p_high = 1.0 - p_low;

    if p < p_low {
        let q = (-2.0 * p.ln()).sqrt();
        (((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5])
            / ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0)
    } else if p <= p_high {
        let q = p - 0.5;
        let r = q * q;
        (((((a[0] * r + a[1]) * r + a[2]) * r + a[3]) * r + a[4]) * r + a[5]) * q
            / (((((b[0] * r + b[1]) * r + b[2]) * r + b[3]) * r + b[4]) * r + 1.0)
    } else {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        -(((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5])
            / ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0)
    }
}

// =============================================================================
// Histogram Viewer Configuration
// =============================================================================

/// Histogram viewer configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistogramConfig {
    /// Display mode
    pub mode: HistogramMode,

    /// Number of bins
    pub num_bins: usize,

    /// Show Gaussian fit overlay
    pub show_gaussian_fit: bool,

    /// Show statistics panel
    pub show_stats: bool,

    /// Show specification limits
    pub show_spec_limits: bool,

    /// Lower specification limit
    pub lower_spec: Option<f64>,

    /// Upper specification limit
    pub upper_spec: Option<f64>,

    /// Show mean line
    pub show_mean: bool,

    /// Show median line
    pub show_median: bool,

    /// Show sigma lines (±1σ, ±2σ, ±3σ)
    pub show_sigma_lines: bool,

    /// Bar color
    pub bar_color: String,

    /// Gaussian fit color
    pub fit_color: String,

    /// Spec limit color
    pub spec_color: String,

    /// Background color
    pub background_color: String,
}

impl Default for HistogramConfig {
    fn default() -> Self {
        Self {
            mode: HistogramMode::Count,
            num_bins: 30,
            show_gaussian_fit: true,
            show_stats: true,
            show_spec_limits: false,
            lower_spec: None,
            upper_spec: None,
            show_mean: true,
            show_median: false,
            show_sigma_lines: false,
            bar_color: "#4CAF50".to_string(),
            fit_color: "#ff9800".to_string(),
            spec_color: "#f44336".to_string(),
            background_color: "#1a1a2e".to_string(),
        }
    }
}

// =============================================================================
// Histogram Viewer State
// =============================================================================

/// Complete histogram viewer state
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HistogramViewerState {
    /// Configuration
    pub config: HistogramConfig,

    /// Histogram data sets
    pub histograms: Vec<HistogramData>,

    /// Active histogram index
    pub active_index: Option<usize>,

    /// Cached yield analysis
    pub yield_analysis: Option<YieldAnalysis>,
}

impl HistogramViewerState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a histogram
    pub fn add_histogram(&mut self, data: HistogramData) {
        self.histograms.push(data);
        if self.active_index.is_none() {
            self.active_index = Some(0);
        }
    }

    /// Get active histogram
    pub fn active_histogram(&self) -> Option<&HistogramData> {
        self.active_index.and_then(|i| self.histograms.get(i))
    }

    /// Get active histogram (mutable)
    pub fn active_histogram_mut(&mut self) -> Option<&mut HistogramData> {
        self.active_index.and_then(|i| self.histograms.get_mut(i))
    }

    /// Update yield analysis for active histogram
    pub fn update_yield_analysis(&mut self) {
        if let Some(hist) = self.active_histogram() {
            let analysis = hist.yield_analysis(self.config.lower_spec, self.config.upper_spec);
            self.yield_analysis = Some(analysis);
        }
    }

    /// Set bin count and recompute
    pub fn set_bin_count(&mut self, num_bins: usize) {
        self.config.num_bins = num_bins;
        if let Some(hist) = self.active_histogram_mut() {
            hist.rebin(num_bins);
        }
    }

    /// Clear all histograms
    pub fn clear(&mut self) {
        self.histograms.clear();
        self.active_index = None;
        self.yield_analysis = None;
    }
}

// =============================================================================
// Formatting Helpers
// =============================================================================

/// Format a value with engineering notation
pub fn format_value(val: f64) -> String {
    let abs = val.abs();
    if abs >= 1e9 {
        format!("{:.3}G", val / 1e9)
    } else if abs >= 1e6 {
        format!("{:.3}M", val / 1e6)
    } else if abs >= 1e3 {
        format!("{:.3}k", val / 1e3)
    } else if abs >= 1.0 {
        format!("{:.3}", val)
    } else if abs >= 1e-3 {
        format!("{:.3}m", val * 1e3)
    } else if abs >= 1e-6 {
        format!("{:.3}µ", val * 1e6)
    } else if abs >= 1e-9 {
        format!("{:.3}n", val * 1e9)
    } else if abs >= 1e-12 {
        format!("{:.3}p", val * 1e12)
    } else {
        format!("{:.3e}", val)
    }
}

/// Format percentage
pub fn format_percent(pct: f64) -> String {
    format!("{:.2}%", pct)
}

/// Format sigma level
pub fn format_sigma(sigma: f64) -> String {
    format!("{:.2}σ", sigma)
}

// =============================================================================
// Histogram Viewer UI Component
// =============================================================================

/// Histogram viewer component properties
#[derive(Props, Clone, PartialEq)]
pub struct HistogramViewerComponentProps {
    /// Viewer state
    pub state: HistogramViewerState,

    /// Canvas width
    #[props(default = 800)]
    pub width: u32,

    /// Canvas height
    #[props(default = 500)]
    pub height: u32,

    /// Show statistics panel
    #[props(default = true)]
    pub show_stats: bool,
}

/// Histogram viewer component
#[component]
pub fn HistogramViewerComponent(props: HistogramViewerComponentProps) -> Element {
    let margins = HistoMargins {
        top: 40.0,
        bottom: 60.0,
        left: 70.0,
        right: if props.show_stats { 250.0 } else { 40.0 },
    };

    let plot_width = (props.width as f64) - margins.left - margins.right;
    let plot_height = (props.height as f64) - margins.top - margins.bottom;

    let active_hist = props.state.active_histogram();
    let config = &props.state.config;

    rsx! {
        div {
            class: "histogram-viewer-container",
            style: "display: flex; background: {config.background_color}; border-radius: 8px; padding: 10px;",

            svg {
                width: "{props.width}",
                height: "{props.height}",
                view_box: "0 0 {props.width} {props.height}",

                // Background
                rect {
                    x: "0",
                    y: "0",
                    width: "{props.width}",
                    height: "{props.height}",
                    fill: "{config.background_color}",
                }

                // Plot area
                rect {
                    x: "{margins.left}",
                    y: "{margins.top}",
                    width: "{plot_width}",
                    height: "{plot_height}",
                    fill: "#0d0d1a",
                    stroke: "#333",
                }

                // Title
                text {
                    x: "{(props.width as f64) / 2.0}",
                    y: "20",
                    fill: "#fff",
                    "font-size": "14",
                    "text-anchor": "middle",
                    "font-weight": "bold",
                    if let Some(hist) = active_hist {
                        "Histogram: {hist.name}"
                    } else {
                        "Histogram Viewer"
                    }
                }

                // Render histogram if data available
                if let Some(hist) = active_hist {
                    { render_histogram_bars(hist, config, margins.left, margins.top, plot_width, plot_height) }

                    // Gaussian fit overlay
                    if config.show_gaussian_fit {
                        { render_gaussian_fit(hist, margins.left, margins.top, plot_width, plot_height, &config.fit_color) }
                    }

                    // Mean line
                    if config.show_mean {
                        { render_mean_line(hist, margins.left, margins.top, plot_width, plot_height) }
                    }

                    // Specification limits
                    if config.show_spec_limits {
                        { render_spec_limits(config, hist, margins.left, margins.top, plot_width, plot_height) }
                    }

                    // Axis labels
                    { render_hist_axis_labels(hist, config, margins.left, margins.top, plot_width, plot_height) }
                }
            }

            // Statistics panel
            if props.show_stats {
                { render_stats_panel(&props.state) }
            }
        }
    }
}

/// Histogram margins
struct HistoMargins {
    top: f64,
    bottom: f64,
    left: f64,
    right: f64,
}

/// Render histogram bars
fn render_histogram_bars(
    hist: &HistogramData,
    config: &HistogramConfig,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
) -> Element {
    let display_values = hist.get_display_values(config.mode);
    if display_values.is_empty() || hist.bins.is_empty() {
        return rsx! {};
    }

    let max_val = display_values.iter().cloned().fold(0.0_f64, f64::max);
    if max_val < 1e-15 {
        return rsx! {};
    }

    let bin_width = w / hist.bins.len() as f64;

    rsx! {
        g { class: "histogram-bars",
            for (i, val) in display_values.iter().enumerate() {
                { render_single_bar(i, *val, max_val, &config.bar_color, x, y, w, h, bin_width) }
            }
        }
    }
}

fn render_single_bar(
    i: usize,
    val: f64,
    max_val: f64,
    color: &str,
    x: f64,
    y: f64,
    _w: f64,
    h: f64,
    bin_width: f64,
) -> Element {
    let bar_height = (val / max_val) * h;
    let bar_x = x + i as f64 * bin_width;
    let bar_y = y + h - bar_height;

    rsx! {
        rect {
            x: "{bar_x + 1.0}",
            y: "{bar_y}",
            width: "{bin_width - 2.0}",
            height: "{bar_height}",
            fill: "{color}",
            opacity: "0.8",
        }
    }
}

/// Render Gaussian fit curve
fn render_gaussian_fit(
    hist: &HistogramData,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    color: &str,
) -> Element {
    let fit_values = hist.gaussian_fit_values();
    if fit_values.is_empty() {
        return rsx! {};
    }

    let max_fit = fit_values.iter().cloned().fold(0.0_f64, f64::max);
    if max_fit < 1e-15 {
        return rsx! {};
    }

    let points: Vec<(f64, f64)> = fit_values
        .iter()
        .enumerate()
        .map(|(i, val)| {
            let px = x + (i as f64 / (fit_values.len() - 1).max(1) as f64) * w;
            let py = y + h - (val / max_fit) * h;
            (px, py)
        })
        .collect();

    let path_data = histo_path_from_coords(&points);

    rsx! {
        path {
            d: "{path_data}",
            stroke: "{color}",
            "stroke-width": "2",
            fill: "none",
        }
    }
}

/// Render mean line
fn render_mean_line(hist: &HistogramData, x: f64, y: f64, w: f64, h: f64) -> Element {
    if hist.bins.is_empty() {
        return rsx! {};
    }

    let stats = &hist.stats;
    let x_min = hist.bins.first().map(|b| b.left).unwrap_or(0.0);
    let x_max = hist.bins.last().map(|b| b.right).unwrap_or(1.0);
    let x_range = x_max - x_min;

    if x_range < 1e-15 {
        return rsx! {};
    }

    let mean_x = x + ((stats.mean - x_min) / x_range) * w;

    rsx! {
        line {
            x1: "{mean_x}",
            y1: "{y}",
            x2: "{mean_x}",
            y2: "{y + h}",
            stroke: "#2196F3",
            "stroke-width": "2",
            "stroke-dasharray": "4,2",
        }
        text {
            x: "{mean_x + 5.0}",
            y: "{y + 15.0}",
            fill: "#2196F3",
            "font-size": "10",
            "μ"
        }
    }
}

/// Render specification limits
fn render_spec_limits(
    config: &HistogramConfig,
    hist: &HistogramData,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
) -> Element {
    if hist.bins.is_empty() {
        return rsx! {};
    }

    let x_min = hist.bins.first().map(|b| b.left).unwrap_or(0.0);
    let x_max = hist.bins.last().map(|b| b.right).unwrap_or(1.0);
    let x_range = x_max - x_min;

    if x_range < 1e-15 {
        return rsx! {};
    }

    rsx! {
        g { class: "spec-limits",
            // Lower spec
            if let Some(lower) = config.lower_spec {
                if lower >= x_min && lower <= x_max {
                    {
                        let lx = x + ((lower - x_min) / x_range) * w;
                        rsx! {
                            line {
                                x1: "{lx}",
                                y1: "{y}",
                                x2: "{lx}",
                                y2: "{y + h}",
                                stroke: "{config.spec_color}",
                                "stroke-width": "2",
                            }
                            text {
                                x: "{lx - 5.0}",
                                y: "{y + h + 15.0}",
                                fill: "{config.spec_color}",
                                "font-size": "10",
                                "text-anchor": "end",
                                "LSL"
                            }
                        }
                    }
                }
            }
            // Upper spec
            if let Some(upper) = config.upper_spec {
                if upper >= x_min && upper <= x_max {
                    {
                        let ux = x + ((upper - x_min) / x_range) * w;
                        rsx! {
                            line {
                                x1: "{ux}",
                                y1: "{y}",
                                x2: "{ux}",
                                y2: "{y + h}",
                                stroke: "{config.spec_color}",
                                "stroke-width": "2",
                            }
                            text {
                                x: "{ux + 5.0}",
                                y: "{y + h + 15.0}",
                                fill: "{config.spec_color}",
                                "font-size": "10",
                                "text-anchor": "start",
                                "USL"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Render axis labels
fn render_hist_axis_labels(
    hist: &HistogramData,
    config: &HistogramConfig,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
) -> Element {
    let x_min = hist.bins.first().map(|b| b.left).unwrap_or(0.0);
    let x_max = hist.bins.last().map(|b| b.right).unwrap_or(1.0);

    let unit_str = hist.unit.as_deref().unwrap_or("");

    rsx! {
        g { class: "axis-labels",
            // X-axis label
            text {
                x: "{x + w / 2.0}",
                y: "{y + h + 45.0}",
                fill: "#888",
                "font-size": "12",
                "text-anchor": "middle",
                if !unit_str.is_empty() {
                    "Value ({unit_str})"
                } else {
                    "Value"
                }
            }
            // Y-axis label
            text {
                x: "15",
                y: "{y + h / 2.0}",
                fill: "#888",
                "font-size": "12",
                "text-anchor": "middle",
                transform: "rotate(-90, 15, {y + h / 2.0})",
                "{config.mode.y_axis_label()}"
            }
            // X-axis tick labels
            text {
                x: "{x}",
                y: "{y + h + 20.0}",
                fill: "#888",
                "font-size": "10",
                "text-anchor": "middle",
                "{format_value(x_min)}"
            }
            text {
                x: "{x + w}",
                y: "{y + h + 20.0}",
                fill: "#888",
                "font-size": "10",
                "text-anchor": "middle",
                "{format_value(x_max)}"
            }
        }
    }
}

/// Render statistics panel
fn render_stats_panel(state: &HistogramViewerState) -> Element {
    let active_hist = state.active_histogram();

    rsx! {
        div {
            class: "stats-panel",
            style: "margin-left: 20px; color: #fff; font-family: monospace; font-size: 11px; min-width: 220px;",

            h3 {
                style: "margin: 0 0 10px 0; color: #4CAF50;",
                "Statistics"
            }

            if let Some(hist) = active_hist {
                // Basic stats
                div { style: "margin-bottom: 12px;",
                    div { style: "color: #888;", "Central Tendency" }
                    div { "Mean: {format_value(hist.stats.mean)}" }
                    div { "Median: {format_value(hist.stats.median)}" }
                    div { "Mode: {format_value(hist.stats.mode)}" }
                }

                // Spread
                div { style: "margin-bottom: 12px;",
                    div { style: "color: #888;", "Spread" }
                    div { "Std Dev: {format_value(hist.stats.std_dev)}" }
                    div { "Range: {format_value(hist.stats.range)}" }
                    div { "IQR: {format_value(hist.stats.iqr)}" }
                }

                // Extremes
                div { style: "margin-bottom: 12px;",
                    div { style: "color: #888;", "Extremes" }
                    div { "Min: {format_value(hist.stats.min)}" }
                    div { "Max: {format_value(hist.stats.max)}" }
                }

                // Percentiles
                div { style: "margin-bottom: 12px;",
                    div { style: "color: #888;", "Percentiles" }
                    div { "P5: {format_value(hist.stats.p05)}" }
                    div { "P25: {format_value(hist.stats.p25)}" }
                    div { "P75: {format_value(hist.stats.p75)}" }
                    div { "P95: {format_value(hist.stats.p95)}" }
                }

                // Shape
                div { style: "margin-bottom: 12px;",
                    div { style: "color: #888;", "Shape" }
                    div { "Skewness: {hist.stats.skewness:.3}" }
                    div { "Kurtosis: {hist.stats.kurtosis:.3}" }
                    div { "CV: {format_percent(hist.stats.cv * 100.0)}" }
                }

                // Sample info
                div { style: "margin-bottom: 12px;",
                    div { style: "color: #888;", "Sample" }
                    div { "N: {hist.stats.count}" }
                    div { "Bins: {hist.num_bins}" }
                }
            }

            // Yield analysis
            if let Some(yield_analysis) = &state.yield_analysis {
                div { style: "margin-top: 10px; padding: 10px; background: #222; border-radius: 4px;",
                    div { style: "color: #4CAF50; font-weight: bold; margin-bottom: 5px;", "Yield Analysis" }
                    div { "Total: {yield_analysis.total}" }
                    div { style: if yield_analysis.yield_percent >= 99.0 { "color: #4CAF50;" } else if yield_analysis.yield_percent >= 95.0 { "color: #ff9800;" } else { "color: #f44336;" },
                        "Yield: {format_percent(yield_analysis.yield_percent)}"
                    }
                    div { "Pass: {yield_analysis.pass}" }
                    div { "Fail: {yield_analysis.fail}" }
                    div { "Defect PPM: {yield_analysis.defect_ppm():.1}" }
                    div { "Sigma: {format_sigma(yield_analysis.sigma_level())}" }
                }
            }
        }
    }
}

/// Convert coordinate pairs to SVG path
fn histo_path_from_coords(points: &[(f64, f64)]) -> String {
    if points.is_empty() {
        return String::new();
    }

    let mut path = String::with_capacity(points.len() * 20);
    for (i, (x, y)) in points.iter().enumerate() {
        if i == 0 {
            path.push_str(&format!("M {:.1} {:.1}", x, y));
        } else {
            path.push_str(&format!(" L {:.1} {:.1}", x, y));
        }
    }
    path
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Distribution Type Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_distribution_type_all() {
        assert_eq!(DistributionType::all().len(), 4);
    }

    #[test]
    fn test_distribution_type_display() {
        assert!(DistributionType::Normal.display_name().contains("Normal"));
    }

    // -------------------------------------------------------------------------
    // Histogram Mode Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_histogram_mode_all() {
        assert_eq!(HistogramMode::all().len(), 4);
    }

    #[test]
    fn test_histogram_mode_y_label() {
        assert!(HistogramMode::PDF.y_axis_label().contains("Density"));
        assert!(HistogramMode::CDF.y_axis_label().contains("Cumulative"));
    }

    // -------------------------------------------------------------------------
    // Statistics Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_statistics_empty() {
        let stats = Statistics::from_data(&[]);
        assert_eq!(stats.count, 0);
    }

    #[test]
    fn test_statistics_single_value() {
        let stats = Statistics::from_data(&[5.0]);
        assert_eq!(stats.count, 1);
        assert_eq!(stats.mean, 5.0);
        assert_eq!(stats.min, 5.0);
        assert_eq!(stats.max, 5.0);
    }

    #[test]
    fn test_statistics_basic() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let stats = Statistics::from_data(&data);

        assert_eq!(stats.count, 5);
        assert!((stats.mean - 3.0).abs() < 0.01);
        assert!((stats.median - 3.0).abs() < 0.01);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
        assert_eq!(stats.range, 4.0);
    }

    #[test]
    fn test_statistics_std_dev() {
        // Known variance data
        let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let stats = Statistics::from_data(&data);

        // Mean = 5, sample variance ≈ 4, std ≈ 2
        assert!((stats.mean - 5.0).abs() < 0.01);
        assert!((stats.std_dev - 2.0).abs() < 0.2);
    }

    #[test]
    fn test_statistics_percentiles() {
        let data: Vec<f64> = (0..100).map(|i| i as f64).collect();
        let stats = Statistics::from_data(&data);

        // Approximate percentiles
        assert!((stats.p25 - 24.75).abs() < 1.0);
        assert!((stats.median - 49.5).abs() < 1.0);
        assert!((stats.p75 - 74.25).abs() < 1.0);
    }

    #[test]
    fn test_statistics_iqr() {
        let data: Vec<f64> = (0..100).map(|i| i as f64).collect();
        let stats = Statistics::from_data(&data);
        assert!((stats.iqr - 49.5).abs() < 1.0);
    }

    #[test]
    fn test_statistics_is_normal() {
        // Normally distributed data should pass
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let stats = Statistics::from_data(&data);
        assert!(stats.is_approximately_normal());
    }

    #[test]
    fn test_statistics_cv() {
        let data = vec![10.0, 10.0, 10.0, 10.0]; // Zero CV
        let stats = Statistics::from_data(&data);
        assert!(stats.cv.abs() < 0.01);
    }

    #[test]
    fn test_statistics_cpk() {
        let data: Vec<f64> = (0..100).map(|i| 100.0 + (i as f64 - 50.0) / 10.0).collect();
        let stats = Statistics::from_data(&data);
        let cpk = stats.cpk(90.0, 110.0);
        assert!(cpk > 0.0);
    }

    #[test]
    fn test_statistics_yield() {
        let data = vec![90.0, 95.0, 100.0, 105.0, 110.0, 115.0];
        let stats = Statistics::from_data(&data);
        let y = stats.yield_percent(92.0, 108.0, &data);
        // 95, 100, 105 pass (3 out of 6 = 50%)
        assert!((y - 50.0).abs() < 0.1);
    }

    // -------------------------------------------------------------------------
    // Percentile Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_percentile_edge_cases() {
        let sorted = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(percentile(&sorted, 0.0), 1.0);
        assert_eq!(percentile(&sorted, 1.0), 5.0);
    }

    #[test]
    fn test_percentile_empty() {
        assert_eq!(percentile(&[], 0.5), 0.0);
    }

    #[test]
    fn test_percentile_interpolation() {
        let sorted = vec![0.0, 10.0];
        let p = percentile(&sorted, 0.5);
        assert!((p - 5.0).abs() < 0.01);
    }

    // -------------------------------------------------------------------------
    // Histogram Data Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_histogram_data_from_data() {
        let data = vec![1.0, 2.0, 2.0, 3.0, 4.0];
        let hist = HistogramData::from_data("Test", data, 5);

        assert_eq!(hist.name, "Test");
        assert_eq!(hist.num_bins, 5);
        assert_eq!(hist.values.len(), 5);
        assert!(!hist.bins.is_empty());
    }

    #[test]
    fn test_histogram_data_with_unit() {
        let hist = HistogramData::from_data("Voltage", vec![1.0, 2.0], 5).with_unit("V");
        assert_eq!(hist.unit, Some("V".to_string()));
    }

    #[test]
    fn test_histogram_rebin() {
        let mut hist = HistogramData::from_data("Test", vec![1.0, 2.0, 3.0, 4.0, 5.0], 5);
        hist.rebin(10);
        assert_eq!(hist.num_bins, 10);
        assert_eq!(hist.bins.len(), 10);
    }

    #[test]
    fn test_histogram_display_values_count() {
        let hist = HistogramData::from_data("Test", vec![1.0, 1.5, 2.0, 3.0], 3);
        let counts = hist.get_display_values(HistogramMode::Count);
        assert_eq!(counts.iter().map(|c| *c as usize).sum::<usize>(), 4);
    }

    #[test]
    fn test_histogram_display_values_cdf() {
        let hist = HistogramData::from_data("Test", vec![1.0, 2.0, 3.0, 4.0], 4);
        let cdf = hist.get_display_values(HistogramMode::CDF);
        // Last CDF value should be 1.0
        assert!((cdf.last().unwrap() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_histogram_gaussian_fit() {
        let hist = HistogramData::from_data("Test", vec![1.0, 2.0, 3.0, 4.0, 5.0], 5);
        let fit = hist.gaussian_fit_values();
        assert_eq!(fit.len(), 5);
        // All values should be positive
        assert!(fit.iter().all(|&v| v >= 0.0));
    }

    #[test]
    fn test_histogram_yield_analysis() {
        let hist = HistogramData::from_data("Test", vec![1.0, 2.0, 3.0, 4.0, 5.0], 5);
        let yield_data = hist.yield_analysis(Some(2.0), Some(4.0));

        assert_eq!(yield_data.total, 5);
        assert_eq!(yield_data.pass, 3); // 2, 3, 4
        assert_eq!(yield_data.fail, 2); // 1, 5
    }

    // -------------------------------------------------------------------------
    // Yield Analysis Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_yield_analysis_defect_ppm() {
        let analysis = YieldAnalysis {
            total: 1_000_000,
            pass: 999_990,
            fail: 10,
            low_fail: 5,
            high_fail: 5,
            yield_percent: 99.999,
            lower_spec: Some(0.0),
            upper_spec: Some(1.0),
        };

        assert!((analysis.defect_ppm() - 10.0).abs() < 0.1);
    }

    #[test]
    fn test_yield_analysis_sigma_level() {
        // Very low defect rate should give high sigma
        let analysis = YieldAnalysis {
            total: 1_000_000,
            pass: 999_997,
            fail: 3,
            low_fail: 1,
            high_fail: 2,
            yield_percent: 99.9997,
            lower_spec: None,
            upper_spec: None,
        };

        let sigma = analysis.sigma_level();
        assert!(sigma > 4.0); // Should be around 4.5σ for 3 ppm
    }

    // -------------------------------------------------------------------------
    // Config Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_histogram_config_default() {
        let config = HistogramConfig::default();
        assert!(config.num_bins > 0);
        assert!(config.show_stats);
    }

    // -------------------------------------------------------------------------
    // State Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_histogram_viewer_state_new() {
        let state = HistogramViewerState::new();
        assert!(state.histograms.is_empty());
        assert!(state.active_index.is_none());
    }

    #[test]
    fn test_histogram_viewer_add_histogram() {
        let mut state = HistogramViewerState::new();
        let hist = HistogramData::from_data("Test", vec![1.0, 2.0, 3.0], 5);
        state.add_histogram(hist);

        assert_eq!(state.histograms.len(), 1);
        assert_eq!(state.active_index, Some(0));
    }

    #[test]
    fn test_histogram_viewer_set_bin_count() {
        let mut state = HistogramViewerState::new();
        let hist = HistogramData::from_data("Test", vec![1.0, 2.0, 3.0, 4.0], 5);
        state.add_histogram(hist);
        state.set_bin_count(10);

        assert_eq!(state.config.num_bins, 10);
        assert_eq!(state.active_histogram().unwrap().num_bins, 10);
    }

    #[test]
    fn test_histogram_viewer_clear() {
        let mut state = HistogramViewerState::new();
        state.add_histogram(HistogramData::from_data("Test", vec![1.0], 5));
        state.clear();

        assert!(state.histograms.is_empty());
        assert!(state.active_index.is_none());
    }

    // -------------------------------------------------------------------------
    // Formatting Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_format_value_large() {
        assert!(format_value(1e9).contains("G"));
        assert!(format_value(1e6).contains("M"));
        assert!(format_value(1e3).contains("k"));
    }

    #[test]
    fn test_format_value_small() {
        assert!(format_value(1e-3).contains("m"));
        assert!(format_value(1e-6).contains("µ"));
        assert!(format_value(1e-9).contains("n"));
        assert!(format_value(1e-12).contains("p"));
    }

    #[test]
    fn test_format_percent() {
        assert!(format_percent(99.5).contains("99.50%"));
    }

    #[test]
    fn test_format_sigma() {
        assert!(format_sigma(3.5).contains("3.50σ"));
    }

    // -------------------------------------------------------------------------
    // Gaussian Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_gaussian_pdf_center() {
        // PDF should be highest at mean
        let at_mean = gaussian_pdf(0.0, 0.0, 1.0);
        let off_mean = gaussian_pdf(1.0, 0.0, 1.0);
        assert!(at_mean > off_mean);
    }

    #[test]
    fn test_gaussian_pdf_zero_sigma() {
        let p = gaussian_pdf(0.0, 0.0, 0.0);
        assert_eq!(p, 0.0);
    }

    // -------------------------------------------------------------------------
    // Inverse Normal Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_inverse_normal_bounds() {
        assert!(inverse_normal_cdf(0.0).is_infinite());
        assert!(inverse_normal_cdf(1.0).is_infinite());
    }

    #[test]
    fn test_inverse_normal_median() {
        let z = inverse_normal_cdf(0.5);
        assert!(z.abs() < 0.01);
    }

    #[test]
    fn test_inverse_normal_known_values() {
        // Φ^(-1)(0.84) ≈ 1
        let z = inverse_normal_cdf(0.84134);
        assert!((z - 1.0).abs() < 0.01);
    }

    // -------------------------------------------------------------------------
    // Serialization Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_statistics_serialize() {
        let stats = Statistics::from_data(&[1.0, 2.0, 3.0]);
        let json = serde_json::to_string(&stats).unwrap();
        assert!(json.contains("mean"));
    }

    #[test]
    fn test_histogram_data_roundtrip() {
        let hist = HistogramData::from_data("Test", vec![1.0, 2.0, 3.0], 5);
        let json = serde_json::to_string(&hist).unwrap();
        let parsed: HistogramData = serde_json::from_str(&json).unwrap();
        assert_eq!(hist.name, parsed.name);
    }

    #[test]
    fn test_histogram_config_roundtrip() {
        let config = HistogramConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let parsed: HistogramConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config, parsed);
    }

    #[test]
    fn test_histogram_state_roundtrip() {
        let mut state = HistogramViewerState::new();
        state.add_histogram(HistogramData::from_data("Test", vec![1.0, 2.0], 5));
        let json = serde_json::to_string(&state).unwrap();
        let parsed: HistogramViewerState = serde_json::from_str(&json).unwrap();
        assert_eq!(state.histograms.len(), parsed.histograms.len());
    }
}
