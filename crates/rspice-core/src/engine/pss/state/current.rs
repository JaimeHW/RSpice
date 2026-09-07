//! Independent winding currents, junction KCL and consistent flux derivatives.

use super::*;

#[derive(Debug, Clone, Copy)]
struct CurrentTreeEdge {
    parent: usize,
    child: usize,
    winding: usize,
    sign: Value,
}

/// Contract the other electrical ports, then choose a spanning forest of the
/// winding graph. Cotree currents are independent; tree currents follow from
/// component KCL. Storage and state installation are linear in graph size.
#[derive(Debug, Clone)]
pub(super) struct PssCurrentBasis {
    pub representatives: Vec<usize>,
    components: Vec<(usize, usize)>,
    tree: Vec<CurrentTreeEdge>,
    incident: Vec<Vec<(usize, Value)>>,
    winding_by_branch: Vec<usize>,
    has_mutual: bool,
}

fn root(parents: &mut [usize], mut node: usize) -> usize {
    while parents[node] != node {
        parents[node] = parents[parents[node]];
        node = parents[node];
    }
    node
}

impl PssCurrentBasis {
    pub(super) fn new(circuit: &CircuitData) -> Self {
        let count = circuit.inductors.len();
        let nodes = circuit.num_nodes() + 1;
        let mut winding_by_branch = if count == 0 {
            Vec::new()
        } else {
            vec![usize::MAX; circuit.num_branches() + 1]
        };
        for (index, &branch) in circuit.inductors.branch_indices.iter().enumerate() {
            winding_by_branch[branch] = index;
        }
        let has_mutual = circuit
            .coupled_inductor_pairs
            .iter()
            .any(|pair| pair.device.m != 0.0);
        use crate::engine::periodic_capability::PeriodicDeviceFamily as F;
        if count == 0
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
            // Future state-family admission must declare its electrical ports
            // before participating in topological state reduction.
            return Self {
                representatives: (0..count).collect(),
                components: vec![(0, 0); count],
                tree: Vec::new(),
                incident: vec![Vec::new()],
                winding_by_branch,
                has_mutual,
            };
        }
        let mut parents: Vec<_> = (0..nodes).collect();
        let mut join = |pos, neg| {
            let pos = root(&mut parents, pos);
            let neg = root(&mut parents, neg);
            parents[pos] = neg;
        };
        if circuit.global_shunt_conductance != 0.0 {
            for node in 1..nodes {
                join(node, 0);
            }
        }
        // Currents internal to one component cancel when its KCL rows are
        // summed. Controlled-source inputs draw no current and are not ports.
        for stamp in &circuit.resistors.stamps {
            join(stamp.pp.row, stamp.nn.row);
        }
        for (index, stamp) in circuit.capacitors.stamps.iter().enumerate() {
            if circuit.capacitors.capacitances[index] != 0.0 {
                join(stamp.pp.row, stamp.nn.row);
            }
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
                join(pos, neg);
            }
        }
        for diode in &circuit.diodes.devices {
            join(diode.node_anode, diode.node_cathode);
        }
        for source in &circuit.behavioral_sources.voltage_sources {
            join(source.node_pos, source.node_neg);
        }
        for source in &circuit.behavioral_sources.current_sources {
            join(source.node_pos, source.node_neg);
        }
        for line in &circuit.tlines {
            join(line.node1_pos, line.node1_neg);
            join(line.node2_pos, line.node2_neg);
        }
        // Only components touched by a winding need retained graph storage.
        // A large resistive network with one coil must not retain a second
        // node-sized collection of empty adjacency lists in every PSS worker.
        let mut ids = std::collections::HashMap::new();
        let mut component = |node| {
            let node = root(&mut parents, node);
            let next = ids.len();
            *ids.entry(node).or_insert(next)
        };
        let components = circuit
            .inductors
            .node_pos
            .iter()
            .zip(&circuit.inductors.node_neg)
            .map(|(&pos, &neg)| (component(pos), component(neg)))
            .collect::<Vec<_>>();
        let component_count = ids.len();
        let ground = ids.get(&root(&mut parents, 0)).copied();
        let mut incident = vec![Vec::new(); component_count];
        for (index, &(pos, neg)) in components.iter().enumerate() {
            if pos != neg {
                incident[pos].push((index, 1.0));
                incident[neg].push((index, -1.0));
            }
        }
        let mut spanning: Vec<_> = (0..component_count).collect();
        let mut adjacency = vec![Vec::new(); component_count];
        let mut independent = vec![true; count];
        // Reverse insertion retains the first authored current for an
        // ordinary series chain and makes the basis independent of hash order.
        for (index, &(pos, neg)) in components.iter().enumerate().rev() {
            let rp = root(&mut spanning, pos);
            let rn = root(&mut spanning, neg);
            if rp != rn {
                spanning[rp] = rn;
                independent[index] = false;
                adjacency[pos].push((neg, index, -1.0));
                adjacency[neg].push((pos, index, 1.0));
            }
        }
        let representatives = independent
            .iter()
            .enumerate()
            .filter_map(|(index, &free)| free.then_some(index))
            .collect();
        let mut visited = vec![false; component_count];
        let mut tree = Vec::new();
        let mut pending = Vec::new();
        for start in ground.into_iter().chain(0..component_count) {
            if visited[start] {
                continue;
            }
            visited[start] = true;
            pending.push(start);
            while let Some(parent) = pending.pop() {
                for &(child, winding, sign) in &adjacency[parent] {
                    if !visited[child] {
                        visited[child] = true;
                        tree.push(CurrentTreeEdge {
                            parent,
                            child,
                            winding,
                            sign,
                        });
                        pending.push(child);
                    }
                }
            }
        }
        Self {
            representatives,
            components,
            tree,
            incident,
            winding_by_branch,
            has_mutual,
        }
    }

    pub(super) fn workspace_size(&self) -> usize {
        self.incident.len()
    }

    pub(super) fn needs_flux_rates(&self) -> bool {
        !self.tree.is_empty() && self.has_mutual
    }

    pub(super) fn set_state(&self, state: &[Value], currents: &mut [Value], balance: &mut [Value]) {
        currents.fill(0.0);
        balance.fill(0.0);
        for (&index, &current) in self.representatives.iter().zip(state) {
            currents[index] = current;
            let (pos, neg) = self.components[index];
            if pos != neg {
                balance[pos] += current;
                balance[neg] -= current;
            }
        }
        for edge in self.tree.iter().rev() {
            currents[edge.winding] = -edge.sign * balance[edge.child];
            balance[edge.parent] += balance[edge.child];
        }
    }

    /// Sparse physical-current projection into the independent coordinates.
    /// A tree branch sees exactly the cotree currents crossing its subtree.
    pub(super) fn projection(&self, winding: usize) -> Vec<(usize, Value)> {
        if let Some(state) = self
            .representatives
            .iter()
            .position(|&index| index == winding)
        {
            return vec![(state, 1.0)];
        }
        let Some(probe) = self.tree.iter().find(|edge| edge.winding == winding) else {
            return Vec::new();
        };
        let mut inside = vec![false; self.incident.len()];
        inside[probe.child] = true;
        for edge in &self.tree {
            inside[edge.child] |= inside[edge.parent];
        }
        self.representatives
            .iter()
            .enumerate()
            .filter_map(|(state, &index)| {
                let (pos, neg) = self.components[index];
                let crossing = i8::from(inside[pos]) - i8::from(inside[neg]);
                (crossing != 0).then_some((state, -probe.sign * Value::from(crossing)))
            })
            .collect()
    }

    pub(super) fn current_rows<'a>(
        &'a self,
        circuit: &'a CircuitData,
        rates: Option<usize>,
    ) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.representatives.iter().map(move |&index| {
            let branch = rates.map_or(circuit.inductors.branch_indices[index], |first| {
                first + index
            });
            (circuit.num_nodes() + branch - 1, index)
        })
    }

    pub(super) fn is_current_row(
        &self,
        circuit: &CircuitData,
        row: usize,
        rates: Option<usize>,
    ) -> bool {
        let Some(branch) = row
            .checked_sub(circuit.num_nodes())
            .and_then(|r| r.checked_add(1))
        else {
            return false;
        };
        let index = match rates {
            Some(first) => branch.checked_sub(first),
            None => self.winding_by_branch.get(branch).copied(),
        };
        index.is_some_and(|index| self.representatives.binary_search(&index).is_ok())
    }

    /// Initialize with exact current constraints and d(KCL)/dt = 0. With
    /// mutual flux, auxiliary voltage-scaled rates w_i = L_i di_i/dt retain
    /// sparse L di/dt equations instead of forming a dense inverse inductance.
    pub(super) fn initial_matrix(
        &self,
        circuit: &CircuitData,
        rates: Option<usize>,
        mut visit: impl FnMut(usize, usize, Value),
    ) {
        let branch_row =
            |index: usize| circuit.num_nodes() + circuit.inductors.branch_indices[index] - 1;
        let rate_row = |index: usize| circuit.num_nodes() + rates.unwrap() + index - 1;
        for (row, index) in self.current_rows(circuit, rates) {
            visit(row, branch_row(index), 1.0);
        }
        if rates.is_some() {
            for index in 0..circuit.inductors.len() {
                visit(branch_row(index), rate_row(index), -1.0);
            }
            for pair in &circuit.coupled_inductor_pairs {
                let first = self.winding_by_branch[pair.branch1_ordinal];
                let second = self.winding_by_branch[pair.branch2_ordinal];
                visit(
                    branch_row(first),
                    rate_row(second),
                    -pair.device.m / circuit.inductors.inductances[second],
                );
                visit(
                    branch_row(second),
                    rate_row(first),
                    -pair.device.m / circuit.inductors.inductances[first],
                );
            }
        }
        for edge in &self.tree {
            let row = if rates.is_some() {
                rate_row(edge.winding)
            } else {
                branch_row(edge.winding)
            };
            let scale = self.incident[edge.child]
                .iter()
                .map(|&(index, _)| circuit.inductors.inductances[index])
                .fold(Value::INFINITY, Value::min);
            for &(index, sign) in &self.incident[edge.child] {
                let weight = sign * (scale / circuit.inductors.inductances[index]);
                if rates.is_some() {
                    visit(row, rate_row(index), weight);
                } else {
                    for (node, coefficient) in [
                        (circuit.inductors.node_pos[index], weight),
                        (circuit.inductors.node_neg[index], -weight),
                    ] {
                        if node > 0 {
                            visit(row, node - 1, coefficient);
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winding_forest_preserves_kcl_and_every_physical_probe_under_terminal_reversal() {
        let ports = [
            ("out", "a"),
            ("a", "b"),
            ("b", "0"),
            ("a", "0"),
            ("b", "out"),
        ];
        let engine = Engine::default();
        for coupling in [0.0, 0.3] {
            for reversed in 0..32 {
                let mut deck =
                    String::from("winding graph conservation\nV1 in 0 1\nR1 in out 100\n");
                for (index, &(mut pos, mut neg)) in ports.iter().enumerate() {
                    if reversed & (1 << index) != 0 {
                        std::mem::swap(&mut pos, &mut neg);
                    }
                    deck.push_str(&format!(
                        "L{} {pos} {neg} {}u\n",
                        index + 1,
                        (index + 1) * 10
                    ));
                }
                deck.push_str(&format!("K1 L1 L5 {coupling}\n.end\n"));
                let netlist = Netlist::parse(&deck).unwrap();
                let mut circuit = PssCircuit::new(engine.build_circuit(&netlist).unwrap());
                let state = [0.2, -0.3, 0.4];
                assert_eq!(circuit.state_dimension(), state.len());
                circuit.set_state(&state).unwrap();
                let basis = &circuit.basis.currents;
                for edges in &basis.incident {
                    let kcl: f64 = edges
                        .iter()
                        .map(|&(index, sign)| sign * circuit.inductors.i_prev[index])
                        .sum();
                    assert!(
                        kcl.abs() < 1e-15,
                        "k={coupling}, reverse={reversed}, KCL={kcl:e}"
                    );
                }
                for index in 0..ports.len() {
                    let projected: f64 = basis
                        .projection(index)
                        .iter()
                        .map(|&(i, weight)| weight * state[i])
                        .sum();
                    assert!((projected - circuit.inductors.i_prev[index]).abs() < 1e-15);
                }
                assert!(
                    engine
                        .pss_initial_node_solution(&mut circuit, &NoAbort)
                        .is_ok(),
                    "consistent flux rates for k={coupling}, reverse={reversed}"
                );
            }
        }
    }

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
                assert_eq!(basis.projection(0), [(0, 1.0)]);
                assert_eq!(basis.projection(1), [(0, -1.0)]);
            }
        }
    }
}
