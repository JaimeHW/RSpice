use super::*;

/// Snapshot of a MOSFET's cached small-signal operating point, for
/// device operating-point reporting (the Spectre-style OP info table).
#[derive(Debug, Clone, Copy)]
pub struct MosfetOpValues {
    /// Operating region name: "cutoff" | "linear" | "saturation".
    pub region: &'static str,
    /// Drain current (A).
    pub id: Value,
    /// Gate-source voltage (V).
    pub vgs: Value,
    /// Drain-source voltage (V).
    pub vds: Value,
    /// Bulk-source voltage (V).
    pub vbs: Value,
    /// Threshold voltage at the operating back-bias (V).
    pub vth: Value,
    /// Drain saturation voltage (V).
    pub vdsat: Value,
    /// Transconductance dId/dVgs (S).
    pub gm: Value,
    /// Output conductance dId/dVds (S).
    pub gds: Value,
    /// Back-gate transconductance dId/dVbs (S).
    pub gmb: Value,
}

impl Mosfet {
    /// Cached operating-point values from the last accepted Newton solution.
    pub fn op_values(&self) -> MosfetOpValues {
        let (vth, vdsat, gm, gds, gmb) = self.model_space_op_values();
        let id = if self.uses_mos3_core() {
            self.polarity() * self.id
        } else {
            self.id
        };
        MosfetOpValues {
            region: match self.region {
                MosRegion::Cutoff => "cutoff",
                MosRegion::Linear => "linear",
                MosRegion::Saturation => "saturation",
            },
            id,
            vgs: self.vgs,
            vds: self.vds,
            vbs: self.vbs,
            vth,
            vdsat,
            gm,
            gds,
            gmb,
        }
    }

    fn model_space_op_values(&self) -> (Value, Value, Value, Value, Value) {
        if self.uses_mos3_core() {
            let state = self.mos3_state(self.eval_vgs, self.eval_vds, self.eval_vbs);
            return (state.von, state.vdsat, state.gm, state.gds, state.gmb);
        }

        if self.level == 2 {
            let eval = self.level2_evaluate(self.eval_vgs, self.eval_vds, self.eval_vbs);
            return (eval.von, eval.vdsat, self.gm, self.gds, self.gmb);
        }

        if self.level == 6 {
            let (_, von, vdsat) =
                self.level6_meyer_state(self.eval_vgs, self.eval_vds, self.eval_vbs);
            return (von, vdsat, self.gm, self.gds, self.gmb);
        }

        let p = self.polarity();
        let vgs_m = p * self.eval_vgs;
        let vds_m = p * self.eval_vds;
        let vgd_m = vgs_m - vds_m;
        let mode = if vds_m >= 0.0 { 1.0 } else { -1.0 };
        let vg_active = if mode > 0.0 { vgs_m } else { vgd_m };
        let vth = self.vth(self.eval_vbs);
        (vth, (vg_active - vth).max(0.0), self.gm, self.gds, self.gmb)
    }
}

impl Mosfet {
    pub(crate) fn gate_charge_branch_voltages_at(
        &self,
        voltages: &[Value],
    ) -> (Value, Value, Value) {
        let (vgs, vds, vbs) = self.eval_branch_voltages_at(voltages);
        (vgs, vgs - vds, vgs - vbs)
    }

    /// Return the cached transconductance magnitude at the operating point.
    pub fn transconductance(&self) -> Value {
        self.gm.abs()
    }

    /// Return the current thermal-noise coefficient used for channel noise.
    pub fn channel_thermal_noise_gamma(&self) -> Value {
        self.thermal_noise_gamma.max(0.0)
    }

    /// Flicker-noise source terms `(coefficient, current, af, ef)` for a
    /// density of `coefficient·|current|^af / f^ef`, following the SPICE
    /// NLEV laws of mos1noi.c (mos2/mos3 are identical; NLEV defaults to 2
    /// per mos1set.c) under this model's folded-width representation: the
    /// instance multiplicity is folded into `w`, so ngspice's per-finger
    /// width `W/m` and its explicit `m`/`Id/m` factors recombine into the
    /// `m`-power on the coefficient shown at each arm.
    ///
    /// `Leff = L − 2·LATD`; a zero oxide capacitance falls back to the
    /// 100 nm-oxide default exactly as mos1noi.c does.
    pub fn flicker_noise_source_terms(&self) -> Option<(Value, Value, Value, Value)> {
        if self.kf <= 0.0 || !self.kf.is_finite() {
            return None;
        }

        let cox = if self.cox > 0.0 {
            self.cox
        } else {
            3.9 * 8.854214871e-12 / 1e-7
        };
        let leff = (self.l - 2.0 * self.ld).max(1e-18);
        let width = self.w.max(1e-18);
        let m = self.multiplicity.max(1e-12);
        let af = self.af.max(1e-12);
        let ef = self.ef.max(1e-12);

        match self.nlev {
            0 => Some((
                self.kf * m.powf(1.0 - af) / (leff * leff * cox),
                self.drain_current().abs(),
                af,
                ef,
            )),
            1 => Some((
                self.kf * m.powf(2.0 - af) / (width * leff * cox),
                self.drain_current().abs(),
                af,
                ef,
            )),
            // NLEV 2 and 3 share the gm²-based law; AF moves onto the
            // frequency exponent and the explicit m cancels against the
            // folded width.
            _ => {
                let gm = self.transconductance();
                Some((self.kf * gm * gm / (width * leff * cox), 1.0, 1.0, af))
            }
        }
    }

    //=========================================================================
    // BSIM4-style charge-based model for transient analysis
    // Q = ∫C dV ensures charge conservation (dQ/dt = I)
    //=========================================================================

    /// Calculate total gate charges (Qgs, Qgd, Qgb) using charge-based formulation
    ///
    /// # BSIM4 Charge Model
    /// The charge-based model ensures:
    /// - Charge conservation: Qg = Qgs + Qgd + Qgb
    /// - Correct transient currents: Igs = dQgs/dt
    /// - Smooth transitions between operating regions
    ///
    /// Returns (Qgs, Qgd, Qgb) in Coulombs
    pub fn gate_charges(&self) -> (Value, Value, Value) {
        let (cgs_ov, cgd_ov, cgb_ov) = self.overlap_capacitances();
        let p = self.polarity();
        let vgs = p * self.vgs;
        let vds = p * self.vds;
        let vgd = vgs - vds;
        let vgb = vgs - p * self.vbs;

        // Overlap charges (linear with voltage)
        let qgs_ov = cgs_ov * vgs;
        let qgd_ov = cgd_ov * vgd;
        let qgb_ov = cgb_ov * vgb;

        if self.uses_mos3_core() {
            let oxide_cap = self.oxide_capacitance_total();
            let state = self.mos3_state(self.vgs, self.vds, self.vbs);
            let von = p * state.von;
            let vdsat = (p * state.vdsat).max(0.0);
            let intrinsic = |vg_active: Value, vd_active: Value| {
                let vgt = vg_active - von;
                if vgt <= 0.0 {
                    (0.0, 0.0, oxide_cap * vgb)
                } else if vd_active < vdsat {
                    let veff = vgt - vd_active / 2.0;
                    (
                        0.5 * oxide_cap * veff,
                        0.5 * oxide_cap * (veff - vd_active),
                        0.0,
                    )
                } else {
                    ((2.0 / 3.0) * oxide_cap * vgt, 0.0, 0.0)
                }
            };

            let (qgs_int, qgd_int, qgb_int) = if vds >= 0.0 {
                intrinsic(vgs, vds)
            } else {
                let (qgd_int, qgs_int, qgb_int) = intrinsic(vgd, -vds);
                (qgs_int, qgd_int, qgb_int)
            };
            return (qgs_int + qgs_ov, qgd_int + qgd_ov, qgb_int + qgb_ov);
        }

        let channel_length = if self.level == 6 {
            self.level6_effective_length()
        } else {
            self.l
        };
        let cox_wl = self.cox * self.w * channel_length;

        let vth = p * self.vth(self.vbs);
        let vgt = vgs - vth;

        if vgt <= 0.0 {
            // Cutoff: Qgb = Cox * W * L * Vgb, Qgs = Qgd = overlap only
            let qgb_int = cox_wl * vgb;
            (qgs_ov, qgd_ov, qgb_int + qgb_ov)
        } else if vds < vgt {
            // Linear region: symmetric charge sharing
            // Qgs = Qgd = (Cox*W*L/2) * (Vgs - Vth + Vds/2)
            let veff = vgt - vds / 2.0;
            let qgs_int = 0.5 * cox_wl * veff;
            let qgd_int = 0.5 * cox_wl * (veff - vds);
            (qgs_int + qgs_ov, qgd_int + qgd_ov, qgb_ov)
        } else {
            // Saturation: 2/3 of channel charge to source
            // Qgs = (2/3) * Cox * W * L * Vgt
            // Qgd = 0 (pinched off)
            let qgs_int = (2.0 / 3.0) * cox_wl * vgt;
            let qgd_int = 0.0;
            (qgs_int + qgs_ov, qgd_int + qgd_ov, qgb_ov)
        }
    }

    /// Gate-source charge Qgs in Coulombs
    #[inline]
    pub fn qgs(&self) -> Value {
        self.gate_charges().0
    }

    /// Gate-drain charge Qgd in Coulombs
    #[inline]
    pub fn qgd(&self) -> Value {
        self.gate_charges().1
    }

    /// Gate-bulk charge Qgb in Coulombs
    #[inline]
    pub fn qgb(&self) -> Value {
        self.gate_charges().2
    }

    /// Calculate gate current contribution for transient: Ig = dQg/dt
    ///
    /// Given previous charges and timestep, computes:
    /// - Igs = (Qgs - Qgs_prev) / dt
    /// - Igd = (Qgd - Qgd_prev) / dt
    /// - Igb = (Qgb - Qgb_prev) / dt
    pub fn gate_currents(
        &self,
        qgs_prev: Value,
        qgd_prev: Value,
        qgb_prev: Value,
        dt: Value,
    ) -> (Value, Value, Value) {
        if dt <= 0.0 {
            return (0.0, 0.0, 0.0);
        }
        let (qgs, qgd, qgb) = self.gate_charges();
        (
            (qgs - qgs_prev) / dt,
            (qgd - qgd_prev) / dt,
            (qgb - qgb_prev) / dt,
        )
    }

    /// Calculate source/drain series resistance (per side)
    /// Returns resistance in Ohms
    pub fn source_drain_resistance(&self) -> Value {
        // Rsd = RDSW / W (per side, so total is 2x)
        // If RSH is specified, add sheet resistance contribution
        if self.rsh > 0.0 {
            // Assume 1 square of S/D diffusion
            self.rdsw / (self.w * 1e6) + self.rsh
        } else {
            self.rdsw / (self.w * 1e6)
        }
    }

    /// Calculate W/L ratio
    pub fn wl_ratio(&self) -> Value {
        self.w / self.l
    }

    /// Beta = KP * W/L
    pub fn beta(&self) -> Value {
        self.kp * self.wl_ratio()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(what: &str, actual: Value, expected: Value, rel: Value, abs: Value) {
        let diff = (actual - expected).abs();
        let tol = abs.max(rel * expected.abs().max(actual.abs()));
        assert!(
            diff <= tol,
            "{what}: actual={actual:.12e} expected={expected:.12e} diff={diff:.12e} tol={tol:.12e}"
        );
    }

    fn assert_charges_close(
        actual: (Value, Value, Value),
        expected: (Value, Value, Value),
        rel: Value,
        abs: Value,
    ) {
        assert_close("qgs", actual.0, expected.0, rel, abs);
        assert_close("qgd", actual.1, expected.1, rel, abs);
        assert_close("qgb", actual.2, expected.2, rel, abs);
    }

    fn mos3_charge_fixture() -> Mosfet {
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
        mos.mos3_width_narrow = 0.4e-6;
        mos.mos3_width_adjust = 0.1e-6;

        let epssil = 11.7 * EPS0;
        mos.mos3_narrow_factor = mos.mos3_delta * 0.5 * std::f64::consts::PI * epssil / mos.cox;
        mos
    }

    fn expected_gate_charges(
        mos: &Mosfet,
        von: Value,
        vdsat: Value,
        oxide_cap: Value,
    ) -> (Value, Value, Value) {
        let (cgs_ov, cgd_ov, cgb_ov) = mos.overlap_capacitances();
        let p = mos.polarity();
        let vgs = p * mos.vgs;
        let vds = p * mos.vds;
        let vgd = vgs - vds;
        let vgb = vgs - p * mos.vbs;
        let vgt = vgs - von;

        let qgs_ov = cgs_ov * vgs;
        let qgd_ov = cgd_ov * vgd;
        let qgb_ov = cgb_ov * vgb;

        if vgt <= 0.0 {
            (qgs_ov, qgd_ov, oxide_cap * vgb + qgb_ov)
        } else if vds < vdsat {
            let veff = vgt - vds / 2.0;
            (
                0.5 * oxide_cap * veff + qgs_ov,
                0.5 * oxide_cap * (veff - vds) + qgd_ov,
                qgb_ov,
            )
        } else {
            ((2.0 / 3.0) * oxide_cap * vgt + qgs_ov, qgd_ov, qgb_ov)
        }
    }

    #[test]
    fn level3_gate_charges_use_mos3_state_and_effective_geometry() {
        let mut mos = mos3_charge_fixture();
        mos.vgs = 3.0;
        mos.vds = 0.8;
        mos.vbs = -0.6;

        let state = mos.mos3_state(mos.vgs, mos.vds, mos.vbs);
        let p = mos.polarity();
        let expected = expected_gate_charges(
            &mos,
            p * state.von,
            p * state.vdsat,
            mos.oxide_capacitance_total(),
        );
        let old_fallback = expected_gate_charges(
            &mos,
            mos.vth(mos.vbs),
            (p * mos.vgs - mos.vth(mos.vbs)).max(0.0),
            mos.cox * mos.w * mos.l,
        );

        assert!(
            (expected.0 - old_fallback.0).abs()
                + (expected.1 - old_fallback.1).abs()
                + (expected.2 - old_fallback.2).abs()
                > 1.0e-16,
            "fixture must distinguish native MOS3 charges from generic fallback"
        );
        assert_charges_close(mos.gate_charges(), expected, 1.0e-12, 1.0e-24);
    }
}
