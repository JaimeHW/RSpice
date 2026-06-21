//! Reactive companion state and transient recovery helpers.

#![allow(clippy::needless_range_loop)]

use super::*;

impl Engine {
    #[inline]
    pub(super) fn legacy_bjt_charge_branch_voltages(
        snapshot: &BjtChargeSnapshot,
    ) -> (Value, Value, Value) {
        let (vbe, vbc, _vbx, vcs) = Self::legacy_bjt_charge_branch_voltages_with_vbx(snapshot);
        (vbe, vbc, vcs)
    }

    #[inline]
    pub(super) fn legacy_bjt_charge_branch_voltages_with_vbx(
        snapshot: &BjtChargeSnapshot,
    ) -> (Value, Value, Value, Value) {
        let internal = &snapshot.reduction.internal_voltages;
        (
            internal[BJT_VBI_STATE_INDEX] - internal[BJT_VEI_STATE_INDEX],
            internal[BJT_VBI_STATE_INDEX] - internal[BJT_VCI_STATE_INDEX],
            Self::legacy_bjt_charge_branch_voltage(
                snapshot,
                &snapshot.branches[BJT_QBCX_BRANCH_INDEX],
            ),
            Self::legacy_bjt_charge_branch_voltage(
                snapshot,
                &snapshot.branches[BJT_QBCP_BRANCH_INDEX],
            ),
        )
    }

    #[inline]
    pub(super) fn legacy_bjt_charge_branch_voltage(
        snapshot: &BjtChargeSnapshot,
        branch: &BjtChargeBranch,
    ) -> Value {
        Self::legacy_bjt_terminal_voltage(snapshot, branch.pos_internal, branch.pos_external)
            - Self::legacy_bjt_terminal_voltage(snapshot, branch.neg_internal, branch.neg_external)
    }

    #[inline]
    fn legacy_bjt_terminal_voltage(
        snapshot: &BjtChargeSnapshot,
        internal: Option<usize>,
        external: Option<usize>,
    ) -> Value {
        if let Some(idx) = internal {
            snapshot.reduction.internal_voltages[idx]
        } else if let Some(idx) = external {
            snapshot.reduction.external_voltages[idx]
        } else {
            0.0
        }
    }

    /// Breakpoint-style integration restart after a floor-dt livelock.
    ///
    /// Re-seeds every reactive history from the accepted solution exactly
    /// like transient startup (flat capacitor/inductor history, zeroed
    /// capacitor current, maxstep-seeded dt chains) so the truncation
    /// estimators stop differencing the poisoned floor-dt trail. The
    /// transmission-line delay buffers are deliberately left alone — they
    /// hold genuine propagating state, not integrator history.
    pub(super) fn reseed_reactive_histories_for_restart(
        circuit: &mut crate::circuit::Circuit,
        solution: &[Value],
        hinted_max_step: Value,
        bjt_history: &mut BjtTransientHistory,
        jfet_history: &mut JfetTransientHistory,
        diode_history: &mut DiodeTransientHistory,
        mosfet_history: &mut MosfetTransientHistory,
        vdmos_history: &mut VdmosTransientHistory,
        b3soi_history: &mut B3SoiTransientHistory,
        bsim3_history: &mut Bsim3TransientHistory,
        bsim4_history: &mut Bsim4TransientHistory,
    ) {
        for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
            let v = Self::differential_voltage(solution, cap.pp.row, cap.nn.row);
            circuit.capacitors.v_prev[cap_idx] = v;
            circuit.capacitors.v_prev_prev[cap_idx] = v;
            circuit.capacitors.v_prev_prev_prev[cap_idx] = v;
            circuit.capacitors.i_prev[cap_idx] = 0.0;
        }

        for l_idx in 0..circuit.inductors.names.len() {
            let np = circuit.inductors.node_pos[l_idx];
            let nn = circuit.inductors.node_neg[l_idx];
            let v = if np == 0 { 0.0 } else { solution[np - 1] }
                - if nn == 0 { 0.0 } else { solution[nn - 1] };
            circuit.inductors.v_prev[l_idx] = v;
            let br = circuit.inductors.branch_indices[l_idx];
            if br > 0 {
                let br_idx = circuit.num_nodes() + br - 1;
                let i = solution.get(br_idx).copied().unwrap_or(0.0);
                circuit.inductors.i_prev[l_idx] = i;
                circuit.inductors.i_prev_prev[l_idx] = i;
            }
        }
        circuit.update_coupled_inductor_pair_state(solution);
        circuit.update_multi_winding_transformer_state(solution);

        *bjt_history = Self::initialize_bjt_history(circuit, solution);
        bjt_history.accepted_dt_prev = hinted_max_step;
        bjt_history.accepted_dt_prev_prev = hinted_max_step;
        *jfet_history = Self::initialize_jfet_history(circuit, solution);
        jfet_history.accepted_dt_prev = hinted_max_step;
        jfet_history.accepted_dt_prev_prev = hinted_max_step;
        *diode_history = Self::initialize_diode_history(circuit, solution);
        diode_history.accepted_dt_prev = hinted_max_step;
        diode_history.accepted_dt_prev_prev = hinted_max_step;
        *mosfet_history = Self::initialize_mosfet_history(circuit, solution);
        mosfet_history.accepted_dt_prev = hinted_max_step;
        mosfet_history.accepted_dt_prev_prev = hinted_max_step;
        *vdmos_history = Self::initialize_vdmos_history(circuit, solution);
        vdmos_history.accepted_dt_prev = hinted_max_step;
        vdmos_history.accepted_dt_prev_prev = hinted_max_step;
        *b3soi_history = Self::initialize_b3soi_history(circuit, solution);
        *bsim3_history = Self::initialize_bsim3_history(circuit, solution);
        bsim3_history.accepted_dt_prev = hinted_max_step;
        bsim3_history.accepted_dt_prev_prev = hinted_max_step;
        *bsim4_history = Self::initialize_bsim4_history(circuit, solution);
        bsim4_history.accepted_dt_prev = hinted_max_step;
        bsim4_history.accepted_dt_prev_prev = hinted_max_step;
    }

    #[inline]
    pub(super) fn initialize_bjt_history(
        circuit: &crate::circuit::Circuit,
        solution: &[Value],
    ) -> BjtTransientHistory {
        let n = circuit.bjts.devices.len();
        let mut history = BjtTransientHistory {
            vbe_prev: Vec::with_capacity(n),
            vbe_prev_prev: Vec::with_capacity(n),
            ibe_prev: Vec::with_capacity(n),
            vbc_prev: Vec::with_capacity(n),
            vbc_prev_prev: Vec::with_capacity(n),
            ibc_prev: Vec::with_capacity(n),
            vcs_prev: Vec::with_capacity(n),
            vcs_prev_prev: Vec::with_capacity(n),
            ics_prev: Vec::with_capacity(n),
            charge_q_prev: Vec::with_capacity(n),
            charge_q_prev_prev: Vec::with_capacity(n),
            charge_q_prev_prev_prev: Vec::with_capacity(n),
            charge_cq_prev: Vec::with_capacity(n),
            dynamic_internal_prev: Vec::with_capacity(n),
            dynamic_internal_prev_prev: Vec::with_capacity(n),
            dynamic_linear_prev: Vec::with_capacity(n),
            dynamic_linear_prev_prev: Vec::with_capacity(n),
            accepted_dt_prev: 0.0,
            accepted_dt_prev_prev: 0.0,
        };

        for bjt in &circuit.bjts.devices {
            let vc = Self::node_voltage(solution, bjt.node_collector);
            let vb = Self::node_voltage(solution, bjt.node_base);
            let ve = Self::node_voltage(solution, bjt.node_emitter);
            let vs = Self::node_voltage(solution, bjt.node_substrate);
            let vbe = vb - ve;
            let vbc = vb - vc;
            let vcs = vc - vs;

            if bjt.vbic_mna_promoted() {
                // Promoted VBIC: the internal states are part of the solved
                // operating point, so the charge history seeds directly from
                // the solution vector with no nested snapshot solve.
                let (branches, internal, _) = bjt.vbic_mna_charge_state_at_solution(solution);
                let charge_values = branches.map(|branch| branch.charge);
                history.vbe_prev.push(vbe);
                history.vbe_prev_prev.push(vbe);
                history.ibe_prev.push(0.0);
                history.vbc_prev.push(vbc);
                history.vbc_prev_prev.push(vbc);
                history.ibc_prev.push(0.0);
                history.vcs_prev.push(vcs);
                history.vcs_prev_prev.push(vcs);
                history.ics_prev.push(0.0);
                history.charge_q_prev.push(charge_values);
                history.charge_q_prev_prev.push(charge_values);
                history.charge_q_prev_prev_prev.push(charge_values);
                history.charge_cq_prev.push([0.0; BJT_DYNAMIC_CHARGE_COUNT]);
                history.dynamic_internal_prev.push(internal);
                history.dynamic_internal_prev_prev.push(internal);
                history
                    .dynamic_linear_prev
                    .push(VbicPredictorLinearBranchState::default());
                history
                    .dynamic_linear_prev_prev
                    .push(VbicPredictorLinearBranchState::default());
                continue;
            }

            let charge_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
            let (history_vbe, history_vbc, history_vcs) =
                Self::legacy_bjt_charge_branch_voltages(&charge_snapshot);
            history.vbe_prev.push(history_vbe);
            history.vbe_prev_prev.push(history_vbe);
            history.ibe_prev.push(0.0);
            history.vbc_prev.push(history_vbc);
            history.vbc_prev_prev.push(history_vbc);
            history.ibc_prev.push(0.0);
            history.vcs_prev.push(history_vcs);
            history.vcs_prev_prev.push(history_vcs);
            history.ics_prev.push(0.0);

            let mut charge_values = charge_snapshot.branches.map(|branch| branch.charge);
            let (legacy_vbe, legacy_vbc, legacy_vbx, legacy_vcs) =
                Self::legacy_bjt_charge_branch_voltages_with_vbx(&charge_snapshot);
            let charges = bjt.legacy_transient_charge_state_with_vbx(
                legacy_vbe, legacy_vbc, legacy_vbx, legacy_vcs,
            );
            charge_values[BJT_QBE_BRANCH_INDEX] = charges.qbe;
            charge_values[BJT_QBC_BRANCH_INDEX] = charges.qbc;
            charge_values[BJT_QBCX_BRANCH_INDEX] = charges.qbx;
            charge_values[BJT_QBCP_BRANCH_INDEX] = charges.qcs;
            let predictor_linear = Self::vbic_predictor_linear_branch_state(
                bjt,
                [vc, vb, ve, vs],
                charge_snapshot.reduction.internal_voltages,
            );
            history.charge_q_prev.push(charge_values);
            history.charge_q_prev_prev.push(charge_values);
            history.charge_q_prev_prev_prev.push(charge_values);
            history.charge_cq_prev.push([0.0; BJT_DYNAMIC_CHARGE_COUNT]);
            history
                .dynamic_internal_prev
                .push(charge_snapshot.reduction.internal_voltages);
            history
                .dynamic_internal_prev_prev
                .push(charge_snapshot.reduction.internal_voltages);
            history.dynamic_linear_prev.push(predictor_linear);
            history.dynamic_linear_prev_prev.push(predictor_linear);
        }

        history
    }

    #[inline]
    pub(super) fn initialize_jfet_history(
        circuit: &crate::circuit::Circuit,
        solution: &[Value],
    ) -> JfetTransientHistory {
        let n = circuit.jfets.len();
        let mut history = JfetTransientHistory {
            vgs_prev: Vec::with_capacity(n),
            vgs_prev_prev: Vec::with_capacity(n),
            qgs_prev: Vec::with_capacity(n),
            qgs_prev_prev: Vec::with_capacity(n),
            qgs_prev_prev_prev: Vec::with_capacity(n),
            cqgs_prev: Vec::with_capacity(n),
            vgd_prev: Vec::with_capacity(n),
            vgd_prev_prev: Vec::with_capacity(n),
            qgd_prev: Vec::with_capacity(n),
            qgd_prev_prev: Vec::with_capacity(n),
            qgd_prev_prev_prev: Vec::with_capacity(n),
            cqgd_prev: Vec::with_capacity(n),
            vds_prev: Vec::with_capacity(n),
            vds_prev_prev: Vec::with_capacity(n),
            qds_prev: Vec::with_capacity(n),
            qds_prev_prev: Vec::with_capacity(n),
            qds_prev_prev_prev: Vec::with_capacity(n),
            cqds_prev: Vec::with_capacity(n),
            jfet2_vgstrap_prev: Vec::with_capacity(n),
            jfet2_vgdtrap_prev: Vec::with_capacity(n),
            jfet2_power_prev: Vec::with_capacity(n),
            accepted_dt_prev: 0.0,
            accepted_dt_prev_prev: 0.0,
        };

        for jfet in &circuit.jfets {
            let (vgs_eval, vgd_eval) = Self::jfet_branch_voltages(jfet, solution);
            let (vgs_charge, vgd_charge) = Self::jfet_charge_branch_voltages(jfet, solution);
            let jfet2_charge = jfet.analytic_gate_charge_state(
                vgs_eval,
                vgd_eval,
                jfet.analysis_temperature(),
                None,
            );
            let (cgs, cgd) = jfet2_charge
                .map(|charge| (charge.cgs, charge.cgd))
                .unwrap_or_else(|| {
                    jfet.transient_capacitances(vgs_eval, vgd_eval, jfet.analysis_temperature())
                });
            let cds = jfet.transient_drain_source_capacitance();
            let vds_charge = vgs_eval - vgd_eval;
            let qgs = jfet2_charge
                .map(|charge| charge.qgs)
                .unwrap_or_else(|| cgs.max(0.0) * vgs_charge);
            let qgd = jfet2_charge
                .map(|charge| charge.qgd)
                .unwrap_or_else(|| cgd.max(0.0) * vgd_charge);
            let qds = cds.max(0.0) * vds_charge;
            let (_, _, power) =
                jfet.jfet2_next_transient_memory(vgs_eval, vgd_eval, vgs_eval, vgd_eval, 0.0, 0.0);
            history.vgs_prev.push(vgs_charge);
            history.vgs_prev_prev.push(vgs_charge);
            history.qgs_prev.push(qgs);
            history.qgs_prev_prev.push(qgs);
            history.qgs_prev_prev_prev.push(qgs);
            history.cqgs_prev.push(0.0);
            history.vgd_prev.push(vgd_charge);
            history.vgd_prev_prev.push(vgd_charge);
            history.qgd_prev.push(qgd);
            history.qgd_prev_prev.push(qgd);
            history.qgd_prev_prev_prev.push(qgd);
            history.cqgd_prev.push(0.0);
            history.vds_prev.push(vds_charge);
            history.vds_prev_prev.push(vds_charge);
            history.qds_prev.push(qds);
            history.qds_prev_prev.push(qds);
            history.qds_prev_prev_prev.push(qds);
            history.cqds_prev.push(0.0);
            history.jfet2_vgstrap_prev.push(vgs_eval);
            history.jfet2_vgdtrap_prev.push(vgd_eval);
            history.jfet2_power_prev.push(power);
        }

        history
    }

    #[inline]
    pub(super) fn refresh_jfet2_transient_linearizations(
        circuit: &mut crate::circuit::Circuit,
        solution: &[Value],
        dt: Value,
        history: &JfetTransientHistory,
    ) {
        for (idx, jfet) in circuit.jfets.iter_mut().enumerate() {
            jfet.refresh_jfet2_transient_operating_terms(
                solution,
                history.jfet2_vgstrap_prev[idx],
                history.jfet2_vgdtrap_prev[idx],
                history.jfet2_power_prev[idx],
                dt,
            );
        }
    }

    pub(super) fn initialize_diode_history(
        circuit: &crate::circuit::Circuit,
        solution: &[Value],
    ) -> DiodeTransientHistory {
        let n = circuit.diodes.devices.len();
        let mut history = DiodeTransientHistory {
            vd_prev: Vec::with_capacity(n),
            vd_prev_prev: Vec::with_capacity(n),
            qd_prev: Vec::with_capacity(n),
            qd_prev_prev: Vec::with_capacity(n),
            qd_prev_prev_prev: Vec::with_capacity(n),
            cqd_prev: Vec::with_capacity(n),
            accepted_dt_prev: 0.0,
            accepted_dt_prev_prev: 0.0,
        };

        for diode in &circuit.diodes.devices {
            let vd = Self::differential_voltage(solution, diode.node_anode, diode.node_cathode);
            let (qd, _capd) = diode.junction_charge_and_capacitance(vd);
            history.vd_prev.push(vd);
            history.vd_prev_prev.push(vd);
            history.qd_prev.push(qd);
            history.qd_prev_prev.push(qd);
            history.qd_prev_prev_prev.push(qd);
            history.cqd_prev.push(0.0);
        }

        history
    }

    #[inline]
    pub(super) fn initialize_mosfet_history(
        circuit: &crate::circuit::Circuit,
        solution: &[Value],
    ) -> MosfetTransientHistory {
        let n = circuit.mosfets.len();
        let mut history = MosfetTransientHistory {
            vgs_prev: Vec::with_capacity(n),
            vgs_prev_prev: Vec::with_capacity(n),
            capgs_prev_half: Vec::with_capacity(n),
            qgs_prev: Vec::with_capacity(n),
            qgs_prev_prev: Vec::with_capacity(n),
            qgs_prev_prev_prev: Vec::with_capacity(n),
            cqgs_prev: Vec::with_capacity(n),
            vgd_prev: Vec::with_capacity(n),
            vgd_prev_prev: Vec::with_capacity(n),
            capgd_prev_half: Vec::with_capacity(n),
            qgd_prev: Vec::with_capacity(n),
            qgd_prev_prev: Vec::with_capacity(n),
            qgd_prev_prev_prev: Vec::with_capacity(n),
            cqgd_prev: Vec::with_capacity(n),
            vgb_prev: Vec::with_capacity(n),
            vgb_prev_prev: Vec::with_capacity(n),
            capgb_prev_half: Vec::with_capacity(n),
            qgb_prev: Vec::with_capacity(n),
            qgb_prev_prev: Vec::with_capacity(n),
            qgb_prev_prev_prev: Vec::with_capacity(n),
            cqgb_prev: Vec::with_capacity(n),
            vbs_j_prev: Vec::with_capacity(n),
            vbs_j_prev_prev: Vec::with_capacity(n),
            qbs_prev: Vec::with_capacity(n),
            qbs_prev_prev: Vec::with_capacity(n),
            cqbs_prev: Vec::with_capacity(n),
            vbd_j_prev: Vec::with_capacity(n),
            vbd_j_prev_prev: Vec::with_capacity(n),
            qbd_prev: Vec::with_capacity(n),
            qbd_prev_prev: Vec::with_capacity(n),
            cqbd_prev: Vec::with_capacity(n),
            accepted_dt_prev: 0.0,
            accepted_dt_prev_prev: 0.0,
        };

        for mos in &circuit.mosfets.devices {
            let (vgs, vds, vbs) = mos.eval_branch_voltages_at(solution);
            let vgd = vgs - vds;
            let vgb = vgs - vbs;
            let (cgs_half, cgd_half, cgb_half) = mos.transient_capacitance_halves_at(vgs, vds, vbs);
            let (cgs_ov, cgd_ov, cgb_ov) = mos.overlap_capacitances();
            let cgs = 2.0 * cgs_half + cgs_ov;
            let cgd = 2.0 * cgd_half + cgd_ov;
            let cgb = 2.0 * cgb_half + cgb_ov;

            history.vgs_prev.push(vgs);
            history.vgs_prev_prev.push(vgs);
            history.capgs_prev_half.push(cgs_half);
            history.qgs_prev.push(cgs.max(0.0) * vgs);
            history.qgs_prev_prev.push(cgs.max(0.0) * vgs);
            history.qgs_prev_prev_prev.push(cgs.max(0.0) * vgs);
            history.cqgs_prev.push(0.0);

            history.vgd_prev.push(vgd);
            history.vgd_prev_prev.push(vgd);
            history.capgd_prev_half.push(cgd_half);
            history.qgd_prev.push(cgd.max(0.0) * vgd);
            history.qgd_prev_prev.push(cgd.max(0.0) * vgd);
            history.qgd_prev_prev_prev.push(cgd.max(0.0) * vgd);
            history.cqgd_prev.push(0.0);

            history.vgb_prev.push(vgb);
            history.vgb_prev_prev.push(vgb);
            history.capgb_prev_half.push(cgb_half);
            history.qgb_prev.push(cgb.max(0.0) * vgb);
            history.qgb_prev_prev.push(cgb.max(0.0) * vgb);
            history.qgb_prev_prev_prev.push(cgb.max(0.0) * vgb);
            history.cqgb_prev.push(0.0);

            let vbs_j = mos.body_source_charge_branch_voltage(vbs);
            let vbd_j = mos.body_drain_charge_branch_voltage(vds, vbs);
            let (qbs, _) = mos.body_source_junction_charge_and_capacitance_at(vbs);
            let (qbd, _) = mos.body_drain_junction_charge_and_capacitance_at(vds, vbs);
            history.vbs_j_prev.push(vbs_j);
            history.vbs_j_prev_prev.push(vbs_j);
            history.qbs_prev.push(qbs);
            history.qbs_prev_prev.push(qbs);
            history.cqbs_prev.push(0.0);
            history.vbd_j_prev.push(vbd_j);
            history.vbd_j_prev_prev.push(vbd_j);
            history.qbd_prev.push(qbd);
            history.qbd_prev_prev.push(qbd);
            history.cqbd_prev.push(0.0);
        }

        history
    }

    #[inline]
    pub(super) fn initialize_vdmos_history(
        circuit: &crate::circuit::Circuit,
        solution: &[Value],
    ) -> VdmosTransientHistory {
        let n = circuit.vdmoses.len();
        let mut history = VdmosTransientHistory {
            vgs_prev: Vec::with_capacity(n),
            vgs_prev_prev: Vec::with_capacity(n),
            qgs_prev: Vec::with_capacity(n),
            qgs_prev_prev: Vec::with_capacity(n),
            qgs_prev_prev_prev: Vec::with_capacity(n),
            cqgs_prev: Vec::with_capacity(n),
            vgd_prev: Vec::with_capacity(n),
            vgd_prev_prev: Vec::with_capacity(n),
            qgd_prev: Vec::with_capacity(n),
            qgd_prev_prev: Vec::with_capacity(n),
            qgd_prev_prev_prev: Vec::with_capacity(n),
            cqgd_prev: Vec::with_capacity(n),
            vgb_prev: Vec::with_capacity(n),
            vgb_prev_prev: Vec::with_capacity(n),
            qgb_prev: Vec::with_capacity(n),
            qgb_prev_prev: Vec::with_capacity(n),
            qgb_prev_prev_prev: Vec::with_capacity(n),
            cqgb_prev: Vec::with_capacity(n),
            vds_prev: Vec::with_capacity(n),
            vds_prev_prev: Vec::with_capacity(n),
            qds_prev: Vec::with_capacity(n),
            qds_prev_prev: Vec::with_capacity(n),
            qds_prev_prev_prev: Vec::with_capacity(n),
            cqds_prev: Vec::with_capacity(n),
            vbs_prev: Vec::with_capacity(n),
            vbs_prev_prev: Vec::with_capacity(n),
            qbs_prev: Vec::with_capacity(n),
            qbs_prev_prev: Vec::with_capacity(n),
            qbs_prev_prev_prev: Vec::with_capacity(n),
            cqbs_prev: Vec::with_capacity(n),
            vbd_prev: Vec::with_capacity(n),
            vbd_prev_prev: Vec::with_capacity(n),
            qbd_prev: Vec::with_capacity(n),
            qbd_prev_prev: Vec::with_capacity(n),
            qbd_prev_prev_prev: Vec::with_capacity(n),
            cqbd_prev: Vec::with_capacity(n),
            vd1_prev: Vec::with_capacity(n),
            vd1_prev_prev: Vec::with_capacity(n),
            qd1_prev: Vec::with_capacity(n),
            qd1_prev_prev: Vec::with_capacity(n),
            qd1_prev_prev_prev: Vec::with_capacity(n),
            cqd1_prev: Vec::with_capacity(n),
            accepted_dt_prev: 0.0,
            accepted_dt_prev_prev: 0.0,
        };

        for vdmos in &circuit.vdmoses.devices {
            let (vgs, vgd, vgb, vds) = vdmos.transient_charge_branch_voltages_at(solution);
            let vd1 = vdmos.d1_charge_branch_voltage_at(solution);
            let (vbs, vbd) = vdmos.body_charge_branch_voltages_at(solution);
            let (cgs, cgd, cds) = vdmos.capacitances(vgs, vds);
            let cgb = vdmos.gate_bulk_capacitance();
            let (qbs, _) = vdmos.body_source_transient_charge_and_capacitance_at(vbs);
            let (qbd, _) = vdmos.body_drain_transient_charge_and_capacitance_at(vbd);
            let (qd1, _) = vdmos.d1_charge_and_capacitance_at(vd1);
            history.vgs_prev.push(vgs);
            history.vgs_prev_prev.push(vgs);
            history.qgs_prev.push(cgs.max(0.0) * vgs);
            history.qgs_prev_prev.push(cgs.max(0.0) * vgs);
            history.qgs_prev_prev_prev.push(cgs.max(0.0) * vgs);
            history.cqgs_prev.push(0.0);

            history.vgd_prev.push(vgd);
            history.vgd_prev_prev.push(vgd);
            history.qgd_prev.push(cgd.max(0.0) * vgd);
            history.qgd_prev_prev.push(cgd.max(0.0) * vgd);
            history.qgd_prev_prev_prev.push(cgd.max(0.0) * vgd);
            history.cqgd_prev.push(0.0);

            history.vgb_prev.push(vgb);
            history.vgb_prev_prev.push(vgb);
            history.qgb_prev.push(cgb.max(0.0) * vgb);
            history.qgb_prev_prev.push(cgb.max(0.0) * vgb);
            history.qgb_prev_prev_prev.push(cgb.max(0.0) * vgb);
            history.cqgb_prev.push(0.0);

            history.vds_prev.push(vds);
            history.vds_prev_prev.push(vds);
            history.qds_prev.push(cds.max(0.0) * vds);
            history.qds_prev_prev.push(cds.max(0.0) * vds);
            history.qds_prev_prev_prev.push(cds.max(0.0) * vds);
            history.cqds_prev.push(0.0);

            history.vbs_prev.push(vbs);
            history.vbs_prev_prev.push(vbs);
            history.qbs_prev.push(qbs);
            history.qbs_prev_prev.push(qbs);
            history.qbs_prev_prev_prev.push(qbs);
            history.cqbs_prev.push(0.0);

            history.vbd_prev.push(vbd);
            history.vbd_prev_prev.push(vbd);
            history.qbd_prev.push(qbd);
            history.qbd_prev_prev.push(qbd);
            history.qbd_prev_prev_prev.push(qbd);
            history.cqbd_prev.push(0.0);

            history.vd1_prev.push(vd1);
            history.vd1_prev_prev.push(vd1);
            history.qd1_prev.push(qd1);
            history.qd1_prev_prev.push(qd1);
            history.qd1_prev_prev_prev.push(qd1);
            history.cqd1_prev.push(0.0);
        }

        history
    }

    #[inline]
    pub(super) fn stamp_bjt_transient_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &BjtTransientHistory,
        vbic_snapshot_cache: &mut [Option<BjtChargeSnapshot>],
        cache_reuse: VbicCachedSnapshotReuse,
        voltage_abstol: Value,
        reltol: Value,
    ) {
        let effective_method = Self::effective_companion_method(method, trap_order);
        let charge_factor = Self::jfet_companion_geq(effective_method, trap_order, 1.0, dt);
        for (idx, bjt) in circuit.bjts.devices.iter().enumerate() {
            let vc = Self::node_voltage(voltages, bjt.node_collector);
            let vb = Self::node_voltage(voltages, bjt.node_base);
            let ve = Self::node_voltage(voltages, bjt.node_emitter);
            let vs = Self::node_voltage(voltages, bjt.node_substrate);

            if bjt.vbic_mna_promoted() {
                // Promoted VBIC: per-branch charge companions on the actual
                // internal nodes (ngspice NIintegrate discipline), evaluated
                // and linearized at the limited bias cached by the device
                // update for this Newton iterate.
                if charge_factor <= 0.0 {
                    continue;
                }
                let (branches, internal, external) = bjt.vbic_mna_charge_state();
                let mut stamper = StaticMatrixChargeStamper {
                    matrix: &mut *matrix,
                    rhs: &mut *rhs,
                };
                for (branch_idx, branch) in branches.iter().enumerate() {
                    if !branch.is_active() {
                        continue;
                    }
                    let cq = Self::jfet_companion_ccap(
                        effective_method,
                        trap_order,
                        dt,
                        branch.charge,
                        history.charge_q_prev[idx][branch_idx],
                        history.charge_q_prev_prev[idx][branch_idx],
                        history.charge_cq_prev[idx][branch_idx],
                    );
                    Self::stamp_vbic_mna_charge_branch(
                        &mut stamper,
                        bjt,
                        branch,
                        charge_factor,
                        cq,
                        &internal,
                        &external,
                    );
                }
                continue;
            }

            if charge_factor <= 0.0 {
                continue;
            }
            let (snapshot_reuse_abstol, snapshot_reuse_reltol) =
                Self::vbic_runtime_snapshot_reuse_tolerances(voltage_abstol, reltol);
            let cached_snapshot = vbic_snapshot_cache.get(idx).copied().flatten();
            let Some(snapshot) = Self::resolve_vbic_snapshot_for_external_bias_with_linear_history(
                bjt,
                [vc, vb, ve, vs],
                method,
                trap_order,
                dt,
                &history.charge_q_prev[idx],
                &history.charge_q_prev_prev[idx],
                &history.charge_cq_prev[idx],
                history.dynamic_internal_prev.get(idx),
                history.dynamic_internal_prev_prev.get(idx),
                history.dynamic_linear_prev.get(idx),
                history.dynamic_linear_prev_prev.get(idx),
                history.accepted_dt_prev,
                cached_snapshot,
                cache_reuse,
                snapshot_reuse_abstol,
                snapshot_reuse_reltol,
            ) else {
                vbic_snapshot_cache[idx] = None;
                continue;
            };

            let Some(linearization) = Self::assemble_vbic_transient_linearization(
                bjt,
                &snapshot,
                effective_method,
                trap_order,
                dt,
                &history.charge_q_prev[idx],
                &history.charge_q_prev_prev[idx],
                &history.charge_cq_prev[idx],
            ) else {
                vbic_snapshot_cache[idx] = None;
                continue;
            };
            let (base_static_g, base_static_i_eq) =
                Self::vbic_static_stamped_external_system(bjt, &[vc, vb, ve, vs]);
            vbic_snapshot_cache[idx] = Some(snapshot);
            let Some((y_total, reduced_i_eq)) =
                Self::vbic_reduce_transient_external_system(&linearization)
            else {
                vbic_snapshot_cache[idx] = None;
                continue;
            };

            let mut delta = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
            let mut delta_i_eq = [0.0; BJT_EXTERNAL_STATE_DIM];
            for row in 0..BJT_EXTERNAL_STATE_DIM {
                delta_i_eq[row] = reduced_i_eq[row] - base_static_i_eq[row];
                for col in 0..BJT_EXTERNAL_STATE_DIM {
                    delta[row][col] = y_total[row][col] - base_static_g[row][col];
                }
            }
            let nodes = [
                bjt.node_collector,
                bjt.node_base,
                bjt.node_emitter,
                bjt.node_substrate,
            ];
            Self::stamp_external_reduced_system(matrix, rhs, &nodes, &delta, &delta_i_eq);
        }
    }

    /// Stamp one promoted VBIC charge branch as a Norton companion on its
    /// actual matrix nodes. Charge branches use the standard MNA orientation:
    /// the integrated current `cq` leaves the positive node and enters the
    /// negative node, with conductance `ag0 * dq/dv` across every coupled
    /// column and the linearization point folded into the source term.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn stamp_vbic_mna_charge_branch(
        stamper: &mut impl crate::device::MatrixStamper,
        bjt: &crate::device::Bjt,
        branch: &BjtChargeBranch,
        ag0: Value,
        cq: Value,
        internal: &[Value; BJT_INTERNAL_STATE_DIM],
        external: &[Value; BJT_EXTERNAL_STATE_DIM],
    ) {
        let external_nodes = [
            bjt.node_collector,
            bjt.node_base,
            bjt.node_emitter,
            bjt.node_substrate,
        ];
        let mut source = -cq;
        for col in 0..BJT_INTERNAL_STATE_DIM {
            source += ag0 * branch.d_internal[col] * internal[col];
        }
        for col in 0..BJT_EXTERNAL_STATE_DIM {
            source += ag0 * branch.d_external[col] * external[col];
        }

        let mut stamp_row = |row: crate::circuit::NodeId, sign: Value| {
            if row == 0 {
                return;
            }
            for col in 0..BJT_INTERNAL_STATE_DIM {
                let g = ag0 * branch.d_internal[col];
                if g != 0.0 {
                    stamper.stamp(row, bjt.vbic_internal_node(col), sign * g);
                }
            }
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                let g = ag0 * branch.d_external[col];
                if g != 0.0 {
                    stamper.stamp(row, external_nodes[col], sign * g);
                }
            }
            stamper.stamp_rhs(row, sign * source);
        };

        let pos = branch
            .pos_internal
            .map(|idx| bjt.vbic_internal_node(idx))
            .or_else(|| branch.pos_external.map(|idx| external_nodes[idx]));
        let neg = branch
            .neg_internal
            .map(|idx| bjt.vbic_internal_node(idx))
            .or_else(|| branch.neg_external.map(|idx| external_nodes[idx]));
        if let Some(row) = pos {
            stamp_row(row, 1.0);
        }
        if let Some(row) = neg {
            stamp_row(row, -1.0);
        }
    }

    #[inline]
    pub(super) fn stamp_jfet_transient_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &JfetTransientHistory,
        suppress_gate_charge: bool,
    ) {
        let effective_method = Self::effective_companion_method(method, trap_order);
        for (idx, jfet) in circuit.jfets.iter().enumerate() {
            let (vgs_eval, vgd_eval) = Self::jfet_branch_voltages(jfet, voltages);
            let (vgs_charge, vgd_charge) = Self::jfet_charge_branch_voltages(jfet, voltages);
            let jfet2_charge = jfet.analytic_gate_charge_state(
                vgs_eval,
                vgd_eval,
                jfet.analysis_temperature(),
                Some((
                    history.vgs_prev[idx],
                    history.vgd_prev[idx],
                    history.qgs_prev[idx],
                    history.qgd_prev[idx],
                )),
            );
            let (cgs, cgd) = jfet2_charge
                .map(|charge| (charge.cgs, charge.cgd))
                .unwrap_or_else(|| {
                    jfet.transient_capacitances(vgs_eval, vgd_eval, jfet.analysis_temperature())
                });
            let cds = jfet.transient_drain_source_capacitance();
            let vds_charge = vgs_eval - vgd_eval;

            if !suppress_gate_charge && cgs.is_finite() && cgs > 0.0 {
                let (geq, ieq, _q_curr, _cq_curr) = if let Some(charge) = jfet2_charge {
                    Self::nonlinear_charge_companion_terms(
                        effective_method,
                        trap_order,
                        dt,
                        cgs,
                        vgs_charge,
                        charge.qgs,
                        history.qgs_prev[idx],
                        history.qgs_prev_prev[idx],
                        history.cqgs_prev[idx],
                    )
                } else {
                    Self::jfet_companion_terms(
                        effective_method,
                        trap_order,
                        dt,
                        cgs,
                        vgs_charge,
                        history.vgs_prev[idx],
                        history.qgs_prev[idx],
                        history.qgs_prev_prev[idx],
                        history.cqgs_prev[idx],
                    )
                };
                Self::stamp_two_terminal_companion(matrix, rhs, jfet.gate, jfet.source, geq, ieq);
            }

            if !suppress_gate_charge && cgd.is_finite() && cgd > 0.0 {
                let (geq, ieq, _q_curr, _cq_curr) = if let Some(charge) = jfet2_charge {
                    Self::nonlinear_charge_companion_terms(
                        effective_method,
                        trap_order,
                        dt,
                        cgd,
                        vgd_charge,
                        charge.qgd,
                        history.qgd_prev[idx],
                        history.qgd_prev_prev[idx],
                        history.cqgd_prev[idx],
                    )
                } else {
                    Self::jfet_companion_terms(
                        effective_method,
                        trap_order,
                        dt,
                        cgd,
                        vgd_charge,
                        history.vgd_prev[idx],
                        history.qgd_prev[idx],
                        history.qgd_prev_prev[idx],
                        history.cqgd_prev[idx],
                    )
                };
                Self::stamp_two_terminal_companion(matrix, rhs, jfet.gate, jfet.drain, geq, ieq);
            }

            if cds.is_finite() && cds > 0.0 {
                let (geq, ieq, _q_curr, _cq_curr) = Self::jfet_companion_terms(
                    effective_method,
                    trap_order,
                    dt,
                    cds,
                    vds_charge,
                    history.vds_prev[idx],
                    history.qds_prev[idx],
                    history.qds_prev_prev[idx],
                    history.cqds_prev[idx],
                );
                Self::stamp_two_terminal_companion(matrix, rhs, jfet.drain, jfet.source, geq, ieq);
            }
        }
    }

    /// Resolve the matrix slots every diode junction-charge companion will
    /// stamp into; the pattern is frozen for the whole transient run.
    pub(super) fn link_diode_companion_slots(
        circuit: &crate::circuit::Circuit,
        matrix: &crate::solver::StaticMatrix,
    ) -> Vec<TwoTerminalStampSlots> {
        circuit
            .diodes
            .devices
            .iter()
            .map(|diode| TwoTerminalStampSlots::link(matrix, diode.node_anode, diode.node_cathode))
            .collect()
    }

    /// Resolve the matrix slots for the five MOSFET charge companions
    /// (gate-source, gate-drain, gate-bulk, body-source, body-drain).
    pub(super) fn link_mosfet_companion_slots(
        circuit: &crate::circuit::Circuit,
        matrix: &crate::solver::StaticMatrix,
    ) -> Vec<[TwoTerminalStampSlots; 5]> {
        circuit
            .mosfets
            .devices
            .iter()
            .map(|mos| {
                let (bs_pos, bs_neg) = mos.body_source_charge_nodes();
                let (bd_pos, bd_neg) = mos.body_drain_charge_nodes();
                [
                    TwoTerminalStampSlots::link(matrix, mos.node_gate, mos.node_source),
                    TwoTerminalStampSlots::link(matrix, mos.node_gate, mos.node_drain),
                    TwoTerminalStampSlots::link(matrix, mos.node_gate, mos.node_bulk),
                    TwoTerminalStampSlots::link(matrix, bs_pos, bs_neg),
                    TwoTerminalStampSlots::link(matrix, bd_pos, bd_neg),
                ]
            })
            .collect()
    }

    pub(super) fn link_vdmos_companion_slots(
        circuit: &crate::circuit::Circuit,
        matrix: &crate::solver::StaticMatrix,
    ) -> Vec<[TwoTerminalStampSlots; 7]> {
        circuit
            .vdmoses
            .devices
            .iter()
            .map(|vdmos| {
                let (gs_pos, gs_neg) = vdmos.gate_source_charge_nodes();
                let (gd_pos, gd_neg) = vdmos.gate_drain_charge_nodes();
                let (gb_pos, gb_neg) = vdmos.gate_bulk_charge_nodes();
                let (ds_pos, ds_neg) = vdmos.drain_source_charge_nodes();
                let (bs_pos, bs_neg) = vdmos.body_source_charge_nodes();
                let (bd_pos, bd_neg) = vdmos.body_drain_charge_nodes();
                let (d1_pos, d1_neg) = vdmos.d1_charge_nodes();
                [
                    TwoTerminalStampSlots::link(matrix, gs_pos, gs_neg),
                    TwoTerminalStampSlots::link(matrix, gd_pos, gd_neg),
                    TwoTerminalStampSlots::link(matrix, gb_pos, gb_neg),
                    TwoTerminalStampSlots::link(matrix, ds_pos, ds_neg),
                    TwoTerminalStampSlots::link(matrix, bs_pos, bs_neg),
                    TwoTerminalStampSlots::link(matrix, bd_pos, bd_neg),
                    TwoTerminalStampSlots::link(matrix, d1_pos, d1_neg),
                ]
            })
            .collect()
    }

    /// Stamp the diode junction-charge companions (ngspice dioload.c's
    /// `DIOcapCharge` integration). The charge is evaluated from the raw
    /// junction voltage: the conduction stamp's pnjlim limiting is a Newton
    /// iteration aid that leaves converged points untouched, and the
    /// charge-form companion (`nonlinear_charge_companion_terms`) needs the
    /// charge history tracked against one consistent voltage.
    pub(super) fn stamp_diode_transient_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &DiodeTransientHistory,
        slots: &[TwoTerminalStampSlots],
    ) {
        let effective_method = Self::effective_companion_method(method, trap_order);
        for (idx, diode) in circuit.diodes.devices.iter().enumerate() {
            let vd = Self::differential_voltage(voltages, diode.node_anode, diode.node_cathode);
            let (qd, capd) = diode.junction_charge_and_capacitance(vd);
            if !capd.is_finite() || capd <= 0.0 {
                continue;
            }
            let (geq, ieq, _q_curr, _cq_curr) = Self::nonlinear_charge_companion_terms(
                effective_method,
                trap_order,
                dt,
                capd,
                vd,
                qd,
                history.qd_prev[idx],
                history.qd_prev_prev[idx],
                history.cqd_prev[idx],
            );
            Self::stamp_two_terminal_companion_direct(matrix, rhs, &slots[idx], geq, ieq);
        }
    }

    #[inline]
    pub(super) fn stamp_mosfet_transient_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &MosfetTransientHistory,
        suppress_gate_charge: bool,
        slots: &[[TwoTerminalStampSlots; 5]],
    ) {
        let effective_method = Self::effective_companion_method(method, trap_order);

        // NOTE (M3.2, measured 2026-06-12 on mos_array_4096): evaluating
        // these per-device terms on the rayon pool — par_chunks(256) with a
        // serial in-order apply, bit-identical to this loop at any thread
        // count — was 29% SLOWER than this serial walk (stamp 0.52s → 0.67s
        // over the run). Per-iteration term buffers plus fork/join overhead
        // exceed what ~100 ns level-1 evaluations can save even at 4096
        // devices. Parallel device evaluation only pays once a section
        // carries microsecond-scale models (VBIC/BSIM tiers) or the whole
        // iteration (companions + conduction + update) is fused into one
        // pool pass over persistent scratch. The terms helper below stays
        // pure precisely so that fused pass can be built when the model
        // tiers justify it.
        for (idx, mos) in circuit.mosfets.devices.iter().enumerate() {
            let device_terms = Self::mosfet_companion_branch_terms(
                mos,
                idx,
                voltages,
                effective_method,
                trap_order,
                dt,
                history,
                suppress_gate_charge,
            );
            for (branch, &(geq, ieq)) in device_terms.iter().enumerate() {
                if geq > 0.0 {
                    Self::stamp_two_terminal_companion_direct(
                        matrix,
                        rhs,
                        &slots[idx][branch],
                        geq,
                        ieq,
                    );
                }
            }
        }
    }

    /// Charge-companion `(geq, ieq)` for one MOSFET's five reactive branches
    /// (gate-source, gate-drain, gate-bulk, body-source, body-drain) at the
    /// given iterate. Pure: no engine or device state is touched, which is
    /// what lets the transient assembly evaluate devices on the thread pool.
    #[allow(clippy::too_many_arguments)]
    fn mosfet_companion_branch_terms(
        mos: &crate::device::Mosfet,
        idx: usize,
        voltages: &[Value],
        effective_method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &MosfetTransientHistory,
        suppress_gate_charge: bool,
    ) -> [(Value, Value); 5] {
        let mut terms = [(0.0, 0.0); 5];
        let (vgs_eval, vds_eval, vbs_eval) = mos.eval_branch_voltages_at(voltages);

        if !suppress_gate_charge {
            let (vgs, vgd, vgb) = mos.gate_charge_branch_voltages_at(voltages);
            let (cgs_half, cgd_half, cgb_half) =
                mos.transient_capacitance_halves_at(vgs_eval, vds_eval, vbs_eval);
            let (cgs_ov, cgd_ov, cgb_ov) = mos.overlap_capacitances();
            let cgs = cgs_half + history.capgs_prev_half[idx] + cgs_ov;
            let cgd = cgd_half + history.capgd_prev_half[idx] + cgd_ov;
            let cgb = cgb_half + history.capgb_prev_half[idx] + cgb_ov;

            let (geq_gs, ieq_gs, _q, _cq) = Self::jfet_companion_terms(
                effective_method,
                trap_order,
                dt,
                cgs,
                vgs,
                history.vgs_prev[idx],
                history.qgs_prev[idx],
                history.qgs_prev_prev[idx],
                history.cqgs_prev[idx],
            );
            terms[0] = (geq_gs, ieq_gs);

            let (geq_gd, ieq_gd, _q, _cq) = Self::jfet_companion_terms(
                effective_method,
                trap_order,
                dt,
                cgd,
                vgd,
                history.vgd_prev[idx],
                history.qgd_prev[idx],
                history.qgd_prev_prev[idx],
                history.cqgd_prev[idx],
            );
            terms[1] = (geq_gd, ieq_gd);

            let (geq_gb, ieq_gb, _q, _cq) = Self::jfet_companion_terms(
                effective_method,
                trap_order,
                dt,
                cgb,
                vgb,
                history.vgb_prev[idx],
                history.qgb_prev[idx],
                history.qgb_prev_prev[idx],
                history.cqgb_prev[idx],
            );
            terms[2] = (geq_gb, ieq_gb);
        }

        let vbs_j = mos.body_source_charge_branch_voltage(vbs_eval);
        let vbd_j = mos.body_drain_charge_branch_voltage(vds_eval, vbs_eval);
        let (qbs_curr, cbs) = mos.body_source_junction_charge_and_capacitance_at(vbs_eval);
        let (qbd_curr, cbd) = mos.body_drain_junction_charge_and_capacitance_at(vds_eval, vbs_eval);

        let (geq_bs, ieq_bs, _q, _cq) = Self::nonlinear_charge_companion_terms(
            effective_method,
            trap_order,
            dt,
            cbs,
            vbs_j,
            qbs_curr,
            history.qbs_prev[idx],
            history.qbs_prev_prev[idx],
            history.cqbs_prev[idx],
        );
        terms[3] = (geq_bs, ieq_bs);

        let (geq_bd, ieq_bd, _q, _cq) = Self::nonlinear_charge_companion_terms(
            effective_method,
            trap_order,
            dt,
            cbd,
            vbd_j,
            qbd_curr,
            history.qbd_prev[idx],
            history.qbd_prev_prev[idx],
            history.cqbd_prev[idx],
        );
        terms[4] = (geq_bd, ieq_bd);

        terms
    }

    #[inline]
    pub(super) fn stamp_vdmos_transient_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &VdmosTransientHistory,
        slots: &[[TwoTerminalStampSlots; 7]],
    ) {
        let effective_method = Self::effective_companion_method(method, trap_order);
        for (idx, vdmos) in circuit.vdmoses.devices.iter().enumerate() {
            let terms = Self::vdmos_companion_branch_terms(
                vdmos,
                idx,
                voltages,
                effective_method,
                trap_order,
                dt,
                history,
            );
            for (branch, &(geq, ieq)) in terms.iter().enumerate() {
                if geq > 0.0 {
                    Self::stamp_two_terminal_companion_direct(
                        matrix,
                        rhs,
                        &slots[idx][branch],
                        geq,
                        ieq,
                    );
                }
            }
        }
    }

    fn vdmos_companion_branch_terms(
        vdmos: &crate::device::Vdmos,
        idx: usize,
        voltages: &[Value],
        effective_method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &VdmosTransientHistory,
    ) -> [(Value, Value); 7] {
        let mut terms = [(0.0, 0.0); 7];
        let (vgs, vgd, vgb, vds) = vdmos.transient_charge_branch_voltages_at(voltages);
        let vd1 = vdmos.d1_charge_branch_voltage_at(voltages);
        let (vbs, vbd) = vdmos.body_charge_branch_voltages_at(voltages);
        let (cgs, cgd, cds) = vdmos.capacitances(vgs, vds);
        let cgb = vdmos.gate_bulk_capacitance();
        let (qbs, cbs) = vdmos.body_source_transient_charge_and_capacitance_at(vbs);
        let (qbd, cbd) = vdmos.body_drain_transient_charge_and_capacitance_at(vbd);
        let (qd1, cd1) = vdmos.d1_charge_and_capacitance_at(vd1);

        let (geq_gs, ieq_gs, _qgs, _cqgs) = Self::jfet_companion_terms(
            effective_method,
            trap_order,
            dt,
            cgs,
            vgs,
            history.vgs_prev[idx],
            history.qgs_prev[idx],
            history.qgs_prev_prev[idx],
            history.cqgs_prev[idx],
        );
        terms[0] = (geq_gs, ieq_gs);

        let (geq_gd, ieq_gd, _qgd, _cqgd) = Self::jfet_companion_terms(
            effective_method,
            trap_order,
            dt,
            cgd,
            vgd,
            history.vgd_prev[idx],
            history.qgd_prev[idx],
            history.qgd_prev_prev[idx],
            history.cqgd_prev[idx],
        );
        terms[1] = (geq_gd, ieq_gd);

        let (geq_gb, ieq_gb, _qgb, _cqgb) = Self::jfet_companion_terms(
            effective_method,
            trap_order,
            dt,
            cgb,
            vgb,
            history.vgb_prev[idx],
            history.qgb_prev[idx],
            history.qgb_prev_prev[idx],
            history.cqgb_prev[idx],
        );
        terms[2] = (geq_gb, ieq_gb);

        let (geq_ds, ieq_ds, _qds, _cqds) = Self::jfet_companion_terms(
            effective_method,
            trap_order,
            dt,
            cds,
            vds,
            history.vds_prev[idx],
            history.qds_prev[idx],
            history.qds_prev_prev[idx],
            history.cqds_prev[idx],
        );
        terms[3] = (geq_ds, ieq_ds);

        let (geq_bs, ieq_bs, _qbs, _cqbs) = Self::nonlinear_charge_companion_terms(
            effective_method,
            trap_order,
            dt,
            cbs,
            vbs,
            qbs,
            history.qbs_prev[idx],
            history.qbs_prev_prev[idx],
            history.cqbs_prev[idx],
        );
        terms[4] = (geq_bs, ieq_bs);

        let (geq_bd, ieq_bd, _qbd, _cqbd) = Self::nonlinear_charge_companion_terms(
            effective_method,
            trap_order,
            dt,
            cbd,
            vbd,
            qbd,
            history.qbd_prev[idx],
            history.qbd_prev_prev[idx],
            history.cqbd_prev[idx],
        );
        terms[5] = (geq_bd, ieq_bd);

        let (geq_d1, ieq_d1, _qd1, _cqd1) = Self::nonlinear_charge_companion_terms(
            effective_method,
            trap_order,
            dt,
            cd1,
            vd1,
            qd1,
            history.qd1_prev[idx],
            history.qd1_prev_prev[idx],
            history.cqd1_prev[idx],
        );
        terms[6] = (geq_d1, ieq_d1);

        terms
    }

    #[inline]
    pub(super) fn initialize_b3soi_history(
        circuit: &crate::circuit::Circuit,
        solution: &[Value],
    ) -> B3SoiTransientHistory {
        let n = circuit.b3soi.len();
        let mut h = B3SoiTransientHistory {
            qg_prev: Vec::with_capacity(n),
            qg_prev_prev: Vec::with_capacity(n),
            qg_prev_prev_prev: Vec::with_capacity(n),
            cqg_prev: Vec::with_capacity(n),
            qb_prev: Vec::with_capacity(n),
            qb_prev_prev: Vec::with_capacity(n),
            qb_prev_prev_prev: Vec::with_capacity(n),
            cqb_prev: Vec::with_capacity(n),
            qd_prev: Vec::with_capacity(n),
            qd_prev_prev: Vec::with_capacity(n),
            qd_prev_prev_prev: Vec::with_capacity(n),
            cqd_prev: Vec::with_capacity(n),
            qe_prev: Vec::with_capacity(n),
            qe_prev_prev: Vec::with_capacity(n),
            qe_prev_prev_prev: Vec::with_capacity(n),
            cqe_prev: Vec::with_capacity(n),
            qth_prev: Vec::with_capacity(n),
            qth_prev_prev: Vec::with_capacity(n),
            qth_prev_prev_prev: Vec::with_capacity(n),
            cqth_prev: Vec::with_capacity(n),
            accepted_dt_prev: 0.0,
            accepted_dt_prev_prev: 0.0,
        };
        let mut seed = |qg: Value, qb: Value, qd: Value, qe: Value, qth: Value| {
            h.qg_prev.push(qg);
            h.qg_prev_prev.push(qg);
            h.qg_prev_prev_prev.push(qg);
            h.cqg_prev.push(0.0);
            h.qb_prev.push(qb);
            h.qb_prev_prev.push(qb);
            h.qb_prev_prev_prev.push(qb);
            h.cqb_prev.push(0.0);
            h.qd_prev.push(qd);
            h.qd_prev_prev.push(qd);
            h.qd_prev_prev_prev.push(qd);
            h.cqd_prev.push(0.0);
            h.qe_prev.push(qe);
            h.qe_prev_prev.push(qe);
            h.qe_prev_prev_prev.push(qe);
            h.cqe_prev.push(0.0);
            h.qth_prev.push(qth);
            h.qth_prev_prev.push(qth);
            h.qth_prev_prev_prev.push(qth);
            h.cqth_prev.push(0.0);
        };
        // The history is indexed DD devices first, then FD, then PD; the
        // stamp/commit/truncation walks use the same concatenated order.
        // `DEBUG=-1` devices keep an (all-zero) slot so the indexing stays
        // aligned, but contribute no charges.
        for dev in &circuit.b3soi.devices {
            if dev.charges_suppressed() {
                seed(0.0, 0.0, 0.0, 0.0, 0.0);
                continue;
            }
            let c = dev.charge_at(solution);
            seed(c.qg, c.qb, c.qd, c.qe, c.qth);
        }
        for dev in &circuit.b3soi_fd.devices {
            if dev.charges_suppressed() {
                seed(0.0, 0.0, 0.0, 0.0, 0.0);
                continue;
            }
            let c = dev.charge_at(solution);
            seed(c.qg, c.qb, c.qd, c.qe, c.qth);
        }
        for dev in &circuit.b3soi_pd.devices {
            if dev.charges_suppressed() {
                seed(0.0, 0.0, 0.0, 0.0, 0.0);
                continue;
            }
            let c = dev.charge_at(solution);
            seed(c.qg, c.qb, c.qd, c.qe, c.qth);
        }
        h
    }

    /// Integrate one SOI device's node charges with the engine
    /// coefficient and its per-charge history slot, yielding the equivalent
    /// charge currents `(cqg, cqb, cqd, cqe, cqth)`.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn b3soi_companion_currents(
        effective_method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &B3SoiTransientHistory,
        idx: usize,
        qg: Value,
        qb: Value,
        qd: Value,
        qe: Value,
        qth: Value,
    ) -> (Value, Value, Value, Value, Value) {
        let cq = |q: Value, q_prev: Value, q_prev_prev: Value, cq_prev: Value| {
            Self::jfet_companion_ccap(
                effective_method,
                trap_order,
                dt,
                q,
                q_prev,
                q_prev_prev,
                cq_prev,
            )
        };
        (
            cq(
                qg,
                history.qg_prev[idx],
                history.qg_prev_prev[idx],
                history.cqg_prev[idx],
            ),
            cq(
                qb,
                history.qb_prev[idx],
                history.qb_prev_prev[idx],
                history.cqb_prev[idx],
            ),
            cq(
                qd,
                history.qd_prev[idx],
                history.qd_prev_prev[idx],
                history.cqd_prev[idx],
            ),
            cq(
                qe,
                history.qe_prev[idx],
                history.qe_prev_prev[idx],
                history.cqe_prev[idx],
            ),
            cq(
                qth,
                history.qth_prev[idx],
                history.qth_prev_prev[idx],
                history.cqth_prev[idx],
            ),
        )
    }

    /// Commit one SOI device's accepted charges and integrated currents into
    /// its history slot.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn b3soi_commit_history_slot(
        history: &mut B3SoiTransientHistory,
        idx: usize,
        qg: Value,
        qb: Value,
        qd: Value,
        qe: Value,
        qth: Value,
        cqg: Value,
        cqb: Value,
        cqd: Value,
        cqe: Value,
        cqth: Value,
    ) {
        history.qg_prev_prev_prev[idx] = history.qg_prev_prev[idx];
        history.qg_prev_prev[idx] = history.qg_prev[idx];
        history.qg_prev[idx] = qg;
        history.cqg_prev[idx] = cqg;
        history.qb_prev_prev_prev[idx] = history.qb_prev_prev[idx];
        history.qb_prev_prev[idx] = history.qb_prev[idx];
        history.qb_prev[idx] = qb;
        history.cqb_prev[idx] = cqb;
        history.qd_prev_prev_prev[idx] = history.qd_prev_prev[idx];
        history.qd_prev_prev[idx] = history.qd_prev[idx];
        history.qd_prev[idx] = qd;
        history.cqd_prev[idx] = cqd;
        history.qe_prev_prev_prev[idx] = history.qe_prev_prev[idx];
        history.qe_prev_prev[idx] = history.qe_prev[idx];
        history.qe_prev[idx] = qe;
        history.cqe_prev[idx] = cqe;
        history.qth_prev_prev_prev[idx] = history.qth_prev_prev[idx];
        history.qth_prev_prev[idx] = history.qth_prev[idx];
        history.qth_prev[idx] = qth;
        history.cqth_prev[idx] = cqth;
    }

    /// Stamp the B3SOI transient charge companion for every SOI instance.
    ///
    /// Integrates each coupled node charge with the engine's
    /// integration coefficient `ag0` and the per-charge history, then stamps the
    /// coupled `gc**·ag0` capacitance matrix plus the `ceqq*` equivalent charge
    /// currents (B3SOI charge load). Active DD/FD/PD self-heating also stamps
    /// the thermal `qth` companion onto the temperature node.
    #[inline]
    pub(super) fn stamp_b3soi_transient_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &B3SoiTransientHistory,
    ) {
        if !circuit.has_b3soi_devices() {
            return;
        }
        let effective_method = Self::effective_companion_method(method, trap_order);
        // ag0 = the bare integration gain (companion geq for unit capacitance).
        let ag0 = Self::jfet_companion_geq(effective_method, trap_order, 1.0, dt);
        if ag0 <= 0.0 {
            return;
        }
        let mut stamper = StaticMatrixChargeStamper { matrix, rhs };
        let mut idx = 0;
        for dev in &circuit.b3soi.devices {
            if dev.charges_suppressed() {
                idx += 1;
                continue;
            }
            let charge = dev.charge_at(voltages);
            let (cqg, cqb, cqd, cqe, cqth) = Self::b3soi_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                charge.qg,
                charge.qb,
                charge.qd,
                charge.qe,
                charge.qth,
            );
            dev.stamp_charge_companion(
                &charge,
                ag0,
                cqg,
                cqb,
                cqd,
                cqe,
                cqth,
                voltages,
                &mut stamper,
            );
            idx += 1;
        }
        for dev in &circuit.b3soi_fd.devices {
            if dev.charges_suppressed() {
                idx += 1;
                continue;
            }
            let charge = dev.charge_at(voltages);
            let (cqg, cqb, cqd, cqe, cqth) = Self::b3soi_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                charge.qg,
                charge.qb,
                charge.qd,
                charge.qe,
                charge.qth,
            );
            dev.stamp_charge_companion(
                &charge,
                ag0,
                cqg,
                cqb,
                cqd,
                cqe,
                cqth,
                voltages,
                &mut stamper,
            );
            idx += 1;
        }
        for dev in &circuit.b3soi_pd.devices {
            if dev.charges_suppressed() {
                idx += 1;
                continue;
            }
            let charge = dev.charge_at(voltages);
            let (cqg, cqb, cqd, cqe, cqth) = Self::b3soi_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                charge.qg,
                charge.qb,
                charge.qd,
                charge.qe,
                charge.qth,
            );
            dev.stamp_charge_companion(
                &charge,
                ag0,
                cqg,
                cqb,
                cqd,
                cqe,
                cqth,
                voltages,
                &mut stamper,
            );
            idx += 1;
        }
    }

    /// Commit the SOI (DD/FD/PD) charge history after an accepted timestep.
    #[inline]
    pub(super) fn update_b3soi_history(
        circuit: &crate::circuit::Circuit,
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &mut B3SoiTransientHistory,
    ) {
        if !circuit.has_b3soi_devices() {
            return;
        }
        let effective_method = Self::effective_companion_method(method, trap_order);
        let mut idx = 0;
        for dev in &circuit.b3soi.devices {
            if dev.charges_suppressed() {
                idx += 1;
                continue;
            }
            let charge = dev.charge_at(voltages);
            let (cqg, cqb, cqd, cqe, cqth) = Self::b3soi_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                charge.qg,
                charge.qb,
                charge.qd,
                charge.qe,
                charge.qth,
            );
            Self::b3soi_commit_history_slot(
                history, idx, charge.qg, charge.qb, charge.qd, charge.qe, charge.qth, cqg, cqb,
                cqd, cqe, cqth,
            );
            idx += 1;
        }
        for dev in &circuit.b3soi_fd.devices {
            if dev.charges_suppressed() {
                idx += 1;
                continue;
            }
            let charge = dev.charge_at(voltages);
            let (cqg, cqb, cqd, cqe, cqth) = Self::b3soi_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                charge.qg,
                charge.qb,
                charge.qd,
                charge.qe,
                charge.qth,
            );
            Self::b3soi_commit_history_slot(
                history, idx, charge.qg, charge.qb, charge.qd, charge.qe, charge.qth, cqg, cqb,
                cqd, cqe, cqth,
            );
            idx += 1;
        }
        for dev in &circuit.b3soi_pd.devices {
            if dev.charges_suppressed() {
                idx += 1;
                continue;
            }
            let charge = dev.charge_at(voltages);
            let (cqg, cqb, cqd, cqe, cqth) = Self::b3soi_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                charge.qg,
                charge.qb,
                charge.qd,
                charge.qe,
                charge.qth,
            );
            Self::b3soi_commit_history_slot(
                history, idx, charge.qg, charge.qb, charge.qd, charge.qe, charge.qth, cqg, cqb,
                cqd, cqe, cqth,
            );
            idx += 1;
        }
        history.accepted_dt_prev_prev = history.accepted_dt_prev;
        history.accepted_dt_prev = dt;
    }

    #[inline]
    pub(super) fn initialize_bsim3_history(
        circuit: &crate::circuit::Circuit,
        solution: &[Value],
    ) -> Bsim3TransientHistory {
        let n = circuit.bsim3v3.len();
        let mut h = Bsim3TransientHistory {
            qg_prev: Vec::with_capacity(n),
            qg_prev_prev: Vec::with_capacity(n),
            qg_prev_prev_prev: Vec::with_capacity(n),
            cqg_prev: Vec::with_capacity(n),
            qb_prev: Vec::with_capacity(n),
            qb_prev_prev: Vec::with_capacity(n),
            qb_prev_prev_prev: Vec::with_capacity(n),
            cqb_prev: Vec::with_capacity(n),
            qd_prev: Vec::with_capacity(n),
            qd_prev_prev: Vec::with_capacity(n),
            qd_prev_prev_prev: Vec::with_capacity(n),
            cqd_prev: Vec::with_capacity(n),
            qcheq_prev: Vec::with_capacity(n),
            qcheq_prev_prev: Vec::with_capacity(n),
            qcheq_prev_prev_prev: Vec::with_capacity(n),
            cqcheq_prev: Vec::with_capacity(n),
            qcdump_prev: Vec::with_capacity(n),
            qcdump_prev_prev: Vec::with_capacity(n),
            qcdump_prev_prev_prev: Vec::with_capacity(n),
            cqcdump_prev: Vec::with_capacity(n),
            accepted_dt_prev: 0.0,
            accepted_dt_prev_prev: 0.0,
        };
        // Flat seed at the accepted point with zeroed charge currents, the
        // MODEINITTRAN state copy of b3ld.c:2818-2829. States stay
        // per-device (ngspice applies `m` only at stamp time; CKTterr sees
        // the unscaled CKTstate charges).
        for dev in &circuit.bsim3v3.devices {
            let (c, _mode) = dev.charge_at(solution);
            for (q, slots) in [
                (
                    c.qg_state(),
                    [
                        &mut h.qg_prev,
                        &mut h.qg_prev_prev,
                        &mut h.qg_prev_prev_prev,
                    ],
                ),
                (
                    c.qb_state(),
                    [
                        &mut h.qb_prev,
                        &mut h.qb_prev_prev,
                        &mut h.qb_prev_prev_prev,
                    ],
                ),
                (
                    c.qd_state(),
                    [
                        &mut h.qd_prev,
                        &mut h.qd_prev_prev,
                        &mut h.qd_prev_prev_prev,
                    ],
                ),
            ] {
                for slot in slots {
                    slot.push(q);
                }
            }
            h.cqg_prev.push(0.0);
            h.cqb_prev.push(0.0);
            h.cqd_prev.push(0.0);
            let qcheq = if dev.uses_trnqs() { c.qcheq } else { 0.0 };
            for slot in [
                &mut h.qcheq_prev,
                &mut h.qcheq_prev_prev,
                &mut h.qcheq_prev_prev_prev,
            ] {
                slot.push(qcheq);
            }
            let qcdump = if dev.uses_trnqs() {
                dev.trnqs_qcdump_state(solution)
            } else {
                0.0
            };
            for slot in [
                &mut h.qcdump_prev,
                &mut h.qcdump_prev_prev,
                &mut h.qcdump_prev_prev_prev,
            ] {
                slot.push(qcdump);
            }
            h.cqcheq_prev.push(0.0);
            h.cqcdump_prev.push(0.0);
        }
        h
    }

    /// Integrate one BSIM3 device's three composite node charges with the
    /// engine coefficient and its history slot, yielding the equivalent
    /// charge currents `(cqg, cqb, cqd)` (ngspice `NIintegrate` on
    /// `BSIM3qg`/`BSIM3qb`/`BSIM3qd`).
    #[inline]
    fn bsim3_companion_currents(
        effective_method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &Bsim3TransientHistory,
        idx: usize,
        qg: Value,
        qb: Value,
        qd: Value,
    ) -> (Value, Value, Value) {
        let cq = |q: Value, q_prev: Value, q_prev_prev: Value, cq_prev: Value| {
            Self::jfet_companion_ccap(
                effective_method,
                trap_order,
                dt,
                q,
                q_prev,
                q_prev_prev,
                cq_prev,
            )
        };
        (
            cq(
                qg,
                history.qg_prev[idx],
                history.qg_prev_prev[idx],
                history.cqg_prev[idx],
            ),
            cq(
                qb,
                history.qb_prev[idx],
                history.qb_prev_prev[idx],
                history.cqb_prev[idx],
            ),
            cq(
                qd,
                history.qd_prev[idx],
                history.qd_prev_prev[idx],
                history.cqd_prev[idx],
            ),
        )
    }

    #[inline]
    fn bsim3_trnqs_companion_currents(
        effective_method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &Bsim3TransientHistory,
        idx: usize,
        qcheq: Value,
        qcdump: Value,
    ) -> (Value, Value) {
        let cq = |q: Value, q_prev: Value, q_prev_prev: Value, cq_prev: Value| {
            Self::jfet_companion_ccap(
                effective_method,
                trap_order,
                dt,
                q,
                q_prev,
                q_prev_prev,
                cq_prev,
            )
        };
        (
            cq(
                qcheq,
                history.qcheq_prev[idx],
                history.qcheq_prev_prev[idx],
                history.cqcheq_prev[idx],
            ),
            cq(
                qcdump,
                history.qcdump_prev[idx],
                history.qcdump_prev_prev[idx],
                history.cqcdump_prev[idx],
            ),
        )
    }

    /// Stamp the BSIM3 transient charge companion for every instance: the
    /// mode-assembled `gc**·ag0` capacitance matrix plus the `ceqq*`
    /// equivalent charge currents. `NQSMOD=1` also stamps the hidden
    /// charge-deficit row from b3ld.c.
    #[inline]
    pub(super) fn stamp_bsim3_transient_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &Bsim3TransientHistory,
    ) {
        if !circuit.has_bsim3v3_devices() {
            return;
        }
        let effective_method = Self::effective_companion_method(method, trap_order);
        // ag0 = the bare integration gain (companion geq for unit capacitance).
        let ag0 = Self::jfet_companion_geq(effective_method, trap_order, 1.0, dt);
        if ag0 <= 0.0 {
            return;
        }
        let mut stamper = StaticMatrixChargeStamper { matrix, rhs };
        for (idx, dev) in circuit.bsim3v3.devices.iter().enumerate() {
            let (charge, mode) = dev.charge_at(voltages);
            let (qg, qb, qd) = if dev.uses_trnqs() {
                dev.trnqs_state_charges(&charge)
            } else {
                (charge.qg_state(), charge.qb_state(), charge.qd_state())
            };
            let (cqg, cqb, cqd) = Self::bsim3_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                qg,
                qb,
                qd,
            );
            // The history carries per-device charges; the device stamp
            // applies the parallel multiplier itself (b3ld.c: m * ceqq*).
            if dev.uses_trnqs() {
                let qcdump = dev.trnqs_qcdump_state(voltages);
                let (cqcheq, cqcdump) = Self::bsim3_trnqs_companion_currents(
                    effective_method,
                    trap_order,
                    dt,
                    history,
                    idx,
                    charge.qcheq,
                    qcdump,
                );
                dev.stamp_trnqs_charge_companion(
                    &charge,
                    mode,
                    ag0,
                    cqg,
                    cqb,
                    cqd,
                    cqcheq,
                    cqcdump,
                    voltages,
                    &mut stamper,
                );
            } else {
                dev.stamp_charge_companion(
                    &charge,
                    mode,
                    ag0,
                    cqg,
                    cqb,
                    cqd,
                    voltages,
                    &mut stamper,
                );
            }
        }
    }

    /// Commit the BSIM3 charge history after an accepted timestep.
    #[inline]
    pub(super) fn update_bsim3_history(
        circuit: &crate::circuit::Circuit,
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &mut Bsim3TransientHistory,
    ) {
        if !circuit.has_bsim3v3_devices() {
            return;
        }
        let effective_method = Self::effective_companion_method(method, trap_order);
        for (idx, dev) in circuit.bsim3v3.devices.iter().enumerate() {
            let (charge, _mode) = dev.charge_at(voltages);
            let (qg, qb, qd) = if dev.uses_trnqs() {
                dev.trnqs_state_charges(&charge)
            } else {
                (charge.qg_state(), charge.qb_state(), charge.qd_state())
            };
            let (cqg, cqb, cqd) = Self::bsim3_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                qg,
                qb,
                qd,
            );
            history.qg_prev_prev_prev[idx] = history.qg_prev_prev[idx];
            history.qg_prev_prev[idx] = history.qg_prev[idx];
            history.qg_prev[idx] = qg;
            history.cqg_prev[idx] = cqg;
            history.qb_prev_prev_prev[idx] = history.qb_prev_prev[idx];
            history.qb_prev_prev[idx] = history.qb_prev[idx];
            history.qb_prev[idx] = qb;
            history.cqb_prev[idx] = cqb;
            history.qd_prev_prev_prev[idx] = history.qd_prev_prev[idx];
            history.qd_prev_prev[idx] = history.qd_prev[idx];
            history.qd_prev[idx] = qd;
            history.cqd_prev[idx] = cqd;
            let qcheq = if dev.uses_trnqs() { charge.qcheq } else { 0.0 };
            let qcdump = if dev.uses_trnqs() {
                dev.trnqs_qcdump_state(voltages)
            } else {
                0.0
            };
            let (cqcheq, cqcdump) = Self::bsim3_trnqs_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                qcheq,
                qcdump,
            );
            history.qcheq_prev_prev_prev[idx] = history.qcheq_prev_prev[idx];
            history.qcheq_prev_prev[idx] = history.qcheq_prev[idx];
            history.qcheq_prev[idx] = qcheq;
            history.cqcheq_prev[idx] = cqcheq;
            history.qcdump_prev_prev_prev[idx] = history.qcdump_prev_prev[idx];
            history.qcdump_prev_prev[idx] = history.qcdump_prev[idx];
            history.qcdump_prev[idx] = qcdump;
            history.cqcdump_prev[idx] = cqcdump;
        }
        history.accepted_dt_prev_prev = history.accepted_dt_prev;
        history.accepted_dt_prev = dt;
    }

    #[inline]
    pub(super) fn initialize_bsim4_history(
        circuit: &crate::circuit::Circuit,
        solution: &[Value],
    ) -> Bsim4TransientHistory {
        let n = circuit.bsim4v8.len();
        let mut h = Bsim4TransientHistory {
            qg_prev: Vec::with_capacity(n),
            qg_prev_prev: Vec::with_capacity(n),
            qg_prev_prev_prev: Vec::with_capacity(n),
            cqg_prev: Vec::with_capacity(n),
            qgmid_prev: Vec::with_capacity(n),
            qgmid_prev_prev: Vec::with_capacity(n),
            qgmid_prev_prev_prev: Vec::with_capacity(n),
            cqgmid_prev: Vec::with_capacity(n),
            qb_prev: Vec::with_capacity(n),
            qb_prev_prev: Vec::with_capacity(n),
            qb_prev_prev_prev: Vec::with_capacity(n),
            cqb_prev: Vec::with_capacity(n),
            qd_prev: Vec::with_capacity(n),
            qd_prev_prev: Vec::with_capacity(n),
            qd_prev_prev_prev: Vec::with_capacity(n),
            cqd_prev: Vec::with_capacity(n),
            qbs_prev: Vec::with_capacity(n),
            qbs_prev_prev: Vec::with_capacity(n),
            qbs_prev_prev_prev: Vec::with_capacity(n),
            cqbs_prev: Vec::with_capacity(n),
            qbd_prev: Vec::with_capacity(n),
            qbd_prev_prev: Vec::with_capacity(n),
            qbd_prev_prev_prev: Vec::with_capacity(n),
            cqbd_prev: Vec::with_capacity(n),
            qcheq_prev: Vec::with_capacity(n),
            qcheq_prev_prev: Vec::with_capacity(n),
            qcheq_prev_prev_prev: Vec::with_capacity(n),
            cqcheq_prev: Vec::with_capacity(n),
            qcdump_prev: Vec::with_capacity(n),
            qcdump_prev_prev: Vec::with_capacity(n),
            qcdump_prev_prev_prev: Vec::with_capacity(n),
            cqcdump_prev: Vec::with_capacity(n),
            accepted_dt_prev: 0.0,
            accepted_dt_prev_prev: 0.0,
        };
        // Flat seed at the accepted point with zeroed charge currents, the
        // MODEINITTRAN state copy of b4ld.c:4611-4628. States stay
        // per-device (ngspice applies `m` only at stamp time; CKTterr sees
        // the unscaled CKTstate charges).
        for dev in &circuit.bsim4v8.devices {
            let (c, _mode) = dev.charge_at(solution);
            let rbody = dev.rbody_enabled();
            let (qg, qgmid, qb, qd, qbs, qbd) = if dev.uses_trnqs() {
                dev.trnqs_state_charges(&c, solution)
            } else {
                (
                    c.qg_state(),
                    c.qgmid_state(),
                    c.qb_state_for_rbody(rbody),
                    c.qd_state(),
                    c.qbs,
                    c.qbd,
                )
            };
            for (q, slots) in [
                (
                    qg,
                    [
                        &mut h.qg_prev,
                        &mut h.qg_prev_prev,
                        &mut h.qg_prev_prev_prev,
                    ],
                ),
                (
                    qb,
                    [
                        &mut h.qb_prev,
                        &mut h.qb_prev_prev,
                        &mut h.qb_prev_prev_prev,
                    ],
                ),
                (
                    qd,
                    [
                        &mut h.qd_prev,
                        &mut h.qd_prev_prev,
                        &mut h.qd_prev_prev_prev,
                    ],
                ),
            ] {
                for slot in slots {
                    slot.push(q);
                }
            }
            for slot in [
                &mut h.qgmid_prev,
                &mut h.qgmid_prev_prev,
                &mut h.qgmid_prev_prev_prev,
            ] {
                slot.push(qgmid);
            }
            for (q, slots) in [
                (
                    qbs,
                    [
                        &mut h.qbs_prev,
                        &mut h.qbs_prev_prev,
                        &mut h.qbs_prev_prev_prev,
                    ],
                ),
                (
                    qbd,
                    [
                        &mut h.qbd_prev,
                        &mut h.qbd_prev_prev,
                        &mut h.qbd_prev_prev_prev,
                    ],
                ),
            ] {
                for slot in slots {
                    slot.push(q);
                }
            }
            h.cqg_prev.push(0.0);
            h.cqgmid_prev.push(0.0);
            h.cqb_prev.push(0.0);
            h.cqd_prev.push(0.0);
            h.cqbs_prev.push(0.0);
            h.cqbd_prev.push(0.0);
            for slot in [
                &mut h.qcheq_prev,
                &mut h.qcheq_prev_prev,
                &mut h.qcheq_prev_prev_prev,
            ] {
                slot.push(c.qchqs);
            }
            let qcdump = if dev.uses_trnqs() {
                dev.trnqs_qcdump_state(solution)
            } else {
                0.0
            };
            for slot in [
                &mut h.qcdump_prev,
                &mut h.qcdump_prev_prev,
                &mut h.qcdump_prev_prev_prev,
            ] {
                slot.push(qcdump);
            }
            h.cqcheq_prev.push(0.0);
            h.cqcdump_prev.push(0.0);
        }
        h
    }

    /// Integrate one BSIM4 device's charge states with the
    /// engine coefficient and its history slot, yielding the equivalent
    /// charge currents (ngspice `NIintegrate` on `BSIM4qg`/`BSIM4qb`/
    /// `BSIM4qd`, plus `qbs`/`qbd` when `rbodyMod > 0`; b4ld.c:4630-4649).
    #[inline]
    fn bsim4_companion_currents(
        effective_method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &Bsim4TransientHistory,
        idx: usize,
        qg: Value,
        qgmid: Value,
        qb: Value,
        qd: Value,
        qbs: Value,
        qbd: Value,
    ) -> (Value, Value, Value, Value, Value, Value) {
        let cq = |q: Value, q_prev: Value, q_prev_prev: Value, cq_prev: Value| {
            Self::jfet_companion_ccap(
                effective_method,
                trap_order,
                dt,
                q,
                q_prev,
                q_prev_prev,
                cq_prev,
            )
        };
        (
            cq(
                qg,
                history.qg_prev[idx],
                history.qg_prev_prev[idx],
                history.cqg_prev[idx],
            ),
            cq(
                qgmid,
                history.qgmid_prev[idx],
                history.qgmid_prev_prev[idx],
                history.cqgmid_prev[idx],
            ),
            cq(
                qb,
                history.qb_prev[idx],
                history.qb_prev_prev[idx],
                history.cqb_prev[idx],
            ),
            cq(
                qd,
                history.qd_prev[idx],
                history.qd_prev_prev[idx],
                history.cqd_prev[idx],
            ),
            cq(
                qbs,
                history.qbs_prev[idx],
                history.qbs_prev_prev[idx],
                history.cqbs_prev[idx],
            ),
            cq(
                qbd,
                history.qbd_prev[idx],
                history.qbd_prev_prev[idx],
                history.cqbd_prev[idx],
            ),
        )
    }

    #[inline]
    fn bsim4_trnqs_companion_currents(
        effective_method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &Bsim4TransientHistory,
        idx: usize,
        qcheq: Value,
        qcdump: Value,
    ) -> (Value, Value) {
        let cq = |q: Value, q_prev: Value, q_prev_prev: Value, cq_prev: Value| {
            Self::jfet_companion_ccap(
                effective_method,
                trap_order,
                dt,
                q,
                q_prev,
                q_prev_prev,
                cq_prev,
            )
        };
        (
            cq(
                qcheq,
                history.qcheq_prev[idx],
                history.qcheq_prev_prev[idx],
                history.cqcheq_prev[idx],
            ),
            cq(
                qcdump,
                history.qcdump_prev[idx],
                history.qcdump_prev_prev[idx],
                history.cqcdump_prev[idx],
            ),
        )
    }

    /// Stamp the BSIM4 transient charge companion for every instance: the
    /// mode-assembled `gc**·ag0` capacitance matrix plus the `ceqq*`
    /// equivalent charge currents (b4ld.c charge load, trnqsMod = 0).
    #[inline]
    pub(super) fn stamp_bsim4_transient_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &Bsim4TransientHistory,
    ) {
        if !circuit.has_bsim4v8_devices() {
            return;
        }
        let effective_method = Self::effective_companion_method(method, trap_order);
        // ag0 = the bare integration gain (companion geq for unit capacitance).
        let ag0 = Self::jfet_companion_geq(effective_method, trap_order, 1.0, dt);
        if ag0 <= 0.0 {
            return;
        }
        let mut stamper = StaticMatrixChargeStamper { matrix, rhs };
        for (idx, dev) in circuit.bsim4v8.devices.iter().enumerate() {
            let (charge, mode) = dev.charge_at(voltages);
            let rbody = dev.rbody_enabled();
            let (qg, qgmid, qb, qd, qbs, qbd) = if dev.uses_trnqs() {
                dev.trnqs_state_charges(&charge, voltages)
            } else {
                (
                    charge.qg_state(),
                    charge.qgmid_state(),
                    charge.qb_state_for_rbody(rbody),
                    charge.qd_state(),
                    charge.qbs,
                    charge.qbd,
                )
            };
            let (cqg, cqgmid, cqb, cqd, cqbs, cqbd) = Self::bsim4_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                qg,
                qgmid,
                qb,
                qd,
                qbs,
                qbd,
            );
            // The history carries per-device charges; the device stamp
            // applies the parallel multiplier itself (b4ld.c: mult_q * ceqq*).
            if dev.uses_trnqs() {
                let qcdump = dev.trnqs_qcdump_state(voltages);
                let (cqcheq, cqcdump) = Self::bsim4_trnqs_companion_currents(
                    effective_method,
                    trap_order,
                    dt,
                    history,
                    idx,
                    charge.qchqs,
                    qcdump,
                );
                dev.stamp_trnqs_charge_companion(
                    &charge,
                    mode,
                    ag0,
                    cqg,
                    cqb,
                    cqd,
                    cqbs,
                    cqbd,
                    cqcheq,
                    cqcdump,
                    voltages,
                    &mut stamper,
                );
            } else {
                dev.stamp_charge_companion(
                    &charge,
                    mode,
                    ag0,
                    cqg,
                    cqgmid,
                    cqb,
                    cqd,
                    cqbs,
                    cqbd,
                    voltages,
                    &mut stamper,
                );
            }
        }
    }

    /// Commit the BSIM4 charge history after an accepted timestep.
    #[inline]
    pub(super) fn update_bsim4_history(
        circuit: &crate::circuit::Circuit,
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &mut Bsim4TransientHistory,
    ) {
        if !circuit.has_bsim4v8_devices() {
            return;
        }
        let effective_method = Self::effective_companion_method(method, trap_order);
        for (idx, dev) in circuit.bsim4v8.devices.iter().enumerate() {
            let (charge, _mode) = dev.charge_at(voltages);
            let rbody = dev.rbody_enabled();
            let (qg, qgmid, qb, qd, qbs, qbd) = if dev.uses_trnqs() {
                dev.trnqs_state_charges(&charge, voltages)
            } else {
                (
                    charge.qg_state(),
                    charge.qgmid_state(),
                    charge.qb_state_for_rbody(rbody),
                    charge.qd_state(),
                    charge.qbs,
                    charge.qbd,
                )
            };
            let (cqg, cqgmid, cqb, cqd, cqbs, cqbd) = Self::bsim4_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                qg,
                qgmid,
                qb,
                qd,
                qbs,
                qbd,
            );
            history.qg_prev_prev_prev[idx] = history.qg_prev_prev[idx];
            history.qg_prev_prev[idx] = history.qg_prev[idx];
            history.qg_prev[idx] = qg;
            history.cqg_prev[idx] = cqg;
            history.qgmid_prev_prev_prev[idx] = history.qgmid_prev_prev[idx];
            history.qgmid_prev_prev[idx] = history.qgmid_prev[idx];
            history.qgmid_prev[idx] = qgmid;
            history.cqgmid_prev[idx] = cqgmid;
            history.qb_prev_prev_prev[idx] = history.qb_prev_prev[idx];
            history.qb_prev_prev[idx] = history.qb_prev[idx];
            history.qb_prev[idx] = qb;
            history.cqb_prev[idx] = cqb;
            history.qd_prev_prev_prev[idx] = history.qd_prev_prev[idx];
            history.qd_prev_prev[idx] = history.qd_prev[idx];
            history.qd_prev[idx] = qd;
            history.cqd_prev[idx] = cqd;
            history.qbs_prev_prev_prev[idx] = history.qbs_prev_prev[idx];
            history.qbs_prev_prev[idx] = history.qbs_prev[idx];
            history.qbs_prev[idx] = qbs;
            history.cqbs_prev[idx] = cqbs;
            history.qbd_prev_prev_prev[idx] = history.qbd_prev_prev[idx];
            history.qbd_prev_prev[idx] = history.qbd_prev[idx];
            history.qbd_prev[idx] = qbd;
            history.cqbd_prev[idx] = cqbd;
            let qcheq = charge.qchqs;
            let qcdump = if dev.uses_trnqs() {
                dev.trnqs_qcdump_state(voltages)
            } else {
                0.0
            };
            let (cqcheq, cqcdump) = Self::bsim4_trnqs_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                qcheq,
                qcdump,
            );
            history.qcheq_prev_prev_prev[idx] = history.qcheq_prev_prev[idx];
            history.qcheq_prev_prev[idx] = history.qcheq_prev[idx];
            history.qcheq_prev[idx] = qcheq;
            history.cqcheq_prev[idx] = cqcheq;
            history.qcdump_prev_prev_prev[idx] = history.qcdump_prev_prev[idx];
            history.qcdump_prev_prev[idx] = history.qcdump_prev[idx];
            history.qcdump_prev[idx] = qcdump;
            history.cqcdump_prev[idx] = cqcdump;
        }
        history.accepted_dt_prev_prev = history.accepted_dt_prev;
        history.accepted_dt_prev = dt;
    }

    #[inline]
    pub(super) fn stamp_tline_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        time: Value,
        _tline_dc_refs: &[(Value, Value)],
    ) {
        for tl in &circuit.tlines {
            if tl.has_txl_runtime() {
                if let Some(stamp) = tl.txl_transient_stamp(time) {
                    Self::stamp_txl_branch_runtime(matrix, rhs, tl, stamp);
                }
            } else if tl.ltra_branch_matrix_indices().is_some() {
                let response = tl.transient_port_response(time);
                Self::stamp_ltra_branch_runtime(matrix, rhs, tl, response);
            } else {
                let response = tl.transient_port_response(time);
                Self::stamp_tline_two_port(matrix, rhs, tl, response);
            }
        }
    }

    #[inline]
    pub(super) fn stamp_coupled_tline_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        time: Value,
        dt: Value,
        coupled_tline_refs: &[CoupledTlineReferenceState],
    ) {
        for (idx, tl) in circuit.coupled_tlines.iter().enumerate() {
            if tl.uses_native_runtime() {
                Self::stamp_native_cpl_branch_companions(matrix, rhs, tl, time, dt);
                continue;
            }

            let refs = coupled_tline_refs.get(idx).cloned().unwrap_or_default();
            let incoming_near = tl.incoming_near_modal(time, &refs.far_modal);
            let incoming_far = tl.incoming_far_modal(time, &refs.near_modal);
            let eq_near = tl.port_equivalent_current(&incoming_near);
            let eq_far = tl.port_equivalent_current(&incoming_far);

            Self::stamp_shared_reference_port(
                matrix,
                rhs,
                &tl.near_nodes,
                tl.near_ref,
                tl.port_admittance(),
                &eq_near,
            );
            Self::stamp_shared_reference_port(
                matrix,
                rhs,
                &tl.far_nodes,
                tl.far_ref,
                tl.port_admittance(),
                &eq_far,
            );
        }
    }

    /// Stamp the ngspice-faithful CPL branch-current convolution equations.
    ///
    /// Mirrors the matrix pointers written by `CPLload` (cplload.c) for the
    /// transient (non-MODEDC) path:
    /// - KCL: `pos[m]` row gets `+Ibr1[m]`, `far[m]` row gets `+Ibr2[m]`.
    /// - Branch1 row m: `-Ibr1[m] + sum_p aten_h1[m][p] Vpos[p]
    ///   - sum_p f3[m][p] Vfar[p] - sum_p f2[m][p] Ibr2[p] = ff[m]`.
    /// - Branch2 row m: `-Ibr2[m] + sum_p aten_h1[m][p] Vfar[p]
    ///   - sum_p f3[m][p] Vpos[p] - sum_p f2[m][p] Ibr1[p] = gg[m]`.
    ///
    /// When the native step plan is unavailable (e.g. degenerate dt before the
    /// runtime is seeded) the branch rows are anchored with `-Ibr=0` so the
    /// matrix stays non-singular.
    #[inline]
    fn stamp_native_cpl_branch_companions(
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        tl: &crate::device::CoupledTransmissionLine,
        time: Value,
        dt: Value,
    ) {
        let Some(branches) = tl.native_branch_matrix_indices() else {
            return;
        };
        let conductors = tl.conductors();

        // KCL coupling and branch-diagonal anchors are always present.
        for conductor in 0..conductors {
            let Some((b1, b2)) = branches.conductor(conductor) else {
                continue;
            };
            let near = tl.near_nodes[conductor];
            let far = tl.far_nodes[conductor];
            if near > 0 {
                matrix.add(near - 1, b1 - 1, 1.0);
            }
            if far > 0 {
                matrix.add(far - 1, b2 - 1, 1.0);
            }
            matrix.add(b1 - 1, b1 - 1, -1.0);
            matrix.add(b2 - 1, b2 - 1, -1.0);
        }

        let Some(plan) = tl.native_step_plan(time, dt) else {
            return;
        };

        for m in 0..conductors {
            let Some((b1_m, b2_m)) = branches.conductor(m) else {
                continue;
            };
            for p in 0..conductors {
                let Some((b1_p, b2_p)) = branches.conductor(p) else {
                    continue;
                };
                let near_p = tl.near_nodes[p];
                let far_p = tl.far_nodes[p];

                let aten_h1 = plan.aten_h1[m][p];
                if aten_h1 != 0.0 {
                    if near_p > 0 {
                        matrix.add(b1_m - 1, near_p - 1, aten_h1);
                    }
                    if far_p > 0 {
                        matrix.add(b2_m - 1, far_p - 1, aten_h1);
                    }
                }

                if plan.ext {
                    let f3 = plan.f3[m][p];
                    if f3 != 0.0 {
                        if far_p > 0 {
                            matrix.add(b1_m - 1, far_p - 1, -f3);
                        }
                        if near_p > 0 {
                            matrix.add(b2_m - 1, near_p - 1, -f3);
                        }
                    }
                    let f2 = plan.f2[m][p];
                    if f2 != 0.0 {
                        matrix.add(b1_m - 1, b2_p - 1, -f2);
                        matrix.add(b2_m - 1, b1_p - 1, -f2);
                    }
                }
            }
            rhs[b1_m - 1] += plan.ff[m];
            rhs[b2_m - 1] += plan.gg[m];
        }
    }

    #[inline]
    pub(super) fn initialize_tline_history(
        circuit: &mut crate::circuit::Circuit,
        initial_solution: &[Value],
        initial_time: Value,
    ) -> Vec<(Value, Value)> {
        let mut refs = Vec::with_capacity(circuit.tlines.len());
        for tl in &mut circuit.tlines {
            tl.reset();
            let z_port = Self::tline_transient_port_impedance(tl);
            let g = 1.0 / z_port;
            let v1 = Self::differential_voltage(initial_solution, tl.node1_pos, tl.node1_neg);
            let v2 = Self::differential_voltage(initial_solution, tl.node2_pos, tl.node2_neg);
            refs.push((v1, v2));
            if let Some((br1, br2)) = tl.txl_branch_matrix_indices() {
                let i1 = initial_solution.get(br1 - 1).copied().unwrap_or(0.0);
                let i2 = initial_solution.get(br2 - 1).copied().unwrap_or(0.0);
                tl.initialize_txl_history(initial_time, v1, i1, v2, i2);
                continue;
            }
            if let Some((br1, br2)) = tl.ltra_branch_matrix_indices() {
                let i1 = initial_solution.get(br1 - 1).copied().unwrap_or(0.0);
                let i2 = initial_solution.get(br2 - 1).copied().unwrap_or(0.0);
                tl.update_history(initial_time, v1, i1, v2, i2);
                continue;
            }

            // Seed delayed-wave state from the initial OP so pre-edge steady states
            // are preserved (avoids artificial startup droop/ringing).
            // Port equations: i1 = g*(v1 - incoming1), i2 = g*(v2 - incoming2),
            // with incoming1 <- v2 and incoming2 <- v1 at t=0.
            let i1_actual = g * (v1 - v2);
            let i2_actual = g * (v2 - v1);
            let wave_scale = z_port / tl.impedance();
            tl.update_history(
                initial_time,
                v1,
                i1_actual * wave_scale,
                v2,
                i2_actual * wave_scale,
            );
        }
        refs
    }

    #[inline]
    pub(super) fn initialize_coupled_tline_history(
        circuit: &mut crate::circuit::Circuit,
        initial_solution: &[Value],
        initial_time: Value,
    ) -> Vec<CoupledTlineReferenceState> {
        let mut refs = Vec::with_capacity(circuit.coupled_tlines.len());
        for tl in &mut circuit.coupled_tlines {
            tl.reset();
            let near_physical =
                Self::differential_port_voltages(initial_solution, &tl.near_nodes, tl.near_ref);
            let far_physical =
                Self::differential_port_voltages(initial_solution, &tl.far_nodes, tl.far_ref);

            // Seed the ngspice-faithful native convolution runtime from the DC
            // operating point. When active, the line uses the branch-current
            // transient stamp instead of the modal Norton path below, but we
            // still populate the modal reference state to keep the per-line
            // bookkeeping uniform and harmless for native lines.
            if tl.uses_native_runtime()
                && let Err(err) = tl.native_seed_dc(&near_physical, &far_physical)
            {
                log::warn!("{err}");
            }

            let near_modal = tl.modalize_port_voltage(&near_physical);
            let far_modal = tl.modalize_port_voltage(&far_physical);
            let near_currents = tl.port_currents(&near_physical, &far_modal);
            let far_currents = tl.port_currents(&far_physical, &near_modal);
            let near_modal_currents = tl.modalize_port_current(&near_currents);
            let far_modal_currents = tl.modalize_port_current(&far_currents);
            tl.update_modal_history(
                initial_time,
                &near_modal,
                &near_modal_currents,
                &far_modal,
                &far_modal_currents,
            );
            refs.push(CoupledTlineReferenceState {
                near_modal,
                far_modal,
            });
        }
        refs
    }

    #[inline]
    pub(super) fn recover_timestep_after_accepted_step(
        timestep: &mut TimestepController,
        lte_estimator: &LteEstimator,
        accepted_solution: &[Value],
        dt: Value,
        max_step: Value,
        is_strictly_linear_transient: bool,
        expected_source_delta: Value,
        source_activity_growth_cap_enabled: bool,
        accepted_scale: Option<Value>,
    ) {
        let scale = if is_strictly_linear_transient {
            1.0
        } else if let Some(scale) = accepted_scale {
            scale
        } else {
            let (lte, _) = lte_estimator.estimate(accepted_solution, dt);
            lte_estimator.recommend_scale(lte)
        };

        let growth_limit = if source_activity_growth_cap_enabled {
            1.5
        } else {
            2.0
        };
        let mut next_dt = if scale > 1.0 {
            (dt * scale.min(growth_limit)).min(max_step)
        } else {
            (dt * 1.25).min(max_step)
        };
        if source_activity_growth_cap_enabled
            && expected_source_delta.is_finite()
            && expected_source_delta > 0.0
        {
            let source_cap = dt * (SOURCE_ACTIVE_DELTA / expected_source_delta).clamp(1.0, 4.0);
            next_dt = next_dt.min(source_cap);
        }
        timestep.force_step(next_dt);
    }

    #[inline]
    pub(super) fn nonconvergence_retry_timestep(dt: Value, max_step: Value) -> Value {
        if !dt.is_finite() || dt <= 0.0 {
            0.0
        } else {
            // Mirror ngspice's DCtran non-convergence recovery: retract the
            // failed timepoint and retry at one eighth of the rejected step.
            (dt * 0.125).min(max_step)
        }
    }

    #[inline]
    pub(super) fn apply_retry_timestep_floor(
        proposed_dt: Value,
        retry_floor_dt: Option<Value>,
        rejected_dt: Value,
        max_step: Value,
    ) -> Value {
        let rejected_cap = if rejected_dt.is_finite() && rejected_dt > 0.0 {
            rejected_dt.min(max_step)
        } else {
            max_step
        };
        let mut dt = proposed_dt.min(rejected_cap);
        if let Some(floor) = retry_floor_dt
            .filter(|floor| floor.is_finite() && *floor > 0.0)
            .map(|floor| floor.min(rejected_cap))
            .filter(|floor| *floor < rejected_cap * 0.999)
        {
            dt = dt.max(floor);
        }
        dt
    }

    #[inline]
    pub(super) fn is_at_effective_retry_minimum(
        timestep: &TimestepController,
        _retry_floor_dt: Option<Value>,
    ) -> bool {
        // The BJT retry floor is intentionally soft: it may damp the first
        // retreat from a large failed step, but it must not masquerade as the
        // solver's true minimum and trigger force-accept before Newton has had
        // a chance to retry at smaller physical timesteps.
        timestep.is_at_minimum()
    }

    #[inline]
    pub(super) fn should_skip_post_accept_timestep_control_on_first_step(
        accepted_point_count_before_push: usize,
    ) -> bool {
        // ngspice accepts the first transient point without any post-accept
        // truncation/LTE check, then retries the same delta on the next step.
        accepted_point_count_before_push <= 1
    }

    #[inline]
    pub(super) fn force_accept_recovery_timestep(
        dt: Value,
        preferred_min_dt: Value,
        max_step: Value,
        vbic_exact_limit: Option<Value>,
    ) -> Value {
        let mut next_dt = if dt.is_finite() && dt > 0.0 {
            if dt < preferred_min_dt {
                (dt * preferred_min_dt)
                    .sqrt()
                    .max(dt)
                    .min(preferred_min_dt)
                    .min(max_step)
            } else {
                (dt * 0.5).max(preferred_min_dt).min(max_step)
            }
        } else {
            preferred_min_dt.min(max_step)
        };
        if let Some(limit) =
            vbic_exact_limit.filter(|limit| limit.is_finite() && *limit > dt * 1.001)
        {
            next_dt = next_dt.min(limit.min(max_step));
        }
        next_dt
    }

    #[inline]
    pub(super) fn limit_transient_node_voltage_updates(
        proposal: &mut [Value],
        previous: &[Value],
        num_nodes: usize,
        delta_limit: Value,
        protected_nodes: &[bool],
    ) -> bool {
        let mut changed = false;
        for i in 0..num_nodes {
            if protected_nodes.get(i).copied().unwrap_or(false) {
                continue;
            }
            let old = previous[i];
            let delta = proposal[i] - old;
            if delta.is_finite() && delta.abs() > delta_limit {
                proposal[i] = old + delta.signum() * delta_limit;
                changed = true;
            }
        }
        changed
    }

    #[inline]
    pub(super) fn bounded_force_accept_candidate(
        circuit: &crate::circuit::Circuit,
        previous_solution: &[Value],
        candidate_solution: &[Value],
        accepted_time: Value,
        num_nodes: usize,
        force_accept_delta_limit: Value,
        protected_nodes: &[bool],
        ideal_output_pairs: &[(crate::NodeId, crate::NodeId)],
    ) -> Vec<Value> {
        let mut bounded = candidate_solution.to_vec();
        for i in 0..num_nodes {
            if protected_nodes.get(i).copied().unwrap_or(false) {
                continue;
            }
            let old = previous_solution[i];
            let delta = bounded[i] - old;
            if delta.is_finite() && delta.abs() > force_accept_delta_limit {
                bounded[i] = old + delta.signum() * force_accept_delta_limit;
            }
        }
        circuit.enforce_ideal_voltage_constraints(&mut bounded, accepted_time);
        // Force-accept is a last-resort recovery path, so keep every ideal
        // output supernode close to the previous accepted common mode instead of
        // letting protected source nodes drag a nonphysical midpoint into the
        // newly accepted state.
        Self::clip_ideal_output_common_modes(
            previous_solution,
            &mut bounded,
            force_accept_delta_limit,
            ideal_output_pairs,
        );
        Self::restore_algebraic_branch_currents(
            circuit,
            previous_solution,
            &mut bounded,
            num_nodes,
        );
        bounded
    }

    #[inline]
    pub(super) fn restore_algebraic_branch_currents(
        circuit: &crate::circuit::Circuit,
        previous_solution: &[Value],
        candidate_solution: &mut [Value],
        num_nodes: usize,
    ) {
        let mut restore_branch = |branch_ordinal: usize| {
            if branch_ordinal == 0 {
                return;
            }
            let Some(solution_idx) = num_nodes.checked_add(branch_ordinal - 1) else {
                return;
            };
            let Some(previous_value) = previous_solution.get(solution_idx).copied() else {
                return;
            };
            if let Some(candidate_value) = candidate_solution.get_mut(solution_idx) {
                *candidate_value = previous_value;
            }
        };

        for &branch_ordinal in &circuit.voltage_sources.branch_indices {
            restore_branch(branch_ordinal);
        }
        // Keep dependent-source algebraic currents from the latest solver
        // candidate. Their output-branch currents directly close KCL at the
        // controlled output nodes, so blindly snapping them back to the
        // previously accepted state can inject a large node residual even when
        // the committed output voltage satisfies the ideal source relation.
    }

    #[inline]
    pub(super) fn clip_ideal_output_common_modes(
        previous_solution: &[Value],
        candidate_solution: &mut [Value],
        common_mode_delta_limit: Value,
        ideal_output_pairs: &[(crate::NodeId, crate::NodeId)],
    ) {
        for &(node_pos, node_neg) in ideal_output_pairs {
            Self::clip_two_terminal_common_mode(
                previous_solution,
                candidate_solution,
                node_pos,
                node_neg,
                common_mode_delta_limit,
            );
        }
    }

    #[inline]
    pub(super) fn clip_two_terminal_common_mode(
        previous_solution: &[Value],
        candidate_solution: &mut [Value],
        node_pos: usize,
        node_neg: usize,
        common_mode_delta_limit: Value,
    ) {
        if common_mode_delta_limit <= 0.0 || node_pos == 0 || node_neg == 0 {
            return;
        }

        let vp_idx = node_pos - 1;
        let vn_idx = node_neg - 1;
        if vp_idx >= previous_solution.len()
            || vn_idx >= previous_solution.len()
            || vp_idx >= candidate_solution.len()
            || vn_idx >= candidate_solution.len()
        {
            return;
        }

        let prev_vp = previous_solution[vp_idx];
        let prev_vn = previous_solution[vn_idx];
        let cand_vp = candidate_solution[vp_idx];
        let cand_vn = candidate_solution[vn_idx];
        if !(prev_vp.is_finite()
            && prev_vn.is_finite()
            && cand_vp.is_finite()
            && cand_vn.is_finite())
        {
            return;
        }

        let prev_midpoint = 0.5 * (prev_vp + prev_vn);
        let cand_midpoint = 0.5 * (cand_vp + cand_vn);
        let midpoint_delta = cand_midpoint - prev_midpoint;
        if midpoint_delta.abs() <= common_mode_delta_limit {
            return;
        }

        let clipped_midpoint = prev_midpoint + midpoint_delta.signum() * common_mode_delta_limit;
        let half_diff = 0.5 * (cand_vp - cand_vn);
        candidate_solution[vp_idx] = clipped_midpoint + half_diff;
        candidate_solution[vn_idx] = clipped_midpoint - half_diff;
    }

    #[inline]
    pub(super) fn update_reactive_history(
        circuit: &mut crate::circuit::Circuit,
        accepted_solution: &[Value],
        accepted_time: Value,
        dt: Value,
        method: IntegrationMethod,
        trap_order: u8,
        bjt_history: &mut BjtTransientHistory,
        jfet_history: &mut JfetTransientHistory,
        diode_history: &mut DiodeTransientHistory,
        mosfet_history: &mut MosfetTransientHistory,
        vdmos_history: &mut VdmosTransientHistory,
        b3soi_history: &mut B3SoiTransientHistory,
        bsim3_history: &mut Bsim3TransientHistory,
        bsim4_history: &mut Bsim4TransientHistory,
        vbic_snapshots: Option<&[Option<BjtChargeSnapshot>]>,
        mosfet_caps: Option<&[(Value, Value, Value)]>,
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
    ) {
        for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
            let np = cap.pp.row;
            let nn = cap.nn.row;
            let v_new = Self::differential_voltage(accepted_solution, np, nn);

            // Compute new capacitor current from OLD history before rotating it.
            let coeff_update = CompanionCoefficients::for_method(Self::effective_companion_method(
                method, trap_order,
            ));
            let geq = coeff_update.capacitor_geq(circuit.capacitors.capacitances[cap_idx], dt);
            let ieq = coeff_update.capacitor_ieq(
                circuit.capacitors.capacitances[cap_idx],
                dt,
                circuit.capacitors.v_prev[cap_idx],
                circuit.capacitors.v_prev_prev[cap_idx],
                circuit.capacitors.i_prev[cap_idx],
            );
            let i_new = geq * v_new - ieq;

            let v_old = circuit.capacitors.v_prev[cap_idx];
            circuit.capacitors.v_prev_prev_prev[cap_idx] = circuit.capacitors.v_prev_prev[cap_idx];
            circuit.capacitors.v_prev_prev[cap_idx] = v_old;
            circuit.capacitors.v_prev[cap_idx] = v_new;
            circuit.capacitors.i_prev[cap_idx] = i_new;
        }

        for l_idx in 0..circuit.inductors.names.len() {
            let br = circuit.inductors.branch_indices[l_idx];
            if br > 0 {
                let br_idx = circuit.num_nodes() + br - 1;
                let i_new = accepted_solution[br_idx];
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

        // Update transmission-line delayed-wave history from the accepted state.
        for (idx, tl) in circuit.tlines.iter_mut().enumerate() {
            let previous_forward = tl.launched_forward_wave();
            let previous_backward = tl.launched_backward_wave();
            let v1 = Self::differential_voltage(accepted_solution, tl.node1_pos, tl.node1_neg);
            let v2 = Self::differential_voltage(accepted_solution, tl.node2_pos, tl.node2_neg);
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

        let effective_method = Self::effective_companion_method(method, trap_order);
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
                        effective_method,
                        trap_order,
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
                method,
                trap_order,
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
                let cq_curr = Self::jfet_companion_ccap(
                    effective_method,
                    trap_order,
                    dt,
                    charge,
                    q_prev,
                    q_prev_prev,
                    cq_prev,
                );
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
                        method,
                        trap_order,
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
                        method,
                        trap_order,
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
                        method,
                        trap_order,
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
                        method,
                        trap_order,
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
                    method,
                    trap_order,
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
            let (qd, capd) = diode.junction_charge_and_capacitance(vd);
            diode_history.vd_prev_prev[idx] = diode_history.vd_prev[idx];
            diode_history.vd_prev[idx] = vd;
            if capd.is_finite() && capd > 0.0 {
                let (_geq, _ieq, qd_curr, cqd_curr) = Self::nonlinear_charge_companion_terms(
                    effective_method,
                    trap_order,
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

        for (idx, mos) in circuit.mosfets.devices.iter().enumerate() {
            let (vgs, vds, vbs) = mos.eval_branch_voltages_at(accepted_solution);
            let vgd = vgs - vds;
            let vgb = vgs - vbs;
            // The truncation walk already evaluated the Meyer halves on this
            // accepted solution; reuse them when the caller captured them.
            let (cgs_half, cgd_half, cgb_half) = match mosfet_caps {
                Some(cache) => cache[idx],
                None => mos.transient_capacitance_halves_at(vgs, vds, vbs),
            };
            let (cgs_ov, cgd_ov, cgb_ov) = mos.overlap_capacitances();
            let cgs = cgs_half + mosfet_history.capgs_prev_half[idx] + cgs_ov;
            let cgd = cgd_half + mosfet_history.capgd_prev_half[idx] + cgd_ov;
            let cgb = cgb_half + mosfet_history.capgb_prev_half[idx] + cgb_ov;
            mosfet_history.vgs_prev_prev[idx] = mosfet_history.vgs_prev[idx];
            mosfet_history.vgs_prev[idx] = vgs;
            mosfet_history.capgs_prev_half[idx] = cgs_half;
            mosfet_history.vgd_prev_prev[idx] = mosfet_history.vgd_prev[idx];
            mosfet_history.vgd_prev[idx] = vgd;
            mosfet_history.capgd_prev_half[idx] = cgd_half;
            mosfet_history.vgb_prev_prev[idx] = mosfet_history.vgb_prev[idx];
            mosfet_history.vgb_prev[idx] = vgb;
            mosfet_history.capgb_prev_half[idx] = cgb_half;
            if !suppress_gate_charge_history {
                let (_geq_gs, _ieq_gs, qgs_curr, cqgs_curr) = Self::jfet_companion_terms(
                    method,
                    trap_order,
                    dt,
                    cgs,
                    vgs,
                    mosfet_history.vgs_prev_prev[idx],
                    mosfet_history.qgs_prev[idx],
                    mosfet_history.qgs_prev_prev[idx],
                    mosfet_history.cqgs_prev[idx],
                );
                mosfet_history.qgs_prev_prev_prev[idx] = mosfet_history.qgs_prev_prev[idx];
                mosfet_history.qgs_prev_prev[idx] = mosfet_history.qgs_prev[idx];
                mosfet_history.qgs_prev[idx] = qgs_curr;
                mosfet_history.cqgs_prev[idx] = cqgs_curr;

                let (_geq_gd, _ieq_gd, qgd_curr, cqgd_curr) = Self::jfet_companion_terms(
                    method,
                    trap_order,
                    dt,
                    cgd,
                    vgd,
                    mosfet_history.vgd_prev_prev[idx],
                    mosfet_history.qgd_prev[idx],
                    mosfet_history.qgd_prev_prev[idx],
                    mosfet_history.cqgd_prev[idx],
                );
                mosfet_history.qgd_prev_prev_prev[idx] = mosfet_history.qgd_prev_prev[idx];
                mosfet_history.qgd_prev_prev[idx] = mosfet_history.qgd_prev[idx];
                mosfet_history.qgd_prev[idx] = qgd_curr;
                mosfet_history.cqgd_prev[idx] = cqgd_curr;

                let (_geq_gb, _ieq_gb, qgb_curr, cqgb_curr) = Self::jfet_companion_terms(
                    method,
                    trap_order,
                    dt,
                    cgb,
                    vgb,
                    mosfet_history.vgb_prev_prev[idx],
                    mosfet_history.qgb_prev[idx],
                    mosfet_history.qgb_prev_prev[idx],
                    mosfet_history.cqgb_prev[idx],
                );
                mosfet_history.qgb_prev_prev_prev[idx] = mosfet_history.qgb_prev_prev[idx];
                mosfet_history.qgb_prev_prev[idx] = mosfet_history.qgb_prev[idx];
                mosfet_history.qgb_prev[idx] = qgb_curr;
                mosfet_history.cqgb_prev[idx] = cqgb_curr;
            }

            let vbs_j = mos.body_source_charge_branch_voltage(vbs);
            let vbd_j = mos.body_drain_charge_branch_voltage(vds, vbs);
            let (qbs_exact, cbs) = mos.body_source_junction_charge_and_capacitance_at(vbs);
            let (_geq_bs, _ieq_bs, qbs_curr, cqbs_curr) = Self::nonlinear_charge_companion_terms(
                method,
                trap_order,
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

            let (qbd_exact, cbd) = mos.body_drain_junction_charge_and_capacitance_at(vds, vbs);
            let (_geq_bd, _ieq_bd, qbd_curr, cqbd_curr) = Self::nonlinear_charge_companion_terms(
                method,
                trap_order,
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
                method,
                trap_order,
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
                method,
                trap_order,
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
                method,
                trap_order,
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
                method,
                trap_order,
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
                method,
                trap_order,
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
                method,
                trap_order,
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
                method,
                trap_order,
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

        Self::update_b3soi_history(
            circuit,
            accepted_solution,
            method,
            trap_order,
            dt,
            b3soi_history,
        );
        Self::update_bsim3_history(
            circuit,
            accepted_solution,
            method,
            trap_order,
            dt,
            bsim3_history,
        );
        Self::update_bsim4_history(
            circuit,
            accepted_solution,
            method,
            trap_order,
            dt,
            bsim4_history,
        );
    }
}

/// Adapter exposing the transient [`StaticMatrix`] + RHS pair as a
/// [`MatrixStamper`] for devices that stamp through the generic trait (the
/// B3SOIDD charge companion). Maps 1-indexed device NodeIds to the 0-indexed
/// matrix/RHS, matching `CircuitData`'s own stamper convention.
struct StaticMatrixChargeStamper<'a> {
    matrix: &'a mut crate::solver::StaticMatrix,
    rhs: &'a mut [Value],
}

impl crate::device::MatrixStamper for StaticMatrixChargeStamper<'_> {
    #[inline]
    fn stamp(&mut self, row: crate::circuit::NodeId, col: crate::circuit::NodeId, value: Value) {
        if row > 0 && col > 0 {
            self.matrix.add(row - 1, col - 1, value);
        }
    }

    #[inline]
    fn stamp_rhs(&mut self, index: crate::circuit::NodeId, value: Value) {
        if index > 0 && index <= self.rhs.len() {
            self.rhs[index - 1] += value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Netlist;

    #[test]
    fn pvdmos_companion_slots_follow_polarity_normalized_charge_voltages() {
        let deck = "\
PVDMOS charge slot orientation
VD d 0 -0.5
VG g 0 0
VS s 0 0
M1 d g s 0 PM W=1 L=1u
.MODEL PM PMOS LEVEL=18
+ VTO=-100
+ RD=0
+ RS=0
+ RG=0
+ CGDO=1e-11
+ CGSO=1e-11
+ CGBO=1e-11
+ CBD=0
+ CBS=0
+ D1CJO=1e-12
+ D1TT=0
.OP
.END
";
        let netlist = Netlist::parse(deck).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let mut circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let matrix = engine.build_matrix(&circuit).expect("matrix builds");
        circuit.link_indices(&matrix);

        let slots = Engine::link_vdmos_companion_slots(&circuit, &matrix);
        let vdmos = &circuit.vdmoses.devices[0];
        let di = vdmos.drain_int.unwrap_or(vdmos.drain);
        let si = vdmos.source_int.unwrap_or(vdmos.source);
        let d1p = vdmos.d1_prime.unwrap_or(vdmos.source);

        let expected = [
            (si, vdmos.gate),
            (di, vdmos.gate),
            (vdmos.bulk, vdmos.gate),
            (si, di),
            (si, vdmos.bulk),
            (di, vdmos.bulk),
            (d1p, vdmos.drain),
        ];

        for (branch, (slot, expected_nodes)) in slots[0].iter().zip(expected).enumerate() {
            assert_eq!(
                (slot.pos, slot.neg),
                expected_nodes,
                "PVDMOS companion branch {branch} must be oriented with the polarity-normalized charge voltage"
            );
        }
    }
}
