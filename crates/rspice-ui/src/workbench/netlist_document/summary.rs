use std::collections::HashMap;

use crate::common::AppState;
#[cfg(test)]
use crate::state::SpecEntry;
use crate::state::{AcBodeMetrics, AcBodeSummary, SimulationRun, ac_bode_summary_for_run};

#[cfg(test)]
#[derive(Debug, Clone, PartialEq)]
pub(super) struct MeasurementDelta {
    pub name: String,
    pub old: f64,
    pub new: f64,
    pub improved: Option<bool>,
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq)]
pub(super) struct BodeDelta {
    pub name: &'static str,
    pub unit: &'static str,
    pub old: f64,
    pub new: f64,
    pub improved: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct NetlistRunSummary {
    pub stability: AcBodeMetrics,
    pub bode: Option<AcBodeSummary>,
    pub measurements: HashMap<String, (String, f64)>,
}

pub(super) fn active_run_summary(state: &AppState) -> Option<NetlistRunSummary> {
    state.simulation.active_run().map(run_summary)
}

#[cfg(test)]
pub(super) fn measurement_deltas(state: &AppState, max_items: usize) -> Vec<MeasurementDelta> {
    if max_items == 0 {
        return Vec::new();
    }

    let Some((latest_run, previous_run)) = latest_two_runs(&state.simulation.runs) else {
        return Vec::new();
    };
    let latest = run_summary(latest_run);
    let previous = run_summary(previous_run);
    if latest.measurements.is_empty() || previous.measurements.is_empty() {
        return Vec::new();
    }

    let mut keys: Vec<&String> = latest.measurements.keys().collect();
    keys.sort();
    let mut out = Vec::new();
    for key in keys {
        if out.len() == max_items {
            break;
        }
        let (name, new) = &latest.measurements[key];
        let Some((_, old)) = previous.measurements.get(key) else {
            continue;
        };
        if old == new {
            continue;
        }
        out.push(MeasurementDelta {
            name: name.clone(),
            old: *old,
            new: *new,
            improved: delta_verdict(&state.workspace.specs, name, *old, *new),
        });
    }

    out
}

#[cfg(test)]
pub(super) fn bode_deltas(state: &AppState, max_items: usize) -> Vec<BodeDelta> {
    if max_items == 0 {
        return Vec::new();
    }

    let Some((latest_run, previous_run)) = latest_two_runs(&state.simulation.runs) else {
        return Vec::new();
    };
    let previous = run_summary(previous_run);
    let latest = run_summary(latest_run);
    if previous.bode.is_none() || latest.bode.is_none() {
        return Vec::new();
    }

    let old = previous.stability;
    let new = latest.stability;
    [
        metric_delta("PM", "deg", old.pm_deg, new.pm_deg, Some(true)),
        metric_delta("GM", "dB", old.gm_db, new.gm_db, Some(true)),
        metric_delta("UGF", "Hz", old.ugf, new.ugf, None),
        metric_delta("f-3dB", "Hz", old.f3db, new.f3db, None),
        metric_delta("A_dc", "dB", old.adc_db, new.adc_db, None),
    ]
    .into_iter()
    .flatten()
    .take(max_items)
    .collect()
}

#[cfg(test)]
fn latest_two_runs(runs: &[SimulationRun]) -> Option<(&SimulationRun, &SimulationRun)> {
    let mut by_id: Vec<&SimulationRun> = runs.iter().collect();
    by_id.sort_by(|a, b| b.id.cmp(&a.id));
    Some((*by_id.first()?, *by_id.get(1)?))
}

fn run_summary(run: &SimulationRun) -> NetlistRunSummary {
    let bode = ac_bode_summary_for_run(run);
    let stability = bode
        .as_ref()
        .map(|summary| summary.metrics)
        .unwrap_or_default();
    NetlistRunSummary {
        stability,
        bode,
        measurements: run_measurements(run),
    }
}

fn run_measurements(run: &SimulationRun) -> HashMap<String, (String, f64)> {
    let mut out = HashMap::new();
    for analysis in &run.analyses {
        for measurement in &analysis.measurements {
            if let Some(value) = measurement.value {
                out.insert(
                    measurement.name.to_ascii_lowercase(),
                    (measurement.name.clone(), value),
                );
            }
        }
    }
    out
}

#[cfg(test)]
fn delta_verdict(specs: &[SpecEntry], name: &str, old: f64, new: f64) -> Option<bool> {
    let spec = specs
        .iter()
        .find(|spec| spec.measurement.eq_ignore_ascii_case(name))?;
    let (old_v, new_v) = (spec.violation(old), spec.violation(new));
    if old_v == new_v {
        return None;
    }
    Some(new_v < old_v)
}

#[cfg(test)]
fn metric_delta(
    name: &'static str,
    unit: &'static str,
    old: Option<f64>,
    new: Option<f64>,
    higher_is_better: Option<bool>,
) -> Option<BodeDelta> {
    let (old, new) = (old?, new?);
    if !old.is_finite() || !new.is_finite() || !meaningfully_changed(old, new) {
        return None;
    }
    let improved = higher_is_better.map(|higher| if higher { new > old } else { new < old });
    Some(BodeDelta {
        name,
        unit,
        old,
        new,
        improved,
    })
}

#[cfg(test)]
fn meaningfully_changed(old: f64, new: f64) -> bool {
    let scale = old.abs().max(new.abs()).max(1.0);
    (old - new).abs() > scale * 1.0e-9
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResult, AnalysisType, WaveformData};

    fn run_with_measurement(id: u64, name: &str, value: f64) -> SimulationRun {
        let mut run = SimulationRun::new(id);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "AC")
                .with_measurements(vec![rspice_core::MeasureResult::success(name, value)]),
        );
        run
    }

    fn run_with_bode(id: u64, magnitude: &[f64], phase: &[f64]) -> SimulationRun {
        let frequency = [1.0, 10.0, 100.0, 1000.0];
        let mut run = SimulationRun::new(id);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![
                wave("|V(out)|", &frequency, magnitude),
                wave("phase(V(out))", &frequency, phase),
            ]),
        );
        run
    }

    fn wave(name: &str, x: &[f64], y: &[f64]) -> WaveformData {
        WaveformData::new(name, x.to_vec(), y.to_vec(), "#fff")
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1.0e-9,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn delta_verdict_uses_spec_violation_distance() {
        let specs = vec![SpecEntry {
            measurement: "gain".to_string(),
            min: Some(10.0),
            max: None,
            unit: "dB".to_string(),
        }];

        assert_eq!(delta_verdict(&specs, "GAIN", 8.0, 9.0), Some(true));
        assert_eq!(delta_verdict(&specs, "GAIN", 9.0, 8.0), Some(false));
        assert_eq!(delta_verdict(&specs, "GAIN", 11.0, 12.0), None);
    }

    #[test]
    fn measurement_deltas_compare_latest_to_previous_run() {
        let mut state = AppState::default();
        state.workspace.specs.push(SpecEntry {
            measurement: "gain".to_string(),
            min: Some(10.0),
            max: None,
            unit: "dB".to_string(),
        });
        state
            .simulation
            .runs
            .push(run_with_measurement(1, "gain", 8.0));
        state
            .simulation
            .runs
            .push(run_with_measurement(2, "gain", 9.0));

        let deltas = measurement_deltas(&state, 4);

        assert_eq!(deltas.len(), 1);
        assert_eq!(deltas[0].name, "gain");
        assert_eq!(deltas[0].old, 8.0);
        assert_eq!(deltas[0].new, 9.0);
        assert_eq!(deltas[0].improved, Some(true));
    }

    #[test]
    fn measurement_deltas_follow_newest_first_run_history() {
        let mut state = AppState::default();
        state.workspace.specs.push(SpecEntry {
            measurement: "gain".to_string(),
            min: Some(10.0),
            max: None,
            unit: "dB".to_string(),
        });
        state.simulation.start_run().add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "AC")
                .with_measurements(vec![rspice_core::MeasureResult::success("gain", 8.0)]),
        );
        state.simulation.start_run().add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "AC")
                .with_measurements(vec![rspice_core::MeasureResult::success("gain", 9.0)]),
        );

        let deltas = measurement_deltas(&state, 4);

        assert_eq!(deltas.len(), 1);
        assert_eq!(deltas[0].old, 8.0);
        assert_eq!(deltas[0].new, 9.0);
        assert_eq!(deltas[0].improved, Some(true));
    }

    #[test]
    fn bode_deltas_compare_latest_stability_summary_to_previous_run() {
        let mut state = AppState::default();
        state.simulation.runs.push(run_with_bode(
            1,
            &[10.0, 1.0, 0.1, 0.01],
            &[-90.0, -135.0, -170.0, -190.0],
        ));
        state.simulation.runs.push(run_with_bode(
            2,
            &[10.0, 1.0, 0.01, 0.001],
            &[-60.0, -120.0, -170.0, -190.0],
        ));

        let deltas = bode_deltas(&state, 2);

        assert_eq!(deltas.len(), 2);
        assert_eq!(deltas[0].name, "PM");
        assert_eq!(deltas[0].unit, "deg");
        assert_close(deltas[0].old, 45.0);
        assert_close(deltas[0].new, 60.0);
        assert_eq!(deltas[0].improved, Some(true));
        assert_eq!(deltas[1].name, "GM");
        assert_eq!(deltas[1].unit, "dB");
        assert_eq!(deltas[1].improved, Some(true));
    }

    #[test]
    fn bode_deltas_follow_newest_first_run_history() {
        let mut state = AppState::default();
        let mut previous =
            run_with_bode(1, &[10.0, 1.0, 0.1, 0.01], &[-90.0, -135.0, -170.0, -190.0]);
        state
            .simulation
            .start_run()
            .add_analysis(previous.analyses.remove(0));
        let mut latest = run_with_bode(
            2,
            &[10.0, 1.0, 0.01, 0.001],
            &[-60.0, -120.0, -170.0, -190.0],
        );
        state
            .simulation
            .start_run()
            .add_analysis(latest.analyses.remove(0));

        let deltas = bode_deltas(&state, 2);

        assert_eq!(deltas.len(), 2);
        assert_eq!(deltas[0].name, "PM");
        assert_close(deltas[0].old, 45.0);
        assert_close(deltas[0].new, 60.0);
        assert_eq!(deltas[0].improved, Some(true));
    }

    #[test]
    fn bode_deltas_skip_missing_or_unchanged_metrics() {
        let mut state = AppState::default();
        state
            .simulation
            .runs
            .push(run_with_bode(1, &[10.0, 1.0, 0.1, 0.01], &[-90.0; 4]));
        state
            .simulation
            .runs
            .push(run_with_bode(2, &[10.0, 1.0, 0.1, 0.01], &[-90.0; 4]));

        assert!(bode_deltas(&state, 4).is_empty());
    }

    #[test]
    fn active_run_summary_carries_measurements_and_ac_metrics() {
        let frequency = [1.0, 10.0, 100.0, 1000.0];
        let mut run = SimulationRun::new(3);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "AC")
                .with_waveforms(vec![
                    wave("|V(out)|", &frequency, &[10.0, 1.0, 0.1, 0.01]),
                    wave(
                        "phase(V(out))",
                        &frequency,
                        &[-45.0, -135.0, -180.0, -225.0],
                    ),
                ])
                .with_measurements(vec![rspice_core::MeasureResult::success("gain", 42.0)]),
        );

        let mut state = AppState::default();
        state.simulation.runs.push(run);
        state.simulation.active_run_idx = Some(0);

        let summary = active_run_summary(&state).expect("active summary");

        assert_eq!(summary.measurements["gain"].1, 42.0);
        assert!(summary.bode.is_some());
        assert_close(summary.stability.ugf.unwrap(), 10.0);
        assert_close(summary.stability.pm_deg.unwrap(), 45.0);
    }
}
