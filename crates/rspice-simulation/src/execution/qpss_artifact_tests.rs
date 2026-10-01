//! QPSS service integration with authenticated dependency artifacts.

use crate::execution_artifact::{
    ExecutionArtifactEnvelope, PreparedDependencyBinding, ResolvedExecutionDependencies,
};
use crate::execution_options::SpecExecutionOptions;
use crate::prepared_dependency::validate_prepared_dependency_contract_with_options;
use crate::results::SimulationResult;
use rspice_app_types::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use rspice_simulation_contract::config::FrequencySweep;
use std::collections::HashMap;

fn setup() -> (
    AnalysisSpec,
    SimulationResult,
    ContentDigest,
    PreparedDependencyBinding,
    ExecutionArtifactEnvelope,
) {
    let draft = rspice_simulation_contract::quasi_periodic_draft::QpssDraft {
        tones: "1k,1414.213562373095".into(),
        harmonics: "1,1".into(),
        ..Default::default()
    };
    let spec = draft.to_spec().unwrap();
    let data = crate::engine_services::run_qpss_analysis_with_source_path_and_abort(
        "QPSS artifact\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\nI1 0 out SIN(0 .001 1414.213562373095)\nBmemory memory 0 V=1k*sdt(v(out)-v(memory))\nRmemory memory 0 1k\n.end\n",
        spec.qpss_config().unwrap(), None, &rspice_core::NoAbort,
    ).unwrap();
    let result = SimulationResult::from_qpss_operating_point(data.operating_point).unwrap();
    let snapshot = ContentDigest::from_bytes([19; 32]);
    let binding = PreparedDependencyBinding::qpss_state(
        AnalysisInstanceId::new(),
        ObjectRevision::new(7).unwrap(),
        ContentDigest::from_bytes([20; 32]),
    );
    let artifact = ExecutionArtifactEnvelope::from_qpss_result_with_environment(
        snapshot,
        binding.producer_instance_id(),
        binding.producer_source_revision(),
        binding.producer_config_digest(),
        &spec,
        &result,
        None,
    )
    .unwrap()
    .unwrap();
    (spec, result, snapshot, binding, artifact)
}

fn consumers() -> Vec<AnalysisSpec> {
    vec![
        AnalysisSpec::Qpac {
            start_freq: 10.0,
            stop_freq: 100.0,
            points_per_unit: 3,
            sweep: FrequencySweep::Linear,
            input_source: "V1".into(),
            output_node: "out".into(),
            output_ref: "0".into(),
            input_lattice: vec![0, 0],
            output_lattice: vec![1, -1],
            controls: Default::default(),
        },
        AnalysisSpec::Qpxf {
            start_freq: 10.0,
            stop_freq: 100.0,
            points_per_unit: 3,
            sweep: FrequencySweep::Linear,
            input_source: "V1".into(),
            output_node: "out".into(),
            output_ref: "0".into(),
            input_lattice: vec![0, 0],
            output_lattice: vec![1, -1],
            group_delay: true,
            controls: Default::default(),
        },
        AnalysisSpec::Qpnoise {
            start_freq: 10.0,
            stop_freq: 100.0,
            points_per_unit: 3,
            sweep: FrequencySweep::Linear,
            input_source: "V1".into(),
            output_node: "out".into(),
            output_ref: "0".into(),
            lattice_min: vec![-1, -1],
            lattice_max: vec![1, 1],
            integrated_noise: true,
            contributor_ranking: true,
            controls: Default::default(),
        },
    ]
}

#[test]
fn qpss_artifact_binds_and_transfers_exact_independent_tone_state() {
    let (spec, _, snapshot, binding, artifact) = setup();
    let resolved = ResolvedExecutionDependencies::resolve(
        snapshot,
        vec![binding.clone()],
        &HashMap::from([(binding.producer_instance_id(), artifact)]),
    )
    .unwrap();
    for consumer in consumers() {
        validate_prepared_dependency_contract_with_options(
            &consumer,
            &SpecExecutionOptions::default(),
            &spec,
        )
        .unwrap();
        resolved
            .validate_for_spec(&consumer, &SpecExecutionOptions::default())
            .unwrap();
        assert!(
            validate_prepared_dependency_contract_with_options(
                &consumer,
                &SpecExecutionOptions::default(),
                &AnalysisSpec::LegacyDcOp
            )
            .is_err()
        );
        let mut autonomous = spec.clone();
        if let AnalysisSpec::Qpss { autonomous, .. } = &mut autonomous {
            *autonomous = true;
        }
        assert!(
            validate_prepared_dependency_contract_with_options(
                &consumer,
                &SpecExecutionOptions::default(),
                &autonomous
            )
            .is_err()
        );
    }
    assert!(resolved.hb_state().is_err());
    let point = resolved.qpss_state().unwrap().operating_point();
    assert_eq!(point.integral_names(), ["B:BMEMORY:sdt:0"]);
    assert_eq!(
        serde_json::to_value(resolved.qpss_state().unwrap()).unwrap()["spectral_real"]
            .as_array()
            .unwrap()
            .len(),
        point.complete_spectra().len()
    );
    let (metadata, buffers) =
        crate::runner::worker_contract::copy_dependency_transfer(&resolved).unwrap();
    let restored = ResolvedExecutionDependencies::decode_transfer(&metadata, buffers).unwrap();
    assert_eq!(
        restored.qpss_state().unwrap().operating_point(),
        resolved.qpss_state().unwrap().operating_point()
    );
    let json = serde_json::to_string(&resolved).unwrap();
    let native: ResolvedExecutionDependencies = serde_json::from_str(&json).unwrap();
    native
        .validate_for_spec(&consumers()[0], &SpecExecutionOptions::default())
        .unwrap();
    let (metadata, mut buffers) =
        crate::runner::worker_contract::copy_dependency_transfer(&resolved).unwrap();
    // The last imaginary row belongs to the integral, not a displayed trace.
    buffers.last_mut().unwrap()[0] += 1e-6;
    assert!(ResolvedExecutionDependencies::decode_transfer(&metadata, buffers).is_err());
}

#[test]
fn qpss_artifact_rejects_changed_producer_and_transferred_evidence() {
    let (mut spec, result, snapshot, binding, artifact) = setup();
    if let AnalysisSpec::Qpss { controls, .. } = &mut spec {
        controls.max_backtracks += 1;
    }
    assert!(
        ExecutionArtifactEnvelope::from_qpss_result_with_environment(
            snapshot,
            binding.producer_instance_id(),
            binding.producer_source_revision(),
            binding.producer_config_digest(),
            &spec,
            &result,
            None
        )
        .is_err()
    );
    let artifacts = HashMap::from([(binding.producer_instance_id(), artifact)]);
    assert!(
        ResolvedExecutionDependencies::resolve(
            ContentDigest::from_bytes([99; 32]),
            vec![binding.clone()],
            &artifacts
        )
        .is_err()
    );
    let resolved =
        ResolvedExecutionDependencies::resolve(snapshot, vec![binding], &artifacts).unwrap();
    let (metadata, mut buffers) =
        crate::runner::worker_contract::copy_dependency_transfer(&resolved).unwrap();
    buffers[0][0] += 1.0;
    assert!(ResolvedExecutionDependencies::decode_transfer(&metadata, buffers).is_err());
    let (metadata, buffers) =
        crate::runner::worker_contract::copy_dependency_transfer(&resolved).unwrap();
    let mut metadata: serde_json::Value = serde_json::from_str(&metadata).unwrap();
    metadata["artifacts"][0]["payload"]["QpssState"]["point"]["config"]["solver"]["relative_tolerance"] =
        serde_json::json!(0.01);
    assert!(
        ResolvedExecutionDependencies::decode_transfer(
            &serde_json::to_string(&metadata).unwrap(),
            buffers
        )
        .is_err()
    );
    let (metadata, buffers) =
        crate::runner::worker_contract::copy_dependency_transfer(&resolved).unwrap();
    let mut metadata: serde_json::Value = serde_json::from_str(&metadata).unwrap();
    metadata["artifacts"][0]["payload"]["QpssState"]["real"][1]["buffer"] = serde_json::json!(0);
    assert!(
        ResolvedExecutionDependencies::decode_transfer(
            &serde_json::to_string(&metadata).unwrap(),
            buffers
        )
        .is_err()
    );
}
