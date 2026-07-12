use super::*;

impl CircuitData {
    pub fn new() -> Self {
        let mut node_map = HashMap::new();
        // Ground is always node 0
        node_map.insert("0".to_string(), 0);
        node_map.insert("gnd".to_string(), 0);
        node_map.insert("GND".to_string(), 0);

        Self {
            node_map,
            has_explicit_ground_reference: false,
            branch_names: HashMap::new(),
            branch_name_by_ordinal: Vec::new(),
            num_nodes: 0,
            num_branches: 0,
            resistors: Resistors::new(),
            resistor_branches: ResistorBranches::new(),
            capacitors: Capacitors::new(),
            inductors: Inductors::new(),
            voltage_sources: VoltageSources::new(),
            current_sources: CurrentSources::new(),
            diodes: Diodes::new(),
            bjts: Bjts::new(),
            mosfets: Mosfets::new(),
            b3soi: B3SoiDds::new(),
            b3soi_fd: B3SoiFds::new(),
            b3soi_pd: B3SoiPds::new(),
            b3soi_gmin_scale: 1.0e-6,
            bsim3v3: Bsim3v3s::new(),
            bsim4v8: Bsim4v8s::new(),
            ekv26s: EkvMosfets::new(),
            ekv3s: Ekv3Mosfets::new(),
            vdmoses: Vdmoses::new(),
            jfets: Vec::new(),
            vcvs: Vcvs::new(),
            vccs: Vccs::new(),
            cccs: Cccs::new(),
            ccvs: Ccvs::new(),
            pending_cccs: Vec::new(),
            pending_ccvs: Vec::new(),
            pending_iswitch: Vec::new(),
            // New device types
            vswitches: Vec::new(),
            iswitches: Vec::new(),
            generic_switches: Vec::new(),
            tlines: Vec::new(),
            coupled_tlines: Vec::new(),
            couplings: Vec::new(),
            coupled_inductor_pairs: Vec::new(),
            multi_winding_transformers: Vec::new(),
            jiles_atherton_inductors: Vec::new(),
            transient_max_step_hint: None,
            behavioral_sources: BehavioralSources::new(),
            // XSPICE instances
            xspice_instances: Vec::new(),
            xspice_has_event_driven_devices: false,
            xspice_digital_values: HashMap::new(),
            xspice_digital_drivers: HashMap::new(),
            xspice_digital_event_times: HashMap::new(),
            xspice_event_loads: HashMap::new(),
            xspice_real_values: HashMap::new(),
            xspice_real_drivers: HashMap::new(),
            xspice_real_event_times: HashMap::new(),
            xspice_event_queue: EventQueue::new(),
            xspice_touched_digital_nodes: Vec::new(),
            xspice_touched_real_nodes: Vec::new(),
            xspice_registry: Arc::new(CodeModelRegistry::with_builtins()),
            xspice_evaluation_error: None,
            // Verilog-A devices
            #[cfg(feature = "veriloga")]
            veriloga_devices: crate::device::veriloga::VerilogADevices::new(),
            #[cfg(feature = "veriloga-builtins")]
            generated_veriloga_devices:
                crate::device::veriloga_generated::BuiltinVerilogADevices::new(),
            #[cfg(feature = "veriloga-builtins")]
            generated_simulation_parameters:
                crate::device::veriloga_generated::GeneratedSimulationParameters::default(),
        }
    }

    /// Get or create a node ID for the given name
    /// Node "0" is always ground (NodeId 0) - this is the SPICE standard
    pub fn get_or_create_node(&mut self, name: &str) -> NodeId {
        if Self::is_ground_name(name) {
            self.has_explicit_ground_reference = true;
            return 0;
        }

        if let Some(&id) = self.node_map.get(name) {
            return id;
        }

        self.num_nodes += 1;
        self.node_map.insert(name.to_string(), self.num_nodes);
        self.num_nodes
    }

    /// Tighten the circuit-level transient max-step hint.
    pub fn tighten_transient_max_step_hint(&mut self, hint: Value) {
        if !hint.is_finite() || hint <= 0.0 {
            return;
        }

        self.transient_max_step_hint = Some(
            self.transient_max_step_hint
                .map_or(hint, |existing| existing.min(hint)),
        );
    }

    #[inline]
    pub(in crate::circuit) fn is_ground_name(name: &str) -> bool {
        crate::compat::ground::is_spice_ground_name(name)
    }

    /// Look up an existing node ID by name.
    pub fn get_node_by_name(&self, name: &str) -> Option<NodeId> {
        self.node_map
            .get(name)
            .copied()
            .or_else(|| self.node_map.get(&name.to_lowercase()).copied())
            .or_else(|| self.node_map.get(&name.to_uppercase()).copied())
            .or_else(|| {
                self.node_map
                    .iter()
                    .find_map(|(candidate, &id)| candidate.eq_ignore_ascii_case(name).then_some(id))
            })
    }

    /// Resolve behavioral expression references against current node/branch maps.
    pub fn bind_behavioral_references(&mut self) -> Result<(), CircuitError> {
        let node_lookup = self.node_map.clone();
        let branch_lookup = self.branch_names.clone();
        let num_nodes = self.num_nodes;
        self.behavioral_sources
            .bind_references(
                |name: &str| {
                    node_lookup
                        .get(name)
                        .copied()
                        .or_else(|| node_lookup.get(&name.to_lowercase()).copied())
                        .or_else(|| node_lookup.get(&name.to_uppercase()).copied())
                        .or_else(|| {
                            node_lookup.iter().find_map(|(candidate, &id)| {
                                candidate.eq_ignore_ascii_case(name).then_some(id)
                            })
                        })
                },
                |name: &str| {
                    branch_lookup
                        .get(name)
                        .or_else(|| branch_lookup.get(&name.to_uppercase()))
                        .copied()
                        .map(|ordinal| num_nodes + ordinal - 1)
                },
            )
            .map_err(CircuitError::InvalidComponent)
    }

    /// Check whether the circuit explicitly referenced the SPICE ground node.
    pub fn has_explicit_ground_reference(&self) -> bool {
        self.has_explicit_ground_reference
    }

    /// Ensure a ground reference exists. If no explicit node "0" was specified,
    /// pick the first node connected to a voltage source's negative terminal
    /// as the reference.
    /// This should be called after all elements are added but before simulation.
    pub fn ensure_ground_reference(&mut self) {
        if self.has_explicit_ground_reference() {
            return; // Already have explicit ground
        }

        // No explicit ground - pick first voltage source's negative terminal
        // This matches standard behavior
        if !self.voltage_sources.is_empty() {
            let ref_node_id = self.voltage_sources.node_neg[0];
            if ref_node_id > 0 {
                // Find the name of this node and remap it to 0
                let mut ref_node_name = None;
                for (name, &id) in &self.node_map {
                    if id == ref_node_id {
                        ref_node_name = Some(name.clone());
                        break;
                    }
                }

                if let Some(name) = ref_node_name {
                    // Remap this node to ground (0)
                    self.remap_node_to_ground(ref_node_id);
                    log::info!("Auto-selected node '{}' as ground reference", name);
                }
            }
        }
    }

    /// Remap all occurrences of old_node_id to ground (0) and shift all higher
    /// node IDs down by 1 to maintain contiguous matrix indices
    pub(in crate::circuit) fn remap_node_to_ground(&mut self, old_node_id: NodeId) {
        // Update node map
        for (_, id) in self.node_map.iter_mut() {
            *id = Self::remap_node_id(*id, old_node_id);
        }

        // Update all device node references
        // Resistors
        for stamp in &mut self.resistors.stamps {
            Self::remap_stamp_full(stamp, old_node_id);
        }

        // Capacitors
        for stamp in &mut self.capacitors.stamps {
            Self::remap_stamp_full(stamp, old_node_id);
        }

        // Voltage sources
        Self::remap_node_slice(&mut self.voltage_sources.node_pos, old_node_id);
        Self::remap_node_slice(&mut self.voltage_sources.node_neg, old_node_id);

        // Branch-form resistors
        Self::remap_node_slice(&mut self.resistor_branches.node_pos, old_node_id);
        Self::remap_node_slice(&mut self.resistor_branches.node_neg, old_node_id);

        // Current sources
        Self::remap_node_slice(&mut self.current_sources.node_pos, old_node_id);
        Self::remap_node_slice(&mut self.current_sources.node_neg, old_node_id);

        // Inductors
        Self::remap_node_slice(&mut self.inductors.node_pos, old_node_id);
        Self::remap_node_slice(&mut self.inductors.node_neg, old_node_id);
        for diode in &mut self.diodes.devices {
            diode.remap_nodes(old_node_id);
        }
        for bjt in &mut self.bjts.devices {
            bjt.node_collector = Self::remap_node_id(bjt.node_collector, old_node_id);
            bjt.node_base = Self::remap_node_id(bjt.node_base, old_node_id);
            bjt.node_emitter = Self::remap_node_id(bjt.node_emitter, old_node_id);
            bjt.node_substrate = Self::remap_node_id(bjt.node_substrate, old_node_id);
        }
        for mosfet in &mut self.mosfets.devices {
            mosfet.node_drain = Self::remap_node_id(mosfet.node_drain, old_node_id);
            mosfet.node_gate = Self::remap_node_id(mosfet.node_gate, old_node_id);
            mosfet.node_source = Self::remap_node_id(mosfet.node_source, old_node_id);
            mosfet.node_bulk = Self::remap_node_id(mosfet.node_bulk, old_node_id);
        }
        for dev in &mut self.bsim3v3.devices {
            dev.node_drain = Self::remap_node_id(dev.node_drain, old_node_id);
            dev.node_gate = Self::remap_node_id(dev.node_gate, old_node_id);
            dev.node_source = Self::remap_node_id(dev.node_source, old_node_id);
            dev.node_bulk = Self::remap_node_id(dev.node_bulk, old_node_id);
            dev.node_charge_deficit = Self::remap_node_id(dev.node_charge_deficit, old_node_id);
        }
        for dev in &mut self.bsim4v8.devices {
            dev.node_drain_external = Self::remap_node_id(dev.node_drain_external, old_node_id);
            dev.node_drain = Self::remap_node_id(dev.node_drain, old_node_id);
            dev.node_gate_external = Self::remap_node_id(dev.node_gate_external, old_node_id);
            dev.node_gate_mid = Self::remap_node_id(dev.node_gate_mid, old_node_id);
            dev.node_gate = Self::remap_node_id(dev.node_gate, old_node_id);
            dev.node_source_external = Self::remap_node_id(dev.node_source_external, old_node_id);
            dev.node_source = Self::remap_node_id(dev.node_source, old_node_id);
            dev.node_bulk_external = Self::remap_node_id(dev.node_bulk_external, old_node_id);
            dev.node_bulk = Self::remap_node_id(dev.node_bulk, old_node_id);
            dev.node_drain_body = Self::remap_node_id(dev.node_drain_body, old_node_id);
            dev.node_source_body = Self::remap_node_id(dev.node_source_body, old_node_id);
            dev.node_charge_deficit = Self::remap_node_id(dev.node_charge_deficit, old_node_id);
        }
        for dev in &mut self.ekv26s.devices {
            dev.node_drain = Self::remap_node_id(dev.node_drain, old_node_id);
            dev.node_gate = Self::remap_node_id(dev.node_gate, old_node_id);
            dev.node_source = Self::remap_node_id(dev.node_source, old_node_id);
            dev.node_bulk = Self::remap_node_id(dev.node_bulk, old_node_id);
        }
        for dev in &mut self.ekv3s.devices {
            dev.node_drain = Self::remap_node_id(dev.node_drain, old_node_id);
            dev.node_gate = Self::remap_node_id(dev.node_gate, old_node_id);
            dev.node_source = Self::remap_node_id(dev.node_source, old_node_id);
            dev.node_bulk = Self::remap_node_id(dev.node_bulk, old_node_id);
        }
        for vdmos in &mut self.vdmoses.devices {
            vdmos.drain = Self::remap_node_id(vdmos.drain, old_node_id);
            vdmos.gate = Self::remap_node_id(vdmos.gate, old_node_id);
            vdmos.source = Self::remap_node_id(vdmos.source, old_node_id);
            vdmos.bulk = Self::remap_node_id(vdmos.bulk, old_node_id);
            vdmos.drain_int = vdmos
                .drain_int
                .map(|node| Self::remap_node_id(node, old_node_id));
            vdmos.drain_drift = vdmos
                .drain_drift
                .map(|node| Self::remap_node_id(node, old_node_id));
            vdmos.source_int = vdmos
                .source_int
                .map(|node| Self::remap_node_id(node, old_node_id));
            vdmos.d1_prime = vdmos
                .d1_prime
                .map(|node| Self::remap_node_id(node, old_node_id));
        }
        for jfet in &mut self.jfets {
            jfet.drain = Self::remap_node_id(jfet.drain, old_node_id);
            jfet.gate = Self::remap_node_id(jfet.gate, old_node_id);
            jfet.source = Self::remap_node_id(jfet.source, old_node_id);
            jfet.external_drain = Self::remap_node_id(jfet.external_drain, old_node_id);
            jfet.external_source = Self::remap_node_id(jfet.external_source, old_node_id);
        }
        Self::remap_node_slice(&mut self.vcvs.node_pos, old_node_id);
        Self::remap_node_slice(&mut self.vcvs.node_neg, old_node_id);
        Self::remap_node_slice(&mut self.vcvs.ctrl_pos, old_node_id);
        Self::remap_node_slice(&mut self.vcvs.ctrl_neg, old_node_id);
        Self::remap_node_slice(&mut self.vccs.node_pos, old_node_id);
        Self::remap_node_slice(&mut self.vccs.node_neg, old_node_id);
        Self::remap_node_slice(&mut self.vccs.ctrl_pos, old_node_id);
        Self::remap_node_slice(&mut self.vccs.ctrl_neg, old_node_id);
        Self::remap_node_slice(&mut self.cccs.node_pos, old_node_id);
        Self::remap_node_slice(&mut self.cccs.node_neg, old_node_id);
        Self::remap_node_slice(&mut self.ccvs.node_pos, old_node_id);
        Self::remap_node_slice(&mut self.ccvs.node_neg, old_node_id);
        for switch in &mut self.vswitches {
            switch.node_pos = Self::remap_node_id(switch.node_pos, old_node_id);
            switch.node_neg = Self::remap_node_id(switch.node_neg, old_node_id);
            switch.ctrl_pos = Self::remap_node_id(switch.ctrl_pos, old_node_id);
            switch.ctrl_neg = Self::remap_node_id(switch.ctrl_neg, old_node_id);
        }
        for switch in &mut self.iswitches {
            switch.node_pos = Self::remap_node_id(switch.node_pos, old_node_id);
            switch.node_neg = Self::remap_node_id(switch.node_neg, old_node_id);
        }
        for switch in &mut self.generic_switches {
            switch.node_pos = Self::remap_node_id(switch.node_pos, old_node_id);
            switch.node_neg = Self::remap_node_id(switch.node_neg, old_node_id);
        }
        for tline in &mut self.tlines {
            tline.node1_pos = Self::remap_node_id(tline.node1_pos, old_node_id);
            tline.node1_neg = Self::remap_node_id(tline.node1_neg, old_node_id);
            tline.node2_pos = Self::remap_node_id(tline.node2_pos, old_node_id);
            tline.node2_neg = Self::remap_node_id(tline.node2_neg, old_node_id);
        }
        for tline in &mut self.coupled_tlines {
            Self::remap_node_slice(&mut tline.near_nodes, old_node_id);
            Self::remap_node_slice(&mut tline.far_nodes, old_node_id);
            tline.near_ref = Self::remap_node_id(tline.near_ref, old_node_id);
            tline.far_ref = Self::remap_node_id(tline.far_ref, old_node_id);
        }
        for binding in &mut self.coupled_inductor_pairs {
            binding.device.node1_pos = Self::remap_node_id(binding.device.node1_pos, old_node_id);
            binding.device.node1_neg = Self::remap_node_id(binding.device.node1_neg, old_node_id);
            binding.device.node2_pos = Self::remap_node_id(binding.device.node2_pos, old_node_id);
            binding.device.node2_neg = Self::remap_node_id(binding.device.node2_neg, old_node_id);
        }
        for binding in &mut self.multi_winding_transformers {
            for (pos, neg) in &mut binding.device.nodes {
                *pos = Self::remap_node_id(*pos, old_node_id);
                *neg = Self::remap_node_id(*neg, old_node_id);
            }
        }
        for binding in &mut self.jiles_atherton_inductors {
            binding.device.node_pos = Self::remap_node_id(binding.device.node_pos, old_node_id);
            binding.device.node_neg = Self::remap_node_id(binding.device.node_neg, old_node_id);
        }

        // Behavioral sources
        for source in &mut self.behavioral_sources.voltage_sources {
            source.node_pos = Self::remap_node_id(source.node_pos, old_node_id);
            source.node_neg = Self::remap_node_id(source.node_neg, old_node_id);
        }
        for source in &mut self.behavioral_sources.current_sources {
            source.node_pos = Self::remap_node_id(source.node_pos, old_node_id);
            source.node_neg = Self::remap_node_id(source.node_neg, old_node_id);
        }

        for instance in &mut self.xspice_instances {
            instance.remap_circuit_nodes(|node| Self::remap_node_id(node, old_node_id));
        }

        #[cfg(feature = "veriloga")]
        self.veriloga_devices
            .remap_circuit_nodes(|node| Self::remap_node_id(node, old_node_id));

        self.has_explicit_ground_reference = true;

        // Decrement num_nodes since one node is now ground
        if self.num_nodes > 0 {
            self.num_nodes -= 1;
        }
    }

    /// Helper to remap a two-terminal stamp with full shifting
    pub(in crate::circuit) fn remap_stamp_full(stamp: &mut TwoTerminalStamp, old_id: NodeId) {
        stamp.pp.row = Self::remap_node_id(stamp.pp.row, old_id);
        stamp.pp.col = Self::remap_node_id(stamp.pp.col, old_id);
        stamp.pn.row = Self::remap_node_id(stamp.pn.row, old_id);
        stamp.pn.col = Self::remap_node_id(stamp.pn.col, old_id);
        stamp.np.row = Self::remap_node_id(stamp.np.row, old_id);
        stamp.np.col = Self::remap_node_id(stamp.np.col, old_id);
        stamp.nn.row = Self::remap_node_id(stamp.nn.row, old_id);
        stamp.nn.col = Self::remap_node_id(stamp.nn.col, old_id);
    }

    #[inline]
    pub(in crate::circuit) fn remap_node_id(id: NodeId, old_id: NodeId) -> NodeId {
        if id == old_id {
            0
        } else if id > old_id {
            id - 1
        } else {
            id
        }
    }

    pub(in crate::circuit) fn remap_node_slice(nodes: &mut [NodeId], old_id: NodeId) {
        for node in nodes {
            *node = Self::remap_node_id(*node, old_id);
        }
    }

    /// Allocate a branch current variable - returns branch ordinal (1-indexed)
    /// Note: The stored value is the branch ordinal, NOT the matrix index.
    /// Use get_branch_matrix_index() to get the actual matrix row/column.
    pub fn allocate_branch(&mut self) -> NodeId {
        self.num_branches += 1;
        self.branch_name_by_ordinal.push(None);
        self.num_branches // Return branch ordinal (1, 2, 3...)
    }

    /// Allocate a branch and register it with the given element name
    /// This allows CCCS/CCVS to look up control branches by name
    pub fn allocate_branch_named(&mut self, name: &str) -> NodeId {
        let branch = self.allocate_branch();
        if let Some(slot) = self.branch_name_by_ordinal.get_mut(branch - 1) {
            *slot = Some(name.to_string());
        }
        // Store both original and uppercase for case-insensitive lookup
        self.branch_names.insert(name.to_string(), branch);
        self.branch_names.insert(name.to_uppercase(), branch);
        branch
    }

    /// Look up a branch ordinal by element name (for CCCS/CCVS control element)
    /// Returns None if the element is not found
    pub fn get_branch_by_name(&self, name: &str) -> Option<NodeId> {
        self.branch_names
            .get(name)
            .or_else(|| self.branch_names.get(&name.to_uppercase()))
            .copied()
    }

    /// Return the set of branch-bearing element names that can be used as probes.
    pub fn branch_probe_names(&self) -> Vec<String> {
        let mut names = Vec::with_capacity(
            self.inductors.names.len()
                + self.resistor_branches.names.len()
                + self.voltage_sources.names.len()
                + self.ccvs.len()
                + self.behavioral_sources.voltage_sources.len(),
        );
        names.extend(self.inductors.names.iter().cloned());
        names.extend(self.resistor_branches.names.iter().cloned());
        names.extend(self.voltage_sources.names.iter().cloned());
        names.extend(self.ccvs.names.iter().cloned());
        names.extend(
            self.behavioral_sources
                .voltage_sources
                .iter()
                .map(|source| source.name.clone()),
        );
        names
    }

    /// Return the canonical set of inductor probe names.
    pub fn inductor_probe_names(&self) -> Vec<String> {
        self.inductors.names.clone()
    }

    /// Resolve a probe name to the inductor state tracked during periodic RF analyses.
    pub fn resolve_inductor_probe(&self, probe_name: &str) -> Option<InductorProbeInfo> {
        let branch_ordinal = self.get_branch_by_name(probe_name)?;
        self.inductor_probe_for_branch(branch_ordinal)
    }

    /// Resolve an existing branch ordinal to the owning inductor probe metadata.
    pub fn inductor_probe_for_branch(&self, branch_ordinal: NodeId) -> Option<InductorProbeInfo> {
        let inductor_index = self
            .inductors
            .branch_indices
            .iter()
            .position(|branch| *branch == branch_ordinal)?;

        Some(InductorProbeInfo {
            canonical_name: self.inductors.names.get(inductor_index)?.clone(),
            branch_ordinal,
            state_index: self.capacitors.len() + inductor_index,
        })
    }

    /// Register a CCCS element for pending control branch resolution
    /// The control_element_name will be resolved after all elements are added
    pub fn add_cccs_pending(&mut self, cccs_index: usize, control_element_name: String) {
        self.pending_cccs.push((cccs_index, control_element_name));
    }

    /// Register a CCVS element for pending control branch resolution
    pub fn add_ccvs_pending(&mut self, ccvs_index: usize, control_element_name: String) {
        self.pending_ccvs.push((ccvs_index, control_element_name));
    }

    /// Register an ISWITCH element for pending control branch resolution.
    pub fn add_iswitch_pending(&mut self, iswitch_index: usize, control_element_name: String) {
        self.pending_iswitch
            .push((iswitch_index, control_element_name));
    }

    /// Register a Jiles-Atherton inductor runtime binding.
    pub fn add_jiles_atherton_inductor(
        &mut self,
        inductor_index: usize,
        branch_ordinal: NodeId,
        device: crate::device::passive::JilesAthertonInductor,
    ) {
        self.jiles_atherton_inductors.push(JilesAthertonBinding {
            inductor_index,
            branch_ordinal,
            device,
        });
    }

    /// Register a coupled inductor pair runtime binding.
    pub fn add_coupled_inductor_pair(
        &mut self,
        branch1_ordinal: NodeId,
        branch2_ordinal: NodeId,
        device: crate::device::CoupledInductorPair,
    ) {
        self.coupled_inductor_pairs
            .push(CoupledInductorPairBinding {
                branch1_ordinal,
                branch2_ordinal,
                device,
            });
    }

    /// Register a multi-winding transformer runtime binding.
    pub fn add_multi_winding_transformer(
        &mut self,
        branch_ordinals: Vec<NodeId>,
        device: crate::device::MultiWindingTransformer,
    ) {
        self.multi_winding_transformers
            .push(MultiWindingTransformerBinding {
                branch_ordinals,
                device,
            });
    }

    /// Resolve all pending CCCS/CCVS/ISWITCH control element references.
    /// Call this after all elements have been added to the circuit
    /// Returns an error if any control element is not found
    pub fn resolve_control_elements(&mut self) -> Result<(), CircuitError> {
        // Resolve CCCS control branches
        for (cccs_idx, control_name) in self.pending_cccs.drain(..).collect::<Vec<_>>() {
            let branch = self.get_branch_by_name(&control_name).ok_or_else(|| {
                CircuitError::InvalidComponent(format!(
                    "CCCS control element not found: {}",
                    control_name
                ))
            })?;
            if cccs_idx < self.cccs.ctrl_branch.len() {
                self.cccs.ctrl_branch[cccs_idx] = branch;
            }
        }

        // Resolve CCVS control branches
        for (ccvs_idx, control_name) in self.pending_ccvs.drain(..).collect::<Vec<_>>() {
            let branch = self.get_branch_by_name(&control_name).ok_or_else(|| {
                CircuitError::InvalidComponent(format!(
                    "CCVS control element not found: {}",
                    control_name
                ))
            })?;
            if ccvs_idx < self.ccvs.ctrl_branch.len() {
                self.ccvs.ctrl_branch[ccvs_idx] = branch;
            }
        }

        // Resolve current-controlled switch control branches.
        // CurrentSwitch expects a matrix variable index (1-based) so convert
        // from branch ordinal after final node count is known.
        for (iswitch_idx, control_name) in self.pending_iswitch.drain(..).collect::<Vec<_>>() {
            let branch_ordinal = self.get_branch_by_name(&control_name).ok_or_else(|| {
                CircuitError::InvalidComponent(format!(
                    "ISWITCH control element not found: {}",
                    control_name
                ))
            })?;
            let branch_matrix_index = self.get_branch_matrix_index(branch_ordinal);
            if let Some(sw) = self.iswitches.get_mut(iswitch_idx) {
                sw.set_ctrl_branch(branch_matrix_index);
            }
        }

        Ok(())
    }

    /// Resolve XSPICE `%vnam` branch-current references after all branch-bearing
    /// elements have been allocated.
    pub fn resolve_xspice_branch_references(&mut self) -> Result<(), CircuitError> {
        let branch_lookup = self.branch_names.clone();
        let current_sources = self.current_sources.clone();
        for instance in &mut self.xspice_instances {
            instance
                .resolve_branch_references(
                    |name| {
                        branch_lookup
                            .get(name)
                            .or_else(|| branch_lookup.get(&name.to_uppercase()))
                            .copied()
                    },
                    |name| current_sources.index_by_name(name),
                )
                .map_err(|err| CircuitError::InvalidComponent(err.to_string()))?;
        }
        Ok(())
    }

    /// Convert branch ordinal to matrix index
    /// Branch ordinals start at 1, matrix indices for branches start at num_nodes
    pub fn get_branch_matrix_index(&self, branch_ordinal: NodeId) -> usize {
        self.num_nodes + branch_ordinal
    }

    /// Total matrix size
    pub fn matrix_size(&self) -> usize {
        self.num_nodes + self.num_branches
    }

    /// Number of nodes (excluding ground)
    pub fn num_nodes(&self) -> usize {
        self.num_nodes
    }

    /// Number of branches
    pub fn num_branches(&self) -> usize {
        self.num_branches
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::Vdmos;

    #[test]
    fn vdmos_remap_updates_drain_drift_node() {
        let mut circuit = CircuitData::new();
        let drain = circuit.get_or_create_node("d");
        let gate = circuit.get_or_create_node("g");
        let source = circuit.get_or_create_node("s");
        let bulk = circuit.get_or_create_node("b");
        let drain_drift = circuit.get_or_create_node("dd");
        let drain_int = circuit.get_or_create_node("di");
        let source_int = circuit.get_or_create_node("si");
        let d1_prime = circuit.get_or_create_node("d1p");

        let mut vdmos = Vdmos::new_nvdmos("m1".to_string(), drain, gate, source);
        vdmos.set_bulk_node(bulk);
        vdmos.set_drain_drift_node(drain_drift);
        vdmos.set_internal_nodes(drain_int, source_int);
        vdmos.set_d1_prime_node(d1_prime);
        circuit.vdmoses.add(vdmos);

        circuit.remap_node_to_ground(drain_drift);

        let vdmos = &circuit.vdmoses.devices[0];
        assert_eq!(vdmos.drain_drift, Some(0));
        assert_eq!(vdmos.drain_int, Some(drain_int - 1));
        assert_eq!(vdmos.source_int, Some(source_int - 1));
        assert_eq!(vdmos.d1_prime, Some(d1_prime - 1));
    }
}
