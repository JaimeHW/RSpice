//! Building a [`CircuitData`] up from parsed netlist elements.
//!
//! Holds the `CircuitData` constructors and the `add_*` element entry points,
//! plus branch allocation and the deferred wiring that cannot be resolved at
//! insertion time: current-controlled sources naming a controlling element
//! that may not exist yet, and behavioral sources whose expressions reference
//! nodes and branches by name. Those land as `*_pending` entries and are
//! bound once the whole deck has been added.

use super::*;
use std::collections::HashSet;

impl CircuitData {
    pub fn new() -> Self {
        let mut node_map = HashMap::new();
        // Ground is always node 0
        node_map.insert("0".to_string(), 0);

        Self {
            node_map,
            has_explicit_ground_reference: false,
            branch_names: HashMap::new(),
            branch_name_by_ordinal: Vec::new(),
            num_nodes: 0,
            net_kinds: NetKinds::default(),
            analog_touched_nodes: None,
            num_branches: 0,
            hidden_state_count: 0,
            resistors: Resistors::new(),
            parameter_direction: None,
            resistor_noise_temperature_provenance: None,
            resistor_branches: ResistorBranches::new(),
            capacitors: Capacitors::new(),
            inductors: Inductors::new(),
            voltage_sources: VoltageSources::new(),
            current_sources: CurrentSources::new(),
            xyce_topology_order: Vec::new(),
            xyce_load_plan: xyce_load::XyceLoadPlan::default(),
            xyce_linear_f_operator: xyce_load::XyceLinearDaeOperator::default(),
            diodes: Diodes::new(),
            bjts: Bjts::new(),
            mosfets: Mosfets::new(),
            b3soi: B3SoiDds::new(),
            b3soi_fd: B3SoiFds::new(),
            b3soi_pd: B3SoiPds::new(),
            b3soi_gmin_scale: 1.0e-6,
            no_dc_path_nodes: Vec::new(),
            fatal_no_dc_path_nodes: Vec::new(),
            global_shunt_conductance: 0.0,
            terminal_probe_nodes: HashMap::new(),
            terminal_probe_source_names: HashSet::new(),
            veriloga_terminal_meters: Vec::new(),
            mos_terminal_layouts: HashMap::new(),
            bjt_terminal_layouts: HashMap::new(),
            device_model_safety: HashMap::new(),
            dc_floating_component_by_node: Vec::new(),
            dc_floating_component_nodes: Vec::new(),
            dc_floating_component_is_certain: Vec::new(),
            bsim3v3: Bsim3v3s::new(),
            bsim4v8: Bsim4v8s::new(),
            ekv26s: EkvMosfets::new(),
            ekv3s: Ekv3Mosfets::new(),
            vdmoses: Vdmoses::new(),
            jfets: Vec::new(),
            xyce_memristors: Vec::new(),
            non_electrical_state_nodes: HashSet::new(),
            xyce_memristor_operating_point_mode: false,
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
            xyce_core_groups: Vec::new(),
            xyce_core_trial_invalid: false,
            xyce_core_transient_residuals: Vec::new(),
            xyce_core_transient_roundoff: Vec::new(),
            transient_max_step_hint: None,
            behavioral_sources: BehavioralSources::new(),
            scheduler: super::scheduler::CircuitScheduler::new(),
            // XSPICE instances
            xspice_instances: Vec::new(),
            xspice_has_event_driven_devices: false,
            xspice_event_loads: Default::default(),
            xspice_touched_digital_nodes: Vec::new(),
            xspice_touched_real_nodes: Vec::new(),
            xspice_dispatch_pending: Vec::new(),
            xspice_dispatch_next_pending: Vec::new(),
            xspice_output_iterates: XspiceOutputIterates::default(),
            xspice_registry: Arc::new(CodeModelRegistry::with_builtins()),
            xspice_evaluation_error: None,
            xspice_evaluation_warning: None,
            xspice_resource_failure: None,
            transient_device_noise: None,
            // Verilog-A devices
            #[cfg(feature = "veriloga")]
            veriloga_devices: crate::device::veriloga::VerilogADevices::new(),
            #[cfg(feature = "veriloga")]
            mixed_signal_hosts: Vec::new(),
            #[cfg(feature = "veriloga")]
            ground_census: Default::default(),
            #[cfg(feature = "veriloga-builtins-base")]
            generated_veriloga_devices:
                crate::device::veriloga_builtins::BuiltinVerilogADevices::new(),
            #[cfg(any(feature = "veriloga", feature = "veriloga-builtins-base"))]
            generated_simulation_parameters:
                rspice_veriloga_runtime::GeneratedSimulationParameters::default(),
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

    /// Apply an ambient-relative resistor-noise temperature to the newest
    /// resistor while clearing any absolute `TEMP` provenance for that slot.
    pub(crate) fn set_last_resistor_noise_temperature_offset(&mut self, offset_kelvin: Value) {
        self.resistors
            .set_last_noise_temperature_offset(offset_kelvin);
        let Some(index) = self.resistors.len().checked_sub(1) else {
            return;
        };
        let clear_provenance = self
            .resistor_noise_temperature_provenance
            .as_mut()
            .is_some_and(|provenance| {
                provenance.clear(index);
                provenance.is_empty()
            });
        if clear_provenance {
            self.resistor_noise_temperature_provenance = None;
        }
    }

    /// Retain authored resistor `TEMP` exactly rather than reconstructing it
    /// from an ambient-relative difference that can lose all low bits.
    pub(crate) fn set_last_resistor_absolute_noise_temperature(&mut self, temperature: Value) {
        self.resistors.set_last_noise_temperature_offset(0.0);
        let Some(index) = self.resistors.len().checked_sub(1) else {
            return;
        };
        self.resistor_noise_temperature_provenance
            .get_or_insert_default()
            .set_absolute(index, temperature);
    }

    /// Authored absolute resistor-noise temperature, when the instance used
    /// `TEMP` rather than the analysis temperature plus `DTEMP`.
    #[inline]
    pub(crate) fn resistor_absolute_noise_temperature(&self, index: usize) -> Option<Value> {
        self.resistor_noise_temperature_provenance
            .as_ref()
            .and_then(|provenance| provenance.get(index))
    }

    /// Number of explicitly materialized absolute resistor-noise temperature
    /// slots. Trailing omissions are the canonical compact representation of
    /// authored ambient temperature; a count beyond the resistor array is an
    /// invalid metadata state.
    #[inline]
    pub(crate) fn resistor_absolute_noise_temperature_count(&self) -> usize {
        self.resistor_noise_temperature_provenance
            .as_ref()
            .map_or(0, ResistorNoiseTemperatureProvenance::materialized_len)
    }

    /// Resolve a resistor's physical noise temperature without a lossy
    /// subtract-then-add round trip through an extreme analysis temperature.
    #[inline]
    pub(crate) fn resistor_noise_temperature(&self, index: usize, ambient: Value) -> Value {
        self.resistor_absolute_noise_temperature(index)
            .unwrap_or_else(|| ambient + self.resistors.noise_temperature_offset(index))
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
        crate::naming::is_spice_ground_name(name)
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

    /// Retain the parser-certified partition of nodes that lack a
    /// DC-conducting path to ground after final node remapping.
    pub(crate) fn set_dc_floating_components(
        &mut self,
        components: Vec<Vec<String>>,
        component_is_certain: Vec<bool>,
    ) {
        debug_assert_eq!(components.len(), component_is_certain.len());
        let mut component_by_node = vec![None; self.num_nodes.saturating_add(1)];
        for (component_index, nodes) in components.iter().enumerate() {
            for node in nodes {
                if let Some(node_id) = self.get_node_by_name(node)
                    && node_id > 0
                    && let Some(slot) = component_by_node.get_mut(node_id)
                {
                    *slot = Some(component_index);
                }
            }
        }
        self.dc_floating_component_by_node = component_by_node;
        self.dc_floating_component_nodes = components;
        self.dc_floating_component_is_certain = component_is_certain;
    }

    /// Resolve behavioral expression references against current node/branch maps.
    pub fn bind_behavioral_references(
        &mut self,
        known_device_names: &HashSet<String>,
    ) -> Result<(), CircuitError> {
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
                    if let Some(ordinal) = branch_lookup
                        .get(name)
                        .or_else(|| branch_lookup.get(&name.to_uppercase()))
                        .copied()
                    {
                        crate::device::BehavioralBranchResolution::Branch(num_nodes + ordinal - 1)
                    } else if known_device_names.contains(&name.to_ascii_uppercase()) {
                        crate::device::BehavioralBranchResolution::DeviceWithoutBranch
                    } else {
                        crate::device::BehavioralBranchResolution::MissingDevice
                    }
                },
            )
            .map_err(CircuitError::from)?;
        self.capacitors
            .bind_value_expression_references(
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
            .map_err(CircuitError::InvalidComponent)?;

        let node_lookup = self.node_map.clone();
        let branch_lookup = self.branch_names.clone();
        let num_nodes = self.num_nodes;
        for switch in &mut self.generic_switches {
            switch
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
                .map_err(CircuitError::InvalidComponent)?;
        }
        Ok(())
    }

    /// Check whether the circuit explicitly referenced the SPICE ground node.
    pub fn has_explicit_ground_reference(&self) -> bool {
        self.has_explicit_ground_reference
    }

    /// Make `name` the reference, node 0, of a deck that names no ground.
    ///
    /// The build calls this before any node exists, so every device, native or
    /// generated, and every internal node is built against the final
    /// numbering, and nothing is renumbered afterwards. `name` keeps resolving
    /// to node 0 through [`Self::get_node_by_name`].
    pub(crate) fn reference_ground_at(&mut self, name: &str) {
        debug_assert_eq!(
            self.num_nodes, 0,
            "the reference is chosen before any node exists"
        );
        self.node_map.insert(name.to_string(), 0);
        self.has_explicit_ground_reference = true;
        log::info!("Auto-selected node '{}' as ground reference", name);
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
                + self
                    .capacitors
                    .ic_branch_indices
                    .iter()
                    .filter(|branch| branch.is_some())
                    .count()
                + self.voltage_sources.names.len()
                + self.ccvs.len()
                + self.behavioral_sources.voltage_sources.len(),
        );
        names.extend(self.inductors.names.iter().cloned());
        names.extend(self.resistor_branches.names.iter().cloned());
        names.extend(
            self.capacitors
                .names
                .iter()
                .zip(&self.capacitors.ic_branch_indices)
                .filter(|(_, branch)| branch.is_some())
                .map(|(name, _)| name.clone()),
        );
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
            hidden_m_slot: None,
            hidden_r_slot: None,
            device,
            core_output_name: None,
            core_bh_si_units: false,
        });
    }

    /// Register a nonlinear Xyce Core binding with its canonical internal
    /// output namespace.
    pub fn add_xyce_core_inductor(
        &mut self,
        inductor_index: usize,
        branch_ordinal: NodeId,
        device: crate::device::passive::JilesAthertonInductor,
        core_output_name: String,
        core_bh_si_units: bool,
    ) {
        let hidden_m_slot = device
            .has_xyce_core_m_equation()
            .then(|| self.allocate_hidden_state());
        let hidden_r_slot = (!device.is_xyce_core_level2()).then(|| self.allocate_hidden_state());
        self.jiles_atherton_inductors.push(JilesAthertonBinding {
            inductor_index,
            branch_ordinal,
            hidden_m_slot,
            hidden_r_slot,
            device,
            core_output_name: Some(core_output_name),
            core_bh_si_units,
        });
    }

    /// Register a shared multi-winding Xyce Core DAE after all of its
    /// component inductor branches have been allocated.
    pub fn add_xyce_core_group(
        &mut self,
        device: crate::device::passive::JilesAthertonInductor,
        core_output_name: String,
        windings: Vec<XyceCoreWindingBinding>,
        coupling: Value,
        core_bh_si_units: bool,
    ) {
        let hidden_m_slot = device
            .has_xyce_core_m_equation()
            .then(|| self.allocate_hidden_state());
        let hidden_r_slot = (!device.is_xyce_core_level2()).then(|| self.allocate_hidden_state());
        let winding_count = windings.len();
        self.xyce_core_groups.push(XyceCoreGroupBinding {
            core_output_name,
            device,
            hidden_m_slot,
            hidden_r_slot,
            windings,
            xyce_q_history: vec![0.0; winding_count],
            coupling,
            core_bh_si_units,
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

    /// Resolve all pending CCCS/CCVS/ISWITCH control element references.
    /// Call this after all elements have been added to the circuit
    /// Returns an error if any control element is not found
    pub fn resolve_control_elements(&mut self) -> Result<(), CircuitError> {
        let capacitor_ic_names = self
            .capacitors
            .names
            .iter()
            .zip(&self.capacitors.ic_branch_indices)
            .filter(|(_, branch)| branch.is_some())
            .map(|(name, _)| name.to_ascii_uppercase())
            .collect::<HashSet<_>>();
        let capacitor_ic_control_is_unsupported =
            |name: &str| capacitor_ic_names.contains(&name.to_ascii_uppercase());
        // Resolve CCCS control branches
        for (cccs_idx, control_name) in self.pending_cccs.drain(..).collect::<Vec<_>>() {
            if capacitor_ic_control_is_unsupported(&control_name) {
                return Err(CircuitError::InvalidComponent(format!(
                    "CCCS control '{}' is a capacitor lead current; Xyce F/H and current-switch controls require a voltage-source branch",
                    control_name
                )));
            }
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
            if capacitor_ic_control_is_unsupported(&control_name) {
                return Err(CircuitError::InvalidComponent(format!(
                    "CCVS control '{}' is a capacitor lead current; Xyce F/H and current-switch controls require a voltage-source branch",
                    control_name
                )));
            }
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
            if capacitor_ic_control_is_unsupported(&control_name) {
                return Err(CircuitError::InvalidComponent(format!(
                    "ISWITCH control '{}' is a capacitor lead current; Xyce F/H and current-switch controls require a voltage-source branch",
                    control_name
                )));
            }
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
        let capacitor_ic_names = self
            .capacitors
            .names
            .iter()
            .zip(&self.capacitors.ic_branch_indices)
            .filter(|(_, branch)| branch.is_some())
            .map(|(name, _)| name.to_ascii_uppercase())
            .collect::<HashSet<_>>();
        let current_sources = self.current_sources.clone();
        for instance in &mut self.xspice_instances {
            instance
                .make_mut()
                .resolve_branch_references(
                    |name| {
                        if capacitor_ic_names.contains(&name.to_ascii_uppercase()) {
                            return None;
                        }
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

    /// Convert a private hidden-state slot to its 1-based matrix index.
    /// Public node and branch indices remain contiguous before this suffix.
    #[inline]
    pub fn get_hidden_state_matrix_index(&self, slot: usize) -> usize {
        self.num_nodes + self.num_branches + slot + 1
    }

    /// Total matrix size
    pub fn matrix_size(&self) -> usize {
        self.num_nodes + self.num_branches + self.hidden_state_count
    }

    /// Number of nodes (excluding ground)
    pub fn num_nodes(&self) -> usize {
        self.num_nodes
    }

    /// Number of branches
    pub fn num_branches(&self) -> usize {
        self.num_branches
    }

    /// Number of private scalar state variables appended to the matrix.
    #[inline]
    pub fn hidden_state_count(&self) -> usize {
        self.hidden_state_count
    }

    #[inline]
    fn allocate_hidden_state(&mut self) -> usize {
        let slot = self.hidden_state_count;
        self.hidden_state_count += 1;
        slot
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resistor_absolute_noise_temperature_survives_extreme_ambient() {
        let mut circuit = CircuitData::new();
        circuit.resistors.add("R1".to_string(), 1, 0, 1.0);
        circuit.set_last_resistor_absolute_noise_temperature(300.15);
        assert_eq!(
            circuit.resistor_noise_temperature(0, 1.0e20).to_bits(),
            300.15_f64.to_bits()
        );
        assert_eq!(circuit.resistors.noise_temperature_offset(0), 0.0);
    }

    #[test]
    fn resistor_noise_temperature_provenance_is_lazy_compact_and_value_exact() {
        let mut circuit = CircuitData::new();
        assert!(circuit.resistor_noise_temperature_provenance.is_none());

        circuit.resistors.add("R1".to_string(), 1, 0, 1.0);
        circuit.set_last_resistor_absolute_noise_temperature(Value::NAN);
        assert_eq!(
            circuit
                .resistor_absolute_noise_temperature(0)
                .expect("authored non-finite value remains available for fail-closed analysis")
                .to_bits(),
            Value::NAN.to_bits()
        );

        circuit.set_last_resistor_noise_temperature_offset(25.0);
        assert!(circuit.resistor_noise_temperature_provenance.is_none());
        assert_eq!(circuit.resistor_absolute_noise_temperature_count(), 0);

        circuit.set_last_resistor_absolute_noise_temperature(300.15);
        circuit.resistors.add("R2".to_string(), 2, 0, 2.0);
        circuit.set_last_resistor_absolute_noise_temperature(325.15);
        assert_eq!(circuit.resistor_absolute_noise_temperature_count(), 2);

        circuit.set_last_resistor_noise_temperature_offset(50.0);
        assert_eq!(
            circuit.resistor_absolute_noise_temperature_count(),
            1,
            "clearing the last absolute temperature removes trailing omissions"
        );
        assert_eq!(
            circuit.resistor_noise_temperature(0, 1.0e20).to_bits(),
            300.15_f64.to_bits()
        );
        assert_eq!(
            circuit.resistor_noise_temperature(1, 275.15).to_bits(),
            (275.15_f64 + 50.0).to_bits()
        );
    }

    #[test]
    fn current_switch_rejects_capacitor_lead_current_control() {
        let mut circuit = CircuitData::new();
        let node = circuit.get_or_create_node("n");
        let branch = circuit.allocate_branch_named("C1");
        circuit
            .capacitors
            .add_with_ic_branch("C1".to_string(), node, 0, 1.0, 1.0, branch);
        circuit.add_iswitch_pending(0, "C1".to_string());

        let error = circuit
            .resolve_control_elements()
            .expect_err("ISWITCH capacitor-current control must be rejected");
        assert!(
            error
                .to_string()
                .contains("require a voltage-source branch")
        );
    }

    /// A gate whose output delay grows with the total load on its output net:
    /// 1 ns plus 1 ns per picofarad of `TOTAL_LOAD(out)`. It drives a constant
    /// one, so every instance posts exactly one event at its delay.
    struct LoadDelayedGate;

    impl crate::xspice::CodeModel for LoadDelayedGate {
        fn name(&self) -> &str {
            "load_delayed_gate"
        }

        fn ports(&self) -> &[crate::xspice::PortSpec] {
            use crate::xspice::{PortSpec, PortType};
            static PORTS: std::sync::OnceLock<Vec<PortSpec>> = std::sync::OnceLock::new();
            PORTS.get_or_init(|| {
                vec![
                    PortSpec::input("in", PortType::Digital),
                    PortSpec::output("out", PortType::Digital),
                ]
            })
        }

        fn parameters(&self) -> &[crate::xspice::ParamSpec] {
            &[]
        }

        fn init(&self, _ctx: &mut crate::xspice::CmContext) -> crate::xspice::CmResult<()> {
            Ok(())
        }

        fn evaluate(&self, ctx: &mut crate::xspice::CmContext) -> crate::xspice::CmResult<()> {
            let delay = 1.0e-9 + 1.0e3 * ctx.port_total_load("out");
            ctx.set_output_digital("out", crate::xspice::DigitalValue::one(), delay);
            Ok(())
        }
    }

    /// Three buffers on nets `a`, `b`, `c`, built from a deck whose reference
    /// is `reference`, then evaluated with [`LoadDelayedGate`] on the
    /// connections and the load sums the build gave them.
    ///
    /// The loads are `a` 1 pF (`A1`'s input) and `b` 5 pF (`A2` and `A3`), so
    /// `A1` drives `b` at 6 ns, `A2` drives `c` at 1 ns and `A3` drives `a` at
    /// 2 ns. The reference is the deck's second net, so every event net is
    /// numbered after it.
    fn load_delayed_gates(reference: &str) -> Vec<(String, Value)> {
        use crate::xspice::{SharedXspiceInstance, XspiceInstance};
        let deck = format!(
            "* three buffers whose delay reads TOTAL_LOAD(out)\n\
             v1 vdd {reference} 1\n\
             r1 vdd {reference} 1k\n\
             a1 a b g1\n\
             .model g1 d_buffer (input_load=1p)\n\
             a2 b c g2\n\
             .model g2 d_buffer (input_load=2p)\n\
             a3 b a g3\n\
             .model g3 d_buffer (input_load=3p)\n\
             .end\n"
        );
        let netlist = crate::Netlist::parse(&deck).expect("deck parses");
        let mut circuit = crate::engine::Engine::default()
            .build_circuit(&netlist)
            .expect("circuit builds");
        assert_eq!(circuit.get_node_by_name(reference), Some(0));
        for shared in &mut circuit.xspice_instances {
            let built = shared.make_mut();
            let swapped = XspiceInstance::new(
                built.name.clone(),
                std::sync::Arc::new(LoadDelayedGate),
                built.connections().to_vec(),
                &[],
                &[],
                &[],
                &[],
            )
            .expect("gate constructs");
            *shared = SharedXspiceInstance::new(swapped);
        }
        circuit.invalidate_xspice_event_dispatch();
        circuit.evaluate_xspice_with_analysis(
            0.0,
            1.0e-9,
            &vec![0.0; circuit.matrix_size()],
            crate::xspice::AnalysisType::Transient,
        );
        assert_eq!(circuit.take_xspice_evaluation_error(), None);
        let mut events = Vec::new();
        circuit
            .scheduler
            .xspice_event_queue
            .make_mut()
            .run_due_events(1.0, |event| {
                events.push((event.instance.to_ascii_uppercase(), event.time))
            })
            .expect("the queue drains");
        events.sort_by(|left, right| left.0.cmp(&right.0));
        events
    }

    /// A deck without node 0 takes its reference before any event net is
    /// numbered, so every code model reads its own output net's total load:
    /// the same event times as the deck that names the reference 0.
    #[test]
    fn a_deck_without_node_zero_reads_each_event_nets_own_total_load() {
        let grounded = load_delayed_gates("0");
        let expected = [("A1", 6.0e-9), ("A2", 1.0e-9), ("A3", 2.0e-9)];
        assert_eq!(grounded.len(), expected.len(), "{grounded:?}");
        for ((instance, time), (name, at)) in grounded.iter().zip(expected) {
            assert_eq!(instance, name);
            assert!(
                (time - at).abs() <= 1.0e-21,
                "{instance} with node 0: {time:e} s, expected {at:e} s"
            );
        }
        assert_eq!(load_delayed_gates("reference"), grounded);
    }
}
