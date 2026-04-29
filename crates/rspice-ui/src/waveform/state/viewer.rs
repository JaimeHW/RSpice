use serde::{Deserialize, Serialize};

use crate::waveform::measurements::MeasurementCache;

use super::{
    BoxSelection, CursorState, DataBounds, TraceData, TraceStyle, ViewTransform, WaveformPanel,
};

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
    pub spec_overlays: Vec<crate::waveform::spec_overlay::SpecOverlay>,
    /// Legend UI state (filter/sort/collapse). Runtime-only UI preference.
    #[serde(skip)]
    pub legend_state: crate::waveform::legend::LegendState,
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
    pub export_options: crate::waveform::export::ExportOptions,
    /// Optional export status message shown in the export panel.
    #[serde(skip)]
    pub export_status: Option<String>,
    /// Cached per-trace measurement results used by the measurements panel.
    #[serde(skip)]
    pub measurement_cache: MeasurementCache,
    /// Optional user-resized right pane width in pixels. `None` means auto-fit.
    #[serde(default)]
    pub right_pane_width: Option<f32>,
    /// Runtime auto-fit width hint captured from rendered content.
    #[serde(skip)]
    pub right_pane_auto_width_hint: f32,
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

    /// Remove marker at explicit index.
    pub fn remove_marker_at(&mut self, index: usize) -> bool {
        if index >= self.markers.len() {
            return false;
        }
        self.markers.remove(index);
        true
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
        self.measurement_cache.clear();
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
