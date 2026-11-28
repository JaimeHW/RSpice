//! An AC, noise or transfer-function analysis solves its own operating point:
//! a test holding only that analysis checks clean and runs, and an operating
//! point beside it changes none of its numbers.

use crate::simulation::controller::test_execution::{pvt, run_generated_batch};
use crate::simulation::plan::{AnalysisDraft, AnalysisKind};
use crate::state::{AnalysisResult, AnalysisType, SimulationRun};
use rspice_results::analysis_payload::AnalysisResultPayload;
use rspice_results::run::SimulationRunLifecycle;
use rspice_results::transfer_function::TransferFunctionScalarEvidence;

/// `VDD` (AC 1, DC 1.8) into R1 = R2 = 1k with 1 pF across R2: the authored
/// divider of the run-set fixtures, with `draft` added to its test and, when
/// asked, an operating point before it.
fn divider_with(draft: &AnalysisDraft, with_operating_point: bool) -> SimulationRun {
    let mut state = pvt::divider();
    let plan = state.model.sim_setup.analysis_plan.as_mut().unwrap();
    if with_operating_point {
        plan.insert(AnalysisKind::OperatingPoint).unwrap();
    }
    let (id, _) = plan.insert(draft.kind()).unwrap();
    let draft = draft.clone();
    plan.edit(id, move |slot| *slot = draft).unwrap();
    assert!(
        plan.validation_issues().is_empty(),
        "{:?}",
        plan.validation_issues()
    );
    run_generated_batch(state, SimulationRunLifecycle::Completed)
}

fn analysis(run: &SimulationRun, kind: AnalysisType) -> &AnalysisResult {
    run.analyses
        .iter()
        .find(|analysis| analysis.analysis_type == kind)
        .unwrap_or_else(|| panic!("{kind:?} in {:?}", run.analyses))
}

/// Every number the analysis retained, bit for bit: each waveform's axis,
/// values and complex parts, its measurements, and its typed result.
fn numbers(analysis: &AnalysisResult) -> Vec<(String, Vec<u64>)> {
    let bits = |values: &rspice_results::waveform::WaveformValues| {
        values.samples(..).map(f64::to_bits).collect::<Vec<_>>()
    };
    let mut numbers = Vec::new();
    for waveform in &analysis.waveforms {
        numbers.push((format!("{} x", waveform.name), bits(&waveform.x)));
        numbers.push((format!("{} y", waveform.name), bits(&waveform.y)));
        if let Some(complex) = &waveform.complex {
            numbers.push((format!("{} re", waveform.name), bits(&complex.real)));
            numbers.push((format!("{} im", waveform.name), bits(&complex.imag)));
        }
    }
    for measurement in &analysis.measurements {
        numbers.push((
            format!("measurement {}", measurement.name),
            measurement
                .value
                .iter()
                .map(|value| value.to_bits())
                .collect(),
        ));
    }
    numbers.push((
        "typed result".to_owned(),
        serde_json::to_vec(&analysis.result_payload)
            .unwrap()
            .into_iter()
            .map(u64::from)
            .collect(),
    ));
    numbers
}

fn alone_and_beside_an_operating_point(draft: AnalysisDraft, kind: AnalysisType) -> SimulationRun {
    let alone = divider_with(&draft, false);
    let beside = divider_with(&draft, true);
    assert_eq!(beside.analyses.len(), 2, "{:?}", beside.analyses);
    let solved = analysis(&alone, kind);
    assert!(
        !solved.waveforms.is_empty() || solved.result_payload.is_some(),
        "{solved:?}"
    );
    let numbers_alone = numbers(solved);
    assert_eq!(
        numbers_alone,
        numbers(analysis(&beside, kind)),
        "an operating point beside {kind:?} changes none of its numbers"
    );
    alone
}

#[test]
fn an_ac_analysis_alone_runs_and_matches_one_run_beside_an_operating_point() {
    let run = alone_and_beside_an_operating_point(
        AnalysisDraft::Ac(rspice_simulation_contract::drafts::AcSetup {
            fstart: "1k".into(),
            fstop: "10G".into(),
            points: "10".into(),
            sweep: 0,
        }),
        AnalysisType::Ac,
    );
    let ac = analysis(&run, AnalysisType::Ac);
    let out = ac
        .waveforms
        .iter()
        .find(|waveform| waveform.name.eq_ignore_ascii_case("V(out)"))
        .unwrap_or_else(|| panic!("V(out) in {:?}", ac.waveforms));
    let complex = out.complex.as_ref().expect("an AC trace is complex");
    let frequencies = out.x.read_whole().unwrap();
    let real = complex.real.read_whole().unwrap();
    let imag = complex.imag.read_whole().unwrap();
    assert_eq!(frequencies.len(), real.len());
    // 1/2 through R1 = R2 = 1k, rolled off by 1 pF across their 500 ohm
    // Thevenin resistance.
    for ((frequency, re), im) in frequencies.iter().zip(real.iter()).zip(imag.iter()) {
        let value = re.hypot(*im);
        let omega_tau = 2.0 * std::f64::consts::PI * frequency * 500.0 * 1e-12;
        let expected = 0.5 / (1.0 + omega_tau * omega_tau).sqrt();
        assert!(
            (value / expected - 1.0).abs() < 1e-9,
            "|V(out)| at {frequency} Hz is {value}, the closed form {expected}"
        );
    }
}

#[test]
fn a_noise_analysis_alone_runs_and_matches_one_run_beside_an_operating_point() {
    let mut draft = rspice_simulation_contract::drafts::NoiseDraft {
        output: "out".into(),
        input: "VDD".into(),
        ..Default::default()
    };
    draft.points = "5".into();
    alone_and_beside_an_operating_point(AnalysisDraft::Noise(draft), AnalysisType::Noise);
}

#[test]
fn a_transfer_function_alone_runs_and_matches_one_run_beside_an_operating_point() {
    let mut draft = AnalysisDraft::for_kind(AnalysisKind::TransferFunction);
    let AnalysisDraft::TransferFunction(transfer) = &mut draft else {
        unreachable!("a transfer function's draft");
    };
    transfer.input_source = "VDD".into();
    transfer.output_expression = "V(out)".into();
    let run = alone_and_beside_an_operating_point(draft, AnalysisType::Tf);
    let Some(AnalysisResultPayload::TransferFunction {
        gain,
        input_resistance,
        output_resistance,
        ..
    }) = &analysis(&run, AnalysisType::Tf).result_payload
    else {
        panic!("a transfer function's typed result");
    };
    // R2/(R1+R2), R1+R2 and R1||R2.
    for (scalar, expected) in [
        (gain, 0.5),
        (input_resistance, 2e3),
        (output_resistance, 500.0),
    ] {
        let Some(TransferFunctionScalarEvidence::Finite(value)) = scalar else {
            panic!("{scalar:?} against {expected}");
        };
        assert!(
            (value / expected - 1.0).abs() < 1e-9,
            "{value} against {expected}"
        );
    }
}
