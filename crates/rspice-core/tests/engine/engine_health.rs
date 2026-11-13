use rspice_core::{
    AtomicAbort, Engine, ResourceKind, SimulationConfig, SimulationError, SimulationErrorCode,
};

#[test]
fn readiness_probe_exercises_parser_and_solver() {
    let report = Engine::try_new(SimulationConfig::default())
        .expect("valid engine")
        .health_check()
        .expect("default backend is ready");

    assert_eq!(report.element_count, 2);
    assert_eq!(report.node_count, 1);
    assert_eq!(report.branch_count, 1);
    assert!((report.output_voltage - 1.0).abs() <= 1.0e-12);
}

#[test]
fn readiness_probe_honors_cancellation() {
    let abort = AtomicAbort::new();
    abort.set();
    let error = Engine::default()
        .health_check_with_abort(&abort)
        .expect_err("cancelled readiness probe must stop");
    assert!(matches!(error, SimulationError::Aborted));
    assert_eq!(error.descriptor().code, SimulationErrorCode::Aborted);
}

#[test]
fn readiness_probe_honors_the_engine_resource_policy() {
    let mut config = SimulationConfig::default();
    config.resource_limits.max_netlist_bytes = 1;
    let error = Engine::try_new(config)
        .expect("resource policy is a valid configuration")
        .health_check()
        .expect_err("probe must use the configured parser limit");
    let descriptor = error.descriptor();
    let limit = descriptor.resource_limit.expect("numeric resource details");
    assert_eq!(descriptor.code, SimulationErrorCode::ResourceLimit);
    assert_eq!(limit.resource, ResourceKind::NetlistBytes);
    assert!(limit.requested > limit.limit);
}
