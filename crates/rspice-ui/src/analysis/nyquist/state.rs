//! Nyquist Plot State Management
//!
//! Viewer state for Nyquist/Polar plot display.

use super::data::NyquistData;

// =============================================================================
// Display Options
// =============================================================================

/// What to show on the plot
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NyquistOverlay {
    /// Just the curve
    #[default]
    None,
    /// Show unity circle
    UnityCircle,
    /// Show stability info
    StabilityInfo,
    /// All overlays
    All,
}

impl NyquistOverlay {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::UnityCircle => "Unity Circle",
            Self::StabilityInfo => "Stability",
            Self::All => "All",
        }
    }

    /// All options
    pub fn all() -> &'static [NyquistOverlay] {
        &[
            Self::None,
            Self::UnityCircle,
            Self::StabilityInfo,
            Self::All,
        ]
    }

    /// Show unity circle?
    pub fn show_unity_circle(&self) -> bool {
        matches!(self, Self::UnityCircle | Self::All)
    }

    /// Show stability info?
    pub fn show_stability(&self) -> bool {
        matches!(self, Self::StabilityInfo | Self::All)
    }
}

// =============================================================================
// Nyquist State
// =============================================================================

/// Complete Nyquist plot viewer state
#[derive(Debug, Clone)]
pub struct NyquistState {
    /// Nyquist data curves
    pub curves: Vec<NyquistData>,
    /// Selected curve index
    pub selected: usize,
    /// Overlay options
    pub overlay: NyquistOverlay,
    /// Show grid
    pub show_grid: bool,
    /// Show critical point marker
    pub show_critical_point: bool,
    /// Show frequency annotations
    pub show_freq_annotations: bool,
    /// Equal axis scaling
    pub equal_axes: bool,
    /// Real axis range
    pub real_min: f64,
    pub real_max: f64,
    /// Imaginary axis range
    pub imag_min: f64,
    pub imag_max: f64,
    /// Auto-scale
    pub auto_scale: bool,
}

impl Default for NyquistState {
    fn default() -> Self {
        Self {
            curves: Vec::new(),
            selected: 0,
            overlay: NyquistOverlay::All,
            show_grid: true,
            show_critical_point: true,
            show_freq_annotations: true,
            equal_axes: true,
            real_min: -2.0,
            real_max: 2.0,
            imag_min: -2.0,
            imag_max: 2.0,
            auto_scale: true,
        }
    }
}

impl NyquistState {
    /// Create new state
    pub fn new() -> Self {
        Self::default()
    }

    /// Load data
    pub fn load_data(&mut self, data: NyquistData) {
        self.curves = vec![data];
        self.selected = 0;
        self.update_auto_scale();
    }

    /// Add curve
    pub fn add_curve(&mut self, data: NyquistData) {
        self.curves.push(data);
        self.update_auto_scale();
    }

    /// Clear all curves
    pub fn clear(&mut self) {
        self.curves.clear();
        self.selected = 0;
    }

    /// Current curve
    pub fn current_curve(&self) -> Option<&NyquistData> {
        self.curves.get(self.selected)
    }

    /// Number of curves
    pub fn curve_count(&self) -> usize {
        self.curves.len()
    }

    /// Is empty?
    pub fn is_empty(&self) -> bool {
        self.curves.is_empty()
    }

    /// Update auto-scale ranges
    pub fn update_auto_scale(&mut self) {
        if !self.auto_scale || self.curves.is_empty() {
            return;
        }

        let mut real_min = f64::MAX;
        let mut real_max = f64::MIN;
        let mut imag_min = f64::MAX;
        let mut imag_max = f64::MIN;

        for curve in &self.curves {
            if let Some((lo, hi)) = curve.real_range() {
                real_min = real_min.min(lo);
                real_max = real_max.max(hi);
            }
            if let Some((lo, hi)) = curve.imag_range() {
                imag_min = imag_min.min(lo);
                imag_max = imag_max.max(hi);
            }
        }

        // Include critical point (-1, 0)
        real_min = real_min.min(-1.5);
        real_max = real_max.max(0.5);
        imag_min = imag_min.min(-1.0);
        imag_max = imag_max.max(1.0);

        // Add padding
        let real_pad = (real_max - real_min) * 0.1;
        let imag_pad = (imag_max - imag_min) * 0.1;

        self.real_min = real_min - real_pad;
        self.real_max = real_max + real_pad;
        self.imag_min = imag_min - imag_pad;
        self.imag_max = imag_max + imag_pad;

        // Make square if equal axes enabled
        if self.equal_axes {
            let real_range = self.real_max - self.real_min;
            let imag_range = self.imag_max - self.imag_min;
            let max_range = real_range.max(imag_range);

            let real_center = (self.real_max + self.real_min) / 2.0;
            let imag_center = (self.imag_max + self.imag_min) / 2.0;

            self.real_min = real_center - max_range / 2.0;
            self.real_max = real_center + max_range / 2.0;
            self.imag_min = imag_center - max_range / 2.0;
            self.imag_max = imag_center + max_range / 2.0;
        }
    }

    /// Set overlay mode
    pub fn set_overlay(&mut self, overlay: NyquistOverlay) {
        self.overlay = overlay;
    }

    /// Toggle grid
    pub fn toggle_grid(&mut self) {
        self.show_grid = !self.show_grid;
    }

    /// Toggle critical point
    pub fn toggle_critical_point(&mut self) {
        self.show_critical_point = !self.show_critical_point;
    }

    /// Toggle equal axes
    pub fn toggle_equal_axes(&mut self) {
        self.equal_axes = !self.equal_axes;
        self.update_auto_scale();
    }

    /// Get encirclement count for current curve
    pub fn encirclements(&self) -> Option<i32> {
        self.current_curve().map(|c| c.count_encirclements())
    }

    /// Is current curve stable?
    pub fn is_stable(&self) -> Option<bool> {
        self.current_curve().map(|c| c.is_stable_open_loop())
    }
}

// =============================================================================
// Tests
// =============================================================================
