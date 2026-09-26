//! Validate the application waveform view against complete retained SOA observations.

use super::{SoaSourceHistory, WaveformData};

pub(super) fn validate_report(
    source: &SoaSourceHistory,
    time: &[f64],
    waves: &[WaveformData],
) -> Result<(), String> {
    if source.time != time {
        return Err("SOA source history contradicts its retained observation axis".into());
    }
    let reporting_time = &waves.first().ok_or("SOA reporting view has no traces")?.x;
    source.validate_report_columns(
        reporting_time,
        waves.iter().map(|wave| {
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
