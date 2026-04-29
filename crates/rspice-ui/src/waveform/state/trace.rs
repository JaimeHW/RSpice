use serde::{Deserialize, Serialize};
use std::sync::Arc;

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
    pub x: Arc<[f64]>,
    /// Y-axis data points (typically voltage or current)
    pub y: Arc<[f64]>,
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
            x: Arc::<[f64]>::from([]),
            y: Arc::<[f64]>::from([]),
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
    pub fn new(
        name: impl Into<String>,
        x: impl Into<Arc<[f64]>>,
        y: impl Into<Arc<[f64]>>,
    ) -> Self {
        let mut trace = Self {
            name: name.into(),
            x: x.into(),
            y: y.into(),
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

        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;
        let mut has_finite_y = false;
        for &value in self.y.iter() {
            if !value.is_finite() {
                continue;
            }
            has_finite_y = true;
            y_min = y_min.min(value);
            y_max = y_max.max(value);
        }
        self.cached_y_min = has_finite_y.then_some(y_min);
        self.cached_y_max = has_finite_y.then_some(y_max);
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
