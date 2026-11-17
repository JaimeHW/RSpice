//! Host presentation hooks around exact retained waveform data.

use super::RetainedWaveform;

/// Output calculations operate on exact samples. Hosts may retain presentation
/// alongside them; these hooks never supply numerical data to a calculation.
pub trait OutputWaveform: AsRef<RetainedWaveform> + AsMut<RetainedWaveform> + Clone {
    fn from_retained(data: RetainedWaveform) -> Self;

    fn set_visible(&mut self, _visible: bool) {}
    fn adopt_presentation(&mut self, _source: Self) {}

    fn into_output_preview(self, maximum_samples: usize) -> Result<Self, String>;
}

impl OutputWaveform for RetainedWaveform {
    fn from_retained(data: RetainedWaveform) -> Self {
        data
    }

    fn into_output_preview(self, maximum_samples: usize) -> Result<Self, String> {
        self.into_bounded_preview(maximum_samples)
    }
}
