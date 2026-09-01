use rspice_core::solver::SolverError;
use rspice_core::{
    ResourceKind, ResourceLimitError, ResultSchemaMismatchError, SimulationConfigError,
    SimulationError, SimulationErrorCategory, SimulationErrorCode,
};

#[test]
fn resource_failures_publish_stable_numeric_metadata() {
    let limit = ResourceLimitError {
        resource: ResourceKind::AnalysisPoints,
        requested: 101,
        limit: 100,
    };

    for error in [
        SimulationError::ResourceLimit(limit),
        SimulationError::Configuration(SimulationConfigError::ResourceLimit(limit)),
    ] {
        let descriptor = error.descriptor();
        assert_eq!(descriptor.code, SimulationErrorCode::ResourceLimit);
        assert_eq!(descriptor.code.as_str(), "resource_limit");
        assert_eq!(descriptor.category, SimulationErrorCategory::ResourceLimit);
        assert!(!descriptor.retryable);
        assert_eq!(descriptor.resource_limit, Some(limit));
        assert_eq!(descriptor.iterations, None);
    }
}

#[test]
fn convergence_metadata_covers_engine_and_nested_solver_failures() {
    let direct = SimulationError::ConvergenceFailed(17).descriptor();
    assert_eq!(direct.code, SimulationErrorCode::ConvergenceError);
    assert_eq!(direct.category, SimulationErrorCategory::Convergence);
    assert_eq!(direct.iterations, Some(17));
    assert!(!direct.retryable);

    let nested = SimulationError::Solver(SolverError::ConvergenceFailed(23)).descriptor();
    assert_eq!(nested.code, SimulationErrorCode::SolverError);
    assert_eq!(nested.category, SimulationErrorCategory::Solver);
    assert_eq!(nested.iterations, Some(23));
}

#[test]
fn cancellation_is_the_only_automatically_retryable_failure() {
    let descriptor = SimulationError::Aborted.descriptor();
    assert_eq!(descriptor.code, SimulationErrorCode::Aborted);
    assert_eq!(descriptor.category, SimulationErrorCategory::Cancellation);
    assert!(descriptor.retryable);
    assert_eq!(descriptor.iterations, None);
    assert_eq!(descriptor.resource_limit, None);
}

#[test]
fn result_schema_mismatch_is_a_typed_non_retryable_output_failure() {
    let expected_names = vec!["0".to_string(), "out".to_string()];
    let actual_names = vec!["out".to_string(), "0".to_string()];
    let error = SimulationError::result_schema_mismatch(
        "AC",
        Some("frequency point 7 (1.0000000000000000e+6 Hz)".to_string()),
        "node voltages",
        expected_names.clone(),
        actual_names.clone(),
        2,
        1,
    );

    let descriptor = error.descriptor();
    assert_eq!(descriptor.code, SimulationErrorCode::ResultSchemaMismatch);
    assert_eq!(descriptor.code.as_str(), "result_schema_mismatch");
    assert_eq!(descriptor.category, SimulationErrorCategory::Output);
    assert!(!descriptor.retryable);
    assert_eq!(descriptor.iterations, None);
    assert_eq!(descriptor.resource_limit, None);
    assert_eq!(
        error.to_string(),
        "result schema mismatch for AC analysis at frequency point 7 (1.0000000000000000e+6 Hz) in node voltages: expected names [\"0\", \"out\"] with 2 value(s), got names [\"out\", \"0\"] with 1 value(s)"
    );

    let SimulationError::ResultSchemaMismatch(detail) = error else {
        panic!("typed result-schema variant was lost");
    };
    assert_eq!(
        *detail,
        ResultSchemaMismatchError {
            analysis: "AC".to_string(),
            coordinate: Some("frequency point 7 (1.0000000000000000e+6 Hz)".to_string()),
            signal_family: "node voltages".to_string(),
            expected_names,
            actual_names,
            expected_value_count: 2,
            actual_value_count: 1,
        }
    );
}

#[test]
fn schema_mismatch_without_a_coordinate_preserves_empty_registries() {
    let detail = ResultSchemaMismatchError::new(
        "TRAN",
        None,
        "branch currents",
        Vec::new(),
        vec!["V1".to_string()],
        0,
        1,
    );
    assert_eq!(
        detail.to_string(),
        "result schema mismatch for TRAN analysis in branch currents: expected names [] with 0 value(s), got names [\"V1\"] with 1 value(s)"
    );
    assert_eq!(detail.coordinate, None);
}
