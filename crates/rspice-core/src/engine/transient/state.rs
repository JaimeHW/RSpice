//! Reactive companion state and transient recovery helpers.

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
            let charge_snapshot = bjt.charge_snapshot(vc, vb, ve, vs);
            let (history_vbe, history_vbc, history_vcs) = if bjt.uses_vbic_dynamic_charges() {
                (vbe, vbc, vcs)
            } else {
                Self::legacy_bjt_charge_branch_voltages(&charge_snapshot)
            };
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
            if !bjt.uses_vbic_dynamic_charges() {
                let (legacy_vbe, legacy_vbc, legacy_vbx, legacy_vcs) =
                    Self::legacy_bjt_charge_branch_voltages_with_vbx(&charge_snapshot);
                let charges = bjt.legacy_transient_charge_state_with_vbx(
                    legacy_vbe, legacy_vbc, legacy_vbx, legacy_vcs,
                );
                charge_values[BJT_QBE_BRANCH_INDEX] = charges.qbe;
                charge_values[BJT_QBC_BRANCH_INDEX] = charges.qbc;
                charge_values[BJT_QBCX_BRANCH_INDEX] = charges.qbx;
                charge_values[BJT_QBCP_BRANCH_INDEX] = charges.qcs;
            }
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
            cqds_prev: Vec::with_capacity(n),
            accepted_dt_prev: 0.0,
            accepted_dt_prev_prev: 0.0,
        };

        for jfet in &circuit.jfets {
            let (vgs_eval, vgd_eval) = Self::jfet_branch_voltages(jfet, solution);
            let (vgs_charge, vgd_charge) = Self::jfet_charge_branch_voltages(jfet, solution);
            let (cgs, cgd) = jfet.transient_capacitances(vgs_eval, vgd_eval, jfet.params.tnom);
            let cds = jfet.transient_drain_source_capacitance();
            let vds_charge = vgs_eval - vgd_eval;
            let qgs = cgs.max(0.0) * vgs_charge;
            let qgd = cgd.max(0.0) * vgd_charge;
            let qds = cds.max(0.0) * vds_charge;
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
            history.cqds_prev.push(0.0);
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

            if bjt.uses_vbic_dynamic_charges() && charge_factor > 0.0 {
                let (snapshot_reuse_abstol, snapshot_reuse_reltol) =
                    Self::vbic_runtime_snapshot_reuse_tolerances(voltage_abstol, reltol);
                let cached_snapshot = vbic_snapshot_cache.get(idx).copied().flatten();
                let snapshot_start = std::time::Instant::now();
                // Stamping-time snapshot resolves run under the same
                // deterministic evaluation budget as continuation solves: a
                // mid-Newton iterate can request a wildly off-bias snapshot
                // (multi-volt forward junctions), and an unbounded inner solve
                // there burns >10s per device before the outer loop gets a
                // chance to reject the iterate and cut dt.
                let Some(snapshot) = Self::with_vbic_best_effort_eval_budget(
                    Self::VBIC_CONTINUATION_EVAL_BUDGET,
                    || {
                        Self::resolve_vbic_snapshot_for_external_bias_with_linear_history(
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
                        )
                    },
                ) else {
                    vbic_snapshot_cache[idx] = None;
                    continue;
                };
                let snapshot_elapsed = snapshot_start.elapsed();
                static VBIC_SNAPSHOT_RESOLVE_LOG_COUNT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                if snapshot_elapsed.as_millis() >= 100 {
                    let log_count = VBIC_SNAPSHOT_RESOLVE_LOG_COUNT
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if log_count < 40 {
                        log::warn!(
                            "Slow VBIC snapshot resolve {} dt={:.3e} trap_order={} ext=({:.6e}, {:.6e}, {:.6e}, {:.6e}) cached={} elapsed={:.3?}",
                            bjt.name,
                            dt,
                            trap_order,
                            vc,
                            vb,
                            ve,
                            vs,
                            cached_snapshot.is_some(),
                            snapshot_elapsed,
                        );
                    }
                }
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
                let base_static_g = snapshot.reduction.g_reduced;
                vbic_snapshot_cache[idx] = Some(snapshot);

                let Some((y_total, reduced_i_eq)) =
                    Self::vbic_reduce_transient_external_system(&linearization)
                else {
                    vbic_snapshot_cache[idx] = None;
                    continue;
                };
                let (_base_static_g, base_static_i_eq) = Self::vbic_static_stamped_external_system(
                    bjt,
                    &snapshot.reduction.external_voltages,
                );

                let mut delta = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
                let mut delta_i_eq = [0.0; BJT_EXTERNAL_STATE_DIM];
                for row in 0..BJT_EXTERNAL_STATE_DIM {
                    delta_i_eq[row] = reduced_i_eq[row] - base_static_i_eq[row];
                    for col in 0..BJT_EXTERNAL_STATE_DIM {
                        delta[row][col] = y_total[row][col] - base_static_g[row][col];
                    }
                }
                let max_delta_i_eq = delta_i_eq
                    .iter()
                    .map(|value| value.abs())
                    .fold(0.0, Value::max);
                static VBIC_DELTA_LOG_COUNT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                let delta_log_count =
                    VBIC_DELTA_LOG_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if max_delta_i_eq > 1.0 && delta_log_count < 20 {
                    log::warn!(
                        "VBIC transient delta {} max|di_eq|={:.3e}: total={:?} static={:?} delta={:?} xf=({:.3e}, {:.3e}) vrth={:.3e}",
                        bjt.name,
                        max_delta_i_eq,
                        reduced_i_eq,
                        base_static_i_eq,
                        delta_i_eq,
                        snapshot.reduction.internal_voltages[BJT_DELAY_XF1_STATE_INDEX],
                        snapshot.reduction.internal_voltages[BJT_DELAY_XF2_STATE_INDEX],
                        snapshot.reduction.internal_voltages[BJT_THERMAL_STATE_INDEX],
                    );
                }
                let nodes = [
                    bjt.node_collector,
                    bjt.node_base,
                    bjt.node_emitter,
                    bjt.node_substrate,
                ];
                Self::stamp_external_reduced_system(matrix, rhs, &nodes, &delta, &delta_i_eq);
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
            let (cgs, cgd) = jfet.transient_capacitances(vgs_eval, vgd_eval, jfet.params.tnom);
            let cds = jfet.transient_drain_source_capacitance();
            let vds_charge = vgs_eval - vgd_eval;

            if !suppress_gate_charge && cgs.is_finite() && cgs > 0.0 {
                let (geq, ieq, _q_curr, _cq_curr) = Self::jfet_companion_terms(
                    effective_method,
                    trap_order,
                    dt,
                    cgs,
                    vgs_charge,
                    history.vgs_prev[idx],
                    history.qgs_prev[idx],
                    history.qgs_prev_prev[idx],
                    history.cqgs_prev[idx],
                );
                Self::stamp_two_terminal_companion(matrix, rhs, jfet.gate, jfet.source, geq, ieq);
            }

            if !suppress_gate_charge && cgd.is_finite() && cgd > 0.0 {
                let (geq, ieq, _q_curr, _cq_curr) = Self::jfet_companion_terms(
                    effective_method,
                    trap_order,
                    dt,
                    cgd,
                    vgd_charge,
                    history.vgd_prev[idx],
                    history.qgd_prev[idx],
                    history.qgd_prev_prev[idx],
                    history.cqgd_prev[idx],
                );
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
    ) {
        let effective_method = Self::effective_companion_method(method, trap_order);
        for (idx, mos) in circuit.mosfets.devices.iter().enumerate() {
            let (vgs_eval, vds_eval, vbs_eval) = mos.eval_branch_voltages_at(voltages);
            let (vgs, vgd, vgb) = mos.gate_charge_branch_voltages_at(voltages);
            let (cgs_half, cgd_half, cgb_half) =
                mos.transient_capacitance_halves_at(vgs_eval, vds_eval, vbs_eval);
            let (cgs_ov, cgd_ov, cgb_ov) = mos.overlap_capacitances();
            let cgs = cgs_half + history.capgs_prev_half[idx] + cgs_ov;
            let cgd = cgd_half + history.capgd_prev_half[idx] + cgd_ov;
            let cgb = cgb_half + history.capgb_prev_half[idx] + cgb_ov;

            if !suppress_gate_charge {
                let (geq_gs, ieq_gs, _qgs_curr, _cqgs_curr) = Self::jfet_companion_terms(
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
                if geq_gs > 0.0 {
                    Self::stamp_two_terminal_companion(
                        matrix,
                        rhs,
                        mos.node_gate,
                        mos.node_source,
                        geq_gs,
                        ieq_gs,
                    );
                }

                let (geq_gd, ieq_gd, _qgd_curr, _cqgd_curr) = Self::jfet_companion_terms(
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
                if geq_gd > 0.0 {
                    Self::stamp_two_terminal_companion(
                        matrix,
                        rhs,
                        mos.node_gate,
                        mos.node_drain,
                        geq_gd,
                        ieq_gd,
                    );
                }

                let (geq_gb, ieq_gb, _qgb_curr, _cqgb_curr) = Self::jfet_companion_terms(
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
                if geq_gb > 0.0 {
                    Self::stamp_two_terminal_companion(
                        matrix,
                        rhs,
                        mos.node_gate,
                        mos.node_bulk,
                        geq_gb,
                        ieq_gb,
                    );
                }
            }

            let vbs_j = mos.body_source_charge_branch_voltage(vbs_eval);
            let vbd_j = mos.body_drain_charge_branch_voltage(vds_eval, vbs_eval);
            let (qbs_curr, cbs) = mos.body_source_junction_charge_and_capacitance_at(vbs_eval);
            let (qbd_curr, cbd) =
                mos.body_drain_junction_charge_and_capacitance_at(vds_eval, vbs_eval);
            let (bs_pos, bs_neg) = mos.body_source_charge_nodes();
            let (bd_pos, bd_neg) = mos.body_drain_charge_nodes();

            let (geq_bs, ieq_bs, _qbs_curr, _cqbs_curr) = Self::nonlinear_charge_companion_terms(
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
            if geq_bs > 0.0 {
                Self::stamp_two_terminal_companion(matrix, rhs, bs_pos, bs_neg, geq_bs, ieq_bs);
            }

            let (geq_bd, ieq_bd, _qbd_curr, _cqbd_curr) = Self::nonlinear_charge_companion_terms(
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
            if geq_bd > 0.0 {
                Self::stamp_two_terminal_companion(matrix, rhs, bd_pos, bd_neg, geq_bd, ieq_bd);
            }
        }
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
            accepted_dt_prev: 0.0,
            accepted_dt_prev_prev: 0.0,
        };
        let mut seed = |qg: Value, qb: Value, qd: Value, qe: Value| {
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
        };
        // The history is indexed DD devices first, then FD, then PD; the
        // stamp/commit/truncation walks use the same concatenated order.
        for dev in &circuit.b3soi.devices {
            let c = dev.charge_at(solution);
            seed(c.qg, c.qb, c.qd, c.qe);
        }
        for dev in &circuit.b3soi_fd.devices {
            let c = dev.charge_at(solution);
            seed(c.qg, c.qb, c.qd, c.qe);
        }
        for dev in &circuit.b3soi_pd.devices {
            let c = dev.charge_at(solution);
            seed(c.qg, c.qb, c.qd, c.qe);
        }
        h
    }

    /// Integrate one SOI device's four node charges with the engine
    /// coefficient and its per-charge history slot, yielding the equivalent
    /// charge currents `(cqg, cqb, cqd, cqe)`.
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
    ) -> (Value, Value, Value, Value) {
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
        cqg: Value,
        cqb: Value,
        cqd: Value,
        cqe: Value,
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
    }

    /// Stamp the B3SOIDD transient charge companion for every SOI instance.
    ///
    /// Integrates each of the four coupled node charges with the engine's
    /// integration coefficient `ag0` and the per-charge history, then stamps the
    /// coupled `gc**·ag0` capacitance matrix plus the `ceqq*` equivalent charge
    /// currents (b3soiddld.c charge load). KCL-symmetric: the device routes the
    /// full 5-terminal coupling so charge is conserved across g/b/d/s/e.
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
            let charge = dev.charge_at(voltages);
            let (cqg, cqb, cqd, cqe) = Self::b3soi_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                charge.qg,
                charge.qb,
                charge.qd,
                charge.qe,
            );
            dev.stamp_charge_companion(
                &charge, ag0, cqg, cqb, cqd, cqe, voltages, &mut stamper,
            );
            idx += 1;
        }
        for dev in &circuit.b3soi_fd.devices {
            let charge = dev.charge_at(voltages);
            let (cqg, cqb, cqd, cqe) = Self::b3soi_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                charge.qg,
                charge.qb,
                charge.qd,
                charge.qe,
            );
            dev.stamp_charge_companion(
                &charge, ag0, cqg, cqb, cqd, cqe, voltages, &mut stamper,
            );
            idx += 1;
        }
        for dev in &circuit.b3soi_pd.devices {
            let charge = dev.charge_at(voltages);
            let (cqg, cqb, cqd, cqe) = Self::b3soi_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                charge.qg,
                charge.qb,
                charge.qd,
                charge.qe,
            );
            dev.stamp_charge_companion(
                &charge, ag0, cqg, cqb, cqd, cqe, voltages, &mut stamper,
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
            let charge = dev.charge_at(voltages);
            let (cqg, cqb, cqd, cqe) = Self::b3soi_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                charge.qg,
                charge.qb,
                charge.qd,
                charge.qe,
            );
            Self::b3soi_commit_history_slot(
                history, idx, charge.qg, charge.qb, charge.qd, charge.qe, cqg, cqb, cqd, cqe,
            );
            idx += 1;
        }
        for dev in &circuit.b3soi_fd.devices {
            let charge = dev.charge_at(voltages);
            let (cqg, cqb, cqd, cqe) = Self::b3soi_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                charge.qg,
                charge.qb,
                charge.qd,
                charge.qe,
            );
            Self::b3soi_commit_history_slot(
                history, idx, charge.qg, charge.qb, charge.qd, charge.qe, cqg, cqb, cqd, cqe,
            );
            idx += 1;
        }
        for dev in &circuit.b3soi_pd.devices {
            let charge = dev.charge_at(voltages);
            let (cqg, cqb, cqd, cqe) = Self::b3soi_companion_currents(
                effective_method,
                trap_order,
                dt,
                history,
                idx,
                charge.qg,
                charge.qb,
                charge.qd,
                charge.qe,
            );
            Self::b3soi_commit_history_slot(
                history, idx, charge.qg, charge.qb, charge.qd, charge.qe, cqg, cqb, cqd, cqe,
            );
            idx += 1;
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
            if tl.uses_native_runtime() {
                if let Err(err) = tl.native_seed_dc(&near_physical, &far_physical) {
                    log::warn!("{err}");
                }
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
    pub(super) fn limit_vbic_transient_external_updates(
        circuit: &crate::circuit::Circuit,
        proposal: &mut [Value],
        previous: &[Value],
        accepted: &[Value],
        num_nodes: usize,
        protected_nodes: &[bool],
        accepted_delta_limit: Value,
    ) -> bool {
        let mut changed = Self::limit_vbic_external_updates(
            circuit,
            proposal,
            previous,
            num_nodes,
            Some(protected_nodes),
            true,
        );
        if !std::ptr::eq(previous.as_ptr(), accepted.as_ptr()) {
            changed |= Self::limit_vbic_external_updates(
                circuit,
                proposal,
                accepted,
                num_nodes,
                Some(protected_nodes),
                true,
            );
        }
        if accepted_delta_limit.is_finite() && accepted_delta_limit > 0.0 {
            for bjt in &circuit.bjts.devices {
                if !bjt.uses_vbic_dynamic_charges() || bjt.td <= 0.0 {
                    continue;
                }
                for node in [
                    bjt.node_collector,
                    bjt.node_base,
                    bjt.node_emitter,
                    bjt.node_substrate,
                ] {
                    if node == 0 {
                        continue;
                    }
                    let proposal_idx = node - 1;
                    if proposal_idx >= num_nodes
                        || protected_nodes.get(proposal_idx).copied().unwrap_or(false)
                    {
                        continue;
                    }
                    let accepted_value = accepted[proposal_idx];
                    let proposal_value = proposal[proposal_idx];
                    let delta = proposal_value - accepted_value;
                    if !delta.is_finite() || delta.abs() <= accepted_delta_limit {
                        continue;
                    }
                    proposal[proposal_idx] = accepted_value + delta.signum() * accepted_delta_limit;
                    changed = true;
                }
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
            restore_branch(branch_ordinal as usize);
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
        mosfet_history: &mut MosfetTransientHistory,
        b3soi_history: &mut B3SoiTransientHistory,
        vbic_snapshots: Option<&[Option<BjtChargeSnapshot>]>,
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
                let far_physical = Self::differential_port_voltages(
                    accepted_solution,
                    &tl.far_nodes,
                    tl.far_ref,
                );
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
            if bjt.uses_vbic_dynamic_charges() {
                let snapshot_reuse_abstol = voltage_abstol.min(VBIC_HISTORY_SNAPSHOT_REUSE_ABSTOL);
                let snapshot_reuse_reltol = voltage_reltol.min(VBIC_HISTORY_SNAPSHOT_REUSE_RELTOL);
                let cached_snapshot = vbic_snapshots
                    .and_then(|cache| cache.get(idx))
                    .copied()
                    .flatten();
                let Some(snapshot) =
                    Self::resolve_vbic_snapshot_for_external_bias_with_linear_history(
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
                    )
                else {
                    continue;
                };
                for (branch_idx, branch) in snapshot.branches.iter().enumerate() {
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
                bjt_history.dynamic_internal_prev[idx] = snapshot.reduction.internal_voltages;
                let predictor_linear = Self::vbic_predictor_linear_branch_state(
                    bjt,
                    external,
                    snapshot.reduction.internal_voltages,
                );
                bjt_history.dynamic_linear_prev_prev[idx] = bjt_history.dynamic_linear_prev[idx];
                bjt_history.dynamic_linear_prev[idx] = predictor_linear;
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
            let (cgs, cgd) = jfet.transient_capacitances(vgs_eval, vgd_eval, jfet.params.tnom);
            let cds = jfet.transient_drain_source_capacitance();
            let vds_charge = vgs_eval - vgd_eval;
            jfet_history.vgs_prev_prev[idx] = jfet_history.vgs_prev[idx];
            jfet_history.vgs_prev[idx] = vgs_charge;
            jfet_history.vgd_prev_prev[idx] = jfet_history.vgd_prev[idx];
            jfet_history.vgd_prev[idx] = vgd_charge;
            jfet_history.vds_prev_prev[idx] = jfet_history.vds_prev[idx];
            jfet_history.vds_prev[idx] = vds_charge;
            if !suppress_gate_charge_history {
                let (_geq_gs, _ieq_gs, qgs_curr, cqgs_curr) = Self::jfet_companion_terms(
                    method,
                    trap_order,
                    dt,
                    cgs,
                    vgs_charge,
                    jfet_history.vgs_prev_prev[idx],
                    jfet_history.qgs_prev[idx],
                    jfet_history.qgs_prev_prev[idx],
                    jfet_history.cqgs_prev[idx],
                );
                jfet_history.qgs_prev_prev_prev[idx] = jfet_history.qgs_prev_prev[idx];
                jfet_history.qgs_prev_prev[idx] = jfet_history.qgs_prev[idx];
                jfet_history.qgs_prev[idx] = qgs_curr;
                jfet_history.cqgs_prev[idx] = cqgs_curr;

                let (_geq_gd, _ieq_gd, qgd_curr, cqgd_curr) = Self::jfet_companion_terms(
                    method,
                    trap_order,
                    dt,
                    cgd,
                    vgd_charge,
                    jfet_history.vgd_prev_prev[idx],
                    jfet_history.qgd_prev[idx],
                    jfet_history.qgd_prev_prev[idx],
                    jfet_history.cqgd_prev[idx],
                );
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
                jfet_history.qds_prev_prev[idx] = jfet_history.qds_prev[idx];
                jfet_history.qds_prev[idx] = qds_curr;
                jfet_history.cqds_prev[idx] = cqds_curr;
            }
        }
        jfet_history.accepted_dt_prev_prev = jfet_history.accepted_dt_prev;
        jfet_history.accepted_dt_prev = dt;

        for (idx, mos) in circuit.mosfets.devices.iter().enumerate() {
            let (vgs, vds, vbs) = mos.eval_branch_voltages_at(accepted_solution);
            let vgd = vgs - vds;
            let vgb = vgs - vbs;
            let (cgs_half, cgd_half, cgb_half) = mos.transient_capacitance_halves_at(vgs, vds, vbs);
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

        Self::update_b3soi_history(circuit, accepted_solution, method, trap_order, dt, b3soi_history);
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
