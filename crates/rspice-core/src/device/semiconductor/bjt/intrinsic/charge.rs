//! Legacy and VBIC transport-charge linearization helpers.

use super::*;

impl Bjt {
    pub(in crate::device::semiconductor::bjt) fn legacy_transport_charge_state(
        &self,
        vbe_eff: Value,
        vbc_eff: Value,
    ) -> TransportChargeState {
        // Xyce N_DEV_BJT.C includes GMIN in iBE/iBC before transport and
        // stored charge. Ngspice bjtload.c applies it to leakage branches
        // instead, so it must not generate GP transport or diffusion charge.
        let gmin = if self.xyce_compatibility {
            self.nonlinear_branch_gmin()
        } else {
            0.0
        };
        let reverse_is = self.legacy_reverse_saturation_current();
        let (ifi, gfi) = self.legacy_junction_iv(LegacyCurrent::Forward, self.is, vbe_eff, self.nf);
        let (iri, gri) =
            self.legacy_junction_iv(LegacyCurrent::Reverse, reverse_is, vbc_eff, self.nr);
        let (ifi, gfi) = (ifi + gmin * vbe_eff, gfi + gmin);
        let (iri, gri) = (iri + gmin * vbc_eff, gri + gmin);

        let raw_q1_inv =
            1.0 - if self.var.is_finite() && self.var > 0.0 {
                vbe_eff / self.var
            } else {
                0.0
            } - if self.vaf.is_finite() && self.vaf > 0.0 {
                vbc_eff / self.vaf
            } else {
                0.0
            };
        let (q1_inv, dq1_inv_draw_q1_inv) = Self::smooth_positive_floor(raw_q1_inv, 1e-9);
        let q1 = 1.0 / q1_inv.max(1e-18);
        let dq1_dvbe_eff = if self.var.is_finite() && self.var > 0.0 {
            dq1_inv_draw_q1_inv / (self.var * q1_inv * q1_inv)
        } else {
            0.0
        };
        let dq1_dvbc_eff = if self.vaf.is_finite() && self.vaf > 0.0 {
            dq1_inv_draw_q1_inv / (self.vaf * q1_inv * q1_inv)
        } else {
            0.0
        };

        // Divide the already-scaled current by its scaled knee. Forming
        // 1/IKF first overflows for small, valid parallel instances.
        let normalized = |current: Value, knee: Value| {
            if knee > 0.0 { current / knee } else { 0.0 }
        };
        let (qb, dqb_dvbe_eff, dqb_dvbc_eff) = if self.ikf == 0.0 && self.ikr == 0.0 {
            (q1.max(1e-12), dq1_dvbe_eff, dq1_dvbc_eff)
        } else {
            let q2 = normalized(ifi, self.ikf) + normalized(iri, self.ikr);
            let dq2_dvbe_eff = normalized(gfi, self.ikf);
            let dq2_dvbc_eff = normalized(gri, self.ikr);
            let rolloff_arg = (1.0 + 4.0 * q2).max(0.0);
            let (rolloff_term, drolloff_dq2) = if self.nkf_given {
                let nkf = self.nkf.clamp(1e-12, 1.0);
                if rolloff_arg > 0.0 {
                    let term = rolloff_arg.powf(nkf).max(1e-18);
                    (term, 4.0 * nkf * term / rolloff_arg)
                } else {
                    (1.0, 0.0)
                }
            } else if rolloff_arg > 0.0 {
                let term = rolloff_arg.sqrt().max(1e-18);
                (term, 2.0 / term)
            } else {
                (1.0, 0.0)
            };
            (
                (0.5 * q1 * (1.0 + rolloff_term)).max(1e-12),
                0.5 * (1.0 + rolloff_term) * dq1_dvbe_eff + 0.5 * q1 * drolloff_dq2 * dq2_dvbe_eff,
                0.5 * (1.0 + rolloff_term) * dq1_dvbc_eff + 0.5 * q1 * drolloff_dq2 * dq2_dvbc_eff,
            )
        };

        let itzf = ifi / qb;
        let ditzf_dvbe_eff = gfi / qb - ifi * dqb_dvbe_eff / (qb * qb);
        let ditzf_dvbc_eff = -ifi * dqb_dvbc_eff / (qb * qb);
        let itzr = iri / qb;
        let ditzr_dvbe_eff = -iri * dqb_dvbe_eff / (qb * qb);
        let ditzr_dvbc_eff = gri / qb - iri * dqb_dvbc_eff / (qb * qb);

        TransportChargeState {
            q1,
            qb,
            ifi,
            iri,
            gfi,
            gri,
            dq1_dvbe_eff,
            dq1_dvbc_eff,
            itzf,
            itzr,
            dqb_dvbe_eff,
            dqb_dvbc_eff,
            ditzf_dvbe_eff,
            ditzf_dvbc_eff,
            ditzr_dvbe_eff,
            ditzr_dvbc_eff,
        }
    }

    pub(crate) fn legacy_transient_charge_state_with_vbx(
        &self,
        vbe: Value,
        vbc: Value,
        vbx: Value,
        vcs: Value,
    ) -> LegacyTransientChargeState {
        let p = self.polarity();
        let vbe_eff = p * vbe;
        let vbc_eff = p * vbc;
        let vbx_eff = p * vbx;
        let substrate_sign = self.substrate_topology.ngspice_sign();
        let substrate_polarity = p * substrate_sign;
        let vsub_eff = -substrate_polarity * vcs;
        let transport = self.legacy_transport_charge_state(vbe_eff, vbc_eff);

        let mut argtf = 0.0;
        let mut arg2 = 0.0;
        let mut arg3 = 0.0;
        if self.tf != 0.0 && vbe_eff > 0.0 && self.xtf != 0.0 {
            argtf = self.xtf;
            let mut ovtf = 0.0;
            if self.vtf > 0.0 {
                ovtf = 1.0 / (self.vtf * 1.44);
                argtf *= Self::limited_exp(vbc_eff * ovtf).0;
            }
            arg2 = argtf;
            if self.itf > 0.0 {
                let temp = self.legacy_transit_current_fraction(transport.ifi);
                argtf *= temp * temp;
                arg2 = argtf * (3.0 - temp - temp);
            }
            arg3 = transport.ifi * argtf * ovtf;
        }

        let qb = transport.qb.max(1e-18);
        let (qbe_diffusion_current, gbe_dynamic, geqcb_dynamic) = if self.tf != 0.0 && vbe_eff > 0.0
        {
            let qbe_diffusion_current = transport.ifi * (1.0 + argtf) / qb;
            let gbe_dynamic = (transport.gfi * (1.0 + arg2)
                - qbe_diffusion_current * transport.dqb_dvbe_eff)
                / qb;
            let geqcb_dynamic = (arg3 - qbe_diffusion_current * transport.dqb_dvbc_eff) / qb;
            (qbe_diffusion_current, gbe_dynamic, geqcb_dynamic)
        } else if self.tf != 0.0 {
            (transport.ifi, transport.gfi, 0.0)
        } else {
            (0.0, 0.0, 0.0)
        };

        let (qbe_dep_norm, capbe_dep) =
            self.vbic_depletion_charge_and_derivative(vbe_eff, self.vje, self.mje, self.fc, 0.0);
        let (qbc_dep_norm, capbc_dep) =
            self.vbic_depletion_charge_and_derivative(vbc_eff, self.vjc, self.mjc, self.fc, 0.0);
        let (qbx_dep_norm, capbx_dep) =
            self.vbic_depletion_charge_and_derivative(vbx_eff, self.vjc, self.mjc, self.fc, 0.0);
        let (qsub_norm, capsub_dep) =
            self.vbic_depletion_charge_and_derivative(vsub_eff, self.ps, self.ms, 0.0, 0.0);
        let cjc_internal = self.cjc * self.xcjc;
        let cjc_external = self.cjc - cjc_internal;

        LegacyTransientChargeState {
            qbe: p * (self.tf * qbe_diffusion_current + self.cje * qbe_dep_norm) + self.cbeo * vbe,
            capbe: self.tf * gbe_dynamic + self.cje * capbe_dep + self.cbeo,
            capbe_vbc: self.tf * geqcb_dynamic,
            qbc: p * (self.tr * transport.iri + cjc_internal * qbc_dep_norm) + self.cbco * vbc,
            capbc: self.tr * transport.gri + cjc_internal * capbc_dep + self.cbco,
            qbx: p * (cjc_external * qbx_dep_norm),
            capbx: cjc_external * capbx_dep,
            qcs: -substrate_polarity * (self.cjcp * qsub_norm),
            capcs: self.cjcp * capsub_dep,
        }
    }

    /// I / (I + ITF * AREA * M), with I already in instance current units.
    /// Normalize the sum instead of imposing an absolute current floor.
    fn legacy_transit_current_fraction(&self, current: Value) -> Value {
        let scale = self.instance_scale();
        let knee = self.itf * scale;
        if knee.is_normal() {
            let normalization = current.max(knee);
            let normalized_current = current / normalization;
            normalized_current / (normalized_current + knee / normalization)
        } else {
            // Avoid overflow/underflow in the scaled knee itself. When the
            // product overflows both factors exceed one, so these divisions
            // only decrease the finite current and cannot overflow early.
            let ratio = (current / self.itf) / scale;
            if ratio > 1.0 {
                1.0 / (1.0 + 1.0 / ratio)
            } else {
                ratio / (1.0 + ratio)
            }
        }
    }

    pub(in crate::device::semiconductor::bjt) fn vbic_transport_charge_state(
        &self,
        vbe_eff: Value,
        vbc_eff: Value,
    ) -> TransportChargeState {
        let (ifi, gfi) =
            self.vbic_diode_iv(self.is, vbe_eff, self.nf, self.vbic_junction_limits.ifi);
        let (iri, gri) = self.vbic_diode_iv(
            self.is * self.isrr.max(0.0),
            vbc_eff,
            self.nr,
            self.vbic_junction_limits.iri,
        );

        let (q1, dq1_dvbe_eff, dq1_dvbc_eff, _) = self.vbic_low_injection_charge(vbe_eff, vbc_eff);
        // Divide the already-scaled current by its scaled knee. Forming
        // 1/IKF first overflows for small, valid parallel instances.
        let normalized = |current: Value, knee: Value| {
            if knee > 0.0 { current / knee } else { 0.0 }
        };
        let q2 = normalized(ifi, self.ikf) + normalized(iri, self.ikr);
        let (qb, dqb_dq1, dqb_dq2, _) = self.vbic_base_charge(q1, q2);
        let dqb_dvbe_eff = dqb_dq1 * dq1_dvbe_eff + dqb_dq2 * normalized(gfi, self.ikf);
        let dqb_dvbc_eff = dqb_dq1 * dq1_dvbc_eff + dqb_dq2 * normalized(gri, self.ikr);

        let itzf = ifi / qb;
        let ditzf_dvbe_eff = gfi / qb - ifi * dqb_dvbe_eff / (qb * qb);
        let ditzf_dvbc_eff = -ifi * dqb_dvbc_eff / (qb * qb);
        let itzr = iri / qb;
        let ditzr_dvbe_eff = -iri * dqb_dvbe_eff / (qb * qb);
        let ditzr_dvbc_eff = gri / qb - iri * dqb_dvbc_eff / (qb * qb);

        TransportChargeState {
            q1,
            qb,
            ifi,
            iri,
            gfi,
            gri,
            dq1_dvbe_eff,
            dq1_dvbc_eff,
            itzf,
            itzr,
            dqb_dvbe_eff,
            dqb_dvbc_eff,
            ditzf_dvbe_eff,
            ditzf_dvbc_eff,
            ditzr_dvbe_eff,
            ditzr_dvbc_eff,
        }
    }

    /// Low-injection charge and its voltage partials, plus the local-temperature
    /// partial due only to the Early voltages (all other mappings held fixed).
    fn vbic_low_injection_charge(
        &self,
        vbe_eff: Value,
        vbc_eff: Value,
    ) -> (Value, Value, Value, Value) {
        let (qdbe, dqdbe_dvbe_eff) = self
            .vbic_depletion_charge_and_derivative(vbe_eff, self.vje, self.mje, self.fc, self.aje);
        let (qdbc, dqdbc_dvbc_eff) = self
            .vbic_depletion_charge_and_derivative(vbc_eff, self.vjc, self.mjc, self.fc, self.ajc);

        let (vaf, var) = self.vbic_early_voltages();
        let q1z =
            1.0 + if var.is_finite() && var > 0.0 {
                qdbe / var
            } else {
                0.0
            } + if vaf.is_finite() && vaf > 0.0 {
                qdbc / vaf
            } else {
                0.0
            };
        let q1_shift = q1z - 1e-4;
        let q1_sqrt = (q1_shift * q1_shift + 1e-8).sqrt();
        let q1 = 0.5 * (q1_sqrt + q1_shift) + 1e-4;
        let dq1_dq1z = 0.5 * (q1_shift / q1_sqrt + 1.0);
        let dq1_dvbe_eff = dq1_dq1z
            * if var.is_finite() && var > 0.0 {
                dqdbe_dvbe_eff / var
            } else {
                0.0
            };
        let dq1_dvbc_eff = dq1_dq1z
            * if vaf.is_finite() && vaf > 0.0 {
                dqdbc_dvbc_eff / vaf
            } else {
                0.0
            };

        let early_slope = |charge: Value, mapped: Value, nominal: Value, tc: Value| {
            if self.vbic_13 && tc != 0.0 && mapped.is_finite() && mapped > 0.0 {
                -(charge / mapped) * (nominal * tc / mapped)
            } else {
                0.0
            }
        };
        let dq1_dt = dq1_dq1z
            * (early_slope(qdbe, var, self.var, self.tcver)
                + early_slope(qdbc, vaf, self.vaf, self.tcvef));
        (q1, dq1_dvbe_eff, dq1_dvbc_eff, dq1_dt)
    }

    /// Returns qb, its q1/q2 partials, and qb - q1*dqb/dq1. The final
    /// expression is evaluated without cancellation for transit-time charge.
    fn vbic_base_charge(&self, q1: Value, q2: Value) -> (Value, Value, Value, Value) {
        let nkf = self.nkf.max(1e-12);
        if self.qbm < 0.5 {
            let inv_nkf = 1.0 / nkf;
            // q1 is positive by construction (at least 1e-4).
            let xvar3 = q1.powf(inv_nkf);
            let xvar1 = xvar3 + 4.0 * q2;
            let (xvar4, dxvar4) = self.vbic_high_injection_power(xvar1, nkf);
            (
                0.5 * (q1 + xvar4),
                0.5 * (1.0 + dxvar4 * inv_nkf * xvar3 / q1),
                2.0 * dxvar4,
                if dxvar4 == 0.0 {
                    0.5 * xvar4
                } else {
                    2.0 * q2 * (xvar4 / xvar1)
                },
            )
        } else {
            let xvar1 = 1.0 + 4.0 * q2;
            let (xvar2, dxvar2) = self.vbic_high_injection_power(xvar1, nkf);
            (
                0.5 * q1 * (1.0 + xvar2),
                0.5 * (1.0 + xvar2),
                2.0 * q1 * dxvar2,
                0.0,
            )
        }
    }

    pub(in crate::device::semiconductor::bjt) fn vbic_early_thermal_derivatives(
        &self,
        vbe_eff: Value,
        vbc_eff: Value,
        transport: TransportChargeState,
        temperature_slope: Value,
    ) -> VbicEarlyThermalDerivatives {
        if !self.vbic_13 || (self.tcvef == 0.0 && self.tcver == 0.0) || temperature_slope == 0.0 {
            return VbicEarlyThermalDerivatives::default();
        }
        let dq1_dt = self.vbic_low_injection_charge(vbe_eff, vbc_eff).3 * temperature_slope;
        let q2 = if self.ikf > 0.0 {
            (1.0 / self.ikf) * transport.ifi
        } else {
            0.0
        } + if self.ikr > 0.0 {
            (1.0 / self.ikr) * transport.iri
        } else {
            0.0
        };
        let (_, dqb_dq1, _, tail) = self.vbic_base_charge(transport.q1, q2);
        let dqb_dt = dqb_dq1 * dq1_dt;
        VbicEarlyThermalDerivatives {
            qb: dqb_dt,
            itzf: -transport.itzf * (dqb_dt / transport.qb),
            itzr: -transport.itzr * (dqb_dt / transport.qb),
            forward_charge: transport.ifi
                * ((self.qtf * tail - dqb_dq1) / transport.qb)
                * (dq1_dt / transport.qb),
        }
    }

    pub(in crate::device::semiconductor::bjt) fn transport_charge_state(
        &self,
        vbe_eff: Value,
        vbc_eff: Value,
    ) -> TransportChargeState {
        match self.charge_model {
            BjtChargeModel::LegacyGummelPoon => {
                self.legacy_transport_charge_state(vbe_eff, vbc_eff)
            }
            BjtChargeModel::Vbic => self.vbic_transport_charge_state(vbe_eff, vbc_eff),
        }
    }

    pub(in crate::device::semiconductor::bjt) fn base_collector_current_state(
        &self,
        transport: TransportChargeState,
        vbc_eff: Value,
    ) -> BaseCollectorCurrentState {
        let ((ibci, gbci), (ibcn, gbcn)) = if self.charge_model == BjtChargeModel::LegacyGummelPoon
        {
            (
                self.legacy_junction_iv(
                    LegacyCurrent::CollectorIdealLeakage,
                    self.ibci,
                    vbc_eff,
                    self.nci,
                ),
                self.legacy_junction_iv(
                    LegacyCurrent::CollectorLeakage,
                    self.ibcn,
                    vbc_eff,
                    self.ncn,
                ),
            )
        } else {
            (
                self.vbic_diode_iv(self.ibci, vbc_eff, self.nci, self.vbic_junction_limits.ibci),
                self.vbic_diode_iv(self.ibcn, vbc_eff, self.ncn, self.vbic_junction_limits.ibcn),
            )
        };
        let ibcj = ibci + ibcn;
        let dibcj_dvbc_eff = gbci + gbcn;

        if self.avc1 <= 0.0 {
            return BaseCollectorCurrentState {
                ibc: ibcj,
                dibc_dvbe_eff: 0.0,
                dibc_dvbc_eff: dibcj_dvbc_eff,
            };
        }

        let (avalf, davalf_dvbc_eff) = if self.vbic_13 {
            self.vbic13_avalanche_factor(vbc_eff, self.vjc, self.mjc, self.avc1, self.avc2)
        } else {
            let vl_arg = self.vjc - vbc_eff;
            let vl_sqrt = (vl_arg * vl_arg + 0.01).sqrt().max(1e-18);
            let vl = 0.5 * (vl_sqrt + vl_arg);
            let dvl_dvbc_eff = 0.5 * (-vl_arg / vl_sqrt - 1.0);

            let power = self.mjc - 1.0;
            let vl_safe = vl.max(1e-18);
            let vl_power = vl_safe.powf(power);
            let d_vl_power_dvbc_eff = power * vl_safe.powf(power - 1.0) * dvl_dvbc_eff;

            let avalanche_arg = -self.avc2.max(0.0) * vl_power;
            let (avalanche_exp, d_avalanche_exp_darg) = Self::limited_exp(avalanche_arg);
            let d_avalanche_arg_dvbc_eff = -self.avc2.max(0.0) * d_vl_power_dvbc_eff;
            let avalf = self.avc1 * vl * avalanche_exp;
            let davalf_dvbc_eff = self.avc1
                * (dvl_dvbc_eff * avalanche_exp
                    + vl * d_avalanche_exp_darg * d_avalanche_arg_dvbc_eff);
            (avalf, davalf_dvbc_eff)
        };

        let transport_minus_ibcj = transport.itzf - transport.itzr - ibcj;
        let d_transport_minus_ibcj_dvbe_eff = transport.ditzf_dvbe_eff - transport.ditzr_dvbe_eff;
        let d_transport_minus_ibcj_dvbc_eff =
            transport.ditzf_dvbc_eff - transport.ditzr_dvbc_eff - dibcj_dvbc_eff;

        let igc = transport_minus_ibcj * avalf;
        let digc_dvbe_eff = d_transport_minus_ibcj_dvbe_eff * avalf;
        let digc_dvbc_eff =
            d_transport_minus_ibcj_dvbc_eff * avalf + transport_minus_ibcj * davalf_dvbc_eff;

        BaseCollectorCurrentState {
            ibc: ibcj - igc,
            dibc_dvbe_eff: -digc_dvbe_eff,
            dibc_dvbc_eff: dibcj_dvbc_eff - digc_dvbc_eff,
        }
    }

    #[inline]
    fn vbic13_reverse_be_exp_lina(value: Value, max_value: Value, slope: Value) -> (Value, Value) {
        if !value.is_finite() || !max_value.is_finite() || !slope.is_finite() || slope <= 0.0 {
            return (0.0, 0.0);
        }

        if value < max_value {
            let arg = (value * slope).clamp(-80.0, 80.0);
            let exp_value = arg.exp();
            return (exp_value, slope * exp_value);
        }

        let limit_arg = (max_value * slope).clamp(-80.0, 80.0);
        let limit_exp = limit_arg.exp();
        let continuation = 1.0 + (value - max_value) * slope;
        (limit_exp * continuation, limit_exp * slope)
    }

    fn vbic13_reverse_be_breakdown_current(&self, vbe_eff: Value) -> (Value, Value) {
        if self.charge_model != BjtChargeModel::Vbic
            || self.vbbe_nominal <= 0.0
            || self.ibbe <= 0.0
            || self.nbbe <= 0.0
            || self.vt <= 0.0
        {
            return (0.0, 0.0);
        }

        let denom = self.nbbe * self.vt.max(1e-18);
        let afac = 1.0 / denom;
        let bias = -self.vbbe - vbe_eff;
        if self.vbic_13 {
            let (current, conductance) =
                Self::vbic_scaled_exp_lina(self.ibbe, bias, denom, self.vbic_junction_limits.ibbe);
            return (self.ibbe * self.ebbe - current, conductance);
        }
        let model_ibbe = self.ibbe_nominal.max(1e-300);
        let max_value = denom * (self.ebbe.max(0.0) + 1.0 / model_ibbe).ln();
        let (expx, dexpx_dbias) = Self::vbic13_reverse_be_exp_lina(bias, max_value, afac);
        (-self.ibbe * (expx - self.ebbe), self.ibbe * dexpx_dbias)
    }

    pub(in crate::device::semiconductor::bjt) fn linearize_currents_with_branches(
        &self,
        vbe: Value,
        vbex: Value,
        vbc: Value,
    ) -> (BjtLinearization, BjtIntrinsicBranches) {
        let p = self.polarity();
        let vbe_eff = p * vbe;
        let vbex_eff = p * vbex;
        let vbc_eff = p * vbc;
        let transport = self.transport_charge_state(vbe_eff, vbc_eff);
        let bc = self.base_collector_current_state(transport, vbc_eff);

        // Numerical junction parallels participate in continuation and in
        // the same branch currents and derivatives used for Newton loads.
        let gmin = self.nonlinear_branch_gmin();
        let (ibe_breakdown, dibe_breakdown_dvbe) =
            self.vbic13_reverse_be_breakdown_current(vbe_eff);
        let wbe = if self.charge_model == BjtChargeModel::Vbic {
            self.wbe
        } else {
            1.0
        };
        // Legacy GP defines the ideal base current from the same forward
        // junction current used by transport (`I_BE / BF`).  Reusing the
        // transport branch preserves the model's NF, temperature scaling,
        // and junction GMIN semantics.  `IBEI` is a VBIC base-current
        // parameter; using it as the legacy ideal branch would silently
        // change the exponential whenever NF != 1 (and diverges from both
        // Xyce's BJTload and ngspice's bjtload).
        let legacy_model = self.charge_model == BjtChargeModel::LegacyGummelPoon;
        let (ibe_normal, dibe_normal_dvbe) = if legacy_model {
            let ideal_scale = 1.0 / self.bf.max(1e-18);
            let (iben, gben) =
                self.legacy_junction_iv(LegacyCurrent::BaseLeakage, self.iben, vbe_eff, self.nen);
            (
                transport.ifi * ideal_scale + iben,
                transport.gfi * ideal_scale + gben,
            )
        } else {
            let (ibei, gbei) =
                self.vbic_diode_iv(self.ibei, vbe_eff, self.nei, self.vbic_junction_limits.ibei);
            let (iben, gben) =
                self.vbic_diode_iv(self.iben, vbe_eff, self.nen, self.vbic_junction_limits.iben);
            (ibei + iben, gbei + gben)
        };
        let (ibex_normal, dibex_normal_dvbex) = if self.charge_model == BjtChargeModel::Vbic {
            let (ibei, gbei) = self.vbic_diode_iv(
                self.ibei,
                vbex_eff,
                self.nei,
                self.vbic_junction_limits.ibei,
            );
            let (iben, gben) = self.vbic_diode_iv(
                self.iben,
                vbex_eff,
                self.nen,
                self.vbic_junction_limits.iben,
            );
            ((1.0 - wbe) * (ibei + iben), (1.0 - wbe) * (gbei + gben))
        } else {
            (0.0, 0.0)
        };
        let ibe_intrinsic_breakdown = wbe * ibe_breakdown;
        let dibe_intrinsic_breakdown_dvbe = wbe * dibe_breakdown_dvbe;
        let ibex_breakdown = (1.0 - wbe) * ibe_breakdown;
        let dibex_breakdown_dvbe = (1.0 - wbe) * dibe_breakdown_dvbe;
        let legacy_reverse_base_scale = if legacy_model {
            1.0 / self.br.max(1e-18)
        } else {
            0.0
        };
        // Xyce GP already carries GMIN through transport. Ngspice GP
        // adds it directly to the BE/BC leakage branches, without BF/BR
        // division; VBIC also has direct junction parallels.
        let direct_gmin = if legacy_model && self.xyce_compatibility {
            0.0
        } else {
            gmin
        };
        let ib_be = wbe * ibe_normal + ibe_intrinsic_breakdown + direct_gmin * vbe_eff;
        let dibe_dvbe = wbe * dibe_normal_dvbe + dibe_intrinsic_breakdown_dvbe + direct_gmin;
        let reverse_base_current = legacy_reverse_base_scale * transport.iri;
        let reverse_base_dvbc = legacy_reverse_base_scale * transport.gri;
        let ibc = bc.ibc + reverse_base_current + direct_gmin * vbc_eff;
        let dibc_dvbe = bc.dibc_dvbe_eff;
        let dibc_dvbc = bc.dibc_dvbc_eff + reverse_base_dvbc + direct_gmin;
        let iciei = transport.itzf - transport.itzr;
        let diciei_dvbe = transport.ditzf_dvbe_eff - transport.ditzr_dvbe_eff;
        let diciei_dvbc = transport.ditzf_dvbc_eff - transport.ditzr_dvbc_eff;
        let ibe_branch = Self::branch_from_vbe_vbc(p * ib_be, dibe_dvbe, 0.0);
        let mut ibex_branch =
            Self::branch_from_vbe_vbc(p * ibex_breakdown, dibex_breakdown_dvbe, 0.0);
        if self.charge_model == BjtChargeModel::Vbic {
            let ibex_by_vbex = ibex_normal + gmin * vbex_eff;
            let dibex_by_vbex = dibex_normal_dvbex + gmin;
            ibex_branch.current += p * ibex_by_vbex;
            ibex_branch.d_internal[IDX_VBX] += dibex_by_vbex;
            ibex_branch.d_internal[IDX_VEI] -= dibex_by_vbex;
        }
        let ibc_branch = Self::branch_from_vbe_vbc(p * ibc, bc.dibc_dvbe_eff, dibc_dvbc);
        let iciei_branch = Self::branch_from_vbe_vbc(p * iciei, diciei_dvbe, diciei_dvbc);
        let linearized = BjtLinearization {
            // The intrinsic collector terminal sees both the transport branch
            // (collector to emitter) and the opposing B-C junction branch.
            ic: p * (iciei - ibc),
            ib: p * (ib_be + ibc),
            dic_dvbe: diciei_dvbe - dibc_dvbe,
            dic_dvbc: diciei_dvbc - dibc_dvbc,
            dic_dvrth: 0.0,
            dib_dvbe: dibe_dvbe + dibc_dvbe,
            dib_dvbc: dibc_dvbc,
            dib_dvrth: 0.0,
            qb: transport.qb,
            dqb_dvbe: p * transport.dqb_dvbe_eff,
            dqb_dvbc: p * transport.dqb_dvbc_eff,
            dqb_dvrth: 0.0,
        };

        (
            linearized,
            BjtIntrinsicBranches {
                ibe: ibe_branch,
                ibex: ibex_branch,
                ibc: ibc_branch,
                iciei: iciei_branch,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn ngspice_gp_gmin_does_not_change_transport_or_stored_charge() {
        let mut model = Bjt::new_npn("q".into(), 1, 2, 0).with_params(&HashMap::from([
            ("IS".into(), 1e-14),
            ("TF".into(), 1e-9),
            ("TR".into(), 2e-9),
            ("IKF".into(), 1e-3),
            ("IKR".into(), 2e-3),
            ("VAF".into(), 40.0),
            ("VAR".into(), 10.0),
            ("XTF".into(), 3.0),
            ("ITF".into(), 1e-4),
        ]));
        model.set_junction_gmin(0.0);
        let mut conditioned = model.clone();
        conditioned.set_junction_gmin(1e-3);
        for (vbe, vbc) in [(0.1, -0.9), (0.7, -0.3), (-0.2, 0.6)] {
            let a = model.legacy_transport_charge_state(vbe, vbc);
            let b = conditioned.legacy_transport_charge_state(vbe, vbc);
            for (a, b) in [
                (a.qb, b.qb),
                (a.itzf, b.itzf),
                (a.itzr, b.itzr),
                (a.ditzf_dvbe_eff, b.ditzf_dvbe_eff),
                (a.ditzr_dvbc_eff, b.ditzr_dvbc_eff),
            ] {
                assert_eq!(a, b);
            }
            let a = model.legacy_transient_charge_state_with_vbx(vbe, vbc, vbc, 0.0);
            let b = conditioned.legacy_transient_charge_state_with_vbx(vbe, vbc, vbc, 0.0);
            for (a, b) in [
                (a.qbe, b.qbe),
                (a.qbc, b.qbc),
                (a.capbe, b.capbe),
                (a.capbe_vbc, b.capbe_vbc),
                (a.capbc, b.capbc),
            ] {
                assert_eq!(a, b);
            }
        }
    }

    #[test]
    fn legacy_itf_charge_and_derivatives_obey_instance_scaling() {
        for (isat, knee, scales) in [
            (1e-16, 1e-4, &[1e-200, 1e-30, 1e-12, 0.1, 3.0, 1e12][..]),
            (1e-26, 1e-26, &[1e-200, 1e-30, 0.1, 3.0, 1e12][..]),
        ] {
            for p in [1.0, -1.0] {
                let mut unit = if p > 0.0 {
                    Bjt::new_npn("q".into(), 1, 2, 0)
                } else {
                    Bjt::new_pnp("q".into(), 1, 2, 0)
                }
                .with_params(&HashMap::from([
                    ("LEVEL".into(), 1.0),
                    ("IS".into(), isat),
                    ("TF".into(), 1e-9),
                    ("XTF".into(), 3.0),
                    ("VTF".into(), 10.0),
                    ("ITF".into(), knee),
                    ("VAF".into(), 40.0),
                    ("VAR".into(), 10.0),
                ]));
                unit.set_junction_gmin(0.0);
                for &scale in scales {
                    for parameter in ["M", "AREA"] {
                        let scaled = unit
                            .clone()
                            .with_instance_params(&[(parameter.into(), scale)]);
                        for (vbe, vbc) in [(0.03, -0.1), (0.7, -4.3), (0.7, 0.2)] {
                            let reference = unit.legacy_transient_charge_state_with_vbx(
                                p * vbe,
                                p * vbc,
                                0.0,
                                0.0,
                            );
                            let charge = scaled.legacy_transient_charge_state_with_vbx(
                                p * vbe,
                                p * vbc,
                                0.0,
                                0.0,
                            );
                            for (actual, expected) in [
                                (charge.qbe, reference.qbe),
                                (charge.capbe, reference.capbe),
                                (charge.capbe_vbc, reference.capbe_vbc),
                            ] {
                                assert!(
                                    (actual / scale - expected).abs() < 1e-12 * expected.abs(),
                                    "IS={isat} {parameter}={scale} p={p} bias=({vbe},{vbc}): {actual:e} != {expected:e} * scale"
                                );
                            }
                            let h = 1e-7;
                            for (column, derivative) in
                                [charge.capbe, charge.capbe_vbc].into_iter().enumerate()
                            {
                                let mut hi = [p * vbe, p * vbc];
                                let mut lo = hi;
                                hi[column] += h;
                                lo[column] -= h;
                                let numeric = (scaled
                                    .legacy_transient_charge_state_with_vbx(hi[0], hi[1], 0.0, 0.0)
                                    .qbe
                                    - scaled
                                        .legacy_transient_charge_state_with_vbx(
                                            lo[0], lo[1], 0.0, 0.0,
                                        )
                                        .qbe)
                                    / (2.0 * h);
                                assert!(
                                    (numeric - derivative).abs() < 2e-6 * derivative.abs(),
                                    "{parameter}={scale} p={p} column={column}: {derivative:e} != {numeric:e}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn legacy_itf_fraction_preserves_extreme_current_ratios() {
        let mut model = Bjt::new_npn("q".into(), 1, 2, 0);
        for (current, itf, scale, expected) in [
            (1e308, 1e308, 1.0, 0.5),
            (1e308, 1e308, 4.0, 0.2),
            (1e-3, 1e308, 4.0, 2.5e-312),
            (1e-310, 1e-310, 1.0, 0.5),
            (1e-310, 1e-310, 0.25, 0.8),
            (1e-310, 1e-310, 1e-18, 1.0),
            (0.0, 1e-310, 1e-18, 0.0),
        ] {
            model.itf = itf;
            model.m = scale;
            let actual = model.legacy_transit_current_fraction(current);
            assert!(
                (actual - expected).abs() <= expected.abs() * 1e-12 + f64::from_bits(1),
                "I={current} ITF={itf} M={scale}: {actual} != {expected}"
            );
        }
    }

    #[test]
    fn vbic_signed_transport_and_charge_jacobian_cover_reverse_bias_and_rolloff_floor() {
        for level in [4.0, 11.0, 12.0] {
            for qbm in [0.0, 1.0] {
                for knee in [1e-10, 1e-6] {
                    let model = Bjt::new_npn("q".into(), 1, 2, 0).with_params(&HashMap::from([
                        ("LEVEL".into(), level),
                        ("IS".into(), 1e-8),
                        ("ISRR".into(), 0.7),
                        ("NF".into(), 1.1),
                        ("NR".into(), 1.3),
                        ("VEF".into(), 3.0),
                        ("VER".into(), 4.0),
                        ("QBM".into(), qbm),
                        ("NKF".into(), 0.4),
                        ("IKF".into(), knee),
                        ("IKR".into(), knee),
                    ]));
                    for (vbe, vbc) in [(-0.11, -0.09), (-0.03, 0.08), (0.08, -0.03), (0.0, 0.0)] {
                        let state = model.vbic_transport_charge_state(vbe, vbc);
                        let expected_if = model.is * (vbe / (model.nf * model.vt)).exp_m1();
                        let expected_ir =
                            model.is * model.isrr * (vbc / (model.nr * model.vt)).exp_m1();
                        assert!((state.ifi - expected_if).abs() < 1e-20);
                        assert!((state.iri - expected_ir).abs() < 1e-20);
                        if level >= 11.0 && knee == 1e-10 && vbe < 0.0 && vbc < 0.0 {
                            let power = 1e-8_f64.powf(model.nkf);
                            let expected_qb = if qbm < 0.5 {
                                0.5 * (state.q1 + power)
                            } else {
                                0.5 * state.q1 * (1.0 + power)
                            };
                            assert_eq!(state.qb, expected_qb);
                        }
                        let h = 1e-7;
                        for (column, derivatives) in [
                            [
                                state.dqb_dvbe_eff,
                                state.ditzf_dvbe_eff,
                                state.ditzr_dvbe_eff,
                            ],
                            [
                                state.dqb_dvbc_eff,
                                state.ditzf_dvbc_eff,
                                state.ditzr_dvbc_eff,
                            ],
                        ]
                        .into_iter()
                        .enumerate()
                        {
                            let mut plus = [vbe, vbc];
                            let mut minus = plus;
                            plus[column] += h;
                            minus[column] -= h;
                            let plus = model.vbic_transport_charge_state(plus[0], plus[1]);
                            let minus = model.vbic_transport_charge_state(minus[0], minus[1]);
                            for ((hi, lo), actual) in [plus.qb, plus.itzf, plus.itzr]
                                .into_iter()
                                .zip([minus.qb, minus.itzf, minus.itzr])
                                .zip(derivatives)
                            {
                                let fd = (hi - lo) / (2.0 * h);
                                assert!(
                                    (actual - fd).abs() < 2e-6 * actual.abs().max(fd.abs()) + 1e-15,
                                    "level={level} QBM={qbm} knee={knee} bias=({vbe},{vbc}) column={column}: {actual:e} != {fd:e}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
