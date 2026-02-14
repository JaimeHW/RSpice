//! Eye Diagram State Management
//!
//! Viewer state for eye diagram display including mode selection,
//! persistence settings, and mask configuration.

use super::data::EyeData;
use super::measurements::EyeMeasurements;

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
        self.data = data;
        self.recalculate_measurements();
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

        self.mask.violation_count = 0;
        self.mask.total_samples = 0;

        for trace in &self.data.traces {
            let n = trace.time.len().min(trace.amplitude.len());
            for i in 0..n {
                let t_norm = trace.time[i] / self.data.ui_count as f64;
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
        self.mode = mode;
    }

    /// Toggle mask display
    pub fn toggle_mask(&mut self) {
        self.show_mask = !self.show_mask;
        if self.show_mask {
            self.mask.enabled = true;
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
}

// =============================================================================
// Color Map
// =============================================================================

/// Color map for persistence/density display
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
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
