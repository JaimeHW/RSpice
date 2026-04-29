use std::sync::Arc;

use super::super::data::{FftData, SpectrumAnalysis, SpectrumNormalization};
use super::super::pipeline::{
    FftInputOptions, FftInputPolicy, FftTimeWindow, MAX_REFERENCE_RESAMPLE_POINTS, MIN_FFT_SAMPLES,
    PreparedFftInput,
};
use super::*;
impl FftState {
    /// Create new state
    pub fn new() -> Self {
        Self::default()
    }

    /// Load FFT data and analyze
    pub fn load_data(&mut self, mut data: FftData) {
        data.convert_normalization(self.normalization);
        let analysis = SpectrumAnalysis::analyze(&data, self.num_harmonics);
        self.data = Some(data);
        self.analysis = Some(analysis);
        self.source_cache = None;
        self.mark_spectrum_changed();
        self.update_auto_scale();
    }

    /// Load prepared uniformly sampled source and compute FFT using current settings.
    pub fn load_prepared_input(&mut self, input: PreparedFftInput) {
        if self.selected_source.is_none() {
            self.selected_source = Some(input.name.clone());
        }
        self.source_cache = Some(FftSourceCache {
            name: input.name,
            samples: Arc::from(input.samples),
            sample_rate: input.sample_rate,
            original_count: input.original_count,
            decimation_factor: input.decimation_factor,
        });
        self.sync_sample_count_control_value();
        self.recompute_from_source();
    }

    /// Select preferred source trace name.
    pub fn set_selected_source(&mut self, source_name: Option<String>) {
        self.selected_source = source_name;
    }

    /// Set FFT input fidelity mode.
    pub fn set_input_fidelity(&mut self, input_fidelity: InputFidelity) {
        self.input_fidelity = input_fidelity;
    }

    /// Active FFT input pipeline policy.
    pub fn input_policy(&self) -> FftInputPolicy {
        self.input_fidelity.input_policy()
    }

    /// Build pipeline input options for a source timeline.
    pub fn input_options_for_waveform(&self, source_time: &[f64]) -> FftInputOptions {
        self.input_options_for_bounds(finite_time_bounds(source_time))
    }

    /// Build pipeline input options from source bounds.
    pub fn input_options_for_bounds(&self, source_bounds: Option<(f64, f64)>) -> FftInputOptions {
        let time_window = if self.time_window_auto {
            None
        } else if let Some((min_t, max_t)) = source_bounds {
            let (mut start, mut end) =
                if self.time_window_start.is_finite() && self.time_window_end.is_finite() {
                    (
                        self.time_window_start.clamp(min_t, max_t),
                        self.time_window_end.clamp(min_t, max_t),
                    )
                } else {
                    (min_t, max_t)
                };
            if end <= start {
                start = min_t;
                end = max_t;
            }
            if end > start {
                Some(FftTimeWindow::new(start, end))
            } else {
                None
            }
        } else {
            None
        };

        let target_samples = if self.sample_count_auto {
            None
        } else {
            Some(
                self.sample_count
                    .clamp(MIN_FFT_SAMPLES, MAX_REFERENCE_RESAMPLE_POINTS),
            )
        };

        FftInputOptions::with_policy(self.input_policy())
            .with_time_window(time_window)
            .with_target_samples(target_samples)
    }

    /// Keep the UI `N` control value synchronized with the effective FFT input.
    ///
    /// - In auto mode, mirror the prepared source sample count (when available).
    /// - In manual mode, clamp to valid FFT bounds.
    pub fn sync_sample_count_control_value(&mut self) {
        if self.sample_count_auto {
            if let Some(sample_len) = self
                .source_cache
                .as_ref()
                .map(|source| source.samples.len())
            {
                self.sample_count =
                    sample_len.clamp(MIN_FFT_SAMPLES, MAX_REFERENCE_RESAMPLE_POINTS);
            }
        } else {
            self.sample_count = self
                .sample_count
                .clamp(MIN_FFT_SAMPLES, MAX_REFERENCE_RESAMPLE_POINTS);
        }
    }

    /// Recompute FFT data from cached source using current window.
    pub fn recompute_from_source(&mut self) {
        let Some(source) = self.source_cache.as_ref() else {
            return;
        };
        let mut data = FftData::from_time_domain(
            &format!("FFT({})", source.name),
            &source.samples,
            source.sample_rate,
            self.window,
        );
        data.convert_normalization(self.normalization);
        if data.is_empty() {
            self.data = None;
            self.analysis = None;
            self.mark_spectrum_changed();
            return;
        }
        let analysis = SpectrumAnalysis::analyze(&data, self.num_harmonics);
        self.data = Some(data);
        self.analysis = Some(analysis);
        self.mark_spectrum_changed();
        self.update_auto_scale();
    }

    /// Set amplitude normalization mode.
    ///
    /// This performs an in-place O(N) rescale on loaded bins to avoid expensive
    /// FFT recomputation for simple RMS/Peak toggles.
    pub fn set_normalization(&mut self, normalization: SpectrumNormalization) {
        if self.normalization == normalization {
            return;
        }
        self.normalization = normalization;

        if let Some(data) = self.data.as_mut() {
            data.convert_normalization(normalization);
            self.recompute_analysis();
            self.mark_spectrum_changed();
            self.update_auto_scale();
            return;
        }

        if self.source_cache.is_some() {
            self.recompute_from_source();
        }
    }

    /// Recompute scalar analysis from currently loaded spectrum data.
    pub fn recompute_analysis(&mut self) {
        if let Some(data) = self.data.as_ref() {
            self.analysis = Some(SpectrumAnalysis::analyze(data, self.num_harmonics));
        } else {
            self.analysis = None;
        }
    }

    /// Clear data
    pub fn clear(&mut self) {
        self.data = None;
        self.analysis = None;
        self.source_cache = None;
        self.clear_markers();
        self.mark_spectrum_changed();
    }

    /// Has data?
    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    /// Is empty?
    pub fn is_empty(&self) -> bool {
        self.data.is_none()
    }
}

fn finite_time_bounds(time: &[f64]) -> Option<(f64, f64)> {
    let start = time.iter().copied().find(|t| t.is_finite())?;
    let end = time.iter().copied().rfind(|t| t.is_finite())?;
    if end > start {
        Some((start, end))
    } else {
        None
    }
}
