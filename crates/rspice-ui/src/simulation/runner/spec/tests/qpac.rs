//! QPAC editor, worker and service boundaries preserve the same physical request.
use super::*;
use crate::simulation::plan::{QpssDraft, QuasiPeriodicAcDraft};
use crate::simulation::runner::worker_contract::WorkerAnalysisSpec;
use rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod as Method;

fn authored() -> QuasiPeriodicAcDraft {
    QuasiPeriodicAcDraft {
        explicit_offsets: "0, 37, 127".into(),
        input_lattice: "0, 1, -1".into(),
        output_lattice: "0, 1, -1".into(),
        magnitude: ".2".into(),
        phase_degrees: "73".into(),
        linear_method: Method::Krylov,
        krylov_restart: "16".into(),
        krylov_cycles: "12".into(),
        linear_tolerance: "2e-11".into(),
        current_absolute_tolerance: "3e-13".into(),
        voltage_absolute_tolerance: "4e-10".into(),
        ..Default::default()
    }
}

#[test]
fn qpac_controls_survive_draft_worker_native_card_and_dependency_dispatch() {
    let draft: QuasiPeriodicAcDraft = ron::from_str(&ron::to_string(&authored()).unwrap()).unwrap();
    let spec = draft.to_spec().unwrap();
    let worker = WorkerAnalysisSpec::try_from(&spec).unwrap();
    let restored: AnalysisSpec =
        serde_json::from_str::<WorkerAnalysisSpec>(&serde_json::to_string(&worker).unwrap())
            .unwrap()
            .into();
    assert_eq!(restored, spec);
    let card = restored.qpac_card().unwrap();
    assert_eq!(AnalysisSpec::from_qpac_card(&card).unwrap(), spec);
    let mut producer = QpssDraft::default();
    producer.tones = "1k, 1414.2135623730951, 1732.0508075688772".into();
    producer.harmonics = "1, 1, 1".into();
    producer.relative_tolerance = "3e-8".into();
    let deck = "QPAC Studio\nV1 in 0 DC 1\nR1 in out 1k\nC1 out 0 100n\n.temp 42\n.end\n";
    let point = svc_runner::run_qpss_analysis_with_source_path_and_abort(
        deck,
        producer.to_spec().unwrap().driven_qpss_config().unwrap(),
        None,
        &rspice_core::abort_signal::NoAbort,
    )
    .unwrap()
    .operating_point;
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
    use crate::simulation::execution::{ExecutionArtifactEnvelope, PreparedDependencyBinding};
    let snapshot = ContentDigest::from_bytes([31; 32]);
    let binding = PreparedDependencyBinding::qpss_state(
        AnalysisInstanceId::new(),
        ObjectRevision::new(1).unwrap(),
        ContentDigest::from_bytes([32; 32]),
    );
    let producer_spec = producer.to_spec().unwrap();
    let artifact = ExecutionArtifactEnvelope::from_qpss_result(
        snapshot,
        binding.producer_instance_id(),
        binding.producer_source_revision(),
        binding.producer_config_digest(),
        &producer_spec,
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
    assert!(run(&deck.replace("R1 in out 1k", "R1 in out 2k"), &dependencies).is_err());
    let simulation = run(deck, &dependencies).unwrap();
    let SimulationResult::Qpac {
        response: result, ..
    } = simulation
    else {
        panic!("QPAC returned a different result family")
    };
    assert_eq!(
        result.metadata.operating_point_identity,
        point.retained_identity()
    );
    assert_eq!(result.metadata.request.input_lattice, [0, 1, -1]);
    assert_eq!(result.metadata.request.solver.linear.method, Method::Krylov);
    assert_eq!(result.metadata.request.solver.linear.restart, 16);
    assert_eq!(result.metadata.request.solver.linear.max_cycles, 12);
    assert_eq!(
        result.metadata.request.solver.linear.relative_tolerance,
        2e-11
    );
    assert_eq!(
        result.metadata.request.solver.current_absolute_tolerance,
        3e-13
    );
    assert_eq!(
        result.metadata.request.solver.voltage_absolute_tolerance,
        4e-10
    );
    for (i, &offset) in result.metadata.request.offsets_hz.iter().enumerate() {
        let frequency = offset + 1414.213562373095 - 1732.0508075688772;
        let expected = rspice_core::Complex64::new(1.0, 0.0)
            / rspice_core::Complex64::new(1.0, std::f64::consts::TAU * frequency * 1e-4);
        assert!((result.output_transfer[i] - expected).norm() < 1e-9);
        assert!(
            (result.output_response[i]
                - expected * rspice_core::Complex64::from_polar(0.2, 73.0_f64.to_radians()))
            .norm()
                < 1e-9
        );
    }
}

#[test]
fn qpac_controls_preserve_legacy_defaults_and_ignore_inactive_editor_buffers() {
    let defaults = QuasiPeriodicAcDraft::default();
    let legacy = defaults.to_spec().unwrap();
    let mut json = serde_json::to_value(&legacy).unwrap();
    json["Qpac"].as_object_mut().unwrap().remove("controls");
    assert_eq!(
        serde_json::from_value::<AnalysisSpec>(json).unwrap(),
        legacy
    );
    let legacy_draft: QuasiPeriodicAcDraft = serde_json::from_value(serde_json::json!({"sweep":defaults.sweep,"input_source":"V1","output_node":"out","output_ref":"0","input_lattice":"0, 0","output_lattice":"0, 0"})).unwrap();
    assert_eq!(legacy_draft.to_spec().unwrap(), legacy);
    let mut draft = authored();
    draft.sweep.start = "unfinished".into();
    draft.sweep.stop = "unfinished".into();
    draft.sweep.points = "unfinished".into();
    draft.linear_method = Method::Direct;
    draft.krylov_restart = "unfinished".into();
    draft.krylov_cycles = "unfinished".into();
    assert!(draft.to_spec().is_ok());
    draft.linear_tolerance = "unfinished".into();
    assert!(
        draft.to_spec().is_err(),
        "direct solves still use the authored physical-equation tolerance"
    );
    draft.linear_tolerance = "1e-10".into();
    draft.linear_method = Method::Krylov;
    assert!(draft.to_spec().is_err());
    draft.linear_method = Method::Direct;
    draft.explicit_offsets.clear();
    assert!(draft.to_spec().is_err());
    draft = authored();
    draft.output_lattice = "0,1".into();
    assert!(draft.to_spec().is_err());
}

#[test]
fn qpac_generated_sweep_controls_survive_worker_and_native_resolution() {
    for (mode, start, stop, expected) in [
        (
            0,
            "10",
            "800",
            vec![10.0, 10.0 * 10_f64.sqrt(), 100.0, 100.0 * 10_f64.sqrt()],
        ),
        (2, "-100", "100", vec![-100.0, 100.0]),
    ] {
        let mut draft = QuasiPeriodicAcDraft::default();
        draft.sweep.sweep = mode;
        draft.sweep.start = start.into();
        draft.sweep.stop = stop.into();
        draft.sweep.points = "2".into();
        let spec = draft.to_spec().unwrap();
        let worker = WorkerAnalysisSpec::try_from(&spec).unwrap();
        let restored: AnalysisSpec =
            serde_json::from_str::<WorkerAnalysisSpec>(&serde_json::to_string(&worker).unwrap())
                .unwrap()
                .into();
        let card = restored.qpac_card().unwrap();
        let resolved = rspice_core::engine::QpacRequest::from_qpac_card(&card).unwrap();
        assert_eq!(resolved.offsets_hz.len(), expected.len());
        assert_eq!(
            rspice_core::engine::QpacRequest::validate_qpac_card(
                &card,
                &rspice_core::ResourceLimits::default()
            )
            .unwrap(),
            expected.len()
        );
        for (actual, expected) in resolved.offsets_hz.iter().zip(expected) {
            assert!((actual - expected).abs() < 1e-12 * expected.abs().max(1.0));
        }
    }
}
