use super::*;

#[derive(Debug, Clone, Copy)]
enum PeriodicMnaRegistration {
    VoltageSource(usize),
    Inductor(usize),
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
            solver.add_nonlinear_device(
                NonlinearDeviceInstance::diode(anode, cathode, diode.is, diode.n)
                    .with_thermal_voltage(diode.vt)
                    .with_junction_caps(
                        DepletionCap::new(diode.cj0, diode.vj, diode.m, diode.fc),
                        DepletionCap::none(),
                        diode.tt,
                        0.0,
                    ),
            );
        }

        for bjt in &circuit.bjts.devices {
            let collector = Self::hb_node_to_solver_index(bjt.node_collector, num_nodes);
            let base = Self::hb_node_to_solver_index(bjt.node_base, num_nodes);
            let emitter = Self::hb_node_to_solver_index(bjt.node_emitter, num_nodes);
            let instance = match bjt.bjt_type {
                crate::device::BjtType::Npn => NonlinearDeviceInstance::npn_bjt(
                    collector, base, emitter, bjt.is, bjt.bf, bjt.br, bjt.nf, bjt.nr, bjt.vaf,
                ),
                crate::device::BjtType::Pnp => NonlinearDeviceInstance::pnp_bjt(
                    collector, base, emitter, bjt.is, bjt.bf, bjt.br, bjt.nf, bjt.nr, bjt.vaf,
                ),
            };
            solver.add_nonlinear_device(instance.with_thermal_voltage(bjt.vt).with_junction_caps(
                DepletionCap::new(bjt.cje, bjt.vje, bjt.mje, bjt.fc),
                DepletionCap::new(bjt.cjc, bjt.vjc, bjt.mjc, bjt.fc),
                bjt.tf,
                bjt.tr,
            ));
        }

        for mos in &circuit.mosfets.devices {
            let drain = Self::hb_node_to_solver_index(mos.node_drain, num_nodes);
            let gate = Self::hb_node_to_solver_index(mos.node_gate, num_nodes);
            let source = Self::hb_node_to_solver_index(mos.node_source, num_nodes);
            let bulk = Self::hb_node_to_solver_index(mos.node_bulk, num_nodes);
            let kp = mos.kp.max(1e-18);
            let instance = match mos.mos_type {
                crate::device::MosType::Nmos => NonlinearDeviceInstance::nmos(
                    drain, gate, source, bulk, mos.vto, kp, mos.lambda,
                ),
                // The solver works in the polarity frame: the effective
                // threshold is -VTO, which keeps depletion PMOS negative.
                crate::device::MosType::Pmos => NonlinearDeviceInstance::pmos(
                    drain, gate, source, bulk, -mos.vto, kp, mos.lambda,
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
            let leff = (mos.l - 2.0 * mos.ld).max(1e-12);
            let instance = instance
                .with_thermal_voltage(mos.vt)
                .with_body_effect(mos.gamma, mos.phi)
                .with_intrinsic_gate(mos.cox * mos.w * leff)
                .with_bulk_junctions(
                    DepletionCap::new(cbs0, mos.pb, mos.mj, mos.fc),
                    DepletionCap::new(cbd0, mos.pb, mos.mj, mos.fc),
                    is_s,
                    is_d,
                );
            if let Some(temp_k) = mos.noise_absolute_temperature {
                solver.add_nonlinear_device_with_absolute_noise_temperature(instance, temp_k);
            } else {
                solver.add_nonlinear_device_with_noise_temperature_offset(
                    instance,
                    mos.noise_temperature_offset,
                );
            }

            // Gate overlap capacitances are bias-independent in level 1:
            // stamp them as ordinary linear capacitors.
            let cgs_ov = (mos.cgso * mos.w).max(0.0);
            let cgd_ov = (mos.cgdo * mos.w).max(0.0);
            let cgb_ov = (mos.cgbo * leff).max(0.0);
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
            let beta = jfet.params.beta.max(1e-18);
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
                0.0,
            );
            if let Some(temp_k) = jfet.noise_absolute_temperature {
                solver.add_nonlinear_device_with_absolute_noise_temperature(instance, temp_k);
            } else {
                solver
                    .add_nonlinear_device_with_noise_temperature_offset(instance, jfet.noise_dtemp);
            }
        }

        for sw in &circuit.vswitches {
            let node_pos = Self::hb_node_to_solver_index(sw.node_pos, num_nodes);
            let node_neg = Self::hb_node_to_solver_index(sw.node_neg, num_nodes);
            let ctrl_pos = Self::hb_node_to_solver_index(sw.ctrl_pos, num_nodes);
            let ctrl_neg = Self::hb_node_to_solver_index(sw.ctrl_neg, num_nodes);
            solver.add_voltage_switch(
                node_pos, node_neg, ctrl_pos, ctrl_neg, sw.vt, sw.vh, sw.ron, sw.roff, sw.smooth,
            );
        }

        for sw in &circuit.iswitches {
            let Ok(ctrl) = Self::hb_resolve_iswitch_control(circuit, sw, num_nodes) else {
                continue;
            };
            let node_pos = Self::hb_node_to_solver_index(sw.node_pos, num_nodes);
            let node_neg = Self::hb_node_to_solver_index(sw.node_neg, num_nodes);
            solver.add_current_switch(
                node_pos,
                node_neg,
                ctrl.ctrl_pos,
                ctrl.ctrl_neg,
                sw.it + ctrl.control_current_bias,
                sw.ih,
                sw.ron,
                sw.roff,
                sw.smooth,
                HB_NORTON_G,
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

            // Stamp conductance matrix
            self.hb_stamp_admittance(solver, np, nn, g, true);
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

    /// Stamp inductors into HB solver L matrix
    ///
    /// In the frequency domain, inductors have admittance Y_L = 1/(jωL).
    /// The solver handles the frequency-dependent admittance at each harmonic:
    /// - DC (k=0): short circuit (large conductance)
    /// - AC (k>0): Y_L = -j/(k*ω₀*L)
    pub(in crate::engine::hb) fn hb_stamp_inductors(
        &self,
        circuit: &CircuitData,
        solver: &mut HbSolver,
    ) {
        for i in 0..circuit.inductors.len() {
            let np = circuit.inductors.node_pos[i];
            let nn = circuit.inductors.node_neg[i];
            let l = circuit.inductors.inductances[i];

            // Stamp inductance matrix
            self.hb_stamp_inductance(solver, np, nn, l);
        }
    }

    /// Register every supported exact HB MNA branch in the circuit's
    /// canonical one-based branch order.
    ///
    /// Linear HB, PAC, and PNoise support independent voltage-source and
    /// uncoupled-inductor branch equations. If authored voltage-source spectra
    /// were registered first, the exact descriptors retain them for the
    /// large-signal solve; otherwise they describe zero-valued small-signal
    /// constraints. The caller rejects other branch families before this
    /// boundary, and this routine independently proves a complete unique map.
    pub(in crate::engine::hb) fn hb_stamp_periodic_mna_branches(
        &self,
        circuit: &CircuitData,
        solver: &mut HbSolver,
    ) -> Result<(), SimulationError> {
        let branch_count = circuit.num_branches();
        let represented_count = circuit
            .voltage_sources
            .len()
            .checked_add(circuit.inductors.len())
            .ok_or_else(|| {
                SimulationError::Circuit(
                    "periodic MNA supported-branch count overflows this platform".to_string(),
                )
            })?;
        if represented_count != branch_count {
            return Err(SimulationError::Circuit(format!(
                "periodic MNA supports {represented_count} voltage-source/inductor branches, but the circuit declares {branch_count} canonical branches"
            )));
        }

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
            }
        }
        Ok(())
    }

    /// Stamp a two-terminal inductance into HB solver L matrix
    pub(in crate::engine::hb) fn hb_stamp_inductance(
        &self,
        solver: &mut HbSolver,
        np: usize,
        nn: usize,
        value: Value,
    ) {
        // Standard MNA stamp pattern for two-terminal inductor
        if np > 0 && nn > 0 {
            // Both nodes are non-ground
            let i = np - 1;
            let j = nn - 1;
            solver.add_inductance(i, i, value);
            solver.add_inductance(i, j, -value);
            solver.add_inductance(j, i, -value);
            solver.add_inductance(j, j, value);
        } else if np > 0 {
            // nn is ground
            let i = np - 1;
            solver.add_inductance(i, i, value);
        } else if nn > 0 {
            // np is ground
            let i = nn - 1;
            solver.add_inductance(i, i, value);
        }
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
            let spectrum =
                Self::hb_source_spectrum(dc, ac_mag, ac_phase, spec, config, &harmonics)?;
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

    /// Stamp ideal voltage sources as stiff Norton equivalents for nonlinear HB.
    ///
    /// Nonlinear HB Newton currently solves in node-voltage space only. Converting
    /// ideal voltage sources to Norton form avoids branch-current unknowns while
    /// preserving source waveforms with a very small equivalent source resistance.
    pub(in crate::engine::hb) fn hb_stamp_voltage_sources_norton(
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

            let spectrum =
                Self::hb_source_spectrum(dc, ac_mag, ac_phase, spec, config, &harmonics)?;

            self.hb_stamp_admittance(solver, np, nn, HB_NORTON_G, true);

            // Norton conversion of `V` in series with Rs = 1/G: a current source
            // G*V injecting INTO the positive node, in parallel with G. The
            // independent-current-source convention (current pulled out of the
            // positive node) does not apply here.
            let i_dc = spectrum.dc * HB_NORTON_G;
            if np > 0 {
                solver.add_dc_source(np - 1, i_dc);
            }
            if nn > 0 {
                solver.add_dc_source(nn - 1, -i_dc);
            }

            for (harmonic, amplitude, phase) in spectrum.harmonics {
                let i_ac = amplitude * HB_NORTON_G;
                if i_ac.abs() > 1e-30 {
                    if np > 0 {
                        solver.add_harmonic_source(np - 1, harmonic, i_ac, phase);
                    }
                    if nn > 0 {
                        solver.add_harmonic_source(nn - 1, harmonic, -i_ac, phase);
                    }
                }
            }
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
            let spectrum =
                Self::hb_source_spectrum(dc, ac_mag, ac_phase, spec, config, &harmonics)?;

            // Stamp DC component (harmonic 0)
            if np > 0 {
                solver.add_dc_source(np - 1, -spectrum.dc); // Current leaves at + terminal
            }
            if nn > 0 {
                solver.add_dc_source(nn - 1, spectrum.dc); // Current enters at - terminal
            }

            for (harmonic, amplitude, phase) in spectrum.harmonics {
                if amplitude.abs() > 1e-30 {
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
    use crate::analysis::harmonic_balance::DC_SHORT_CONDUCTANCE;

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
}
