//! Bode Plot State Management
//!
//! Viewer state for Bode plot display including axis modes and trace selection.

use super::data::BodeData;

// =============================================================================
// Display Mode
// =============================================================================

/// Bode plot display mode
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BodeDisplayMode {
    /// Magnitude and phase (both)
    #[default]
    Both,
    /// Magnitude only
    MagnitudeOnly,
    /// Phase only
    PhaseOnly,
}

impl BodeDisplayMode {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Both => "Mag + Phase",
            Self::MagnitudeOnly => "Magnitude",
            Self::PhaseOnly => "Phase",
        }
    }

    /// Get all modes
    pub fn all() -> &'static [BodeDisplayMode] {
        &[Self::Both, Self::MagnitudeOnly, Self::PhaseOnly]
    }

    /// Show magnitude plot?
    pub fn show_magnitude(&self) -> bool {
        matches!(self, Self::Both | Self::MagnitudeOnly)
    }

    /// Show phase plot?
    pub fn show_phase(&self) -> bool {
        matches!(self, Self::Both | Self::PhaseOnly)
    }
}

// =============================================================================
// Phase Wrap Mode
// =============================================================================

/// Phase display wrapping mode
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PhaseWrapMode {
    /// Wrap to ±180°
    #[default]
    Wrap180,
    /// Wrap to 0-360°
    Wrap360,
    /// Continuous (unwrapped)
    Continuous,
}

impl PhaseWrapMode {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Wrap180 => "±180°",
            Self::Wrap360 => "0-360°",
            Self::Continuous => "Continuous",
        }
    }
}

// =============================================================================
// Bode Plot State
// =============================================================================

/// Complete Bode plot viewer state
#[derive(Debug, Clone)]
pub struct BodePlotState {
    /// Display mode
    pub mode: BodeDisplayMode,
    /// Bode data
    pub data: BodeData,
    /// Phase wrap mode
    pub phase_wrap: PhaseWrapMode,
    /// Show grid
    pub show_grid: bool,
    /// Show margins
    pub show_margins: bool,
    /// Show cursor
    pub show_cursor: bool,
    /// Cursor frequency (Hz)
    pub cursor_frequency: Option<f64>,
    /// Selected trace index
    pub selected_trace: usize,
    /// Magnitude axis min (dB)
    pub mag_min: f64,
    /// Magnitude axis max (dB)
    pub mag_max: f64,
    /// Auto-scale magnitude
    pub mag_auto: bool,
    /// Phase axis min (degrees)
    pub phase_min: f64,
    /// Phase axis max (degrees)
    pub phase_max: f64,
    /// Auto-scale phase
    pub phase_auto: bool,
}

impl Default for BodePlotState {
    fn default() -> Self {
        Self {
            mode: BodeDisplayMode::Both,
            data: BodeData::new(),
            phase_wrap: PhaseWrapMode::Wrap180,
            show_grid: true,
            show_margins: true,
            show_cursor: true,
            cursor_frequency: None,
            selected_trace: 0,
            mag_min: -60.0,
            mag_max: 60.0,
            mag_auto: true,
            phase_min: -180.0,
            phase_max: 0.0,
            phase_auto: true,
        }
    }
}

impl BodePlotState {
    /// Create new state
    pub fn new() -> Self {
        Self::default()
    }

    /// Load data
    pub fn load_data(&mut self, data: BodeData) {
        self.data = data;
        self.update_auto_scale();
    }

    /// Update auto-scale ranges
    pub fn update_auto_scale(&mut self) {
        if self.mag_auto
            && let Some((min, max)) = self.data.magnitude_range_db()
        {
            let padding = (max - min) * 0.1;
            self.mag_min = (min - padding).floor();
            self.mag_max = (max + padding).ceil();
        }

        if self.phase_auto {
            // Default to -180 to 0 for typical stable systems
            self.phase_min = -180.0;
            self.phase_max = 0.0;
        }
    }

    /// Set display mode
    pub fn set_mode(&mut self, mode: BodeDisplayMode) {
        self.mode = mode;
    }

    /// Toggle margins display
    pub fn toggle_margins(&mut self) {
        self.show_margins = !self.show_margins;
    }

    /// Toggle grid
    pub fn toggle_grid(&mut self) {
        self.show_grid = !self.show_grid;
    }

    /// Toggle cursor
    pub fn toggle_cursor(&mut self) {
        self.show_cursor = !self.show_cursor;
    }

    /// Set cursor frequency
    pub fn set_cursor(&mut self, frequency: Option<f64>) {
        self.cursor_frequency = frequency;
    }

    /// Get number of traces
    pub fn trace_count(&self) -> usize {
        self.data.response_count()
    }

    /// Is empty?
    pub fn is_empty(&self) -> bool {
        self.data.response_count() == 0
    }

    /// Get stability margins (if calculated)
    pub fn margins(&self) -> Option<&super::data::StabilityMargins> {
        self.data.margins.as_ref()
    }
}

// =============================================================================
// Tests
// =============================================================================
