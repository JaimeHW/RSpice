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
    let mut sampler =
        PreparedEventCircuit::for_finite_voltages(&circuit, 1e-20, &options, &NoAbort).unwrap();
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
    let sampler =
        PreparedEventCircuit::for_finite_voltages(&circuit, 1e-20, &options, &NoAbort).unwrap();
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

#[test]
fn nodal_behavioral_current_supplies_algebraic_coordinate_rates_and_domain_backtracking() {
    let circuit = build(
        "nodal current rates\nB0 x 0 V={1+time^2}\nB1 0 y I={.001*(v(x)+time)^3}\nR1 y 0 1k\n.end\n",
    );
    let options = options();
    let x = circuit.get_node_by_name("x").unwrap() - 1;
    let y = circuit.get_node_by_name("y").unwrap() - 1;
    let mut incoming = vec![0.0; circuit.matrix_size()];
    incoming[x] = 1.25;
    incoming[y] = 1.75_f64.powi(3);
    let mut sampler =
        PreparedEventCircuit::for_finite_voltages(&circuit, 1e-20, &options, &NoAbort).unwrap();
    let topology = sampler
        .topology(0.5, SourceTimeSide::RightLimit, &options, &NoAbort)
        .unwrap();
    let result = topology
        .solve(
            &incoming,
            &vec![0.0; incoming.len()],
            &options,
            &NoAbort,
            |state, abort| {
                sampler.sample(0.5, SourceTimeSide::RightLimit, state, &[], &options, abort)
            },
        )
        .unwrap();
    close(result.solution[y], 1.75_f64.powi(3), 1e-12);
    close(result.coordinate_rates[x].unwrap(), 1.0, 1e-12);
    close(
        result.coordinate_rates[y].unwrap(),
        6.0 * 1.75_f64.powi(2),
        1e-12,
    );

    let circuit = build("nonlinear trial domain\nB1 n 0 I={exp(v(n))}\n.end\n");
    let mut sampler =
        PreparedEventCircuit::for_finite_voltages(&circuit, 1e-20, &options, &NoAbort).unwrap();
    let invalid = sampler
        .sample(
            0.0,
            SourceTimeSide::RightLimit,
            &[1000.0],
            &[],
            &options,
            &NoAbort,
        )
        .unwrap();
    assert!(invalid.nonfinite(1).unwrap());
    let valid = sampler
        .sample(
            0.0,
            SourceTimeSide::RightLimit,
            &[0.0],
            &[],
            &options,
            &NoAbort,
        )
        .unwrap();
    assert!(!valid.nonfinite(1).unwrap());
    close(valid.f.values[0], 1.0, 1e-15);
    close(entry(&valid.f, 0, 0), 1.0, 1e-15);
    let mut bounded = options.clone();
    bounded.limits.max_result_values = 32_000;
    assert!(
        matches!(sampler.sample(0.0, SourceTimeSide::RightLimit, &[0.0], &[], &bounded, &NoAbort),
        Err(SimulationError::ResourceLimit(e)) if e.limit == 32_000 && e.requested > e.limit)
    );
}

#[test]
fn nodal_voltage_constraints_retain_nonlinear_feedback_rates_and_charge_fanout() {
    for fanout in [false, true] {
        let extra = if fanout {
            "F1 w 0 B1 2\nCW w 0 3u\nRW w 0 1k\n"
        } else {
            ""
        };
        let circuit = build(&format!(
            "nonlinear voltage constraint\nB0 x 0 V={{1+time^2}}\nB1 y 0 V={{(v(x)+time)^2-.05*v(y)^2}}\nCY y 0 2u\nRY y 0 1k\n{extra}.end\n"
        ));
        let options = options();
        let x = circuit.get_node_by_name("x").unwrap() - 1;
        let y = circuit.get_node_by_name("y").unwrap() - 1;
        let mut sampler =
            PreparedEventCircuit::for_finite_voltages(&circuit, 1e-20, &options, &NoAbort).unwrap();
        let topology = sampler
            .topology(0.5, SourceTimeSide::RightLimit, &options, &NoAbort)
            .unwrap();
        let zero = vec![0.0; circuit.matrix_size()];
        let result = topology
            .solve(&zero, &zero, &options, &NoAbort, |state, abort| {
                sampler.sample(0.5, SourceTimeSide::RightLimit, state, &[], &options, abort)
            })
            .unwrap();
        // y + .05*y^2 = (x+t)^2; these roots and derivatives are independent
        // of the expression evaluator and the event Newton implementation.
        let expected = ((1.0_f64 + 0.2 * 1.75_f64.powi(2)).sqrt() - 1.0) / 0.1;
        let rate = 7.0 / (1.0 + 0.1 * expected);
        close(result.solution[x], 1.25, 1e-12);
        close(result.solution[y], expected, 1e-12);
        close(result.coordinate_rates[x].unwrap(), 1.0, 1e-12);
        close(result.coordinate_rates[y].unwrap(), rate, 1e-12);
        let source = circuit
            .behavioral_sources
            .voltage_sources
            .iter()
            .find(|s| s.name.eq_ignore_ascii_case("b1"))
            .unwrap();
        let branch = circuit.num_nodes() + source.branch_ordinal - 1;
        let current = -expected / 1000.0 - 2e-6 * rate;
        close(result.solution[branch], current, 1e-14);
        let index = topology
            .source_branches()
            .position(|b| b == branch)
            .unwrap();
        close(result.source_impulses[index], -2e-6 * expected, 1e-17);
        if fanout {
            let w = circuit.get_node_by_name("w").unwrap() - 1;
            let voltage = 4.0 / 3.0 * expected;
            close(result.solution[w], voltage, 1e-12);
            close(
                result.coordinate_rates[w].unwrap(),
                (-2.0 * current - voltage / 1000.0) / 3e-6,
                1e-9,
            );
        }
    }
}

#[test]
fn nodal_voltage_physical_samples_preserve_domain_and_resource_failures() {
    let circuit = build("voltage trial domain\nB1 y 0 V={exp(v(x))}\nVX x 0 0\n.end\n");
    let options = options();
    let x = circuit.get_node_by_name("x").unwrap() - 1;
    let mut sampler =
        PreparedEventCircuit::for_finite_voltages(&circuit, 1e-20, &options, &NoAbort).unwrap();
    let mut state = vec![0.0; circuit.matrix_size()];
    state[x] = 1000.0;
    assert!(
        sampler
            .sample(
                0.0,
                SourceTimeSide::RightLimit,
                &state,
                &[],
                &options,
                &NoAbort
            )
            .unwrap()
            .nonfinite(state.len())
            .unwrap()
    );
    state[x] = 0.0;
    assert!(
        !sampler
            .sample(
                0.0,
                SourceTimeSide::RightLimit,
                &state,
                &[],
                &options,
                &NoAbort
            )
            .unwrap()
            .nonfinite(state.len())
            .unwrap()
    );
    let mut bounded = options.clone();
    bounded.limits.max_result_values = 32_000;
    assert!(
        matches!(sampler.sample(0.0, SourceTimeSide::RightLimit, &state, &[], &bounded, &NoAbort),
        Err(SimulationError::ResourceLimit(e)) if e.limit==32_000 && e.requested>e.limit)
    );
}

#[test]
fn nodal_voltage_seed_backtracks_overflow_and_conserves_incoming_charge() {
    let circuit = build(
        "voltage seed domain\nVX x 0 2\nB1 y 0 V={v(y)-exp(v(y))+v(x)}\nCY y 0 2u\nRY y 0 1k\n.end\n",
    );
    let options = options();
    let x = circuit.get_node_by_name("x").unwrap() - 1;
    let y = circuit.get_node_by_name("y").unwrap() - 1;
    let mut incoming = vec![0.0; circuit.matrix_size()];
    incoming[x] = (-10.0_f64).exp();
    incoming[y] = -10.0;
    let mut charge = vec![0.0; incoming.len()];
    charge[y] = -20e-6;
    let original = incoming.clone();
    let mut sampler =
        PreparedEventCircuit::for_finite_voltages(&circuit, 1e-20, &options, &NoAbort).unwrap();
    let topology = sampler
        .topology(0.0, SourceTimeSide::RightLimit, &options, &NoAbort)
        .unwrap();
    let mut invalid_probes = 0;
    let result = topology
        .solve(&incoming, &charge, &options, &NoAbort, |state, abort| {
            let sample =
                sampler.sample(0.0, SourceTimeSide::RightLimit, state, &[], &options, abort)?;
            invalid_probes += usize::from(sample.nonfinite(state.len())?);
            Ok(sample)
        })
        .unwrap();
    // exp(y)=x changes from exp(-10) to 2. The first local Newton guess
    // overflows exp; the accepted root and displaced charge remain physical.
    assert!(invalid_probes > 0);
    assert_eq!(incoming, original);
    close(result.solution[x], 2.0, 1e-12);
    close(result.solution[y], 2.0_f64.ln(), 1e-12);
    close(result.coordinate_rates[y].unwrap(), 0.0, 1e-12);
    let source = &circuit.behavioral_sources.voltage_sources[0];
    let branch = circuit.num_nodes() + source.branch_ordinal - 1;
    close(result.solution[branch], -2.0_f64.ln() / 1000.0, 1e-14);
    let index = topology
        .source_branches()
        .position(|b| b == branch)
        .unwrap();
    close(
        result.source_impulses[index],
        -2e-6 * (2.0_f64.ln() + 10.0),
        1e-17,
    );
}
