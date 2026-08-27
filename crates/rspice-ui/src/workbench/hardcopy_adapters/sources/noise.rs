//! Which retained ordinary-noise spectrum a hardcopy is allowed to print.
//!
//! These predicates mirror the ones the ordinary-noise sheet reads in
//! `result_document::bode`, deliberately and by necessity: that module's
//! copies are `pub(super)` to `result_document`, so the printed page cannot
//! call them without widening a viewer's privacy. They are kept here as one
//! block, beside a note saying what they mirror, rather than scattered
//! through the resolver — an offering that disagrees with the sheet is the
//! defect this module exists to make visible.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RetainedNoiseReference {
    Input,
    Output,
}

pub(super) fn retained_noise_reference(name: &str) -> Option<RetainedNoiseReference> {
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

pub(super) fn retained_noise_contributor(name: &str) -> bool {
    let name = name.trim().to_ascii_lowercase();
    name.starts_with("noise(") && name.ends_with(')')
}

pub(super) fn retained_noise_waveform_is_renderable(waveform: &WaveformData) -> bool {
    if waveform.x.len() != waveform.y.len() || waveform.x.len() < 2 {
        return false;
    }
    if waveform
        .y
        .iter()
        .any(|density| !density.is_finite() || *density <= 0.0)
    {
        return false;
    }
    let mut previous = None;
    for frequency in waveform.x.iter().copied() {
        if !frequency.is_finite()
            || frequency <= 0.0
            || previous.is_some_and(|previous| frequency <= previous)
        {
            return false;
        }
        previous = Some(frequency);
    }
    true
}

/// Whether an analysis carries an ordinary-noise spectrum a page may show.
///
/// The success gate is the load-bearing clause and it was absent here. A
/// failed noise solve retains whatever vectors the engine emitted before it
/// gave up; the sheet refuses to draw them, and a printed page carries more
/// authority than the screen does, not less. Mirrors
/// `result_document::bode::ordinary_noise_spectrum_is_renderable`.
pub(super) fn ordinary_noise_spectrum_is_renderable(analysis: &AnalysisResult) -> bool {
    analysis.success
        && matches!(
            analysis.analysis_type,
            AnalysisType::Noise | AnalysisType::Hbnoise
        )
        && analysis.waveforms.iter().any(|waveform| {
            retained_noise_reference(&waveform.name).is_some()
                && retained_noise_waveform_is_renderable(waveform)
        })
}

/// Analyses whose selection states which noise result the reader means.
///
/// A selection outside this family — a transient carried over from another
/// viewer — says nothing about noise at all. Mirrors
/// `result_document::bode::is_noise_analysis`.
pub(super) const fn is_noise_analysis(analysis_type: AnalysisType) -> bool {
    matches!(
        analysis_type,
        AnalysisType::Noise | AnalysisType::Pnoise | AnalysisType::Hbnoise | AnalysisType::Qpnoise
    )
}

/// The analysis an ordinary-noise hardcopy binds to.
///
/// A selected noise analysis binds strictly. If it carries no renderable
/// ordinary spectrum — a phase-noise result, or one whose solve failed — the
/// page has nothing to print; quietly substituting a neighbouring result
/// would put a different analysis's contributors under the reader's
/// selection, on paper, under the selected analysis's name. The run-wide
/// fallback applies only when the selection expresses no noise intent for the
/// binding to honour. Mirrors
/// `result_document::bode::selected_noise_analysis_index`.
pub(super) fn selected_noise_analysis_index(
    globally_selected: Option<usize>,
    run: &SimulationRun,
) -> Option<usize> {
    if let Some(selected) = globally_selected
        && let Some(analysis) = run.analyses.get(selected)
        && is_noise_analysis(analysis.analysis_type)
    {
        return ordinary_noise_spectrum_is_renderable(analysis).then_some(selected);
    }
    run.analyses
        .iter()
        .position(ordinary_noise_spectrum_is_renderable)
}
