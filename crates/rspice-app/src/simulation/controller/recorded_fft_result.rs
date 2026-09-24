//! Retaining one recorded `.FFT` spectrum as a result.
//!
//! Layer: controller, result retention. The coefficients become one complex
//! waveform in the Fourier family — which is the family the Results contract
//! already defines as bins, a window and a derivation receipt — and everything
//! a waveform cannot say becomes the result's payload.
//!
//! An incomplete record retains no waveform at all. It is still a successful
//! result: the engine's own contract treats a short history as a typed
//! outcome, and the honest picture of one is the sentence, never an empty
//! plot and never an invented bin.

use std::collections::HashMap;

use crate::simulation::results::RecordedFftSpectrum;
use crate::state::{AnalysisResult, AnalysisResultPayload, AnalysisType};

/// The unit the engine's own rule gives this spectrum's magnitudes.
fn magnitude_unit(spectrum: &RecordedFftSpectrum) -> String {
    use rspice_core::execution::SignalUnit;
    let unit = rspice_core::execution::transient_fft_output_unit(
        &spectrum.evidence.physical_type,
        rspice_simulation_contract::config::fft_format_to_core(spectrum.evidence.format),
    );
    match unit {
        Ok(SignalUnit::Volt) => "V".to_owned(),
        Ok(SignalUnit::Ampere) => "A".to_owned(),
        // Spelled the way the Fourier family already spells a normalized
        // spectrum, because it is the same quantity.
        Ok(SignalUnit::Dimensionless) => "ratio".to_owned(),
        // A braced parameter has a unit the deck never declared. Naming one
        // would assert a physical fact nobody stated.
        Ok(SignalUnit::Unspecified) | Err(_) => String::new(),
        Ok(other) => other.symbol(),
    }
}

/// The producer-side waveform map this spectrum retains, for the one complex
/// conversion every retained coefficient spectrum goes through.
///
/// Empty for an incomplete record: there is nothing to draw.
fn spectrum_waveforms(
    spectrum: &RecordedFftSpectrum,
) -> HashMap<String, crate::simulation::results::WaveformData> {
    if !spectrum.evidence.status.is_complete() {
        return HashMap::new();
    }
    let name = spectrum.evidence.output.clone();
    let mut waveform = crate::simulation::results::WaveformData::new_complex(
        name.clone(),
        spectrum.frequency.clone(),
        spectrum.real.clone(),
        spectrum.imaginary.clone(),
    );
    waveform.y_unit = magnitude_unit(spectrum);
    HashMap::from([(name, waveform)])
}

/// Build the retained result for one recorded spectrum.
///
/// `complex_waveforms` is the controller's own complex conversion, passed in
/// rather than duplicated: magnitude and phase are derived exactly once, by
/// the same code every other retained coefficient spectrum goes through.
pub(super) fn analysis_result(
    analysis_type: AnalysisType,
    label: &str,
    spectrum: &RecordedFftSpectrum,
    complex_waveforms: impl FnOnce(
        Vec<f64>,
        HashMap<String, crate::simulation::results::WaveformData>,
    ) -> Vec<crate::state::WaveformData>,
) -> AnalysisResult {
    if let Err(error) = spectrum.validate() {
        return AnalysisResult::failed(
            1,
            analysis_type,
            label.to_string(),
            format!("Invalid recorded FFT spectrum: {error}"),
        );
    }
    let mut result = AnalysisResult::new(1, analysis_type, label.to_string());
    if spectrum.evidence.status.is_complete() {
        let waveforms = complex_waveforms(spectrum.frequency.clone(), spectrum_waveforms(spectrum));
        result = result.with_waveforms(waveforms);
    }
    result.with_result_payload(AnalysisResultPayload::FftSpectrum {
        spectrum: spectrum.evidence.clone(),
    })
}

/// The Console line a short record earns at completion.
pub(in crate::simulation) fn incomplete_history_notice(
    spectrum: &RecordedFftSpectrum,
) -> Option<String> {
    let evidence = &spectrum.evidence;
    let crate::state::FftSpectrumStatusEvidence::IncompleteHistory {
        available_start_s,
        available_stop_s,
    } = evidence.status
    else {
        return None;
    };
    Some(format!(
        "FFT history is incomplete — the transient retained {available_start_s:.6e} s … \
         {available_stop_s:.6e} s; the transform needs samples through {:.6e} s",
        evidence.last_sample_time_s()
    ))
}
