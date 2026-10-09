//! Admit derived delay gaps only when the retained complex transfer proves them.

use super::{AnalysisResult, AnalysisType, RetainedWaveform};
use rspice_core::Complex64;
use rspice_core::analysis::pxf::TransferPoint;

pub(super) fn verified_group_delay<W: AsRef<RetainedWaveform>>(
    result: &AnalysisResult<W>,
    delay: &RetainedWaveform,
) -> bool {
    if result.analysis_type != AnalysisType::Pxf
        || delay.name != "group_delay"
        || delay.unit.as_deref() != Some("s")
        || delay.complex.is_some()
    {
        return false;
    }
    let mut sources = result
        .waveforms
        .iter()
        .map(AsRef::as_ref)
        .filter_map(|wave| wave.complex.as_ref().map(|complex| (wave, complex)));
    let Some((source, complex)) = sources.next() else {
        return false;
    };
    if sources.next().is_some()
        || !complex.source_name.starts_with("H(sb")
        || source.x.len() < 2
        || source.x.len() - 1 != delay.x.len()
        || delay.x.len() != delay.y.len()
        || complex.real.len() != source.x.len()
        || complex.imag.len() != source.x.len()
        || source.x.iter().any(|f| !f.is_finite() || *f <= 0.0)
        || source.x.windows(2).any(|pair| pair[0] >= pair[1])
        || complex
            .real
            .iter()
            .chain(complex.imag.iter())
            .any(|v| !v.is_finite())
    {
        return false;
    }
    // Delay depends on swept offsets and complex transfer, independently of
    // the sideband labels and absolute converted output frequencies.
    let points = source
        .x
        .iter()
        .zip(complex.real.iter())
        .zip(complex.imag.iter())
        .map(|((&frequency, &real), &imaginary)| TransferPoint {
            freq_in: frequency,
            freq_out: frequency,
            transfer: Complex64::new(real, imaginary),
            sideband_in: 0,
            sideband_out: 0,
        });
    points
        .clone()
        .zip(points.skip(1))
        .zip(delay.x.iter().zip(delay.y.iter()))
        .all(|((left, right), (&frequency, &value))| {
            let expected = left.group_delay(&right);
            frequency == left.freq_in.midpoint(right.freq_in)
                && (value == expected || (value.is_nan() && expected.is_nan()))
        })
}
