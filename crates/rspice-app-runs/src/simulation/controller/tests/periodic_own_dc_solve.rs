//! A PSS or HB the test starts from no operating point finds its own DC
//! point: it checks clean, runs through the run controller, and its periodic
//! state and the analysis built on it match the run seeded from an operating
//! point.

use super::periodic_port_noise::network;
use crate::simulation::controller::test_execution::run_generated_batch;
use crate::state::{AnalysisResult, AnalysisType, SimulationRun};
use rspice_results::run::SimulationRunLifecycle;

fn analysis(run: &SimulationRun, kind: AnalysisType) -> &AnalysisResult {
    run.analyses
        .iter()
        .find(|analysis| analysis.analysis_type == kind)
        .unwrap_or_else(|| panic!("{kind:?} in {:?}", run.analyses))
}

/// Every waveform the analysis retained: its axis, values and complex parts.
fn waveforms(analysis: &AnalysisResult) -> Vec<(String, Vec<f64>)> {
    let read =
        |values: &rspice_results::waveform::WaveformValues| values.samples(..).collect::<Vec<_>>();
    let mut waveforms = Vec::new();
    for waveform in &analysis.waveforms {
        waveforms.push((format!("{} x", waveform.name), read(&waveform.x)));
        waveforms.push((format!("{} y", waveform.name), read(&waveform.y)));
        if let Some(complex) = &waveform.complex {
            waveforms.push((format!("{} re", waveform.name), read(&complex.real)));
            waveforms.push((format!("{} im", waveform.name), read(&complex.imag)));
        }
    }
    waveforms
}

#[test]
fn a_periodic_solve_alone_runs_and_matches_one_started_from_an_operating_point() {
    for hb in [false, true] {
        let state = network(hb, false);
        let plan = state.model.sim_setup.analysis_plan.as_ref().unwrap();
        assert!(
            plan.validation_issues().is_empty(),
            "{:?}",
            plan.validation_issues()
        );
        let own = run_generated_batch(state, SimulationRunLifecycle::Completed);
        let seeded = run_generated_batch(network(hb, true), SimulationRunLifecycle::Completed);
        assert!(
            !own.analyses
                .iter()
                .any(|analysis| analysis.analysis_type == AnalysisType::DcOp),
            "an own DC solve adds no operating point to the run"
        );
        let kinds = if hb {
            [AnalysisType::HarmonicBalance, AnalysisType::Hbsp]
        } else {
            [AnalysisType::Pss, AnalysisType::Psp]
        };
        for kind in kinds {
            let own = waveforms(analysis(&own, kind));
            let seeded = waveforms(analysis(&seeded, kind));
            assert!(!own.is_empty(), "{kind:?} retained no waveform");
            assert_eq!(
                own.iter().map(|(name, _)| name).collect::<Vec<_>>(),
                seeded.iter().map(|(name, _)| name).collect::<Vec<_>>(),
                "{kind:?}"
            );
            for ((name, own), (_, seeded)) in own.iter().zip(&seeded) {
                assert_eq!(own.len(), seeded.len(), "{kind:?} {name}");
                let scale = seeded
                    .iter()
                    .fold(1e-12_f64, |scale, value| scale.max(value.abs()));
                for (own, seeded) in own.iter().zip(seeded) {
                    assert!(
                        (own - seeded).abs() <= 1e-9 * scale,
                        "{kind:?} {name}: {own} from its own DC solve, {seeded} seeded"
                    );
                }
            }
        }
    }
}
