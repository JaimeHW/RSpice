//! FFT plot view state: axis ranges, zoom, and cursor position.

use super::super::data::FftPoint;
use super::super::window::WindowFunction;
use super::*;

impl FftState {
    /// Update auto-scale ranges
    pub fn update_auto_scale(&mut self) {
        if let Some(ref data) = self.data {
            if self.freq_auto
                && let Some((min, max)) = data.frequency_range()
            {
                self.freq_min = min;
                self.freq_max = max;
            }

            if self.mag_auto {
                let mut min_value = f64::INFINITY;
                let mut max_value = f64::NEG_INFINITY;
                let mut has_finite = false;
                for point in &data.points {
                    let value = self.display_magnitude(point);
                    if value.is_finite() {
                        has_finite = true;
                        min_value = min_value.min(value);
                        max_value = max_value.max(value);
                    }
                }

                if has_finite {
                    let span = (max_value - min_value).abs();
                    let padding = if span > 0.0 { span * 0.1 } else { 1.0 };
                    self.mag_min = (min_value - padding).floor().max(-300.0);
                    self.mag_max = (max_value + padding).ceil().min(120.0);
                }
            }
        }
    }

    /// Display magnitude for a spectrum point. The viewer plots dB on a linear
    /// frequency axis; there is no control to select another scale.
    pub fn display_magnitude(&self, point: &FftPoint) -> f64 {
        point.magnitude_db()
    }

    /// Set window function
    pub fn set_window(&mut self, window: WindowFunction) -> Result<(), FftFailure> {
        if self.window == window && self.has_data() {
            return Ok(());
        }
        self.window = window;
        self.recompute_from_source()
    }

    /// Monotonic revision of the displayed spectrum; bumped on every
    /// recompute. Display caches key on this rather than data identity.
    pub fn spectrum_revision(&self) -> u64 {
        self.spectrum_revision
    }

    pub(super) fn mark_spectrum_changed(&mut self) {
        self.spectrum_revision = self.spectrum_revision.wrapping_add(1);
    }
}
