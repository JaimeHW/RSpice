//! Distribution presentation settings. Samples belong to retained results;
//! derived bins and descriptive moments belong to the result view plan.

use super::HistogramDisplayMode;
use rspice_results::visualization_document::{HistogramPlotKind, HistogramReading};

/// Histogram presentation settings, independent of the active population.
#[derive(Debug, Clone, PartialEq)]
pub struct HistogramState {
    /// Display mode
    pub mode: HistogramDisplayMode,
    /// The samples against their normal quantiles instead of the bars.
    pub normal_quantile: bool,
    /// Exact measurement name; ordering is not a measurement identity.
    pub selected: Option<String>,
    /// Number of bins (for rebuilding)
    pub bin_count: usize,
    /// Custom range enabled
    pub custom_range: bool,
    /// Custom range values
    pub custom_min: f64,
    pub custom_max: f64,
    /// Whether the fitted normal distribution is drawn over the bars.
    pub normal_fit: bool,
    /// The reader's own specification limits, judged where the run states
    /// none for the measurement.
    pub lower_limit: Option<f64>,
    pub upper_limit: Option<f64>,
}

impl Default for HistogramState {
    fn default() -> Self {
        Self {
            mode: HistogramDisplayMode::Count,
            normal_quantile: false,
            selected: None,
            bin_count: 50,
            custom_range: false,
            custom_min: 0.0,
            custom_max: 1.0,
            normal_fit: true,
            lower_limit: None,
            upper_limit: None,
        }
    }
}

impl HistogramState {
    /// Forget the selected measurement when the result context is cleared.
    pub fn clear_selection(&mut self) {
        self.selected = None;
    }

    /// The reading the view keeps for these settings.
    pub fn reading(&self) -> HistogramReading {
        HistogramReading {
            measurement: self.selected.clone(),
            plot: if self.normal_quantile {
                HistogramPlotKind::NormalQuantile
            } else {
                match self.mode {
                    HistogramDisplayMode::Count => HistogramPlotKind::Count,
                    HistogramDisplayMode::Pdf => HistogramPlotKind::Density,
                    HistogramDisplayMode::Percent => HistogramPlotKind::Percent,
                    HistogramDisplayMode::Cdf => HistogramPlotKind::Cumulative,
                }
            },
            bins: u16::try_from(self.bin_count.clamp(1, 1000)).unwrap_or(1000),
            range: (self.custom_range
                && self.custom_min.is_finite()
                && self.custom_max.is_finite()
                && self.custom_min < self.custom_max)
                .then_some([self.custom_min, self.custom_max]),
            normal_fit: self.normal_fit,
            lower_limit: self.lower_limit,
            upper_limit: self.upper_limit,
        }
    }

    /// Take what the view kept. The bars keep the last distribution mode
    /// while the normal-quantile plot is shown, so leaving it returns to them.
    pub fn apply(&mut self, reading: &HistogramReading) {
        self.selected.clone_from(&reading.measurement);
        self.normal_quantile = reading.plot == HistogramPlotKind::NormalQuantile;
        self.mode = match reading.plot {
            HistogramPlotKind::Count => HistogramDisplayMode::Count,
            HistogramPlotKind::Density => HistogramDisplayMode::Pdf,
            HistogramPlotKind::Percent => HistogramDisplayMode::Percent,
            HistogramPlotKind::Cumulative => HistogramDisplayMode::Cdf,
            HistogramPlotKind::NormalQuantile => self.mode,
        };
        self.bin_count = usize::from(reading.bins.clamp(1, 1000));
        match reading.range {
            Some([low, high]) => {
                self.custom_range = true;
                self.custom_min = low;
                self.custom_max = high;
            }
            None => self.custom_range = false,
        }
        self.normal_fit = reading.normal_fit;
        self.lower_limit = reading.lower_limit;
        self.upper_limit = reading.upper_limit;
    }
}

pub use rspice_results::histogram::measurement_index;
