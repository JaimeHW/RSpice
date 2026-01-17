//! Waveform Viewer
//!
//! Canvas-based waveform plotting with zoom, pan, and cursor support.
//! Supports GPU-accelerated rendering when available.

use dioxus::html::input_data::keyboard_types::Key;
use dioxus::prelude::*;
use std::sync::{Arc, Mutex};

use crate::state::waveform_math;
use crate::state::SimulationState;
use crate::theme::Theme;
use crate::views::waveform_gpu::{
    is_gpu_available, WaveformGpuState, WaveformPainter, WaveformTrace,
};

/// View state for zoom and pan (shared X-axis across all panes)
#[derive(Debug, Clone, Copy, PartialEq)]
struct ViewState {
    /// X-axis range (time or frequency) - shared across all panes
    x_min: f64,
    x_max: f64,
    /// Y-axis range (default for single pane mode)
    y_min: f64,
    y_max: f64,
    /// Whether we're currently panning
    is_panning: bool,
    /// Last mouse position during pan
    pan_start_x: f64,
    pan_start_y: f64,
    /// Plot area dimensions (updated on mouse events)
    plot_width: f64,
    plot_height: f64,
    /// True if mouse actually moved during pan/drag (prevents click on drag end)
    did_drag: bool,
}

/// State for a single waveform pane (independent Y-axis)
#[derive(Debug, Clone, PartialEq)]
struct PaneState {
    /// Unique pane ID
    id: u32,
    /// Y-axis range for this pane
    y_min: f64,
    y_max: f64,
    /// Waveform indices assigned to this pane
    waveform_indices: Vec<usize>,
    /// Pane height ratio (1.0 = equal share)
    height_ratio: f64,
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
    fn new(id: u32) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    /// Fit Y-axis to waveforms in this pane
    fn fit_to_waveforms(&mut self, waveforms: &[crate::state::WaveformData]) {
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

/// Cursor state for measurements
#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct CursorState {
    /// Primary cursor position (time value)
    cursor1: Option<f64>,
    /// Secondary cursor position (time value)
    cursor2: Option<f64>,
    /// Which cursor to place next (toggle between 1 and 2)
    next_cursor: u8,
}

/// Cursor value at a specific time for a single trace
#[derive(Debug, Clone, Default)]
struct CursorValue {
    /// Trace name
    name: String,
    /// Trace color for display
    color: String,
    /// Interpolated Y value at cursor X
    value: f64,
}

/// Cursor values for all traces at a cursor position
#[derive(Debug, Clone, Default)]
struct CursorValues {
    /// X position (time)
    x: f64,
    /// Values per trace
    traces: Vec<CursorValue>,
}

impl CursorState {
    /// Get delta between cursors
    fn delta(&self) -> Option<f64> {
        match (self.cursor1, self.cursor2) {
            (Some(c1), Some(c2)) => Some((c2 - c1).abs()),
            _ => None,
        }
    }

    /// Get frequency (1/delta)
    fn frequency(&self) -> Option<f64> {
        self.delta().map(|d| if d > 0.0 { 1.0 / d } else { 0.0 })
    }

    /// Place a cursor at the given time
    fn place(&mut self, time: f64) {
        if self.next_cursor == 0 || self.cursor1.is_none() {
            self.cursor1 = Some(time);
            self.next_cursor = 1;
        } else {
            self.cursor2 = Some(time);
            self.next_cursor = 0;
        }
    }

    /// Clear all cursors
    #[allow(dead_code)]
    fn clear(&mut self) {
        self.cursor1 = None;
        self.cursor2 = None;
        self.next_cursor = 0;
    }
}

/// Interpolate Y value at a specific X position
fn interpolate_y_at_x(x_data: &[f64], y_data: &[f64], target_x: f64) -> Option<f64> {
    if x_data.len() != y_data.len() || x_data.is_empty() {
        return None;
    }

    // Find the two points bracketing target_x
    let mut left_idx = 0;
    for (i, &x) in x_data.iter().enumerate() {
        if x <= target_x {
            left_idx = i;
        } else {
            break;
        }
    }

    // Handle edge cases
    if left_idx >= x_data.len() - 1 {
        return Some(y_data[y_data.len() - 1]);
    }

    let right_idx = left_idx + 1;
    let x0 = x_data[left_idx];
    let x1 = x_data[right_idx];
    let y0 = y_data[left_idx];
    let y1 = y_data[right_idx];

    // Linear interpolation
    if (x1 - x0).abs() < 1e-15 {
        return Some(y0);
    }

    let t = (target_x - x0) / (x1 - x0);
    Some(y0 + t * (y1 - y0))
}

/// Get cursor values for all visible traces at a given X position
fn get_cursor_values(x_pos: f64, waveforms: &[crate::state::WaveformData]) -> CursorValues {
    let mut values = CursorValues {
        x: x_pos,
        traces: Vec::new(),
    };

    for wf in waveforms.iter().filter(|w| w.visible) {
        if let Some(y_val) = interpolate_y_at_x(&wf.x, &wf.y, x_pos) {
            values.traces.push(CursorValue {
                name: wf.name.clone(),
                color: wf.color.clone(),
                value: y_val,
            });
        }
    }

    values
}

/// Box selection for zoom-to-region feature
#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct BoxSelection {
    /// Whether currently dragging to select
    is_selecting: bool,
    /// Start point (in data coordinates: x=time, y=voltage)
    start_x: f64,
    start_y: f64,
    /// Current end point (in data coordinates)
    end_x: f64,
    end_y: f64,
    /// Start point in element coordinates for drawing
    start_elem_x: f64,
    start_elem_y: f64,
}

impl BoxSelection {
    /// Start a box selection at the given data coordinates
    fn start(&mut self, x: f64, y: f64, elem_x: f64, elem_y: f64) {
        self.is_selecting = true;
        self.start_x = x;
        self.start_y = y;
        self.end_x = x;
        self.end_y = y;
        self.start_elem_x = elem_x;
        self.start_elem_y = elem_y;
    }

    /// Update the end point during drag
    fn update(&mut self, x: f64, y: f64) {
        self.end_x = x;
        self.end_y = y;
    }

    /// Finish selection and return the selected region (x_min, x_max, y_min, y_max)
    fn finish(&mut self) -> Option<(f64, f64, f64, f64)> {
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

    fn cancel(&mut self) {
        self.is_selecting = false;
    }
}

/// State for multi-pane waveform viewer
#[derive(Debug, Clone, Default)]
struct WaveformViewerState {
    /// Pane configurations (each pane has its own Y-axis range)
    panes: Vec<PaneState>,
    /// Which trace indices are assigned to which pane
    trace_pane_map: Vec<usize>,
    /// Active pane index (for keyboard focus)
    active_pane: usize,
}

impl WaveformViewerState {
    /// Create with a single default pane
    fn new() -> Self {
        Self {
            panes: vec![PaneState::default()],
            trace_pane_map: Vec::new(),
            active_pane: 0,
        }
    }

    /// Add a new pane and return its index
    fn add_pane(&mut self) -> usize {
        let id = self.panes.len() as u32;
        self.panes.push(PaneState::new(id));
        self.panes.len() - 1
    }

    /// Assign a trace to a pane
    fn assign_trace_to_pane(&mut self, trace_idx: usize, pane_idx: usize) {
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

    /// Get pane index for a trace (defaults to 0)
    fn get_trace_pane(&self, trace_idx: usize) -> usize {
        self.trace_pane_map.get(trace_idx).copied().unwrap_or(0)
    }
}

/// Waveform measurements
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
struct WaveformMeasurements {
    vpp: f64,  // Peak-to-peak voltage
    vmax: f64, // Maximum voltage
    vmin: f64, // Minimum voltage
    vavg: f64, // Average voltage
    vrms: f64, // RMS voltage
}

impl WaveformMeasurements {
    /// Calculate measurements from Y data
    fn from_data(y: &[f64]) -> Self {
        if y.is_empty() {
            return Self::default();
        }

        let vmax = y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let vmin = y.iter().cloned().fold(f64::INFINITY, f64::min);
        let vpp = vmax - vmin;
        let vavg = y.iter().sum::<f64>() / y.len() as f64;
        let vrms = (y.iter().map(|v| v * v).sum::<f64>() / y.len() as f64).sqrt();

        Self {
            vpp,
            vmax,
            vmin,
            vavg,
            vrms,
        }
    }
}

/// Interpolate Y value at a given X position
fn interpolate_y(x: &[f64], y: &[f64], target_x: f64) -> Option<f64> {
    if x.len() < 2 || y.len() < 2 || x.len() != y.len() {
        return None;
    }

    // Find the two points surrounding target_x
    for i in 0..x.len() - 1 {
        if x[i] <= target_x && target_x <= x[i + 1] {
            // Linear interpolation
            let t = (target_x - x[i]) / (x[i + 1] - x[i]);
            return Some(y[i] + t * (y[i + 1] - y[i]));
        }
    }
    None
}

/// Format frequency with appropriate SI prefix
fn format_frequency(f: f64) -> String {
    if f >= 1e9 {
        format!("{:.2} GHz", f / 1e9)
    } else if f >= 1e6 {
        format!("{:.2} MHz", f / 1e6)
    } else if f >= 1e3 {
        format!("{:.2} kHz", f / 1e3)
    } else if f >= 1.0 {
        format!("{:.2} Hz", f)
    } else if f >= 1e-3 {
        format!("{:.2} mHz", f * 1e3)
    } else {
        format!("{:.4} Hz", f)
    }
}

/// Format voltage with appropriate SI prefix
fn format_voltage(v: f64) -> String {
    let abs_v = v.abs();
    if abs_v >= 1e3 {
        format!("{:.3} kV", v / 1e3)
    } else if abs_v >= 1.0 {
        format!("{:.3} V", v)
    } else if abs_v >= 1e-3 {
        format!("{:.3} mV", v * 1e3)
    } else if abs_v >= 1e-6 {
        format!("{:.3} µV", v * 1e6)
    } else {
        format!("{:.3} nV", v * 1e9)
    }
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            x_min: 0.0,
            x_max: 5e-3, // 5ms default
            y_min: -1.5,
            y_max: 1.5,
            is_panning: false,
            pan_start_x: 0.0,
            pan_start_y: 0.0,
            plot_width: 800.0, // Default, updated dynamically on mouse events
            plot_height: 400.0,
            did_drag: false,
        }
    }
}

impl ViewState {
    /// Zoom around a point (mouse position as fraction 0-1)
    fn zoom(&mut self, factor: f64, mouse_x_frac: f64, mouse_y_frac: f64) {
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
    }

    /// Pan by delta (in data units)
    fn pan(&mut self, dx: f64, dy: f64) {
        self.x_min += dx;
        self.x_max += dx;
        self.y_min += dy;
        self.y_max += dy;
    }

    /// Fit to waveform data
    fn fit_to_data(&mut self, waveforms: &[crate::state::WaveformData]) {
        if waveforms.is_empty() {
            *self = Self::default();
            return;
        }

        let mut x_min = f64::INFINITY;
        let mut x_max = f64::NEG_INFINITY;
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;

        for wf in waveforms {
            for &x in &wf.x {
                x_min = x_min.min(x);
                x_max = x_max.max(x);
            }
            for &y in &wf.y {
                y_min = y_min.min(y);
                y_max = y_max.max(y);
            }
        }

        // Add 10% margin
        let x_margin = (x_max - x_min) * 0.05;
        let y_margin = (y_max - y_min) * 0.1;

        self.x_min = x_min - x_margin;
        self.x_max = x_max + x_margin;
        self.y_min = y_min - y_margin;
        self.y_max = y_max + y_margin;
    }
}

/// Waveform viewer component
#[component]
pub fn Waveform() -> Element {
    let theme: Signal<Theme> = use_context();
    let mut sim_state: Signal<SimulationState> = use_context();
    let th = theme.read();

    let waveforms = &sim_state.read().waveforms;
    let mut view_state = use_signal(ViewState::default);
    let cursor_state = use_signal(CursorState::default);
    let box_selection = use_signal(BoxSelection::default);
    let _viewer_state = use_signal(WaveformViewerState::new);

    // Auto-fit view when waveforms change
    let _waveform_count = waveforms.len();
    use_effect(move || {
        let waveforms = &sim_state.read().waveforms;
        if !waveforms.is_empty() {
            view_state.write().fit_to_data(waveforms);
        }
    });

    rsx! {
        div {
            class: "waveform-viewer",
            style: "
                display: flex;
                flex-direction: column;
                width: 100%;
                height: 100%;
                background: {th.bg_secondary()};
            ",

            // Waveform header with controls
            WaveformHeader {
                on_fit: move |_| {
                    let waveforms = &sim_state.read().waveforms;
                    view_state.write().fit_to_data(waveforms);
                },
                on_zoom_in: move |_| {
                    view_state.write().zoom(0.8, 0.5, 0.5);
                },
                on_zoom_out: move |_| {
                    view_state.write().zoom(1.25, 0.5, 0.5);
                },
                on_add_trace: move |expr: String| {
                    // Build signal map from existing waveforms
                    let state = sim_state.read();
                    let signals: std::collections::HashMap<String, (Vec<f64>, Vec<f64>)> =
                        state.waveforms.iter()
                            .map(|wf| (wf.name.clone(), (wf.x.clone(), wf.y.clone())))
                            .collect();
                    drop(state);

                    // Try to evaluate the expression
                    match waveform_math::eval_expression(&expr, &signals) {
                        Ok((x_data, y_data)) => {
                            // Generate a color for the new trace
                            let trace_count = sim_state.read().waveforms.len();
                            let theme: Signal<Theme> = use_context();
                            let color = theme.read().trace_color(trace_count).to_string();

                            // Add the computed waveform
                            sim_state.write().waveforms.push(crate::state::WaveformData {
                                name: expr,
                                x: x_data,
                                y: y_data,
                                color,
                                visible: true,
                            });
                        }
                        Err(e) => {
                            // Log error (in a real app would show in UI)
                            eprintln!("Expression error: {}", e);
                        }
                    }
                },
            }

            // Main waveform area
            div {
                style: "
                    flex: 1;
                    display: flex;
                    overflow: hidden;
                ",

                // Y-axis labels
                YAxisLabels { view: *view_state.read() }

                // Plot area with zoom/pan/cursors/box selection
                WaveformPlotArea {
                    view_state: view_state,
                    cursor_state: cursor_state,
                    box_selection: box_selection,
                    waveforms: waveforms.clone(),
                }

                // Legend
                div {
                    class: "legend",
                    style: "
                        width: 120px;
                        padding: {Theme::SPACING_SM};
                        border-left: 1px solid {th.border()};
                        overflow-y: auto;
                    ",

                    div {
                        style: "
                            font-size: {Theme::FONT_SIZE_SM};
                            font-weight: 600;
                            color: {th.text_secondary()};
                            margin-bottom: {Theme::SPACING_SM};
                        ",
                        "Traces"
                    }

                    if waveforms.is_empty() {
                        // Demo legend
                        LegendItem {
                            name: "V(out)".to_string(),
                            color: th.trace_color(0).to_string(),
                            visible: true,
                            on_toggle: move |_| {}, // Demo, no toggle
                        }
                    } else {
                        for (idx, wf) in waveforms.iter().enumerate() {
                            LegendItem {
                                key: "{wf.name}-{idx}",
                                name: wf.name.clone(),
                                color: wf.color.clone(),
                                visible: wf.visible,
                                on_toggle: move |_| {
                                    // Toggle visibility in SimulationState
                                    if let Some(wf) = sim_state.write().waveforms.get_mut(idx) {
                                        wf.visible = !wf.visible;
                                    }
                                },
                            }
                        }
                    }
                }
            }

            // X-axis with time labels
            XAxisLabels { view: *view_state.read() }
        }
    }
}

/// Plot area with mouse interaction for zoom/pan
#[component]
fn WaveformPlotArea(
    mut view_state: Signal<ViewState>,
    mut cursor_state: Signal<CursorState>,
    mut box_selection: Signal<BoxSelection>,
    waveforms: Vec<crate::state::WaveformData>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let view = *view_state.read();
    let cursors = *cursor_state.read();
    let box_sel = *box_selection.read();

    // Determine cursor style based on pan state
    let cursor_style = if view.is_panning {
        "grabbing"
    } else {
        "crosshair"
    };

    rsx! {
        div {
            class: "plot-area",
            style: "
                flex: 1;
                position: relative;
                background: {th.bg_primary()};
                overflow: hidden;
                cursor: {cursor_style};
                user-select: none;
                -webkit-user-select: none;
            ",

            // Get actual element dimensions on mount
            onmounted: move |e| {
                spawn(async move {
                    if let Ok(rect) = e.data().get_client_rect().await {
                        view_state.write().plot_width = rect.width().max(100.0);
                        view_state.write().plot_height = rect.height().max(100.0);
                    }
                });
            },

            // Mouse wheel zoom
            onwheel: move |e| {
                let delta = e.delta();
                let delta_y = match delta {
                    dioxus::html::geometry::WheelDelta::Pixels(p) => p.y,
                    dioxus::html::geometry::WheelDelta::Lines(l) => l.y * 20.0,
                    dioxus::html::geometry::WheelDelta::Pages(p) => p.y * 100.0,
                };
                let factor = if delta_y > 0.0 { 1.15 } else { 0.87 };
                // Zoom at center for now (could use mouse position with element coords)
                view_state.write().zoom(factor, 0.5, 0.5);
            },

            // Pan with mouse drag, or box zoom with shift+drag
            onmousedown: move |e| {
                // Prevent native browser drag behavior
                e.prevent_default();

                let elem = e.element_coordinates();
                let vs = *view_state.read();

                // Convert element coords to data coords for box selection
                let plot_width = vs.plot_width.max(100.0);
                let plot_height = vs.plot_height.max(100.0);
                let data_x = vs.x_min + (elem.x / plot_width) * (vs.x_max - vs.x_min);
                let data_y = vs.y_max - (elem.y / plot_height) * (vs.y_max - vs.y_min);

                if e.modifiers().shift() {
                    // Shift+drag = box zoom selection
                    box_selection.write().start(data_x, data_y, elem.x, elem.y);
                } else {
                    // Normal drag = pan
                    let mut vs = view_state.write();
                    vs.is_panning = true;
                    vs.pan_start_x = elem.x;
                    vs.pan_start_y = elem.y;
                    vs.did_drag = false; // Reset drag flag
                }
            },

            onmouseup: move |_| {
                // Finish box selection if active
                if let Some((x_min, x_max, y_min, y_max)) = box_selection.write().finish() {
                    // Zoom to selected region
                    let mut vs = view_state.write();
                    vs.x_min = x_min;
                    vs.x_max = x_max;
                    vs.y_min = y_min;
                    vs.y_max = y_max;
                }
                view_state.write().is_panning = false;
            },

            onmouseleave: move |_| {
                box_selection.write().cancel();
                view_state.write().is_panning = false;
            },

            // Click to place cursor (only if not dragging)
            onclick: move |e| {
                // Skip cursor placement if we just finished a drag
                if view_state.read().did_drag {
                    view_state.write().did_drag = false;
                    return;
                }

                let elem = e.element_coordinates();
                let vs = *view_state.read();
                let plot_width = vs.plot_width.max(100.0);

                // Convert element X to data X (time)
                let data_x = vs.x_min + (elem.x / plot_width) * (vs.x_max - vs.x_min);

                // Don't place cursor if we're in a selection operation
                if !box_selection.read().is_selecting {
                    cursor_state.write().place(data_x);
                }
            },

            onmousemove: move |e| {
                let elem = e.element_coordinates();
                let vs = *view_state.read();
                let plot_width = vs.plot_width.max(100.0);
                let plot_height = vs.plot_height.max(100.0);

                // Update box selection if active
                if box_selection.read().is_selecting {
                    let data_x = vs.x_min + (elem.x / plot_width) * (vs.x_max - vs.x_min);
                    let data_y = vs.y_max - (elem.y / plot_height) * (vs.y_max - vs.y_min);
                    box_selection.write().update(data_x, data_y);
                } else if vs.is_panning {
                    // Calculate pixel deltas using element-relative coordinates
                    let pixel_dx = elem.x - vs.pan_start_x;
                    let pixel_dy = elem.y - vs.pan_start_y;

                    let x_range = vs.x_max - vs.x_min;
                    let y_range = vs.y_max - vs.y_min;

                    // Convert pixel movement to data coordinate movement
                    let data_dx = -(pixel_dx / plot_width) * x_range;
                    let data_dy = (pixel_dy / plot_height) * y_range;

                    let mut new_vs = view_state.write();
                    new_vs.pan(data_dx, data_dy);
                    new_vs.pan_start_x = elem.x;
                    new_vs.pan_start_y = elem.y;
                    new_vs.did_drag = true; // Mark that a drag occurred
                }
            },

            // Grid lines
            WaveformGrid {}

            // Waveform traces - GPU or SVG rendering
            if is_gpu_available() {
                // GPU-accelerated rendering - key forces re-render on view change
                {
                    let view_key = format!("{:.6}_{:.6}_{:.6}_{:.6}", view.x_min, view.x_max, view.y_min, view.y_max);
                    rsx! {
                        GpuWaveformCanvas {
                            key: "{view_key}",
                            view: view,
                            waveforms: waveforms.clone(),
                        }
                    }
                }
            } else {
                // SVG fallback
                if waveforms.is_empty() {
                    {
                        let n = 500;
                        let x_range = view.x_max - view.x_min;
                        let step = x_range / (n - 1) as f64;
                        let demo_x: Vec<f64> = (0..n).map(|i| view.x_min + i as f64 * step).collect();
                        let demo_y: Vec<f64> = demo_x.iter()
                            .map(|t| (2.0 * std::f64::consts::PI * 1000.0 * t).sin())
                            .collect();
                        let demo_name = "demo";
                        rsx! {
                            WaveformTraceView {
                                key: "{demo_name}",
                                x: demo_x,
                                y: demo_y,
                                color: "#22c55e".to_string(),
                                view: view,
                            }
                        }
                    }
                } else {
                    for wf in waveforms.iter().filter(|w| w.visible) {
                        WaveformTraceView {
                            key: "{wf.name}",
                            x: wf.x.clone(),
                            y: wf.y.clone(),
                            color: wf.color.clone(),
                            view: view,
                        }
                    }
                }
            }

            // Cursor lines
            svg {
                style: "
                    position: absolute;
                    inset: 0;
                    width: 100%;
                    height: 100%;
                    pointer-events: none;
                ",

                // Cursor 1 (yellow)
                if let Some(c1) = cursors.cursor1 {
                    {
                        let x_frac = (c1 - view.x_min) / (view.x_max - view.x_min);
                        let x_pct = format!("{}%", x_frac * 100.0);
                        rsx! {
                            line {
                                x1: "{x_pct}",
                                y1: "0",
                                x2: "{x_pct}",
                                y2: "100%",
                                stroke: "#eab308",
                                stroke_width: "1",
                                stroke_dasharray: "4",
                            }
                        }
                    }
                }

                // Cursor 2 (cyan)
                if let Some(c2) = cursors.cursor2 {
                    {
                        let x_frac = (c2 - view.x_min) / (view.x_max - view.x_min);
                        let x_pct = format!("{}%", x_frac * 100.0);
                        rsx! {
                            line {
                                x1: "{x_pct}",
                                y1: "0",
                                x2: "{x_pct}",
                                y2: "100%",
                                stroke: "#06b6d4",
                                stroke_width: "1",
                                stroke_dasharray: "4",
                            }
                        }
                    }
                }
            }

            // Box selection overlay (during Shift+drag)
            if box_sel.is_selecting {
                {
                    let x1_frac = ((box_sel.start_x - view.x_min) / (view.x_max - view.x_min)).clamp(0.0, 1.0);
                    let x2_frac = ((box_sel.end_x - view.x_min) / (view.x_max - view.x_min)).clamp(0.0, 1.0);
                    let y1_frac = ((view.y_max - box_sel.start_y) / (view.y_max - view.y_min)).clamp(0.0, 1.0);
                    let y2_frac = ((view.y_max - box_sel.end_y) / (view.y_max - view.y_min)).clamp(0.0, 1.0);

                    let x_pct = x1_frac.min(x2_frac) * 100.0;
                    let y_pct = y1_frac.min(y2_frac) * 100.0;
                    let w_pct = (x1_frac - x2_frac).abs() * 100.0;
                    let h_pct = (y1_frac - y2_frac).abs() * 100.0;

                    rsx! {
                        div {
                            style: "
                                position: absolute;
                                left: {x_pct}%;
                                top: {y_pct}%;
                                width: {w_pct}%;
                                height: {h_pct}%;
                                background: rgba(59, 130, 246, 0.2);
                                border: 2px solid #3b82f6;
                                pointer-events: none;
                                z-index: 20;
                            "
                        }
                    }
                }
            }

            // Cursor readout overlay with per-trace values
            if cursors.cursor1.is_some() || cursors.cursor2.is_some() {
                {
                    // Compute cursor values for each cursor
                    let c1_values = cursors.cursor1.map(|t| get_cursor_values(t, &waveforms));
                    let c2_values = cursors.cursor2.map(|t| get_cursor_values(t, &waveforms));

                    rsx! {
                        div {
                            style: "
                                position: absolute;
                                top: 8px;
                                right: 8px;
                                background: rgba(0, 0, 0, 0.9);
                                padding: 10px 14px;
                                border-radius: 8px;
                                font-family: {Theme::FONT_MONO};
                                font-size: 11px;
                                min-width: 180px;
                                max-width: 280px;
                                box-shadow: 0 4px 12px rgba(0,0,0,0.4);
                                border: 1px solid rgba(255,255,255,0.1);
                                z-index: 30;
                            ",

                            // Title
                            div {
                                style: "color: #9ca3af; font-size: 10px; margin-bottom: 8px; text-transform: uppercase; letter-spacing: 0.5px; border-bottom: 1px solid rgba(255,255,255,0.1); padding-bottom: 4px;",
                                "Cursor Readout"
                            }

                            // Cursor 1 section
                            if let Some(ref vals) = c1_values {
                                div {
                                    style: "margin-bottom: 8px;",
                                    div {
                                        style: "color: #eab308; font-weight: 600; margin-bottom: 4px; display: flex; justify-content: space-between;",
                                        span { "C1:" }
                                        span { "{format_time(vals.x)}" }
                                    }
                                    for tv in vals.traces.iter() {
                                        div {
                                            style: "display: flex; justify-content: space-between; padding: 1px 0; color: {tv.color};",
                                            span { style: "opacity: 0.8;", "{tv.name}:" }
                                            span { style: "font-weight: 500;", "{format_voltage(tv.value)}" }
                                        }
                                    }
                                }
                            }

                            // Cursor 2 section
                            if let Some(ref vals) = c2_values {
                                div {
                                    style: "margin-bottom: 8px;",
                                    div {
                                        style: "color: #06b6d4; font-weight: 600; margin-bottom: 4px; display: flex; justify-content: space-between;",
                                        span { "C2:" }
                                        span { "{format_time(vals.x)}" }
                                    }
                                    for tv in vals.traces.iter() {
                                        div {
                                            style: "display: flex; justify-content: space-between; padding: 1px 0; color: {tv.color};",
                                            span { style: "opacity: 0.8;", "{tv.name}:" }
                                            span { style: "font-weight: 500;", "{format_voltage(tv.value)}" }
                                        }
                                    }
                                }
                            }

                            // Delta and frequency
                            if let Some(delta) = cursors.delta() {
                                div {
                                    style: "border-top: 1px solid rgba(255,255,255,0.1); margin-top: 4px; padding-top: 6px;",
                                    div {
                                        style: "color: #22c55e; margin-bottom: 2px; display: flex; justify-content: space-between;",
                                        span { "Δt:" }
                                        span { style: "font-weight: 500;", "{format_time(delta)}" }
                                    }
                                    if let Some(freq) = cursors.frequency() {
                                        div {
                                            style: "color: #a78bfa; display: flex; justify-content: space-between;",
                                            span { "Freq:" }
                                            span { style: "font-weight: 500;", "{format_frequency(freq)}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Format time with appropriate SI prefix
fn format_time(t: f64) -> String {
    let abs_t = t.abs();
    if abs_t < 1e-9 {
        format!("{:.2}ps", t * 1e12)
    } else if abs_t < 1e-6 {
        format!("{:.2}ns", t * 1e9)
    } else if abs_t < 1e-3 {
        format!("{:.2}µs", t * 1e6)
    } else if abs_t < 1.0 {
        format!("{:.2}ms", t * 1e3)
    } else {
        format!("{:.2}s", t)
    }
}

/// GPU-accelerated waveform canvas component
#[component]
fn GpuWaveformCanvas(view: ViewState, waveforms: Vec<crate::state::WaveformData>) -> Element {
    // Build traces from waveform data
    let traces: Vec<WaveformTrace> = if waveforms.is_empty() {
        // Demo waveform - generate X values covering current view
        let n = 1000;
        let x_range = view.x_max - view.x_min;
        let step = x_range / (n - 1) as f64;
        let x: Vec<f64> = (0..n).map(|i| view.x_min + i as f64 * step).collect();
        let y: Vec<f64> = x
            .iter()
            .map(|t| (2.0 * std::f64::consts::PI * 1000.0 * t).sin())
            .collect();
        vec![WaveformTrace {
            x,
            y,
            color: [0.133, 0.773, 0.369, 1.0], // #22c55e
            name: "V(out)".to_string(),
        }]
    } else {
        waveforms
            .iter()
            .map(|wf| {
                let color = parse_hex_color(&wf.color);
                WaveformTrace {
                    x: wf.x.clone(),
                    y: wf.y.clone(),
                    color,
                    name: wf.name.clone(),
                }
            })
            .collect()
    };

    // Create GPU state and render
    let gpu_state = Arc::new(Mutex::new(WaveformGpuState {
        traces,
        x_min: view.x_min,
        x_max: view.x_max,
        y_min: view.y_min,
        y_max: view.y_max,
        dirty: true,
    }));

    let mut painter = WaveformPainter::new(gpu_state);
    // Render at higher resolution for sharper display on large monitors
    let img_src = painter.render_to_base64(2400, 600).unwrap_or_default();

    rsx! {
        if !img_src.is_empty() {
            img {
                style: "
                    position: absolute;
                    inset: 0;
                    width: 100%;
                    height: 100%;
                    object-fit: fill;
                    image-rendering: auto;
                ",
                src: "{img_src}",
            }
        }
    }
}

/// Parse hex color to RGBA floats
fn parse_hex_color(hex: &str) -> [f32; 4] {
    let hex = hex.trim_start_matches('#');
    if hex.len() >= 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255) as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255) as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255) as f32 / 255.0;
        [r, g, b, 1.0]
    } else {
        [1.0, 1.0, 1.0, 1.0]
    }
}

/// Y-axis labels with actual values
#[component]
fn YAxisLabels(view: ViewState) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let labels = generate_axis_labels(view.y_min, view.y_max, 5);

    rsx! {
        div {
            class: "y-axis",
            style: "
                width: 60px;
                display: flex;
                flex-direction: column;
                justify-content: space-between;
                padding: {Theme::SPACING_SM} {Theme::SPACING_XS};
                font-family: {Theme::FONT_MONO};
                font-size: 10px;
                color: {th.text_muted()};
                text-align: right;
                border-right: 1px solid {th.border()};
            ",
            for label in labels.iter().rev() {
                span { "{label}" }
            }
        }
    }
}

/// X-axis labels with actual values
#[component]
fn XAxisLabels(view: ViewState) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let labels = generate_time_labels(view.x_min, view.x_max, 6);

    rsx! {
        div {
            class: "x-axis",
            style: "
                display: flex;
                justify-content: space-between;
                padding: {Theme::SPACING_XS} {Theme::SPACING_MD};
                padding-left: 70px;
                padding-right: 130px;
                font-family: {Theme::FONT_MONO};
                font-size: 10px;
                color: {th.text_muted()};
                border-top: 1px solid {th.border()};
                background: {th.bg_tertiary()};
            ",
            for label in labels.iter() {
                span { "{label}" }
            }
        }
    }
}

/// Generate axis labels
fn generate_axis_labels(min: f64, max: f64, count: usize) -> Vec<String> {
    let step = (max - min) / (count - 1) as f64;
    (0..count)
        .map(|i| {
            let val = min + i as f64 * step;
            format!("{:.2}V", val)
        })
        .collect()
}

/// Generate time labels with appropriate SI prefix
fn generate_time_labels(min: f64, max: f64, count: usize) -> Vec<String> {
    let range = max - min;
    let (scale, suffix) = if range < 1e-6 {
        (1e9, "ns")
    } else if range < 1e-3 {
        (1e6, "µs")
    } else if range < 1.0 {
        (1e3, "ms")
    } else {
        (1.0, "s")
    };

    let step = range / (count - 1) as f64;
    (0..count)
        .map(|i| {
            let val = (min + i as f64 * step) * scale;
            format!("{:.1}{}", val, suffix)
        })
        .collect()
}

/// Waveform header with controls and expression input
#[component]
fn WaveformHeader(
    on_fit: EventHandler<MouseEvent>,
    on_zoom_in: EventHandler<MouseEvent>,
    on_zoom_out: EventHandler<MouseEvent>,
    on_add_trace: EventHandler<String>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let mut expr_input = use_signal(|| String::new());
    let mut error_msg = use_signal(|| Option::<String>::None);

    rsx! {
        div {
            style: "
                display: flex;
                align-items: center;
                padding: {Theme::SPACING_XS} {Theme::SPACING_MD};
                background: {th.bg_tertiary()};
                border-bottom: 1px solid {th.border()};
                gap: {Theme::SPACING_MD};
            ",

            // Title
            span {
                style: "
                    font-size: {Theme::FONT_SIZE_SM};
                    font-weight: 600;
                    color: {th.text_secondary()};
                ",
                "Waveform Viewer"
            }

            // Expression input
            div {
                style: "
                    display: flex;
                    align-items: center;
                    gap: {Theme::SPACING_XS};
                    flex: 1;
                    max-width: 300px;
                ",

                input {
                    r#type: "text",
                    placeholder: "Add trace: V(out), I(R1)*1000, db(V(out))...",
                    value: "{expr_input}",
                    style: "
                        flex: 1;
                        padding: 4px 8px;
                        background: {th.bg_primary()};
                        border: 1px solid {th.border()};
                        border-radius: {Theme::RADIUS_SM};
                        color: {th.text_primary()};
                        font-size: 11px;
                        font-family: {Theme::FONT_MONO};
                        outline: none;
                    ",
                    oninput: move |e| {
                        expr_input.set(e.value().clone());
                        error_msg.set(None);
                    },
                    onkeydown: move |e| {
                        if e.key() == Key::Enter {
                            let expr = expr_input.read().clone();
                            if !expr.trim().is_empty() {
                                on_add_trace.call(expr.clone());
                                expr_input.set(String::new());
                            }
                        }
                    },
                }

                // Add button
                button {
                    style: "
                        padding: 4px 8px;
                        background: #3b82f6;
                        border: none;
                        border-radius: {Theme::RADIUS_SM};
                        color: white;
                        font-size: 11px;
                        cursor: pointer;
                    ",
                    onclick: move |_| {
                        let expr = expr_input.read().clone();
                        if !expr.trim().is_empty() {
                            on_add_trace.call(expr.clone());
                            expr_input.set(String::new());
                        }
                    },
                    "Add"
                }
            }

            // Error message
            if let Some(err) = error_msg.read().as_ref() {
                span {
                    style: "color: #ef4444; font-size: 10px;",
                    "{err}"
                }
            }

            // Spacer
            div { style: "flex: 1;" }

            // View controls
            div {
                style: "
                    display: flex;
                    gap: {Theme::SPACING_XS};
                ",

                ControlButton { label: "⊕", title: "Zoom In", onclick: on_zoom_in }
                ControlButton { label: "⊖", title: "Zoom Out", onclick: on_zoom_out }
                ControlButton { label: "⊡", title: "Fit", onclick: on_fit }
                ControlButton { label: "│", title: "Cursor 1" }
                ControlButton { label: "┃", title: "Cursor 2" }
            }
        }
    }
}

/// Small control button
#[component]
fn ControlButton(
    label: &'static str,
    title: &'static str,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let mut hovered = use_signal(|| false);

    let bg = if *hovered.read() {
        th.surface_hover()
    } else {
        th.surface()
    };

    rsx! {
        button {
            title: "{title}",
            style: "
                width: 24px;
                height: 24px;
                display: flex;
                align-items: center;
                justify-content: center;
                background: {bg};
                border: none;
                border-radius: {Theme::RADIUS_SM};
                color: {th.text_primary()};
                font-size: 12px;
                cursor: pointer;
                transition: background {Theme::TRANSITION_FAST};
            ",
            onmouseenter: move |_| hovered.set(true),
            onmouseleave: move |_| hovered.set(false),
            onclick: move |e| onclick.call(e),
            "{label}"
        }
    }
}

/// Grid lines for the waveform plot
#[component]
fn WaveformGrid() -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    rsx! {
        svg {
            style: "
                position: absolute;
                inset: 0;
                width: 100%;
                height: 100%;
            ",

            // Horizontal grid lines
            for i in 0..5 {
                line {
                    x1: "0",
                    y1: "{i as f64 * 25.0}%",
                    x2: "100%",
                    y2: "{i as f64 * 25.0}%",
                    stroke: "{th.border_subtle()}",
                    stroke_width: "1",
                }
            }

            // Vertical grid lines
            for i in 0..7 {
                line {
                    x1: "{i as f64 * 16.67}%",
                    y1: "0",
                    x2: "{i as f64 * 16.67}%",
                    y2: "100%",
                    stroke: "{th.border_subtle()}",
                    stroke_width: "1",
                }
            }

            // Center line (0V reference)
            line {
                x1: "0",
                y1: "50%",
                x2: "100%",
                y2: "50%",
                stroke: "{th.border()}",
                stroke_width: "1",
            }
        }
    }
}

/// Placeholder demo waveform
#[component]
fn WaveformPlaceholder() -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    // Generate a demo sine wave path
    let path = generate_sine_wave_path(100);

    rsx! {
        svg {
            style: "
                position: absolute;
                inset: 0;
                width: 100%;
                height: 100%;
            ",
            view_box: "0 0 100 100",
            preserve_aspect_ratio: "none",

            path {
                d: "{path}",
                fill: "none",
                stroke: "{th.trace_color(0)}",
                stroke_width: "1",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                vector_effect: "non-scaling-stroke",
            }
        }
    }
}

/// Generate SVG path for a sine wave (coordinates in 0-100 range)
fn generate_sine_wave_path(points: usize) -> String {
    let mut path = String::with_capacity(points * 20);

    for i in 0..points {
        let x = (i as f64 / points as f64) * 100.0;
        let t = (i as f64 / points as f64) * 4.0 * std::f64::consts::PI;
        let y = 50.0 - (t.sin() * 35.0);

        if i == 0 {
            path.push_str(&format!("M {:.2} {:.2}", x, y));
        } else {
            path.push_str(&format!(" L {:.2} {:.2}", x, y));
        }
    }

    path
}

/// Waveform trace with view transform applied
#[component]
fn WaveformTraceView(x: Vec<f64>, y: Vec<f64>, color: String, view: ViewState) -> Element {
    // Generate SVG path from data points, applying view transform
    let path = if x.is_empty() || y.is_empty() {
        String::new()
    } else {
        let x_range = (view.x_max - view.x_min).max(1e-12);
        let y_range = (view.y_max - view.y_min).max(1e-12);

        let mut path = String::new();
        for (i, (xi, yi)) in x.iter().zip(y.iter()).enumerate() {
            // Transform to view coordinates (0-100 range)
            let px = ((xi - view.x_min) / x_range) * 100.0;
            let py = 100.0 - ((yi - view.y_min) / y_range) * 100.0;

            // Skip points outside view
            if px < -10.0 || px > 110.0 {
                continue;
            }

            if path.is_empty() {
                path.push_str(&format!("M {:.2} {:.2}", px, py));
            } else {
                path.push_str(&format!(" L {:.2} {:.2}", px, py));
            }
        }
        path
    };

    rsx! {
        svg {
            style: "
                position: absolute;
                inset: 0;
                width: 100%;
                height: 100%;
            ",
            view_box: "0 0 100 100",
            preserve_aspect_ratio: "none",

            path {
                d: "{path}",
                fill: "none",
                stroke: "{color}",
                stroke_width: "1",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                vector_effect: "non-scaling-stroke",
            }
        }
    }
}

/// Legend item with visibility toggle
#[component]
fn LegendItem(
    name: String,
    color: String,
    visible: bool,
    on_toggle: EventHandler<MouseEvent>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let mut hovered = use_signal(|| false);

    let opacity = if visible { "1" } else { "0.4" };
    let bg = if *hovered.read() {
        th.surface_hover()
    } else {
        "transparent"
    };
    let checkbox_bg = if visible { "#3b82f6" } else { "transparent" };

    rsx! {
        div {
            style: "
                display: flex;
                align-items: center;
                gap: {Theme::SPACING_XS};
                padding: 4px 6px;
                margin: 0 -6px;
                font-size: {Theme::FONT_SIZE_SM};
                cursor: pointer;
                border-radius: {Theme::RADIUS_SM};
                background: {bg};
                opacity: {opacity};
                transition: all {Theme::TRANSITION_FAST};
            ",
            onmouseenter: move |_| hovered.set(true),
            onmouseleave: move |_| hovered.set(false),
            onclick: move |e| on_toggle.call(e),

            // Visibility checkbox
            div {
                style: "
                    width: 14px;
                    height: 14px;
                    border: 1px solid {th.border()};
                    border-radius: 3px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    background: {checkbox_bg};
                    transition: background {Theme::TRANSITION_FAST};
                ",
                if visible {
                    span {
                        style: "color: white; font-size: 10px; font-weight: bold;",
                        "✓"
                    }
                }
            }

            // Color swatch
            div {
                style: "
                    width: 12px;
                    height: 3px;
                    background: {color};
                    border-radius: 1px;
                "
            }

            // Name
            span {
                style: "color: {th.text_primary()}; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                "{name}"
            }
        }
    }
}
