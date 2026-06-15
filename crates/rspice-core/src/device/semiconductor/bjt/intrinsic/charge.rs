//! Legacy and VBIC transport-charge linearization helpers.

use super::*;

impl Bjt {
    pub(in crate::device::semiconductor::bjt) fn legacy_transport_charge_state(
        &self,
        vbe_eff: Value,
        vbc_eff: Value,
    ) -> TransportChargeState {
        let ifi = self.diode_current(vbe_eff, self.nf);
        let iri = self.diode_current_with_is(self.is * self.isrr.max(0.0), vbc_eff, self.nr);
        let gfi = self.diode_conductance(vbe_eff, self.nf);
        let gri = self.diode_conductance_with_is(self.is * self.isrr.max(0.0), vbc_eff, self.nr);

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

        let inv_rolloff_f = if self.ikf > 0.0 { 1.0 / self.ikf } else { 0.0 };
        let inv_rolloff_r = if self.ikr > 0.0 { 1.0 / self.ikr } else { 0.0 };
        let (qb, dqb_dvbe_eff, dqb_dvbc_eff) = if inv_rolloff_f == 0.0 && inv_rolloff_r == 0.0 {
            (q1.max(1e-12), dq1_dvbe_eff, dq1_dvbc_eff)
        } else {
            let q2 = inv_rolloff_f * ifi + inv_rolloff_r * iri;
            let dq2_dvbe_eff = inv_rolloff_f * gfi;
            let dq2_dvbc_eff = inv_rolloff_r * gri;
            let sqrt_arg = (1.0 + 4.0 * q2).max(0.0);
            let sqrt_term = if sqrt_arg > 0.0 {
                sqrt_arg.sqrt().max(1e-18)
            } else {
                1.0
            };
            (
                (0.5 * q1 * (1.0 + sqrt_term)).max(1e-12),
                0.5 * (1.0 + sqrt_term) * dq1_dvbe_eff + q1 * dq2_dvbe_eff / sqrt_term,
                0.5 * (1.0 + sqrt_term) * dq1_dvbc_eff + q1 * dq2_dvbc_eff / sqrt_term,
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

    pub(crate) fn legacy_transient_charge_state(
        &self,
        vbe: Value,
        vbc: Value,
        vcs: Value,
    ) -> LegacyTransientChargeState {
        self.legacy_transient_charge_state_with_vbx(vbe, vbc, vbc, vcs)
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
                let temp = transport.ifi / (transport.ifi + self.itf).max(1e-18);
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
            qbe: p * (self.tf * qbe_diffusion_current + self.cje * qbe_dep_norm + self.cbeo * vbe),
            capbe: (self.tf * gbe_dynamic + self.cje * capbe_dep + self.cbeo).max(0.0),
            capbe_vbc: self.tf * geqcb_dynamic,
            qbc: p * (self.tr * transport.iri + cjc_internal * qbc_dep_norm + self.cbco * vbc),
            capbc: (self.tr * transport.gri + cjc_internal * capbc_dep + self.cbco).max(0.0),
            qbx: p * (cjc_external * qbx_dep_norm),
            capbx: (cjc_external * capbx_dep).max(0.0),
            qcs: -substrate_polarity * (self.cjcp * qsub_norm),
            capcs: (self.cjcp * capsub_dep).max(0.0),
        }
    }

    #[inline]
    pub(crate) fn legacy_charge_branch_voltages(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> (Value, Value, Value) {
        let static_internal = if self.cache_matches_external_biases(vc, vb, ve, vs) {
            self.internal_state_vector()
        } else {
            let internal = self.dynamic_internal_state_seed(vc, vb, ve, vs);
            let mut static_internal = [0.0; INTERNAL_DIM];
            static_internal.copy_from_slice(&internal[..INTERNAL_DIM]);
            static_internal
        };
        let external = [vc, vb, ve, vs];
        let terminal_voltage = |terminal: (Option<usize>, Option<usize>)| -> Value {
            if let Some(idx) = terminal.0 {
                static_internal[idx]
            } else if let Some(idx) = terminal.1 {
                external[idx]
            } else {
                0.0
            }
        };
        let substrate_connection = self.legacy_charge_substrate_connection_terminal();
        let substrate_terminal = self.legacy_charge_substrate_terminal();
        (
            static_internal[IDX_VBI] - static_internal[IDX_VEI],
            static_internal[IDX_VBI] - static_internal[IDX_VCI],
            terminal_voltage(substrate_connection) - terminal_voltage(substrate_terminal),
        )
    }

    pub(in crate::device::semiconductor::bjt) fn vbic_transport_charge_state(
        &self,
        vbe_eff: Value,
        vbc_eff: Value,
    ) -> TransportChargeState {
        let ifi = self.diode_current(vbe_eff, self.nf).max(0.0);
        let iri = self
            .diode_current_with_is(self.is * self.isrr.max(0.0), vbc_eff, self.nr)
            .max(0.0);
        let gfi = self.diode_conductance(vbe_eff, self.nf);
        let gri = self.diode_conductance_with_is(self.is * self.isrr.max(0.0), vbc_eff, self.nr);

        let (qdbe, dqdbe_dvbe_eff) = self
            .vbic_depletion_charge_and_derivative(vbe_eff, self.vje, self.mje, self.fc, self.aje);
        let (qdbc, dqdbc_dvbc_eff) = self
            .vbic_depletion_charge_and_derivative(vbc_eff, self.vjc, self.mjc, self.fc, self.ajc);

        let q1z =
            1.0 + if self.var.is_finite() && self.var > 0.0 {
                qdbe / self.var
            } else {
                0.0
            } + if self.vaf.is_finite() && self.vaf > 0.0 {
                qdbc / self.vaf
            } else {
                0.0
            };
        let q1_shift = q1z - 1e-4;
        let q1_sqrt = (q1_shift * q1_shift + 1e-8).sqrt();
        let q1 = 0.5 * (q1_sqrt + q1_shift) + 1e-4;
        let dq1_dq1z = 0.5 * (q1_shift / q1_sqrt + 1.0);
        let dq1_dvbe_eff = dq1_dq1z
            * if self.var.is_finite() && self.var > 0.0 {
                dqdbe_dvbe_eff / self.var
            } else {
                0.0
            };
        let dq1_dvbc_eff = dq1_dq1z
            * if self.vaf.is_finite() && self.vaf > 0.0 {
                dqdbc_dvbc_eff / self.vaf
            } else {
                0.0
            };

        let inv_rolloff_f = if self.ikf > 0.0 { 1.0 / self.ikf } else { 0.0 };
        let inv_rolloff_r = if self.ikr > 0.0 { 1.0 / self.ikr } else { 0.0 };
        let q2 = inv_rolloff_f * ifi + inv_rolloff_r * iri;
        let dq2_dvbe_eff = inv_rolloff_f * gfi;
        let dq2_dvbc_eff = inv_rolloff_r * gri;
        let nkf = self.nkf.max(1e-12);
        let (qb, dqb_dvbe_eff, dqb_dvbc_eff) = if self.qbm < 0.5 {
            let inv_nkf = 1.0 / nkf;
            let xvar3 = q1.max(1e-18).powf(inv_nkf);
            let dxvar3_dvbe_eff = if q1 > 0.0 {
                xvar3 * inv_nkf * dq1_dvbe_eff / q1.max(1e-18)
            } else {
                0.0
            };
            let dxvar3_dvbc_eff = if q1 > 0.0 {
                xvar3 * inv_nkf * dq1_dvbc_eff / q1.max(1e-18)
            } else {
                0.0
            };
            let xvar1 = (xvar3 + 4.0 * q2).max(1e-18);
            let dxvar1_dvbe_eff = dxvar3_dvbe_eff + 4.0 * dq2_dvbe_eff;
            let dxvar1_dvbc_eff = dxvar3_dvbc_eff + 4.0 * dq2_dvbc_eff;
            let xvar4 = xvar1.powf(nkf);
            let dxvar4_dvbe_eff = xvar4 * nkf * dxvar1_dvbe_eff / xvar1;
            let dxvar4_dvbc_eff = xvar4 * nkf * dxvar1_dvbc_eff / xvar1;
            (
                (0.5 * (q1 + xvar4)).max(1e-12),
                0.5 * (dq1_dvbe_eff + dxvar4_dvbe_eff),
                0.5 * (dq1_dvbc_eff + dxvar4_dvbc_eff),
            )
        } else {
            let xvar1 = (1.0 + 4.0 * q2).max(1e-18);
            let dxvar1_dvbe_eff = 4.0 * dq2_dvbe_eff;
            let dxvar1_dvbc_eff = 4.0 * dq2_dvbc_eff;
            let xvar2 = xvar1.powf(nkf);
            let dxvar2_dvbe_eff = xvar2 * nkf * dxvar1_dvbe_eff / xvar1;
            let dxvar2_dvbc_eff = xvar2 * nkf * dxvar1_dvbc_eff / xvar1;
            (
                (0.5 * q1 * (1.0 + xvar2)).max(1e-12),
                0.5 * (1.0 + xvar2) * dq1_dvbe_eff + 0.5 * q1 * dxvar2_dvbe_eff,
                0.5 * (1.0 + xvar2) * dq1_dvbc_eff + 0.5 * q1 * dxvar2_dvbc_eff,
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
        let ibcj = self.diode_current_with_is(self.ibci, vbc_eff, self.nci)
            + self.diode_current_with_is(self.ibcn, vbc_eff, self.ncn);
        let dibcj_dvbc_eff = self.diode_conductance_with_is(self.ibci, vbc_eff, self.nci)
            + self.diode_conductance_with_is(self.ibcn, vbc_eff, self.ncn);

        if self.avc1 <= 0.0 {
            return BaseCollectorCurrentState {
                ibc: ibcj,
                dibc_dvbe_eff: 0.0,
                dibc_dvbc_eff: dibcj_dvbc_eff,
            };
        }

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
            * (dvl_dvbc_eff * avalanche_exp + vl * d_avalanche_exp_darg * d_avalanche_arg_dvbc_eff);

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

    pub(in crate::device::semiconductor::bjt) fn linearize_currents_with_branches(
        &self,
        vbe: Value,
        vbc: Value,
    ) -> (BjtLinearization, BjtIntrinsicBranches) {
        let p = self.polarity();
        let vbe_eff = p * vbe;
        let vbc_eff = p * vbc;
        let transport = self.transport_charge_state(vbe_eff, vbc_eff);
        let bc = self.base_collector_current_state(transport, vbc_eff);

        // ngspice load discipline (bjtload.c / vbicload.c): every junction
        // current carries a `CKTgmin` parallel. The parallels keep junction
        // rows nonsingular at saturation boundaries, and gmin stepping ramps
        // them through the device equations, not just the matrix diagonal.
        let gmin = self.junction_gmin;
        let ib_be = self.diode_current_with_is(self.ibei, vbe_eff, self.nei)
            + self.diode_current_with_is(self.iben, vbe_eff, self.nen)
            + gmin * vbe_eff;
        let dibe_dvbe = self.gbe(vbe) + gmin;
        let ibc = bc.ibc + gmin * vbc_eff;
        let dibc_dvbc = bc.dibc_dvbc_eff + gmin;
        let iciei = transport.itzf - transport.itzr;
        let diciei_dvbe = transport.ditzf_dvbe_eff - transport.ditzr_dvbe_eff;
        let diciei_dvbc = transport.ditzf_dvbc_eff - transport.ditzr_dvbc_eff;
        let ibe_branch = Self::branch_from_vbe_vbc(p * ib_be, dibe_dvbe, 0.0);
        let ibc_branch = Self::branch_from_vbe_vbc(p * ibc, bc.dibc_dvbe_eff, dibc_dvbc);
        let iciei_branch = Self::branch_from_vbe_vbc(p * iciei, diciei_dvbe, diciei_dvbc);
        let linearized = BjtLinearization {
            // The intrinsic collector terminal sees both the transport branch
            // (collector to emitter) and the opposing B-C junction branch.
            ic: p * (iciei - ibc),
            ib: p * (ib_be + ibc),
            dic_dvbe: diciei_dvbe - bc.dibc_dvbe_eff,
            dic_dvbc: diciei_dvbc - dibc_dvbc,
            dic_dvrth: 0.0,
            dib_dvbe: dibe_dvbe + bc.dibc_dvbe_eff,
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
                ibc: ibc_branch,
                iciei: iciei_branch,
            },
        )
    }
}
