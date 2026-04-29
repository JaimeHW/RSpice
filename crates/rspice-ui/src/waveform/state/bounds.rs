use serde::{Deserialize, Serialize};

use super::TraceData;

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
