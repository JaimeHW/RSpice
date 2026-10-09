use super::*;

#[test]
fn prepared_current_control_uses_finite_inductor_state_and_its_rate() {
    let circuit =
        build("finite current control\nV1 n 0 1\nL1 n 0 1u\nF1 out 0 L1 2\nR1 out 0 1k\n.end\n");
    let options = options();
    let mut sampler = PreparedEventCircuit::new(&circuit, 1e-20, &options, &NoAbort).unwrap();
    let topology = sampler
        .topology(0.0, SourceTimeSide::RightLimit, &options, &NoAbort)
        .unwrap();
    let state = topology
        .solve(
            &vec![0.0; circuit.matrix_size()],
            &vec![0.0; circuit.matrix_size()],
            &options,
            &NoAbort,
            |state, abort| {
                sampler.sample(0.0, SourceTimeSide::RightLimit, state, &[], &options, abort)
            },
        )
        .unwrap();
    let branch = circuit.num_nodes() + circuit.inductors.branch_indices[0] - 1;
    let output = circuit.get_node_by_name("out").unwrap() - 1;
    close(state.solution[branch], 0.0, 1e-16);
    close(state.solution[output], 0.0, 1e-12);
    close(state.coordinate_rates[branch].unwrap(), 1e6, 1e-6);
    close(state.coordinate_rates[output].unwrap(), -2e9, 1e-4);
}

#[test]
fn prepared_controlled_sources_transfer_constraint_rates_and_current_jacobians() {
    let circuit = build(
        "controlled event\nV1 ctrl 0 PWL(0 1 1 2)\nE1 n 0 ctrl 0 -2\nG1 m 0 n 0 .003\nRM m 0 2k\nCN n 0 1u\nRN n 0 1k\n.end\n",
    );
    let options = options();
    let mut sampler = PreparedEventCircuit::new(&circuit, 1e-20, &options, &NoAbort).unwrap();
    let topology = sampler
        .topology(0.0, SourceTimeSide::RightLimit, &options, &NoAbort)
        .unwrap();
    let result = topology
        .solve(
            &vec![0.0; circuit.matrix_size()],
            &vec![0.0; circuit.matrix_size()],
            &options,
            &NoAbort,
            |state, abort| {
                sampler.sample(0.0, SourceTimeSide::RightLimit, state, &[], &options, abort)
            },
        )
        .unwrap();
    for (node, value, rate) in [("ctrl", 1.0, 1.0), ("n", -2.0, -2.0), ("m", 12.0, 12.0)] {
        let index = circuit.get_node_by_name(node).unwrap() - 1;
        close(result.solution[index], value, 1e-12);
        close(result.coordinate_rates[index].unwrap(), rate, 1e-12);
    }
    let source_branch = circuit.num_nodes() + circuit.vcvs.branch_indices[0] - 1;
    close(result.solution[source_branch], 0.002002, 1e-14);
    let sources: Vec<_> = topology.source_branches().collect();
    let impulse = result.source_impulses[sources
        .iter()
        .position(|&branch| branch == source_branch)
        .unwrap()];
    close(impulse, 2e-6, 1e-20);
}

#[test]
fn prepared_controlled_sources_refuse_invalid_topology_and_coefficients() {
    for invalid in 0..5 {
        let mut circuit = build(
            "controlled validation\nV1 c 0 1\nE1 e 0 c 0 2\nG1 g 0 c 0 .001\nR1 e 0 1k\nR2 g 0 1k\n.end\n",
        );
        match invalid {
            0 => {
                circuit.vcvs.ctrl_pos.clear();
            }
            1 => {
                circuit.vccs.node_neg.clear();
            }
            2 => {
                circuit.vccs.ctrl_neg[0] = circuit.num_nodes() + 1;
            }
            3 => {
                circuit.vcvs.gains[0] = Value::NAN;
            }
            _ => {
                circuit.vccs.transconductances[0] = Value::INFINITY;
            }
        }
        assert!(matches!(
            PreparedEventCircuit::new(&circuit, 1e-20, &options(), &NoAbort),
            Err(SimulationError::Circuit(_))
        ));
    }
}

#[test]
fn resistive_ccvs_feedback_preserves_original_constraint_current_rate_and_impulse() {
    let circuit = build(
        "CCVS feedback\nV1 ref 0 PWL(0 1 1 2)\nRC b ref 2\nH1 b 0 RC 1\nCB b 0 1u\nRB b 0 1k\n.options device zeroresistancetol=2\n.end\n",
    );
    let options = options();
    let mut sampler = PreparedEventCircuit::new(&circuit, 1e-20, &options, &NoAbort).unwrap();
    let topology = sampler
        .topology(0.0, SourceTimeSide::RightLimit, &options, &NoAbort)
        .unwrap();
    let result = topology
        .solve(
            &vec![0.0; circuit.matrix_size()],
            &vec![0.0; circuit.matrix_size()],
            &options,
            &NoAbort,
            |state, abort| {
                sampler.sample(0.0, SourceTimeSide::RightLimit, state, &[], &options, abort)
            },
        )
        .unwrap();
    let b = circuit.get_node_by_name("b").unwrap() - 1;
    let rc = circuit.num_nodes() + circuit.resistor_branches.branch_indices[0] - 1;
    let h = circuit.num_nodes() + circuit.ccvs.branch_indices[0] - 1;
    close(result.solution[b], -1.0, 1e-12);
    close(result.solution[rc], -1.0, 1e-12);
    close(result.coordinate_rates[b].unwrap(), -1.0, 1e-12);
    close(result.coordinate_rates[rc].unwrap(), -1.0, 1e-12);
    close(result.solution[h], 1.001001, 1e-12);
    let index = topology
        .source_branches()
        .position(|branch| branch == h)
        .unwrap();
    close(result.source_impulses[index], 1e-6, 1e-20);
}

#[test]
fn ccvs_event_admission_rejects_unowned_controls_and_invalid_metadata() {
    let circuit = build(
        "CCVS validation\nV1 c 0 1\nRC c 0 1\nH1 b 0 RC 2\nR1 b 0 1k\n.options device zeroresistancetol=2\n.end\n",
    );
    for mutation in 0..13 {
        let mut candidate = circuit.clone();
        match mutation {
            0 => candidate.ccvs.node_pos.clear(),
            1 => candidate.ccvs.node_neg.clear(),
            2 => candidate.ccvs.branch_indices.clear(),
            3 => candidate.ccvs.ctrl_branch.clear(),
            4 => candidate.ccvs.transresistances.clear(),
            5 => candidate.ccvs.node_pos[0] = candidate.num_nodes() + 1,
            6 => candidate.ccvs.branch_indices[0] = 0,
            7 => candidate.ccvs.ctrl_branch[0] = 0,
            8 => candidate.ccvs.ctrl_branch[0] = candidate.num_branches() + 1,
            9 => candidate.ccvs.transresistances[0] = Value::NAN,
            10 => candidate.resistor_branches.resistances[0] = 0.0,
            11 => candidate.ccvs.branch_indices[0] = candidate.voltage_sources.branch_indices[0],
            _ => candidate.ccvs.branch_indices[0] = candidate.resistor_branches.branch_indices[0],
        }
        assert!(
            PreparedEventCircuit::new(&candidate, 1e-20, &options(), &NoAbort).is_err(),
            "mutation {mutation}"
        );
    }
    let mut zero = circuit.clone();
    zero.ccvs.transresistances[0] = 0.0;
    zero.ccvs.ctrl_branch[0] = zero.voltage_sources.branch_indices[0];
    assert!(PreparedEventCircuit::new(&zero, 1e-20, &options(), &NoAbort).is_ok());
}

#[test]
fn line_event_selection_checks_ccvs_control_ownership() {
    let mut circuit = build(
        "CCVS line eligibility\nV1 c 0 1\nRC c 0 1\nH1 near 0 RC 2\nT1 near 0 far 0 Z0=50 TD=1n\nRL far 0 50\n.options device zeroresistancetol=2\n.end\n",
    );
    assert!(PreparedEventCircuit::supports_scalar_line_events(&circuit));
    circuit.ccvs.ctrl_branch[0] = circuit.voltage_sources.branch_indices[0];
    assert!(!PreparedEventCircuit::supports_scalar_line_events(&circuit));
    circuit.ccvs.transresistances[0] = 0.0;
    assert!(PreparedEventCircuit::supports_scalar_line_events(&circuit));
}

#[test]
fn zero_transresistance_does_not_transfer_a_control_current_impulse() {
    let circuit =
        build("zero CCVS\nV1 c 0 PWL(0 1 1 2)\nC1 c 0 2u\nH1 b 0 V1 0\nR1 b 0 1k\n.end\n");
    let options = options();
    let mut sampler = PreparedEventCircuit::new(&circuit, 1e-20, &options, &NoAbort).unwrap();
    let topology = sampler
        .topology(0.0, SourceTimeSide::RightLimit, &options, &NoAbort)
        .unwrap();
    let result = topology
        .solve(
            &vec![0.0; circuit.matrix_size()],
            &vec![0.0; circuit.matrix_size()],
            &options,
            &NoAbort,
            |state, abort| {
                sampler.sample(0.0, SourceTimeSide::RightLimit, state, &[], &options, abort)
            },
        )
        .unwrap();
    let b = circuit.get_node_by_name("b").unwrap() - 1;
    let h = circuit.num_nodes() + circuit.ccvs.branch_indices[0] - 1;
    let v = circuit.num_nodes() + circuit.voltage_sources.branch_indices[0] - 1;
    close(result.solution[b], 0.0, 1e-12);
    close(result.solution[h], 0.0, 1e-16);
    close(result.coordinate_rates[b].unwrap(), 0.0, 1e-12);
    close(result.solution[v], -2e-6, 1e-16);
    for (index, branch) in topology.source_branches().enumerate() {
        close(
            result.source_impulses[index],
            if branch == h { 0.0 } else { -2e-6 },
            1e-20,
        );
    }
}
