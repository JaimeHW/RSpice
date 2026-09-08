//! Shooting coordinates and accepted physical charge for the PSS traversal.

use super::*;
use crate::numerics::integration::TwoTerminalChargeHistory;

mod current;
use current::PssCurrentBasis;

#[derive(Debug, Clone, Copy)]
enum VoltageBranch {
    Capacitor(usize),
    Diode(usize),
    Bjt {
        device: usize,
        charge: usize,
        pos: usize,
        neg: usize,
    },
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

/// A spanning forest removes redundant electrical charge-voltage coordinates.
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
        for (device, bjt) in circuit.bjts.devices.iter().enumerate() {
            for (charge, nodes) in bjt
                .vbic_electrical_charge_storage_nodes()
                .into_iter()
                .enumerate()
            {
                if let Some((pos, neg)) = nodes
                    && add(pos, neg, ForestValue::State(voltage_branches.len()))
                {
                    voltage_branches.push(VoltageBranch::Bjt {
                        device,
                        charge,
                        pos,
                        neg,
                    });
                }
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
                VoltageBranch::Bjt { device, charge, .. } => {
                    const NAMES: [&str; 8] =
                        ["qbe", "qbex", "qbc", "qbcx", "qbep", "qbeo", "qbco", "qbcp"];
                    format!("Q:{}:{}", circuit.bjts.devices[device].name, NAMES[charge])
                }
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
            VoltageBranch::Bjt { pos, neg, .. } => (pos, neg),
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
    pub(super) bjt_history: super::super::transient::BjtTransientHistory,
    pub(super) bjt_snapshot_cache: Vec<Option<crate::device::semiconductor::BjtChargeSnapshot>>,
    /// Trial currents computed before a small Newton voltage correction is
    /// rounded into the absolute solution. Read only on accepted steps.
    pub(super) capacitor_trial_currents: Vec<Value>,
    /// Free winding-current offsets from the previous accepted sample,
    /// captured before the Newton correction is rounded into absolute current.
    /// Ephemeral: overwritten before every physical trial residual check.
    inductor_trial_offsets: Vec<[Value; 2]>,
    /// Numerical mesh only. Authored source defaults remain in CircuitData's
    /// SourceTimeBasis and must not change when this grid is refined.
    pub(in crate::engine) integration_steps: usize,
    pub(in crate::engine) integration_mesh: Option<PssIntegrationMesh>,
    /// Qualification-only traversal; published and replayed orbits always
    /// use the configured method. Derivative workers clone this choice.
    pub(super) probe_precision_floor: bool,
    basis: PssStateBasis,
    solution_scratch: Vec<Value>,
    current_balance: Vec<Value>,
    initial_flux_rates: Option<usize>,
    current_source_rates: Vec<Value>,
    current_source_offsets: [Vec<Value>; 3],
    current_source_times: [Value; 2],
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
        let capacitor_trial_currents = vec![0.0; circuit.capacitors.len()];
        let inductor_trial_offsets = vec![[0.0; 2]; circuit.inductors.len()];
        let basis = PssStateBasis::new(&circuit);
        let solution_scratch = vec![0.0; circuit.matrix_size() + 1];
        let current_balance = vec![0.0; basis.currents.workspace_size()];
        let current_source_rates = if basis.currents.has_prescribed_currents() {
            vec![0.0; circuit.inductors.len()]
        } else {
            Vec::new()
        };
        let current_source_offsets = std::array::from_fn(|_| vec![0.0; current_source_rates.len()]);
        let diode_history = TwoTerminalChargeHistory::from_biases(
            circuit
                .diodes
                .devices
                .iter()
                .map(|diode| (0.0, diode.junction_charge_and_capacitance(0.0).0)),
        );
        let bjt_history = Engine::initialize_bjt_history(
            &circuit,
            &solution_scratch[1..],
            super::super::transient::ReactiveHistorySeed::SolvedBias,
        );
        let bjt_snapshot_cache = vec![None; circuit.bjts.len()];
        Self {
            circuit,
            diode_history,
            bjt_history,
            bjt_snapshot_cache,
            capacitor_trial_currents,
            inductor_trial_offsets,
            integration_steps: 0,
            integration_mesh: None,
            probe_precision_floor: false,
            basis,
            solution_scratch,
            current_balance,
            initial_flux_rates: None,
            current_source_rates,
            current_source_offsets,
            current_source_times: [0.0; 2],
        }
    }

    pub(in crate::engine) fn grid_steps(&self, config: &crate::analysis::PssConfig) -> usize {
        self.integration_mesh.as_ref().map_or_else(
            || self.integration_steps.max(config.points_per_period),
            PssIntegrationMesh::steps,
        )
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
                VoltageBranch::Bjt { pos, neg, .. } => {
                    self.solution_scratch[pos] - self.solution_scratch[neg]
                }
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
        self.current_source_times = [0.0; 2];
        assert_eq!(
            state.len(),
            self.state_dimension(),
            "PSS shooting-state shape must match its basis"
        );
        self.solution_scratch.fill(0.0);
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
            self.solution_scratch[edge.to] = self.solution_scratch[edge.from] + edge.sign * value;
        }
        let circuit = &mut self.circuit;
        for (index, stamp) in circuit.capacitors.stamps.iter().enumerate() {
            let voltage = self.solution_scratch[stamp.pp.row] - self.solution_scratch[stamp.nn.row];
            circuit.capacitors.v_prev[index] = voltage;
            circuit.capacitors.v_prev_prev[index] = voltage;
            circuit.capacitors.v_prev_prev_prev[index] = voltage;
            circuit.capacitors.i_prev[index] = 0.0;
            circuit.capacitors.i_eq[index] = 0.0;
        }
        self.diode_history
            .reset_biases(circuit.diodes.devices.iter().map(|diode| {
                let voltage = self.solution_scratch[diode.node_anode]
                    - self.solution_scratch[diode.node_cathode];
                (voltage, diode.junction_charge_and_capacitance(voltage).0)
            }));
        self.basis.currents.source_balance(
            &circuit.current_sources,
            &mut self.current_balance,
            false,
        )?;
        self.basis.currents.set_state(
            &state[self.basis.voltage_branches.len()..],
            &mut circuit.inductors.i_prev,
            &mut self.current_balance,
        );
        for index in 0..circuit.inductors.len() {
            let current = circuit.inductors.i_prev[index];
            circuit.inductors.i_prev_prev[index] = current;
            circuit.inductors.i_prev_prev_prev[index] = current;
            circuit.inductors.v_prev[index] = 0.0;
            self.solution_scratch[circuit.num_nodes() + circuit.inductors.branch_indices[index]] =
                current;
        }
        // Mutual flux shares these physical winding currents. Restart both
        // accepted history generations for each shooting perturbation; a
        // prior period or derivative probe must never leak into the next.
        circuit.reset_coupled_inductor_pair_state(&self.solution_scratch[1..]);
        self.bjt_history = Engine::initialize_bjt_history(
            circuit,
            &self.solution_scratch[1..],
            super::super::transient::ReactiveHistorySeed::SolvedBias,
        );
        self.bjt_snapshot_cache.fill(None);
        Ok(())
    }

    pub(super) fn seed_bjt_history(&mut self, solution: &[Value]) {
        self.solution_scratch[1..].copy_from_slice(solution);
        self.bjt_history = Engine::initialize_bjt_history(
            &self.circuit,
            solution,
            super::super::transient::ReactiveHistorySeed::SolvedBias,
        );
        self.bjt_snapshot_cache.fill(None);
    }

    pub(super) fn accept_node_solution(&mut self, solution: &[Value]) {
        self.solution_scratch[1..].copy_from_slice(solution);
    }

    pub(super) fn initial_solution_guess(&self) -> Vec<Value> {
        let mut solution = vec![0.0; self.matrix_size()];
        solution[..self.solution_scratch.len() - 1].copy_from_slice(&self.solution_scratch[1..]);
        solution
    }

    /// An exact initialization constraint carries displacement current while
    /// fixing the accepted branch voltage. Adding only forest coordinates
    /// avoids redundant ideal-source loops for parallel charge branches.
    pub(super) fn add_initial_constraints(&mut self) -> Result<(), SimulationError> {
        self.basis.currents.source_balance(
            &self.circuit.current_sources,
            &mut self.current_balance,
            true,
        )?;
        for index in 0..self.basis.voltage_branches.len() {
            let (pos, neg) = self.basis.voltage_nodes(&self.circuit, index);
            let value = match self.basis.voltage_branches[index] {
                VoltageBranch::Capacitor(index) => self.capacitors.v_prev[index],
                VoltageBranch::Diode(index) => self.diode_history.vd_prev[index],
                VoltageBranch::Bjt { pos, neg, .. } => {
                    self.solution_scratch[pos] - self.solution_scratch[neg]
                }
            };
            // An IC capacitor already owns a physical current unknown. Reuse
            // it for the voltage constraint rather than leaving its original
            // row empty beside an unnecessary auxiliary branch.
            let existing_branch = match self.basis.voltage_branches[index] {
                VoltageBranch::Capacitor(index) => self.capacitors.ic_branch_indices[index],
                VoltageBranch::Diode(_) | VoltageBranch::Bjt { .. } => None,
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
        if self.basis.currents.needs_flux_rates() {
            let first = self.circuit.allocate_branch();
            self.initial_flux_rates = Some(first);
            for _ in 1..self.inductors.len() {
                self.circuit.allocate_branch();
            }
        }
        Ok(())
    }

    pub(super) fn initial_extra_pattern(&self) -> Vec<(usize, usize)> {
        let mut entries = Vec::new();
        self.basis.currents.initial_matrix(
            &self.circuit,
            self.initial_flux_rates,
            |row, col, _| entries.push((row, col)),
        );
        entries
    }

    pub(super) fn has_initial_flux_rates(&self) -> bool {
        self.initial_flux_rates.is_some()
    }

    pub(super) fn stamp_initial_inductor_constraints(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
    ) -> Result<(), SimulationError> {
        self.basis.currents.initial_matrix(
            &self.circuit,
            self.initial_flux_rates,
            |row, col, weight| matrix.add(row, col, weight),
        );
        for (row, index) in self
            .basis
            .currents
            .current_rows(&self.circuit, self.initial_flux_rates)
        {
            rhs[row] = self.inductors.i_prev[index];
        }
        self.basis.currents.initial_rate_rhs(
            &self.circuit,
            self.initial_flux_rates,
            &self.current_balance,
            |row, value| rhs[row] = value,
        )?;
        Ok(())
    }

    pub(super) fn is_initial_current_row(&self, row: usize) -> bool {
        self.basis
            .currents
            .is_current_row(&self.circuit, row, self.initial_flux_rates)
    }

    pub(super) fn initialize_prescribed_currents(&mut self) -> Result<(), SimulationError> {
        if !self.current_source_rates.is_empty() {
            self.set_state(&self.extract_state())?;
        }
        Ok(())
    }

    pub(super) fn ensure_regular_prescribed_currents(
        &self,
        period: Value,
    ) -> Result<(), SimulationError> {
        self.basis
            .currents
            .ensure_regular_forcing(&self.circuit.current_sources, period)
    }

    pub(super) fn stamp_prescribed_current_correction(
        &mut self,
        rhs: &mut [Value],
        step: PssCompanionStep<'_>,
    ) -> Result<(), SimulationError> {
        if !self.current_source_rates.is_empty() {
            self.basis.currents.source_companion(
                &self.circuit.current_sources,
                &mut self.current_balance,
                &mut self.current_source_rates,
                &mut self.current_source_offsets,
                self.current_source_times,
                step,
            )?;
            self.basis.currents.add_flux_rhs(
                &self.circuit,
                |index| {
                    self.current_source_rates[index]
                        - step.coeff.inductor_charge_derivative_correction(
                            1.0,
                            step.dt,
                            self.current_source_offsets[0][index],
                            self.current_source_offsets[1][index],
                            self.current_source_offsets[2][index],
                        )
                },
                rhs,
            )?;
        }
        Ok(())
    }

    pub(super) fn accept_source_time(&mut self, time: Value) {
        self.current_source_times = [time, self.current_source_times[0]];
    }

    pub(super) fn capture_capacitor_trial_currents(
        &mut self,
        iterate: &[Value],
        correction: &[Value],
        step: PssCompanionStep<'_>,
    ) {
        let caps = &self.circuit.capacitors;
        let voltage = |values: &[Value], node| if node == 0 { 0.0 } else { values[node - 1] };
        for (index, stamp) in caps.stamps.iter().enumerate() {
            self.capacitor_trial_currents[index] =
                if let Some(branch) = caps.ic_branch_indices[index] {
                    let row = self.circuit.num_nodes() + branch - 1;
                    iterate[row] + correction[row]
                } else {
                    let present = step.coeff.capacitor_current(
                        caps.capacitances[index],
                        step.dt,
                        voltage(iterate, stamp.pp.row) - voltage(iterate, stamp.nn.row),
                        caps.v_prev[index],
                        caps.v_prev_prev[index],
                        caps.i_prev[index],
                    );
                    step.coeff
                        .capacitor_geq(caps.capacitances[index], step.dt)
                        .mul_add(
                            voltage(correction, stamp.pp.row) - voltage(correction, stamp.nn.row),
                            present,
                        )
                };
        }
    }

    pub(super) fn capture_inductor_trial_offsets(
        &mut self,
        iterate: &[Value],
        correction: &[Value],
    ) {
        for (index, &branch) in self.circuit.inductors.branch_indices.iter().enumerate() {
            let row = self.circuit.num_nodes() + branch - 1;
            let samples = [
                iterate[row],
                self.circuit.inductors.i_prev[index],
                self.circuit.inductors.i_prev_prev[index],
            ];
            let [present, previous, older] = if self.current_source_rates.is_empty() {
                samples
            } else {
                self.basis.currents.free_current_samples(
                    branch,
                    samples,
                    &self.current_source_offsets,
                )
            };
            self.inductor_trial_offsets[index] =
                [(present - previous) + correction[row], older - previous];
        }
    }

    /// Evaluate winding equations from flux differences, sharing TRAN's
    /// cancellation-resistant residual and the affine forcing stamped by PSS.
    /// The caller has just stamped this same trial, so the forcing workspace
    /// contains its prescribed current samples and analytic rates.
    pub(super) fn stabilize_inductor_correction_rhs(
        &self,
        rhs: &mut [Value],
        iterate: &[Value],
        step: PssCompanionStep<'_>,
        trial: bool,
    ) -> Result<(), SimulationError> {
        self.circuit
            .stabilize_inductor_transient_correction_rhs_with_current_map(
                rhs,
                iterate,
                step.dt,
                step.coeff,
                |branch, samples| {
                    let branch = branch - self.circuit.num_nodes();
                    if trial {
                        let index = self.basis.currents.winding_index(branch);
                        let [present, older] = self.inductor_trial_offsets[index];
                        [present, 0.0, older]
                    } else {
                        let [present, previous, older] = if self.current_source_rates.is_empty() {
                            samples
                        } else {
                            self.basis.currents.free_current_samples(
                                branch,
                                samples,
                                &self.current_source_offsets,
                            )
                        };
                        // Use the same origin in the Newton residual and
                        // its trial certificate. Multiplying absolute
                        // currents by L before differencing would restore
                        // the flux-rounding error on every iteration.
                        [present - previous, 0.0, older - previous]
                    }
                },
            );
        if !self.current_source_rates.is_empty() {
            self.basis.currents.add_flux_rhs(
                &self.circuit,
                |index| self.current_source_rates[index],
                rhs,
            )?;
        }
        Ok(())
    }

    /// A physical winding current may be a signed sum of independent states.
    pub(in crate::engine) fn inductor_probe_projection(
        &self,
        name: &str,
    ) -> Option<(String, Vec<(usize, Value)>)> {
        let index = self
            .inductors
            .names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(name))?;
        let offset = self.basis.voltage_branches.len();
        Some((
            self.inductors.names[index].clone(),
            self.basis
                .currents
                .projection(index)
                .into_iter()
                .map(|(state, weight)| (state + offset, weight))
                .collect(),
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
    fn vbic_storage_forest_removes_constant_charge_and_parallel_coordinates() {
        for (extra, state_names) in [
            ("", vec!["Q:Q1:qbe"]),
            ("C1 b 0 1p\n", vec!["C:C1"]),
            ("Vb b 0 0.1\n", Vec::new()),
        ] {
            let netlist = Netlist::parse(&format!("VBIC state forest\nVc c 0 1\nR1 b 0 1k\nQ1 c b 0 vm\n.model vm NPN(LEVEL=4 CJE=10p CJC=20p CBEO=30p CBCO=40p RCX=0 RCI=0 RBX=0 RBI=0 RBP=0 QCO=1p GAMM=0 TF=0 TR=0)\n{extra}.end\n")).unwrap();
            let mut circuit = PssCircuit::new(Engine::default().build_circuit(&netlist).unwrap());
            assert_eq!(circuit.basis.names(&circuit), state_names);
            let state = vec![0.1; state_names.len()];
            circuit.set_state(&state).unwrap();
            assert_eq!(circuit.extract_state(), state);
            assert_eq!(circuit.clone().bjt_history, circuit.bjt_history);
        }
    }

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
