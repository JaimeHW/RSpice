use super::basic::{calculate_max, calculate_mean, calculate_min};
use super::timing::{calculate_period, calculate_rise_time};
use crate::waveform::state::TraceData;

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
