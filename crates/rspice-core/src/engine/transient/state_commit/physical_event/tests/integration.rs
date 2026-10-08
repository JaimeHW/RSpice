use super::*;

#[test]
fn integrated_event_does_not_erase_a_real_subtolerance_current_jump() {
    let (engine, mut circuit, _, incoming, history) = fixture(
        "Tiny real event\nI1 0 n PWL(0 0 1n 0 1n 1e-20 2n 1e-20)\nR1 n 0 1k\nC1 n 0 1e-30\n.end\n",
    );
    let sources = causal::roots(&mut circuit, 2e-9);
    let before = history.clone();
    let coeff = CompanionCoefficients::backward_euler();
    let point = engine
        .prepare_physical_event(
            &circuit,
            &history,
            PhysicalEventStep {
                integration_coefficients: Some(&coeff),
                incoming: &incoming,
                time: 1e-9,
                dt: 1e-9,
                phase_events: PhysicalEventOrders::FromCauses {
                    sources: &sources,
                    accepted_time: 0.0,
                },
            },
            &options(),
            1e-20,
            &NoAbort,
        )
        .unwrap();
    let node = circuit.get_node_by_name("n").unwrap() - 1;
    close(point.state.coordinate_rates[node].unwrap(), 1e10, 1e-4);
    close(point.capacitors[0].current, 1e-20, 1e-30);
    assert_eq!(history, before);
}

#[test]
fn integration_reference_requires_unchanged_authored_forcing_and_an_integration_owner() {
    for source in [
        "I1 0 n PWL(0 0 1n 0 1n 1e-20 2n 1e-20)",
        "V1 n 0 PWL(0 0 1n 0 1n 1e-20 2n 1e-20)",
    ] {
        let (_, mut circuit, _, incoming, history) = fixture(&format!(
            "Forcing identity\n{source}\nR1 n 0 1k\nC1 n 0 1p\n.end\n",
        ));
        let sources = causal::roots(&mut circuit, 2e-9);
        let sampler = PreparedEventCircuit::new(&circuit, 1e-20, &options(), &NoAbort).unwrap();
        let coeff = CompanionCoefficients::backward_euler();
        for integration in [None, Some(&coeff)] {
            let step = PhysicalEventStep {
                integration_coefficients: integration,
                incoming: &incoming,
                time: 1e-9,
                dt: 1e-9,
                phase_events: PhysicalEventOrders::FromCauses {
                    sources: &sources,
                    accepted_time: 0.0,
                },
            };
            assert!(
                super::super::integration::currents(
                    &circuit,
                    &history,
                    &step,
                    &sampler,
                    &[],
                    &NoAbort
                )
                .unwrap()
                .is_none()
            );
        }
    }
}

#[test]
fn physical_event_restart_discards_incoming_mutual_flux_history() {
    let (_, mut circuit, _, mut solution, mut history) = fixture(
        "Restarted mutual flux\nR1 a 0 2\nR2 b 0 3\nL1 a 0 .5\nL2 b 0 .25\nK1 L1 L2 -.4\n.end\n",
    );
    let nodes = circuit.num_nodes();
    for currents in [[0.1, -0.3], [0.2, -0.1]] {
        for (index, &branch) in circuit.inductors.branch_indices.iter().enumerate() {
            solution[nodes + branch - 1] = currents[index];
            circuit.inductors.i_prev_prev[index] = circuit.inductors.i_prev[index];
            circuit.inductors.i_prev[index] = currents[index];
        }
        circuit.update_coupled_inductor_pair_state(&solution);
    }
    Engine::restart_physical_event_history(&mut circuit, &mut history);
    assert_flat_mutual_flux(&circuit, &solution);
}

#[test]
fn integration_reference_conserves_capacitor_current_and_mutual_flux_for_each_companion() {
    let time = 1.0;
    let dt = 0.2;
    let previous_dt = 0.3;
    let previous = time - dt;
    let older = previous - previous_dt;
    for coupling in [-0.5, 0.5] {
        let (_, mut circuit, _, mut incoming, history) = fixture(&format!(
            "Storage reference\nR1 a 0 1k\nR2 b 0 2k\nC1 a b 2u\nL1 a 0 2\nL2 b 0 8\nK1 L1 L2 {coupling}\n.end\n",
        ));
        let a = circuit.get_node_by_name("a").unwrap() - 1;
        let b = circuit.get_node_by_name("b").unwrap() - 1;
        incoming[a] = time * time;
        incoming[b] = 0.0;
        circuit.capacitors.v_prev[0] = previous * previous;
        circuit.capacitors.v_prev_prev[0] = older * older;
        circuit.capacitors.i_prev[0] = 2e-6 * 2.0 * previous;
        let mutual = 4.0 * coupling;
        let currents = |t: Value| [t * t + 2.0 * t, 3.0 * t * t - t];
        let rates = |t: Value| [2.0 * t + 2.0, 6.0 * t - 1.0];
        let previous_rates = rates(previous);
        let nodes = circuit.num_nodes();
        for (index, &ordinal) in circuit.inductors.branch_indices.iter().enumerate() {
            incoming[nodes + ordinal - 1] = currents(time)[index];
            circuit.inductors.i_prev[index] = currents(previous)[index];
            circuit.inductors.i_prev_prev[index] = currents(older)[index];
            circuit.inductors.v_prev[index] =
                [2.0, 8.0][index] * previous_rates[index] + mutual * previous_rates[1 - index];
        }
        let sampler = PreparedEventCircuit::new(&circuit, 1e-20, &options(), &NoAbort).unwrap();
        for (coeff, at) in [
            (CompanionCoefficients::backward_euler(), time - dt / 2.0),
            (CompanionCoefficients::trapezoidal(), time),
            (
                CompanionCoefficients::gear2_variable_step(dt, previous_dt),
                time,
            ),
        ] {
            let step = PhysicalEventStep {
                integration_coefficients: Some(&coeff),
                incoming: &incoming,
                time,
                dt,
                phase_events: PhysicalEventOrders::Declared(&[]),
            };
            let reference = super::super::integration::currents(
                &circuit,
                &history,
                &step,
                &sampler,
                &[],
                &NoAbort,
            )
            .unwrap()
            .unwrap();
            close(reference[a], 2e-6 * 2.0 * at, 1e-18);
            close(reference[b], -2e-6 * 2.0 * at, 1e-18);
            let rates = rates(at);
            for (index, &ordinal) in circuit.inductors.branch_indices.iter().enumerate() {
                close(
                    reference[nodes + ordinal - 1],
                    -[2.0, 8.0][index] * rates[index] - mutual * rates[1 - index],
                    1e-12,
                );
            }
        }
    }
}
