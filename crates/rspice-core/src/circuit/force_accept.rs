use super::*;

impl CircuitData {
    #[inline]
    pub(in crate::circuit) fn mark_force_accept_protected_node(mask: &mut [bool], node: NodeId) {
        if node > 0
            && let Some(slot) = mask.get_mut(node - 1)
        {
            *slot = true;
        }
    }

    #[inline]
    pub(in crate::circuit) fn add_force_accept_topology_edge(
        graph: &mut [Vec<NodeId>],
        node_a: NodeId,
        node_b: NodeId,
    ) {
        if node_a == node_b {
            return;
        }
        let Some(neighbors_a) = graph.get_mut(node_a) else {
            return;
        };
        neighbors_a.push(node_b);
        let Some(neighbors_b) = graph.get_mut(node_b) else {
            return;
        };
        neighbors_b.push(node_a);
    }

    #[inline]
    pub(in crate::circuit) fn add_force_accept_topology_clique(
        graph: &mut [Vec<NodeId>],
        nodes: &[NodeId],
    ) {
        for (idx, &node_a) in nodes.iter().enumerate() {
            if node_a >= graph.len() {
                continue;
            }
            for &node_b in &nodes[idx + 1..] {
                if node_b >= graph.len() {
                    continue;
                }
                Self::add_force_accept_topology_edge(graph, node_a, node_b);
            }
        }
    }

    #[inline]
    pub(in crate::circuit) fn add_force_accept_topology_clique_nonzero(
        graph: &mut [Vec<NodeId>],
        nodes: &[NodeId],
    ) {
        let nonzero: Vec<NodeId> = nodes.iter().copied().filter(|&node| node > 0).collect();
        Self::add_force_accept_topology_clique(graph, &nonzero);
    }

    pub(in crate::circuit) fn force_accept_ground_reachable_nodes(&self) -> Vec<bool> {
        let mut graph = vec![Vec::new(); self.num_nodes() + 1];

        // Ideal voltage-output elements preserve a fixed differential but only
        // anchor common mode when the surrounding physical network provides an
        // absolute reference. Model that supernode connectivity explicitly.
        for idx in 0..self.voltage_sources.len() {
            Self::add_force_accept_topology_edge(
                &mut graph,
                self.voltage_sources.node_pos[idx],
                self.voltage_sources.node_neg[idx],
            );
        }
        for idx in 0..self.vcvs.len() {
            Self::add_force_accept_topology_edge(
                &mut graph,
                self.vcvs.node_pos[idx],
                self.vcvs.node_neg[idx],
            );
        }
        for idx in 0..self.ccvs.len() {
            Self::add_force_accept_topology_edge(
                &mut graph,
                self.ccvs.node_pos[idx],
                self.ccvs.node_neg[idx],
            );
        }
        for source in &self.behavioral_sources.voltage_sources {
            Self::add_force_accept_topology_edge(&mut graph, source.node_pos, source.node_neg);
        }
        self.for_each_xspice_ideal_voltage_constraint(|node_pos, node_neg| {
            Self::add_force_accept_topology_edge(&mut graph, node_pos, node_neg);
        });

        // Physical devices and transient companions anchor node common mode when
        // they provide a path into the grounded circuit.
        for stamp in &self.resistors.stamps {
            Self::add_force_accept_topology_edge(&mut graph, stamp.pp.row, stamp.nn.row);
        }
        for idx in 0..self.resistor_branches.len() {
            Self::add_force_accept_topology_edge(
                &mut graph,
                self.resistor_branches.node_pos[idx],
                self.resistor_branches.node_neg[idx],
            );
        }
        for stamp in &self.capacitors.stamps {
            Self::add_force_accept_topology_edge(&mut graph, stamp.pp.row, stamp.nn.row);
        }
        for idx in 0..self.inductors.len() {
            Self::add_force_accept_topology_edge(
                &mut graph,
                self.inductors.node_pos[idx],
                self.inductors.node_neg[idx],
            );
        }
        for binding in &self.coupled_inductor_pairs {
            Self::add_force_accept_topology_edge(
                &mut graph,
                binding.device.node1_pos,
                binding.device.node1_neg,
            );
            Self::add_force_accept_topology_edge(
                &mut graph,
                binding.device.node2_pos,
                binding.device.node2_neg,
            );
        }
        for binding in &self.multi_winding_transformers {
            for &(node_pos, node_neg) in &binding.device.nodes {
                Self::add_force_accept_topology_edge(&mut graph, node_pos, node_neg);
            }
        }
        for tl in &self.tlines {
            Self::add_force_accept_topology_edge(&mut graph, tl.node1_pos, tl.node2_pos);
            Self::add_force_accept_topology_edge(&mut graph, tl.node1_neg, tl.node2_neg);
        }
        for tl in &self.coupled_tlines {
            for conductor in 0..tl.conductors() {
                Self::add_force_accept_topology_edge(
                    &mut graph,
                    tl.near_nodes[conductor],
                    tl.far_nodes[conductor],
                );
            }
        }
        for diode in &self.diodes.devices {
            Self::add_force_accept_topology_edge(&mut graph, diode.node_anode, diode.node_cathode);
        }
        for bjt in &self.bjts.devices {
            Self::add_force_accept_topology_clique(
                &mut graph,
                &[
                    bjt.node_collector,
                    bjt.node_base,
                    bjt.node_emitter,
                    bjt.node_substrate,
                ],
            );
        }
        for mosfet in &self.mosfets.devices {
            Self::add_force_accept_topology_clique(
                &mut graph,
                &[
                    mosfet.node_drain,
                    mosfet.node_gate,
                    mosfet.node_source,
                    mosfet.node_bulk,
                ],
            );
        }
        for dev in &self.b3soi.devices {
            let (drain, gate, source, back_gate, body) = dev.charge_nodes();
            Self::add_force_accept_topology_clique_nonzero(
                &mut graph,
                &[drain, gate, source, back_gate, body],
            );
        }
        for dev in &self.b3soi_fd.devices {
            let (drain, gate, source, back_gate, body) = dev.charge_nodes();
            Self::add_force_accept_topology_clique_nonzero(
                &mut graph,
                &[drain, gate, source, back_gate, body],
            );
        }
        for dev in &self.b3soi_pd.devices {
            let (drain, gate, source, back_gate, body) = dev.charge_nodes();
            Self::add_force_accept_topology_clique_nonzero(
                &mut graph,
                &[drain, gate, source, back_gate, body],
            );
        }
        for dev in &self.ekv26s.devices {
            Self::add_force_accept_topology_clique(&mut graph, &dev.nodes());
        }
        for dev in &self.ekv3s.devices {
            Self::add_force_accept_topology_clique(&mut graph, &dev.nodes());
        }
        for vdmos in &self.vdmoses.devices {
            Self::add_force_accept_topology_clique(
                &mut graph,
                &[
                    vdmos.drain,
                    vdmos.gate,
                    vdmos.source,
                    vdmos
                        .drain_drift
                        .unwrap_or_else(|| vdmos.drain_int.unwrap_or(vdmos.drain)),
                    vdmos.drain_int.unwrap_or(vdmos.drain),
                    vdmos.source_int.unwrap_or(vdmos.source),
                ],
            );
        }
        for jfet in &self.jfets {
            Self::add_force_accept_topology_clique(
                &mut graph,
                &[jfet.drain, jfet.gate, jfet.source],
            );
        }
        for switch in &self.vswitches {
            Self::add_force_accept_topology_edge(&mut graph, switch.node_pos, switch.node_neg);
        }
        for switch in &self.iswitches {
            Self::add_force_accept_topology_edge(&mut graph, switch.node_pos, switch.node_neg);
        }
        for switch in &self.generic_switches {
            Self::add_force_accept_topology_edge(&mut graph, switch.node_pos, switch.node_neg);
        }

        let mut reachable = vec![false; graph.len()];
        let mut stack = vec![0];
        reachable[0] = true;

        while let Some(node) = stack.pop() {
            for &neighbor in &graph[node] {
                if let Some(flag) = reachable.get_mut(neighbor)
                    && !*flag
                {
                    *flag = true;
                    stack.push(neighbor);
                }
            }
        }

        reachable
    }

    /// Return the ideal voltage-output pairs whose common mode is still
    /// topologically floating with respect to the grounded physical network.
    ///
    /// Force-accept should only clip the common mode of these outputs. Anchored
    /// outputs, like the `V2` source in the VBIC diffamp regression, must be
    /// left to the surrounding circuit equations.
    pub fn ideal_voltage_output_pairs(&self) -> Vec<(NodeId, NodeId)> {
        let mut pairs = Vec::new();
        let mut push_pair = |node_pos: NodeId, node_neg: NodeId| {
            if node_pos != 0 && node_neg != 0 {
                pairs.push((node_pos, node_neg));
            }
        };

        for idx in 0..self.voltage_sources.len() {
            push_pair(
                self.voltage_sources.node_pos[idx],
                self.voltage_sources.node_neg[idx],
            );
        }
        for idx in 0..self.resistor_branches.len() {
            push_pair(
                self.resistor_branches.node_pos[idx],
                self.resistor_branches.node_neg[idx],
            );
        }
        for idx in 0..self.vcvs.len() {
            push_pair(self.vcvs.node_pos[idx], self.vcvs.node_neg[idx]);
        }
        for idx in 0..self.ccvs.len() {
            push_pair(self.ccvs.node_pos[idx], self.ccvs.node_neg[idx]);
        }
        for source in &self.behavioral_sources.voltage_sources {
            push_pair(source.node_pos, source.node_neg);
        }
        self.for_each_xspice_ideal_voltage_constraint(|node_pos, node_neg| {
            push_pair(node_pos, node_neg);
        });
        for instance in &self.xspice_instances {
            for (port_idx, port) in instance.ports().iter().enumerate() {
                let is_voltage_output = matches!(port.direction, crate::xspice::PortDirection::Out)
                    && matches!(
                        port.default_type,
                        crate::xspice::PortType::Voltage
                            | crate::xspice::PortType::DifferentialVoltage
                    );
                if !is_voltage_output {
                    continue;
                }
                match instance.connection_at(port_idx) {
                    Some(crate::xspice::PortConnection::Analog(node)) => push_pair(*node, 0),
                    Some(crate::xspice::PortConnection::AnalogVector(nodes)) => {
                        for node in nodes {
                            push_pair(*node, 0);
                        }
                    }
                    Some(crate::xspice::PortConnection::Differential(pos, neg)) => {
                        push_pair(*pos, *neg)
                    }
                    Some(crate::xspice::PortConnection::TypedAnalogVector(elements)) => {
                        for element in elements {
                            match element {
                                crate::xspice::AnalogInputConnection::Node(node) => {
                                    push_pair(*node, 0);
                                }
                                crate::xspice::AnalogInputConnection::Differential(pos, neg)
                                | crate::xspice::AnalogInputConnection::CurrentOutput {
                                    pos,
                                    neg,
                                }
                                | crate::xspice::AnalogInputConnection::Hybrid {
                                    pos, neg, ..
                                } => push_pair(*pos, *neg),
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        pairs
    }

    pub fn force_accept_floating_ideal_output_pairs(&self) -> Vec<(NodeId, NodeId)> {
        let reachable = self.force_accept_ground_reachable_nodes();
        self.ideal_voltage_output_pairs()
            .into_iter()
            .filter(|&(node_pos, node_neg)| {
                let pos_reachable = reachable.get(node_pos).copied().unwrap_or(false);
                let neg_reachable = reachable.get(node_neg).copied().unwrap_or(false);
                !(pos_reachable || neg_reachable)
            })
            .collect()
    }

    /// Nodes driven by ideal voltage-output elements should not be post-clamped
    /// after a force-accepted Newton step because that would immediately break
    /// the ideal constraint equation the solver is trying to preserve.
    pub fn force_accept_protected_nodes(&self) -> Vec<bool> {
        let mut mask = vec![false; self.num_nodes()];
        let ground_reachable = self.force_accept_ground_reachable_nodes();

        for idx in 0..self.voltage_sources.len() {
            Self::mark_force_accept_protected_node(&mut mask, self.voltage_sources.node_pos[idx]);
            Self::mark_force_accept_protected_node(&mut mask, self.voltage_sources.node_neg[idx]);
        }
        for idx in 0..self.resistor_branches.len() {
            Self::mark_force_accept_protected_node(&mut mask, self.resistor_branches.node_pos[idx]);
            Self::mark_force_accept_protected_node(&mut mask, self.resistor_branches.node_neg[idx]);
        }
        for idx in 0..self.vcvs.len() {
            Self::mark_force_accept_protected_node(&mut mask, self.vcvs.node_pos[idx]);
            Self::mark_force_accept_protected_node(&mut mask, self.vcvs.node_neg[idx]);
        }
        for idx in 0..self.ccvs.len() {
            Self::mark_force_accept_protected_node(&mut mask, self.ccvs.node_pos[idx]);
            Self::mark_force_accept_protected_node(&mut mask, self.ccvs.node_neg[idx]);
        }
        for source in &self.behavioral_sources.voltage_sources {
            Self::mark_force_accept_protected_node(&mut mask, source.node_pos);
            Self::mark_force_accept_protected_node(&mut mask, source.node_neg);
        }
        self.for_each_xspice_ideal_voltage_constraint(|node_pos, node_neg| {
            Self::mark_force_accept_protected_node(&mut mask, node_pos);
            Self::mark_force_accept_protected_node(&mut mask, node_neg);
        });
        for instance in &self.xspice_instances {
            for (port_idx, port) in instance.ports().iter().enumerate() {
                let is_voltage_output = matches!(port.direction, crate::xspice::PortDirection::Out)
                    && matches!(
                        port.default_type,
                        crate::xspice::PortType::Voltage
                            | crate::xspice::PortType::DifferentialVoltage
                    );
                if !is_voltage_output {
                    continue;
                }
                match instance.connection_at(port_idx) {
                    Some(crate::xspice::PortConnection::Analog(node)) => {
                        Self::mark_force_accept_protected_node(&mut mask, *node);
                    }
                    Some(crate::xspice::PortConnection::AnalogVector(nodes)) => {
                        for node in nodes {
                            Self::mark_force_accept_protected_node(&mut mask, *node);
                        }
                    }
                    Some(crate::xspice::PortConnection::Differential(pos, neg)) => {
                        Self::mark_force_accept_protected_node(&mut mask, *pos);
                        Self::mark_force_accept_protected_node(&mut mask, *neg);
                    }
                    Some(crate::xspice::PortConnection::CurrentOutput { pos, neg }) => {
                        Self::mark_force_accept_protected_node(&mut mask, *pos);
                        Self::mark_force_accept_protected_node(&mut mask, *neg);
                    }
                    Some(crate::xspice::PortConnection::TypedAnalogVector(elements)) => {
                        for element in elements {
                            match element {
                                crate::xspice::AnalogInputConnection::Node(node) => {
                                    Self::mark_force_accept_protected_node(&mut mask, *node);
                                }
                                crate::xspice::AnalogInputConnection::Differential(pos, neg)
                                | crate::xspice::AnalogInputConnection::CurrentOutput {
                                    pos,
                                    neg,
                                }
                                | crate::xspice::AnalogInputConnection::Hybrid {
                                    pos, neg, ..
                                } => {
                                    Self::mark_force_accept_protected_node(&mut mask, *pos);
                                    Self::mark_force_accept_protected_node(&mut mask, *neg);
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // A terminal that belongs only to an independent current source and is
        // not connected to the grounded physical/transient network has a
        // gmin-only common-mode bias. Let it jump to that meaningless bias
        // instead of throttling every Newton iteration by the global voltage
        // delta limiter.
        for idx in 0..self.current_sources.len() {
            for node in [
                self.current_sources.node_pos[idx],
                self.current_sources.node_neg[idx],
            ] {
                if node > 0 && !ground_reachable.get(node).copied().unwrap_or(false) {
                    Self::mark_force_accept_protected_node(&mut mask, node);
                }
            }
        }

        mask
    }

    pub(crate) fn xspice_ideal_voltage_constraint_nodes(&self) -> Vec<NodeId> {
        let mut nodes = Vec::new();
        self.for_each_xspice_ideal_voltage_constraint(|node_pos, node_neg| {
            if node_pos > 0 {
                nodes.push(node_pos);
            }
            if node_neg > 0 {
                nodes.push(node_neg);
            }
        });
        nodes.sort_unstable();
        nodes.dedup();
        nodes
    }

    fn for_each_xspice_ideal_voltage_constraint(&self, mut visit: impl FnMut(NodeId, NodeId)) {
        for instance in &self.xspice_instances {
            for connection in instance.connections() {
                match connection {
                    crate::xspice::PortConnection::CurrentProbe { pos, neg, .. }
                    | crate::xspice::PortConnection::Hybrid { pos, neg, .. } => {
                        visit(*pos, *neg);
                    }
                    crate::xspice::PortConnection::TypedAnalogVector(elements) => {
                        for element in elements {
                            match element {
                                crate::xspice::AnalogInputConnection::CurrentProbe {
                                    pos,
                                    neg,
                                    ..
                                }
                                | crate::xspice::AnalogInputConnection::Hybrid {
                                    pos, neg, ..
                                } => visit(*pos, *neg),
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    /// Re-project all ideal voltage-output equations after a force-accepted
    /// timestep so the accepted state remains consistent with independent and
    /// controlled source constraints.
    pub fn enforce_ideal_voltage_constraints(&self, solution: &mut [Value], time: Value) -> bool {
        let mut changed = self
            .voltage_sources
            .enforce_voltage_constraints(solution, time);
        changed |= self.enforce_dependent_voltage_constraints(solution);
        changed
    }

    /// Re-project ideal voltage-output equations for a DC operating-point
    /// iterate, using independent source DC values rather than any transient
    /// waveform value at t=0.
    pub fn enforce_dc_ideal_voltage_constraints(&self, solution: &mut [Value]) -> bool {
        let mut changed = self
            .voltage_sources
            .enforce_dc_voltage_constraints(solution);
        changed |= self.enforce_dependent_voltage_constraints(solution);
        changed
    }

    /// Re-project ideal voltage-output equations for source-stepping DC solves.
    pub fn enforce_scaled_dc_ideal_voltage_constraints(
        &self,
        solution: &mut [Value],
        source_scale: Value,
    ) -> bool {
        let mut changed = self
            .voltage_sources
            .enforce_scaled_dc_voltage_constraints(solution, source_scale);
        changed |= self.enforce_dependent_voltage_constraints(solution);
        changed
    }

    fn enforce_dependent_voltage_constraints(&self, solution: &mut [Value]) -> bool {
        let mut changed = self
            .resistor_branches
            .enforce_voltage_constraints(solution, self.num_nodes);

        for idx in 0..self.vcvs.len() {
            let Some(v_ctrl_pos) = solution_node_voltage(solution, self.vcvs.ctrl_pos[idx]) else {
                continue;
            };
            let Some(v_ctrl_neg) = solution_node_voltage(solution, self.vcvs.ctrl_neg[idx]) else {
                continue;
            };
            let target_voltage = self.vcvs.gains[idx] * (v_ctrl_pos - v_ctrl_neg);
            changed |= project_two_terminal_voltage(
                solution,
                self.vcvs.node_pos[idx],
                self.vcvs.node_neg[idx],
                target_voltage,
            );
        }

        for idx in 0..self.ccvs.len() {
            let ctrl_branch = self.ccvs.ctrl_branch[idx];
            if ctrl_branch == 0 {
                continue;
            }
            let ctrl_idx = self.get_branch_matrix_index(ctrl_branch).saturating_sub(1);
            let Some(&ctrl_current) = solution.get(ctrl_idx) else {
                continue;
            };
            if !ctrl_current.is_finite() {
                continue;
            }
            let target_voltage = self.ccvs.transresistances[idx] * ctrl_current;
            changed |= project_two_terminal_voltage(
                solution,
                self.ccvs.node_pos[idx],
                self.ccvs.node_neg[idx],
                target_voltage,
            );
        }
        changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::{BehavioralVoltageSource, Vdmos};
    use crate::xspice::{PortConnection, XspiceInstance, models::Gain};
    use std::sync::Arc;

    #[test]
    fn vdmos_drain_drift_participates_in_force_accept_topology() {
        let mut circuit = CircuitData::new();
        let drain = circuit.get_or_create_node("d");
        let gate = circuit.get_or_create_node("g");
        let source = circuit.get_or_create_node("s");
        let drain_drift = circuit.get_or_create_node("dd");
        let drain_int = circuit.get_or_create_node("di");

        circuit.resistors.add("ranchor".to_string(), 0, drain, 1.0);
        let mut vdmos = Vdmos::new_nvdmos("m1".to_string(), drain, gate, source);
        vdmos.set_drain_drift_node(drain_drift);
        vdmos.set_internal_nodes(drain_int, source);
        circuit.vdmoses.add(vdmos);

        let reachable = circuit.force_accept_ground_reachable_nodes();

        assert!(reachable[drain_drift]);
        assert!(reachable[drain_int]);
    }

    #[test]
    fn behavioral_voltage_source_output_is_force_accept_protected() {
        let mut circuit = CircuitData::new();
        let out = circuit.get_or_create_node("out");
        let source =
            BehavioralVoltageSource::new("b1".to_string(), out, 0, 1, "1.0").expect("b source");
        circuit.behavioral_sources.add_voltage(source);

        let protected = circuit.force_accept_protected_nodes();

        assert!(protected[out - 1]);
    }

    #[test]
    fn xspice_current_probe_is_force_accept_protected_like_zero_volt_source() {
        let mut circuit = CircuitData::new();
        let sense = circuit.get_or_create_node("sense");
        let out = circuit.get_or_create_node("out");
        let branch = circuit.allocate_branch_named("a1#in#sense");
        let instance = XspiceInstance::new(
            "a1",
            Arc::new(Gain),
            vec![
                PortConnection::CurrentProbe {
                    pos: sense,
                    neg: 0,
                    branch_ordinal: branch,
                },
                PortConnection::Analog(out),
            ],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("gain instance with current probe constructs");
        circuit.add_xspice_instance(instance);

        let reachable = circuit.force_accept_ground_reachable_nodes();
        let protected = circuit.force_accept_protected_nodes();
        let lte_excluded_nodes = circuit.xspice_ideal_voltage_constraint_nodes();

        assert!(reachable[sense]);
        assert!(protected[sense - 1]);
        assert!(lte_excluded_nodes.contains(&sense));
    }

    #[test]
    fn xspice_current_output_is_force_accept_protected() {
        let mut circuit = CircuitData::new();
        let input = circuit.get_or_create_node("in");
        let out = circuit.get_or_create_node("out");
        let instance = XspiceInstance::new(
            "a1",
            Arc::new(Gain),
            vec![
                PortConnection::Analog(input),
                PortConnection::CurrentOutput { pos: out, neg: 0 },
            ],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("gain instance with current output constructs");
        circuit.add_xspice_instance(instance);

        let protected = circuit.force_accept_protected_nodes();

        assert!(protected[out - 1]);
    }
}
