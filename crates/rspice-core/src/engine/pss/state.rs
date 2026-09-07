//! Shooting coordinates and accepted physical charge for the PSS traversal.

use super::*;
use crate::numerics::integration::TwoTerminalChargeHistory;

mod current;
use current::PssCurrentBasis;

#[derive(Debug, Clone, Copy)]
enum VoltageBranch {
    Capacitor(usize),
    Diode(usize),
}

#[derive(Debug, Clone, Copy)]
enum ForestValue {
    State(usize),
    Source(usize),
    BehavioralSource(usize),
}

#[derive(Debug, Clone, Copy)]
struct ForestEdge {
    from: usize,
    to: usize,
    value: ForestValue,
    sign: Value,
}

/// A spanning forest removes redundant capacitor/diode voltage coordinates.
/// Independent voltage sources enter the forest first: their prescribed
/// voltages are constraints, never shooting unknowns. The remaining branch
/// voltages follow from this forest, including opposite terminal orientations
/// and charge branches parallel to, or in loops with, other branches.
#[derive(Debug, Clone)]
pub(super) struct PssStateBasis {
    voltage_branches: Vec<VoltageBranch>,
    forest: Vec<ForestEdge>,
    currents: PssCurrentBasis,
}

impl PssStateBasis {
    pub(super) fn new(circuit: &CircuitData) -> Self {
        let node_count = circuit.num_nodes() + 1;
        let mut parents: Vec<usize> = (0..node_count).collect();
        fn root(parents: &mut [usize], mut node: usize) -> usize {
            while parents[node] != node {
                parents[node] = parents[parents[node]];
                node = parents[node];
            }
            node
        }
        let mut adjacency = vec![Vec::new(); node_count];
        let mut voltage_branches = Vec::new();
        let mut add = |pos: usize, neg: usize, value: ForestValue| {
            let rp = root(&mut parents, pos);
            let rn = root(&mut parents, neg);
            if rp == rn {
                return false;
            }
            parents[rp] = rn;
            adjacency[pos].push((neg, value, -1.0));
            adjacency[neg].push((pos, value, 1.0));
            true
        };
        for (index, (&pos, &neg)) in circuit
            .voltage_sources
            .node_pos
            .iter()
            .zip(&circuit.voltage_sources.node_neg)
            .enumerate()
        {
            add(pos, neg, ForestValue::Source(index));
        }
        for (index, source) in circuit
            .behavioral_sources
            .voltage_sources
            .iter()
            .enumerate()
        {
            if !source.is_solution_dependent() {
                add(
                    source.node_pos,
                    source.node_neg,
                    ForestValue::BehavioralSource(index),
                );
            }
        }
        for (index, stamp) in circuit.capacitors.stamps.iter().enumerate() {
            if circuit.capacitors.capacitances[index] != 0.0
                && add(
                    stamp.pp.row,
                    stamp.nn.row,
                    ForestValue::State(voltage_branches.len()),
                )
            {
                voltage_branches.push(VoltageBranch::Capacitor(index));
            }
        }
        for (index, diode) in circuit.diodes.devices.iter().enumerate() {
            if diode.has_charge_storage()
                && add(
                    diode.node_anode,
                    diode.node_cathode,
                    ForestValue::State(voltage_branches.len()),
                )
            {
                voltage_branches.push(VoltageBranch::Diode(index));
            }
        }
        let mut visited = vec![false; node_count];
        let mut forest = Vec::new();
        let mut pending = Vec::new();
        for start in 0..node_count {
            if visited[start] {
                continue;
            }
            visited[start] = true;
            pending.push(start);
            while let Some(from) = pending.pop() {
                for &(to, value, sign) in &adjacency[from] {
                    if !visited[to] {
                        visited[to] = true;
                        forest.push(ForestEdge {
                            from,
                            to,
                            value,
                            sign,
                        });
                        pending.push(to);
                    }
                }
            }
        }
        Self {
            voltage_branches,
            forest,
            currents: PssCurrentBasis::new(circuit),
        }
    }

    pub(super) fn names(&self, circuit: &CircuitData) -> Vec<String> {
        self.voltage_branches
            .iter()
            .map(|branch| match *branch {
                VoltageBranch::Capacitor(index) => format!("C:{}", circuit.capacitors.names[index]),
                VoltageBranch::Diode(index) => format!("D:{}", circuit.diodes.devices[index].name),
            })
            .chain(
                self.currents
                    .representatives
                    .iter()
                    .map(|&index| format!("L:{}", circuit.inductors.names[index])),
            )
            .collect()
    }

    fn voltage_nodes(&self, circuit: &CircuitData, index: usize) -> (usize, usize) {
        match self.voltage_branches[index] {
            VoltageBranch::Capacitor(index) => {
                let stamp = &circuit.capacitors.stamps[index];
                (stamp.pp.row, stamp.nn.row)
            }
            VoltageBranch::Diode(index) => {
                let diode = &circuit.diodes.devices[index];
                (diode.node_anode, diode.node_cathode)
            }
        }
    }
}

/// A worker owns its circuit, accepted charge and coordinate basis together.
/// Cloning for a derivative probe captures every accepted diode history lane;
/// each period then resets those histories from the supplied shooting state.
#[derive(Debug, Clone)]
pub(in crate::engine) struct PssCircuit {
    pub(super) circuit: CircuitData,
    pub(super) diode_history: TwoTerminalChargeHistory,
    basis: PssStateBasis,
    voltage_scratch: Vec<Value>,
}

impl std::ops::Deref for PssCircuit {
    type Target = CircuitData;
    fn deref(&self) -> &Self::Target {
        &self.circuit
    }
}

impl std::ops::DerefMut for PssCircuit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.circuit
    }
}

impl PssCircuit {
    pub(in crate::engine) fn new(circuit: CircuitData) -> Self {
        let basis = PssStateBasis::new(&circuit);
        let voltage_scratch = vec![0.0; circuit.num_nodes() + 1];
        let diode_history = TwoTerminalChargeHistory::from_biases(
            circuit
                .diodes
                .devices
                .iter()
                .map(|diode| (0.0, diode.junction_charge_and_capacitance(0.0).0)),
        );
        Self {
            circuit,
            diode_history,
            basis,
            voltage_scratch,
        }
    }

    pub(in crate::engine) fn state_dimension(&self) -> usize {
        self.basis.voltage_branches.len() + self.basis.currents.representatives.len()
    }

    pub(super) fn extract_state(&self) -> Vec<Value> {
        self.basis
            .voltage_branches
            .iter()
            .map(|branch| match *branch {
                VoltageBranch::Capacitor(index) => self.capacitors.v_prev[index],
                VoltageBranch::Diode(index) => self.diode_history.vd_prev[index],
            })
            .chain(
                self.basis
                    .currents
                    .representatives
                    .iter()
                    .map(|&index| self.inductors.i_prev[index]),
            )
            .collect()
    }

    pub(super) fn set_state(&mut self, state: &[Value]) -> Result<(), SimulationError> {
        assert_eq!(
            state.len(),
            self.state_dimension(),
            "PSS shooting-state shape must match its basis"
        );
        self.voltage_scratch.fill(0.0);
        for edge in &self.basis.forest {
            let value = match edge.value {
                ForestValue::State(index) => state[index],
                ForestValue::Source(index) => {
                    self.circuit.voltage_sources.transient_value_at(index, 0.0)
                }
                ForestValue::BehavioralSource(index) => {
                    self.circuit.behavioral_sources.voltage_sources[index]
                        .evaluate(&[], 0.0)
                        .map_err(|error| SimulationError::Circuit(error.to_string()))?
                }
            };
            self.voltage_scratch[edge.to] = self.voltage_scratch[edge.from] + edge.sign * value;
        }
        let circuit = &mut self.circuit;
        for (index, stamp) in circuit.capacitors.stamps.iter().enumerate() {
            let voltage = self.voltage_scratch[stamp.pp.row] - self.voltage_scratch[stamp.nn.row];
            circuit.capacitors.v_prev[index] = voltage;
            circuit.capacitors.v_prev_prev[index] = voltage;
            circuit.capacitors.v_prev_prev_prev[index] = voltage;
            circuit.capacitors.i_prev[index] = 0.0;
            circuit.capacitors.i_eq[index] = 0.0;
        }
        self.diode_history
            .reset_biases(circuit.diodes.devices.iter().map(|diode| {
                let voltage = self.voltage_scratch[diode.node_anode]
                    - self.voltage_scratch[diode.node_cathode];
                (voltage, diode.junction_charge_and_capacitance(voltage).0)
            }));
        for (index, coordinate) in self.basis.currents.coordinates.iter().enumerate() {
            let current =
                coordinate.sign * state[self.basis.voltage_branches.len() + coordinate.state];
            circuit.inductors.i_prev[index] = current;
            circuit.inductors.i_prev_prev[index] = current;
            circuit.inductors.i_prev_prev_prev[index] = current;
            circuit.inductors.v_prev[index] = 0.0;
        }
        Ok(())
    }

    /// An exact initialization constraint carries displacement current while
    /// fixing the accepted branch voltage. Adding only forest coordinates
    /// avoids redundant ideal-source loops for parallel charge branches.
    pub(super) fn add_initial_voltage_constraints(&mut self) {
        for index in 0..self.basis.voltage_branches.len() {
            let (pos, neg) = self.basis.voltage_nodes(&self.circuit, index);
            let value = match self.basis.voltage_branches[index] {
                VoltageBranch::Capacitor(index) => self.capacitors.v_prev[index],
                VoltageBranch::Diode(index) => self.diode_history.vd_prev[index],
            };
            // An IC capacitor already owns a physical current unknown. Reuse
            // it for the voltage constraint rather than leaving its original
            // row empty beside an unnecessary auxiliary branch.
            let existing_branch = match self.basis.voltage_branches[index] {
                VoltageBranch::Capacitor(index) => self.capacitors.ic_branch_indices[index],
                VoltageBranch::Diode(_) => None,
            };
            let branch = existing_branch.unwrap_or_else(|| self.circuit.allocate_branch());
            self.circuit.voltage_sources.add(
                format!("@pss:charge:{index}"),
                pos,
                neg,
                branch,
                value,
            );
        }
    }

    pub(super) fn initial_extra_pattern(&self) -> Vec<(usize, usize)> {
        let mut entries = Vec::new();
        self.basis
            .currents
            .voltage_constraints(&self.circuit, |row, col, _| entries.push((row, col)));
        entries
    }

    pub(super) fn stamp_initial_inductor_constraints(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
    ) {
        for &index in &self.basis.currents.representatives {
            let row = self.num_nodes() + self.inductors.branch_indices[index] - 1;
            matrix.add(row, row, 1.0);
            rhs[row] = self.inductors.i_prev[index];
        }
        self.basis
            .currents
            .voltage_constraints(&self.circuit, |row, col, weight| {
                matrix.add(row, col, weight)
            });
    }

    pub(super) fn is_initial_current_row(&self, row: usize) -> bool {
        self.basis
            .currents
            .representatives
            .iter()
            .any(|&index| row == self.num_nodes() + self.inductors.branch_indices[index] - 1)
    }

    /// Series currents differ only by orientation, so their absolute modal
    /// participation resolves to the same independent current coordinate.
    pub(in crate::engine) fn inductor_probe_coordinate(
        &self,
        name: &str,
    ) -> Option<(String, usize)> {
        let index = self
            .inductors
            .names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(name))?;
        Some((
            self.inductors.names[index].clone(),
            self.basis.voltage_branches.len() + self.basis.currents.coordinates[index].state,
        ))
    }

    /// Map an MNA solution perturbation into the same independent coordinates
    /// used by shooting and the oscillator-noise adjoint.
    pub(in crate::engine) fn project_perturbation<'a>(
        &'a self,
        solution: &'a [Value],
    ) -> impl Iterator<Item = Value> + 'a {
        let voltage = move |node| if node == 0 { 0.0 } else { solution[node - 1] };
        (0..self.basis.voltage_branches.len())
            .map(move |index| {
                let (pos, neg) = self.basis.voltage_nodes(&self.circuit, index);
                voltage(pos) - voltage(neg)
            })
            .chain(
                self.basis
                    .currents
                    .representatives
                    .iter()
                    .map(move |&index| {
                        solution[self.num_nodes() + self.inductors.branch_indices[index] - 1]
                    }),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn voltage_forest_preserves_all_charge_branches_in_a_loop() {
        let netlist = Netlist::parse("charge loop\nV1 a 0 2\nR1 b 0 1k\nR2 c 0 1k\nC1 a b 1n\nC2 b c 2n\nC3 a c 3n\nD1 c a dm\n.model dm D(IS=1e-30 CJO=1n M=0)\n.end\n").unwrap();
        let mut circuit = PssCircuit::new(Engine::default().build_circuit(&netlist).unwrap());
        assert_eq!(circuit.state_dimension(), 2);
        assert_eq!(circuit.basis.names(&circuit), ["C:C1", "C:C2"]);
        circuit.set_state(&[0.25, -0.5]).unwrap();
        assert_eq!(circuit.capacitors.v_prev, [0.25, -0.5, -0.25]);
        assert_eq!(circuit.diode_history.vd_prev, [0.25]);
        assert!((circuit.diode_history.qd_prev[0] - 0.25e-9).abs() < 1e-24);
        assert_eq!(circuit.extract_state(), [0.25, -0.5]);
        let copy = circuit.clone();
        assert_eq!(copy.diode_history, circuit.diode_history);
        assert_eq!(copy.extract_state(), circuit.extract_state());
    }
}
