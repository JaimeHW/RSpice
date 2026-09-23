//! Physical charge coordinates and history at a native BSIM4 periodic origin.

use super::*;

impl Bsim4v8Device {
    /// DC anchors the otherwise unused deficit node. A dynamic load owns its
    /// physical relaxation/storage equation and must remove that DC anchor.
    pub(crate) fn remove_trnqs_dc_anchor(&self, matrix: &mut impl MatrixStamper) {
        if self.uses_trnqs() {
            matrix.stamp(
                self.node_charge_deficit,
                self.node_charge_deficit,
                -self.multiplier * self.gmin.max(1e-12),
            );
        }
    }

    /// Stored channel charge divided by M*1n. The electrical terminals may
    /// move algebraically while this storage coordinate remains constrained.
    pub(crate) fn shooting_nqs_state(&self, solution: &[Value]) -> (Value, [Value; 12]) {
        let (charge, mode) = self.charge_at_with_probe(solution, true);
        let gate = -(charge.cggb + charge.cbgb);
        let drain = -(charge.cgdb + charge.cbdb);
        let source = -(charge.cgsb + charge.cbsb);
        let bulk = -(gate + drain + source);
        let (drain, source) = if mode > 0 {
            (drain, source)
        } else {
            (source, drain)
        };
        let mut derivatives = [0.0; 12];
        derivatives[1] = -drain / TRNQS_SCALING;
        derivatives[4] = -gate / TRNQS_SCALING;
        derivatives[6] = -source / TRNQS_SCALING;
        derivatives[8] = -bulk / TRNQS_SCALING;
        derivatives[11] = 1.0;
        (
            Self::node_voltage(solution, self.node_charge_deficit)
                - self.core.mtype * charge.qchqs / TRNQS_SCALING,
            derivatives,
        )
    }

    pub(crate) fn shooting_nqs_seed(&self, state: Value, solution: &[Value]) -> Value {
        Self::node_voltage(solution, self.node_charge_deficit) + state
            - self.shooting_nqs_state(solution).0
    }

    pub(crate) fn stamp_shooting_initial_charge(
        &self,
        solution: &[Value],
        matrix: &mut impl MatrixStamper,
        physical_probe: bool,
    ) {
        let (charge, mode) = self.charge_at_with_probe(solution, physical_probe);
        if self.uses_trnqs() {
            self.stamp_ac_charge_matrix(&Self::trnqs_overlap_charge(&charge), mode, 1.0, matrix);
            matrix.stamp(
                self.node_charge_deficit,
                self.node_charge_deficit,
                self.multiplier * TRNQS_SCALING,
            );
        } else {
            self.stamp_ac_charge_matrix(&charge, mode, 1.0, matrix);
        }
    }

    pub(crate) fn stamp_shooting_initial_relaxation(
        &self,
        solution: &[Value],
        matrix: &mut impl MatrixStamper,
        physical_probe: bool,
    ) {
        if self.uses_trnqs() {
            self.remove_trnqs_dc_anchor(matrix);
            let (charge, mode) = self.charge_at_with_probe(solution, physical_probe);
            self.stamp_trnqs_charge_companion_with_probe(
                &charge,
                mode,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                solution,
                matrix,
                physical_probe,
            );
        }
    }

    /// Storage dependencies, not a reciprocal-capacitor decomposition. Split
    /// gate/body nodes retain independent coordinates only when they store Q.
    pub(crate) fn shooting_terminal_storage_nodes(&self) -> [Option<(NodeId, NodeId)>; 8] {
        let p = &self.core.size;
        let t = &self.core.model_temp;
        let inst = &self.core.inst;
        let intrinsic = !self.uses_trnqs() && self.core.model.xpart >= 0.0;
        let extended = self.core.model.cap_mod != 0;
        let gate = if self.core.model.rgate_mod == 3 {
            self.node_gate_mid
        } else {
            self.node_gate
        };
        let active = [
            intrinsic,
            intrinsic,
            intrinsic,
            p.cgdo != 0.0 || (extended && p.cgdl != 0.0),
            p.cgso != 0.0 || (extended && p.cgsl != 0.0),
            p.cgbo != 0.0,
            t.d_unit_area_temp_jct_cap * inst.adeff != 0.0
                || t.d_unit_length_sidewall_temp_jct_cap * inst.pdeff != 0.0
                || t.d_unit_length_gate_sidewall_temp_jct_cap * p.weff_cj * inst.nf != 0.0,
            t.s_unit_area_temp_jct_cap * inst.aseff != 0.0
                || t.s_unit_length_sidewall_temp_jct_cap * inst.pseff != 0.0
                || t.s_unit_length_gate_sidewall_temp_jct_cap * p.weff_cj * inst.nf != 0.0,
        ];
        let nodes = [
            (self.node_gate, self.node_drain),
            (self.node_gate, self.node_source),
            (self.node_gate, self.node_bulk),
            (gate, self.node_drain),
            (gate, self.node_source),
            (gate, self.node_bulk),
            (self.node_drain, self.node_drain_body),
            (self.node_source, self.node_source_body),
        ];
        std::array::from_fn(|i| (active[i] && nodes[i].0 != nodes[i].1).then_some(nodes[i]))
    }

    /// Per-instance, device-polarity native states: Qg, Qgmid, Qb, Qd,
    /// Qbs, Qbd, equilibrium channel charge, stored charge deficit.
    pub(crate) fn periodic_history_sample(
        &self,
        solution: &[Value],
        rates: &[Value],
    ) -> Result<([Value; 8], [Value; 8]), String> {
        let (charge, mode) = self.charge_at_with_probe(solution, true);
        let (qg, qmid, qb, qd, qbs, qbd) = if self.uses_trnqs() {
            self.trnqs_state_charges_with_probe(&charge, solution, true)
        } else {
            (
                charge.qg_state(),
                charge.qgmid_state(),
                charge.qb_state_for_rbody(self.rbody_enabled()),
                charge.qd_state(),
                charge.qbs,
                charge.qbd,
            )
        };
        let storage = if self.uses_trnqs() {
            Self::trnqs_overlap_charge(&charge)
        } else {
            charge.clone()
        };
        let c = Self::charge_matrix_for_rgate(&storage, mode, self.core.model.rgate_mod);
        let rate = |node| Self::node_voltage(rates, node);
        let (dg, dm, dd, ds, db, ddb, dsb) = (
            rate(self.node_gate),
            rate(self.node_gate_mid),
            rate(self.node_drain),
            rate(self.node_source),
            rate(self.node_bulk),
            rate(self.node_drain_body),
            rate(self.node_source_body),
        );
        let mt = self.core.mtype;
        let flow =
            |g, mid, d, s| mt * (g * (dg - db) + mid * (dm - db) + d * (dd - db) + s * (ds - db));
        let cg = flow(c.gcggb, 0.0, c.gcgdb, c.gcgsb);
        let cmid = flow(0.0, c.gcgmgmb, c.gcgmdb, c.gcgmsb);
        let mut cb = flow(c.gcbgb, c.gcbgmb, c.gcbdb, c.gcbsb);
        let mut cd = flow(c.gcdgb, c.gcdgmb, c.gcddb, c.gcdsb);
        if self.rbody_enabled() {
            cd += mt * charge.capbd * (db - ddb);
            cb += mt * (charge.capbd * (dd - db) + charge.capbs * (ds - db));
        }
        let cbs = mt * charge.capbs * (dsb - ds);
        let cbd = mt * charge.capbd * (ddb - dd);
        let (qcheq, qcdump, ccheq, ccdump) = if self.uses_trnqs() {
            let gate = -(charge.cggb + charge.cbgb);
            let drain = -(charge.cgdb + charge.cbdb);
            let source = -(charge.cgsb + charge.cbsb);
            let bulk = -(gate + drain + source);
            let (drain, source) = if mode > 0 {
                (drain, source)
            } else {
                (source, drain)
            };
            (
                charge.qchqs,
                self.trnqs_qcdump_state(solution),
                mt * (gate * dg + drain * dd + source * ds + bulk * db),
                self.trnqs_qcdump_state(rates),
            )
        } else {
            (0.0, 0.0, 0.0, 0.0)
        };
        let charges = [qg, qmid, qb, qd, qbs, qbd, qcheq, qcdump];
        let currents = [cg, cmid, cb, cd, cbs, cbd, ccheq, ccdump];
        if charges.iter().chain(&currents).any(|v| !v.is_finite()) {
            return Err(format!(
                "BSIM4 '{}' has non-finite periodic charge history",
                self.name
            ));
        }
        Ok((charges, currents))
    }

    pub(crate) fn seed_accepted_periodic_bias(&mut self, solution: &[Value]) {
        let bias = self.raw_branch_voltages(solution);
        let junction = self.raw_junction_bias(solution);
        self.op = self.eval_dc(bias, junction);
        self.bias = bias;
        self.converged_ref = bias;
        self.junction_bias = junction.unwrap_or(Bsim4v8JunctionBias {
            vbs: bias.vbs,
            vbd: bias.vbs - bias.vds,
        });
        self.converged_junction_ref = self.junction_bias;
        self.von_prev = self.op.von;
        self.has_history = true;
        self.limit_anchor_valid.set(true);
        self.last_limited.set(false);
        self.initial_off_seed_pending = false;
        self.initial_off_seed_evaluations = 0;
        self.initial_off_seed_raw = None;
    }
}
