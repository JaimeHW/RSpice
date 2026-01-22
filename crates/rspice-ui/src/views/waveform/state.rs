//! Waveform viewer state types.
//!
//! Contains view state for zoom/pan, pane management, cursor positioning,
//! and box selection for the waveform viewer.

/// View state for zoom and pan (shared X-axis across all panes).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewState {
    /// X-axis range (time or frequency) - shared across all panes
    pub x_min: f64,
    pub x_max: f64,
    /// Y-axis range (default for single pane mode)
    pub y_min: f64,
    pub y_max: f64,
    /// Data bounds (for constraining pan/zoom to actual data)
    /// Format: (data_x_min, data_x_max, data_y_min, data_y_max)
    pub data_bounds: Option<(f64, f64, f64, f64)>,
    /// Whether we're currently panning
    pub is_panning: bool,
    /// Last mouse position during pan
    pub pan_start_x: f64,
    pub pan_start_y: f64,
    /// Plot area dimensions (updated on mouse events)
    pub plot_width: f64,
    pub plot_height: f64,
    /// True if mouse actually moved during pan/drag (prevents click on drag end)
    pub did_drag: bool,
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            x_min: 0.0,
            x_max: 5e-3, // 5ms default
            y_min: -1.5,
            y_max: 1.5,
            data_bounds: None,
            is_panning: false,
            pan_start_x: 0.0,
            pan_start_y: 0.0,
            plot_width: 800.0,
            plot_height: 400.0,
            did_drag: false,
        }
    }
}

impl ViewState {
    /// Constrain view to data bounds (prevents panning into empty regions).
    /// Called after pan/zoom operations to keep view within data range.
    /// X-axis lower bound is ALWAYS 0 (time can never be negative).
    fn clamp_to_bounds(&mut self) {
        // HARD RULE: Time can never be negative - always clamp x_min to >= 0
        if self.x_min < 0.0 {
            let shift = -self.x_min;
            self.x_min = 0.0;
            self.x_max += shift;
        }

        // Clamp to data bounds if they exist
        if let Some((_, data_x_max, data_y_min, data_y_max)) = self.data_bounds {
            let view_x_range = self.x_max - self.x_min;
            let view_y_range = self.y_max - self.y_min;
            let data_y_range = data_y_max - data_y_min;

            // Clamp X-axis max: don't pan beyond end of data
            if self.x_max > data_x_max && view_x_range < data_x_max * 2.0 {
                let shift = self.x_max - data_x_max;
                self.x_max = data_x_max;
                self.x_min = (self.x_min - shift).max(0.0); // Keep x_min >= 0
            }

            // Clamp Y-axis: allow some freedom but not too far from data
            if self.y_min < data_y_min && view_y_range < data_y_range * 3.0 {
                let shift = data_y_min - self.y_min;
                self.y_min = data_y_min;
                self.y_max += shift;
            }
            if self.y_max > data_y_max && view_y_range < data_y_range * 3.0 {
                let shift = self.y_max - data_y_max;
                self.y_max = data_y_max;
                self.y_min -= shift;
            }
        }
    }

    /// Zoom around a point (mouse position as fraction 0-1).
    pub fn zoom(&mut self, factor: f64, mouse_x_frac: f64, mouse_y_frac: f64) {
        let x_range = self.x_max - self.x_min;
        let y_range = self.y_max - self.y_min;

        // Calculate the point under the mouse in data coordinates
        let x_point = self.x_min + mouse_x_frac * x_range;
        let y_point = self.y_max - mouse_y_frac * y_range; // Y is inverted

        // Apply zoom factor
        let new_x_range = x_range * factor;
        let new_y_range = y_range * factor;

        // Reposition so the point stays under the mouse
        self.x_min = x_point - mouse_x_frac * new_x_range;
        self.x_max = x_point + (1.0 - mouse_x_frac) * new_x_range;
        self.y_min = y_point - (1.0 - mouse_y_frac) * new_y_range;
        self.y_max = y_point + mouse_y_frac * new_y_range;

        self.clamp_to_bounds();
    }

    /// Pan by delta (in data units).
    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.x_min += dx;
        self.x_max += dx;
        self.y_min += dy;
        self.y_max += dy;

        self.clamp_to_bounds();
    }

    /// Zoom X-axis only around a point (for horizontal-only zoom with Shift+scroll).
    /// mouse_x_frac: mouse position as fraction of plot width (0-1)
    pub fn zoom_x_only(&mut self, factor: f64, mouse_x_frac: f64) {
        let x_range = self.x_max - self.x_min;
        let x_point = self.x_min + mouse_x_frac * x_range;
        let new_x_range = x_range * factor;
        self.x_min = x_point - mouse_x_frac * new_x_range;
        self.x_max = x_point + (1.0 - mouse_x_frac) * new_x_range;
    }

    /// Zoom Y-axis only around a point (for vertical-only zoom with Ctrl+scroll).
    /// mouse_y_frac: mouse position as fraction of plot height (0-1, top=0)
    pub fn zoom_y_only(&mut self, factor: f64, mouse_y_frac: f64) {
        let y_range = self.y_max - self.y_min;
        let y_point = self.y_max - mouse_y_frac * y_range; // Y is inverted
        let new_y_range = y_range * factor;
        self.y_min = y_point - (1.0 - mouse_y_frac) * new_y_range;
        self.y_max = y_point + mouse_y_frac * new_y_range;
    }

    /// Fit X-axis (horizontal) to waveform data, preserving current Y-axis.
    pub fn fit_x_to_data(&mut self, waveforms: &[crate::state::WaveformData]) {
        if waveforms.is_empty() {
            return;
        }
        let mut x_min = f64::INFINITY;
        let mut x_max = f64::NEG_INFINITY;
        for wf in waveforms {
            for &x in &wf.x {
                x_min = x_min.min(x);
                x_max = x_max.max(x);
            }
        }
        if x_min.is_finite() && x_max.is_finite() {
            // Exact fit to data - no margins
            self.x_min = x_min;
            self.x_max = x_max;
        }
    }

    /// Fit Y-axis (vertical) to waveform data, preserving current X-axis.
    pub fn fit_y_to_data(&mut self, waveforms: &[crate::state::WaveformData]) {
        if waveforms.is_empty() {
            return;
        }
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;
        for wf in waveforms {
            for &y in &wf.y {
                y_min = y_min.min(y);
                y_max = y_max.max(y);
            }
        }
        if y_min.is_finite() && y_max.is_finite() {
            let y_margin = (y_max - y_min).max(0.1) * 0.1;
            self.y_min = y_min - y_margin;
            self.y_max = y_max + y_margin;
        }
    }

    /// Fit to waveform data.
    /// Sets data_bounds to constrain pan/zoom to actual data region.
    /// X-axis lower bound is always 0 (time can't be negative).
    pub fn fit_to_data(&mut self, waveforms: &[crate::state::WaveformData]) {
        if waveforms.is_empty() {
            *self = Self::default();
            return;
        }

        let mut x_max = f64::NEG_INFINITY;
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;

        for wf in waveforms {
            for &x in &wf.x {
                x_max = x_max.max(x);
            }
            for &y in &wf.y {
                y_min = y_min.min(y);
                y_max = y_max.max(y);
            }
        }

        // Time always starts at 0 (simulation time can't be negative)
        let data_x_min = 0.0;
        let data_x_max = if x_max.is_finite() { x_max } else { 5e-3 };

        // Add margins for Y-axis
        let y_margin = (y_max - y_min).max(0.1) * 0.1;
        let data_y_min = if y_min.is_finite() {
            y_min - y_margin
        } else {
            -1.5
        };
        let data_y_max = if y_max.is_finite() {
            y_max + y_margin
        } else {
            1.5
        };

        // Set view to data bounds (exact fit, no margin)
        self.x_min = data_x_min;
        self.x_max = data_x_max;
        self.y_min = data_y_min;
        self.y_max = data_y_max;

        // Store bounds for pan/zoom clamping
        self.data_bounds = Some((data_x_min, data_x_max, data_y_min, data_y_max));
    }
}

/// State for a single waveform pane (independent Y-axis).
#[derive(Debug, Clone, PartialEq)]
pub struct PaneState {
    /// Unique pane ID
    pub id: u32,
    /// Y-axis range for this pane
    pub y_min: f64,
    pub y_max: f64,
    /// Waveform indices assigned to this pane
    pub waveform_indices: Vec<usize>,
    /// Pane height ratio (1.0 = equal share)
    pub height_ratio: f64,
}

impl Default for PaneState {
    fn default() -> Self {
        Self {
            id: 0,
            y_min: -1.5,
            y_max: 1.5,
            waveform_indices: Vec::new(),
            height_ratio: 1.0,
        }
    }
}

impl PaneState {
    /// Create a new pane with the given ID.
    pub fn new(id: u32) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    /// Fit Y-axis to waveforms in this pane.
    pub fn fit_to_waveforms(&mut self, waveforms: &[crate::state::WaveformData]) {
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;

        for &idx in &self.waveform_indices {
            if let Some(wf) = waveforms.get(idx) {
                for &y in &wf.y {
                    y_min = y_min.min(y);
                    y_max = y_max.max(y);
                }
            }
        }

        if y_min.is_finite() && y_max.is_finite() {
            let margin = (y_max - y_min).max(0.1) * 0.1;
            self.y_min = y_min - margin;
            self.y_max = y_max + margin;
        }
    }
}

/// Cursor state for measurements.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CursorState {
    /// Primary cursor position (time value)
    pub cursor1: Option<f64>,
    /// Secondary cursor position (time value)
    pub cursor2: Option<f64>,
    /// Which cursor to place next (toggle between 1 and 2)
    pub next_cursor: u8,
}

impl CursorState {
    /// Get delta between cursors.
    pub fn delta(&self) -> Option<f64> {
        match (self.cursor1, self.cursor2) {
            (Some(c1), Some(c2)) => Some((c2 - c1).abs()),
            _ => None,
        }
    }

    /// Get frequency (1/delta).
    pub fn frequency(&self) -> Option<f64> {
        self.delta().map(|d| if d > 0.0 { 1.0 / d } else { 0.0 })
    }

    /// Place a cursor at the given time.
    pub fn place(&mut self, time: f64) {
        if self.next_cursor == 0 || self.cursor1.is_none() {
            self.cursor1 = Some(time);
            self.next_cursor = 1;
        } else {
            self.cursor2 = Some(time);
            self.next_cursor = 0;
        }
    }

    /// Clear all cursors.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.cursor1 = None;
        self.cursor2 = None;
        self.next_cursor = 0;
    }
}

/// Box selection for zoom-to-region feature.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct BoxSelection {
    /// Whether currently dragging to select
    pub is_selecting: bool,
    /// Start point (in data coordinates: x=time, y=voltage)
    pub start_x: f64,
    pub start_y: f64,
    /// Current end point (in data coordinates)
    pub end_x: f64,
    pub end_y: f64,
    /// Start point in element coordinates for drawing
    pub start_elem_x: f64,
    pub start_elem_y: f64,
    /// Plot element bounds (client coords) for global coordinate conversion
    /// Stores (left, top, width, height) of the plot element when selection started
    pub plot_rect: (f64, f64, f64, f64),
}

impl BoxSelection {
    /// Start a box selection at the given data coordinates.
    /// Also stores the plot element bounding rect for global coordinate conversion.
    pub fn start(
        &mut self,
        x: f64,
        y: f64,
        elem_x: f64,
        elem_y: f64,
        plot_rect: (f64, f64, f64, f64),
    ) {
        self.is_selecting = true;
        self.start_x = x;
        self.start_y = y;
        self.end_x = x;
        self.end_y = y;
        self.start_elem_x = elem_x;
        self.start_elem_y = elem_y;
        self.plot_rect = plot_rect;
    }

    /// Update the end point during drag.
    pub fn update(&mut self, x: f64, y: f64) {
        self.end_x = x;
        self.end_y = y;
    }

    /// Finish selection and return the selected region (x_min, x_max, y_min, y_max).
    pub fn finish(&mut self) -> Option<(f64, f64, f64, f64)> {
        if !self.is_selecting {
            return None;
        }
        self.is_selecting = false;

        // Normalize coordinates (ensure min < max)
        let x_min = self.start_x.min(self.end_x);
        let x_max = self.start_x.max(self.end_x);
        let y_min = self.start_y.min(self.end_y);
        let y_max = self.start_y.max(self.end_y);

        // Only return if selection has meaningful size
        let x_range = x_max - x_min;
        let y_range = y_max - y_min;
        if x_range > 1e-12 && y_range > 1e-12 {
            Some((x_min, x_max, y_min, y_max))
        } else {
            None
        }
    }

    /// Cancel the current box selection.
    pub fn cancel(&mut self) {
        self.is_selecting = false;
    }
}

/// State for multi-pane waveform viewer.
#[derive(Debug, Clone, Default)]
pub struct WaveformViewerState {
    /// Pane configurations (each pane has its own Y-axis range)
    pub panes: Vec<PaneState>,
    /// Which trace indices are assigned to which pane
    pub trace_pane_map: Vec<usize>,
    /// Active pane index (for keyboard focus)
    pub active_pane: usize,
}

impl WaveformViewerState {
    /// Create with a single default pane.
    pub fn new() -> Self {
        Self {
            panes: vec![PaneState::default()],
            trace_pane_map: Vec::new(),
            active_pane: 0,
        }
    }

    /// Add a new pane and return its index.
    pub fn add_pane(&mut self) -> usize {
        let id = self.panes.len() as u32;
        self.panes.push(PaneState::new(id));
        self.panes.len() - 1
    }

    /// Assign a trace to a pane.
    pub fn assign_trace_to_pane(&mut self, trace_idx: usize, pane_idx: usize) {
        // Extend trace_pane_map if needed
        while self.trace_pane_map.len() <= trace_idx {
            self.trace_pane_map.push(0); // Default to first pane
        }
        self.trace_pane_map[trace_idx] = pane_idx;

        // Update pane's waveform_indices
        if let Some(pane) = self.panes.get_mut(pane_idx) {
            if !pane.waveform_indices.contains(&trace_idx) {
                pane.waveform_indices.push(trace_idx);
            }
        }
    }

    /// Get pane index for a trace (defaults to 0).
    #[allow(dead_code)]
    pub fn get_trace_pane(&self, trace_idx: usize) -> usize {
        self.trace_pane_map.get(trace_idx).copied().unwrap_or(0)
    }
}
