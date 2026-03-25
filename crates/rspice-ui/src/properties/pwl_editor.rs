//! PWL (Piecewise Linear) Editor Module
//!
//! Commercial-grade graphical editor for PWL time-value pairs.
//! Matches Cadence Spectre's PWL source editing capabilities.
//!
//! # Architecture
//!
//! - `PwlPoint`: Single time-value pair with engineering notation support
//! - `PwlData`: Collection of points with parsing, validation, and serialization
//! - `PwlEditorState`: UI state for table-based editing
//! - `render_pwl_editor`: egui widget for interactive editing
//!
//! # SPICE PWL Format
//!
//! Standard format: `PWL(t1 v1 t2 v2 t3 v3 ...)`
//! Internal string format: `0 0 1n 1 2n 0 3n 1` (space-separated pairs)

use crate::properties::{format_engineering_value, parse_engineering_value};
use egui::{Color32, RichText, Ui};
use serde::{Deserialize, Serialize};
use std::fmt;

// =============================================================================
// PWL Point
// =============================================================================

/// A single time-value point in a PWL waveform.
///
/// Represents one vertex of the piecewise linear function.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PwlPoint {
    /// Time coordinate in seconds
    pub time: f64,
    /// Value (voltage or current) at this time
    pub value: f64,
}

impl PwlPoint {
    /// Create a new PWL point.
    pub fn new(time: f64, value: f64) -> Self {
        Self { time, value }
    }

    /// Create origin point (0, 0).
    pub fn origin() -> Self {
        Self::new(0.0, 0.0)
    }

    /// Format time with engineering notation.
    pub fn time_string(&self) -> String {
        format!("{} s", format_engineering_value(self.time))
    }

    /// Format value with engineering notation and unit.
    pub fn value_string(&self, unit: &str) -> String {
        format!("{} {}", format_engineering_value(self.value), unit)
    }

    /// Validate that time is non-negative.
    pub fn validate(&self) -> Result<(), PwlValidationError> {
        if self.time < 0.0 {
            return Err(PwlValidationError::NegativeTime(self.time));
        }
        if !self.time.is_finite() {
            return Err(PwlValidationError::InvalidTime(self.time));
        }
        if !self.value.is_finite() {
            return Err(PwlValidationError::InvalidValue(self.value));
        }
        Ok(())
    }
}

impl Default for PwlPoint {
    fn default() -> Self {
        Self::origin()
    }
}

// =============================================================================
// PWL Validation Error
// =============================================================================

/// Errors that can occur during PWL data validation.
#[derive(Debug, Clone, PartialEq)]
pub enum PwlValidationError {
    /// Time value is negative.
    NegativeTime(f64),
    /// Time value is not finite (NaN or Inf).
    InvalidTime(f64),
    /// Value is not finite (NaN or Inf).
    InvalidValue(f64),
    /// Time values are not monotonically increasing.
    NonMonotonicTime { index: usize, prev: f64, curr: f64 },
    /// Duplicate time value detected.
    DuplicateTime { index: usize, time: f64 },
    /// Parse error for time string.
    TimeParseError { index: usize, text: String },
    /// Parse error for value string.
    ValueParseError { index: usize, text: String },
    /// Empty data when at least one point is required.
    EmptyData,
}

impl fmt::Display for PwlValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NegativeTime(t) => write!(f, "Time cannot be negative: {}", t),
            Self::InvalidTime(t) => write!(f, "Invalid time value: {}", t),
            Self::InvalidValue(v) => write!(f, "Invalid value: {}", v),
            Self::NonMonotonicTime { index, prev, curr } => {
                write!(
                    f,
                    "Time must be strictly increasing: point {} has t={} after t={}",
                    index, curr, prev
                )
            }
            Self::DuplicateTime { index, time } => {
                write!(f, "Duplicate time at point {}: t={}", index, time)
            }
            Self::TimeParseError { index, text } => {
                write!(f, "Cannot parse time at point {}: '{}'", index, text)
            }
            Self::ValueParseError { index, text } => {
                write!(f, "Cannot parse value at point {}: '{}'", index, text)
            }
            Self::EmptyData => write!(f, "PWL data cannot be empty"),
        }
    }
}

impl std::error::Error for PwlValidationError {}

// =============================================================================
// PWL Data
// =============================================================================

/// Collection of PWL points with parsing, validation, and serialization.
///
/// Maintains points in time-sorted order and validates monotonicity.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PwlData {
    /// Time-value points (sorted by time).
    points: Vec<PwlPoint>,
    /// Whether to repeat the waveform.
    pub repeat: bool,
    /// Time delay before waveform starts.
    pub delay: f64,
}

impl PwlData {
    /// Create empty PWL data.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create PWL data with initial points.
    pub fn with_points(points: Vec<PwlPoint>) -> Self {
        let mut data = Self {
            points,
            repeat: false,
            delay: 0.0,
        };
        data.sort_by_time();
        data
    }

    /// Parse PWL data from space-separated string format.
    ///
    /// Format: "t1 v1 t2 v2 t3 v3 ..."
    ///
    /// Supports engineering notation (e.g., "0 0 1n 1 2n 0")
    pub fn parse(s: &str) -> Result<Self, PwlValidationError> {
        let s = s.trim();
        if s.is_empty() {
            return Ok(Self::new());
        }

        let tokens: Vec<&str> = s.split_whitespace().collect();

        // Must have even number of tokens (time-value pairs)
        if !tokens.len().is_multiple_of(2) {
            return Err(PwlValidationError::ValueParseError {
                index: tokens.len() / 2,
                text: "Odd number of values - expected time-value pairs".to_string(),
            });
        }

        let mut points = Vec::with_capacity(tokens.len() / 2);

        for (i, chunk) in tokens.chunks(2).enumerate() {
            let time = parse_engineering_value(chunk[0]).map_err(|_| {
                PwlValidationError::TimeParseError {
                    index: i,
                    text: chunk[0].to_string(),
                }
            })?;

            let value = parse_engineering_value(chunk[1]).map_err(|_| {
                PwlValidationError::ValueParseError {
                    index: i,
                    text: chunk[1].to_string(),
                }
            })?;

            points.push(PwlPoint::new(time, value));
        }

        let data = Self::with_points(points);
        data.validate()?;
        Ok(data)
    }

    /// Serialize to space-separated string format.
    ///
    /// Uses engineering notation for compact representation.
    pub fn serialize(&self) -> String {
        self.points
            .iter()
            .map(|p| {
                format!(
                    "{} {}",
                    format_engineering_for_spice(p.time),
                    format_engineering_for_spice(p.value)
                )
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Validate the PWL data.
    ///
    /// Checks:
    /// - All times are non-negative and finite
    /// - All values are finite
    /// - Times are strictly monotonically increasing
    pub fn validate(&self) -> Result<(), PwlValidationError> {
        // Validate individual points
        for point in &self.points {
            point.validate()?;
        }

        // Check monotonicity
        for i in 1..self.points.len() {
            let prev = self.points[i - 1].time;
            let curr = self.points[i].time;

            if curr < prev {
                return Err(PwlValidationError::NonMonotonicTime {
                    index: i,
                    prev,
                    curr,
                });
            }
            if (curr - prev).abs() < 1e-18 {
                return Err(PwlValidationError::DuplicateTime {
                    index: i,
                    time: curr,
                });
            }
        }

        Ok(())
    }

    /// Get the number of points.
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Get points slice.
    pub fn points(&self) -> &[PwlPoint] {
        &self.points
    }

    /// Get mutable points slice.
    pub fn points_mut(&mut self) -> &mut Vec<PwlPoint> {
        &mut self.points
    }

    /// Add a new point (will be sorted by time).
    pub fn add_point(&mut self, point: PwlPoint) {
        self.points.push(point);
        self.sort_by_time();
    }

    /// Insert a point at the given index.
    pub fn insert_point(&mut self, index: usize, point: PwlPoint) {
        if index <= self.points.len() {
            self.points.insert(index, point);
        }
    }

    /// Remove a point at the given index.
    pub fn remove_point(&mut self, index: usize) -> Option<PwlPoint> {
        if index < self.points.len() {
            Some(self.points.remove(index))
        } else {
            None
        }
    }

    /// Update a point at the given index.
    pub fn update_point(&mut self, index: usize, point: PwlPoint) {
        if index < self.points.len() {
            self.points[index] = point;
        }
    }

    /// Sort points by time.
    pub fn sort_by_time(&mut self) {
        self.points.sort_by(|a, b| {
            a.time
                .partial_cmp(&b.time)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// Clear all points.
    pub fn clear(&mut self) {
        self.points.clear();
    }

    /// Get time range (min, max).
    pub fn time_range(&self) -> Option<(f64, f64)> {
        if self.points.is_empty() {
            return None;
        }
        let min = self.points.first().map(|p| p.time).unwrap_or(0.0);
        let max = self.points.last().map(|p| p.time).unwrap_or(0.0);
        Some((min, max))
    }

    /// Get value range (min, max).
    pub fn value_range(&self) -> Option<(f64, f64)> {
        if self.points.is_empty() {
            return None;
        }
        let min = self
            .points
            .iter()
            .map(|p| p.value)
            .fold(f64::INFINITY, f64::min);
        let max = self
            .points
            .iter()
            .map(|p| p.value)
            .fold(f64::NEG_INFINITY, f64::max);
        Some((min, max))
    }

    /// Interpolate value at a given time.
    pub fn interpolate(&self, t: f64) -> Option<f64> {
        if self.points.is_empty() {
            return None;
        }

        // Handle delay
        let t = t - self.delay;
        if t < 0.0 {
            return Some(self.points.first()?.value);
        }

        // Handle repeat
        let t = if self.repeat {
            if let Some((_, max)) = self.time_range() {
                if max > 0.0 { t % max } else { t }
            } else {
                t
            }
        } else {
            t
        };

        // Before first point
        if t <= self.points[0].time {
            return Some(self.points[0].value);
        }

        // After last point
        if t >= self.points.last()?.time {
            return Some(self.points.last()?.value);
        }

        // Find bracketing points and interpolate
        for i in 1..self.points.len() {
            if t <= self.points[i].time {
                let p0 = &self.points[i - 1];
                let p1 = &self.points[i];
                let dt = p1.time - p0.time;
                if dt.abs() < 1e-18 {
                    return Some(p0.value);
                }
                let alpha = (t - p0.time) / dt;
                return Some(p0.value + alpha * (p1.value - p0.value));
            }
        }

        Some(self.points.last()?.value)
    }

    /// Generate standard pulse waveform.
    pub fn pulse(v_low: f64, v_high: f64, period: f64, duty: f64, rise: f64, fall: f64) -> Self {
        let pw = period * duty;
        let points = vec![
            PwlPoint::new(0.0, v_low),
            PwlPoint::new(rise, v_high),
            PwlPoint::new(rise + pw, v_high),
            PwlPoint::new(rise + pw + fall, v_low),
            PwlPoint::new(period, v_low),
        ];
        Self {
            points,
            repeat: true,
            delay: 0.0,
        }
    }

    /// Generate ramp waveform.
    pub fn ramp(v_start: f64, v_end: f64, t_rise: f64) -> Self {
        Self::with_points(vec![
            PwlPoint::new(0.0, v_start),
            PwlPoint::new(t_rise, v_end),
        ])
    }
}

/// Format a value with engineering notation for SPICE compatibility.
fn format_engineering_for_spice(value: f64) -> String {
    let abs_value = value.abs();

    if abs_value == 0.0 {
        return "0".to_string();
    }

    let (scaled, suffix) = if abs_value >= 1e12 {
        (value / 1e12, "T")
    } else if abs_value >= 1e9 {
        (value / 1e9, "G")
    } else if abs_value >= 1e6 {
        (value / 1e6, "Meg")
    } else if abs_value >= 1e3 {
        (value / 1e3, "k")
    } else if abs_value >= 1.0 {
        (value, "")
    } else if abs_value >= 1e-3 {
        (value * 1e3, "m")
    } else if abs_value >= 1e-6 {
        (value * 1e6, "u")
    } else if abs_value >= 1e-9 {
        (value * 1e9, "n")
    } else if abs_value >= 1e-12 {
        (value * 1e12, "p")
    } else if abs_value >= 1e-15 {
        (value * 1e15, "f")
    } else {
        (value * 1e18, "a")
    };

    // Format with appropriate precision
    let eps = 1e-9;
    let is_int = (scaled.round() - scaled).abs() < eps;

    if is_int {
        format!("{:.0}{}", scaled.round(), suffix)
    } else {
        // Trim trailing zeros
        let formatted = format!("{:.6}", scaled);
        let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
        format!("{}{}", trimmed, suffix)
    }
}

// =============================================================================
// PWL Editor State
// =============================================================================

/// State for the PWL editor UI widget.
#[derive(Debug, Clone, Default)]
pub struct PwlEditorState {
    /// The PWL data being edited.
    pub data: PwlData,
    /// Selected row index (if any).
    pub selected_row: Option<usize>,
    /// Text buffers for editing (time, value).
    pub edit_buffers: Vec<(String, String)>,
    /// Validation error for the current data.
    pub validation_error: Option<String>,
    /// Whether the editor is in "add new point" mode.
    pub adding_point: bool,
    /// Buffer for new point time.
    pub new_time: String,
    /// Buffer for new point value.
    pub new_value: String,
    /// Unit for values (V or A).
    pub value_unit: String,
    /// Whether the data has been modified since opening.
    pub is_modified: bool,
}

impl PwlEditorState {
    /// Create new editor state.
    pub fn new() -> Self {
        Self {
            value_unit: "V".to_string(),
            ..Default::default()
        }
    }

    /// Initialize from PWL data string.
    pub fn from_string(s: &str, value_unit: &str) -> Self {
        let data = PwlData::parse(s).unwrap_or_default();
        let edit_buffers = data
            .points()
            .iter()
            .map(|p| {
                (
                    format_engineering_for_spice(p.time),
                    format_engineering_for_spice(p.value),
                )
            })
            .collect();

        Self {
            data,
            edit_buffers,
            value_unit: value_unit.to_string(),
            ..Default::default()
        }
    }

    /// Sync edit buffers from data.
    pub fn sync_buffers_from_data(&mut self) {
        self.edit_buffers = self
            .data
            .points()
            .iter()
            .map(|p| {
                (
                    format_engineering_for_spice(p.time),
                    format_engineering_for_spice(p.value),
                )
            })
            .collect();
    }

    /// Sync data from edit buffers.
    pub fn sync_data_from_buffers(&mut self) -> Result<(), PwlValidationError> {
        let mut new_points = Vec::with_capacity(self.edit_buffers.len());

        for (i, (time_str, value_str)) in self.edit_buffers.iter().enumerate() {
            let time = parse_engineering_value(time_str).map_err(|_| {
                PwlValidationError::TimeParseError {
                    index: i,
                    text: time_str.clone(),
                }
            })?;

            let value = parse_engineering_value(value_str).map_err(|_| {
                PwlValidationError::ValueParseError {
                    index: i,
                    text: value_str.clone(),
                }
            })?;

            new_points.push(PwlPoint::new(time, value));
        }

        self.data = PwlData::with_points(new_points);
        self.data.validate()?;
        self.validation_error = None;
        Ok(())
    }

    /// Add a new point.
    pub fn add_point(&mut self) {
        if self.new_time.is_empty() {
            self.new_time = "0".to_string();
        }
        if self.new_value.is_empty() {
            self.new_value = "0".to_string();
        }

        let time = parse_engineering_value(&self.new_time).unwrap_or(0.0);
        let value = parse_engineering_value(&self.new_value).unwrap_or(0.0);

        self.data.add_point(PwlPoint::new(time, value));
        self.sync_buffers_from_data();
        self.new_time.clear();
        self.new_value.clear();
        self.adding_point = false;
    }

    /// Delete the selected point.
    pub fn delete_selected(&mut self) {
        if let Some(idx) = self.selected_row {
            self.data.remove_point(idx);
            self.sync_buffers_from_data();
            self.selected_row = None;
        }
    }

    /// Check if data has been modified.
    pub fn is_valid(&self) -> bool {
        self.validation_error.is_none() && self.data.validate().is_ok()
    }
}

impl fmt::Display for PwlEditorState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data.serialize())
    }
}

// =============================================================================
// PWL Editor Rendering
// =============================================================================

/// Result of PWL editor interaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PwlEditorResult {
    /// No change.
    None,
    /// Data was modified.
    Modified,
}

/// Render the PWL editor widget.
///
/// Returns `PwlEditorResult::Modified` if the data changed.
pub fn render_pwl_editor(ui: &mut Ui, state: &mut PwlEditorState) -> PwlEditorResult {
    let mut result = PwlEditorResult::None;

    ui.vertical(|ui| {
        // Header with add button
        ui.horizontal(|ui| {
            ui.label(RichText::new("PWL Points").strong());
            ui.separator();
            if ui.button("➕ Add Point").clicked() {
                state.adding_point = true;
            }
            if state.selected_row.is_some() && ui.button("🗑 Delete").clicked() {
                state.delete_selected();
                result = PwlEditorResult::Modified;
            }
        });

        ui.separator();

        // Add new point row
        if state.adding_point {
            ui.horizontal(|ui| {
                ui.label("Time:");
                ui.add(egui::TextEdit::singleline(&mut state.new_time).desired_width(80.0));
                ui.label("s");
                ui.separator();
                ui.label("Value:");
                ui.add(egui::TextEdit::singleline(&mut state.new_value).desired_width(80.0));
                ui.label(&state.value_unit);

                if ui.button("✓ Add").clicked() {
                    state.add_point();
                    result = PwlEditorResult::Modified;
                }
                if ui.button("✗ Cancel").clicked() {
                    state.adding_point = false;
                    state.new_time.clear();
                    state.new_value.clear();
                }
            });
            ui.separator();
        }

        // Table header
        ui.horizontal(|ui| {
            ui.label(RichText::new("#").monospace().weak());
            ui.add_space(20.0);
            ui.label(RichText::new("Time (s)").monospace());
            ui.add_space(60.0);
            ui.label(RichText::new(format!("Value ({})", state.value_unit)).monospace());
        });

        ui.separator();

        // Points table
        egui::ScrollArea::vertical()
            .max_height(200.0)
            .show(ui, |ui| {
                let mut data_changed = false;

                for i in 0..state.edit_buffers.len() {
                    let is_selected = state.selected_row == Some(i);
                    let row_response = ui.horizontal(|ui| {
                        // Row number
                        let num_text = RichText::new(format!("{}", i + 1)).monospace();
                        if is_selected {
                            ui.label(num_text.color(Color32::YELLOW));
                        } else {
                            ui.label(num_text.weak());
                        }

                        ui.add_space(10.0);

                        // Time field
                        let time_response = ui.add(
                            egui::TextEdit::singleline(&mut state.edit_buffers[i].0)
                                .desired_width(80.0)
                                .font(egui::TextStyle::Monospace),
                        );
                        if time_response.changed() {
                            data_changed = true;
                        }

                        ui.add_space(10.0);

                        // Value field
                        let value_response = ui.add(
                            egui::TextEdit::singleline(&mut state.edit_buffers[i].1)
                                .desired_width(80.0)
                                .font(egui::TextStyle::Monospace),
                        );
                        if value_response.changed() {
                            data_changed = true;
                        }
                    });

                    // Handle row selection
                    if row_response.response.clicked() {
                        state.selected_row = Some(i);
                    }
                }

                if data_changed {
                    match state.sync_data_from_buffers() {
                        Ok(()) => {
                            state.validation_error = None;
                            result = PwlEditorResult::Modified;
                        }
                        Err(e) => {
                            state.validation_error = Some(e.to_string());
                        }
                    }
                }
            });

        // Validation error display
        if let Some(error) = &state.validation_error {
            ui.separator();
            ui.colored_label(Color32::RED, format!("⚠ {}", error));
        }

        // Summary
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("{} points", state.data.len())).weak());
            if let Some((t_min, t_max)) = state.data.time_range() {
                ui.separator();
                ui.label(
                    RichText::new(format!(
                        "Time: {} → {}",
                        format_engineering_for_spice(t_min),
                        format_engineering_for_spice(t_max)
                    ))
                    .weak(),
                );
            }
            if let Some((v_min, v_max)) = state.data.value_range() {
                ui.separator();
                ui.label(
                    RichText::new(format!(
                        "Value: {} → {} {}",
                        format_engineering_for_spice(v_min),
                        format_engineering_for_spice(v_max),
                        state.value_unit
                    ))
                    .weak(),
                );
            }
        });
    });

    result
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // PwlPoint Tests
    // =========================================================================

    #[test]
    fn test_pwl_point_new() {
        let p = PwlPoint::new(1e-9, 2.5);
        assert_eq!(p.time, 1e-9);
        assert_eq!(p.value, 2.5);
    }

    #[test]
    fn test_pwl_point_origin() {
        let p = PwlPoint::origin();
        assert_eq!(p.time, 0.0);
        assert_eq!(p.value, 0.0);
    }

    #[test]
    fn test_pwl_point_default() {
        let p = PwlPoint::default();
        assert_eq!(p, PwlPoint::origin());
    }

    #[test]
    fn test_pwl_point_validate_ok() {
        let p = PwlPoint::new(1e-9, 2.5);
        assert!(p.validate().is_ok());
    }

    #[test]
    fn test_pwl_point_validate_negative_time() {
        let p = PwlPoint::new(-1e-9, 2.5);
        let err = p.validate().unwrap_err();
        assert!(matches!(err, PwlValidationError::NegativeTime(_)));
    }

    #[test]
    fn test_pwl_point_validate_nan_time() {
        let p = PwlPoint::new(f64::NAN, 2.5);
        let err = p.validate().unwrap_err();
        assert!(matches!(err, PwlValidationError::InvalidTime(_)));
    }

    #[test]
    fn test_pwl_point_validate_nan_value() {
        let p = PwlPoint::new(1e-9, f64::NAN);
        let err = p.validate().unwrap_err();
        assert!(matches!(err, PwlValidationError::InvalidValue(_)));
    }

    #[test]
    fn test_pwl_point_validate_inf_time() {
        let p = PwlPoint::new(f64::INFINITY, 2.5);
        let err = p.validate().unwrap_err();
        assert!(matches!(err, PwlValidationError::InvalidTime(_)));
    }

    // =========================================================================
    // PwlData Parse Tests
    // =========================================================================

    #[test]
    fn test_pwl_data_parse_empty() {
        let data = PwlData::parse("").unwrap();
        assert!(data.is_empty());
    }

    #[test]
    fn test_pwl_data_parse_whitespace_only() {
        let data = PwlData::parse("   \t  \n  ").unwrap();
        assert!(data.is_empty());
    }

    #[test]
    fn test_pwl_data_parse_single_point() {
        let data = PwlData::parse("0 1").unwrap();
        assert_eq!(data.len(), 1);
        assert_eq!(data.points()[0].time, 0.0);
        assert_eq!(data.points()[0].value, 1.0);
    }

    #[test]
    fn test_pwl_data_parse_multiple_points() {
        let data = PwlData::parse("0 0 1 1 2 0").unwrap();
        assert_eq!(data.len(), 3);
        assert_eq!(data.points()[0], PwlPoint::new(0.0, 0.0));
        assert_eq!(data.points()[1], PwlPoint::new(1.0, 1.0));
        assert_eq!(data.points()[2], PwlPoint::new(2.0, 0.0));
    }

    #[test]
    fn test_pwl_data_parse_engineering_notation() {
        let data = PwlData::parse("0 0 1n 1 2n 0 3n 1").unwrap();
        assert_eq!(data.len(), 4);
        assert!((data.points()[1].time - 1e-9).abs() < 1e-18);
        assert!((data.points()[2].time - 2e-9).abs() < 1e-18);
        assert!((data.points()[3].time - 3e-9).abs() < 1e-18);
    }

    #[test]
    fn test_pwl_data_parse_negative_values() {
        let data = PwlData::parse("0 -1 1 1 2 -0.5").unwrap();
        assert_eq!(data.len(), 3);
        assert_eq!(data.points()[0].value, -1.0);
        assert_eq!(data.points()[2].value, -0.5);
    }

    #[test]
    fn test_pwl_data_parse_odd_tokens_error() {
        let result = PwlData::parse("0 1 2");
        assert!(result.is_err());
    }

    #[test]
    fn test_pwl_data_parse_invalid_time_error() {
        let result = PwlData::parse("abc 1 2n 0");
        assert!(result.is_err());
        if let Err(PwlValidationError::TimeParseError { index, .. }) = result {
            assert_eq!(index, 0);
        } else {
            panic!("Expected TimeParseError");
        }
    }

    #[test]
    fn test_pwl_data_parse_invalid_value_error() {
        let result = PwlData::parse("0 xyz 2n 0");
        assert!(result.is_err());
        if let Err(PwlValidationError::ValueParseError { index, .. }) = result {
            assert_eq!(index, 0);
        } else {
            panic!("Expected ValueParseError");
        }
    }

    // =========================================================================
    // PwlData Serialize Tests
    // =========================================================================

    #[test]
    fn test_pwl_data_serialize_empty() {
        let data = PwlData::new();
        assert_eq!(data.serialize(), "");
    }

    #[test]
    fn test_pwl_data_serialize_single_point() {
        let data = PwlData::with_points(vec![PwlPoint::new(0.0, 1.0)]);
        assert_eq!(data.serialize(), "0 1");
    }

    #[test]
    fn test_pwl_data_serialize_multiple_points() {
        let data = PwlData::with_points(vec![
            PwlPoint::new(0.0, 0.0),
            PwlPoint::new(1e-9, 1.0),
            PwlPoint::new(2e-9, 0.0),
        ]);
        let s = data.serialize();
        assert!(s.contains("1n"));
        assert!(s.contains("2n"));
    }

    #[test]
    fn test_pwl_data_round_trip() {
        let original = "0 0 1n 1 2n 0 3n 1";
        let data = PwlData::parse(original).unwrap();
        let serialized = data.serialize();
        let reparsed = PwlData::parse(&serialized).unwrap();

        assert_eq!(data.len(), reparsed.len());
        for (a, b) in data.points().iter().zip(reparsed.points().iter()) {
            assert!((a.time - b.time).abs() < 1e-18);
            assert!((a.value - b.value).abs() < 1e-9);
        }
    }

    // =========================================================================
    // PwlData Validation Tests
    // =========================================================================

    #[test]
    fn test_pwl_data_validate_ok() {
        let data = PwlData::with_points(vec![
            PwlPoint::new(0.0, 0.0),
            PwlPoint::new(1e-9, 1.0),
            PwlPoint::new(2e-9, 0.0),
        ]);
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_pwl_data_validate_non_monotonic() {
        let data = PwlData {
            points: vec![
                PwlPoint::new(0.0, 0.0),
                PwlPoint::new(2e-9, 1.0),
                PwlPoint::new(1e-9, 0.0), // Out of order
            ],
            repeat: false,
            delay: 0.0,
        };
        // Note: with_points sorts, so we use direct construction
        let err = data.validate().unwrap_err();
        assert!(matches!(err, PwlValidationError::NonMonotonicTime { .. }));
    }

    #[test]
    fn test_pwl_data_validate_duplicate_time() {
        let data = PwlData {
            points: vec![
                PwlPoint::new(0.0, 0.0),
                PwlPoint::new(1e-9, 1.0),
                PwlPoint::new(1e-9, 0.5), // Duplicate time
            ],
            repeat: false,
            delay: 0.0,
        };
        let err = data.validate().unwrap_err();
        assert!(matches!(err, PwlValidationError::DuplicateTime { .. }));
    }

    #[test]
    fn test_pwl_data_validate_negative_time() {
        let data = PwlData {
            points: vec![PwlPoint::new(-1e-9, 0.0), PwlPoint::new(1e-9, 1.0)],
            repeat: false,
            delay: 0.0,
        };
        let err = data.validate().unwrap_err();
        assert!(matches!(err, PwlValidationError::NegativeTime(_)));
    }

    // =========================================================================
    // PwlData Modification Tests
    // =========================================================================

    #[test]
    fn test_pwl_data_add_point() {
        let mut data = PwlData::new();
        data.add_point(PwlPoint::new(1e-9, 1.0));
        data.add_point(PwlPoint::new(0.0, 0.0));

        // Should be sorted by time
        assert_eq!(data.len(), 2);
        assert_eq!(data.points()[0].time, 0.0);
        assert_eq!(data.points()[1].time, 1e-9);
    }

    #[test]
    fn test_pwl_data_remove_point() {
        let mut data = PwlData::with_points(vec![
            PwlPoint::new(0.0, 0.0),
            PwlPoint::new(1e-9, 1.0),
            PwlPoint::new(2e-9, 0.0),
        ]);

        let removed = data.remove_point(1).unwrap();
        assert_eq!(removed.time, 1e-9);
        assert_eq!(data.len(), 2);
    }

    #[test]
    fn test_pwl_data_update_point() {
        let mut data =
            PwlData::with_points(vec![PwlPoint::new(0.0, 0.0), PwlPoint::new(1e-9, 1.0)]);

        data.update_point(1, PwlPoint::new(1e-9, 2.0));
        assert_eq!(data.points()[1].value, 2.0);
    }

    #[test]
    fn test_pwl_data_clear() {
        let mut data =
            PwlData::with_points(vec![PwlPoint::new(0.0, 0.0), PwlPoint::new(1e-9, 1.0)]);

        data.clear();
        assert!(data.is_empty());
    }

    // =========================================================================
    // PwlData Range Tests
    // =========================================================================

    #[test]
    fn test_pwl_data_time_range() {
        let data = PwlData::with_points(vec![
            PwlPoint::new(0.0, 0.0),
            PwlPoint::new(1e-9, 1.0),
            PwlPoint::new(5e-9, 0.0),
        ]);

        let (min, max) = data.time_range().unwrap();
        assert_eq!(min, 0.0);
        assert_eq!(max, 5e-9);
    }

    #[test]
    fn test_pwl_data_time_range_empty() {
        let data = PwlData::new();
        assert!(data.time_range().is_none());
    }

    #[test]
    fn test_pwl_data_value_range() {
        let data = PwlData::with_points(vec![
            PwlPoint::new(0.0, -1.0),
            PwlPoint::new(1e-9, 2.0),
            PwlPoint::new(2e-9, 0.5),
        ]);

        let (min, max) = data.value_range().unwrap();
        assert_eq!(min, -1.0);
        assert_eq!(max, 2.0);
    }

    // =========================================================================
    // PwlData Interpolation Tests
    // =========================================================================

    #[test]
    fn test_pwl_data_interpolate_exact_point() {
        let data = PwlData::with_points(vec![
            PwlPoint::new(0.0, 0.0),
            PwlPoint::new(1e-9, 1.0),
            PwlPoint::new(2e-9, 0.0),
        ]);

        assert!((data.interpolate(0.0).unwrap() - 0.0).abs() < 1e-9);
        assert!((data.interpolate(1e-9).unwrap() - 1.0).abs() < 1e-9);
        assert!((data.interpolate(2e-9).unwrap() - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_pwl_data_interpolate_midpoint() {
        let data = PwlData::with_points(vec![PwlPoint::new(0.0, 0.0), PwlPoint::new(2e-9, 2.0)]);

        let v = data.interpolate(1e-9).unwrap();
        assert!((v - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_pwl_data_interpolate_before_first() {
        let data = PwlData::with_points(vec![PwlPoint::new(1e-9, 1.0), PwlPoint::new(2e-9, 0.0)]);

        // Before first point returns first value
        assert!((data.interpolate(0.0).unwrap() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_pwl_data_interpolate_after_last() {
        let data = PwlData::with_points(vec![PwlPoint::new(0.0, 0.0), PwlPoint::new(1e-9, 1.0)]);

        // After last point returns last value
        assert!((data.interpolate(5e-9).unwrap() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_pwl_data_interpolate_empty() {
        let data = PwlData::new();
        assert!(data.interpolate(1e-9).is_none());
    }

    #[test]
    fn test_pwl_data_interpolate_with_delay() {
        let mut data =
            PwlData::with_points(vec![PwlPoint::new(0.0, 0.0), PwlPoint::new(1e-9, 1.0)]);
        data.delay = 1e-9;

        // Before delay, returns first value
        assert!((data.interpolate(0.0).unwrap() - 0.0).abs() < 1e-9);
        // At delay start
        assert!((data.interpolate(1e-9).unwrap() - 0.0).abs() < 1e-9);
        // After delay
        assert!((data.interpolate(2e-9).unwrap() - 1.0).abs() < 1e-9);
    }

    // =========================================================================
    // PwlData Generator Tests
    // =========================================================================

    #[test]
    fn test_pwl_data_pulse() {
        let data = PwlData::pulse(0.0, 1.0, 10e-9, 0.5, 1e-9, 1e-9);
        assert_eq!(data.len(), 5);
        assert!(data.repeat);
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_pwl_data_ramp() {
        let data = PwlData::ramp(0.0, 3.3, 100e-9);
        assert_eq!(data.len(), 2);
        assert!((data.points()[0].value - 0.0).abs() < 1e-9);
        assert!((data.points()[1].value - 3.3).abs() < 1e-9);
    }

    // =========================================================================
    // Format Engineering Tests
    // =========================================================================

    #[test]
    fn test_format_engineering_zero() {
        assert_eq!(format_engineering_for_spice(0.0), "0");
    }

    #[test]
    fn test_format_engineering_nano() {
        assert_eq!(format_engineering_for_spice(1e-9), "1n");
        assert_eq!(format_engineering_for_spice(2.5e-9), "2.5n");
    }

    #[test]
    fn test_format_engineering_micro() {
        assert_eq!(format_engineering_for_spice(1e-6), "1u");
    }

    #[test]
    fn test_format_engineering_milli() {
        assert_eq!(format_engineering_for_spice(1e-3), "1m");
    }

    #[test]
    fn test_format_engineering_kilo() {
        assert_eq!(format_engineering_for_spice(1e3), "1k");
    }

    #[test]
    fn test_format_engineering_mega() {
        assert_eq!(format_engineering_for_spice(1e6), "1Meg");
    }

    #[test]
    fn test_format_engineering_negative() {
        assert_eq!(format_engineering_for_spice(-1e-9), "-1n");
    }

    // =========================================================================
    // PwlEditorState Tests
    // =========================================================================

    #[test]
    fn test_editor_state_new() {
        let state = PwlEditorState::new();
        assert!(state.data.is_empty());
        assert_eq!(state.value_unit, "V");
        assert!(state.selected_row.is_none());
    }

    #[test]
    fn test_editor_state_from_string() {
        let state = PwlEditorState::from_string("0 0 1n 1 2n 0", "V");
        assert_eq!(state.data.len(), 3);
        assert_eq!(state.edit_buffers.len(), 3);
        assert_eq!(state.value_unit, "V");
    }

    #[test]
    fn test_editor_state_to_string() {
        let state = PwlEditorState::from_string("0 0 1n 1 2n 0", "A");
        let s = state.to_string();
        assert!(s.contains("1n"));
        assert!(s.contains("2n"));
    }

    #[test]
    fn test_editor_state_add_point() {
        let mut state = PwlEditorState::new();
        state.new_time = "5n".to_string();
        state.new_value = "2.5".to_string();
        state.add_point();

        assert_eq!(state.data.len(), 1);
        assert!((state.data.points()[0].time - 5e-9).abs() < 1e-18);
        assert!((state.data.points()[0].value - 2.5).abs() < 1e-9);
    }

    #[test]
    fn test_editor_state_delete_selected() {
        let mut state = PwlEditorState::from_string("0 0 1n 1 2n 0", "V");
        state.selected_row = Some(1);
        state.delete_selected();

        assert_eq!(state.data.len(), 2);
        assert!(state.selected_row.is_none());
    }

    #[test]
    fn test_editor_state_sync_buffers() {
        let mut state = PwlEditorState::from_string("0 0 1n 1", "V");
        state.data.add_point(PwlPoint::new(2e-9, 0.5));
        state.sync_buffers_from_data();

        assert_eq!(state.edit_buffers.len(), 3);
    }

    #[test]
    fn test_editor_state_sync_data_from_buffers() {
        let mut state = PwlEditorState::from_string("0 0 1n 1", "V");
        state.edit_buffers[1].1 = "2".to_string();
        state.sync_data_from_buffers().unwrap();

        assert!((state.data.points()[1].value - 2.0).abs() < 1e-9);
    }

    #[test]
    fn test_editor_state_is_valid() {
        let state = PwlEditorState::from_string("0 0 1n 1 2n 0", "V");
        assert!(state.is_valid());
    }

    #[test]
    fn test_editor_state_is_valid_with_error() {
        let mut state = PwlEditorState::new();
        state.validation_error = Some("Test error".to_string());
        assert!(!state.is_valid());
    }
}
