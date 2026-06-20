use super::*;

impl CircuitData {
    #[inline]
    pub(in crate::circuit) fn mark_force_accept_protected_node(mask: &mut [bool], node: NodeId) {
        if node > 0 {
            if let Some(slot) = mask.get_mut(node - 1) {
                *slot = true;
            }
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

        // Physical devices and transient companions anchor node common mode when
        // they provide a path into the grounded circuit.
        for stamp in &self.resistors.stamps {
            Self::add_force_accept_topology_edge(&mut graph, stamp.pp.row, stamp.nn.row);
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
        for idx in 0..self.vcvs.len() {
            push_pair(self.vcvs.node_pos[idx], self.vcvs.node_neg[idx]);
        }
        for idx in 0..self.ccvs.len() {
            push_pair(self.ccvs.node_pos[idx], self.ccvs.node_neg[idx]);
        }
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
                    Some(crate::xspice::PortConnection::Differential(pos, neg)) => {
                        push_pair(*pos, *neg)
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

        for idx in 0..self.voltage_sources.len() {
            Self::mark_force_accept_protected_node(&mut mask, self.voltage_sources.node_pos[idx]);
            Self::mark_force_accept_protected_node(&mut mask, self.voltage_sources.node_neg[idx]);
        }
        for idx in 0..self.vcvs.len() {
            Self::mark_force_accept_protected_node(&mut mask, self.vcvs.node_pos[idx]);
            Self::mark_force_accept_protected_node(&mut mask, self.vcvs.node_neg[idx]);
        }
        for idx in 0..self.ccvs.len() {
            Self::mark_force_accept_protected_node(&mut mask, self.ccvs.node_pos[idx]);
            Self::mark_force_accept_protected_node(&mut mask, self.ccvs.node_neg[idx]);
        }
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
                    Some(crate::xspice::PortConnection::Differential(pos, neg)) => {
                        Self::mark_force_accept_protected_node(&mut mask, *pos);
                        Self::mark_force_accept_protected_node(&mut mask, *neg);
                    }
                    _ => {}
                }
            }
        }

        mask
    }

    /// Re-project all ideal voltage-output equations after a force-accepted
    /// timestep so the accepted state remains consistent with independent and
    /// controlled source constraints.
    pub fn enforce_ideal_voltage_constraints(&self, solution: &mut [Value], time: Value) -> bool {
        let mut changed = self
            .voltage_sources
            .enforce_voltage_constraints(solution, time);

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
    use crate::device::Vdmos;

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
}
