//! Waveform Measurements
//!
//! Commercial-grade waveform measurement calculations matching
//! the capabilities of Cadence ViVA and similar tools.
//!
//! # Supported Measurements
//!
//! - **Amplitude**: min, max, pk-pk, mean, RMS
//! - **Timing**: rise time, fall time, period, frequency, duty cycle
//! - **Statistics**: standard deviation, variance
//! - **Area**: integral under curve

use super::state::TraceData;

// =============================================================================
// Measurement Types
// =============================================================================

/// Waveform measurement result
#[derive(Debug, Clone, PartialEq)]
pub struct Measurement {
    /// Measurement name
    pub name: &'static str,
    /// Measurement value
    pub value: f64,
    /// Unit string
    pub unit: &'static str,
}

impl Measurement {
    /// Create a new measurement
    pub fn new(name: &'static str, value: f64, unit: &'static str) -> Self {
        Self { name, value, unit }
    }
}

/// Complete measurement set for a trace
#[derive(Debug, Clone, Default)]
pub struct TraceMeasurements {
    /// Trace name
    pub trace_name: String,
    /// Minimum value
    pub min: Option<f64>,
    /// Maximum value
    pub max: Option<f64>,
    /// Peak-to-peak
    pub pk_pk: Option<f64>,
    /// Mean (average)
    pub mean: Option<f64>,
    /// RMS value
    pub rms: Option<f64>,
    /// Standard deviation
    pub std_dev: Option<f64>,
    /// Rise time (10%-90%)
    pub rise_time: Option<f64>,
    /// Fall time (90%-10%)
    pub fall_time: Option<f64>,
    /// Period (for periodic signals)
    pub period: Option<f64>,
    /// Frequency (1/period)
    pub frequency: Option<f64>,
    /// Duty cycle (%) for digital-like signals
    pub duty_cycle: Option<f64>,
    /// Integral (area under curve)
    pub integral: Option<f64>,
}

#[derive(Debug, Clone, Copy, Default)]
struct BasicStats {
    count: usize,
    min: f64,
    max: f64,
    mean: f64,
    sum_squares: f64,
    m2: f64,
}

impl BasicStats {
    fn from_samples(samples: &[f64]) -> Option<Self> {
        let mut stats = Self {
            count: 0,
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            mean: 0.0,
            sum_squares: 0.0,
            m2: 0.0,
        };

        for &value in samples {
            if !value.is_finite() {
                continue;
            }

            stats.count += 1;
            stats.min = stats.min.min(value);
            stats.max = stats.max.max(value);
            stats.sum_squares += value * value;

            let delta = value - stats.mean;
            stats.mean += delta / stats.count as f64;
            let delta2 = value - stats.mean;
            stats.m2 += delta * delta2;
        }

        (stats.count > 0).then_some(stats)
    }

    fn pk_pk(self) -> f64 {
        self.max - self.min
    }

    fn rms(self) -> f64 {
        (self.sum_squares / self.count as f64).sqrt()
    }

    fn std_dev(self) -> Option<f64> {
        (self.count > 1).then(|| (self.m2 / (self.count - 1) as f64).sqrt())
    }
}

// =============================================================================
// Measurement Cache
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TraceSignature {
    sample_count: usize,
    x_first_bits: u64,
    x_last_bits: u64,
    y_first_bits: u64,
    y_last_bits: u64,
}

impl TraceSignature {
    fn from_trace(trace: &TraceData) -> Self {
        Self {
            sample_count: trace.len(),
            x_first_bits: trace.x.first().copied().unwrap_or(0.0).to_bits(),
            x_last_bits: trace.x.last().copied().unwrap_or(0.0).to_bits(),
            y_first_bits: trace.y.first().copied().unwrap_or(0.0).to_bits(),
            y_last_bits: trace.y.last().copied().unwrap_or(0.0).to_bits(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RangeKey {
    start_bits: u64,
    end_bits: u64,
}

impl RangeKey {
    fn from_range((start, end): (f64, f64)) -> Self {
        Self {
            start_bits: start.to_bits(),
            end_bits: end.to_bits(),
        }
    }
}

#[derive(Debug, Clone)]
struct MeasurementCacheEntry {
    signature: TraceSignature,
    range: Option<RangeKey>,
    measurements: TraceMeasurements,
}

/// Runtime cache for waveform measurement panel results.
///
/// The measurement panel is rendered every frame. Without caching, all metrics
/// are recomputed per trace on each redraw, which scales poorly with large
/// sample sets. This cache stores the last computed result per trace index and
/// cursor-range mode, and refreshes only when inputs change.
#[derive(Debug, Clone, Default)]
pub struct MeasurementCache {
    entries: Vec<Option<MeasurementCacheEntry>>,
}

impl MeasurementCache {
    /// Clear all cached entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Shrink cached entries to match current trace count.
    pub fn truncate_to_trace_count(&mut self, trace_count: usize) {
        self.entries.truncate(trace_count);
    }

    /// Get cached measurements for a trace/range key or compute and cache them.
    pub fn get_or_compute<'a>(
        &'a mut self,
        trace_index: usize,
        trace: &TraceData,
        range: Option<(f64, f64)>,
    ) -> &'a TraceMeasurements {
        if self.entries.len() <= trace_index {
            self.entries.resize_with(trace_index + 1, || None);
        }

        let signature = TraceSignature::from_trace(trace);
        let range_key = range.map(RangeKey::from_range);

        let refresh_required = self.entries[trace_index]
            .as_ref()
            .is_none_or(|entry| entry.signature != signature || entry.range != range_key);

        if refresh_required {
            let measurements = if let Some((start, end)) = range {
                calculate_measurements_in_range(trace, start, end)
            } else {
                calculate_all_measurements(trace)
            };

            self.entries[trace_index] = Some(MeasurementCacheEntry {
                signature,
                range: range_key,
                measurements,
            });
        }

        &self.entries[trace_index]
            .get_or_insert_with(|| MeasurementCacheEntry {
                signature,
                range: range_key,
                measurements: if let Some((start, end)) = range {
                    calculate_measurements_in_range(trace, start, end)
                } else {
                    calculate_all_measurements(trace)
                },
            })
            .measurements
    }
}

// =============================================================================
// Basic Amplitude Measurements
// =============================================================================

/// Calculate minimum value in a region
pub fn calculate_min(y_data: &[f64]) -> Option<f64> {
    BasicStats::from_samples(y_data).map(|stats| stats.min)
}

/// Calculate maximum value in a region
pub fn calculate_max(y_data: &[f64]) -> Option<f64> {
    BasicStats::from_samples(y_data).map(|stats| stats.max)
}

/// Calculate peak-to-peak amplitude
pub fn calculate_pk_pk(y_data: &[f64]) -> Option<f64> {
    BasicStats::from_samples(y_data).map(BasicStats::pk_pk)
}

/// Calculate mean (average) value
pub fn calculate_mean(y_data: &[f64]) -> Option<f64> {
    BasicStats::from_samples(y_data).map(|stats| stats.mean)
}

/// Calculate RMS (root mean square) value
pub fn calculate_rms(y_data: &[f64]) -> Option<f64> {
    BasicStats::from_samples(y_data).map(BasicStats::rms)
}

/// Calculate standard deviation
pub fn calculate_std_dev(y_data: &[f64]) -> Option<f64> {
    BasicStats::from_samples(y_data).and_then(BasicStats::std_dev)
}

// =============================================================================
// Timing Measurements
// =============================================================================

/// Threshold crossing detection result
#[derive(Debug, Clone)]
pub struct Crossing {
    /// Index of the first point before crossing
    pub index: usize,
    /// Interpolated X position of crossing
    pub x: f64,
    /// Whether this is a rising or falling edge
    pub rising: bool,
}

/// Find all threshold crossings in the waveform
pub fn find_crossings(x_data: &[f64], y_data: &[f64], threshold: f64) -> Vec<Crossing> {
    if x_data.len() < 2 || y_data.len() < 2 {
        return Vec::new();
    }

    let n = x_data.len().min(y_data.len());
    let mut crossings = Vec::new();

    for i in 0..n - 1 {
        let y0 = y_data[i];
        let y1 = y_data[i + 1];

        if !y0.is_finite() || !y1.is_finite() {
            continue;
        }

        // Check for rising crossing
        if y0 < threshold && y1 >= threshold {
            let x = interpolate_crossing(x_data[i], x_data[i + 1], y0, y1, threshold);
            crossings.push(Crossing {
                index: i,
                x,
                rising: true,
            });
        }
        // Check for falling crossing
        else if y0 >= threshold && y1 < threshold {
            let x = interpolate_crossing(x_data[i], x_data[i + 1], y0, y1, threshold);
            crossings.push(Crossing {
                index: i,
                x,
                rising: false,
            });
        }
    }

    crossings
}

/// Interpolate X position of threshold crossing
fn interpolate_crossing(x0: f64, x1: f64, y0: f64, y1: f64, threshold: f64) -> f64 {
    let dy = y1 - y0;
    if dy.abs() < 1e-15 {
        return (x0 + x1) / 2.0;
    }
    let t = (threshold - y0) / dy;
    x0 + t * (x1 - x0)
}

/// Calculate rise time (10% to 90% of swing)
pub fn calculate_rise_time(x_data: &[f64], y_data: &[f64]) -> Option<f64> {
    let min = calculate_min(y_data)?;
    let max = calculate_max(y_data)?;
    let range = max - min;

    if range < 1e-15 {
        return None;
    }

    let low_threshold = min + 0.1 * range;
    let high_threshold = min + 0.9 * range;

    // Find first rising edge through both thresholds
    let low_crossings = find_crossings(x_data, y_data, low_threshold);
    let high_crossings = find_crossings(x_data, y_data, high_threshold);

    // Find first rising low crossing
    let first_low_rise = low_crossings.iter().find(|c| c.rising)?;

    // Find first rising high crossing after the low crossing
    let first_high_rise = high_crossings
        .iter()
        .find(|c| c.rising && c.x > first_low_rise.x)?;

    Some(first_high_rise.x - first_low_rise.x)
}

/// Calculate fall time (90% to 10% of swing)
pub fn calculate_fall_time(x_data: &[f64], y_data: &[f64]) -> Option<f64> {
    let min = calculate_min(y_data)?;
    let max = calculate_max(y_data)?;
    let range = max - min;

    if range < 1e-15 {
        return None;
    }

    let high_threshold = min + 0.9 * range;
    let low_threshold = min + 0.1 * range;

    let high_crossings = find_crossings(x_data, y_data, high_threshold);
    let low_crossings = find_crossings(x_data, y_data, low_threshold);

    // Find first falling high crossing
    let first_high_fall = high_crossings.iter().find(|c| !c.rising)?;

    // Find first falling low crossing after the high crossing
    let first_low_fall = low_crossings
        .iter()
        .find(|c| !c.rising && c.x > first_high_fall.x)?;

    Some(first_low_fall.x - first_high_fall.x)
}

/// Calculate period from rising edge to rising edge
pub fn calculate_period(x_data: &[f64], y_data: &[f64]) -> Option<f64> {
    let mean = calculate_mean(y_data)?;
    let crossings = find_crossings(x_data, y_data, mean);

    // Find two consecutive rising crossings
    let rising: Vec<&Crossing> = crossings.iter().filter(|c| c.rising).collect();

    if rising.len() < 2 {
        return None;
    }

    // Average of all periods
    let mut periods = Vec::new();
    for i in 1..rising.len() {
        periods.push(rising[i].x - rising[i - 1].x);
    }

    if periods.is_empty() {
        return None;
    }

    Some(periods.iter().sum::<f64>() / periods.len() as f64)
}

/// Calculate frequency (1/period)
pub fn calculate_frequency(x_data: &[f64], y_data: &[f64]) -> Option<f64> {
    calculate_period(x_data, y_data).map(|p| if p > 0.0 { 1.0 / p } else { 0.0 })
}

/// Calculate duty cycle (percentage of time signal is above mean)
pub fn calculate_duty_cycle(x_data: &[f64], y_data: &[f64]) -> Option<f64> {
    if x_data.len() < 2 || y_data.len() < 2 {
        return None;
    }

    let mean = calculate_mean(y_data)?;
    let n = x_data.len().min(y_data.len());

    let mut high_time = 0.0;
    let mut total_time = 0.0;

    for i in 0..n - 1 {
        let dt = x_data[i + 1] - x_data[i];
        if dt > 0.0 && y_data[i].is_finite() {
            total_time += dt;
            if y_data[i] >= mean {
                high_time += dt;
            }
        }
    }

    if total_time > 0.0 {
        Some(100.0 * high_time / total_time)
    } else {
        None
    }
}

// =============================================================================
// Integral/Area Measurements
// =============================================================================

/// Calculate integral (area under curve) using trapezoidal rule
pub fn calculate_integral(x_data: &[f64], y_data: &[f64]) -> Option<f64> {
    if x_data.len() < 2 || y_data.len() < 2 {
        return None;
    }

    let n = x_data.len().min(y_data.len());
    let mut integral = 0.0;

    for i in 0..n - 1 {
        let y0 = y_data[i];
        let y1 = y_data[i + 1];
        let dx = x_data[i + 1] - x_data[i];

        if y0.is_finite() && y1.is_finite() && dx.is_finite() {
            // Trapezoidal rule
            integral += 0.5 * (y0 + y1) * dx;
        }
    }

    Some(integral)
}

// =============================================================================
// Complete Measurement Calculation
// =============================================================================

/// Calculate all measurements for a trace
pub fn calculate_all_measurements(trace: &TraceData) -> TraceMeasurements {
    let x = &trace.x;
    let y = &trace.y;
    let basic = BasicStats::from_samples(y);

    TraceMeasurements {
        trace_name: trace.name.clone(),
        min: basic.map(|stats| stats.min),
        max: basic.map(|stats| stats.max),
        pk_pk: basic.map(BasicStats::pk_pk),
        mean: basic.map(|stats| stats.mean),
        rms: basic.map(BasicStats::rms),
        std_dev: basic.and_then(BasicStats::std_dev),
        rise_time: calculate_rise_time(x, y),
        fall_time: calculate_fall_time(x, y),
        period: calculate_period(x, y),
        frequency: calculate_frequency(x, y),
        duty_cycle: calculate_duty_cycle(x, y),
        integral: calculate_integral(x, y),
    }
}

/// Calculate measurements over a specified X range
pub fn calculate_measurements_in_range(
    trace: &TraceData,
    x_start: f64,
    x_end: f64,
) -> TraceMeasurements {
    let n = trace.len();
    let start_idx = trace.x.partition_point(|x| *x < x_start).min(n);
    let mut end_idx = (trace.x.partition_point(|x| *x <= x_end) + 1).min(n);
    if end_idx <= start_idx {
        end_idx = (start_idx + 1).min(n);
    }

    // Create sliced trace
    let x_slice = &trace.x[start_idx..end_idx.min(n)];
    let y_slice = &trace.y[start_idx..end_idx.min(n)];
    let basic = BasicStats::from_samples(y_slice);

    TraceMeasurements {
        trace_name: trace.name.clone(),
        min: basic.map(|stats| stats.min),
        max: basic.map(|stats| stats.max),
        pk_pk: basic.map(BasicStats::pk_pk),
        mean: basic.map(|stats| stats.mean),
        rms: basic.map(BasicStats::rms),
        std_dev: basic.and_then(BasicStats::std_dev),
        rise_time: calculate_rise_time(x_slice, y_slice),
        fall_time: calculate_fall_time(x_slice, y_slice),
        period: calculate_period(x_slice, y_slice),
        frequency: calculate_frequency(x_slice, y_slice),
        duty_cycle: calculate_duty_cycle(x_slice, y_slice),
        integral: calculate_integral(x_slice, y_slice),
    }
}

// =============================================================================
// Measurement Annotations
// =============================================================================

/// Type of measurement annotation marker
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnnotationType {
    /// Point marker (single X,Y location)
    #[default]
    Point,
    /// Vertical line marker (at specific X)
    VerticalLine,
    /// Horizontal line marker (at specific Y)
    HorizontalLine,
    /// Time delta between two X points
    DeltaTime,
    /// Voltage/amplitude delta between two Y points
    DeltaAmplitude,
    /// Rise time marker (shows 10%-90% region)
    RiseTime,
    /// Fall time marker (shows 90%-10% region)
    FallTime,
    /// Crossing marker (at threshold crossing)
    Crossing,
    /// Period marker (one complete cycle)
    Period,
    /// Frequency marker
    Frequency,
}

impl AnnotationType {
    /// Display name for annotation type
    pub fn display_name(&self) -> &'static str {
        match self {
            AnnotationType::Point => "Point",
            AnnotationType::VerticalLine => "Vertical Line",
            AnnotationType::HorizontalLine => "Horizontal Line",
            AnnotationType::DeltaTime => "ΔT",
            AnnotationType::DeltaAmplitude => "ΔV",
            AnnotationType::RiseTime => "Rise Time",
            AnnotationType::FallTime => "Fall Time",
            AnnotationType::Crossing => "Crossing",
            AnnotationType::Period => "Period",
            AnnotationType::Frequency => "Frequency",
        }
    }

    /// All annotation types for UI menus
    pub const ALL: [AnnotationType; 10] = [
        AnnotationType::Point,
        AnnotationType::VerticalLine,
        AnnotationType::HorizontalLine,
        AnnotationType::DeltaTime,
        AnnotationType::DeltaAmplitude,
        AnnotationType::RiseTime,
        AnnotationType::FallTime,
        AnnotationType::Crossing,
        AnnotationType::Period,
        AnnotationType::Frequency,
    ];
}

/// Callout position relative to the marker
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CalloutPosition {
    /// Above the marker
    #[default]
    Above,
    /// Below the marker
    Below,
    /// To the left
    Left,
    /// To the right
    Right,
    /// Auto-position to avoid overlaps
    Auto,
}

/// A visual measurement annotation on a waveform
#[derive(Debug, Clone)]
pub struct MeasurementAnnotation {
    /// Unique annotation ID
    pub id: u64,
    /// Annotation type
    pub annotation_type: AnnotationType,
    /// Associated trace name
    pub trace_name: String,
    /// Primary X position (data coordinates)
    pub x1: f64,
    /// Primary Y position (data coordinates)
    pub y1: f64,
    /// Secondary X position (for delta measurements)
    pub x2: Option<f64>,
    /// Secondary Y position (for delta measurements)
    pub y2: Option<f64>,
    /// Measurement value (calculated or user-specified)
    pub value: f64,
    /// Display label (auto-generated or user-specified)
    pub label: String,
    /// Unit string for display
    pub unit: String,
    /// Callout position
    pub callout_position: CalloutPosition,
    /// Annotation color as RGBA
    pub color: [u8; 4],
    /// Whether annotation is visible
    pub visible: bool,
    /// Whether annotation is locked (can't be moved)
    pub locked: bool,
}

impl Default for MeasurementAnnotation {
    fn default() -> Self {
        Self {
            id: 0,
            annotation_type: AnnotationType::Point,
            trace_name: String::new(),
            x1: 0.0,
            y1: 0.0,
            x2: None,
            y2: None,
            value: 0.0,
            label: String::new(),
            unit: String::new(),
            callout_position: CalloutPosition::Above,
            color: [255, 200, 50, 255], // Yellow/gold
            visible: true,
            locked: false,
        }
    }
}

impl MeasurementAnnotation {
    /// Create a new point annotation
    pub fn point(trace: &str, x: f64, y: f64, label: impl Into<String>) -> Self {
        Self {
            annotation_type: AnnotationType::Point,
            trace_name: trace.to_string(),
            x1: x,
            y1: y,
            label: label.into(),
            ..Default::default()
        }
    }

    /// Create a delta-time annotation between two points
    pub fn delta_time(trace: &str, x1: f64, x2: f64, y: f64) -> Self {
        let dt = (x2 - x1).abs();
        Self {
            annotation_type: AnnotationType::DeltaTime,
            trace_name: trace.to_string(),
            x1,
            y1: y,
            x2: Some(x2),
            y2: Some(y),
            value: dt,
            label: format!("ΔT = {:.3e}", dt),
            unit: "s".to_string(),
            ..Default::default()
        }
    }

    /// Create a delta-amplitude annotation between two points
    pub fn delta_amplitude(trace: &str, x: f64, y1: f64, y2: f64) -> Self {
        let dv = (y2 - y1).abs();
        Self {
            annotation_type: AnnotationType::DeltaAmplitude,
            trace_name: trace.to_string(),
            x1: x,
            y1,
            x2: Some(x),
            y2: Some(y2),
            value: dv,
            label: format!("ΔV = {:.3e}", dv),
            unit: "V".to_string(),
            ..Default::default()
        }
    }

    /// Create a rise time annotation from measurement
    pub fn from_rise_time(trace: &TraceData) -> Option<Self> {
        let rise = calculate_rise_time(&trace.x, &trace.y)?;
        let min = calculate_min(&trace.y)?;
        let max = calculate_max(&trace.y)?;
        let low = min + 0.1 * (max - min);
        let high = min + 0.9 * (max - min);

        Some(Self {
            annotation_type: AnnotationType::RiseTime,
            trace_name: trace.name.clone(),
            x1: trace.x.first().copied().unwrap_or(0.0),
            y1: low,
            x2: Some(trace.x.first().copied().unwrap_or(0.0) + rise),
            y2: Some(high),
            value: rise,
            label: format!("tr = {:.3e}s", rise),
            unit: "s".to_string(),
            color: [50, 255, 50, 255], // Green
            ..Default::default()
        })
    }

    /// Create a period annotation from measurement
    pub fn from_period(trace: &TraceData) -> Option<Self> {
        let period = calculate_period(&trace.x, &trace.y)?;
        let mean = calculate_mean(&trace.y)?;

        Some(Self {
            annotation_type: AnnotationType::Period,
            trace_name: trace.name.clone(),
            x1: trace.x.first().copied().unwrap_or(0.0),
            y1: mean,
            x2: Some(trace.x.first().copied().unwrap_or(0.0) + period),
            y2: Some(mean),
            value: period,
            label: format!("T = {:.3e}s", period),
            unit: "s".to_string(),
            color: [100, 150, 255, 255], // Blue
            ..Default::default()
        })
    }

    /// Set annotation ID
    pub fn with_id(mut self, id: u64) -> Self {
        self.id = id;
        self
    }

    /// Set callout position
    pub fn with_callout(mut self, position: CalloutPosition) -> Self {
        self.callout_position = position;
        self
    }

    /// Set color
    pub fn with_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.color = [r, g, b, 255];
        self
    }
}

/// Collection of measurement annotations for a viewer
#[derive(Debug, Clone, Default)]
pub struct AnnotationSet {
    /// All annotations
    annotations: Vec<MeasurementAnnotation>,
    /// Next available ID
    next_id: u64,
}

impl AnnotationSet {
    /// Create a new empty annotation set
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an annotation and return its ID
    pub fn add(&mut self, mut annotation: MeasurementAnnotation) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        annotation.id = id;
        self.annotations.push(annotation);
        id
    }

    /// Remove an annotation by ID
    pub fn remove(&mut self, id: u64) -> bool {
        if let Some(pos) = self.annotations.iter().position(|a| a.id == id) {
            self.annotations.remove(pos);
            true
        } else {
            false
        }
    }

    /// Get annotation by ID
    pub fn get(&self, id: u64) -> Option<&MeasurementAnnotation> {
        self.annotations.iter().find(|a| a.id == id)
    }

    /// Get mutable annotation by ID
    pub fn get_mut(&mut self, id: u64) -> Option<&mut MeasurementAnnotation> {
        self.annotations.iter_mut().find(|a| a.id == id)
    }

    /// Get all annotations for a specific trace
    pub fn for_trace(&self, trace_name: &str) -> Vec<&MeasurementAnnotation> {
        self.annotations
            .iter()
            .filter(|a| a.trace_name == trace_name && a.visible)
            .collect()
    }

    /// Get all visible annotations
    pub fn visible(&self) -> impl Iterator<Item = &MeasurementAnnotation> {
        self.annotations.iter().filter(|a| a.visible)
    }

    /// Total annotation count
    pub fn len(&self) -> usize {
        self.annotations.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.annotations.is_empty()
    }

    /// Clear all annotations
    pub fn clear(&mut self) {
        self.annotations.clear();
    }

    /// Toggle visibility of an annotation
    pub fn toggle_visibility(&mut self, id: u64) {
        if let Some(a) = self.get_mut(id) {
            a.visible = !a.visible;
        }
    }

    /// Lock/unlock an annotation
    pub fn toggle_lock(&mut self, id: u64) {
        if let Some(a) = self.get_mut(id) {
            a.locked = !a.locked;
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_sine_wave(n: usize, amplitude: f64, period: f64) -> (Vec<f64>, Vec<f64>) {
        // Add small phase offset to avoid starting exactly at zero crossing
        let phase_offset = period * 0.01;
        let x: Vec<f64> = (0..n)
            .map(|i| i as f64 * period / n as f64 * 2.0 + phase_offset)
            .collect();
        let y: Vec<f64> = x
            .iter()
            .map(|&t| amplitude * (2.0 * std::f64::consts::PI * t / period).sin())
            .collect();
        (x, y)
    }

    fn make_square_wave(n: usize, amplitude: f64, period: f64) -> (Vec<f64>, Vec<f64>) {
        let x: Vec<f64> = (0..n).map(|i| i as f64 * period / n as f64 * 2.0).collect();
        let y: Vec<f64> = x
            .iter()
            .map(|&t| {
                if (t / period).fract() < 0.5 {
                    amplitude
                } else {
                    -amplitude
                }
            })
            .collect();
        (x, y)
    }

    // =========================================================================
    // Basic Amplitude Tests
    // =========================================================================

    #[test]
    fn test_calculate_min() {
        let y = vec![1.0, -2.0, 3.0, 0.0];
        assert_eq!(calculate_min(&y), Some(-2.0));
    }

    #[test]
    fn test_calculate_min_with_nan() {
        let y = vec![1.0, f64::NAN, -2.0, 3.0];
        assert_eq!(calculate_min(&y), Some(-2.0));
    }

    #[test]
    fn test_calculate_min_empty() {
        let y: Vec<f64> = vec![];
        assert_eq!(calculate_min(&y), None);
    }

    #[test]
    fn test_calculate_max() {
        let y = vec![1.0, -2.0, 3.0, 0.0];
        assert_eq!(calculate_max(&y), Some(3.0));
    }

    #[test]
    fn test_calculate_pk_pk() {
        let y = vec![1.0, -2.0, 3.0, 0.0];
        assert_eq!(calculate_pk_pk(&y), Some(5.0));
    }

    #[test]
    fn test_calculate_mean() {
        let y = vec![2.0, 4.0, 6.0, 8.0];
        assert_eq!(calculate_mean(&y), Some(5.0));
    }

    #[test]
    fn test_calculate_mean_single() {
        let y = vec![5.0];
        assert_eq!(calculate_mean(&y), Some(5.0));
    }

    #[test]
    fn test_calculate_rms_dc() {
        // RMS of DC signal equals the DC value
        let y = vec![5.0; 100];
        assert!((calculate_rms(&y).unwrap() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_calculate_rms_sine() {
        // RMS of sine wave = amplitude / sqrt(2)
        let (_, y) = make_sine_wave(1000, 1.0, 1e-3);
        let rms = calculate_rms(&y).unwrap();
        let expected = 1.0 / 2.0_f64.sqrt();
        assert!((rms - expected).abs() < 0.01);
    }

    #[test]
    fn test_calculate_std_dev_constant() {
        let y = vec![5.0; 100];
        assert!((calculate_std_dev(&y).unwrap() - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_calculate_std_dev() {
        let y = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        // Sample std dev = sqrt(32/7) ≈ 2.138
        let std_dev = calculate_std_dev(&y).unwrap();
        assert!((std_dev - 2.138).abs() < 0.1);
    }

    // =========================================================================
    // Crossing Detection Tests
    // =========================================================================

    #[test]
    fn test_find_crossings_empty() {
        let x: Vec<f64> = vec![];
        let y: Vec<f64> = vec![];
        assert!(find_crossings(&x, &y, 0.0).is_empty());
    }

    #[test]
    fn test_find_crossings_sine() {
        let (x, y) = make_sine_wave(1000, 1.0, 1e-3);
        let crossings = find_crossings(&x, &y, 0.0);

        // Should have ~4 crossings (2 rising, 2 falling) for 2 periods
        assert!(crossings.len() >= 3);

        // Rising and falling crossings should alternate
        for i in 1..crossings.len() {
            assert_ne!(crossings[i].rising, crossings[i - 1].rising);
        }
    }

    #[test]
    fn test_interpolate_crossing() {
        let x = interpolate_crossing(0.0, 1.0, -1.0, 1.0, 0.0);
        assert!((x - 0.5).abs() < 0.001);
    }

    // =========================================================================
    // Timing Measurement Tests
    // =========================================================================

    #[test]
    fn test_calculate_period_sine() {
        let period_expected = 1e-3;
        let (x, y) = make_sine_wave(10000, 1.0, period_expected);

        let period = calculate_period(&x, &y);
        assert!(period.is_some());
        assert!((period.unwrap() - period_expected).abs() < period_expected * 0.01);
    }

    #[test]
    fn test_calculate_frequency_sine() {
        let period = 1e-3;
        let freq_expected = 1000.0;
        let (x, y) = make_sine_wave(10000, 1.0, period);

        let freq = calculate_frequency(&x, &y);
        assert!(freq.is_some());
        assert!((freq.unwrap() - freq_expected).abs() < freq_expected * 0.01);
    }

    #[test]
    fn test_calculate_duty_cycle_square() {
        let (x, y) = make_square_wave(1000, 1.0, 1e-3);

        let duty = calculate_duty_cycle(&x, &y);
        assert!(duty.is_some());
        assert!((duty.unwrap() - 50.0).abs() < 1.0); // Should be ~50%
    }

    #[test]
    fn test_calculate_rise_time() {
        // Create a signal with a known rise time
        let n = 1000;
        let rise_time_expected = 1e-6;
        let x: Vec<f64> = (0..n).map(|i| i as f64 * 1e-8).collect();
        let y: Vec<f64> = x
            .iter()
            .map(|&t| {
                if t < 1e-6 {
                    0.0
                } else if t < 1e-6 + rise_time_expected {
                    (t - 1e-6) / rise_time_expected
                } else {
                    1.0
                }
            })
            .collect();

        let rise = calculate_rise_time(&x, &y);
        assert!(rise.is_some());
        // Within 20% due to interpolation
        assert!((rise.unwrap() - rise_time_expected * 0.8).abs() < rise_time_expected * 0.5);
    }

    // =========================================================================
    // Integral Tests
    // =========================================================================

    #[test]
    fn test_calculate_integral_constant() {
        // Integral of constant 1.0 from 0 to 1 = 1.0
        let x: Vec<f64> = (0..100).map(|i| i as f64 / 100.0).collect();
        let y: Vec<f64> = vec![1.0; 100];

        let integral = calculate_integral(&x, &y);
        assert!(integral.is_some());
        assert!((integral.unwrap() - 1.0).abs() < 0.02);
    }

    #[test]
    fn test_calculate_integral_triangle() {
        // Integral of triangle from 0 to 1 with peak 1 = 0.5
        let n = 1000;
        let x: Vec<f64> = (0..n).map(|i| i as f64 / n as f64).collect();
        let y: Vec<f64> = x
            .iter()
            .map(|&t| if t < 0.5 { 2.0 * t } else { 2.0 * (1.0 - t) })
            .collect();

        let integral = calculate_integral(&x, &y);
        assert!(integral.is_some());
        assert!((integral.unwrap() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_calculate_integral_sine() {
        // Integral of sine over full period = 0
        let (x, y) = make_sine_wave(1000, 1.0, 1e-3);

        // Take exactly one period
        let period_end_idx = x.iter().position(|&t| t >= 1e-3).unwrap_or(x.len());
        let x_slice = &x[..period_end_idx];
        let y_slice = &y[..period_end_idx];

        let integral = calculate_integral(x_slice, y_slice);
        assert!(integral.is_some());
        assert!(integral.unwrap().abs() < 0.01); // Should be ~0
    }

    // =========================================================================
    // Full Measurement Tests
    // =========================================================================

    #[test]
    fn test_calculate_all_measurements() {
        let trace = TraceData::new(
            "V(out)",
            vec![0.0, 1e-6, 2e-6, 3e-6, 4e-6],
            vec![0.0, 1.0, 0.0, -1.0, 0.0],
        );

        let meas = calculate_all_measurements(&trace);

        assert_eq!(meas.trace_name, "V(out)");
        assert!(meas.min.is_some());
        assert!(meas.max.is_some());
        assert!(meas.pk_pk.is_some());
        assert_eq!(meas.pk_pk.unwrap(), 2.0);
    }

    #[test]
    fn test_calculate_measurements_in_range() {
        let trace = TraceData::new(
            "test",
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
            vec![0.0, 1.0, 2.0, 3.0, 2.0, 1.0],
        );

        // Measure in range [1.0, 3.0]
        let meas = calculate_measurements_in_range(&trace, 1.0, 3.0);

        // In this range, min=1.0, max=3.0
        assert!(meas.min.is_some());
        assert!((meas.min.unwrap() - 1.0).abs() < 0.1);
        assert!((meas.max.unwrap() - 3.0).abs() < 0.1);
    }

    #[test]
    fn test_measurement_cache_reuses_entry_for_identical_trace_and_range() {
        let trace = TraceData::new(
            "cache-hit",
            vec![0.0, 1.0, 2.0, 3.0],
            vec![0.0, 2.0, 1.0, 3.0],
        );
        let mut cache = MeasurementCache::default();

        let first_ptr = cache.get_or_compute(0, &trace, None) as *const TraceMeasurements;
        let second_ptr = cache.get_or_compute(0, &trace, None) as *const TraceMeasurements;

        assert_eq!(first_ptr, second_ptr);
        assert_eq!(cache.entries.len(), 1);
    }

    #[test]
    fn test_measurement_cache_refreshes_when_range_changes() {
        let trace = TraceData::new(
            "cache-range",
            vec![0.0, 1.0, 2.0, 3.0, 4.0],
            vec![0.0, 1.0, 2.0, 3.0, 4.0],
        );
        let mut cache = MeasurementCache::default();

        let all_max = cache.get_or_compute(0, &trace, None).max;
        let range_max = cache.get_or_compute(0, &trace, Some((0.0, 2.0))).max;

        assert_eq!(all_max, Some(4.0));
        assert_eq!(range_max, Some(3.0));
    }

    #[test]
    fn test_measurement_cache_refreshes_when_trace_signature_changes() {
        let trace_a = TraceData::new("sig", vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 2.0]);
        let trace_b = TraceData::new("sig", vec![0.0, 1.0, 2.0], vec![0.0, 10.0, 20.0]);
        let mut cache = MeasurementCache::default();

        let first = cache.get_or_compute(0, &trace_a, None).max;
        let second = cache.get_or_compute(0, &trace_b, None).max;

        assert_eq!(first, Some(2.0));
        assert_eq!(second, Some(20.0));
    }
}
