use serde::{Deserialize, Serialize};

use super::{DataBounds, TraceData};

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
