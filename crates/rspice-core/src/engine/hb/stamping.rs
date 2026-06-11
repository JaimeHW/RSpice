use super::*;

impl Engine {
    pub(in crate::engine::hb) fn hb_stamp_supported_nonlinear_devices(
        &self,
        circuit: &CircuitData,
        solver: &mut HbSolver,
        num_nodes: usize,
    ) {
        use crate::analysis::advanced::harmonic_balance::{DepletionCap, NonlinearDeviceInstance};

        for diode in &circuit.diodes.devices {
            let anode = Self::hb_node_to_solver_index(diode.node_anode, num_nodes);
            let cathode = Self::hb_node_to_solver_index(diode.node_cathode, num_nodes);
            solver.add_nonlinear_device(
                NonlinearDeviceInstance::diode(anode, cathode, diode.is, diode.n)
                    .with_junction_caps(
                        DepletionCap::new(diode.cj0, diode.vj, diode.m, 0.5),
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
            solver.add_nonlinear_device(instance.with_junction_caps(
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
            match mos.mos_type {
                crate::device::MosType::Nmos => {
                    solver.add_nmos(drain, gate, source, bulk, kp, mos.vto, mos.lambda);
                }
                crate::device::MosType::Pmos => {
                    // The solver works in the polarity frame: the effective
                    // threshold is -VTO, which keeps depletion PMOS negative.
                    solver.add_pmos(drain, gate, source, bulk, kp, -mos.vto, mos.lambda);
                }
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
            solver.add_nonlinear_device(instance.with_junction_caps(
                DepletionCap::new(jfet.params.cgs, jfet.params.pb, jfet.params.m, 0.5),
                DepletionCap::new(jfet.params.cgd, jfet.params.pb, jfet.params.m, 0.5),
                0.0,
                0.0,
            ));
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
        drive_tones: &[HbDriveTone],
    ) {
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
            let (ac_mag, ac_phase) = Self::hb_source_drive_terms(ac_mag, ac_phase, spec);
            let source_name = circuit
                .voltage_sources
                .names
                .get(i)
                .map(|name| name.as_str())
                .unwrap_or("");
            let harmonics = Self::hb_drive_harmonics_for_source(drive_tones, source_name);
            if ac_mag.abs() <= 1e-30 || harmonics.is_empty() {
                solver.add_voltage_source_branch(np, nn, dc);
                continue;
            }

            let harmonic_terms: Vec<(usize, Value, Value)> = harmonics
                .iter()
                .copied()
                .map(|harmonic| (harmonic, ac_mag, ac_phase))
                .collect();
            solver.add_voltage_source_branch_harmonics(np, nn, dc, &harmonic_terms);
        }
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
        drive_tones: &[HbDriveTone],
    ) {
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
            let (ac_mag, ac_phase) = Self::hb_source_drive_terms(ac_mag, ac_phase, spec);
            let source_name = circuit
                .voltage_sources
                .names
                .get(i)
                .map(|name| name.as_str())
                .unwrap_or("");
            let harmonics = Self::hb_drive_harmonics_for_source(drive_tones, source_name);

            self.hb_stamp_admittance(solver, np, nn, HB_NORTON_G, true);

            // Norton conversion of `V` in series with Rs = 1/G: a current source
            // G*V injecting INTO the positive node, in parallel with G. The
            // independent-current-source convention (current pulled out of the
            // positive node) does not apply here.
            let i_dc = dc * HB_NORTON_G;
            if np > 0 {
                solver.add_dc_source(np - 1, i_dc);
            }
            if nn > 0 {
                solver.add_dc_source(nn - 1, -i_dc);
            }

            let i_ac = ac_mag * HB_NORTON_G;
            if i_ac.abs() > 1e-30 && !harmonics.is_empty() {
                for harmonic in harmonics {
                    if np > 0 {
                        solver.add_harmonic_source(np - 1, harmonic, i_ac, ac_phase);
                    }
                    if nn > 0 {
                        solver.add_harmonic_source(nn - 1, harmonic, -i_ac, ac_phase);
                    }
                }
            }
        }
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
        drive_tones: &[HbDriveTone],
    ) {
        for i in 0..circuit.current_sources.len() {
            let np = circuit.current_sources.node_pos[i];
            let nn = circuit.current_sources.node_neg[i];
            let dc = circuit.current_sources.dc_values[i];

            // Stamp DC component (harmonic 0)
            if np > 0 {
                solver.add_dc_source(np - 1, -dc); // Current leaves at + terminal
            }
            if nn > 0 {
                solver.add_dc_source(nn - 1, dc); // Current enters at - terminal
            }

            // Stamp AC component across configured drive harmonics.
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
            let (ac_mag, ac_phase) = Self::hb_source_drive_terms(ac_mag, ac_phase, spec);
            let source_name = circuit
                .current_sources
                .names
                .get(i)
                .map(|name| name.as_str())
                .unwrap_or("");
            let harmonics = Self::hb_drive_harmonics_for_source(drive_tones, source_name);

            if ac_mag.abs() > 1e-30 {
                for harmonic in harmonics {
                    if np > 0 {
                        // Current leaves at + terminal.
                        solver.add_harmonic_source(np - 1, harmonic, -ac_mag, ac_phase);
                    }
                    if nn > 0 {
                        // Current enters at - terminal.
                        solver.add_harmonic_source(nn - 1, harmonic, ac_mag, ac_phase);
                    }
                }
            }
        }
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
