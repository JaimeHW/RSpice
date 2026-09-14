use super::*;
use crate::abort_signal::NoAbort;
use rspice_veriloga_runtime::transport_delay::{DelayCheckpoint, DelayConfiguration};

mod native;

fn options() -> EventOptions {
    EventOptions {
        limits: ResourceLimits::default(),
        solver: SolverOptions::default(),
        nodal_gmin: 0.0,
        iterations: 80,
        backtracks: 32,
        voltage_tolerance: 1e-11,
        current_tolerance: 1e-13,
        charge_tolerance: 1e-25,
        relative_tolerance: 1e-11,
    }
}

fn build(text: &str) -> crate::CircuitData {
    let deck = crate::Netlist::parse(text).unwrap();
    crate::Engine::default().build_circuit(&deck).unwrap()
}

fn close(actual: Value, expected: Value, absolute: Value) {
    assert!(
        (actual - expected).abs() <= absolute + 1e-10 * expected.abs(),
        "{actual:e} != {expected:e}"
    );
}

fn entry(stamp: &EventStamp, row: usize, column: usize) -> Value {
    stamp.rows[row]
        .iter()
        .filter(|(c, _)| *c == column)
        .map(|(_, value)| value)
        .sum()
}

#[test]
fn prepared_event_circuit_solves_rlc_jump_and_sided_source_rates() {
    let circuit = build(
        "RLC source event\nV1 s 0 PWL(0 0 1 0 1 2 2 4)\nR1 s n 4\nL1 n 0 .5\nC1 s 0 3u\nI1 0 s PWL(0 0 1 0 1 .01 2 .02)\n.end\n",
    );
    let options = options();
    let mut sampler = PreparedEventCircuit::new(&circuit, 1e-20, &options, &NoAbort).unwrap();
    let incoming = vec![0.0; circuit.matrix_size()];
    let left = sampler
        .sample(
            1.0,
            SourceTimeSide::LeftLimit,
            &incoming,
            &[],
            &options,
            &NoAbort,
        )
        .unwrap();
    let topology = sampler
        .topology(1.0, SourceTimeSide::RightLimit, &options, &NoAbort)
        .unwrap();
    let outgoing = topology
        .solve(&incoming, &left.q.values, &options, &NoAbort, |state, _| {
            sampler.sample(
                1.0,
                SourceTimeSide::RightLimit,
                state,
                &[],
                &options,
                &NoAbort,
            )
        })
        .unwrap();
    let s = circuit.get_node_by_name("s").unwrap() - 1;
    let n = circuit.get_node_by_name("n").unwrap() - 1;
    let l = circuit.num_nodes() + circuit.inductors.branch_indices[0] - 1;
    let v = circuit.num_nodes() + circuit.voltage_sources.branch_indices[0] - 1;
    close(outgoing.solution[s], 2.0, 1e-12);
    close(outgoing.solution[n], 2.0, 1e-12);
    close(outgoing.solution[l], 0.0, 1e-14);
    close(outgoing.solution[v], 0.01 - 6e-6, 1e-14);
    close(outgoing.source_impulses[0], -6e-6, 1e-18);
    close(outgoing.coordinate_rates[s].unwrap(), 2.0, 1e-12);
    close(outgoing.coordinate_rates[n].unwrap(), -14.0, 1e-12);
    close(outgoing.coordinate_rates[l].unwrap(), 4.0, 1e-12);
    let right = sampler
        .sample(
            1.0,
            SourceTimeSide::RightLimit,
            &outgoing.solution,
            &[],
            &options,
            &NoAbort,
        )
        .unwrap();
    close(left.f_time[s], 0.0, 0.0);
    close(right.f_time[s], -0.01, 1e-15);
}

#[test]
fn prepared_event_circuit_owns_zero_resistor_current_once() {
    let circuit = build("zero resistor\nV1 a 0 PWL(0 0 1 0 1 2)\nR0 a b 0\nR1 b 0 100\n.end\n");
    assert_eq!(circuit.resistor_branches.len(), 1);
    let options = options();
    let mut sampler = PreparedEventCircuit::new(&circuit, 1e-20, &options, &NoAbort).unwrap();
    let topology = sampler
        .topology(1.0, SourceTimeSide::RightLimit, &options, &NoAbort)
        .unwrap();
    let zeros = vec![0.0; circuit.matrix_size()];
    let outgoing = topology
        .solve(&zeros, &zeros, &options, &NoAbort, |state, _| {
            sampler.sample(
                1.0,
                SourceTimeSide::RightLimit,
                state,
                &[],
                &options,
                &NoAbort,
            )
        })
        .unwrap();
    let b = circuit.get_node_by_name("b").unwrap() - 1;
    let r = circuit.num_nodes() + circuit.resistor_branches.branch_indices[0] - 1;
    let v = circuit.num_nodes() + circuit.voltage_sources.branch_indices[0] - 1;
    close(outgoing.solution[b], 2.0, 1e-12);
    close(outgoing.solution[r], 0.02, 1e-14);
    close(outgoing.solution[v], -0.02, 1e-14);
    assert!(outgoing.source_impulses.iter().all(|value| *value == 0.0));
}

fn coupled(k: Value) -> crate::CircuitData {
    build(&format!(
        "coupled event\nV1 a 0 1\nV2 b 0 0\nL1 a 0 2\nL2 b 0 8\nK1 L1 L2 {k}\n.end\n"
    ))
}

#[test]
fn prepared_event_circuit_stamps_each_authored_mutual_overlay_once() {
    for k in [0.5, -0.5] {
        let circuit = coupled(k);
        let options = options();
        let mut sampler = PreparedEventCircuit::new(&circuit, 1e-20, &options, &NoAbort).unwrap();
        let a = circuit.num_nodes() + circuit.inductors.branch_indices[0] - 1;
        let b = circuit.num_nodes() + circuit.inductors.branch_indices[1] - 1;
        let mut state = vec![0.0; circuit.matrix_size()];
        state[a] = 0.3;
        state[b] = -0.1;
        let sample = sampler
            .sample(
                0.0,
                SourceTimeSide::RightLimit,
                &state,
                &[],
                &options,
                &NoAbort,
            )
            .unwrap();
        let m = k * 4.0;
        close(sample.q.values[a], -2.0 * 0.3 + m * 0.1, 1e-14);
        close(sample.q.values[b], 8.0 * 0.1 - m * 0.3, 1e-14);
        close(entry(&sample.q, a, b), -m, 0.0);
        close(entry(&sample.q, b, a), -m, 0.0);
        let topology = sampler
            .topology(0.0, SourceTimeSide::RightLimit, &options, &NoAbort)
            .unwrap();
        let outgoing = topology
            .solve(&state, &sample.q.values, &options, &NoAbort, |state, _| {
                sampler.sample(
                    0.0,
                    SourceTimeSide::RightLimit,
                    state,
                    &[],
                    &options,
                    &NoAbort,
                )
            })
            .unwrap();
        close(outgoing.solution[a], 0.3, 1e-13);
        close(outgoing.solution[b], -0.1, 1e-13);
        close(
            outgoing.coordinate_rates[a].unwrap(),
            8.0 / (16.0 - m * m),
            1e-13,
        );
        close(
            outgoing.coordinate_rates[b].unwrap(),
            -m / (16.0 - m * m),
            1e-13,
        );
    }
}

#[test]
fn prepared_event_circuit_refuses_mismatched_coupling_identity_and_coefficients() {
    for mutation in 0..8 {
        let mut circuit = coupled(0.5);
        match mutation {
            0 => circuit.coupled_inductor_pairs[0].device.m = 2.1,
            1 => circuit.coupled_inductor_pairs[0].device.k = -0.5,
            2 => circuit.coupled_inductor_pairs[0].device.node1_pos = 0,
            3 => circuit.coupled_inductor_pairs[0].device.name = "unowned".into(),
            4 => {
                circuit.coupled_inductor_pairs[0].branch1_ordinal =
                    circuit.coupled_inductor_pairs[0].branch2_ordinal
            }
            5 => circuit.couplings[0].inductor_names[1] = "L1".into(),
            6 => circuit.couplings.clear(),
            _ => circuit.coupled_inductor_pairs.clear(),
        }
        let failure = PreparedEventCircuit::new(&circuit, 1e-20, &options(), &NoAbort)
            .err()
            .unwrap();
        assert!(
            failure.to_string().contains("coupling"),
            "{mutation}: {failure}"
        );
    }
}

#[test]
fn prepared_event_circuit_refuses_unowned_equations_and_obeys_limits_and_abort() {
    let options = options();
    let unsupported = build("unsupported event\nV1 a 0 1\nD1 a 0 dm\n.model dm D\n.end\n");
    let failure = PreparedEventCircuit::new(&unsupported, 1e-20, &options, &NoAbort)
        .err()
        .unwrap();
    assert!(failure.to_string().contains("diode"), "{failure}");
    let mut circuit = build("event\nV1 a 0 1\nR1 a 0 1k\n.end\n");
    let mut restricted = self::options();
    restricted.limits.max_matrix_unknowns = 1;
    assert!(matches!(
        PreparedEventCircuit::new(&circuit, 1e-20, &restricted, &NoAbort),
        Err(SimulationError::ResourceLimit(_))
    ));
    let sampler = PreparedEventCircuit::new(&circuit, 1e-20, &options, &NoAbort).unwrap();
    assert!(
        sampler
            .topology(0.0, SourceTimeSide::RightLimit, &restricted, &NoAbort)
            .is_err()
    );
    assert!(
        sampler
            .topology(0.0, SourceTimeSide::Published, &options, &NoAbort)
            .is_err()
    );
    struct Stop;
    impl AbortSignal for Stop {
        fn is_aborted(&self) -> bool {
            true
        }
    }
    assert!(matches!(
        PreparedEventCircuit::new(&circuit, 1e-20, &options, &Stop),
        Err(SimulationError::Aborted)
    ));
    circuit.allocate_branch();
    let failure = PreparedEventCircuit::new(&circuit, 1e-20, &options, &NoAbort)
        .err()
        .unwrap();
    assert!(
        failure.to_string().contains("no physical event owner"),
        "{failure}"
    );
}

#[test]
fn prepared_event_circuit_preserves_numeric_trial_failure_for_backtracking() {
    let mut circuit = build("overflow probe\nR1 a 0 1\n.end\n");
    circuit.resistors.conductances[0] = 1e200;
    let options = options();
    let mut sampler = PreparedEventCircuit::new(&circuit, 1e-20, &options, &NoAbort).unwrap();
    let invalid = sampler
        .sample(
            0.0,
            SourceTimeSide::RightLimit,
            &[1e200],
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
            &[1e-200],
            &[],
            &options,
            &NoAbort,
        )
        .unwrap();
    assert!(!valid.nonfinite(1).unwrap());
    close(valid.f.values[0], 1.0, 1e-15);
}

#[test]
fn prepared_event_circuit_checks_storage_budget_and_refuses_auxiliary_capacitor_rows() {
    let mut circuit = build("capacitor event\nV1 a 0 1\nC1 a 0 1p\n.end\n");
    let mut options = options();
    options.limits.max_result_values = circuit.matrix_size() * 64;
    assert!(matches!(
        PreparedEventCircuit::new(&circuit, 1e-20, &options, &NoAbort),
        Err(SimulationError::ResourceLimit(_))
    ));
    options.limits = ResourceLimits::default();
    circuit.capacitors.ic_branch_indices[0] = Some(circuit.voltage_sources.branch_indices[0]);
    let failure = PreparedEventCircuit::new(&circuit, 1e-20, &options, &NoAbort)
        .err()
        .unwrap();
    assert!(
        failure.to_string().contains("auxiliary current rows"),
        "{failure}"
    );
    circuit.capacitors.ic_branch_indices[0] = None;
    circuit.capacitors.capacitances.clear();
    let failure = PreparedEventCircuit::new(&circuit, 1e-20, &options, &NoAbort)
        .err()
        .unwrap();
    assert!(
        failure.to_string().contains("unaligned capacitor"),
        "{failure}"
    );
}
