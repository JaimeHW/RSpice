use super::*;

/// Snapshot of a MOSFET's cached small-signal operating point, for
/// device operating-point reporting (the Spectre-style OP info table).
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct MosfetOpValues {
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

/// A model-selected flicker law whose coefficient need not fit in f64.
#[derive(Debug)]
#[doc(hidden)]
pub struct FlickerNoiseTerms {
    pub coefficient: Value,
    pub binary_scale: i32,
    pub current: Value,
    pub af: Value,
    pub ef: Value,
}

impl Mosfet {
    /// Cached operating-point values from the last accepted Newton solution.
    #[doc(hidden)]
    pub fn op_values(&self) -> MosfetOpValues {
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
    #[doc(hidden)]
    pub fn gate_charge_branch_voltages_at(&self, voltages: &[Value]) -> (Value, Value, Value) {
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
    #[doc(hidden)]
    pub fn channel_noise_conductance(
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
    /// `coefficient * 2^binary_scale * |current|^af / f^ef`. Each parallel instance contributes
    /// independently. Ngspice levels 1/2/3 use NLEV (default 2); native MOS6
    /// extends that law because ngspice supplies no MOS6 noise callback.
    /// Xyce levels 1/2/3/6 use current^AF divided by W*Leff*Cox²*f.
    /// MOS9 uses that current law with W-2*WD, without XL/XW mask shifts.
    /// Ngspice MOS3 also uses W-2*WD for its width-dependent NLEV laws.
    /// BSIM1/2 keep their own effective geometry and Cox units.
    #[doc(hidden)]
    pub fn flicker_noise_source_terms(
        &self,
        dialect: crate::config::SpiceDialect,
    ) -> Result<Option<FlickerNoiseTerms>, &'static str> {
        use crate::numerics::product_binary_normalization;
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
            let divisors = model
                .flicker_noise_divisors(self.w, self.l)
                .ok_or("legacy BSIM flicker noise requires positive effective W, L and TOX")?;
            let coefficient =
                product_binary_normalization(&[self.kf, self.multiplicity], &divisors);
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
                    product_binary_normalization(&[self.kf, self.multiplicity], &divisors),
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
                    product_binary_normalization(
                        &[self.kf, gm, gm],
                        &[self.multiplicity, width, leff, cox],
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
        let (coefficient, binary_scale) = coefficient;
        if !coefficient.is_finite() || coefficient <= 0.0 {
            return Err("MOS flicker noise coefficient is not representable");
        }
        // Both references use exp(AF*log(max(|Id|, N_MINLOG))). This
        // preserves the authored noise source at cutoff, including AF <= 0.
        Ok(Some(FlickerNoiseTerms {
            coefficient,
            binary_scale,
            current: current.abs().max(1e-38),
            af,
            ef,
        }))
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
            let terms = mos
                .flicker_noise_source_terms(SpiceDialect::Ngspice)
                .unwrap()
                .unwrap();
            let coefficient = libm::scalbn(terms.coefficient, terms.binary_scale);
            assert!(
                (coefficient - expected).abs() < expected * 2e-15,
                "NLEV={nlev}: {coefficient:e} vs {expected:e}"
            );
        }
    }
}
