//! Signal Integrity Analysis
//!
//! High-speed digital signal analysis for SerDes, DDR, PCIe, and other
//! high-speed interfaces. Includes eye diagram generation, jitter analysis,
//! and signal quality metrics.
//!
//! # Features
//!
//! - **Eye Diagram**: Statistical eye with density plots
//! - **Jitter Analysis**: TJ, RJ, DJ decomposition
//! - **Bathtub Curves**: BER vs sampling point
//! - **Eye Metrics**: Height, width, opening, mask margins

#![allow(clippy::needless_range_loop)]
#![cfg_attr(test, allow(clippy::field_reassign_with_default))]
use crate::Value;
use std::f64::consts::PI;

//=============================================================================
// Eye Diagram Configuration
//=============================================================================

/// Configuration for eye diagram generation
#[derive(Debug, Clone)]
pub struct EyeConfig {
    /// Data rate (bits per second)
    pub data_rate: Value,
    /// Number of unit intervals (UIs) to display (typically 2)
    pub num_ui: usize,
    /// Horizontal resolution (bins across eye)
    pub h_resolution: usize,
    /// Vertical resolution (bins in amplitude)
    pub v_resolution: usize,
    /// Voltage threshold for logic high (normalized to 0-1)
    pub v_high: Value,
    /// Voltage threshold for logic low (normalized to 0-1)
    pub v_low: Value,
    /// Reference crossing level (typically 0.5 for differential)
    pub crossing_level: Value,
    /// Enable histogram accumulation
    pub enable_histogram: bool,
}

impl Default for EyeConfig {
    fn default() -> Self {
        Self {
            data_rate: 10e9, // 10 Gbps
            num_ui: 2,
            h_resolution: 256,
            v_resolution: 256,
            v_high: 0.8,
            v_low: 0.2,
            crossing_level: 0.5,
            enable_histogram: true,
        }
    }
}

impl EyeConfig {
    /// Create config for specific data rate
    pub fn for_rate(data_rate: Value) -> Self {
        Self {
            data_rate,
            ..Default::default()
        }
    }

    /// Get unit interval duration
    pub fn ui(&self) -> Value {
        1.0 / self.data_rate
    }

    /// Set resolution
    pub fn with_resolution(mut self, h: usize, v: usize) -> Self {
        self.h_resolution = h;
        self.v_resolution = v;
        self
    }
}

//=============================================================================
// Eye Diagram Data
//=============================================================================

/// Statistical eye diagram
#[derive(Debug, Clone)]
pub struct EyeDiagram {
    /// Configuration used
    pub config: EyeConfig,
    /// 2D histogram [time_bin][voltage_bin] -> hit count
    pub histogram: Vec<Vec<usize>>,
    /// Total number of samples
    pub total_samples: usize,
    /// Voltage range (min, max)
    pub voltage_range: (Value, Value),
    /// Number of unit intervals captured
    pub captured_uis: usize,
}

impl EyeDiagram {
    /// Create new empty eye diagram
    pub fn new(config: EyeConfig) -> Self {
        let histogram = vec![vec![0; config.v_resolution]; config.h_resolution];
        Self {
            config,
            histogram,
            total_samples: 0,
            voltage_range: (0.0, 1.0),
            captured_uis: 0,
        }
    }

    /// Add a waveform to the eye
    pub fn add_waveform(&mut self, time: &[Value], voltage: &[Value]) {
        if time.len() != voltage.len() || time.len() < 2 {
            return;
        }

        // Find voltage range from data
        let v_min = voltage.iter().cloned().fold(f64::INFINITY, f64::min);
        let v_max = voltage.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        // Update voltage range (expanding)
        self.voltage_range.0 = self.voltage_range.0.min(v_min);
        self.voltage_range.1 = self.voltage_range.1.max(v_max);

        let ui = self.config.ui();

        // Find clock recovery - detect crossings
        let crossings = self.find_crossings(time, voltage);

        if crossings.is_empty() {
            return;
        }

        // Fold waveform using recovered clock
        for &crossing_time in &crossings {
            self.add_segment(time, voltage, crossing_time, ui);
        }

        self.captured_uis += crossings.len();
    }

    /// Find zero crossings in waveform
    fn find_crossings(&self, time: &[Value], voltage: &[Value]) -> Vec<Value> {
        let mut crossings = Vec::new();
        let v_range = self.voltage_range.1 - self.voltage_range.0;
        let threshold = self.voltage_range.0 + v_range * self.config.crossing_level;

        for i in 1..voltage.len() {
            let v_prev = voltage[i - 1];
            let v_curr = voltage[i];

            // Rising edge crossing
            if v_prev < threshold && v_curr >= threshold {
                // Linear interpolation for precise crossing time
                let alpha = (threshold - v_prev) / (v_curr - v_prev);
                let t_cross = time[i - 1] + alpha * (time[i] - time[i - 1]);
                crossings.push(t_cross);
            }
        }

        crossings
    }

    /// Add a segment centered at crossing
    fn add_segment(&mut self, time: &[Value], voltage: &[Value], crossing_time: Value, ui: Value) {
        let total_ui_width = ui * self.config.num_ui as Value;
        let t_start = crossing_time - total_ui_width / 2.0;

        let v_range = self.voltage_range.1 - self.voltage_range.0;
        if v_range < 1e-12 {
            return;
        }

        // Sample waveform at histogram resolution
        for h_bin in 0..self.config.h_resolution {
            let t = t_start
                + (h_bin as Value + 0.5) * total_ui_width / self.config.h_resolution as Value;

            // Interpolate voltage at this time
            if let Some(v) = Self::interpolate_voltage(time, voltage, t) {
                // Normalize voltage to 0-1
                let v_norm = (v - self.voltage_range.0) / v_range;
                let v_clamped = v_norm.clamp(0.0, 0.9999);

                let v_bin = (v_clamped * self.config.v_resolution as Value) as usize;
                let v_bin = v_bin.min(self.config.v_resolution - 1);

                self.histogram[h_bin][v_bin] += 1;
                self.total_samples += 1;
            }
        }
    }

    /// Interpolate voltage at time t
    fn interpolate_voltage(time: &[Value], voltage: &[Value], t: Value) -> Option<Value> {
        if t < time[0] || t > time[time.len() - 1] {
            return None;
        }

        for i in 1..time.len() {
            if time[i - 1] <= t && t <= time[i] {
                let alpha = (t - time[i - 1]) / (time[i] - time[i - 1]);
                return Some(voltage[i - 1] + alpha * (voltage[i] - voltage[i - 1]));
            }
        }

        None
    }

    /// Get metrics from the eye
    pub fn metrics(&self) -> EyeMetrics {
        let mut metrics = EyeMetrics::default();

        if self.total_samples == 0 {
            return metrics;
        }

        // Find eye opening - largest clear rectangle in center
        let center_h = self.config.h_resolution / 2;
        let center_v = self.config.v_resolution / 2;

        // Find vertical opening at center
        let mut v_low = center_v;
        let mut v_high = center_v;

        for v in (0..center_v).rev() {
            if self.histogram[center_h][v] == 0 {
                v_low = v;
            } else {
                break;
            }
        }

        for v in center_v..self.config.v_resolution {
            if self.histogram[center_h][v] == 0 {
                v_high = v;
            } else {
                break;
            }
        }

        // Eye height in normalized units
        let v_range = self.voltage_range.1 - self.voltage_range.0;
        metrics.eye_height = (v_high - v_low) as Value / self.config.v_resolution as Value;
        metrics.eye_height_voltage = metrics.eye_height * v_range;

        // Find horizontal opening at center voltage
        let mut h_left = center_h;
        let mut h_right = center_h;

        for h in (0..center_h).rev() {
            if self.histogram[h][center_v] == 0 {
                h_left = h;
            } else {
                break;
            }
        }

        for h in center_h..self.config.h_resolution {
            if self.histogram[h][center_v] == 0 {
                h_right = h;
            } else {
                break;
            }
        }

        // Eye width in UI
        let ui_per_bin = self.config.num_ui as Value / self.config.h_resolution as Value;
        metrics.eye_width_ui = (h_right - h_left) as Value * ui_per_bin;
        metrics.eye_width_time = metrics.eye_width_ui * self.config.ui();

        // Eye opening area (normalized)
        metrics.eye_area = metrics.eye_height * metrics.eye_width_ui;

        // Estimate jitter from crossing histogram
        metrics.crossing_histogram = self.get_crossing_histogram();

        metrics
    }

    /// Get histogram of crossing times
    fn get_crossing_histogram(&self) -> Vec<usize> {
        let mut crossing_hist = vec![0; self.config.h_resolution];
        let mid_v = self.config.v_resolution / 2;
        let tolerance = self.config.v_resolution / 20; // 5% window

        for h in 0..self.config.h_resolution {
            for v in mid_v.saturating_sub(tolerance)
                ..=(mid_v + tolerance).min(self.config.v_resolution - 1)
            {
                crossing_hist[h] += self.histogram[h][v];
            }
        }

        crossing_hist
    }

    /// Get density at a point (normalized 0-1)
    pub fn density_at(&self, h_frac: Value, v_frac: Value) -> Value {
        if self.total_samples == 0 {
            return 0.0;
        }

        let h_bin = (h_frac * self.config.h_resolution as Value) as usize;
        let v_bin = (v_frac * self.config.v_resolution as Value) as usize;

        let h_bin = h_bin.min(self.config.h_resolution - 1);
        let v_bin = v_bin.min(self.config.v_resolution - 1);

        self.histogram[h_bin][v_bin] as Value / self.total_samples as Value
    }
}

//=============================================================================
// Eye Metrics
//=============================================================================

/// Metrics extracted from eye diagram
#[derive(Debug, Clone, Default)]
pub struct EyeMetrics {
    /// Eye height (normalized 0-1)
    pub eye_height: Value,
    /// Eye height in volts
    pub eye_height_voltage: Value,
    /// Eye width in UI
    pub eye_width_ui: Value,
    /// Eye width in seconds
    pub eye_width_time: Value,
    /// Eye area (height Ã— width, normalized)
    pub eye_area: Value,
    /// Crossing point histogram (for jitter estimation)
    pub crossing_histogram: Vec<usize>,
}

impl EyeMetrics {
    /// Check if eye meets specification
    pub fn meets_spec(&self, min_height: Value, min_width_ui: Value) -> bool {
        self.eye_height >= min_height && self.eye_width_ui >= min_width_ui
    }
}

//=============================================================================
// Jitter Analysis
//=============================================================================

/// Jitter decomposition results
#[derive(Debug, Clone, Default)]
pub struct JitterAnalysis {
    /// Total jitter at specified BER (usually 1e-12)
    pub tj: Value,
    /// Random jitter (1-sigma)
    pub rj_rms: Value,
    /// Deterministic jitter (peak-to-peak)
    pub dj_pp: Value,
    /// Periodic jitter (peak-to-peak)
    pub pj_pp: Value,
    /// Data-dependent jitter (peak-to-peak)
    pub ddj_pp: Value,
    /// Duty cycle distortion
    pub dcd: Value,
    /// BER used for TJ calculation
    pub target_ber: Value,
}

impl JitterAnalysis {
    /// Calculate from crossing times
    pub fn from_crossings(crossings: &[Value], ui: Value, target_ber: Value) -> Self {
        if crossings.len() < 10 {
            return Self::default();
        }

        // Calculate timing errors (deviation from ideal)
        let mut tie: Vec<Value> = Vec::with_capacity(crossings.len());
        for (i, &t) in crossings.iter().enumerate() {
            let ideal_t = crossings[0] + (i as Value) * ui;
            tie.push(t - ideal_t);
        }

        // Compute statistics
        let mean = tie.iter().sum::<Value>() / tie.len() as Value;
        let variance = tie.iter().map(|&t| (t - mean).powi(2)).sum::<Value>() / tie.len() as Value;
        let std_dev = variance.sqrt();

        // RJ is Gaussian - use standard deviation
        let rj_rms = std_dev;

        // DJ = TIE range - RJ contribution
        let tie_min = tie.iter().cloned().fold(f64::INFINITY, f64::min);
        let tie_max = tie.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let tie_range = tie_max - tie_min;

        // Estimate DJ from tail behavior
        // DJ-pp â‰ˆ TIE range - 6Ã—RJ (removes 99.7% Gaussian contribution)
        let dj_pp = (tie_range - 6.0 * rj_rms).max(0.0);

        // Total jitter at target BER
        // TJ = DJ + 2 Ã— Q Ã— RJ, where Q corresponds to BER
        let q_factor = Self::ber_to_q(target_ber);
        let tj = dj_pp + 2.0 * q_factor * rj_rms;

        // Estimate periodic jitter from FFT of TIE
        let pj_pp = Self::estimate_periodic_jitter(&tie, ui);

        // DDJ from pattern analysis (simplified)
        let ddj_pp = Self::estimate_ddj(&tie);

        // DCD from alternating edge analysis
        let dcd = Self::estimate_dcd(&tie);

        Self {
            tj,
            rj_rms,
            dj_pp,
            pj_pp,
            ddj_pp,
            dcd,
            target_ber,
        }
    }

    /// Convert BER to Q-factor
    fn ber_to_q(ber: Value) -> Value {
        // Q â‰ˆ sqrt(2) Ã— erfc_inv(2Ã—BER)
        // Approximation for common BER values
        if ber <= 1e-15 {
            8.2 // Q for BER=1e-16
        } else if ber <= 1e-12 {
            7.0 // Q for BER=1e-12
        } else if ber <= 1e-9 {
            6.0 // Q for BER=1e-9
        } else if ber <= 1e-6 {
            4.75 // Q for BER=1e-6
        } else {
            3.7 // Q for BER=1e-4
        }
    }

    /// Estimate periodic jitter using basic spectral analysis
    fn estimate_periodic_jitter(tie: &[Value], _ui: Value) -> Value {
        if tie.len() < 32 {
            return 0.0;
        }

        // Simple DFT magnitude at fundamental frequency
        let n = tie.len();
        let mut max_mag = 0.0_f64;

        for k in 1..n / 4 {
            let mut re = 0.0;
            let mut im = 0.0;

            for (i, &t) in tie.iter().enumerate() {
                let angle = 2.0 * PI * k as Value * i as Value / n as Value;
                re += t * angle.cos();
                im += t * angle.sin();
            }

            let mag = (re * re + im * im).sqrt() / n as Value;
            max_mag = max_mag.max(mag);
        }

        // PJ-pp â‰ˆ 2 Ã— peak magnitude
        2.0 * max_mag
    }

    /// Estimate data-dependent jitter
    fn estimate_ddj(tie: &[Value]) -> Value {
        if tie.len() < 10 {
            return 0.0;
        }

        // DDJ shows up as bimodal distribution
        // Simple estimate: difference between odd/even edge means
        let odd: Vec<_> = tie.iter().step_by(2).copied().collect();
        let even: Vec<_> = tie.iter().skip(1).step_by(2).copied().collect();

        if odd.is_empty() || even.is_empty() {
            return 0.0;
        }

        let odd_mean = odd.iter().sum::<Value>() / odd.len() as Value;
        let even_mean = even.iter().sum::<Value>() / even.len() as Value;

        (odd_mean - even_mean).abs()
    }

    /// Estimate duty cycle distortion
    fn estimate_dcd(tie: &[Value]) -> Value {
        Self::estimate_ddj(tie) / 2.0
    }

    /// Convert to UI (normalized)
    pub fn tj_ui(&self, ui: Value) -> Value {
        self.tj / ui
    }

    /// Check if jitter is within specification
    pub fn meets_spec(&self, max_tj_ui: Value, ui: Value) -> bool {
        self.tj_ui(ui) <= max_tj_ui
    }
}

//=============================================================================
// Bathtub Curve
//=============================================================================

/// Bathtub curve for BER vs sampling point
#[derive(Debug, Clone)]
pub struct BathtubCurve {
    /// Phase offset (0 to 1 UI)
    pub phase: Vec<Value>,
    /// Log10(BER) at each phase
    pub log_ber: Vec<Value>,
    /// Eye opening at target BER
    pub eye_opening_ui: Value,
    /// Optimal sampling phase
    pub optimal_phase: Value,
}

impl BathtubCurve {
    /// Generate from jitter analysis
    pub fn from_jitter(jitter: &JitterAnalysis, ui: Value, num_points: usize) -> Self {
        let mut phase = Vec::with_capacity(num_points);
        let mut log_ber = Vec::with_capacity(num_points);

        let rj = jitter.rj_rms;
        let dj = jitter.dj_pp;

        for i in 0..num_points {
            let p = i as Value / (num_points - 1) as Value;
            phase.push(p);

            // Distance from edge (minimum is better)
            let dist_from_edge = (0.5 - (p - 0.5).abs()) * ui - dj / 2.0;

            if dist_from_edge <= 0.0 || rj <= 0.0 {
                log_ber.push(-1.0); // BER = 0.1 (very bad)
            } else {
                // Q = distance / RJ
                let q = dist_from_edge / rj;
                // BER â‰ˆ erfc(Q/sqrt(2))/2 â‰ˆ exp(-QÂ²/2) for large Q
                let log_ber_val = -0.5 * q * q / std::f64::consts::LN_10;
                log_ber.push(log_ber_val.max(-20.0)); // Cap at BER=1e-20
            }
        }

        // Find eye opening at BER=1e-12 (log_ber = -12)
        let target_log_ber = -12.0;
        let mut left_edge: Option<Value> = None;
        let mut right_edge: Option<Value> = None;

        // Find leftmost point meeting BER threshold (scanning from left)
        for i in 0..num_points / 2 {
            if log_ber[i] < target_log_ber {
                left_edge = Some(phase[i]);
                break;
            }
        }

        // Find rightmost point meeting BER threshold (scanning from right)
        for i in (num_points / 2..num_points).rev() {
            if log_ber[i] < target_log_ber {
                right_edge = Some(phase[i]);
                break;
            }
        }

        // If either edge wasn't found, eye is closed
        let (eye_opening_ui, optimal_phase) = match (left_edge, right_edge) {
            (Some(left), Some(right)) if right > left => (right - left, (left + right) / 2.0),
            _ => (0.0, 0.5), // Eye is closed - no valid opening found
        };

        Self {
            phase,
            log_ber,
            eye_opening_ui,
            optimal_phase,
        }
    }

    /// Get BER at specific phase
    pub fn ber_at(&self, p: Value) -> Value {
        if self.phase.is_empty() {
            return 1.0;
        }

        // Find nearest phase point
        let mut idx = 0;
        let mut min_dist = f64::INFINITY;

        for (i, &ph) in self.phase.iter().enumerate() {
            let dist = (ph - p).abs();
            if dist < min_dist {
                min_dist = dist;
                idx = i;
            }
        }

        10.0_f64.powf(self.log_ber[idx])
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eye_config_default() {
        let config = EyeConfig::default();
        assert!(config.data_rate > 0.0);
        assert!(config.ui() > 0.0);
    }

    #[test]
    fn test_eye_config_for_rate() {
        let config = EyeConfig::for_rate(28e9);
        assert_eq!(config.data_rate, 28e9);
    }

    #[test]
    fn test_eye_config_ui() {
        let config = EyeConfig::for_rate(10e9);
        assert!((config.ui() - 100e-12).abs() < 1e-15);
    }

    #[test]
    fn test_eye_diagram_creation() {
        let config = EyeConfig::default();
        let eye = EyeDiagram::new(config);

        assert_eq!(eye.total_samples, 0);
        assert_eq!(eye.histogram.len(), 256);
    }

    #[test]
    fn test_eye_diagram_add_waveform() {
        let config = EyeConfig::for_rate(1e9); // 1 Gbps for easy testing
        let mut eye = EyeDiagram::new(config);

        // Create a simple square wave
        let ui = 1e-9;
        let time: Vec<Value> = (0..1000).map(|i| i as Value * 10e-12).collect();
        let voltage: Vec<Value> = time
            .iter()
            .map(|&t| {
                let bit = ((t / ui) as usize) % 2;
                if bit == 0 { 0.1 } else { 0.9 }
            })
            .collect();

        eye.add_waveform(&time, &voltage);
        assert!(eye.total_samples > 0);
    }

    #[test]
    fn test_eye_diagram_metrics() {
        let config = EyeConfig::for_rate(1e9).with_resolution(64, 64);
        let mut eye = EyeDiagram::new(config);

        // Create ideal eye pattern
        let ui = 1e-9;
        let time: Vec<Value> = (0..10000).map(|i| i as Value * 1e-12).collect();
        let voltage: Vec<Value> = time
            .iter()
            .map(|&t| {
                let phase = (t / ui).fract();
                // Smooth transitions at 0 and 0.5
                if phase < 0.1 || (phase > 0.4 && phase < 0.6) || phase > 0.9 {
                    0.5 // Transition region
                } else if phase < 0.4 {
                    0.9 // High
                } else {
                    0.1 // Low
                }
            })
            .collect();

        eye.add_waveform(&time, &voltage);

        let metrics = eye.metrics();
        assert!(metrics.eye_height >= 0.0);
        assert!(metrics.eye_width_ui >= 0.0);
    }

    #[test]
    fn test_eye_metrics_spec_check() {
        let mut metrics = EyeMetrics::default();
        metrics.eye_height = 0.5;
        metrics.eye_width_ui = 0.6;

        assert!(metrics.meets_spec(0.4, 0.5));
        assert!(!metrics.meets_spec(0.6, 0.5));
    }

    #[test]
    fn test_jitter_analysis_empty() {
        let jitter = JitterAnalysis::from_crossings(&[], 1e-9, 1e-12);
        assert_eq!(jitter.tj, 0.0);
    }

    #[test]
    fn test_jitter_analysis_basic() {
        let ui = 1e-9;

        // Create crossing times with some jitter
        let mut crossings: Vec<Value> = (0..100).map(|i| i as Value * ui).collect();

        // Add random-ish jitter
        for (i, t) in crossings.iter_mut().enumerate() {
            *t += (i as Value * 0.1).sin() * 10e-12; // 10ps periodic jitter
        }

        let jitter = JitterAnalysis::from_crossings(&crossings, ui, 1e-12);

        assert!(jitter.rj_rms >= 0.0);
        assert!(jitter.tj >= 0.0);
    }

    #[test]
    fn test_jitter_meets_spec() {
        let mut jitter = JitterAnalysis::default();
        jitter.tj = 30e-12;

        let ui = 100e-12;
        assert!(jitter.meets_spec(0.35, ui)); // 35% UI
        assert!(!jitter.meets_spec(0.25, ui)); // 25% UI
    }

    #[test]
    fn test_jitter_tj_ui() {
        let mut jitter = JitterAnalysis::default();
        jitter.tj = 30e-12;

        let ui = 100e-12;
        assert!((jitter.tj_ui(ui) - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_bathtub_curve_from_jitter() {
        let mut jitter = JitterAnalysis::default();
        jitter.rj_rms = 5e-12;
        jitter.dj_pp = 10e-12;

        let ui = 100e-12;
        let bathtub = BathtubCurve::from_jitter(&jitter, ui, 101);

        assert_eq!(bathtub.phase.len(), 101);
        assert_eq!(bathtub.log_ber.len(), 101);
        assert!(bathtub.optimal_phase > 0.0);
        assert!(bathtub.optimal_phase < 1.0);
    }

    #[test]
    fn test_bathtub_curve_symmetry() {
        let mut jitter = JitterAnalysis::default();
        jitter.rj_rms = 5e-12;
        jitter.dj_pp = 10e-12;

        let ui = 100e-12;
        let bathtub = BathtubCurve::from_jitter(&jitter, ui, 101);

        // Should be roughly symmetric around 0.5
        let ber_left = bathtub.ber_at(0.25);
        let ber_right = bathtub.ber_at(0.75);

        // Both should be similar (within order of magnitude)
        let ratio = ber_left / ber_right;
        assert!(ratio > 0.01 && ratio < 100.0);
    }

    #[test]
    fn test_bathtub_curve_optimal_in_center() {
        let mut jitter = JitterAnalysis::default();
        jitter.rj_rms = 5e-12;
        jitter.dj_pp = 10e-12;

        let ui = 100e-12;
        let bathtub = BathtubCurve::from_jitter(&jitter, ui, 101);

        // Optimal phase should be near 0.5
        assert!((bathtub.optimal_phase - 0.5).abs() < 0.1);
    }

    #[test]
    fn test_bathtub_eye_opening() {
        let mut jitter = JitterAnalysis::default();
        jitter.rj_rms = 5e-12;
        jitter.dj_pp = 10e-12;

        let ui = 100e-12;
        let bathtub = BathtubCurve::from_jitter(&jitter, ui, 101);

        // Eye opening should be positive and less than 1 UI
        assert!(bathtub.eye_opening_ui > 0.0);
        assert!(bathtub.eye_opening_ui < 1.0);
    }

    #[test]
    fn test_ber_to_q() {
        // Basic sanity check
        let q_1e12 = JitterAnalysis::ber_to_q(1e-12);
        let q_1e9 = JitterAnalysis::ber_to_q(1e-9);

        // Higher BER means lower Q
        assert!(q_1e12 > q_1e9);
    }

    #[test]
    fn test_eye_density() {
        let config = EyeConfig::for_rate(1e9).with_resolution(64, 64);
        let mut eye = EyeDiagram::new(config);

        // Add some data
        let time: Vec<Value> = (0..1000).map(|i| i as Value * 1e-12).collect();
        let voltage: Vec<Value> = time.iter().map(|&t| (t * 1e12).sin() * 0.4 + 0.5).collect();

        eye.add_waveform(&time, &voltage);

        // Density should be normalized
        let density = eye.density_at(0.5, 0.5);
        assert!(density >= 0.0);
        assert!(density <= 1.0);
    }

    // =========================================================================
    // Additional Exhaustive Tests for Commercial-Grade Coverage
    // =========================================================================

    #[test]
    fn test_eye_config_with_resolution() {
        let config = EyeConfig::default().with_resolution(512, 128);
        assert_eq!(config.h_resolution, 512);
        assert_eq!(config.v_resolution, 128);
    }

    #[test]
    fn test_eye_config_various_data_rates() {
        // Test common SerDes rates
        let rates = [1e9, 2.5e9, 5e9, 10e9, 25e9, 28e9, 56e9, 112e9];
        for rate in rates {
            let config = EyeConfig::for_rate(rate);
            let ui = config.ui();
            assert!((ui - 1.0 / rate).abs() < 1e-20);
        }
    }

    #[test]
    fn test_eye_diagram_empty_waveform() {
        let config = EyeConfig::default();
        let mut eye = EyeDiagram::new(config);

        // Empty input should be handled gracefully
        eye.add_waveform(&[], &[]);
        assert_eq!(eye.total_samples, 0);

        // Metrics should return defaults
        let metrics = eye.metrics();
        assert_eq!(metrics.eye_height, 0.0);
    }

    #[test]
    fn test_eye_diagram_single_point() {
        let config = EyeConfig::default();
        let mut eye = EyeDiagram::new(config);

        // Single point should be handled
        eye.add_waveform(&[0.0], &[0.5]);
        assert_eq!(eye.total_samples, 0); // Not enough for crossings
    }

    #[test]
    fn test_eye_diagram_no_crossings() {
        let config = EyeConfig::for_rate(1e9);
        let mut eye = EyeDiagram::new(config);

        // DC signal with no crossings
        let time: Vec<Value> = (0..100).map(|i| i as Value * 1e-12).collect();
        let voltage: Vec<Value> = vec![0.8; 100]; // Constant high

        eye.add_waveform(&time, &voltage);
        assert_eq!(eye.captured_uis, 0); // No crossings found
    }

    #[test]
    fn test_eye_diagram_interpolation_accuracy() {
        // Test that voltage interpolation is mathematically correct
        let time = vec![0.0, 1.0, 2.0, 3.0];
        let voltage = vec![0.0, 1.0, 2.0, 3.0];

        // Interpolate at midpoints
        let v_05 = EyeDiagram::interpolate_voltage(&time, &voltage, 0.5).unwrap();
        let v_15 = EyeDiagram::interpolate_voltage(&time, &voltage, 1.5).unwrap();
        let v_25 = EyeDiagram::interpolate_voltage(&time, &voltage, 2.5).unwrap();

        assert!((v_05 - 0.5).abs() < 1e-10);
        assert!((v_15 - 1.5).abs() < 1e-10);
        assert!((v_25 - 2.5).abs() < 1e-10);
    }

    #[test]
    fn test_eye_diagram_interpolation_out_of_range() {
        let time = vec![1.0, 2.0, 3.0];
        let voltage = vec![1.0, 2.0, 3.0];

        // Before first point
        assert!(EyeDiagram::interpolate_voltage(&time, &voltage, 0.5).is_none());
        // After last point
        assert!(EyeDiagram::interpolate_voltage(&time, &voltage, 3.5).is_none());
    }

    #[test]
    fn test_eye_metrics_default() {
        let metrics = EyeMetrics::default();
        assert_eq!(metrics.eye_height, 0.0);
        assert_eq!(metrics.eye_width_ui, 0.0);
        assert_eq!(metrics.eye_area, 0.0);
        assert!(metrics.crossing_histogram.is_empty());
    }

    #[test]
    fn test_eye_metrics_spec_edge_cases() {
        let mut metrics = EyeMetrics::default();

        // Exactly at spec
        metrics.eye_height = 0.5;
        metrics.eye_width_ui = 0.6;
        assert!(metrics.meets_spec(0.5, 0.6)); // Equal should pass

        // Just below spec
        metrics.eye_height = 0.499;
        assert!(!metrics.meets_spec(0.5, 0.6));
    }

    #[test]
    fn test_jitter_analysis_few_crossings() {
        // Less than 10 crossings should return default
        let crossings: Vec<Value> = (0..5).map(|i| i as Value * 1e-9).collect();
        let jitter = JitterAnalysis::from_crossings(&crossings, 1e-9, 1e-12);
        assert_eq!(jitter.tj, 0.0);
    }

    #[test]
    fn test_jitter_analysis_perfect_clock() {
        let ui = 1e-9;
        // Perfect clock with no jitter
        let crossings: Vec<Value> = (0..100).map(|i| i as Value * ui).collect();

        let jitter = JitterAnalysis::from_crossings(&crossings, ui, 1e-12);

        // RJ should be near zero (just numerical noise)
        assert!(jitter.rj_rms < 1e-15);
    }

    #[test]
    fn test_jitter_analysis_large_rj() {
        let ui = 1e-9;
        // Add significant random jitter
        let crossings: Vec<Value> = (0..100)
            .map(|i| {
                let base = i as Value * ui;
                // Pseudo-random jitter using sin
                base + (i as Value * 0.7).sin() * 50e-12
            })
            .collect();

        let jitter = JitterAnalysis::from_crossings(&crossings, ui, 1e-12);

        // Should detect significant RJ
        assert!(jitter.rj_rms > 10e-12);
        assert!(jitter.tj > jitter.rj_rms); // TJ should include RJ
    }

    #[test]
    fn test_jitter_analysis_periodic_jitter() {
        let ui = 1e-9;
        // Add strong periodic jitter
        let crossings: Vec<Value> = (0..200)
            .map(|i| {
                let base = i as Value * ui;
                // Periodic jitter at 1/20 of data rate
                base + (2.0 * PI * i as Value / 20.0).sin() * 30e-12
            })
            .collect();

        let jitter = JitterAnalysis::from_crossings(&crossings, ui, 1e-12);

        // Should detect periodic jitter
        assert!(jitter.pj_pp > 0.0);
    }

    #[test]
    fn test_jitter_ber_to_q_full_range() {
        // Test all BER ranges
        assert!(JitterAnalysis::ber_to_q(1e-16) > 8.0);
        assert!(JitterAnalysis::ber_to_q(1e-12) >= 7.0);
        assert!(JitterAnalysis::ber_to_q(1e-9) >= 6.0);
        assert!(JitterAnalysis::ber_to_q(1e-6) >= 4.5);
        assert!(JitterAnalysis::ber_to_q(1e-3) >= 3.0);

        // Q should decrease with increasing BER
        let bers = [1e-15, 1e-12, 1e-9, 1e-6, 1e-3];
        for i in 1..bers.len() {
            let q_lower = JitterAnalysis::ber_to_q(bers[i]);
            let q_higher = JitterAnalysis::ber_to_q(bers[i - 1]);
            assert!(q_higher >= q_lower);
        }
    }

    #[test]
    fn test_jitter_default() {
        let jitter = JitterAnalysis::default();
        assert_eq!(jitter.tj, 0.0);
        assert_eq!(jitter.rj_rms, 0.0);
        assert_eq!(jitter.dj_pp, 0.0);
        assert_eq!(jitter.pj_pp, 0.0);
        assert_eq!(jitter.ddj_pp, 0.0);
        assert_eq!(jitter.dcd, 0.0);
    }

    #[test]
    fn test_bathtub_curve_edge_ber() {
        let mut jitter = JitterAnalysis::default();
        jitter.rj_rms = 5e-12;
        jitter.dj_pp = 10e-12;

        let ui = 100e-12;
        let bathtub = BathtubCurve::from_jitter(&jitter, ui, 101);

        // BER at edges should be worse (higher) than center
        let ber_edge = bathtub.ber_at(0.0);
        let ber_center = bathtub.ber_at(0.5);
        assert!(ber_edge > ber_center);
    }

    #[test]
    fn test_bathtub_curve_high_jitter() {
        let mut jitter = JitterAnalysis::default();
        // High jitter that closes the eye
        jitter.rj_rms = 20e-12;
        jitter.dj_pp = 50e-12;

        let ui = 100e-12;
        let bathtub = BathtubCurve::from_jitter(&jitter, ui, 101);

        // Eye opening should be very small or zero
        assert!(bathtub.eye_opening_ui < 0.5);
    }

    #[test]
    fn test_bathtub_curve_low_jitter() {
        let mut jitter = JitterAnalysis::default();
        // Very low jitter - wide open eye
        jitter.rj_rms = 1e-12;
        jitter.dj_pp = 2e-12;

        let ui = 100e-12;
        let bathtub = BathtubCurve::from_jitter(&jitter, ui, 101);

        // Eye opening should be close to 1 UI
        assert!(bathtub.eye_opening_ui > 0.8);
    }

    #[test]
    fn test_bathtub_ber_at_empty() {
        // Empty bathtub should return 1.0 (worst BER)
        let bathtub = BathtubCurve {
            phase: vec![],
            log_ber: vec![],
            eye_opening_ui: 0.0,
            optimal_phase: 0.5,
        };
        assert_eq!(bathtub.ber_at(0.5), 1.0);
    }

    #[test]
    fn test_eye_multiple_waveforms() {
        let config = EyeConfig::for_rate(1e9).with_resolution(64, 64);
        let mut eye = EyeDiagram::new(config);

        let ui = 1e-9;

        // Add multiple waveform segments
        for offset in 0..5 {
            let t_offset = offset as Value * 100e-9;
            let time: Vec<Value> = (0..1000).map(|i| t_offset + i as Value * 10e-12).collect();
            let voltage: Vec<Value> = time
                .iter()
                .map(|&t| {
                    let bit = ((t / ui) as usize) % 2;
                    if bit == 0 { 0.1 } else { 0.9 }
                })
                .collect();
            eye.add_waveform(&time, &voltage);
        }

        // Should have accumulated samples from all waveforms
        assert!(eye.total_samples > 1000);
    }

    #[test]
    fn test_eye_voltage_range_expansion() {
        let config = EyeConfig::for_rate(1e9);
        let mut eye = EyeDiagram::new(config);

        // First waveform: 0.2 to 0.8
        let time1: Vec<Value> = (0..100).map(|i| i as Value * 1e-12).collect();
        let voltage1: Vec<Value> = time1.iter().map(|_| 0.5).collect();
        eye.add_waveform(&time1, &voltage1);

        let range1 = eye.voltage_range;

        // Second waveform: 0.0 to 1.0 (wider range)
        let time2: Vec<Value> = (0..100).map(|i| (100 + i) as Value * 1e-12).collect();
        let voltage2: Vec<Value> = time2
            .iter()
            .enumerate()
            .map(|(i, _)| if i % 2 == 0 { 0.0 } else { 1.0 })
            .collect();
        eye.add_waveform(&time2, &voltage2);

        // Range should expand
        assert!(eye.voltage_range.0 <= range1.0);
        assert!(eye.voltage_range.1 >= range1.1);
    }

    #[test]
    fn test_jitter_ddj_estimation() {
        let ui = 1e-9;

        // Create pattern with clear DDJ (alternating bias)
        let crossings: Vec<Value> = (0..100)
            .map(|i| {
                let base = i as Value * ui;
                // Odd edges early, even edges late
                if i % 2 == 0 {
                    base + 10e-12
                } else {
                    base - 10e-12
                }
            })
            .collect();

        let jitter = JitterAnalysis::from_crossings(&crossings, ui, 1e-12);

        // DDJ should be detected
        assert!(jitter.ddj_pp > 5e-12);
    }

    #[test]
    fn test_crossing_histogram_generation() {
        let config = EyeConfig::for_rate(1e9).with_resolution(64, 64);
        let mut eye = EyeDiagram::new(config);

        let ui = 1e-9;
        let time: Vec<Value> = (0..10000).map(|i| i as Value * 1e-12).collect();
        let voltage: Vec<Value> = time
            .iter()
            .map(|&t| {
                let phase = (t / ui).fract();
                if phase < 0.1 {
                    0.5 + phase * 4.0 // Rising
                } else if phase < 0.5 {
                    0.9
                } else if phase < 0.6 {
                    0.9 - (phase - 0.5) * 8.0 // Falling
                } else {
                    0.1
                }
            })
            .collect();

        eye.add_waveform(&time, &voltage);

        let metrics = eye.metrics();
        // Crossing histogram should have been generated
        assert_eq!(metrics.crossing_histogram.len(), 64);
    }
}
