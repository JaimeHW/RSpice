//! Charge truncation and transient LTE control helpers.

use super::*;

impl Engine {
    #[inline]
    pub(super) fn collect_vbic_truncation_charge_state(
        circuit: &crate::circuit::Circuit,
        voltages: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &BjtTransientHistory,
        vbic_snapshot_cache: Option<&[Option<BjtChargeSnapshot>]>,
        snapshot_reuse_abstol: Value,
        snapshot_reuse_reltol: Value,
    ) -> Option<Vec<Value>> {
        let vbic_device_count = circuit
            .bjts
            .devices
            .iter()
            .filter(|bjt| bjt.uses_vbic_dynamic_charges())
            .count();
        if vbic_device_count == 0 {
            return None;
        }

        let mut charges =
            Vec::with_capacity(vbic_device_count.saturating_mul(BJT_VBIC_TRUNCATION_BRANCH_COUNT));

        for (idx, bjt) in circuit.bjts.devices.iter().enumerate() {
            if !bjt.uses_vbic_dynamic_charges() {
                continue;
            }

            let vc = Self::node_voltage(voltages, bjt.node_collector);
            let vb = Self::node_voltage(voltages, bjt.node_base);
            let ve = Self::node_voltage(voltages, bjt.node_emitter);
            let vs = Self::node_voltage(voltages, bjt.node_substrate);
            let snapshot = Self::resolve_vbic_snapshot_for_external_bias_with_linear_history(
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
                vbic_snapshot_cache.and_then(|cache| cache.get(idx).copied().flatten()),
                VbicCachedSnapshotReuse::SeedOnly,
                snapshot_reuse_abstol,
                snapshot_reuse_reltol,
            )?;

            charges.extend(
                snapshot.branches[..BJT_VBIC_TRUNCATION_BRANCH_COUNT]
                    .iter()
                    .map(|branch| branch.charge),
            );
        }

        Some(charges)
    }

    #[inline]
    pub(super) fn ngspice_vbic_truncation_factor(method: IntegrationMethod, order: u8) -> Value {
        match order.max(1) {
            1 => 0.5,
            2 => match method {
                IntegrationMethod::Gear2 => 0.222_222_222_2,
                _ => 0.083_333_333_33,
            },
            _ => 0.083_333_333_33,
        }
    }

    #[inline]
    pub(super) fn ngspice_charge_truncation_limit(
        q_curr: Value,
        q_prev: Value,
        q_prev_prev: Value,
        q_prev_prev_prev: Value,
        cq_curr: Value,
        cq_prev: Value,
        dt: Value,
        prev_dt: Value,
        prev_prev_dt: Value,
        method: IntegrationMethod,
        trap_order: u8,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        if !dt.is_finite() || dt <= 0.0 {
            return None;
        }

        let mut order = trap_order.clamp(1, 2);
        if !prev_dt.is_finite() || prev_dt <= 0.0 {
            return None;
        }
        if order >= 2 && (!prev_prev_dt.is_finite() || prev_prev_dt <= 0.0) {
            order = 1;
        }

        let volttol = current_abstol + reltol * cq_curr.abs().max(cq_prev.abs());
        let chargetol = reltol * q_curr.abs().max(q_prev.abs()).max(charge_abstol) / dt;
        let tol = volttol.max(chargetol);
        if !tol.is_finite() || tol <= 0.0 {
            return None;
        }

        let mut diff = [q_curr, q_prev, q_prev_prev, q_prev_prev_prev];
        let delta_old = [dt, prev_dt, prev_prev_dt];
        let mut deltmp = delta_old;
        let mut j = usize::from(order);
        loop {
            for i in 0..=j {
                let denom = deltmp[i];
                if !denom.is_finite() || denom <= 0.0 {
                    return None;
                }
                diff[i] = (diff[i] - diff[i + 1]) / denom;
            }
            if j == 0 {
                break;
            }
            j -= 1;
            for i in 0..=j {
                deltmp[i] = deltmp[i + 1] + delta_old[i];
            }
        }

        let factor = Self::ngspice_vbic_truncation_factor(method, order);
        let denom = current_abstol.max(factor * diff[0].abs());
        if !denom.is_finite() || denom <= 0.0 {
            return None;
        }

        if !trtol.is_finite() || trtol <= 0.0 {
            return None;
        }

        let mut limit = trtol * tol / denom;
        if order >= 2 {
            limit = limit.sqrt();
        }
        (limit.is_finite() && limit > 0.0).then_some(limit)
    }

    #[inline]
    pub(super) fn capacitor_ngspice_truncation_limit(
        circuit: &crate::circuit::Circuit,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        prev_dt: Value,
        prev_prev_dt: Value,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        if !prev_dt.is_finite() || prev_dt <= 0.0 {
            return None;
        }

        let effective_method = Self::effective_companion_method(method, trap_order);
        let coeff = CompanionCoefficients::for_method(effective_method);
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        for (idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
            let capacitance = circuit.capacitors.capacitances[idx];
            if !capacitance.is_finite() || capacitance <= 0.0 {
                continue;
            }

            let voltage = Self::differential_voltage(candidate_solution, cap.pp.row, cap.nn.row);
            let q_curr = capacitance * voltage;
            let q_prev = capacitance * circuit.capacitors.v_prev[idx];
            let q_prev_prev = capacitance * circuit.capacitors.v_prev_prev[idx];
            let q_prev_prev_prev = capacitance * circuit.capacitors.v_prev_prev_prev[idx];
            let geq = coeff.capacitor_geq(capacitance, dt);
            let ieq = coeff.capacitor_ieq(
                capacitance,
                dt,
                circuit.capacitors.v_prev[idx],
                circuit.capacitors.v_prev_prev[idx],
                circuit.capacitors.i_prev[idx],
            );
            let cq_curr = geq * voltage - ieq;
            let cq_prev = circuit.capacitors.i_prev[idx];

            let Some(branch_limit) = Self::ngspice_charge_truncation_limit(
                q_curr,
                q_prev,
                q_prev_prev,
                q_prev_prev_prev,
                cq_curr,
                cq_prev,
                dt,
                prev_dt,
                prev_prev_dt,
                effective_method,
                trap_order,
                reltol,
                current_abstol,
                charge_abstol,
                trtol,
            ) else {
                continue;
            };
            found_branch = true;
            limit = limit.min(branch_limit);
        }

        found_branch.then_some(limit)
    }

    #[inline]
    pub(super) fn vbic_ngspice_truncation_limit(
        circuit: &crate::circuit::Circuit,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &BjtTransientHistory,
        vbic_snapshot_cache: &[Option<BjtChargeSnapshot>],
        voltage_abstol: Value,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        let effective_method = Self::effective_companion_method(method, trap_order);
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        for (idx, bjt) in circuit.bjts.devices.iter().enumerate() {
            if !bjt.uses_vbic_dynamic_charges() {
                continue;
            }

            let vc = Self::node_voltage(candidate_solution, bjt.node_collector);
            let vb = Self::node_voltage(candidate_solution, bjt.node_base);
            let ve = Self::node_voltage(candidate_solution, bjt.node_emitter);
            let vs = Self::node_voltage(candidate_solution, bjt.node_substrate);
            let candidate_external = [vc, vb, ve, vs];
            let cached_snapshot =
                vbic_snapshot_cache
                    .get(idx)
                    .copied()
                    .flatten()
                    .filter(|snapshot| {
                        Self::vbic_snapshot_matches_external_bias_exact(
                            snapshot,
                            &candidate_external,
                        )
                    });
            let snapshot = if let Some(snapshot) = cached_snapshot {
                snapshot
            } else {
                let (snapshot_reuse_abstol, snapshot_reuse_reltol) =
                    Self::vbic_runtime_snapshot_reuse_tolerances(voltage_abstol, reltol);
                Self::resolve_vbic_snapshot_for_external_bias_with_linear_history(
                    bjt,
                    candidate_external,
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
                    vbic_snapshot_cache.get(idx).copied().flatten(),
                    VbicCachedSnapshotReuse::SeedOnly,
                    snapshot_reuse_abstol,
                    snapshot_reuse_reltol,
                )?
            };

            for branch_idx in 0..BJT_VBIC_TRUNCATION_BRANCH_COUNT {
                let q_curr = snapshot.branches[branch_idx].charge;
                let q_prev = history.charge_q_prev[idx][branch_idx];
                let q_prev_prev = history.charge_q_prev_prev[idx][branch_idx];
                let q_prev_prev_prev = history.charge_q_prev_prev_prev[idx][branch_idx];
                let cq_prev = history.charge_cq_prev[idx][branch_idx];
                let cq_curr = Self::jfet_companion_ccap(
                    effective_method,
                    trap_order,
                    dt,
                    q_curr,
                    q_prev,
                    q_prev_prev,
                    cq_prev,
                );

                let Some(branch_limit) = Self::ngspice_charge_truncation_limit(
                    q_curr,
                    q_prev,
                    q_prev_prev,
                    q_prev_prev_prev,
                    cq_curr,
                    cq_prev,
                    dt,
                    history.accepted_dt_prev,
                    history.accepted_dt_prev_prev,
                    effective_method,
                    trap_order,
                    reltol,
                    current_abstol,
                    charge_abstol,
                    trtol,
                ) else {
                    continue;
                };
                found_branch = true;
                limit = limit.min(branch_limit);
            }
        }

        found_branch.then_some(limit)
    }

    #[inline]
    pub(super) fn legacy_bjt_ngspice_truncation_limit(
        circuit: &crate::circuit::Circuit,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &BjtTransientHistory,
        vbic_snapshot_cache: &[Option<BjtChargeSnapshot>],
        voltage_abstol: Value,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        let effective_method = Self::effective_companion_method(method, trap_order);
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        for (idx, bjt) in circuit.bjts.devices.iter().enumerate() {
            if bjt.uses_vbic_dynamic_charges() {
                continue;
            }

            let vc = Self::node_voltage(candidate_solution, bjt.node_collector);
            let vb = Self::node_voltage(candidate_solution, bjt.node_base);
            let ve = Self::node_voltage(candidate_solution, bjt.node_emitter);
            let vs = Self::node_voltage(candidate_solution, bjt.node_substrate);
            let candidate_external = [vc, vb, ve, vs];
            let snapshot_reuse_abstol = voltage_abstol.min(VBIC_HISTORY_SNAPSHOT_REUSE_ABSTOL);
            let snapshot_reuse_reltol = reltol.min(VBIC_HISTORY_SNAPSHOT_REUSE_RELTOL);
            let snapshot = Self::resolve_vbic_snapshot_for_external_bias_with_linear_history(
                bjt,
                candidate_external,
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
                vbic_snapshot_cache.get(idx).copied().flatten(),
                VbicCachedSnapshotReuse::SeedOnly,
                snapshot_reuse_abstol,
                snapshot_reuse_reltol,
            )?;

            // Match ngspice's legacy BJT CKTterr coverage: qbe, qbc, qsub,
            // and true qbcx only when an internal collector-resistance branch
            // exists. Branch 3 is the XCJC external split charge in the
            // legacy backend, so it is integrated but not used as a separate
            // truncation limiter.
            for branch_idx in [
                BJT_QBE_BRANCH_INDEX,
                BJT_QBC_BRANCH_INDEX,
                BJT_QBCP_BRANCH_INDEX,
            ] {
                let branch = snapshot.branches[branch_idx];
                if !branch.is_active() {
                    continue;
                }
                let q_curr = branch.charge;
                if !q_curr.is_finite() {
                    continue;
                }
                let q_prev = history.charge_q_prev[idx][branch_idx];
                let q_prev_prev = history.charge_q_prev_prev[idx][branch_idx];
                let q_prev_prev_prev = history.charge_q_prev_prev_prev[idx][branch_idx];
                let cq_prev = history.charge_cq_prev[idx][branch_idx];
                let cq_curr = Self::jfet_companion_ccap(
                    effective_method,
                    trap_order,
                    dt,
                    q_curr,
                    q_prev,
                    q_prev_prev,
                    cq_prev,
                );

                let Some(branch_limit) = Self::ngspice_charge_truncation_limit(
                    q_curr,
                    q_prev,
                    q_prev_prev,
                    q_prev_prev_prev,
                    cq_curr,
                    cq_prev,
                    dt,
                    history.accepted_dt_prev,
                    history.accepted_dt_prev_prev,
                    effective_method,
                    trap_order,
                    reltol,
                    current_abstol,
                    charge_abstol,
                    trtol,
                ) else {
                    continue;
                };
                found_branch = true;
                limit = limit.min(branch_limit);
            }
        }

        found_branch.then_some(limit)
    }

    #[inline]
    pub(super) fn bjt_ngspice_truncation_limit(
        circuit: &crate::circuit::Circuit,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &BjtTransientHistory,
        vbic_snapshot_cache: &[Option<BjtChargeSnapshot>],
        voltage_abstol: Value,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        if let Some(vbic_limit) = Self::vbic_ngspice_truncation_limit(
            circuit,
            candidate_solution,
            method,
            trap_order,
            dt,
            history,
            vbic_snapshot_cache,
            voltage_abstol,
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        ) {
            limit = limit.min(vbic_limit);
            found_branch = true;
        }

        if let Some(legacy_limit) = Self::legacy_bjt_ngspice_truncation_limit(
            circuit,
            candidate_solution,
            method,
            trap_order,
            dt,
            history,
            vbic_snapshot_cache,
            voltage_abstol,
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        ) {
            limit = limit.min(legacy_limit);
            found_branch = true;
        }

        found_branch.then_some(limit)
    }

    #[inline]
    pub(super) fn jfet_ngspice_truncation_limit(
        circuit: &crate::circuit::Circuit,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &JfetTransientHistory,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        if history.accepted_dt_prev <= 0.0 || !history.accepted_dt_prev.is_finite() {
            return None;
        }

        let effective_method = Self::effective_companion_method(method, trap_order);
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        for (idx, jfet) in circuit.jfets.iter().enumerate() {
            let (vgs_eval, vgd_eval) = Self::jfet_branch_voltages(jfet, candidate_solution);
            let (vgs_charge, vgd_charge) =
                Self::jfet_charge_branch_voltages(jfet, candidate_solution);
            let (cgs, cgd) = jfet.transient_capacitances(vgs_eval, vgd_eval, jfet.params.tnom);

            for (
                capacitance,
                voltage,
                voltage_prev,
                q_prev,
                q_prev_prev,
                q_prev_prev_prev,
                cq_prev,
            ) in [
                (
                    cgs,
                    vgs_charge,
                    history.vgs_prev[idx],
                    history.qgs_prev[idx],
                    history.qgs_prev_prev[idx],
                    history.qgs_prev_prev_prev[idx],
                    history.cqgs_prev[idx],
                ),
                (
                    cgd,
                    vgd_charge,
                    history.vgd_prev[idx],
                    history.qgd_prev[idx],
                    history.qgd_prev_prev[idx],
                    history.qgd_prev_prev_prev[idx],
                    history.cqgd_prev[idx],
                ),
            ] {
                if !capacitance.is_finite() || capacitance <= 0.0 {
                    continue;
                }

                let (_geq, _ieq, q_curr, cq_curr) = Self::jfet_companion_terms(
                    effective_method,
                    trap_order,
                    dt,
                    capacitance,
                    voltage,
                    voltage_prev,
                    q_prev,
                    q_prev_prev,
                    cq_prev,
                );
                let Some(branch_limit) = Self::ngspice_charge_truncation_limit(
                    q_curr,
                    q_prev,
                    q_prev_prev,
                    q_prev_prev_prev,
                    cq_curr,
                    cq_prev,
                    dt,
                    history.accepted_dt_prev,
                    history.accepted_dt_prev_prev,
                    effective_method,
                    trap_order,
                    reltol,
                    current_abstol,
                    charge_abstol,
                    trtol,
                ) else {
                    continue;
                };
                found_branch = true;
                limit = limit.min(branch_limit);
            }
        }

        found_branch.then_some(limit)
    }

    #[inline]
    pub(super) fn mosfet_ngspice_truncation_limit(
        circuit: &crate::circuit::Circuit,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &MosfetTransientHistory,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        if history.accepted_dt_prev <= 0.0 || !history.accepted_dt_prev.is_finite() {
            return None;
        }

        let effective_method = Self::effective_companion_method(method, trap_order);
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        for (idx, mos) in circuit.mosfets.devices.iter().enumerate() {
            let (vgs_eval, vds_eval, vbs_eval) = mos.eval_branch_voltages_at(candidate_solution);
            let (vgs, vgd, vgb) = mos.gate_charge_branch_voltages_at(candidate_solution);
            let (cgs_half, cgd_half, cgb_half) =
                mos.transient_capacitance_halves_at(vgs_eval, vds_eval, vbs_eval);
            let (cgs_ov, cgd_ov, cgb_ov) = mos.overlap_capacitances();

            for (
                _branch,
                capacitance,
                voltage,
                voltage_prev,
                q_prev,
                q_prev_prev,
                q_prev_prev_prev,
                cq_prev,
            ) in [
                (
                    "qgs",
                    cgs_half + history.capgs_prev_half[idx] + cgs_ov,
                    vgs,
                    history.vgs_prev[idx],
                    history.qgs_prev[idx],
                    history.qgs_prev_prev[idx],
                    history.qgs_prev_prev_prev[idx],
                    history.cqgs_prev[idx],
                ),
                (
                    "qgd",
                    cgd_half + history.capgd_prev_half[idx] + cgd_ov,
                    vgd,
                    history.vgd_prev[idx],
                    history.qgd_prev[idx],
                    history.qgd_prev_prev[idx],
                    history.qgd_prev_prev_prev[idx],
                    history.cqgd_prev[idx],
                ),
                (
                    "qgb",
                    cgb_half + history.capgb_prev_half[idx] + cgb_ov,
                    vgb,
                    history.vgb_prev[idx],
                    history.qgb_prev[idx],
                    history.qgb_prev_prev[idx],
                    history.qgb_prev_prev_prev[idx],
                    history.cqgb_prev[idx],
                ),
            ] {
                if !capacitance.is_finite() || capacitance <= 0.0 {
                    continue;
                }

                let (_geq, _ieq, q_curr, cq_curr) = Self::jfet_companion_terms(
                    effective_method,
                    trap_order,
                    dt,
                    capacitance,
                    voltage,
                    voltage_prev,
                    q_prev,
                    q_prev_prev,
                    cq_prev,
                );
                let Some(branch_limit) = Self::ngspice_charge_truncation_limit(
                    q_curr,
                    q_prev,
                    q_prev_prev,
                    q_prev_prev_prev,
                    cq_curr,
                    cq_prev,
                    dt,
                    history.accepted_dt_prev,
                    history.accepted_dt_prev_prev,
                    effective_method,
                    trap_order,
                    reltol,
                    current_abstol,
                    charge_abstol,
                    trtol,
                ) else {
                    continue;
                };
                found_branch = true;
                limit = limit.min(branch_limit);
            }
        }

        found_branch.then_some(limit)
    }

    #[inline]
    pub(super) fn min_truncation_limit(
        first: Option<Value>,
        second: Option<Value>,
    ) -> Option<Value> {
        match (first, second) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    #[inline]
    pub(super) fn ltra_candidate_truncation_limit(
        circuit: &crate::circuit::Circuit,
        candidate_solution: &[Value],
        candidate_time: Value,
    ) -> Option<Value> {
        let mut limit = Value::INFINITY;
        let mut found_line = false;

        for tl in &circuit.tlines {
            let Some((br1, br2)) = tl.ltra_branch_matrix_indices() else {
                continue;
            };

            let v1 = Self::differential_voltage(candidate_solution, tl.node1_pos, tl.node1_neg);
            let v2 = Self::differential_voltage(candidate_solution, tl.node2_pos, tl.node2_neg);
            let i1 = candidate_solution.get(br1 - 1).copied().unwrap_or(0.0);
            let i2 = candidate_solution.get(br2 - 1).copied().unwrap_or(0.0);
            let Some(line_limit) = tl
                .ltra_candidate_truncation_limit(candidate_time, v1, i1, v2, i2)
                .filter(|line_limit| line_limit.is_finite() && *line_limit > 0.0)
            else {
                continue;
            };

            limit = limit.min(line_limit);
            found_line = true;
        }

        found_line.then_some(limit)
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub(super) fn ngspice_device_truncation_limit(
        circuit: &crate::circuit::Circuit,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        bjt_history: &BjtTransientHistory,
        vbic_snapshot_cache: &[Option<BjtChargeSnapshot>],
        jfet_history: &JfetTransientHistory,
        mosfet_history: &MosfetTransientHistory,
        suppress_gate_charge: bool,
        voltage_abstol: Value,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        let capacitor_limit = if !circuit.capacitors.is_empty() {
            Self::capacitor_ngspice_truncation_limit(
                circuit,
                candidate_solution,
                method,
                trap_order,
                dt,
                mosfet_history.accepted_dt_prev,
                mosfet_history.accepted_dt_prev_prev,
                reltol,
                current_abstol,
                charge_abstol,
                trtol,
            )
            .filter(|limit| limit.is_finite() && *limit > 0.0)
        } else {
            None
        };
        let bjt_limit = if !circuit.bjts.devices.is_empty() {
            Self::bjt_ngspice_truncation_limit(
                circuit,
                candidate_solution,
                method,
                trap_order,
                dt,
                bjt_history,
                vbic_snapshot_cache,
                voltage_abstol,
                reltol,
                current_abstol,
                charge_abstol,
                trtol,
            )
            .filter(|limit| limit.is_finite() && *limit > 0.0)
        } else {
            None
        };
        let jfet_limit = if !suppress_gate_charge && !circuit.jfets.is_empty() {
            Self::jfet_ngspice_truncation_limit(
                circuit,
                candidate_solution,
                method,
                trap_order,
                dt,
                jfet_history,
                reltol,
                current_abstol,
                charge_abstol,
                trtol,
            )
            .filter(|limit| limit.is_finite() && *limit > 0.0)
        } else {
            None
        };
        let mosfet_limit = if !suppress_gate_charge && !circuit.mosfets.is_empty() {
            Self::mosfet_ngspice_truncation_limit(
                circuit,
                candidate_solution,
                method,
                trap_order,
                dt,
                mosfet_history,
                reltol,
                current_abstol,
                charge_abstol,
                trtol,
            )
            .filter(|limit| limit.is_finite() && *limit > 0.0)
        } else {
            None
        };

        Self::min_truncation_limit(
            Self::min_truncation_limit(
                Self::min_truncation_limit(capacitor_limit, bjt_limit),
                jfet_limit,
            ),
            mosfet_limit,
        )
    }

    #[inline]
    pub(super) fn should_retry_ngspice_charge_truncation(limit: Value, dt: Value) -> bool {
        limit.is_finite() && dt.is_finite() && dt > 0.0 && limit <= 0.9 * dt
    }

    #[inline]
    pub(super) fn should_promote_ngspice_charge_truncation(limit: Value, dt: Value) -> bool {
        limit.is_finite() && dt.is_finite() && dt > 0.0 && limit > 1.05 * dt
    }

    #[inline]
    pub(super) fn next_trapezoidal_order_after_accepted_step(
        current_order: u8,
        hit_breakpoint: bool,
        should_promote: bool,
    ) -> u8 {
        if hit_breakpoint {
            1
        } else if current_order >= 2 || should_promote {
            2
        } else {
            1
        }
    }

    #[inline]
    pub(super) fn trapezoidal_order_after_timestep_control_reject(current_order: u8) -> u8 {
        current_order.max(1)
    }

    #[inline]
    pub(super) fn vbic_charge_lte_startup_window_end(
        hinted_max_step: Value,
        smallest_vbic_excess_phase_td: Option<Value>,
    ) -> Value {
        let maxstep_window = if hinted_max_step.is_finite() && hinted_max_step > 0.0 {
            hinted_max_step * 0.1
        } else {
            Value::INFINITY
        };
        let td_window = smallest_vbic_excess_phase_td
            .filter(|td| td.is_finite() && *td > 0.0)
            .map(|td| td * 32.0)
            .unwrap_or(maxstep_window);

        maxstep_window.min(td_window)
    }

    #[inline]
    pub(super) fn vbic_excess_phase_startup_step_cap(
        hinted_max_step: Value,
        smallest_vbic_excess_phase_td: Option<Value>,
    ) -> Option<Value> {
        let td = smallest_vbic_excess_phase_td.filter(|td| td.is_finite() && *td > 0.0)?;
        let hinted_max_step = if hinted_max_step.is_finite() && hinted_max_step > 0.0 {
            hinted_max_step
        } else {
            Value::INFINITY
        };

        Some((td * 0.25).clamp(1e-15, hinted_max_step))
    }

    #[inline]
    pub(super) fn should_use_vbic_charge_lte_startup_guard(
        has_vbic_excess_phase: bool,
        step_time: Value,
        hinted_max_step: Value,
        smallest_vbic_excess_phase_td: Option<Value>,
    ) -> bool {
        has_vbic_excess_phase
            && step_time.is_finite()
            && step_time
                <= Self::vbic_charge_lte_startup_window_end(
                    hinted_max_step,
                    smallest_vbic_excess_phase_td,
                )
    }

    #[inline]
    pub(super) fn should_hold_vbic_excess_phase_first_order(
        has_vbic_excess_phase: bool,
        _accepted_time: Value,
        _hinted_max_step: Value,
        _smallest_vbic_excess_phase_td: Option<Value>,
    ) -> bool {
        // ngspice does not impose a VBIC-specific first-order hold once the
        // current timepoint has been accepted. Promotion back to order 2 is
        // controlled solely by the same order-2 truncation check used for any
        // trapezoidal transient step.
        let _ = has_vbic_excess_phase;
        false
    }

    #[inline]
    pub(super) fn should_use_vbic_charge_lte_estimator(
        has_vbic_excess_phase: bool,
        step_time: Value,
        hinted_max_step: Value,
        smallest_vbic_excess_phase_td: Option<Value>,
        dt: Value,
        preferred_min_dt: Value,
    ) -> bool {
        dt.is_finite()
            && dt > preferred_min_dt
            && Self::should_use_vbic_charge_lte_startup_guard(
                has_vbic_excess_phase,
                step_time,
                hinted_max_step,
                smallest_vbic_excess_phase_td,
            )
    }

    #[inline]
    pub(super) fn should_defer_voltage_lte_to_vbic_truncation(
        has_vbic_excess_phase: bool,
        step_time: Value,
        hinted_max_step: Value,
        smallest_vbic_excess_phase_td: Option<Value>,
        vbic_truncation_limit: Option<Value>,
        using_vbic_charge_lte_estimator: bool,
    ) -> bool {
        has_vbic_excess_phase
            && !using_vbic_charge_lte_estimator
            && vbic_truncation_limit.is_some()
            && Self::should_use_vbic_charge_lte_startup_guard(
                has_vbic_excess_phase,
                step_time,
                hinted_max_step,
                smallest_vbic_excess_phase_td,
            )
    }

    #[inline]
    pub(super) fn bjt_charge_truncation_covers_transient_lte(
        circuit: &crate::circuit::Circuit,
        bjt_truncation_limit: Option<Value>,
    ) -> bool {
        bjt_truncation_limit.is_some()
            && !circuit.bjts.devices.is_empty()
            && circuit.capacitors.is_empty()
            && circuit.inductors.is_empty()
            && circuit.diodes.is_empty()
            && circuit.mosfets.is_empty()
            && circuit.jfets.is_empty()
            && circuit.tlines.is_empty()
            && circuit.coupled_tlines.is_empty()
            && circuit.coupled_inductor_pairs.is_empty()
            && circuit.multi_winding_transformers.is_empty()
            && circuit.jiles_atherton_inductors.is_empty()
            && !circuit.has_xspice_devices()
    }

    #[inline]
    pub(super) fn jfet_charge_truncation_covers_transient_lte(
        circuit: &crate::circuit::Circuit,
        jfet_truncation_limit: Option<Value>,
    ) -> bool {
        jfet_truncation_limit.is_some()
            && !circuit.jfets.is_empty()
            && circuit.capacitors.is_empty()
            && circuit.inductors.is_empty()
            && circuit.diodes.is_empty()
            && circuit.bjts.devices.is_empty()
            && circuit.mosfets.is_empty()
            && circuit.tlines.is_empty()
            && circuit.coupled_tlines.is_empty()
            && circuit.coupled_inductor_pairs.is_empty()
            && circuit.multi_winding_transformers.is_empty()
            && circuit.jiles_atherton_inductors.is_empty()
            && !circuit.has_xspice_devices()
    }

    #[inline]
    pub(super) fn mosfet_charge_truncation_covers_transient_lte(
        circuit: &crate::circuit::Circuit,
        mosfet_truncation_limit: Option<Value>,
    ) -> bool {
        mosfet_truncation_limit.is_some()
            && !circuit.mosfets.is_empty()
            && circuit.capacitors.is_empty()
            && circuit.inductors.is_empty()
            && circuit.diodes.is_empty()
            && circuit.bjts.devices.is_empty()
            && circuit.jfets.is_empty()
            && circuit.tlines.is_empty()
            && circuit.coupled_tlines.is_empty()
            && circuit.coupled_inductor_pairs.is_empty()
            && circuit.multi_winding_transformers.is_empty()
            && circuit.jiles_atherton_inductors.is_empty()
            && !circuit.has_xspice_devices()
    }

    #[inline]
    pub(super) fn ngspice_device_truncation_covers_transient_lte(
        circuit: &crate::circuit::Circuit,
        capacitor_truncation_limit: Option<Value>,
        bjt_truncation_limit: Option<Value>,
        jfet_truncation_limit: Option<Value>,
        mosfet_truncation_limit: Option<Value>,
    ) -> bool {
        if circuit.has_xspice_devices()
            || !circuit.diodes.is_empty()
            || !circuit.inductors.is_empty()
            || !circuit.coupled_inductor_pairs.is_empty()
            || !circuit.multi_winding_transformers.is_empty()
            || !circuit.jiles_atherton_inductors.is_empty()
        {
            return false;
        }

        let capacitor_controlled =
            circuit.capacitors.is_empty() || capacitor_truncation_limit.is_some();
        let bjt_controlled = circuit.bjts.devices.is_empty() || bjt_truncation_limit.is_some();
        let jfet_controlled = circuit.jfets.is_empty() || jfet_truncation_limit.is_some();
        let mosfet_controlled = circuit.mosfets.is_empty() || mosfet_truncation_limit.is_some();

        capacitor_controlled && bjt_controlled && jfet_controlled && mosfet_controlled
    }

    #[inline]
    pub(super) fn estimate_transient_lte(
        circuit: &crate::circuit::Circuit,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        is_strictly_linear_transient: bool,
        bjt_history: &BjtTransientHistory,
        voltage_lte_estimator: &LteEstimator,
        vbic_charge_lte_estimator: Option<&LteEstimator>,
        vbic_snapshot_cache: Option<&[Option<BjtChargeSnapshot>]>,
        voltage_abstol: Value,
        reltol: Value,
    ) -> (Value, bool, bool) {
        if is_strictly_linear_transient {
            return (0.0, true, false);
        }

        let (snapshot_reuse_abstol, snapshot_reuse_reltol) =
            Self::vbic_runtime_snapshot_reuse_tolerances(voltage_abstol, reltol);
        if let Some(charge_lte_estimator) = vbic_charge_lte_estimator
            && let Some(vbic_charge_state) = Self::collect_vbic_truncation_charge_state(
                circuit,
                candidate_solution,
                method,
                trap_order,
                dt,
                bjt_history,
                vbic_snapshot_cache,
                snapshot_reuse_abstol,
                snapshot_reuse_reltol,
            )
        {
            let (lte, accept) = charge_lte_estimator.estimate(&vbic_charge_state, dt);
            return (lte, accept, true);
        }

        let (lte, accept) =
            voltage_lte_estimator.estimate_prefix(candidate_solution, circuit.num_nodes(), dt);
        (lte, accept, false)
    }

    #[inline]
    pub(super) fn recommend_transient_lte_scale(
        voltage_lte_estimator: &LteEstimator,
        vbic_charge_lte_estimator: Option<&LteEstimator>,
        lte: Value,
        uses_vbic_charge_lte: bool,
    ) -> Value {
        if uses_vbic_charge_lte {
            vbic_charge_lte_estimator
                .unwrap_or(voltage_lte_estimator)
                .recommend_scale(lte)
        } else {
            voltage_lte_estimator.recommend_scale(lte)
        }
    }

    #[inline]
    pub(super) fn trapezoidal_order_trial_timestep_limit(
        circuit: &crate::circuit::Circuit,
        accepted_solution: &[Value],
        method: IntegrationMethod,
        dt: Value,
        is_strictly_linear_transient: bool,
        history: &BjtTransientHistory,
        jfet_history: &JfetTransientHistory,
        mosfet_history: &MosfetTransientHistory,
        voltage_lte_estimator: &LteEstimator,
        vbic_charge_lte_estimator: Option<&LteEstimator>,
        vbic_snapshot_cache: &[Option<BjtChargeSnapshot>],
        voltage_abstol: Value,
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<TrapezoidalOrderTrial> {
        if !matches!(
            method,
            IntegrationMethod::Trapezoidal | IntegrationMethod::TrapGear
        ) {
            return None;
        }
        if !(dt.is_finite() && dt > 0.0) {
            return None;
        }
        // Match ngspice startup behavior: keep order-1 through the first accepted
        // transient step, then run the order-2 trial truncation check. The trial
        // limit still caps the next step when it is not large enough to promote.
        if !(history.accepted_dt_prev.is_finite() && history.accepted_dt_prev > 0.0) {
            return None;
        }

        if let Some(limit) = Self::ngspice_device_truncation_limit(
            circuit,
            accepted_solution,
            method,
            2,
            dt,
            history,
            vbic_snapshot_cache,
            jfet_history,
            mosfet_history,
            false,
            voltage_abstol,
            reltol,
            current_abstol,
            charge_abstol,
            trtol,
        ) {
            return Some(TrapezoidalOrderTrial {
                limit,
                promote: Self::should_promote_ngspice_charge_truncation(limit, dt),
            });
        }

        let (candidate_lte, accept, uses_vbic_charge_lte) = Self::estimate_transient_lte(
            circuit,
            accepted_solution,
            method,
            2,
            dt,
            is_strictly_linear_transient,
            history,
            voltage_lte_estimator,
            vbic_charge_lte_estimator,
            Some(vbic_snapshot_cache),
            voltage_abstol,
            reltol,
        );
        if !accept {
            return None;
        }

        let candidate_scale = if is_strictly_linear_transient {
            1.0
        } else {
            Self::recommend_transient_lte_scale(
                voltage_lte_estimator,
                vbic_charge_lte_estimator,
                candidate_lte,
                uses_vbic_charge_lte,
            )
        };
        if candidate_scale >= 0.95 {
            Some(TrapezoidalOrderTrial {
                limit: Value::INFINITY,
                promote: true,
            })
        } else {
            None
        }
    }

    #[inline]
    pub(super) fn record_vbic_truncation_charge_state(
        estimator: &mut Option<LteEstimator>,
        circuit: &crate::circuit::Circuit,
        accepted_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &BjtTransientHistory,
        vbic_snapshot_cache: Option<&[Option<BjtChargeSnapshot>]>,
        method_order: u32,
    ) {
        let Some(estimator) = estimator.as_mut() else {
            return;
        };

        if let Some(vbic_charge_state) = Self::collect_vbic_truncation_charge_state(
            circuit,
            accepted_solution,
            method,
            trap_order,
            dt,
            history,
            vbic_snapshot_cache,
            VBIC_HISTORY_SNAPSHOT_REUSE_ABSTOL,
            VBIC_HISTORY_SNAPSHOT_REUSE_RELTOL,
        ) {
            estimator.record(&vbic_charge_state, dt);
            estimator.set_method_order(method_order);
        }
    }

    #[inline]
    pub(super) fn lu_decompose_small_dense_real<const N: usize>(
        matrix: &[[Value; N]; N],
        dim: usize,
    ) -> Option<([[Value; N]; N], [usize; N])> {
        if dim == 0 {
            let mut pivots = [0usize; N];
            for (idx, pivot) in pivots.iter_mut().enumerate() {
                *pivot = idx;
            }
            return Some((*matrix, pivots));
        }

        let mut lu = *matrix;
        let mut pivots = [0usize; N];
        for (idx, pivot) in pivots.iter_mut().enumerate() {
            *pivot = idx;
        }

        for pivot in 0..dim {
            let mut best = pivot;
            let mut best_abs = lu[pivot][pivot].abs();
            for row in (pivot + 1)..dim {
                let value = lu[row][pivot].abs();
                if value > best_abs {
                    best = row;
                    best_abs = value;
                }
            }
            if best_abs < 1e-18 {
                return None;
            }
            if best != pivot {
                lu.swap(pivot, best);
                pivots.swap(pivot, best);
            }

            let pivot_value = lu[pivot][pivot];
            for row in (pivot + 1)..dim {
                lu[row][pivot] /= pivot_value;
                let factor = lu[row][pivot];
                for col in (pivot + 1)..dim {
                    lu[row][col] -= factor * lu[pivot][col];
                }
            }
        }

        Some((lu, pivots))
    }

    #[inline]
    pub(super) fn lu_solve_small_dense_real<const N: usize>(
        lu: &[[Value; N]; N],
        pivots: &[usize; N],
        rhs: &[Value; N],
        dim: usize,
    ) -> Option<[Value; N]> {
        if dim == 0 {
            return Some([0.0; N]);
        }

        let mut x = [0.0; N];
        for row in 0..dim {
            x[row] = rhs[pivots[row]];
            for col in 0..row {
                x[row] -= lu[row][col] * x[col];
            }
        }

        for row in (0..dim).rev() {
            for col in (row + 1)..dim {
                x[row] -= lu[row][col] * x[col];
            }
            let diag = lu[row][row];
            if diag.abs() < 1e-18 {
                return None;
            }
            x[row] /= diag;
        }

        Some(x)
    }
}
