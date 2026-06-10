use super::super::data::{FftData, FftPoint};
use super::super::window::WindowFunction;
use super::*;

impl FftState {
    /// Update auto-scale ranges
    pub fn update_auto_scale(&mut self) {
        if let Some(ref data) = self.data {
            if self.freq_auto
                && let Some((min, max)) = data.frequency_range()
            {
                match self.freq_scale {
                    FrequencyScale::Linear => {
                        self.freq_min = min;
                        self.freq_max = max;
                    }
                    FrequencyScale::Log => {
                        self.freq_min = first_positive_frequency(data).unwrap_or(1e-12);
                        self.freq_max = max.max(self.freq_min * 1.01);
                    }
                }
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

                    match self.mag_scale {
                        MagnitudeScale::Linear => {
                            self.mag_min = (min_value - padding).max(0.0);
                            self.mag_max = (max_value + padding).max(self.mag_min + 1e-9);
                        }
                        MagnitudeScale::DB | MagnitudeScale::DBm | MagnitudeScale::DBc => {
                            self.mag_min = (min_value - padding).floor().max(-300.0);
                            self.mag_max = (max_value + padding).ceil().min(120.0);
                        }
                    }
                }
            }
        }
    }

    /// Convert a spectrum point to currently selected display magnitude.
    pub fn display_magnitude(&self, point: &FftPoint) -> f64 {
        match self.mag_scale {
            MagnitudeScale::DB => point.magnitude_db(),
            MagnitudeScale::DBm => point.magnitude_dbm(self.z0),
            MagnitudeScale::Linear => point.magnitude,
            MagnitudeScale::DBc => {
                let fundamental_db = self
                    .analysis
                    .as_ref()
                    .and_then(|analysis| analysis.fundamental_db)
                    .unwrap_or(0.0);
                point.magnitude_db() - fundamental_db
            }
        }
    }

    /// Set window function
    pub fn set_window(&mut self, window: WindowFunction) {
        if self.window == window {
            return;
        }
        self.window = window;
        self.recompute_from_source();
    }

    /// Set magnitude scale
    pub fn set_mag_scale(&mut self, scale: MagnitudeScale) {
        if self.mag_scale == scale {
            return;
        }
        self.mag_scale = scale;
        if self.mag_auto {
            self.update_auto_scale();
        }
    }

    /// Set frequency scale
    pub fn set_freq_scale(&mut self, scale: FrequencyScale) {
        if self.freq_scale == scale {
            return;
        }
        self.freq_scale = scale;
        if self.freq_auto {
            self.update_auto_scale();
        } else if self.freq_scale == FrequencyScale::Log {
            self.freq_min = self.freq_min.max(1e-12);
            if self.freq_max <= self.freq_min {
                self.freq_max = self.freq_min * 1.01;
            }
        }
    }

    /// Set number of harmonics for distortion analysis.
    pub fn set_num_harmonics(&mut self, num_harmonics: usize) {
        self.num_harmonics = num_harmonics.max(1);
        self.recompute_analysis();
    }

    pub fn ensure_peak_cache(&mut self) {
        let Some(data) = self.data.as_ref() else {
            self.peak_cache = PeakCache::default();
            return;
        };
        let threshold_bits = self.peak_threshold_db.to_bits();
        if self.peak_cache.spectrum_revision == self.spectrum_revision
            && self.peak_cache.threshold_bits == threshold_bits
        {
            return;
        }

        self.peak_cache.spectrum_revision = self.spectrum_revision;
        self.peak_cache.threshold_bits = threshold_bits;
        self.peak_cache.peak_indices = data.find_peak_indices(self.peak_threshold_db);
    }

    pub fn cached_peak_indices(&self) -> &[usize] {
        &self.peak_cache.peak_indices
    }

    /// Monotonic revision of the displayed spectrum; bumped on every
    /// recompute. Display caches key on this rather than data identity.
    pub fn spectrum_revision(&self) -> u64 {
        self.spectrum_revision
    }

    pub(super) fn mark_spectrum_changed(&mut self) {
        self.spectrum_revision = self.spectrum_revision.wrapping_add(1);
        self.peak_cache = PeakCache::default();
    }
}

fn first_positive_frequency(data: &FftData) -> Option<f64> {
    data.points
        .iter()
        .map(|p| p.frequency)
        .find(|freq| freq.is_finite() && *freq > 0.0)
}
