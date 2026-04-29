use serde::{Deserialize, Serialize};

use super::TraceData;

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
            if let Some(trace) = all_traces.get(idx)
                && trace.visible
            {
                if let Some(min) = trace.y_min() {
                    y_min = y_min.min(min);
                }
                if let Some(max) = trace.y_max() {
                    y_max = y_max.max(max);
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
