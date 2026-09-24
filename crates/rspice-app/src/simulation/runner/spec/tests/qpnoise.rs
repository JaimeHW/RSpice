//! The runnable Studio analysis consumes the authenticated producer dependency.
use super::*;

#[test]
fn autonomous_qpnoise_studio_preserves_options_correlations_worker_and_saved_results() {
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
    use crate::simulation::execution::{ExecutionArtifactEnvelope, PreparedDependencyBinding};
    use crate::simulation::plan::{
        QpnoiseOutputDraft, QpnoiseSourceSelection, QpssDraft, QuasiPeriodicNoiseDraft,
    };
    use crate::simulation::runner::worker_contract::{
        WorkerAnalysisSpec, round_trip_response_for_test,
    };
    use rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod;
    use rspice_core::engine::{QpnoiseFrequencyAxis, QpnoiseValue};
    use std::f64::consts::{SQRT_2, TAU};
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
    let point = svc_runner::run_qpss_analysis_with_source_path_and_abort(
        deck,
        producer.qpss_config().unwrap(),
        None,
        &rspice_core::NoAbort,
    )
    .unwrap()
    .operating_point;
    let snapshot = ContentDigest::from_bytes([84; 32]);
    let binding = PreparedDependencyBinding::qpss_state(
        AnalysisInstanceId::new(),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([85; 32]),
    );
    let artifact = ExecutionArtifactEnvelope::from_qpss_result(
        snapshot,
        binding.producer_instance_id(),
        binding.producer_source_revision(),
        binding.producer_config_digest(),
        &producer,
        &SimulationResult::from_qpss_operating_point(point.clone()).unwrap(),
    )
    .unwrap()
    .unwrap();
    let deps = ResolvedExecutionDependencies::resolve(
        snapshot,
        vec![binding.clone()],
        &HashMap::from([(binding.producer_instance_id(), artifact)]),
    )
    .unwrap();
    let (metadata, buffers) = deps.encode_transfer().unwrap();
    let deps = ResolvedExecutionDependencies::decode_transfer(&metadata, buffers).unwrap();
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
    let worker = WorkerAnalysisSpec::try_from(&spec).unwrap();
    let restored: AnalysisSpec =
        serde_json::from_str::<WorkerAnalysisSpec>(&serde_json::to_string(&worker).unwrap())
            .unwrap()
            .into();
    assert_eq!(restored, spec);
    deps.validate_for_spec(&restored, &Default::default())
        .unwrap();
    let result = run_spec_request(
        &EngineBridge::new(),
        restored,
        Default::default(),
        deck,
        None,
        &deps,
        &rspice_core::NoAbort,
    )
    .unwrap();
    let result = round_trip_response_for_test(result);
    let SimulationResult::Qpnoise { response, .. } = &result else {
        panic!("QPNOISE expected")
    };
    assert_eq!(
        response.metadata.operating_point_identity,
        point.retained_identity()
    );
    assert_eq!(
        response.metadata.request.linear.method,
        QuasiPeriodicLinearMethod::Krylov
    );
    assert_eq!(response.metadata.request.linear.restart, 48);
    assert_eq!(response.metadata.request.linear.max_cycles, 8);
    assert_eq!(response.metadata.request.linear.relative_tolerance, 1e-9);
    assert_eq!(response.sources.len(), 2);
    assert_eq!(response.outputs.len(), 2);
    let s = 4.0 * 1.380649e-23 * 300.15;
    for (i, offset) in [0.005, 0.01].into_iter().enumerate() {
        let d = TAU * offset;
        let power = |w: f64| 1.0 / (4.0 + w * w) + 1.0 / (w * w);
        let expected = s / 4.0 * (power(d) + power(2.0 + d));
        assert!((response.total_covariances[i].values[0].re / expected - 1.0).abs() < 1e-7);
        let cross = rspice_core::Complex64::new(0.0, s / 4.0 * (power(d) - power(2.0 + d)));
        assert!((response.total_covariances[i].values[1] - cross).norm() / expected < 1e-7);
    }
    for output in &response.outputs {
        assert!(matches!(
            output.integrated.as_ref().unwrap().output_rms,
            QpnoiseValue::Finite(_)
        ));
        assert!(output.input_noise.is_some());
        assert_eq!(output.ranking.as_ref().unwrap().len(), 2);
    }
    let retained = crate::simulation::controller::SimulationController::new()
        .convert_to_analysis_result_with_metadata_owned(
            result,
            crate::state::AnalysisType::Qpnoise,
            "QP oscillator noise",
        );
    retained.validate_retained_evidence().unwrap();
    let saved = crate::io::project_io::ProjectAnalysisResult::from(&retained);
    let decoded: crate::io::project_io::ProjectAnalysisResult =
        serde_json::from_str(&serde_json::to_string(&saved).unwrap()).unwrap();
    assert_eq!(decoded, saved);
}

#[test]
fn qpnoise_result_dependency_dispatch_requires_exact_retained_qpss_state() {
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
    use crate::simulation::execution::{ExecutionArtifactEnvelope, PreparedDependencyBinding};
    let deck = "Noise dispatch\nV1 in 0 DC 1\nRs in out 1k\nRl out 0 2k\n.end\n";
    let producer = crate::simulation::plan::QpssDraft {
        tones: "1000,1414.2135623730951".into(),
        harmonics: "1,1".into(),
        ..Default::default()
    }
    .to_spec()
    .unwrap();
    let point = svc_runner::run_qpss_analysis_with_source_path_and_abort(
        deck,
        producer.driven_qpss_config().unwrap(),
        None,
        &rspice_core::NoAbort,
    )
    .unwrap()
    .operating_point;
    let snapshot = ContentDigest::from_bytes([51; 32]);
    let binding = PreparedDependencyBinding::qpss_state(
        AnalysisInstanceId::new(),
        ObjectRevision::new(1).unwrap(),
        ContentDigest::from_bytes([52; 32]),
    );
    let artifact = ExecutionArtifactEnvelope::from_qpss_result(
        snapshot,
        binding.producer_instance_id(),
        binding.producer_source_revision(),
        binding.producer_config_digest(),
        &producer,
        &SimulationResult::from_qpss_operating_point(point.clone()).unwrap(),
    )
    .unwrap()
    .unwrap();
    let dependencies = ResolvedExecutionDependencies::resolve(
        snapshot,
        vec![binding.clone()],
        &std::collections::HashMap::from([(binding.producer_instance_id(), artifact)]),
    )
    .unwrap();
    let (metadata, buffers) = dependencies.encode_transfer().unwrap();
    let dependencies = ResolvedExecutionDependencies::decode_transfer(&metadata, buffers).unwrap();
    let spec = crate::simulation::plan::QuasiPeriodicNoiseDraft {
        explicit_frequencies: "100,300,700".into(),
        noise_figure: true,
        source_resistor: "Rs".into(),
        ..Default::default()
    }
    .to_spec()
    .unwrap();
    let run = |deck: &str, dependencies: &ResolvedExecutionDependencies| {
        run_spec_request(
            &EngineBridge::new(),
            spec.clone(),
            SpecExecutionOptions::default(),
            deck,
            None,
            dependencies,
            &rspice_core::NoAbort,
        )
    };
    assert!(run(deck, &ResolvedExecutionDependencies::default()).is_err());
    assert!(run(&deck.replace("Rs in out 1k", "Rs in out 2k"), &dependencies).is_err());
    let SimulationResult::Qpnoise { response, .. } = run(deck, &dependencies).unwrap() else {
        panic!("wrong result")
    };
    assert_eq!(
        response.metadata.operating_point_identity,
        point.retained_identity()
    );
    assert_eq!(
        response.metadata.request.frequencies_hz,
        [100.0, 300.0, 700.0]
    );
    assert!(response.outputs[0].integrated.is_some());
    assert!(response.outputs[0].ranking.is_some());
    assert!(response.outputs[0].noise_figure_db.is_some());
}
