//! Waveform Viewer State Management
//!
//! This module defines all state types for the waveform viewer, following
//! commercial EDA patterns for centralized, observable, serializable state.
//!
//! # State Hierarchy
//!
//! - `WaveformViewerState`: Top-level container for all viewer state
//! - `ViewTransform`: Coordinate space and zoom/pan state
//! - `TraceData`: Individual waveform data and display properties
//! - `CursorState`: Measurement cursor positions and mode
//! - `BoxSelection`: Region selection for box zoom

use serde::{Deserialize, Serialize};

// =============================================================================
// View Transform
// =============================================================================

/// Coordinate transform between data space and screen space.
///
/// This is the core of the waveform viewer's viewport management,
/// handling zoom, pan, and coordinate conversion with commercial-grade
/// precision.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewTransform {
    /// Minimum X value visible (data coordinates, typically time in seconds)
    pub x_min: f64,
    /// Maximum X value visible (data coordinates)
    pub x_max: f64,
    /// Minimum Y value visible (data coordinates, typically voltage)
    pub y_min: f64,
    /// Maximum Y value visible (data coordinates)
    pub y_max: f64,

    /// Plot canvas width in pixels (updated on resize)
    pub plot_width: f64,
    /// Plot canvas height in pixels (updated on resize)
    pub plot_height: f64,

    /// Whether a pan operation is in progress
    pub is_panning: bool,
    /// Pan start position (screen X coordinate)
    pub pan_start_x: f64,
    /// Pan start position (screen Y coordinate)
    pub pan_start_y: f64,
    /// Whether user has dragged (vs just clicked)
    pub did_drag: bool,
}

impl Default for ViewTransform {
    fn default() -> Self {
        Self {
            x_min: 0.0,
            x_max: 1e-6, // Default 1µs window
            y_min: -1.0,
            y_max: 1.0,
            plot_width: 800.0,
            plot_height: 400.0,
            is_panning: false,
            pan_start_x: 0.0,
            pan_start_y: 0.0,
            did_drag: false,
        }
    }
}

impl ViewTransform {
    /// Create a new view transform with specified bounds
    pub fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
            ..Default::default()
        }
    }

    /// Get the X-axis range in data units
    #[inline]
    pub fn x_range(&self) -> f64 {
        self.x_max - self.x_min
    }

    /// Get the Y-axis range in data units
    #[inline]
    pub fn y_range(&self) -> f64 {
        self.y_max - self.y_min
    }

    /// Convert data X coordinate to screen X coordinate
    #[inline]
    pub fn data_to_screen_x(&self, data_x: f64) -> f64 {
        let frac = (data_x - self.x_min) / self.x_range();
        frac * self.plot_width
    }

    /// Convert data Y coordinate to screen Y coordinate
    /// Note: Screen Y is inverted (0 at top)
    #[inline]
    pub fn data_to_screen_y(&self, data_y: f64) -> f64 {
        let frac = (self.y_max - data_y) / self.y_range();
        frac * self.plot_height
    }

    /// Convert screen X coordinate to data X coordinate
    #[inline]
    pub fn screen_to_data_x(&self, screen_x: f64) -> f64 {
        let frac = screen_x / self.plot_width.max(1.0);
        self.x_min + frac * self.x_range()
    }

    /// Convert screen Y coordinate to data Y coordinate
    #[inline]
    pub fn screen_to_data_y(&self, screen_y: f64) -> f64 {
        let frac = screen_y / self.plot_height.max(1.0);
        self.y_max - frac * self.y_range()
    }

    /// Apply zoom centered at a fractional position (0.0 = left/top, 1.0 = right/bottom)
    ///
    /// Factor < 1.0 zooms in, factor > 1.0 zooms out.
    pub fn zoom(&mut self, factor: f64, center_x_frac: f64, center_y_frac: f64) {
        let center_x = self.x_min + center_x_frac * self.x_range();
        let center_y = self.y_min + center_y_frac * self.y_range();

        let new_x_range = self.x_range() * factor;
        let new_y_range = self.y_range() * factor;

        self.x_min = center_x - center_x_frac * new_x_range;
        self.x_max = center_x + (1.0 - center_x_frac) * new_x_range;
        self.y_min = center_y - center_y_frac * new_y_range;
        self.y_max = center_y + (1.0 - center_y_frac) * new_y_range;
    }

    /// Apply horizontal-only zoom (time axis)
    pub fn zoom_x_only(&mut self, factor: f64, center_x_frac: f64) {
        let center_x = self.x_min + center_x_frac * self.x_range();
        let new_x_range = self.x_range() * factor;

        self.x_min = center_x - center_x_frac * new_x_range;
        self.x_max = center_x + (1.0 - center_x_frac) * new_x_range;
    }

    /// Apply vertical-only zoom (amplitude axis)
    pub fn zoom_y_only(&mut self, factor: f64, center_y_frac: f64) {
        let center_y = self.y_min + center_y_frac * self.y_range();
        let new_y_range = self.y_range() * factor;

        self.y_min = center_y - center_y_frac * new_y_range;
        self.y_max = center_y + (1.0 - center_y_frac) * new_y_range;
    }

    /// Pan by a delta in data coordinates
    pub fn pan(&mut self, delta_x: f64, delta_y: f64) {
        self.x_min += delta_x;
        self.x_max += delta_x;
        self.y_min += delta_y;
        self.y_max += delta_y;
    }

    /// Fit view to encompass all provided traces
    pub fn fit_to_traces(&mut self, traces: &[TraceData]) {
        if traces.is_empty() {
            return;
        }

        let mut x_min = f64::MAX;
        let mut x_max = f64::MIN;
        let mut y_min = f64::MAX;
        let mut y_max = f64::MIN;

        for trace in traces.iter().filter(|t| t.visible) {
            if let (Some(trace_x_min), Some(trace_x_max)) = (trace.x_min(), trace.x_max()) {
                x_min = x_min.min(trace_x_min);
                x_max = x_max.max(trace_x_max);
            }
            if let (Some(trace_y_min), Some(trace_y_max)) = (trace.y_min(), trace.y_max()) {
                y_min = y_min.min(trace_y_min);
                y_max = y_max.max(trace_y_max);
            }
        }

        // Add margin (5% on each side)
        if x_min < x_max {
            let margin = (x_max - x_min) * 0.05;
            self.x_min = x_min - margin;
            self.x_max = x_max + margin;
        }
        if y_min < y_max {
            let margin = (y_max - y_min) * 0.05;
            self.y_min = y_min - margin;
            self.y_max = y_max + margin;
        }
    }

    /// Fit X axis only (preserve Y range)
    pub fn fit_x_to_traces(&mut self, traces: &[TraceData]) {
        if traces.is_empty() {
            return;
        }

        let mut x_min = f64::MAX;
        let mut x_max = f64::MIN;

        for trace in traces.iter().filter(|t| t.visible) {
            if let (Some(trace_x_min), Some(trace_x_max)) = (trace.x_min(), trace.x_max()) {
                x_min = x_min.min(trace_x_min);
                x_max = x_max.max(trace_x_max);
            }
        }

        if x_min < x_max {
            let margin = (x_max - x_min) * 0.05;
            self.x_min = x_min - margin;
            self.x_max = x_max + margin;
        }
    }

    /// Fit Y axis only (preserve X range)
    pub fn fit_y_to_traces(&mut self, traces: &[TraceData]) {
        if traces.is_empty() {
            return;
        }

        let mut y_min = f64::MAX;
        let mut y_max = f64::MIN;

        for trace in traces.iter().filter(|t| t.visible) {
            if let (Some(trace_y_min), Some(trace_y_max)) = (trace.y_min(), trace.y_max()) {
                y_min = y_min.min(trace_y_min);
                y_max = y_max.max(trace_y_max);
            }
        }

        if y_min < y_max {
            let margin = (y_max - y_min) * 0.05;
            self.y_min = y_min - margin;
            self.y_max = y_max + margin;
        }
    }
}

// =============================================================================
// Trace Data
// =============================================================================

/// Visual style for a waveform trace
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraceStyle {
    /// Line color (egui Color32 represented as RGBA u8 values)
    pub color: [u8; 4],
    /// Line width in pixels
    pub width: f32,
    /// Whether to show data point markers
    pub show_markers: bool,
    /// Marker size in pixels
    pub marker_size: f32,
}

impl Default for TraceStyle {
    fn default() -> Self {
        Self {
            color: [100, 200, 255, 255], // Light blue
            width: 1.5,
            show_markers: false,
            marker_size: 4.0,
        }
    }
}

impl TraceStyle {
    /// Create a new trace style with the given color
    pub fn with_color(r: u8, g: u8, b: u8) -> Self {
        Self {
            color: [r, g, b, 255],
            ..Default::default()
        }
    }

    /// Convert to egui Color32
    pub fn to_color32(&self) -> egui::Color32 {
        egui::Color32::from_rgba_unmultiplied(
            self.color[0],
            self.color[1],
            self.color[2],
            self.color[3],
        )
    }
}

/// Waveform trace data with cached statistics
///
/// This structure optimizes for both rendering performance and measurement
/// calculations by caching min/max values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraceData {
    /// Trace identifier (typically signal name like "V(out)")
    pub name: String,
    /// X-axis data points (typically time in seconds)
    pub x: Vec<f64>,
    /// Y-axis data points (typically voltage or current)
    pub y: Vec<f64>,
    /// Visual style
    pub style: TraceStyle,
    /// Whether this trace is visible
    pub visible: bool,
    /// Whether this trace is highlighted (cross-probe)
    pub highlighted: bool,

    // Cached statistics (computed lazily)
    cached_x_min: Option<f64>,
    cached_x_max: Option<f64>,
    cached_y_min: Option<f64>,
    cached_y_max: Option<f64>,
}

impl Default for TraceData {
    fn default() -> Self {
        Self {
            name: String::new(),
            x: Vec::new(),
            y: Vec::new(),
            style: TraceStyle::default(),
            visible: true,
            highlighted: false,
            cached_x_min: None,
            cached_x_max: None,
            cached_y_min: None,
            cached_y_max: None,
        }
    }
}

impl TraceData {
    /// Create a new trace with the given name and data
    pub fn new(name: impl Into<String>, x: Vec<f64>, y: Vec<f64>) -> Self {
        let mut trace = Self {
            name: name.into(),
            x,
            y,
            ..Default::default()
        };
        trace.compute_statistics();
        trace
    }

    /// Create a new trace with custom style
    pub fn with_style(mut self, style: TraceStyle) -> Self {
        self.style = style;
        self
    }

    /// Number of data points
    #[inline]
    pub fn len(&self) -> usize {
        self.x.len().min(self.y.len())
    }

    /// Check if trace has no data
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.x.is_empty() || self.y.is_empty()
    }

    /// Compute and cache statistics
    pub fn compute_statistics(&mut self) {
        if self.x.is_empty() {
            self.cached_x_min = None;
            self.cached_x_max = None;
        } else {
            self.cached_x_min = self.x.iter().copied().reduce(f64::min);
            self.cached_x_max = self.x.iter().copied().reduce(f64::max);
        }

        if self.y.is_empty() {
            self.cached_y_min = None;
            self.cached_y_max = None;
        } else {
            // Filter out NaN/Inf for robustness
            let valid_y: Vec<f64> = self.y.iter().copied().filter(|v| v.is_finite()).collect();
            if valid_y.is_empty() {
                self.cached_y_min = None;
                self.cached_y_max = None;
            } else {
                self.cached_y_min = valid_y.iter().copied().reduce(f64::min);
                self.cached_y_max = valid_y.iter().copied().reduce(f64::max);
            }
        }
    }

    /// Get cached X minimum
    #[inline]
    pub fn x_min(&self) -> Option<f64> {
        self.cached_x_min
    }

    /// Get cached X maximum
    #[inline]
    pub fn x_max(&self) -> Option<f64> {
        self.cached_x_max
    }

    /// Get cached Y minimum
    #[inline]
    pub fn y_min(&self) -> Option<f64> {
        self.cached_y_min
    }

    /// Get cached Y maximum
    #[inline]
    pub fn y_max(&self) -> Option<f64> {
        self.cached_y_max
    }

    /// Interpolate Y value at a given X position using linear interpolation
    ///
    /// Returns None if X is out of range or data is empty.
    pub fn interpolate_at(&self, target_x: f64) -> Option<f64> {
        if self.is_empty() {
            return None;
        }

        let n = self.len();

        // Binary search for the interval containing target_x
        let idx = match self.x[..n]
            .binary_search_by(|x| x.partial_cmp(&target_x).unwrap_or(std::cmp::Ordering::Less))
        {
            Ok(i) => return Some(self.y[i]), // Exact match
            Err(i) => i,
        };

        if idx == 0 {
            // Before first point - extrapolate or return first
            return Some(self.y[0]);
        }
        if idx >= n {
            // After last point - extrapolate or return last
            return Some(self.y[n - 1]);
        }

        // Linear interpolation
        let x0 = self.x[idx - 1];
        let x1 = self.x[idx];
        let y0 = self.y[idx - 1];
        let y1 = self.y[idx];

        let t = (target_x - x0) / (x1 - x0);
        Some(y0 + t * (y1 - y0))
    }
}

// =============================================================================
// Cursor State
// =============================================================================

/// Cursor mode for waveform measurements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CursorMode {
    /// No cursors active
    #[default]
    None,
    /// Single cursor for point measurement
    Single,
    /// Dual cursors for delta measurement
    Delta,
}

/// Cursor state for waveform measurements
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CursorState {
    /// Current cursor mode
    pub mode: CursorMode,
    /// Position of cursor 1 (X coordinate in data space)
    pub cursor1_x: Option<f64>,
    /// Position of cursor 2 (X coordinate in data space)
    pub cursor2_x: Option<f64>,
    /// Whether cursor 1 is being dragged
    pub dragging_cursor1: bool,
    /// Whether cursor 2 is being dragged
    pub dragging_cursor2: bool,
}

impl CursorState {
    /// Place a cursor at the given X position
    ///
    /// If no cursor is active, places cursor 1.
    /// If cursor 1 is active, places cursor 2 and enters delta mode.
    pub fn place(&mut self, x: f64) {
        match self.mode {
            CursorMode::None => {
                self.cursor1_x = Some(x);
                self.mode = CursorMode::Single;
            }
            CursorMode::Single => {
                self.cursor2_x = Some(x);
                self.mode = CursorMode::Delta;
            }
            CursorMode::Delta => {
                // In delta mode, click replaces the second cursor
                self.cursor2_x = Some(x);
            }
        }
    }

    /// Clear all cursors
    pub fn clear(&mut self) {
        self.mode = CursorMode::None;
        self.cursor1_x = None;
        self.cursor2_x = None;
        self.dragging_cursor1 = false;
        self.dragging_cursor2 = false;
    }

    /// Get delta between cursors (if both are set)
    pub fn delta_x(&self) -> Option<f64> {
        match (self.cursor1_x, self.cursor2_x) {
            (Some(x1), Some(x2)) => Some((x2 - x1).abs()),
            _ => None,
        }
    }

    /// Check if any cursor is active
    pub fn is_active(&self) -> bool {
        self.cursor1_x.is_some()
    }
}

// =============================================================================
// Box Selection
// =============================================================================

/// Box selection state for zoom-to-region
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BoxSelection {
    /// Whether selection is in progress
    pub is_selecting: bool,
    /// Start X in data coordinates
    pub start_x: f64,
    /// Start Y in data coordinates
    pub start_y: f64,
    /// End X in data coordinates
    pub end_x: f64,
    /// End Y in data coordinates
    pub end_y: f64,
    /// Start X in screen coordinates (for rendering)
    pub screen_start_x: f64,
    /// Start Y in screen coordinates
    pub screen_start_y: f64,
    /// Plot rectangle (left, top, width, height) for global coordinate conversion
    pub plot_rect: (f64, f64, f64, f64),
}

impl BoxSelection {
    /// Start a new box selection
    pub fn start(
        &mut self,
        data_x: f64,
        data_y: f64,
        screen_x: f64,
        screen_y: f64,
        plot_rect: (f64, f64, f64, f64),
    ) {
        self.is_selecting = true;
        self.start_x = data_x;
        self.start_y = data_y;
        self.end_x = data_x;
        self.end_y = data_y;
        self.screen_start_x = screen_x;
        self.screen_start_y = screen_y;
        self.plot_rect = plot_rect;
    }

    /// Update the selection endpoint
    pub fn update(&mut self, data_x: f64, data_y: f64) {
        self.end_x = data_x;
        self.end_y = data_y;
    }

    /// Finish selection and return the selected region (x_min, x_max, y_min, y_max)
    ///
    /// Returns None if selection was too small (click without drag).
    pub fn finish(&mut self) -> Option<(f64, f64, f64, f64)> {
        if !self.is_selecting {
            return None;
        }

        self.is_selecting = false;

        let x_min = self.start_x.min(self.end_x);
        let x_max = self.start_x.max(self.end_x);
        let y_min = self.start_y.min(self.end_y);
        let y_max = self.start_y.max(self.end_y);

        // Require minimum selection size (avoid accidental zoom)
        let min_range = 1e-12;
        if (x_max - x_min) < min_range || (y_max - y_min) < min_range {
            return None;
        }

        Some((x_min, x_max, y_min, y_max))
    }

    /// Cancel the selection
    pub fn cancel(&mut self) {
        self.is_selecting = false;
    }
}

// =============================================================================
// Waveform Viewer State
// =============================================================================

/// Complete state for the waveform viewer
///
/// This is the top-level container that holds all viewer state.
/// It can be serialized for session persistence.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WaveformViewerState {
    /// View transform (zoom/pan state)
    pub view: ViewTransform,
    /// All loaded traces
    pub traces: Vec<TraceData>,
    /// Cursor state
    pub cursors: CursorState,
    /// Box selection state
    pub box_selection: BoxSelection,
    /// Whether measurement panel is visible
    pub show_measurements: bool,
    /// Whether export panel is visible
    pub show_export: bool,
    /// Last expression evaluation error
    pub expression_error: Option<String>,
}

impl WaveformViewerState {
    /// Create a new viewer state
    pub fn new() -> Self {
        Self::default()
    }

    /// Load traces from simulation waveforms
    pub fn load_from_simulation(&mut self, waveforms: &[crate::state::WaveformData]) {
        self.traces = waveforms
            .iter()
            .enumerate()
            .map(|(i, wf)| {
                let mut trace = TraceData::new(&wf.name, wf.x.clone(), wf.y.clone());
                trace.visible = wf.visible;
                // Parse color from string or use default palette
                trace.style = TraceStyle::with_color(
                    Self::palette_color(i).0,
                    Self::palette_color(i).1,
                    Self::palette_color(i).2,
                );
                trace
            })
            .collect();

        // Auto-fit on load
        self.view.fit_to_traces(&self.traces);
    }

    /// Get a color from the default palette
    fn palette_color(index: usize) -> (u8, u8, u8) {
        const PALETTE: [(u8, u8, u8); 10] = [
            (59, 130, 246),  // Blue
            (16, 185, 129),  // Green
            (249, 115, 22),  // Orange
            (139, 92, 246),  // Purple
            (236, 72, 153),  // Pink
            (234, 179, 8),   // Yellow
            (20, 184, 166),  // Teal
            (239, 68, 68),   // Red
            (168, 162, 158), // Gray
            (132, 204, 22),  // Lime
        ];
        PALETTE[index % PALETTE.len()]
    }

    /// Toggle visibility of a trace by index
    pub fn toggle_trace_visibility(&mut self, index: usize) {
        if let Some(trace) = self.traces.get_mut(index) {
            trace.visible = !trace.visible;
        }
    }

    /// Set highlight state for a trace by name
    pub fn set_trace_highlight(&mut self, name: &str, highlighted: bool) {
        for trace in &mut self.traces {
            if trace.name == name {
                trace.highlighted = highlighted;
            }
        }
    }

    /// Clear all highlights
    pub fn clear_highlights(&mut self) {
        for trace in &mut self.traces {
            trace.highlighted = false;
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // ViewTransform Tests
    // =========================================================================

    #[test]
    fn test_view_transform_default() {
        let vt = ViewTransform::default();
        assert_eq!(vt.x_min, 0.0);
        assert_eq!(vt.x_max, 1e-6);
        assert_eq!(vt.y_min, -1.0);
        assert_eq!(vt.y_max, 1.0);
    }

    #[test]
    fn test_view_transform_ranges() {
        let vt = ViewTransform::new(0.0, 10.0, -5.0, 5.0);
        assert_eq!(vt.x_range(), 10.0);
        assert_eq!(vt.y_range(), 10.0);
    }

    #[test]
    fn test_view_transform_data_to_screen() {
        let mut vt = ViewTransform::new(0.0, 100.0, 0.0, 100.0);
        vt.plot_width = 1000.0;
        vt.plot_height = 500.0;

        // At 50% data, should be at 50% screen
        assert!((vt.data_to_screen_x(50.0) - 500.0).abs() < 1e-10);
        // Y is inverted: data 100 -> screen 0, data 0 -> screen 500
        assert!((vt.data_to_screen_y(100.0) - 0.0).abs() < 1e-10);
        assert!((vt.data_to_screen_y(0.0) - 500.0).abs() < 1e-10);
    }

    #[test]
    fn test_view_transform_screen_to_data() {
        let mut vt = ViewTransform::new(0.0, 100.0, 0.0, 100.0);
        vt.plot_width = 1000.0;
        vt.plot_height = 500.0;

        assert!((vt.screen_to_data_x(500.0) - 50.0).abs() < 1e-10);
        assert!((vt.screen_to_data_y(0.0) - 100.0).abs() < 1e-10);
        assert!((vt.screen_to_data_y(500.0) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_view_transform_zoom_in_center() {
        let mut vt = ViewTransform::new(0.0, 100.0, 0.0, 100.0);
        vt.zoom(0.5, 0.5, 0.5); // Zoom in 2x at center

        // Range should be halved, centered at 50
        assert!((vt.x_range() - 50.0).abs() < 1e-10);
        assert!((vt.y_range() - 50.0).abs() < 1e-10);
        assert!((vt.x_min - 25.0).abs() < 1e-10);
        assert!((vt.x_max - 75.0).abs() < 1e-10);
    }

    #[test]
    fn test_view_transform_zoom_out_center() {
        let mut vt = ViewTransform::new(25.0, 75.0, 25.0, 75.0);
        vt.zoom(2.0, 0.5, 0.5); // Zoom out 2x at center

        assert!((vt.x_range() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_view_transform_zoom_at_corner() {
        let mut vt = ViewTransform::new(0.0, 100.0, 0.0, 100.0);
        vt.zoom(0.5, 0.0, 0.0); // Zoom in at top-left corner

        // Should zoom keeping (0,0) fixed
        assert!((vt.x_min - 0.0).abs() < 1e-10);
        assert!((vt.x_max - 50.0).abs() < 1e-10);
    }

    #[test]
    fn test_view_transform_zoom_x_only() {
        let mut vt = ViewTransform::new(0.0, 100.0, 0.0, 100.0);
        let original_y_range = vt.y_range();
        vt.zoom_x_only(0.5, 0.5);

        assert!((vt.x_range() - 50.0).abs() < 1e-10);
        assert!((vt.y_range() - original_y_range).abs() < 1e-10);
    }

    #[test]
    fn test_view_transform_zoom_y_only() {
        let mut vt = ViewTransform::new(0.0, 100.0, 0.0, 100.0);
        let original_x_range = vt.x_range();
        vt.zoom_y_only(0.5, 0.5);

        assert!((vt.x_range() - original_x_range).abs() < 1e-10);
        assert!((vt.y_range() - 50.0).abs() < 1e-10);
    }

    #[test]
    fn test_view_transform_pan() {
        let mut vt = ViewTransform::new(0.0, 100.0, 0.0, 100.0);
        vt.pan(10.0, -5.0);

        assert_eq!(vt.x_min, 10.0);
        assert_eq!(vt.x_max, 110.0);
        assert_eq!(vt.y_min, -5.0);
        assert_eq!(vt.y_max, 95.0);
    }

    // =========================================================================
    // TraceData Tests
    // =========================================================================

    #[test]
    fn test_trace_data_new() {
        let trace = TraceData::new("V(out)", vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 2.0]);
        assert_eq!(trace.name, "V(out)");
        assert_eq!(trace.len(), 3);
        assert!(!trace.is_empty());
    }

    #[test]
    fn test_trace_data_statistics() {
        let trace = TraceData::new("test", vec![0.0, 1.0, 2.0, 3.0], vec![-1.0, 2.0, 0.5, 1.0]);

        assert_eq!(trace.x_min(), Some(0.0));
        assert_eq!(trace.x_max(), Some(3.0));
        assert_eq!(trace.y_min(), Some(-1.0));
        assert_eq!(trace.y_max(), Some(2.0));
    }

    #[test]
    fn test_trace_data_empty() {
        let trace = TraceData::default();
        assert!(trace.is_empty());
        assert_eq!(trace.x_min(), None);
        assert_eq!(trace.y_max(), None);
    }

    #[test]
    fn test_trace_data_interpolate_exact() {
        let trace = TraceData::new("test", vec![0.0, 1.0, 2.0], vec![0.0, 10.0, 20.0]);

        assert_eq!(trace.interpolate_at(0.0), Some(0.0));
        assert_eq!(trace.interpolate_at(1.0), Some(10.0));
        assert_eq!(trace.interpolate_at(2.0), Some(20.0));
    }

    #[test]
    fn test_trace_data_interpolate_between() {
        let trace = TraceData::new("test", vec![0.0, 1.0, 2.0], vec![0.0, 10.0, 20.0]);

        assert!((trace.interpolate_at(0.5).unwrap() - 5.0).abs() < 1e-10);
        assert!((trace.interpolate_at(1.5).unwrap() - 15.0).abs() < 1e-10);
    }

    #[test]
    fn test_trace_data_interpolate_extrapolate() {
        let trace = TraceData::new("test", vec![1.0, 2.0], vec![10.0, 20.0]);

        // Before range - returns first value
        assert_eq!(trace.interpolate_at(0.0), Some(10.0));
        // After range - returns last value
        assert_eq!(trace.interpolate_at(3.0), Some(20.0));
    }

    #[test]
    fn test_trace_data_with_nan() {
        let mut trace = TraceData::new("test", vec![0.0, 1.0, 2.0], vec![1.0, f64::NAN, 3.0]);
        trace.compute_statistics();

        // NaN should be filtered out of statistics
        assert_eq!(trace.y_min(), Some(1.0));
        assert_eq!(trace.y_max(), Some(3.0));
    }

    // =========================================================================
    // TraceStyle Tests
    // =========================================================================

    #[test]
    fn test_trace_style_default() {
        let style = TraceStyle::default();
        assert_eq!(style.width, 1.5);
        assert!(!style.show_markers);
    }

    #[test]
    fn test_trace_style_with_color() {
        let style = TraceStyle::with_color(255, 0, 128);
        assert_eq!(style.color, [255, 0, 128, 255]);
    }

    #[test]
    fn test_trace_style_to_color32() {
        let style = TraceStyle::with_color(100, 150, 200);
        let color = style.to_color32();
        assert_eq!(color, egui::Color32::from_rgb(100, 150, 200));
    }

    // =========================================================================
    // CursorState Tests
    // =========================================================================

    #[test]
    fn test_cursor_state_default() {
        let cs = CursorState::default();
        assert_eq!(cs.mode, CursorMode::None);
        assert_eq!(cs.cursor1_x, None);
        assert_eq!(cs.cursor2_x, None);
    }

    #[test]
    fn test_cursor_state_place_first() {
        let mut cs = CursorState::default();
        cs.place(5.0);

        assert_eq!(cs.mode, CursorMode::Single);
        assert_eq!(cs.cursor1_x, Some(5.0));
        assert_eq!(cs.cursor2_x, None);
    }

    #[test]
    fn test_cursor_state_place_second() {
        let mut cs = CursorState::default();
        cs.place(5.0);
        cs.place(10.0);

        assert_eq!(cs.mode, CursorMode::Delta);
        assert_eq!(cs.cursor1_x, Some(5.0));
        assert_eq!(cs.cursor2_x, Some(10.0));
    }

    #[test]
    fn test_cursor_state_delta() {
        let mut cs = CursorState::default();
        cs.place(5.0);
        cs.place(15.0);

        assert_eq!(cs.delta_x(), Some(10.0));
    }

    #[test]
    fn test_cursor_state_delta_reversed() {
        let mut cs = CursorState::default();
        cs.place(15.0);
        cs.place(5.0);

        // Delta should be absolute
        assert_eq!(cs.delta_x(), Some(10.0));
    }

    #[test]
    fn test_cursor_state_clear() {
        let mut cs = CursorState::default();
        cs.place(5.0);
        cs.place(10.0);
        cs.clear();

        assert_eq!(cs.mode, CursorMode::None);
        assert_eq!(cs.cursor1_x, None);
        assert_eq!(cs.cursor2_x, None);
    }

    #[test]
    fn test_cursor_state_replace_second_in_delta_mode() {
        let mut cs = CursorState::default();
        cs.place(5.0);
        cs.place(10.0);
        cs.place(20.0); // Should replace cursor2

        assert_eq!(cs.mode, CursorMode::Delta);
        assert_eq!(cs.cursor1_x, Some(5.0));
        assert_eq!(cs.cursor2_x, Some(20.0));
    }

    // =========================================================================
    // BoxSelection Tests
    // =========================================================================

    #[test]
    fn test_box_selection_default() {
        let bs = BoxSelection::default();
        assert!(!bs.is_selecting);
    }

    #[test]
    fn test_box_selection_start() {
        let mut bs = BoxSelection::default();
        bs.start(0.0, 0.0, 100.0, 100.0, (0.0, 0.0, 800.0, 600.0));

        assert!(bs.is_selecting);
        assert_eq!(bs.start_x, 0.0);
        assert_eq!(bs.start_y, 0.0);
    }

    #[test]
    fn test_box_selection_update() {
        let mut bs = BoxSelection::default();
        bs.start(0.0, 0.0, 100.0, 100.0, (0.0, 0.0, 800.0, 600.0));
        bs.update(10.0, 5.0);

        assert_eq!(bs.end_x, 10.0);
        assert_eq!(bs.end_y, 5.0);
    }

    #[test]
    fn test_box_selection_finish_valid() {
        let mut bs = BoxSelection::default();
        bs.start(0.0, 0.0, 100.0, 100.0, (0.0, 0.0, 800.0, 600.0));
        bs.update(10.0, 5.0);

        let result = bs.finish();
        assert!(result.is_some());
        let (x_min, x_max, y_min, y_max) = result.unwrap();
        assert_eq!(x_min, 0.0);
        assert_eq!(x_max, 10.0);
        assert_eq!(y_min, 0.0);
        assert_eq!(y_max, 5.0);
        assert!(!bs.is_selecting);
    }

    #[test]
    fn test_box_selection_finish_too_small() {
        let mut bs = BoxSelection::default();
        bs.start(0.0, 0.0, 100.0, 100.0, (0.0, 0.0, 800.0, 600.0));
        // No update - selection is zero size

        let result = bs.finish();
        assert!(result.is_none());
    }

    #[test]
    fn test_box_selection_cancel() {
        let mut bs = BoxSelection::default();
        bs.start(0.0, 0.0, 100.0, 100.0, (0.0, 0.0, 800.0, 600.0));
        bs.cancel();

        assert!(!bs.is_selecting);
    }

    // =========================================================================
    // WaveformViewerState Tests
    // =========================================================================

    #[test]
    fn test_waveform_viewer_state_default() {
        let state = WaveformViewerState::new();
        assert!(state.traces.is_empty());
        assert!(!state.show_measurements);
    }

    #[test]
    fn test_waveform_viewer_toggle_visibility() {
        let mut state = WaveformViewerState::new();
        state
            .traces
            .push(TraceData::new("test", vec![0.0], vec![0.0]));

        assert!(state.traces[0].visible);
        state.toggle_trace_visibility(0);
        assert!(!state.traces[0].visible);
        state.toggle_trace_visibility(0);
        assert!(state.traces[0].visible);
    }

    #[test]
    fn test_waveform_viewer_highlight() {
        let mut state = WaveformViewerState::new();
        state
            .traces
            .push(TraceData::new("V(out)", vec![0.0], vec![0.0]));
        state
            .traces
            .push(TraceData::new("I(r1)", vec![0.0], vec![0.0]));

        state.set_trace_highlight("V(out)", true);
        assert!(state.traces[0].highlighted);
        assert!(!state.traces[1].highlighted);

        state.clear_highlights();
        assert!(!state.traces[0].highlighted);
        assert!(!state.traces[1].highlighted);
    }

    #[test]
    fn test_waveform_viewer_palette_colors() {
        // Verify palette cycles correctly
        for i in 0..20 {
            let color = WaveformViewerState::palette_color(i);
            assert!(color.0 > 0 || color.1 > 0 || color.2 > 0);
        }
    }

    // =========================================================================
    // Fit-to-data Tests
    // =========================================================================

    #[test]
    fn test_fit_to_traces_single() {
        let mut vt = ViewTransform::default();
        let traces = vec![TraceData::new(
            "test",
            vec![0.0, 1.0, 2.0],
            vec![-1.0, 0.0, 1.0],
        )];
        vt.fit_to_traces(&traces);

        // Should encompass data with margin
        assert!(vt.x_min < 0.0);
        assert!(vt.x_max > 2.0);
        assert!(vt.y_min < -1.0);
        assert!(vt.y_max > 1.0);
    }

    #[test]
    fn test_fit_to_traces_multiple() {
        let mut vt = ViewTransform::default();
        let traces = vec![
            TraceData::new("t1", vec![0.0, 1.0], vec![0.0, 1.0]),
            TraceData::new("t2", vec![5.0, 10.0], vec![-5.0, 5.0]),
        ];
        vt.fit_to_traces(&traces);

        // Should encompass both traces
        assert!(vt.x_min < 0.0);
        assert!(vt.x_max > 10.0);
        assert!(vt.y_min < -5.0);
        assert!(vt.y_max > 5.0);
    }

    #[test]
    fn test_fit_to_traces_hidden_excluded() {
        let mut vt = ViewTransform::default();
        let mut visible = TraceData::new("visible", vec![0.0, 1.0], vec![0.0, 1.0]);
        visible.visible = true;

        let mut hidden = TraceData::new("hidden", vec![0.0, 100.0], vec![-100.0, 100.0]);
        hidden.visible = false;

        let traces = vec![visible, hidden];
        vt.fit_to_traces(&traces);

        // Should fit only to visible trace
        assert!(vt.x_max < 10.0); // Not expanded to hidden trace's range
    }

    #[test]
    fn test_fit_to_traces_empty() {
        let mut vt = ViewTransform::new(0.0, 1.0, 0.0, 1.0);
        let traces: Vec<TraceData> = vec![];
        vt.fit_to_traces(&traces);

        // Should not change
        assert_eq!(vt.x_min, 0.0);
        assert_eq!(vt.x_max, 1.0);
    }
}
