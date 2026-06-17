use std::sync::Arc;

use crate::common::AppState;
use crate::state::AnalysisType;
use crate::ui::plot::sample_at;

#[derive(Debug, Clone)]
pub(super) struct NetlistRunSummary {
    pub frequency: Arc<[f64]>,
    pub gain_db: Arc<[f64]>,
    pub stability: StabilitySummary,
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct StabilitySummary {
    pub adc_db: Option<f64>,
    pub ugf: Option<f64>,
    pub pm_deg: Option<f64>,
    pub gm_db: Option<f64>,
    pub f180: Option<f64>,
    pub f3db: Option<f64>,
    pub gain_extremes: Option<(f64, f64)>,
}

pub(super) fn active_run_summary(state: &mut AppState) -> Option<NetlistRunSummary> {
    let simulation = &state.simulation;
    let run = simulation.active_run()?;
    let (analysis_index, analysis) = run.analyses.iter().enumerate().find(|(_, analysis)| {
        analysis.analysis_type == AnalysisType::Ac && !analysis.waveforms.is_empty()
    })?;

    let (mag_index, mag) = analysis
        .waveforms
        .iter()
        .enumerate()
        .filter(|(_, waveform)| waveform.name.starts_with('|'))
        .max_by_key(|(_, waveform)| waveform.visible)?;
    let base = mag.name.trim_start_matches('|').trim_end_matches('|');
    let phase = analysis
        .waveforms
        .iter()
        .find(|waveform| waveform.name == format!("phase({base})"))
        .map(|waveform| Arc::clone(&waveform.y));

    let gain_db = state
        .shell
        .results
        .derived
        .db((analysis_index as u64) << 32 | mag_index as u64, &mag.y);
    let frequency = Arc::clone(&mag.x);
    let adc_db = gain_db.first().copied();
    let ugf = crossing(&frequency, &gain_db, 0.0);
    let f3db = adc_db.and_then(|adc| crossing(&frequency, &gain_db, adc - 3.0));
    let (pm_deg, f180, gm_db) = match phase {
        Some(phase) => {
            let pm = ugf.map(|f| 180.0 + sample_at(&frequency, &phase, f));
            let f180 = crossing(&frequency, &phase, -180.0);
            let gm = f180.map(|f| -sample_at(&frequency, &gain_db, f));
            (pm, f180, gm)
        }
        None => (None, None, None),
    };

    Some(NetlistRunSummary {
        frequency,
        gain_db: Arc::clone(&gain_db),
        stability: StabilitySummary {
            adc_db,
            ugf,
            pm_deg,
            gm_db,
            f180,
            f3db,
            gain_extremes: finite_extremes(&gain_db),
        },
    })
}

pub(super) fn crossing(frequency: &[f64], series: &[f64], level: f64) -> Option<f64> {
    let n = frequency.len().min(series.len());
    for i in 1..n {
        let (y0, y1) = (series[i - 1] - level, series[i] - level);
        if y0 == 0.0 {
            return Some(frequency[i - 1]);
        }
        if y0 * y1 < 0.0 {
            let t = y0 / (y0 - y1);
            let (l0, l1) = (frequency[i - 1].log10(), frequency[i].log10());
            return Some(10f64.powf(l0 + t * (l1 - l0)));
        }
    }
    None
}

fn finite_extremes(values: &[f64]) -> Option<(f64, f64)> {
    values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .fold(None, |acc, value| match acc {
            Some((lo, hi)) => Some((lo.min(value), hi.max(value))),
            None => Some((value, value)),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crossing_interpolates_in_log_frequency() {
        let frequency = [1.0, 10.0, 100.0];
        let gain = [20.0, 0.0, -20.0];

        assert_eq!(crossing(&frequency, &gain, 0.0), Some(10.0));
    }

    #[test]
    fn crossing_returns_none_without_crossing() {
        let frequency = [1.0, 10.0, 100.0];
        let gain = [20.0, 10.0, 2.0];

        assert_eq!(crossing(&frequency, &gain, 0.0), None);
    }
}
