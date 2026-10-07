//! STB keeps complete raw samples and displays each defined Bode segment separately.
use super::AnalysisResultPayload;
use crate::waveform::RetainedWaveform;
use rspice_core::analysis::stb::{BodePoint, StbResult};
use std::sync::Arc;

impl AnalysisResultPayload {
    pub fn stb_waveforms(response: &StbResult) -> Result<Vec<RetainedWaveform>, String> {
        response
            .validate_with_abort(
                &rspice_core::ResourceLimits::default(),
                &rspice_core::NoAbort,
            )
            .map_err(|e| e.to_string())?;
        let points = &response.bode_points;
        let frequencies = Arc::new(points.iter().map(|p| p.frequency).collect::<Vec<_>>());
        let mut traces = vec![
            RetainedWaveform::new(
                "Re(Loop Gain)",
                frequencies.clone(),
                points.iter().map(|p| p.loop_gain.re).collect::<Vec<_>>(),
            )
            .with_unit("1"),
            RetainedWaveform::new(
                "Im(Loop Gain)",
                frequencies.clone(),
                points.iter().map(|p| p.loop_gain.im).collect::<Vec<_>>(),
            )
            .with_unit("1"),
        ];
        append_segments(&mut traces, points, "Loop Gain (dB)", "dB", |p| {
            p.magnitude_db
        });
        append_segments(&mut traces, points, "Loop Phase (deg)", "deg", |p| {
            p.phase_deg
        });
        traces.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(traces)
    }
}

fn append_segments(
    traces: &mut Vec<RetainedWaveform>,
    points: &[BodePoint],
    name: &str,
    unit: &str,
    value: impl Fn(&BodePoint) -> Option<f64>,
) {
    let mut start = 0;
    let mut segment = 0;
    while start < points.len() {
        if value(&points[start]).is_none() {
            start += 1;
            continue;
        }
        let mut end = start + 1;
        while end < points.len() && value(&points[end]).is_some() {
            end += 1;
        }
        segment += 1;
        let name = if start == 0 && end == points.len() {
            name.to_owned()
        } else {
            format!("{name} [segment {segment}]")
        };
        traces.push(
            RetainedWaveform::new(
                name,
                Arc::new(
                    points[start..end]
                        .iter()
                        .map(|p| p.frequency)
                        .collect::<Vec<_>>(),
                ),
                points[start..end]
                    .iter()
                    .filter_map(&value)
                    .collect::<Vec<_>>(),
            )
            .with_unit(unit),
        );
        start = end;
    }
}
