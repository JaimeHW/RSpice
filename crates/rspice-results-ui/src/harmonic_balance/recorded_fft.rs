//! Recorded FFT evidence, incomplete-history captions, logarithmic ordinate and inspector rows.

use rspice_app_types::quantity::QuantityPresentationPolicy;
use rspice_results::{
    analysis_payload::AnalysisResultPayload, analysis_result::AnalysisResult,
    fft::spectrum::FftSpectrumEvidence,
};
use rspice_ui_kit::plot::{Axis, fmt_si, fmt_si_readout};

use crate::presentation::{frequency_readout, plain_readout};

/// Stems are one painted segment per coefficient, so they are drawn only for
/// a spectrum short enough that the cost is bounded.
pub(super) const MAX_STEMMED_COEFFICIENTS: usize = 512;

/// The recorded-FFT evidence of this result, if it carries any.
pub fn evidence<W>(analysis: &AnalysisResult<W>) -> Option<&FftSpectrumEvidence> {
    match analysis.result_payload.as_ref() {
        Some(AnalysisResultPayload::FftSpectrum { spectrum }) => Some(spectrum),
        _ => None,
    }
}

/// The sentence a short record earns, in place of an empty plot.
///
/// Never a generic "no data" hint: the engine treats an incomplete history as
/// a typed outcome, and the reader needs the two numbers that say why.
pub fn incomplete_history_sentence<W>(analysis: &AnalysisResult<W>) -> Option<String> {
    let spectrum = evidence(analysis)?;
    let rspice_results::fft::spectrum::FftSpectrumStatusEvidence::IncompleteHistory {
        available_start_s,
        available_stop_s,
    } = spectrum.status
    else {
        return None;
    };
    Some(format!(
        "FFT history is incomplete — the transient retained {} … {}; the transform needs \
         samples through {}",
        fmt_si(available_start_s, "s", 4),
        fmt_si(available_stop_s, "s", 4),
        fmt_si(spectrum.last_sample_time_s(), "s", 4)
    ))
}

/// A logarithmic-decade ordinate over the engine's own linear magnitudes.
///
/// The floor is ten decades below the largest coefficient, or the smallest
/// positive magnitude when the spectrum does not reach that far: a spectrum
/// with an exact zero bin has no logarithm, and clamping is what keeps the
/// axis honest rather than inventing a noise floor.
pub(super) fn decade_ordinate(
    smallest_positive: Option<f64>,
    largest: f64,
    unit: &str,
) -> Option<Axis> {
    if !largest.is_finite() || largest <= 0.0 {
        return None;
    }
    let floor = (largest * 1.0e-10).max(smallest_positive.unwrap_or(f64::MIN_POSITIVE));
    if !floor.is_finite() || floor <= 0.0 || floor >= largest {
        return None;
    }
    Some(Axis::log_decades(floor, largest * 1.2, unit).with_label("magnitude"))
}

/// The inspector rows a recorded spectrum states: what the engine
/// transformed, over which record, and the figures of merit it measured.
///
/// Sample counts, the card's own format and mode keywords, the window's
/// coherent gain and the fundamental's bin index are bookkeeping the reader
/// cannot act on. Sampling states itself only when it was interpolated, which
/// is the case that qualifies every number under it.
pub(super) fn inspector_rows(
    spectrum: &FftSpectrumEvidence,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) -> Vec<(&'static str, String, bool)> {
    let digits = significant_digits;
    let mut rows = vec![
        ("Output", spectrum.output.clone(), false),
        ("Window", spectrum.window.clone(), false),
        (
            "Record",
            format!(
                "{} … {}",
                fmt_si_readout(spectrum.start_time_s, "s", digits),
                fmt_si_readout(spectrum.stop_time_s, "s", digits)
            ),
            false,
        ),
        (
            "Resolution",
            frequency_readout(quantity_policy, spectrum.frequency_resolution_hz, digits),
            false,
        ),
    ];
    if !spectrum.accurate_sampling {
        rows.push(("Sampling", "interpolated".to_owned(), false));
    }
    rows.push((
        "Fundamental",
        frequency_readout(
            quantity_policy,
            spectrum.fundamental_bin as f64 * spectrum.frequency_resolution_hz,
            digits,
        ),
        true,
    ));
    // A level takes no SI prefix: half a decibel is `0.5 dB`, not `500 mdB`.
    if let Some(metrics) = &spectrum.metrics {
        rows.push(("THD", plain_readout(metrics.thd_db, "dB", digits), false));
        rows.push(("SNDR", plain_readout(metrics.sndr_db, "dB", digits), false));
        rows.push(("SNR", plain_readout(metrics.snr_db, "dB", digits), false));
        rows.push(("SFDR", plain_readout(metrics.sfdr_db, "dB", digits), false));
        rows.push((
            "ENOB",
            plain_readout(metrics.enob_bits, "bits", digits),
            true,
        ));
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_results::analysis_type::AnalysisType;
    type AnalysisResult =
        rspice_results::analysis_result::AnalysisResult<crate::waveform::WaveformData>;

    fn evidence_fixture(
        status: rspice_results::fft::spectrum::FftSpectrumStatusEvidence,
    ) -> FftSpectrumEvidence {
        FftSpectrumEvidence {
            status,
            output: "V(OUT)".to_owned(),
            physical_type: "voltage".to_owned(),
            start_time_s: 0.0,
            stop_time_s: 8.0e-3,
            sample_interval_s: 8.0e-3 / 256.0,
            point_count: 256,
            accurate_sampling: true,
            format: rspice_results::fft::FftSpectrumFormatEvidence::Unnormalized,
            mode: rspice_results::fft::spectrum::FftSpectrumModeEvidence::HspiceCompatible,
            window: "RECT".to_owned(),
            alpha: 3.0,
            coherent_gain: 1.0,
            frequency_resolution_hz: 125.0,
            fundamental_bin: 8,
            minimum_metric_bin: 1,
            maximum_metric_bin: 128,
            metrics: None,
        }
    }

    #[test]
    fn an_incomplete_fft_history_is_stated_not_plotted() {
        let spectrum = evidence_fixture(
            rspice_results::fft::spectrum::FftSpectrumStatusEvidence::IncompleteHistory {
                available_start_s: 0.0,
                available_stop_s: 2.0e-3,
            },
        );
        let analysis = AnalysisResult::new(1, AnalysisType::Fourier, "FFT", 0.0)
            .with_result_payload(AnalysisResultPayload::FftSpectrum {
                spectrum: spectrum.clone(),
            });
        // A successful result with no waveform: the record is the fact, and
        // an empty plot would be the only dishonest way to show it.
        assert!(analysis.success);
        assert!(analysis.waveforms.is_empty());
        let sentence = incomplete_history_sentence(&analysis).expect("a stated reason");
        assert!(
            sentence.starts_with("FFT history is incomplete"),
            "{sentence}"
        );
        assert!(sentence.contains("the transform needs samples through"));

        let complete = AnalysisResult::new(1, AnalysisType::Fourier, "FFT", 0.0)
            .with_result_payload(AnalysisResultPayload::FftSpectrum {
                spectrum: evidence_fixture(
                    rspice_results::fft::spectrum::FftSpectrumStatusEvidence::Complete,
                ),
            });
        assert!(incomplete_history_sentence(&complete).is_none());
        assert_eq!(evidence(&complete).map(|e| e.point_count), Some(256));
    }

    #[test]
    fn the_retained_spectrum_sheet_draws_a_recorded_fft_on_decades() {
        let spectrum =
            evidence_fixture(rspice_results::fft::spectrum::FftSpectrumStatusEvidence::Complete);
        let recorded = AnalysisResult::new(1, AnalysisType::Fourier, "FFT", 0.0)
            .with_result_payload(AnalysisResultPayload::FftSpectrum {
                spectrum: spectrum.clone(),
            });
        // The sheet asks the payload, so only a recorded FFT gets the
        // logarithmic ordinate and the transform's own inspector rows.
        assert!(evidence(&recorded).is_some());
        let axis = decade_ordinate(Some(1.0e-6), 1.0, "V").expect("a drawable ordinate");
        assert_eq!(axis.label.as_deref(), Some("magnitude"));
        assert!(axis.min > 0.0 && axis.min < 1.0);
        assert!(axis.ticks.len() > 1, "a decade axis has decade ticks");

        // Harmonic balance and `.FOUR` carry no such payload, so nothing here
        // reaches them and their rendering is exactly what it was.
        let harmonic_balance = AnalysisResult::new(2, AnalysisType::HarmonicBalance, "HB", 0.0);
        let fourier = AnalysisResult::new(3, AnalysisType::Fourier, "FOURIER", 0.0);
        assert!(evidence(&harmonic_balance).is_none());
        assert!(evidence(&fourier).is_none());
        assert!(incomplete_history_sentence(&harmonic_balance).is_none());
        assert!(incomplete_history_sentence(&fourier).is_none());
        // The retained fixture is short enough for coefficient stems.
        assert!(spectrum.bin_count() < MAX_STEMMED_COEFFICIENTS);
    }

    #[test]
    fn the_decade_ordinate_never_reaches_zero() {
        let axis = decade_ordinate(Some(1.0e-4), 1.0, "V").expect("a drawable ordinate");
        assert!(axis.min > 0.0);
        assert!(axis.max >= 1.0);
        // A spectrum of exact zeros has no logarithm, and no axis is the
        // honest answer rather than an invented floor.
        assert!(decade_ordinate(None, 0.0, "V").is_none());
    }

    #[test]
    fn the_inspector_reads_the_transform_the_engine_performed() {
        let rows = inspector_rows(
            &evidence_fixture(rspice_results::fft::spectrum::FftSpectrumStatusEvidence::Complete),
            &QuantityPresentationPolicy::default(),
            4,
        );
        let labels = rows.iter().map(|(label, _, _)| *label).collect::<Vec<_>>();
        assert_eq!(
            labels,
            ["Output", "Window", "Record", "Resolution", "Fundamental"]
        );
        assert!(
            rows.iter()
                .any(|(label, value, _)| *label == "Window" && value == "RECT")
        );
        // The fundamental is a frequency, not a bin index.
        assert!(
            rows.iter()
                .any(|(label, value, _)| *label == "Fundamental" && value == "1 kHz"),
            "{rows:?}"
        );
    }

    /// Frequencies read in the unit Frequency display chose.
    #[test]
    fn the_inspector_frequencies_follow_the_frequency_display() {
        let policy = QuantityPresentationPolicy {
            frequency_display: rspice_app_types::quantity::FrequencyDisplay::RadiansPerSecond,
            ..QuantityPresentationPolicy::default()
        };
        let rows = inspector_rows(
            &evidence_fixture(rspice_results::fft::spectrum::FftSpectrumStatusEvidence::Complete),
            &policy,
            4,
        );
        assert!(
            rows.iter()
                .any(|(label, value, _)| *label == "Fundamental" && value == "6.283 krad/s"),
            "{rows:?}"
        );
    }

    /// The figures of merit read at the reader's significant digits, and a
    /// level never takes an SI prefix.
    #[test]
    fn the_figures_of_merit_read_at_the_chosen_significant_digits() {
        let mut spectrum =
            evidence_fixture(rspice_results::fft::spectrum::FftSpectrumStatusEvidence::Complete);
        spectrum.metrics = Some(rspice_results::fft::spectrum::FftMetricsEvidence {
            fundamental_magnitude: 1.0,
            thd_ratio: 1.0e-3,
            thd_db: -60.123_456,
            sndr_db: 0.5,
            enob_bits: 9.876_543,
            snr_db: 61.234_567,
            sfdr_db: 70.0,
            sfdr_spur_bin: None,
            sfdr_spur_frequency_hz: None,
            largest_harmonics: Vec::new(),
        });
        let policy = QuantityPresentationPolicy::default();
        let value = |digits, wanted: &str| {
            inspector_rows(&spectrum, &policy, digits)
                .into_iter()
                .find(|(label, _, _)| *label == wanted)
                .map(|(_, value, _)| value)
                .expect("row present")
        };
        assert_eq!(value(4, "THD"), "-60.12 dB");
        assert_eq!(value(6, "THD"), "-60.1235 dB");
        assert_eq!(value(4, "SNDR"), "0.5 dB");
        assert_eq!(value(4, "SFDR"), "70 dB");
        assert_eq!(value(3, "ENOB"), "9.88 bits");
        assert_eq!(value(6, "Fundamental"), "1 kHz");
    }

    /// Exact sampling is the normal case, so it says nothing; interpolation
    /// qualifies every number under it, so it does.
    #[test]
    fn sampling_is_stated_only_when_it_was_interpolated() {
        let mut spectrum =
            evidence_fixture(rspice_results::fft::spectrum::FftSpectrumStatusEvidence::Complete);
        let policy = QuantityPresentationPolicy::default();
        assert!(
            !inspector_rows(&spectrum, &policy, 4)
                .iter()
                .any(|(label, _, _)| *label == "Sampling")
        );
        spectrum.accurate_sampling = false;
        assert!(
            inspector_rows(&spectrum, &policy, 4)
                .iter()
                .any(|(label, value, _)| *label == "Sampling" && value == "interpolated")
        );
    }
}
