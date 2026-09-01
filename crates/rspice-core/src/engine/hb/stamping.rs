use super::*;

#[derive(Debug, Clone, Copy)]
enum PeriodicMnaRegistration {
    VoltageSource(usize),
    Inductor(usize),
    TransformerWinding(usize, usize),
    Resistor(usize),
    Vcvs(usize),
    Ccvs(usize),
}

impl Engine {
    pub(in crate::engine::hb) fn hb_stamp_supported_nonlinear_devices(
        &self,
        circuit: &CircuitData,
        solver: &mut HbSolver,
        num_nodes: usize,
    ) {
        use crate::analysis::harmonic_balance::{DepletionCap, NonlinearDeviceInstance};

        for diode in &circuit.diodes.devices {
            let anode = Self::hb_node_to_solver_index(diode.node_anode, num_nodes);
            let cathode = Self::hb_node_to_solver_index(diode.node_cathode, num_nodes);
            solver.add_named_resolved_diode(
                diode.name.clone(),
                NonlinearDeviceInstance::diode(anode, cathode, diode.is, diode.n)
                    .with_thermal_voltage(diode.vt)
                    .with_junction_caps(
                        DepletionCap::new(diode.cj0, diode.vj, diode.m, diode.fc),
                        DepletionCap::none(),
                        diode.tt,
                    ),
                diode.resolved_level_one_junction(),
            );
        }

        for mos in &circuit.mosfets.devices {
            let drain = Self::hb_node_to_solver_index(mos.node_drain, num_nodes);
            let gate = Self::hb_node_to_solver_index(mos.node_gate, num_nodes);
            let source = Self::hb_node_to_solver_index(mos.node_source, num_nodes);
            let bulk = Self::hb_node_to_solver_index(mos.node_bulk, num_nodes);
            let leff = mos.l - 2.0 * mos.ld;
            let beta = mos.kp * mos.w / leff;
            let instance = match mos.mos_type {
                crate::device::MosType::Nmos => NonlinearDeviceInstance::nmos(
                    drain, gate, source, bulk, mos.vto, beta, mos.lambda,
                ),
                // The solver works in the polarity frame: the effective
                // threshold is -VTO, which keeps depletion PMOS negative.
                crate::device::MosType::Pmos => NonlinearDeviceInstance::pmos(
                    drain, gate, source, bulk, -mos.vto, beta, mos.lambda,
                ),
            };
            // Effective bulk-junction zero-bias capacitances: explicit
            // CBD/CBS overrides, else bottom density times area, plus the
            // sidewall density times perimeter folded at the bottom grading.
            let cbs0 = mos
                .source_bulk_cap_zero_bias
                .unwrap_or(mos.cj * mos.source_area)
                .max(0.0)
                + (mos.cjsw * mos.source_perimeter).max(0.0);
            let cbd0 = mos
                .drain_bulk_cap_zero_bias
                .unwrap_or(mos.cj * mos.drain_area)
                .max(0.0)
                + (mos.cjsw * mos.drain_perimeter).max(0.0);
            let is_s = if mos.js_bulk > 0.0 && mos.source_area > 0.0 {
                mos.js_bulk * mos.source_area
            } else {
                mos.is_bulk
            };
            let is_d = if mos.js_bulk > 0.0 && mos.drain_area > 0.0 {
                mos.js_bulk * mos.drain_area
            } else {
                mos.is_bulk
            };
            // Intrinsic channel charge: total oxide capacitance over the
            // effective (lateral-diffusion-shortened) channel.
            let instance = instance
                .with_thermal_voltage(mos.vt)
                .with_body_effect(mos.gamma, mos.phi)
                .with_channel_noise_gamma(mos.channel_thermal_noise_gamma())
                .with_intrinsic_gate(mos.cox * mos.w * leff)
                .with_bulk_junctions(
                    DepletionCap::new(cbs0, mos.pb, mos.mj, mos.fc),
                    DepletionCap::new(cbd0, mos.pb, mos.mj, mos.fc),
                    is_s,
                    is_d,
                );
            if let Some(temp_k) = mos.noise_absolute_temperature {
                solver.add_named_nonlinear_device_with_absolute_noise_temperature(
                    mos.name.clone(),
                    instance,
                    temp_k,
                );
            } else {
                solver.add_named_nonlinear_device_with_noise_temperature_offset(
                    mos.name.clone(),
                    instance,
                    mos.noise_temperature_offset,
                );
            }

            // Gate overlap capacitances are bias-independent in level 1:
            // stamp them as ordinary linear capacitors.
            let cgs_ov = mos.cgso * mos.w;
            let cgd_ov = mos.cgdo * mos.w;
            let cgb_ov = mos.cgbo * leff;
            if cgs_ov > 0.0 {
                self.hb_stamp_admittance(solver, mos.node_gate, mos.node_source, cgs_ov, false);
            }
            if cgd_ov > 0.0 {
                self.hb_stamp_admittance(solver, mos.node_gate, mos.node_drain, cgd_ov, false);
            }
            if cgb_ov > 0.0 {
                self.hb_stamp_admittance(solver, mos.node_gate, mos.node_bulk, cgb_ov, false);
            }
        }

        for jfet in &circuit.jfets {
            let drain = Self::hb_node_to_solver_index(jfet.drain, num_nodes);
            let gate = Self::hb_node_to_solver_index(jfet.gate, num_nodes);
            let source = Self::hb_node_to_solver_index(jfet.source, num_nodes);
            let beta = jfet.params.beta;
            let instance = match jfet.jfet_type {
                crate::device::JfetType::NJF => NonlinearDeviceInstance::njfet(
                    drain,
                    gate,
                    source,
                    jfet.params.vto,
                    beta,
                    jfet.params.lambda,
                    jfet.params.is,
                ),
                crate::device::JfetType::PJF => NonlinearDeviceInstance::pjfet(
                    drain,
                    gate,
                    source,
                    jfet.params.vto,
                    beta,
                    jfet.params.lambda,
                    jfet.params.is,
                ),
            };
            let vt_jfet = crate::constants::K_BOLTZMANN * jfet.resolved_instance_temperature()
                / crate::constants::Q_ELECTRON;
            let instance = instance.with_thermal_voltage(vt_jfet).with_junction_caps(
                DepletionCap::new(jfet.params.cgs, jfet.params.pb, jfet.params.m, 0.5),
                DepletionCap::new(jfet.params.cgd, jfet.params.pb, jfet.params.m, 0.5),
                0.0,
            );
            if let Some(temp_k) = jfet.noise_absolute_temperature {
                solver.add_named_nonlinear_device_with_absolute_noise_temperature(
                    jfet.name.clone(),
                    instance,
                    temp_k,
                );
            } else {
                solver.add_named_nonlinear_device_with_noise_temperature_offset(
                    jfet.name.clone(),
                    instance,
                    jfet.noise_dtemp,
                );
            }
        }

        for sw in &circuit.vswitches {
            let node_pos = Self::hb_node_to_solver_index(sw.node_pos, num_nodes);
            let node_neg = Self::hb_node_to_solver_index(sw.node_neg, num_nodes);
            let ctrl_pos = Self::hb_node_to_solver_index(sw.ctrl_pos, num_nodes);
            let ctrl_neg = Self::hb_node_to_solver_index(sw.ctrl_neg, num_nodes);
            solver.add_named_nonlinear_device(
                sw.name.clone(),
                NonlinearDeviceInstance::voltage_switch(
                    node_pos, node_neg, ctrl_pos, ctrl_neg, sw.vt, sw.vh, sw.ron, sw.roff,
                    sw.smooth,
                ),
            );
        }

        #[cfg(feature = "veriloga")]
        for device in circuit.veriloga_devices().iter() {
            solver.add_veriloga_device(device.clone());
        }
    }

    /// Stamp resistors into HB solver G matrix
    pub(in crate::engine::hb) fn hb_stamp_resistors(
        &self,
        circuit: &CircuitData,
        solver: &mut HbSolver,
    ) {
        for i in 0..circuit.resistors.len() {
            let np = circuit.resistors.stamps[i].pp.row;
            let nn = circuit.resistors.stamps[i].nn.row;
            let g = circuit.resistors.conductances[i];
            let small_signal_g = circuit.resistors.small_signal_conductance(i);

            self.hb_stamp_conductance_pair(solver, np, nn, g, small_signal_g);
        }
        if circuit.global_shunt_conductance != 0.0 {
            for node in 1..=circuit.num_nodes() {
                if !circuit.is_non_electrical_state_matrix_index(node - 1) {
                    self.hb_stamp_admittance(
                        solver,
                        node,
                        0,
                        circuit.global_shunt_conductance,
                        true,
                    );
                }
            }
        }
    }

    fn hb_stamp_conductance_pair(
        &self,
        solver: &mut HbSolver,
        np: usize,
        nn: usize,
        conductance: Value,
        small_signal_conductance: Value,
    ) {
        let mut stamp = |row: usize, column: usize, sign: Value| {
            solver.add_conductance_with_small_signal(
                row,
                column,
                sign * conductance,
                sign * small_signal_conductance,
            );
        };
        if np > 0 && nn > 0 {
            let i = np - 1;
            let j = nn - 1;
            stamp(i, i, 1.0);
            stamp(i, j, -1.0);
            stamp(j, i, -1.0);
            stamp(j, j, 1.0);
        } else if np > 0 {
            stamp(np - 1, np - 1, 1.0);
        } else if nn > 0 {
            stamp(nn - 1, nn - 1, 1.0);
        }
    }

    /// Stamp capacitors into HB solver C matrix
    pub(in crate::engine::hb) fn hb_stamp_capacitors(
        &self,
        circuit: &CircuitData,
        solver: &mut HbSolver,
    ) {
        for i in 0..circuit.capacitors.len() {
            let np = circuit.capacitors.stamps[i].pp.row;
            let nn = circuit.capacitors.stamps[i].nn.row;
            let c = circuit.capacitors.capacitances[i];

            // Stamp capacitance matrix
            self.hb_stamp_admittance(solver, np, nn, c, false);
        }
    }

    /// Register every supported exact HB MNA branch in the circuit's
    /// canonical one-based branch order.
    ///
    /// Linear HB, PAC, and PNoise support independent and controlled
    /// voltage-source, coupled/uncoupled-inductor, transformer-winding, and
    /// branch-form resistor equations. If authored
    /// voltage-source spectra were registered first, the exact descriptors
    /// retain them for the large-signal solve; otherwise they describe
    /// zero-valued small-signal constraints. The caller rejects other branch
    /// families before this boundary, and this routine independently proves a
    /// complete unique map.
    pub(in crate::engine::hb) fn hb_stamp_periodic_mna_branches(
        &self,
        circuit: &CircuitData,
        solver: &mut HbSolver,
    ) -> Result<(), SimulationError> {
        let branch_count = circuit.num_branches();
        let mut registrations = Vec::new();
        registrations
            .try_reserve_exact(branch_count)
            .map_err(|error| {
                SimulationError::Circuit(format!(
                    "periodic MNA canonical branch-map allocation failed: {error}"
                ))
            })?;
        registrations.resize(branch_count, None);

        for source_index in 0..circuit.voltage_sources.len() {
            let name = circuit
                .voltage_sources
                .names
                .get(source_index)
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "periodic MNA voltage-source storage is missing name row {source_index}"
                    ))
                })?;
            circuit
                .voltage_sources
                .node_pos
                .get(source_index)
                .zip(circuit.voltage_sources.node_neg.get(source_index))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "periodic MNA voltage source '{name}' has incomplete terminal storage"
                    ))
                })?;
            let branch_ordinal = *circuit
                .voltage_sources
                .branch_indices
                .get(source_index)
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "periodic MNA voltage source '{name}' is missing its canonical branch ordinal"
                    ))
                })?;
            let slot_index = branch_ordinal.checked_sub(1).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "periodic MNA voltage source '{name}' has invalid branch ordinal 0"
                ))
            })?;
            let slot = registrations.get_mut(slot_index).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "periodic MNA voltage source '{name}' has branch ordinal {branch_ordinal}, outside 1..={branch_count}"
                ))
            })?;
            if slot
                .replace(PeriodicMnaRegistration::VoltageSource(source_index))
                .is_some()
            {
                return Err(SimulationError::Circuit(format!(
                    "periodic MNA branch ordinal {branch_ordinal} is assigned more than once"
                )));
            }
        }

        for inductor_index in 0..circuit.inductors.len() {
            let name = circuit.inductors.names.get(inductor_index).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "periodic MNA inductor storage is missing name row {inductor_index}"
                ))
            })?;
            circuit
                .inductors
                .node_pos
                .get(inductor_index)
                .zip(circuit.inductors.node_neg.get(inductor_index))
                .zip(circuit.inductors.inductances.get(inductor_index))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "periodic MNA inductor '{name}' has incomplete terminal/value storage"
                    ))
                })?;
            let branch_ordinal = *circuit
                .inductors
                .branch_indices
                .get(inductor_index)
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "periodic MNA inductor '{name}' is missing its canonical branch ordinal"
                    ))
                })?;
            let slot_index = branch_ordinal.checked_sub(1).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "periodic MNA inductor '{name}' has invalid branch ordinal 0"
                ))
            })?;
            let slot = registrations.get_mut(slot_index).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "periodic MNA inductor '{name}' has branch ordinal {branch_ordinal}, outside 1..={branch_count}"
                ))
            })?;
            if slot
                .replace(PeriodicMnaRegistration::Inductor(inductor_index))
                .is_some()
            {
                return Err(SimulationError::Circuit(format!(
                    "periodic MNA branch ordinal {branch_ordinal} is assigned more than once"
                )));
            }
        }

        for resistor_index in 0..circuit.resistor_branches.len() {
            let name = circuit
                .resistor_branches
                .names
                .get(resistor_index)
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "periodic MNA branch-form resistor storage is missing name row {resistor_index}"
                    ))
                })?;
            circuit
                .resistor_branches
                .node_pos
                .get(resistor_index)
                .zip(circuit.resistor_branches.node_neg.get(resistor_index))
                .zip(circuit.resistor_branches.resistances.get(resistor_index))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "periodic MNA branch-form resistor '{name}' has incomplete terminal/value storage"
                    ))
                })?;
            let branch_ordinal = *circuit
                .resistor_branches
                .branch_indices
                .get(resistor_index)
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "periodic MNA branch-form resistor '{name}' is missing its canonical branch ordinal"
                    ))
                })?;
            let slot_index = branch_ordinal.checked_sub(1).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "periodic MNA branch-form resistor '{name}' has invalid branch ordinal 0"
                ))
            })?;
            let slot = registrations.get_mut(slot_index).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "periodic MNA branch-form resistor '{name}' has branch ordinal {branch_ordinal}, outside 1..={branch_count}"
                ))
            })?;
            if slot
                .replace(PeriodicMnaRegistration::Resistor(resistor_index))
                .is_some()
            {
                return Err(SimulationError::Circuit(format!(
                    "periodic MNA branch ordinal {branch_ordinal} is assigned more than once"
                )));
            }
        }

        for vcvs_index in 0..circuit.vcvs.len() {
            let name = circuit.vcvs.names.get(vcvs_index).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "periodic MNA VCVS storage is missing name row {vcvs_index}"
                ))
            })?;
            circuit
                .vcvs
                .node_pos
                .get(vcvs_index)
                .zip(circuit.vcvs.node_neg.get(vcvs_index))
                .zip(circuit.vcvs.ctrl_pos.get(vcvs_index))
                .zip(circuit.vcvs.ctrl_neg.get(vcvs_index))
                .zip(circuit.vcvs.gains.get(vcvs_index))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "periodic MNA VCVS '{name}' has incomplete terminal/control/value storage"
                    ))
                })?;
            let branch_ordinal = *circuit.vcvs.branch_indices.get(vcvs_index).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "periodic MNA VCVS '{name}' is missing its canonical branch ordinal"
                ))
            })?;
            let slot_index = branch_ordinal.checked_sub(1).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "periodic MNA VCVS '{name}' has invalid branch ordinal 0"
                ))
            })?;
            let slot = registrations.get_mut(slot_index).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "periodic MNA VCVS '{name}' has branch ordinal {branch_ordinal}, outside 1..={branch_count}"
                ))
            })?;
            if slot
                .replace(PeriodicMnaRegistration::Vcvs(vcvs_index))
                .is_some()
            {
                return Err(SimulationError::Circuit(format!(
                    "periodic MNA branch ordinal {branch_ordinal} is assigned more than once"
                )));
            }
        }

        for ccvs_index in 0..circuit.ccvs.len() {
            let name = circuit.ccvs.names.get(ccvs_index).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "periodic MNA CCVS storage is missing name row {ccvs_index}"
                ))
            })?;
            circuit
                .ccvs
                .node_pos
                .get(ccvs_index)
                .zip(circuit.ccvs.node_neg.get(ccvs_index))
                .zip(circuit.ccvs.ctrl_branch.get(ccvs_index))
                .zip(circuit.ccvs.transresistances.get(ccvs_index))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "periodic MNA CCVS '{name}' has incomplete terminal/control/value storage"
                    ))
                })?;
            let branch_ordinal = *circuit.ccvs.branch_indices.get(ccvs_index).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "periodic MNA CCVS '{name}' is missing its canonical branch ordinal"
                ))
            })?;
            let slot_index = branch_ordinal.checked_sub(1).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "periodic MNA CCVS '{name}' has invalid branch ordinal 0"
                ))
            })?;
            let slot = registrations.get_mut(slot_index).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "periodic MNA CCVS '{name}' has branch ordinal {branch_ordinal}, outside 1..={branch_count}"
                ))
            })?;
            if slot
                .replace(PeriodicMnaRegistration::Ccvs(ccvs_index))
                .is_some()
            {
                return Err(SimulationError::Circuit(format!(
                    "periodic MNA branch ordinal {branch_ordinal} is assigned more than once"
                )));
            }
        }

        // A multi-winding binding may either own otherwise-unassigned
        // canonical winding branches or overlay standalone inductor branches.
        // In both cases each self term has exactly one owner; only mutual
        // off-diagonals are added after the complete registry is established.
        for (binding_index, binding) in circuit.multi_winding_transformers.iter().enumerate() {
            let device = &binding.device;
            let winding_count = device.num_windings;
            if winding_count == 0
                || device.nodes.len() != winding_count
                || device.inductances.len() != winding_count
                || device.branches.len() != winding_count
                || device.coupling_matrix.len() != winding_count
                || device
                    .coupling_matrix
                    .iter()
                    .any(|row| row.len() != winding_count)
                || binding.branch_ordinals.len() != winding_count
            {
                return Err(SimulationError::Circuit(format!(
                    "periodic MNA transformer '{}' has malformed winding/matrix cardinality",
                    device.name
                )));
            }
            for winding_index in 0..winding_count {
                let branch_ordinal = binding.branch_ordinals[winding_index];
                let slot_index = branch_ordinal.checked_sub(1).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "periodic MNA transformer '{}' winding {} has invalid branch ordinal 0",
                        device.name,
                        winding_index + 1
                    ))
                })?;
                let slot = registrations.get_mut(slot_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "periodic MNA transformer '{}' winding {} has branch ordinal {branch_ordinal}, outside 1..={branch_count}",
                        device.name,
                        winding_index + 1
                    ))
                })?;
                match slot {
                    None => {
                        *slot = Some(PeriodicMnaRegistration::TransformerWinding(
                            binding_index,
                            winding_index,
                        ));
                    }
                    Some(PeriodicMnaRegistration::Inductor(inductor_index)) => {
                        if circuit.inductors.node_pos[*inductor_index]
                            != device.nodes[winding_index].0
                            || circuit.inductors.node_neg[*inductor_index]
                                != device.nodes[winding_index].1
                            || circuit.inductors.inductances[*inductor_index]
                                != device.inductances[winding_index]
                        {
                            return Err(SimulationError::Circuit(format!(
                                "periodic MNA transformer '{}' winding {} disagrees with standalone inductor branch {branch_ordinal}",
                                device.name,
                                winding_index + 1
                            )));
                        }
                    }
                    Some(_) => {
                        return Err(SimulationError::Circuit(format!(
                            "periodic MNA transformer '{}' winding {} aliases a non-inductor or another transformer branch at ordinal {branch_ordinal}",
                            device.name,
                            winding_index + 1
                        )));
                    }
                }
            }
        }

        for (slot_index, registration) in registrations.into_iter().enumerate() {
            let branch_ordinal = slot_index.checked_add(1).ok_or_else(|| {
                SimulationError::Circuit(
                    "periodic MNA branch ordinal exceeds this platform".to_string(),
                )
            })?;
            match registration.ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "periodic MNA canonical branch ordinal {branch_ordinal} is unassigned"
                ))
            })? {
                PeriodicMnaRegistration::VoltageSource(source_index) => {
                    let name = &circuit.voltage_sources.names[source_index];
                    solver
                        .try_add_periodic_voltage_source_branch(
                            circuit.voltage_sources.node_pos[source_index],
                            circuit.voltage_sources.node_neg[source_index],
                            source_index,
                            branch_ordinal,
                            name,
                        )
                        .map_err(|error| {
                            SimulationError::Circuit(format!(
                                "periodic MNA voltage-source registration failed: {error}"
                            ))
                        })?;
                }
                PeriodicMnaRegistration::Inductor(inductor_index) => {
                    let name = &circuit.inductors.names[inductor_index];
                    solver
                        .try_add_periodic_inductor_branch(
                            circuit.inductors.node_pos[inductor_index],
                            circuit.inductors.node_neg[inductor_index],
                            circuit.inductors.inductances[inductor_index],
                            branch_ordinal,
                            name,
                        )
                        .map_err(|error| {
                            SimulationError::Circuit(format!(
                                "periodic MNA inductor registration failed: {error}"
                            ))
                        })?;
                }
                PeriodicMnaRegistration::TransformerWinding(binding_index, winding_index) => {
                    let binding = &circuit.multi_winding_transformers[binding_index];
                    let device = &binding.device;
                    let name = format!("{}#{}", device.name, winding_index + 1);
                    solver
                        .try_add_periodic_inductor_branch(
                            device.nodes[winding_index].0,
                            device.nodes[winding_index].1,
                            device.inductances[winding_index],
                            branch_ordinal,
                            &name,
                        )
                        .map_err(|error| {
                            SimulationError::Circuit(format!(
                                "periodic MNA transformer-winding registration failed: {error}"
                            ))
                        })?;
                }
                PeriodicMnaRegistration::Resistor(resistor_index) => {
                    let name = &circuit.resistor_branches.names[resistor_index];
                    solver
                        .try_add_periodic_resistor_branch(
                            circuit.resistor_branches.node_pos[resistor_index],
                            circuit.resistor_branches.node_neg[resistor_index],
                            circuit.resistor_branches.resistances[resistor_index],
                            circuit.resistor_branches.small_signal_resistances[resistor_index],
                            branch_ordinal,
                            name,
                        )
                        .map_err(|error| {
                            SimulationError::Circuit(format!(
                                "periodic MNA branch-form resistor registration failed: {error}"
                            ))
                        })?;
                }
                PeriodicMnaRegistration::Vcvs(vcvs_index) => {
                    let name = &circuit.vcvs.names[vcvs_index];
                    solver
                        .try_add_periodic_controlled_voltage_source_branch(
                            circuit.vcvs.node_pos[vcvs_index],
                            circuit.vcvs.node_neg[vcvs_index],
                            branch_ordinal,
                            name,
                        )
                        .map_err(|error| {
                            SimulationError::Circuit(format!(
                                "periodic MNA VCVS registration failed: {error}"
                            ))
                        })?;
                }
                PeriodicMnaRegistration::Ccvs(ccvs_index) => {
                    let name = &circuit.ccvs.names[ccvs_index];
                    solver
                        .try_add_periodic_controlled_voltage_source_branch(
                            circuit.ccvs.node_pos[ccvs_index],
                            circuit.ccvs.node_neg[ccvs_index],
                            branch_ordinal,
                            name,
                        )
                        .map_err(|error| {
                            SimulationError::Circuit(format!(
                                "periodic MNA CCVS registration failed: {error}"
                            ))
                        })?;
                }
            }
        }
        self.hb_stamp_controlled_sources(circuit, solver, branch_count)?;
        self.hb_stamp_mutual_inductances(circuit, solver, branch_count)?;
        Ok(())
    }

    /// Validate authored magnetic topology and stamp each mutual inductance
    /// once into the augmented exact-MNA branch block.
    fn hb_stamp_mutual_inductances(
        &self,
        circuit: &CircuitData,
        solver: &mut HbSolver,
        branch_count: usize,
    ) -> Result<(), SimulationError> {
        let num_nodes = circuit.num_nodes();
        let branch_index = |ordinal: usize| num_nodes + ordinal - 1;
        let mut inductor_by_branch = vec![None; branch_count];
        for (inductor_index, &ordinal) in circuit.inductors.branch_indices.iter().enumerate() {
            let slot = ordinal
                .checked_sub(1)
                .and_then(|index| inductor_by_branch.get_mut(index))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "periodic mutual-inductance inductor '{}' has invalid branch ordinal {ordinal}",
                        circuit
                            .inductors
                            .names
                            .get(inductor_index)
                            .map(String::as_str)
                            .unwrap_or("<missing>")
                    ))
                })?;
            if slot.replace(inductor_index).is_some() {
                return Err(SimulationError::Circuit(format!(
                    "periodic mutual-inductance topology assigns inductor branch ordinal {ordinal} more than once"
                )));
            }
        }

        // The authored K records are retained after builder resolution. Prove
        // that every requested pair has exactly one derived runtime overlay,
        // without numerically stamping the retained record a second time.
        for coupling in &circuit.couplings {
            if coupling.name.trim().is_empty() || coupling.inductor_names.len() < 2 {
                return Err(SimulationError::Circuit(format!(
                    "periodic mutual coupling '{}' names fewer than two inductors",
                    coupling.name
                )));
            }
            let mut indices = Vec::with_capacity(coupling.inductor_names.len());
            for inductor_name in &coupling.inductor_names {
                let mut matches = circuit
                    .inductors
                    .names
                    .iter()
                    .enumerate()
                    .filter(|(_, name)| name.eq_ignore_ascii_case(inductor_name));
                let index = matches.next().map(|(index, _)| index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "periodic mutual coupling '{}' references missing inductor '{}'",
                        coupling.name, inductor_name
                    ))
                })?;
                if matches.next().is_some() {
                    return Err(SimulationError::Circuit(format!(
                        "periodic mutual coupling '{}' references non-unique inductor name '{}'",
                        coupling.name, inductor_name
                    )));
                }
                if indices.contains(&index) {
                    return Err(SimulationError::Circuit(format!(
                        "periodic mutual coupling '{}' repeats inductor '{}'",
                        coupling.name, inductor_name
                    )));
                }
                indices.push(index);
            }
            for left in 0..indices.len() {
                for right in (left + 1)..indices.len() {
                    let i = indices[left];
                    let j = indices[right];
                    let ordinal_i = circuit.inductors.branch_indices[i];
                    let ordinal_j = circuit.inductors.branch_indices[j];
                    let overlays = circuit
                        .coupled_inductor_pairs
                        .iter()
                        .filter(|binding| {
                            binding.device.name.eq_ignore_ascii_case(&coupling.name)
                                && ((binding.branch1_ordinal == ordinal_i
                                    && binding.branch2_ordinal == ordinal_j)
                                    || (binding.branch1_ordinal == ordinal_j
                                        && binding.branch2_ordinal == ordinal_i))
                        })
                        .collect::<Vec<_>>();
                    if overlays.len() != 1 {
                        return Err(SimulationError::Circuit(format!(
                            "periodic mutual coupling '{}' pair ('{}', '{}') resolves to {} runtime overlays; expected exactly one",
                            coupling.name,
                            circuit.inductors.names[i],
                            circuit.inductors.names[j],
                            overlays.len()
                        )));
                    }
                    let expected = coupling.mutual_inductance(
                        circuit.inductors.inductances[i],
                        circuit.inductors.inductances[j],
                    );
                    let actual = overlays[0].device.m;
                    let tolerance =
                        32.0 * Value::EPSILON * expected.abs().max(actual.abs()).max(1.0);
                    if !actual.is_finite() || (actual - expected).abs() > tolerance {
                        return Err(SimulationError::Circuit(format!(
                            "periodic mutual coupling '{}' pair ('{}', '{}') has runtime mutual inductance {actual}, expected {expected}",
                            coupling.name, circuit.inductors.names[i], circuit.inductors.names[j]
                        )));
                    }
                }
            }
        }

        for binding in &circuit.coupled_inductor_pairs {
            let device = &binding.device;
            let ordinal1 = binding.branch1_ordinal;
            let ordinal2 = binding.branch2_ordinal;
            if ordinal1 == ordinal2 {
                return Err(SimulationError::Circuit(format!(
                    "periodic mutual pair '{}' aliases branch ordinal {ordinal1} on both windings",
                    device.name
                )));
            }
            let inductor1 = ordinal1
                .checked_sub(1)
                .and_then(|index| inductor_by_branch.get(index))
                .and_then(|index| *index)
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "periodic mutual pair '{}' has missing first inductor control branch ordinal {ordinal1}",
                        device.name
                    ))
                })?;
            let inductor2 = ordinal2
                .checked_sub(1)
                .and_then(|index| inductor_by_branch.get(index))
                .and_then(|index| *index)
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "periodic mutual pair '{}' has missing second inductor control branch ordinal {ordinal2}",
                        device.name
                    ))
                })?;
            if circuit.inductors.node_pos[inductor1] != device.node1_pos
                || circuit.inductors.node_neg[inductor1] != device.node1_neg
                || circuit.inductors.inductances[inductor1] != device.l1
                || circuit.inductors.node_pos[inductor2] != device.node2_pos
                || circuit.inductors.node_neg[inductor2] != device.node2_neg
                || circuit.inductors.inductances[inductor2] != device.l2
                || !device.m.is_finite()
            {
                return Err(SimulationError::Circuit(format!(
                    "periodic mutual pair '{}' disagrees with its canonical inductor branches",
                    device.name
                )));
            }
            for (row, column) in [(ordinal1, ordinal2), (ordinal2, ordinal1)] {
                solver
                    .try_add_exact_mna_inductance_entry(
                        branch_index(row),
                        branch_index(column),
                        device.m,
                        &device.name,
                    )
                    .map_err(|error| SimulationError::Circuit(error.to_string()))?;
            }
        }

        for binding in &circuit.multi_winding_transformers {
            let device = &binding.device;
            let winding_count = device.num_windings;
            if device.name.trim().is_empty()
                || winding_count == 0
                || device.nodes.len() != winding_count
                || device.inductances.len() != winding_count
                || device.branches.len() != winding_count
                || binding.branch_ordinals.len() != winding_count
                || device.coupling_matrix.len() != winding_count
                || device
                    .coupling_matrix
                    .iter()
                    .any(|row| row.len() != winding_count)
            {
                return Err(SimulationError::Circuit(format!(
                    "periodic MNA transformer '{}' has malformed winding/matrix cardinality",
                    device.name
                )));
            }
            for winding in 0..winding_count {
                let (pos, neg) = device.nodes[winding];
                let ordinal = binding.branch_ordinals[winding];
                let expected_matrix_branch = num_nodes.checked_add(ordinal).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "periodic MNA transformer '{}' winding {} branch index exceeds this platform",
                        device.name,
                        winding + 1
                    ))
                })?;
                if pos > num_nodes
                    || neg > num_nodes
                    || pos == neg
                    || !device.inductances[winding].is_finite()
                    || device.inductances[winding] == 0.0
                    || device.branches[winding] != Some(expected_matrix_branch)
                {
                    return Err(SimulationError::Circuit(format!(
                        "periodic MNA transformer '{}' winding {} has malformed terminals, inductance, or canonical branch binding",
                        device.name,
                        winding + 1
                    )));
                }
                if binding.branch_ordinals[..winding].contains(&ordinal) {
                    return Err(SimulationError::Circuit(format!(
                        "periodic MNA transformer '{}' repeats branch ordinal {ordinal}",
                        device.name
                    )));
                }
                for column in 0..winding_count {
                    if !device.coupling_matrix[winding][column].is_finite() {
                        return Err(SimulationError::Circuit(format!(
                            "periodic MNA transformer '{}' has non-finite coupling coefficient ({winding}, {column})",
                            device.name
                        )));
                    }
                    let mutual = device.mutual_inductance(winding, column);
                    if !mutual.is_finite() {
                        return Err(SimulationError::Circuit(format!(
                            "periodic MNA transformer '{}' has non-finite inductance matrix entry ({winding}, {column})",
                            device.name
                        )));
                    }
                    if winding != column {
                        let reverse = device.mutual_inductance(column, winding);
                        let tolerance =
                            32.0 * Value::EPSILON * mutual.abs().max(reverse.abs()).max(1.0);
                        if (mutual - reverse).abs() > tolerance {
                            return Err(SimulationError::Circuit(format!(
                                "periodic MNA transformer '{}' has asymmetric mutual inductance entries ({winding}, {column})",
                                device.name
                            )));
                        }
                        solver
                            .try_add_exact_mna_inductance_entry(
                                branch_index(ordinal),
                                branch_index(binding.branch_ordinals[column]),
                                mutual,
                                &device.name,
                            )
                            .map_err(|error| SimulationError::Circuit(error.to_string()))?;
                    }
                }
            }
        }
        Ok(())
    }

    fn hb_stamp_controlled_sources(
        &self,
        circuit: &CircuitData,
        solver: &mut HbSolver,
        branch_count: usize,
    ) -> Result<(), SimulationError> {
        let num_nodes = circuit.num_nodes();
        let checked_node = |name: &str, role: &str, node: usize| {
            if node <= num_nodes {
                Ok(node)
            } else {
                Err(SimulationError::Circuit(format!(
                    "periodic MNA controlled source '{name}' has {role} node {node} outside 0..={num_nodes}"
                )))
            }
        };
        let checked_branch = |name: &str, role: &str, branch: usize| {
            if (1..=branch_count).contains(&branch) {
                Ok(branch)
            } else {
                Err(SimulationError::Circuit(format!(
                    "periodic MNA controlled source '{name}' has missing or invalid {role} branch ordinal {branch}; expected 1..={branch_count}"
                )))
            }
        };
        let branch_index = |ordinal: usize| num_nodes + ordinal - 1;

        if circuit.vccs.names.len() != circuit.vccs.node_pos.len()
            || circuit.vccs.names.len() != circuit.vccs.node_neg.len()
            || circuit.vccs.names.len() != circuit.vccs.ctrl_pos.len()
            || circuit.vccs.names.len() != circuit.vccs.ctrl_neg.len()
            || circuit.vccs.names.len() != circuit.vccs.transconductances.len()
        {
            return Err(SimulationError::Circuit(
                "periodic MNA VCCS storage is misaligned".to_string(),
            ));
        }
        for index in 0..circuit.vccs.len() {
            let name = &circuit.vccs.names[index];
            let np = checked_node(name, "positive output", circuit.vccs.node_pos[index])?;
            let nn = checked_node(name, "negative output", circuit.vccs.node_neg[index])?;
            let cp = checked_node(name, "positive control", circuit.vccs.ctrl_pos[index])?;
            let cn = checked_node(name, "negative control", circuit.vccs.ctrl_neg[index])?;
            let gm = circuit.vccs.transconductances[index];
            if !gm.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "periodic MNA VCCS '{name}' has non-finite transconductance"
                )));
            }
            for (row, column, coefficient) in
                [(np, cp, gm), (np, cn, -gm), (nn, cp, -gm), (nn, cn, gm)]
            {
                if row > 0 && column > 0 {
                    solver.add_conductance(row - 1, column - 1, coefficient);
                }
            }
        }

        for index in 0..circuit.vcvs.len() {
            let name = &circuit.vcvs.names[index];
            let cp = checked_node(name, "positive control", circuit.vcvs.ctrl_pos[index])?;
            let cn = checked_node(name, "negative control", circuit.vcvs.ctrl_neg[index])?;
            let branch = checked_branch(name, "output", circuit.vcvs.branch_indices[index])?;
            let gain = circuit.vcvs.gains[index];
            if !gain.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "periodic MNA VCVS '{name}' has non-finite gain"
                )));
            }
            for (control, coefficient) in [(cp, -gain), (cn, gain)] {
                if control > 0 {
                    solver
                        .try_add_exact_mna_static_entry(
                            branch_index(branch),
                            control - 1,
                            coefficient,
                            name,
                        )
                        .map_err(|error| SimulationError::Circuit(error.to_string()))?;
                }
            }
        }

        if circuit.cccs.names.len() != circuit.cccs.node_pos.len()
            || circuit.cccs.names.len() != circuit.cccs.node_neg.len()
            || circuit.cccs.names.len() != circuit.cccs.ctrl_branch.len()
            || circuit.cccs.names.len() != circuit.cccs.gains.len()
        {
            return Err(SimulationError::Circuit(
                "periodic MNA CCCS storage is misaligned".to_string(),
            ));
        }
        for index in 0..circuit.cccs.len() {
            let name = &circuit.cccs.names[index];
            let np = checked_node(name, "positive output", circuit.cccs.node_pos[index])?;
            let nn = checked_node(name, "negative output", circuit.cccs.node_neg[index])?;
            let control = checked_branch(name, "control", circuit.cccs.ctrl_branch[index])?;
            let gain = circuit.cccs.gains[index];
            if !gain.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "periodic MNA CCCS '{name}' has non-finite gain"
                )));
            }
            for (output, coefficient) in [(np, gain), (nn, -gain)] {
                if output > 0 {
                    solver
                        .try_add_exact_mna_static_entry(
                            output - 1,
                            branch_index(control),
                            coefficient,
                            name,
                        )
                        .map_err(|error| SimulationError::Circuit(error.to_string()))?;
                }
            }
        }

        for index in 0..circuit.ccvs.len() {
            let name = &circuit.ccvs.names[index];
            let output = checked_branch(name, "output", circuit.ccvs.branch_indices[index])?;
            let control = checked_branch(name, "control", circuit.ccvs.ctrl_branch[index])?;
            let rm = circuit.ccvs.transresistances[index];
            if !rm.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "periodic MNA CCVS '{name}' has non-finite transresistance"
                )));
            }
            solver
                .try_add_exact_mna_static_entry(
                    branch_index(output),
                    branch_index(control),
                    -rm,
                    name,
                )
                .map_err(|error| SimulationError::Circuit(error.to_string()))?;
        }
        Ok(())
    }

    /// Stamp ideal voltage sources into HB solver using MNA branch equations.
    pub(in crate::engine::hb) fn hb_stamp_voltage_sources(
        &self,
        circuit: &CircuitData,
        solver: &mut HbSolver,
        config: &HbConfig,
        drive_tones: &[HbDriveTone],
    ) -> Result<(), SimulationError> {
        for i in 0..circuit.voltage_sources.len() {
            let np = circuit.voltage_sources.node_pos[i];
            let nn = circuit.voltage_sources.node_neg[i];
            let dc = circuit.voltage_sources.dc_values[i];
            let ac_mag = circuit
                .voltage_sources
                .ac_magnitudes
                .get(i)
                .copied()
                .unwrap_or(0.0);
            let ac_phase = circuit
                .voltage_sources
                .ac_phases
                .get(i)
                .copied()
                .unwrap_or(0.0);
            let spec = circuit
                .voltage_sources
                .source_specs
                .get(i)
                .and_then(|s| s.as_ref());
            let source_name = circuit
                .voltage_sources
                .names
                .get(i)
                .map(|name| name.as_str())
                .unwrap_or("");
            let harmonics = Self::hb_drive_harmonics_for_source(drive_tones, source_name);
            let spectrum = Self::hb_source_spectrum(
                dc,
                ac_mag,
                ac_phase,
                spec,
                config,
                &harmonics,
                self.config.spice_dialect,
            )?;
            solver
                .try_add_named_voltage_source_branch_harmonics(
                    np,
                    nn,
                    spectrum.dc,
                    &spectrum.harmonics,
                    source_name,
                )
                .map_err(|error| {
                    SimulationError::Circuit(format!(
                        "HB voltage-source registration failed: {error}"
                    ))
                })?;
        }
        Ok(())
    }

    /// Stamp current sources into HB solver
    ///
    /// Stamps both DC and AC components:
    /// - DC component goes into harmonic 0
    /// - AC component is applied to configured HB drive harmonics with magnitude and phase
    pub(in crate::engine::hb) fn hb_stamp_current_sources(
        &self,
        circuit: &CircuitData,
        solver: &mut HbSolver,
        config: &HbConfig,
        drive_tones: &[HbDriveTone],
    ) -> Result<(), SimulationError> {
        for i in 0..circuit.current_sources.len() {
            let np = circuit.current_sources.node_pos[i];
            let nn = circuit.current_sources.node_neg[i];
            let dc = circuit.current_sources.dc_values[i];

            let ac_mag = circuit
                .current_sources
                .ac_magnitudes
                .get(i)
                .copied()
                .unwrap_or(0.0);
            let ac_phase = circuit
                .current_sources
                .ac_phases
                .get(i)
                .copied()
                .unwrap_or(0.0);
            let spec = circuit
                .current_sources
                .source_specs
                .get(i)
                .and_then(|s| s.as_ref());
            let source_name = circuit
                .current_sources
                .names
                .get(i)
                .map(|name| name.as_str())
                .unwrap_or("");
            let harmonics = Self::hb_drive_harmonics_for_source(drive_tones, source_name);
            let spectrum = Self::hb_source_spectrum(
                dc,
                ac_mag,
                ac_phase,
                spec,
                config,
                &harmonics,
                self.config.spice_dialect,
            )?;

            // Stamp DC component (harmonic 0)
            if np > 0 {
                solver.add_dc_source(np - 1, -spectrum.dc); // Current leaves at + terminal
            }
            if nn > 0 {
                solver.add_dc_source(nn - 1, spectrum.dc); // Current enters at - terminal
            }

            for (harmonic, amplitude, phase) in spectrum.harmonics {
                if amplitude != 0.0 {
                    if np > 0 {
                        // Current leaves at + terminal.
                        solver.add_harmonic_source(np - 1, harmonic, -amplitude, phase);
                    }
                    if nn > 0 {
                        // Current enters at - terminal.
                        solver.add_harmonic_source(nn - 1, harmonic, amplitude, phase);
                    }
                }
            }
        }
        Ok(())
    }

    /// Stamp a two-terminal admittance (conductance or capacitance) into HB solver
    /// - is_conductance: true stamps into G matrix, false stamps into C matrix
    pub(in crate::engine::hb) fn hb_stamp_admittance(
        &self,
        solver: &mut HbSolver,
        np: usize,
        nn: usize,
        value: Value,
        is_conductance: bool,
    ) {
        // Standard MNA stamp pattern for two-terminal element
        if np > 0 && nn > 0 {
            // Both nodes are non-ground
            let i = np - 1;
            let j = nn - 1;
            if is_conductance {
                solver.add_conductance(i, i, value);
                solver.add_conductance(i, j, -value);
                solver.add_conductance(j, i, -value);
                solver.add_conductance(j, j, value);
            } else {
                solver.add_capacitance(i, i, value);
                solver.add_capacitance(i, j, -value);
                solver.add_capacitance(j, i, -value);
                solver.add_capacitance(j, j, value);
            }
        } else if np > 0 {
            // nn is ground
            let i = np - 1;
            if is_conductance {
                solver.add_conductance(i, i, value);
            } else {
                solver.add_capacitance(i, i, value);
            }
        } else if nn > 0 {
            // np is ground
            let i = nn - 1;
            if is_conductance {
                solver.add_conductance(i, i, value);
            } else {
                solver.add_capacitance(i, i, value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SimulationConfig;
    use crate::analysis::harmonic_balance::{DC_SHORT_CONDUCTANCE, HbSolverState};
    use crate::circuit::MultiWindingTransformerBinding;
    use crate::device::{CoupledInductorPair, MultiWindingTransformer};

    #[test]
    fn hb_engine_stamps_dangling_series_rl_as_a_dc_short() {
        let netlist = Netlist::parse(
            "HB dangling series R-L DC topology\n\
             V1 source 0 1\n\
             Rsource source line 50\n\
             Cline line 0 420p\n\
             Rcable line internal 0.12\n\
             Lcable internal dangling 1.2u\n\
             .end\n",
        )
        .expect("R-L fixture parses");
        let analysis = Engine::new(SimulationConfig::default())
            .run_hb(&netlist, HbConfig::new(1.0e6).with_harmonics(1))
            .expect("native HB R-L fixture solves");
        let result = &analysis.result;

        assert!(analysis.converged && result.is_valid());
        assert!(
            result.residual_norm <= 1.0e-9,
            "HB residual was {:.17e}",
            result.residual_norm
        );
        let dc = |name: &str| {
            result
                .spectral_voltages
                .iter()
                .find(|voltage| voltage.node_name.eq_ignore_ascii_case(name))
                .unwrap_or_else(|| panic!("missing HB node {name}"))
                .dc()
        };
        let source = dc("source");
        let line = dc("line");
        let internal = dc("internal");
        let dangling = dc("dangling");
        for (name, voltage) in [
            ("source", source),
            ("line", line),
            ("internal", internal),
            ("dangling", dangling),
        ] {
            assert!(
                (voltage - 1.0).abs() <= 1.0e-8,
                "{name} DC voltage was {voltage:.17e}"
            );
        }

        let source_resistor_current = (source - line) / 50.0;
        let cable_resistor_current = (line - internal) / 0.12;
        let inductor_current = DC_SHORT_CONDUCTANCE * (internal - dangling);
        assert!(source_resistor_current.abs() <= 1.0e-9);
        assert!(cable_resistor_current.abs() <= 1.0e-9);
        assert!(inductor_current.abs() <= 1.0e-9);
        assert!((cable_resistor_current - inductor_current).abs() <= 1.0e-9);

        let source_branch = result
            .mna_branch_currents
            .iter()
            .find(|branch| branch.device_name.eq_ignore_ascii_case("V1"))
            .expect("source MNA branch is retained");
        assert!(source_branch.coefficients[0].norm() <= 1.0e-9);
        let inductor = result
            .reactive_spectra
            .iter()
            .find(|reactive| reactive.device_name.eq_ignore_ascii_case("Lcable"))
            .expect("inductor spectrum is retained");
        assert!(inductor.voltage_coefficients[0].norm() <= 1.0e-9);
        assert!(inductor.current_coefficients[0].norm() <= 1.0e-9);
    }

    #[test]
    fn periodic_cccs_missing_control_branch_fails_closed() {
        let mut circuit = CircuitData::new();
        let out = circuit.get_or_create_node("out");
        circuit.cccs.add("Fbad".to_string(), out, 0, 0, 2.0);
        let engine = Engine::new(SimulationConfig::default());
        let mut solver = HbSolver::try_new(HbConfig::new(1.0e6).with_harmonics(1), 1)
            .expect("solver fixture is valid");

        let error = engine
            .hb_stamp_periodic_mna_branches(&circuit, &mut solver)
            .expect_err("an unresolved CCCS control branch must not be stamped");
        let message = error.to_string();
        assert!(
            message.contains("Fbad")
                && message.contains("control")
                && message.contains("ordinal 0"),
            "unexpected missing-control diagnostic: {message}"
        );
    }

    #[test]
    fn periodic_ccvs_out_of_range_control_branch_fails_closed() {
        let mut circuit = CircuitData::new();
        let out = circuit.get_or_create_node("out");
        let branch = circuit.allocate_branch_named("Hbad");
        circuit
            .ccvs
            .add("Hbad".to_string(), out, 0, branch, branch + 1, 50.0);
        let engine = Engine::new(SimulationConfig::default());
        let mut solver = HbSolver::try_new(HbConfig::new(1.0e6).with_harmonics(1), 1)
            .expect("solver fixture is valid");

        let error = engine
            .hb_stamp_periodic_mna_branches(&circuit, &mut solver)
            .expect_err("an out-of-range CCVS control branch must not be stamped");
        let message = error.to_string();
        assert!(
            message.contains("Hbad")
                && message.contains("control")
                && message.contains("expected 1..=1"),
            "unexpected malformed-control diagnostic: {message}"
        );
    }

    #[test]
    fn periodic_multi_winding_transformer_preserves_mutual_sign_at_all_harmonics() {
        let mut circuit = CircuitData::new();
        let primary = circuit.get_or_create_node("primary");
        let secondary = circuit.get_or_create_node("secondary");
        let branch1 = circuit.allocate_branch_named("T1#1");
        let branch2 = circuit.allocate_branch_named("T1#2");
        let l1: Value = 100.0e-6;
        let l2: Value = 25.0e-6;
        let k: Value = 0.8;
        let mutual = k * (l1 * l2).sqrt();
        let mut transformer = MultiWindingTransformer::new(
            "T1".to_string(),
            vec![(primary, 0), (secondary, 0)],
            vec![l1, l2],
            vec![vec![1.0, k], vec![k, 1.0]],
        );
        transformer.set_branches(vec![
            circuit.get_branch_matrix_index(branch1),
            circuit.get_branch_matrix_index(branch2),
        ]);
        circuit
            .multi_winding_transformers
            .push(MultiWindingTransformerBinding {
                branch_ordinals: vec![branch1, branch2],
                device: transformer,
            });

        let fundamental = 1.0e6;
        let harmonics = 3;
        let engine = Engine::new(SimulationConfig::default());
        let mut solver = HbSolver::try_new(
            HbConfig::new(fundamental).with_harmonics(harmonics),
            circuit.num_nodes(),
        )
        .expect("solver fixture is valid");
        engine
            .hb_stamp_periodic_mna_branches(&circuit, &mut solver)
            .expect("multi-winding branch topology stamps");
        let load = 50.0;
        solver.add_conductance(secondary - 1, secondary - 1, 1.0 / load);
        for harmonic in 1..=harmonics {
            solver.set_harmonic_source(primary - 1, harmonic, 1.0, 0.0);
        }
        let mut state = HbSolverState::new(circuit.num_nodes(), harmonics);
        solver
            .solve_linear(&mut state)
            .expect("multi-winding exact HB solve completes");

        let leakage = l2 - mutual * mutual / l1;
        for harmonic in 1..=harmonics {
            let omega = 2.0 * std::f64::consts::PI * fundamental * harmonic as Value;
            let expected =
                Complex64::new(mutual / l1, 0.0) / Complex64::new(1.0, omega * leakage / load);
            let actual = state.x[secondary - 1][harmonic] / state.x[primary - 1][harmonic];
            let scale = expected.norm().max(1.0);
            assert!(
                (actual - expected).norm() <= 3.0e-10 * scale,
                "harmonic {harmonic}: actual={actual}, expected={expected}"
            );
        }
    }

    #[test]
    fn periodic_mutual_pair_missing_canonical_inductor_branch_fails_closed() {
        let mut circuit = CircuitData::new();
        let primary = circuit.get_or_create_node("primary");
        let secondary = circuit.get_or_create_node("secondary");
        let branch1 = circuit.allocate_branch_named("L1");
        let branch2 = circuit.allocate_branch_named("L2");
        circuit
            .inductors
            .add("L1".to_string(), primary, 0, branch1, 100.0e-6);
        circuit
            .inductors
            .add("L2".to_string(), secondary, 0, branch2, 25.0e-6);
        circuit.add_coupled_inductor_pair(
            branch1,
            branch2 + 1,
            CoupledInductorPair::new(
                "Kbad".to_string(),
                primary,
                0,
                100.0e-6,
                secondary,
                0,
                25.0e-6,
                0.8,
            ),
        );
        let engine = Engine::new(SimulationConfig::default());
        let mut solver =
            HbSolver::try_new(HbConfig::new(1.0e6).with_harmonics(1), circuit.num_nodes())
                .expect("solver fixture is valid");
        let error = engine
            .hb_stamp_periodic_mna_branches(&circuit, &mut solver)
            .expect_err("a mutual overlay with a missing branch must fail closed");
        let message = error.to_string();
        assert!(
            message.contains("Kbad")
                && message.contains("missing second inductor")
                && message.contains("ordinal 3"),
            "unexpected malformed mutual topology diagnostic: {message}"
        );
    }

    #[test]
    fn periodic_authored_coupling_missing_resolved_overlay_fails_closed() {
        let mut circuit = CircuitData::new();
        let primary = circuit.get_or_create_node("primary");
        let secondary = circuit.get_or_create_node("secondary");
        let branch1 = circuit.allocate_branch_named("L1");
        let branch2 = circuit.allocate_branch_named("L2");
        circuit
            .inductors
            .add("L1".to_string(), primary, 0, branch1, 100.0e-6);
        circuit
            .inductors
            .add("L2".to_string(), secondary, 0, branch2, 25.0e-6);
        circuit.couplings.push(crate::device::InductorCoupling::new(
            "Kmissing".to_string(),
            vec!["L1".to_string(), "L2".to_string()],
            0.8,
        ));
        let engine = Engine::new(SimulationConfig::default());
        let mut solver =
            HbSolver::try_new(HbConfig::new(1.0e6).with_harmonics(1), circuit.num_nodes())
                .expect("solver fixture is valid");
        let error = engine
            .hb_stamp_periodic_mna_branches(&circuit, &mut solver)
            .expect_err("an unresolved authored K card must fail closed");
        let message = error.to_string();
        assert!(
            message.contains("Kmissing")
                && message.contains("resolves to 0 runtime overlays")
                && message.contains("expected exactly one"),
            "unexpected unresolved coupling diagnostic: {message}"
        );
    }

    #[test]
    fn periodic_transformer_missing_device_branch_binding_fails_closed() {
        let mut circuit = CircuitData::new();
        let primary = circuit.get_or_create_node("primary");
        let secondary = circuit.get_or_create_node("secondary");
        let branch1 = circuit.allocate_branch_named("Tbad#1");
        let branch2 = circuit.allocate_branch_named("Tbad#2");
        let transformer = MultiWindingTransformer::new(
            "Tbad".to_string(),
            vec![(primary, 0), (secondary, 0)],
            vec![100.0e-6, 25.0e-6],
            vec![vec![1.0, 0.8], vec![0.8, 1.0]],
        );
        circuit
            .multi_winding_transformers
            .push(MultiWindingTransformerBinding {
                branch_ordinals: vec![branch1, branch2],
                device: transformer,
            });
        let engine = Engine::new(SimulationConfig::default());
        let mut solver =
            HbSolver::try_new(HbConfig::new(1.0e6).with_harmonics(1), circuit.num_nodes())
                .expect("solver fixture is valid");
        let error = engine
            .hb_stamp_periodic_mna_branches(&circuit, &mut solver)
            .expect_err("a transformer with missing device branch indices must fail closed");
        let message = error.to_string();
        assert!(
            message.contains("Tbad")
                && message.contains("winding 1")
                && message.contains("canonical branch binding"),
            "unexpected transformer branch diagnostic: {message}"
        );
    }
}
