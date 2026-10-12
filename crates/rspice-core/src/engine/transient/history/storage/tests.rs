use super::*;
use crate::{SimulationConfig, SimulationErrorCategory, SimulationErrorCode};

mod initialization;

fn fail_reservation_after<T>(count: usize, operation: impl FnOnce() -> T) -> T {
    BjtTransientHistory::with_allocation_failure_after(count, operation)
}

fn assert_allocation(error: &SimulationError) {
    assert!(
        matches!(error, SimulationError::Allocation { .. }),
        "{error}"
    );
    let descriptor = error.descriptor();
    assert_eq!(descriptor.code, SimulationErrorCode::AllocationFailed);
    assert_eq!(descriptor.code.as_str(), "allocation_failed");
    assert_eq!(descriptor.category, SimulationErrorCategory::ResourceLimit);
    assert!(descriptor.resource_limit.is_none());
    assert!(!descriptor.retryable);
    assert!(std::error::Error::source(error).is_some());
}

#[test]
fn impossible_history_reservation_keeps_the_allocator_failure_typed() {
    let error = reserve_values::<u8>(usize::MAX, "BJT test history").unwrap_err();
    assert_allocation(&error);
}

#[test]
fn public_capture_and_restore_preserve_allocation_failures_and_accepted_state() {
    let netlist = Netlist::parse("phase copy failures\nVC c 0 2\nVB b 0 .7\nQ1 c b 0 qm\n.model qm NPN IS=1e-16 BF=100 TF=1n PTF=90\n.end\n").unwrap();
    let engine = Engine::new(SimulationConfig::default());
    // Count the startup reservations through the same public run route, so each
    // capture fault lands in final checkpoint materialization rather than
    // accidentally testing an earlier physical-startup copy.
    let (baseline, startup_reservations) = fail_reservation_after(usize::MAX, || {
        let result = engine.run_tran(&netlist, 1e-10, 1e-12).unwrap();
        (
            result,
            usize::MAX - HISTORY_RESERVATIONS_BEFORE_FAILURE.get().unwrap(),
        )
    });
    let (_, checkpoint) = engine
        .run_tran_checkpointed(&netlist, 1e-10, 1e-12)
        .unwrap();
    let encoded = checkpoint.to_text();
    let mut limited_config = SimulationConfig::default();
    limited_config.resource_limits.max_transport_history_bytes = 0;
    let limited = Engine::new(limited_config);
    let policy_error = fail_reservation_after(0, || {
        let result = limited.run_tran_resume(&netlist, &checkpoint, 2e-10, 1e-12);
        assert_eq!(HISTORY_RESERVATIONS_BEFORE_FAILURE.get(), Some(0));
        result.unwrap_err()
    });
    assert!(matches!(policy_error, SimulationError::ResourceLimit(error)
        if error.resource == crate::ResourceKind::TransportHistoryBytes));

    let (continued, _) = engine
        .run_tran_resume(&netlist, &checkpoint, 2e-10, 1e-12)
        .unwrap();
    for count in [0, 1, 5, 12] {
        let failed_capture = fail_reservation_after(startup_reservations + count, || {
            engine.run_tran_checkpointed(&netlist, 1e-10, 1e-12)
        });
        assert_allocation(&failed_capture.unwrap_err());
        let failed_restore = fail_reservation_after(count, || {
            engine.run_tran_resume(&netlist, &checkpoint, 2e-10, 1e-12)
        });
        assert_allocation(&failed_restore.unwrap_err());
        assert_eq!(checkpoint.to_text(), encoded);
    }
    let rerun = engine.run_tran(&netlist, 1e-10, 1e-12).unwrap();
    assert_eq!(rerun.time, baseline.time);
    assert_eq!(rerun.voltages, baseline.voltages);
    assert_eq!(rerun.branch_currents, baseline.branch_currents);
    let (resumed, _) = engine
        .run_tran_resume(&netlist, &checkpoint, 2e-10, 1e-12)
        .unwrap();
    assert_eq!(resumed.time, continued.time);
    assert_eq!(resumed.voltages, continued.voltages);
    assert_eq!(resumed.branch_currents, continued.branch_currents);
}
