//! Performance Summary Table
//!
//! Automated extraction and display of key circuit specifications
//! from simulation results, matching Cadence's Performance Browser.
//!
//! # Supported Metrics
//!
//! - **Gain**: DC gain, AC gain at specific frequency
//! - **Bandwidth**: -3dB point, unity-gain frequency
//! - **Phase Margin**: Stability margin at crossover
//! - **Gain Margin**: Stability margin where phase = -180°
//! - **Slew Rate**: Rising and falling slew rates
//! - **Power**: Total quiescent power dissipation
//! - **Noise**: Input-referred and output-referred noise

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// Performance Metric Types
// =============================================================================

/// Type of performance metric
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MetricType {
    /// DC gain in dB
    DcGain,
    /// AC gain at specific frequency in dB
    AcGain,
    /// Unity gain frequency (0dB crossover)
    Ugbw,
    /// -3dB bandwidth
    Bandwidth3dB,
    /// Phase margin in degrees
    PhaseMargin,
    /// Gain margin in dB
    GainMargin,
    /// Rising slew rate in V/us
    SlewRateRising,
    /// Falling slew rate in V/us
    SlewRateFalling,
    /// Quiescent power dissipation
    QuiescentPower,
    /// Input-referred noise voltage
    InputNoise,
    /// Output-referred noise voltage
    OutputNoise,
    /// Common-mode rejection ratio
    Cmrr,
    /// Power supply rejection ratio
    Psrr,
    /// Output swing (positive)
    OutputSwingPos,
    /// Output swing (negative)
    OutputSwingNeg,
    /// Input offset voltage
    InputOffset,
    /// Settling time
    SettlingTime,
    /// Total harmonic distortion
    Thd,
    /// Custom user-defined metric
    Custom,
}

impl MetricType {
    /// Display name for this metric
    pub fn display_name(&self) -> &'static str {
        match self {
            MetricType::DcGain => "DC Gain",
            MetricType::AcGain => "AC Gain",
            MetricType::Ugbw => "Unity Gain BW",
            MetricType::Bandwidth3dB => "-3dB Bandwidth",
            MetricType::PhaseMargin => "Phase Margin",
            MetricType::GainMargin => "Gain Margin",
            MetricType::SlewRateRising => "Slew Rate (+)",
            MetricType::SlewRateFalling => "Slew Rate (-)",
            MetricType::QuiescentPower => "Quiescent Power",
            MetricType::InputNoise => "Input Noise",
            MetricType::OutputNoise => "Output Noise",
            MetricType::Cmrr => "CMRR",
            MetricType::Psrr => "PSRR",
            MetricType::OutputSwingPos => "Output Swing (+)",
            MetricType::OutputSwingNeg => "Output Swing (-)",
            MetricType::InputOffset => "Input Offset",
            MetricType::SettlingTime => "Settling Time",
            MetricType::Thd => "THD",
            MetricType::Custom => "Custom",
        }
    }

    /// Default unit for this metric
    pub fn default_unit(&self) -> &'static str {
        match self {
            MetricType::DcGain => "dB",
            MetricType::AcGain => "dB",
            MetricType::Ugbw => "Hz",
            MetricType::Bandwidth3dB => "Hz",
            MetricType::PhaseMargin => "°",
            MetricType::GainMargin => "dB",
            MetricType::SlewRateRising => "V/μs",
            MetricType::SlewRateFalling => "V/μs",
            MetricType::QuiescentPower => "W",
            MetricType::InputNoise => "V/√Hz",
            MetricType::OutputNoise => "V/√Hz",
            MetricType::Cmrr => "dB",
            MetricType::Psrr => "dB",
            MetricType::OutputSwingPos => "V",
            MetricType::OutputSwingNeg => "V",
            MetricType::InputOffset => "V",
            MetricType::SettlingTime => "s",
            MetricType::Thd => "%",
            MetricType::Custom => "",
        }
    }

    /// All standard amplifier metrics
    pub const AMPLIFIER_METRICS: [MetricType; 10] = [
        MetricType::DcGain,
        MetricType::Ugbw,
        MetricType::Bandwidth3dB,
        MetricType::PhaseMargin,
        MetricType::GainMargin,
        MetricType::SlewRateRising,
        MetricType::SlewRateFalling,
        MetricType::QuiescentPower,
        MetricType::OutputSwingPos,
        MetricType::OutputSwingNeg,
    ];
}

/// Status of a performance metric
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum MetricStatus {
    /// Not yet measured
    #[default]
    Pending,
    /// Successfully measured
    Measured,
    /// Measurement failed
    Failed,
    /// Specification met
    Pass,
    /// Specification violated
    Fail,
}

impl MetricStatus {
    /// Color string for this status
    pub fn color(&self) -> &'static str {
        match self {
            MetricStatus::Pending => "#888888",
            MetricStatus::Measured => "#3b82f6",
            MetricStatus::Failed => "#ef4444",
            MetricStatus::Pass => "#22c55e",
            MetricStatus::Fail => "#f97316",
        }
    }
}

// =============================================================================
// Performance Metric
// =============================================================================

/// A single performance metric with value and specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetric {
    /// Metric type
    pub metric_type: MetricType,
    /// Custom name (for custom metrics)
    pub name: String,
    /// Measured value
    pub value: Option<f64>,
    /// Unit string
    pub unit: String,
    /// Minimum specification
    pub spec_min: Option<f64>,
    /// Maximum specification
    pub spec_max: Option<f64>,
    /// Typical/target value
    pub spec_typ: Option<f64>,
    /// Measurement status
    pub status: MetricStatus,
    /// Corner name (if from corner analysis)
    pub corner: Option<String>,
    /// Additional notes
    pub notes: String,
}

impl Default for PerformanceMetric {
    fn default() -> Self {
        Self {
            metric_type: MetricType::Custom,
            name: String::new(),
            value: None,
            unit: String::new(),
            spec_min: None,
            spec_max: None,
            spec_typ: None,
            status: MetricStatus::Pending,
            corner: None,
            notes: String::new(),
        }
    }
}

impl PerformanceMetric {
    /// Create a new metric of the given type
    pub fn new(metric_type: MetricType) -> Self {
        Self {
            metric_type,
            name: metric_type.display_name().to_string(),
            unit: metric_type.default_unit().to_string(),
            ..Default::default()
        }
    }

    /// Create a custom metric with a name
    pub fn custom(name: impl Into<String>, unit: impl Into<String>) -> Self {
        Self {
            metric_type: MetricType::Custom,
            name: name.into(),
            unit: unit.into(),
            ..Default::default()
        }
    }

    /// Set the measured value
    pub fn with_value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.status = MetricStatus::Measured;
        self
    }

    /// Set minimum specification
    pub fn with_min(mut self, min: f64) -> Self {
        self.spec_min = Some(min);
        self
    }

    /// Set maximum specification
    pub fn with_max(mut self, max: f64) -> Self {
        self.spec_max = Some(max);
        self
    }

    /// Set typical specification
    pub fn with_typ(mut self, typ: f64) -> Self {
        self.spec_typ = Some(typ);
        self
    }

    /// Set corner name
    pub fn with_corner(mut self, corner: impl Into<String>) -> Self {
        self.corner = Some(corner.into());
        self
    }

    /// Check if value meets specification and update status
    pub fn check_spec(&mut self) {
        if let Some(value) = self.value {
            let meets_min = self.spec_min.is_none_or(|min| value >= min);
            let meets_max = self.spec_max.is_none_or(|max| value <= max);
            self.status = if meets_min && meets_max {
                MetricStatus::Pass
            } else {
                MetricStatus::Fail
            };
        }
    }

    /// Format value for display
    pub fn formatted_value(&self) -> String {
        match self.value {
            Some(v) => format_engineering(v, &self.unit),
            None => "—".to_string(),
        }
    }
}

/// Format a value in engineering notation with unit
fn format_engineering(value: f64, unit: &str) -> String {
    let abs_val = value.abs();
    let (scaled, prefix) = if abs_val >= 1e12 {
        (value / 1e12, "T")
    } else if abs_val >= 1e9 {
        (value / 1e9, "G")
    } else if abs_val >= 1e6 {
        (value / 1e6, "M")
    } else if abs_val >= 1e3 {
        (value / 1e3, "k")
    } else if abs_val >= 1.0 {
        (value, "")
    } else if abs_val >= 1e-3 {
        (value * 1e3, "m")
    } else if abs_val >= 1e-6 {
        (value * 1e6, "μ")
    } else if abs_val >= 1e-9 {
        (value * 1e9, "n")
    } else if abs_val >= 1e-12 {
        (value * 1e12, "p")
    } else if abs_val >= 1e-15 {
        (value * 1e15, "f")
    } else {
        (value, "")
    };

    if scaled.abs() >= 100.0 {
        format!("{:.1}{}{}", scaled, prefix, unit)
    } else if scaled.abs() >= 10.0 {
        format!("{:.2}{}{}", scaled, prefix, unit)
    } else {
        format!("{:.3}{}{}", scaled, prefix, unit)
    }
}

// =============================================================================
// Performance Summary
// =============================================================================

/// Complete performance summary table
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerformanceSummary {
    /// Summary name/title
    pub name: String,
    /// All metrics organized by type
    metrics: Vec<PerformanceMetric>,
    /// Grouped by corner for multi-corner analysis
    corner_metrics: HashMap<String, Vec<PerformanceMetric>>,
    /// Last update timestamp
    pub last_updated: Option<String>,
}

impl PerformanceSummary {
    /// Create a new empty summary
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Add a metric
    pub fn add_metric(&mut self, metric: PerformanceMetric) {
        self.metrics.push(metric);
    }

    /// Add a metric for a specific corner
    pub fn add_corner_metric(&mut self, corner: &str, metric: PerformanceMetric) {
        self.corner_metrics
            .entry(corner.to_string())
            .or_default()
            .push(metric);
    }

    /// Get all metrics
    pub fn metrics(&self) -> &[PerformanceMetric] {
        &self.metrics
    }

    /// Get metrics for a specific corner
    pub fn corner_metrics(&self, corner: &str) -> Option<&Vec<PerformanceMetric>> {
        self.corner_metrics.get(corner)
    }

    /// Get all corner names
    pub fn corners(&self) -> Vec<&String> {
        self.corner_metrics.keys().collect()
    }

    /// Get a metric by type
    pub fn get(&self, metric_type: MetricType) -> Option<&PerformanceMetric> {
        self.metrics.iter().find(|m| m.metric_type == metric_type)
    }

    /// Get a mutable metric by type
    pub fn get_mut(&mut self, metric_type: MetricType) -> Option<&mut PerformanceMetric> {
        self.metrics
            .iter_mut()
            .find(|m| m.metric_type == metric_type)
    }

    /// Count metrics by status
    pub fn count_by_status(&self) -> HashMap<MetricStatus, usize> {
        let mut counts = HashMap::new();
        for metric in &self.metrics {
            *counts.entry(metric.status).or_insert(0) += 1;
        }
        counts
    }

    /// Check if all specs are met
    pub fn all_pass(&self) -> bool {
        self.metrics
            .iter()
            .filter(|m| m.spec_min.is_some() || m.spec_max.is_some())
            .all(|m| m.status == MetricStatus::Pass)
    }

    /// Initialize with standard amplifier metrics
    pub fn init_amplifier_metrics(&mut self) {
        for metric_type in MetricType::AMPLIFIER_METRICS {
            self.add_metric(PerformanceMetric::new(metric_type));
        }
    }

    /// Clear all metrics
    pub fn clear(&mut self) {
        self.metrics.clear();
        self.corner_metrics.clear();
    }

    /// Total metric count
    pub fn len(&self) -> usize {
        self.metrics.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.metrics.is_empty()
    }
}

// =============================================================================
// Metric Extraction Functions
// =============================================================================

/// Extract DC gain from simulation results
pub fn extract_dc_gain(output_values: &[f64], input_values: &[f64]) -> Option<f64> {
    if output_values.is_empty() || input_values.is_empty() {
        return None;
    }

    // Find small-signal gain around midpoint
    let n = output_values.len().min(input_values.len());
    if n < 3 {
        return None;
    }

    let mid = n / 2;
    let delta_out = output_values[mid + 1] - output_values[mid - 1];
    let delta_in = input_values[mid + 1] - input_values[mid - 1];

    if delta_in.abs() < 1e-12 {
        return None;
    }

    Some(20.0 * (delta_out / delta_in).abs().log10())
}

/// Extract -3dB bandwidth from AC response
pub fn extract_bandwidth_3db(freq: &[f64], magnitude_db: &[f64]) -> Option<f64> {
    if freq.is_empty() || magnitude_db.is_empty() {
        return None;
    }

    let n = freq.len().min(magnitude_db.len());
    let dc_gain = magnitude_db[0];
    let threshold = dc_gain - 3.0;

    // Find crossing point
    for i in 1..n {
        if magnitude_db[i] <= threshold && magnitude_db[i - 1] > threshold {
            // Linear interpolation
            let f0 = freq[i - 1];
            let f1 = freq[i];
            let m0 = magnitude_db[i - 1];
            let m1 = magnitude_db[i];
            let t = (threshold - m0) / (m1 - m0);
            return Some(f0 + t * (f1 - f0));
        }
    }

    None
}

/// Extract unity gain bandwidth from AC response
pub fn extract_ugbw(freq: &[f64], magnitude_db: &[f64]) -> Option<f64> {
    if freq.is_empty() || magnitude_db.is_empty() {
        return None;
    }

    let n = freq.len().min(magnitude_db.len());

    // Find 0dB crossing
    for i in 1..n {
        if magnitude_db[i] <= 0.0 && magnitude_db[i - 1] > 0.0 {
            let f0 = freq[i - 1];
            let f1 = freq[i];
            let m0 = magnitude_db[i - 1];
            let m1 = magnitude_db[i];
            let t = (0.0 - m0) / (m1 - m0);
            return Some(f0 + t * (f1 - f0));
        }
    }

    None
}

/// Extract phase margin from AC response
pub fn extract_phase_margin(freq: &[f64], magnitude_db: &[f64], phase_deg: &[f64]) -> Option<f64> {
    // Find unity gain frequency
    let ugbw = extract_ugbw(freq, magnitude_db)?;

    let n = freq.len().min(phase_deg.len());

    // Find phase at unity gain frequency
    for i in 1..n {
        if freq[i] >= ugbw {
            // Linear interpolation
            let f0 = freq[i - 1];
            let f1 = freq[i];
            let p0 = phase_deg[i - 1];
            let p1 = phase_deg[i];
            let t = (ugbw - f0) / (f1 - f0);
            let phase_at_ugbw = p0 + t * (p1 - p0);
            // Phase margin = 180 + phase at crossover
            return Some(180.0 + phase_at_ugbw);
        }
    }

    None
}

/// Extract slew rate from transient step response
pub fn extract_slew_rate(time: &[f64], voltage: &[f64]) -> Option<(f64, f64)> {
    if time.is_empty() || voltage.is_empty() {
        return None;
    }

    let n = time.len().min(voltage.len());
    if n < 3 {
        return None;
    }

    let mut max_rising = 0.0f64;
    let mut max_falling = 0.0f64;

    for i in 1..n {
        let dt = time[i] - time[i - 1];
        if dt > 0.0 {
            let dv = voltage[i] - voltage[i - 1];
            let slew = dv / dt;
            if slew > 0.0 {
                max_rising = max_rising.max(slew);
            } else {
                max_falling = max_falling.max(slew.abs());
            }
        }
    }

    // Convert to V/μs
    Some((max_rising * 1e-6, max_falling * 1e-6))
}

// =============================================================================
// Tests
// =============================================================================
