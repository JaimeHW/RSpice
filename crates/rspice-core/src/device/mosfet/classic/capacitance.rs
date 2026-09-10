use super::*;

/// Independent gate-to-source/drain/bulk charge flows. Columns are the
/// physical Vgs, Vds and Vbs derivatives; intrinsic terminal charge is coupled.
#[derive(Clone, Copy, Debug)]
pub(crate) struct LegacyBsimGateCharge {
    pub(crate) charges: [Value; 3],
    pub(crate) derivatives: [[Value; 3]; 3],
}

impl Mosfet {
    pub(crate) fn legacy_gate_charge_at(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> Option<LegacyBsimGateCharge> {
        use crate::device::mosfet::dual::Dual3;
        self.legacy_bsim_model.as_ref()?;
        let Some(model) = self.legacy_bsim_sized.as_ref() else {
            return Some(LegacyBsimGateCharge {
                charges: [Value::NAN; 3],
                derivatives: [[Value::NAN; 3]; 3],
            });
        };
        let gate = Dual3::variable(vgs, 0);
        let drain = Dual3::variable(vds, 1);
        let bulk = Dual3::variable(vbs, 2);
        let p = self.polarity();
        let forward = p * vds >= 0.0;
        let [qg, qb, qd] = if forward {
            model.terminal_charges(p * gate, p * drain, p * bulk)
        } else {
            model.terminal_charges(p * (gate - drain), -p * drain, p * (bulk - drain))
        };
        let qs = -(qg + qb + qd);
        let (qs, qd) = if forward { (qs, qd) } else { (qd, qs) };
        let (cgs, cgd, cgb) = self.overlap_capacitances();
        let scale = p * self.multiplicity;
        let flows = [
            -scale * qs + cgs * gate,
            -scale * qd + cgd * (gate - drain),
            -scale * qb + cgb * (gate - bulk),
        ];
        Some(LegacyBsimGateCharge {
            charges: flows.map(|q| q.value),
            derivatives: flows.map(|q| q.derivative),
        })
    }

    /// Stamp a charge-conserving coupled companion. AC uses zero currents and
    /// zero bias with gain=omega; transient uses its integration gain and dQ/dt.
    pub(crate) fn stamp_legacy_gate_charge(
        &self,
        charge: &LegacyBsimGateCharge,
        gain: Value,
        currents: [Value; 3],
        bias: [Value; 3],
        stamper: &mut impl MatrixStamper,
    ) {
        let columns = [
            self.node_gate,
            self.node_drain,
            self.node_bulk,
            self.node_source,
        ];
        for (branch, negative) in [self.node_source, self.node_drain, self.node_bulk]
            .into_iter()
            .enumerate()
        {
            let c = charge.derivatives[branch];
            let row = [c[0], c[1], c[2], -(c[0] + c[1] + c[2])];
            for (column, derivative) in columns.into_iter().zip(row) {
                stamper.stamp(self.node_gate, column, gain * derivative);
                stamper.stamp(negative, column, -gain * derivative);
            }
            let ieq = gain * (c[0] * bias[0] + c[1] * bias[1] + c[2] * bias[2]) - currents[branch];
            stamper.stamp_rhs(self.node_gate, ieq);
            stamper.stamp_rhs(negative, -ieq);
        }
    }

    /// Calculate effective threshold voltage with body effect and fallback short-channel effects.
    ///
    /// For Level 1 and MOS6: standard body effect formula.
    /// For opt-in simplified fallback levels: approximate short-channel Vth roll-off.
    pub(in crate::device::mosfet::classic) fn vth(&self, vbs: Value) -> Value {
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
        let vth_base = if vbs_eff == 0.0 {
            vto_eff
        } else {
            vto_eff + self.gamma * (phi_vbs.sqrt() - self.phi.sqrt())
        };

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
    pub(in crate::device::mosfet::classic) fn level6_effective_length(&self) -> Value {
        let leff = self.l - 2.0 * self.ld;
        leff.max(1e-12)
    }

    #[inline]
    pub(in crate::device::mosfet::classic) fn classic_meyer_effective_length(&self) -> Value {
        match self.level {
            1 => self.l - 2.0 * self.ld,
            6 => self.level6_effective_length(),
            2 => self.level2_effective_length(),
            3 | 9 => self.mos3_effective_length(),
            _ => self.l,
        }
    }

    #[inline]
    pub(crate) fn oxide_capacitance_total(&self) -> Value {
        if let Some(model) = &self.legacy_bsim_sized {
            return model.oxide_capacitance() * self.multiplicity;
        }
        self.cox
            * self.classic_meyer_effective_width()
            * self.classic_meyer_effective_length()
            * self.multiplicity
    }

    #[inline]
    pub(in crate::device::mosfet::classic) fn classic_meyer_effective_width(&self) -> Value {
        if self.uses_mos3_core() {
            self.mos3_effective_width()
        } else {
            self.w
        }
    }

    #[inline]
    pub(in crate::device::mosfet::classic) fn meyer_intrinsic_capacitances(
        vgs: Value,
        vgd: Value,
        vgb: Value,
        von: Value,
        vdsat: Value,
        phi: Value,
        oxide_cap: Value,
    ) -> (Value, Value, Value) {
        let _ = vgb;
        // Berkeley/ngspice DEVqmeyer clamps VDSAT before both the weak-
        // inversion and above-threshold partitions. Xyce deliberately does
        // not; keep its law in `xyce_meyer_intrinsic_capacitances`.
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

    /// Xyce's classic MOS `qmeyer` charge partition used by native MOS3.
    ///
    /// The weak-inversion branch deliberately keeps `Cgd` at zero and does
    /// not partition it by `Vds/Vdsat`; that is the canonical Xyce MOS3
    /// equation.  Keep this separate from the legacy generic Meyer helper so
    /// the established Level-1/2 compatibility path is not changed without
    /// its own oracle qualification.
    pub(in crate::device::mosfet::classic) fn xyce_meyer_intrinsic_capacitances(
        vgs: Value,
        vgd: Value,
        _vgb: Value,
        von: Value,
        vdsat: Value,
        phi: Value,
        oxide_cap: Value,
    ) -> (Value, Value, Value) {
        let vgst = vgs - von;
        if vgst <= -phi {
            (0.0, 0.0, oxide_cap / 2.0)
        } else if vgst <= -phi / 2.0 {
            (0.0, 0.0, -vgst * oxide_cap / (2.0 * phi))
        } else if vgst <= 0.0 {
            (
                vgst * oxide_cap / (1.5 * phi) + oxide_cap / 3.0,
                0.0,
                -vgst * oxide_cap / (2.0 * phi),
            )
        } else {
            let vds = vgs - vgd;
            if vdsat <= vds {
                (oxide_cap / 3.0, 0.0, 0.0)
            } else {
                let vddif = 2.0 * vdsat - vds;
                let vddif1 = vdsat - vds;
                let vddif2 = vddif * vddif;
                (
                    oxide_cap * (1.0 - vddif1 * vddif1 / vddif2) / 3.0,
                    oxide_cap * (1.0 - vdsat * vdsat / vddif2) / 3.0,
                    0.0,
                )
            }
        }
    }

    pub(in crate::device::mosfet::classic) fn level6_meyer_state(
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

        let phi = if self.level == 1 {
            self.phi
        } else {
            self.phi.max(1e-12)
        };
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

    /// Onset in the effective source frame used by the MOS1 channel law.
    #[inline]
    pub(in crate::device::mosfet::classic) fn level1_onset_with_sqrt_phi(
        &self,
        vds: Value,
        vbs: Value,
        sqrt_phi: Value,
    ) -> Value {
        let p = self.polarity();
        let body = if p * vds >= 0.0 {
            p * vbs
        } else {
            p * vbs - p * vds
        };
        crate::device::semiconductor::mos1_threshold(
            p * self.vto,
            self.gamma,
            self.phi,
            sqrt_phi,
            body,
        )
        .0
    }

    /// Mode, onset and saturation voltage in the model's effective source frame.
    pub(in crate::device::mosfet::classic) fn classic_meyer_state(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
        sqrt_phi: Value,
    ) -> (Value, Value, Value) {
        let p = self.polarity();
        let mode = if p * vds >= 0.0 { 1.0 } else { -1.0 };
        if self.level == 6 {
            return self.level6_meyer_state(vgs, vds, vbs);
        }
        if self.level == 2 {
            let eval = self.level2_evaluate(vgs, vds, vbs);
            return (mode, eval.von, eval.vdsat);
        }
        if self.uses_mos3_core() {
            let state = self.mos3_state(vgs, vds, vbs);
            return (mode, p * state.von, p * state.vdsat);
        }
        let von = if self.level == 1 {
            self.level1_onset_with_sqrt_phi(vds, vbs, sqrt_phi)
        } else {
            self.vth(vbs)
        };
        let vg_active = if mode > 0.0 {
            p * vgs
        } else {
            p * vgs - p * vds
        };
        (mode, von, (vg_active - von).max(0.0))
    }

    pub(crate) fn transient_capacitance_halves_at(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, Value, Value) {
        if self.uses_legacy_bsim() {
            // Legacy BSIM uses terminal charge and a coupled Jacobian instead.
            return (0.0, 0.0, 0.0);
        }
        self.meyer_capacitance_halves_at(
            vgs,
            vds,
            vbs,
            self.oxide_capacitance_total(),
            self.phi.sqrt(),
        )
    }

    fn meyer_capacitance_halves_at(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
        oxide_cap: Value,
        sqrt_phi: Value,
    ) -> (Value, Value, Value) {
        let phi = if self.level == 1 {
            self.phi
        } else {
            self.phi.max(1e-12)
        };
        let p = self.polarity();
        let vgs_m = p * vgs;
        let vgd_m = vgs_m - p * vds;
        let vgb_m = vgs_m - p * vbs;
        let (mode, von, vdsat) = self.classic_meyer_state(vgs, vds, vbs, sqrt_phi);
        let partition = |gate, drain| {
            if self.body_junction_model == MosBodyJunctionModel::XyceClassicLinearizedReverse {
                Self::xyce_meyer_intrinsic_capacitances(
                    gate, drain, vgb_m, von, vdsat, phi, oxide_cap,
                )
            } else {
                Self::meyer_intrinsic_capacitances(gate, drain, vgb_m, von, vdsat, phi, oxide_cap)
            }
        };
        if mode > 0.0 {
            partition(vgs_m, vgd_m)
        } else {
            let (cgd, cgs, cgb) = partition(vgd_m, vgs_m);
            (cgs, cgd, cgb)
        }
    }

    /// Reuse setup-invariant geometry and surface potential for MOS1.
    pub(crate) fn transient_capacitance_halves_with_constants(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
        constants: &ClassicMosTransientConstants,
    ) -> (Value, Value, Value) {
        if self.level != 1 || self.legacy_bsim_sized.is_some() {
            return self.transient_capacitance_halves_at(vgs, vds, vbs);
        }
        self.meyer_capacitance_halves_at(
            vgs,
            vds,
            vbs,
            constants.oxide_capacitance_total,
            constants.sqrt_phi,
        )
    }

    #[inline]
    pub(crate) fn overlap_capacitances_with_constants(
        &self,
        constants: &ClassicMosTransientConstants,
    ) -> (Value, Value, Value) {
        constants.overlap_capacitances
    }

    /// Calculate overlap capacitances for AC analysis
    /// Returns (Cgs_overlap, Cgd_overlap, Cgb_overlap)
    pub(crate) fn overlap_capacitances(&self) -> (Value, Value, Value) {
        let (width, cgb_length) = self.legacy_bsim_model.as_ref().map_or_else(
            || {
                (
                    self.classic_meyer_effective_width(),
                    self.classic_meyer_effective_length(),
                )
            },
            |model| model.overlap_dimensions(self.w, self.l),
        );
        // Cgs_overlap = CGSO * W
        let cgs = self.cgso * width * self.multiplicity;
        // Cgd_overlap = CGDO * W
        let cgd = self.cgdo * width * self.multiplicity;
        let cgb = self.cgbo * cgb_length * self.multiplicity;

        (cgs, cgd, cgb)
    }

    /// Total AC Meyer capacitances (Cgs, Cgd, Cgb), including overlap,
    /// at the same evaluated bias used by the current linearization.
    pub(crate) fn ac_capacitances_at(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, Value, Value) {
        let (cgs_ov, cgd_ov, cgb_ov) = self.overlap_capacitances();
        let (cgs, cgd, cgb) = self.transient_capacitance_halves_at(vgs, vds, vbs);
        (2.0 * cgs + cgs_ov, 2.0 * cgd + cgd_ov, 2.0 * cgb + cgb_ov)
    }

    #[inline]
    pub(in crate::device::mosfet::classic) fn cached_eval_branch_voltages(
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

    /// Raw and limited branch voltages for an iterate whose classic-MOS
    /// nonlinear cache has already been verified by the transient engine.
    #[inline]
    pub(crate) fn verified_cached_transient_branch_voltages(
        &self,
    ) -> ((Value, Value, Value), (Value, Value, Value)) {
        debug_assert!(self.has_branch_history);
        debug_assert!(self.vgs.is_finite() && self.vds.is_finite() && self.vbs.is_finite());
        debug_assert!(
            self.eval_vgs.is_finite() && self.eval_vds.is_finite() && self.eval_vbs.is_finite()
        );
        (
            (self.vgs, self.vds, self.vbs),
            (self.eval_vgs, self.eval_vds, self.eval_vbs),
        )
    }

    /// The branch voltages of a `UIC` startup that authored an `IC=` vector.
    ///
    /// `mos1load.c:396-408` assigns `vds/vgs/vbs` from the vector outright and
    /// leaves the whole limiting block to the `else` arm, so the seeded bias
    /// must reach the charge state unlimited.  Limiting it against the device
    /// state primed from the ordinary solution — which under `UIC` is zero
    /// everywhere the deck did not name — folds the vector back toward that
    /// zero and defeats the parameter.
    #[inline]
    pub(crate) fn unlimited_branch_voltages_at(&self, voltages: &[Value]) -> (Value, Value, Value) {
        self.branch_voltages(voltages)
    }

    #[inline]
    pub(crate) fn eval_branch_voltages_at(&self, voltages: &[Value]) -> (Value, Value, Value) {
        let (vgs, vds, vbs) = self.branch_voltages(voltages);
        if self.has_branch_history
            && self.vgs == vgs
            && self.vds == vds
            && self.vbs == vbs
            && let Some(eval) = self.cached_eval_branch_voltages()
        {
            return eval;
        }

        self.limited_branch_voltages_for_eval(vgs, vds, vbs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn legacy_charge_mos(level: i32, p: Value, xpart: Value) -> Mosfet {
        let params = std::collections::HashMap::from([
            ("LEVEL".into(), level as Value),
            ("TOX".into(), 0.03),
            ("VFB".into(), -0.7),
            ("PHI".into(), 0.6),
            ("K1".into(), 0.5),
            ("K2".into(), 0.02),
            ("ETA0".into(), 0.02),
            ("ETAB".into(), 0.01),
            ("VBB".into(), -5.0),
            ("VDD".into(), 5.0),
            ("XPART".into(), xpart),
            ("CGSO".into(), 2e-10),
            ("CGDO".into(), 3e-10),
            ("CGBO".into(), 4e-10),
            ("DL".into(), 0.1),
            ("DW".into(), 0.2),
        ]);
        let mos = if p > 0.0 {
            Mosfet::new_nmos("m".into(), 1, 2, 3, 4)
        } else {
            Mosfet::new_pmos("m".into(), 1, 2, 3, 4)
        };
        mos.with_params(&params).with_instance_params(&[
            ("W".into(), 2e-6),
            ("L".into(), 1e-6),
            ("M".into(), 1.5),
            ("NF".into(), 2.0),
        ])
    }

    #[test]
    fn legacy_bsim_charge_jacobian_matches_finite_differences() {
        for level in [4, 5] {
            for p in [1.0, -1.0] {
                for xpart in [0.0, 1.0, 2.0] {
                    let mos = legacy_charge_mos(level, p, xpart);
                    for bias in [
                        [-2.0, 0.2, -0.3],
                        [0.1, 0.2, -0.3],
                        [1.5, 0.2, -0.3],
                        [1.5, 2.0, -0.3],
                        [1.5, -0.2, -0.3],
                        [1.5, 0.2, 0.2],
                        [12.0, 11.0, -12.0],
                    ] {
                        let bias = bias.map(|v| p * v);
                        let charge = mos
                            .legacy_gate_charge_at(bias[0], bias[1], bias[2])
                            .unwrap();
                        assert!(charge.charges.iter().all(|q| q.is_finite()));
                        for column in 0..3 {
                            let mut plus = bias;
                            let mut minus = bias;
                            plus[column] += 1e-6;
                            minus[column] -= 1e-6;
                            let qp = mos
                                .legacy_gate_charge_at(plus[0], plus[1], plus[2])
                                .unwrap();
                            let qm = mos
                                .legacy_gate_charge_at(minus[0], minus[1], minus[2])
                                .unwrap();
                            for row in 0..3 {
                                let finite_difference = (qp.charges[row] - qm.charges[row]) / 2e-6;
                                assert_close(
                                    &format!(
                                        "L{level} p={p} XPART={xpart} {bias:?} ({row},{column})"
                                    ),
                                    charge.derivatives[row][column],
                                    finite_difference,
                                    2e-6,
                                    2e-23,
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn legacy_bsim_charge_stamp_conserves_charge_and_reference_voltage() {
        #[derive(Default)]
        struct Stamp {
            matrix: [[Value; 4]; 4],
            rhs: [Value; 4],
        }
        impl MatrixStamper for Stamp {
            fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
                self.matrix[row - 1][col - 1] += value;
            }
            fn stamp_rhs(&mut self, row: NodeId, value: Value) {
                self.rhs[row - 1] += value;
            }
        }
        for level in [4, 5] {
            for p in [1.0, -1.0] {
                let mos = legacy_charge_mos(level, p, 0.0);
                let bias = [p * 1.5, p * 0.2, p * -0.3];
                let charge = mos
                    .legacy_gate_charge_at(bias[0], bias[1], bias[2])
                    .unwrap();
                let mut stamp = Stamp::default();
                mos.stamp_legacy_gate_charge(
                    &charge,
                    1.0,
                    [1e-15, -2e-15, 4e-15],
                    bias,
                    &mut stamp,
                );
                for row in stamp.matrix {
                    assert!(row.iter().sum::<Value>().abs() < 1e-29);
                }
                for col in 0..4 {
                    assert!(stamp.matrix.iter().map(|row| row[col]).sum::<Value>().abs() < 1e-29);
                }
                assert!(stamp.rhs.iter().sum::<Value>().abs() < 1e-29);
                assert!(
                    (stamp.matrix[0][1] - stamp.matrix[1][0]).abs() > 1e-17,
                    "BSIM charge must retain its nonreciprocal terms"
                );
                let mut symmetric = mos.clone();
                symmetric.cgdo = symmetric.cgso;
                let forward = symmetric
                    .legacy_gate_charge_at(bias[0], bias[1], bias[2])
                    .unwrap();
                let reverse = symmetric
                    .legacy_gate_charge_at(bias[0] - bias[1], -bias[1], bias[2] - bias[1])
                    .unwrap();
                for (a, b) in forward.charges.into_iter().zip([
                    reverse.charges[1],
                    reverse.charges[0],
                    reverse.charges[2],
                ]) {
                    assert_close("reverse charge", a, b, 1e-13, 1e-29);
                }
            }
        }
    }

    #[test]
    fn legacy_bsim_charge_matches_berkeley_evaluator_values() {
        // ngspice 46 b1eval.c/b2eval.c, called directly with this sized card.
        // Direct charge output avoids b1ld.c/b2ld.c reverse-mode AC state
        // storage and the BSIM1 parser's presence-only XPART flag handling.
        let cases: &[(i32, [Value; 3], [Value; 3])] = &[
            (
                4,
                [-2.0, 0.2, -0.3],
                [-2.16e-15, -3.564e-15, -7.633859999999999e-15],
            ),
            (
                4,
                [0.1, 0.2, -0.3],
                [
                    1.0799999999999998e-16,
                    -1.6200000000000014e-16,
                    2.7963976590954825e-15,
                ],
            ),
            (
                4,
                [1.5, 0.2, -0.3],
                [
                    4.564564363457661e-15,
                    4.835213440870462e-15,
                    4.898393237400814e-15,
                ],
            ),
            (
                4,
                [1.5, 2.0, -0.3],
                [
                    4.138710089273431e-15,
                    8.6914005951562e-16,
                    5.099746757162225e-15,
                ],
            ),
            (
                4,
                [1.5, -0.2, -0.3],
                [
                    5.0582017333184725e-15,
                    6.4100880648231046e-15,
                    4.59253121455456e-15,
                ],
            ),
            (5, [-2.0, 0.2, -0.3], [-2.16e-15, -3.564e-15, -7.42986e-15]),
            (
                5,
                [0.1, 0.2, -0.3],
                [
                    1.0799999999999998e-16,
                    -1.6200000000000014e-16,
                    2.9382368947364877e-15,
                ],
            ),
            (
                5,
                [1.5, 0.2, -0.3],
                [
                    4.616779740106061e-15,
                    4.879102857756852e-15,
                    4.466722057605473e-15,
                ],
            ),
            (
                5,
                [1.5, 2.0, -0.3],
                [
                    4.25506237727343e-15,
                    9.467082515156202e-16,
                    4.4096646856054735e-15,
                ],
            ),
            (
                5,
                [1.5, -0.2, -0.3],
                [
                    5.089308289413676e-15,
                    6.450908409423569e-15,
                    3.996070273337982e-15,
                ],
            ),
        ];
        for &(level, bias, expected) in cases {
            for p in [1.0, -1.0] {
                let mos = legacy_charge_mos(level, p, 0.0);
                let actual = mos
                    .legacy_gate_charge_at(p * bias[0], p * bias[1], p * bias[2])
                    .unwrap();
                for (a, q) in actual.charges.into_iter().zip(expected) {
                    assert_close("Berkeley charge", a, p * q, 2e-13, 2e-28);
                }
            }
        }
    }

    #[test]
    fn legacy_bsim_charge_uses_oxide_units_geometry_and_partition() {
        for level in [4, 5] {
            let mut mos = legacy_charge_mos(level, 1.0, 0.0);
            let cox = 3.453e-13 / (0.03 * 1e-4) * 1e4;
            assert_close("SI oxide density", mos.cox, cox, 1e-14, 0.0);
            assert_close(
                "oxide area",
                mos.oxide_capacitance_total(),
                cox * 1.8e-6 * 0.9e-6 * 3.0,
                1e-14,
                0.0,
            );
            let (cgs, cgd, cgb) = mos.overlap_capacitances();
            assert_close("G-S overlap", cgs, 2e-10 * 1.8e-6 * 3.0, 1e-14, 0.0);
            assert_close("G-D overlap", cgd, 3e-10 * 1.8e-6 * 3.0, 1e-14, 0.0);
            assert_close(
                "G-B overlap",
                cgb,
                4e-10 * if level == 4 { 1e-6 } else { 0.9e-6 } * 3.0,
                1e-14,
                0.0,
            );
            mos.cgso = 0.0;
            mos.cgdo = 0.0;
            mos.cgbo = 0.0;
            let sat = mos.legacy_gate_charge_at(2.0, 3.0, -0.3).unwrap();
            assert_close(
                "40/60 partition",
                sat.charges[1] / (sat.charges[0] + sat.charges[1]),
                0.4,
                1e-13,
                0.0,
            );
            let mut partitioned = legacy_charge_mos(level, 1.0, if level == 4 { 1.0 } else { 2.0 });
            partitioned.cgso = 0.0;
            partitioned.cgdo = 0.0;
            partitioned.cgbo = 0.0;
            let sat = partitioned.legacy_gate_charge_at(2.0, 3.0, -0.3).unwrap();
            assert_eq!(sat.charges[1], 0.0);
            if level == 5 {
                assert_eq!(sat.charges, [0.0; 3]);
            }
        }
    }

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
        mos.mos3_width_narrow = 0.4e-6;
        mos.mos3_width_adjust = 0.1e-6;

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
        let oxide_cap = mos.cox * mos.mos3_effective_width() * mos.mos3_effective_length();
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
        let weff = mos.mos3_effective_width();
        (
            2.0 * cgs_int + mos.cgso * weff,
            2.0 * cgd_int + mos.cgdo * weff,
            2.0 * cgb_int + mos.cgbo * leff,
        )
    }

    #[test]
    fn mos1_meyer_regions_use_effective_source_and_cached_threshold() {
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
            for vto in [-0.3, 1.0] {
                mos.vto = p * vto;
                let von = vto - 0.4 * 0.2 / (2.0 * 0.6_f64.sqrt());
                for xyce in [false, true] {
                    mos.body_junction_model = if xyce {
                        MosBodyJunctionModel::XyceClassicLinearizedReverse
                    } else {
                        MosBodyJunctionModel::NgspiceReverseClamp
                    };
                    let constants = mos.classic_transient_constants();
                    for (overdrive, drain, caps) in [
                        (-1.0, 0.1, (0.0, 0.0, 1.0)),
                        (-0.45, 0.1, (0.0, 0.0, 0.75)),
                        (
                            -0.15,
                            0.0,
                            if xyce {
                                (1.0 / 3.0, 0.0, 0.25)
                            } else {
                                (0.25, 0.25, 0.25)
                            },
                        ),
                        (0.4, 0.2, (16.0 / 27.0, 10.0 / 27.0, 0.0)),
                        (0.4, 2.0, (2.0 / 3.0, 0.0, 0.0)),
                    ] {
                        for reverse in [false, true] {
                            let (vg, vd, vb) = if reverse {
                                (von + overdrive - drain, -drain, 0.2 - drain)
                            } else {
                                (von + overdrive, drain, 0.2)
                            };
                            let (vg, vd, vb) = (p * vg, p * vd, p * vb);
                            let expected = if reverse && drain > 0.0 {
                                (caps.1, caps.0, caps.2)
                            } else {
                                caps
                            };
                            assert_caps_close(
                                mos.ac_capacitances_at(vg, vd, vb),
                                expected,
                                2e-14,
                                2e-15,
                            );
                            let cached = mos.transient_capacitance_halves_with_constants(
                                vg, vd, vb, &constants,
                            );
                            assert_eq!(cached, mos.transient_capacitance_halves_at(vg, vd, vb));
                            assert!(
                                (mos.model_space_onset_voltage(vg, vd, vb) - von).abs() < 1e-14
                            );
                        }
                    }
                }
            }
        }
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
    fn verified_cached_transient_voltages_match_updated_iterate_exactly() {
        let candidate = [1.3, 1.9, 0.2, -0.1];
        let mut mos = Mosfet::new_nmos("m1".to_string(), 1, 2, 3, 4);
        mos.update(&candidate);

        let (raw, evaluated) = mos.verified_cached_transient_branch_voltages();
        assert_eq!(raw, mos.branch_voltages(&candidate));
        assert_eq!(evaluated, mos.eval_branch_voltages_at(&candidate));
    }

    #[test]
    fn level3_meyer_capacitance_uses_mos3_effective_length() {
        let mos = mos3_capacitance_fixture();
        let leff = mos.l - 2.0 * mos.ld + mos.mos3_length_adjust;
        let weff = mos.w - 2.0 * mos.mos3_width_narrow + mos.mos3_width_adjust;

        assert_close(
            "classic Meyer Leff",
            mos.classic_meyer_effective_length(),
            leff,
            0.0,
            1.0e-30,
        );
        assert_close(
            "classic Meyer Weff",
            mos.classic_meyer_effective_width(),
            weff,
            0.0,
            1.0e-30,
        );
        assert_close(
            "oxide cap",
            mos.oxide_capacitance_total(),
            mos.cox * weff * leff,
            0.0,
            1.0e-30,
        );
        assert_close(
            "gate-source overlap",
            mos.overlap_capacitances().0,
            mos.cgso * weff,
            0.0,
            1.0e-30,
        );
        assert_close(
            "gate-drain overlap",
            mos.overlap_capacitances().1,
            mos.cgdo * weff,
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
    fn level3_transient_meyer_uses_xyce_weak_inversion_partition() {
        let mut mos = mos3_capacitance_fixture();
        mos.body_junction_model = MosBodyJunctionModel::XyceClassicLinearizedReverse;
        let vbs = -0.6;
        let von = mos.mos3_state(0.0, 0.2, vbs).von;
        let vgs = von - 0.05;
        let vds = 0.2;
        let oxide_cap = mos.oxide_capacitance_total();
        let expected = (
            (vgs - von) * oxide_cap / (1.5 * mos.phi) + oxide_cap / 3.0,
            0.0,
            -(vgs - von) * oxide_cap / (2.0 * mos.phi),
        );

        assert_caps_close(
            mos.transient_capacitance_halves_at(vgs, vds, vbs),
            expected,
            1.0e-12,
            1.0e-24,
        );
    }

    #[test]
    fn classic_meyer_keeps_ngspice_weak_inversion_partition_and_vdsat_floor() {
        let phi = 0.6;
        let oxide_cap = 3.0e-12;
        let von = 0.8;
        let vgs = 0.75;
        let vds = 0.01;
        let raw_vdsat = 0.0;
        let capgs_unpartitioned = (vgs - von) * oxide_cap / (1.5 * phi) + oxide_cap / 3.0;
        let vdsat = 0.025;
        let vddif = 2.0 * vdsat - vds;
        let expected = (
            capgs_unpartitioned * (1.0 - (vdsat - vds) * (vdsat - vds) / (vddif * vddif)),
            capgs_unpartitioned * (1.0 - vdsat * vdsat / (vddif * vddif)),
            -(vgs - von) * oxide_cap / (2.0 * phi),
        );

        assert_caps_close(
            Mosfet::meyer_intrinsic_capacitances(
                vgs,
                vgs - vds,
                vgs,
                von,
                raw_vdsat,
                phi,
                oxide_cap,
            ),
            expected,
            1.0e-12,
            1.0e-24,
        );
        let xyce = Mosfet::xyce_meyer_intrinsic_capacitances(
            vgs,
            vgs - vds,
            vgs,
            von,
            raw_vdsat,
            phi,
            oxide_cap,
        );
        assert_close("Xyce Cgs", xyce.0, capgs_unpartitioned, 1.0e-12, 1.0e-24);
        assert_eq!(xyce.1, 0.0);
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
        assert_caps_close(
            mos.ac_capacitances_at(mos.vgs, mos.vds, mos.vbs),
            expected,
            1.0e-12,
            1.0e-24,
        );
    }
}
