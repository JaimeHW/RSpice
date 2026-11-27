//! Validate the exact waveform view against complete retained SOA observations.

use super::{RetainedWaveform, SoaSourceHistory};

pub(super) fn validate_report<W: AsRef<RetainedWaveform>>(
    source: &SoaSourceHistory,
    time: &[f64],
    waves: &[W],
) -> Result<(), String> {
    if source.time != time {
        return Err("SOA source history contradicts its time axis".into());
    }
    // The reporting view is asked of the reader service whole: the history holds every
    // observation in memory, so a view of it fits there too.
    fn whole(
        values: &crate::waveform::WaveformValues,
    ) -> Result<crate::waveform::HeldColumn, String> {
        values.fetch_whole_or_wait().into_result()
    }
    let reporting_time = whole(
        &waves
            .first()
            .ok_or("SOA reporting view has no traces")?
            .as_ref()
            .x,
    )?;
    let columns = waves
        .iter()
        .map(AsRef::as_ref)
        .map(|wave| Ok((wave, whole(&wave.x)?, whole(&wave.y)?)))
        .collect::<Result<Vec<_>, String>>()?;
    source.validate_report_columns(
        &reporting_time,
        columns.iter().map(|(wave, x, y)| {
            (
                wave.name.as_str(),
                &x[..],
                &y[..],
                wave.unit.as_deref(),
                wave.complex.is_some(),
            )
        }),
    )
}
