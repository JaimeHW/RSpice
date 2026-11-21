//! Structural evidence for exact retained coefficient spectra.

use crate::{analysis_type::AnalysisType, waveform::RetainedWaveform};

pub mod lines;
pub mod settings;

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
        .samples(..)
        .zip(waveform.y.samples(..))
        .all(|(frequency, magnitude)| {
            frequency.is_finite() && frequency >= 0.0 && magnitude.is_finite()
        })
        && waveform.x.ascends()
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

/// What a distortion analysis' product ratio measures, by the product the
/// engine names: its harmonic or intermodulation order and the frequency
/// it falls at.
fn distortion_product(product: &str) -> Option<&'static str> {
    Some(match product {
        "2f1" => "HD2",
        "3f1" => "HD3",
        "2f2" => "HD2 of f2",
        "f1+f2" => "IM2 f1+f2",
        "f1-f2" => "IM2 f1\u{2212}f2",
        "2f1-f2" => "IM3 2f1\u{2212}f2",
        "2f2-f1" => "IM3 2f2\u{2212}f1",
        _ => return None,
    })
}

/// The name a curve of a distortion (`.disto`) analysis is shown by: a
/// product ratio the engine names `V(out) 2f1/F1` reads `HD2 V(out)`, and
/// one it names `V(out) 2f1-f2/F1` reads `IM3 2f1−f2 V(out)`, inside the
/// magnitude bars or the phase call the curve is drawn as. Any other curve,
/// and any curve of another analysis, keeps its name.
#[must_use]
pub fn distortion_curve_name(analysis_type: AnalysisType, name: &str) -> std::borrow::Cow<'_, str> {
    use std::borrow::Cow;
    if analysis_type != AnalysisType::Disto {
        return Cow::Borrowed(name);
    }
    let (open, inner, close) = if let Some(inner) = name
        .strip_prefix('|')
        .and_then(|rest| rest.strip_suffix('|'))
    {
        ("|", inner, "|")
    } else if let Some(inner) = name
        .strip_prefix("phase(")
        .and_then(|rest| rest.strip_suffix(')'))
    {
        ("phase(", inner, ")")
    } else {
        ("", name, "")
    };
    let renamed = inner
        .strip_suffix("/F1")
        .and_then(|ratio| ratio.rsplit_once(' '))
        .and_then(|(signal, product)| {
            distortion_product(product).map(|label| format!("{open}{label} {signal}{close}"))
        });
    renamed.map_or(Cow::Borrowed(name), Cow::Owned)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each product ratio reads as the distortion it measures; the
    /// fundamentals, THD and other analyses' curves keep their names.
    #[test]
    fn distortion_curves_are_named_for_the_product_they_measure() {
        let name = |name| distortion_curve_name(AnalysisType::Disto, name).into_owned();
        assert_eq!(name("|V(out) 2f1/F1|"), "|HD2 V(out)|");
        assert_eq!(name("phase(V(out) 3f1/F1)"), "phase(HD3 V(out))");
        assert_eq!(name("V(out) 2f1-f2/F1"), "IM3 2f1\u{2212}f2 V(out)");
        assert_eq!(name("|I(V1) f1+f2/F1|"), "|IM2 f1+f2 I(V1)|");
        assert_eq!(name("|V(a b) 2f2-f1/F1|"), "|IM3 2f2\u{2212}f1 V(a b)|");
        assert_eq!(name("|V(out) F1|"), "|V(out) F1|");
        assert_eq!(name("V(out) THD"), "V(out) THD");
        assert_eq!(
            distortion_curve_name(AnalysisType::Ac, "|V(out) 2f1/F1|"),
            "|V(out) 2f1/F1|"
        );
    }
}
