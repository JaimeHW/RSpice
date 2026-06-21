//! Pure AC/Bode summary extraction shared by Results and netlist summaries.

use std::sync::Arc;

use super::{AnalysisResult, AnalysisType, SharedWaveformValues, SimulationRun, WaveformData};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct AcBodeMetrics {
    pub adc_db: Option<f64>,
    pub ugf: Option<f64>,
    pub pm_deg: Option<f64>,
    pub f180: Option<f64>,
    pub gm_db: Option<f64>,
    pub f3db: Option<f64>,
    pub gain_extremes: (f64, f64),
    pub phase_extremes: Option<(f64, f64)>,
}

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

pub fn ac_bode_summary_for_run(run: &SimulationRun) -> Option<AcBodeSummary> {
    let (analysis_index, analysis) = run.analyses.iter().enumerate().find(|(_, analysis)| {
        analysis.analysis_type == AnalysisType::Ac && !analysis.waveforms.is_empty()
    })?;
    ac_bode_summary_for_analysis(analysis, analysis_index)
}

pub fn ac_bode_summary_for_analysis(
    analysis: &AnalysisResult,
    analysis_index: usize,
) -> Option<AcBodeSummary> {
    if analysis.analysis_type != AnalysisType::Ac || analysis.waveforms.is_empty() {
        return None;
    }

    let (mag_index, mag) = select_magnitude_trace(&analysis.waveforms)?;
    let signal = mag.name.trim_start_matches('|').trim_end_matches('|');
    let phase_name = format!("phase({signal})");
    let phase = analysis
        .waveforms
        .iter()
        .enumerate()
        .find(|(_, waveform)| waveform.name == phase_name);

    let frequency = Arc::clone(&mag.x);
    let gain_db = magnitude_to_db(&mag.y);
    let phase_deg = phase.map(|(_, waveform)| Arc::clone(&waveform.y));
    let phase_index = phase.map(|(index, _)| index);
    let metrics = metrics_from_curves(
        frequency.as_slice(),
        gain_db.as_slice(),
        phase_deg.as_ref().map(|values| values.as_slice()),
    );

    Some(AcBodeSummary {
        signal: signal.to_owned(),
        frequency,
        gain_db,
        phase_deg,
        metrics,
        analysis_index,
        mag_index,
        phase_index,
    })
}

pub fn log_frequency_crossing(frequency: &[f64], series: &[f64], level: f64) -> Option<f64> {
    let n = frequency.len().min(series.len());
    for i in 1..n {
        let (f0, f1) = (frequency[i - 1], frequency[i]);
        if f0 <= 0.0 || f1 <= 0.0 {
            continue;
        }
        let (y0, y1) = (series[i - 1] - level, series[i] - level);
        if y0 == 0.0 {
            return Some(f0);
        }
        if y0 * y1 < 0.0 {
            let t = y0 / (y0 - y1);
            let (l0, l1) = (f0.log10(), f1.log10());
            return Some(10f64.powf(l0 + t * (l1 - l0)));
        }
    }
    None
}

fn select_magnitude_trace(waveforms: &[WaveformData]) -> Option<(usize, &WaveformData)> {
    waveforms
        .iter()
        .enumerate()
        .filter(|(_, waveform)| waveform.name.starts_with('|'))
        .max_by_key(|(_, waveform)| waveform.visible)
}

fn magnitude_to_db(magnitude: &[f64]) -> SharedWaveformValues {
    Arc::new(
        magnitude
            .iter()
            .map(|&m| 20.0 * m.max(1e-30).log10())
            .collect::<Vec<_>>(),
    )
}

fn metrics_from_curves(
    frequency: &[f64],
    gain_db: &[f64],
    phase_deg: Option<&[f64]>,
) -> AcBodeMetrics {
    let adc_db = gain_db.first().copied();
    let ugf = log_frequency_crossing(frequency, gain_db, 0.0);
    let f3db = adc_db.and_then(|adc| log_frequency_crossing(frequency, gain_db, adc - 3.0));
    let mut metrics = AcBodeMetrics {
        adc_db,
        ugf,
        pm_deg: None,
        f180: None,
        gm_db: None,
        f3db,
        gain_extremes: finite_extremes(gain_db).unwrap_or((0.0, 0.0)),
        phase_extremes: phase_deg.and_then(finite_extremes),
    };

    if let Some(phase) = phase_deg {
        if let Some(ugf) = metrics.ugf {
            metrics.pm_deg = Some(180.0 + sample_at(frequency, phase, ugf));
        }
        metrics.f180 = log_frequency_crossing(frequency, phase, -180.0);
        if let Some(f180) = metrics.f180 {
            metrics.gm_db = Some(-sample_at(frequency, gain_db, f180));
        }
    }

    metrics
}

fn finite_extremes(values: &[f64]) -> Option<(f64, f64)> {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for &value in values {
        if value.is_finite() {
            lo = lo.min(value);
            hi = hi.max(value);
        }
    }
    (lo <= hi).then_some((lo, hi))
}

fn sample_at(x: &[f64], y: &[f64], xq: f64) -> f64 {
    let n = x.len().min(y.len());
    if n == 0 {
        return 0.0;
    }
    if xq <= x[0] {
        return y[0];
    }
    if xq >= x[n - 1] {
        return y[n - 1];
    }
    let hi = x[..n].partition_point(|&value| value < xq).max(1);
    let lo = hi - 1;
    let span = x[hi] - x[lo];
    if span <= 0.0 {
        return y[lo];
    }
    let t = (xq - x[lo]) / span;
    y[lo] + t * (y[hi] - y[lo])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ac_analysis(waveforms: Vec<WaveformData>) -> AnalysisResult {
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(waveforms)
    }

    fn wave(name: &str, x: &[f64], y: &[f64], visible: bool) -> WaveformData {
        let mut waveform = WaveformData::new(name, x.to_vec(), y.to_vec(), "#fff");
        waveform.visible = visible;
        waveform
    }

    fn assert_close(actual: Option<f64>, expected: f64) {
        let actual = actual.expect("metric exists");
        assert!(
            (actual - expected).abs() < 1.0e-9,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn log_frequency_crossing_interpolates_between_positive_frequencies() {
        let f = [1.0, 10.0, 100.0];
        let y = [20.0, 0.0, -20.0];

        assert_eq!(log_frequency_crossing(&f, &y, 0.0), Some(10.0));
    }

    #[test]
    fn ac_summary_selects_magnitude_phase_pair_and_computes_margins() {
        let frequency = [1.0, 10.0, 100.0, 1000.0];
        let magnitude = [10.0, 1.0, 0.1, 0.01];
        let phase = [-45.0, -135.0, -180.0, -225.0];
        let analysis = ac_analysis(vec![
            wave("|V(out)|", &frequency, &magnitude, true),
            wave("phase(V(out))", &frequency, &phase, true),
        ]);

        let summary = ac_bode_summary_for_analysis(&analysis, 3).expect("AC summary");

        assert_eq!(summary.signal, "V(out)");
        assert_eq!(summary.analysis_index, 3);
        assert_eq!(summary.mag_index, 0);
        assert_eq!(summary.phase_index, Some(1));
        assert_close(summary.metrics.adc_db, 20.0);
        assert_close(summary.metrics.ugf, 10.0);
        assert_close(summary.metrics.pm_deg, 45.0);
        assert_close(summary.metrics.f180, 100.0);
        assert_close(summary.metrics.gm_db, 20.0);
        assert_close(summary.metrics.f3db, 10f64.powf(0.15));
        assert_eq!(summary.metrics.gain_extremes, (-40.0, 20.0));
        assert_eq!(summary.metrics.phase_extremes, Some((-225.0, -45.0)));
    }

    #[test]
    fn ac_summary_omits_phase_metrics_when_phase_trace_is_missing() {
        let frequency = [1.0, 10.0, 100.0];
        let magnitude = [2.0, 1.0, 0.5];
        let analysis = ac_analysis(vec![wave("|V(out)|", &frequency, &magnitude, true)]);

        let summary = ac_bode_summary_for_analysis(&analysis, 0).expect("AC summary");

        assert!(summary.phase_deg.is_none());
        assert_eq!(summary.metrics.pm_deg, None);
        assert_eq!(summary.metrics.f180, None);
        assert_eq!(summary.metrics.gm_db, None);
    }

    #[test]
    fn ac_summary_prefers_visible_magnitude_trace() {
        let frequency = [1.0, 10.0];
        let hidden = [1.0, 1.0];
        let visible = [10.0, 1.0];
        let analysis = ac_analysis(vec![
            wave("|V(in)|", &frequency, &hidden, false),
            wave("|V(out)|", &frequency, &visible, true),
        ]);

        let summary = ac_bode_summary_for_analysis(&analysis, 0).expect("AC summary");

        assert_eq!(summary.signal, "V(out)");
        assert_eq!(summary.mag_index, 1);
    }

    #[test]
    fn ac_summary_uses_last_matching_magnitude_when_visibility_ties() {
        let frequency = [1.0, 10.0];
        let first = [1.0, 1.0];
        let last = [10.0, 1.0];
        let analysis = ac_analysis(vec![
            wave("|V(first)|", &frequency, &first, true),
            wave("|V(last)|", &frequency, &last, true),
        ]);

        let summary = ac_bode_summary_for_analysis(&analysis, 0).expect("AC summary");

        assert_eq!(summary.signal, "V(last)");
        assert_eq!(summary.mag_index, 1);
    }

    #[test]
    fn ac_summary_for_run_uses_first_ac_analysis_with_waveforms() {
        let frequency = [1.0, 10.0];
        let magnitude = [10.0, 1.0];
        let mut run = SimulationRun::new(7);
        run.add_analysis(AnalysisResult::new(1, AnalysisType::DcOp, "OP"));
        run.add_analysis(ac_analysis(vec![wave(
            "|V(out)|", &frequency, &magnitude, true,
        )]));

        let summary = ac_bode_summary_for_run(&run).expect("AC summary");

        assert_eq!(summary.analysis_index, 1);
        assert_eq!(summary.signal, "V(out)");
    }

    #[test]
    fn ac_summary_for_run_does_not_skip_unusable_first_ac_analysis() {
        let frequency = [1.0, 10.0];
        let magnitude = [10.0, 1.0];
        let mut run = SimulationRun::new(7);
        run.add_analysis(ac_analysis(vec![wave(
            "phase(V(in))",
            &frequency,
            &magnitude,
            true,
        )]));
        run.add_analysis(ac_analysis(vec![wave(
            "|V(out)|", &frequency, &magnitude, true,
        )]));

        assert_eq!(ac_bode_summary_for_run(&run), None);
    }
}
