use rspice_core::solver::SolverError;
use rspice_core::{
    ResourceKind, ResourceLimitError, SimulationConfigError, SimulationError,
    SimulationErrorCategory, SimulationErrorCode,
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
