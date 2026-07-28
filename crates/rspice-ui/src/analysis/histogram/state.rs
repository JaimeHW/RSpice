//! Histogram viewer state: the loaded distributions and how they are shown.
//!
//! Trimmed to what the Distribution viewer and the Monte-Carlo result path
//! actually touch. The axis-scale and distribution-overlay enums, their
//! toggles, and the per-mode display names lived here for a controls row that
//! was never built; the fitting they would have driven
//! (`NormalParams`/`LogNormalParams`) is gone from `statistics` for the same
//! reason.

use super::data::Histogram;
use super::statistics::HistogramStats;

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

/// Complete histogram viewer state
#[derive(Debug, Clone)]
pub struct HistogramState {
    /// Display mode
    pub mode: HistogramDisplayMode,
    /// Multiple histograms (for comparison)
    pub histograms: Vec<Histogram>,
    /// Selected histogram index
    pub selected: usize,
    /// Calculated statistics for each histogram
    pub stats: Vec<HistogramStats>,
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
            histograms: Vec::new(),
            selected: 0,
            stats: Vec::new(),
            bin_count: 50,
            custom_range: false,
            custom_min: 0.0,
            custom_max: 1.0,
        }
    }
}

impl HistogramState {
    /// Replace the contents with a single histogram.
    pub fn load_histogram(&mut self, hist: Histogram) {
        self.histograms = vec![hist];
        self.recalculate_stats();
        self.selected = 0;
    }

    /// Append a histogram for comparison against the ones already loaded.
    pub fn add_histogram(&mut self, hist: Histogram) {
        self.histograms.push(hist);
        self.stats.push(HistogramStats::from_histogram(
            self.histograms.last().unwrap(),
        ));
    }

    /// Clear all histograms
    pub fn clear(&mut self) {
        self.histograms.clear();
        self.stats.clear();
        self.selected = 0;
    }

    /// Recalculate statistics for all histograms
    fn recalculate_stats(&mut self) {
        self.stats = self
            .histograms
            .iter()
            .map(HistogramStats::from_histogram)
            .collect();
    }

    /// Is empty?
    pub fn is_empty(&self) -> bool {
        self.histograms.is_empty()
    }
}
