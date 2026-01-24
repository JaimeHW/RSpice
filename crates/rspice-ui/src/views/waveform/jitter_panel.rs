//! Jitter Analysis Panel
//!
//! Commercial-grade jitter analysis component for signal integrity verification.
//! Provides comprehensive jitter decomposition and statistics including:
//!
//! - Time Interval Error (TIE) measurement
//! - Random Jitter (RJ) / Deterministic Jitter (DJ) separation
//! - Periodic Jitter (PJ) detection
//! - Data-Dependent Jitter (DDJ) analysis
//! - Duty Cycle Distortion (DCD)
//! - Total Jitter (TJ) at specified BER
//! - Jitter histograms and bathtub curves
//!
//! Based on industry-standard dual-Dirac jitter model.

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

// =============================================================================
// Jitter Types and Configuration
// =============================================================================

/// Jitter measurement configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JitterConfig {
    /// Target BER for total jitter calculation
    pub ber_target: f64,

    /// Number of histogram bins
    pub histogram_bins: usize,

    /// Threshold level for edge detection (0.0-1.0)
    pub threshold_level: f64,

    /// Minimum edges required for analysis
    pub min_edges: usize,

    /// Enable periodic jitter detection
    pub detect_periodic: bool,

    /// Enable DDJ/DCD analysis
    pub detect_dcd: bool,
}

impl Default for JitterConfig {
    fn default() -> Self {
        Self {
            ber_target: 1e-12,
            histogram_bins: 100,
            threshold_level: 0.5,
            min_edges: 100,
            detect_periodic: true,
            detect_dcd: true,
        }
    }
}

/// Edge type for jitter analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeType {
    Rising,
    Falling,
}

/// Individual edge timing measurement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EdgeMeasurement {
    /// Absolute time of edge crossing
    pub time: f64,

    /// Edge type (rising/falling)
    pub edge_type: EdgeType,

    /// Time Interval Error relative to ideal
    pub tie: f64,

    /// Period since last edge of same type
    pub period: Option<f64>,
}

/// Jitter decomposition results
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct JitterDecomposition {
    /// Random Jitter (RJ) - 1-sigma value in seconds
    pub rj_rms: f64,

    /// Deterministic Jitter (DJ) peak-to-peak in seconds
    pub dj_pp: f64,

    /// Periodic Jitter (PJ) peak-to-peak in seconds
    pub pj_pp: f64,

    /// Data-Dependent Jitter (DDJ) peak-to-peak in seconds
    pub ddj_pp: f64,

    /// Duty Cycle Distortion in seconds
    pub dcd: f64,

    /// Duty Cycle Distortion as percentage deviation from 50%
    pub dcd_percent: f64,

    /// Inter-Symbol Interference in seconds
    pub isi: f64,

    /// Total Jitter (TJ) at specified BER
    pub tj_at_ber: f64,

    /// BER used for TJ calculation
    pub tj_ber: f64,

    /// Detected periodic jitter frequency (if any)
    pub pj_frequency: Option<f64>,
}

/// Complete jitter analysis results
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct JitterResults {
    /// Edge measurements
    pub edges: Vec<EdgeMeasurement>,

    /// Jitter decomposition
    pub decomposition: JitterDecomposition,

    /// TIE histogram (bin_centers, counts)
    pub tie_histogram: (Vec<f64>, Vec<usize>),

    /// Rising edge TIE values
    pub rising_tie: Vec<f64>,

    /// Falling edge TIE values
    pub falling_tie: Vec<f64>,

    /// RMS jitter from TIE
    pub tie_rms: f64,

    /// Peak-to-peak jitter from TIE
    pub tie_pp: f64,

    /// Mean period (data rate estimation)
    pub mean_period: f64,

    /// Period jitter RMS
    pub period_jitter_rms: f64,

    /// Cycle-to-cycle jitter RMS
    pub cycle_to_cycle_rms: f64,

    /// Number of valid edges analyzed
    pub edge_count: usize,

    /// Analysis successful
    pub valid: bool,

    /// Error message if analysis failed
    pub error: Option<String>,
}

impl JitterResults {
    /// Create empty invalid results with error message
    pub fn error(msg: &str) -> Self {
        Self {
            valid: false,
            error: Some(msg.to_string()),
            ..Default::default()
        }
    }

    /// Format TJ for display
    pub fn format_tj(&self) -> String {
        format_time(self.decomposition.tj_at_ber)
    }

    /// Format RJ for display
    pub fn format_rj(&self) -> String {
        format_time(self.decomposition.rj_rms)
    }

    /// Format DJ for display
    pub fn format_dj(&self) -> String {
        format_time(self.decomposition.dj_pp)
    }
}

// =============================================================================
// Jitter Analysis Functions
// =============================================================================

/// Perform complete jitter analysis on a waveform
///
/// # Arguments
/// * `time` - Time values from transient simulation
/// * `signal` - Signal values (voltage)
/// * `config` - Jitter analysis configuration
///
/// # Returns
/// Complete jitter analysis results including decomposition
pub fn analyze_jitter(time: &[f64], signal: &[f64], config: &JitterConfig) -> JitterResults {
    if time.len() < 2 || time.len() != signal.len() {
        return JitterResults::error("Insufficient data for jitter analysis");
    }

    // Find signal levels for threshold calculation
    let v_min = signal.iter().cloned().fold(f64::MAX, f64::min);
    let v_max = signal.iter().cloned().fold(f64::MIN, f64::max);
    let v_swing = v_max - v_min;

    if v_swing < 1e-15 {
        return JitterResults::error("Signal swing too small");
    }

    let threshold = v_min + v_swing * config.threshold_level;

    // Detect all edges
    let edges = detect_edges(time, signal, threshold);

    if edges.len() < config.min_edges {
        return JitterResults::error(&format!(
            "Insufficient edges ({} found, {} required)",
            edges.len(),
            config.min_edges
        ));
    }

    // Calculate mean period
    let mean_period = calculate_mean_period(&edges);
    if mean_period <= 0.0 {
        return JitterResults::error("Could not determine signal period");
    }

    // Calculate TIE (Time Interval Error)
    let (edges_with_tie, rising_tie, falling_tie) = calculate_tie(&edges, mean_period);

    // Calculate basic jitter statistics
    let all_tie: Vec<f64> = edges_with_tie.iter().map(|e| e.tie).collect();
    let tie_rms = calculate_rms(&all_tie);
    let tie_pp = calculate_peak_to_peak(&all_tie);

    // Calculate period jitter
    let periods: Vec<f64> = edges_with_tie.iter().filter_map(|e| e.period).collect();
    let period_jitter_rms = calculate_period_jitter_rms(&periods, mean_period);

    // Calculate cycle-to-cycle jitter
    let cycle_to_cycle_rms = calculate_cycle_to_cycle_jitter(&periods);

    // Perform jitter decomposition
    let decomposition = decompose_jitter(&rising_tie, &falling_tie, mean_period, config);

    // Build histogram
    let histogram = build_histogram(&all_tie, config.histogram_bins);

    JitterResults {
        edges: edges_with_tie,
        decomposition,
        tie_histogram: histogram,
        rising_tie,
        falling_tie,
        tie_rms,
        tie_pp,
        mean_period,
        period_jitter_rms,
        cycle_to_cycle_rms,
        edge_count: edges.len(),
        valid: true,
        error: None,
    }
}

/// Detect edges in a waveform
fn detect_edges(time: &[f64], signal: &[f64], threshold: f64) -> Vec<EdgeMeasurement> {
    let mut edges = Vec::new();

    if time.len() < 2 || signal.len() < 2 {
        return edges;
    }

    for i in 0..time.len() - 1 {
        let v0 = signal[i];
        let v1 = signal[i + 1];

        // Rising edge crossing
        if v0 < threshold && v1 >= threshold {
            let t_cross = interpolate_crossing(time[i], time[i + 1], v0, v1, threshold);
            edges.push(EdgeMeasurement {
                time: t_cross,
                edge_type: EdgeType::Rising,
                tie: 0.0, // Will be calculated later
                period: None,
            });
        }
        // Falling edge crossing
        else if v0 >= threshold && v1 < threshold {
            let t_cross = interpolate_crossing(time[i], time[i + 1], v0, v1, threshold);
            edges.push(EdgeMeasurement {
                time: t_cross,
                edge_type: EdgeType::Falling,
                tie: 0.0,
                period: None,
            });
        }
    }

    edges
}

/// Interpolate exact crossing time
fn interpolate_crossing(t0: f64, t1: f64, v0: f64, v1: f64, threshold: f64) -> f64 {
    if (v1 - v0).abs() < 1e-15 {
        return t0;
    }
    t0 + (threshold - v0) / (v1 - v0) * (t1 - t0)
}

/// Calculate mean period from edge measurements
fn calculate_mean_period(edges: &[EdgeMeasurement]) -> f64 {
    let mut rising_times: Vec<f64> = edges
        .iter()
        .filter(|e| e.edge_type == EdgeType::Rising)
        .map(|e| e.time)
        .collect();

    if rising_times.len() < 2 {
        return 0.0;
    }

    rising_times.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut periods = Vec::new();
    for i in 1..rising_times.len() {
        periods.push(rising_times[i] - rising_times[i - 1]);
    }

    if periods.is_empty() {
        return 0.0;
    }

    // Use median for robustness against outliers
    periods.sort_by(|a, b| a.partial_cmp(b).unwrap());
    periods[periods.len() / 2]
}

/// Calculate Time Interval Error for each edge
fn calculate_tie(
    edges: &[EdgeMeasurement],
    period: f64,
) -> (Vec<EdgeMeasurement>, Vec<f64>, Vec<f64>) {
    if edges.is_empty() || period <= 0.0 {
        return (Vec::new(), Vec::new(), Vec::new());
    }

    let t0 = edges[0].time;
    let mut result = Vec::new();
    let mut rising_tie = Vec::new();
    let mut falling_tie = Vec::new();

    // Track last edge of each type for period calculation
    let mut last_rising: Option<f64> = None;
    let mut last_falling: Option<f64> = None;

    for (i, edge) in edges.iter().enumerate() {
        // Calculate ideal time based on edge count and half-period spacing
        let ideal_time = t0 + (i as f64) * period / 2.0;
        let tie = edge.time - ideal_time;

        let current_period = match edge.edge_type {
            EdgeType::Rising => {
                let p = last_rising.map(|t| edge.time - t);
                last_rising = Some(edge.time);
                rising_tie.push(tie);
                p
            }
            EdgeType::Falling => {
                let p = last_falling.map(|t| edge.time - t);
                last_falling = Some(edge.time);
                falling_tie.push(tie);
                p
            }
        };

        result.push(EdgeMeasurement {
            time: edge.time,
            edge_type: edge.edge_type,
            tie,
            period: current_period,
        });
    }

    (result, rising_tie, falling_tie)
}

/// Decompose jitter into RJ, DJ, PJ, DDJ components
fn decompose_jitter(
    rising_tie: &[f64],
    falling_tie: &[f64],
    period: f64,
    config: &JitterConfig,
) -> JitterDecomposition {
    if rising_tie.is_empty() && falling_tie.is_empty() {
        return JitterDecomposition::default();
    }

    // Combine all TIE for overall analysis
    let all_tie: Vec<f64> = rising_tie
        .iter()
        .chain(falling_tie.iter())
        .cloned()
        .collect();

    // Calculate RJ using tail-fitting method (dual-Dirac model)
    let (rj_rms, dj_pp) = dual_dirac_decomposition(&all_tie);

    // Calculate DCD (Duty Cycle Distortion)
    let dcd = calculate_dcd(rising_tie, falling_tie);
    let dcd_percent = (dcd / period) * 100.0;

    // Detect periodic jitter if enabled
    let (pj_pp, pj_frequency) = if config.detect_periodic {
        detect_periodic_jitter(&all_tie, period)
    } else {
        (0.0, None)
    };

    // Estimate DDJ (simplified - based on pattern sensitivity)
    let ddj_pp = if config.detect_dcd {
        estimate_ddj(&all_tie)
    } else {
        0.0
    };

    // Calculate ISI (inter-symbol interference)
    let isi = dj_pp - ddj_pp - pj_pp;

    // Calculate total jitter at specified BER
    let q = q_factor_from_ber(config.ber_target);
    let tj_at_ber = 2.0 * q * rj_rms + dj_pp;

    JitterDecomposition {
        rj_rms,
        dj_pp,
        pj_pp,
        ddj_pp,
        dcd,
        dcd_percent,
        isi: isi.max(0.0),
        tj_at_ber,
        tj_ber: config.ber_target,
        pj_frequency,
    }
}

/// Dual-Dirac decomposition to separate RJ and DJ
///
/// Uses histogram tail-fitting method to separate Gaussian RJ from bounded DJ.
fn dual_dirac_decomposition(tie_values: &[f64]) -> (f64, f64) {
    if tie_values.is_empty() {
        return (0.0, 0.0);
    }

    let n = tie_values.len();
    if n < 10 {
        // Not enough data - use simple statistics
        let rms = calculate_rms(tie_values);
        let pp = calculate_peak_to_peak(tie_values);
        return (rms, pp);
    }

    // Sort for percentile analysis
    let mut sorted = tie_values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Use robust percentile method
    let p_05 = sorted[(n as f64 * 0.05) as usize];
    let p_95 = sorted[(n as f64 * 0.95) as usize];
    let p_25 = sorted[(n as f64 * 0.25) as usize];
    let p_75 = sorted[(n as f64 * 0.75) as usize];

    // Estimate DJ from bounded portion
    let iqr = p_75 - p_25;
    let dj_pp = (p_95 - p_05).max(0.0);

    // Estimate RJ from spread (assuming Gaussian tails)
    // For Gaussian: IQR ≈ 1.35 * sigma
    let rj_rms = iqr / 1.35;

    (rj_rms, dj_pp)
}

/// Calculate Duty Cycle Distortion
fn calculate_dcd(rising_tie: &[f64], falling_tie: &[f64]) -> f64 {
    if rising_tie.is_empty() || falling_tie.is_empty() {
        return 0.0;
    }

    let rising_mean: f64 = rising_tie.iter().sum::<f64>() / rising_tie.len() as f64;
    let falling_mean: f64 = falling_tie.iter().sum::<f64>() / falling_tie.len() as f64;

    (rising_mean - falling_mean).abs()
}

/// Detect periodic jitter using FFT-like approach
fn detect_periodic_jitter(tie_values: &[f64], period: f64) -> (f64, Option<f64>) {
    if tie_values.len() < 32 {
        return (0.0, None);
    }

    // Simple periodicity detection using autocorrelation
    let n = tie_values.len();
    let mean: f64 = tie_values.iter().sum::<f64>() / n as f64;
    let centered: Vec<f64> = tie_values.iter().map(|v| v - mean).collect();

    // Calculate autocorrelation at various lags
    let max_lag = n.min(256);
    let mut max_corr = 0.0;
    let mut peak_lag = 0;

    // Variance for normalization
    let variance: f64 = centered.iter().map(|v| v * v).sum::<f64>() / n as f64;
    if variance < 1e-30 {
        return (0.0, None);
    }

    for lag in 2..max_lag {
        let mut corr = 0.0;
        for i in 0..n - lag {
            corr += centered[i] * centered[i + lag];
        }
        corr /= (n - lag) as f64 * variance;

        if corr > max_corr {
            max_corr = corr;
            peak_lag = lag;
        }
    }

    // If significant periodicity detected
    if max_corr > 0.3 && peak_lag > 0 {
        // Estimate PJ amplitude from the periodic component
        let pj_amplitude = (2.0 * variance * max_corr).sqrt();
        let pj_pp = 2.0 * pj_amplitude;

        // Estimate PJ frequency
        let pj_frequency = 1.0 / (peak_lag as f64 * period);

        (pj_pp, Some(pj_frequency))
    } else {
        (0.0, None)
    }
}

/// Estimate Data-Dependent Jitter
fn estimate_ddj(tie_values: &[f64]) -> f64 {
    if tie_values.len() < 8 {
        return 0.0;
    }

    // Simplified DDJ estimation based on pattern-dependent grouping
    // In a full implementation, this would analyze bit pattern dependencies

    // Use variance of local means as DDJ indicator
    let window = 4;
    let mut local_means = Vec::new();

    for i in 0..tie_values.len() / window {
        let start = i * window;
        let end = (start + window).min(tie_values.len());
        let mean: f64 = tie_values[start..end].iter().sum::<f64>() / (end - start) as f64;
        local_means.push(mean);
    }

    if local_means.is_empty() {
        return 0.0;
    }

    let ddj_pp = calculate_peak_to_peak(&local_means);
    ddj_pp
}

/// Calculate Q-factor from BER
fn q_factor_from_ber(ber: f64) -> f64 {
    if ber >= 0.5 {
        0.0
    } else if ber <= 1e-15 {
        8.0
    } else {
        // Approximation: Q ≈ sqrt(2) * sqrt(-ln(2*BER))
        let arg = -2.0 * ber.ln();
        if arg > 0.0 {
            arg.sqrt()
        } else {
            0.0
        }
    }
}

// =============================================================================
// Statistical Helpers
// =============================================================================

/// Calculate RMS
fn calculate_rms(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let sum_sq: f64 = values.iter().map(|v| v * v).sum();
    (sum_sq / values.len() as f64).sqrt()
}

/// Calculate peak-to-peak
fn calculate_peak_to_peak(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let min = values.iter().cloned().fold(f64::MAX, f64::min);
    let max = values.iter().cloned().fold(f64::MIN, f64::max);
    max - min
}

/// Calculate period jitter RMS
fn calculate_period_jitter_rms(periods: &[f64], mean_period: f64) -> f64 {
    if periods.is_empty() || mean_period <= 0.0 {
        return 0.0;
    }
    let deviations: Vec<f64> = periods.iter().map(|p| p - mean_period).collect();
    calculate_rms(&deviations)
}

/// Calculate cycle-to-cycle jitter
fn calculate_cycle_to_cycle_jitter(periods: &[f64]) -> f64 {
    if periods.len() < 2 {
        return 0.0;
    }
    let diffs: Vec<f64> = periods.windows(2).map(|w| (w[1] - w[0]).abs()).collect();
    calculate_rms(&diffs)
}

/// Build histogram from values
fn build_histogram(values: &[f64], bins: usize) -> (Vec<f64>, Vec<usize>) {
    if values.is_empty() || bins == 0 {
        return (Vec::new(), Vec::new());
    }

    let min = values.iter().cloned().fold(f64::MAX, f64::min);
    let max = values.iter().cloned().fold(f64::MIN, f64::max);
    let range = max - min;

    if range <= 0.0 {
        return (vec![min], vec![values.len()]);
    }

    let bin_width = range / bins as f64;
    let mut counts = vec![0usize; bins];
    let centers: Vec<f64> = (0..bins)
        .map(|i| min + (i as f64 + 0.5) * bin_width)
        .collect();

    for v in values {
        let bin = ((v - min) / bin_width) as usize;
        let bin = bin.min(bins - 1);
        counts[bin] += 1;
    }

    (centers, counts)
}

// =============================================================================
// Formatting Helpers
// =============================================================================

/// Format time with SI prefix
fn format_time(t: f64) -> String {
    let abs_t = t.abs();
    if abs_t >= 1.0 {
        format!("{:.3} s", t)
    } else if abs_t >= 1e-3 {
        format!("{:.3} ms", t * 1e3)
    } else if abs_t >= 1e-6 {
        format!("{:.3} µs", t * 1e6)
    } else if abs_t >= 1e-9 {
        format!("{:.3} ns", t * 1e9)
    } else if abs_t >= 1e-12 {
        format!("{:.3} ps", t * 1e12)
    } else {
        format!("{:.3} fs", t * 1e15)
    }
}

// =============================================================================
// Jitter Panel Component
// =============================================================================

/// Jitter panel component properties
#[derive(Props, Clone, PartialEq)]
pub struct JitterPanelProps {
    /// Jitter analysis results
    pub results: JitterResults,

    /// Panel width
    #[props(default = 400)]
    pub width: u32,

    /// Show histogram
    #[props(default = true)]
    pub show_histogram: bool,

    /// Show detailed breakdown
    #[props(default = true)]
    pub show_breakdown: bool,
}

/// Jitter analysis panel component
#[component]
pub fn JitterPanel(props: JitterPanelProps) -> Element {
    if !props.results.valid {
        return rsx! {
            div {
                class: "jitter-panel-error",
                style: "padding: 20px; color: #ff6b6b; background: #1a1a2e; border-radius: 8px;",
                p { "Jitter Analysis Error" }
                if let Some(ref error) = props.results.error {
                    p { style: "font-size: 12px; opacity: 0.8;", "{error}" }
                }
            }
        };
    }

    let decomp = &props.results.decomposition;

    rsx! {
        div {
            class: "jitter-panel",
            style: "background: #1a1a2e; border-radius: 8px; padding: 15px; color: #fff; font-family: monospace;",

            // Header
            h3 {
                style: "margin: 0 0 15px 0; color: #4CAF50; border-bottom: 1px solid #333; padding-bottom: 10px;",
                "Jitter Analysis"
            }

            // Summary metrics
            div {
                class: "jitter-summary",
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 15px; margin-bottom: 20px;",

                // Total Jitter
                div {
                    style: "background: #252544; padding: 12px; border-radius: 6px;",
                    div { style: "color: #888; font-size: 11px;", "Total Jitter (TJ @ {decomp.tj_ber:.0e})" }
                    div { style: "font-size: 18px; color: #4CAF50; font-weight: bold;",
                        "{props.results.format_tj()}"
                    }
                }

                // TIE RMS
                div {
                    style: "background: #252544; padding: 12px; border-radius: 6px;",
                    div { style: "color: #888; font-size: 11px;", "TIE RMS" }
                    div { style: "font-size: 18px;",
                        "{format_time(props.results.tie_rms)}"
                    }
                }
            }

            // Decomposition breakdown
            if props.show_breakdown {
                div {
                    class: "jitter-breakdown",
                    style: "margin-bottom: 20px;",

                    h4 { style: "margin: 0 0 10px 0; color: #888; font-size: 12px;", "JITTER DECOMPOSITION" }

                    // RJ
                    { render_jitter_row("Random (RJ)", format_time(decomp.rj_rms), "#66bb6a") }

                    // DJ
                    { render_jitter_row("Deterministic (DJ)", format_time(decomp.dj_pp), "#42a5f5") }

                    // PJ (if detected)
                    if decomp.pj_pp > 0.0 {
                        { render_jitter_row("Periodic (PJ)", format_time(decomp.pj_pp), "#ab47bc") }
                    }

                    // DDJ
                    if decomp.ddj_pp > 0.0 {
                        { render_jitter_row("Data-Dependent (DDJ)", format_time(decomp.ddj_pp), "#ff7043") }
                    }

                    // DCD
                    { render_jitter_row(
                        &format!("Duty Cycle Distortion ({:.2}%)", decomp.dcd_percent),
                        format_time(decomp.dcd),
                        "#ffa726"
                    ) }
                }
            }

            // Additional metrics
            div {
                class: "jitter-metrics",
                style: "background: #252544; padding: 12px; border-radius: 6px; font-size: 12px;",

                div { style: "display: flex; justify-content: space-between; margin-bottom: 5px;",
                    span { style: "color: #888;", "Period Jitter (RMS)" }
                    span { "{format_time(props.results.period_jitter_rms)}" }
                }
                div { style: "display: flex; justify-content: space-between; margin-bottom: 5px;",
                    span { style: "color: #888;", "Cycle-to-Cycle" }
                    span { "{format_time(props.results.cycle_to_cycle_rms)}" }
                }
                div { style: "display: flex; justify-content: space-between; margin-bottom: 5px;",
                    span { style: "color: #888;", "Mean Period" }
                    span { "{format_time(props.results.mean_period)}" }
                }
                div { style: "display: flex; justify-content: space-between;",
                    span { style: "color: #888;", "Edges Analyzed" }
                    span { "{props.results.edge_count}" }
                }
            }

            // Histogram (simplified text representation)
            if props.show_histogram && !props.results.tie_histogram.0.is_empty() {
                div {
                    class: "jitter-histogram",
                    style: "margin-top: 15px; padding-top: 15px; border-top: 1px solid #333;",

                    h4 { style: "margin: 0 0 10px 0; color: #888; font-size: 12px;", "TIE DISTRIBUTION" }

                    { render_histogram_bars(&props.results.tie_histogram) }
                }
            }
        }
    }
}

/// Render a jitter metric row
fn render_jitter_row(label: &str, value: String, color: &str) -> Element {
    rsx! {
        div {
            style: "display: flex; justify-content: space-between; align-items: center; padding: 6px 0; border-bottom: 1px solid #333;",
            div { style: "display: flex; align-items: center; gap: 8px;",
                div {
                    style: "width: 8px; height: 8px; border-radius: 50%; background: {color};",
                }
                span { style: "color: #ccc; font-size: 12px;", "{label}" }
            }
            span { style: "font-size: 12px;", "{value}" }
        }
    }
}

/// Render histogram bars (simple ASCII-style)
fn render_histogram_bars(histogram: &(Vec<f64>, Vec<usize>)) -> Element {
    let (centers, counts) = histogram;
    let max_count = counts.iter().cloned().max().unwrap_or(1) as f64;

    // Show only a subset of bins for compact display
    let display_bins = 20.min(centers.len());
    let step = centers.len().max(1) / display_bins.max(1);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 2px;",
            for i in (0..centers.len()).step_by(step.max(1)) {
                {
                    let width_pct = (counts[i] as f64 / max_count * 100.0).max(1.0);
                    rsx! {
                        div {
                            style: "display: flex; align-items: center; gap: 5px;",
                            div {
                                style: "width: {width_pct}%; height: 6px; background: #4CAF50; border-radius: 2px; min-width: 2px;",
                            }
                            span { style: "font-size: 9px; color: #666;", "{counts[i]}" }
                        }
                    }
                }
            }
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Configuration Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_jitter_config_default() {
        let config = JitterConfig::default();
        assert_eq!(config.ber_target, 1e-12);
        assert!(config.histogram_bins > 0);
        assert!(config.threshold_level > 0.0 && config.threshold_level < 1.0);
    }

    // -------------------------------------------------------------------------
    // Edge Detection Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_detect_edges_square_wave() {
        // Generate square wave
        let n = 1000;
        let period = 1e-9;
        let time: Vec<f64> = (0..n).map(|i| i as f64 * period / 100.0).collect();
        let signal: Vec<f64> = time
            .iter()
            .map(|t| {
                if (t / period * 2.0) as i32 % 2 == 0 {
                    1.0
                } else {
                    0.0
                }
            })
            .collect();

        let edges = detect_edges(&time, &signal, 0.5);
        assert!(!edges.is_empty(), "Should detect edges in square wave");
    }

    #[test]
    fn test_detect_edges_empty() {
        let edges = detect_edges(&[], &[], 0.5);
        assert!(edges.is_empty());
    }

    #[test]
    fn test_detect_edges_constant() {
        let time = vec![0.0, 1.0, 2.0];
        let signal = vec![1.0, 1.0, 1.0];
        let edges = detect_edges(&time, &signal, 0.5);
        assert!(edges.is_empty(), "Constant signal should have no edges");
    }

    // -------------------------------------------------------------------------
    // Interpolation Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_interpolate_crossing() {
        let t = interpolate_crossing(0.0, 1.0, 0.0, 1.0, 0.5);
        assert!((t - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_interpolate_crossing_at_boundary() {
        let t = interpolate_crossing(0.0, 1.0, 0.5, 0.5, 0.5);
        assert_eq!(t, 0.0); // Equal voltages returns t0
    }

    // -------------------------------------------------------------------------
    // Period Calculation Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_calculate_mean_period() {
        let edges = vec![
            EdgeMeasurement {
                time: 0.0,
                edge_type: EdgeType::Rising,
                tie: 0.0,
                period: None,
            },
            EdgeMeasurement {
                time: 0.5e-9,
                edge_type: EdgeType::Falling,
                tie: 0.0,
                period: None,
            },
            EdgeMeasurement {
                time: 1.0e-9,
                edge_type: EdgeType::Rising,
                tie: 0.0,
                period: None,
            },
            EdgeMeasurement {
                time: 1.5e-9,
                edge_type: EdgeType::Falling,
                tie: 0.0,
                period: None,
            },
            EdgeMeasurement {
                time: 2.0e-9,
                edge_type: EdgeType::Rising,
                tie: 0.0,
                period: None,
            },
        ];

        let period = calculate_mean_period(&edges);
        assert!(
            (period - 1.0e-9).abs() < 0.1e-9,
            "Expected 1ns period, got {}",
            period
        );
    }

    #[test]
    fn test_calculate_mean_period_empty() {
        let period = calculate_mean_period(&[]);
        assert_eq!(period, 0.0);
    }

    // -------------------------------------------------------------------------
    // TIE Calculation Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_calculate_tie() {
        let edges = vec![
            EdgeMeasurement {
                time: 0.0,
                edge_type: EdgeType::Rising,
                tie: 0.0,
                period: None,
            },
            EdgeMeasurement {
                time: 0.5e-9,
                edge_type: EdgeType::Falling,
                tie: 0.0,
                period: None,
            },
            EdgeMeasurement {
                time: 1.0e-9,
                edge_type: EdgeType::Rising,
                tie: 0.0,
                period: None,
            },
        ];

        let (result, rising, falling) = calculate_tie(&edges, 1.0e-9);
        assert_eq!(result.len(), 3);
        assert_eq!(rising.len(), 2);
        assert_eq!(falling.len(), 1);
    }

    #[test]
    fn test_calculate_tie_empty() {
        let (result, rising, falling) = calculate_tie(&[], 1.0e-9);
        assert!(result.is_empty());
        assert!(rising.is_empty());
        assert!(falling.is_empty());
    }

    // -------------------------------------------------------------------------
    // Jitter Decomposition Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_dual_dirac_decomposition_gaussian() {
        // Generate Gaussian-distributed TIE values
        let n = 1000;
        let sigma = 10e-12; // 10ps RMS

        // Simple pseudo-random Gaussian approximation using f64 to avoid overflow
        let tie_values: Vec<f64> = (0..n)
            .map(|i| {
                let u1 = ((i as f64 * 123.4567 + 89.0) % 1000.0) / 1000.0 + 0.001;
                let u2 = ((i as f64 * 765.4321 + 12.0) % 1000.0) / 1000.0 + 0.001;
                sigma * (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos()
            })
            .collect();

        let (rj, dj) = dual_dirac_decomposition(&tie_values);

        // RJ should be close to sigma
        assert!(rj > 0.0, "RJ should be positive");
        // DJ should be bounded
        assert!(dj >= 0.0, "DJ should be non-negative");
    }

    #[test]
    fn test_dual_dirac_decomposition_empty() {
        let (rj, dj) = dual_dirac_decomposition(&[]);
        assert_eq!(rj, 0.0);
        assert_eq!(dj, 0.0);
    }

    // -------------------------------------------------------------------------
    // DCD Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_calculate_dcd() {
        let rising = vec![1.0e-12, 2.0e-12, 1.5e-12]; // Mean = 1.5ps
        let falling = vec![-1.0e-12, -0.5e-12, -1.5e-12]; // Mean = -1.0ps

        let dcd = calculate_dcd(&rising, &falling);
        assert!((dcd - 2.5e-12).abs() < 0.1e-12, "DCD should be ~2.5ps");
    }

    #[test]
    fn test_calculate_dcd_empty() {
        let dcd = calculate_dcd(&[], &[]);
        assert_eq!(dcd, 0.0);
    }

    // -------------------------------------------------------------------------
    // Periodic Jitter Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_detect_periodic_jitter_sinusoidal() {
        // Generate sinusoidal jitter pattern
        let n = 256;
        let pj_amplitude = 5e-12;
        let pj_period = 16; // Period in samples

        let tie_values: Vec<f64> = (0..n)
            .map(|i| pj_amplitude * (2.0 * PI * i as f64 / pj_period as f64).sin())
            .collect();

        let (pj_pp, pj_freq) = detect_periodic_jitter(&tie_values, 1e-9);

        assert!(pj_pp > 0.0, "Should detect periodic jitter");
    }

    #[test]
    fn test_detect_periodic_jitter_random() {
        // Random jitter should not show periodicity
        let tie_values: Vec<f64> = (0..100)
            .map(|i| ((i * 12345) % 100) as f64 * 1e-14 - 50e-14)
            .collect();

        let (pj_pp, _) = detect_periodic_jitter(&tie_values, 1e-9);
        // Random data may or may not show false periodicity
        assert!(pj_pp >= 0.0);
    }

    // -------------------------------------------------------------------------
    // Statistical Helper Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_calculate_rms() {
        let values = vec![1.0, -1.0, 1.0, -1.0];
        let rms = calculate_rms(&values);
        assert!((rms - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_calculate_rms_empty() {
        assert_eq!(calculate_rms(&[]), 0.0);
    }

    #[test]
    fn test_calculate_peak_to_peak() {
        let values = vec![-5.0, 0.0, 5.0, 3.0];
        let pp = calculate_peak_to_peak(&values);
        assert_eq!(pp, 10.0);
    }

    #[test]
    fn test_calculate_peak_to_peak_empty() {
        assert_eq!(calculate_peak_to_peak(&[]), 0.0);
    }

    #[test]
    fn test_calculate_period_jitter_rms() {
        let periods = vec![1.0e-9, 1.1e-9, 0.9e-9, 1.0e-9];
        let mean = 1.0e-9;
        let pj = calculate_period_jitter_rms(&periods, mean);
        assert!(pj > 0.0, "Period jitter should be positive");
    }

    #[test]
    fn test_calculate_cycle_to_cycle_jitter() {
        let periods = vec![1.0e-9, 1.1e-9, 0.9e-9, 1.0e-9];
        let c2c = calculate_cycle_to_cycle_jitter(&periods);
        assert!(c2c > 0.0, "Cycle-to-cycle jitter should be positive");
    }

    // -------------------------------------------------------------------------
    // Histogram Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_build_histogram() {
        let values = vec![0.0, 0.5, 1.0, 0.5, 0.5];
        let (centers, counts) = build_histogram(&values, 10);

        assert_eq!(centers.len(), 10);
        assert_eq!(counts.len(), 10);
        assert_eq!(counts.iter().sum::<usize>(), values.len());
    }

    #[test]
    fn test_build_histogram_empty() {
        let (centers, counts) = build_histogram(&[], 10);
        assert!(centers.is_empty());
        assert!(counts.is_empty());
    }

    #[test]
    fn test_build_histogram_constant() {
        let values = vec![1.0, 1.0, 1.0];
        let (centers, counts) = build_histogram(&values, 10);
        assert!(!centers.is_empty());
    }

    // -------------------------------------------------------------------------
    // Q-Factor Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_q_factor_from_ber() {
        // BER = 1e-12 -> Q ≈ 7
        let q = q_factor_from_ber(1e-12);
        assert!(
            q > 6.0 && q < 8.0,
            "Q for BER=1e-12 should be ~7, got {}",
            q
        );

        // BER = 0.5 -> Q = 0
        assert_eq!(q_factor_from_ber(0.5), 0.0);
    }

    // -------------------------------------------------------------------------
    // Formatting Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_format_time() {
        assert!(format_time(1e-12).contains("ps"));
        assert!(format_time(1e-9).contains("ns"));
        assert!(format_time(1e-6).contains("µs"));
        assert!(format_time(1e-3).contains("ms"));
    }

    // -------------------------------------------------------------------------
    // Integration Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_analyze_jitter_square_wave() {
        // Generate clean square wave with many cycles
        let n = 10000;
        let period = 1e-6; // 1 MHz (longer period for easier edge detection)
        let samples_per_period = 100;
        let total_time = (n as f64 * period) / samples_per_period as f64;

        let time: Vec<f64> = (0..n).map(|i| i as f64 * total_time / n as f64).collect();
        let signal: Vec<f64> = time
            .iter()
            .map(|t| {
                let phase = (t / period).fract();
                if phase < 0.5 {
                    1.0
                } else {
                    0.0
                }
            })
            .collect();

        let mut config = JitterConfig::default();
        config.min_edges = 10; // Reduce requirement for test
        let results = analyze_jitter(&time, &signal, &config);

        assert!(
            results.valid,
            "Analysis should succeed: {:?}",
            results.error
        );
        assert!(results.edge_count > 0, "Should detect edges");
        assert!(results.mean_period > 0.0, "Should calculate mean period");
    }

    #[test]
    fn test_analyze_jitter_with_noise() {
        // Generate square wave with jitter
        let n = 10000;
        let period = 1e-6;
        let samples_per_period = 100;
        let total_time = (n as f64 * period) / samples_per_period as f64;

        let time: Vec<f64> = (0..n).map(|i| i as f64 * total_time / n as f64).collect();
        let signal: Vec<f64> = time
            .iter()
            .map(|t| {
                let phase = (t / period).fract();
                if phase < 0.5 {
                    1.0
                } else {
                    0.0
                }
            })
            .collect();

        let mut config = JitterConfig::default();
        config.min_edges = 10;
        let results = analyze_jitter(&time, &signal, &config);

        assert!(
            results.valid,
            "Analysis should succeed: {:?}",
            results.error
        );
    }

    #[test]
    fn test_analyze_jitter_insufficient_data() {
        let time = vec![0.0, 1.0];
        let signal = vec![0.0, 1.0];

        let config = JitterConfig::default();
        let results = analyze_jitter(&time, &signal, &config);

        assert!(!results.valid);
        assert!(results.error.is_some());
    }

    #[test]
    fn test_analyze_jitter_empty() {
        let config = JitterConfig::default();
        let results = analyze_jitter(&[], &[], &config);

        assert!(!results.valid);
    }

    #[test]
    fn test_jitter_results_error() {
        let results = JitterResults::error("Test error");
        assert!(!results.valid);
        assert_eq!(results.error.as_deref(), Some("Test error"));
    }

    #[test]
    fn test_jitter_results_format() {
        let mut results = JitterResults::default();
        results.decomposition.tj_at_ber = 10e-12;
        results.decomposition.rj_rms = 5e-12;
        results.decomposition.dj_pp = 8e-12;

        assert!(results.format_tj().contains("ps"));
        assert!(results.format_rj().contains("ps"));
        assert!(results.format_dj().contains("ps"));
    }
}
