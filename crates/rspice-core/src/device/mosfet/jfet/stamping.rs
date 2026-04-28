//! Matrix linking, direct stamping, and nonlinear-device trait integration.

use super::*;

impl Jfet {
    /// Link this device to a StaticMatrix for O(1) direct stamping.
    pub fn link(&mut self, matrix: &StaticMatrix) {
        let d = self.drain;
        let g = self.gate;
        let s = self.source;

        if d > 0 {
            self.indices.dd = matrix.get_index(d - 1, d - 1);
        }
        if d > 0 && g > 0 {
            self.indices.dg = matrix.get_index(d - 1, g - 1);
        }
        if d > 0 && s > 0 {
            self.indices.ds = matrix.get_index(d - 1, s - 1);
        }

        if g > 0 && d > 0 {
            self.indices.gd = matrix.get_index(g - 1, d - 1);
        }
        if g > 0 {
            self.indices.gg = matrix.get_index(g - 1, g - 1);
        }
        if g > 0 && s > 0 {
            self.indices.gs = matrix.get_index(g - 1, s - 1);
        }

        if s > 0 && d > 0 {
            self.indices.sd = matrix.get_index(s - 1, d - 1);
        }
        if s > 0 && g > 0 {
            self.indices.sg = matrix.get_index(s - 1, g - 1);
        }
        if s > 0 {
            self.indices.ss = matrix.get_index(s - 1, s - 1);
        }
    }

    /// Stamp using O(1) direct indexing (call after `link`).
    pub fn stamp_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value], voltages: &[Value]) {
        let (vgs, vds, vgd) = self.state_or_raw_branch_voltages(voltages);

        let (ids, gm, gds, igs, igd, ggs, ggd, vds_linear) = if self.eval_valid {
            (
                self.eval_ids,
                self.eval_gm,
                self.eval_gds,
                self.eval_igs,
                self.eval_igd,
                self.eval_ggs,
                self.eval_ggd,
                self.eval_vds_linear,
            )
        } else {
            self.compute_operating_terms(vgs, vds, vgd)
        };
        let ids_eq = ids - gm * vgs - gds * vds_linear;
        let igs_eq = igs - ggs * vgs;
        let igd_eq = igd - ggd * vgd;

        // Drain row
        if let Some(idx) = self.indices.dd {
            matrix.stamp_direct(idx, gds + ggd);
        }
        if let Some(idx) = self.indices.dg {
            matrix.stamp_direct(idx, gm - ggd);
        }
        if let Some(idx) = self.indices.ds {
            matrix.stamp_direct(idx, -gm - gds);
        }

        // Gate row
        if let Some(idx) = self.indices.gd {
            matrix.stamp_direct(idx, -ggd);
        }
        if let Some(idx) = self.indices.gg {
            matrix.stamp_direct(idx, ggs + ggd);
        }
        if let Some(idx) = self.indices.gs {
            matrix.stamp_direct(idx, -ggs);
        }

        // Source row
        if let Some(idx) = self.indices.sd {
            matrix.stamp_direct(idx, -gds);
        }
        if let Some(idx) = self.indices.sg {
            matrix.stamp_direct(idx, -gm - ggs);
        }
        if let Some(idx) = self.indices.ss {
            matrix.stamp_direct(idx, gm + gds + ggs);
        }

        if self.drain > 0 {
            rhs[self.drain - 1] -= ids_eq - igd_eq;
        }
        if self.gate > 0 {
            rhs[self.gate - 1] -= igs_eq + igd_eq;
        }
        if self.source > 0 {
            rhs[self.source - 1] += ids_eq + igs_eq;
        }
    }
}

impl NonlinearDevice for Jfet {
    fn update(&mut self, voltages: &[Value]) {
        let vd = Self::node_voltage(voltages, self.drain);
        let vg = Self::node_voltage(voltages, self.gate);
        let vs = Self::node_voltage(voltages, self.source);
        let vgs_raw = vg - vs;
        let vgd_raw = vg - vd;
        if self.matches_last_raw_branch_input(vgs_raw, vgd_raw) {
            if self.vgs.is_finite() && self.vds.is_finite() {
                self.vgs_prev = self.vgs;
                self.vds_prev = self.vds;
            }
            return;
        }

        let vgs_prev = self.vgs;
        let vds_prev = self.vds;
        let vgd_prev = if vgs_prev.is_finite() && vds_prev.is_finite() {
            vgs_prev - vds_prev
        } else {
            Value::NAN
        };

        self.vgs_prev = vgs_prev;
        self.vds_prev = vds_prev;

        let mut vgs = vgs_raw;
        let mut vgd = vgd_raw;
        let mut limiter_applied = false;

        if matches!(self.params.channel_model, JfetChannelModel::Hfet1) {
            if matches!(self.params.hfet_level, 2..=4)
                && vgs_prev.is_finite()
                && vgd_prev.is_finite()
            {
                let (_, temp_source, temp_drain) = self.resolved_temperatures(self.params.tnom);
                let n = self.params.n.max(1e-12);
                let vtes = (n * self.thermal_voltage(temp_source)).max(1e-12);
                let vted = (n * self.thermal_voltage(temp_drain)).max(1e-12);
                let vcrits = self.mesa_gate_vcrit(temp_source, vtes);
                let vcritd = self.mesa_gate_vcrit(temp_drain, vted);
                let vgs_limited = Self::pnjlim(vgs, vgs_prev, vtes, vcrits);
                let vgd_limited = Self::pnjlim(vgd, vgd_prev, vted, vcritd);
                limiter_applied |= (vgs_limited - vgs).abs() > 0.0;
                limiter_applied |= (vgd_limited - vgd).abs() > 0.0;
                vgs = vgs_limited;
                vgd = vgd_limited;
            }
            let (vgs_limited, vgd_limited) = self.hfet_limited_branch_voltages(vgs, vgd);
            vgs = vgs_limited;
            vgd = vgd_limited;
        }

        let mut bypassed = false;
        if self.eval_valid && vgs_prev.is_finite() && vgd_prev.is_finite() {
            const RELTOL: Value = 1e-3;
            const VOLT_TOL: Value = 1e-6;
            const ABSTOL: Value = 1e-12;

            let delvgs = vgs - vgs_prev;
            let delvgd = vgd - vgd_prev;
            let delvds = delvgs - delvgd;

            let cghat = self.lin_cg + self.eval_ggs * delvgs + self.eval_ggd * delvgd;
            let cdhat = self.lin_cd + self.eval_gm * delvgs + self.eval_gds * delvds
                - self.eval_ggd * delvgd;

            let vgs_ok = delvgs.abs() <= RELTOL * vgs.abs().max(vgs_prev.abs()) + VOLT_TOL;
            let vgd_ok = delvgd.abs() <= RELTOL * vgd.abs().max(vgd_prev.abs()) + VOLT_TOL;
            let cg_ok =
                (cghat - self.lin_cg).abs() <= RELTOL * cghat.abs().max(self.lin_cg.abs()) + ABSTOL;
            let cd_ok =
                (cdhat - self.lin_cd).abs() <= RELTOL * cdhat.abs().max(self.lin_cd.abs()) + ABSTOL;

            if vgs_ok && vgd_ok && cg_ok && cd_ok {
                vgs = vgs_prev;
                vgd = vgd_prev;
                bypassed = true;
            }
        }

        let vds = vgs - vgd;
        self.vgs = vgs;
        self.vds = vds;
        self.limiter_applied = limiter_applied;
        self.last_raw_vgs = vgs_raw;
        self.last_raw_vgd = vgd_raw;

        if !bypassed {
            let (ids, gm, gds, igs, igd, ggs, ggd, vds_linear) =
                self.compute_operating_terms(vgs, vds, vgd);
            self.eval_ids = ids;
            self.eval_gm = gm;
            self.eval_gds = gds;
            self.eval_igs = igs;
            self.eval_igd = igd;
            self.eval_ggs = ggs;
            self.eval_ggd = ggd;
            self.eval_vds_linear = vds_linear;
            self.lin_vgs = vgs;
            self.lin_vgd = vgd;
            self.lin_cg = igs + igd;
            self.lin_cd = ids - igd;
            self.eval_valid = true;
        }
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let (vgs, vds, vgd) = self.state_or_raw_branch_voltages(voltages);

        let (ids, gm, gds, igs, igd, ggs, ggd, vds_linear) = if self.eval_valid {
            (
                self.eval_ids,
                self.eval_gm,
                self.eval_gds,
                self.eval_igs,
                self.eval_igd,
                self.eval_ggs,
                self.eval_ggd,
                self.eval_vds_linear,
            )
        } else {
            self.compute_operating_terms(vgs, vds, vgd)
        };
        let ids_eq = ids - gm * vgs - gds * vds_linear;
        let igs_eq = igs - ggs * vgs;
        let igd_eq = igd - ggd * vgd;

        matrix.stamp(self.drain, self.drain, gds + ggd);
        matrix.stamp(self.drain, self.gate, gm - ggd);
        matrix.stamp(self.drain, self.source, -gm - gds);

        matrix.stamp(self.gate, self.drain, -ggd);
        matrix.stamp(self.gate, self.gate, ggs + ggd);
        matrix.stamp(self.gate, self.source, -ggs);

        matrix.stamp(self.source, self.drain, -gds);
        matrix.stamp(self.source, self.gate, -gm - ggs);
        matrix.stamp(self.source, self.source, gm + gds + ggs);

        matrix.stamp_rhs(self.drain, -ids_eq + igd_eq);
        matrix.stamp_rhs(self.gate, -igs_eq - igd_eq);
        matrix.stamp_rhs(self.source, ids_eq + igs_eq);
    }

    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        const RELTOL: Value = 1e-3;
        let tolerance = criteria.voltage_tolerance();

        if self.limiter_applied {
            return false;
        }

        if !self.vgs.is_finite()
            || !self.vgs_prev.is_finite()
            || !self.vds.is_finite()
            || !self.vds_prev.is_finite()
        {
            return false;
        }

        let vgs_diff = (self.vgs - self.vgs_prev).abs();
        let vds_diff = (self.vds - self.vds_prev).abs();
        let vgs_tol = RELTOL * self.vgs.abs().max(self.vgs_prev.abs()) + tolerance;
        let vds_tol = RELTOL * self.vds.abs().max(self.vds_prev.abs()) + tolerance;

        vgs_diff < vgs_tol && vds_diff < vds_tol
    }
}

//=============================================================================
// Tests
//=============================================================================
