//! Structural evidence for exact retained coefficient spectra.

use crate::{analysis_type::AnalysisType, waveform::RetainedWaveform};

pub fn spectrum_trace_is_renderable(
    waveform: &RetainedWaveform,
    mut note_scan: impl FnMut(),
) -> bool {
    if waveform.complex.is_none() || waveform.x.len() != waveform.y.len() || waveform.x.is_empty() {
        return false;
    }
    note_scan();
    waveform
        .x
        .iter()
        .zip(waveform.y.iter())
        .all(|(&frequency, &magnitude)| {
            frequency.is_finite() && frequency >= 0.0 && magnitude.is_finite()
        })
        && waveform.x.windows(2).all(|window| window[0] <= window[1])
}

pub fn analysis_is_renderable<W: AsRef<RetainedWaveform>>(
    success: bool,
    analysis_type: AnalysisType,
    waveforms: &[W],
    mut note_scan: impl FnMut(),
) -> bool {
    success
        && matches!(
            analysis_type,
            AnalysisType::HarmonicBalance | AnalysisType::Fourier | AnalysisType::Qpss
        )
        && waveforms
            .iter()
            .any(|waveform| spectrum_trace_is_renderable(waveform.as_ref(), &mut note_scan))
}
