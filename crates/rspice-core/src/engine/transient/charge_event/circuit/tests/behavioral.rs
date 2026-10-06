use super::*;

#[test]
fn prescribed_behavioral_voltage_and_current_supply_physical_rates_and_kcl() {
    let circuit = build(
        "behavioral physical equations\nBV n 0 V={1+time^2}\nRN n 0 1k\nCN n 0 2u\nBI n 0 I={.002*time}\nBD 0 m I={.003*time^2}\nRM m 0 2k\n.end\n",
    );
    let options = options();
    let n = circuit.get_node_by_name("n").unwrap() - 1;
    let m = circuit.get_node_by_name("m").unwrap() - 1;
    let mut incoming = vec![0.0; circuit.matrix_size()];
    incoming[n] = 1.25;
    incoming[m] = 1.5;
    let mut sampler = PreparedEventCircuit::new(&circuit, 1e-20, &options, &NoAbort).unwrap();
    let sample = sampler
        .sample(
            0.5,
            SourceTimeSide::LeftLimit,
            &incoming,
            &[],
            &options,
            &NoAbort,
        )
        .unwrap();
    let topology = sampler
        .topology(0.5, SourceTimeSide::RightLimit, &options, &NoAbort)
        .unwrap();
    let result = topology
        .solve_continuous(
            &incoming,
            &sample.q.values,
            &options,
            &NoAbort,
            |state, abort| {
                sampler.sample(0.5, SourceTimeSide::RightLimit, state, &[], &options, abort)
            },
        )
        .unwrap();
    close(result.coordinate_rates[n].unwrap(), 1.0, 1e-12);
    close(result.coordinate_rates[m].unwrap(), 6.0, 1e-12);
    let source = &circuit.behavioral_sources.voltage_sources[0];
    close(
        result.solution[circuit.num_nodes() + source.branch_ordinal - 1],
        -1.25 / 1000.0 - 0.001 - 2e-6,
        1e-14,
    );
    assert_eq!(result.source_impulses, [0.0]);
}

#[test]
fn behavioral_derivative_workspace_preserves_typed_limits_and_cancellation() {
    let circuit =
        build("bounded behavioral event\nBV n 0 V={sin(time)+cos(time)}\nR n 0 1k\n.end\n");
    let mut options = options();
    options.limits.max_result_values = 32_000;
    let sampler = PreparedEventCircuit::new(&circuit, 1e-20, &options, &NoAbort).unwrap();
    assert!(
        matches!(sampler.topology(0.5, SourceTimeSide::RightLimit, &options, &NoAbort), Err(SimulationError::ResourceLimit(error)) if error.resource==ResourceKind::ResultValues && error.limit==32_000 && error.requested>error.limit)
    );
    options.limits = ResourceLimits::default();
    use std::sync::atomic::{AtomicUsize, Ordering};
    struct Cancel(AtomicUsize);
    impl crate::abort_signal::AbortSignal for Cancel {
        fn is_aborted(&self) -> bool {
            self.0.fetch_add(1, Ordering::SeqCst) + 1 >= 6
        }
    }
    let cancel = Cancel(AtomicUsize::new(0));
    assert!(matches!(
        sampler.topology(0.5, SourceTimeSide::RightLimit, &options, &cancel),
        Err(SimulationError::Aborted)
    ));
    assert_eq!(cancel.0.load(Ordering::SeqCst), 6);
    assert!(
        sampler
            .topology(0.5, SourceTimeSide::RightLimit, &options, &NoAbort)
            .is_ok()
    );
}
