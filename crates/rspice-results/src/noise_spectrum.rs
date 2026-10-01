//! Ordinary-noise trace identity and exact retained-spectrum validation.

use crate::waveform::RetainedWaveform;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetainedNoiseReference {
    Input,
    Output,
}

pub fn retained_noise_reference(name: &str) -> Option<RetainedNoiseReference> {
    let name = name
        .trim()
        .to_ascii_lowercase()
        .replace([' ', '-', '.'], "");
    if matches!(
        name.as_str(),
        "inoise"
            | "inoise_spectrum"
            | "inoisespectrum"
            | "v(inoise)"
            | "v(inoise_spectrum)"
            | "v(inoisespectrum)"
    ) {
        Some(RetainedNoiseReference::Input)
    } else if matches!(
        name.as_str(),
        "onoise"
            | "onoise_spectrum"
            | "onoisespectrum"
            | "v(onoise)"
            | "v(onoise_spectrum)"
            | "v(onoisespectrum)"
    ) {
        Some(RetainedNoiseReference::Output)
    } else {
        None
    }
}

pub fn retained_noise_contributor(name: &str) -> bool {
    let name = name.trim().to_ascii_lowercase();
    name.starts_with("noise(") && name.ends_with(')')
}

/// Which retained traces one analysis' ordinary-noise spectrum is made of.
///
/// Resolving it walks every sample of every candidate density — each value
/// finite and nonnegative, each frequency positive and strictly ascending — and
/// compares each contributor's frequency axis against the anchor's. That is
/// the whole structural half of the noise sheet, and the tab strip, the
/// spectrum card and the contributor table each asked for it independently,
/// on every frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoiseSpectrumShape {
    /// The density the sheet anchors its frequency axis on: the
    /// input-referred spectrum when one is retained, else the
    /// output-referred one.
    pub anchor: usize,
    /// Renderable densities sharing that axis. An input-referred spectrum
    /// stands alone, so it counts one.
    pub trace_count: usize,
}

/// Resolve input/output reference and matching contributors without copying samples.
/// `note_scan` observes each admitted sample scan for the caller's work accounting.
pub fn resolve_noise_spectrum_shape<W: AsRef<RetainedWaveform>>(
    waveforms: &[W],
    mut note_scan: impl FnMut(),
) -> Option<NoiseSpectrumShape> {
    let input_referred = waveforms.iter().position(|waveform| {
        let waveform = waveform.as_ref();
        retained_noise_reference(&waveform.name) == Some(RetainedNoiseReference::Input)
            && noise_waveform_is_renderable(waveform, &mut note_scan)
    });
    let anchor = match input_referred {
        Some(index) => index,
        None => waveforms.iter().position(|waveform| {
            let waveform = waveform.as_ref();
            retained_noise_reference(&waveform.name) == Some(RetainedNoiseReference::Output)
                && noise_waveform_is_renderable(waveform, &mut note_scan)
        })?,
    };
    let frequency = &waveforms[anchor].as_ref().x;
    let trace_count = if input_referred.is_some() {
        1
    } else {
        waveforms
            .iter()
            .filter(|waveform| {
                let waveform = waveform.as_ref();
                retained_noise_reference(&waveform.name) != Some(RetainedNoiseReference::Input)
                    && (retained_noise_reference(&waveform.name)
                        == Some(RetainedNoiseReference::Output)
                        || retained_noise_contributor(&waveform.name))
                    && noise_waveform_is_renderable(waveform, &mut note_scan)
                    && waveform.x.as_slice() == frequency.as_slice()
            })
            .count()
    };
    Some(NoiseSpectrumShape {
        anchor,
        trace_count,
    })
}

fn noise_waveform_is_renderable(waveform: &RetainedWaveform, note_scan: &mut impl FnMut()) -> bool {
    if waveform.x.len() != waveform.y.len() || waveform.x.is_empty() {
        return false;
    }
    note_scan();
    if waveform
        .y
        .iter()
        .any(|value| !value.is_finite() || *value < 0.0)
    {
        return false;
    }
    let mut previous = None;
    let mut positive_count = 0_usize;
    for frequency in waveform.x.iter().copied() {
        if !frequency.is_finite() || frequency <= 0.0 {
            return false;
        }
        if previous.is_some_and(|previous| frequency <= previous) {
            return false;
        }
        previous = Some(frequency);
        positive_count += 1;
    }
    positive_count >= 1
}

/// Whether a retained ordinary-noise density has a finite positive ascending axis.
pub fn retained_noise_waveform_is_renderable<W: AsRef<RetainedWaveform>>(waveform: &W) -> bool {
    noise_waveform_is_renderable(waveform.as_ref(), &mut || {})
}

/// The ordinary-noise result kinds admitted by both display and publication.
pub fn is_ordinary_noise_result(
    success: bool,
    analysis_type: crate::analysis_type::AnalysisType,
) -> bool {
    use crate::analysis_type::AnalysisType;
    success && matches!(analysis_type, AnalysisType::Noise | AnalysisType::Hbnoise)
}

/// Resolve an ordinary-noise offering from retained data with optional scan accounting.
pub fn ordinary_noise_spectrum_is_renderable<W: AsRef<RetainedWaveform>>(
    success: bool,
    analysis_type: crate::analysis_type::AnalysisType,
    waveforms: &[W],
    note_scan: impl FnMut(),
) -> bool {
    is_ordinary_noise_result(success, analysis_type)
        && resolve_noise_spectrum_shape(waveforms, note_scan).is_some()
}

/// QPNOISE shape classification; consumers still validate its retained evidence.
#[cfg(feature = "engine-evidence")]
pub fn qpnoise_is_renderable<W>(analysis: &crate::analysis_result::AnalysisResult<W>) -> bool {
    analysis.analysis_type == crate::analysis_type::AnalysisType::Qpnoise
        && matches!(
            analysis.result_payload,
            Some(crate::analysis_payload::AnalysisResultPayload::Qpnoise { .. })
        )
        && !analysis.waveforms.is_empty()
}
