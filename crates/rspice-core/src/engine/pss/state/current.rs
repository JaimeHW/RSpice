//! Independent winding currents, junction KCL and consistent flux derivatives.

use super::*;

#[derive(Debug, Clone, Copy)]
struct CurrentTreeEdge {
    parent: usize,
    child: usize,
    winding: usize,
    sign: Value,
    voltage_scale: Value,
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
    source_ports: Vec<(usize, Option<usize>, Option<usize>)>,
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
                        | F::Jfet
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
                source_ports: Vec::new(),
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
        // The admitted classic JFET owns both gate-junction current branches,
        // including their model GMIN, even when its capacitances are zero.
        for jfet in &circuit.jfets {
            join(jfet.gate, jfet.source);
            join(jfet.gate, jfet.drain);
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
        // A source supplies a prescribed injection, not a free conductive
        // path. Keeping its ports separate exposes affine winding cutsets.
        let mut source_ports: Vec<_> = circuit
            .current_sources
            .node_pos
            .iter()
            .zip(&circuit.current_sources.node_neg)
            .enumerate()
            .filter_map(|(index, (&pos, &neg))| {
                let pos = ids.get(&root(&mut parents, pos)).copied();
                let neg = ids.get(&root(&mut parents, neg)).copied();
                (pos != neg).then_some((index, pos, neg))
            })
            .collect();
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
                            voltage_scale: incident[child]
                                .iter()
                                .map(|&(index, _)| circuit.inductors.inductances[index])
                                .fold(Value::INFINITY, Value::min),
                        });
                        pending.push(child);
                    }
                }
            }
        }
        // Injections at a forest root do not prescribe any tree current.
        // Exclude them so unrelated source waveforms need no differentiation
        // or extra admission restrictions.
        let mut constrained = vec![false; component_count];
        for edge in &tree {
            constrained[edge.child] = true;
        }
        source_ports.retain(|&(_, pos, neg)| {
            pos.is_some_and(|pos| constrained[pos]) || neg.is_some_and(|neg| constrained[neg])
        });
        Self {
            representatives,
            components,
            tree,
            incident,
            winding_by_branch,
            has_mutual,
            source_ports,
        }
    }

    pub(super) fn workspace_size(&self) -> usize {
        self.incident.len()
    }

    pub(super) fn needs_flux_rates(&self) -> bool {
        !self.tree.is_empty() && self.has_mutual
    }

    pub(super) fn has_prescribed_currents(&self) -> bool {
        !self.tree.is_empty() && !self.source_ports.is_empty()
    }

    pub(super) fn ensure_regular_forcing(
        &self,
        sources: &crate::circuit::CurrentSources,
        period: Value,
    ) -> Result<(), SimulationError> {
        for &(index, _, _) in &self.source_ports {
            if !sources.has_regular_periodic_waveform(index, period) {
                return Err(SimulationError::Circuit(format!(
                    "PSS cannot certify prescribed current '{}' as continuous and periodic with period {period:e} s; a current cutset requires a regular periodic drive",
                    sources.names[index],
                )));
            }
        }
        Ok(())
    }

    pub(super) fn source_balance(
        &self,
        sources: &crate::circuit::CurrentSources,
        balance: &mut [Value],
        derivative: bool,
    ) -> Result<(), SimulationError> {
        self.source_balance_with(
            sources,
            balance,
            if derivative {
                "right-hand time derivative at time zero"
            } else {
                "value at time zero"
            },
            |index| {
                if derivative {
                    sources.right_derivative_at_time(index, 0.0)
                } else {
                    sources.value_at_time(index, 0.0)
                }
            },
        )
    }

    fn source_balance_with(
        &self,
        sources: &crate::circuit::CurrentSources,
        balance: &mut [Value],
        quantity: &str,
        mut evaluate: impl FnMut(usize) -> Value,
    ) -> Result<(), SimulationError> {
        balance.fill(0.0);
        for &(index, pos, neg) in &self.source_ports {
            let value = evaluate(index);
            if !value.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "PSS prescribed current '{}' has no finite {quantity}",
                    sources.names[index],
                )));
            }
            if let Some(pos) = pos {
                balance[pos] += value;
            }
            if let Some(neg) = neg {
                balance[neg] -= value;
            }
        }
        if balance.iter().any(|value| !value.is_finite()) {
            return Err(SimulationError::Circuit(
                "PSS prescribed-current balance is non-finite".to_owned(),
            ));
        }
        Ok(())
    }

    /// Apply the companion only to the free part of I = P*x + q(t), then
    /// restore the exact prescribed derivative q'(t). Differentiating the
    /// sampled forcing itself would create parasitic trapezoidal voltage
    /// oscillations in a current-source cutset, despite exact current KCL.
    pub(super) fn source_companion(
        &self,
        sources: &crate::circuit::CurrentSources,
        balance: &mut [Value],
        rates: &mut [Value],
        offsets: &mut [Vec<Value>; 3],
        times: [Value; 2],
        step: PssCompanionStep<'_>,
    ) -> Result<(), SimulationError> {
        self.source_balance_with(sources, balance, "current-constraint companion", |index| {
            let derivative = sources.right_derivative_at_time(index, step.t_next);
            let previous_derivative = if step.coeff.coeff_i_n == 0.0 {
                0.0
            } else {
                sources.right_derivative_at_time(index, times[0])
            };
            derivative + step.coeff.coeff_i_n * previous_derivative
        })?;
        self.set_state(&[], rates, balance);
        for (offset, time) in offsets.iter_mut().zip([step.t_next, times[0], times[1]]) {
            self.source_balance_with(sources, balance, "current-constraint value", |index| {
                sources.value_at_time(index, time)
            })?;
            self.set_state(&[], offset, balance);
        }
        Ok(())
    }

    pub(super) fn winding_index(&self, branch: usize) -> usize {
        self.winding_by_branch[branch]
    }

    /// Remove q(t) from each physical I = P*x + q(t) sample before flux
    /// multiplication. Subtracting separately differentiated L*I and q loses
    /// the small ripple voltage when the prescribed current has a large bias.
    pub(super) fn free_current_samples(
        &self,
        branch: usize,
        samples: [Value; 3],
        offsets: &[Vec<Value>; 3],
    ) -> [Value; 3] {
        let winding = self.winding_by_branch[branch];
        std::array::from_fn(|history| samples[history] - offsets[history][winding])
    }

    pub(super) fn add_flux_rhs(
        &self,
        circuit: &CircuitData,
        rate: impl Fn(usize) -> Value,
        rhs: &mut [Value],
    ) -> Result<(), SimulationError> {
        for index in 0..circuit.inductors.len() {
            let row = circuit.num_nodes() + circuit.inductors.branch_indices[index] - 1;
            rhs[row] += circuit.inductors.inductances[index] * rate(index);
        }
        for pair in &circuit.coupled_inductor_pairs {
            let first = self.winding_by_branch[pair.branch1_ordinal];
            let second = self.winding_by_branch[pair.branch2_ordinal];
            rhs[circuit.num_nodes() + pair.branch1_ordinal - 1] += pair.device.m * rate(second);
            rhs[circuit.num_nodes() + pair.branch2_ordinal - 1] += pair.device.m * rate(first);
        }
        if circuit
            .inductors
            .branch_indices
            .iter()
            .any(|&branch| !rhs[circuit.num_nodes() + branch - 1].is_finite())
        {
            return Err(SimulationError::Circuit(
                "PSS prescribed-current flux correction is non-finite".to_owned(),
            ));
        }
        Ok(())
    }

    pub(super) fn set_state(&self, state: &[Value], currents: &mut [Value], balance: &mut [Value]) {
        currents.fill(0.0);
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
            let scale = edge.voltage_scale;
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

    pub(super) fn initial_rate_rhs(
        &self,
        circuit: &CircuitData,
        rates: Option<usize>,
        derivatives: &[Value],
        mut visit: impl FnMut(usize, Value),
    ) -> Result<(), SimulationError> {
        for edge in &self.tree {
            let branch = rates.map_or(circuit.inductors.branch_indices[edge.winding], |first| {
                first + edge.winding
            });
            let value = -edge.voltage_scale * derivatives[edge.child];
            if !value.is_finite() {
                return Err(SimulationError::Circuit(
                    "PSS prescribed-current initial flux rate is non-finite".to_owned(),
                ));
            }
            visit(circuit.num_nodes() + branch - 1, value);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn injected_current_junctions_preserve_physical_kcl_and_differentiated_kcl() {
        let ports = [
            ("out", "a"),
            ("a", "b"),
            ("b", "0"),
            ("a", "0"),
            ("b", "out"),
        ];
        let engine = Engine::default();
        for reversed in 0..32 {
            let mut deck = String::from(
                "affine winding junctions\nV1 in 0 1\nR1 in out 100\nI1 0 a SIN(2m 1m 1meg 0 0 37)\nI2 b 0 DC -3m\n",
            );
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
            deck.push_str("K1 L1 L5 0.3\n.end\n");
            let netlist = Netlist::parse(&deck).unwrap();
            let mut circuit = PssCircuit::new(engine.build_circuit(&netlist).unwrap()).unwrap();
            assert_eq!(circuit.state_dimension(), 3);
            circuit.set_state(&[0.0; 3]).unwrap();
            let offset = circuit.inductors.i_prev.clone();
            let state = [0.2, -0.3, 0.4];
            circuit.set_state(&state).unwrap();
            for (index, &offset) in offset.iter().enumerate() {
                let projected: f64 = circuit
                    .basis
                    .currents
                    .projection(index)
                    .iter()
                    .map(|&(i, weight)| state[i] * weight)
                    .sum();
                assert!((circuit.inductors.i_prev[index] - projected - offset).abs() < 1e-15);
            }
            let solution = engine
                .pss_initial_node_solution(&mut circuit, &NoAbort)
                .unwrap();
            let mut rates = Vec::new();
            for index in 0..5 {
                let voltage = |node| if node == 0 { 0.0 } else { solution[node - 1] };
                rates.push(
                    (voltage(circuit.inductors.node_pos[index])
                        - voltage(circuit.inductors.node_neg[index]))
                        / circuit.inductors.inductances[index],
                );
            }
            // Invert just the two-winding block of the physical flux matrix.
            let first = rates[0];
            let fifth = rates[4];
            let mutual = 0.3 * (10e-6_f64 * 50e-6).sqrt();
            rates[0] = (first - mutual / 10e-6 * fifth) / (1.0 - 0.3 * 0.3);
            rates[4] = (fifth - mutual / 50e-6 * first) / (1.0 - 0.3 * 0.3);
            for name in ["a", "b"] {
                let node = circuit.get_node_by_name(name).unwrap();
                let mut current_kcl = 0.0;
                let mut rate_kcl = 0.0;
                for (index, &rate) in rates.iter().enumerate() {
                    let sign = f64::from(circuit.inductors.node_pos[index] == node)
                        - f64::from(circuit.inductors.node_neg[index] == node);
                    current_kcl += sign * circuit.inductors.i_prev[index];
                    rate_kcl += sign * rate;
                }
                for index in 0..circuit.current_sources.names.len() {
                    let sign = f64::from(circuit.current_sources.node_pos[index] == node)
                        - f64::from(circuit.current_sources.node_neg[index] == node);
                    current_kcl += sign * circuit.current_sources.value_at_time(index, 0.0);
                    rate_kcl += sign * circuit.current_sources.right_derivative_at_time(index, 0.0);
                }
                assert!(
                    current_kcl.abs() < 1e-15,
                    "{name}, reverse={reversed}, KCL={current_kcl:e}"
                );
                assert!(
                    rate_kcl.abs() < 1e-7,
                    "{name}, reverse={reversed}, dKCL={rate_kcl:e}"
                );
            }
        }
    }

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
                let mut circuit = PssCircuit::new(engine.build_circuit(&netlist).unwrap()).unwrap();
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
        let mut circuit = PssCircuit::new(engine.build_circuit(&netlist).unwrap()).unwrap();
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
