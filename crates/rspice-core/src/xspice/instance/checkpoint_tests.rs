use super::*;

#[test]
fn instance_runtime_checkpoint_preserves_non_skipping_model_dispatch() {
    struct Counter;
    impl CodeModel for Counter {
        fn name(&self) -> &str {
            "counter"
        }
        fn ports(&self) -> &[PortSpec] {
            models::DigitalPullup.ports()
        }
        fn parameters(&self) -> &[ParamSpec] {
            &[]
        }
        fn init(&self, c: &mut CmContext) -> CmResult<()> {
            c.allocate_int_states(1);
            Ok(())
        }
        fn checkpoint_support(&self, _: &CmContext) -> XspiceCheckpointSupport {
            XspiceCheckpointSupport::Serializable
        }
        fn evaluate(&self, c: &mut CmContext) -> CmResult<()> {
            c.set_int_state(0, c.int_state(0) + 1);
            c.set_output_digital(
                "out",
                if c.int_state(0) % 2 == 0 {
                    DigitalValue::zero()
                } else {
                    DigitalValue::one()
                },
                0.0,
            );
            Ok(())
        }
    }
    let mut original = make(Arc::new(Counter));
    drive(
        &mut original,
        &mut SharedXspiceEventQueue::new(),
        0.0,
        DigitalValue::zero(),
    );
    drive(
        &mut original,
        &mut SharedXspiceEventQueue::new(),
        4e-9,
        DigitalValue::zero(),
    );
    let image = saved(&original, 4e-9);
    assert!(image.signature.is_none());
    assert!(image.dirty);
    let mut receiving = make(Arc::new(Counter));
    drive(
        &mut receiving,
        &mut SharedXspiceEventQueue::new(),
        0.0,
        DigitalValue::zero(),
    );
    let mut resumed = receiving
        .restored_runtime_checkpoint(&image, 4e-9, 0, 0, Default::default())
        .unwrap();
    assert_eq!(resumed.context.int_state(0), 2);
    for instance in [&mut original, &mut resumed] {
        drive(
            instance,
            &mut SharedXspiceEventQueue::new(),
            6e-9,
            DigitalValue::zero(),
        );
        assert_eq!(instance.context.int_state(0), 3);
    }
    assert_eq!(saved(&original, 6e-9), saved(&resumed, 6e-9));
    let mut bad = image.clone();
    bad.dirty = false;
    assert!(
        receiving
            .restored_runtime_checkpoint(&bad, 4e-9, 0, 0, Default::default())
            .is_err()
    );
}

#[test]
fn instance_runtime_checkpoint_checks_inverted_inputs_inout_ownership_and_real_payloads() {
    struct Observer {
        ports: Vec<PortSpec>,
    }
    impl CodeModel for Observer {
        fn name(&self) -> &str {
            "observer"
        }
        fn ports(&self) -> &[PortSpec] {
            &self.ports
        }
        fn parameters(&self) -> &[ParamSpec] {
            &[]
        }
        fn init(&self, _: &mut CmContext) -> CmResult<()> {
            Ok(())
        }
        fn evaluate(&self, _: &mut CmContext) -> CmResult<()> {
            Ok(())
        }
        fn checkpoint_support(&self, _: &CmContext) -> XspiceCheckpointSupport {
            XspiceCheckpointSupport::Serializable
        }
    }
    let mut io = PortSpec::input("io", PortType::Digital);
    io.direction = PortDirection::InOut;
    let model = Arc::new(Observer {
        ports: vec![
            io,
            PortSpec::input("r", PortType::Real),
            PortSpec::vector_input("rv", PortType::Real),
        ],
    });
    let mut instance = XspiceInstance::new(
        "A",
        model,
        vec![
            PortConnection::DigitalInverted(1),
            PortConnection::Real(2),
            PortConnection::RealVector(vec![3, 4]),
        ],
        &[],
        &[],
        &[],
        &[],
    )
    .unwrap();
    instance.init().unwrap();
    let bits = HashMap::from([(1, DigitalValue::one())]);
    let times = HashMap::from([(1, 2e-9)]);
    let reals = HashMap::from([
        (2, -0.0),
        (3, f64::from_bits(0x7ff8_0000_0000_5678)),
        (4, 5.0),
    ]);
    let real_times = HashMap::from([(2, 1e-9)]);
    let loads = HashMap::from([(1, 1e-12)]);
    let inputs = || XspiceEventInputs {
        digital_values: &bits,
        digital_event_times: &times,
        event_total_loads: &loads,
        real_values: &reals,
        real_event_times: &real_times,
    };
    let own = DigitalValue::new(
        crate::xspice::DigitalState::Zero,
        crate::xspice::DigitalStrength::Resistive,
    );
    let drivers = HashMap::from([(
        1,
        std::collections::BTreeMap::from([
            (("A".into(), "io".into(), 0), own),
            (("other".into(), "out".into(), 0), DigitalValue::one()),
        ]),
    )]);
    instance.update_inputs(&[], 0, inputs(), &[]).unwrap();
    instance.update_committed_digital_outputs(&drivers, 2e-9, |_| false);
    instance
        .evaluate(
            2e-9,
            1e-9,
            AnalysisType::Transient,
            EvaluationPhase::AcceptedStep,
        )
        .unwrap();
    instance.accept_timestep();
    let pristine = saved(&instance, 2e-9);
    instance
        .validate_runtime_event_observations(inputs(), &drivers, |_| Ok(None))
        .unwrap();
    instance
        .validate_runtime_event_observations(inputs(), &drivers, |_| Ok(Some(DigitalValue::one())))
        .unwrap();
    assert!(
        instance
            .validate_runtime_event_observations(inputs(), &drivers, |_| Ok(Some(
                DigitalValue::zero()
            )))
            .is_err()
    );
    let mut wrong_real = reals.clone();
    wrong_real.insert(2, 0.0);
    assert!(
        instance
            .validate_runtime_event_observations(
                XspiceEventInputs {
                    real_values: &wrong_real,
                    ..inputs()
                },
                &drivers,
                |_| Ok(None)
            )
            .is_err()
    );
    wrong_real = reals.clone();
    wrong_real.insert(3, f64::from_bits(0x7ff8_0000_0000_1234));
    assert!(
        instance
            .validate_runtime_event_observations(
                XspiceEventInputs {
                    real_values: &wrong_real,
                    ..inputs()
                },
                &drivers,
                |_| Ok(None)
            )
            .is_err()
    );
    let wrong_time = HashMap::from([(1, 1e-9)]);
    assert!(
        instance
            .validate_runtime_event_observations(
                XspiceEventInputs {
                    digital_event_times: &wrong_time,
                    ..inputs()
                },
                &drivers,
                |_| Ok(None)
            )
            .is_err()
    );
    assert!(
        instance
            .validate_runtime_event_observations(
                XspiceEventInputs {
                    event_total_loads: &HashMap::new(),
                    ..inputs()
                },
                &drivers,
                |_| Ok(None)
            )
            .is_err()
    );
    let mut wrong_driver = drivers.clone();
    wrong_driver
        .get_mut(&1)
        .unwrap()
        .remove(&("A".into(), "io".into(), 0));
    assert!(
        instance
            .validate_runtime_event_observations(inputs(), &wrong_driver, |_| Ok(None))
            .is_err()
    );
    assert_eq!(saved(&instance, 2e-9), pristine);
}

use crate::xspice::event::{SharedXspiceEventQueue, SharedXspiceEventValues};
use crate::xspice::event_checkpoint::{EventCheckpointTopology, XspiceEventCheckpoint};
use crate::xspice::{EventValue, PortDirection, models};

fn make(model: Arc<dyn CodeModel>) -> XspiceInstance {
    let mut next = 1;
    let ports: Vec<_> = model
        .ports()
        .iter()
        .map(|p| {
            if p.is_vector {
                let count = p.vector_min_len.unwrap_or(2).max(2);
                let nodes = (next..next + count).collect();
                next += count;
                PortConnection::DigitalVector(nodes)
            } else {
                let node = next;
                next += 1;
                PortConnection::Digital(node)
            }
        })
        .collect();
    let parameters = model
        .parameters()
        .iter()
        .filter_map(|p| match p.name.as_str() {
            "rise_delay" | "fall_delay" | "delay" => Some((p.name.clone(), 4e-9)),
            "inertial_delay" => Some((p.name.clone(), 1.0)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let mut instance =
        XspiceInstance::new("Agate", model, ports, &parameters, &[], &[], &[]).unwrap();
    instance.init().unwrap();
    instance
}
fn drive(
    instance: &mut XspiceInstance,
    queue: &mut SharedXspiceEventQueue,
    t: f64,
    level: DigitalValue,
) {
    let mut values = HashMap::new();
    let mut times = HashMap::new();
    instance.for_each_event_input_net(|_, node| {
        values.insert(node, level);
        times.insert(node, t);
    });
    instance
        .update_inputs(
            &[],
            0,
            XspiceEventInputs {
                digital_values: &values,
                digital_event_times: &times,
                event_total_loads: &HashMap::new(),
                real_values: &HashMap::new(),
                real_event_times: &HashMap::new(),
            },
            &[],
        )
        .unwrap();
    instance.mark_event_inputs_dirty();
    instance
        .evaluate(
            t,
            1e-9,
            AnalysisType::Transient,
            EvaluationPhase::AcceptedStep,
        )
        .unwrap();
    instance.schedule_events(queue.make_mut(), t);
    instance.accept_timestep();
}
fn drain(
    queue: &mut SharedXspiceEventQueue,
    values: &mut SharedXspiceEventValues,
    time: f64,
) -> Vec<(u64, usize, DigitalValue)> {
    let mut emitted = Vec::new();
    queue
        .make_mut()
        .run_due_events(time, |event| {
            let EventValue::Digital(value) = event.value else {
                panic!("digital gate")
            };
            let bank = values.make_mut();
            bank.digital_values.insert(event.node_id, value);
            bank.digital_event_times.insert(event.node_id, event.time);
            bank.digital_drivers
                .entry(event.node_id)
                .or_default()
                .insert((event.instance, event.port_name, event.driver_index), value);
            emitted.push((event.time.to_bits(), event.node_id, value));
        })
        .unwrap();
    emitted
}
fn topology(instance: &XspiceInstance) -> EventCheckpointTopology {
    let mut targets = Vec::new();
    instance.for_each_digital_output_driver(|target| targets.push((target, false)));
    EventCheckpointTopology::new(targets, [], []).unwrap()
}
fn saved(instance: &XspiceInstance, time: f64) -> InstanceRuntimeCheckpoint {
    instance
        .runtime_checkpoint(time, 0, Default::default())
        .unwrap()
}

#[test]
fn instance_runtime_checkpoint_restarts_the_builtin_gate_family_without_duplicate_execution() {
    let models: Vec<Arc<dyn CodeModel>> = vec![
        Arc::new(models::DigitalAnd),
        Arc::new(models::DigitalOr),
        Arc::new(models::DigitalXor),
        Arc::new(models::DigitalNand),
        Arc::new(models::DigitalNor),
        Arc::new(models::DigitalXnor),
        Arc::new(models::DigitalInverter),
        Arc::new(models::DigitalBuffer),
        Arc::new(models::DigitalTristate),
        Arc::new(models::DigitalPullup),
        Arc::new(models::DigitalPulldown),
        Arc::new(models::DigitalOpenCollector),
        Arc::new(models::DigitalOpenEmitter),
    ];
    for model in models {
        let mut original = make(model.clone());
        let mut queue = SharedXspiceEventQueue::new();
        let mut values = SharedXspiceEventValues::default();
        drive(&mut original, &mut queue, 0.0, DigitalValue::one());
        drain(&mut queue, &mut values, 0.0);
        drive(&mut original, &mut queue, 4e-9, DigitalValue::zero());
        drain(&mut queue, &mut values, 4e-9);
        let image = saved(&original, 4e-9);
        let event_image = XspiceEventCheckpoint::capture(
            &queue,
            &values,
            4e-9,
            &topology(&original),
            Default::default(),
        )
        .unwrap();
        let image: InstanceRuntimeCheckpoint =
            serde_json::from_slice(&serde_json::to_vec(&image).unwrap()).unwrap();
        let event_image: XspiceEventCheckpoint =
            serde_json::from_slice(&serde_json::to_vec(&event_image).unwrap()).unwrap();
        let mut receiving = make(model);
        let mut unused = SharedXspiceEventQueue::new();
        drive(&mut receiving, &mut unused, 0.0, DigitalValue::one());
        receiving.port_context_solution_num_nodes = None; // Reconstructed, never persisted.
        let mut resumed = receiving
            .restored_runtime_checkpoint(&image, 4e-9, 0, 0, Default::default())
            .unwrap();
        let (mut resumed_queue, mut resumed_values) = event_image
            .restore(
                &SharedXspiceEventQueue::new(),
                &SharedXspiceEventValues::default(),
                4e-9,
                &topology(&resumed),
                Default::default(),
            )
            .unwrap();
        assert_eq!(saved(&resumed, 4e-9), image);
        assert!(!resumed.event_inputs_dirty());
        // Calling the unchanged model must retain its prior dispatch signature.
        // Losing it would emit a duplicate pull-up/down output even with no input.
        resumed
            .evaluate(
                4e-9,
                1e-9,
                AnalysisType::Transient,
                EvaluationPhase::AcceptedStep,
            )
            .unwrap();
        assert!(!resumed.has_pending_events(), "{}", resumed.model_name());
        assert_eq!(saved(&resumed, 4e-9), image);
        drive(&mut original, &mut queue, 6e-9, DigitalValue::one());
        drive(&mut resumed, &mut resumed_queue, 6e-9, DigitalValue::one());
        assert_eq!(
            drain(&mut queue, &mut values, 12e-9),
            drain(&mut resumed_queue, &mut resumed_values, 12e-9),
            "{}",
            original.model_name()
        );
        assert_eq!(saved(&original, 12e-9), saved(&resumed, 12e-9));
        assert_eq!(
            XspiceEventCheckpoint::capture(
                &queue,
                &values,
                12e-9,
                &topology(&original),
                Default::default()
            )
            .unwrap(),
            XspiceEventCheckpoint::capture(
                &resumed_queue,
                &resumed_values,
                12e-9,
                &topology(&resumed),
                Default::default()
            )
            .unwrap()
        );
        assert!(
            !original.checkpoint_support().is_supported(),
            "legacy resume must stay guarded"
        );
    }
}

#[test]
fn instance_runtime_checkpoint_refuses_unsettled_dispatch_corrupt_signatures_and_changed_wiring() {
    let mut original = make(Arc::new(models::DigitalInverter));
    let mut queue = SharedXspiceEventQueue::new();
    drive(&mut original, &mut queue, 0.0, DigitalValue::one());
    let image = saved(&original, 0.0);
    let mut receiver = make(Arc::new(models::DigitalInverter));
    drive(
        &mut receiver,
        &mut SharedXspiceEventQueue::new(),
        0.0,
        DigitalValue::one(),
    );
    let pristine = saved(&receiver, 0.0);
    let mut cases = Vec::new();
    let mut bad = image.clone();
    bad.version += 1;
    cases.push(bad);
    let mut bad = image.clone();
    bad.dirty = true;
    cases.push(bad);
    let mut bad = image.clone();
    bad.signature = None;
    cases.push(bad);
    let mut bad = image.clone();
    bad.signature.as_mut().unwrap()[0].time = Some(1e-9_f64.to_bits());
    cases.push(bad);
    let mut bad = image.clone();
    bad.signature.as_mut().unwrap()[0].value = SignatureValue::Digital(DigitalValue::zero());
    cases.push(bad);
    let mut bad = image.clone();
    bad.signature.as_mut().unwrap()[0].value = SignatureValue::Real(0);
    cases.push(bad);
    let mut bad = image.clone();
    bad.signature.as_mut().unwrap().push(SignatureEntry {
        time: None,
        value: SignatureValue::Digital(DigitalValue::one()),
    });
    cases.push(bad);
    for bad in cases {
        assert!(
            receiver
                .restored_runtime_checkpoint(&bad, 0.0, 0, 0, Default::default())
                .is_err()
        );
        assert_eq!(saved(&receiver, 0.0), pristine);
    }
    for change in [
        |v: &mut XspiceInstance| v.name = "another".into(),
        |v: &mut XspiceInstance| v.connections[0] = PortConnection::DigitalInverted(1),
        |v: &mut XspiceInstance| v.connections[0] = PortConnection::Digital(99),
        |v: &mut XspiceInstance| v.mixed_input_thresholds_bound = true,
        |v: &mut XspiceInstance| v.output_branches[1] = Some(1),
    ] {
        let mut incompatible = receiver.clone();
        change(&mut incompatible);
        assert!(
            incompatible
                .restored_runtime_checkpoint(&image, 0.0, 0, 0, Default::default())
                .is_err()
        );
    }
    let mut dirty = original.clone();
    dirty.mark_event_inputs_dirty();
    assert!(
        dirty
            .runtime_checkpoint(0.0, 0, Default::default())
            .is_err()
    );
    let limits = InstanceCheckpointLimits {
        context: ContextCheckpointLimits {
            max_items: 1,
            ..Default::default()
        },
    };
    assert!(original.runtime_checkpoint(0.0, 0, limits).is_err());
    assert!(
        receiver
            .restored_runtime_checkpoint(&image, 0.0, 0, 0, limits)
            .is_err()
    );
    assert!(
        receiver
            .restored_runtime_checkpoint(&image, 0.0, 1, 0, Default::default())
            .is_err()
    );
}

#[test]
fn instance_runtime_checkpoint_requires_explicit_model_capability_and_resource_free_context() {
    // Same port names and state shape do not waive a model's capability contract.
    struct Undeclared;
    impl CodeModel for Undeclared {
        fn name(&self) -> &str {
            "d_inverter"
        }
        fn ports(&self) -> &[PortSpec] {
            models::DigitalInverter.ports()
        }
        fn parameters(&self) -> &[ParamSpec] {
            models::DigitalInverter.parameters()
        }
        fn init(&self, c: &mut CmContext) -> CmResult<()> {
            models::DigitalInverter.init(c)
        }
        fn evaluate(&self, c: &mut CmContext) -> CmResult<()> {
            models::DigitalInverter.evaluate(c)
        }
    }
    let mut instance = make(Arc::new(models::DigitalInverter));
    drive(
        &mut instance,
        &mut SharedXspiceEventQueue::new(),
        0.0,
        DigitalValue::one(),
    );
    let image = saved(&instance, 0.0);
    let mut unsupported = make(Arc::new(Undeclared));
    drive(
        &mut unsupported,
        &mut SharedXspiceEventQueue::new(),
        0.0,
        DigitalValue::one(),
    );
    assert!(
        unsupported
            .runtime_checkpoint(0.0, 0, Default::default())
            .is_err()
    );
    assert!(
        unsupported
            .restored_runtime_checkpoint(&image, 0.0, 0, 0, Default::default())
            .is_err()
    );
    instance.context.set_resource("opaque", Arc::new(123_u64));
    assert!(
        instance
            .runtime_checkpoint(0.0, 0, Default::default())
            .is_err()
    );
    assert!(
        instance
            .restored_runtime_checkpoint(&image, 0.0, 0, 0, Default::default())
            .is_err()
    );
}
