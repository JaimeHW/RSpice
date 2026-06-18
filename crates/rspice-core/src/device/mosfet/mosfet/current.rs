use super::*;

impl Mosfet {
    /// Voltages with intrinsic source/drain swapped.
    ///
    /// (Vgs', Vds', Vbs') correspond to using original drain as intrinsic source:
    /// - Vgs' = Vgs - Vds = Vg - Vd
    /// - Vds' = -Vds = Vs - Vd
    /// - Vbs' = Vbs - Vds = Vb - Vd
    pub(in crate::device::mosfet::mosfet) fn reverse_voltages(
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, Value, Value) {
        (vgs - vds, -vds, vbs - vds)
    }

    pub(in crate::device::mosfet::mosfet) fn legacy_bsim_current(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, MosRegion) {
        let Some(legacy) = &self.legacy_bsim_sized else {
            return (0.0, MosRegion::Cutoff);
        };
        if !vgs.is_finite() || !vds.is_finite() || !vbs.is_finite() {
            return (0.0, MosRegion::Cutoff);
        }

        let p = self.polarity();
        let vgs_m = p * vgs;
        let vds_m = p * vds;
        let vbs_m = p * vbs;
        let (current, region, mode) = if vds_m >= 0.0 {
            let (current, region) = legacy.evaluate(vgs_m, vds_m, vbs_m);
            (current, region, 1.0)
        } else {
            let (current, region) = legacy.evaluate(vgs_m - vds_m, -vds_m, vbs_m - vds_m);
            (current, region, -1.0)
        };
        let current = if current.is_finite() { current } else { 0.0 };

        (p * mode * current, Self::legacy_region_to_mos(region))
    }

    pub(in crate::device::mosfet::mosfet) fn legacy_bsim_linearized_operating_point(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, MosRegion, Value, Value, Value) {
        if !vgs.is_finite() || !vds.is_finite() || !vbs.is_finite() {
            return (0.0, MosRegion::Cutoff, 0.0, 0.0, 0.0);
        }

        let (id, region) = self.legacy_bsim_current(vgs, vds, vbs);
        let derivative = |dvgs: Value, dvds: Value, dvbs: Value, step: Value| -> Value {
            if step <= 0.0 || !step.is_finite() {
                return 0.0;
            }
            let (plus, _) =
                self.legacy_bsim_current(vgs + dvgs * step, vds + dvds * step, vbs + dvbs * step);
            let (minus, _) =
                self.legacy_bsim_current(vgs - dvgs * step, vds - dvds * step, vbs - dvbs * step);
            let slope = (plus - minus) / (2.0 * step);
            if slope.is_finite() { slope } else { 0.0 }
        };

        let gm_step = 1.0e-6 * vgs.abs().max(1.0);
        let gds_step = 1.0e-6 * vds.abs().max(1.0);
        let gmb_step = 1.0e-6 * vbs.abs().max(1.0);
        let gm = derivative(1.0, 0.0, 0.0, gm_step);
        let gds = derivative(0.0, 1.0, 0.0, gds_step);
        let gmb = derivative(0.0, 0.0, 1.0, gmb_step);
        (id, region, gm, gds, gmb)
    }

    pub(in crate::device::mosfet::mosfet) fn legacy_region_to_mos(
        region: LegacyBsimRegion,
    ) -> MosRegion {
        match region {
            LegacyBsimRegion::Cutoff => MosRegion::Cutoff,
            LegacyBsimRegion::Linear => MosRegion::Linear,
            LegacyBsimRegion::Saturation => MosRegion::Saturation,
        }
    }

    /// Determine operating region and calculate drain current
    pub(in crate::device::mosfet::mosfet) fn calculate_id(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, MosRegion) {
        if self.legacy_bsim_sized.is_some() {
            return self.legacy_bsim_current(vgs, vds, vbs);
        }

        if self.level == 6 {
            return self.calculate_id_level6(vgs, vds, vbs);
        }

        if self.level == 1 {
            let (id, region, _, _, _) = self.level1_operating_point(vgs, vds, vbs);
            return (id, region);
        }

        if self.level == 2 {
            let eval = self.level2_evaluate(vgs, vds, vbs);
            return (eval.id, eval.region);
        }

        if self.level == 3 {
            return self.calculate_id_mos3(vgs, vds, vbs);
        }

        // Superimpose forward and reverse-oriented channel currents to preserve
        // source/drain symmetry while maintaining smooth behavior around Vds = 0.
        let (id_forward, region_forward) = self.calculate_id_forward(vgs, vds, vbs);

        let (vgs_rev, vds_rev, vbs_rev) = Self::reverse_voltages(vgs, vds, vbs);
        let (id_reverse_fwd, region_reverse) = self.calculate_id_forward(vgs_rev, vds_rev, vbs_rev);
        let id = id_forward - id_reverse_fwd;

        // Region is used for reporting only; choose the dominant orientation.
        let region = if id_forward.abs() >= id_reverse_fwd.abs() {
            region_forward
        } else {
            region_reverse
        };

        (id, region)
    }

    /// Source-matched ngspice MOS1 operating point.
    ///
    /// The Level-1 path follows the Shichman-Hodges block in
    /// `mos1load.c`: polarity folding, explicit normal/inverse mode
    /// selection, body-effect onset voltage, and the original analytic
    /// derivatives transformed back to the instance terminal orientation.
    pub(in crate::device::mosfet::mosfet) fn level1_operating_point(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, MosRegion, Value, Value, Value) {
        if !vgs.is_finite() || !vds.is_finite() || !vbs.is_finite() {
            return (0.0, MosRegion::Cutoff, 0.0, 0.0, 0.0);
        }

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

        let phi = self.phi.max(1.0e-12);
        let sqrt_phi = phi.sqrt();
        let sarg = if vbsvbd <= 0.0 {
            (phi - vbsvbd).max(0.0).sqrt()
        } else {
            (sqrt_phi - vbsvbd / (sqrt_phi + sqrt_phi)).max(0.0)
        };

        let von = p * self.vto + self.gamma * (sarg - sqrt_phi);
        let vgst = vg_active - von;
        if !vgst.is_finite() || vgst <= 0.0 {
            return (0.0, MosRegion::Cutoff, 0.0, 0.0, 0.0);
        }

        let arg = if sarg <= 0.0 {
            0.0
        } else {
            self.gamma / (sarg + sarg)
        };
        let effective_length = (self.l - 2.0 * self.ld).max(1.0e-12);
        let beta = self.kp * self.w / effective_length;
        let betap = beta * (1.0 + self.lambda * vdshere);

        let (cdrain, region, gm_model, gds_model, gmb_model) = if vgst <= vdshere {
            let cdrain = 0.5 * betap * vgst * vgst;
            let gm = betap * vgst;
            let gds = 0.5 * self.lambda * beta * vgst * vgst;
            let gmb = gm * arg;
            (cdrain, MosRegion::Saturation, gm, gds, gmb)
        } else {
            let cdrain = betap * vdshere * (vgst - 0.5 * vdshere);
            let gm = betap * vdshere;
            let gds =
                betap * (vgst - vdshere) + self.lambda * beta * vdshere * (vgst - 0.5 * vdshere);
            let gmb = gm * arg;
            (cdrain, MosRegion::Linear, gm, gds, gmb)
        };

        let id = p * mode * cdrain;
        let (gm, gds, gmb) = if mode > 0.0 {
            (gm_model, gds_model, gmb_model)
        } else {
            (-gm_model, gm_model + gds_model + gmb_model, -gmb_model)
        };

        let sanitize = |value: Value| if value.is_finite() { value } else { 0.0 };
        (
            sanitize(id),
            region,
            sanitize(gm),
            sanitize(gds),
            sanitize(gmb),
        )
    }

    pub(in crate::device::mosfet::mosfet) fn calculate_id_forward(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, MosRegion) {
        if self.level == 6 {
            self.calculate_id_level6(vgs, vds, vbs)
        } else if self.level >= 3 {
            self.calculate_id_bsim3(vgs, vds, vbs)
        } else {
            self.calculate_id_level1(vgs, vds, vbs)
        }
    }

    /// Level 1 (Shichman-Hodges) drain current calculation with C1 continuous transitions
    ///
    /// Uses smooth blending between regions to ensure continuous first derivatives,
    /// which is critical for Newton-Raphson convergence.
    pub(in crate::device::mosfet::mosfet) fn calculate_id_level1(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, MosRegion) {
        let p = self.polarity();
        let vgs_eff = p * vgs;
        let vds_eff = smooth_positive(p * vds, VDS_SMOOTHING); // Ensure positive Vds
        let vth = self.vth(vbs);

        // Gate overdrive with smooth cutoff transition
        // vgt_smooth ≈ 0 when vgs < vth, ≈ (vgs - vth) when vgs > vth
        let vgt_raw = vgs_eff - vth;
        let vgt = smooth_positive(vgt_raw, SMOOTH_VOLTAGE);

        // Determine effective region for reporting (but calculations are smooth)
        let region = if vgt_raw <= -SMOOTH_VOLTAGE {
            MosRegion::Cutoff
        } else if vds_eff < vgt - SMOOTH_VOLTAGE {
            MosRegion::Linear
        } else {
            MosRegion::Saturation
        };

        // Smooth saturation voltage: Vdsat = min(Vgt, Vds) but smooth
        // This naturally blends linear and saturation regions
        let vdsat = smooth_min(vgt, vds_eff, SMOOTH_VOLTAGE);

        // Unified current equation that smoothly transitions between regions:
        // In linear: Id = beta * (Vgt * Vds - Vds²/2)
        // In saturation: Id = beta/2 * Vgt² (when Vds = Vgt)
        //
        // Using Vdsat as the effective drain voltage gives us both:
        // Id = beta * (Vgt * Vdsat - Vdsat²/2) * (1 + lambda * Vds)
        let id_core = self.beta() * (vgt * vdsat - 0.5 * vdsat * vdsat);
        let id = p * id_core * (1.0 + self.lambda * vds_eff);

        (id, region)
    }

    /// Simplified short-channel fallback current.
    /// Includes:
    /// - Mobility degradation due to vertical electric field
    /// - Velocity saturation
    /// - Drain-Induced Barrier Lowering (DIBL)
    /// - Channel length modulation
    ///
    /// Simplified short-channel fallback current with C1 continuous transitions.
    pub(in crate::device::mosfet::mosfet) fn calculate_id_bsim3(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, MosRegion) {
        let p = self.polarity();
        let vgs_eff = p * vgs;
        let vds_eff = smooth_positive(p * vds, VDS_SMOOTHING);
        let vbs_eff = p * vbs;

        // DIBL: threshold voltage reduction with drain bias (smooth minimum for Vth)
        let vth_dibl = self.vth(vbs) - self.eta0 * vds_eff - self.etab * vbs_eff * vds_eff;
        let vth = smooth_max(vth_dibl, 0.1, SMOOTH_VOLTAGE);

        // Gate overdrive with smooth transition
        let vgt_raw = vgs_eff - vth;
        let vgt = smooth_positive(vgt_raw, SMOOTH_VOLTAGE);

        // Subthreshold current blended smoothly with above-threshold current
        let vt = 0.0259; // Thermal voltage at 300K
        let n = self.nfactor;
        // Smooth blend factor: 0 when well above threshold, 1 when below
        let subthreshold_blend = 1.0 - smooth_step(vgt_raw, SMOOTH_VOLTAGE);
        let i_sub = 1e-12 * (vgt_raw.min(100.0 * vt) / (n * vt)).exp().min(1e6);

        // Mobility degradation (vertical field effect)
        let eeff = vgt / 6e-9; // Assume tox = 6nm
        let mobility = self.u0 / (1.0 + self.ua * eeff + self.ub * eeff * eeff);

        // Effective beta with mobility degradation
        let beta_eff = mobility * 1e-4 * self.cox * self.wl_ratio();

        // Saturation voltage with velocity saturation (smooth formulation)
        let vsat_over_l = self.vsat / self.l;
        let mu_m2 = mobility * 1e-4;
        let vdsat_vel = vgt / (1.0 + vgt / (self.l * vsat_over_l / mu_m2).max(1e-6));

        // Smooth min between Vds and Vdsat for unified linear/saturation
        let vdsat = smooth_min(vdsat_vel, vds_eff, SMOOTH_VOLTAGE);

        // Channel length modulation (smooth)
        let vds_over_vdsat = vds_eff / vdsat_vel.max(1e-6);
        let clm_arg = smooth_positive(vds_over_vdsat - 1.0, 0.01);
        let clm = 1.0 + self.pclm * clm_arg.ln_1p();

        // Unified current equation
        let id_above = beta_eff * (vgt * vdsat - 0.5 * vdsat * vdsat) * clm;

        // Blend subthreshold and above-threshold currents
        let id = p * (subthreshold_blend * i_sub + (1.0 - subthreshold_blend) * id_above);

        // Region determination (for reporting only)
        let region = if vgt_raw <= -SMOOTH_VOLTAGE {
            MosRegion::Cutoff
        } else if vds_eff < vdsat_vel - SMOOTH_VOLTAGE {
            MosRegion::Linear
        } else {
            MosRegion::Saturation
        };

        (id, region)
    }

    /// Source-matched ngspice MOS6 operating point.
    ///
    /// The Level-6 implementation follows the ngspice `mos6load.c` equations:
    /// - polarity-folded model-space voltages
    /// - explicit normal/inverse mode selection from `Vds`
    /// - `vgon`, `vdsat`, `idsat`, and linear-region backoff
    /// - direct inverse-mode Jacobian transform back into original variables
    pub(in crate::device::mosfet::mosfet) fn level6_operating_point(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, MosRegion, Value, Value, Value) {
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
        let vgon = vg_active - von;
        if !vgon.is_finite() || vgon <= 0.0 {
            return (0.0, MosRegion::Cutoff, 0.0, 0.0, 0.0);
        }

        let vonbm = if sarg1 <= 0.0 {
            0.0
        } else if vbsvbd <= 0.0 {
            self.gamma1 + self.gamma / (2.0 * sarg1)
        } else {
            self.gamma1 + self.gamma / (2.0 * sqrt_phi.max(1e-12))
        };

        let betac = self.kc * self.w / self.level6_effective_length();
        let vdsat = self.kv * vgon.powf(self.nv);
        let idsat = betac * vgon.powf(self.nc);
        let lambda = self.lambda0 - self.lambda1 * vbsvbd;

        let mut region = MosRegion::Saturation;
        let mut cdrain = idsat * (1.0 + lambda * vdshere);
        let mut gm_model = cdrain * self.nc / vgon;
        let mut gds_model = gm_model * self.sigma + idsat * lambda;
        let mut gmb_model = gm_model * vonbm - idsat * self.lambda1 * vdshere;

        if vdsat > vdshere {
            region = MosRegion::Linear;
            let vdst = vdshere / vdsat;
            let vdst2 = (2.0 - vdst) * vdst;
            let vdstg = -vdst * self.nv / vgon;
            let ivdst1 = cdrain * (2.0 - 2.0 * vdst);
            cdrain *= vdst2;
            gm_model = gm_model * vdst2 + ivdst1 * vdstg;
            gds_model = gds_model * vdst2 + ivdst1 * (1.0 / vdsat + vdstg * self.sigma);
            gmb_model = gmb_model * vdst2 + ivdst1 * vdstg * vonbm;
        }

        let id = p * mode * cdrain;
        let (gm, gds, gmb) = if mode > 0.0 {
            (gm_model, gds_model, gmb_model)
        } else {
            (-gm_model, gm_model + gds_model + gmb_model, -gmb_model)
        };

        let sanitize = |value: Value| if value.is_finite() { value } else { 0.0 };
        (
            sanitize(id),
            region,
            sanitize(gm),
            sanitize(gds),
            sanitize(gmb),
        )
    }

    pub(in crate::device::mosfet::mosfet) fn calculate_id_level6(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, MosRegion) {
        let (id, region, _, _, _) = self.level6_operating_point(vgs, vds, vbs);
        (id, region)
    }

    /// Calculate (gm, gds, gmb) including both forward and reverse-oriented
    /// channel contributions for source/drain symmetry.
    ///
    /// Levels 1/2/3/6 and the legacy BSIM ports use their model-specific
    /// linearizations. Unsupported simplified levels such as LEVEL=7
    /// differentiate the exact composed current the residual stamps, so their
    /// Jacobian is consistent with `calculate_id` by construction.
    pub(in crate::device::mosfet::mosfet) fn small_signal(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, Value, Value) {
        if self.legacy_bsim_sized.is_some() {
            let (_, _, gm, gds, gmb) = self.legacy_bsim_linearized_operating_point(vgs, vds, vbs);
            return (gm, gds, gmb);
        }

        if self.level == 6 {
            let (_, _, gm, gds, gmb) = self.level6_operating_point(vgs, vds, vbs);
            return (gm, gds, gmb);
        }

        if self.level == 2 {
            let (_, _, gm, gds, gmb) = self.level2_operating_point(vgs, vds, vbs);
            return (gm, gds, gmb);
        }

        if self.level == 3 {
            return self.mos3_terminal_small_signal(vgs, vds, vbs);
        }

        if self.level == 1 {
            let (_, _, gm, gds, gmb) = self.level1_operating_point(vgs, vds, vbs);
            return (gm, gds, gmb);
        }

        // Unsupported simplified fallthroughs: differentiate the exact
        // composed current the residual stamps so the Jacobian is consistent
        // with `calculate_id` by construction. The closed-form expressions
        // previously used here were Level-1 formulas that ignored mobility
        // degradation, velocity saturation, CLM, and subthreshold conduction,
        // so Newton iterated against wrong slopes and AC linearization was
        // wrong. `calculate_id` is C1-smooth with blending widths >= 0.01 V,
        // so a 1 uV central difference sits well inside the smooth regions.
        const FD_STEP: Value = 1.0e-6;
        let id_at = |vgs: Value, vds: Value, vbs: Value| self.calculate_id(vgs, vds, vbs).0;
        let half = 0.5 / FD_STEP;
        let gm = (id_at(vgs + FD_STEP, vds, vbs) - id_at(vgs - FD_STEP, vds, vbs)) * half;
        let gds = (id_at(vgs, vds + FD_STEP, vbs) - id_at(vgs, vds - FD_STEP, vbs)) * half;
        let gmb = (id_at(vgs, vds, vbs + FD_STEP) - id_at(vgs, vds, vbs - FD_STEP)) * half;

        let sanitize = |g: Value| if g.is_finite() { g } else { 0.0 };
        // Keep a tiny output-conductance floor for Newton conditioning, as
        // the previous path did.
        (sanitize(gm), sanitize(gds).max(1e-12), sanitize(gmb))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn simplified_nmos() -> Mosfet {
        let mut params = HashMap::new();
        params.insert("VTO".to_string(), 0.6);
        params.insert("KP".to_string(), 120e-6);
        params.insert("GAMMA".to_string(), 0.4);
        params.insert("PHI".to_string(), 0.7);
        params.insert("LAMBDA".to_string(), 0.02);
        Mosfet::new_nmos("m1".to_string(), 1, 2, 3, 0)
            .with_level(7)
            .with_params(&params)
    }

    fn level3_nmos() -> Mosfet {
        let mut params = HashMap::new();
        params.insert("VTO".to_string(), 0.6);
        params.insert("KP".to_string(), 120e-6);
        params.insert("GAMMA".to_string(), 0.4);
        params.insert("PHI".to_string(), 0.7);
        params.insert("LAMBDA".to_string(), 0.02);
        Mosfet::new_nmos("m1".to_string(), 1, 2, 3, 0)
            .with_level(3)
            .with_params(&params)
    }

    fn mos3_oracle_nmos() -> Mosfet {
        let mut params = HashMap::new();
        params.insert("W".to_string(), 12.0e-6);
        params.insert("L".to_string(), 1.2e-6);
        params.insert("VTO".to_string(), 0.72);
        params.insert("KP".to_string(), 55.0e-6);
        params.insert("GAMMA".to_string(), 0.62);
        params.insert("PHI".to_string(), 0.68);
        params.insert("TOX".to_string(), 22.0e-9);
        params.insert("LD".to_string(), 0.08e-6);
        params.insert("ETA".to_string(), 0.18);
        params.insert("THETA".to_string(), 0.05);
        params.insert("KAPPA".to_string(), 0.35);
        params.insert("NFS".to_string(), 8.0e11);
        params.insert("VMAX".to_string(), 8.0e4);
        params.insert("XJ".to_string(), 0.18e-6);
        params.insert("DELTA".to_string(), 0.22);
        Mosfet::new_nmos("m1".to_string(), 1, 2, 3, 0)
            .with_level(3)
            .with_params(&params)
    }

    fn mos3_oracle_pmos() -> Mosfet {
        let mut params = HashMap::new();
        params.insert("W".to_string(), 18.0e-6);
        params.insert("L".to_string(), 1.5e-6);
        params.insert("VTO".to_string(), -0.82);
        params.insert("KP".to_string(), 32.0e-6);
        params.insert("GAMMA".to_string(), 0.55);
        params.insert("PHI".to_string(), 0.7);
        params.insert("TOX".to_string(), 24.0e-9);
        params.insert("LD".to_string(), 0.06e-6);
        params.insert("ETA".to_string(), 0.12);
        params.insert("THETA".to_string(), 0.04);
        params.insert("KAPPA".to_string(), 0.28);
        params.insert("NFS".to_string(), 5.0e11);
        params.insert("VMAX".to_string(), 7.0e4);
        params.insert("XJ".to_string(), 0.2e-6);
        params.insert("DELTA".to_string(), 0.18);
        Mosfet::new_pmos("m1".to_string(), 1, 2, 3, 0)
            .with_level(3)
            .with_params(&params)
    }

    fn mos3_alpha_nmos() -> Mosfet {
        let mut params = HashMap::new();
        params.insert("W".to_string(), 8.0e-6);
        params.insert("L".to_string(), 0.6e-6);
        params.insert("VTO".to_string(), 0.68);
        params.insert("KP".to_string(), 70.0e-6);
        params.insert("GAMMA".to_string(), 0.58);
        params.insert("PHI".to_string(), 0.68);
        params.insert("TOX".to_string(), 18.0e-9);
        params.insert("NSUB".to_string(), 1.0e16);
        params.insert("LD".to_string(), 0.04e-6);
        params.insert("ETA".to_string(), 0.22);
        params.insert("THETA".to_string(), 0.07);
        params.insert("KAPPA".to_string(), 0.42);
        params.insert("NFS".to_string(), 7.0e11);
        params.insert("VMAX".to_string(), 0.0);
        params.insert("XJ".to_string(), 0.12e-6);
        params.insert("DELTA".to_string(), 0.24);
        Mosfet::new_nmos("m1".to_string(), 1, 2, 3, 0)
            .with_level(3)
            .with_params(&params)
    }

    fn assert_small_signal_matches_state_transform(
        m: &Mosfet,
        bias: (Value, Value, Value),
        label: &str,
    ) {
        let (vgs, vds, vbs) = bias;
        let state = m.mos3_state(vgs, vds, vbs);
        let p = m.polarity();
        let vds_m = p * vds;
        let expected = if vds_m >= 0.0 {
            (state.gm, state.gds, state.gmb)
        } else {
            (-state.gm, state.gm + state.gds + state.gmb, -state.gmb)
        };
        let (gm, gds, gmb) = m.small_signal(vgs, vds, vbs);
        let values = [
            ("gm", gm, expected.0),
            ("gds", gds, expected.1),
            ("gmb", gmb, expected.2),
        ];
        for (name, got, want) in values {
            let tol = 1.0e-14 * want.abs().max(got.abs()).max(1.0);
            assert!(
                (got - want).abs() <= tol,
                "{label} {name} terminal transform mismatch: \
                 got={got:.12e} expected={want:.12e} tol={tol:.12e}"
            );
        }
    }

    #[test]
    fn level3_native_current_uses_mos3_state_for_inverse_bias() {
        let m = level3_nmos();
        let (vgs, vds, vbs) = (0.9, -0.7, -0.2);
        let state = m.mos3_state(vgs, vds, vbs);

        let (actual_id, actual_region) = m.calculate_id(vgs, vds, vbs);

        assert_eq!(actual_region, state.region);
        assert!(
            (actual_id - state.ids).abs() <= 1.0e-18,
            "LEVEL=3 current must come from native MOS3 state: actual={actual_id:.12e} \
             state={:.12e}",
            state.ids
        );
        assert!(
            actual_id <= 0.0,
            "inverse-mode NMOS current should be negative"
        );
    }

    #[test]
    fn level3_small_signal_uses_terminal_state_transform() {
        let cases = [
            ("nmos normal", mos3_oracle_nmos(), (3.0, 2.5, -0.6)),
            ("pmos normal", mos3_oracle_pmos(), (-2.4, -2.8, 0.3)),
            ("nmos inverse", mos3_oracle_nmos(), (0.9, -0.7, -0.2)),
            ("weak alpha clm", mos3_alpha_nmos(), (-0.9, 2.2, -0.4)),
        ];

        for (label, mos, bias) in cases {
            assert_small_signal_matches_state_transform(&mos, bias, label);
        }
    }

    /// The simplified short-channel path must report derivatives consistent
    /// with the current it stamps: gm = dId/dVgs, gds = dId/dVds, and
    /// gmb = dId/dVbs of `calculate_id`. The closed-form Level-1 expressions
    /// this replaced violated that for any card where mobility degradation,
    /// velocity saturation, CLM, or subthreshold conduction mattered.
    #[test]
    fn simplified_path_small_signal_matches_current_derivatives() {
        let m = simplified_nmos();
        // Saturation, triode, subthreshold, body-biased, and reverse-mode.
        let bias_points = [
            (1.5, 1.2, 0.0),
            (1.5, 0.2, 0.0),
            (0.3, 1.0, 0.0),
            (1.2, 0.8, -0.5),
            (0.9, -0.7, -0.2),
        ];
        let h = 1e-4;
        for (vgs, vds, vbs) in bias_points {
            let (gm, gds, gmb) = m.small_signal(vgs, vds, vbs);
            let gm_ref = (m.calculate_id(vgs + h, vds, vbs).0
                - m.calculate_id(vgs - h, vds, vbs).0)
                / (2.0 * h);
            let gds_ref = (m.calculate_id(vgs, vds + h, vbs).0
                - m.calculate_id(vgs, vds - h, vbs).0)
                / (2.0 * h);
            let gmb_ref = (m.calculate_id(vgs, vds, vbs + h).0
                - m.calculate_id(vgs, vds, vbs - h).0)
                / (2.0 * h);
            for (got, want, name) in [
                (gm, gm_ref, "gm"),
                (gds, gds_ref, "gds"),
                (gmb, gmb_ref, "gmb"),
            ] {
                let tol = 1e-3 * want.abs().max(1e-9);
                assert!(
                    (got - want).abs() <= tol,
                    "{name} inconsistent at (vgs={vgs}, vds={vds}, vbs={vbs}): \
                     got {got:.6e}, derivative of calculate_id is {want:.6e}"
                );
            }
        }
    }
}
