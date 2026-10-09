//! A successful OP dependency must not reset the periodic consumer's budgets.

use super::*;
use rspice_core::{NoAbort, ResourceLimits};
use rspice_simulation_contract::quasi_periodic_draft::QpssDraft;

const PERIODIC: &str = "Periodic policy\nP1 p1 0 SIN(0 .1 1Meg) PORT=1 Z0=50\nP2 p2 0 SIN(0 0 1Meg) PORT=2 Z0=50\nR1 p1 p2 100\n.end\n";
const QUASI_PERIODIC: &str =
    "QP policy\nV1 in 0 DC 0 AC .1\nI1 0 out DC 0 AC .0001\nR1 in out 1k\nR2 out 0 1k\n.end\n";

fn qpss_spec() -> AnalysisSpec {
    QpssDraft {
        tones: "1k, 1414.213562373095".into(),
        harmonics: "1, 1".into(),
        collocation_points: "8, 8".into(),
        source_tones: "V1=1; I1=2".into(),
        dc_initialization: true,
        ..Default::default()
    }
    .to_spec()
    .unwrap()
}

#[test]
fn periodic_producers_keep_limits_after_resolving_a_successful_op() {
    for (spec, deck) in [
        (pss_producer_spec(), PERIODIC),
        (hb_producer_spec(), PERIODIC),
        (qpss_spec(), QUASI_PERIODIC),
    ] {
        let dependencies = op_dependencies(deck, deck, deck, Default::default());
        let mut limits = ResourceLimits::default();
        limits.max_matrix_unknowns = 1000;
        let run = |limits| {
            run_spec_request(
                &EngineBridge::new().with_resource_limits(limits),
                spec.clone(),
                Default::default(),
                deck,
                None,
                &dependencies,
                &NoAbort,
            )
        };
        run(limits).expect("periodic solve accepts custom limits");
        for resource in ["matrix_unknowns", "analysis_points", "result_values"] {
            let mut limited = limits;
            match resource {
                "matrix_unknowns" => limited.max_matrix_unknowns = 1,
                "analysis_points" => limited.max_analysis_points = 2,
                _ => limited.max_result_values = 1,
            }
            let error = run(limited).unwrap_err();
            assert!(
                matches!(&error, SimulationError::ResourceLimit { resource: actual, .. } if actual == resource),
                "{spec:?}: {error:?}"
            );
        }
    }
}

#[test]
fn qp_consumers_preserve_limits_after_a_worker_round_trip() {
    use serde_json::json;
    let deck = QUASI_PERIODIC;
    let producer = qpss_spec();
    let dependencies = op_dependencies(deck, deck, deck, Default::default());
    let result = run_spec_request(
        &EngineBridge::new(),
        producer.clone(),
        Default::default(),
        deck,
        None,
        &dependencies,
        &NoAbort,
    )
    .unwrap();
    let snapshot = digest(0x71);
    let binding = PreparedDependencyBinding::qpss_state(
        AnalysisInstanceId::new(),
        ObjectRevision::INITIAL,
        digest(0x72),
    );
    let artifact = ExecutionArtifactEnvelope::from_qpss_result_with_environment(
        snapshot,
        binding.producer_instance_id(),
        binding.producer_source_revision(),
        binding.producer_config_digest(),
        &producer,
        &result,
        Some(
            dependencies
                .dc_operating_point_seed()
                .unwrap()
                .environment(),
        ),
    )
    .unwrap()
    .unwrap();
    let mut dependencies = ResolvedExecutionDependencies::resolve(
        snapshot,
        vec![binding.clone()],
        &HashMap::from([(binding.producer_instance_id(), artifact)]),
    )
    .unwrap();
    dependencies.bind_source(deck, rspice_design::netlist_document::content_digest(deck));
    let (metadata, buffers) =
        crate::runner::worker_contract::copy_dependency_transfer(&dependencies).unwrap();
    let dependencies = ResolvedExecutionDependencies::decode_transfer(&metadata, buffers).unwrap();
    for request in [
        json!({"Qpac": {
            "start_freq": 10.0, "stop_freq": 100.0, "points_per_unit": 3, "sweep": "Linear",
            "input_source": "V1", "output_node": "out", "output_ref": "0",
            "input_lattice": [0, 0], "output_lattice": [0, 0]
        }}),
        json!({"Qpxf": {
            "start_freq": 10.0, "stop_freq": 100.0, "points_per_unit": 3, "sweep": "Linear",
            "input_source": "V1", "output_node": "out", "output_ref": "0",
            "input_lattice": [0, 0], "output_lattice": [0, 0], "group_delay": false
        }}),
        json!({"Qpnoise": {
            "start_freq": 10.0, "stop_freq": 100.0, "points_per_unit": 3, "sweep": "Linear",
            "input_source": "V1", "output_node": "out", "output_ref": "0",
            "lattice_min": [-1, -1], "lattice_max": [1, 1],
            "integrated_noise": true, "contributor_ranking": true
        }}),
    ] {
        let spec: AnalysisSpec = serde_json::from_value(request).unwrap();
        let mut limits = ResourceLimits::default();
        limits.max_matrix_unknowns = 1000;
        let run = |limits| {
            run_spec_request(
                &EngineBridge::new().with_resource_limits(limits),
                spec.clone(),
                Default::default(),
                deck,
                None,
                &dependencies,
                &NoAbort,
            )
        };
        run(limits).expect("consumer accepts custom limits and the transferred orbit");
        for resource in [
            "netlist_bytes",
            "matrix_unknowns",
            "analysis_points",
            "result_values",
        ] {
            let mut limited = limits;
            match resource {
                "netlist_bytes" => limited.max_netlist_bytes = 1,
                "matrix_unknowns" => limited.max_matrix_unknowns = 1,
                "analysis_points" => limited.max_analysis_points = 2,
                _ => limited.max_result_values = 1,
            }
            let error = run(limited).unwrap_err();
            assert!(
                matches!(&error, SimulationError::ResourceLimit { resource: actual, .. } if actual == resource),
                "{spec:?}: {error:?}"
            );
        }
    }
}

#[test]
fn pss_spectrum_checks_its_own_point_and_value_footprint() {
    let dependencies = transferred_pss_dependencies(PERIODIC);
    let mut limits = ResourceLimits::default();
    limits.max_matrix_unknowns = 1000;
    let run = |limits| {
        run_spec_request(
            &EngineBridge::new().with_resource_limits(limits),
            AnalysisSpec::PssSpectrum { num_harmonics: 8 },
            Default::default(),
            PERIODIC,
            None,
            &dependencies,
            &NoAbort,
        )
    };
    run(limits).unwrap();
    for (points, values, resource) in [(2, 1000, "analysis_points"), (1000, 1, "result_values")] {
        let mut limited = limits;
        limited.max_analysis_points = points;
        limited.max_result_values = values;
        let error = run(limited).unwrap_err();
        assert!(
            matches!(&error, SimulationError::ResourceLimit { resource: actual, .. } if actual == resource),
            "{error:?}"
        );
    }
}

/// Exercise policy enforcement on a carrier that was already solved and
/// transferred under a different policy, while callers check its physical oracle.
pub(super) fn run_consumer_with_limit_checks(
    spec: AnalysisSpec,
    options: SpecExecutionOptions,
    deck: &str,
    dependencies: &ResolvedExecutionDependencies,
) -> SimulationResult {
    let mut limits = ResourceLimits::default();
    limits.max_matrix_unknowns = 1000;
    let run = |limits| {
        run_spec_request(
            &EngineBridge::new().with_resource_limits(limits),
            spec.clone(),
            options.clone(),
            deck,
            None,
            dependencies,
            &NoAbort,
        )
    };
    let result = run(limits).expect("consumer accepts the captured custom policy");
    if matches!(spec, AnalysisSpec::Pstb) {
        // This two-mode fixture fits the native eigensolver's reservation,
        // but its complete modes plus six retained plots need 52 values.
        let mut display_limited = limits;
        display_limited.max_result_values = 48;
        assert!(matches!(
            run(display_limited),
            Err(SimulationError::ResourceLimit { resource, requested: 52, limit: 48 })
                if resource == "result_values"
        ));
    }
    for resource in [
        "netlist_bytes",
        "matrix_unknowns",
        "analysis_points",
        "result_values",
    ] {
        let mut limited = limits;
        match resource {
            "netlist_bytes" => limited.max_netlist_bytes = 1,
            "matrix_unknowns" => limited.max_matrix_unknowns = 1,
            "analysis_points" => {
                limited.max_analysis_points = if matches!(spec, AnalysisSpec::Pstb) {
                    1
                } else {
                    2
                };
            }
            _ => limited.max_result_values = 1,
        }
        let error = run(limited).unwrap_err();
        assert!(
            matches!(&error, SimulationError::ResourceLimit { resource: actual, .. } if actual == resource),
            "{spec:?}: expected {resource}, got {error:?}"
        );
    }
    result
}
