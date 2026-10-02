//! Distribution presentation settings. Samples belong to retained results;
//! derived bins and descriptive moments belong to the result view plan.

use super::HistogramDisplayMode;

/// Histogram presentation settings, independent of the active population.
#[derive(Debug, Clone)]
pub struct HistogramState {
    /// Display mode
    pub mode: HistogramDisplayMode,
    /// Exact measurement name; ordering is not a measurement identity.
    pub selected: Option<String>,
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
            selected: None,
            bin_count: 50,
            custom_range: false,
            custom_min: 0.0,
            custom_max: 1.0,
        }
    }
}

impl HistogramState {
    /// Forget the selected measurement when the result context is cleared.
    pub fn clear_selection(&mut self) {
        self.selected = None;
    }
}

pub use rspice_results::histogram::measurement_index;
