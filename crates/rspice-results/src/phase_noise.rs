//! Exact retained phase-noise evidence, spectra and spot values.

use crate::analysis_type::AnalysisType;
use crate::family_metadata::{AnalysisResultFamilyMetadata, PeriodicNoiseOutputQuantity};
use crate::waveform::RetainedWaveform;

pub fn retained_phase_noise_carrier(
    analysis_type: AnalysisType,
    metadata: Option<&AnalysisResultFamilyMetadata>,
) -> Option<f64> {
    let metadata = metadata?;
    if metadata.validate_for(analysis_type).is_err() {
        return None;
    }
    let AnalysisResultFamilyMetadata::PeriodicNoise {
        output_quantity: PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
        carrier_frequency_hz: Some(carrier_frequency_hz),
    } = metadata
    else {
        return None;
    };
    Some(*carrier_frequency_hz)
}

/// A waveform name is phase-noise evidence only when it says so directly.
///
/// `onoise` and `inoise` deliberately do *not* qualify: the retained result
/// adapter uses those names for ordinary periodic noise too, and assigning a
/// dBc/Hz meaning to them would invent a carrier normalization.
fn is_explicit_phase_noise_name(name: &str) -> bool {
    let compact = name
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .collect::<String>()
        .to_ascii_lowercase();
    compact.contains("phasenoise") || compact == "lf"
}

pub fn phase_noise_waveform_is_renderable(
    waveform: &RetainedWaveform,
    mut note_scan: impl FnMut(),
) -> bool {
    if !is_explicit_phase_noise_name(&waveform.name)
        || waveform.x.len() != waveform.y.len()
        || waveform.x.len() < 2
    {
        return false;
    }
    note_scan();

    waveform
        .x
        .iter()
        .zip(waveform.y.iter())
        .try_fold(None, |previous, (&offset, &level)| {
            (offset.is_finite()
                && offset > 0.0
                && level.is_finite()
                && previous.is_none_or(|previous| offset > previous))
            .then_some(Some(offset))
        })
        .is_some()
}

/// `true` if this analysis contains a usable, explicitly-labelled retained
/// phase-noise trace.  Callers use this for viewer availability; it never
/// widens PNOISE into a phase-noise assertion by analysis type alone.
pub fn phase_noise_is_renderable<W: AsRef<RetainedWaveform>>(
    success: bool,
    analysis_type: AnalysisType,
    metadata: Option<&AnalysisResultFamilyMetadata>,
    waveforms: &[W],
    mut note_scan: impl FnMut(),
) -> bool {
    success
        && matches!(analysis_type, AnalysisType::Pnoise | AnalysisType::Qpnoise)
        && retained_phase_noise_carrier(analysis_type, metadata).is_some()
        && waveforms
            .iter()
            .any(|waveform| phase_noise_waveform_is_renderable(waveform.as_ref(), &mut note_scan))
}

/// Return the exact retained value at the requested offset.  This intentionally
/// refuses interpolation: a displayed 1 MHz spot value is an assertion that
/// the analysis retained that sample, not an invented crossing.
pub fn exact_retained_value_at(offsets: &[f64], levels: &[f64], target_offset: f64) -> Option<f64> {
    offsets
        .iter()
        .position(|offset| *offset == target_offset)
        .and_then(|index| levels.get(index).copied())
        .filter(|level| level.is_finite())
}

/// A unique, successful, finite nonnegative retained phase/jitter measurement.
#[cfg(feature = "engine-evidence")]
pub fn retained_measurement<W>(
    analysis: &crate::analysis_result::AnalysisResult<W>,
    name: &str,
) -> Option<f64> {
    let mut matching = analysis
        .measurements
        .iter()
        .filter(|measurement| measurement.name == name);
    let measurement = matching.next()?;
    let value = measurement.value?;
    (matching.next().is_none()
        && measurement.passed
        && measurement.error.is_none()
        && value.is_finite()
        && value >= 0.0)
        .then_some(value)
}

/// Successful finite nonnegative device shares, in retained measurement order.
#[cfg(feature = "engine-evidence")]
pub fn retained_device_noise_shares(
    measurements: &[rspice_core::MeasureResult],
) -> impl Iterator<Item = (&str, f64)> {
    measurements.iter().filter_map(|measurement| {
        let name = measurement
            .name
            .strip_prefix("noise_share_percent(")?
            .strip_suffix(')')?;
        let value = measurement.value?;
        (measurement.passed && measurement.error.is_none() && value.is_finite() && value >= 0.0)
            .then_some((name, value))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn waveform(name: &str) -> RetainedWaveform {
        RetainedWaveform::new(name, vec![1.0, 1.0e3, 1.0e6], vec![-72.0, -103.0, -132.0])
    }

    #[test]
    fn phase_noise_rejects_invalid_log_axis_and_spot_requires_retained_sample() {
        let invalid = RetainedWaveform::new(
            "phase_noise",
            vec![1.0, 0.0, 1.0e6],
            vec![-80.0, -100.0, -130.0],
        );
        assert!(!phase_noise_waveform_is_renderable(&invalid, || {}));

        let trace = waveform("phase_noise");
        assert_eq!(
            exact_retained_value_at(&trace.x, &trace.y, 1.0e6),
            Some(-132.0)
        );
        assert_eq!(exact_retained_value_at(&trace.x, &trace.y, 10.0), None);
    }
}
