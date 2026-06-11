//! Intrinsic branch current and derivative builders.

use super::*;

impl Bjt {
    pub(in crate::device::semiconductor::bjt) fn intrinsic_terminal_derivatives(
        &self,
        linearized: BjtLinearization,
    ) -> (
        [Value; INTERNAL_DIM],
        [Value; INTERNAL_DIM],
        [Value; INTERNAL_DIM],
    ) {
        let mut collector = [0.0; INTERNAL_DIM];
        collector[IDX_VCI] = -linearized.dic_dvbc;
        collector[IDX_VBI] = linearized.dic_dvbe + linearized.dic_dvbc;
        collector[IDX_VEI] = -linearized.dic_dvbe;
        collector[IDX_VRTH] = linearized.dic_dvrth;

        let mut base = [0.0; INTERNAL_DIM];
        base[IDX_VCI] = -linearized.dib_dvbc;
        base[IDX_VBI] = linearized.dib_dvbe + linearized.dib_dvbc;
        base[IDX_VEI] = -linearized.dib_dvbe;
        base[IDX_VRTH] = linearized.dib_dvrth;

        let mut emitter = [0.0; INTERNAL_DIM];
        for idx in 0..INTERNAL_DIM {
            emitter[idx] = -(collector[idx] + base[idx]);
        }

        (collector, base, emitter)
    }

    pub(in crate::device::semiconductor::bjt) fn ircx_branch(
        &self,
        vc: Value,
        vcx: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rcx) {
            return branch;
        }

        let g = 1.0 / self.rcx.max(1e-12);
        branch.current = g * (vc - vcx);
        branch.d_internal[IDX_VCX] = -g;
        branch.d_external[0] = g;
        branch
    }

    pub(in crate::device::semiconductor::bjt) fn irbx_branch(
        &self,
        vb: Value,
        vbx: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rbx) {
            return branch;
        }

        let g = 1.0 / self.rbx.max(1e-12);
        branch.current = g * (vb - vbx);
        branch.d_internal[IDX_VBX] = -g;
        branch.d_external[1] = g;
        branch
    }

    pub(in crate::device::semiconductor::bjt) fn ire_branch(
        &self,
        ve: Value,
        vei: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.re) {
            return branch;
        }

        let g = 1.0 / self.re.max(1e-12);
        branch.current = g * (ve - vei);
        branch.d_internal[IDX_VEI] = -g;
        branch.d_external[2] = g;
        branch
    }

    pub(in crate::device::semiconductor::bjt) fn irbi_branch(
        &self,
        linearized: BjtLinearization,
        vbx: Value,
        vbi: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rbi) {
            return branch;
        }

        let rb = self.rbi.max(1e-12);
        let vrbi = vbx - vbi;
        let qb = linearized.qb.max(1e-12);
        let scale = vrbi / rb;
        let dqb_dvbi = linearized.dqb_dvbe + linearized.dqb_dvbc;
        let dqb_dvci = -linearized.dqb_dvbc;
        let dqb_dvei = -linearized.dqb_dvbe;

        branch.current = scale * qb;
        branch.d_internal[IDX_VBX] = qb / rb;
        branch.d_internal[IDX_VBI] = -qb / rb + scale * dqb_dvbi;
        branch.d_internal[IDX_VCI] = scale * dqb_dvci;
        branch.d_internal[IDX_VEI] = scale * dqb_dvei;
        branch
    }

    pub(in crate::device::semiconductor::bjt) fn ibep_branch(
        &self,
        vbx: Value,
        vbp: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        // ngspice vbicload.c stamps the `CKTgmin` parallel on Vbep
        // unconditionally, even when the parasitic diode currents are zero.
        let gmin = self.junction_gmin;
        branch.current = gmin * (vbx - vbp);
        branch.d_internal[IDX_VBX] = gmin;
        branch.d_internal[IDX_VBP] = -gmin;
        if self.ibeip <= 0.0 && self.ibenp <= 0.0 {
            return branch;
        }

        let p = self.polarity();
        let vbep_eff = p * (vbx - vbp);
        let ibeip = self.diode_current_with_is(self.ibeip, vbep_eff, self.nci.max(1e-12));
        let ibenp = self.diode_current_with_is(self.ibenp, vbep_eff, self.ncn.max(1e-12));
        let gbep = self.diode_conductance_with_is(self.ibeip, vbep_eff, self.nci.max(1e-12))
            + self.diode_conductance_with_is(self.ibenp, vbep_eff, self.ncn.max(1e-12));

        branch.current += p * (ibeip + ibenp);
        branch.d_internal[IDX_VBX] += gbep;
        branch.d_internal[IDX_VBP] -= gbep;
        branch
    }

    pub(in crate::device::semiconductor::bjt) fn parasitic_transport_state(
        &self,
        vbx: Value,
        vbi: Value,
        vci: Value,
        vbp: Value,
        vsi: Value,
    ) -> ParasiticTransportState {
        let mut state = ParasiticTransportState {
            qbp: 1.0,
            d_qbp: [0.0; INTERNAL_DIM],
            ifp: 0.0,
            d_ifp: [0.0; INTERNAL_DIM],
            irp: 0.0,
            d_irp: [0.0; INTERNAL_DIM],
        };

        if self.isp <= 0.0 {
            return state;
        }

        let p = self.polarity();
        let nfp_vt = (self.nfp.max(1e-12) * self.vt.max(1e-12)).max(1e-18);
        let vbep_eff = p * (vbx - vbp);
        let vbci_eff = p * (vbi - vci);
        let vbcp_eff = p * (vsi - vbp);

        let (exp_bep, dexp_bep_darg) = Self::limited_exp(vbep_eff / nfp_vt);
        let (exp_bci, dexp_bci_darg) = Self::limited_exp(vbci_eff / nfp_vt);
        let d_ifp_d_vbep_eff = self.isp * self.wsp * dexp_bep_darg / nfp_vt;
        let d_ifp_d_vbci_eff = self.isp * (1.0 - self.wsp) * dexp_bci_darg / nfp_vt;
        state.ifp = self.isp * (self.wsp * exp_bep + (1.0 - self.wsp) * exp_bci - 1.0);
        state.d_ifp[IDX_VBX] = d_ifp_d_vbep_eff * p;
        state.d_ifp[IDX_VBP] = -d_ifp_d_vbep_eff * p;
        state.d_ifp[IDX_VBI] = d_ifp_d_vbci_eff * p;
        state.d_ifp[IDX_VCI] = -d_ifp_d_vbci_eff * p;

        let iikp = if self.ikp.is_finite() && self.ikp > 0.0 {
            1.0 / self.ikp
        } else {
            0.0
        };
        let sqrt_term = (1.0 + 4.0 * state.ifp * iikp).max(1e-18).sqrt();
        state.qbp = (0.5 * (1.0 + sqrt_term)).max(1e-12);
        if iikp > 0.0 {
            let d_qbp_d_ifp = iikp / sqrt_term;
            for idx in 0..INTERNAL_DIM {
                state.d_qbp[idx] = d_qbp_d_ifp * state.d_ifp[idx];
            }
        }

        let (exp_bcp, dexp_bcp_darg) = Self::limited_exp(vbcp_eff / nfp_vt);
        let d_irp_d_vbcp_eff = self.isp * dexp_bcp_darg / nfp_vt;
        state.irp = self.isp * (exp_bcp - 1.0);
        state.d_irp[IDX_VSI] = d_irp_d_vbcp_eff * p;
        state.d_irp[IDX_VBP] = -d_irp_d_vbcp_eff * p;

        state
    }

    pub(in crate::device::semiconductor::bjt) fn irbp_branch(
        &self,
        vbx: Value,
        vbi: Value,
        vcx: Value,
        vci: Value,
        vbp: Value,
        vsi: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rbp) {
            return branch;
        }

        let parasitic = self.parasitic_transport_state(vbx, vbi, vci, vbp, vsi);
        let rbp = self.rbp.max(1e-12);
        let vrbp = vbp - vcx;
        let scale = vrbp / rbp;

        branch.current = scale * parasitic.qbp;
        branch.d_internal[IDX_VCX] = -parasitic.qbp / rbp;
        branch.d_internal[IDX_VBP] = parasitic.qbp / rbp + scale * parasitic.d_qbp[IDX_VBP];
        branch.d_internal[IDX_VBX] = scale * parasitic.d_qbp[IDX_VBX];
        branch.d_internal[IDX_VBI] = scale * parasitic.d_qbp[IDX_VBI];
        branch.d_internal[IDX_VCI] = scale * parasitic.d_qbp[IDX_VCI];
        branch.d_internal[IDX_VSI] = scale * parasitic.d_qbp[IDX_VSI];
        branch
    }

    pub(in crate::device::semiconductor::bjt) fn ibcp_branch(
        &self,
        vbp: Value,
        vsi: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        // ngspice vbicload.c stamps the `CKTgmin` parallel on Vbcp
        // unconditionally, even when the parasitic diode currents are zero.
        let gmin = self.junction_gmin;
        branch.current = gmin * (vsi - vbp);
        branch.d_internal[IDX_VSI] = gmin;
        branch.d_internal[IDX_VBP] = -gmin;
        if self.ibcip <= 0.0 && self.ibcnp <= 0.0 {
            return branch;
        }

        let p = self.polarity();
        let vbcp_eff = p * (vsi - vbp);
        let ibcip = self.diode_current_with_is(self.ibcip, vbcp_eff, self.ncip.max(1e-12));
        let ibcnp = self.diode_current_with_is(self.ibcnp, vbcp_eff, self.ncnp.max(1e-12));
        let gbcp = self.diode_conductance_with_is(self.ibcip, vbcp_eff, self.ncip.max(1e-12))
            + self.diode_conductance_with_is(self.ibcnp, vbcp_eff, self.ncnp.max(1e-12));

        branch.current += p * (ibcip + ibcnp);
        branch.d_internal[IDX_VSI] += gbcp;
        branch.d_internal[IDX_VBP] -= gbcp;
        branch
    }

    pub(in crate::device::semiconductor::bjt) fn iccp_branch(
        &self,
        vbx: Value,
        vbi: Value,
        vci: Value,
        vbp: Value,
        vsi: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if self.isp <= 0.0 {
            return branch;
        }

        let parasitic = self.parasitic_transport_state(vbx, vbi, vci, vbp, vsi);
        let p = self.polarity();
        let inv_qbp = 1.0 / parasitic.qbp.max(1e-12);
        let delta = parasitic.ifp - parasitic.irp;

        branch.current = p * delta * inv_qbp;
        for idx in 0..INTERNAL_DIM {
            branch.d_internal[idx] = p
                * ((parasitic.d_ifp[idx] - parasitic.d_irp[idx]) * inv_qbp
                    - delta * parasitic.d_qbp[idx] * inv_qbp * inv_qbp);
        }
        branch
    }

    pub(in crate::device::semiconductor::bjt) fn irs_branch(
        &self,
        vs: Value,
        vsi: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rs) {
            return branch;
        }

        let g = 1.0 / self.rs.max(1e-12);
        branch.current = g * (vs - vsi);
        branch.d_internal[IDX_VSI] = -g;
        branch.d_external[EXT_S] = g;
        branch
    }

    pub(in crate::device::semiconductor::bjt) fn irci_branch(
        &self,
        vcx: Value,
        vci: Value,
        vbi: Value,
    ) -> BranchLinearization {
        self.irci_branch_with_self_conductance(vcx, vci, vbi).0
    }

    /// Kull epi branch together with its `dIrci/dVrci` partial in ngspice's
    /// parameterization. The three controlling voltages satisfy
    /// `vrci = vbci − vbcx` identically, so this partial is not recoverable
    /// from the node-space derivatives; vbicload.c stores it as the
    /// `Irci_Vrci` state (gmin parallel included) and vbicnoise.c reads that
    /// state as the thermal-noise conductance of the epi resistance.
    pub(in crate::device::semiconductor::bjt) fn irci_branch_with_self_conductance(
        &self,
        vcx: Value,
        vci: Value,
        vbi: Value,
    ) -> (BranchLinearization, Value) {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rci) {
            return (branch, 0.0);
        }

        let p = self.polarity();
        let vt = self.vt.max(1e-12);
        let rci = self.rci.max(1e-12);
        let gamm = self.gamm.max(0.0);
        let ivo = if self.vo.is_finite() && self.vo > 0.0 {
            1.0 / self.vo
        } else {
            0.0
        };
        let ihrcf = if self.hrcf.is_finite() && self.hrcf > 0.0 {
            1.0 / self.hrcf
        } else {
            0.0
        };

        let vrci_eff = p * (vcx - vci);
        let vbci_eff = p * (vbi - vci);
        let vbcx_eff = p * (vbi - vcx);

        let (exp_bci, dexp_bci_darg) = Self::limited_exp(vbci_eff / vt);
        let (exp_bcx, dexp_bcx_darg) = Self::limited_exp(vbcx_eff / vt);
        let d_exp_bci_dvbci_eff = dexp_bci_darg / vt;
        let d_exp_bcx_dvbcx_eff = dexp_bcx_darg / vt;

        let kbci = (1.0 + gamm * exp_bci).sqrt().max(1e-12);
        let kbcx = (1.0 + gamm * exp_bcx).sqrt().max(1e-12);
        let d_kbci_dvbci_eff = if gamm > 0.0 {
            gamm * d_exp_bci_dvbci_eff / (2.0 * kbci)
        } else {
            0.0
        };
        let d_kbcx_dvbcx_eff = if gamm > 0.0 {
            gamm * d_exp_bcx_dvbcx_eff / (2.0 * kbcx)
        } else {
            0.0
        };

        let ratio = ((kbci + 1.0) / (kbcx + 1.0)).max(1e-18);
        let log_ratio = ratio.ln();
        let d_ratio_dkbci = 1.0 / (kbcx + 1.0);
        let d_ratio_dkbcx = -(kbci + 1.0) / (kbcx + 1.0).powi(2);
        let d_log_ratio_dkbci = d_ratio_dkbci / ratio;
        let d_log_ratio_dkbcx = d_ratio_dkbcx / ratio;

        let iohm = (vrci_eff + vt * (kbci - kbcx - log_ratio)) / rci;
        let d_iohm_dvrci_eff = 1.0 / rci;
        let d_iohm_dvbci_eff = vt * d_kbci_dvbci_eff * (1.0 - d_log_ratio_dkbci) / rci;
        let d_iohm_dvbcx_eff = vt * d_kbcx_dvbcx_eff * (-1.0 - d_log_ratio_dkbcx) / rci;

        let sqrt_vrci = (vrci_eff * vrci_eff + 0.01).sqrt();
        let denom = 1.0 + 0.5 * ivo * ihrcf * sqrt_vrci;
        let d_denom_dvrci_eff = if ivo > 0.0 && ihrcf > 0.0 {
            0.5 * ivo * ihrcf * vrci_eff / sqrt_vrci
        } else {
            0.0
        };

        let derf_scale = ivo * rci;
        let derf = if derf_scale > 0.0 {
            derf_scale * iohm / denom
        } else {
            0.0
        };
        let d_derf_dvrci_eff = if derf_scale > 0.0 {
            derf_scale * (d_iohm_dvrci_eff / denom - iohm * d_denom_dvrci_eff / denom.powi(2))
        } else {
            0.0
        };
        let d_derf_dvbci_eff = if derf_scale > 0.0 {
            derf_scale * d_iohm_dvbci_eff / denom
        } else {
            0.0
        };
        let d_derf_dvbcx_eff = if derf_scale > 0.0 {
            derf_scale * d_iohm_dvbcx_eff / denom
        } else {
            0.0
        };

        let irci_scale = (1.0 + derf * derf).sqrt();
        let inv_irci_scale = 1.0 / irci_scale;
        let common = -iohm * derf / (irci_scale * irci_scale * irci_scale);
        // ngspice vbicload.c gives Irci a `CKTgmin` parallel per controlling
        // voltage (Vrci, Vbci, Vbcx). These are what keep the CX/CI rows
        // nonsingular when the Kull epi exponential crosses its saturation
        // knife edge (the very rows ngspice reports as singular on decks it
        // cannot solve), and gmin stepping ramps them with `CKTgmin`.
        let gmin = self.junction_gmin;
        let d_irci_eff_dvrci_eff =
            d_iohm_dvrci_eff * inv_irci_scale + common * d_derf_dvrci_eff + gmin;
        let d_irci_eff_dvbci_eff =
            d_iohm_dvbci_eff * inv_irci_scale + common * d_derf_dvbci_eff + gmin;
        let d_irci_eff_dvbcx_eff =
            d_iohm_dvbcx_eff * inv_irci_scale + common * d_derf_dvbcx_eff + gmin;
        let irci_eff = iohm * inv_irci_scale + gmin * (vrci_eff + vbci_eff + vbcx_eff);

        branch.current = p * irci_eff;
        branch.d_internal[IDX_VCX] = d_irci_eff_dvrci_eff - d_irci_eff_dvbcx_eff;
        branch.d_internal[IDX_VCI] = -(d_irci_eff_dvrci_eff + d_irci_eff_dvbci_eff);
        branch.d_internal[IDX_VBI] = d_irci_eff_dvbci_eff + d_irci_eff_dvbcx_eff;
        (branch, d_irci_eff_dvrci_eff)
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn thermal_derivative_step(
        &self,
        vrth: Value,
    ) -> Value {
        // Use a small relative perturbation to keep Vrth-derivative finite
        // differences accurate for strongly temperature-sensitive currents.
        ((self.requested_temperature() + vrth).abs().max(1.0) * 1e-6).clamp(1e-7, 1e-3)
    }
}
