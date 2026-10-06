use super::*;

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
