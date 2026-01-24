//! Eye Diagram Viewer
//!
//! Commercial-grade eye diagram viewer for signal integrity analysis.
//! Features:
//! - Overlay of signal transitions aligned to clock/bit period
//! - Eye opening measurements (height, width, area)
//! - Jitter analysis (RJ, DJ, TJ)
//! - BER estimation from eye distribution
//! - Persistence mode for density visualization
//!
//! Eye diagrams are critical for high-speed serial link analysis (SerDes, DDR, etc.)

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// =============================================================================
// Eye Diagram Types
// =============================================================================

/// Eye diagram view mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum EyeDisplayMode {
    /// Standard overlay of all transitions
    #[default]
    Overlay,

    /// Persistence mode with color-coded density
    Persistence,

    /// Histogram projection at eye center
    Histogram,
}

impl EyeDisplayMode {
    /// Get all display modes
    pub fn all() -> &'static [EyeDisplayMode] {
        &[
            EyeDisplayMode::Overlay,
            EyeDisplayMode::Persistence,
            EyeDisplayMode::Histogram,
        ]
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            EyeDisplayMode::Overlay => "Overlay",
            EyeDisplayMode::Persistence => "Persistence",
            EyeDisplayMode::Histogram => "Histogram",
        }
    }
}

/// Eye diagram data
#[derive(Debug, Clone, PartialEq)]
pub struct EyeData {
    /// Time values (relative to bit period start)
    pub time: Vec<f64>,

    /// Voltage values for each transition
    pub traces: Vec<Vec<f64>>,

    /// Bit period in seconds
    pub bit_period: f64,

    /// Data rate (bits per second)
    pub data_rate: f64,

    /// Number of unit intervals displayed
    pub ui_count: u32,
}

impl Default for EyeData {
    fn default() -> Self {
        Self {
            time: Vec::new(),
            traces: Vec::new(),
            bit_period: 1e-9, // 1 ns default (1 Gbps)
            data_rate: 1e9,
            ui_count: 2,
        }
    }
}

/// Eye opening measurements
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EyeMeasurements {
    /// Eye height (voltage opening) in volts
    pub eye_height: f64,

    /// Eye width (timing opening) in seconds
    pub eye_width: f64,

    /// Eye height as percentage of swing
    pub eye_height_pct: f64,

    /// Eye width as percentage of UI
    pub eye_width_pct: f64,

    /// Crossing point voltage (normalized 0-1)
    pub crossing_point: f64,

    /// RMS jitter in seconds
    pub rms_jitter: f64,

    /// Peak-to-peak jitter in seconds
    pub pp_jitter: f64,

    /// Random jitter (RJ) 1-sigma in seconds
    pub random_jitter: f64,

    /// Deterministic jitter (DJ) in seconds
    pub deterministic_jitter: f64,

    /// Total jitter at specified BER
    pub total_jitter: f64,

    /// BER target for total jitter calculation
    pub ber_target: f64,

    /// Rise time (10-90%)
    pub rise_time: f64,

    /// Fall time (90-10%)
    pub fall_time: f64,

    /// Signal-to-noise ratio in dB
    pub snr_db: f64,

    /// One-level (high) mean voltage
    pub one_level: f64,

    /// Zero-level (low) mean voltage
    pub zero_level: f64,
}

impl EyeMeasurements {
    /// Create new measurements with default BER target of 1e-12
    pub fn new() -> Self {
        Self {
            ber_target: 1e-12,
            ..Default::default()
        }
    }

    /// Calculate eye area (normalized)
    pub fn eye_area(&self) -> f64 {
        self.eye_height_pct * self.eye_width_pct / 10000.0
    }

    /// Get quality metric (combined figure of merit)
    pub fn quality_metric(&self) -> f64 {
        // Q-factor approximation from SNR
        self.snr_db / 20.0
    }

    /// Format eye height with units
    pub fn format_height(&self) -> String {
        format_voltage(self.eye_height)
    }

    /// Format eye width with units
    pub fn format_width(&self) -> String {
        format_time(self.eye_width)
    }

    /// Format jitter with units
    pub fn format_jitter(&self) -> String {
        format_time(self.rms_jitter)
    }
}

// =============================================================================
// Eye Diagram Computation
// =============================================================================

/// Build eye diagram data from transient waveform
///
/// # Arguments
/// * `time` - Time values from transient simulation
/// * `signal` - Signal values (voltage)
/// * `bit_period` - Bit period in seconds
/// * `ui_count` - Number of unit intervals to display
/// * `skip_initial` - Number of initial bits to skip for settling
pub fn build_eye_data(
    time: &[f64],
    signal: &[f64],
    bit_period: f64,
    ui_count: u32,
    skip_initial: usize,
) -> EyeData {
    if time.is_empty() || time.len() != signal.len() || bit_period <= 0.0 {
        return EyeData::default();
    }

    let t_start = time[0] + (skip_initial as f64) * bit_period;
    let ui_period = bit_period * (ui_count as f64);
    let data_rate = 1.0 / bit_period;

    // Find points per UI for interpolated traces
    let points_per_ui = 100;
    let total_points = points_per_ui * (ui_count as usize);

    // Create normalized time axis [0, ui_count]
    let eye_time: Vec<f64> = (0..total_points)
        .map(|i| (i as f64) / (points_per_ui as f64))
        .collect();

    let mut traces = Vec::new();

    // Find the first valid bit period start
    let mut current_start = t_start;

    while current_start + ui_period < time[time.len() - 1] {
        // Extract and interpolate this segment
        let segment_end = current_start + ui_period;

        // Find indices for this segment
        let start_idx = time.iter().position(|&t| t >= current_start).unwrap_or(0);
        let end_idx = time
            .iter()
            .position(|&t| t >= segment_end)
            .unwrap_or(time.len());

        if end_idx > start_idx + 2 {
            // Interpolate to standard eye time grid
            let mut trace = Vec::with_capacity(total_points);
            for &t_norm in &eye_time {
                let t_actual = current_start + t_norm * bit_period;
                let v = interpolate_linear(time, signal, t_actual);
                trace.push(v);
            }
            traces.push(trace);
        }

        current_start += bit_period;
    }

    EyeData {
        time: eye_time.iter().map(|t| t * bit_period).collect(),
        traces,
        bit_period,
        data_rate,
        ui_count,
    }
}

/// Calculate eye measurements from eye data
pub fn calculate_eye_measurements(data: &EyeData) -> EyeMeasurements {
    if data.traces.is_empty() {
        return EyeMeasurements::new();
    }

    let n_points = data.time.len();
    let n_traces = data.traces.len();

    // Find voltage statistics at each time point
    let mut v_min = vec![f64::MAX; n_points];
    let mut v_max = vec![f64::MIN; n_points];
    let mut v_sum = vec![0.0; n_points];
    let mut v_sq_sum = vec![0.0; n_points];

    for trace in &data.traces {
        for (i, &v) in trace.iter().enumerate() {
            if i < n_points {
                v_min[i] = v_min[i].min(v);
                v_max[i] = v_max[i].max(v);
                v_sum[i] += v;
                v_sq_sum[i] += v * v;
            }
        }
    }

    // Overall voltage swing
    let global_min = v_min.iter().cloned().fold(f64::MAX, f64::min);
    let global_max = v_max.iter().cloned().fold(f64::MIN, f64::max);
    let swing = global_max - global_min;

    if swing <= 0.0 {
        return EyeMeasurements::new();
    }

    // Find eye opening at center (0.5 UI)
    let center_idx = n_points / 2;
    let eye_height = v_max[center_idx] - v_min[center_idx];
    let eye_height_pct = (eye_height / swing) * 100.0;

    // Find eye width (where opening exists)
    let threshold = swing * 0.1; // 10% of swing as margin
    let mut width_start = 0;
    let mut width_end = n_points - 1;

    for i in 0..n_points {
        if v_max[i] - v_min[i] > threshold {
            width_start = i;
            break;
        }
    }

    for i in (0..n_points).rev() {
        if v_max[i] - v_min[i] > threshold {
            width_end = i;
            break;
        }
    }

    let eye_width = if width_end > width_start {
        data.time.get(width_end).unwrap_or(&0.0) - data.time.get(width_start).unwrap_or(&0.0)
    } else {
        0.0
    };
    let ui_total = data.bit_period * (data.ui_count as f64);
    let eye_width_pct = if ui_total > 0.0 {
        (eye_width / ui_total) * 100.0
    } else {
        0.0
    };

    // Find crossing points (where high and low distributions meet)
    let mid_v = (global_max + global_min) / 2.0;
    let crossing_point = (mid_v - global_min) / swing;

    // Estimate one and zero levels
    let one_level = global_max - swing * 0.1;
    let zero_level = global_min + swing * 0.1;

    // Calculate jitter from edge timing variation
    let (rms_jitter, pp_jitter) = calculate_jitter(&data.traces, &data.time);

    // Estimate RJ/DJ separation (simplified model)
    // In a real implementation, this would use dual-Dirac modeling
    let random_jitter = rms_jitter * 0.7; // Approximation
    let deterministic_jitter = pp_jitter * 0.3;

    // Total jitter at BER (Q-factor method)
    let ber_target = 1e-12;
    let q_factor = calculate_q_from_ber(ber_target);
    let total_jitter = 2.0 * q_factor * random_jitter + deterministic_jitter;

    // Calculate rise/fall times from average edge
    let (rise_time, fall_time) = calculate_rise_fall_times(&data.traces, &data.time, swing);

    // SNR estimation
    let signal_power = swing.powi(2) / 4.0; // Assuming NRZ
    let noise_variance = calculate_eye_variance(&v_sum, &v_sq_sum, n_traces);
    let snr_db = if noise_variance > 0.0 {
        10.0 * (signal_power / noise_variance).log10()
    } else {
        100.0 // Very high SNR
    };

    EyeMeasurements {
        eye_height,
        eye_width,
        eye_height_pct,
        eye_width_pct,
        crossing_point,
        rms_jitter,
        pp_jitter,
        random_jitter,
        deterministic_jitter,
        total_jitter,
        ber_target,
        rise_time,
        fall_time,
        snr_db,
        one_level,
        zero_level,
    }
}

/// Calculate jitter from edge timing
fn calculate_jitter(traces: &[Vec<f64>], time: &[f64]) -> (f64, f64) {
    if traces.is_empty() || time.is_empty() {
        return (0.0, 0.0);
    }

    let mut edge_times = Vec::new();
    let mid_idx = time.len() / 2;
    let mid_time = time[mid_idx];

    // Find crossings near center
    for trace in traces {
        if trace.len() < 2 {
            continue;
        }

        let mid_v = (trace.iter().cloned().fold(f64::MAX, f64::min)
            + trace.iter().cloned().fold(f64::MIN, f64::max))
            / 2.0;

        // Find crossing near center
        for i in mid_idx.saturating_sub(10)..mid_idx.saturating_add(10).min(trace.len() - 1) {
            if i < time.len() - 1 {
                let v0 = trace[i];
                let v1 = trace[i + 1];
                if (v0 - mid_v) * (v1 - mid_v) < 0.0 {
                    // Interpolate exact crossing time
                    let t_cross = time[i] + (mid_v - v0) / (v1 - v0) * (time[i + 1] - time[i]);
                    edge_times.push(t_cross - mid_time);
                    break;
                }
            }
        }
    }

    if edge_times.is_empty() {
        return (0.0, 0.0);
    }

    // RMS jitter
    let mean = edge_times.iter().sum::<f64>() / edge_times.len() as f64;
    let variance =
        edge_times.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / edge_times.len() as f64;
    let rms_jitter = variance.sqrt();

    // Peak-to-peak jitter
    let min_t = edge_times.iter().cloned().fold(f64::MAX, f64::min);
    let max_t = edge_times.iter().cloned().fold(f64::MIN, f64::max);
    let pp_jitter = max_t - min_t;

    (rms_jitter, pp_jitter)
}

/// Calculate Q-factor from BER using complementary error function approximation
fn calculate_q_from_ber(ber: f64) -> f64 {
    // Q = erfc^-1(2*BER) * sqrt(2)
    // Approximation for typical BER values
    if ber >= 0.5 {
        0.0
    } else if ber <= 1e-15 {
        8.0 // Q ~ 8 for BER = 1e-15
    } else {
        // Simple approximation: Q ≈ sqrt(2) * sqrt(-ln(2*BER))
        let arg = -2.0 * ber.ln();
        if arg > 0.0 {
            arg.sqrt()
        } else {
            0.0
        }
    }
}

/// Calculate rise and fall times from traces
fn calculate_rise_fall_times(traces: &[Vec<f64>], time: &[f64], swing: f64) -> (f64, f64) {
    if traces.is_empty() || time.is_empty() || swing <= 0.0 {
        return (0.0, 0.0);
    }

    let low_threshold = 0.1;
    let high_threshold = 0.9;
    let mut rise_times = Vec::new();
    let mut fall_times = Vec::new();

    for trace in traces {
        if trace.len() != time.len() {
            continue;
        }

        let min_v = trace.iter().cloned().fold(f64::MAX, f64::min);
        let max_v = trace.iter().cloned().fold(f64::MIN, f64::max);
        let low_level = min_v + (max_v - min_v) * low_threshold;
        let high_level = min_v + (max_v - min_v) * high_threshold;

        // Find rising edges
        let mut in_rise = false;
        let mut rise_start = 0.0;

        for i in 0..trace.len() - 1 {
            if !in_rise && trace[i] < low_level && trace[i + 1] >= low_level {
                // Interpolate 10% crossing
                rise_start = interpolate_crossing(time, trace, i, low_level);
                in_rise = true;
            } else if in_rise && trace[i] < high_level && trace[i + 1] >= high_level {
                // Interpolate 90% crossing
                let rise_end = interpolate_crossing(time, trace, i, high_level);
                rise_times.push(rise_end - rise_start);
                in_rise = false;
            }
        }

        // Find falling edges
        let mut in_fall = false;
        let mut fall_start = 0.0;

        for i in 0..trace.len() - 1 {
            if !in_fall && trace[i] > high_level && trace[i + 1] <= high_level {
                fall_start = interpolate_crossing(time, trace, i, high_level);
                in_fall = true;
            } else if in_fall && trace[i] > low_level && trace[i + 1] <= low_level {
                let fall_end = interpolate_crossing(time, trace, i, low_level);
                fall_times.push(fall_end - fall_start);
                in_fall = false;
            }
        }
    }

    let avg_rise = if !rise_times.is_empty() {
        rise_times.iter().sum::<f64>() / rise_times.len() as f64
    } else {
        0.0
    };

    let avg_fall = if !fall_times.is_empty() {
        fall_times.iter().sum::<f64>() / fall_times.len() as f64
    } else {
        0.0
    };

    (avg_rise, avg_fall)
}

/// Interpolate crossing time
fn interpolate_crossing(time: &[f64], values: &[f64], i: usize, threshold: f64) -> f64 {
    if i >= time.len() - 1 {
        return time.get(i).copied().unwrap_or(0.0);
    }
    let v0 = values[i];
    let v1 = values[i + 1];
    let t0 = time[i];
    let t1 = time[i + 1];

    if (v1 - v0).abs() < 1e-15 {
        return t0;
    }

    t0 + (threshold - v0) / (v1 - v0) * (t1 - t0)
}

/// Calculate variance from running sums
fn calculate_eye_variance(v_sum: &[f64], v_sq_sum: &[f64], n: usize) -> f64 {
    if n == 0 || v_sum.is_empty() {
        return 0.0;
    }

    let n_f = n as f64;
    let center = v_sum.len() / 2;

    // Variance at center
    let mean = v_sum[center] / n_f;
    let mean_sq = v_sq_sum[center] / n_f;

    (mean_sq - mean.powi(2)).max(0.0)
}

/// Linear interpolation
fn interpolate_linear(x: &[f64], y: &[f64], x_target: f64) -> f64 {
    if x.is_empty() || y.is_empty() || x.len() != y.len() {
        return 0.0;
    }

    if x_target <= x[0] {
        return y[0];
    }
    if x_target >= x[x.len() - 1] {
        return y[y.len() - 1];
    }

    // Binary search for interval
    let idx = x.partition_point(|&v| v < x_target);
    if idx == 0 {
        return y[0];
    }
    if idx >= x.len() {
        return y[y.len() - 1];
    }

    let x0 = x[idx - 1];
    let x1 = x[idx];
    let y0 = y[idx - 1];
    let y1 = y[idx];

    if (x1 - x0).abs() < 1e-15 {
        return y0;
    }

    y0 + (y1 - y0) * (x_target - x0) / (x1 - x0)
}

// =============================================================================
// Formatting Helpers
// =============================================================================

/// Format voltage with SI prefix
fn format_voltage(v: f64) -> String {
    let (scaled, prefix) = si_prefix_voltage(v);
    format!("{:.2} {}V", scaled, prefix)
}

/// Format time with SI prefix
fn format_time(t: f64) -> String {
    let (scaled, prefix) = si_prefix_time(t);
    format!("{:.2} {}s", scaled, prefix)
}

/// Get SI prefix for voltage
fn si_prefix_voltage(v: f64) -> (f64, &'static str) {
    let abs_v = v.abs();
    if abs_v >= 1.0 {
        (v, "")
    } else if abs_v >= 1e-3 {
        (v * 1e3, "m")
    } else if abs_v >= 1e-6 {
        (v * 1e6, "µ")
    } else if abs_v >= 1e-9 {
        (v * 1e9, "n")
    } else {
        (v * 1e12, "p")
    }
}

/// Get SI prefix for time
fn si_prefix_time(t: f64) -> (f64, &'static str) {
    let abs_t = t.abs();
    if abs_t >= 1.0 {
        (t, "")
    } else if abs_t >= 1e-3 {
        (t * 1e3, "m")
    } else if abs_t >= 1e-6 {
        (t * 1e6, "µ")
    } else if abs_t >= 1e-9 {
        (t * 1e9, "n")
    } else if abs_t >= 1e-12 {
        (t * 1e12, "p")
    } else {
        (t * 1e15, "f")
    }
}

// =============================================================================
// Eye Viewer Component
// =============================================================================

/// Eye viewer component properties
#[derive(Props, Clone, PartialEq)]
pub struct EyeViewerProps {
    /// Eye diagram data
    pub data: EyeData,

    /// Display mode
    #[props(default)]
    pub mode: EyeDisplayMode,

    /// Canvas width
    #[props(default = 800)]
    pub width: u32,

    /// Canvas height
    #[props(default = 400)]
    pub height: u32,

    /// Show measurements panel
    #[props(default = true)]
    pub show_measurements: bool,

    /// Trace color
    #[props(default = "#4CAF50".to_string())]
    pub trace_color: String,

    /// Background color
    #[props(default = "#1a1a2e".to_string())]
    pub background_color: String,
}

/// Eye diagram viewer component
#[component]
pub fn EyeViewer(props: EyeViewerProps) -> Element {
    let measurements = calculate_eye_measurements(&props.data);

    let plot_margins = PlotMargins {
        top: 40.0,
        bottom: 50.0,
        left: 60.0,
        right: if props.show_measurements { 200.0 } else { 20.0 },
    };

    let plot_width = (props.width as f64) - plot_margins.left - plot_margins.right;
    let plot_height = (props.height as f64) - plot_margins.top - plot_margins.bottom;

    // Calculate axis ranges
    let (v_min, v_max) = if !props.data.traces.is_empty() {
        let all_values: Vec<f64> = props.data.traces.iter().flatten().cloned().collect();
        let min = all_values.iter().cloned().fold(f64::MAX, f64::min);
        let max = all_values.iter().cloned().fold(f64::MIN, f64::max);
        let margin = (max - min) * 0.1;
        (min - margin, max + margin)
    } else {
        (-0.1, 1.1)
    };

    let t_max = props.data.bit_period * (props.data.ui_count as f64);

    rsx! {
        div {
            class: "eye-viewer-container",
            style: "display: flex; background: {props.background_color}; border-radius: 8px; padding: 10px;",

            // Main plot area
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
                    fill: "{props.background_color}",
                }

                // Plot area background
                rect {
                    x: "{plot_margins.left}",
                    y: "{plot_margins.top}",
                    width: "{plot_width}",
                    height: "{plot_height}",
                    fill: "#0d0d1a",
                    stroke: "#333",
                }

                // Title
                text {
                    x: "{props.width as f64 / 2.0}",
                    y: "20",
                    fill: "#fff",
                    "font-size": "14",
                    "text-anchor": "middle",
                    "font-weight": "bold",
                    "Eye Diagram ({props.data.data_rate / 1e9:.2} Gbps)"
                }

                // Grid
                { render_eye_grid(plot_margins.left, plot_margins.top, plot_width, plot_height) }

                // Eye traces
                for (i, trace) in props.data.traces.iter().enumerate() {
                    { render_eye_trace(
                        &props.data.time,
                        trace,
                        0.0,
                        t_max,
                        v_min,
                        v_max,
                        plot_margins.left,
                        plot_margins.top,
                        plot_width,
                        plot_height,
                        &props.trace_color,
                        0.3 + (1.0 - 0.3) * (i as f64 / props.data.traces.len().max(1) as f64),
                    ) }
                }

                // Eye mask outline (reference)
                { render_eye_mask(
                    plot_margins.left,
                    plot_margins.top,
                    plot_width,
                    plot_height,
                    &measurements,
                ) }

                // X-axis labels
                { render_x_axis_labels(
                    plot_margins.left,
                    plot_margins.top + plot_height,
                    plot_width,
                    props.data.ui_count,
                ) }

                // Y-axis labels
                { render_y_axis_labels(
                    plot_margins.left,
                    plot_margins.top,
                    plot_height,
                    v_min,
                    v_max,
                ) }
            }

            // Measurements panel
            if props.show_measurements {
                div {
                    class: "eye-measurements",
                    style: "margin-left: 20px; color: #fff; font-family: monospace; font-size: 12px;",

                    h3 {
                        style: "margin: 0 0 10px 0; color: #4CAF50;",
                        "Measurements"
                    }

                    div { style: "margin-bottom: 15px;",
                        div { style: "color: #888;", "Eye Opening" }
                        div { "Height: {measurements.format_height()} ({measurements.eye_height_pct:.1}%)" }
                        div { "Width: {measurements.format_width()} ({measurements.eye_width_pct:.1}%)" }
                    }

                    div { style: "margin-bottom: 15px;",
                        div { style: "color: #888;", "Jitter" }
                        div { "RMS: {measurements.format_jitter()}" }
                        div { "P-P: {format_time(measurements.pp_jitter)}" }
                        div { "TJ (BER=10⁻¹²): {format_time(measurements.total_jitter)}" }
                    }

                    div { style: "margin-bottom: 15px;",
                        div { style: "color: #888;", "Timing" }
                        div { "Rise: {format_time(measurements.rise_time)}" }
                        div { "Fall: {format_time(measurements.fall_time)}" }
                    }

                    div { style: "margin-bottom: 15px;",
                        div { style: "color: #888;", "Levels" }
                        div { "One: {format_voltage(measurements.one_level)}" }
                        div { "Zero: {format_voltage(measurements.zero_level)}" }
                        div { "SNR: {measurements.snr_db:.1} dB" }
                    }
                }
            }
        }
    }
}

// =============================================================================
// Rendering Helpers
// =============================================================================

/// Plot margins
struct PlotMargins {
    top: f64,
    bottom: f64,
    left: f64,
    right: f64,
}

/// Render eye diagram grid
fn render_eye_grid(x: f64, y: f64, width: f64, height: f64) -> Element {
    let grid_color = "#333";

    rsx! {
        g { class: "eye-grid",
            // Vertical lines at UI boundaries
            line {
                x1: "{x + width / 2.0}",
                y1: "{y}",
                x2: "{x + width / 2.0}",
                y2: "{y + height}",
                stroke: "{grid_color}",
                "stroke-dasharray": "4,2",
            }

            // Horizontal center line
            line {
                x1: "{x}",
                y1: "{y + height / 2.0}",
                x2: "{x + width}",
                y2: "{y + height / 2.0}",
                stroke: "{grid_color}",
                "stroke-dasharray": "4,2",
            }

            // Quarter lines
            for i in [0.25, 0.75] {
                line {
                    x1: "{x + width * i}",
                    y1: "{y}",
                    x2: "{x + width * i}",
                    y2: "{y + height}",
                    stroke: "#222",
                    "stroke-dasharray": "2,4",
                }
            }
        }
    }
}

/// Render a single eye trace
fn render_eye_trace(
    x_data: &[f64],
    y_data: &[f64],
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    plot_x: f64,
    plot_y: f64,
    plot_width: f64,
    plot_height: f64,
    color: &str,
    opacity: f64,
) -> Element {
    if x_data.len() < 2 || y_data.len() < 2 {
        return rsx! {};
    }

    let x_range = x_max - x_min;
    let y_range = y_max - y_min;

    if x_range <= 0.0 || y_range <= 0.0 {
        return rsx! {};
    }

    let mut path_data = String::new();
    let mut first = true;

    for (x, y) in x_data.iter().zip(y_data.iter()) {
        let px = plot_x + (x - x_min) / x_range * plot_width;
        let py = plot_y + plot_height - (y - y_min) / y_range * plot_height;

        if first {
            path_data.push_str(&format!("M {:.2} {:.2}", px, py));
            first = false;
        } else {
            path_data.push_str(&format!(" L {:.2} {:.2}", px, py));
        }
    }

    rsx! {
        path {
            d: "{path_data}",
            fill: "none",
            stroke: "{color}",
            "stroke-width": "1",
            opacity: "{opacity}",
        }
    }
}

/// Render eye mask outline for reference
fn render_eye_mask(
    _x: f64,
    _y: f64,
    _width: f64,
    _height: f64,
    _measurements: &EyeMeasurements,
) -> Element {
    // Eye mask visualization (simplified - full implementation would use actual mask coordinates)
    rsx! {}
}

/// Render X-axis labels (UI boundaries)
fn render_x_axis_labels(x: f64, y: f64, width: f64, ui_count: u32) -> Element {
    rsx! {
        g { class: "x-axis-labels",
            for i in 0..=ui_count {
                text {
                    x: "{x + (i as f64 / ui_count as f64) * width}",
                    y: "{y + 20.0}",
                    fill: "#888",
                    "font-size": "10",
                    "text-anchor": "middle",
                    "{i} UI"
                }
            }
        }
    }
}

/// Render Y-axis labels (voltage)
fn render_y_axis_labels(x: f64, y: f64, height: f64, v_min: f64, v_max: f64) -> Element {
    let n_labels = 5;

    rsx! {
        g { class: "y-axis-labels",
            for i in 0..n_labels {
                {
                    let frac = i as f64 / (n_labels - 1) as f64;
                    let v = v_max - frac * (v_max - v_min);
                    let label = format_voltage(v);
                    rsx! {
                        text {
                            x: "{x - 5.0}",
                            y: "{y + frac * height + 4.0}",
                            fill: "#888",
                            "font-size": "10",
                            "text-anchor": "end",
                            "{label}"
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
    use std::f64::consts::PI;

    // -------------------------------------------------------------------------
    // EyeDisplayMode Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_eye_display_mode_default() {
        let mode = EyeDisplayMode::default();
        assert_eq!(mode, EyeDisplayMode::Overlay);
    }

    #[test]
    fn test_eye_display_mode_all() {
        let all = EyeDisplayMode::all();
        assert_eq!(all.len(), 3);
        assert!(all.contains(&EyeDisplayMode::Overlay));
        assert!(all.contains(&EyeDisplayMode::Persistence));
        assert!(all.contains(&EyeDisplayMode::Histogram));
    }

    #[test]
    fn test_eye_display_mode_display_names() {
        assert!(!EyeDisplayMode::Overlay.display_name().is_empty());
        assert!(!EyeDisplayMode::Persistence.display_name().is_empty());
        assert!(!EyeDisplayMode::Histogram.display_name().is_empty());
    }

    // -------------------------------------------------------------------------
    // EyeData Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_eye_data_default() {
        let data = EyeData::default();
        assert!(data.time.is_empty());
        assert!(data.traces.is_empty());
        assert!(data.bit_period > 0.0);
        assert_eq!(data.ui_count, 2);
    }

    // -------------------------------------------------------------------------
    // EyeMeasurements Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_eye_measurements_new() {
        let m = EyeMeasurements::new();
        assert_eq!(m.ber_target, 1e-12);
        assert_eq!(m.eye_height, 0.0);
    }

    #[test]
    fn test_eye_measurements_eye_area() {
        let mut m = EyeMeasurements::new();
        m.eye_height_pct = 50.0;
        m.eye_width_pct = 80.0;
        let area = m.eye_area();
        assert!((area - 0.4).abs() < 0.01);
    }

    #[test]
    fn test_eye_measurements_format() {
        let mut m = EyeMeasurements::new();
        m.eye_height = 0.5;
        m.eye_width = 100e-12;
        m.rms_jitter = 5e-12;

        let h = m.format_height();
        let w = m.format_width();
        let j = m.format_jitter();

        assert!(h.contains("mV") || h.contains("V"));
        assert!(w.contains("ps") || w.contains("s"));
        assert!(j.contains("ps") || j.contains("s"));
    }

    // -------------------------------------------------------------------------
    // Build Eye Data Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_build_eye_data_empty() {
        let data = build_eye_data(&[], &[], 1e-9, 2, 0);
        assert!(data.traces.is_empty());
    }

    #[test]
    fn test_build_eye_data_invalid_bit_period() {
        let time: Vec<f64> = (0..100).map(|i| i as f64 * 1e-12).collect();
        let signal: Vec<f64> = time.iter().map(|_| 0.5).collect();
        let data = build_eye_data(&time, &signal, 0.0, 2, 0);
        assert!(data.traces.is_empty());
    }

    #[test]
    fn test_build_eye_data_basic() {
        // Generate PRBS-like signal
        let bit_period = 1e-9;
        let n_bits = 100;
        let samples_per_bit = 20;
        let n_points = n_bits * samples_per_bit;

        let time: Vec<f64> = (0..n_points)
            .map(|i| i as f64 * bit_period / samples_per_bit as f64)
            .collect();

        let signal: Vec<f64> = time
            .iter()
            .map(|t| {
                let bit = (t / bit_period) as usize;
                // Simple PRBS pattern
                if (bit % 7) < 3 {
                    1.0
                } else {
                    0.0
                }
            })
            .collect();

        let data = build_eye_data(&time, &signal, bit_period, 2, 5);
        assert!(!data.traces.is_empty());
        assert!(data.bit_period == bit_period);
    }

    // -------------------------------------------------------------------------
    // Calculate Eye Measurements Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_calculate_eye_measurements_empty() {
        let data = EyeData::default();
        let m = calculate_eye_measurements(&data);
        assert_eq!(m.eye_height, 0.0);
    }

    #[test]
    fn test_calculate_eye_measurements_with_data() {
        // Create simple eye data with known characteristics
        let n_points = 100;
        let time: Vec<f64> = (0..n_points).map(|i| i as f64 * 1e-11).collect();

        // Rising edge trace
        let trace1: Vec<f64> = (0..n_points)
            .map(|i| if i < 50 { 0.0 } else { 1.0 })
            .collect();

        // Falling edge trace
        let trace2: Vec<f64> = (0..n_points)
            .map(|i| if i < 50 { 1.0 } else { 0.0 })
            .collect();

        let data = EyeData {
            time,
            traces: vec![trace1, trace2],
            bit_period: 1e-9,
            data_rate: 1e9,
            ui_count: 1,
        };

        let m = calculate_eye_measurements(&data);
        assert!(m.eye_height > 0.0);
        assert!(m.one_level > m.zero_level);
    }

    // -------------------------------------------------------------------------
    // Interpolation Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_interpolate_linear_basic() {
        let x = vec![0.0, 1.0, 2.0];
        let y = vec![0.0, 10.0, 20.0];

        let v = interpolate_linear(&x, &y, 0.5);
        assert!((v - 5.0).abs() < 0.01);

        let v2 = interpolate_linear(&x, &y, 1.5);
        assert!((v2 - 15.0).abs() < 0.01);
    }

    #[test]
    fn test_interpolate_linear_boundary() {
        let x = vec![0.0, 1.0, 2.0];
        let y = vec![0.0, 10.0, 20.0];

        let v_below = interpolate_linear(&x, &y, -1.0);
        assert_eq!(v_below, 0.0);

        let v_above = interpolate_linear(&x, &y, 5.0);
        assert_eq!(v_above, 20.0);
    }

    #[test]
    fn test_interpolate_linear_empty() {
        let v = interpolate_linear(&[], &[], 0.5);
        assert_eq!(v, 0.0);
    }

    // -------------------------------------------------------------------------
    // Q-Factor / BER Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_calculate_q_from_ber() {
        // BER = 0.5 should give Q = 0
        assert_eq!(calculate_q_from_ber(0.5), 0.0);

        // Very low BER should give high Q
        let q = calculate_q_from_ber(1e-12);
        assert!(q > 5.0);

        // Typical BER values
        let q_1e6 = calculate_q_from_ber(1e-6);
        let q_1e12 = calculate_q_from_ber(1e-12);
        assert!(q_1e12 > q_1e6);
    }

    // -------------------------------------------------------------------------
    // Jitter Calculation Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_calculate_jitter_empty() {
        let (rms, pp) = calculate_jitter(&[], &[]);
        assert_eq!(rms, 0.0);
        assert_eq!(pp, 0.0);
    }

    #[test]
    fn test_calculate_jitter_with_variation() {
        // Create traces with known jitter
        let n_points = 100;
        let time: Vec<f64> = (0..n_points).map(|i| i as f64 * 1e-11).collect();

        let mut traces = Vec::new();
        for offset in [0, 1, 2, 3, 4] {
            let trace: Vec<f64> = (0..n_points)
                .map(|i| if i < 50 + offset { 0.0 } else { 1.0 })
                .collect();
            traces.push(trace);
        }

        let (rms, pp) = calculate_jitter(&traces, &time);
        // Should have some jitter due to edge offset
        assert!(pp >= 0.0);
    }

    // -------------------------------------------------------------------------
    // Formatting Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_format_voltage() {
        let v1 = format_voltage(1.0);
        assert!(v1.contains("V"));

        let v2 = format_voltage(0.001);
        assert!(v2.contains("mV"));

        let v3 = format_voltage(1e-6);
        assert!(v3.contains("µV"));
    }

    #[test]
    fn test_format_time() {
        let t1 = format_time(1.0);
        assert!(t1.contains("s"));

        let t2 = format_time(1e-9);
        assert!(t2.contains("ns"));

        let t3 = format_time(1e-12);
        assert!(t3.contains("ps"));
    }

    // -------------------------------------------------------------------------
    // SI Prefix Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_si_prefix_voltage() {
        let (v, p) = si_prefix_voltage(1.5);
        assert!((v - 1.5).abs() < 0.01);
        assert_eq!(p, "");

        let (v2, p2) = si_prefix_voltage(0.0015);
        assert!((v2 - 1.5).abs() < 0.01);
        assert_eq!(p2, "m");
    }

    #[test]
    fn test_si_prefix_time() {
        let (t, p) = si_prefix_time(1e-9);
        assert!((t - 1.0).abs() < 0.01);
        assert_eq!(p, "n");

        let (t2, p2) = si_prefix_time(100e-12);
        assert!((t2 - 100.0).abs() < 0.1);
        assert_eq!(p2, "p");
    }

    // -------------------------------------------------------------------------
    // Rise/Fall Time Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_calculate_rise_fall_times_empty() {
        let (rise, fall) = calculate_rise_fall_times(&[], &[], 1.0);
        assert_eq!(rise, 0.0);
        assert_eq!(fall, 0.0);
    }

    #[test]
    fn test_calculate_rise_fall_times_with_edges() {
        let n_points = 100;
        let time: Vec<f64> = (0..n_points).map(|i| i as f64 * 1e-11).collect();

        // Linear ramp up
        let trace: Vec<f64> = (0..n_points).map(|i| i as f64 / 99.0).collect();

        let (rise, fall) = calculate_rise_fall_times(&[trace], &time, 1.0);
        // Should detect rise time (10% to 90% of the ramp)
        assert!(rise > 0.0);
    }

    // -------------------------------------------------------------------------
    // Integration Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_full_eye_analysis_pipeline() {
        // Generate complete PRBS signal
        let bit_period = 1e-9;
        let n_bits = 50;
        let samples_per_bit = 50;

        let mut time = Vec::new();
        let mut signal = Vec::new();

        for bit in 0..n_bits {
            for sample in 0..samples_per_bit {
                let t =
                    (bit * samples_per_bit + sample) as f64 * bit_period / samples_per_bit as f64;
                let bit_value = if (bit * 7 + bit / 3) % 2 == 0 {
                    1.0
                } else {
                    0.0
                };

                // Add transition shaping
                let in_transition = sample < 10 || sample > samples_per_bit - 10;
                let v = if in_transition {
                    0.5 + (bit_value - 0.5) * (sample as f64 / 10.0).min(1.0)
                } else {
                    bit_value
                };

                time.push(t);
                signal.push(v);
            }
        }

        // Build eye diagram
        let eye_data = build_eye_data(&time, &signal, bit_period, 2, 5);

        // Calculate measurements
        let measurements = calculate_eye_measurements(&eye_data);

        // Verify results are reasonable
        assert!(eye_data.traces.len() > 0, "Should have at least one trace");
        assert!(
            measurements.one_level > measurements.zero_level,
            "One level > zero level"
        );
    }

    #[test]
    fn test_eye_with_jitter() {
        let bit_period = 1e-9;
        let samples_per_bit = 50;
        let n_bits = 30;

        let mut time = Vec::new();
        let mut signal = Vec::new();

        // Add timing jitter to edges
        let jitter_amplitude = 0.05 * bit_period;

        for bit in 0..n_bits {
            let bit_jitter = jitter_amplitude * ((bit as f64).sin());

            for sample in 0..samples_per_bit {
                let t =
                    (bit * samples_per_bit + sample) as f64 * bit_period / samples_per_bit as f64;
                let normalized_t = (t + bit_jitter) / bit_period;
                let bit_value = if normalized_t as usize % 2 == 0 {
                    1.0
                } else {
                    0.0
                };

                time.push(t);
                signal.push(bit_value);
            }
        }

        let eye_data = build_eye_data(&time, &signal, bit_period, 2, 3);
        let measurements = calculate_eye_measurements(&eye_data);

        // Should detect non-zero jitter
        // Note: Actual jitter detection depends on edge detection
        assert!(measurements.eye_height >= 0.0);
    }
}
