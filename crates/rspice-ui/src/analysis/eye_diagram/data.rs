//! Eye Diagram Data Structures
//!
//! Core data types for eye diagram construction and analysis.
//! Supports alignment of transient waveforms to bit period for overlay display.

#![allow(clippy::too_many_arguments)]

// =============================================================================
// Eye Trace
// =============================================================================

/// A single trace segment for eye diagram overlay
#[derive(Debug, Clone, PartialEq)]
pub struct EyeTrace {
    /// Normalized time values (0.0 to ui_count as fraction of bit period)
    pub time: Vec<f64>,
    /// Signal amplitude values
    pub amplitude: Vec<f64>,
}

impl EyeTrace {
    /// Create new trace
    pub fn new(time: Vec<f64>, amplitude: Vec<f64>) -> Self {
        Self { time, amplitude }
    }

    /// Number of points in trace
    pub fn len(&self) -> usize {
        self.time.len().min(self.amplitude.len())
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get min/max amplitude
    pub fn amplitude_range(&self) -> Option<(f64, f64)> {
        if self.amplitude.is_empty() {
            return None;
        }
        let mut min = f64::MAX;
        let mut max = f64::MIN;
        for &v in &self.amplitude {
            if v.is_finite() {
                min = min.min(v);
                max = max.max(v);
            }
        }
        if min <= max { Some((min, max)) } else { None }
    }
}

// =============================================================================
// Eye Data
// =============================================================================

/// Complete eye diagram data
#[derive(Debug, Clone)]
pub struct EyeData {
    /// All overlaid traces
    pub traces: Vec<EyeTrace>,
    /// Bit period in seconds
    pub bit_period: f64,
    /// Number of unit intervals displayed
    pub ui_count: u32,
    /// Data rate in bits per second
    pub data_rate: f64,
    /// Signal swing (peak-to-peak)
    pub swing: f64,
    /// High level (VOH)
    pub v_high: f64,
    /// Low level (VOL)
    pub v_low: f64,
    /// Crossover voltage (typically mid-swing)
    pub v_cross: f64,
}

impl Default for EyeData {
    fn default() -> Self {
        Self {
            traces: Vec::new(),
            bit_period: 1e-9, // 1ns default
            ui_count: 2,
            data_rate: 1e9, // 1 Gbps default
            swing: 0.8,     // 800mV default
            v_high: 0.4,
            v_low: -0.4,
            v_cross: 0.0,
        }
    }
}

impl EyeData {
    /// Create new empty eye data with specified parameters
    pub fn new(bit_period: f64, ui_count: u32) -> Self {
        Self {
            bit_period,
            ui_count,
            data_rate: 1.0 / bit_period,
            ..Default::default()
        }
    }

    /// Number of traces
    pub fn trace_count(&self) -> usize {
        self.traces.len()
    }

    /// Total number of points across all traces
    pub fn total_points(&self) -> usize {
        self.traces.iter().map(|t| t.len()).sum()
    }

    /// Add a trace
    pub fn add_trace(&mut self, trace: EyeTrace) {
        self.traces.push(trace);
    }

    /// Clear all traces
    pub fn clear(&mut self) {
        self.traces.clear();
    }

    /// Calculate amplitude statistics across all traces
    pub fn amplitude_stats(&self) -> Option<AmplitudeStats> {
        if self.traces.is_empty() {
            return None;
        }

        let mut all_values = Vec::new();
        for trace in &self.traces {
            all_values.extend(trace.amplitude.iter().copied().filter(|v| v.is_finite()));
        }

        if all_values.is_empty() {
            return None;
        }

        let n = all_values.len() as f64;
        let mean = all_values.iter().sum::<f64>() / n;
        let variance = all_values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;

        all_values.sort_by(|a, b| a.total_cmp(b));

        Some(AmplitudeStats {
            min: all_values.first().copied().unwrap_or(0.0),
            max: all_values.last().copied().unwrap_or(0.0),
            mean,
            std_dev: variance.sqrt(),
            count: all_values.len(),
        })
    }
}

/// Amplitude statistics
#[derive(Debug, Clone, Default)]
pub struct AmplitudeStats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub std_dev: f64,
    pub count: usize,
}

// =============================================================================
// Eye Diagram Builder
// =============================================================================

/// Builder for constructing eye diagrams from transient data
#[derive(Debug, Clone)]
pub struct EyeDataBuilder {
    /// Bit period in seconds
    bit_period: f64,
    /// Number of unit intervals to display
    ui_count: u32,
    /// Number of initial bits to skip for settling
    skip_initial: usize,
    /// Points per UI for interpolation
    points_per_ui: usize,
}

impl Default for EyeDataBuilder {
    fn default() -> Self {
        Self {
            bit_period: 1e-9,
            ui_count: 2,
            skip_initial: 10,
            points_per_ui: 100,
        }
    }
}

impl EyeDataBuilder {
    /// Create new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set bit period
    pub fn bit_period(mut self, period: f64) -> Self {
        self.bit_period = period;
        self
    }

    /// Set from data rate (bits per second)
    pub fn data_rate(mut self, rate: f64) -> Self {
        if rate > 0.0 {
            self.bit_period = 1.0 / rate;
        }
        self
    }

    /// Set number of unit intervals to display
    pub fn ui_count(mut self, count: u32) -> Self {
        self.ui_count = count.max(1);
        self
    }

    /// Set number of initial bits to skip
    pub fn skip_initial(mut self, skip: usize) -> Self {
        self.skip_initial = skip;
        self
    }

    /// Set interpolation resolution
    pub fn points_per_ui(mut self, points: usize) -> Self {
        self.points_per_ui = points.max(10);
        self
    }

    /// Build eye diagram from transient waveform
    pub fn build(&self, time: &[f64], signal: &[f64]) -> EyeData {
        let n = time.len().min(signal.len());
        if n < 2 || self.bit_period <= 0.0 {
            return EyeData::new(self.bit_period, self.ui_count);
        }

        let mut eye_data = EyeData::new(self.bit_period, self.ui_count);
        let window_duration = self.bit_period * self.ui_count as f64;
        let skip_time = self.bit_period * self.skip_initial as f64;

        // Find first usable time point
        let start_idx = time.iter().position(|&t| t >= skip_time).unwrap_or(0);
        if start_idx >= n - 1 {
            return eye_data;
        }

        let first_time = time[start_idx];
        let last_time = time[n - 1];

        // Calculate voltage levels from data
        let valid_samples: Vec<f64> = signal[start_idx..n]
            .iter()
            .copied()
            .filter(|v| v.is_finite())
            .collect();

        if valid_samples.is_empty() {
            return eye_data;
        }

        let v_min = valid_samples.iter().copied().fold(f64::MAX, f64::min);
        let v_max = valid_samples.iter().copied().fold(f64::MIN, f64::max);
        eye_data.v_low = v_min;
        eye_data.v_high = v_max;
        eye_data.swing = v_max - v_min;
        eye_data.v_cross = (v_max + v_min) / 2.0;
        eye_data.data_rate = 1.0 / self.bit_period;

        let resampled_points = (self.ui_count as usize)
            .saturating_mul(self.points_per_ui)
            .saturating_add(1)
            .max(2);

        // Build traces by slicing waveform into windows and resampling each window
        // to a consistent point count for stable rendering and measurement fidelity.
        let mut window_start = first_time;
        let mut sample_start_idx = start_idx;
        let mut sample_end_idx = start_idx;
        while window_start + window_duration <= last_time {
            let window_end = window_start + window_duration;

            while sample_start_idx < n && time[sample_start_idx] < window_start {
                sample_start_idx += 1;
            }
            if sample_end_idx < sample_start_idx {
                sample_end_idx = sample_start_idx;
            }
            while sample_end_idx < n && time[sample_end_idx] <= window_end {
                sample_end_idx += 1;
            }

            let interp_start_idx = sample_start_idx.saturating_sub(1);
            let interp_end_idx = (sample_end_idx + 1).min(n);
            if interp_end_idx.saturating_sub(interp_start_idx) >= 2
                && let Some(trace) = resample_window_trace(
                    &time[..n],
                    &signal[..n],
                    interp_start_idx,
                    interp_end_idx,
                    window_start,
                    window_duration,
                    self.bit_period,
                    resampled_points,
                )
            {
                eye_data.add_trace(trace);
            }

            window_start += self.bit_period;
        }

        eye_data
    }
}

fn resample_window_trace(
    time: &[f64],
    signal: &[f64],
    start_idx: usize,
    end_idx: usize,
    window_start: f64,
    window_duration: f64,
    bit_period: f64,
    sample_count: usize,
) -> Option<EyeTrace> {
    if end_idx <= start_idx + 1 || sample_count < 2 || !window_duration.is_finite() {
        return None;
    }

    let first_t = time[start_idx];
    let last_t = time[end_idx - 1];
    if !first_t.is_finite() || !last_t.is_finite() || last_t <= first_t {
        return None;
    }

    let mut trace_time = Vec::with_capacity(sample_count);
    let mut trace_amp = Vec::with_capacity(sample_count);
    let step = window_duration / (sample_count - 1) as f64;
    let mut segment_idx = start_idx;

    for j in 0..sample_count {
        let target_t = window_start + j as f64 * step;
        if target_t < first_t || target_t > last_t {
            continue;
        }

        while segment_idx + 1 < end_idx && time[segment_idx + 1] < target_t {
            segment_idx += 1;
        }
        if segment_idx + 1 >= end_idx {
            break;
        }

        let t0 = time[segment_idx];
        let t1 = time[segment_idx + 1];
        let v0 = signal[segment_idx];
        let v1 = signal[segment_idx + 1];
        if !t0.is_finite() || !t1.is_finite() || !v0.is_finite() || !v1.is_finite() {
            continue;
        }

        let dt = t1 - t0;
        if dt <= 0.0 {
            continue;
        }

        let alpha = ((target_t - t0) / dt).clamp(0.0, 1.0);
        let amplitude = v0 + alpha * (v1 - v0);
        let normalized_t = (target_t - window_start) / bit_period;

        if normalized_t.is_finite() && amplitude.is_finite() {
            trace_time.push(normalized_t);
            trace_amp.push(amplitude);
        }
    }

    if trace_time.len() >= 2 {
        Some(EyeTrace::new(trace_time, trace_amp))
    } else {
        None
    }
}

// =============================================================================
// Edge Detection
// =============================================================================

/// Detected edge in waveform
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edge {
    /// Time of edge crossing
    pub time: f64,
    /// Is this a rising edge
    pub rising: bool,
    /// Slew rate (V/s)
    pub slew_rate: f64,
}

/// Find edges in waveform (threshold crossings)
pub fn find_edges(time: &[f64], signal: &[f64], threshold: f64) -> Vec<Edge> {
    let n = time.len().min(signal.len());
    if n < 2 {
        return Vec::new();
    }

    let mut edges = Vec::new();

    for i in 0..n - 1 {
        let v0 = signal[i];
        let v1 = signal[i + 1];
        let t0 = time[i];
        let t1 = time[i + 1];

        if !v0.is_finite() || !v1.is_finite() || !t0.is_finite() || !t1.is_finite() {
            continue;
        }

        let dt = t1 - t0;
        if dt <= 0.0 {
            continue;
        }

        // Rising edge
        if v0 < threshold && v1 >= threshold {
            let t_cross = t0 + (threshold - v0) / (v1 - v0) * dt;
            let slew = (v1 - v0) / dt;
            edges.push(Edge {
                time: t_cross,
                rising: true,
                slew_rate: slew,
            });
        }
        // Falling edge
        else if v0 >= threshold && v1 < threshold {
            let t_cross = t0 + (v0 - threshold) / (v0 - v1) * dt;
            let slew = (v1 - v0) / dt;
            edges.push(Edge {
                time: t_cross,
                rising: false,
                slew_rate: slew.abs(),
            });
        }
    }

    edges
}

/// Calculate edge rate (rise or fall time) between two thresholds
pub fn calculate_edge_rate(
    time: &[f64],
    signal: &[f64],
    v_low_thresh: f64,
    v_high_thresh: f64,
    rising: bool,
) -> Option<f64> {
    let n = time.len().min(signal.len());
    if n < 2 {
        return None;
    }

    let mut edge_times = Vec::new();

    for i in 0..n - 1 {
        let v0 = signal[i];
        let v1 = signal[i + 1];
        let t0 = time[i];
        let t1 = time[i + 1];

        if !v0.is_finite() || !v1.is_finite() {
            continue;
        }

        let dt = t1 - t0;
        if dt <= 0.0 {
            continue;
        }

        if rising {
            // Find low threshold crossing
            if v0 < v_low_thresh && v1 >= v_low_thresh {
                let t_low = t0 + (v_low_thresh - v0) / (v1 - v0) * dt;

                // Look ahead for high threshold crossing
                for j in i..n - 1 {
                    let va = signal[j];
                    let vb = signal[j + 1];
                    let ta = time[j];
                    let tb = time[j + 1];

                    if va < v_high_thresh && vb >= v_high_thresh {
                        let t_high = ta + (v_high_thresh - va) / (vb - va) * (tb - ta);
                        edge_times.push(t_high - t_low);
                        break;
                    }
                }
            }
        } else {
            // Find high threshold crossing (falling edge starts high)
            if v0 >= v_high_thresh && v1 < v_high_thresh {
                let t_high = t0 + (v0 - v_high_thresh) / (v0 - v1) * dt;

                // Look ahead for low threshold crossing
                for j in i..n - 1 {
                    let va = signal[j];
                    let vb = signal[j + 1];
                    let ta = time[j];
                    let tb = time[j + 1];

                    if va >= v_low_thresh && vb < v_low_thresh {
                        let t_low = ta + (va - v_low_thresh) / (va - vb) * (tb - ta);
                        edge_times.push(t_low - t_high);
                        break;
                    }
                }
            }
        }
    }

    if edge_times.is_empty() {
        return None;
    }

    // Return mean edge time
    Some(edge_times.iter().sum::<f64>() / edge_times.len() as f64)
}

// =============================================================================
// Tests
// =============================================================================

