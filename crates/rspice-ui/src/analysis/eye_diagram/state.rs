//! Eye Diagram State Management
//!
//! Viewer state for eye diagram display including mode selection,
//! persistence settings, and mask configuration.

use super::data::EyeData;
use super::measurements::EyeMeasurements;

const EYE_VIEW_TIME_DIVISIONS: f64 = 10.0;
const EYE_VIEW_VOLTAGE_DIVISIONS: f64 = 8.0;
const EYE_MAX_MARKERS: usize = 16;

// =============================================================================
// Display Mode
// =============================================================================

/// Eye diagram display mode
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum EyeDisplayMode {
    /// Overlay mode - all traces visible
    #[default]
    Overlay,
    /// Persistence/density mode - heat map
    Persistence,
    /// Single trace view
    SingleTrace,
}

impl EyeDisplayMode {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Overlay => "Overlay",
            Self::Persistence => "Persistence",
            Self::SingleTrace => "Single Trace",
        }
    }

    /// Get all modes
    pub fn all() -> &'static [EyeDisplayMode] {
        &[Self::Overlay, Self::Persistence, Self::SingleTrace]
    }
}

// =============================================================================
// Mask Configuration
// =============================================================================

/// Eye mask for compliance testing
#[derive(Debug, Clone)]
pub struct EyeMask {
    /// Mask is enabled
    pub enabled: bool,
    /// Mask name (e.g., "100GBASE-KR4")
    pub name: String,
    /// Inner polygon (forbidden region)
    pub inner: MaskPolygon,
    /// Outer polygon (boundary)
    pub outer: Option<MaskPolygon>,
    /// Mask violation count
    pub violation_count: usize,
    /// Total samples tested
    pub total_samples: usize,
}

impl Default for EyeMask {
    fn default() -> Self {
        Self {
            enabled: false,
            name: "Generic".to_string(),
            inner: MaskPolygon::default_inner(),
            outer: None,
            violation_count: 0,
            total_samples: 0,
        }
    }
}

impl EyeMask {
    /// Create new mask
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// Check if a point violates the mask
    pub fn check_violation(&self, t_normalized: f64, v_normalized: f64) -> bool {
        self.inner.contains(t_normalized, v_normalized)
    }

    /// Get mask margin (minimum distance to mask)
    pub fn get_margin(&self) -> f64 {
        if self.total_samples == 0 {
            return 1.0;
        }
        1.0 - (self.violation_count as f64 / self.total_samples as f64)
    }

    /// Is mask passing (no violations)?
    pub fn is_passing(&self) -> bool {
        self.violation_count == 0
    }
}

/// Polygon for mask definition
#[derive(Debug, Clone, Default)]
pub struct MaskPolygon {
    /// Points as (t_normalized, v_normalized) pairs
    pub points: Vec<(f64, f64)>,
}

impl MaskPolygon {
    /// Default inner mask (hexagonal eye opening)
    pub fn default_inner() -> Self {
        Self {
            points: vec![
                (0.35, 0.0),   // Left point
                (0.40, 0.25),  // Upper left
                (0.60, 0.25),  // Upper right
                (0.65, 0.0),   // Right point
                (0.60, -0.25), // Lower right
                (0.40, -0.25), // Lower left
            ],
        }
    }

    /// Check if point is inside polygon (ray casting)
    pub fn contains(&self, x: f64, y: f64) -> bool {
        if self.points.len() < 3 {
            return false;
        }

        let mut inside = false;
        let n = self.points.len();

        for i in 0..n {
            let j = (i + 1) % n;
            let (xi, yi) = self.points[i];
            let (xj, yj) = self.points[j];

            if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi) {
                inside = !inside;
            }
        }

        inside
    }
}

// =============================================================================
// Cursor / View State
// =============================================================================

/// Cursor mode for eye viewer measurements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EyeCursorMode {
    /// No cursors active.
    #[default]
    None,
    /// Single cursor active.
    Single,
    /// Two cursors active (delta mode).
    Delta,
}

/// Cursor state for eye viewer measurements.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EyeCursorState {
    /// Current cursor mode.
    pub mode: EyeCursorMode,
    /// Cursor 1 time in seconds.
    pub cursor1_time_s: Option<f64>,
    /// Cursor 2 time in seconds.
    pub cursor2_time_s: Option<f64>,
}

impl EyeCursorState {
    /// Place cursor in waveform-style sequential behavior.
    pub fn place(&mut self, time_s: f64) {
        if !time_s.is_finite() {
            return;
        }
        match self.mode {
            EyeCursorMode::None => {
                self.cursor1_time_s = Some(time_s);
                self.mode = EyeCursorMode::Single;
            }
            EyeCursorMode::Single => {
                self.cursor2_time_s = Some(time_s);
                self.mode = EyeCursorMode::Delta;
            }
            EyeCursorMode::Delta => {
                self.cursor2_time_s = Some(time_s);
            }
        }
    }

    /// Clear both cursors.
    pub fn clear(&mut self) {
        self.mode = EyeCursorMode::None;
        self.cursor1_time_s = None;
        self.cursor2_time_s = None;
    }

    /// Delta time between two active cursors.
    pub fn delta_time(&self) -> Option<f64> {
        match (self.cursor1_time_s, self.cursor2_time_s) {
            (Some(a), Some(b)) => Some((b - a).abs()),
            _ => None,
        }
    }
}

/// Active eye-plot view range (time and voltage).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EyeViewRange {
    /// Visible time minimum in seconds.
    pub time_min_s: f64,
    /// Visible time maximum in seconds.
    pub time_max_s: f64,
    /// Visible voltage minimum.
    pub voltage_min: f64,
    /// Visible voltage maximum.
    pub voltage_max: f64,
}

impl Default for EyeViewRange {
    fn default() -> Self {
        Self {
            time_min_s: 0.0,
            time_max_s: 1e-9,
            voltage_min: -0.5,
            voltage_max: 0.5,
        }
    }
}

impl EyeViewRange {
    /// Time span in seconds.
    pub fn time_span(self) -> f64 {
        self.time_max_s - self.time_min_s
    }

    /// Voltage span.
    pub fn voltage_span(self) -> f64 {
        self.voltage_max - self.voltage_min
    }

    /// Enforce finite non-degenerate range.
    pub fn sanitize(&mut self) {
        if !self.time_min_s.is_finite()
            || !self.time_max_s.is_finite()
            || self.time_max_s <= self.time_min_s
        {
            self.time_min_s = 0.0;
            self.time_max_s = 1e-9;
        }
        if !self.voltage_min.is_finite()
            || !self.voltage_max.is_finite()
            || self.voltage_max <= self.voltage_min
        {
            self.voltage_min = -0.5;
            self.voltage_max = 0.5;
        }
    }
}

/// Cache key for persistence rendering.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EyePersistenceCacheKey {
    pub width: usize,
    pub height: usize,
    pub trace_count: usize,
    pub total_points: usize,
    pub ui_count: u32,
    pub time_min_s: f64,
    pub time_max_s: f64,
    pub voltage_min: f64,
    pub voltage_max: f64,
    pub decay_quantized: u16,
}

/// Cached persistence grid.
#[derive(Debug, Clone)]
pub struct EyePersistenceCache {
    pub key: EyePersistenceCacheKey,
    pub width: usize,
    pub height: usize,
    pub nonzero_bins: Vec<(usize, usize, u32)>,
    pub max_count: u32,
}

// =============================================================================
// Eye Diagram State
// =============================================================================

/// Complete eye diagram viewer state
#[derive(Debug, Clone)]
pub struct EyeDiagramState {
    /// Display mode
    pub mode: EyeDisplayMode,
    /// Eye diagram data
    pub data: EyeData,
    /// Calculated measurements
    pub measurements: EyeMeasurements,
    /// Eye mask configuration
    pub mask: EyeMask,
    /// Persistence decay factor (0.0-1.0)
    pub persistence_decay: f32,
    /// Color map for persistence mode
    pub color_map: ColorMap,
    /// Show grid
    pub show_grid: bool,
    /// Show measurements overlay
    pub show_measurements: bool,
    /// Show mask
    pub show_mask: bool,
    /// Current selected trace (for single trace mode)
    pub selected_trace: Option<usize>,
    /// Horizontal scale (UI per division)
    pub h_scale: f64,
    /// Vertical scale (volts per division)
    pub v_scale: f64,
    /// Number of UI to display
    pub ui_count: u32,
    /// Interactive view range for time/voltage axes.
    pub view: EyeViewRange,
    /// Eye measurement cursors.
    pub cursors: EyeCursorState,
    /// User markers on the eye time axis.
    pub markers: Vec<f64>,
    /// Cached persistence grid for current view/data signature.
    pub persistence_cache: Option<EyePersistenceCache>,
    /// Optional user-resized measurements pane width in pixels. `None` means auto-fit.
    pub measurements_pane_width: Option<f32>,
    /// Runtime auto-fit width hint captured from rendered content.
    pub measurements_pane_auto_width_hint: f32,
}

impl Default for EyeDiagramState {
    fn default() -> Self {
        Self {
            mode: EyeDisplayMode::Overlay,
            data: EyeData::default(),
            measurements: EyeMeasurements::default(),
            mask: EyeMask::default(),
            persistence_decay: 0.95,
            color_map: ColorMap::default(),
            show_grid: true,
            show_measurements: true,
            show_mask: false,
            selected_trace: None,
            h_scale: 0.5,
            v_scale: 0.2,
            ui_count: 2,
            view: EyeViewRange::default(),
            cursors: EyeCursorState::default(),
            markers: Vec::new(),
            persistence_cache: None,
            measurements_pane_width: None,
            measurements_pane_auto_width_hint: 0.0,
        }
    }
}

impl EyeDiagramState {
    /// Create new state
    pub fn new() -> Self {
        Self::default()
    }

    /// Load eye data and recalculate measurements
    pub fn load_data(&mut self, data: EyeData) {
        self.ui_count = data.ui_count.max(1) as u32;
        self.data = data;
        if let Some(idx) = self.selected_trace {
            if idx >= self.data.traces.len() {
                self.selected_trace = None;
            }
        }
        self.cursors.clear();
        self.clear_markers();
        self.reset_view_to_data();
        self.invalidate_persistence_cache();
        self.recalculate_measurements();
        if self.show_mask {
            self.mask.enabled = true;
            self.run_mask_test();
        }
    }

    /// Recalculate measurements from current data
    pub fn recalculate_measurements(&mut self) {
        self.measurements = super::measurements::calculate_eye_measurements(&self.data);
    }

    /// Run mask test
    pub fn run_mask_test(&mut self) {
        if !self.mask.enabled {
            return;
        }
        let ui_count = self.ui_count.max(1) as f64;

        self.mask.violation_count = 0;
        self.mask.total_samples = 0;

        for trace in &self.data.traces {
            let n = trace.time.len().min(trace.amplitude.len());
            for i in 0..n {
                let t_norm = trace.time[i] / ui_count;
                let v_norm = if self.data.swing > 0.0 {
                    (trace.amplitude[i] - self.data.v_cross) / self.data.swing
                } else {
                    0.0
                };

                if self.mask.check_violation(t_norm, v_norm) {
                    self.mask.violation_count += 1;
                }
                self.mask.total_samples += 1;
            }
        }
    }

    /// Get mask test result string
    pub fn mask_result_string(&self) -> String {
        if !self.mask.enabled {
            return "Mask disabled".to_string();
        }
        if self.mask.is_passing() {
            format!("PASS ({} samples)", self.mask.total_samples)
        } else {
            format!(
                "FAIL ({} violations / {} samples)",
                self.mask.violation_count, self.mask.total_samples
            )
        }
    }

    /// Set display mode
    pub fn set_mode(&mut self, mode: EyeDisplayMode) {
        if self.mode != mode {
            self.mode = mode;
            self.invalidate_persistence_cache();
        }
    }

    /// Toggle mask display
    pub fn toggle_mask(&mut self) {
        self.show_mask = !self.show_mask;
        self.mask.enabled = self.show_mask;
        if self.show_mask {
            self.run_mask_test();
        }
    }

    /// Toggle measurements display
    pub fn toggle_measurements(&mut self) {
        self.show_measurements = !self.show_measurements;
    }

    /// Select specific trace for single trace mode
    pub fn select_trace(&mut self, index: Option<usize>) {
        self.selected_trace = index;
    }

    /// Get number of traces
    pub fn trace_count(&self) -> usize {
        self.data.trace_count()
    }

    /// Full eye time span in seconds represented by loaded data.
    pub fn full_time_span_seconds(&self) -> f64 {
        self.ui_count.max(1) as f64 * self.data.bit_period.max(1e-18)
    }

    /// Full voltage bounds inferred from loaded data.
    pub fn full_voltage_bounds(&self) -> (f64, f64) {
        let swing = if self.data.swing.is_finite() && self.data.swing > 0.0 {
            self.data.swing
        } else {
            (self.data.v_high - self.data.v_low).abs().max(1e-3)
        };
        let half = swing * 0.5;
        (self.data.v_cross - half, self.data.v_cross + half)
    }

    /// Reset current view to full data range.
    pub fn reset_view_to_data(&mut self) {
        let time_span = self.full_time_span_seconds().max(1e-18);
        let (v_min, v_max) = self.full_voltage_bounds();
        self.view = EyeViewRange {
            time_min_s: 0.0,
            time_max_s: time_span,
            voltage_min: v_min,
            voltage_max: v_max,
        };
        self.sync_scales_from_view();
    }

    /// Clamp interactive view to full data bounds.
    pub fn clamp_view_to_data(&mut self) {
        let full_time = self.full_time_span_seconds().max(1e-18);
        let (full_v_min, full_v_max) = self.full_voltage_bounds();

        self.view.sanitize();

        let t_span = self.view.time_span().clamp(1e-18, full_time);
        if self.view.time_min_s < 0.0 {
            self.view.time_min_s = 0.0;
        }
        if self.view.time_max_s > full_time {
            self.view.time_max_s = full_time;
        }
        if self.view.time_span() != t_span {
            self.view.time_max_s = (self.view.time_min_s + t_span).min(full_time);
        }
        if self.view.time_span() < t_span {
            self.view.time_min_s = (self.view.time_max_s - t_span).max(0.0);
        }

        let full_v_span = (full_v_max - full_v_min).max(1e-9);
        let v_span = self.view.voltage_span().clamp(1e-9, full_v_span);
        if self.view.voltage_min < full_v_min {
            self.view.voltage_min = full_v_min;
        }
        if self.view.voltage_max > full_v_max {
            self.view.voltage_max = full_v_max;
        }
        if self.view.voltage_span() != v_span {
            self.view.voltage_max = (self.view.voltage_min + v_span).min(full_v_max);
        }
        if self.view.voltage_span() < v_span {
            self.view.voltage_min = (self.view.voltage_max - v_span).max(full_v_min);
        }

        self.sync_scales_from_view();
    }

    /// Pan view by time and voltage deltas.
    pub fn pan_view(&mut self, delta_time_s: f64, delta_voltage: f64) {
        if !delta_time_s.is_finite() || !delta_voltage.is_finite() {
            return;
        }
        self.view.time_min_s += delta_time_s;
        self.view.time_max_s += delta_time_s;
        self.view.voltage_min += delta_voltage;
        self.view.voltage_max += delta_voltage;
        self.clamp_view_to_data();
    }

    /// Zoom view around a time/voltage anchor.
    pub fn zoom_view(&mut self, factor: f64, center_time_s: f64, center_voltage: f64) {
        if !factor.is_finite() || factor <= 0.0 {
            return;
        }
        let t_span = (self.view.time_span() * factor).max(1e-18);
        let v_span = (self.view.voltage_span() * factor).max(1e-9);

        let x_frac = if self.view.time_span() > 0.0 {
            ((center_time_s - self.view.time_min_s) / self.view.time_span()).clamp(0.0, 1.0)
        } else {
            0.5
        };
        let y_frac = if self.view.voltage_span() > 0.0 {
            ((center_voltage - self.view.voltage_min) / self.view.voltage_span()).clamp(0.0, 1.0)
        } else {
            0.5
        };

        self.view.time_min_s = center_time_s - x_frac * t_span;
        self.view.time_max_s = self.view.time_min_s + t_span;
        self.view.voltage_min = center_voltage - y_frac * v_span;
        self.view.voltage_max = self.view.voltage_min + v_span;
        self.clamp_view_to_data();
    }

    /// Add a marker at the given time coordinate.
    pub fn add_marker(&mut self, time_s: f64) {
        if !time_s.is_finite() {
            return;
        }
        const MERGE_EPS: f64 = 1e-18;
        if self
            .markers
            .iter()
            .any(|m| (*m - time_s).abs() <= MERGE_EPS)
        {
            return;
        }
        self.markers.push(time_s);
        self.markers.sort_by(|a, b| a.total_cmp(b));
        if self.markers.len() > EYE_MAX_MARKERS {
            self.markers.remove(0);
        }
    }

    /// Remove marker nearest to target if within tolerance.
    pub fn remove_nearest_marker(&mut self, time_s: f64, tolerance_s: f64) -> bool {
        if !time_s.is_finite() || !tolerance_s.is_finite() || tolerance_s < 0.0 {
            return false;
        }
        let Some((idx, dist)) = self
            .markers
            .iter()
            .enumerate()
            .map(|(idx, marker)| (idx, (*marker - time_s).abs()))
            .min_by(|a, b| a.1.total_cmp(&b.1))
        else {
            return false;
        };
        if dist <= tolerance_s {
            self.markers.remove(idx);
            true
        } else {
            false
        }
    }

    /// Remove marker at index.
    pub fn remove_marker_at(&mut self, index: usize) -> bool {
        if index >= self.markers.len() {
            return false;
        }
        self.markers.remove(index);
        true
    }

    /// Clear all markers.
    pub fn clear_markers(&mut self) {
        self.markers.clear();
    }

    /// Invalidate persistence render cache.
    pub fn invalidate_persistence_cache(&mut self) {
        self.persistence_cache = None;
    }

    /// Update axis scale display values from current view.
    pub fn sync_scales_from_view(&mut self) {
        self.h_scale = (self.view.time_span() / EYE_VIEW_TIME_DIVISIONS).max(1e-18);
        self.v_scale = (self.view.voltage_span() / EYE_VIEW_VOLTAGE_DIVISIONS).max(1e-9);
    }

    /// Apply current scale controls around view center.
    pub fn apply_scale_controls(&mut self) {
        let center_t = (self.view.time_min_s + self.view.time_max_s) * 0.5;
        let center_v = (self.view.voltage_min + self.view.voltage_max) * 0.5;
        let full_time = self.full_time_span_seconds().max(1e-18);
        let (full_v_min, full_v_max) = self.full_voltage_bounds();
        let full_v_span = (full_v_max - full_v_min).max(1e-9);

        let desired_t_span =
            (self.h_scale.max(1e-18) * EYE_VIEW_TIME_DIVISIONS).clamp(1e-18, full_time);
        let desired_v_span =
            (self.v_scale.max(1e-9) * EYE_VIEW_VOLTAGE_DIVISIONS).clamp(1e-9, full_v_span);

        self.view.time_min_s = center_t - desired_t_span * 0.5;
        self.view.time_max_s = center_t + desired_t_span * 0.5;
        self.view.voltage_min = center_v - desired_v_span * 0.5;
        self.view.voltage_max = center_v + desired_v_span * 0.5;
        self.clamp_view_to_data();
    }
}

// =============================================================================
// Color Map
// =============================================================================

/// Color map for persistence/density display
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorMap {
    /// Hot (black -> red -> yellow -> white)
    Hot,
    /// Cool (black -> blue -> cyan -> white)
    Cool,
    /// Viridis (perceptually uniform)
    Viridis,
    /// Rainbow
    Rainbow,
    /// Green phosphor (classic scope look)
    #[default]
    Phosphor,
}

impl ColorMap {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Hot => "Hot",
            Self::Cool => "Cool",
            Self::Viridis => "Viridis",
            Self::Rainbow => "Rainbow",
            Self::Phosphor => "Phosphor",
        }
    }

    /// Get all color maps
    pub fn all() -> &'static [ColorMap] {
        &[
            Self::Hot,
            Self::Cool,
            Self::Viridis,
            Self::Rainbow,
            Self::Phosphor,
        ]
    }

    /// Map intensity (0-1) to RGB color
    pub fn map(&self, intensity: f32) -> (u8, u8, u8) {
        let t = intensity.clamp(0.0, 1.0);

        match self {
            Self::Hot => {
                let r = (t * 3.0).min(1.0);
                let g = ((t - 0.33) * 3.0).clamp(0.0, 1.0);
                let b = ((t - 0.66) * 3.0).clamp(0.0, 1.0);
                (
                    (r * 255.0).round() as u8,
                    (g * 255.0).round() as u8,
                    (b * 255.0).round() as u8,
                )
            }
            Self::Cool => {
                let r = ((t - 0.66) * 3.0).clamp(0.0, 1.0);
                let g = ((t - 0.33) * 3.0).clamp(0.0, 1.0);
                let b = (t * 3.0).min(1.0);
                (
                    (r * 255.0).round() as u8,
                    (g * 255.0).round() as u8,
                    (b * 255.0).round() as u8,
                )
            }
            Self::Viridis => {
                // Simplified Viridis approximation
                let r = (0.267 + 0.329 * t + 0.278 * t * t).min(1.0);
                let g = (0.004 + 0.873 * t - 0.268 * t * t).clamp(0.0, 1.0);
                let b = (0.329 + 0.475 * t - 0.785 * t * t).clamp(0.0, 1.0);
                ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
            }
            Self::Rainbow => {
                let hue = t * 5.0;
                let (r, g, b) = match hue as u32 {
                    0 => (1.0, hue, 0.0),
                    1 => (2.0 - hue, 1.0, 0.0),
                    2 => (0.0, 1.0, hue - 2.0),
                    3 => (0.0, 4.0 - hue, 1.0),
                    4 => (hue - 4.0, 0.0, 1.0),
                    _ => (1.0, 0.0, 1.0),
                };
                (
                    (r.min(1.0) * 255.0) as u8,
                    (g.min(1.0) * 255.0) as u8,
                    (b.min(1.0) * 255.0) as u8,
                )
            }
            Self::Phosphor => {
                // Green phosphor with slight cyan tint
                let g = t * 0.9 + 0.1 * t * t;
                let r = t * 0.1;
                let b = t * 0.2;
                ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
            }
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
    // Display Mode Tests
    // =========================================================================

    #[test]
    fn test_display_mode_default() {
        let mode = EyeDisplayMode::default();
        assert_eq!(mode, EyeDisplayMode::Overlay);
    }

    #[test]
    fn test_display_mode_names() {
        assert_eq!(EyeDisplayMode::Overlay.display_name(), "Overlay");
        assert_eq!(EyeDisplayMode::Persistence.display_name(), "Persistence");
    }

    #[test]
    fn test_display_mode_all() {
        let modes = EyeDisplayMode::all();
        assert_eq!(modes.len(), 3);
    }

    // =========================================================================
    // Mask Polygon Tests
    // =========================================================================

    #[test]
    fn test_polygon_contains_inside() {
        let poly = MaskPolygon {
            points: vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        };
        assert!(poly.contains(0.5, 0.5));
    }

    #[test]
    fn test_polygon_contains_outside() {
        let poly = MaskPolygon {
            points: vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        };
        assert!(!poly.contains(1.5, 0.5));
        assert!(!poly.contains(-0.5, 0.5));
    }

    #[test]
    fn test_polygon_empty() {
        let poly = MaskPolygon { points: Vec::new() };
        assert!(!poly.contains(0.5, 0.5));
    }

    #[test]
    fn test_polygon_too_few_points() {
        let poly = MaskPolygon {
            points: vec![(0.0, 0.0), (1.0, 1.0)],
        };
        assert!(!poly.contains(0.5, 0.5));
    }

    #[test]
    fn test_default_inner_mask() {
        let inner = MaskPolygon::default_inner();
        assert_eq!(inner.points.len(), 6);

        // Center should be inside
        assert!(inner.contains(0.5, 0.0));

        // Outside should not be inside
        assert!(!inner.contains(0.1, 0.0));
        assert!(!inner.contains(0.9, 0.0));
    }

    // =========================================================================
    // Eye Mask Tests
    // =========================================================================

    #[test]
    fn test_mask_new() {
        let mask = EyeMask::new("Test Mask");
        assert_eq!(mask.name, "Test Mask");
        assert!(!mask.enabled);
    }

    #[test]
    fn test_mask_default() {
        let mask = EyeMask::default();
        assert!(!mask.enabled);
        assert_eq!(mask.violation_count, 0);
    }

    #[test]
    fn test_mask_is_passing() {
        let mask = EyeMask::default();
        assert!(mask.is_passing());
    }

    #[test]
    fn test_mask_not_passing() {
        let mut mask = EyeMask::default();
        mask.violation_count = 5;
        assert!(!mask.is_passing());
    }

    #[test]
    fn test_mask_margin() {
        let mut mask = EyeMask::default();
        mask.total_samples = 100;
        mask.violation_count = 0;
        assert_eq!(mask.get_margin(), 1.0);

        mask.violation_count = 10;
        assert!((mask.get_margin() - 0.9).abs() < 0.01);
    }

    // =========================================================================
    // Eye Diagram State Tests
    // =========================================================================

    #[test]
    fn test_state_new() {
        let state = EyeDiagramState::new();
        assert_eq!(state.mode, EyeDisplayMode::Overlay);
        assert!(state.show_grid);
        assert!(state.show_measurements);
        assert!(state.markers.is_empty());
        assert_eq!(state.cursors.mode, EyeCursorMode::None);
    }

    #[test]
    fn test_state_default() {
        let state = EyeDiagramState::default();
        assert_eq!(state.ui_count, 2);
        assert!(!state.show_mask);
    }

    #[test]
    fn test_state_set_mode() {
        let mut state = EyeDiagramState::new();
        state.set_mode(EyeDisplayMode::Persistence);
        assert_eq!(state.mode, EyeDisplayMode::Persistence);
    }

    #[test]
    fn test_state_toggle_mask() {
        let mut state = EyeDiagramState::new();
        assert!(!state.show_mask);

        state.toggle_mask();
        assert!(state.show_mask);
        assert!(state.mask.enabled);
    }

    #[test]
    fn test_state_toggle_mask_off_disables_mask_engine() {
        let mut state = EyeDiagramState::new();
        state.toggle_mask();
        assert!(state.mask.enabled);
        state.toggle_mask();
        assert!(!state.show_mask);
        assert!(!state.mask.enabled);
    }

    #[test]
    fn test_state_toggle_measurements() {
        let mut state = EyeDiagramState::new();
        let initial = state.show_measurements;

        state.toggle_measurements();
        assert_ne!(state.show_measurements, initial);
    }

    #[test]
    fn test_state_select_trace() {
        let mut state = EyeDiagramState::new();
        assert!(state.selected_trace.is_none());

        state.select_trace(Some(5));
        assert_eq!(state.selected_trace, Some(5));

        state.select_trace(None);
        assert!(state.selected_trace.is_none());
    }

    #[test]
    fn test_state_trace_count() {
        let state = EyeDiagramState::new();
        assert_eq!(state.trace_count(), 0);
    }

    #[test]
    fn test_load_data_syncs_ui_count_and_clamps_selected_trace() {
        let mut state = EyeDiagramState::new();
        state.selected_trace = Some(99);
        let mut data = EyeData::new(100e-12, 3);
        data.add_trace(super::super::data::EyeTrace::new(
            vec![0.0, 0.5, 1.0],
            vec![-0.1, 0.1, -0.1],
        ));
        state.load_data(data);
        assert_eq!(state.ui_count, 3);
        assert_eq!(state.selected_trace, None);
    }

    #[test]
    fn test_state_load_data_resets_interaction_state() {
        let mut state = EyeDiagramState::new();
        state.cursors.place(1e-9);
        state.add_marker(1e-9);
        assert!(!state.markers.is_empty());
        assert!(state.cursors.cursor1_time_s.is_some());
        let mut data = EyeData::new(100e-12, 2);
        data.add_trace(super::super::data::EyeTrace::new(
            vec![0.0, 0.5, 1.0],
            vec![-0.1, 0.1, -0.1],
        ));
        state.load_data(data);
        assert!(state.markers.is_empty());
        assert_eq!(state.cursors.mode, EyeCursorMode::None);
        assert!(state.view.time_max_s > state.view.time_min_s);
    }

    #[test]
    fn test_state_marker_management_behaves_like_waveform() {
        let mut state = EyeDiagramState::new();
        state.add_marker(2.0e-9);
        state.add_marker(1.0e-9);
        state.add_marker(2.0e-9); // duplicate ignored
        state.add_marker(f64::NAN); // invalid ignored
        assert_eq!(state.markers, vec![1.0e-9, 2.0e-9]);
        assert!(state.remove_nearest_marker(2.01e-9, 20e-12));
        assert_eq!(state.markers, vec![1.0e-9]);
        assert!(!state.remove_nearest_marker(2.01e-9, 1e-12));
        state.clear_markers();
        assert!(state.markers.is_empty());
    }

    #[test]
    fn test_state_zoom_and_pan_clamped_to_data_bounds() {
        let mut state = EyeDiagramState::new();
        let mut data = EyeData::new(100e-12, 2);
        data.add_trace(super::super::data::EyeTrace::new(
            vec![0.0, 0.5, 1.0, 1.5, 2.0],
            vec![-0.2, 0.2, -0.2, 0.2, -0.2],
        ));
        state.load_data(data);
        let full_time = state.full_time_span_seconds();
        state.pan_view(-full_time, 0.0);
        assert!(state.view.time_min_s >= 0.0);
        state.zoom_view(0.01, 0.0, 0.0);
        assert!(state.view.time_span() > 0.0);
        assert!(state.h_scale > 0.0);
        assert!(state.v_scale > 0.0);
    }

    #[test]
    fn test_state_mask_result_disabled() {
        let state = EyeDiagramState::new();
        assert!(state.mask_result_string().contains("disabled"));
    }

    #[test]
    fn test_state_mask_result_pass() {
        let mut state = EyeDiagramState::new();
        state.mask.enabled = true;
        state.mask.total_samples = 1000;
        state.mask.violation_count = 0;
        assert!(state.mask_result_string().contains("PASS"));
    }

    #[test]
    fn test_state_mask_result_fail() {
        let mut state = EyeDiagramState::new();
        state.mask.enabled = true;
        state.mask.total_samples = 1000;
        state.mask.violation_count = 10;
        assert!(state.mask_result_string().contains("FAIL"));
    }

    #[test]
    fn test_cursor_state_place_and_delta() {
        let mut cursors = EyeCursorState::default();
        cursors.place(1.0e-9);
        assert_eq!(cursors.mode, EyeCursorMode::Single);
        cursors.place(1.4e-9);
        assert_eq!(cursors.mode, EyeCursorMode::Delta);
        let delta = cursors.delta_time().expect("delta");
        assert!((delta - 0.4e-9).abs() < 1e-18);
        cursors.clear();
        assert_eq!(cursors.mode, EyeCursorMode::None);
    }

    // =========================================================================
    // Color Map Tests
    // =========================================================================

    #[test]
    fn test_colormap_default() {
        let map = ColorMap::default();
        assert_eq!(map, ColorMap::Phosphor);
    }

    #[test]
    fn test_colormap_names() {
        assert_eq!(ColorMap::Hot.display_name(), "Hot");
        assert_eq!(ColorMap::Phosphor.display_name(), "Phosphor");
    }

    #[test]
    fn test_colormap_all() {
        let maps = ColorMap::all();
        assert_eq!(maps.len(), 5);
    }

    #[test]
    fn test_colormap_hot_black() {
        let (r, g, b) = ColorMap::Hot.map(0.0);
        assert_eq!(r, 0);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
    }

    #[test]
    fn test_colormap_hot_white() {
        let (r, g, b) = ColorMap::Hot.map(1.0);
        assert_eq!(r, 255);
        assert_eq!(g, 255);
        assert_eq!(b, 255);
    }

    #[test]
    fn test_colormap_phosphor_green() {
        let (r, g, b) = ColorMap::Phosphor.map(1.0);
        assert!(g > r);
        assert!(g > b);
    }

    #[test]
    fn test_colormap_clamp() {
        // Test that out-of-range values are clamped
        let (r1, g1, b1) = ColorMap::Hot.map(-0.5);
        let (r2, g2, b2) = ColorMap::Hot.map(0.0);
        assert_eq!((r1, g1, b1), (r2, g2, b2));

        let (r3, g3, b3) = ColorMap::Hot.map(1.5);
        let (r4, g4, b4) = ColorMap::Hot.map(1.0);
        assert_eq!((r3, g3, b3), (r4, g4, b4));
    }
}
