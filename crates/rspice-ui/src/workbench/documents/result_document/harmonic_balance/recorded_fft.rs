//! What the retained coefficient-spectrum sheet does differently for a
//! recorded `.FFT`.
//!
//! Layer: Results document, viewer. Only three things differ, and each is
//! here because the retained payload says something a harmonic-balance result
//! does not: the ordinate is drawn in logarithmic decades over the engine's
//! own linear magnitudes (the same picture as dB, with no derived array and
//! no second cache), a short record states its sentence instead of showing an
//! empty plot, and the inspector reads the transform the engine actually
//! performed rather than "Not retained".
//!
//! Nothing here computes a transform, a window or a dB array. HB, Fourier and
//! PSS-spectrum rendering is untouched: every entry point is gated on the
//! recorded-FFT payload.

use crate::state::{AnalysisResult, AnalysisResultPayload, FftSpectrumEvidence};
use crate::ui::plot::{Axis, fmt_si};

/// Stems are one painted segment per coefficient, so they are drawn only for
/// a spectrum short enough that the cost is bounded.
pub(super) const MAX_STEMMED_COEFFICIENTS: usize = 512;

/// The recorded-FFT evidence of this result, if it carries any.
pub(super) fn evidence(analysis: &AnalysisResult) -> Option<&FftSpectrumEvidence> {
    match analysis.result_payload.as_ref() {
        Some(AnalysisResultPayload::FftSpectrum { spectrum }) => Some(spectrum),
        _ => None,
    }
}

/// The sentence a short record earns, in place of an empty plot.
///
/// Never a generic "no data" hint: the engine treats an incomplete history as
/// a typed outcome, and the reader needs the two numbers that say why.
pub(super) fn incomplete_history_sentence(analysis: &AnalysisResult) -> Option<String> {
    let spectrum = evidence(analysis)?;
    let crate::state::FftSpectrumStatusEvidence::IncompleteHistory {
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

/// The inspector rows a recorded spectrum states, in place of the
/// harmonic-balance sheet's "Not retained" rows.
pub(super) fn inspector_rows(spectrum: &FftSpectrumEvidence) -> Vec<(&'static str, String, bool)> {
    let mut rows = vec![
        ("Output", spectrum.output.clone(), false),
        ("Window", spectrum.window.clone(), false),
        ("Samples", spectrum.point_count.to_string(), true),
        (
            "Record",
            format!(
                "{} … {}",
                fmt_si(spectrum.start_time_s, "s", 4),
                fmt_si(spectrum.stop_time_s, "s", 4)
            ),
            false,
        ),
        (
            "Resolution",
            fmt_si(spectrum.frequency_resolution_hz, "Hz", 4),
            false,
        ),
        ("Format", spectrum.format.keyword().to_owned(), false),
        ("Mode", spectrum.mode.label().to_owned(), false),
        (
            "Sampling",
            if spectrum.accurate_sampling {
                "exact sample times".to_owned()
            } else {
                "interpolated".to_owned()
            },
            false,
        ),
        (
            "Coherent gain",
            fmt_si(spectrum.coherent_gain, "", 6),
            false,
        ),
        (
            "Fundamental",
            format!(
                "bin {} · {}",
                spectrum.fundamental_bin,
                fmt_si(
                    spectrum.fundamental_bin as f64 * spectrum.frequency_resolution_hz,
                    "Hz",
                    4
                )
            ),
            true,
        ),
    ];
    if let Some(metrics) = &spectrum.metrics {
        rows.push(("THD", fmt_si(metrics.thd_db, "dB", 4), false));
        rows.push(("SNDR", fmt_si(metrics.sndr_db, "dB", 4), false));
        rows.push(("SNR", fmt_si(metrics.snr_db, "dB", 4), false));
        rows.push(("SFDR", fmt_si(metrics.sfdr_db, "dB", 4), false));
        rows.push(("ENOB", format!("{:.2} bits", metrics.enob_bits), true));
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::AnalysisType;

    fn evidence_fixture(status: crate::state::FftSpectrumStatusEvidence) -> FftSpectrumEvidence {
        FftSpectrumEvidence {
            status,
            output: "V(OUT)".to_owned(),
            physical_type: "voltage".to_owned(),
            start_time_s: 0.0,
            stop_time_s: 8.0e-3,
            sample_interval_s: 8.0e-3 / 256.0,
            point_count: 256,
            accurate_sampling: true,
            format: crate::state::FftSpectrumFormatEvidence::Unnormalized,
            mode: crate::state::FftSpectrumModeEvidence::HspiceCompatible,
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
        let spectrum =
            evidence_fixture(crate::state::FftSpectrumStatusEvidence::IncompleteHistory {
                available_start_s: 0.0,
                available_stop_s: 2.0e-3,
            });
        let analysis = AnalysisResult::new(1, AnalysisType::Fourier, "FFT").with_result_payload(
            AnalysisResultPayload::FftSpectrum {
                spectrum: spectrum.clone(),
            },
        );
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

        let complete = AnalysisResult::new(1, AnalysisType::Fourier, "FFT").with_result_payload(
            AnalysisResultPayload::FftSpectrum {
                spectrum: evidence_fixture(crate::state::FftSpectrumStatusEvidence::Complete),
            },
        );
        assert!(incomplete_history_sentence(&complete).is_none());
        assert_eq!(evidence(&complete).map(|e| e.point_count), Some(256));
    }

    #[test]
    fn the_retained_spectrum_sheet_draws_a_recorded_fft_on_decades() {
        let spectrum = evidence_fixture(crate::state::FftSpectrumStatusEvidence::Complete);
        let recorded = AnalysisResult::new(1, AnalysisType::Fourier, "FFT").with_result_payload(
            AnalysisResultPayload::FftSpectrum {
                spectrum: spectrum.clone(),
            },
        );
        // The sheet asks the payload, so only a recorded FFT gets the
        // logarithmic ordinate and the transform's own inspector rows.
        assert!(evidence(&recorded).is_some());
        let axis = decade_ordinate(Some(1.0e-6), 1.0, "V").expect("a drawable ordinate");
        assert_eq!(axis.label.as_deref(), Some("magnitude"));
        assert!(axis.min > 0.0 && axis.min < 1.0);
        assert!(axis.ticks.len() > 1, "a decade axis has decade ticks");

        // Harmonic balance and `.FOUR` carry no such payload, so nothing here
        // reaches them and their rendering is exactly what it was.
        let harmonic_balance = AnalysisResult::new(2, AnalysisType::HarmonicBalance, "HB");
        let fourier = AnalysisResult::new(3, AnalysisType::Fourier, "FOURIER");
        assert!(evidence(&harmonic_balance).is_none());
        assert!(evidence(&fourier).is_none());
        assert!(incomplete_history_sentence(&harmonic_balance).is_none());
        assert!(incomplete_history_sentence(&fourier).is_none());
        // And a long spectrum draws no stems: one painted segment per
        // coefficient is a per-frame cost, and half a million is not a plot.
        assert!(spectrum.bin_count() < MAX_STEMMED_COEFFICIENTS);
        assert!(1_048_576 / 2 + 1 > MAX_STEMMED_COEFFICIENTS);
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
        let rows = inspector_rows(&evidence_fixture(
            crate::state::FftSpectrumStatusEvidence::Complete,
        ));
        let labels = rows.iter().map(|(label, _, _)| *label).collect::<Vec<_>>();
        assert_eq!(
            labels,
            [
                "Output",
                "Window",
                "Samples",
                "Record",
                "Resolution",
                "Format",
                "Mode",
                "Sampling",
                "Coherent gain",
                "Fundamental",
            ]
        );
        assert!(
            rows.iter()
                .any(|(label, value, _)| *label == "Window" && value == "RECT")
        );
        assert!(
            rows.iter()
                .any(|(label, value, _)| *label == "Sampling" && value == "exact sample times")
        );
    }
}
