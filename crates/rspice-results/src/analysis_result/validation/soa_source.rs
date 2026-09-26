//! Validate the exact waveform view against complete retained SOA observations.

use super::{RetainedWaveform, SoaSourceHistory};

pub(super) fn validate_report<W: AsRef<RetainedWaveform>>(
    source: &SoaSourceHistory,
    time: &[f64],
    waves: &[W],
) -> Result<(), String> {
    if source.time != time {
        return Err("SOA source history contradicts its retained observation axis".into());
    }
    let reporting_time = &waves
        .first()
        .ok_or("SOA reporting view has no traces")?
        .as_ref()
        .x;
    source.validate_report_columns(
        reporting_time,
        waves.iter().map(AsRef::as_ref).map(|wave| {
            (
                wave.name.as_str(),
                wave.x.as_slice(),
                wave.y.as_slice(),
                wave.unit.as_deref(),
                wave.complex.is_some(),
            )
        }),
    )
}
