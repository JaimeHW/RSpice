//! A successful OP dependency must not reset the periodic consumer's budgets.

use super::*;
use rspice_core::{NoAbort, ResourceLimits};
use rspice_simulation_contract::quasi_periodic_draft::QpssDraft;

const PERIODIC: &str = "Periodic policy\nP1 p1 0 SIN(0 .1 1Meg) PORT=1 Z0=50\nP2 p2 0 SIN(0 0 1Meg) PORT=2 Z0=50\nR1 p1 p2 100\n.end\n";
const QUASI_PERIODIC: &str =
    "QP policy\nV1 in 0 DC 0 AC .1\nI1 0 out DC 0 AC .0001\nR1 in out 1k\nR2 out 0 1k\n.end\n";

#[test]
fn periodic_producers_keep_limits_after_resolving_a_successful_op() {
    let qpss = QpssDraft {
        tones: "1k, 1414.213562373095".into(),
        harmonics: "1, 1".into(),
        collocation_points: "8, 8".into(),
        source_tones: "V1=1; I1=2".into(),
        dc_initialization: true,
        ..Default::default()
    }
    .to_spec()
    .unwrap();
    for (spec, deck) in [
        (pss_producer_spec(), PERIODIC),
        (hb_producer_spec(), PERIODIC),
        (qpss, QUASI_PERIODIC),
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
