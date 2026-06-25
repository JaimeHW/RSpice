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
        ekv26_history: &mut Ekv26TransientHistory,
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
        *ekv26_history = Self::initialize_ekv26_history(circuit, solution);
        ekv26_history.accepted_dt_prev = hinted_max_step;
        ekv26_history.accepted_dt_prev_prev = hinted_max_step;
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
