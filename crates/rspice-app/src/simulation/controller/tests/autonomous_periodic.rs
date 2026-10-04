//! Project persistence for autonomous periodic runs and retained solver responses.
use crate::io::project_io::{ProjectAnalysisResult, ProjectSimulationResults};
use crate::state::{AnalysisResultPayload, AnalysisType, SimulationRun, SimulationState};
use rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod;
use rspice_core::engine::{
    Engine, QpacRequest, QpnoiseFrequencyAxis, QpnoiseRequest, QpssConfig, QpssOperatingPoint,
    QpxfFrequencyAxis, QpxfRequest,
};
use rspice_simulation::results::SimulationResult;
use rspice_simulation_contract::quasi_periodic_draft::{
    QpnoiseOutputDraft, QpnoiseSourceSelection, QpssDraft, QuasiPeriodicAcDraft,
    QuasiPeriodicNoiseDraft, QuasiPeriodicTransferDraft,
};
use std::{
    f64::consts::{SQRT_2, TAU},
    sync::Arc,
};

fn round_trip(run: SimulationRun) -> SimulationRun {
    for analysis in &run.analyses {
        analysis.validate_retained_evidence().unwrap();
        let saved = ProjectAnalysisResult::from(analysis);
        let decoded: ProjectAnalysisResult =
            serde_json::from_str(&serde_json::to_string(&saved).unwrap()).unwrap();
        assert_eq!(decoded, saved);
    }
    let mut state = SimulationState::default();
    state.retained.next_run_id = run.id;
    state.retained.runs = vec![run.clone()].into();
    let saved = crate::io::capture_simulation_results(&state);
    saved.validate().unwrap();
    let decoded: ProjectSimulationResults =
        serde_json::from_str(&serde_json::to_string(&saved).unwrap()).unwrap();
    assert_eq!(decoded, saved);
    let loaded = crate::io::simulation_state_from_results(decoded).unwrap();
    let restored = &loaded.retained.runs[0];
    restored.validate_provenance().unwrap();
    assert_eq!(restored.analyses.len(), run.analyses.len());
    for (original, result) in run.analyses.iter().zip(&restored.analyses) {
        result.validate_retained_evidence().unwrap();
        assert_eq!(result.result_data_digest(), original.result_data_digest());
        assert_eq!(result.result_payload, original.result_payload);
    }
    restored.clone()
}

// Consumer persistence only needs authentic retained solver responses. Prepared
// app scheduling of autonomous QP consumers is not part of this DTO contract.
fn response_source(
    deck: &str,
    config: QpssConfig,
) -> (Engine, rspice_core::Netlist, QpssOperatingPoint) {
    let netlist = rspice_core::Netlist::parse(deck).unwrap();
    let mut settings = rspice_simulation::netlist_preparation::build_engine_config(&netlist, None);
    settings.tolerance = config.solver.relative_tolerance;
    let engine = Engine::try_new_with_resolved_config(settings).unwrap();
    let point = engine.run_qpss(&netlist, config).unwrap();
    (engine, netlist, point)
}

fn round_trip_response(
    result: SimulationResult,
    kind: AnalysisType,
) -> crate::state::AnalysisResult {
    let retained = super::super::SimulationController::new()
        .convert_to_analysis_result_with_metadata_owned(result, kind, "QP response");
    retained.validate_retained_evidence().unwrap();
    let saved = ProjectAnalysisResult::from(&retained);
    let decoded: ProjectAnalysisResult =
        serde_json::from_str(&serde_json::to_string(&saved).unwrap()).unwrap();
    assert_eq!(decoded, saved);
    retained
}

#[test]
fn autonomous_qpss_project_round_trip_preserves_solved_oscillator_controls() {
    let draft = QpssDraft {
        tones: format!("{},{}", 1.2 / TAU, SQRT_2 / TAU),
        harmonics: "2,3".into(),
        collocation_points: "13,25".into(),
        relative_tolerance: "1e-8".into(),
        autonomous: true,
        oscillator_node: "x".into(),
        oscillator_tone: "1".into(),
        oscillator_phase_tuple: "1,0".into(),
        oscillator_amplitude: ".8".into(),
        oscillator_minimum_amplitude: ".01".into(),
        oscillator_frequency_step: ".15".into(),
        oscillator_seeds: "y,.8,-90".into(),
        ..Default::default()
    };
    let config = draft.to_spec().unwrap().qpss_config().unwrap();
    let deck = format!(
        "QPSS oscillator\nCx x 0 1\nCy y 0 1\nBx 0 x I={{(1-v(x)^2-v(y)^2)*v(x)-(1+.1*sqrt(2)*cos(sqrt(2)*time))*v(y)}}\nBy 0 y I={{(1-v(x)^2-v(y)^2)*v(y)+(1+.1*sqrt(2)*cos(sqrt(2)*time))*v(x)}}\n{}\n.end\n",
        config.to_spice().unwrap()
    );
    let run = round_trip(crate::simulation::controller::test_execution::run_manual_batch(&deck));
    let analysis = run
        .analyses
        .iter()
        .find(|result| result.analysis_type == AnalysisType::Qpss)
        .unwrap();
    let Some(AnalysisResultPayload::Qpss { operating_point }) = &analysis.result_payload else {
        panic!("QPSS payload expected");
    };
    assert_eq!(operating_point.config(), &config);
    assert!((operating_point.oscillator_frequency_hz().unwrap() - 1.0 / TAU).abs() < 1e-8);
}

#[test]
fn autonomous_qpac_qpxf_project_round_trip_preserves_phase_response() {
    let producer = QpssDraft {
        tones: format!("{},{}", 1.2 / TAU, SQRT_2 / TAU),
        harmonics: "3,1".into(),
        collocation_points: "17,9".into(),
        relative_tolerance: "1e-9".into(),
        autonomous: true,
        oscillator_node: "x".into(),
        oscillator_amplitude: ".8".into(),
        oscillator_seeds: "y,.8,-90".into(),
        ..Default::default()
    }
    .to_spec()
    .unwrap();
    let deck = "QP phase response\nCx x 0 1\nCy y 0 1\nIprobe 0 x DC 0\nBx 0 x I={(1+.1*cos(sqrt(2)*time))*(1-v(x)^2-v(y)^2)*v(x)-v(y)}\nBy 0 y I={(1+.1*cos(sqrt(2)*time))*(1-v(x)^2-v(y)^2)*v(y)+v(x)}\n.end\n";
    let ac = QuasiPeriodicAcDraft {
        explicit_offsets: "-.03,1e-20,.04".into(),
        input_source: "Iprobe".into(),
        input_lattice: "1,0".into(),
        output_lattice: "1,0".into(),
        output_node: "x".into(),
        magnitude: ".2".into(),
        phase_degrees: "37".into(),
        ..Default::default()
    }
    .to_spec()
    .unwrap();
    let xf = QuasiPeriodicTransferDraft {
        explicit_frequencies: "-.03,1e-20,.04".into(),
        frequency_axis: QpxfFrequencyAxis::Offset,
        input_source: "Iprobe".into(),
        input_lattice: "1,0".into(),
        output_lattice: "1,0".into(),
        output_node: "x".into(),
        group_delay: true,
        ..Default::default()
    }
    .to_spec()
    .unwrap();
    let (engine, netlist, point) = response_source(deck, producer.qpss_config().unwrap());
    let ac = engine
        .run_qpac_from_qpss(
            &netlist,
            QpacRequest::from_qpac_card(&ac.qpac_card().unwrap()).unwrap(),
            &point,
        )
        .unwrap();
    let xf = engine
        .run_qpxf_from_qpss(
            &netlist,
            QpxfRequest::from_qpxf_card(&xf.qpxf_card().unwrap()).unwrap(),
            &point,
        )
        .unwrap();
    round_trip_response(
        SimulationResult::from_qpac_response(Arc::new(ac)).unwrap(),
        AnalysisType::Qpac,
    );
    round_trip_response(
        SimulationResult::from_qpxf_response(Arc::new(xf)).unwrap(),
        AnalysisType::Qpxf,
    );
}

#[test]
fn autonomous_qpnoise_project_round_trip_preserves_correlations_and_solver_options() {
    let deck = "Studio oscillator noise\nCx x 0 1\nCy y 0 1\nRx x 0 1\nRy y 0 1\nIprobe 0 x DC 0\nBx 0 x I={(2-v(x)^2-v(y)^2)*v(x)-v(y)}\nBy 0 y I={(2-v(x)^2-v(y)^2)*v(y)+v(x)}\n.end\n";
    let producer = QpssDraft {
        tones: format!("{},{}", 1.2 / TAU, SQRT_2 / TAU),
        harmonics: "3,1".into(),
        collocation_points: "17,9".into(),
        relative_tolerance: "1e-9".into(),
        autonomous: true,
        oscillator_node: "x".into(),
        oscillator_amplitude: ".8".into(),
        oscillator_seeds: "y,.8,-90".into(),
        ..Default::default()
    }
    .to_spec()
    .unwrap();
    let draft = QuasiPeriodicNoiseDraft {
        explicit_frequencies: "0.005,0.01".into(),
        frequency_axis: QpnoiseFrequencyAxis::Offset,
        output_node: "x".into(),
        output_lattice: "1,0".into(),
        additional_outputs: vec![QpnoiseOutputDraft {
            node: "y".into(),
            lattice: "1,0".into(),
            ..Default::default()
        }],
        input_source: "Iprobe".into(),
        input_lattice: "1,0".into(),
        source_selection: QpnoiseSourceSelection::Only,
        source_names: "RX thermal\nRY thermal".into(),
        linear_method: QuasiPeriodicLinearMethod::Krylov,
        krylov_restart: "48".into(),
        krylov_cycles: "8".into(),
        linear_tolerance: "1e-9".into(),
        ..Default::default()
    };
    let draft: QuasiPeriodicNoiseDraft = ron::from_str(&ron::to_string(&draft).unwrap()).unwrap();
    let spec = draft.to_spec().unwrap();
    let (engine, netlist, point) = response_source(deck, producer.qpss_config().unwrap());
    let response = engine
        .run_qpnoise_from_qpss(
            &netlist,
            QpnoiseRequest::from_qpnoise_card(&spec.qpnoise_card().unwrap()).unwrap(),
            &point,
        )
        .unwrap();
    let analysis = round_trip_response(
        SimulationResult::from_qpnoise_response(Arc::new(response)).unwrap(),
        AnalysisType::Qpnoise,
    );
    let Some(AnalysisResultPayload::Qpnoise { response }) = &analysis.result_payload else {
        panic!("QPNOISE payload expected");
    };
    assert_eq!(
        response.metadata.request.linear.method,
        QuasiPeriodicLinearMethod::Krylov
    );
    assert_eq!(response.metadata.request.linear.restart, 48);
    assert_eq!(response.metadata.request.linear.max_cycles, 8);
    assert_eq!(response.metadata.request.linear.relative_tolerance, 1e-9);
    assert_eq!(response.sources.len(), 2);
    assert_eq!(response.outputs.len(), 2);
}
