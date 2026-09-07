//! Independent currents and consistent voltages for uncoupled series windings.

use super::*;

#[derive(Debug, Clone, Copy)]
pub(super) struct CurrentCoordinate {
    pub state: usize,
    pub sign: Value,
}

#[derive(Debug, Clone)]
pub(super) struct PssCurrentBasis {
    pub representatives: Vec<usize>,
    pub coordinates: Vec<CurrentCoordinate>,
}

impl PssCurrentBasis {
    pub(super) fn new(circuit: &CircuitData) -> Self {
        let count = circuit.inductors.len();
        use crate::engine::periodic_capability::PeriodicDeviceFamily as F;
        if count <= 1
            || F::ALL.into_iter().any(|family| {
                !matches!(
                    family,
                    F::Resistor
                        | F::ResistorBranch
                        | F::Capacitor
                        | F::Inductor
                        | F::VoltageSource
                        | F::CurrentSource
                        | F::Vcvs
                        | F::Vccs
                        | F::Cccs
                        | F::Ccvs
                        | F::Diode
                        | F::BehavioralSource
                        | F::TransmissionLine
                        | F::InductorCoupling
                        | F::CoupledInductorPair
                ) && family.instance_count(circuit) != 0
            })
        {
            // A future state-family admission must supply its electrical
            // output ports before it can participate in this reduction.
            return Self {
                representatives: (0..count).collect(),
                coordinates: (0..count)
                    .map(|state| CurrentCoordinate { state, sign: 1.0 })
                    .collect(),
            };
        }
        let mut loaded = vec![circuit.global_shunt_conductance != 0.0; circuit.num_nodes() + 1];
        loaded[0] = true;
        let mut mark = |pos: usize, neg: usize| {
            loaded[pos] = true;
            loaded[neg] = true;
        };
        // A node touched by another current-carrying element is not a pure
        // series junction. Control inputs draw no current. This deliberately
        // makes no claim about general controlled-source/current cutsets.
        for stamp in &circuit.resistors.stamps {
            mark(stamp.pp.row, stamp.nn.row);
        }
        for stamp in &circuit.capacitors.stamps {
            mark(stamp.pp.row, stamp.nn.row);
        }
        for (pos, neg) in [
            (
                &circuit.resistor_branches.node_pos,
                &circuit.resistor_branches.node_neg,
            ),
            (
                &circuit.voltage_sources.node_pos,
                &circuit.voltage_sources.node_neg,
            ),
            (
                &circuit.current_sources.node_pos,
                &circuit.current_sources.node_neg,
            ),
            (&circuit.vcvs.node_pos, &circuit.vcvs.node_neg),
            (&circuit.vccs.node_pos, &circuit.vccs.node_neg),
            (&circuit.ccvs.node_pos, &circuit.ccvs.node_neg),
            (&circuit.cccs.node_pos, &circuit.cccs.node_neg),
        ] {
            for (&pos, &neg) in pos.iter().zip(neg) {
                mark(pos, neg);
            }
        }
        for diode in &circuit.diodes.devices {
            mark(diode.node_anode, diode.node_cathode);
        }
        for source in &circuit.behavioral_sources.voltage_sources {
            mark(source.node_pos, source.node_neg);
        }
        for source in &circuit.behavioral_sources.current_sources {
            mark(source.node_pos, source.node_neg);
        }
        for line in &circuit.tlines {
            mark(line.node1_pos, line.node1_neg);
            mark(line.node2_pos, line.node2_neg);
        }
        // Coupled-winding voltage consistency requires the full mutual
        // inductance operator; do not infer uncoupled voltage ratios there.
        for pair in &circuit.coupled_inductor_pairs {
            mark(pair.device.node1_pos, pair.device.node1_neg);
            mark(pair.device.node2_pos, pair.device.node2_neg);
        }
        for index in 0..count {
            let inductance = circuit.inductors.inductances[index];
            if !inductance.is_finite() || inductance <= 0.0 {
                mark(
                    circuit.inductors.node_pos[index],
                    circuit.inductors.node_neg[index],
                );
            }
        }
        let mut incident = vec![Vec::new(); loaded.len()];
        for index in 0..count {
            let pos = circuit.inductors.node_pos[index];
            let neg = circuit.inductors.node_neg[index];
            if pos == neg || circuit.inductors.inductances[index] <= 0.0 {
                continue;
            }
            incident[pos].push((index, 1.0));
            incident[neg].push((index, -1.0));
        }
        let mut adjacency = vec![Vec::new(); count];
        for (node, edges) in incident.iter().enumerate() {
            if !loaded[node]
                && let &[(first, first_sign), (second, second_sign)] = edges.as_slice()
            {
                let sign = -first_sign * second_sign;
                adjacency[first].push((second, sign));
                adjacency[second].push((first, sign));
            }
        }
        let mut coordinates = vec![
            CurrentCoordinate {
                state: usize::MAX,
                sign: 1.0
            };
            count
        ];
        let mut representatives = Vec::new();
        let mut pending = Vec::new();
        for start in 0..count {
            if coordinates[start].state != usize::MAX {
                continue;
            }
            let state = representatives.len();
            representatives.push(start);
            coordinates[start] = CurrentCoordinate { state, sign: 1.0 };
            pending.push(start);
            while let Some(from) = pending.pop() {
                for &(to, sign) in &adjacency[from] {
                    if coordinates[to].state == usize::MAX {
                        coordinates[to] = CurrentCoordinate {
                            state,
                            sign: coordinates[from].sign * sign,
                        };
                        pending.push(to);
                    }
                }
            }
        }
        Self {
            representatives,
            coordinates,
        }
    }

    /// Non-representative winding rows impose equal oriented di/dt. Keeping
    /// these voltage equations prevents an unconstrained internal node at
    /// t=0 after the independent currents have been fixed.
    pub(super) fn voltage_constraints(
        &self,
        circuit: &CircuitData,
        mut visit: impl FnMut(usize, usize, Value),
    ) {
        for (index, coordinate) in self.coordinates.iter().enumerate() {
            let representative = self.representatives[coordinate.state];
            if index == representative {
                continue;
            }
            let own_l = circuit.inductors.inductances[index];
            let reference_l = circuit.inductors.inductances[representative];
            let scale = own_l.max(reference_l);
            let own_weight = reference_l / scale;
            let reference_weight = -coordinate.sign * (own_l / scale);
            let row = circuit.num_nodes() + circuit.inductors.branch_indices[index] - 1;
            for (node, weight) in [
                (circuit.inductors.node_pos[index], own_weight),
                (circuit.inductors.node_neg[index], -own_weight),
                (circuit.inductors.node_pos[representative], reference_weight),
                (
                    circuit.inductors.node_neg[representative],
                    -reference_weight,
                ),
            ] {
                if node > 0 {
                    visit(row, node - 1, weight);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_mutual_group_does_not_disable_other_series_current_coordinates() {
        let engine = Engine::default();
        let netlist = Netlist::parse(
            "mixed magnetic coordinates\nV1 in 0 1\nR1 in out 1k\nL1 out mid 40u\nL2 mid 0 60u\nR3 in a 50\nL3 a 0 100u\nL4 b 0 200u\nR4 b 0 100\nK1 L3 L4 0.6\n.end\n",
        ).unwrap();
        let mut circuit = PssCircuit::new(engine.build_circuit(&netlist).unwrap());
        assert_eq!(circuit.state_dimension(), 3);
        circuit.set_state(&[1e-4, 2e-3, -1e-3]).unwrap();
        let solution = engine
            .pss_initial_node_solution(&mut circuit, &NoAbort)
            .unwrap();
        let node = |name: &str| circuit.get_node_by_name(name).unwrap() - 1;
        assert!((solution[node("mid")] - 0.6 * solution[node("out")]).abs() < 1e-12);
    }

    #[test]
    fn a_loaded_junction_does_not_constrain_the_two_winding_currents() {
        for (load, expected_coordinates) in [("", 1), ("Rload mid 0 1k", 2), ("Cload mid 0 1n", 2)]
        {
            let netlist = Netlist::parse(&format!(
                "current-coordinate rank\nV1 in 0 1\nR1 in out 1k\nL1 out mid 1u\nL2 0 mid 2u\n{load}\n.end\n"
            )).unwrap();
            let circuit = Engine::default().build_circuit(&netlist).unwrap();
            let basis = PssCurrentBasis::new(&circuit);
            assert_eq!(basis.representatives.len(), expected_coordinates, "{load}");
            if load.is_empty() {
                assert_eq!(basis.coordinates[0].state, basis.coordinates[1].state);
                assert_eq!(basis.coordinates[0].sign, -basis.coordinates[1].sign);
            }
        }
    }
}
