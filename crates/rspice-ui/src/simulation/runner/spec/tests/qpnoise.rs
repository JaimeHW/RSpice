//! The runnable Studio analysis consumes the authenticated producer dependency.
use super::*;
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
