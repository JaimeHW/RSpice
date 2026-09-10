use super::*;

/// Snapshot of a MOSFET's cached small-signal operating point, for
/// device operating-point reporting (the Spectre-style OP info table).
#[derive(Debug, Clone, Copy)]
pub(crate) struct MosfetOpValues {
    /// Operating region: cutoff, linear or saturation.
    pub region: crate::op_label::OpLabel,
    /// Drain current (A).
    pub id: Value,
    /// Gate-source voltage (V).
    pub vgs: Value,
    /// Drain-source voltage (V).
    pub vds: Value,
    /// Bulk-source voltage (V).
    pub vbs: Value,
    /// Threshold at the effective source back-bias, with physical polarity (V).
    pub vth: Value,
    /// Drain saturation voltage with physical polarity (V).
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
    pub(crate) fn op_values(&self) -> MosfetOpValues {
        let (vth, vdsat, gm, gds, gmb) = self.reported_op_values();
        let id = if self.uses_mos3_core() {
            self.polarity() * self.id
        } else {
            self.id
        };
        MosfetOpValues {
            region: match self.region {
                MosRegion::Cutoff => crate::op_label::OpLabel::CUTOFF,
                MosRegion::Linear => crate::op_label::OpLabel::LINEAR,
                MosRegion::Saturation => crate::op_label::OpLabel::SATURATION,
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

    fn reported_op_values(&self) -> (Value, Value, Value, Value, Value) {
        if self.uses_mos3_core() {
            let state = self.mos3_state(self.eval_vgs, self.eval_vds, self.eval_vbs);
            return (state.von, state.vdsat, state.gm, state.gds, state.gmb);
        }

        if matches!(self.level, 1 | 2 | 6) {
            let (_, von, vdsat) = self.classic_meyer_state(
                self.eval_vgs,
                self.eval_vds,
                self.eval_vbs,
                self.phi.sqrt(),
            );
            let p = self.polarity();
            return (p * von, p * vdsat, self.gm, self.gds, self.gmb);
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
    pub(crate) fn transconductance(&self) -> Value {
        self.gm.abs()
    }

    /// Resolve channel-noise parameters once for ordinary and periodic
    /// analyses. GDSNOI participates only in ngspice MOS1/2/3 NLEV=3.
    pub(crate) fn channel_noise_parameters(
        &self,
        dialect: crate::config::SpiceDialect,
    ) -> Result<(Value, Option<Value>), &'static str> {
        if self.legacy_bsim_model.is_some() {
            return Ok((2.0 / 3.0, None));
        }
        let gamma = self.thermal_noise_gamma;
        if !gamma.is_finite() || gamma < 0.0 {
            return Err("MOS channel noise requires finite nonnegative TNOIA/NOIA and GAMMA_NOISE");
        }
        if dialect != crate::config::SpiceDialect::Xyce
            && matches!(self.level, 1 | 2 | 3 | 6)
            && !(0..=3).contains(&self.nlev)
        {
            return Err("MOS noise requires integer NLEV in 0..=3");
        }
        let nlev3 = dialect != crate::config::SpiceDialect::Xyce
            && matches!(self.level, 1..=3)
            && self.nlev == 3;
        if nlev3 && (!self.gdsnoi.is_finite() || self.gdsnoi < 0.0) {
            return Err("MOS NLEV=3 channel noise requires finite nonnegative GDSNOI");
        }
        Ok((gamma, nlev3.then_some(self.gdsnoi)))
    }

    /// Conductance multiplying 4*k*T in the channel thermal-noise PSD.
    pub(crate) fn channel_noise_conductance(
        &self,
        dialect: crate::config::SpiceDialect,
    ) -> Result<Value, &'static str> {
        let (gamma, nlev3) = self.channel_noise_parameters(dialect)?;
        let conductance = if let Some(gdsnoi) = nlev3 {
            let p = self.polarity();
            let vds_m = p * self.eval_vds;
            let vg_active = p * self.eval_vgs - vds_m.min(0.0);
            let (von, vdsat) = match self.level {
                1 => {
                    let vb_active = p * self.eval_vbs - vds_m.min(0.0);
                    let (von, _) = crate::device::semiconductor::mos1_threshold(
                        p * self.vto,
                        self.gamma,
                        self.phi,
                        self.phi.sqrt(),
                        vb_active,
                    );
                    (von, (vg_active - von).max(0.0))
                }
                2 => {
                    let state = self.level2_evaluate(self.eval_vgs, self.eval_vds, self.eval_vbs);
                    (state.von, state.vdsat)
                }
                _ => {
                    let state = self.mos3_state(self.eval_vgs, self.eval_vds, self.eval_vbs);
                    (p * state.von, p * state.vdsat)
                }
            };
            let overdrive = vg_active - von;
            let shape =
                crate::device::semiconductor::mos_nlev3_noise_shape(overdrive, vds_m.abs(), vdsat)?;
            crate::numerics::scaled_exp_product(
                &[
                    gamma,
                    gdsnoi,
                    self.kp,
                    self.w,
                    self.multiplicity,
                    overdrive.max(0.0),
                    shape,
                ],
                &[self.l - 2.0 * self.ld],
                0.0,
            )
        } else {
            crate::numerics::scaled_exp_product(&[gamma, self.transconductance()], &[], 0.0)
        };
        if !conductance.is_finite() || conductance < 0.0 {
            return Err("MOS channel-noise conductance must be finite and nonnegative");
        }
        Ok(conductance)
    }

    /// Flicker terms `(coefficient, current, af, ef)` for
    /// `coefficient * |current|^af / f^ef`. Each parallel instance contributes
    /// independently. Ngspice levels 1/2/3 use NLEV (default 2); native MOS6
    /// extends that law because ngspice supplies no MOS6 noise callback.
    /// Xyce levels 1/2/3/6 use current^AF divided by W*Leff*Cox²*f.
    /// MOS9 uses that current law with W-2*WD, without XL/XW mask shifts.
    /// Ngspice MOS3 also uses W-2*WD for its width-dependent NLEV laws.
    /// BSIM1/2 keep their own effective geometry and Cox units.
    pub(crate) fn flicker_noise_source_terms(
        &self,
        dialect: crate::config::SpiceDialect,
    ) -> Result<Option<(Value, Value, Value, Value)>, &'static str> {
        use crate::numerics::scaled_exp_product;
        let xyce =
            dialect == crate::config::SpiceDialect::Xyce && matches!(self.level, 1 | 2 | 3 | 6);
        let fixed_current_law = xyce || self.level == 9;
        if self.legacy_bsim_model.is_none() && !fixed_current_law && !(0..=3).contains(&self.nlev) {
            return Err("MOS noise requires integer NLEV in 0..=3");
        }
        if !self.kf.is_finite() || self.kf < 0.0 {
            return Err("MOS flicker noise requires finite KF >= 0");
        }
        if self.kf == 0.0 {
            return Ok(None);
        }
        if !self.af.is_finite() {
            return Err("MOS flicker noise requires finite AF");
        }
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err("MOS flicker noise requires finite positive multiplicity");
        }
        // Cached id enters the physical drain for every family, including
        // MOS3/9; the OP display converts those families to model polarity.
        let (coefficient, current, af, ef) = if let Some(model) = &self.legacy_bsim_model {
            let denominator = model.flicker_noise_denominator(self.w, self.l).ok_or(
                "legacy BSIM flicker noise requires positive effective W, L and TOX with representable normalization",
            )?;
            let coefficient =
                scaled_exp_product(&[self.kf, self.multiplicity], &[denominator], 0.0);
            // B1cd/B2cd are net drain current, including the body diode.
            // The ngspice-46 noise routines accidentally use the state offset
            // as a number; use the current that the load routine stores there.
            let current = (self.id - self.polarity() * self.ibd) / self.multiplicity;
            (coefficient, current, self.af, 1.0)
        } else {
            if !self.cox.is_finite() || self.cox < 0.0 {
                return Err("MOS flicker noise requires finite nonnegative Cox");
            }
            let cox = if self.cox == 0.0 {
                // mos1noi.c and Xyce's MOS1/6 fallback: 100 nm oxide.
                3.9 * 8.854214871e-12 / 1e-7
            } else {
                self.cox
            };
            let leff = self.l - 2.0 * self.ld;
            let width = if self.level == 9 || (self.level == 3 && !xyce && self.nlev != 0) {
                self.w - 2.0 * self.mos3_width_narrow
            } else {
                self.w
            };
            if !leff.is_finite() || leff <= 0.0 || !width.is_finite() || width <= 0.0 {
                return Err("MOS flicker noise requires finite positive noise width and L-2*LD");
            }
            if fixed_current_law || self.nlev == 0 || self.nlev == 1 {
                let divisors = if fixed_current_law {
                    [width, leff, cox, cox]
                } else if self.nlev == 0 {
                    [leff, leff, cox, 1.0]
                } else {
                    [width, leff, cox, 1.0]
                };
                (
                    scaled_exp_product(&[self.kf, self.multiplicity], &divisors, 0.0),
                    (self.id - self.polarity() * self.ibd) / self.multiplicity,
                    self.af,
                    if fixed_current_law { 1.0 } else { self.ef },
                )
            } else {
                // NLEV 2/3 use gm², with AF on frequency. Form the product
                // from total gm to avoid rounding gm/M to zero prematurely.
                let gm = self.transconductance();
                if !gm.is_finite() {
                    return Err("MOS flicker noise transconductance must be finite");
                }
                if gm == 0.0 {
                    return Ok(None);
                }
                (
                    scaled_exp_product(
                        &[self.kf, gm, gm],
                        &[self.multiplicity, width, leff, cox],
                        0.0,
                    ),
                    1.0,
                    1.0,
                    self.af,
                )
            }
        };
        if !ef.is_finite() {
            return Err("MOS flicker noise requires finite EF");
        }
        if !current.is_finite() {
            return Err("MOS flicker noise drain current must be finite");
        }
        if !coefficient.is_finite() || coefficient <= 0.0 {
            return Err("MOS flicker noise coefficient is not representable");
        }
        // Both references use exp(AF*log(max(|Id|, N_MINLOG))). This
        // preserves the authored noise source at cutoff, including AF <= 0.
        Ok(Some((coefficient, current.abs().max(1e-38), af, ef)))
    }

    //=========================================================================
    // Gate-charge estimates for device inspection.
    //=========================================================================

    /// Estimate physical gate-to-source/drain/bulk charges in coulombs.
    ///
    /// Legacy BSIM returns its terminal-charge flows, including overlap.
    /// Classic MOS retains a region-based estimate using its channel onset
    /// and saturation voltage. Meyer capacitances are not an integrable
    /// terminal-charge model: transient simulation integrates the accepted
    /// capacitance/voltage history instead of differentiating this estimate.
    pub fn gate_charges(&self) -> (Value, Value, Value) {
        if let Some(charge) = self.legacy_gate_charge_at(self.vgs, self.vds, self.vbs) {
            let [qgs, qgd, qgb] = charge.charges;
            return (qgs, qgd, qgb);
        }
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

        let oxide_cap = self.oxide_capacitance_total();
        let (mode, von, vdsat) =
            self.classic_meyer_state(self.vgs, self.vds, self.vbs, self.phi.sqrt());
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
        let (qgs, qgd, qgb) = if mode > 0.0 {
            intrinsic(vgs, vds)
        } else {
            let (qgd, qgs, qgb) = intrinsic(vgd, -vds);
            (qgs, qgd, qgb)
        };
        (p * (qgs + qgs_ov), p * (qgd + qgd_ov), p * (qgb + qgb_ov))
    }

    /// Calculate W/L ratio
    pub(crate) fn wl_ratio(&self) -> Value {
        self.w / self.l
    }

    /// Beta = KP * W/L
    pub(crate) fn beta(&self) -> Value {
        self.kp * self.wl_ratio() * self.multiplicity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_noise_parameters_reject_active_invalid_controls() {
        use crate::config::SpiceDialect;
        use std::collections::HashMap;
        for level in [1, 2, 3, 6, 9] {
            for name in ["TNOIA", "NOIA", "GAMMA_NOISE", "GDSNOI"] {
                for value in [-1.0, Value::NAN, Value::INFINITY] {
                    let mut params =
                        HashMap::from([("LEVEL".into(), level as Value), ("NLEV".into(), 3.0)]);
                    params.insert(name.into(), value);
                    let mos = Mosfet::new_nmos("M1".into(), 1, 2, 0, 3).with_params(&params);
                    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
                        let result = mos.channel_noise_parameters(dialect);
                        let ignored =
                            name == "GDSNOI" && (level > 3 || dialect == SpiceDialect::Xyce);
                        assert_eq!(
                            result.is_ok(),
                            ignored,
                            "L{level} {name}={value} {dialect:?}: {result:?}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn mos_noise_parameters_retain_authored_exponents_and_reject_active_invalid_values() {
        use crate::config::SpiceDialect;
        use std::collections::HashMap;
        for level in [1, 2, 3, 4, 5, 6, 9] {
            for (name, values) in [
                ("KF", vec![-1.0, Value::NAN, Value::INFINITY]),
                ("AF", vec![-1.0, 0.0, Value::NAN, Value::INFINITY]),
                ("EF", vec![-1.0, 0.0, Value::NAN, Value::INFINITY]),
            ] {
                for value in values {
                    let mut params = HashMap::from([
                        ("LEVEL".into(), level as Value),
                        ("TOX".into(), 0.03),
                        ("KF".into(), 1e-20),
                        ("NLEV".into(), 0.0),
                    ]);
                    params.insert(name.into(), value);
                    let mos = Mosfet::new_nmos("M1".into(), 1, 2, 0, 0).with_params(&params);
                    let retained = match name {
                        "KF" => mos.kf,
                        "AF" => mos.af,
                        _ => mos.ef,
                    };
                    assert_eq!(retained.to_bits(), value.to_bits(), "L{level} {name}");
                    let result = mos.flicker_noise_source_terms(SpiceDialect::Ngspice);
                    if name == "KF"
                        || (!value.is_finite() && (name != "EF" || !matches!(level, 4 | 5 | 9)))
                    {
                        assert!(
                            result.unwrap_err().contains(name),
                            "L{level} {name}={value}"
                        );
                    } else {
                        assert!(result.is_ok(), "L{level} {name}={value}: {result:?}");
                    }
                }
            }
        }
    }

    #[test]
    fn classic_mos_invalid_noise_selector_is_not_truncated_or_defaulted() {
        for value in [-1.0, 0.5, 4.0, Value::NAN, Value::INFINITY] {
            let mos = Mosfet::new_nmos("M1".into(), 1, 2, 0, 0)
                .with_params(&std::collections::HashMap::from([("NLEV".into(), value)]));
            assert!(
                mos.flicker_noise_source_terms(crate::config::SpiceDialect::Ngspice)
                    .unwrap_err()
                    .contains("NLEV")
            );
        }
    }

    #[test]
    fn mos_flicker_normalization_preserves_representable_products() {
        use crate::config::SpiceDialect;
        for (nlev, kf, multiplicity, length, width, gm, expected) in [
            (0, 1e-300, 1.0, 1e-200, 1e-200, 1.0, 1e100),
            (1, 1e200, 1e200, 1e100, 1e100, 1.0, 1e200),
            (2, 1e200, 1e200, 1e-150, 1e-150, 1e-200, 1e-100),
            (3, 1e-200, 1e-200, 1e-200, 1e-200, 1e-200, 1.0),
        ] {
            let mut mos = Mosfet::new_nmos("M1".into(), 1, 2, 0, 0);
            mos.nlev = nlev;
            mos.kf = kf;
            mos.multiplicity = multiplicity;
            mos.l = length;
            mos.w = width;
            mos.cox = 1.0;
            mos.gm = gm;
            let (coefficient, _, _, _) = mos
                .flicker_noise_source_terms(SpiceDialect::Ngspice)
                .unwrap()
                .unwrap();
            assert!(
                (coefficient - expected).abs() < expected * 2e-15,
                "NLEV={nlev}: {coefficient:e} vs {expected:e}"
            );
        }
    }

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
    fn classic_gate_charge_estimates_preserve_physical_polarity_and_orientation() {
        for mut mos in [
            Mosfet::new_nmos("M1".into(), 1, 2, 3, 4),
            Mosfet::new_pmos("M1".into(), 1, 2, 3, 4),
        ] {
            let p = mos.polarity();
            mos.gamma = 0.4;
            mos.phi = 0.6;
            mos.cox = 1.0;
            mos.w = 1.0;
            mos.l = 1.0;
            mos.cgso = 0.1;
            mos.cgdo = 0.2;
            mos.cgbo = 0.3;
            for vto in [-0.3, 1.0] {
                mos.vto = p * vto;
                let gate = vto - 0.4 * 0.2 / (2.0 * 0.6_f64.sqrt()) + 0.4;
                for reverse in [false, true] {
                    let (vg, vd, vb) = if reverse {
                        (gate - 2.0, -2.0, -1.8)
                    } else {
                        (gate, 2.0, 0.2)
                    };
                    mos.vgs = p * vg;
                    mos.vds = p * vd;
                    mos.vbs = p * vb;
                    let q = 2.0 / 3.0 * 0.4;
                    let expected = (
                        p * (if reverse { 0.0 } else { q }) + 0.1 * mos.vgs,
                        p * (if reverse { q } else { 0.0 }) + 0.2 * (mos.vgs - mos.vds),
                        0.3 * (mos.vgs - mos.vbs),
                    );
                    assert_charges_close(mos.gate_charges(), expected, 1e-14, 1e-14);
                }
            }
        }
    }

    /// The intrinsic gate charge scales with `Leff = L - 2·LD`, not the drawn
    /// length — the same oxide capacitance the Meyer capacitances use and the
    /// same one ngspice builds as `oxideCapFactor · EffectiveLength · W · m`.
    /// A deck with a large lateral diffusion makes the two differ sharply:
    /// `general/mosamp.cir` draws L=12.7 µm with LD=2.4485 µm, so the drawn
    /// length overstates the charge by 1.63x.
    #[test]
    fn classic_gate_charges_scale_with_the_effective_channel_length() {
        for level in [1_i32, 2, 6] {
            let mut mos = Mosfet::new_nmos(format!("m{level}"), 1, 2, 3, 0);
            mos.level = level;
            mos.w = 100.0e-6;
            mos.l = 12.7e-6;
            mos.ld = 2.4485e-6;
            // Intrinsic charge is the subject of this fixture, so supply an
            // explicit oxide capacitance instead of relying on a MOS1
            // constructor default that Berkeley SPICE does not define.
            mos.cox = 3.9 * 8.854_214_871e-12 / 100.0e-9;
            mos.cgso = 0.0;
            mos.cgdo = 0.0;
            mos.cgbo = 0.0;
            // Saturation, so the intrinsic term is (2/3)·Cox_eff·Vgt.
            mos.vgs = 5.0;
            mos.vds = 5.0;
            mos.vbs = 0.0;

            let (qgs, _, _) = mos.gate_charges();
            let effective = mos.oxide_capacitance_total();
            let drawn = mos.cox * mos.w * mos.l;
            assert!(
                drawn > effective * 1.6,
                "the fixture must actually separate drawn from effective geometry"
            );

            let vgt = mos.vgs - mos.vth(mos.vbs);
            let expected = (2.0 / 3.0) * effective * vgt;
            assert!(
                (qgs - expected).abs() <= expected.abs() * 1e-12,
                "level {level}: qgs={qgs:e} expected {expected:e} \
                 (drawn geometry would give {:e})",
                (2.0 / 3.0) * drawn * vgt
            );
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
