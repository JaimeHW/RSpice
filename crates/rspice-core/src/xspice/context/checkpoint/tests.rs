use super::*;

#[test]
fn context_checkpoint_resumes_a_reinitialized_builtin_inertial_inverter() {
    use crate::xspice::{CodeModel, PortConnection, models::DigitalInverter};
    let model = DigitalInverter;
    let ports = ContextPortLayout::from_ports(
        model.ports(),
        &[PortConnection::Digital(1), PortConnection::Digital(2)],
    )
    .unwrap();
    let initialized = || {
        let mut c = CmContext::new();
        for (name, value) in [
            ("rise_delay", 4e-9),
            ("fall_delay", 4e-9),
            ("inertial_delay", 1.0),
        ] {
            c.set_param(name, value);
        }
        c.set_port_width("in", 1);
        c.set_port_width("out", 1);
        c.init_output("out", PortType::Digital);
        model.init(&mut c).unwrap();
        c.analysis = AnalysisType::Transient;
        c.call_type = CallType::TransientAnalysis;
        c.evaluation_phase = EvaluationPhase::AcceptedStep;
        c.set_input_digital("in", DigitalValue::one());
        c.set_input_digital_event_time("in", 0.0);
        model.evaluate(&mut c).unwrap();
        c.drain_pending_events().for_each(drop);
        c
    };
    let mut original = initialized();
    original.time = 4e-9;
    original.set_input_digital("in", DigitalValue::zero());
    original.set_input_digital_event_time("in", 4e-9);
    model.evaluate(&mut original).unwrap();
    let first: Vec<_> = original.drain_pending_events().collect();
    assert_eq!(first.len(), 1);
    assert_eq!(first[0].delay, 4e-9);
    let image = capture(&original, &ports, 4e-9);
    let image: ContextRuntimeCheckpoint =
        serde_json::from_slice(&serde_json::to_vec(&image).unwrap()).unwrap();
    let mut resumed = image
        .restore(&initialized(), &ports, 4e-9, 4, Default::default())
        .unwrap();
    for c in [&mut original, &mut resumed] {
        c.time = 6e-9;
        c.set_input_digital("in", DigitalValue::one());
        c.set_input_digital_event_time("in", 6e-9);
        model.evaluate(c).unwrap();
        let pending: Vec<_> = c.drain_pending_events().collect();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].values, vec![DigitalValue::zero()]);
        assert_eq!(
            pending[0].delay.to_bits(),
            ((8e-9_f64 - 6e-9_f64) / 2.0).to_bits()
        );
    }
    assert_eq!(
        capture(&original, &ports, 6e-9),
        capture(&resumed, &ports, 6e-9)
    );
}

fn fixture() -> (CmContext, ContextPortLayout) {
    let mut c = CmContext::new();
    c.time = 4e-9;
    c.time_prev = 3e-9;
    c.timestep = 1e-9;
    c.analysis = AnalysisType::Transient;
    c.call_type = CallType::TransientAnalysis;
    c.evaluation_phase = EvaluationPhase::AcceptedStep;
    c.transient_companion = CompanionCoefficients::trapezoidal();
    c.xyce_one_step_order2 = true;
    c.iteration = 7;
    c.allocate_states(3);
    c.allocate_int_states(2);
    // Opaque model state can encode bit patterns, not just finite numbers.
    c.state = vec![-0.0, f64::from_bits(0xfff8_0000_0000_1234), 3.0];
    c.state_prev = vec![2.0, -0.0, 5.0];
    c.int_state = vec![i64::MIN, 41];
    c.set_param("gain", 2.0);
    c.set_string_param("mode", "hold");
    c.set_real_vector_param("table", vec![1.0, 2.0]);
    c.set_string_param("mode", "hold"); // Preserve revision/cache causality too.
    let mut ports = ContextPortLayout::default();
    let analog = ContextPortShape {
        domain: 0,
        vector: true,
        width: 2,
    };
    let digital = ContextPortShape {
        domain: 1,
        vector: false,
        width: 1,
    };
    let real = ContextPortShape {
        domain: 2,
        vector: true,
        width: 2,
    };
    ports.inputs.insert("a".into(), analog);
    ports.inputs.insert("io".into(), digital);
    ports.inputs.insert("r".into(), real);
    ports.outputs.insert("o".into(), analog);
    ports.outputs.insert("io".into(), digital);
    for (name, p) in ports.inputs.iter().chain(&ports.outputs) {
        c.set_port_width(name, p.width);
    }
    c.set_input_analog_vector_from_fn("a", 2, |i| AnalogValue::new(i as f64))
        .unwrap();
    c.set_input_digital("io", DigitalValue::zero());
    c.set_input_digital_event_time("io", 3e-9);
    c.set_input_real_vector_from_fn("r", 2, |i| {
        if i == 0 {
            -0.0
        } else {
            f64::from_bits(0x7ff8_0000_0000_5678)
        }
    })
    .unwrap();
    c.init_output_vector("o", PortType::Voltage, 2);
    c.set_output_vector_with_partials("o", vec![1.25, 2.5], vec![0.25, 0.5])
        .unwrap();
    c.init_output("io", PortType::Digital);
    c.set_committed_digital_output("io", 0, DigitalValue::zero());
    c.set_other_digital_drivers("io", 0, DigitalValue::high_z(), 3e-9);
    c.set_output_digital_inertial("io", DigitalValue::one(), 4e-9, DigitalValue::zero(), None);
    c.drain_pending_events().for_each(drop);
    for (t, v) in [(2e-9, 1.0), (4e-9, 3.0)] {
        c.record_transient_history("transport", t, vec![v, -0.0], 10e-9);
    }
    let transition = AnalogTransition {
        state: true,
        event_time: 2e-9,
        transition_start: 3e-9,
        transition_end: 7e-9,
    };
    c.set_input_analog_vector_transitions("a", vec![Some(transition), None]);
    c.set_output_analog_transition("o", transition);
    c.stamp_conductance(0, 1, 0.5);
    c.stamp_rhs(1, -2.0);
    c.stamp_static_conductance(2, 3, 1.5);
    c.stamp_static_rhs(3, -0.0);
    (c, ports)
}
fn capture(c: &CmContext, p: &ContextPortLayout, t: f64) -> ContextRuntimeCheckpoint {
    ContextRuntimeCheckpoint::capture(c, p, t, 4, Default::default()).unwrap()
}

#[test]
fn context_checkpoint_continues_inertial_cancellation_history_and_analog_outputs() {
    let (mut original, ports) = fixture();
    let image = capture(&original, &ports, 4e-9);
    let bytes = serde_json::to_vec(&image).unwrap();
    let image: ContextRuntimeCheckpoint = serde_json::from_slice(&bytes).unwrap();
    let (mut receiving, _) = fixture();
    receiving.state.fill(99.0);
    receiving.state_prev.fill(99.0);
    receiving.int_state.fill(99);
    receiving.transient_histories.clear();
    receiving.inertial_outputs.clear();
    receiving.input_event_times.clear();
    receiving.inputs.clear();
    receiving.committed_digital_outputs.clear();
    receiving.other_digital_drivers.clear();
    receiving.output_analog_transitions.clear();
    receiving.clear_stamps();
    receiving.resource_limits.max_external_data_values = 9;
    let mut resumed = image
        .restore(&receiving, &ports, 4e-9, 4, Default::default())
        .unwrap();
    assert_eq!(capture(&resumed, &ports, 4e-9), image);
    assert_eq!(resumed.resource_limits.max_external_data_values, 9);
    assert_eq!(resumed.state[1].to_bits(), 0xfff8_0000_0000_1234);
    assert_eq!(
        resumed.input_real_vector_values("r").unwrap()[1].to_bits(),
        0x7ff8_0000_0000_5678
    );
    assert_eq!(resumed.stamps, vec![(0, 1, 0.5)]);
    assert_eq!(resumed.output_prev("io"), 0.0);
    assert_eq!(
        resumed.committed_digital_output("io", 0),
        Some(DigitalValue::zero())
    );

    for c in [&mut original, &mut resumed] {
        c.time = 6e-9;
        // The short opposite pulse cancels the unfinished edge at half its
        // remaining delay. Losing inertial state would schedule 4 ns instead.
        c.set_output_digital_inertial("io", DigitalValue::zero(), 4e-9, DigitalValue::zero(), None);
        let events: Vec<_> = c.drain_pending_events().collect();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].values, vec![DigitalValue::zero()]);
        assert_eq!(
            events[0].delay.to_bits(),
            ((8e-9_f64 - 6e-9_f64) / 2.0).to_bits()
        );
        assert_eq!(
            c.transient_history_at_or_after("transport", 3e-9),
            Some(vec![3.0, -0.0])
        );
        c.record_transient_history("transport", 6e-9, vec![5.0, 7.0], 3e-9);
        assert_eq!(c.transient_histories["transport"].len(), 2);
        c.set_output_vector_with_partials("o", vec![2.5, 5.0], vec![0.5, 1.0])
            .unwrap();
        assert_eq!(c.output_vector_prev("o"), vec![1.25, 2.5]);
        c.advance_state();
    }
    assert_eq!(
        capture(&resumed, &ports, 6e-9),
        capture(&original, &ports, 6e-9)
    );
}

#[test]
fn context_checkpoint_rejects_corrupt_shapes_clocks_histories_and_indices_without_mutation() {
    let (ctx, ports) = fixture();
    let image = capture(&ctx, &ports, 4e-9);
    let original = serde_json::to_value(&image).unwrap();
    let mut missing = image.clone();
    missing.inputs.remove(0);
    assert!(
        missing
            .restore(&ctx, &ports, 4e-9, 4, Default::default())
            .is_err()
    );
    let mut cases = Vec::new();
    macro_rules! corrupt {
        ($body:expr) => {{
            let mut v = original.clone();
            ($body)(&mut v);
            cases.push(v);
        }};
    }
    corrupt!(|v: &mut serde_json::Value| v["version"] = 2.into());
    corrupt!(|v: &mut serde_json::Value| v["time"] = (5e-9_f64).to_bits().into());
    corrupt!(|v: &mut serde_json::Value| v["time_prev"] = (5e-9_f64).to_bits().into());
    corrupt!(|v: &mut serde_json::Value| v["phase"] = 1.into());
    corrupt!(|v: &mut serde_json::Value| v["call"] = 2.into());
    corrupt!(|v: &mut serde_json::Value| v["companion"][0] = f64::INFINITY.to_bits().into());
    corrupt!(|v: &mut serde_json::Value| v["state"].as_array_mut().unwrap().pop());
    corrupt!(|v: &mut serde_json::Value| v["inputs"][0][0] = "bogus".into());
    corrupt!(|v: &mut serde_json::Value| v["outputs"][0][1] = serde_json::json!({"Real":0}));
    corrupt!(|v: &mut serde_json::Value| v["other"][0][1][0][1] = (5e-9_f64).to_bits().into());
    corrupt!(|v: &mut serde_json::Value| v["next_string_revision"] = 0.into());
    corrupt!(|v: &mut serde_json::Value| v["histories"][0][1][1][0] = (1e-9_f64).to_bits().into());
    corrupt!(|v: &mut serde_json::Value| v["histories"][0][1][0][1][0] = f64::NAN.to_bits().into());
    corrupt!(
        |v: &mut serde_json::Value| v["input_transitions"][0][1][0]["end"] =
            (1e-9_f64).to_bits().into()
    );
    corrupt!(|v: &mut serde_json::Value| v["inertial"][0][1][0] = (-2.0_f64).to_bits().into());
    corrupt!(|v: &mut serde_json::Value| v["stamps"][0][0] = 4.into());
    corrupt!(|v: &mut serde_json::Value| {
        let x = v["outputs"][0].clone();
        v["outputs"].as_array_mut().unwrap().push(x);
    });
    for (i, bad) in cases.into_iter().enumerate() {
        let bad: ContextRuntimeCheckpoint = serde_json::from_value(bad).unwrap();
        assert!(
            bad.restore(&ctx, &ports, 4e-9, 4, Default::default())
                .is_err(),
            "case {i}"
        );
        assert_eq!(capture(&ctx, &ports, 4e-9), image);
    }
    for limits in [
        ContextCheckpointLimits {
            max_items: 0,
            ..Default::default()
        },
        ContextCheckpointLimits {
            max_name_bytes: 0,
            ..Default::default()
        },
    ] {
        assert!(ContextRuntimeCheckpoint::capture(&ctx, &ports, 4e-9, 4, limits).is_err());
        assert!(image.restore(&ctx, &ports, 4e-9, 4, limits).is_err());
    }
}

#[test]
fn context_checkpoint_authenticates_configuration_and_refuses_unpublished_or_host_state() {
    let (ctx, ports) = fixture();
    let image = capture(&ctx, &ports, 4e-9);
    for change in [
        |c: &mut CmContext| {
            c.set_param("gain", 3.0);
        },
        |c: &mut CmContext| {
            c.temperature += 1.0;
        },
        |c: &mut CmContext| {
            c.set_port_width("a", 3);
        },
        |c: &mut CmContext| {
            c.port_total_loads.insert("io".into(), 1.0);
        },
        |c: &mut CmContext| {
            c.set_real_vector_param("table", vec![1.0, 3.0]);
        },
        |c: &mut CmContext| {
            c.set_string_param("mode", "pass");
        },
    ] {
        let mut receiving = ctx.clone();
        change(&mut receiving);
        assert!(
            image
                .restore(&receiving, &ports, 4e-9, 4, Default::default())
                .is_err()
        );
    }
    for change in [
        |c: &mut CmContext| c.request_breakpoint(8e-9),
        |c: &mut CmContext| c.set_output_digital("io", DigitalValue::one(), 0.0),
        |c: &mut CmContext| c.set_resource("host", Arc::new(123_u64)),
    ] {
        let mut receiving = ctx.clone();
        change(&mut receiving);
        assert!(
            ContextRuntimeCheckpoint::capture(&receiving, &ports, 4e-9, 4, Default::default())
                .is_err()
        );
        assert!(
            image
                .restore(&receiving, &ports, 4e-9, 4, Default::default())
                .is_err()
        );
    }
    // The outer owner may accept a circuit trial; the last call's phase is
    // retained metadata, not proof that a circuit trial is still open.
    let mut trial = ctx.clone();
    trial.evaluation_phase = EvaluationPhase::CircuitTrial;
    let trial_image = capture(&trial, &ports, 4e-9);
    let restored = trial_image
        .restore(&ctx, &ports, 4e-9, 4, Default::default())
        .unwrap();
    assert_eq!(restored.evaluation_phase, EvaluationPhase::CircuitTrial);
    assert_eq!(capture(&ctx, &ports, 4e-9), image);
}

#[test]
fn context_checkpoint_preserves_wrapped_revisions_and_close_history_replacements() {
    let (mut ctx, ports) = fixture();
    ctx.next_string_param_revision = u64::MAX;
    ctx.set_string_param("mode", "hold");
    ctx.next_real_vector_param_revision = u64::MAX;
    ctx.set_real_vector_param("table", vec![1.0, 2.0]);
    for t in [0.0, 1.5 * f64::EPSILON, 0.8 * f64::EPSILON] {
        ctx.record_transient_history("close", t, vec![1.0], 1.0);
    }
    assert_eq!(ctx.transient_histories["close"].len(), 2);
    let image = capture(&ctx, &ports, 4e-9);
    let (receiving, _) = fixture();
    let restored = image
        .restore(&receiving, &ports, 4e-9, 4, Default::default())
        .unwrap();
    assert_eq!(restored.string_param_revisions["mode"], u64::MAX);
    assert_eq!(restored.next_string_param_revision, 1);
    assert_eq!(restored.real_vector_param_revisions["table"], u64::MAX);
    assert_eq!(restored.next_real_vector_param_revision, 1);
    assert_eq!(capture(&restored, &ports, 4e-9), image);
}
