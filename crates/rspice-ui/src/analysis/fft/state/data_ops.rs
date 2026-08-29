//! Recomputing the spectrum when its inputs change.

use std::sync::Arc;

use super::super::data::{FftData, SpectrumAnalysis};
use super::super::pipeline::{
    FftInputError, FftInputOptions, FftInputPolicy, FftTimeWindow, MAX_REFERENCE_RESAMPLE_POINTS,
    MIN_FFT_SAMPLES, PreparedFftInput,
};
use super::*;
impl FftState {
    /// Load FFT data and analyze
    #[cfg(test)]
    pub fn load_data(&mut self, mut data: FftData) {
        data.convert_normalization(self.normalization)
            .expect("finite imported FFT fixture normalization");
        let analysis = SpectrumAnalysis::analyze(&data, self.num_harmonics);
        self.data = Some(data);
        self.analysis = Some(analysis);
        self.source_cache = None;
        self.last_error = None;
        self.mark_spectrum_changed();
        self.update_auto_scale();
    }

    /// Load prepared uniformly sampled source and compute FFT using current settings.
    pub fn load_prepared_input(&mut self, input: PreparedFftInput) -> Result<(), FftBuildError> {
        if self.selected_source.is_none() {
            self.selected_source = Some(input.name.clone());
        }
        self.source_cache = Some(FftSourceCache {
            name: input.name,
            // Moving the existing Vec avoids Arc<[T]>'s second large copy.
            samples: Arc::new(input.samples),
            sample_rate: input.sample_rate,
        });
        self.sync_sample_count_control_value();
        self.recompute_from_source()
    }

    /// Select preferred source trace name.
    pub fn set_selected_source(&mut self, source_name: Option<String>) {
        self.selected_source = source_name;
    }

    /// Change the displayed amplitude convention and recompute all dependent
    /// metrics from the same prepared time-domain source. Imported test data
    /// without a retained source is converted in place instead.
    pub fn set_normalization(
        &mut self,
        normalization: SpectrumNormalization,
    ) -> Result<(), FftBuildError> {
        if self.normalization == normalization {
            return Ok(());
        }
        self.normalization = normalization;
        if self.source_cache.is_some() {
            return self.recompute_from_source();
        }
        let Some(data) = self.data.as_mut() else {
            return Ok(());
        };
        match data.convert_normalization(normalization) {
            Ok(()) => {
                self.analysis = Some(SpectrumAnalysis::analyze(data, self.num_harmonics));
                self.last_error = None;
                self.mark_spectrum_changed();
                self.update_auto_scale();
                Ok(())
            }
            Err(error) => {
                self.data = None;
                self.analysis = None;
                self.last_error = Some(error.clone().into());
                self.mark_spectrum_changed();
                Err(error)
            }
        }
    }

    /// Active FFT input pipeline policy.
    pub fn input_policy(&self) -> FftInputPolicy {
        self.input_fidelity.input_policy()
    }

    /// Build pipeline input options for a source timeline.
    pub fn input_options_for_waveform(&self, _source_time: &[f64]) -> FftInputOptions {
        self.input_options_for_bounds(None)
    }

    /// Build pipeline input options from source bounds.
    pub fn input_options_for_bounds(&self, _source_bounds: Option<(f64, f64)>) -> FftInputOptions {
        let time_window = if self.time_window_auto {
            None
        } else {
            Some(FftTimeWindow::new(
                self.time_window_start,
                self.time_window_end,
            ))
        };

        let target_samples = if self.sample_count_auto {
            None
        } else {
            Some(self.sample_count)
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
    pub fn recompute_from_source(&mut self) -> Result<(), FftBuildError> {
        let Some(source) = self.source_cache.as_ref() else {
            return Ok(());
        };
        let result = FftData::from_time_domain_with_normalization(
            &source.name,
            &source.samples,
            source.sample_rate,
            self.window,
            self.normalization,
        );
        match result {
            Ok(data) => {
                let analysis = SpectrumAnalysis::analyze(&data, self.num_harmonics);
                self.data = Some(data);
                self.analysis = Some(analysis);
                self.last_error = None;
                self.mark_spectrum_changed();
                self.update_auto_scale();
                Ok(())
            }
            Err(error) => {
                self.data = None;
                self.analysis = None;
                self.last_error = Some(error.clone().into());
                self.mark_spectrum_changed();
                Err(error)
            }
        }
    }

    /// Record a fail-closed input-preparation error transactionally.
    pub fn record_input_error(&mut self, error: FftInputError) {
        self.data = None;
        self.analysis = None;
        self.source_cache = None;
        self.last_error = Some(error.into());
        self.mark_spectrum_changed();
    }

    /// Record an unexpected asynchronous worker termination transactionally.
    pub fn record_worker_disconnect(&mut self) {
        self.data = None;
        self.analysis = None;
        self.source_cache = None;
        self.last_error = Some(FftFailure::WorkerDisconnected);
        self.mark_spectrum_changed();
    }

    /// Clear data
    pub fn clear(&mut self) {
        self.data = None;
        self.analysis = None;
        self.source_cache = None;
        self.last_error = None;
        self.marker_frequencies.clear();
        self.mark_spectrum_changed();
    }

    /// Has data?
    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::fft::data::SpectrumNormalization;

    #[test]
    fn normalization_change_recomputes_spectrum_metrics_and_revision() {
        let samples = (0..64)
            .map(|index| (index as f64 * std::f64::consts::TAU / 8.0).sin())
            .collect::<Vec<_>>();
        let input = PreparedFftInput {
            name: "V(out)".to_owned(),
            samples,
            sample_rate: 64.0,
            original_count: 64,
            decimation_factor: 1,
        };
        let mut state = FftState::default();
        state
            .load_prepared_input(input)
            .expect("finite qualified normalization fixture");
        let rms_level = state
            .analysis
            .as_ref()
            .and_then(|analysis| analysis.fundamental_db)
            .unwrap();
        let revision = state.spectrum_revision();

        state
            .set_normalization(SpectrumNormalization::Peak)
            .expect("representable normalization change");

        let peak_level = state
            .analysis
            .as_ref()
            .and_then(|analysis| analysis.fundamental_db)
            .unwrap();
        assert_eq!(
            state.data.as_ref().unwrap().normalization,
            SpectrumNormalization::Peak
        );
        assert!(state.spectrum_revision() > revision);
        assert!((peak_level - rms_level - 3.010_299_956_639_812).abs() < 1.0e-9);
    }

    #[test]
    fn failed_rebuild_clears_stale_spectrum_and_retains_typed_diagnostic() {
        let valid = PreparedFftInput {
            name: "V(valid)".to_owned(),
            samples: vec![0.0; 16],
            sample_rate: 16.0,
            original_count: 16,
            decimation_factor: 1,
        };
        let mut state = FftState::default();
        state
            .load_prepared_input(valid.clone())
            .expect("finite qualified source");
        assert!(state.data.is_some());
        assert!(state.analysis.is_some());
        assert!(state.last_error.is_none());

        let mut invalid = valid.clone();
        invalid.name = "V(invalid)".to_owned();
        invalid.samples[7] = f64::NAN;
        assert!(matches!(
            state.load_prepared_input(invalid),
            Err(FftBuildError::NonFiniteInputSample { index: 7, .. })
        ));
        assert!(state.data.is_none());
        assert!(state.analysis.is_none());
        assert!(matches!(
            state.last_error,
            Some(FftFailure::Build(FftBuildError::NonFiniteInputSample {
                index: 7,
                ..
            }))
        ));

        state
            .load_prepared_input(valid)
            .expect("a later valid build recovers");
        assert!(state.data.is_some());
        assert!(state.analysis.is_some());
        assert!(state.last_error.is_none());
    }

    #[test]
    fn input_failure_is_transactional_and_a_valid_retry_clears_it() {
        let mut state = FftState::default();
        state.record_input_error(FftInputError::LengthMismatch {
            time_count: 17,
            value_count: 16,
        });
        assert!(!state.has_data());
        assert!(state.source_cache.is_none());
        assert!(matches!(
            state.last_error,
            Some(FftFailure::Input(FftInputError::LengthMismatch {
                time_count: 17,
                value_count: 16
            }))
        ));

        state
            .load_prepared_input(PreparedFftInput {
                name: "V(recovered)".to_owned(),
                samples: vec![0.0; 16],
                sample_rate: 16.0,
                original_count: 16,
                decimation_factor: 1,
            })
            .expect("valid preparation recovers after an input failure");
        assert!(state.has_data());
        assert!(state.last_error.is_none());
    }
}
