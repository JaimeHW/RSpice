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

#[cfg(test)]
mod tests {
    use super::super::data::HistogramBuilder;
    use super::*;

    // =========================================================================
    // Display Mode Tests
    // =========================================================================

    #[test]
    fn test_display_mode_default() {
        let mode = HistogramDisplayMode::default();
        assert_eq!(mode, HistogramDisplayMode::Count);
    }

    #[test]
    fn test_display_mode_names() {
        assert_eq!(HistogramDisplayMode::Count.display_name(), "Count");
        assert_eq!(HistogramDisplayMode::Pdf.display_name(), "PDF");
        assert_eq!(HistogramDisplayMode::Cdf.display_name(), "CDF");
    }

    #[test]
    fn test_display_mode_all() {
        let modes = HistogramDisplayMode::all();
        assert_eq!(modes.len(), 4);
    }

    // =========================================================================
    // Axis Scale Tests
    // =========================================================================

    #[test]
    fn test_axis_scale_default() {
        let scale = AxisScale::default();
        assert_eq!(scale, AxisScale::Linear);
    }

    #[test]
    fn test_axis_scale_names() {
        assert_eq!(AxisScale::Linear.display_name(), "Linear");
        assert_eq!(AxisScale::Log.display_name(), "Log");
    }

    // =========================================================================
    // Distribution Overlay Tests
    // =========================================================================

    #[test]
    fn test_overlay_default() {
        let overlay = DistributionOverlay::default();
        assert_eq!(overlay, DistributionOverlay::None);
    }

    #[test]
    fn test_overlay_names() {
        assert_eq!(DistributionOverlay::None.display_name(), "None");
        assert_eq!(DistributionOverlay::Normal.display_name(), "Normal");
    }

    #[test]
    fn test_overlay_all() {
        let overlays = DistributionOverlay::all();
        assert_eq!(overlays.len(), 3);
    }

    // =========================================================================
    // Histogram State Tests
    // =========================================================================

    #[test]
    fn test_state_new() {
        let state = HistogramState::new();
        assert!(state.is_empty());
        assert!(state.show_grid);
        assert!(state.show_stats);
    }

    #[test]
    fn test_state_default() {
        let state = HistogramState::default();
        assert_eq!(state.mode, HistogramDisplayMode::Count);
        assert_eq!(state.y_scale, AxisScale::Linear);
        assert_eq!(state.bin_count, 50);
    }

    #[test]
    fn test_state_load_histogram() {
        let mut state = HistogramState::new();
        let hist = HistogramBuilder::new().build(&[1.0, 2.0, 3.0]);

        state.load_histogram(hist);

        assert_eq!(state.histogram_count(), 1);
        assert!(!state.is_empty());
        assert!(state.current_histogram().is_some());
        assert!(state.current_stats().is_some());
    }

    #[test]
    fn test_state_add_histogram() {
        let mut state = HistogramState::new();
        let hist1 = HistogramBuilder::new().name("A").build(&[1.0, 2.0]);
        let hist2 = HistogramBuilder::new().name("B").build(&[3.0, 4.0]);

        state.load_histogram(hist1);
        state.add_histogram(hist2);

        assert_eq!(state.histogram_count(), 2);
    }

    #[test]
    fn test_state_remove_histogram() {
        let mut state = HistogramState::new();
        state.load_histogram(HistogramBuilder::new().build(&[1.0]));
        state.add_histogram(HistogramBuilder::new().build(&[2.0]));

        state.remove_histogram(0);

        assert_eq!(state.histogram_count(), 1);
    }

    #[test]
    fn test_state_clear() {
        let mut state = HistogramState::new();
        state.load_histogram(HistogramBuilder::new().build(&[1.0, 2.0]));

        state.clear();

        assert!(state.is_empty());
        assert_eq!(state.stats.len(), 0);
    }

    #[test]
    fn test_state_set_mode() {
        let mut state = HistogramState::new();
        state.set_mode(HistogramDisplayMode::Pdf);
        assert_eq!(state.mode, HistogramDisplayMode::Pdf);
    }

    #[test]
    fn test_state_toggle_log_scale() {
        let mut state = HistogramState::new();
        assert_eq!(state.y_scale, AxisScale::Linear);

        state.toggle_log_scale();
        assert_eq!(state.y_scale, AxisScale::Log);

        state.toggle_log_scale();
        assert_eq!(state.y_scale, AxisScale::Linear);
    }

    #[test]
    fn test_state_toggle_stats() {
        let mut state = HistogramState::new();
        let initial = state.show_stats;

        state.toggle_stats();
        assert_ne!(state.show_stats, initial);
    }

    #[test]
    fn test_state_toggle_grid() {
        let mut state = HistogramState::new();
        let initial = state.show_grid;

        state.toggle_grid();
        assert_ne!(state.show_grid, initial);
    }

    #[test]
    fn test_state_set_overlay() {
        let mut state = HistogramState::new();
        state.set_overlay(DistributionOverlay::Normal);
        assert_eq!(state.overlay, DistributionOverlay::Normal);
    }

    #[test]
    fn test_state_select() {
        let mut state = HistogramState::new();
        state.load_histogram(HistogramBuilder::new().build(&[1.0]));
        state.add_histogram(HistogramBuilder::new().build(&[2.0]));

        state.select(1);
        assert_eq!(state.selected, 1);

        // Invalid index is ignored
        state.select(100);
        assert_eq!(state.selected, 1);
    }

    #[test]
    fn test_state_set_bin_count() {
        let mut state = HistogramState::new();

        state.set_bin_count(100);
        assert_eq!(state.bin_count, 100);

        state.set_bin_count(0);
        assert_eq!(state.bin_count, 1); // Min 1

        state.set_bin_count(10000);
        assert_eq!(state.bin_count, 1000); // Max 1000
    }
}
