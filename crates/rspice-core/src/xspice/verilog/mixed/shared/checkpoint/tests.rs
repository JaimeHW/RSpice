use super::*;

const OBSERVER: &str = "`timescale 1ns/1ps\nmodule observer(a); input a; electrical a;
    integer count=0; reg timed; initial begin timed=0; #2 timed=1; end
    always @(absdelta(V(a),0.25,1n,0.05)) count=count+1;
    endmodule";

fn fixture(source: &str) -> (MixedDigitalCoordinator, Vec<MixedSignalHost>) {
    let compiled = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    })
    .compile_runtime(source, None)
    .unwrap();
    let model = Arc::new(compiled.model);
    let mut hosts: Vec<_> = ["first", "second"]
        .iter()
        .enumerate()
        .map(|(i, name)| {
            MixedSignalHost::from_compiled(
                name,
                Arc::clone(&model),
                &compiled.canonical_ir,
                &[i + 1],
                SchedulerLimits::default(),
                &rspice_veriloga::NoPipelineControl,
            )
            .unwrap()
        })
        .collect();
    let mut coordinator = MixedDigitalCoordinator::enroll(
        &mut hosts,
        &Default::default(),
        &rspice_veriloga::NoPipelineControl,
    )
    .unwrap();
    for host in &mut hosts {
        host.begin_analog_analysis(2).unwrap();
        host.start_digital_execution().unwrap();
    }
    coordinator.start().unwrap();
    (coordinator, hosts)
}

fn step(
    coordinator: &mut MixedDigitalCoordinator,
    hosts: &mut [MixedSignalHost],
    time: f64,
    value: f64,
) {
    let dt = time - coordinator.accepted_time.unwrap_or(0.0);
    let mut cursor = coordinator.open_trial(time, false).unwrap();
    for host in hosts.iter_mut() {
        host.begin_trial(
            time,
            dt,
            IntegrationCoefficients::inactive(),
            time == 0.0,
            false,
        )
        .unwrap();
        if cursor.opened_on_scheduled_activation() {
            host.note_scheduled_activation();
        }
    }
    let solution = [value, value * 2.0];
    let mut quiet = false;
    for _ in 0..8 {
        coordinator.advance(&mut cursor, hosts, &solution).unwrap();
        let mut changed = coordinator.synchronize(hosts).unwrap();
        for host in hosts.iter_mut() {
            host.stamp(&solution, |_, _, _| {}, |_, _| {}).unwrap();
            changed |= host.settle_analog_bridges(&solution).unwrap();
        }
        changed |= coordinator
            .publish_adc(&crate::abort_signal::NoAbort, &mut cursor, hosts, &solution)
            .unwrap();
        changed |= coordinator.synchronize(hosts).unwrap();
        if !changed {
            quiet = true;
            break;
        }
    }
    assert!(quiet, "shared trial did not settle");
    for host in hosts {
        host.accept_trial().unwrap();
    }
    coordinator.commit_trial(cursor);
}

#[test]
fn coordinator_checkpoint_resumes_observers_and_multiple_instance_timers() {
    let (mut original, mut hosts) = fixture(OBSERVER);
    step(&mut original, &mut hosts, 0.0, 0.0);
    step(&mut original, &mut hosts, 0.1e-9, 1.0);
    let image = original.checkpoint(Default::default()).unwrap();
    assert!(
        image.observers.iter().all(|words| words[1] & 8 != 0),
        "suppressed delta events must survive"
    );
    let bytes = serde_json::to_vec(&image).unwrap();
    let transported = serde_json::from_slice(&bytes).unwrap();
    let (mut template, templates) = fixture(OBSERVER);
    template.set_interval_event_limit(99);
    let mut restored = template
        .restored_checkpoint(&transported, Default::default())
        .unwrap();
    assert_eq!(restored.interval_event_limit(), 99);
    assert_eq!(image, restored.checkpoint(Default::default()).unwrap());
    let mut restored_hosts: Vec<_> = hosts
        .iter()
        .zip(&templates)
        .map(|(host, template)| {
            let image = host.participant_checkpoint(Default::default()).unwrap();
            let bytes = serde_json::to_vec(&image).unwrap();
            let image = serde_json::from_slice(&bytes).unwrap();
            template
                .restored_participant_checkpoint(&image, &restored, Default::default())
                .unwrap()
        })
        .collect();
    for (time, value) in [(1e-9, 1.0), (1.1e-9, 0.98), (1.2e-9, 0.94), (2e-9, 0.94)] {
        step(&mut original, &mut hosts, time, value);
        step(&mut restored, &mut restored_hosts, time, value);
        assert_eq!(
            original.checkpoint(Default::default()).unwrap(),
            restored.checkpoint(Default::default()).unwrap()
        );
        for (left, right) in hosts.iter().zip(&restored_hosts) {
            assert_eq!(
                left.read_digital("count").unwrap(),
                right.read_digital("count").unwrap()
            );
            assert_eq!(
                left.read_digital("timed").unwrap(),
                right.read_digital("timed").unwrap()
            );
        }
    }
    assert_eq!(hosts[0].read_digital("timed").unwrap(), "1");
}

#[test]
fn coordinator_checkpoint_refuses_partial_trials_and_corrupt_history_without_mutation() {
    let (mut coordinator, mut hosts) = fixture(OBSERVER);
    assert!(coordinator.checkpoint(Default::default()).is_err());
    step(&mut coordinator, &mut hosts, 0.0, 0.0);
    step(&mut coordinator, &mut hosts, 0.1e-9, 1.0);
    let good = coordinator.checkpoint(Default::default()).unwrap();
    let mut corrupt = Vec::new();
    let mut bad = good.clone();
    bad.version += 1;
    corrupt.push(bad);
    let mut bad = good.clone();
    bad.topology[0] ^= 1;
    corrupt.push(bad);
    let mut bad = good.clone();
    bad.time = f64::NAN.to_bits();
    corrupt.push(bad);
    let mut bad = good.clone();
    bad.observers.pop();
    corrupt.push(bad);
    let mut bad = good.clone();
    bad.probes.pop();
    corrupt.push(bad);
    let mut bad = good.clone();
    bad.observers[0][6] = 3;
    corrupt.push(bad);
    let operand = usize::from(coordinator.digital.plan().absdelta[0].operands[0]);
    let mut bad = good.clone();
    bad.probes[operand] = Some(0.5f64.to_bits());
    corrupt.push(bad);
    let enable = usize::from(coordinator.digital.plan().absdelta[0].operands[4]);
    let mut bad = good.clone();
    bad.probes[enable] = Some(0.0f64.to_bits());
    corrupt.push(bad);
    for bad in corrupt {
        assert!(
            coordinator
                .restored_checkpoint(&bad, Default::default())
                .is_err()
        );
        assert_eq!(good, coordinator.checkpoint(Default::default()).unwrap());
    }
    let mut limits = HostCheckpointLimits::default();
    limits.digital.max_items = 0;
    assert!(coordinator.restored_checkpoint(&good, limits).is_err());
    let cursor = coordinator.open_trial(0.2e-9, false).unwrap();
    assert!(coordinator.open_trial(0.2e-9, false).is_err());
    assert!(coordinator.checkpoint(Default::default()).is_err());
    assert!(
        coordinator
            .restored_checkpoint(&good, Default::default())
            .is_err()
    );
    coordinator.rollback_trial(cursor);
    assert_eq!(good, coordinator.checkpoint(Default::default()).unwrap());
    coordinator.set_analog_step_floor(1e-12);
    assert!(
        coordinator
            .restored_checkpoint(&good, Default::default())
            .is_err()
    );
}

#[test]
fn coordinator_checkpoint_authenticates_clock_and_preserves_an_idle_analog_interval() {
    let source =
        "module timer(a); input a; electrical a; reg q; initial begin q=0; #2 q=1; end endmodule";
    let (mut coordinator, mut hosts) = fixture(source);
    step(&mut coordinator, &mut hosts, 0.0, 0.0);
    step(&mut coordinator, &mut hosts, 0.6e-9, 0.0);
    let good = coordinator.checkpoint(Default::default()).unwrap();
    assert!(good.observers.is_empty() && good.probes.is_empty());
    let mut bad = good.clone();
    bad.time = 3e-9f64.to_bits();
    assert!(
        coordinator
            .restored_checkpoint(&bad, Default::default())
            .is_err()
    );
    let (template, _) = fixture(source);
    let mut restored = template
        .restored_checkpoint(&good, Default::default())
        .unwrap();
    step(&mut restored, &mut hosts, 2e-9, 0.0);
    assert_eq!(hosts[0].read_digital("q").unwrap(), "1");
    let mut bad = restored.checkpoint(Default::default()).unwrap();
    bad.time = 1e-9f64.to_bits();
    assert!(
        restored
            .restored_checkpoint(&bad, Default::default())
            .is_err()
    );
}
