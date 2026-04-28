//! Histogram State Management
//!
//! Viewer state for histogram display including axis modes and overlay options.

use super::data::Histogram;
use super::statistics::HistogramStats;

// =============================================================================
// Display Mode
// =============================================================================

/// Histogram display mode
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum HistogramDisplayMode {
    /// Bar chart (count per bin)
    #[default]
    Count,
    /// Probability density function
    Pdf,
    /// Cumulative distribution function
    Cdf,
    /// Percent of total
    Percent,
}

impl HistogramDisplayMode {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Count => "Count",
            Self::Pdf => "PDF",
            Self::Cdf => "CDF",
            Self::Percent => "Percent",
        }
    }

    /// Get all modes
    pub fn all() -> &'static [HistogramDisplayMode] {
        &[Self::Count, Self::Pdf, Self::Cdf, Self::Percent]
    }
}

// =============================================================================
// Axis Scale
// =============================================================================

/// Vertical axis scale
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AxisScale {
    /// Linear scale
    #[default]
    Linear,
    /// Logarithmic scale
    Log,
}

impl AxisScale {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Linear => "Linear",
            Self::Log => "Log",
        }
    }
}

// =============================================================================
// Overlay Options
// =============================================================================

/// Distribution overlay options
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DistributionOverlay {
    /// No overlay
    #[default]
    None,
    /// Normal distribution fit
    Normal,
    /// Log-normal distribution fit
    LogNormal,
}

impl DistributionOverlay {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Normal => "Normal",
            Self::LogNormal => "Log-Normal",
        }
    }

    /// Get all overlays
    pub fn all() -> &'static [DistributionOverlay] {
        &[Self::None, Self::Normal, Self::LogNormal]
    }
}

// =============================================================================
// Histogram State
// =============================================================================

/// Complete histogram viewer state
#[derive(Debug, Clone)]
pub struct HistogramState {
    /// Display mode
    pub mode: HistogramDisplayMode,
    /// Vertical axis scale
    pub y_scale: AxisScale,
    /// Multiple histograms (for comparison)
    pub histograms: Vec<Histogram>,
    /// Selected histogram index
    pub selected: usize,
    /// Calculated statistics for each histogram
    pub stats: Vec<HistogramStats>,
    /// Distribution overlay type
    pub overlay: DistributionOverlay,
    /// Show grid
    pub show_grid: bool,
    /// Show statistics panel
    pub show_stats: bool,
    /// Number of bins (for rebuilding)
    pub bin_count: usize,
    /// Custom range enabled
    pub custom_range: bool,
    /// Custom range values
    pub custom_min: f64,
    pub custom_max: f64,
}

impl Default for HistogramState {
    fn default() -> Self {
        Self {
            mode: HistogramDisplayMode::Count,
            y_scale: AxisScale::Linear,
            histograms: Vec::new(),
            selected: 0,
            stats: Vec::new(),
            overlay: DistributionOverlay::None,
            show_grid: true,
            show_stats: true,
            bin_count: 50,
            custom_range: false,
            custom_min: 0.0,
            custom_max: 1.0,
        }
    }
}

impl HistogramState {
    /// Create new state
    pub fn new() -> Self {
        Self::default()
    }

    /// Load a single histogram
    pub fn load_histogram(&mut self, hist: Histogram) {
        self.histograms = vec![hist];
        self.recalculate_stats();
        self.selected = 0;
    }

    /// Add histogram to comparison
    pub fn add_histogram(&mut self, hist: Histogram) {
        self.histograms.push(hist);
        self.stats.push(HistogramStats::from_histogram(
            self.histograms.last().unwrap(),
        ));
    }

    /// Remove histogram by index
    pub fn remove_histogram(&mut self, index: usize) {
        if index < self.histograms.len() {
            self.histograms.remove(index);
            self.stats.remove(index);
            if self.selected >= self.histograms.len() && !self.histograms.is_empty() {
                self.selected = self.histograms.len() - 1;
            }
        }
    }

    /// Clear all histograms
    pub fn clear(&mut self) {
        self.histograms.clear();
        self.stats.clear();
        self.selected = 0;
    }

    /// Get currently selected histogram
    pub fn current_histogram(&self) -> Option<&Histogram> {
        self.histograms.get(self.selected)
    }

    /// Get current statistics
    pub fn current_stats(&self) -> Option<&HistogramStats> {
        self.stats.get(self.selected)
    }

    /// Recalculate statistics for all histograms
    pub fn recalculate_stats(&mut self) {
        self.stats = self
            .histograms
            .iter()
            .map(HistogramStats::from_histogram)
            .collect();
    }

    /// Set display mode
    pub fn set_mode(&mut self, mode: HistogramDisplayMode) {
        self.mode = mode;
    }

    /// Toggle y-axis scale
    pub fn toggle_log_scale(&mut self) {
        self.y_scale = match self.y_scale {
            AxisScale::Linear => AxisScale::Log,
            AxisScale::Log => AxisScale::Linear,
        };
    }

    /// Toggle statistics panel
    pub fn toggle_stats(&mut self) {
        self.show_stats = !self.show_stats;
    }

    /// Toggle grid
    pub fn toggle_grid(&mut self) {
        self.show_grid = !self.show_grid;
    }

    /// Set distribution overlay
    pub fn set_overlay(&mut self, overlay: DistributionOverlay) {
        self.overlay = overlay;
    }

    /// Number of histograms
    pub fn histogram_count(&self) -> usize {
        self.histograms.len()
    }

    /// Is empty?
    pub fn is_empty(&self) -> bool {
        self.histograms.is_empty()
    }

    /// Select histogram by index
    pub fn select(&mut self, index: usize) {
        if index < self.histograms.len() {
            self.selected = index;
        }
    }

    /// Set bin count and rebuild if needed
    pub fn set_bin_count(&mut self, count: usize) {
        self.bin_count = count.clamp(1, 1000);
    }
}

// =============================================================================
// Tests
// =============================================================================

