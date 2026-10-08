//! Retained and live voltage observations preserve units, coverage and sequence.
use rspice_core::{VoltageImpulseDerivative, VoltageImpulsePoint, VoltageImpulseTrace};
use rspice_results::events::TransientEventHistory;
use rspice_results::voltage_impulses::VoltageImpulseHistoryEvidence;
use rspice_simulation::live_transient::{
    LiveTransientPublisher, LiveTransientQueue, VoltageImpulseBuffer, VoltageImpulseDelta,
    VoltageImpulseUpdate,
};
use rspice_simulation_contract::worker_events::WorkerEventHistory;
use std::sync::{Arc, Mutex};

fn trace() -> VoltageImpulseTrace {
    VoltageImpulseTrace {
        node_name: "out".into(),
        complete: true,
        points: vec![VoltageImpulsePoint {
            time: 0.3,
            volt_seconds: -0.002,
        }],
        derivatives: vec![VoltageImpulseDerivative {
            time: 0.3,
            order: 2,
            coefficient: 1e-21,
        }],
    }
}

#[test]
fn voltage_histories_survive_worker_round_trip_and_remain_independent_of_current() {
    let events = TransientEventHistory {
        voltage_impulses: Some(VoltageImpulseHistoryEvidence {
            start_time_s: 0.0,
            stop_time_s: 1.0,
            delivery_complete: true,
            traces: vec![trace()],
        }),
        ..Default::default()
    };
    assert!(!events.is_empty());
    let wire = serde_json::to_string(&WorkerEventHistory::from(events.clone())).unwrap();
    assert!(wire.contains("voltSeconds"));
    assert!(!wire.contains("chargeCoulombs"));
    let decoded: WorkerEventHistory = serde_json::from_str(&wire).unwrap();
    assert_eq!(TransientEventHistory::from(decoded), events);
    events.voltage_impulses.unwrap().validate().unwrap();
}

#[test]
fn live_voltage_suffixes_preserve_exact_events_and_never_replay_points() {
    let queue = Arc::new(Mutex::new(LiveTransientQueue::default()));
    let publisher = LiveTransientPublisher::new(Some(Arc::clone(&queue)), None);
    let mut source = vec![trace()];
    source[0].points.clear();
    source[0].derivatives.clear();
    let publish = |source: &[VoltageImpulseTrace], time: &[f64]| {
        publisher.observe(rspice_core::abort_signal::TransientSample {
            time,
            node_names: &[],
            node_voltages: &[],
            branch_names: &[],
            branch_currents: &[],
            current_impulses: None,
            voltage_impulses: Some(source),
            digital_values: &[],
            digital_buses: &[],
            real_values: &[],
        });
    };
    publish(&source, &[0.0]);
    source[0] = trace();
    publish(&source, &[0.0, 1.0]);
    let mut buffer = VoltageImpulseBuffer::default();
    for delta in queue.lock().unwrap().drain() {
        assert!(delta.current_impulses.is_none());
        if let Some(delta) = delta.voltage_impulses {
            let wire = serde_json::to_string(&delta).unwrap();
            let restored: VoltageImpulseDelta = serde_json::from_str(&wire).unwrap();
            assert_eq!(restored, delta);
            buffer.ingest(restored.clone());
            buffer.ingest(restored);
        }
    }
    let history = buffer.history().unwrap();
    assert!(history.delivery_complete);
    assert_eq!(history.traces, source);
    history.validate().unwrap();
    publish(&source, &[0.0, 1.0]);
    for delta in queue.lock().unwrap().drain() {
        if let Some(delta) = delta.voltage_impulses {
            assert!(delta.traces.is_empty());
        }
    }
}

#[test]
fn live_voltage_gaps_and_malformed_coefficients_revoke_delivery_coverage() {
    let trace = trace();
    let mut delta = VoltageImpulseDelta {
        start_time_s: 0.0,
        stop_time_s: 1.0,
        first_sequence: 0,
        last_sequence: 0,
        delivery_complete: true,
        traces: vec![VoltageImpulseUpdate {
            owner: trace.node_name,
            complete: true,
            points: trace.points,
            derivatives: trace.derivatives,
        }],
    };
    for malformed in [false, true] {
        let mut buffer = VoltageImpulseBuffer::default();
        buffer.ingest(delta.clone());
        let mut next = delta.clone();
        next.first_sequence = if malformed { 1 } else { 2 };
        next.last_sequence = next.first_sequence;
        if malformed {
            next.traces[0].points[0].volt_seconds = f64::NAN;
        } else {
            next.traces.clear();
        }
        buffer.ingest(next);
        assert!(!buffer.history().unwrap().delivery_complete);
    }
    delta.traces[0].owner.clear();
    let mut buffer = VoltageImpulseBuffer::default();
    buffer.ingest(delta);
    assert!(buffer.history().is_none());
}
