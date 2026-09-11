//! Shooting coordinates and accepted physical charge for the PSS traversal.

use super::*;
use crate::numerics::integration::TwoTerminalChargeHistory;

mod current;
use current::PssCurrentBasis;
mod voltage;
use voltage::{
    InitialChargeRates, PssDescriptor, PssVoltageConstraintBuilder, PssVoltageConstraints,
};
mod forcing;
use forcing::BehavioralForcing;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ConstraintSource {
    Voltage,
    Current,
    BehavioralVoltage,
    BehavioralCurrent,
    Diode,
    DiodeVoltage,
}

impl ConstraintSource {
    fn is_behavioral(self) -> bool {
        matches!(self, Self::BehavioralVoltage | Self::BehavioralCurrent)
    }

    fn is_nonlinear(self) -> bool {
        matches!(self, Self::Diode | Self::DiodeVoltage)
    }
}

/// Physical units of a node-coordinate unknown. Thermal equations use kelvin
/// rise and watts; their numerical absolute scale follows the node tolerance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CoordinateUnit {
    Voltage,
    Temperature,
    Current,
}

#[derive(Debug, Clone, Copy)]
enum ChargeBranch {
    Capacitor(usize),
    Diode(usize),
    Bjt {
        device: usize,
        charge: usize,
        pos: usize,
        neg: usize,
    },
    Jfet {
        device: usize,
        charge: usize,
        pos: usize,
        neg: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ForestValue {
    Zero,
    State(usize),
    CurrentState(usize),
    SourceDerivative {
        index: usize,
        order: usize,
        kind: ConstraintSource,
    },
    Source(usize),
    BehavioralSource(usize),
}

impl ForestValue {
    fn source(self) -> Option<(ConstraintSource, usize, usize)> {
        match self {
            Self::Source(index) => Some((ConstraintSource::Voltage, index, 0)),
            Self::BehavioralSource(index) => Some((ConstraintSource::BehavioralVoltage, index, 0)),
            Self::SourceDerivative { index, order, kind } => Some((kind, index, order)),
            _ => None,
        }
    }

    fn differentiated(self) -> Result<Self, SimulationError> {
        let Some((kind, index, order)) = self.source() else {
            return Err(SimulationError::Circuit(
                "PSS descriptor attempted to differentiate a free state as forcing".to_owned(),
            ));
        };
        Ok(Self::SourceDerivative {
            index,
            order: order.checked_add(1).ok_or_else(|| {
                SimulationError::Circuit("PSS source derivative order overflow".to_owned())
            })?,
            kind,
        })
    }

    fn forcing(
        self,
        circuit: &CircuitData,
        time: Value,
        extra_order: usize,
    ) -> Result<Value, SimulationError> {
        let Some((kind, index, order)) = self.source().filter(|(kind, _, _)| {
            matches!(kind, ConstraintSource::Voltage | ConstraintSource::Current)
        }) else {
            return Err(SimulationError::Circuit(
                "PSS descriptor forcing is not an independent source".to_owned(),
            ));
        };
        let order = order.checked_add(extra_order).ok_or_else(|| {
            SimulationError::Circuit("PSS source derivative order overflow".to_owned())
        })?;
        let (name, value) = if kind == ConstraintSource::Current {
            (
                &circuit.current_sources.names[index],
                circuit
                    .current_sources
                    .time_derivative_at(index, time, order),
            )
        } else {
            (
                &circuit.voltage_sources.names[index],
                circuit
                    .voltage_sources
                    .time_derivative_at(index, time, order),
            )
        };
        value.filter(|value| value.is_finite()).ok_or_else(|| SimulationError::Circuit(format!(
            "PSS coupled state constraint requires a finite analytic derivative of order {order} for source {name} at t={time:e}"
        )))
    }

    fn evaluate(
        self,
        circuit: &mut CircuitData,
        state: &[Value],
        voltage_count: usize,
    ) -> Result<Value, SimulationError> {
        match self {
            Self::Zero => Ok(0.0),
            Self::State(index) => Ok(state[index]),
            Self::CurrentState(index) => Ok(state[voltage_count + index]),
            Self::Source(_) | Self::SourceDerivative { .. } => self.forcing(circuit, 0.0, 0),
            Self::BehavioralSource(index) => circuit.behavioral_sources.voltage_sources[index]
                .evaluate(&[], 0.0)
                .map_err(|error| SimulationError::Circuit(error.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ForestEdge {
    from: usize,
    to: usize,
    value: ForestValue,
    sign: Value,
}

/// A spanning forest removes redundant storage-node coordinates.
/// Ideal amplifier relations use an exact sparse reduction instead, retaining
/// independent differential charge voltages through cascaded control paths.
/// Independent voltage sources enter the forest first: their prescribed
/// voltages are constraints, never shooting unknowns. Exact zero-ohm branches
/// prescribe zero voltage in the same forest. The remaining branch
/// voltages follow from this forest, including opposite terminal orientations
/// and charge branches parallel to, or in loops with, other branches.
#[derive(Debug, Clone)]
pub(super) struct PssStateBasis {
    charge_branches: Vec<ChargeBranch>,
    node_units: Vec<CoordinateUnit>,
    forest: Vec<ForestEdge>,
    voltage_constraints: Option<PssVoltageConstraints>,
    currents: PssCurrentBasis,
    descriptor: Option<PssDescriptor>,
}

impl PssStateBasis {
    pub(super) fn new(
        circuit: &CircuitData,
        limits: crate::resource::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        if PssDescriptor::applies(circuit)
            && let Some(basis) = PssDescriptor::build(circuit, limits, abort)?
        {
            return Ok(basis);
        }
        let node_count = circuit.num_nodes() + 1;
        let mut voltage_constraints = if circuit.vcvs.is_empty() {
            None
        } else {
            let mut constraints = PssVoltageConstraintBuilder::new(node_count, limits)?;
            for index in 0..circuit.vcvs.len() {
                constraints.add(
                    [
                        (circuit.vcvs.node_pos[index], 1.0),
                        (circuit.vcvs.node_neg[index], -1.0),
                        (circuit.vcvs.ctrl_pos[index], -circuit.vcvs.gains[index]),
                        (circuit.vcvs.ctrl_neg[index], circuit.vcvs.gains[index]),
                    ],
                    ForestValue::Zero,
                    abort,
                )?;
            }
            Some(constraints)
        };
        let forest_node_count = if voltage_constraints.is_some() {
            0
        } else {
            node_count
        };
        let mut parents: Vec<usize> = (0..forest_node_count).collect();
        fn root(parents: &mut [usize], mut node: usize) -> usize {
            while parents[node] != node {
                parents[node] = parents[parents[node]];
                node = parents[node];
            }
            node
        }
        let mut adjacency = vec![Vec::new(); forest_node_count];
        let mut charge_branches = Vec::new();
        let mut add = |pos: usize, neg: usize, value: ForestValue| {
            if let Some(constraints) = &mut voltage_constraints {
                return constraints.add([(pos, 1.0), (neg, -1.0)], value, abort);
            }
            let rp = root(&mut parents, pos);
            let rn = root(&mut parents, neg);
            if rp == rn {
                return Ok(false);
            }
            parents[rp] = rn;
            adjacency[pos].push((neg, value, -1.0));
            adjacency[neg].push((pos, value, 1.0));
            Ok(true)
        };
        for (index, (&pos, &neg)) in circuit
            .voltage_sources
            .node_pos
            .iter()
            .zip(&circuit.voltage_sources.node_neg)
            .enumerate()
        {
            add(pos, neg, ForestValue::Source(index))?;
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
                )?;
            }
        }
        // Use the effective large-signal resistance, not the reported value,
        // AC resistance or branch-selection tolerance. A finite nonzero R
        // still leaves a charge coordinate free, however small it is.
        for ((&pos, &neg), &resistance) in circuit
            .resistor_branches
            .node_pos
            .iter()
            .zip(&circuit.resistor_branches.node_neg)
            .zip(&circuit.resistor_branches.resistances)
        {
            if resistance == 0.0 {
                add(pos, neg, ForestValue::Zero)?;
            }
        }
        for (index, stamp) in circuit.capacitors.stamps.iter().enumerate() {
            if circuit.capacitors.capacitances[index] != 0.0
                && add(
                    stamp.pp.row,
                    stamp.nn.row,
                    ForestValue::State(charge_branches.len()),
                )?
            {
                charge_branches.push(ChargeBranch::Capacitor(index));
            }
        }
        for (index, diode) in circuit.diodes.devices.iter().enumerate() {
            if diode.has_charge_storage()
                && add(
                    diode.node_anode,
                    diode.node_cathode,
                    ForestValue::State(charge_branches.len()),
                )?
            {
                charge_branches.push(ChargeBranch::Diode(index));
            }
        }
        for (device, bjt) in circuit.bjts.devices.iter().enumerate() {
            for (charge, nodes) in bjt.charge_storage_nodes().into_iter().enumerate() {
                if let Some((pos, neg)) = nodes
                    && add(pos, neg, ForestValue::State(charge_branches.len()))?
                {
                    charge_branches.push(ChargeBranch::Bjt {
                        device,
                        charge,
                        pos,
                        neg,
                    });
                }
            }
        }
        for (device, jfet) in circuit.jfets.iter().enumerate() {
            for (charge, nodes) in jfet.classic_charge_storage_nodes().into_iter().enumerate() {
                if let Some((pos, neg)) = nodes
                    && add(pos, neg, ForestValue::State(charge_branches.len()))?
                {
                    charge_branches.push(ChargeBranch::Jfet {
                        device,
                        charge,
                        pos,
                        neg,
                    });
                }
            }
        }
        let mut visited = vec![false; forest_node_count];
        let mut forest = Vec::new();
        let mut pending = Vec::new();
        for start in 0..forest_node_count {
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
        let mut node_units = vec![CoordinateUnit::Voltage; node_count];
        for bjt in &circuit.bjts.devices {
            if bjt.node_rth != 0 {
                node_units[bjt.node_rth] = CoordinateUnit::Temperature;
            }
            for node in [bjt.node_xf1, bjt.node_xf2] {
                if node != 0 {
                    node_units[node] = CoordinateUnit::Current;
                }
            }
        }
        Ok(Self {
            charge_branches,
            node_units,
            forest,
            voltage_constraints: voltage_constraints
                .map(|constraints| constraints.finish(circuit, abort))
                .transpose()?,
            currents: PssCurrentBasis::new(circuit),
            descriptor: None,
        })
    }

    pub(super) fn names(&self, circuit: &CircuitData) -> Vec<String> {
        self.charge_branches
            .iter()
            .map(|branch| match *branch {
                ChargeBranch::Capacitor(index) => format!("C:{}", circuit.capacitors.names[index]),
                ChargeBranch::Diode(index) => format!("D:{}", circuit.diodes.devices[index].name),
                ChargeBranch::Bjt { device, charge, .. } => {
                    const NAMES: [&str; 11] = [
                        "qbe", "qbex", "qbc", "qbcx", "qbep", "qbeo", "qbco", "qbcp", "qcth",
                        "qxf1", "qxf2",
                    ];
                    let bjt = &circuit.bjts.devices[device];
                    let name = if bjt.uses_legacy_gummel_poon() {
                        match charge {
                            3 => "qbx",
                            7 => "qcs",
                            _ => NAMES[charge],
                        }
                    } else {
                        NAMES[charge]
                    };
                    format!("Q:{}:{}", bjt.name, name)
                }
                ChargeBranch::Jfet { device, charge, .. } => {
                    format!(
                        "J:{}:{}",
                        circuit.jfets[device].name,
                        ["qgs", "qgd"][charge]
                    )
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
        match self.charge_branches[index] {
            ChargeBranch::Capacitor(index) => {
                let stamp = &circuit.capacitors.stamps[index];
                (stamp.pp.row, stamp.nn.row)
            }
            ChargeBranch::Diode(index) => {
                let diode = &circuit.diodes.devices[index];
                (diode.node_anode, diode.node_cathode)
            }
            ChargeBranch::Bjt { pos, neg, .. } | ChargeBranch::Jfet { pos, neg, .. } => (pos, neg),
        }
    }
}

/// A worker owns its circuit, accepted charge and coordinate basis together.
/// Cloning for a derivative probe captures every accepted semiconductor history
/// lane; each period resets those histories from the supplied shooting state.
#[derive(Debug, Clone)]
pub(in crate::engine) struct PssCircuit {
    pub(super) circuit: CircuitData,
    pub(super) diode_history: TwoTerminalChargeHistory,
    pub(super) bjt_history: super::super::transient::BjtTransientHistory,
    pub(super) jfet_history: super::super::transient::JfetTransientHistory,
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
    initial_charge_rates: Option<InitialChargeRates>,
    current_source_rates: Vec<Value>,
    current_source_offsets: [Vec<Value>; 3],
    charge_source_rates: Vec<Value>,
    charge_source_offsets: [Vec<Value>; 3],
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
    #[cfg(test)]
    pub(in crate::engine) fn new(circuit: CircuitData) -> Result<Self, SimulationError> {
        Self::new_with_abort(
            circuit,
            crate::resource::ResourceLimits::default(),
            &crate::abort_signal::NoAbort,
        )
    }

    pub(in crate::engine) fn new_with_abort(
        circuit: CircuitData,
        limits: crate::resource::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        let capacitor_trial_currents = vec![0.0; circuit.capacitors.len()];
        let inductor_trial_offsets = vec![[0.0; 2]; circuit.inductors.len()];
        let basis = PssStateBasis::new(&circuit, limits, abort)?;
        let solution_scratch = vec![0.0; circuit.matrix_size() + 1];
        let current_balance = vec![0.0; basis.currents.workspace_size()];
        let current_source_rates = if basis.descriptor.as_ref().map_or_else(
            || basis.currents.has_prescribed_currents(),
            |descriptor| descriptor.has_prescribed_currents(&circuit),
        ) {
            vec![0.0; circuit.inductors.len()]
        } else {
            Vec::new()
        };
        let current_source_offsets = std::array::from_fn(|_| vec![0.0; current_source_rates.len()]);
        let charge_source_rates = if basis
            .descriptor
            .as_ref()
            .is_some_and(PssDescriptor::has_prescribed_charge)
        {
            vec![0.0; circuit.capacitors.len()]
        } else {
            Vec::new()
        };
        let charge_source_offsets = std::array::from_fn(|_| vec![0.0; charge_source_rates.len()]);
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
        let jfet_history = Engine::initialize_jfet_history(
            &circuit,
            &solution_scratch[1..],
            super::super::transient::ReactiveHistorySeed::SolvedBias,
        );
        Ok(Self {
            circuit,
            diode_history,
            bjt_history,
            jfet_history,
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
            initial_charge_rates: None,
            current_source_rates,
            current_source_offsets,
            charge_source_rates,
            charge_source_offsets,
            current_source_times: [0.0; 2],
        })
    }

    pub(in crate::engine) fn grid_steps(&self, config: &crate::analysis::PssConfig) -> usize {
        self.integration_mesh.as_ref().map_or_else(
            || self.integration_steps.max(config.points_per_period),
            PssIntegrationMesh::steps,
        )
    }

    pub(in crate::engine) fn state_dimension(&self) -> usize {
        self.basis.charge_branches.len() + self.basis.currents.representatives.len()
    }

    pub(in crate::engine) fn state_basis_names(&self) -> Vec<String> {
        self.basis.names(&self.circuit)
    }

    fn is_current_coordinate(&self, index: usize) -> bool {
        if index >= self.basis.charge_branches.len() {
            return true; // Independent winding currents.
        }
        let (pos, neg) = self.basis.voltage_nodes(&self.circuit, index);
        self.basis.node_units[pos] == CoordinateUnit::Current
            || self.basis.node_units[neg] == CoordinateUnit::Current
    }

    /// Same normalized perturbation for voltage, thermal and current states.
    /// A current's characteristic scale follows ABSTOL/VNTOL instead of an
    /// implicit one ampere. Retained states and derivatives stay in SI units.
    pub(in crate::engine) fn perturbation_scale(
        &self,
        index: usize,
        value: Value,
        current_scale: Value,
    ) -> Value {
        let unit = if self.is_current_coordinate(index) {
            current_scale
        } else {
            1.0
        };
        unit.max(value.abs())
    }

    pub(super) fn solution_abstol(&self, index: usize, node: Value, current: Value) -> Value {
        if index >= self.num_nodes() || self.basis.node_units[index + 1] == CoordinateUnit::Current
        {
            current
        } else {
            node
        }
    }

    pub(super) fn extract_state(&self) -> Vec<Value> {
        self.basis
            .charge_branches
            .iter()
            .map(|branch| match *branch {
                ChargeBranch::Capacitor(index) => self.capacitors.v_prev[index],
                ChargeBranch::Diode(index) => self.diode_history.vd_prev[index],
                ChargeBranch::Bjt { pos, neg, .. } | ChargeBranch::Jfet { pos, neg, .. } => {
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
        assert_eq!(
            state.len(),
            self.state_dimension(),
            "PSS shooting-state shape must match its basis"
        );
        if self.basis.descriptor.is_none() {
            self.solution_scratch.fill(0.0);
        }
        if let Some(descriptor) = &self.basis.descriptor {
            self.solution_scratch = descriptor.solve(
                &mut self.circuit,
                state,
                self.basis.charge_branches.len(),
                0.0,
            )?;
        }
        if let Some(constraints) = &self.basis.voltage_constraints {
            constraints.solve(&mut self.solution_scratch, |value| {
                value.evaluate(&mut self.circuit, state, self.basis.charge_branches.len())
            })?;
        }
        for edge in &self.basis.forest {
            let value =
                edge.value
                    .evaluate(&mut self.circuit, state, self.basis.charge_branches.len())?;
            self.solution_scratch[edge.to] = self.solution_scratch[edge.from] + edge.sign * value;
        }
        self.current_source_times = [0.0; 2];
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
        if self.basis.descriptor.is_some() {
            for (index, &branch) in circuit.inductors.branch_indices.iter().enumerate() {
                circuit.inductors.i_prev[index] =
                    self.solution_scratch[circuit.num_nodes() + branch];
            }
        } else {
            self.basis.currents.source_balance(
                &circuit.current_sources,
                &mut self.current_balance,
                false,
            )?;
            self.basis.currents.set_state(
                &state[self.basis.charge_branches.len()..],
                &mut circuit.inductors.i_prev,
                &mut self.current_balance,
            );
        }
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
        self.jfet_history = Engine::initialize_jfet_history(
            circuit,
            &self.solution_scratch[1..],
            super::super::transient::ReactiveHistorySeed::SolvedBias,
        );
        Ok(())
    }

    pub(super) fn seed_semiconductor_history(&mut self, solution: &[Value]) {
        self.solution_scratch[1..].copy_from_slice(solution);
        self.bjt_history = Engine::initialize_bjt_history(
            &self.circuit,
            solution,
            super::super::transient::ReactiveHistorySeed::SolvedBias,
        );
        self.bjt_snapshot_cache.fill(None);
        self.jfet_history = Engine::initialize_jfet_history(
            &self.circuit,
            solution,
            super::super::transient::ReactiveHistorySeed::SolvedBias,
        );
    }

    pub(in crate::engine) fn bjt_noise_snapshots(
        &self,
    ) -> &[Option<crate::device::semiconductor::BjtChargeSnapshot>] {
        &self.bjt_snapshot_cache
    }

    pub(super) fn accept_node_solution(&mut self, solution: &[Value]) {
        self.solution_scratch[1..].copy_from_slice(solution);
    }

    pub(super) fn initial_solution_guess(&self) -> Vec<Value> {
        let mut solution = vec![0.0; self.matrix_size()];
        solution[..self.solution_scratch.len() - 1].copy_from_slice(&self.solution_scratch[1..]);
        solution
    }

    pub(super) fn descriptor_initial_solution(
        &mut self,
        abort: &dyn AbortSignal,
    ) -> Result<Option<Vec<Value>>, SimulationError> {
        let Some(descriptor) = &self.basis.descriptor else {
            return Ok(None);
        };
        let state = self.extract_state();
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        self.solution_scratch = descriptor.solve(
            &mut self.circuit,
            &state,
            self.basis.charge_branches.len(),
            0.0,
        )?;
        Ok(Some(self.solution_scratch[1..].to_vec()))
    }

    /// A descriptor without independent states specifies every MNA unknown
    /// at the prepared time. This is a candidate for physical certification,
    /// not permission to skip the device residual and companion checks.
    pub(super) fn project_forced_solution(
        &mut self,
        time: Value,
        solution: &mut [Value],
    ) -> Result<bool, SimulationError> {
        if self.state_dimension() != 0 {
            return Ok(false);
        }
        let Some(descriptor) = &self.basis.descriptor else {
            return Ok(false);
        };
        let projected = descriptor.solve(&mut self.circuit, &[], 0, time)?;
        solution.copy_from_slice(&projected[1..]);
        Ok(true)
    }

    /// An exact initialization constraint carries displacement current while
    /// fixing the accepted branch voltage. Adding only forest coordinates
    /// avoids redundant ideal-source loops for parallel charge branches.
    pub(super) fn add_initial_constraints(
        &mut self,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        self.basis.currents.source_balance(
            &self.circuit.current_sources,
            &mut self.current_balance,
            true,
        )?;
        if let Some(constraints) = &self.basis.voltage_constraints {
            let branches = (0..self.basis.charge_branches.len())
                .map(|_| self.circuit.allocate_branch())
                .collect();
            self.initial_charge_rates = Some(InitialChargeRates::new(constraints, branches));
        }
        for index in 0..self.basis.charge_branches.len() {
            if self.initial_charge_rates.is_some() {
                break;
            }
            let (pos, neg) = self.basis.voltage_nodes(&self.circuit, index);
            let value = match self.basis.charge_branches[index] {
                ChargeBranch::Capacitor(index) => self.capacitors.v_prev[index],
                ChargeBranch::Diode(index) => self.diode_history.vd_prev[index],
                ChargeBranch::Bjt { pos, neg, .. } | ChargeBranch::Jfet { pos, neg, .. } => {
                    self.solution_scratch[pos] - self.solution_scratch[neg]
                }
            };
            // An IC capacitor already owns a physical current unknown. Reuse
            // it for the voltage constraint rather than leaving its original
            // row empty beside an unnecessary auxiliary branch.
            let existing_branch = match self.basis.charge_branches[index] {
                ChargeBranch::Capacitor(index) => self.capacitors.ic_branch_indices[index],
                ChargeBranch::Diode(_) | ChargeBranch::Bjt { .. } | ChargeBranch::Jfet { .. } => {
                    None
                }
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
        if let Some(rates) = &self.initial_charge_rates {
            for (index, &branch) in rates.branches.iter().enumerate() {
                let (pos, neg) = self.basis.voltage_nodes(&self.circuit, index);
                for node in [pos, neg] {
                    if node != 0 {
                        entries.push((self.num_nodes() + branch - 1, node - 1));
                    }
                }
            }
        }
        entries
    }

    pub(super) fn initial_charge_pattern(
        &self,
        matrix: &StaticMatrix,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<(usize, usize)>, SimulationError> {
        self.initial_charge_rates.as_ref().map_or_else(
            || Ok(Vec::new()),
            |rates| rates.extra_pattern(matrix, self.num_nodes(), abort),
        )
    }

    pub(super) fn link_initial_charge_pattern(
        &mut self,
        matrix: &StaticMatrix,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        if let Some(rates) = &mut self.initial_charge_rates {
            rates.link_pattern(matrix, abort)?;
        }
        Ok(())
    }

    pub(super) fn has_initial_charge_rates(&self) -> bool {
        self.initial_charge_rates.is_some()
    }

    pub(super) fn stamp_initial_charge_constraints(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        physical_probe: bool,
    ) -> Result<(), SimulationError> {
        let Some(rates) = &self.initial_charge_rates else {
            return Ok(());
        };
        let nodes = self.circuit.num_nodes();
        let mut stamps = Vec::new();
        let coeff = CompanionCoefficients::for_method(IntegrationMethod::BackwardEuler);
        matrix.with_probe_values(|charge, unused_rhs| {
            self.circuit
                .capacitors
                .stamp_transient_norton_companions(charge, unused_rhs, 1.0, &coeff);
            for (index, branch) in self.circuit.capacitors.ic_branch_indices.iter().enumerate() {
                if let Some(branch) = branch {
                    let row = nodes + branch - 1;
                    let stamp = &self.circuit.capacitors.stamps[index];
                    let capacitance = self.circuit.capacitors.capacitances[index];
                    for (node, sign) in [(stamp.pp.row, -1.0), (stamp.nn.row, 1.0)] {
                        if node != 0 {
                            charge.add(row, node - 1, sign * capacitance);
                        }
                    }
                }
            }
            self.circuit.diodes.stamp_charge_companions(
                charge,
                unused_rhs,
                solution,
                &coeff,
                1.0,
                &self.diode_history,
                physical_probe,
            );
            Engine::stamp_bjt_transient_companions(
                super::super::transient::TransientCompanionStamp {
                    circuit: &self.circuit,
                    matrix: charge,
                    rhs: unused_rhs,
                    voltages: solution,
                    coeff: &coeff,
                    dt: 1.0,
                },
                &self.bjt_history,
                &mut self.bjt_snapshot_cache,
                false,
            )?;
            Engine::stamp_jfet_transient_companions(
                super::super::transient::TransientCompanionStamp {
                    circuit: &self.circuit,
                    matrix: charge,
                    rhs: unused_rhs,
                    voltages: solution,
                    coeff: &coeff,
                    dt: 1.0,
                },
                &self.jfet_history,
                false,
            );
            rates.project(charge.values_mut(), &self.circuit, rhs, &mut stamps)
        })?;
        for (row, col, value) in stamps {
            matrix.add(row, col, value);
        }
        for (index, &branch) in rates.branches.iter().enumerate() {
            let row = nodes + branch - 1;
            let (pos, neg) = self.basis.voltage_nodes(&self.circuit, index);
            for (node, sign) in [(pos, 1.0), (neg, -1.0)] {
                if node != 0 {
                    matrix.add(row, node - 1, sign);
                }
            }
            rhs[row] = self.solution_scratch[pos] - self.solution_scratch[neg];
        }
        for (index, branch) in self.circuit.capacitors.ic_branch_indices.iter().enumerate() {
            if let Some(branch) = branch {
                let row = nodes + branch - 1;
                let stamp = &self.circuit.capacitors.stamps[index];
                matrix.add(row, row, 1.0);
                for (node, sign) in [(stamp.pp.row, 1.0), (stamp.nn.row, -1.0)] {
                    if node != 0 {
                        matrix.add(node - 1, row, sign);
                    }
                }
            }
        }
        Ok(())
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
            || self.initial_charge_rates.as_ref().is_some_and(|rates| {
                rates.branches.iter().enumerate().any(|(index, &branch)| {
                    row == self.num_nodes() + branch - 1 && self.is_current_coordinate(index)
                })
            })
            || self
                .voltage_sources
                .branch_indices
                .iter()
                .enumerate()
                .any(|(index, &branch)| {
                    row == self.num_nodes() + branch - 1
                        && [
                            self.voltage_sources.node_pos[index],
                            self.voltage_sources.node_neg[index],
                        ]
                        .into_iter()
                        .any(|node| self.basis.node_units[node] == CoordinateUnit::Current)
                })
    }

    pub(super) fn initialize_prescribed_currents(&mut self) -> Result<(), SimulationError> {
        if !self.current_source_rates.is_empty() || self.basis.descriptor.is_some() {
            self.set_state(&self.extract_state())?;
        }
        Ok(())
    }

    pub(super) fn ensure_regular_prescribed_currents(
        &self,
        period: Value,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        if let Some(descriptor) = &self.basis.descriptor {
            return descriptor.ensure_regular_forcing(&self.circuit, period, abort);
        }
        self.basis
            .currents
            .ensure_regular_forcing(&self.circuit.current_sources, period)
    }

    pub(in crate::engine) fn prepare_prescribed_forcing(
        &mut self,
        time: Value,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        if let Some(descriptor) = &mut self.basis.descriptor {
            descriptor.prepare_forcing(
                &self.circuit,
                [
                    time,
                    self.current_source_times[0],
                    self.current_source_times[1],
                ],
                abort,
            )?;
        }
        Ok(())
    }

    pub(super) fn stamp_prescribed_current_correction(
        &mut self,
        rhs: &mut [Value],
        step: PssCompanionStep<'_>,
    ) -> Result<(), SimulationError> {
        if !self.current_source_rates.is_empty() {
            if let Some(descriptor) = &self.basis.descriptor {
                descriptor.source_companion(
                    &self.circuit,
                    &mut self.current_source_rates,
                    &mut self.current_source_offsets,
                    self.current_source_times,
                    step,
                )?;
            } else {
                self.basis.currents.source_companion(
                    &self.circuit.current_sources,
                    &mut self.current_balance,
                    &mut self.current_source_rates,
                    &mut self.current_source_offsets,
                    self.current_source_times,
                    step,
                )?;
            }
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
            self.capacitor_trial_currents[index] = if let Some(branch) =
                caps.ic_branch_indices[index]
                && self.charge_source_rates.is_empty()
            {
                let row = self.circuit.num_nodes() + branch - 1;
                iterate[row] + correction[row]
            } else {
                let present = self.capacitor_companion_current(
                    index,
                    voltage(iterate, stamp.pp.row) - voltage(iterate, stamp.nn.row),
                    step,
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

    fn capacitor_companion_current(
        &self,
        index: usize,
        present: Value,
        step: PssCompanionStep<'_>,
    ) -> Value {
        let caps = &self.circuit.capacitors;
        let samples = [present, caps.v_prev[index], caps.v_prev_prev[index]];
        let free = if self.charge_source_rates.is_empty() {
            samples
        } else {
            std::array::from_fn(|history| {
                samples[history] - self.charge_source_offsets[history][index]
            })
        };
        let current = step.coeff.capacitor_current(
            caps.capacitances[index],
            step.dt,
            free[0],
            free[1],
            free[2],
            caps.i_prev[index],
        );
        if self.charge_source_rates.is_empty() {
            current
        } else {
            caps.capacitances[index].mul_add(self.charge_source_rates[index], current)
        }
    }

    pub(super) fn stamp_capacitor_correction(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        iterate: &[Value],
        step: PssCompanionStep<'_>,
    ) {
        let voltage = |node| if node == 0 { 0.0 } else { iterate[node - 1] };
        self.circuit
            .capacitors
            .stamp_norton_correction_with_current(matrix, rhs, step.dt, step.coeff, |index| {
                let stamp = &self.circuit.capacitors.stamps[index];
                self.capacitor_companion_current(
                    index,
                    voltage(stamp.pp.row) - voltage(stamp.nn.row),
                    step,
                )
            });
        if !self.charge_source_rates.is_empty() {
            for (index, branch) in self.circuit.capacitors.ic_branch_indices.iter().enumerate() {
                if let Some(branch) = branch {
                    let row = self.num_nodes() + branch - 1;
                    let diagonal = matrix.get_index(row, row).unwrap();
                    let scale = matrix.values_mut()[diagonal.offset()];
                    let stamp = &self.circuit.capacitors.stamps[index];
                    rhs[row] = scale
                        * (self.capacitor_companion_current(
                            index,
                            voltage(stamp.pp.row) - voltage(stamp.nn.row),
                            step,
                        ) - iterate[row]);
                }
            }
        }
    }

    pub(super) fn stamp_charge_forcing(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        step: PssCompanionStep<'_>,
        norton: bool,
    ) -> Result<(), SimulationError> {
        if self.charge_source_rates.is_empty() {
            return Ok(());
        }
        self.basis.descriptor.as_ref().unwrap().charge_companion(
            &self.circuit,
            &mut self.charge_source_rates,
            &mut self.charge_source_offsets,
            self.current_source_times,
            step,
        )?;
        for (index, stamp) in self.circuit.capacitors.stamps.iter().enumerate() {
            let cap = self.circuit.capacitors.capacitances[index];
            let difference = step.coeff.capacitor_current(
                cap,
                step.dt,
                self.charge_source_offsets[0][index],
                self.charge_source_offsets[1][index],
                self.charge_source_offsets[2][index],
                0.0,
            );
            let correction = cap.mul_add(self.charge_source_rates[index], -difference);
            if !correction.is_finite() {
                return Err(SimulationError::Circuit(
                    "PSS prescribed charge correction is non-finite".to_owned(),
                ));
            }
            if let Some(branch) = self.circuit.capacitors.ic_branch_indices[index] {
                if !norton {
                    let row = self.num_nodes() + branch - 1;
                    let diagonal = matrix.get_index(row, row).unwrap();
                    let scale = matrix.values_mut()[diagonal.offset()];
                    rhs[row] += scale * correction;
                }
            } else if norton {
                if stamp.pp.row != 0 {
                    rhs[stamp.pp.row - 1] -= correction;
                }
                if stamp.nn.row != 0 {
                    rhs[stamp.nn.row - 1] += correction;
                }
            }
        }
        Ok(())
    }

    pub(super) fn capacitor_current_residuals<'a>(
        &'a self,
        solution: &'a [Value],
    ) -> impl Iterator<Item = (Value, Value)> + 'a {
        let nodes = self.num_nodes();
        self.circuit
            .capacitors
            .ic_branch_indices
            .iter()
            .enumerate()
            .filter_map(move |(index, branch)| {
                branch
                    .filter(|_| !self.charge_source_rates.is_empty())
                    .map(|branch| {
                        (
                            solution[nodes + branch - 1],
                            self.capacitor_trial_currents[index],
                        )
                    })
            })
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
        let offset = self.basis.charge_branches.len();
        let projection = self.basis.descriptor.as_ref().map_or_else(
            || self.basis.currents.projection(index),
            |descriptor| descriptor.projection(&self.circuit, index),
        );
        Some((
            self.inductors.names[index].clone(),
            projection
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
        (0..self.basis.charge_branches.len())
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
    fn vcvs_constraints_restore_differential_cascaded_charge_states() {
        let engine = Engine::default();
        for gain in [0.0, 2.0, -0.25, 1e-12] {
            for reversed in [false, true] {
                let control = if reversed { "b a" } else { "a b" };
                let deck = Netlist::parse(&format!(
                    "Differential controlled charge\nVa a 0 2\nRb b 0 1k\nVc c 0 3\nE1 d c {control} {gain:e}\nE2 e 0 d c -2\nC1 a b 1u\nC2 d c 2u\nC3 e 0 3u\nC4 d e 4u\n.end\n"
                )).unwrap();
                let mut circuit =
                    PssCircuit::new(Engine::default().build_circuit(&deck).unwrap()).unwrap();
                assert_eq!(circuit.basis.names(&circuit), ["C:C1"]);
                for voltage in [-0.5, 0.75] {
                    circuit.set_state(&[voltage]).unwrap();
                    let output = if reversed {
                        -gain * voltage
                    } else {
                        gain * voltage
                    };
                    for (&actual, expected) in circuit.capacitors.v_prev.iter().zip([
                        voltage,
                        output,
                        -2.0 * output,
                        3.0 + 3.0 * output,
                    ]) {
                        assert!((actual - expected).abs() <= 2e-15 * expected.abs().max(1.0));
                    }
                    assert!((circuit.extract_state()[0] - voltage).abs() < 1e-15);
                    let solution = engine
                        .pss_initial_node_solution(&mut circuit, &crate::abort_signal::NoAbort)
                        .unwrap();
                    let rate = (2.0 - voltage) / (1e3 * 1e-6) * if reversed { -gain } else { gain };
                    for (index, expected) in [-14e-6 * rate, 18e-6 * rate].into_iter().enumerate() {
                        let current =
                            solution[circuit.num_nodes() + circuit.vcvs.branch_indices[index] - 1];
                        assert!(
                            (current - expected).abs() < 2e-12 * expected.abs().max(1e-6),
                            "gain {gain}, current {current} vs {expected}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn vcvs_initialization_identifies_the_source_without_an_outgoing_derivative() {
        for dialect in [
            crate::config::SpiceDialect::Ngspice,
            crate::config::SpiceDialect::Xyce,
        ] {
            for expression in ["floor(-sin(2*pi*time))", "sqrt(sin(2*pi*time)^2)"] {
                let deck = Netlist::parse(&format!(
                    "Initial source derivative diagnostic\nB1 aux 0 V=1\nRaux aux 0 1\nB2 in 0 V={expression}\nR1 in 0 1\nE1 out 0 in 0 2\nC1 out 0 0.2\n.end\n"
                )).unwrap();
                let engine = Engine::new(
                    super::super::super::SimulationConfig::default().with_spice_dialect(dialect),
                );
                // The descriptor validates supported Taylor programs during
                // construction; legacy operators are diagnosed during the
                // augmented initialization solve. Both must name B2.
                let error = PssCircuit::new(engine.build_circuit(&deck).unwrap())
                    .and_then(|mut circuit| {
                        circuit.set_state(&[])?;
                        engine
                            .pss_initial_node_solution(&mut circuit, &crate::abort_signal::NoAbort)
                    })
                    .unwrap_err();
                let required = if expression.starts_with("floor") {
                    "analytic outgoing derivative"
                } else {
                    "analytic derivative through order 1"
                };
                assert!(
                    matches!(error, SimulationError::Circuit(ref message)
                    if message.contains("source B2") && message.contains(required)),
                    "{error}"
                );
            }
        }
    }

    #[test]
    fn vcvs_initialization_retains_ic_capacitor_and_semiconductor_charge_currents() {
        for input in [
            "V1 in 0 SIN(0 0.01 1)",
            "B1 in 0 V=0.01*sin(2*pi*time)",
            "B1 in 0 V=spice_sin(0,0.01,1)",
        ] {
            let deck = Netlist::parse(&format!(
                "Controlled physical initial currents\n{input}\nR1 in 0 1k\nE1 out 0 in 0 2\nC1 out 0 0.2 IC=0\nD1 out 0 DM\nJ1 0 out 0 JM\n.model DM D(IS=0 CJO=0.05 M=0)\n.model JM NJF(CGS=0.07 CGD=0.11)\n.end\n"
            )).unwrap();
            let engine = Engine::new(
                super::super::super::SimulationConfig::default()
                    .with_spice_dialect(crate::config::SpiceDialect::Xyce),
            );
            let mut circuit = PssCircuit::new(engine.build_circuit(&deck).unwrap()).unwrap();
            assert_eq!(circuit.state_dimension(), 0);
            circuit.set_state(&[]).unwrap();
            let solution = engine
                .pss_initial_node_solution(&mut circuit, &crate::abort_signal::NoAbort)
                .unwrap();
            let rate = 0.02 * std::f64::consts::TAU;
            let source = circuit.num_nodes() + circuit.vcvs.branch_indices[0] - 1;
            let capacitor =
                circuit.num_nodes() + circuit.capacitors.ic_branch_indices[0].unwrap() - 1;
            assert!(
                (solution[source] + (0.2 + 0.05 + 0.07 + 0.11) * rate).abs() < 2e-12,
                "{input}: {}",
                solution[source]
            );
            assert!((solution[capacitor] - 0.2 * rate).abs() < 2e-12);
        }
    }

    #[test]
    fn jfet_storage_forest_restores_charge_history_and_removes_redundant_coordinates() {
        for (extra, names) in [
            ("", vec!["J:J1:qgs", "J:J1:qgd"]),
            ("C1 g s 1p\n", vec!["C:C1", "J:J1:qgd"]),
            ("Vgs g s -0.2\n", vec!["J:J1:qgd"]),
            ("Vgs g s -0.2\nVgd g d -0.3\n", Vec::new()),
        ] {
            let netlist = Netlist::parse(&format!(
                "JFET charge forest\nVd d 0 1\nRg g 0 1k\nRs s 0 100\nJ1 d g s jm\n.model jm NJF(CGS=1n CGD=2n)\n{extra}.end\n"
            )).unwrap();
            let engine = Engine::default();
            let mut circuit = PssCircuit::new(engine.build_circuit(&netlist).unwrap()).unwrap();
            assert_eq!(circuit.basis.names(&circuit), names);
            let state = vec![-0.3; names.len()];
            circuit.set_state(&state).unwrap();
            let accepted = circuit.jfet_history.clone();
            for (actual, expected) in circuit.extract_state().iter().zip(&state) {
                assert!((actual - expected).abs() < 1e-15);
            }
            assert_eq!(circuit.clone().jfet_history, accepted);
            circuit.set_state(&vec![-0.7; names.len()]).unwrap();
            circuit.set_state(&state).unwrap();
            assert_eq!(
                circuit.jfet_history, accepted,
                "a prior period/probe cannot leak into a new shooting seed"
            );
        }
    }

    #[test]
    fn jfet_ports_preserve_prescribed_winding_cutsets() {
        let netlist = Netlist::parse(
            "JFET winding cutsets\nI1 0 a SIN(0 1m 1meg)\nL1 a b 1u\nL2 b d 2u\nRg g 0 1k\nJ1 d g 0 jm\n.model jm NJF(CGS=1n CGD=2n)\n.end\n",
        ).unwrap();
        let circuit = PssCircuit::new(Engine::default().build_circuit(&netlist).unwrap()).unwrap();
        assert_eq!(circuit.basis.names(&circuit), ["J:J1:qgs", "J:J1:qgd"]);
        assert!(circuit.basis.currents.representatives.is_empty());
    }

    #[test]
    fn vbic_thermal_delay_coordinates_preserve_units_and_initial_constraints() {
        for source in ["V1 b 0 0.1", "V1 drive 0 0.1\nE1 b 0 drive 0 1"] {
            let netlist = Netlist::parse(&format!(
                "VBIC physical state basis\n{source}\nVc c 0 1\nQ1 c b 0 vm\n.model vm NPN(LEVEL=4 CJE=10p RCI=0 RBI=0 SELFT=1 RTH=100 CTH=5n TD=400n)\n.end"
            )).unwrap();
            let mut circuit =
                PssCircuit::new(Engine::default().build_circuit(&netlist).unwrap()).unwrap();
            assert_eq!(
                circuit.state_basis_names(),
                ["Q:Q1:qcth", "Q:Q1:qxf1", "Q:Q1:qxf2"]
            );
            let state = [12.0, 2e-6, 3e-6];
            circuit.set_state(&state).unwrap();
            assert_eq!(circuit.extract_state(), state);
            let perturbation = circuit
                .project_perturbation(&circuit.solution_scratch[1..])
                .collect::<Vec<_>>();
            assert_eq!(perturbation, state);
            assert_eq!(circuit.perturbation_scale(0, 0.0, 1e-6), 1.0);
            assert_eq!(circuit.perturbation_scale(1, 0.0, 1e-6), 1e-6);
            circuit.add_initial_constraints(&NoAbort).unwrap();
            let rows = (circuit.num_nodes()..circuit.matrix_size())
                .filter(|&row| circuit.is_initial_current_row(row))
                .count();
            assert_eq!(
                rows, 2,
                "thermal constraint uses K; both delay constraints use A"
            );
            let history = circuit.bjt_history.clone();
            circuit.set_state(&[8.0, -4e-6, 7e-6]).unwrap();
            assert_ne!(circuit.bjt_history, history);
            circuit.set_state(&state).unwrap();
            assert_eq!(circuit.bjt_history, history);
        }
    }

    #[test]
    fn vbic_storage_forest_removes_constant_charge_and_parallel_coordinates() {
        for (extra, state_names) in [
            ("", vec!["Q:Q1:qbe"]),
            ("C1 b 0 1p\n", vec!["C:C1"]),
            ("Vb b 0 0.1\n", Vec::new()),
        ] {
            let netlist = Netlist::parse(&format!("VBIC state forest\nVc c 0 1\nR1 b 0 1k\nQ1 c b 0 vm\n.model vm NPN(LEVEL=4 CJE=10p CJC=20p CBEO=30p CBCO=40p RCX=0 RCI=0 RBX=0 RBI=0 RBP=0 QCO=1p GAMM=0 TF=0 TR=0)\n{extra}.end\n")).unwrap();
            let mut circuit =
                PssCircuit::new(Engine::default().build_circuit(&netlist).unwrap()).unwrap();
            assert_eq!(circuit.basis.names(&circuit), state_names);
            let state = vec![0.1; state_names.len()];
            circuit.set_state(&state).unwrap();
            assert_eq!(circuit.extract_state(), state);
            assert_eq!(circuit.clone().bjt_history, circuit.bjt_history);
        }
    }

    #[test]
    fn voltage_forest_reduces_exact_shorts_but_preserves_near_zero_charge() {
        for resistance in [0.0_f64, -0.0, 1e-12, -1e-12] {
            for (first, second) in [("a mid", "mid b"), ("mid a", "b mid")] {
                let netlist = Netlist::parse(&format!(
                    "Shorted charge forest\nV1 a 0 2\nRz1 {first} {resistance:e} AC=0\nRz2 {second} 0\nR1 c 0 1k\nC1 a b 1n\nC2 b c 2n\nC3 a c 3n\nD1 c a dm\n.model dm D(IS=0 CJO=1n M=0)\n.options device zeroresistancetol=1\n.end\n"
                )).unwrap();
                let mut circuit =
                    PssCircuit::new(Engine::default().build_circuit(&netlist).unwrap()).unwrap();
                let (names, values, first_voltage) = if resistance == 0.0 {
                    (vec!["C:C2"], vec![0.5], 0.0)
                } else {
                    (vec!["C:C1", "C:C2"], vec![0.25, 0.5], 0.25)
                };
                assert_eq!(circuit.basis.names(&circuit), names, "R={resistance}");
                circuit.set_state(&values).unwrap();
                assert_eq!(circuit.extract_state(), values);
                assert_eq!(
                    circuit.capacitors.v_prev,
                    [first_voltage, 0.5, first_voltage + 0.5]
                );
                assert_eq!(circuit.diode_history.vd_prev, [-first_voltage - 0.5]);
                assert!(
                    (circuit.diode_history.qd_prev[0] + (first_voltage + 0.5) * 1e-9).abs() < 1e-24
                );
            }
        }
    }

    #[test]
    fn voltage_forest_preserves_all_charge_branches_in_a_loop() {
        let netlist = Netlist::parse("charge loop\nV1 a 0 2\nR1 b 0 1k\nR2 c 0 1k\nC1 a b 1n\nC2 b c 2n\nC3 a c 3n\nD1 c a dm\n.model dm D(IS=1e-30 CJO=1n M=0)\n.end\n").unwrap();
        let mut circuit =
            PssCircuit::new(Engine::default().build_circuit(&netlist).unwrap()).unwrap();
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
