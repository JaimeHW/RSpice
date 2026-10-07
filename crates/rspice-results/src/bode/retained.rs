//! Exact Bode projection from retained traces with an explicit magnitude preference.

use super::{AcBodeMetrics, metrics_from_curves};
use crate::analysis_type::AnalysisType;
use crate::waveform::{RetainedWaveform, SharedWaveformValues};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct AcBodeSummary {
    pub signal: String,
    pub frequency: SharedWaveformValues,
    pub gain_db: SharedWaveformValues,
    pub phase_deg: Option<SharedWaveformValues>,
    pub metrics: AcBodeMetrics,
    pub analysis_index: usize,
    pub mag_index: usize,
    pub phase_index: Option<usize>,
}

/// Which retained traces a frequency response is made of, and nothing more.
///
/// The shape question — is there a magnitude trace with a phase trace that
/// matches it? — is answered from names and indices alone, in time
/// proportional to the number of traces rather than the number of samples.
/// [`AcBodeSummary`] is the shape plus the measurement: a decibel conversion
/// of the whole magnitude vector, an unwrapped copy of the phase, and every
/// crossing search over both. Asking the shape question through the summary
/// meant a caller who only wanted to know whether a Bode sheet could be
/// offered paid for all of it — once per analysis in the run, on every frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcBodeShape {
    pub signal: String,
    pub analysis_index: usize,
    pub mag_index: usize,
    pub phase_index: Option<usize>,
}

fn stb_trace_suffix<'a>(name: &'a str, prefix: &str) -> Option<&'a str> {
    let suffix = name.strip_prefix(prefix)?;
    if suffix.is_empty()
        || suffix
            .strip_prefix(" [segment ")
            .and_then(|s| s.strip_suffix(']'))
            .and_then(|s| s.parse::<usize>().ok())
            .is_some_and(|n| n > 0)
    {
        Some(suffix)
    } else {
        None
    }
}

/// Identify the core's complete Bode traces and its separately retained defined segments.
pub fn is_stb_bode_trace(name: &str) -> bool {
    stb_trace_suffix(name, "Loop Gain (dB)").is_some()
        || stb_trace_suffix(name, "Loop Phase (deg)").is_some()
}

/// Resolve named magnitude/phase traces without scanning their samples.
/// The caller supplies preference; the last matching trace wins equal preference.
pub fn ac_bode_shape_for_analysis<W: AsRef<RetainedWaveform>>(
    analysis_type: AnalysisType,
    waveforms: &[W],
    analysis_index: usize,
    prefer_magnitude: impl Fn(&W) -> bool,
) -> Option<AcBodeShape> {
    if !analysis_type.is_bode_response() || waveforms.is_empty() {
        return None;
    }
    let (signal, mag_index, phase_index) = if analysis_type == AnalysisType::Stb {
        let mag_index = waveforms
            .iter()
            .enumerate()
            .filter(|(_, w)| stb_trace_suffix(&w.as_ref().name, "Loop Gain (dB)").is_some())
            .max_by_key(|(_, w)| prefer_magnitude(w))?
            .0;
        let suffix = stb_trace_suffix(&waveforms[mag_index].as_ref().name, "Loop Gain (dB)")?;
        let phase_name = format!("Loop Phase (deg){suffix}");
        let phase_index = waveforms
            .iter()
            .position(|waveform| waveform.as_ref().name == phase_name);
        (format!("Loop Gain{suffix}"), mag_index, phase_index)
    } else {
        let (mag_index, mag) = select_magnitude_trace(waveforms, prefer_magnitude)?;
        let signal = mag
            .name
            .trim_start_matches('|')
            .trim_end_matches('|')
            .to_owned();
        let phase_name = format!("phase({signal})");
        let phase_index = waveforms
            .iter()
            .position(|waveform| waveform.as_ref().name == phase_name);
        (signal, mag_index, phase_index)
    };
    Some(AcBodeShape {
        signal,
        analysis_index,
        mag_index,
        phase_index,
    })
}

/// Project the retained response and derive its exact frequency-domain metrics.
pub fn ac_bode_summary_for_analysis<W: AsRef<RetainedWaveform>>(
    analysis_type: AnalysisType,
    waveforms: &[W],
    analysis_index: usize,
    prefer_magnitude: impl Fn(&W) -> bool,
) -> Option<AcBodeSummary> {
    // A translated multi-tone transfer is not a scalar feedback loop.
    // Its signed offset axis does not support ordinary AC bandwidth/margin claims.
    if matches!(analysis_type, AnalysisType::Qpac | AnalysisType::Qpxf) {
        return None;
    }
    let AcBodeShape {
        signal,
        analysis_index,
        mag_index,
        phase_index,
    } = ac_bode_shape_for_analysis(analysis_type, waveforms, analysis_index, prefer_magnitude)?;

    let mag = waveforms.get(mag_index)?.as_ref();
    // STB retains decibels; every other response retains linear magnitude.
    let gain_db = if analysis_type == AnalysisType::Stb {
        Arc::clone(&mag.y)
    } else {
        magnitude_to_db(&mag.y)
    };
    let frequency = Arc::clone(&mag.x);
    let phase_deg = phase_index
        .and_then(|index| waveforms.get(index))
        .map(|waveform| Arc::clone(&waveform.as_ref().y));
    let metrics = metrics_from_curves(
        frequency.as_slice(),
        gain_db.as_slice(),
        phase_deg.as_ref().map(|values| values.as_slice()),
    );

    Some(AcBodeSummary {
        signal,
        frequency,
        gain_db,
        phase_deg,
        metrics,
        analysis_index,
        mag_index,
        phase_index,
    })
}

fn select_magnitude_trace<W: AsRef<RetainedWaveform>>(
    waveforms: &[W],
    prefer_magnitude: impl Fn(&W) -> bool,
) -> Option<(usize, &RetainedWaveform)> {
    waveforms
        .iter()
        .enumerate()
        .filter(|(_, waveform)| waveform.as_ref().name.starts_with('|'))
        .max_by_key(|(_, waveform)| prefer_magnitude(waveform))
        .map(|(index, waveform)| (index, waveform.as_ref()))
}

fn magnitude_to_db(magnitude: &[f64]) -> SharedWaveformValues {
    Arc::new(
        magnitude
            .iter()
            .map(|&m| 20.0 * m.log10())
            .collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod tests;
