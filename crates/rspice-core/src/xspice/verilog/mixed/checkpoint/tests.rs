use super::*;

const SOURCE: &str = "module bridge(p,n,adc,dac); inout p,n; electrical p,n;
    input adc; wire adc; output dac; reg dac;
    initial dac=0;
    always @(posedge adc) begin dac<=1; #2 dac<=0; end
    real integrated;
    analog begin integrated=idt(V(p,n),0.0);
      I(p,n)<+V(p,n)/1000.0 + integrated*1e-6 + ddt(1e-12*V(p,n));
    end endmodule";

fn fixture() -> (MixedDigitalCoordinator, MixedSignalHost) {
    let compiled = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    })
    .compile_runtime(SOURCE, None)
    .unwrap();
    let mut host = MixedSignalHost::from_compiled(
        "dut",
        Arc::new(compiled.model),
        &compiled.canonical_ir,
        &[1, 0],
        SchedulerLimits::default(),
        &rspice_veriloga::NoPipelineControl,
    )
    .unwrap();
    host.add_adc_bridge("adc", 0, (2, 0), 0.4, 0.6).unwrap();
    host.add_dac_bridge("dac", 0, (3, 0), 0.0, 5.0, 100.0)
        .unwrap();
    let mut coordinator = MixedDigitalCoordinator::enroll(
        std::slice::from_mut(&mut host),
        &Default::default(),
        &rspice_veriloga::NoPipelineControl,
    )
    .unwrap();
    host.begin_analog_analysis(2).unwrap();
    host.start_digital_execution().unwrap();
    coordinator.start().unwrap();
    (coordinator, host)
}

fn step(
    coordinator: &mut MixedDigitalCoordinator,
    host: &mut MixedSignalHost,
    time: f64,
    solution: &[f64],
) -> (Vec<(usize, usize, u64)>, Vec<(usize, u64)>) {
    let dt = time - host.state.accepted_time;
    let mut cursor = coordinator.open_trial(time, false).unwrap();
    host.begin_trial(
        time,
        dt,
        IntegrationCoefficients::backward_euler(dt).unwrap(),
        time == 0.0,
        false,
    )
    .unwrap();
    if cursor.opened_on_scheduled_activation() {
        host.note_scheduled_activation();
    }
    let hosts = std::slice::from_mut(host);
    let mut quiet = false;
    let mut stamp = (Vec::new(), Vec::new());
    for _ in 0..8 {
        coordinator.advance(&mut cursor, hosts, solution).unwrap();
        let mut changed = coordinator.synchronize(hosts).unwrap();
        stamp.0.clear();
        stamp.1.clear();
        hosts[0]
            .stamp(
                solution,
                |r, c, v| stamp.0.push((r, c, v.to_bits())),
                |r, v| stamp.1.push((r, v.to_bits())),
            )
            .unwrap();
        changed |= hosts[0].settle_analog_bridges(solution).unwrap();
        changed |= coordinator
            .publish_adc(&crate::abort_signal::NoAbort, &mut cursor, hosts, solution)
            .unwrap();
        changed |= coordinator.synchronize(hosts).unwrap();
        if !changed {
            quiet = true;
            break;
        }
    }
    assert!(quiet);
    hosts[0].accept_trial().unwrap();
    coordinator.commit_trial(cursor);
    stamp
}

#[test]
fn participant_checkpoint_resumes_analog_operators_and_adc_dac_histories_in_a_rebuilt_module() {
    let (mut coordinator, mut host) = fixture();
    step(&mut coordinator, &mut host, 0.0, &[0.0; 3]);
    step(&mut coordinator, &mut host, 1e-9, &[1.0, 1.0, 0.0]);
    let saved = host.participant_checkpoint(Default::default()).unwrap();
    let digital = coordinator.checkpoint(Default::default()).unwrap();
    let transported: ParticipantCheckpoint =
        serde_json::from_slice(&serde_json::to_vec(&saved).unwrap()).unwrap();
    let (template, receiving) = fixture();
    let mut resumed_coordinator = template
        .restored_checkpoint(&digital, Default::default())
        .unwrap();
    let mut resumed = receiving
        .restored_participant_checkpoint(&transported, &resumed_coordinator, Default::default())
        .unwrap();
    assert_eq!(
        saved,
        resumed.participant_checkpoint(Default::default()).unwrap()
    );
    assert_eq!(host.read_digital("dac").unwrap(), "1");
    assert_eq!(resumed.read_digital("dac").unwrap(), "1");
    let timer_time = coordinator.next_event_time().unwrap().unwrap().1;
    for (time, solution) in [
        (2e-9, [0.7, 1.0, 5.0]),
        (timer_time, [0.5, 1.0, 5.0]),
        (4e-9, [0.1, 0.0, 0.0]),
    ] {
        assert_eq!(
            step(&mut coordinator, &mut host, time, &solution),
            step(&mut resumed_coordinator, &mut resumed, time, &solution)
        );
        assert_eq!(
            host.participant_checkpoint(Default::default()).unwrap(),
            resumed.participant_checkpoint(Default::default()).unwrap()
        );
        assert_eq!(
            coordinator.checkpoint(Default::default()).unwrap(),
            resumed_coordinator.checkpoint(Default::default()).unwrap()
        );
    }
    assert_eq!(resumed.read_digital("dac").unwrap(), "0");
}

#[test]
fn participant_checkpoint_rejects_corrupt_boundaries_and_incompatible_configuration_atomically() {
    let (mut coordinator, mut host) = fixture();
    step(&mut coordinator, &mut host, 0.0, &[0.0; 3]);
    step(&mut coordinator, &mut host, 1e-9, &[1.0, 1.0, 0.0]);
    let good = host.participant_checkpoint(Default::default()).unwrap();
    let mut corrupt = Vec::new();
    let mut bad = good.clone();
    bad.version += 1;
    corrupt.push(bad);
    let mut bad = good.clone();
    bad.topology[0] ^= 1;
    corrupt.push(bad);
    let mut bad = good.clone();
    bad.analog_identity[3].push('x');
    corrupt.push(bad);
    let mut bad = good.clone();
    bad.analog.pop();
    corrupt.push(bad);
    let mut bad = good.clone();
    bad.tick += 1;
    corrupt.push(bad);
    let mut bad = good.clone();
    bad.time = 2e-9f64.to_bits();
    corrupt.push(bad);
    let mut bad = good.clone();
    bad.inputs.integration[0] = 2;
    corrupt.push(bad);
    let mut bad = good.clone();
    bad.inputs.state_integration = [0; 5];
    corrupt.push(bad);
    let mut bad = good.clone();
    bad.inputs.timestep = f64::NAN.to_bits();
    corrupt.push(bad);
    let mut bad = good.clone();
    bad.adc_voltages[0] = f64::INFINITY.to_bits();
    corrupt.push(bad);
    let mut bad = good.clone();
    bad.adc_decisions[0] = Some(3);
    corrupt.push(bad);
    let mut bad = good.clone();
    bad.adc_transitions[0] = Some(2e-9f64.to_bits());
    corrupt.push(bad);
    let mut bad = good.clone();
    bad.dac_history[0].recent ^= 1;
    corrupt.push(bad);
    let mut bad = good.clone();
    bad.adc_history[0].filled = 0;
    corrupt.push(bad);
    let mut bad = good.clone();
    bad.adc_history[0].filled = 9;
    corrupt.push(bad);
    let mut bad = good.clone();
    bad.adc_history[0].run = u32::MAX;
    corrupt.push(bad);
    let mut bad = good.clone();
    bad.adc_transitions.clear();
    corrupt.push(bad);
    for bad in corrupt {
        assert!(
            host.restored_participant_checkpoint(&bad, &coordinator, Default::default())
                .is_err()
        );
        assert_eq!(
            good,
            host.participant_checkpoint(Default::default()).unwrap()
        );
    }
    for limits in [
        ParticipantCheckpointLimits {
            max_items: 0,
            ..Default::default()
        },
        ParticipantCheckpointLimits {
            max_analog_words: 0,
            ..Default::default()
        },
    ] {
        assert!(
            host.restored_participant_checkpoint(&good, &coordinator, limits)
                .is_err()
        );
    }
    let mut other = host.clone();
    other.state.bridges.make_mut().dac[0].resistance *= 2.0;
    assert!(
        other
            .restored_participant_checkpoint(&good, &coordinator, Default::default())
            .is_err()
    );
    let cursor = coordinator.open_trial(2e-9, false).unwrap();
    assert!(
        host.restored_participant_checkpoint(&good, &coordinator, Default::default())
            .is_err()
    );
    coordinator.rollback_trial(cursor);
    host.begin_trial(
        2e-9,
        1e-9,
        IntegrationCoefficients::inactive(),
        false,
        false,
    )
    .unwrap();
    assert!(host.participant_checkpoint(Default::default()).is_err());
    assert!(
        host.restored_participant_checkpoint(&good, &coordinator, Default::default())
            .is_err()
    );
    host.reject_trial().unwrap();
    assert_eq!(
        good,
        host.participant_checkpoint(Default::default()).unwrap()
    );
}
