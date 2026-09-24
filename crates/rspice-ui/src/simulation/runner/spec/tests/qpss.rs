//! Verify every driven QPSS control through draft, worker and engine boundaries.
use super::*;
use crate::simulation::plan::QpssDraft;
use crate::simulation::runner::worker_contract::WorkerAnalysisSpec;
use rspice_core::analysis::quasi_periodic::QuasiPeriodicSampling;
use rspice_core::engine::QpssInitialState;

fn authored() -> QpssDraft {
    QpssDraft {
        tones: "1k, 1414.213562373095".into(),
        harmonics: "1, 1".into(),
        max_iterations: "19".into(),
        relative_tolerance: "1e-8".into(),
        current_absolute_tolerance: "2e-13".into(),
        voltage_absolute_tolerance: "3e-10".into(),
        max_backtracks: "7".into(),
        max_mixing_order: "1".into(),
        collocation_points: "8, 16".into(),
        source_tones: "V1=1; I1=2".into(),
        linear_method: rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod::Krylov,
        krylov_restart: "16".into(),
        krylov_cycles: "12".into(),
        linear_tolerance: "2e-11".into(),
        dc_initialization: true,
        ..Default::default()
    }
}

#[test]
fn autonomous_qpac_qpxf_studio_dependency_worker_and_save_preserve_phase_response() {
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
    use crate::simulation::execution::{ExecutionArtifactEnvelope, PreparedDependencyBinding};
    use crate::simulation::plan::{QuasiPeriodicAcDraft, QuasiPeriodicTransferDraft};
    use crate::simulation::runner::worker_contract::round_trip_response_for_test;
    use rspice_core::engine::QpxfFrequencyAxis;
    use std::f64::consts::{SQRT_2, TAU};
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
    let point = svc_runner::run_qpss_analysis_with_source_path_and_abort(
        deck,
        producer.qpss_config().unwrap(),
        None,
        &rspice_core::NoAbort,
    )
    .unwrap()
    .operating_point;
    let snapshot = ContentDigest::from_bytes([81; 32]);
    let binding = PreparedDependencyBinding::qpss_state(
        AnalysisInstanceId::new(),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([82; 32]),
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
    let mut results = vec![];
    for spec in [ac, xf] {
        let worker = WorkerAnalysisSpec::try_from(&spec).unwrap();
        let restored: AnalysisSpec =
            serde_json::from_str::<WorkerAnalysisSpec>(&serde_json::to_string(&worker).unwrap())
                .unwrap()
                .into();
        assert_eq!(spec, restored);
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
        results.push(round_trip_response_for_test(result));
    }
    let SimulationResult::Qpac { response: ac, .. } = &results[0] else {
        panic!("QPAC expected")
    };
    let SimulationResult::Qpxf { response: xf, .. } = &results[1] else {
        panic!("QPXF expected")
    };
    assert_eq!(
        ac.metadata.operating_point_identity,
        point.retained_identity()
    );
    assert_eq!(
        xf.metadata.operating_point_identity,
        point.retained_identity()
    );
    for (forward, adjoint) in ac.output_transfer.iter().zip(&xf.transfers[0].values) {
        assert!((*forward - *adjoint).norm() / forward.norm().max(1.0) < 1e-7);
    }
    assert!(
        ac.output_transfer[1].norm() > 1e18,
        "phase response must retain the near-carrier pole"
    );
    assert!(xf.transfers[0].group_delay.is_some());
    for (result, kind) in results.into_iter().zip([
        crate::state::AnalysisType::Qpac,
        crate::state::AnalysisType::Qpxf,
    ]) {
        let retained = crate::simulation::controller::SimulationController::new()
            .convert_to_analysis_result_with_metadata_owned(result, kind, "QP response");
        retained.validate_retained_evidence().unwrap();
        let saved = crate::io::project_io::ProjectAnalysisResult::from(&retained);
        let decoded: crate::io::project_io::ProjectAnalysisResult =
            serde_json::from_str(&serde_json::to_string(&saved).unwrap()).unwrap();
        assert_eq!(decoded, saved);
    }
    let zero = QuasiPeriodicAcDraft {
        explicit_offsets: "0".into(),
        input_source: "Iprobe".into(),
        output_node: "x".into(),
        input_lattice: "1,0".into(),
        output_lattice: "1,0".into(),
        ..Default::default()
    };
    let error = run_spec_request(
        &EngineBridge::new(),
        zero.to_spec().unwrap(),
        Default::default(),
        deck,
        None,
        &deps,
        &rspice_core::NoAbort,
    )
    .unwrap_err();
    assert!(error.to_string().contains("nonzero"));
    let mut xf = QuasiPeriodicTransferDraft {
        explicit_frequencies: format!("{}", point.oscillator_frequency_hz().unwrap() + 0.04),
        input_source: "Iprobe".into(),
        output_node: "x".into(),
        input_lattice: "1,0".into(),
        output_lattice: "1,0".into(),
        ..Default::default()
    };
    let output_axis = run_spec_request(
        &EngineBridge::new(),
        xf.to_spec().unwrap(),
        Default::default(),
        deck,
        None,
        &deps,
        &rspice_core::NoAbort,
    )
    .unwrap();
    xf.frequency_axis = QpxfFrequencyAxis::Offset;
    xf.explicit_frequencies = "0.04".into();
    let offset_axis = run_spec_request(
        &EngineBridge::new(),
        xf.to_spec().unwrap(),
        Default::default(),
        deck,
        None,
        &deps,
        &rspice_core::NoAbort,
    )
    .unwrap();
    let (SimulationResult::Qpxf { response: a, .. }, SimulationResult::Qpxf { response: b, .. }) =
        (output_axis, offset_axis)
    else {
        panic!("QPXF expected")
    };
    assert!((a.transfers[0].values[0] - b.transfers[0].values[0]).norm() < 1e-9);
}

#[test]
fn autonomous_qpss_studio_controls_reach_worker_engine_and_saved_results() {
    use crate::simulation::runner::worker_contract::round_trip_response_for_test;
    use std::f64::consts::{SQRT_2, TAU};
    let mut draft = QpssDraft {
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
    let persisted: QpssDraft = ron::from_str(&ron::to_string(&draft).unwrap()).unwrap();
    let spec = persisted.to_spec().unwrap();
    let worker = WorkerAnalysisSpec::try_from(&spec).unwrap();
    let restored: AnalysisSpec =
        serde_json::from_str::<WorkerAnalysisSpec>(&serde_json::to_string(&worker).unwrap())
            .unwrap()
            .into();
    assert_eq!(restored, spec);
    let config = restored.qpss_config().unwrap();
    assert_eq!(
        config.oscillator.as_ref().unwrap().additional_seeds[0].phase_degrees,
        -90.0
    );
    assert_eq!(
        AnalysisSpec::from_qpss_config(config.clone())
            .qpss_config()
            .unwrap(),
        config
    );
    let deck = format!(
        "QPSS oscillator\nCx x 0 1\nCy y 0 1\nBx 0 x I={{(1-v(x)^2-v(y)^2)*v(x)-(1+.1*sqrt(2)*cos(sqrt(2)*time))*v(y)}}\nBy 0 y I={{(1-v(x)^2-v(y)^2)*v(y)+(1+.1*sqrt(2)*cos(sqrt(2)*time))*v(x)}}\n{}\n.end\n",
        config.to_spice().unwrap()
    );
    let dependencies = op_dependencies(&deck, &deck, &deck, Default::default());
    let result = run_spec_request(
        &EngineBridge::new(),
        restored,
        SpecExecutionOptions::default(),
        &deck,
        None,
        &dependencies,
        &rspice_core::NoAbort,
    )
    .unwrap();
    let measurement = result
        .study_measurement("scalar:qpss.oscillator_frequency_hz")
        .unwrap();
    assert!((measurement.value.unwrap() - 1.0 / TAU).abs() < 1e-8);
    let restored = round_trip_response_for_test(result);
    let SimulationResult::Qpss {
        operating_point,
        frequencies,
        tuples,
        ..
    } = &restored
    else {
        panic!("QPSS result expected")
    };
    assert_eq!(operating_point.config(), &config);
    let solved = operating_point.oscillator_frequency_hz().unwrap();
    let index = tuples.iter().position(|tuple| tuple == &[1, 0]).unwrap();
    assert_eq!(frequencies[index], solved);
    let retained = crate::simulation::controller::SimulationController::new()
        .convert_to_analysis_result_with_metadata_owned(
            restored,
            crate::state::AnalysisType::Qpss,
            "QPSS",
        );
    retained.validate_retained_evidence().unwrap();
    let saved = crate::io::project_io::ProjectAnalysisResult::from(&retained);
    let decoded: crate::io::project_io::ProjectAnalysisResult =
        serde_json::from_str(&serde_json::to_string(&saved).unwrap()).unwrap();
    assert_eq!(decoded, saved);
    // Inactive raw buffers can be unfinished; activating them must validate.
    draft.autonomous = false;
    draft.oscillator_tone = "unfinished".into();
    assert!(
        draft
            .to_spec()
            .unwrap()
            .qpss_config()
            .unwrap()
            .oscillator
            .is_none()
    );
    draft.autonomous = true;
    assert!(draft.to_spec().is_err());
}

#[test]
fn qpss_controls_survive_draft_worker_and_real_engine_execution() {
    let draft = authored();
    let draft: QpssDraft = ron::from_str(&ron::to_string(&draft).unwrap()).unwrap();
    let spec = draft.to_spec().unwrap();
    let worker = WorkerAnalysisSpec::try_from(&spec).unwrap();
    let restored: AnalysisSpec =
        serde_json::from_str::<WorkerAnalysisSpec>(&serde_json::to_string(&worker).unwrap())
            .unwrap()
            .into();
    assert_eq!(spec, restored);
    let config = restored.driven_qpss_config().unwrap();
    assert_eq!(
        config.grid.sampling,
        QuasiPeriodicSampling::Exact(vec![8, 16])
    );
    assert_eq!(config.grid.max_mixing_order, Some(1));
    assert_eq!(config.solver.max_iterations, 19);
    assert_eq!(config.solver.relative_tolerance, 1e-8);
    assert_eq!(config.solver.current_absolute_tolerance, 2e-13);
    assert_eq!(config.solver.voltage_absolute_tolerance, 3e-10);
    assert_eq!(config.solver.max_backtracks, 7);
    assert_eq!(
        config.solver.linear.method,
        rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod::Krylov
    );
    assert_eq!(config.solver.linear.restart, 16);
    assert_eq!(config.solver.linear.max_cycles, 12);
    assert_eq!(config.solver.linear.relative_tolerance, 2e-11);
    assert_eq!(config.initial_state, QpssInitialState::DcOperatingPoint);
    assert_eq!(config.source_tones[0].source, "V1");
    assert_eq!(config.source_tones[1].tone, 1);
    let netlist = "QPSS card controls\nV1 input 0 DC .1 AC .2 30\nR1 input out 1k\nR2 out 0 1k\nC1 out 0 1u\nI1 0 out DC 0 AC .001 -20\n.end\n";
    let directive = config.to_spice().unwrap();
    let executable = netlist.replace(".end", &format!("{directive}\n.end"));
    let dependencies = op_dependencies(&executable, &executable, &executable, Default::default());
    let result = run_spec_request(
        &EngineBridge::new(),
        restored,
        SpecExecutionOptions::default(),
        &executable,
        None,
        &dependencies,
        &rspice_core::NoAbort,
    )
    .unwrap();
    let SimulationResult::Qpss {
        operating_point, ..
    } = result
    else {
        panic!("QPSS dispatch returned a different result family")
    };
    let data = crate::services::simulation_runner::qpss_data_from_operating_point_with_abort(
        operating_point,
        &rspice_core::NoAbort,
    )
    .unwrap();
    assert_eq!(data.operating_point.config(), &config);
    assert_eq!(data.tuples.len(), 3);
    assert!(data.frequencies.windows(2).all(|pair| pair[0] < pair[1]));
    let output = &data
        .spectra
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("V(out)"))
        .unwrap()
        .1;
    for (tuple, drive) in [
        (
            [1, 0],
            rspice_core::Complex64::from_polar(0.2 / 1e3, 30.0_f64.to_radians()),
        ),
        (
            [0, 1],
            rspice_core::Complex64::from_polar(0.001, -20.0_f64.to_radians()),
        ),
    ] {
        let index = data
            .tuples
            .iter()
            .position(|value| value.as_slice() == tuple)
            .unwrap();
        let admittance = rspice_core::Complex64::new(
            0.002,
            std::f64::consts::TAU * data.frequencies[index] * 1e-6,
        );
        assert!((output[index] - drive / admittance).norm() < 1e-9);
    }
    assert!(
        data.spectra
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case("I(V1)"))
    );
}

#[test]
fn qpss_controls_validate_inactive_and_legacy_fields() {
    let mut default = QpssDraft::default();
    default.harmonics = "1, 1".into();
    let legacy = default.to_spec().unwrap();
    let mut json = serde_json::to_value(&legacy).unwrap();
    json["Qpss"].as_object_mut().unwrap().remove("controls");
    let restored: AnalysisSpec = serde_json::from_value(json).unwrap();
    assert_eq!(legacy, restored);
    let legacy_draft: QpssDraft = serde_json::from_value(serde_json::json!({
        "tones":"1G, 1.001G", "harmonics":"1, 1", "max_iterations":"100", "relative_tolerance":"1e-6", "autonomous":false,"oscillator_node":""
    })).unwrap();
    assert_eq!(legacy_draft.to_spec().unwrap(), legacy);
    let mut duplicate = default.clone();
    duplicate.tones = "1G, 1G".into();
    assert!(duplicate.to_spec().is_err());
    default.collocation_points = "2, 2".into();
    assert!(default.to_spec().unwrap_err().contains("2H+1"));
    default.collocation_points.clear();
    default.max_backtracks = "0".into();
    assert!(default.to_spec().is_ok());
    default.source_tones = "V1=1,V1=1".into();
    assert!(default.to_spec().is_err());
    let mut autonomous = authored();
    autonomous.source_tones.clear();
    autonomous.autonomous = true;
    autonomous.oscillator_node = "out".into();
    assert!(
        autonomous
            .to_spec()
            .unwrap()
            .driven_qpss_config()
            .unwrap_err()
            .contains("autonomous")
    );
}

#[test]
fn qpss_controls_direct_mode_retains_inactive_krylov_draft_buffers() {
    use rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod as Method;
    let mut draft = authored();
    draft.linear_method = Method::Direct;
    draft.krylov_restart = "unfinished".into();
    draft.krylov_cycles = "unfinished".into();
    draft.linear_tolerance = "unfinished".into();
    let config = draft.to_spec().unwrap().driven_qpss_config().unwrap();
    assert_eq!(config.solver.linear.method, Method::Direct);
    assert_eq!(draft.krylov_restart, "unfinished");
    draft.linear_method = Method::Krylov;
    assert!(draft.to_spec().is_err());
}

pub(super) fn op_dependencies(
    basis: &str,
    op_deck: &str,
    consumer_deck: &str,
    config: crate::simulation::dialog::OpConfig,
) -> ResolvedExecutionDependencies {
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
    use crate::simulation::execution::{ExecutionArtifactEnvelope, PreparedDependencyBinding};
    let snapshot = ContentDigest::from_bytes([91; 32]);
    let binding = PreparedDependencyBinding::dc_operating_point_seed(
        AnalysisInstanceId::new(),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([92; 32]),
    );
    let result = EngineBridge::new()
        .run(
            &crate::simulation::AnalysisConfig::DcOp(config.clone()),
            op_deck,
        )
        .unwrap();
    let source = crate::state::content_digest(basis);
    let artifact = ExecutionArtifactEnvelope::from_dc_operating_point_result(
        snapshot,
        binding.producer_instance_id(),
        binding.producer_source_revision(),
        binding.producer_config_digest(),
        source,
        &config,
        &result,
    )
    .unwrap()
    .unwrap();
    let mut dependencies = ResolvedExecutionDependencies::resolve(
        snapshot,
        vec![binding.clone()],
        &HashMap::from([(binding.producer_instance_id(), artifact)]),
    )
    .unwrap();
    dependencies.bind_source(consumer_deck, source);
    let (metadata, buffers) = dependencies.encode_transfer().unwrap();
    ResolvedExecutionDependencies::decode_transfer(&metadata, buffers).unwrap()
}

#[test]
fn qpss_op_handoff_preserves_environment_and_consumers_across_worker_transport() {
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
    use crate::simulation::dialog::{OpConfig, OpTemperatureMode};
    use crate::simulation::execution::{ExecutionArtifactEnvelope, PreparedDependencyBinding};
    use crate::simulation::plan::{
        QpnoiseSourceSelection, QuasiPeriodicAcDraft, QuasiPeriodicNoiseDraft,
        QuasiPeriodicTransferDraft,
    };
    let basis = "Bound QP OP\nV1 in 0 DC .2 AC .3 30\nI1 0 out DC 0 AC .001 -20\nRS in out {1000+10*(TEMP-37)} TC1=.01\nRL out 0 1k\nC1 out 0 100n\n.options TEMP=12 TNOM=27 GMIN=1e-10\n.end\n";
    let op_deck =
        svc_runner::splice_before_terminal_end_card(basis, ".options GMIN=1e-7 RELTOL=1e-8");
    let qp_deck = svc_runner::splice_before_terminal_end_card(basis, ".options GMIN=0 TEMP=12");
    let consumer_deck =
        svc_runner::splice_before_terminal_end_card(basis, ".options GMIN=0 TEMP=77");
    let mut op = OpConfig {
        temperature_celsius: 37.0,
        temperature_mode: OpTemperatureMode::Explicit,
        ..Default::default()
    };
    op.run_point.supply_voltage = Some(2.0);
    op.run_point.nominal_supply_voltage = Some(1.0);
    op.run_point.supply_source_names = vec!["V1".into()];
    let dependencies = op_dependencies(basis, &op_deck, &qp_deck, op);
    let environment = dependencies
        .dc_operating_point_seed()
        .unwrap()
        .environment();
    let run = |spec: AnalysisSpec, deck: &str, deps: &ResolvedExecutionDependencies| {
        run_spec_request(
            &EngineBridge::new(),
            spec,
            Default::default(),
            deck,
            None,
            deps,
            &rspice_core::NoAbort,
        )
    };
    let producer = authored().to_spec().unwrap();
    dependencies
        .validate_for_spec(&producer, &Default::default())
        .unwrap();
    assert!(run(producer.clone(), &qp_deck, &Default::default()).is_err());
    assert!(
        run(
            producer.clone(),
            &qp_deck.replace("RS in out", "RS in 0"),
            &dependencies
        )
        .is_err()
    );
    let mut wrong_basis = dependencies.clone();
    wrong_basis.bind_source(&qp_deck, ContentDigest::from_bytes([99; 32]));
    assert!(run(producer.clone(), &qp_deck, &wrong_basis).is_err());
    let result = run(producer.clone(), &qp_deck, &dependencies).unwrap();
    let dc = result
        .study_measurement("tuple:0,0:real:V(out)")
        .unwrap()
        .value
        .unwrap();
    assert!(
        (dc - 0.4 / 2.1).abs() < 1e-9,
        "OP temperature and explicit supply must reach QPSS"
    );
    let current = result
        .study_measurement("tuple:0,0:real:I(V1)")
        .unwrap()
        .value
        .unwrap();
    assert!((current + 0.4 / 2100.0).abs() < 1e-11);
    let mut zero = producer.clone();
    let AnalysisSpec::Qpss { controls, .. } = &mut zero else {
        unreachable!()
    };
    controls.initial_state = QpssInitialState::Zero;
    let zero = run(zero, &qp_deck, &dependencies).unwrap();
    assert!(
        (zero
            .study_measurement("tuple:0,0:real:V(out)")
            .unwrap()
            .value
            .unwrap()
            - dc)
            .abs()
            < 1e-9
    );

    let snapshot = ContentDigest::from_bytes([91; 32]);
    let binding = PreparedDependencyBinding::qpss_state(
        AnalysisInstanceId::new(),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([93; 32]),
    );
    let artifact = ExecutionArtifactEnvelope::from_qpss_result_with_environment(
        snapshot,
        binding.producer_instance_id(),
        binding.producer_source_revision(),
        binding.producer_config_digest(),
        &producer,
        &result,
        Some(environment),
    )
    .unwrap()
    .unwrap();
    let mut consumers = ResolvedExecutionDependencies::resolve(
        snapshot,
        vec![binding.clone()],
        &HashMap::from([(binding.producer_instance_id(), artifact)]),
    )
    .unwrap();
    consumers.bind_source(&consumer_deck, crate::state::content_digest(basis));
    let (metadata, buffers) = consumers.encode_transfer().unwrap();
    let changed = metadata.replace(
        "\"temperature_celsius\":37.0",
        "\"temperature_celsius\":47.0",
    );
    assert_ne!(changed, metadata);
    assert!(ResolvedExecutionDependencies::decode_transfer(&changed, buffers.clone()).is_err());
    let consumers = ResolvedExecutionDependencies::decode_transfer(&metadata, buffers).unwrap();
    let h = rspice_core::Complex64::new(1.0 / 1100.0, 0.0)
        / rspice_core::Complex64::new(1.0 / 1100.0 + 0.001, std::f64::consts::TAU * 100.0 * 1e-7);
    let qp = QuasiPeriodicAcDraft {
        explicit_offsets: "100,300,700".into(),
        input_source: "V1".into(),
        magnitude: "2".into(),
        phase_degrees: "73".into(),
        ..Default::default()
    }
    .to_spec()
    .unwrap();
    let xf = QuasiPeriodicTransferDraft {
        explicit_frequencies: "100,300,700".into(),
        input_source: "V1".into(),
        ..Default::default()
    }
    .to_spec()
    .unwrap();
    let noise = QuasiPeriodicNoiseDraft {
        explicit_frequencies: "100,300,700".into(),
        source_selection: QpnoiseSourceSelection::Only,
        source_names: "RS thermal".into(),
        ..Default::default()
    }
    .to_spec()
    .unwrap();
    for (spec, observation, expected) in [
        (
            qp,
            "bin:0:real:V(out,0) [k=[0, 0]]",
            (h * rspice_core::Complex64::from_polar(2.0, 73.0_f64.to_radians())).re,
        ),
        (
            xf,
            "bin:0:real:H(V(out,0)/V(V1)) [in=[0, 0]; out=[0, 0]]",
            h.re,
        ),
        (
            noise,
            "bin:0:real:PSD(output 1: V(out,0) [0, 0])",
            4.0 * 1.380649e-23 * 310.15 * 1100.0 * h.norm_sqr(),
        ),
    ] {
        let result = run(spec, &consumer_deck, &consumers).unwrap();
        let actual = result
            .study_measurement(observation)
            .unwrap()
            .value
            .unwrap();
        assert!(
            (actual - expected).abs() < expected.abs() * 1e-7,
            "{observation}: {actual} != {expected}"
        );
    }
}
