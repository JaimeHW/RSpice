//! Exercise the real history ceiling through public resume, without requiring
//! a million nonlinear solves to prepare the constant-bias history fixture.

use super::*;
use crate::{ResourceKind, SimulationErrorCategory, SimulationErrorCode};
use rspice_veriloga_runtime::transport_delay::{DelayBuffer, MAX_DELAY_HISTORY_SAMPLES};

#[test]
fn public_resume_preserves_record_limit_identity_and_the_input_checkpoint() {
    let netlist = Netlist::parse("record ceiling\nVC c 0 2\nVB b 0 .7\nQ1 c b 0 qm\nX1 c b cell\n.model qm NPN IS=1e-16 BF=100 TF=1n PTF=90\n.subckt cell c b\nQ2 c b 0 qm\n.ends\n.end\n").unwrap();
    let engine = Engine::default();
    let (_, checkpoint) = engine
        .run_tran_checkpointed(&netlist, 1e-10, 1e-12)
        .unwrap();
    let (baseline, _) = engine
        .run_tran_resume(&netlist, &checkpoint, 2e-10, 1e-12)
        .unwrap();
    let mut full = checkpoint.clone();
    let junction = &mut full.accepted_junction_history;
    let instance = junction.bjt_names[1].clone();
    assert!(
        instance.contains("X1") && instance.contains("Q2"),
        "{instance}"
    );
    let mut phase = junction.bjt_history.phase[1].as_ref().unwrap().checkpoint();
    let (last, value) = *phase.samples.last().unwrap();
    // Constant bias is unchanged by inserting interpolation knots. Preserve
    // startup's exact sided/event records and the last accepted sample clock.
    let count = MAX_DELAY_HISTORY_SAMPLES - phase.left_limits.len() - phase.event_orders.len();
    phase.samples = (0..count)
        .map(|index| (last * (index as f64 / (count - 1) as f64), value))
        .collect();
    junction.bjt_history.phase[1] = Some(DelayBuffer::from_checkpoint(phase).unwrap());
    let before = full.clone();
    let error = engine
        .run_tran_resume(&netlist, &full, 2e-10, 1e-12)
        .unwrap_err();
    let descriptor = error.descriptor();
    assert_eq!(descriptor.code, SimulationErrorCode::ResourceLimit);
    assert_eq!(descriptor.category, SimulationErrorCategory::ResourceLimit);
    assert!(!descriptor.retryable);
    let metadata = descriptor.resource_limit.unwrap();
    assert_eq!(metadata.resource, ResourceKind::TransportHistoryRecords);
    assert_eq!(metadata.resource.as_str(), "transport_history_records");
    assert_eq!(metadata.requested, MAX_DELAY_HISTORY_SAMPLES + 1);
    assert_eq!(metadata.limit, MAX_DELAY_HISTORY_SAMPLES);
    assert!(
        matches!(&error, SimulationError::DeviceResourceLimit { instance: actual, source }
        if actual == &instance && *source == metadata),
        "{error}"
    );
    assert!(std::error::Error::source(&error).is_some());
    assert_eq!(full, before);
    let (rerun, _) = engine
        .run_tran_resume(&netlist, &checkpoint, 2e-10, 1e-12)
        .unwrap();
    assert_eq!(rerun.time, baseline.time);
    assert_eq!(rerun.voltages, baseline.voltages);
    assert_eq!(rerun.branch_currents, baseline.branch_currents);
}
