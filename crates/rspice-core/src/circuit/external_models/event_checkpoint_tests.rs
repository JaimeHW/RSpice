use super::*;

#[cfg(feature = "veriloga")]
#[test]
fn event_checkpoint_compares_original_shared_contributions_after_hdl_restore() {
    use crate::xspice::event_scheduler::{SchedulerLimits, TimeResolution};
    use crate::xspice::verilog::host::{
        DigitalActiveExchange, DigitalActiveParticipant, DigitalHost, DigitalRunError,
    };
    use crate::xspice::verilog::store::{
        DigitalBitConnection, ExternalBitDriverId, ExternalRealDriverId,
    };
    #[derive(Default)]
    struct Bank {
        bits: Vec<(ExternalBitDriverId, DigitalValue)>,
        reals: Vec<(ExternalRealDriverId, f64)>,
    }
    impl DigitalActiveParticipant for Bank {
        fn settle_active(
            &mut self,
            x: &mut DigitalActiveExchange<'_>,
        ) -> Result<bool, DigitalRunError> {
            x.take_event_changes();
            let bits = std::mem::take(&mut self.bits);
            let reals = std::mem::take(&mut self.reals);
            x.drive_bank(&bits, &reals)?;
            Ok(!bits.is_empty() || !reals.is_empty())
        }
    }
    let plan = rspice_veriloga::VerilogACompiler::new(Default::default())
        .compile_canonical_ir_module(
            "module owners; wire d; wrealsum r; assign d=1'b1; assign r=7.0; endmodule",
            None,
        )
        .unwrap()
        .digital;
    let resolution = TimeResolution::new(plan.timing.precision_exponent).unwrap();
    let mut host = DigitalHost::from_plan(Arc::new(plan), resolution, SchedulerLimits::default());
    let bit = host.signal("d").unwrap();
    let real = host.signal("r").unwrap();
    host.connect_bits(&[vec![DigitalBitConnection {
        signal: bit,
        bit: 0,
    }]])
    .unwrap();
    let bit_ids = host
        .attach_external_bits(&[0], &[(0, target(1, "weak")), (0, target(1, "untouched"))])
        .unwrap();
    let real_ids = host
        .attach_external_reals(
            &[real],
            &[
                (real, target(2, "left")),
                (real, target(2, "right")),
                (real, target(2, "untouched")),
            ],
        )
        .unwrap();
    let producers = vec![
        (target(1, "weak"), false),
        (target(1, "untouched"), false),
        (target(2, "left"), true),
        (target(2, "right"), true),
        (target(2, "untouched"), true),
    ];
    let topology_for = |h: &DigitalHost| {
        EventCheckpointTopology::new(
            producers.clone(),
            [
                (1, EventValue::Digital(h.connected_value(0).unwrap())),
                (2, EventValue::Real(h.read_real(real).unwrap())),
            ],
            h.external_contributions()
                .map(|(target, value)| (target.clone(), value)),
        )
        .unwrap()
    };
    let (mut queue, mut values) = template();
    host.prepare_start().unwrap();
    host.advance_to_with(0, &mut Bank::default()).unwrap();
    values
        .make_mut()
        .digital_values
        .insert(1, host.connected_value(0).unwrap());
    values
        .make_mut()
        .real_values
        .insert(2, host.read_real(real).unwrap());
    // Never-executed XSPICE outputs agree with HDL enrollment defaults.
    XspiceEventCheckpoint::capture(
        &queue,
        &values,
        0.0,
        &topology_for(&host),
        Default::default(),
    )
    .unwrap();

    let weak = DigitalValue::new(DigitalState::Zero, DigitalStrength::Resistive);
    host.settle_with(
        0,
        &mut Bank {
            bits: vec![(bit_ids[0], weak)],
            reals: vec![(real_ids[0], 1.0), (real_ids[1], -1.0)],
        },
    )
    .unwrap();
    for (node, name, value) in [
        (1, "weak", EventValue::Digital(weak)),
        (2, "left", EventValue::Real(1.0)),
        (2, "right", EventValue::Real(-1.0)),
    ] {
        queue.make_mut().schedule(0.0, node, "out", name, 0, value);
    }
    advance(&mut queue, &mut values, 0.0);
    values
        .make_mut()
        .digital_values
        .insert(1, host.connected_value(0).unwrap());
    values
        .make_mut()
        .real_values
        .insert(2, host.read_real(real).unwrap());
    assert_eq!(host.connected_value(0), Some(DigitalValue::one()));
    assert_eq!(host.read_real(real), Some(7.0));
    let hdl_image = host.checkpoint(Default::default()).unwrap();
    let hdl_image = serde_json::from_slice(&serde_json::to_vec(&hdl_image).unwrap()).unwrap();
    let restored_host = host
        .fresh()
        .restored_checkpoint(&hdl_image, Default::default())
        .unwrap();
    let topology = topology_for(&restored_host);
    let image = XspiceEventCheckpoint::capture(&queue, &values, 0.0, &topology, Default::default())
        .unwrap();
    let image: XspiceEventCheckpoint =
        serde_json::from_slice(&serde_json::to_vec(&image).unwrap()).unwrap();
    let (empty_queue, empty_values) = template();
    image
        .restore(
            &empty_queue,
            &empty_values,
            0.0,
            &topology,
            Default::default(),
        )
        .unwrap();

    let source = serde_json::to_value(&image).unwrap();
    let mut weak_wrong = source.clone();
    weak_wrong["nodes"][0]["drivers"][0][1] = serde_json::json!({"Digital":DigitalValue::new(DigitalState::One,DigitalStrength::Resistive)});
    let mut cancellation_wrong = source.clone();
    cancellation_wrong["nodes"][1]["drivers"][0][1] = serde_json::json!({"Real":2.0_f64.to_bits()});
    cancellation_wrong["nodes"][1]["drivers"][1][1] =
        serde_json::json!({"Real":(-2.0_f64).to_bits()});
    let mut missing = source.clone();
    missing["nodes"][1]["drivers"] = serde_json::json!([]);
    // The net observations remain exactly the same in all three images.
    for bad in [weak_wrong, cancellation_wrong, missing] {
        let bad: XspiceEventCheckpoint = serde_json::from_value(bad).unwrap();
        assert!(
            bad.restore(
                &empty_queue,
                &empty_values,
                0.0,
                &topology,
                Default::default()
            )
            .unwrap_err()
            .contains("contribution")
        );
        assert_eq!(
            XspiceEventCheckpoint::capture(&queue, &values, 0.0, &topology, Default::default())
                .unwrap(),
            image
        );
    }

    // Receiving enrollment must be complete and cannot add or duplicate a producer.
    let observed = [
        (1, EventValue::Digital(DigitalValue::one())),
        (2, EventValue::Real(7.0)),
    ];
    let contributions: Vec<_> = restored_host
        .external_contributions()
        .map(|(t, v)| (t.clone(), v))
        .collect();
    let mut omitted = contributions.clone();
    omitted.pop();
    let mut duplicate = contributions.clone();
    duplicate.push(duplicate[0].clone());
    let mut unknown = contributions.clone();
    unknown[0].0.instance = "impostor".into();
    let mut wrong_domain = contributions.clone();
    wrong_domain[0].1 = EventValue::Real(0.0);
    for bad in [omitted, duplicate, unknown, wrong_domain] {
        assert!(EventCheckpointTopology::new(producers.clone(), observed, bad).is_err());
    }

    // An explicit -0.0 is not the untouched real default, even though the sum
    // remains 7.0. Adding its exact XSPICE contribution restores agreement.
    host.settle_with(
        0,
        &mut Bank {
            reals: vec![(real_ids[2], -0.0)],
            ..Default::default()
        },
    )
    .unwrap();
    let topology = topology_for(&host);
    assert!(
        XspiceEventCheckpoint::capture(&queue, &values, 0.0, &topology, Default::default())
            .is_err()
    );
    values
        .make_mut()
        .real_drivers
        .get_mut(&2)
        .unwrap()
        .insert(("untouched".into(), "out".into(), 0), -0.0);
    XspiceEventCheckpoint::capture(&queue, &values, 0.0, &topology, Default::default()).unwrap();
}

use crate::xspice::event_checkpoint::{
    EventCheckpointLimits, EventCheckpointTopology, XspiceEventCheckpoint,
};
use crate::xspice::event_scheduler::EventTarget;
use crate::xspice::{DigitalState, DigitalStrength, DigitalValue, EventValue};

fn target(node: usize, instance: &str) -> EventTarget {
    EventTarget {
        node_id: node,
        instance: instance.into(),
        port_name: "out".into(),
        driver_index: 0,
    }
}
fn topology() -> EventCheckpointTopology {
    EventCheckpointTopology::new(
        [
            (target(1, "a"), false),
            (target(1, "b"), false),
            (target(2, "a"), true),
            (target(2, "b"), true),
            (target(2, "c"), true),
            (target(2, "later"), true),
        ],
        [],
        [],
    )
    .unwrap()
}
fn template() -> (SharedXspiceEventQueue, SharedXspiceEventValues) {
    let mut values = SharedXspiceEventValues::default();
    values.make_mut().traces.configure(Arc::from([true, true]));
    (SharedXspiceEventQueue::new(), values)
}
fn advance(
    queue: &mut SharedXspiceEventQueue,
    values: &mut SharedXspiceEventValues,
    time: f64,
) -> Vec<crate::xspice::event_trace::EventTracePoint> {
    apply_xspice_events_at_or_before(values, queue, &mut Vec::new(), &mut Vec::new(), time)
        .unwrap();
    let mut traces = Vec::new();
    values.make_mut().traces.drain_into(&mut traces);
    traces
}
fn fixture() -> (SharedXspiceEventQueue, SharedXspiceEventValues) {
    let (mut queue, mut values) = template();
    for (instance, value) in [
        ("a", DigitalValue::one()),
        (
            "b",
            DigitalValue::new(DigitalState::Zero, DigitalStrength::Resistive),
        ),
    ] {
        queue
            .make_mut()
            .schedule(0.0, 1, "out", instance, 0, EventValue::Digital(value));
    }
    for (instance, value) in [("c", 1.0), ("b", -1e16), ("a", 1e16)] {
        queue
            .make_mut()
            .schedule(0.0, 2, "out", instance, 0, EventValue::Real(value));
    }
    queue.make_mut().schedule(
        1e-9,
        1,
        "out",
        "a",
        0,
        EventValue::Digital(DigitalValue::one()),
    );
    queue.make_mut().schedule(
        0.65e-9,
        1,
        "out",
        "a",
        0,
        EventValue::Digital(DigitalValue::zero()),
    );
    queue.make_mut().schedule(
        2e-9,
        1,
        "out",
        "a",
        0,
        EventValue::Digital(DigitalValue::one()),
    );
    queue
        .make_mut()
        .schedule(0.65e-9, 2, "out", "c", 0, EventValue::Real(-0.0));
    advance(&mut queue, &mut values, 0.2e-9);
    (queue, values)
}
fn capture(
    queue: &SharedXspiceEventQueue,
    values: &SharedXspiceEventValues,
    time: f64,
) -> XspiceEventCheckpoint {
    XspiceEventCheckpoint::capture(queue, values, time, &topology(), Default::default()).unwrap()
}

#[test]
fn event_checkpoint_resumes_superseded_timers_exact_payloads_drivers_and_waveforms() {
    let (mut queue, mut values) = fixture();
    let image = capture(&queue, &values, 0.2e-9);
    let bytes = serde_json::to_vec(&image).unwrap();
    let transported: XspiceEventCheckpoint = serde_json::from_slice(&bytes).unwrap();
    let (mut receiving_queue, receiving_values) = template();
    // The receiver may already have interned a later producer. The saved
    // registry is runtime state, and must replace that lazy enrollment exactly.
    receiving_queue
        .make_mut()
        .schedule(4e-9, 2, "out", "later", 0, EventValue::Real(5.0));
    let (mut resumed_queue, mut resumed_values) = transported
        .restore(
            &receiving_queue,
            &receiving_values,
            0.2e-9,
            &topology(),
            Default::default(),
        )
        .unwrap();
    assert_eq!(image, capture(&resumed_queue, &resumed_values, 0.2e-9));
    for time in [0.65e-9, 1e-9, 2e-9] {
        assert_eq!(
            advance(&mut queue, &mut values, time),
            advance(&mut resumed_queue, &mut resumed_values, time)
        );
        assert_eq!(
            capture(&queue, &values, time),
            capture(&resumed_queue, &resumed_values, time)
        );
    }
    assert_eq!(
        values.real_drivers[&2][&("c".into(), "out".into(), 0)].to_bits(),
        (-0.0f64).to_bits()
    );
    assert_eq!(values.digital_values[&1], DigitalValue::one());
    assert!(queue.is_empty() && resumed_queue.is_empty());
}

#[test]
fn event_checkpoint_refuses_corrupt_time_domain_resolution_identity_and_budgets_atomically() {
    let (queue, values) = fixture();
    let image = capture(&queue, &values, 0.2e-9);
    let json = serde_json::to_value(&image).unwrap();
    let mut bad = Vec::new();
    let mut v = json.clone();
    v["version"] = 2.into();
    bad.push(v);
    let mut v = json.clone();
    v["nodes"][0]["node"] = 999.into();
    bad.push(v);
    let mut v = json.clone();
    v["nodes"][0]["time"] = 1e-9f64.to_bits().into();
    bad.push(v);
    let mut v = json.clone();
    v["nodes"][0]["time"] = serde_json::Value::Null;
    bad.push(v);
    let mut v = json.clone();
    v["nodes"][0]["drivers"][0][0][0] = "unknown".into();
    bad.push(v);
    let mut v = json.clone();
    v["nodes"][1]["value"]["Real"] = 2.0f64.to_bits().into();
    bad.push(v);
    let mut v = json.clone();
    let duplicate = v["nodes"][0].clone();
    v["nodes"].as_array_mut().unwrap().push(duplicate);
    bad.push(v);
    let mut v = json.clone();
    v["scheduler"]["events"][0]["at_bits"] = 0.2e-9f64.to_bits().into();
    bad.push(v);
    let mut v = json.clone();
    v["scheduler"]["targets"][0]["instance"] = "unknown".into();
    bad.push(v);
    let mut v = json.clone();
    v["scheduler"]["events"][0]["value"] = serde_json::json!({"RealBits": 0});
    bad.push(v);
    let mut v = json.clone();
    v["scheduler"]["current_bits"] = 1e-9f64.to_bits().into();
    bad.push(v);
    for v in bad {
        let corrupt: XspiceEventCheckpoint = serde_json::from_value(v).unwrap();
        assert!(
            corrupt
                .restore(&queue, &values, 0.2e-9, &topology(), Default::default())
                .is_err()
        );
        assert_eq!(image, capture(&queue, &values, 0.2e-9));
    }
    for limits in [
        EventCheckpointLimits {
            max_nodes: 0,
            ..Default::default()
        },
        EventCheckpointLimits {
            max_drivers: 0,
            ..Default::default()
        },
        EventCheckpointLimits {
            max_name_bytes: 0,
            ..Default::default()
        },
    ] {
        assert!(
            image
                .restore(&queue, &values, 0.2e-9, &topology(), limits)
                .is_err()
        );
        assert!(
            XspiceEventCheckpoint::capture(&queue, &values, 0.2e-9, &topology(), limits).is_err()
        );
    }
    let (mut changed_queue, mut changed_values) = fixture();
    apply_xspice_events_at_or_before(
        &mut changed_values,
        &mut changed_queue,
        &mut Vec::new(),
        &mut Vec::new(),
        0.65e-9,
    )
    .unwrap();
    assert!(!changed_values.traces.is_empty());
    assert!(
        XspiceEventCheckpoint::capture(
            &changed_queue,
            &changed_values,
            0.65e-9,
            &topology(),
            Default::default()
        )
        .is_err()
    );
    assert!(
        image
            .restore(
                &changed_queue,
                &changed_values,
                0.2e-9,
                &topology(),
                Default::default()
            )
            .is_err()
    );
}

#[test]
fn event_checkpoint_checks_shared_observations_against_the_restored_hdl_owner() {
    let (queue, mut values) = template();
    let held = DigitalValue::one();
    values.make_mut().digital_values.insert(1, held);
    values.make_mut().digital_event_times.insert(1, 0.0);
    let nan = 0xfff8_0000_1234_5678u64;
    values.make_mut().real_values.insert(2, f64::from_bits(nan));
    values.make_mut().real_event_times.insert(2, 0.0);
    let topology = |real| {
        EventCheckpointTopology::new(
            [],
            [(1, EventValue::Digital(held)), (2, EventValue::Real(real))],
            [],
        )
        .unwrap()
    };
    let image = XspiceEventCheckpoint::capture(
        &queue,
        &values,
        0.0,
        &topology(f64::from_bits(nan)),
        Default::default(),
    )
    .unwrap();
    let transported: XspiceEventCheckpoint =
        serde_json::from_slice(&serde_json::to_vec(&image).unwrap()).unwrap();
    let (empty_queue, empty_values) = template();
    let (_, restored) = transported
        .restore(
            &empty_queue,
            &empty_values,
            0.0,
            &topology(f64::from_bits(nan)),
            Default::default(),
        )
        .unwrap();
    assert_eq!(restored.real_values[&2].to_bits(), nan);
    assert!(
        transported
            .restore(
                &empty_queue,
                &empty_values,
                0.0,
                &topology(1.0),
                Default::default()
            )
            .is_err()
    );
    let mut different_trace = empty_values.clone();
    different_trace
        .make_mut()
        .traces
        .configure(Arc::from([true, false]));
    assert!(
        transported
            .restore(
                &empty_queue,
                &different_trace,
                0.0,
                &topology(f64::from_bits(nan)),
                Default::default()
            )
            .is_err()
    );
}
