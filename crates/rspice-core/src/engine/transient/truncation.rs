//! Charge truncation and transient LTE control helpers.

use super::*;

impl Engine {
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
        reltol: Value,
        current_abstol: Value,
        charge_abstol: Value,
        trtol: Value,
    ) -> Option<Value> {
        let effective_method = Self::effective_companion_method(method, trap_order);
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        for (idx, bjt) in circuit.bjts.devices.iter().enumerate() {
            if !bjt.vbic_mna_promoted() {
                continue;
            }

            // Promoted VBIC: the candidate solution carries the internal node
            // voltages, so the charges evaluate directly at the candidate
            // bias (ngspice VBICtrunc CKTterr over the charge states).
            let (branches, _, _) = bjt.vbic_mna_charge_state_at_solution(candidate_solution);

            for branch_idx in 0..BJT_VBIC_TRUNCATION_BRANCH_COUNT {
                let q_curr = branches[branch_idx].charge;
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
                if branch_limit < dt && std::env::var_os("RSPICE_LTE_DEBUG").is_some() {
                    log::warn!(
                        "BJT LTE bind: dev={idx} branch={branch_idx} q=[{q_curr:.6e},{q_prev:.6e},{q_prev_prev:.6e},{q_prev_prev_prev:.6e}] cq=[{cq_curr:.4e},{cq_prev:.4e}] dts=[{:.4e},{:.4e},{:.4e}] limit={branch_limit:.4e}",
                        dt,
                        history.accepted_dt_prev,
                        history.accepted_dt_prev_prev,
                    );
                }
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
                if branch_limit < dt && std::env::var_os("RSPICE_LTE_DEBUG").is_some() {
                    log::warn!(
                        "legacy BJT LTE bind: dev={idx} branch={branch_idx} q=[{q_curr:.6e},{q_prev:.6e},{q_prev_prev:.6e},{q_prev_prev_prev:.6e}] cq=[{cq_curr:.4e},{cq_prev:.4e}] dts=[{:.4e},{:.4e},{:.4e}] limit={branch_limit:.4e}",
                        dt,
                        history.accepted_dt_prev,
                        history.accepted_dt_prev_prev,
                    );
                }
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

    /// ngspice `DIOtrunc` (CKTterr on the `DIOcapCharge` state): the diode
    /// junction depletion+diffusion charge drives the timestep through the
    /// same divided-difference truncation law as the other junction devices.
    #[inline]
    pub(super) fn diode_ngspice_truncation_limit(
        circuit: &crate::circuit::Circuit,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &DiodeTransientHistory,
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

        for (idx, diode) in circuit.diodes.devices.iter().enumerate() {
            let vd = Self::differential_voltage(
                candidate_solution,
                diode.node_anode,
                diode.node_cathode,
            );
            let (qd, capd) = diode.junction_charge_and_capacitance(vd);
            if !capd.is_finite() || capd <= 0.0 {
                continue;
            }

            let (_geq, _ieq, q_curr, cq_curr) = Self::nonlinear_charge_companion_terms(
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
            let Some(branch_limit) = Self::ngspice_charge_truncation_limit(
                q_curr,
                history.qd_prev[idx],
                history.qd_prev_prev[idx],
                history.qd_prev_prev_prev[idx],
                cq_curr,
                history.cqd_prev[idx],
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
        caps_cache_out: Option<&mut Vec<(Value, Value, Value)>>,
    ) -> Option<Value> {
        if history.accepted_dt_prev <= 0.0 || !history.accepted_dt_prev.is_finite() {
            return None;
        }

        // The Meyer capacitance halves computed here are exactly what the
        // acceptance-path history rotation re-derives on the same candidate
        // solution; the caller can capture them to skip that second walk.
        let mut caps_cache = caps_cache_out;
        if let Some(cache) = caps_cache.as_deref_mut() {
            cache.clear();
            cache.reserve(circuit.mosfets.devices.len());
        }

        let effective_method = Self::effective_companion_method(method, trap_order);
        let mut limit = 2.0 * dt;
        let mut found_branch = false;

        for (idx, mos) in circuit.mosfets.devices.iter().enumerate() {
            let (vgs_eval, vds_eval, vbs_eval) = mos.eval_branch_voltages_at(candidate_solution);
            let (vgs, vgd, vgb) = mos.gate_charge_branch_voltages_at(candidate_solution);
            let (cgs_half, cgd_half, cgb_half) =
                mos.transient_capacitance_halves_at(vgs_eval, vds_eval, vbs_eval);
            if let Some(cache) = caps_cache.as_deref_mut() {
                cache.push((cgs_half, cgd_half, cgb_half));
            }
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
                if branch_limit < dt && std::env::var_os("RSPICE_LTE_DEBUG").is_some() {
                    log::warn!(
                        "MOS LTE bind: dev={idx} branch={_branch} cap={capacitance:.4e} v=[{voltage:.4e},{voltage_prev:.4e}] q=[{q_curr:.6e},{q_prev:.6e},{q_prev_prev:.6e},{q_prev_prev_prev:.6e}] cq=[{cq_curr:.4e},{cq_prev:.4e}] dts=[{:.4e},{:.4e},{:.4e}] limit={branch_limit:.4e}",
                        dt,
                        history.accepted_dt_prev,
                        history.accepted_dt_prev_prev,
                    );
                }
                found_branch = true;
                limit = limit.min(branch_limit);
            }
        }

        found_branch.then_some(limit)
    }

    /// LTE truncation limit for the BSIMSOI (level 56) charge states.
    ///
    /// Mirrors [`Self::mosfet_ngspice_truncation_limit`] but over the four
    /// coupled SOI node charges (qg/qb/qd/qe). The floating-body charge `qb`
    /// participates, so the local-truncation-error step control resolves the
    /// body transient the way ngspice's `B3SOIDDtrunc` does. Returns the
    /// tightest per-charge step bound, or `None` when no SOI charge is active.
    pub(super) fn b3soi_ngspice_truncation_limit(
        circuit: &crate::circuit::Circuit,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &B3SoiTransientHistory,
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

        // The history is indexed DD devices first, then FD, then PD,
        // matching the companion stamp/commit walks.
        let mut device_charges: Vec<(Value, Value, Value, Value)> = Vec::with_capacity(
            circuit.b3soi.devices.len()
                + circuit.b3soi_fd.devices.len()
                + circuit.b3soi_pd.devices.len(),
        );
        for dev in &circuit.b3soi.devices {
            if dev.charges_suppressed() {
                device_charges.push((0.0, 0.0, 0.0, 0.0));
                continue;
            }
            let c = dev.charge_at(candidate_solution);
            device_charges.push((c.qg, c.qb, c.qd, c.qe));
        }
        for dev in &circuit.b3soi_fd.devices {
            if dev.charges_suppressed() {
                device_charges.push((0.0, 0.0, 0.0, 0.0));
                continue;
            }
            let c = dev.charge_at(candidate_solution);
            device_charges.push((c.qg, c.qb, c.qd, c.qe));
        }
        for dev in &circuit.b3soi_pd.devices {
            if dev.charges_suppressed() {
                device_charges.push((0.0, 0.0, 0.0, 0.0));
                continue;
            }
            let c = dev.charge_at(candidate_solution);
            device_charges.push((c.qg, c.qb, c.qd, c.qe));
        }

        for (idx, (qg, qb, qd, qe)) in device_charges.into_iter().enumerate() {
            for (q_curr, q_prev, q_prev_prev, q_prev_prev_prev, cq_prev) in [
                (
                    qg,
                    history.qg_prev[idx],
                    history.qg_prev_prev[idx],
                    history.qg_prev_prev_prev[idx],
                    history.cqg_prev[idx],
                ),
                (
                    qb,
                    history.qb_prev[idx],
                    history.qb_prev_prev[idx],
                    history.qb_prev_prev_prev[idx],
                    history.cqb_prev[idx],
                ),
                (
                    qd,
                    history.qd_prev[idx],
                    history.qd_prev_prev[idx],
                    history.qd_prev_prev_prev[idx],
                    history.cqd_prev[idx],
                ),
                (
                    qe,
                    history.qe_prev[idx],
                    history.qe_prev_prev[idx],
                    history.qe_prev_prev_prev[idx],
                    history.cqe_prev[idx],
                ),
            ] {
                // Integrated charge current at the candidate point.
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

    /// LTE truncation limit for the BSIM3v3.3 (level 8/49) charge states.
    ///
    /// Mirrors [`Self::b3soi_ngspice_truncation_limit`] over the three
    /// composite BSIM3 node charges (`qg`/`qb`/`qd`, junction depletion
    /// charges folded in) — exactly the states `b3trunc.c` feeds `CKTterr`.
    /// Returns the tightest per-charge step bound, or `None` when no charge
    /// is active.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn bsim3_ngspice_truncation_limit(
        circuit: &crate::circuit::Circuit,
        candidate_solution: &[Value],
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        history: &Bsim3TransientHistory,
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

        for (idx, dev) in circuit.bsim3v3.devices.iter().enumerate() {
            let (c, _mode) = dev.charge_at(candidate_solution);
            for (q_curr, q_prev, q_prev_prev, q_prev_prev_prev, cq_prev) in [
                (
                    c.qg_state(),
                    history.qg_prev[idx],
                    history.qg_prev_prev[idx],
                    history.qg_prev_prev_prev[idx],
                    history.cqg_prev[idx],
                ),
                (
                    c.qb_state(),
                    history.qb_prev[idx],
                    history.qb_prev_prev[idx],
                    history.qb_prev_prev_prev[idx],
                    history.cqb_prev[idx],
                ),
                (
                    c.qd_state(),
                    history.qd_prev[idx],
                    history.qd_prev_prev[idx],
                    history.qd_prev_prev_prev[idx],
                    history.cqd_prev[idx],
                ),
            ] {
                // Integrated charge current at the candidate point.
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

    /// Signal-activity step limit: rescale the candidate step so that no
    /// nonlinear-device terminal voltage moves more than `bound` volts in one
    /// step.
    ///
    /// Complements the polynomial charge LTE, which estimates error from
    /// divided differences of sampled charges and is therefore blind to
    /// curvature lying entirely between samples (see
    /// [`crate::constants::DEVICE_ACTIVITY_STEP_BOUND`]). Returns the
    /// proportionally reduced step when the bound is exceeded, `None` when
    /// the candidate respects it.
    pub(super) fn nonlinear_terminal_activity_limit(
        circuit: &crate::circuit::Circuit,
        accepted_solution: &[Value],
        candidate_solution: &[Value],
        dt: Value,
        bound: Value,
    ) -> Option<Value> {
        if !(bound.is_finite() && bound > 0.0 && dt.is_finite() && dt > 0.0) {
            return None;
        }

        let mut max_delta: Value = 0.0;
        let mut consider = |node: usize| {
            if node == 0 {
                return;
            }
            let accepted = accepted_solution.get(node - 1).copied().unwrap_or(0.0);
            let candidate = candidate_solution.get(node - 1).copied().unwrap_or(0.0);
            let delta = (candidate - accepted).abs();
            if delta.is_finite() && delta > max_delta {
                max_delta = delta;
            }
        };

        for mos in &circuit.mosfets.devices {
            consider(mos.node_drain);
            consider(mos.node_gate);
            consider(mos.node_source);
            consider(mos.node_bulk);
        }
        for bjt in &circuit.bjts.devices {
            consider(bjt.node_collector);
            consider(bjt.node_base);
            consider(bjt.node_emitter);
            consider(bjt.node_substrate);
        }
        for jfet in &circuit.jfets {
            consider(jfet.drain);
            consider(jfet.gate);
            consider(jfet.source);
        }

        (max_delta > bound).then(|| dt * bound / max_delta)
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
        diode_history: &DiodeTransientHistory,
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
        let diode_limit = if !circuit.diodes.is_empty() {
            Self::diode_ngspice_truncation_limit(
                circuit,
                candidate_solution,
                method,
                trap_order,
                dt,
                diode_history,
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
                None,
            )
            .filter(|limit| limit.is_finite() && *limit > 0.0)
        } else {
            None
        };

        Self::min_truncation_limit(
            Self::min_truncation_limit(
                Self::min_truncation_limit(
                    Self::min_truncation_limit(capacitor_limit, bjt_limit),
                    jfet_limit,
                ),
                diode_limit,
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
        diode_truncation_limit: Option<Value>,
        mosfet_truncation_limit: Option<Value>,
    ) -> bool {
        if circuit.has_xspice_devices()
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
        // Zero-charge diodes (CJO=0, TT=0) report no truncation limit; the
        // generic node-voltage estimator stays in charge for those decks.
        let diode_controlled =
            circuit.diodes.is_empty() || diode_truncation_limit.is_some();
        let mosfet_controlled = circuit.mosfets.is_empty() || mosfet_truncation_limit.is_some();

        capacitor_controlled
            && bjt_controlled
            && jfet_controlled
            && diode_controlled
            && mosfet_controlled
    }

    #[inline]
    pub(super) fn estimate_transient_lte(
        circuit: &crate::circuit::Circuit,
        candidate_solution: &[Value],
        dt: Value,
        is_strictly_linear_transient: bool,
        voltage_lte_estimator: &LteEstimator,
    ) -> (Value, bool) {
        if is_strictly_linear_transient {
            return (0.0, true);
        }

        voltage_lte_estimator.estimate_prefix(candidate_solution, circuit.num_nodes(), dt)
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
        diode_history: &DiodeTransientHistory,
        mosfet_history: &MosfetTransientHistory,
        voltage_lte_estimator: &LteEstimator,
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
            diode_history,
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

        let (candidate_lte, accept) = Self::estimate_transient_lte(
            circuit,
            accepted_solution,
            dt,
            is_strictly_linear_transient,
            voltage_lte_estimator,
        );
        if !accept {
            return None;
        }

        let candidate_scale = if is_strictly_linear_transient {
            1.0
        } else {
            voltage_lte_estimator.recommend_scale(candidate_lte)
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
