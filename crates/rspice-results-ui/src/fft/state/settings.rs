//! The sheet's settings as a result view keeps them, and taking them in.

use super::super::data::{SpectrumMeasurement, SpectrumNormalization};
use super::super::pipeline::Resampling;
use super::super::window::{KaiserBeta, WindowFunction};
use super::*;
use rspice_results::fft::settings::{
    SpectrumAmplitude, SpectrumResampling, SpectrumSampling, SpectrumSettings, SpectrumWindow,
};

/// What taking in new settings asks of the spectrum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsChange {
    /// Nothing the spectrum depends on changed.
    None,
    /// The spectrum is transformed or measured again from the record it
    /// holds.
    Recomputed,
    /// The record itself changed: the spectrum is dropped, and the sheet
    /// prepares the record again.
    Record,
}

impl FftState {
    /// The settings the sheet shows, as a view keeps them.
    #[must_use]
    pub fn settings(&self) -> SpectrumSettings {
        SpectrumSettings {
            source: self.source_choice.clone(),
            window: window_kind(self.window),
            kaiser_beta: match self.window {
                WindowFunction::Kaiser(beta) => beta.get(),
                _ => self.kaiser_beta.get(),
            },
            amplitude: match self.normalization {
                SpectrumNormalization::Peak => SpectrumAmplitude::Peak,
                SpectrumNormalization::Rms => SpectrumAmplitude::Rms,
            },
            record: (!self.time_window_auto)
                .then_some((self.time_window_start, self.time_window_end)),
            sampling: match (self.sample_count_auto, self.sample_rate) {
                (true, _) => SpectrumSampling::Auto,
                (false, Some(rate)) => SpectrumSampling::Rate(rate),
                (false, None) => SpectrumSampling::Count(self.sample_count),
            },
            resampling: match self.resampling {
                Resampling::Linear => SpectrumResampling::Interpolate,
                Resampling::SampleAndHold => SpectrumResampling::SampleAndHold,
            },
            padding: self.padding,
            averages: self.averages,
            harmonics: self.num_harmonics,
            signal_bins: self.signal_bins,
            fundamental: self.fundamental,
            band: self.band,
            scale: self.scale,
            log_frequency: self.log_frequency,
            phase: self.phase,
        }
    }

    /// Take in `settings`, as a view keeps them or the reader set them, and
    /// say what that asks of the spectrum. A change of record drops the
    /// spectrum; a change of transform or measurement recomputes it from the
    /// record it holds.
    pub fn adopt_settings(&mut self, settings: &SpectrumSettings) -> SettingsChange {
        let before = self.settings();
        if before == *settings {
            return SettingsChange::None;
        }
        self.source_choice = settings.source.clone();
        self.kaiser_beta = KaiserBeta::new(settings.kaiser_beta).unwrap_or_default();
        self.window = settings.window.with_beta(self.kaiser_beta);
        self.normalization = match settings.amplitude {
            SpectrumAmplitude::Peak => SpectrumNormalization::Peak,
            SpectrumAmplitude::Rms => SpectrumNormalization::Rms,
        };
        match settings.record {
            Some((start, stop)) => {
                self.time_window_auto = false;
                self.time_window_start = start;
                self.time_window_end = stop;
            }
            None => self.time_window_auto = true,
        }
        match settings.sampling {
            SpectrumSampling::Auto => {
                self.sample_count_auto = true;
                self.sample_rate = None;
            }
            SpectrumSampling::Count(count) => {
                self.sample_count_auto = false;
                self.sample_count = count;
                self.sample_rate = None;
            }
            SpectrumSampling::Rate(rate) => {
                self.sample_count_auto = false;
                self.sample_rate = Some(rate);
            }
        }
        self.resampling = match settings.resampling {
            SpectrumResampling::Interpolate => Resampling::Linear,
            SpectrumResampling::SampleAndHold => Resampling::SampleAndHold,
        };
        self.padding = settings.padding.max(1);
        self.averages = settings.averages.max(1);
        self.num_harmonics = settings.harmonics;
        self.signal_bins = settings.signal_bins;
        self.fundamental = settings.fundamental;
        self.band = settings.band;
        self.scale = settings.scale;
        self.log_frequency = settings.log_frequency;
        self.phase = settings.phase;

        if !before.same_record(settings) {
            self.clear();
            return SettingsChange::Record;
        }
        let transformed = before.window != settings.window
            || before.kaiser_beta != settings.kaiser_beta
            || before.amplitude != settings.amplitude
            || before.padding != settings.padding
            || before.averages != settings.averages
            || before.harmonics != settings.harmonics
            || before.signal_bins != settings.signal_bins
            || before.fundamental != settings.fundamental
            || before.band != settings.band;
        if !transformed {
            return SettingsChange::None;
        }
        if self.source_cache.is_some() {
            // A failure is kept on the state, where the sheet reports it.
            let _ = self.recompute_from_source();
        } else {
            self.clear();
        }
        SettingsChange::Recomputed
    }

    /// What the figures are measured over, as the reader set it.
    pub(super) fn measurement(&self) -> SpectrumMeasurement {
        SpectrumMeasurement {
            harmonics: self.num_harmonics,
            signal_bins: self.signal_bins,
            fundamental: self.fundamental,
            band: self.band,
        }
    }
}

fn window_kind(window: WindowFunction) -> SpectrumWindow {
    match window {
        WindowFunction::Rectangular => SpectrumWindow::Rectangular,
        WindowFunction::Hanning => SpectrumWindow::Hanning,
        WindowFunction::Hamming => SpectrumWindow::Hamming,
        WindowFunction::Blackman => SpectrumWindow::Blackman,
        WindowFunction::BlackmanHarris => SpectrumWindow::BlackmanHarris,
        WindowFunction::FlatTop => SpectrumWindow::FlatTop,
        WindowFunction::Kaiser(_) => SpectrumWindow::Kaiser,
        WindowFunction::Gaussian => SpectrumWindow::Gaussian,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fft::pipeline::{FftTimeWindow, PreparedFftInput};

    fn loaded() -> FftState {
        let samples = (0..4096)
            .map(|n| (std::f64::consts::TAU * 64.0 * n as f64 / 4096.0).sin())
            .collect();
        let mut state = FftState::default();
        state
            .load_prepared_input(PreparedFftInput {
                name: "V(out)".to_owned(),
                samples,
                sample_rate: 4096.0,
                original_count: 4096,
                decimation_factor: 1,
                dc_level: 0.0,
                record: FftTimeWindow::new(0.0, 1.0),
            })
            .expect("a spectrum");
        state
    }

    /// The settings the sheet reads back are the ones it took in.
    #[test]
    fn settings_taken_in_are_read_back() {
        let mut state = loaded();
        let settings = SpectrumSettings {
            source: Some("V(in)".to_owned()),
            window: SpectrumWindow::Kaiser,
            kaiser_beta: 20.0,
            record: Some((0.25, 0.75)),
            sampling: SpectrumSampling::Rate(8192.0),
            resampling: SpectrumResampling::SampleAndHold,
            padding: 4,
            averages: 2,
            harmonics: 40,
            signal_bins: Some(5),
            fundamental: Some(64.0),
            band: Some((1.0, 1000.0)),
            scale: rspice_results::fft::settings::SpectrumScale::Psd,
            log_frequency: true,
            phase: true,
            ..SpectrumSettings::default()
        };
        assert_eq!(state.adopt_settings(&settings), SettingsChange::Record);
        assert_eq!(state.settings(), settings);
        assert_eq!(state.adopt_settings(&settings), SettingsChange::None);
    }

    /// A window, a β or a harmonic count is measured again from the record
    /// held; another record or source drops the spectrum for the sheet to
    /// prepare.
    #[test]
    fn a_new_transform_recomputes_and_a_new_record_drops_the_spectrum() {
        let mut state = loaded();
        let revision = state.spectrum_revision();
        let kaiser = SpectrumSettings {
            window: SpectrumWindow::Kaiser,
            kaiser_beta: 12.0,
            harmonics: 40,
            ..state.settings()
        };
        assert_eq!(state.adopt_settings(&kaiser), SettingsChange::Recomputed);
        assert!(state.has_data());
        assert_eq!(
            state.data.as_ref().map(|data| data.window),
            Some(WindowFunction::Kaiser(KaiserBeta::new(12.0).unwrap()))
        );
        assert!(state.spectrum_revision() > revision);

        let other_source = SpectrumSettings {
            source: Some("V(in)".to_owned()),
            ..state.settings()
        };
        assert_eq!(state.adopt_settings(&other_source), SettingsChange::Record);
        assert!(!state.has_data());
        assert_eq!(state.source(), Some("V(in)"));
    }
}
