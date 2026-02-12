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

/// Minimum zoom range limits (Spectre-style)
/// These prevent numerical precision issues and infinite grid/tick generation
impl ViewTransform {
    /// Minimum X-axis range (prevents extreme zoom-in)
    /// 1 femtosecond is a reasonable minimum for circuit simulation
    pub const MIN_X_RANGE: f64 = 1e-15;

    /// Minimum Y-axis range (prevents extreme zoom-in)
    pub const MIN_Y_RANGE: f64 = 1e-15;

    /// Enforce minimum zoom range to prevent numerical issues
    /// Call this after any zoom or pan operation
    pub fn enforce_minimum_range(&mut self) {
        let x_range = self.x_range();
        if x_range < Self::MIN_X_RANGE {
            let center = (self.x_min + self.x_max) / 2.0;
            self.x_min = center - Self::MIN_X_RANGE / 2.0;
            self.x_max = center + Self::MIN_X_RANGE / 2.0;
        }

        let y_range = self.y_range();
        if y_range < Self::MIN_Y_RANGE {
            let center = (self.y_min + self.y_max) / 2.0;
            self.y_min = center - Self::MIN_Y_RANGE / 2.0;
            self.y_max = center + Self::MIN_Y_RANGE / 2.0;
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

        // Enforce minimum zoom to prevent numerical issues
        self.enforce_minimum_range();
    }

    /// Apply horizontal-only zoom (time axis)
    pub fn zoom_x_only(&mut self, factor: f64, center_x_frac: f64) {
        let center_x = self.x_min + center_x_frac * self.x_range();
        let new_x_range = self.x_range() * factor;

        self.x_min = center_x - center_x_frac * new_x_range;
        self.x_max = center_x + (1.0 - center_x_frac) * new_x_range;
        self.enforce_minimum_range();
    }

    /// Apply vertical-only zoom (amplitude axis)
    pub fn zoom_y_only(&mut self, factor: f64, center_y_frac: f64) {
        let center_y = self.y_min + center_y_frac * self.y_range();
        let new_y_range = self.y_range() * factor;

        self.y_min = center_y - center_y_frac * new_y_range;
        self.y_max = center_y + (1.0 - center_y_frac) * new_y_range;
        self.enforce_minimum_range();
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

    /// Clamp view to data bounds, preventing pan/zoom into empty regions
    ///
    /// This implements commercial-grade behavior where users cannot pan
    /// to see areas where no data exists (e.g., before t=0 or beyond simulation end).
    /// Allows a small margin (5%) beyond data bounds for context.
    ///
    /// When the viewport range exceeds the data bounds range (e.g., when switching
    /// between analysis types), this forces the viewport to fit the data bounds
    /// rather than showing empty gaps.
    pub fn clamp_to_bounds(&mut self, bounds: &DataBounds) {
        if !bounds.valid {
            return;
        }

        // Allow 5% margin beyond data bounds for context (tight clamping)
        // X-axis: NO margin - strictly prevent viewing before t=0 or after data
        // Y-axis: 5% margin for vertical context
        let x_margin = 0.0; // Strict: no viewing outside data time range
        let y_margin = bounds.y_range() * 0.05;

        let clamp_x_min = bounds.x_min - x_margin;
        let clamp_x_max = bounds.x_max + x_margin;
        let clamp_y_min = bounds.y_min - y_margin;
        let clamp_y_max = bounds.y_max + y_margin;

        let clamp_x_range = clamp_x_max - clamp_x_min;
        let clamp_y_range = clamp_y_max - clamp_y_min;

        // If the current viewport range is larger than the clamping bounds,
        // force-fit to the clamping bounds. This handles switching between
        // analysis types (e.g., transient time domain to AC frequency domain)
        // where the scales are completely different.
        if self.x_range() >= clamp_x_range {
            self.x_min = clamp_x_min;
            self.x_max = clamp_x_max;
        } else {
            // Clamp X range while preserving zoom level
            if self.x_min < clamp_x_min {
                let shift = clamp_x_min - self.x_min;
                self.x_min = clamp_x_min;
                self.x_max += shift;
            }
            if self.x_max > clamp_x_max {
                let shift = self.x_max - clamp_x_max;
                self.x_max = clamp_x_max;
                self.x_min -= shift;
            }
        }

        if self.y_range() >= clamp_y_range {
            self.y_min = clamp_y_min;
            self.y_max = clamp_y_max;
        } else {
            // Clamp Y range while preserving zoom level
            if self.y_min < clamp_y_min {
                let shift = clamp_y_min - self.y_min;
                self.y_min = clamp_y_min;
                self.y_max += shift;
            }
            if self.y_max > clamp_y_max {
                let shift = self.y_max - clamp_y_max;
                self.y_max = clamp_y_max;
                self.y_min -= shift;
            }
        }

        // Final bounds check to ensure we don't exceed clamp bounds
        // This handles edge cases from floating-point arithmetic
        self.x_min = self.x_min.max(clamp_x_min);
        self.x_max = self.x_max.min(clamp_x_max);
        self.y_min = self.y_min.max(clamp_y_min);
        self.y_max = self.y_max.min(clamp_y_max);
    }

    /// Pan with clamping to data bounds
    pub fn pan_clamped(&mut self, delta_x: f64, delta_y: f64, bounds: &DataBounds) {
        self.pan(delta_x, delta_y);
        self.clamp_to_bounds(bounds);
    }

    /// Zoom with clamping to data bounds
    pub fn zoom_clamped(
        &mut self,
        factor: f64,
        center_x_frac: f64,
        center_y_frac: f64,
        bounds: &DataBounds,
    ) {
        self.zoom(factor, center_x_frac, center_y_frac);
        self.clamp_to_bounds(bounds);
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
// Waveform Panel (for multi-panel display)
// =============================================================================

/// A single panel in the multi-panel waveform viewer
///
/// Each panel has its own Y-axis transform and trace assignments,
/// but shares the X-axis with other panels for synchronized time viewing.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WaveformPanel {
    /// Panel identifier
    pub id: usize,
    /// Panel label/title
    pub label: String,
    /// Y-axis transform (independent per panel)
    pub y_min: f64,
    pub y_max: f64,
    /// Indices of traces assigned to this panel
    pub trace_indices: Vec<usize>,
    /// Height fraction (0.0-1.0) of total viewer height
    pub height_fraction: f32,
    /// Whether Y-axis auto-scales to fit traces
    pub auto_scale_y: bool,
    /// Whether this panel is collapsed
    pub collapsed: bool,
}

impl WaveformPanel {
    /// Create a new panel with default settings
    pub fn new(id: usize) -> Self {
        Self {
            id,
            label: format!("Panel {}", id + 1),
            y_min: -1.0,
            y_max: 1.0,
            trace_indices: Vec::new(),
            height_fraction: 1.0,
            auto_scale_y: true,
            collapsed: false,
        }
    }

    /// Create a panel with a specific label
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Add a trace to this panel
    pub fn add_trace(&mut self, trace_index: usize) {
        if !self.trace_indices.contains(&trace_index) {
            self.trace_indices.push(trace_index);
        }
    }

    /// Remove a trace from this panel
    pub fn remove_trace(&mut self, trace_index: usize) {
        self.trace_indices.retain(|&idx| idx != trace_index);
    }

    /// Get the Y range for this panel
    pub fn y_range(&self) -> f64 {
        self.y_max - self.y_min
    }

    /// Fit Y-axis to the given traces
    pub fn fit_y_to_traces(&mut self, all_traces: &[TraceData]) {
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;

        for &idx in &self.trace_indices {
            if let Some(trace) = all_traces.get(idx) {
                if trace.visible {
                    if let Some(min) = trace.y_min() {
                        y_min = y_min.min(min);
                    }
                    if let Some(max) = trace.y_max() {
                        y_max = y_max.max(max);
                    }
                }
            }
        }

        // Add padding and ensure minimum range
        if y_min.is_finite() && y_max.is_finite() {
            let range = y_max - y_min;
            if range < 1e-12 {
                // Single value or very small range
                let center = (y_min + y_max) / 2.0;
                self.y_min = center - 0.5;
                self.y_max = center + 0.5;
            } else {
                let padding = range * 0.1;
                self.y_min = y_min - padding;
                self.y_max = y_max + padding;
            }
        }
    }
}

// =============================================================================
// Data Bounds (for view clamping)
// =============================================================================

/// Bounds of the actual data (computed from traces)
///
/// Used to prevent panning/zooming into regions where no data exists.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DataBounds {
    /// Minimum X value across all traces
    pub x_min: f64,
    /// Maximum X value across all traces
    pub x_max: f64,
    /// Minimum Y value across all traces
    pub y_min: f64,
    /// Maximum Y value across all traces
    pub y_max: f64,
    /// Whether bounds are valid (false if no data)
    pub valid: bool,
}

impl DataBounds {
    /// Compute bounds from traces
    pub fn from_traces(traces: &[TraceData]) -> Self {
        let visible_traces: Vec<_> = traces.iter().filter(|t| t.visible).collect();

        if visible_traces.is_empty() {
            // Fall back to all traces if none visible
            let all_traces: Vec<_> = traces.iter().collect();
            return Self::compute_from_iter(all_traces.into_iter());
        }

        Self::compute_from_iter(visible_traces.into_iter())
    }

    fn compute_from_iter<'a>(traces: impl Iterator<Item = &'a TraceData>) -> Self {
        let mut x_min = f64::MAX;
        let mut x_max = f64::MIN;
        let mut y_min = f64::MAX;
        let mut y_max = f64::MIN;
        let mut has_data = false;

        for trace in traces {
            if let Some(tx_min) = trace.x_min() {
                x_min = x_min.min(tx_min);
                has_data = true;
            }
            if let Some(tx_max) = trace.x_max() {
                x_max = x_max.max(tx_max);
            }
            if let Some(ty_min) = trace.y_min() {
                y_min = y_min.min(ty_min);
            }
            if let Some(ty_max) = trace.y_max() {
                y_max = y_max.max(ty_max);
            }
        }

        if !has_data || x_min > x_max {
            return Self::default();
        }

        // Handle flat data (constant value across all points)
        // Add epsilon margin to ensure valid range for rendering
        const EPSILON: f64 = 1e-12;

        // For X axis: if all points at same X, expand by small amount
        if x_max <= x_min {
            let center = x_min;
            let margin = if center.abs() > EPSILON {
                center.abs() * 0.1
            } else {
                1.0
            };
            x_min = center - margin;
            x_max = center + margin;
        }

        // For Y axis: if all points at same Y (flat line), expand by small amount
        if y_max <= y_min {
            let center = y_min;
            let margin = if center.abs() > EPSILON {
                center.abs() * 0.1
            } else {
                1.0
            };
            y_min = center - margin;
            y_max = center + margin;
        }

        Self {
            x_min,
            x_max,
            y_min,
            y_max,
            valid: true,
        }
    }

    /// Get X range
    pub fn x_range(&self) -> f64 {
        self.x_max - self.x_min
    }

    /// Get Y range  
    pub fn y_range(&self) -> f64 {
        self.y_max - self.y_min
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
    /// View transform (zoom/pan state) - X-axis is shared across panels
    pub view: ViewTransform,
    /// All loaded traces
    pub traces: Vec<TraceData>,
    /// Waveform panels (for multi-panel display)
    pub panels: Vec<WaveformPanel>,
    /// Active panel index (for zoom/pan operations)
    pub active_panel: usize,
    /// Whether multi-panel mode is enabled
    pub multi_panel_enabled: bool,
    /// Cursor state
    pub cursors: CursorState,
    /// User markers (additional vertical references beyond dual cursors)
    pub markers: Vec<f64>,
    /// Box selection state
    pub box_selection: BoxSelection,
    /// Whether measurement panel is visible
    pub show_measurements: bool,
    /// Whether export panel is visible
    pub show_export: bool,
    /// Last expression evaluation error
    pub expression_error: Option<String>,

    // === Axis Labels (analysis-aware) ===
    /// X-axis label (e.g., "Time", "Frequency", "Voltage")
    pub x_axis_label: String,
    /// X-axis unit base (e.g., "s", "Hz", "V")
    pub x_axis_unit: String,
    /// Y-axis label (e.g., "Voltage", "Magnitude")
    pub y_axis_label: String,
    /// Y-axis unit base (e.g., "V", "A", "dB")
    pub y_axis_unit: String,

    // === State persistence fields ===
    /// Data version - tracks which simulation data is loaded.
    /// When simulation.data_version differs, we reload traces.
    pub data_version: u64,

    /// Computed bounds of all trace data (for view clamping)
    pub data_bounds: DataBounds,

    /// Whether an initial fit has been performed
    pub has_initial_fit: bool,

    /// Specification overlays for pass/fail visualization
    pub spec_overlays: Vec<super::spec_overlay::SpecOverlay>,
    /// Legend UI state (filter/sort/collapse). Runtime-only UI preference.
    #[serde(skip)]
    pub legend_state: super::legend::LegendState,
    /// Selected trace for focused measurements/operations.
    #[serde(skip)]
    pub selected_trace: Option<String>,
    /// Scope used for measurement panel calculations.
    #[serde(skip)]
    pub measurement_scope: MeasurementScope,
    /// Whether measurements should use cursor range when dual cursors are active.
    #[serde(skip)]
    pub measurement_use_cursor_range: bool,
    /// Runtime export options controlled by waveform export panel.
    #[serde(skip)]
    pub export_options: super::export::ExportOptions,
    /// Optional export status message shown in the export panel.
    #[serde(skip)]
    pub export_status: Option<String>,
}

/// Scope for waveform measurement display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MeasurementScope {
    /// Calculate measurements only for the selected trace.
    Selected,
    /// Calculate measurements for visible traces.
    #[default]
    Visible,
    /// Calculate measurements for every trace.
    All,
}

impl MeasurementScope {
    /// Display label for UI controls.
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Selected => "Selected",
            Self::Visible => "Visible",
            Self::All => "All",
        }
    }

    /// All supported scope options in deterministic UI order.
    pub fn all() -> &'static [MeasurementScope] {
        &[Self::Selected, Self::Visible, Self::All]
    }
}

impl WaveformViewerState {
    /// Create a new viewer state
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a waveform marker. Maintains sorted marker order and bounded count.
    pub fn add_marker(&mut self, x: f64) {
        if !x.is_finite() {
            return;
        }
        const MAX_MARKERS: usize = 16;
        const MERGE_EPS: f64 = 1e-12;
        if self.markers.iter().any(|m| (*m - x).abs() <= MERGE_EPS) {
            return;
        }
        self.markers.push(x);
        self.markers.sort_by(|a, b| a.total_cmp(b));
        if self.markers.len() > MAX_MARKERS {
            self.markers.remove(0);
        }
    }

    /// Clear all waveform markers.
    pub fn clear_markers(&mut self) {
        self.markers.clear();
    }

    /// Remove the nearest marker within a tolerance window.
    pub fn remove_nearest_marker(&mut self, x: f64, tolerance: f64) -> bool {
        if !x.is_finite() || !tolerance.is_finite() || tolerance < 0.0 {
            return false;
        }
        let Some((idx, dist)) = self
            .markers
            .iter()
            .enumerate()
            .map(|(idx, marker)| (idx, (*marker - x).abs()))
            .min_by(|a, b| a.1.total_cmp(&b.1))
        else {
            return false;
        };
        if dist <= tolerance {
            self.markers.remove(idx);
            true
        } else {
            false
        }
    }

    /// Load traces from simulation waveforms
    ///
    /// This updates the traces and recalculates data bounds.
    /// Does NOT auto-fit - call fit_to_data_bounds() explicitly.
    pub fn load_from_simulation(&mut self, waveforms: &[crate::state::WaveformData]) {
        self.traces = waveforms
            .iter()
            .enumerate()
            .map(|(i, wf)| {
                let mut trace = TraceData::new(&wf.name, wf.x.clone(), wf.y.clone());
                trace.visible = wf.visible;
                // Parse color from waveform or use default palette as fallback
                let (r, g, b) =
                    Self::parse_hex_color(&wf.color).unwrap_or_else(|| Self::palette_color(i));
                trace.style = TraceStyle::with_color(r, g, b);

                // Debug: show trace statistics to diagnose flat waveform issue
                log::info!(
                    "Loaded trace '{}': {} points, y_min={:?}, y_max={:?}",
                    trace.name,
                    trace.len(),
                    trace.y_min(),
                    trace.y_max()
                );

                trace
            })
            .collect();

        // Recalculate data bounds
        self.data_bounds = DataBounds::from_traces(&self.traces);
    }

    /// Fit view to data bounds with appropriate margins
    pub fn fit_to_data_bounds(&mut self) {
        if !self.data_bounds.valid {
            return;
        }

        let bounds = &self.data_bounds;

        // X-axis: NO margin - strictly show only data time range (no viewing before t=0)
        // Y-axis: 10% margin for readability
        let y_margin = bounds.y_range() * 0.10;

        // Ensure minimum Y range to avoid division issues
        let y_margin = y_margin.max(1e-12);

        self.view.x_min = bounds.x_min;
        self.view.x_max = bounds.x_max;
        self.view.y_min = bounds.y_min - y_margin;
        self.view.y_max = bounds.y_max + y_margin;

        self.has_initial_fit = true;
    }

    /// Parse a hex color string like "#3B82F6" to RGB tuple
    fn parse_hex_color(hex: &str) -> Option<(u8, u8, u8)> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some((r, g, b))
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

    // =========================================================================
    // Multi-Panel Management
    // =========================================================================

    /// Add a new panel
    pub fn add_panel(&mut self) -> usize {
        let id = self.panels.len();
        self.panels.push(WaveformPanel::new(id));
        self.redistribute_panel_heights();
        id
    }

    /// Remove a panel by index
    pub fn remove_panel(&mut self, index: usize) {
        if index < self.panels.len() && self.panels.len() > 1 {
            self.panels.remove(index);
            // Re-number panels
            for (i, panel) in self.panels.iter_mut().enumerate() {
                panel.id = i;
            }
            // Adjust active panel if needed
            if self.active_panel >= self.panels.len() {
                self.active_panel = self.panels.len().saturating_sub(1);
            }
            self.redistribute_panel_heights();
        }
    }

    /// Move a trace to a specific panel
    pub fn move_trace_to_panel(&mut self, trace_index: usize, panel_index: usize) {
        // Remove from all panels first
        for panel in &mut self.panels {
            panel.remove_trace(trace_index);
        }
        // Add to target panel
        if let Some(panel) = self.panels.get_mut(panel_index) {
            panel.add_trace(trace_index);
        }
    }

    /// Redistribute panel heights evenly
    pub fn redistribute_panel_heights(&mut self) {
        let visible_count = self.panels.iter().filter(|p| !p.collapsed).count();
        if visible_count > 0 {
            let height = 1.0 / visible_count as f32;
            for panel in &mut self.panels {
                if !panel.collapsed {
                    panel.height_fraction = height;
                }
            }
        }
    }

    /// Fit all panels' Y-axes to their traces
    pub fn fit_all_panels(&mut self) {
        let traces = self.traces.clone();
        for panel in &mut self.panels {
            if panel.auto_scale_y {
                panel.fit_y_to_traces(&traces);
            }
        }
    }

    /// Enable multi-panel mode and create default panel
    pub fn enable_multi_panel(&mut self) {
        if !self.multi_panel_enabled {
            self.multi_panel_enabled = true;
            if self.panels.is_empty() {
                // Create default panel with all traces
                let mut panel = WaveformPanel::new(0).with_label("All Signals");
                for i in 0..self.traces.len() {
                    panel.add_trace(i);
                }
                self.panels.push(panel);
            }
            self.fit_all_panels();
        }
    }

    /// Disable multi-panel mode
    pub fn disable_multi_panel(&mut self) {
        self.multi_panel_enabled = false;
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
        assert!(state.markers.is_empty());
        assert!(!state.show_measurements);
        assert!(!state.show_export);
        assert_eq!(state.measurement_scope, MeasurementScope::Visible);
        assert!(!state.measurement_use_cursor_range);
        assert!(state.export_status.is_none());
        assert!(state.selected_trace.is_none());
    }

    #[test]
    fn test_measurement_scope_metadata() {
        let all = MeasurementScope::all();
        assert_eq!(all.len(), 3);
        assert_eq!(MeasurementScope::Selected.display_name(), "Selected");
        assert_eq!(MeasurementScope::Visible.display_name(), "Visible");
        assert_eq!(MeasurementScope::All.display_name(), "All");
    }

    #[test]
    fn test_waveform_markers_are_sorted_and_deduplicated() {
        let mut state = WaveformViewerState::new();
        state.add_marker(10.0);
        state.add_marker(5.0);
        state.add_marker(10.0); // duplicate
        state.add_marker(f64::NAN); // ignored

        assert_eq!(state.markers, vec![5.0, 10.0]);
        assert!(state.remove_nearest_marker(9.8, 0.5));
        assert_eq!(state.markers, vec![5.0]);
        assert!(!state.remove_nearest_marker(9.8, 0.1));
        state.clear_markers();
        assert!(state.markers.is_empty());
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
