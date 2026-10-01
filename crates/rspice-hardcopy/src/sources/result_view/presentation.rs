//! Validated print controls with the existing prepared-worker wire representation.

use super::*;
use rspice_results::fft::pipeline::{FftInputOptions, FftTimeWindow};
use rspice_results::fft::{InputFidelity, data::SpectrumNormalization, window::WindowFunction};
use rspice_results::histogram::HistogramDisplayMode;
use rspice_results::specification::SpecEntry;
use serde::{Deserialize, Serialize};

/// FFT controls read at capture time, without viewer caches or runtime state.
#[derive(Debug, Clone)]
pub struct QuickFftSettings {
    pub selected_source: Option<String>,
    pub normalization: SpectrumNormalization,
    pub window: WindowFunction,
    pub input_fidelity: InputFidelity,
    pub time_window_auto: bool,
    pub time_window_start: f64,
    pub time_window_end: f64,
    pub sample_count_auto: bool,
    pub sample_count: usize,
}

/// Histogram controls in source coordinates.
#[derive(Debug, Clone)]
pub struct QuickHistogramSettings {
    pub x: Option<(f64, f64)>,
    pub y: Option<(f64, f64)>,
    pub selected: usize,
    pub measurement: Option<String>,
    pub bin_count: usize,
    pub custom_range: bool,
    pub custom_min: f64,
    pub custom_max: f64,
    pub mode: HistogramDisplayMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum PreparedFftNormalization {
    Peak,
    Rms,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum PreparedFftWindow {
    Rectangular,
    Hanning,
    Hamming,
    Blackman,
    BlackmanHarris,
    FlatTop,
    Kaiser,
    Gaussian,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum PreparedHistogramMode {
    Count,
    Pdf,
    Cdf,
    Percent,
}

/// Only the persisted controls that affect quick-result semantic geometry.
/// FFT caches and every other viewer/runtime field are deliberately absent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResultsQuickViewPresentation {
    pub(super) viewer: ResultViewer,
    /// The reading the sheet was carrying: hidden traces, placed cursors,
    /// anchored markers. It travels with the controls because the page is
    /// resolved on the worker, and a page missing the reader's own annotation
    /// is not the page they reviewed.
    #[serde(default)]
    pub(super) overlay: RetainedQuickViewOverlays,
    pub(super) specs: Vec<SpecEntry>,
    pub(super) fft_selected_source: Option<String>,
    fft_normalization: PreparedFftNormalization,
    fft_window: PreparedFftWindow,
    pub(super) fft_input_fidelity: InputFidelity,
    pub(super) fft_time_window_auto: bool,
    pub(super) fft_time_window_start: f64,
    pub(super) fft_time_window_end: f64,
    pub(super) fft_sample_count_auto: bool,
    pub(super) fft_sample_count: usize,
    #[serde(default)]
    pub(super) histogram_x: Option<(f64, f64)>,
    #[serde(default)]
    pub(super) histogram_y: Option<(f64, f64)>,
    pub(super) histogram_selected: usize,
    #[serde(default)]
    pub(super) histogram_measurement: Option<String>,
    pub(super) histogram_bin_count: usize,
    pub(super) histogram_custom_range: bool,
    pub(super) histogram_custom_min: f64,
    pub(super) histogram_custom_max: f64,
    histogram_mode: PreparedHistogramMode,
}

impl ResultsQuickViewPresentation {
    /// Freeze and validate only the controls that affect a printed result.
    pub fn try_new(
        viewer: ResultViewer,
        overlay: RetainedQuickViewOverlays,
        specs: Vec<SpecEntry>,
        fft: QuickFftSettings,
        histogram: QuickHistogramSettings,
    ) -> Result<Self, HardcopySourceError> {
        validate_optional_label(
            "prepared FFT source",
            fft.selected_source.as_deref(),
            DISPLAY_NAME_LIMIT,
        )?;
        let captured = Self {
            viewer,
            overlay,
            specs,
            fft_selected_source: fft.selected_source,
            fft_normalization: match fft.normalization {
                SpectrumNormalization::Peak => PreparedFftNormalization::Peak,
                SpectrumNormalization::Rms => PreparedFftNormalization::Rms,
            },
            fft_window: match fft.window {
                WindowFunction::Rectangular => PreparedFftWindow::Rectangular,
                WindowFunction::Hanning => PreparedFftWindow::Hanning,
                WindowFunction::Hamming => PreparedFftWindow::Hamming,
                WindowFunction::Blackman => PreparedFftWindow::Blackman,
                WindowFunction::BlackmanHarris => PreparedFftWindow::BlackmanHarris,
                WindowFunction::FlatTop => PreparedFftWindow::FlatTop,
                WindowFunction::Kaiser => PreparedFftWindow::Kaiser,
                WindowFunction::Gaussian => PreparedFftWindow::Gaussian,
            },
            fft_input_fidelity: fft.input_fidelity,
            fft_time_window_auto: fft.time_window_auto,
            fft_time_window_start: fft.time_window_start,
            fft_time_window_end: fft.time_window_end,
            fft_sample_count_auto: fft.sample_count_auto,
            fft_sample_count: fft.sample_count,
            histogram_x: histogram.x,
            histogram_y: histogram.y,
            histogram_selected: histogram.selected,
            histogram_measurement: histogram.measurement,
            histogram_bin_count: histogram.bin_count,
            histogram_custom_range: histogram.custom_range,
            histogram_custom_min: histogram.custom_min,
            histogram_custom_max: histogram.custom_max,
            histogram_mode: match histogram.mode {
                HistogramDisplayMode::Count => PreparedHistogramMode::Count,
                HistogramDisplayMode::Pdf => PreparedHistogramMode::Pdf,
                HistogramDisplayMode::Cdf => PreparedHistogramMode::Cdf,
                HistogramDisplayMode::Percent => PreparedHistogramMode::Percent,
            },
        };
        captured.validate()?;
        Ok(captured)
    }

    pub fn validate(&self) -> Result<(), HardcopySourceError> {
        self.overlay.validate()?;
        validate_optional_label(
            "prepared histogram measurement",
            self.histogram_measurement.as_deref(),
            DISPLAY_NAME_LIMIT,
        )?;
        for (low, high) in [self.histogram_x, self.histogram_y].into_iter().flatten() {
            if !low.is_finite() || !high.is_finite() || low >= high || !(high - low).is_finite() {
                return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                    "prepared histogram viewport is not a finite interval".to_owned(),
                ));
            }
        }
        super::super::result_documents::validate_result_specifications(&self.specs)?;
        validate_optional_label(
            "prepared FFT source",
            self.fft_selected_source.as_deref(),
            DISPLAY_NAME_LIMIT,
        )?;
        for (field, value) in [
            ("FFT time-window start", self.fft_time_window_start),
            ("FFT time-window end", self.fft_time_window_end),
            ("histogram custom minimum", self.histogram_custom_min),
            ("histogram custom maximum", self.histogram_custom_max),
        ] {
            if !value.is_finite() {
                return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                    "{field} is not finite"
                )));
            }
        }
        if self.fft_sample_count == 0 {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                "FFT sample count is zero".to_owned(),
            ));
        }
        if self.histogram_bin_count == 0 {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                "histogram bin count is zero".to_owned(),
            ));
        }
        if !self.fft_time_window_auto && self.fft_time_window_start >= self.fft_time_window_end {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                "manual FFT time window is empty or reversed".to_owned(),
            ));
        }
        if self.histogram_custom_range && self.histogram_custom_min >= self.histogram_custom_max {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                "custom histogram range is empty or reversed".to_owned(),
            ));
        }
        Ok(())
    }

    pub fn viewer(&self) -> ResultViewer {
        self.viewer
    }
    pub fn specs(&self) -> &[SpecEntry] {
        &self.specs
    }

    pub(super) fn fft_normalization(&self) -> SpectrumNormalization {
        match self.fft_normalization {
            PreparedFftNormalization::Peak => SpectrumNormalization::Peak,
            PreparedFftNormalization::Rms => SpectrumNormalization::Rms,
        }
    }
    pub(super) fn fft_window(&self) -> WindowFunction {
        match self.fft_window {
            PreparedFftWindow::Rectangular => WindowFunction::Rectangular,
            PreparedFftWindow::Hanning => WindowFunction::Hanning,
            PreparedFftWindow::Hamming => WindowFunction::Hamming,
            PreparedFftWindow::Blackman => WindowFunction::Blackman,
            PreparedFftWindow::BlackmanHarris => WindowFunction::BlackmanHarris,
            PreparedFftWindow::FlatTop => WindowFunction::FlatTop,
            PreparedFftWindow::Kaiser => WindowFunction::Kaiser,
            PreparedFftWindow::Gaussian => WindowFunction::Gaussian,
        }
    }
    pub(super) fn histogram_mode(&self) -> HistogramDisplayMode {
        match self.histogram_mode {
            PreparedHistogramMode::Count => HistogramDisplayMode::Count,
            PreparedHistogramMode::Pdf => HistogramDisplayMode::Pdf,
            PreparedHistogramMode::Cdf => HistogramDisplayMode::Cdf,
            PreparedHistogramMode::Percent => HistogramDisplayMode::Percent,
        }
    }

    pub(super) fn fft_input_options(&self) -> FftInputOptions {
        let policy = self.fft_input_fidelity.input_policy();
        let time_window = if self.fft_time_window_auto {
            None
        } else {
            Some(FftTimeWindow::new(
                self.fft_time_window_start,
                self.fft_time_window_end,
            ))
        };
        let target_samples = if self.fft_sample_count_auto {
            None
        } else {
            Some(self.fft_sample_count)
        };
        FftInputOptions::with_policy(policy)
            .with_time_window(time_window)
            .with_target_samples(target_samples)
    }
}

fn validate_optional_label(
    field: &'static str,
    value: Option<&str>,
    maximum_bytes: usize,
) -> Result<(), HardcopySourceError> {
    if let Some(value) = value {
        validate_label(field, value, maximum_bytes)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn histogram_worker_controls_round_trip_and_validate_the_viewport() {
        for mode in HistogramDisplayMode::ALL {
            let prepared = ResultsQuickViewPresentation::try_new(
                ResultViewer::Hist,
                RetainedQuickViewOverlays::default(),
                Vec::new(),
                QuickFftSettings {
                    selected_source: None,
                    normalization: SpectrumNormalization::Peak,
                    window: WindowFunction::Hanning,
                    input_fidelity: InputFidelity::Reference,
                    time_window_auto: true,
                    time_window_start: 0.0,
                    time_window_end: 1.0,
                    sample_count_auto: true,
                    sample_count: 4096,
                },
                QuickHistogramSettings {
                    x: Some((1e-15, 2e-15)),
                    y: Some((0.0, 100.0)),
                    selected: 0,
                    measurement: Some("gain".to_owned()),
                    bin_count: 20,
                    custom_range: false,
                    custom_min: 0.0,
                    custom_max: 1.0,
                    mode,
                },
            )
            .unwrap();
            let bytes = serde_json::to_vec(&prepared).unwrap();
            let restored = serde_json::from_slice::<ResultsQuickViewPresentation>(&bytes).unwrap();
            restored.validate().unwrap();
            assert_eq!(restored.histogram_mode(), mode);
            assert_eq!(restored.histogram_measurement.as_deref(), Some("gain"));
            assert_eq!(restored.histogram_x, Some((1e-15, 2e-15)));
            assert_eq!(restored.histogram_y, Some((0.0, 100.0)));

            let mut legacy = serde_json::to_value(&prepared).unwrap();
            legacy
                .as_object_mut()
                .unwrap()
                .remove("histogram_measurement");
            legacy.as_object_mut().unwrap().remove("histogram_x");
            legacy.as_object_mut().unwrap().remove("histogram_y");
            let restored = serde_json::from_value::<ResultsQuickViewPresentation>(legacy).unwrap();
            restored.validate().unwrap();
            assert_eq!(restored.histogram_measurement, None);
            assert_eq!(restored.histogram_x, None);
            assert_eq!(restored.histogram_y, None);

            for range in [
                (2.0, 1.0),
                (1.0, 1.0),
                (0.0, f64::INFINITY),
                (-f64::MAX, f64::MAX),
            ] {
                let mut invalid = prepared.clone();
                invalid.histogram_x = Some(range);
                assert!(invalid.validate().is_err());
            }
        }
    }
}
