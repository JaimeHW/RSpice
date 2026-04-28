//! Eye Diagram Measurements
//!
//! Commercial-grade signal integrity measurements from eye diagrams.
//! Includes eye opening, jitter, rise/fall times, and bit error rate estimation.

use super::data::{EyeData, calculate_edge_rate};
use std::f64::consts::PI;

// =============================================================================
// Eye Measurements
// =============================================================================

/// Complete eye diagram measurements
#[derive(Debug, Clone, Default)]
pub struct EyeMeasurements {
    /// Eye height (vertical opening) in volts
    pub eye_height: f64,
    /// Eye width (horizontal opening) as fraction of UI
    pub eye_width: f64,
    /// Eye area (normalized, height × width)
    pub eye_area: f64,
    /// Vertical eye opening margin at center
    pub vertical_margin: f64,
    /// Horizontal eye opening margin at mid-level
    pub horizontal_margin: f64,
    /// Rise time (20%-80%) in seconds
    pub rise_time: f64,
    /// Fall time (80%-20%) in seconds
    pub fall_time: f64,
    /// Total jitter (peak-to-peak) in seconds
    pub jitter_pp: f64,
    /// Random jitter (RMS) in seconds
    pub jitter_rms: f64,
    /// Deterministic jitter (peak-to-peak) in seconds
    pub jitter_dj: f64,
    /// Crossing level (volts)
    pub crossing_level: f64,
    /// Crossing percentage (fraction of swing)
    pub crossing_percentage: f64,
    /// Signal-to-noise ratio in dB
    pub snr_db: f64,
    /// Q-factor (quality factor)
    pub q_factor: f64,
    /// Estimated BER (bit error rate)
    pub estimated_ber: f64,
    /// Data rate in bits per second
    pub data_rate: f64,
    /// Unit interval in seconds
    pub unit_interval: f64,
}

impl EyeMeasurements {
    /// Create new measurements with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate eye area as normalized value
    pub fn calculate_eye_area(&self) -> f64 {
        self.eye_height * self.eye_width
    }

    /// Calculate Q-factor from eye height and noise
    pub fn calculate_q(&self, v_high: f64, v_low: f64, sigma_high: f64, sigma_low: f64) -> f64 {
        let swing = v_high - v_low;
        if swing <= 0.0 || sigma_high <= 0.0 || sigma_low <= 0.0 {
            return 0.0;
        }
        swing / (sigma_high + sigma_low)
    }

    /// Estimate BER from Q-factor using Q-function approximation
    pub fn estimate_ber_from_q(q: f64) -> f64 {
        if q <= 0.0 {
            return 1.0;
        }
        // BER ≈ erfc(Q/√2) / 2 ≈ exp(-Q²/2) / (Q·√(2π))
        let q_sq = q * q;
        if q > 8.0 {
            // Very low BER, use asymptotic
            (-q_sq / 2.0).exp() / (q * (2.0 * PI).sqrt())
        } else {
            // Use complementary error function approximation
            0.5 * erfc_approx(q / 2.0_f64.sqrt())
        }
    }

    /// Format eye height with SI prefix
    pub fn format_height(&self) -> String {
        format_si_voltage(self.eye_height)
    }

    /// Format eye width as percentage of UI
    pub fn format_width(&self) -> String {
        format!("{:.1}% UI", self.eye_width * 100.0)
    }

    /// Format jitter with SI prefix
    pub fn format_jitter(&self) -> String {
        format_si_time(self.jitter_pp)
    }

    /// Format rise time with SI prefix
    pub fn format_rise_time(&self) -> String {
        format_si_time(self.rise_time)
    }

    /// Format fall time with SI prefix
    pub fn format_fall_time(&self) -> String {
        format_si_time(self.fall_time)
    }

    /// Format BER in scientific notation
    pub fn format_ber(&self) -> String {
        if self.estimated_ber <= 0.0 {
            "< 1e-15".to_string()
        } else if self.estimated_ber >= 1.0 {
            "> 1".to_string()
        } else {
            format!("{:.2e}", self.estimated_ber)
        }
    }

    /// Format data rate
    pub fn format_data_rate(&self) -> String {
        if self.data_rate >= 1e12 {
            format!("{:.2} Tbps", self.data_rate / 1e12)
        } else if self.data_rate >= 1e9 {
            format!("{:.2} Gbps", self.data_rate / 1e9)
        } else if self.data_rate >= 1e6 {
            format!("{:.2} Mbps", self.data_rate / 1e6)
        } else {
            format!("{:.2} kbps", self.data_rate / 1e3)
        }
    }
}

// =============================================================================
// Measurement Calculator
// =============================================================================

/// Calculate comprehensive eye measurements from eye data
pub fn calculate_eye_measurements(data: &EyeData) -> EyeMeasurements {
    let mut m = EyeMeasurements::new();

    if data.traces.is_empty() {
        return m;
    }

    m.unit_interval = data.bit_period;
    m.data_rate = data.data_rate;
    m.crossing_level = data.v_cross;

    // Calculate crossing percentage
    if data.swing > 0.0 {
        m.crossing_percentage = (data.v_cross - data.v_low) / data.swing;
    }

    // Calculate eye opening at center of UI
    let center_opening = calculate_eye_opening_at_center(data);
    m.eye_height = center_opening.height;
    m.vertical_margin = center_opening.margin;

    // Calculate horizontal eye opening
    let horizontal_opening = calculate_horizontal_opening(data);
    m.eye_width = horizontal_opening.width_fraction;
    m.horizontal_margin = horizontal_opening.margin_fraction;

    // Calculate eye area
    m.eye_area = m.eye_height * m.eye_width;

    // Calculate jitter from edge crossings
    let jitter_stats = calculate_jitter_stats(data);
    m.jitter_pp = jitter_stats.peak_to_peak;
    m.jitter_rms = jitter_stats.rms;
    m.jitter_dj = jitter_stats.deterministic;

    // Calculate rise/fall times
    let edge_times = calculate_edge_times(data);
    m.rise_time = edge_times.rise;
    m.fall_time = edge_times.fall;

    // Calculate SNR and Q-factor
    let noise_stats = calculate_noise_stats(data);
    if noise_stats.sigma_high > 0.0 && noise_stats.sigma_low > 0.0 {
        m.q_factor = m.calculate_q(
            data.v_high,
            data.v_low,
            noise_stats.sigma_high,
            noise_stats.sigma_low,
        );
        m.estimated_ber = EyeMeasurements::estimate_ber_from_q(m.q_factor);

        // SNR in dB
        let signal_power = data.swing.powi(2);
        let noise_power = noise_stats.sigma_high.powi(2) + noise_stats.sigma_low.powi(2);
        if noise_power > 0.0 {
            m.snr_db = 10.0 * (signal_power / noise_power).log10();
        }
    }

    m
}

// =============================================================================
// Internal Calculation Types
// =============================================================================

/// Eye opening at center
#[derive(Debug, Clone, Default)]
struct CenterOpening {
    height: f64,
    margin: f64,
}

/// Horizontal opening
#[derive(Debug, Clone, Default)]
struct HorizontalOpening {
    width_fraction: f64,
    margin_fraction: f64,
}

/// Jitter statistics
#[derive(Debug, Clone, Default)]
struct JitterStats {
    peak_to_peak: f64,
    rms: f64,
    deterministic: f64,
}

/// Edge timing statistics
#[derive(Debug, Clone, Default)]
struct EdgeTimes {
    rise: f64,
    fall: f64,
}

/// Noise statistics
#[derive(Debug, Clone, Default)]
struct NoiseStats {
    sigma_high: f64,
    sigma_low: f64,
}

// =============================================================================
// Internal Calculation Functions
// =============================================================================

fn calculate_eye_opening_at_center(data: &EyeData) -> CenterOpening {
    if data.traces.is_empty() || data.ui_count == 0 {
        return CenterOpening::default();
    }

    // Sample at center of each UI
    let center_ui = data.ui_count as f64 / 2.0;
    let tolerance = 0.05; // ±5% of UI

    let mut high_samples = Vec::new();
    let mut low_samples = Vec::new();

    for trace in &data.traces {
        for (i, &t) in trace.time.iter().enumerate() {
            if (t - center_ui).abs() < tolerance * data.ui_count as f64 && i < trace.amplitude.len()
            {
                let v = trace.amplitude[i];
                if v.is_finite() {
                    if v >= data.v_cross {
                        high_samples.push(v);
                    } else {
                        low_samples.push(v);
                    }
                }
            }
        }
    }

    if high_samples.is_empty() || low_samples.is_empty() {
        return CenterOpening::default();
    }

    let v_high_min = high_samples.iter().copied().fold(f64::MAX, f64::min);
    let v_low_max = low_samples.iter().copied().fold(f64::MIN, f64::max);

    let height = (v_high_min - v_low_max).max(0.0);

    CenterOpening {
        height,
        margin: height / 2.0,
    }
}

fn calculate_horizontal_opening(data: &EyeData) -> HorizontalOpening {
    if data.traces.is_empty() {
        return HorizontalOpening::default();
    }

    // Find crossing times for each trace
    let mut rising_crossings = Vec::new();
    let mut falling_crossings = Vec::new();

    for trace in &data.traces {
        let n = trace.time.len().min(trace.amplitude.len());
        for i in 0..n.saturating_sub(1) {
            let v0 = trace.amplitude[i];
            let v1 = trace.amplitude[i + 1];
            let t0 = trace.time[i];
            let t1 = trace.time[i + 1];

            if !v0.is_finite() || !v1.is_finite() {
                continue;
            }

            let dt = t1 - t0;
            if dt <= 0.0 {
                continue;
            }

            // Rising crossing
            if v0 < data.v_cross && v1 >= data.v_cross {
                let t = t0 + (data.v_cross - v0) / (v1 - v0) * dt;
                rising_crossings.push(t);
            }
            // Falling crossing
            else if v0 >= data.v_cross && v1 < data.v_cross {
                let t = t0 + (v0 - data.v_cross) / (v0 - v1) * dt;
                falling_crossings.push(t);
            }
        }
    }

    if rising_crossings.is_empty() || falling_crossings.is_empty() {
        return HorizontalOpening::default();
    }

    // Calculate jitter from crossing distribution
    let _rising_mean = rising_crossings.iter().sum::<f64>() / rising_crossings.len() as f64;
    let _falling_mean = falling_crossings.iter().sum::<f64>() / falling_crossings.len() as f64;

    let rising_max = rising_crossings.iter().copied().fold(f64::MIN, f64::max);
    let _rising_min = rising_crossings.iter().copied().fold(f64::MAX, f64::min);
    let _falling_max = falling_crossings.iter().copied().fold(f64::MIN, f64::max);
    let falling_min = falling_crossings.iter().copied().fold(f64::MAX, f64::min);

    // Eye opening is space between latest rising edge and earliest falling edge
    let opening = (falling_min - rising_max).max(0.0);
    // Crossing times are already normalized in UI units, so opening is directly
    // the eye width in UI fractions.
    let width_fraction = opening;

    HorizontalOpening {
        width_fraction: width_fraction.min(1.0),
        margin_fraction: width_fraction / 2.0,
    }
}

fn calculate_jitter_stats(data: &EyeData) -> JitterStats {
    // Collect edge crossings per edge polarity to avoid mixing two eye edges.
    let mut rising_crossings = Vec::new();
    let mut falling_crossings = Vec::new();

    for trace in &data.traces {
        let n = trace.time.len().min(trace.amplitude.len());
        for i in 0..n.saturating_sub(1) {
            let v0 = trace.amplitude[i];
            let v1 = trace.amplitude[i + 1];
            let t0 = trace.time[i];
            let t1 = trace.time[i + 1];

            if !v0.is_finite() || !v1.is_finite() {
                continue;
            }

            let dt = t1 - t0;
            if dt <= 0.0 {
                continue;
            }

            // Any crossing
            if (v0 < data.v_cross && v1 >= data.v_cross)
                || (v0 >= data.v_cross && v1 < data.v_cross)
            {
                let t = if v0 < v1 {
                    t0 + (data.v_cross - v0) / (v1 - v0) * dt
                } else {
                    t0 + (v0 - data.v_cross) / (v0 - v1) * dt
                };
                let t_normalized = t.rem_euclid(1.0);
                if v0 < v1 {
                    rising_crossings.push(t_normalized);
                } else {
                    falling_crossings.push(t_normalized);
                }
            }
        }
    }

    let rising = crossing_phase_stats(&rising_crossings);
    let falling = crossing_phase_stats(&falling_crossings);
    if rising.count == 0 && falling.count == 0 {
        return JitterStats::default();
    }

    let peak_to_peak_ui = rising.peak_to_peak_ui.max(falling.peak_to_peak_ui);
    let total_count = rising.count + falling.count;
    let rms_ui = if total_count > 0 {
        ((rising.rms_ui.powi(2) * rising.count as f64
            + falling.rms_ui.powi(2) * falling.count as f64)
            / total_count as f64)
            .sqrt()
    } else {
        0.0
    };

    let peak_to_peak = peak_to_peak_ui * data.bit_period;
    let rms = rms_ui * data.bit_period;
    // Approximate DDJ by removing RJ envelope from measured total jitter.
    let dj = (peak_to_peak - 14.0 * rms).max(0.0);

    JitterStats {
        peak_to_peak,
        rms,
        deterministic: dj,
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct CrossingPhaseStats {
    peak_to_peak_ui: f64,
    rms_ui: f64,
    count: usize,
}

fn crossing_phase_stats(phases_ui: &[f64]) -> CrossingPhaseStats {
    let mut phases: Vec<f64> = phases_ui
        .iter()
        .copied()
        .filter(|v| v.is_finite())
        .map(|v| v.rem_euclid(1.0))
        .collect();
    if phases.len() < 2 {
        return CrossingPhaseStats {
            count: phases.len(),
            ..CrossingPhaseStats::default()
        };
    }
    phases.sort_by(|a, b| a.total_cmp(b));
    let unwrapped = unwrap_circular_phases(&phases);
    if unwrapped.len() < 2 {
        return CrossingPhaseStats {
            count: unwrapped.len(),
            ..CrossingPhaseStats::default()
        };
    }

    let min = unwrapped[0];
    let max = *unwrapped.last().unwrap_or(&min);
    let peak_to_peak_ui = (max - min).max(0.0);
    let mean = unwrapped.iter().sum::<f64>() / unwrapped.len() as f64;
    let variance =
        unwrapped.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / unwrapped.len() as f64;
    CrossingPhaseStats {
        peak_to_peak_ui,
        rms_ui: variance.sqrt(),
        count: unwrapped.len(),
    }
}

fn unwrap_circular_phases(sorted_phases: &[f64]) -> Vec<f64> {
    if sorted_phases.len() < 2 {
        return sorted_phases.to_vec();
    }

    let mut max_gap = f64::MIN;
    let mut cut_after = 0usize;
    for idx in 0..sorted_phases.len() {
        let next_idx = (idx + 1) % sorted_phases.len();
        let current = sorted_phases[idx];
        let next = if next_idx == 0 {
            sorted_phases[next_idx] + 1.0
        } else {
            sorted_phases[next_idx]
        };
        let gap = next - current;
        if gap > max_gap {
            max_gap = gap;
            cut_after = idx;
        }
    }

    let start = (cut_after + 1) % sorted_phases.len();
    let mut out = Vec::with_capacity(sorted_phases.len());
    for i in 0..sorted_phases.len() {
        let idx = (start + i) % sorted_phases.len();
        let mut value = sorted_phases[idx];
        if idx < start {
            value += 1.0;
        }
        out.push(value);
    }
    out
}

fn calculate_edge_times(data: &EyeData) -> EdgeTimes {
    // Build combined time/signal arrays from all traces
    let mut combined_time = Vec::new();
    let mut combined_signal = Vec::new();

    for trace in &data.traces {
        // Scale time to absolute
        for (i, &t) in trace.time.iter().enumerate() {
            if i < trace.amplitude.len() {
                combined_time.push(t * data.bit_period);
                combined_signal.push(trace.amplitude[i]);
            }
        }
    }

    // Calculate 20%-80% thresholds
    let v_20 = data.v_low + 0.2 * data.swing;
    let v_80 = data.v_low + 0.8 * data.swing;

    let rise =
        calculate_edge_rate(&combined_time, &combined_signal, v_20, v_80, true).unwrap_or(0.0);
    let fall =
        calculate_edge_rate(&combined_time, &combined_signal, v_20, v_80, false).unwrap_or(0.0);

    EdgeTimes { rise, fall }
}

fn calculate_noise_stats(data: &EyeData) -> NoiseStats {
    // Sample at center of eye for noise estimation
    let center_ui = data.ui_count as f64 / 2.0;
    let tolerance = 0.1;

    let mut high_samples = Vec::new();
    let mut low_samples = Vec::new();

    for trace in &data.traces {
        for (i, &t) in trace.time.iter().enumerate() {
            if (t - center_ui).abs() < tolerance && i < trace.amplitude.len() {
                let v = trace.amplitude[i];
                if v.is_finite() {
                    if v >= data.v_cross {
                        high_samples.push(v);
                    } else {
                        low_samples.push(v);
                    }
                }
            }
        }
    }

    let sigma_high = if high_samples.len() > 1 {
        let mean = high_samples.iter().sum::<f64>() / high_samples.len() as f64;
        let variance = high_samples.iter().map(|v| (v - mean).powi(2)).sum::<f64>()
            / high_samples.len() as f64;
        variance.sqrt()
    } else {
        0.0
    };

    let sigma_low = if low_samples.len() > 1 {
        let mean = low_samples.iter().sum::<f64>() / low_samples.len() as f64;
        let variance =
            low_samples.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / low_samples.len() as f64;
        variance.sqrt()
    } else {
        0.0
    };

    NoiseStats {
        sigma_high,
        sigma_low,
    }
}

// =============================================================================
// Utility Functions
// =============================================================================

/// Complementary error function approximation
fn erfc_approx(x: f64) -> f64 {
    // Abramowitz and Stegun approximation
    let t = 1.0 / (1.0 + 0.5 * x.abs());
    let tau = t
        * (-x.powi(2) - 1.26551223
            + 1.00002368 * t
            + 0.37409196 * t.powi(2)
            + 0.09678418 * t.powi(3)
            - 0.18628806 * t.powi(4)
            + 0.27886807 * t.powi(5)
            - 1.13520398 * t.powi(6)
            + 1.48851587 * t.powi(7)
            - 0.82215223 * t.powi(8)
            + 0.17087277 * t.powi(9))
        .exp();

    if x >= 0.0 { tau } else { 2.0 - tau }
}

/// Format voltage with SI prefix
fn format_si_voltage(v: f64) -> String {
    let abs_v = v.abs();
    if abs_v >= 1.0 {
        format!("{:.2} V", v)
    } else if abs_v >= 1e-3 {
        format!("{:.2} mV", v * 1e3)
    } else if abs_v >= 1e-6 {
        format!("{:.2} µV", v * 1e6)
    } else {
        format!("{:.2} nV", v * 1e9)
    }
}

/// Format time with SI prefix
fn format_si_time(t: f64) -> String {
    let abs_t = t.abs();
    if abs_t >= 1.0 {
        format!("{:.2} s", t)
    } else if abs_t >= 1e-3 {
        format!("{:.2} ms", t * 1e3)
    } else if abs_t >= 1e-6 {
        format!("{:.2} µs", t * 1e6)
    } else if abs_t >= 1e-9 {
        format!("{:.2} ns", t * 1e9)
    } else if abs_t >= 1e-12 {
        format!("{:.2} ps", t * 1e12)
    } else {
        format!("{:.2} fs", t * 1e15)
    }
}

// =============================================================================
// Tests
// =============================================================================

