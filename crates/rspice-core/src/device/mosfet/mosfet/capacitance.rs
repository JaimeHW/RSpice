use super::*;

impl Mosfet {
    /// Calculate effective threshold voltage with body effect and fallback short-channel effects.
    ///
    /// For Level 1 and MOS6: standard body effect formula.
    /// For opt-in simplified fallback levels: approximate short-channel Vth roll-off.
    pub(in crate::device::mosfet::mosfet) fn vth(&self, vbs: Value) -> Value {
        if let Some(legacy) = &self.legacy_bsim_sized {
            return legacy.threshold(0.0, self.polarity() * vbs);
        }

        let p = self.polarity();
        let vbs_eff = p * vbs;
        let vto_eff = match self.mos_type {
            MosType::Nmos => self.vto,
            MosType::Pmos => self.vto.abs(),
        };

        // Base body effect: Vth = Vto + gamma * (sqrt(phi - Vbs) - sqrt(phi))
        let phi_vbs = (self.phi - vbs_eff).max(0.0);
        let vth_base = vto_eff + self.gamma * (phi_vbs.sqrt() - self.phi.sqrt());

        if self.level < 3 || self.level == 6 {
            // Level 1 and Level 6 use simple body effect
            return vth_base;
        }

        // Simplified fallback short-channel Vth roll-off
        // Delta_Vth = -DVT0 * L_eff / Ldrawn * (1 + DVT2 * Vbs)
        // where L_eff adjustment factor uses DVT1
        let l_ratio = 1e-6 / self.l.max(1e-9); // Normalize to 1um
        let dvth_sce = -self.dvt0 * l_ratio * (1.0 + self.dvt1 * l_ratio);

        // Body-bias modulation of SCE
        let dvth_bias = self.dvt2 * vbs_eff * l_ratio;

        // Enhanced body effect using K1/K2 (BSIM4 style)
        // Vth = Vto + K1 * sqrt(phi - Vbs) + K2 * (phi - Vbs)
        let vth_k1k2 = vto_eff + self.k1 * phi_vbs.sqrt() + self.k2 * (self.phi - vbs_eff);

        // Blend between GAMMA-based and K1/K2-based body effect based on model level
        // Use K1/K2 formulation for the simplified short-channel fallback.
        vth_k1k2 + dvth_sce + dvth_bias
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn level6_effective_length(&self) -> Value {
        let leff = self.l - 2.0 * self.ld;
        leff.max(1e-12)
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn classic_meyer_effective_length(&self) -> Value {
        match self.level {
            1 | 6 => self.level6_effective_length(),
            2 => self.level2_effective_length(),
            3 => self.mos3_effective_length(),
            _ => self.l,
        }
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn oxide_capacitance_total(&self) -> Value {
        self.cox * self.w * self.classic_meyer_effective_length()
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn meyer_intrinsic_capacitances(
        vgs: Value,
        vgd: Value,
        vgb: Value,
        von: Value,
        vdsat: Value,
        phi: Value,
        oxide_cap: Value,
    ) -> (Value, Value, Value) {
        let _ = vgb;
        const MAGIC_VDS: Value = 0.025;

        let vgst = vgs - von;
        let vds = vgs - vgd;
        let vdsat = vdsat.max(MAGIC_VDS);

        if vgst <= -phi {
            (0.0, 0.0, oxide_cap / 2.0)
        } else if vgst <= -phi / 2.0 {
            (0.0, 0.0, -vgst * oxide_cap / (2.0 * phi))
        } else if vgst <= 0.0 {
            let mut capgs = vgst * oxide_cap / (1.5 * phi) + oxide_cap / 3.0;
            let capgd = if vds >= vdsat {
                0.0
            } else {
                let vddif = 2.0 * vdsat - vds;
                let vddif1 = vdsat - vds;
                let vddif2 = vddif * vddif;
                let capgd = capgs * (1.0 - vdsat * vdsat / vddif2);
                capgs *= 1.0 - vddif1 * vddif1 / vddif2;
                capgd
            };
            let capgb = -vgst * oxide_cap / (2.0 * phi);
            (capgs.max(0.0), capgd.max(0.0), capgb.max(0.0))
        } else if vdsat <= vds {
            (oxide_cap / 3.0, 0.0, 0.0)
        } else {
            let vddif = 2.0 * vdsat - vds;
            let vddif1 = vdsat - vds;
            let vddif2 = vddif * vddif;
            let capgd = oxide_cap * (1.0 - vdsat * vdsat / vddif2) / 3.0;
            let capgs = oxide_cap * (1.0 - vddif1 * vddif1 / vddif2) / 3.0;
            (capgs.max(0.0), capgd.max(0.0), 0.0)
        }
    }

    pub(in crate::device::mosfet::mosfet) fn level6_meyer_state(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, Value, Value) {
        let p = self.polarity();
        let vgs_m = p * vgs;
        let vds_m = p * vds;
        let vbs_m = p * vbs;
        let vbd_m = vbs_m - vds_m;
        let vgd_m = vgs_m - vds_m;

        let mode = if vds_m >= 0.0 { 1.0 } else { -1.0 };
        let vdshere = vds_m * mode;
        let vbsvbd = if mode > 0.0 { vbs_m } else { vbd_m };
        let vg_active = if mode > 0.0 { vgs_m } else { vgd_m };

        let phi = self.phi.max(1e-12);
        let sqrt_phi = phi.sqrt();
        let sarg1 = if vbsvbd <= 0.0 {
            (phi - vbsvbd).max(0.0).sqrt()
        } else {
            (sqrt_phi - vbsvbd / (2.0 * sqrt_phi.max(1e-12))).max(0.0)
        };
        let von = p * self.vto + self.gamma * (sarg1 - sqrt_phi)
            - self.gamma1 * vbsvbd
            - self.sigma * vdshere;
        let vgon = (vg_active - von).max(0.0);
        let vdsat = if vgon > 0.0 {
            self.kv * vgon.powf(self.nv)
        } else {
            0.0
        };

        (mode, von, vdsat)
    }

    pub(crate) fn transient_capacitance_halves_at(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, Value, Value) {
        let oxide_cap = self.oxide_capacitance_total();
        let phi = self.phi.max(1e-12);
        let p = self.polarity();
        let vgs_m = p * vgs;
        let vds_m = p * vds;
        let vbs_m = p * vbs;
        let vgd_m = vgs_m - vds_m;
        let vgb_m = vgs_m - vbs_m;

        let (mode, von, vdsat) = if self.level == 6 {
            self.level6_meyer_state(vgs, vds, vbs)
        } else if self.level == 2 {
            let eval = self.level2_evaluate(vgs, vds, vbs);
            let mode = if vds_m >= 0.0 { 1.0 } else { -1.0 };
            (mode, eval.von, eval.vdsat)
        } else if self.level == 3 {
            let state = self.mos3_state(vgs, vds, vbs);
            let mode = if vds_m >= 0.0 { 1.0 } else { -1.0 };
            let p = self.polarity();
            (mode, p * state.von, p * state.vdsat)
        } else {
            let mode = if vds_m >= 0.0 { 1.0 } else { -1.0 };
            let vg_active = if mode > 0.0 { vgs_m } else { vgd_m };
            let von = self.vth(vbs);
            let vdsat = (vg_active - von).max(0.0);
            (mode, von, vdsat)
        };

        let (cgs_int, cgd_int, cgb_int) = if mode > 0.0 {
            Self::meyer_intrinsic_capacitances(vgs_m, vgd_m, vgb_m, von, vdsat, phi, oxide_cap)
        } else {
            let (capgd_int, capgs_int, capgb_int) =
                Self::meyer_intrinsic_capacitances(vgd_m, vgs_m, vgb_m, von, vdsat, phi, oxide_cap);
            (capgs_int, capgd_int, capgb_int)
        };

        (cgs_int, cgd_int, cgb_int)
    }

    /// Calculate overlap capacitances for AC analysis
    /// Returns (Cgs_overlap, Cgd_overlap, Cgb_overlap)
    pub fn overlap_capacitances(&self) -> (Value, Value, Value) {
        // Cgs_overlap = CGSO * W
        let cgs = self.cgso * self.w;
        // Cgd_overlap = CGDO * W
        let cgd = self.cgdo * self.w;
        let cgb_length = self.classic_meyer_effective_length();
        let cgb = self.cgbo * cgb_length;

        (cgs, cgd, cgb)
    }

    /// Calculate total AC small-signal capacitances using Meyer model
    ///
    /// Returns (Cgs, Cgd, Cgb) including both intrinsic channel capacitances
    /// and overlap capacitances. Values depend on operating region.
    ///
    /// # Meyer Capacitance Model
    /// - Cutoff: Cgb dominates, Cgs = Cgd = overlap only
    /// - Linear: Cgs = Cgd = Cox*W*L/2 + overlap
    /// - Saturation: Cgs = 2/3*Cox*W*L + overlap, Cgd = overlap only
    pub fn ac_capacitances(&self) -> (Value, Value, Value) {
        let (cgs_ov, cgd_ov, cgb_ov) = self.overlap_capacitances();

        if self.level == 3 {
            let (cgs_int, cgd_int, cgb_int) =
                self.transient_capacitance_halves_at(self.vgs, self.vds, self.vbs);
            return (
                2.0 * cgs_int + cgs_ov,
                2.0 * cgd_int + cgd_ov,
                2.0 * cgb_int + cgb_ov,
            );
        }

        // Intrinsic gate oxide capacitance
        let cox_wl = self.cox * self.w * self.classic_meyer_effective_length();

        // Determine operating region from stored values
        let vgs_eff = self.polarity() * self.vgs;
        let vds_eff = self.polarity() * self.vds;
        let vth = self.vth(self.vbs);
        let vgt = vgs_eff - self.polarity() * vth;

        if vgt <= 0.0 {
            // Cutoff region: only overlap capacitances, Cgb = Cox*W*L
            (cgs_ov, cgd_ov, cox_wl + cgb_ov)
        } else if vds_eff < vgt {
            // Linear region: symmetric distribution
            let cgs_int = 0.5 * cox_wl;
            let cgd_int = 0.5 * cox_wl;
            (cgs_int + cgs_ov, cgd_int + cgd_ov, cgb_ov)
        } else {
            // Saturation region: 2/3 to source, nearly zero to drain
            let cgs_int = (2.0 / 3.0) * cox_wl;
            let cgd_int = 0.0; // Small in saturation
            (cgs_int + cgs_ov, cgd_int + cgd_ov, cgb_ov)
        }
    }

    /// Return the cached drain current at the converged operating point.
    pub fn drain_current(&self) -> Value {
        self.id
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn cached_eval_branch_voltages(
        &self,
    ) -> Option<(Value, Value, Value)> {
        if !self.has_branch_history
            || !self.vgs.is_finite()
            || !self.vds.is_finite()
            || !self.vbs.is_finite()
            || !self.eval_vgs.is_finite()
            || !self.eval_vds.is_finite()
            || !self.eval_vbs.is_finite()
        {
            return None;
        }

        Some((self.eval_vgs, self.eval_vds, self.eval_vbs))
    }

    #[inline]
    pub(crate) fn eval_branch_voltages_at(&self, voltages: &[Value]) -> (Value, Value, Value) {
        let (vgs, vds, vbs) = self.branch_voltages(voltages);
        if self.has_branch_history && self.vgs == vgs && self.vds == vds && self.vbs == vbs {
            if let Some(eval) = self.cached_eval_branch_voltages() {
                return eval;
            }
        }

        self.limited_branch_voltages_for_eval(vgs, vds, vbs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(label: &str, actual: Value, expected: Value, rel: Value, abs: Value) {
        let diff = (actual - expected).abs();
        let tol = abs.max(rel * expected.abs().max(actual.abs()));
        assert!(
            diff <= tol,
            "{label}: actual={actual:.12e} expected={expected:.12e} diff={diff:.12e} tol={tol:.12e}"
        );
    }

    fn assert_caps_close(
        actual: (Value, Value, Value),
        expected: (Value, Value, Value),
        rel: Value,
        abs: Value,
    ) {
        assert_close("cgs", actual.0, expected.0, rel, abs);
        assert_close("cgd", actual.1, expected.1, rel, abs);
        assert_close("cgb", actual.2, expected.2, rel, abs);
    }

    fn mos3_capacitance_fixture() -> Mosfet {
        const EPS0: Value = 8.854_214_871e-12;
        let mut mos = Mosfet::new_nmos("m1".to_string(), 1, 2, 3, 4);
        mos.level = 3;
        mos.l = 1.2e-6;
        mos.w = 12.0e-6;
        mos.ld = 0.08e-6;
        mos.vto = 0.72;
        mos.kp = 55.0e-6;
        mos.gamma = 0.62;
        mos.phi = 0.68;
        mos.cox = 3.9 * EPS0 / 22.0e-9;
        mos.u0 = 600.0;
        mos.cgso = 0.9e-10;
        mos.cgdo = 1.1e-10;
        mos.cgbo = 4.0e-10;
        mos.mos3_eta = 0.18;
        mos.mos3_theta = 0.05;
        mos.mos3_kappa = 0.35;
        mos.mos3_delta = 0.22;
        mos.mos3_fast_surface_state_density = 8.0e11;
        mos.mos3_max_drift_velocity = 8.0e4;
        mos.mos3_junction_depth = 0.18e-6;
        mos.mos3_length_adjust = 0.03e-6;

        let epssil = 11.7 * EPS0;
        mos.mos3_narrow_factor = mos.mos3_delta * 0.5 * std::f64::consts::PI * epssil / mos.cox;
        mos
    }

    fn expected_mos3_intrinsic_caps(
        mos: &Mosfet,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, Value, Value) {
        let p = mos.polarity();
        let vgs_m = p * vgs;
        let vds_m = p * vds;
        let vbs_m = p * vbs;
        let vgd_m = vgs_m - vds_m;
        let vgb_m = vgs_m - vbs_m;
        let state = mos.mos3_state(vgs, vds, vbs);
        let von = p * state.von;
        let vdsat = p * state.vdsat;
        let oxide_cap = mos.cox * mos.w * mos.mos3_effective_length();
        let phi = mos.phi.max(1.0e-12);

        if vds_m >= 0.0 {
            Mosfet::meyer_intrinsic_capacitances(vgs_m, vgd_m, vgb_m, von, vdsat, phi, oxide_cap)
        } else {
            let (capgd_int, capgs_int, capgb_int) = Mosfet::meyer_intrinsic_capacitances(
                vgd_m, vgs_m, vgb_m, von, vdsat, phi, oxide_cap,
            );
            (capgs_int, capgd_int, capgb_int)
        }
    }

    fn expected_mos3_ac_caps(
        mos: &Mosfet,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, Value, Value) {
        let (cgs_int, cgd_int, cgb_int) = expected_mos3_intrinsic_caps(mos, vgs, vds, vbs);
        let leff = mos.mos3_effective_length();
        (
            2.0 * cgs_int + mos.cgso * mos.w,
            2.0 * cgd_int + mos.cgdo * mos.w,
            2.0 * cgb_int + mos.cgbo * leff,
        )
    }

    #[test]
    fn level1_meyer_capacitance_uses_lateral_diffusion_effective_length() {
        let mut mos = Mosfet::new_nmos("m1".to_string(), 1, 2, 3, 0);
        mos.level = 1;
        mos.l = 1.2e-6;
        mos.w = 5.0e-6;
        mos.ld = 0.1e-6;
        mos.cox = 1.742e-3;
        mos.cgbo = 4.0e-10;

        let leff = mos.l - 2.0 * mos.ld;
        assert!((mos.oxide_capacitance_total() - mos.cox * mos.w * leff).abs() < 1.0e-30);
        assert!((mos.overlap_capacitances().2 - mos.cgbo * leff).abs() < 1.0e-30);
    }

    #[test]
    fn level3_meyer_capacitance_uses_mos3_effective_length() {
        let mos = mos3_capacitance_fixture();
        let leff = mos.l - 2.0 * mos.ld + mos.mos3_length_adjust;

        assert_close(
            "classic Meyer Leff",
            mos.classic_meyer_effective_length(),
            leff,
            0.0,
            1.0e-30,
        );
        assert_close(
            "oxide cap",
            mos.oxide_capacitance_total(),
            mos.cox * mos.w * leff,
            0.0,
            1.0e-30,
        );
        assert_close(
            "gate-bulk overlap",
            mos.overlap_capacitances().2,
            mos.cgbo * leff,
            0.0,
            1.0e-30,
        );
    }

    #[test]
    fn level3_model_space_onset_uses_mos3_state() {
        let mos = mos3_capacitance_fixture();
        let (vgs, vds, vbs) = (3.0, 2.5, -0.6);
        let expected = mos.mos3_state(vgs, vds, vbs).von;
        let old_fallback = mos.vth(vbs);

        assert!(
            (expected - old_fallback).abs() > 0.25,
            "MOS3 fixture must distinguish native von from generic vth fallback"
        );
        assert_close(
            "MOS3 onset",
            mos.model_space_onset_voltage(vgs, vds, vbs),
            expected,
            1.0e-12,
            1.0e-12,
        );
    }

    #[test]
    fn level3_pmos_model_space_onset_is_polarity_folded() {
        let mut mos = mos3_capacitance_fixture();
        mos.mos_type = MosType::Pmos;
        mos.vto = -0.72;
        let (vgs, vds, vbs) = (-3.0, -2.5, 0.6);
        let expected = mos.polarity() * mos.mos3_state(vgs, vds, vbs).von;

        assert!(
            expected > 0.0,
            "PMOS limiting onset should be returned in model-space volts"
        );
        assert_close(
            "PMOS MOS3 onset",
            mos.model_space_onset_voltage(vgs, vds, vbs),
            expected,
            1.0e-12,
            1.0e-12,
        );
    }

    #[test]
    fn level3_transient_meyer_caps_use_mos3_von_and_vdsat() {
        let mos = mos3_capacitance_fixture();
        let (vgs, vds, vbs) = (3.0, 0.8, -0.6);
        let expected = expected_mos3_intrinsic_caps(&mos, vgs, vds, vbs);

        let old_von = mos.vth(vbs);
        let old_vdsat = (vgs - old_von).max(0.0);
        let old_oxide_cap = mos.cox * mos.w * mos.l;
        let old_fallback = Mosfet::meyer_intrinsic_capacitances(
            vgs,
            vgs - vds,
            vgs - vbs,
            old_von,
            old_vdsat,
            mos.phi,
            old_oxide_cap,
        );
        assert!(
            (expected.0 - old_fallback.0).abs()
                + (expected.1 - old_fallback.1).abs()
                + (expected.2 - old_fallback.2).abs()
                > 1.0e-16,
            "MOS3 fixture must reject the old vth/Vgs-vth Meyer inputs"
        );

        assert_caps_close(
            mos.transient_capacitance_halves_at(vgs, vds, vbs),
            expected,
            1.0e-12,
            1.0e-24,
        );
    }

    #[test]
    fn level3_ac_capacitances_use_mos3_state_and_inverse_swap() {
        let mut mos = mos3_capacitance_fixture();
        mos.vgs = 2.4;
        mos.vds = -0.8;
        mos.vbs = -0.2;
        let expected = expected_mos3_ac_caps(&mos, mos.vgs, mos.vds, mos.vbs);

        assert!(
            (expected.0 - expected.1).abs() > 1.0e-16,
            "inverse-mode fixture must expose source/drain Meyer cap swapping"
        );
        assert_caps_close(mos.ac_capacitances(), expected, 1.0e-12, 1.0e-24);
    }
}
