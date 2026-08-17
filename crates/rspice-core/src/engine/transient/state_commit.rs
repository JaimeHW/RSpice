//! Accepted-step reactive-history commit logic.

use super::*;

impl Engine {
    #[inline]
    fn install_cached_mosfet_gate_companion_charges(
        charges: &MosfetGateCompanionCharges,
        qgs: &mut Value,
        cqgs: &mut Value,
        qgd: &mut Value,
        cqgd: &mut Value,
        qgb: &mut Value,
        cqgb: &mut Value,
    ) {
        (*qgs, *cqgs) = charges[0];
        (*qgd, *cqgd) = charges[1];
        (*qgb, *cqgb) = charges[2];
    }

    #[inline]
    pub(super) fn update_reactive_history(
        &self,
        circuit: &mut crate::circuit::CircuitData,
        accepted_solution: &[Value],
        accepted_time: Value,
        dt: Value,
        coeff: &CompanionCoefficients,
        bsim4_trnqs_coeff: &CompanionCoefficients,
        bjt_history: &mut BjtTransientHistory,
        jfet_history: &mut JfetTransientHistory,
        diode_history: &mut DiodeTransientHistory,
        mosfet_history: &mut MosfetTransientHistory,
        vdmos_history: &mut VdmosTransientHistory,
        b3soi_history: &mut B3SoiTransientHistory,
        bsim3_history: &mut Bsim3TransientHistory,
        bsim4_history: &mut Bsim4TransientHistory,
        ekv26_history: &mut Ekv26TransientHistory,
        xyce_one_step_order2: bool,
        vbic_snapshots: Option<&[Option<BjtChargeSnapshot>]>,
        capacitor_accepted_states: Option<&[CapacitorAcceptedState]>,
        mosfet_caps: Option<&[(Value, Value, Value)]>,
        mosfet_gate_companion_charges: Option<&[MosfetGateCompanionCharges]>,
        suppress_gate_charge_history: bool,
        tline_dc_refs: &[(Value, Value)],
        coupled_tline_refs: &[CoupledTlineReferenceState],
        breakpoints: &mut BreakpointManager,
        tstop: Value,
        voltage_reltol: Value,
        voltage_abstol: Value,
        current_abstol: Value,
        dynamic_breakpoints_added: &mut usize,
        warned_dynamic_breakpoint_cap: &mut bool,
    ) -> Result<(), SimulationError> {
        let num_nodes = circuit.num_nodes();
        let capacitor_accepted_states = capacitor_accepted_states
            .filter(|states| states.len() == circuit.capacitors.stamps.len());
        for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
            if circuit
                .capacitors
                .value_expressions
                .get(cap_idx)
                .and_then(Option::as_ref)
                .is_some()
            {
                continue;
            }
            let (v_new, i_new) = if let Some(states) = capacitor_accepted_states {
                let state = states[cap_idx];
                (state.voltage, state.current)
            } else {
                let np = cap.pp.row;
                let nn = cap.nn.row;
                let v_new = Self::differential_voltage(accepted_solution, np, nn);

                // An IC capacitor's MNA branch is its physical lead current
                // and is numerically authoritative. Ordinary Norton
                // companions have no branch, so reconstruct those from OLD
                // history before rotating it.
                let i_new =
                    if let Some(branch_ordinal) = circuit.capacitors.ic_branch_indices[cap_idx] {
                        accepted_solution[num_nodes + branch_ordinal - 1]
                    } else {
                        let geq = coeff.capacitor_geq(circuit.capacitors.capacitances[cap_idx], dt);
                        let ieq = coeff.capacitor_ieq(
                            circuit.capacitors.capacitances[cap_idx],
                            dt,
                            circuit.capacitors.v_prev[cap_idx],
                            circuit.capacitors.v_prev_prev[cap_idx],
                            circuit.capacitors.i_prev[cap_idx],
                        );
                        geq * v_new - ieq
                    };
                (v_new, i_new)
            };

            let v_old = circuit.capacitors.v_prev[cap_idx];
            circuit.capacitors.v_prev_prev_prev[cap_idx] = circuit.capacitors.v_prev_prev[cap_idx];
            circuit.capacitors.v_prev_prev[cap_idx] = v_old;
            circuit.capacitors.v_prev[cap_idx] = v_new;
            circuit.capacitors.i_prev[cap_idx] = i_new;
        }
        circuit
            .capacitors
            .update_solution_dependent_state_with_coefficients(
                accepted_solution,
                accepted_time,
                dt,
                coeff,
            );

        for l_idx in 0..circuit.inductors.names.len() {
            let br = circuit.inductors.branch_indices[l_idx];
            if br > 0 {
                let br_idx = circuit.num_nodes() + br - 1;
                let i_new = accepted_solution[br_idx];
                circuit.inductors.i_prev_prev_prev[l_idx] = circuit.inductors.i_prev_prev[l_idx];
                circuit.inductors.i_prev_prev[l_idx] = circuit.inductors.i_prev[l_idx];
                circuit.inductors.i_prev[l_idx] = i_new;

                let np = circuit.inductors.node_pos[l_idx];
                let nn = circuit.inductors.node_neg[l_idx];
                let v_new = Self::differential_voltage(accepted_solution, np, nn);
                circuit.inductors.v_prev[l_idx] = v_new;
            }
        }
        circuit.update_coupled_inductor_pair_state(accepted_solution);
        circuit.update_multi_winding_transformer_state(accepted_solution);
        circuit.refresh_jiles_atherton_inductances(accepted_solution);
        circuit.commit_xyce_core_inductances(accepted_solution, dt, xyce_one_step_order2);
        circuit.commit_accepted_nonlinear_state();
        circuit
            .behavioral_sources
            .accept_transient_step(accepted_solution, accepted_time);

        // Update transmission-line delayed-wave history from the accepted state.
        for (idx, tl) in circuit.tlines.iter_mut().enumerate() {
            let previous_forward = tl.launched_forward_wave();
            let previous_backward = tl.launched_backward_wave();
            let v1 = Self::differential_voltage(accepted_solution, tl.node1_pos, tl.node1_neg);
            let v2 = Self::differential_voltage(accepted_solution, tl.node2_pos, tl.node2_neg);
            if tl.is_zero_length_pass_through() {
                continue;
            }
            if let Some((br1, br2)) = tl.txl_branch_matrix_indices() {
                let i1 = accepted_solution.get(br1 - 1).copied().unwrap_or(0.0);
                let i2 = accepted_solution.get(br2 - 1).copied().unwrap_or(0.0);
                tl.accept_txl_history(accepted_time, v1, i1, v2, i2);
                continue;
            }
            if let Some((br1, br2)) = tl.ltra_branch_matrix_indices() {
                let i1 = accepted_solution.get(br1 - 1).copied().unwrap_or(0.0);
                let i2 = accepted_solution.get(br2 - 1).copied().unwrap_or(0.0);
                tl.update_history(accepted_time, v1, i1, v2, i2);
                if !tl.is_distributed_rc() {
                    if let Some(arrival) =
                        tl.ltra_derivative_breakpoint_arrival(voltage_reltol, current_abstol)
                    {
                        Self::schedule_dynamic_tline_breakpoint(
                            breakpoints,
                            arrival,
                            tstop,
                            dynamic_breakpoints_added,
                            warned_dynamic_breakpoint_cap,
                        );
                    }
                }
                tl.compact_ltra_history_if_straight();
                continue;
            }
            let (_v1_ref, _v2_ref) = tline_dc_refs.get(idx).copied().unwrap_or((0.0, 0.0));
            let response = tl.transient_port_response(accepted_time);
            let (i1_actual, i2_actual) = response.port_currents(v1, v2);
            tl.update_history(accepted_time, v1, i1_actual, v2, i2_actual);
            if tl.has_distributed_rlgc() {
                if let Some(arrival) =
                    tl.ltra_derivative_breakpoint_arrival(voltage_reltol, current_abstol)
                {
                    Self::schedule_dynamic_tline_breakpoint(
                        breakpoints,
                        arrival,
                        tstop,
                        dynamic_breakpoints_added,
                        warned_dynamic_breakpoint_cap,
                    );
                }
                tl.compact_ltra_history_if_straight();
            } else {
                Self::maybe_schedule_tline_arrival_breakpoint(
                    breakpoints,
                    accepted_time,
                    tl.delay(),
                    tstop,
                    previous_forward,
                    tl.launched_forward_wave(),
                    voltage_reltol,
                    voltage_abstol,
                    dynamic_breakpoints_added,
                    warned_dynamic_breakpoint_cap,
                );
                Self::maybe_schedule_tline_arrival_breakpoint(
                    breakpoints,
                    accepted_time,
                    tl.delay(),
                    tstop,
                    previous_backward,
                    tl.launched_backward_wave(),
                    voltage_reltol,
                    voltage_abstol,
                    dynamic_breakpoints_added,
                    warned_dynamic_breakpoint_cap,
                );
            }
        }

        for (idx, tl) in circuit.coupled_tlines.iter_mut().enumerate() {
            // Native (ngspice-faithful) lines advance their convolution state
            // from the accepted port voltages and branch currents, then skip the
            // modal Norton bookkeeping entirely (their transient response comes
            // from the branch-current convolution stamp).
            if tl.uses_native_runtime() {
                let near_physical = Self::differential_port_voltages(
                    accepted_solution,
                    &tl.near_nodes,
                    tl.near_ref,
                );
                let far_physical =
                    Self::differential_port_voltages(accepted_solution, &tl.far_nodes, tl.far_ref);
                let conductors = tl.conductors();
                let mut near_i = vec![0.0; conductors];
                let mut far_i = vec![0.0; conductors];
                if let Some(branches) = tl.native_branch_matrix_indices() {
                    for c in 0..conductors {
                        if let Some((b1, b2)) = branches.conductor(c) {
                            near_i[c] = accepted_solution.get(b1 - 1).copied().unwrap_or(0.0);
                            far_i[c] = accepted_solution.get(b2 - 1).copied().unwrap_or(0.0);
                        }
                    }
                }
                tl.native_commit_accepted(
                    accepted_time,
                    &near_physical,
                    &far_physical,
                    &near_i,
                    &far_i,
                );
                continue;
            }

            let previous_mode_launches = tl.launched_modal_waves().collect::<Vec<_>>();
            let refs = coupled_tline_refs.get(idx).cloned().unwrap_or_default();
            let near_physical =
                Self::differential_port_voltages(accepted_solution, &tl.near_nodes, tl.near_ref);
            let far_physical =
                Self::differential_port_voltages(accepted_solution, &tl.far_nodes, tl.far_ref);
            let near_modal = tl.modalize_port_voltage(&near_physical);
            let far_modal = tl.modalize_port_voltage(&far_physical);
            let incoming_near = tl.incoming_near_modal(accepted_time, &refs.far_modal);
            let incoming_far = tl.incoming_far_modal(accepted_time, &refs.near_modal);
            let near_currents = tl.port_currents(&near_physical, &incoming_near);
            let far_currents = tl.port_currents(&far_physical, &incoming_far);
            let near_modal_currents = tl.modalize_port_current(&near_currents);
            let far_modal_currents = tl.modalize_port_current(&far_currents);
            tl.update_modal_history(
                accepted_time,
                &near_modal,
                &near_modal_currents,
                &far_modal,
                &far_modal_currents,
            );
            for (
                (delay, previous_forward, previous_backward),
                (_, current_forward, current_backward),
            ) in previous_mode_launches
                .into_iter()
                .zip(tl.launched_modal_waves())
            {
                Self::maybe_schedule_tline_arrival_breakpoint(
                    breakpoints,
                    accepted_time,
                    delay,
                    tstop,
                    previous_forward,
                    current_forward,
                    voltage_reltol,
                    voltage_abstol,
                    dynamic_breakpoints_added,
                    warned_dynamic_breakpoint_cap,
                );
                Self::maybe_schedule_tline_arrival_breakpoint(
                    breakpoints,
                    accepted_time,
                    delay,
                    tstop,
                    previous_backward,
                    current_backward,
                    voltage_reltol,
                    voltage_abstol,
                    dynamic_breakpoints_added,
                    warned_dynamic_breakpoint_cap,
                );
            }
        }

        for (idx, bjt) in circuit.bjts.devices.iter().enumerate() {
            let vc = Self::node_voltage(accepted_solution, bjt.node_collector);
            let vb = Self::node_voltage(accepted_solution, bjt.node_base);
            let ve = Self::node_voltage(accepted_solution, bjt.node_emitter);
            let vs = Self::node_voltage(accepted_solution, bjt.node_substrate);
            let external = [vc, vb, ve, vs];
            let vbe = vb - ve;
            let vbc = vb - vc;
            let vcs = vc - vs;
            if bjt.vbic_mna_promoted() {
                // Promoted VBIC: the accepted solution already carries the
                // internal node voltages, so the charge history commits from
                // a direct evaluation at the accepted bias.
                let (branches, internal, _) =
                    bjt.vbic_mna_charge_state_at_solution(accepted_solution);
                for (branch_idx, branch) in branches.iter().enumerate() {
                    let q_prev = bjt_history.charge_q_prev[idx][branch_idx];
                    let q_prev_prev = bjt_history.charge_q_prev_prev[idx][branch_idx];
                    let cq_prev = bjt_history.charge_cq_prev[idx][branch_idx];
                    let cq_curr = Self::jfet_companion_ccap(
                        coeff,
                        dt,
                        branch.charge,
                        q_prev,
                        q_prev_prev,
                        cq_prev,
                    );
                    bjt_history.charge_q_prev_prev_prev[idx][branch_idx] = q_prev_prev;
                    bjt_history.charge_q_prev_prev[idx][branch_idx] = q_prev;
                    bjt_history.charge_q_prev[idx][branch_idx] = branch.charge;
                    bjt_history.charge_cq_prev[idx][branch_idx] = cq_curr;
                }
                bjt_history.dynamic_internal_prev_prev[idx] =
                    bjt_history.dynamic_internal_prev[idx];
                bjt_history.dynamic_internal_prev[idx] = internal;
                bjt_history.vbe_prev_prev[idx] = bjt_history.vbe_prev[idx];
                bjt_history.vbe_prev[idx] = vbe;
                bjt_history.ibe_prev[idx] = 0.0;
                bjt_history.vbc_prev_prev[idx] = bjt_history.vbc_prev[idx];
                bjt_history.vbc_prev[idx] = vbc;
                bjt_history.ibc_prev[idx] = 0.0;
                bjt_history.vcs_prev_prev[idx] = bjt_history.vcs_prev[idx];
                bjt_history.vcs_prev[idx] = vcs;
                bjt_history.ics_prev[idx] = 0.0;
                continue;
            }
            let snapshot_reuse_abstol = voltage_abstol.min(VBIC_HISTORY_SNAPSHOT_REUSE_ABSTOL);
            let snapshot_reuse_reltol = voltage_reltol.min(VBIC_HISTORY_SNAPSHOT_REUSE_RELTOL);
            let cached_snapshot = vbic_snapshots
                .and_then(|cache| cache.get(idx))
                .copied()
                .flatten();
            let Some(snapshot) = Self::resolve_vbic_snapshot_for_external_bias_with_linear_history(
                bjt,
                external,
                coeff,
                dt,
                &bjt_history.charge_q_prev[idx],
                &bjt_history.charge_q_prev_prev[idx],
                &bjt_history.charge_cq_prev[idx],
                bjt_history.dynamic_internal_prev.get(idx),
                bjt_history.dynamic_internal_prev_prev.get(idx),
                bjt_history.dynamic_linear_prev.get(idx),
                bjt_history.dynamic_linear_prev_prev.get(idx),
                bjt_history.accepted_dt_prev,
                cached_snapshot,
                VbicCachedSnapshotReuse::SeedOnly,
                snapshot_reuse_abstol,
                snapshot_reuse_reltol,
            ) else {
                continue;
            };
            let (legacy_vbe, legacy_vbc, legacy_vbx, legacy_vcs) =
                Self::legacy_bjt_charge_branch_voltages_with_vbx(&snapshot);
            let legacy_charges = bjt.legacy_transient_charge_state_with_vbx(
                legacy_vbe, legacy_vbc, legacy_vbx, legacy_vcs,
            );
            let mut charge_values = snapshot.branches.map(|branch| branch.charge);
            charge_values[BJT_QBE_BRANCH_INDEX] = legacy_charges.qbe;
            charge_values[BJT_QBC_BRANCH_INDEX] = legacy_charges.qbc;
            charge_values[BJT_QBCX_BRANCH_INDEX] = legacy_charges.qbx;
            charge_values[BJT_QBCP_BRANCH_INDEX] = legacy_charges.qcs;
            let mut cq_currents = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
            for branch_idx in 0..BJT_DYNAMIC_CHARGE_COUNT {
                let charge = charge_values[branch_idx];
                let q_prev = bjt_history.charge_q_prev[idx][branch_idx];
                let q_prev_prev = bjt_history.charge_q_prev_prev[idx][branch_idx];
                let cq_prev = bjt_history.charge_cq_prev[idx][branch_idx];
                let cq_curr =
                    Self::jfet_companion_ccap(coeff, dt, charge, q_prev, q_prev_prev, cq_prev);
                bjt_history.charge_q_prev_prev_prev[idx][branch_idx] = q_prev_prev;
                bjt_history.charge_q_prev_prev[idx][branch_idx] = q_prev;
                bjt_history.charge_q_prev[idx][branch_idx] = charge;
                bjt_history.charge_cq_prev[idx][branch_idx] = cq_curr;
                cq_currents[branch_idx] = cq_curr;
            }
            bjt_history.dynamic_internal_prev_prev[idx] = bjt_history.dynamic_internal_prev[idx];
            bjt_history.dynamic_internal_prev[idx] = snapshot.reduction.internal_voltages;
            let predictor_linear = Self::vbic_predictor_linear_branch_state(
                bjt,
                external,
                snapshot.reduction.internal_voltages,
            );
            bjt_history.dynamic_linear_prev_prev[idx] = bjt_history.dynamic_linear_prev[idx];
            bjt_history.dynamic_linear_prev[idx] = predictor_linear;

            bjt_history.vbe_prev_prev[idx] = bjt_history.vbe_prev[idx];
            bjt_history.vbe_prev[idx] = legacy_vbe;
            bjt_history.ibe_prev[idx] = cq_currents[BJT_QBE_BRANCH_INDEX];
            bjt_history.vbc_prev_prev[idx] = bjt_history.vbc_prev[idx];
            bjt_history.vbc_prev[idx] = legacy_vbc;
            bjt_history.ibc_prev[idx] = cq_currents[BJT_QBC_BRANCH_INDEX];
            bjt_history.vcs_prev_prev[idx] = bjt_history.vcs_prev[idx];
            bjt_history.vcs_prev[idx] = legacy_vcs;
            bjt_history.ics_prev[idx] = cq_currents[BJT_QBCP_BRANCH_INDEX];
        }

        bjt_history.accepted_dt_prev_prev = bjt_history.accepted_dt_prev;
        bjt_history.accepted_dt_prev = dt;

        for (idx, jfet) in circuit.jfets.iter().enumerate() {
            let (vgs_eval, vgd_eval) = Self::jfet_branch_voltages(jfet, accepted_solution);
            let (vgs_charge, vgd_charge) =
                Self::jfet_charge_branch_voltages(jfet, accepted_solution);
            let (vgstrap, vgdtrap, power) = jfet.jfet2_next_transient_memory(
                vgs_eval,
                vgd_eval,
                jfet_history.jfet2_vgstrap_prev[idx],
                jfet_history.jfet2_vgdtrap_prev[idx],
                jfet_history.jfet2_power_prev[idx],
                dt,
            );
            let jfet2_charge = jfet.analytic_gate_charge_state(
                vgs_eval,
                vgd_eval,
                jfet.analysis_temperature(),
                Some((
                    jfet_history.vgs_prev[idx],
                    jfet_history.vgd_prev[idx],
                    jfet_history.qgs_prev[idx],
                    jfet_history.qgd_prev[idx],
                )),
            );
            let (cgs, cgd) = jfet2_charge
                .map(|charge| (charge.cgs, charge.cgd))
                .unwrap_or_else(|| {
                    jfet.transient_capacitances(vgs_eval, vgd_eval, jfet.analysis_temperature())
                });
            let cds = jfet.transient_drain_source_capacitance();
            let vds_charge = vgs_eval - vgd_eval;
            jfet_history.jfet2_vgstrap_prev[idx] = vgstrap;
            jfet_history.jfet2_vgdtrap_prev[idx] = vgdtrap;
            jfet_history.jfet2_power_prev[idx] = power;
            jfet_history.vgs_prev_prev[idx] = jfet_history.vgs_prev[idx];
            jfet_history.vgs_prev[idx] = vgs_charge;
            jfet_history.vgd_prev_prev[idx] = jfet_history.vgd_prev[idx];
            jfet_history.vgd_prev[idx] = vgd_charge;
            jfet_history.vds_prev_prev[idx] = jfet_history.vds_prev[idx];
            jfet_history.vds_prev[idx] = vds_charge;
            if !suppress_gate_charge_history {
                let (_geq_gs, _ieq_gs, qgs_curr, cqgs_curr) = if let Some(charge) = jfet2_charge {
                    Self::nonlinear_charge_companion_terms(
                        coeff,
                        dt,
                        cgs,
                        vgs_charge,
                        charge.qgs,
                        jfet_history.qgs_prev[idx],
                        jfet_history.qgs_prev_prev[idx],
                        jfet_history.cqgs_prev[idx],
                    )
                } else {
                    Self::jfet_companion_terms(
                        coeff,
                        dt,
                        cgs,
                        vgs_charge,
                        jfet_history.vgs_prev_prev[idx],
                        jfet_history.qgs_prev[idx],
                        jfet_history.qgs_prev_prev[idx],
                        jfet_history.cqgs_prev[idx],
                    )
                };
                jfet_history.qgs_prev_prev_prev[idx] = jfet_history.qgs_prev_prev[idx];
                jfet_history.qgs_prev_prev[idx] = jfet_history.qgs_prev[idx];
                jfet_history.qgs_prev[idx] = qgs_curr;
                jfet_history.cqgs_prev[idx] = cqgs_curr;

                let (_geq_gd, _ieq_gd, qgd_curr, cqgd_curr) = if let Some(charge) = jfet2_charge {
                    Self::nonlinear_charge_companion_terms(
                        coeff,
                        dt,
                        cgd,
                        vgd_charge,
                        charge.qgd,
                        jfet_history.qgd_prev[idx],
                        jfet_history.qgd_prev_prev[idx],
                        jfet_history.cqgd_prev[idx],
                    )
                } else {
                    Self::jfet_companion_terms(
                        coeff,
                        dt,
                        cgd,
                        vgd_charge,
                        jfet_history.vgd_prev_prev[idx],
                        jfet_history.qgd_prev[idx],
                        jfet_history.qgd_prev_prev[idx],
                        jfet_history.cqgd_prev[idx],
                    )
                };
                jfet_history.qgd_prev_prev_prev[idx] = jfet_history.qgd_prev_prev[idx];
                jfet_history.qgd_prev_prev[idx] = jfet_history.qgd_prev[idx];
                jfet_history.qgd_prev[idx] = qgd_curr;
                jfet_history.cqgd_prev[idx] = cqgd_curr;
            }
            if cds.is_finite() && cds > 0.0 {
                let (_geq_ds, _ieq_ds, qds_curr, cqds_curr) = Self::jfet_companion_terms(
                    coeff,
                    dt,
                    cds,
                    vds_charge,
                    jfet_history.vds_prev_prev[idx],
                    jfet_history.qds_prev[idx],
                    jfet_history.qds_prev_prev[idx],
                    jfet_history.cqds_prev[idx],
                );
                jfet_history.qds_prev_prev_prev[idx] = jfet_history.qds_prev_prev[idx];
                jfet_history.qds_prev_prev[idx] = jfet_history.qds_prev[idx];
                jfet_history.qds_prev[idx] = qds_curr;
                jfet_history.cqds_prev[idx] = cqds_curr;
            }
        }
        jfet_history.accepted_dt_prev_prev = jfet_history.accepted_dt_prev;
        jfet_history.accepted_dt_prev = dt;

        for (idx, diode) in circuit.diodes.devices.iter().enumerate() {
            let vd =
                Self::differential_voltage(accepted_solution, diode.node_anode, diode.node_cathode);
            diode_history.vd_prev_prev[idx] = diode_history.vd_prev[idx];
            diode_history.vd_prev[idx] = vd;
            let (qd, capd) = diode.junction_charge_and_capacitance(vd);
            if capd.is_finite() && capd > 0.0 {
                let (_geq, _ieq, qd_curr, cqd_curr) = Self::nonlinear_charge_companion_terms(
                    coeff,
                    dt,
                    capd,
                    vd,
                    qd,
                    diode_history.qd_prev[idx],
                    diode_history.qd_prev_prev[idx],
                    diode_history.cqd_prev[idx],
                );
                diode_history.qd_prev_prev_prev[idx] = diode_history.qd_prev_prev[idx];
                diode_history.qd_prev_prev[idx] = diode_history.qd_prev[idx];
                diode_history.qd_prev[idx] = qd_curr;
                diode_history.cqd_prev[idx] = cqd_curr;
            }
        }
        diode_history.accepted_dt_prev_prev = diode_history.accepted_dt_prev;
        diode_history.accepted_dt_prev = dt;

        // Rotate whole accepted-state generations once, as ngspice rotates
        // CKTstate pointers, instead of copying two history levels for every
        // instance and branch. The old `prev_prev` buffers become scratch for
        // the new accepted values; arithmetic below still reads the identical
        // old `prev` and `prev_prev` generations.
        mosfet_history.rotate_gate_generations(suppress_gate_charge_history);
        let mosfet_gate_companion_charges = mosfet_gate_companion_charges
            .filter(|charges| charges.len() == circuit.mosfets.devices.len());

        // Every instance owns one disjoint element in each history vector, so
        // the model arithmetic can run in parallel without reductions or a
        // change in floating-point operation order. Rayon MultiZip stops at
        // the shortest input; reject a broken internal shape explicitly so a
        // release build can never commit only a prefix of device history.
        #[cfg(feature = "parallel")]
        let mosfet_history_updated_in_parallel = {
            use rayon::prelude::*;

            let instance_count = circuit.mosfets.devices.len();
            if let Some(worker_count) = self.classic_mos_parallel_worker_count(instance_count) {
                let history_shapes_match = [
                    &mosfet_history.vgs_prev,
                    &mosfet_history.vgs_prev_prev,
                    &mosfet_history.capgs_prev_half,
                    &mosfet_history.qgs_prev,
                    &mosfet_history.qgs_prev_prev,
                    &mosfet_history.qgs_prev_prev_prev,
                    &mosfet_history.cqgs_prev,
                    &mosfet_history.vgd_prev,
                    &mosfet_history.vgd_prev_prev,
                    &mosfet_history.capgd_prev_half,
                    &mosfet_history.qgd_prev,
                    &mosfet_history.qgd_prev_prev,
                    &mosfet_history.qgd_prev_prev_prev,
                    &mosfet_history.cqgd_prev,
                    &mosfet_history.vgb_prev,
                    &mosfet_history.vgb_prev_prev,
                    &mosfet_history.capgb_prev_half,
                    &mosfet_history.qgb_prev,
                    &mosfet_history.qgb_prev_prev,
                    &mosfet_history.qgb_prev_prev_prev,
                    &mosfet_history.cqgb_prev,
                    &mosfet_history.vbs_j_prev,
                    &mosfet_history.vbs_j_prev_prev,
                    &mosfet_history.qbs_prev,
                    &mosfet_history.qbs_prev_prev,
                    &mosfet_history.cqbs_prev,
                    &mosfet_history.vbd_j_prev,
                    &mosfet_history.vbd_j_prev_prev,
                    &mosfet_history.qbd_prev,
                    &mosfet_history.qbd_prev_prev,
                    &mosfet_history.cqbd_prev,
                ]
                .into_iter()
                .all(|history| history.len() == instance_count);
                let caps_shape_matches =
                    mosfet_caps.is_none_or(|capacitances| capacitances.len() == instance_count);
                let gate_charges_shape_matches = mosfet_gate_companion_charges
                    .is_none_or(|charges| charges.len() == instance_count);
                if !history_shapes_match || !caps_shape_matches || !gate_charges_shape_matches {
                    return Err(SimulationError::Circuit(
                        "classic-MOS transient history shape does not match the device population"
                            .to_string(),
                    ));
                }
                let chunk_size = instance_count.div_ceil(worker_count).max(1);
                let devices = circuit.mosfets.devices.as_slice();
                let vgs_prev_prev = mosfet_history.vgs_prev_prev.as_slice();
                let qgs_prev_prev = mosfet_history.qgs_prev_prev.as_slice();
                let qgs_prev_prev_prev = mosfet_history.qgs_prev_prev_prev.as_slice();
                let vgd_prev_prev = mosfet_history.vgd_prev_prev.as_slice();
                let qgd_prev_prev = mosfet_history.qgd_prev_prev.as_slice();
                let qgd_prev_prev_prev = mosfet_history.qgd_prev_prev_prev.as_slice();
                let vgb_prev_prev = mosfet_history.vgb_prev_prev.as_slice();
                let qgb_prev_prev = mosfet_history.qgb_prev_prev.as_slice();
                let qgb_prev_prev_prev = mosfet_history.qgb_prev_prev_prev.as_slice();

                let gate_outputs = (
                    mosfet_history.vgs_prev.as_mut_slice(),
                    mosfet_history.capgs_prev_half.as_mut_slice(),
                    mosfet_history.qgs_prev.as_mut_slice(),
                    mosfet_history.cqgs_prev.as_mut_slice(),
                    mosfet_history.vgd_prev.as_mut_slice(),
                    mosfet_history.capgd_prev_half.as_mut_slice(),
                    mosfet_history.qgd_prev.as_mut_slice(),
                    mosfet_history.cqgd_prev.as_mut_slice(),
                    mosfet_history.vgb_prev.as_mut_slice(),
                    mosfet_history.capgb_prev_half.as_mut_slice(),
                    mosfet_history.qgb_prev.as_mut_slice(),
                    mosfet_history.cqgb_prev.as_mut_slice(),
                )
                    .into_par_iter();
                let body_outputs = (
                    mosfet_history.vbs_j_prev.as_mut_slice(),
                    mosfet_history.vbs_j_prev_prev.as_mut_slice(),
                    mosfet_history.qbs_prev.as_mut_slice(),
                    mosfet_history.qbs_prev_prev.as_mut_slice(),
                    mosfet_history.cqbs_prev.as_mut_slice(),
                    mosfet_history.vbd_j_prev.as_mut_slice(),
                    mosfet_history.vbd_j_prev_prev.as_mut_slice(),
                    mosfet_history.qbd_prev.as_mut_slice(),
                    mosfet_history.qbd_prev_prev.as_mut_slice(),
                    mosfet_history.cqbd_prev.as_mut_slice(),
                )
                    .into_par_iter();

                self.install_classic_mos_parallel(|| {
                    gate_outputs
                        .zip(body_outputs)
                        .with_min_len(chunk_size)
                        .enumerate()
                        .for_each(
                            |(
                                idx,
                                (
                                    (
                                        vgs_out,
                                        capgs_out,
                                        qgs_out,
                                        cqgs_out,
                                        vgd_out,
                                        capgd_out,
                                        qgd_out,
                                        cqgd_out,
                                        vgb_out,
                                        capgb_out,
                                        qgb_out,
                                        cqgb_out,
                                    ),
                                    (
                                        vbs_j_out,
                                        vbs_j_prev_out,
                                        qbs_out,
                                        qbs_prev_out,
                                        cqbs_out,
                                        vbd_j_out,
                                        vbd_j_prev_out,
                                        qbd_out,
                                        qbd_prev_out,
                                        cqbd_out,
                                    ),
                                ),
                            )| {
                                let mos = &devices[idx];
                                let (vgs, vds, vbs) =
                                    mos.eval_branch_voltages_at(accepted_solution);
                                let vgd = vgs - vds;
                                let vgb = vgs - vbs;
                                let (cgs_half, cgd_half, cgb_half) = match mosfet_caps {
                                    Some(cache) => cache[idx],
                                    None => mos.transient_capacitance_halves_at(vgs, vds, vbs),
                                };
                                let previous_cap_halves = (*capgs_out, *capgd_out, *capgb_out);
                                *vgs_out = vgs;
                                *capgs_out = cgs_half;
                                *vgd_out = vgd;
                                *capgd_out = cgd_half;
                                *vgb_out = vgb;
                                *capgb_out = cgb_half;
                                if !suppress_gate_charge_history {
                                    if let Some(charges) = mosfet_gate_companion_charges {
                                        Self::install_cached_mosfet_gate_companion_charges(
                                            &charges[idx],
                                            qgs_out,
                                            cqgs_out,
                                            qgd_out,
                                            cqgd_out,
                                            qgb_out,
                                            cqgb_out,
                                        );
                                    } else {
                                        let (cgs_ov, cgd_ov, cgb_ov) = mos.overlap_capacitances();
                                        let cgs = cgs_half + previous_cap_halves.0 + cgs_ov;
                                        let cgd = cgd_half + previous_cap_halves.1 + cgd_ov;
                                        let cgb = cgb_half + previous_cap_halves.2 + cgb_ov;
                                        let (_geq, _ieq, q_curr, cq_curr) =
                                            Self::jfet_companion_terms(
                                                coeff,
                                                dt,
                                                cgs,
                                                vgs,
                                                vgs_prev_prev[idx],
                                                qgs_prev_prev[idx],
                                                qgs_prev_prev_prev[idx],
                                                *cqgs_out,
                                            );
                                        *qgs_out = q_curr;
                                        *cqgs_out = cq_curr;

                                        let (_geq, _ieq, q_curr, cq_curr) =
                                            Self::jfet_companion_terms(
                                                coeff,
                                                dt,
                                                cgd,
                                                vgd,
                                                vgd_prev_prev[idx],
                                                qgd_prev_prev[idx],
                                                qgd_prev_prev_prev[idx],
                                                *cqgd_out,
                                            );
                                        *qgd_out = q_curr;
                                        *cqgd_out = cq_curr;

                                        let (_geq, _ieq, q_curr, cq_curr) =
                                            Self::jfet_companion_terms(
                                                coeff,
                                                dt,
                                                cgb,
                                                vgb,
                                                vgb_prev_prev[idx],
                                                qgb_prev_prev[idx],
                                                qgb_prev_prev_prev[idx],
                                                *cqgb_out,
                                            );
                                        *qgb_out = q_curr;
                                        *cqgb_out = cq_curr;
                                    }
                                }

                                let body_charge_mask = mos.body_junction_charge_mask();
                                if body_charge_mask & 1 != 0 {
                                    let vbs_j = mos.body_source_charge_branch_voltage(vbs);
                                    let (q_exact, capacitance) =
                                        mos.body_source_junction_charge_and_capacitance_at(vbs);
                                    let (_geq, _ieq, q_curr, cq_curr) =
                                        Self::nonlinear_charge_companion_terms(
                                            coeff,
                                            dt,
                                            capacitance,
                                            vbs_j,
                                            q_exact,
                                            *qbs_out,
                                            *qbs_prev_out,
                                            *cqbs_out,
                                        );
                                    *vbs_j_prev_out = *vbs_j_out;
                                    *vbs_j_out = vbs_j;
                                    *qbs_prev_out = *qbs_out;
                                    *qbs_out = q_curr;
                                    *cqbs_out = cq_curr;
                                }

                                if body_charge_mask & 2 != 0 {
                                    let vbd_j = mos.body_drain_charge_branch_voltage(vds, vbs);
                                    let (q_exact, capacitance) =
                                        mos.body_drain_junction_charge_and_capacitance_at(vds, vbs);
                                    let (_geq, _ieq, q_curr, cq_curr) =
                                        Self::nonlinear_charge_companion_terms(
                                            coeff,
                                            dt,
                                            capacitance,
                                            vbd_j,
                                            q_exact,
                                            *qbd_out,
                                            *qbd_prev_out,
                                            *cqbd_out,
                                        );
                                    *vbd_j_prev_out = *vbd_j_out;
                                    *vbd_j_out = vbd_j;
                                    *qbd_prev_out = *qbd_out;
                                    *qbd_out = q_curr;
                                    *cqbd_out = cq_curr;
                                }
                            },
                        );
                })?;
                true
            } else {
                false
            }
        };
        #[cfg(not(feature = "parallel"))]
        let mosfet_history_updated_in_parallel = false;

        let serial_mosfet_devices = if mosfet_history_updated_in_parallel {
            &[][..]
        } else {
            circuit.mosfets.devices.as_slice()
        };
        for (idx, mos) in serial_mosfet_devices.iter().enumerate() {
            let (vgs, vds, vbs) = mos.eval_branch_voltages_at(accepted_solution);
            let vgd = vgs - vds;
            let vgb = vgs - vbs;
            // The truncation walk already evaluated the Meyer halves on this
            // accepted solution; reuse them when the caller captured them.
            let (cgs_half, cgd_half, cgb_half) = match mosfet_caps {
                Some(cache) => cache[idx],
                None => mos.transient_capacitance_halves_at(vgs, vds, vbs),
            };
            let previous_cap_halves = (
                mosfet_history.capgs_prev_half[idx],
                mosfet_history.capgd_prev_half[idx],
                mosfet_history.capgb_prev_half[idx],
            );
            mosfet_history.vgs_prev[idx] = vgs;
            mosfet_history.capgs_prev_half[idx] = cgs_half;
            mosfet_history.vgd_prev[idx] = vgd;
            mosfet_history.capgd_prev_half[idx] = cgd_half;
            mosfet_history.vgb_prev[idx] = vgb;
            mosfet_history.capgb_prev_half[idx] = cgb_half;
            if !suppress_gate_charge_history {
                if let Some(charges) = mosfet_gate_companion_charges {
                    Self::install_cached_mosfet_gate_companion_charges(
                        &charges[idx],
                        &mut mosfet_history.qgs_prev[idx],
                        &mut mosfet_history.cqgs_prev[idx],
                        &mut mosfet_history.qgd_prev[idx],
                        &mut mosfet_history.cqgd_prev[idx],
                        &mut mosfet_history.qgb_prev[idx],
                        &mut mosfet_history.cqgb_prev[idx],
                    );
                } else {
                    let (cgs_ov, cgd_ov, cgb_ov) = mos.overlap_capacitances();
                    let cgs = cgs_half + previous_cap_halves.0 + cgs_ov;
                    let cgd = cgd_half + previous_cap_halves.1 + cgd_ov;
                    let cgb = cgb_half + previous_cap_halves.2 + cgb_ov;
                    let (_geq_gs, _ieq_gs, qgs_curr, cqgs_curr) = Self::jfet_companion_terms(
                        coeff,
                        dt,
                        cgs,
                        vgs,
                        mosfet_history.vgs_prev_prev[idx],
                        mosfet_history.qgs_prev_prev[idx],
                        mosfet_history.qgs_prev_prev_prev[idx],
                        mosfet_history.cqgs_prev[idx],
                    );
                    mosfet_history.qgs_prev[idx] = qgs_curr;
                    mosfet_history.cqgs_prev[idx] = cqgs_curr;

                    let (_geq_gd, _ieq_gd, qgd_curr, cqgd_curr) = Self::jfet_companion_terms(
                        coeff,
                        dt,
                        cgd,
                        vgd,
                        mosfet_history.vgd_prev_prev[idx],
                        mosfet_history.qgd_prev_prev[idx],
                        mosfet_history.qgd_prev_prev_prev[idx],
                        mosfet_history.cqgd_prev[idx],
                    );
                    mosfet_history.qgd_prev[idx] = qgd_curr;
                    mosfet_history.cqgd_prev[idx] = cqgd_curr;

                    let (_geq_gb, _ieq_gb, qgb_curr, cqgb_curr) = Self::jfet_companion_terms(
                        coeff,
                        dt,
                        cgb,
                        vgb,
                        mosfet_history.vgb_prev_prev[idx],
                        mosfet_history.qgb_prev_prev[idx],
                        mosfet_history.qgb_prev_prev_prev[idx],
                        mosfet_history.cqgb_prev[idx],
                    );
                    mosfet_history.qgb_prev[idx] = qgb_curr;
                    mosfet_history.cqgb_prev[idx] = cqgb_curr;
                }
            }

            let body_charge_mask = mos.body_junction_charge_mask();
            if body_charge_mask & 1 != 0 {
                let vbs_j = mos.body_source_charge_branch_voltage(vbs);
                let (qbs_exact, cbs) = mos.body_source_junction_charge_and_capacitance_at(vbs);
                let (_geq_bs, _ieq_bs, qbs_curr, cqbs_curr) =
                    Self::nonlinear_charge_companion_terms(
                        coeff,
                        dt,
                        cbs,
                        vbs_j,
                        qbs_exact,
                        mosfet_history.qbs_prev[idx],
                        mosfet_history.qbs_prev_prev[idx],
                        mosfet_history.cqbs_prev[idx],
                    );
                mosfet_history.vbs_j_prev_prev[idx] = mosfet_history.vbs_j_prev[idx];
                mosfet_history.vbs_j_prev[idx] = vbs_j;
                mosfet_history.qbs_prev_prev[idx] = mosfet_history.qbs_prev[idx];
                mosfet_history.qbs_prev[idx] = qbs_curr;
                mosfet_history.cqbs_prev[idx] = cqbs_curr;
            }

            if body_charge_mask & 2 != 0 {
                let vbd_j = mos.body_drain_charge_branch_voltage(vds, vbs);
                let (qbd_exact, cbd) = mos.body_drain_junction_charge_and_capacitance_at(vds, vbs);
                let (_geq_bd, _ieq_bd, qbd_curr, cqbd_curr) =
                    Self::nonlinear_charge_companion_terms(
                        coeff,
                        dt,
                        cbd,
                        vbd_j,
                        qbd_exact,
                        mosfet_history.qbd_prev[idx],
                        mosfet_history.qbd_prev_prev[idx],
                        mosfet_history.cqbd_prev[idx],
                    );
                mosfet_history.vbd_j_prev_prev[idx] = mosfet_history.vbd_j_prev[idx];
                mosfet_history.vbd_j_prev[idx] = vbd_j;
                mosfet_history.qbd_prev_prev[idx] = mosfet_history.qbd_prev[idx];
                mosfet_history.qbd_prev[idx] = qbd_curr;
                mosfet_history.cqbd_prev[idx] = cqbd_curr;
            }
        }
        mosfet_history.accepted_dt_prev_prev = mosfet_history.accepted_dt_prev;
        mosfet_history.accepted_dt_prev = dt;

        for (idx, vdmos) in circuit.vdmoses.devices.iter().enumerate() {
            let (vgs, vgd, vgb, vds) = vdmos.transient_charge_branch_voltages_at(accepted_solution);
            let vd1 = vdmos.d1_charge_branch_voltage_at(accepted_solution);
            let (vbs, vbd) = vdmos.body_charge_branch_voltages_at(accepted_solution);
            let (cgs, cgd, cds) = vdmos.capacitances(vgs, vds);
            let cgb = vdmos.gate_bulk_capacitance();
            let (qbs_exact, cbs) = vdmos.body_source_transient_charge_and_capacitance_at(vbs);
            let (qbd_exact, cbd) = vdmos.body_drain_transient_charge_and_capacitance_at(vbd);
            let (qd1_exact, cd1) = vdmos.d1_charge_and_capacitance_at(vd1);

            vdmos_history.vgs_prev_prev[idx] = vdmos_history.vgs_prev[idx];
            vdmos_history.vgs_prev[idx] = vgs;
            let (_geq_gs, _ieq_gs, qgs_curr, cqgs_curr) = Self::jfet_companion_terms(
                coeff,
                dt,
                cgs,
                vgs,
                vdmos_history.vgs_prev_prev[idx],
                vdmos_history.qgs_prev[idx],
                vdmos_history.qgs_prev_prev[idx],
                vdmos_history.cqgs_prev[idx],
            );
            vdmos_history.qgs_prev_prev_prev[idx] = vdmos_history.qgs_prev_prev[idx];
            vdmos_history.qgs_prev_prev[idx] = vdmos_history.qgs_prev[idx];
            vdmos_history.qgs_prev[idx] = qgs_curr;
            vdmos_history.cqgs_prev[idx] = cqgs_curr;

            vdmos_history.vgd_prev_prev[idx] = vdmos_history.vgd_prev[idx];
            vdmos_history.vgd_prev[idx] = vgd;
            let (_geq_gd, _ieq_gd, qgd_curr, cqgd_curr) = Self::jfet_companion_terms(
                coeff,
                dt,
                cgd,
                vgd,
                vdmos_history.vgd_prev_prev[idx],
                vdmos_history.qgd_prev[idx],
                vdmos_history.qgd_prev_prev[idx],
                vdmos_history.cqgd_prev[idx],
            );
            vdmos_history.qgd_prev_prev_prev[idx] = vdmos_history.qgd_prev_prev[idx];
            vdmos_history.qgd_prev_prev[idx] = vdmos_history.qgd_prev[idx];
            vdmos_history.qgd_prev[idx] = qgd_curr;
            vdmos_history.cqgd_prev[idx] = cqgd_curr;

            vdmos_history.vgb_prev_prev[idx] = vdmos_history.vgb_prev[idx];
            vdmos_history.vgb_prev[idx] = vgb;
            let (_geq_gb, _ieq_gb, qgb_curr, cqgb_curr) = Self::jfet_companion_terms(
                coeff,
                dt,
                cgb,
                vgb,
                vdmos_history.vgb_prev_prev[idx],
                vdmos_history.qgb_prev[idx],
                vdmos_history.qgb_prev_prev[idx],
                vdmos_history.cqgb_prev[idx],
            );
            vdmos_history.qgb_prev_prev_prev[idx] = vdmos_history.qgb_prev_prev[idx];
            vdmos_history.qgb_prev_prev[idx] = vdmos_history.qgb_prev[idx];
            vdmos_history.qgb_prev[idx] = qgb_curr;
            vdmos_history.cqgb_prev[idx] = cqgb_curr;

            vdmos_history.vds_prev_prev[idx] = vdmos_history.vds_prev[idx];
            vdmos_history.vds_prev[idx] = vds;
            let (_geq_ds, _ieq_ds, qds_curr, cqds_curr) = Self::jfet_companion_terms(
                coeff,
                dt,
                cds,
                vds,
                vdmos_history.vds_prev_prev[idx],
                vdmos_history.qds_prev[idx],
                vdmos_history.qds_prev_prev[idx],
                vdmos_history.cqds_prev[idx],
            );
            vdmos_history.qds_prev_prev_prev[idx] = vdmos_history.qds_prev_prev[idx];
            vdmos_history.qds_prev_prev[idx] = vdmos_history.qds_prev[idx];
            vdmos_history.qds_prev[idx] = qds_curr;
            vdmos_history.cqds_prev[idx] = cqds_curr;

            vdmos_history.vbs_prev_prev[idx] = vdmos_history.vbs_prev[idx];
            vdmos_history.vbs_prev[idx] = vbs;
            let (_geq_bs, _ieq_bs, qbs_curr, cqbs_curr) = Self::nonlinear_charge_companion_terms(
                coeff,
                dt,
                cbs,
                vbs,
                qbs_exact,
                vdmos_history.qbs_prev[idx],
                vdmos_history.qbs_prev_prev[idx],
                vdmos_history.cqbs_prev[idx],
            );
            vdmos_history.qbs_prev_prev_prev[idx] = vdmos_history.qbs_prev_prev[idx];
            vdmos_history.qbs_prev_prev[idx] = vdmos_history.qbs_prev[idx];
            vdmos_history.qbs_prev[idx] = qbs_curr;
            vdmos_history.cqbs_prev[idx] = cqbs_curr;

            vdmos_history.vbd_prev_prev[idx] = vdmos_history.vbd_prev[idx];
            vdmos_history.vbd_prev[idx] = vbd;
            let (_geq_bd, _ieq_bd, qbd_curr, cqbd_curr) = Self::nonlinear_charge_companion_terms(
                coeff,
                dt,
                cbd,
                vbd,
                qbd_exact,
                vdmos_history.qbd_prev[idx],
                vdmos_history.qbd_prev_prev[idx],
                vdmos_history.cqbd_prev[idx],
            );
            vdmos_history.qbd_prev_prev_prev[idx] = vdmos_history.qbd_prev_prev[idx];
            vdmos_history.qbd_prev_prev[idx] = vdmos_history.qbd_prev[idx];
            vdmos_history.qbd_prev[idx] = qbd_curr;
            vdmos_history.cqbd_prev[idx] = cqbd_curr;

            vdmos_history.vd1_prev_prev[idx] = vdmos_history.vd1_prev[idx];
            vdmos_history.vd1_prev[idx] = vd1;
            let (_geq_d1, _ieq_d1, qd1_curr, cqd1_curr) = Self::nonlinear_charge_companion_terms(
                coeff,
                dt,
                cd1,
                vd1,
                qd1_exact,
                vdmos_history.qd1_prev[idx],
                vdmos_history.qd1_prev_prev[idx],
                vdmos_history.cqd1_prev[idx],
            );
            vdmos_history.qd1_prev_prev_prev[idx] = vdmos_history.qd1_prev_prev[idx];
            vdmos_history.qd1_prev_prev[idx] = vdmos_history.qd1_prev[idx];
            vdmos_history.qd1_prev[idx] = qd1_curr;
            vdmos_history.cqd1_prev[idx] = cqd1_curr;
        }
        vdmos_history.accepted_dt_prev_prev = vdmos_history.accepted_dt_prev;
        vdmos_history.accepted_dt_prev = dt;

        Self::update_b3soi_history(circuit, accepted_solution, coeff, dt, b3soi_history);
        Self::update_bsim3_history(circuit, accepted_solution, coeff, dt, bsim3_history);
        Self::update_bsim4_history(
            circuit,
            accepted_solution,
            coeff,
            bsim4_trnqs_coeff,
            dt,
            bsim4_history,
        );
        Self::update_ekv26_history(circuit, accepted_solution, coeff, dt, ekv26_history);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cached_mosfet_gate_companion_charges_install_bit_exactly() {
        let charges = [
            (1.25e-15, -2.5e-6),
            (-3.75e-15, 4.5e-6),
            (5.5e-15, -6.25e-6),
        ];
        let mut qgs = Value::NAN;
        let mut cqgs = Value::NAN;
        let mut qgd = Value::NAN;
        let mut cqgd = Value::NAN;
        let mut qgb = Value::NAN;
        let mut cqgb = Value::NAN;

        Engine::install_cached_mosfet_gate_companion_charges(
            &charges, &mut qgs, &mut cqgs, &mut qgd, &mut cqgd, &mut qgb, &mut cqgb,
        );

        assert_eq!(
            [qgs, cqgs, qgd, cqgd, qgb, cqgb].map(Value::to_bits),
            [
                charges[0].0,
                charges[0].1,
                charges[1].0,
                charges[1].1,
                charges[2].0,
                charges[2].1,
            ]
            .map(Value::to_bits)
        );
    }
}
