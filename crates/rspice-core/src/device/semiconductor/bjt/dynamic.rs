//! VBIC dynamic charge reduction and charge-snapshot construction.

use super::*;

impl Bjt {
    pub(super) fn epi_charge_state(&self, vcx: Value, vci: Value, vbi: Value) -> EpiChargeState {
        let mut state = EpiChargeState {
            kbci: 1.0,
            d_kbci: [0.0; INTERNAL_DIM],
            kbcx: 1.0,
            d_kbcx: [0.0; INTERNAL_DIM],
        };

        if self.gamm <= 0.0 {
            return state;
        }

        let p = self.polarity();
        let vt = self.vt.max(1e-12);
        let vbci_eff = p * (vbi - vci);
        let vbcx_eff = p * (vbi - vcx);

        let (exp_bci, dexp_bci_darg) = Self::limited_exp(vbci_eff / vt);
        let (exp_bcx, dexp_bcx_darg) = Self::limited_exp(vbcx_eff / vt);
        let d_exp_bci_dv = dexp_bci_darg / vt;
        let d_exp_bcx_dv = dexp_bcx_darg / vt;

        state.kbci = (1.0 + self.gamm * exp_bci).sqrt().max(1e-12);
        state.kbcx = (1.0 + self.gamm * exp_bcx).sqrt().max(1e-12);

        let d_kbci_dv = self.gamm * d_exp_bci_dv / (2.0 * state.kbci);
        let d_kbcx_dv = self.gamm * d_exp_bcx_dv / (2.0 * state.kbcx);

        state.d_kbci[IDX_VBI] = p * d_kbci_dv;
        state.d_kbci[IDX_VCI] = -p * d_kbci_dv;
        state.d_kbcx[IDX_VBI] = p * d_kbcx_dv;
        state.d_kbcx[IDX_VCX] = -p * d_kbcx_dv;
        state
    }

    pub(super) fn dynamic_reduction_template(
        &self,
        base: BjtReducedLinearization,
    ) -> BjtDynamicReduction {
        let mut reduction = BjtDynamicReduction {
            external_voltages: base.external_voltages,
            g_ee: base.g_ee,
            g_reduced: base.g_reduced,
            ..Default::default()
        };

        for idx in 0..INTERNAL_DIM {
            reduction.internal_voltages[idx] = base.internal_voltages[idx];
        }
        for row in 0..INTERNAL_DIM {
            for col in 0..INTERNAL_DIM {
                reduction.g_ii[row][col] = base.g_ii[row][col];
            }
            reduction.g_ie[row] = base.g_ie[row];
            reduction.z_i_static[row] = base.z_i_static[row];
        }
        for row in 0..EXTERNAL_DIM {
            for col in 0..INTERNAL_DIM {
                reduction.g_ei[row][col] = base.g_ei[row][col];
            }
            reduction.z_e_static[row] = base.z_e_static[row];
        }

        // Default the excess-phase states to decoupled algebraic identities when
        // TD is not active so the dynamic reduction remains well-conditioned.
        reduction.g_ii[IDX_VXF1][IDX_VXF1] = 1.0;
        reduction.g_ii[IDX_VXF2][IDX_VXF2] = 1.0;
        reduction
    }

    pub(super) fn dynamic_charge_inputs(
        &self,
        external: [Value; EXTERNAL_DIM],
        internal: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> BjtDynamicChargeInputs {
        let [vcx, vci, vbx, vbi, vei, vbp, vsi, _vrth, _, _] = internal;
        let [vc, vb, ve, _vs] = external;
        let p = self.polarity();

        let vbe_eff = p * (vbi - vei);
        let vbex_eff = p * (vbx - vei);
        let vbc_eff = p * (vbi - vci);
        let vbep_eff = p * (vbx - vbp);
        let vbcp_eff = p * (vsi - vbp);
        let vbeo_eff = p * (vb - ve);
        let vbco_eff = p * (vb - vc);

        let transport = self.transport_charge_state(vbe_eff, vbc_eff);
        let parasitic = self.parasitic_transport_state(vbx, vbi, vci, vbp, vsi);
        let epi = self.epi_charge_state(vcx, vci, vbi);

        let (qdbe, dqdbe_dvbe_eff) = self
            .vbic_depletion_charge_and_derivative(vbe_eff, self.vje, self.mje, self.fc, self.aje);
        let (qdbex, dqdbex_dvbex_eff) = self
            .vbic_depletion_charge_and_derivative(vbex_eff, self.vje, self.mje, self.fc, self.aje);
        let (qdbc, dqdbc_dvbc_eff) = self
            .vbic_depletion_charge_and_derivative(vbc_eff, self.vjc, self.mjc, self.fc, self.ajc);
        let (qdbep, dqdbep_dvbep_eff) = self
            .vbic_depletion_charge_and_derivative(vbep_eff, self.vjc, self.mjc, self.fc, self.ajc);
        let (qdbcp, dqdbcp_dvbcp_eff) = self
            .vbic_depletion_charge_and_derivative(vbcp_eff, self.ps, self.ms, self.fc, self.ajs);

        let _ = (vbeo_eff, vbco_eff);

        BjtDynamicChargeInputs {
            transport,
            parasitic,
            epi,
            qdbe,
            dqdbe_dvbe_eff,
            qdbex,
            dqdbex_dvbex_eff,
            qdbc,
            dqdbc_dvbc_eff,
            qdbep,
            dqdbep_dvbep_eff,
            qdbcp,
            dqdbcp_dvbcp_eff,
        }
    }

    pub(crate) fn vbic_delay_static_branches(
        &self,
        reduction: &BjtDynamicReduction,
    ) -> [BjtCurrentBranch; 3] {
        let mut branches = [BjtCurrentBranch::default(); 3];
        if !self.uses_vbic_dynamic_charges() || self.td <= 0.0 {
            return branches;
        }

        let [_, _, _, _, _, _, _, _vrth, vxf1, vxf2] = reduction.internal_voltages;
        let p = self.polarity();
        let transport = reduction.vbic_transport;
        let d_itzf_actual_d_vbi = p * (transport.ditzf_dvbe_eff + transport.ditzf_dvbc_eff);
        let d_itzf_actual_d_vci = -p * transport.ditzf_dvbc_eff;
        let d_itzf_actual_d_vei = -p * transport.ditzf_dvbe_eff;
        let d_p_itzf_d_vbi = transport.ditzf_dvbe_eff + transport.ditzf_dvbc_eff;
        let d_p_itzf_d_vci = -transport.ditzf_dvbc_eff;
        let d_p_itzf_d_vei = -transport.ditzf_dvbe_eff;
        let d_itzf_d_vrth = reduction.vbic_d_itzf_d_vrth;

        let mut delta_iciei = BjtCurrentBranch {
            current: p * (vxf2 - transport.itzf),
            pos_internal: Some(IDX_VEI),
            neg_internal: Some(IDX_VCI),
            ..Default::default()
        };
        delta_iciei.d_internal[IDX_VBI] = -d_p_itzf_d_vbi;
        delta_iciei.d_internal[IDX_VCI] = -d_p_itzf_d_vci;
        delta_iciei.d_internal[IDX_VEI] = -d_p_itzf_d_vei;
        delta_iciei.d_internal[IDX_VRTH] = -p * d_itzf_d_vrth;
        delta_iciei.d_internal[IDX_VXF2] = p;
        branches[0] = delta_iciei;

        let mut ixf1 = BjtCurrentBranch {
            current: vxf2 - transport.itzf,
            pos_internal: Some(IDX_VXF1),
            ..Default::default()
        };
        ixf1.d_internal[IDX_VBI] = -d_itzf_actual_d_vbi;
        ixf1.d_internal[IDX_VCI] = -d_itzf_actual_d_vci;
        ixf1.d_internal[IDX_VEI] = -d_itzf_actual_d_vei;
        ixf1.d_internal[IDX_VRTH] = -d_itzf_d_vrth;
        ixf1.d_internal[IDX_VXF2] = 1.0;
        branches[1] = ixf1;

        let mut ixf2 = BjtCurrentBranch {
            current: vxf2 - vxf1,
            pos_internal: Some(IDX_VXF2),
            ..Default::default()
        };
        ixf2.d_internal[IDX_VXF1] = -1.0;
        ixf2.d_internal[IDX_VXF2] = 1.0;
        branches[2] = ixf2;

        branches
    }

    pub(crate) fn vbic_delay_static_thermal_branch(
        &self,
        reduction: &BjtDynamicReduction,
    ) -> BjtCurrentBranch {
        if !self.uses_vbic_dynamic_charges() || self.td <= 0.0 || !self.self_heating_enabled() {
            return BjtCurrentBranch::default();
        }

        let [_, vci, _, _, vei, _, _, _vrth, _, vxf2] = reduction.internal_voltages;
        let p = self.polarity();
        let transport = reduction.vbic_transport;
        let d_p_itzf_d_vbi = transport.ditzf_dvbe_eff + transport.ditzf_dvbc_eff;
        let d_p_itzf_d_vci = -transport.ditzf_dvbc_eff;
        let d_p_itzf_d_vei = -transport.ditzf_dvbe_eff;
        let d_itzf_d_vrth = reduction.vbic_d_itzf_d_vrth;

        let delta_current = p * (vxf2 - transport.itzf);
        let voltage = vci - vei;
        let mut branch = BjtCurrentBranch {
            current: -delta_current * voltage,
            pos_internal: Some(IDX_VRTH),
            ..Default::default()
        };
        branch.d_internal[IDX_VBI] = d_p_itzf_d_vbi * voltage;
        branch.d_internal[IDX_VCI] = d_p_itzf_d_vci * voltage - delta_current;
        branch.d_internal[IDX_VEI] = d_p_itzf_d_vei * voltage + delta_current;
        branch.d_internal[IDX_VRTH] = p * d_itzf_d_vrth * voltage;
        branch.d_internal[IDX_VXF2] = -p * voltage;
        branch
    }

    pub(super) fn apply_vbic_excess_phase_transport(
        &self,
        mut reduction: BjtDynamicReduction,
        transport: TransportChargeState,
        d_itzf_d_vrth: Value,
    ) -> BjtDynamicReduction {
        if !self.uses_vbic_dynamic_charges() || self.td <= 0.0 {
            return reduction;
        }

        let p = self.polarity();

        // Direct forward transport already appears in the 7-state DC Jacobian.
        // Replace that static path with ngspice's excess-phase xf2-controlled path.
        let d_itzf_actual_d_vbi = p * (transport.ditzf_dvbe_eff + transport.ditzf_dvbc_eff);
        let d_itzf_actual_d_vci = -p * transport.ditzf_dvbc_eff;
        let d_itzf_actual_d_vei = -p * transport.ditzf_dvbe_eff;
        let d_p_itzf_d_vbi = transport.ditzf_dvbe_eff + transport.ditzf_dvbc_eff;
        let d_p_itzf_d_vci = -transport.ditzf_dvbc_eff;
        let d_p_itzf_d_vei = -transport.ditzf_dvbe_eff;

        reduction.g_ii[IDX_VCI][IDX_VBI] += d_p_itzf_d_vbi;
        reduction.g_ii[IDX_VCI][IDX_VCI] += d_p_itzf_d_vci;
        reduction.g_ii[IDX_VCI][IDX_VEI] += d_p_itzf_d_vei;
        reduction.g_ii[IDX_VCI][IDX_VRTH] += p * d_itzf_d_vrth;
        reduction.g_ii[IDX_VCI][IDX_VXF2] -= p;

        reduction.g_ii[IDX_VEI][IDX_VBI] -= d_p_itzf_d_vbi;
        reduction.g_ii[IDX_VEI][IDX_VCI] -= d_p_itzf_d_vci;
        reduction.g_ii[IDX_VEI][IDX_VEI] -= d_p_itzf_d_vei;
        reduction.g_ii[IDX_VEI][IDX_VRTH] -= p * d_itzf_d_vrth;
        reduction.g_ii[IDX_VEI][IDX_VXF2] += p;

        reduction.g_ii[IDX_VXF1] = [0.0; BJT_INTERNAL_STATE_DIM];
        reduction.g_ii[IDX_VXF1][IDX_VBI] = -d_itzf_actual_d_vbi;
        reduction.g_ii[IDX_VXF1][IDX_VCI] = -d_itzf_actual_d_vci;
        reduction.g_ii[IDX_VXF1][IDX_VEI] = -d_itzf_actual_d_vei;
        reduction.g_ii[IDX_VXF1][IDX_VRTH] = -d_itzf_d_vrth;
        reduction.g_ii[IDX_VXF1][IDX_VXF2] = 1.0;

        reduction.g_ii[IDX_VXF2] = [0.0; BJT_INTERNAL_STATE_DIM];
        reduction.g_ii[IDX_VXF2][IDX_VXF1] = -1.0;
        reduction.g_ii[IDX_VXF2][IDX_VXF2] = 1.0;

        reduction
    }

    pub(super) fn build_dynamic_reduction_from_transport(
        &self,
        mut reduction: BjtDynamicReduction,
        transport: TransportChargeState,
        d_itzf_d_vrth: Value,
    ) -> BjtDynamicReduction {
        reduction.vbic_transport = transport;
        reduction.vbic_d_itzf_d_vrth = d_itzf_d_vrth;
        if !self.uses_vbic_dynamic_charges() || self.td <= 0.0 {
            return reduction;
        }

        reduction.internal_voltages[IDX_VXF1] = transport.itzf;
        reduction.internal_voltages[IDX_VXF2] = transport.itzf;
        self.apply_vbic_excess_phase_transport(reduction, transport, d_itzf_d_vrth)
    }

    pub(super) fn dynamic_charge_branches_from_inputs(
        &self,
        reduction: &BjtDynamicReduction,
        inputs: BjtDynamicChargeInputs,
    ) -> [BjtChargeBranch; BJT_DYNAMIC_CHARGE_COUNT] {
        let mut branches = [BjtChargeBranch::default(); BJT_DYNAMIC_CHARGE_COUNT];
        if !self.uses_vbic_dynamic_charges() {
            return branches;
        }

        let [_, vci, vbx, vbi, vei, vbp, vsi, vrth, vxf1, vxf2] = reduction.internal_voltages;
        let [vc, vb, ve, _vs] = reduction.external_voltages;
        let p = self.polarity();
        let wbe = self.wbe;
        let transport = inputs.transport;
        let parasitic = inputs.parasitic;
        let epi = inputs.epi;
        let qdbe = inputs.qdbe;
        let dqdbe_dvbe_eff = inputs.dqdbe_dvbe_eff;
        let qdbex = inputs.qdbex;
        let dqdbex_dvbex_eff = inputs.dqdbex_dvbex_eff;
        let qdbc = inputs.qdbc;
        let dqdbc_dvbc_eff = inputs.dqdbc_dvbc_eff;
        let qdbep = inputs.qdbep;
        let dqdbep_dvbep_eff = inputs.dqdbep_dvbep_eff;
        let qdbcp = inputs.qdbcp;
        let dqdbcp_dvbcp_eff = inputs.dqdbcp_dvbcp_eff;
        let vbc_eff = p * (vbi - vci);
        let _vbex_eff = p * (vbx - vei);
        let _vbep_eff = p * (vbx - vbp);
        let vbcp_eff = p * (vsi - vbp);
        let vbeo_eff = p * (vb - ve);
        let vbco_eff = p * (vb - vc);

        let sg_if = if transport.ifi > 0.0 { 1.0 } else { 0.0 };
        let iitf = if self.itf > 0.0 { 1.0 / self.itf } else { 0.0 };
        let ivtf = if self.vtf > 0.0 { 1.0 / self.vtf } else { 0.0 };
        let sl_tf = if self.itf > 0.0 { 0.0 } else { 1.0 };
        let r_if = transport.ifi * sg_if * iitf;
        let dr_if_dvbe_eff = transport.gfi * sg_if * iitf;
        let m_if = r_if / (1.0 + r_if);
        let dm_if_dvbe_eff = dr_if_dvbe_eff / (1.0 + r_if).powi(2);
        let (bc_exp, bc_exp_slope) = Self::limited_exp(vbc_eff * ivtf / 1.44);
        let dbc_exp_dvbc_eff = bc_exp_slope * ivtf / 1.44;
        let tf_base = self.tf * (1.0 + self.qtf * transport.q1);
        let tf_mod = 1.0 + self.xtf * bc_exp * (sl_tf + m_if * m_if) * sg_if;
        let tff = tf_base * tf_mod;
        let dtff_dvbe_eff = self.tf * self.qtf * transport.dq1_dvbe_eff * tf_mod
            + tf_base * self.xtf * bc_exp * (2.0 * m_if * dm_if_dvbe_eff) * sg_if;
        let dtff_dvbc_eff = self.tf * self.qtf * transport.dq1_dvbc_eff * tf_mod
            + tf_base * self.xtf * dbc_exp_dvbc_eff * (sl_tf + m_if * m_if) * sg_if;

        let mut qbe = BjtChargeBranch {
            pos_internal: Some(IDX_VBI),
            neg_internal: Some(IDX_VEI),
            ..Default::default()
        };
        qbe.charge = self.cje * wbe * qdbe + tff * transport.ifi / transport.qb.max(1e-12);
        let qbe_tff = transport.ifi / transport.qb.max(1e-12);
        let qbe_ifi = tff / transport.qb.max(1e-12);
        let qbe_qb = -transport.ifi * tff / transport.qb.max(1e-12).powi(2);
        let dqbe_dvbe_eff = self.cje * wbe * dqdbe_dvbe_eff
            + qbe_tff * dtff_dvbe_eff
            + qbe_ifi * transport.gfi
            + qbe_qb * transport.dqb_dvbe_eff;
        let dqbe_dvbc_eff = qbe_tff * dtff_dvbc_eff + qbe_qb * transport.dqb_dvbc_eff;
        qbe.d_internal[IDX_VBI] = p * (dqbe_dvbe_eff + dqbe_dvbc_eff);
        qbe.d_internal[IDX_VEI] = -p * dqbe_dvbe_eff;
        qbe.d_internal[IDX_VCI] = -p * dqbe_dvbc_eff;
        branches[0] = qbe;

        if self.cje > 0.0 && wbe != 1.0 {
            let mut qbex = BjtChargeBranch {
                pos_internal: Some(IDX_VBX),
                neg_internal: Some(IDX_VEI),
                ..Default::default()
            };
            qbex.charge = self.cje * (1.0 - wbe) * qdbex;
            let dq_dvbex_eff = self.cje * (1.0 - wbe) * dqdbex_dvbex_eff;
            qbex.d_internal[IDX_VBX] = p * dq_dvbex_eff;
            qbex.d_internal[IDX_VEI] = -p * dq_dvbex_eff;
            branches[1] = qbex;
        }

        let mut qbc = BjtChargeBranch {
            pos_internal: Some(IDX_VBI),
            neg_internal: Some(IDX_VCI),
            ..Default::default()
        };
        qbc.charge = self.cjc * qdbc + self.tr * transport.iri + self.qco * epi.kbci;
        let dqbc_dvbc_eff = self.cjc * dqdbc_dvbc_eff + self.tr * transport.gri;
        qbc.d_internal[IDX_VBI] = p * dqbc_dvbc_eff + self.qco * epi.d_kbci[IDX_VBI];
        qbc.d_internal[IDX_VCI] = -p * dqbc_dvbc_eff + self.qco * epi.d_kbci[IDX_VCI];
        branches[2] = qbc;

        if self.qco > 0.0 {
            let mut qbcx = BjtChargeBranch {
                pos_internal: Some(IDX_VBI),
                neg_internal: Some(IDX_VCX),
                ..Default::default()
            };
            qbcx.charge = self.qco * epi.kbcx;
            for idx in 0..INTERNAL_DIM {
                qbcx.d_internal[idx] = self.qco * epi.d_kbcx[idx];
            }
            branches[3] = qbcx;
        }

        if self.cjep > 0.0 || self.tr != 0.0 {
            let mut qbep = BjtChargeBranch {
                pos_internal: Some(IDX_VBX),
                neg_internal: Some(IDX_VBP),
                ..Default::default()
            };
            qbep.charge = self.cjep * qdbep + self.tr * parasitic.ifp;
            let dq_dep = self.cjep * dqdbep_dvbep_eff;
            qbep.d_internal[IDX_VBX] = p * dq_dep + self.tr * parasitic.d_ifp[IDX_VBX];
            qbep.d_internal[IDX_VBP] = -p * dq_dep + self.tr * parasitic.d_ifp[IDX_VBP];
            qbep.d_internal[IDX_VBI] = self.tr * parasitic.d_ifp[IDX_VBI];
            qbep.d_internal[IDX_VCI] = self.tr * parasitic.d_ifp[IDX_VCI];
            branches[4] = qbep;
        }

        if self.cbeo > 0.0 {
            let mut qbeo = BjtChargeBranch {
                pos_external: Some(EXT_B),
                neg_external: Some(EXT_E),
                ..Default::default()
            };
            qbeo.charge = self.cbeo * vbeo_eff;
            qbeo.d_external[EXT_B] = p * self.cbeo;
            qbeo.d_external[EXT_E] = -p * self.cbeo;
            branches[5] = qbeo;
        }

        if self.cbco > 0.0 {
            let mut qbco = BjtChargeBranch {
                pos_external: Some(EXT_B),
                neg_external: Some(EXT_C),
                ..Default::default()
            };
            qbco.charge = self.cbco * vbco_eff;
            qbco.d_external[EXT_B] = p * self.cbco;
            qbco.d_external[EXT_C] = -p * self.cbco;
            branches[6] = qbco;
        }

        if self.cjcp > 0.0 || self.ccso > 0.0 {
            let mut qbcp = BjtChargeBranch {
                pos_internal: Some(IDX_VSI),
                neg_internal: Some(IDX_VBP),
                ..Default::default()
            };
            qbcp.charge = self.cjcp * qdbcp + self.ccso * vbcp_eff;
            let dq_dvbcp_eff = self.cjcp * dqdbcp_dvbcp_eff + self.ccso;
            qbcp.d_internal[IDX_VSI] = p * dq_dvbcp_eff;
            qbcp.d_internal[IDX_VBP] = -p * dq_dvbcp_eff;
            branches[7] = qbcp;
        }

        let cth = self.thermal_capacitance();
        if cth > 0.0 {
            let mut qcth = BjtChargeBranch {
                pos_internal: Some(IDX_VRTH),
                ..Default::default()
            };
            qcth.charge = cth * vrth;
            qcth.d_internal[IDX_VRTH] = cth;
            branches[IDX_QCTH] = qcth;
        }

        if self.td > 0.0 {
            let mut qxf1 = BjtChargeBranch {
                pos_internal: Some(IDX_VXF1),
                ..Default::default()
            };
            qxf1.charge = self.td * vxf1;
            qxf1.d_internal[IDX_VXF1] = self.td;
            branches[IDX_QXF1] = qxf1;

            let mut qxf2 = BjtChargeBranch {
                pos_internal: Some(IDX_VXF2),
                ..Default::default()
            };
            qxf2.charge = self.td * vxf2 / 3.0;
            qxf2.d_internal[IDX_VXF2] = self.td / 3.0;
            branches[IDX_QXF2] = qxf2;
        }

        branches
    }

    /// Evaluate the VBIC dynamic charge branches at an explicit bias,
    /// including the finite-difference d/dVrth charge column and the
    /// excess-phase transport sensitivity when self-heating is active. Shared
    /// by the reduced charge-snapshot path and the MNA-promoted device so
    /// both stamp identical physics.
    pub(super) fn vbic_dynamic_charge_state_at_bias(
        &self,
        external: [Value; EXTERNAL_DIM],
        internal: [Value; BJT_INTERNAL_STATE_DIM],
        cached_inputs: Option<BjtDynamicChargeInputs>,
    ) -> (
        [BjtChargeBranch; BJT_DYNAMIC_CHARGE_COUNT],
        BjtDynamicChargeInputs,
        Value,
    ) {
        // The branch builder reads only the bias voltages from the reduction.
        let bias = BjtDynamicReduction {
            external_voltages: external,
            internal_voltages: internal,
            ..Default::default()
        };
        let vrth = internal[IDX_VRTH];

        if !self.self_heating_enabled() {
            let inputs =
                cached_inputs.unwrap_or_else(|| self.dynamic_charge_inputs(external, internal));
            let branches = self.dynamic_charge_branches_from_inputs(&bias, inputs);
            return (branches, inputs, 0.0);
        }

        let h = self.thermal_derivative_step(vrth);
        let denom = 2.0 * h;

        let mut plus_internal = internal;
        plus_internal[IDX_VRTH] = vrth + h;
        let mut minus_internal = internal;
        minus_internal[IDX_VRTH] = vrth - h;

        let base_inputs = cached_inputs.unwrap_or_else(|| {
            self.with_temperature_variant(vrth, |model| {
                model.dynamic_charge_inputs(external, internal)
            })
        });
        let plus_inputs = self.with_temperature_variant(vrth + h, |model| {
            model.dynamic_charge_inputs(external, plus_internal)
        });
        let minus_inputs = self.with_temperature_variant(vrth - h, |model| {
            model.dynamic_charge_inputs(external, minus_internal)
        });

        let d_itzf_d_vrth = if self.td > 0.0 {
            (plus_inputs.transport.itzf - minus_inputs.transport.itzf) / denom
        } else {
            0.0
        };

        let mut branches = self.with_temperature_variant(vrth, |model| {
            model.dynamic_charge_branches_from_inputs(&bias, base_inputs)
        });
        let mut plus_bias = bias;
        plus_bias.internal_voltages = plus_internal;
        let mut minus_bias = bias;
        minus_bias.internal_voltages = minus_internal;
        let plus_branches = self.with_temperature_variant(vrth + h, |model| {
            model.dynamic_charge_branches_from_inputs(&plus_bias, plus_inputs)
        });
        let minus_branches = self.with_temperature_variant(vrth - h, |model| {
            model.dynamic_charge_branches_from_inputs(&minus_bias, minus_inputs)
        });
        for branch_idx in 0..BJT_DYNAMIC_CHARGE_COUNT {
            branches[branch_idx].d_internal[IDX_VRTH] =
                (plus_branches[branch_idx].charge - minus_branches[branch_idx].charge) / denom;
        }

        (branches, base_inputs, d_itzf_d_vrth)
    }

    pub(super) fn charge_snapshot_from_base(
        &self,
        base: BjtReducedLinearization,
    ) -> BjtChargeSnapshot {
        let template = self.dynamic_reduction_template(base);
        if !self.uses_vbic_dynamic_charges() {
            let branches = self.legacy_dynamic_charge_branches(&template);
            return BjtChargeSnapshot {
                reduction: template,
                branches,
            };
        }

        let vrth = template.internal_voltages[IDX_VRTH];
        let inputs = base.cached_dynamic_inputs.unwrap_or_else(|| {
            if self.self_heating_enabled() {
                self.with_temperature_variant(vrth, |model| {
                    model.dynamic_charge_inputs(
                        template.external_voltages,
                        template.internal_voltages,
                    )
                })
            } else {
                self.dynamic_charge_inputs(template.external_voltages, template.internal_voltages)
            }
        });

        // The snapshot path seeds the excess-phase states at their DC fixed
        // point (vxf1 = vxf2 = Itzf) before evaluating the xf charges.
        let mut charge_internal = template.internal_voltages;
        if self.td > 0.0 {
            charge_internal[IDX_VXF1] = inputs.transport.itzf;
            charge_internal[IDX_VXF2] = inputs.transport.itzf;
        }
        let (branches, _, d_itzf_d_vrth) = self.vbic_dynamic_charge_state_at_bias(
            template.external_voltages,
            charge_internal,
            Some(inputs),
        );
        let reduction =
            self.build_dynamic_reduction_from_transport(template, inputs.transport, d_itzf_d_vrth);

        BjtChargeSnapshot {
            reduction,
            branches,
        }
    }

    pub(super) fn legacy_dynamic_charge_branches(
        &self,
        reduction: &BjtDynamicReduction,
    ) -> [BjtChargeBranch; BJT_DYNAMIC_CHARGE_COUNT] {
        let mut branches = [BjtChargeBranch::default(); BJT_DYNAMIC_CHARGE_COUNT];
        let internal = &reduction.internal_voltages;
        let vbe = internal[IDX_VBI] - internal[IDX_VEI];
        let vbc = internal[IDX_VBI] - internal[IDX_VCI];
        let collector_terminal = self.legacy_charge_collector_terminal();
        let base_terminal = self.legacy_charge_base_terminal();
        let emitter_terminal = self.legacy_charge_emitter_terminal();
        let substrate_terminal = self.legacy_charge_substrate_terminal();
        let substrate_connection_terminal = self.legacy_charge_substrate_connection_terminal();
        let terminal_voltage = |terminal: (Option<usize>, Option<usize>)| -> Value {
            if let Some(idx) = terminal.0 {
                reduction.internal_voltages[idx]
            } else if let Some(idx) = terminal.1 {
                reduction.external_voltages[idx]
            } else {
                0.0
            }
        };
        let vbx = reduction.external_voltages[EXT_B] - terminal_voltage(collector_terminal);
        let vcs =
            terminal_voltage(substrate_connection_terminal) - terminal_voltage(substrate_terminal);
        let charges = self.legacy_transient_charge_state_with_vbx(vbe, vbc, vbx, vcs);

        if charges.qbe.is_finite()
            && charges.capbe.is_finite()
            && charges.capbe_vbc.is_finite()
            && (charges.capbe > 0.0 || charges.capbe_vbc != 0.0)
        {
            let mut d_internal = [0.0; BJT_INTERNAL_STATE_DIM];
            d_internal[IDX_VBI] = charges.capbe + charges.capbe_vbc;
            d_internal[IDX_VCI] = -charges.capbe_vbc;
            d_internal[IDX_VEI] = -charges.capbe;
            branches[0] =
                Self::charge_branch(charges.qbe, d_internal, base_terminal, emitter_terminal);
        }

        if charges.qbc.is_finite() && charges.capbc.is_finite() && charges.capbc > 0.0 {
            let mut d_internal = [0.0; BJT_INTERNAL_STATE_DIM];
            d_internal[IDX_VBI] = charges.capbc;
            d_internal[IDX_VCI] = -charges.capbc;
            branches[2] =
                Self::charge_branch(charges.qbc, d_internal, base_terminal, collector_terminal);
        }

        if charges.qbx.is_finite() && charges.capbx.is_finite() && charges.capbx > 0.0 {
            let mut branch = BjtChargeBranch {
                charge: charges.qbx,
                pos_external: Some(EXT_B),
                neg_internal: collector_terminal.0,
                neg_external: collector_terminal.1,
                ..Default::default()
            };
            branch.d_external[EXT_B] = charges.capbx;
            if let Some(idx) = collector_terminal.0 {
                branch.d_internal[idx] -= charges.capbx;
            } else if let Some(idx) = collector_terminal.1 {
                branch.d_external[idx] -= charges.capbx;
            }
            branches[3] = branch;
        }

        if charges.qcs.is_finite() && charges.capcs.is_finite() && charges.capcs > 0.0 {
            let mut branch = BjtChargeBranch {
                charge: charges.qcs,
                pos_internal: substrate_connection_terminal.0,
                pos_external: substrate_connection_terminal.1,
                neg_internal: substrate_terminal.0,
                neg_external: substrate_terminal.1,
                ..Default::default()
            };
            if let Some(idx) = substrate_connection_terminal.0 {
                branch.d_internal[idx] += charges.capcs;
            } else if let Some(idx) = substrate_connection_terminal.1 {
                branch.d_external[idx] += charges.capcs;
            }
            if let Some(idx) = substrate_terminal.0 {
                branch.d_internal[idx] -= charges.capcs;
            } else if let Some(idx) = substrate_terminal.1 {
                branch.d_external[idx] -= charges.capcs;
            }
            branches[7] = branch;
        }

        branches
    }

    #[inline]
    pub(super) fn charge_branch(
        charge: Value,
        d_internal: [Value; BJT_INTERNAL_STATE_DIM],
        pos: (Option<usize>, Option<usize>),
        neg: (Option<usize>, Option<usize>),
    ) -> BjtChargeBranch {
        BjtChargeBranch {
            charge,
            d_internal,
            pos_internal: pos.0,
            pos_external: pos.1,
            neg_internal: neg.0,
            neg_external: neg.1,
            ..Default::default()
        }
    }

    #[inline]
    pub(super) fn legacy_charge_collector_terminal(&self) -> (Option<usize>, Option<usize>) {
        if Self::series_active(self.rci) {
            (Some(IDX_VCI), None)
        } else if Self::series_active(self.rcx) {
            (Some(IDX_VCX), None)
        } else {
            (None, Some(EXT_C))
        }
    }

    #[inline]
    pub(super) fn legacy_charge_base_terminal(&self) -> (Option<usize>, Option<usize>) {
        if Self::series_active(self.rbi) {
            (Some(IDX_VBI), None)
        } else if Self::series_active(self.rbx) {
            (Some(IDX_VBX), None)
        } else {
            (None, Some(EXT_B))
        }
    }

    #[inline]
    pub(super) fn legacy_charge_emitter_terminal(&self) -> (Option<usize>, Option<usize>) {
        if Self::series_active(self.re) {
            (Some(IDX_VEI), None)
        } else {
            (None, Some(EXT_E))
        }
    }

    #[inline]
    pub(super) fn legacy_charge_substrate_terminal(&self) -> (Option<usize>, Option<usize>) {
        if Self::series_active(self.rs) {
            (Some(IDX_VSI), None)
        } else {
            (None, Some(EXT_S))
        }
    }

    #[inline]
    pub(super) fn legacy_charge_substrate_connection_terminal(
        &self,
    ) -> (Option<usize>, Option<usize>) {
        match self.substrate_topology {
            BjtSubstrateTopology::Vertical => self.legacy_charge_collector_terminal(),
            BjtSubstrateTopology::Lateral => self.legacy_charge_base_terminal(),
        }
    }

    pub(super) fn dynamic_reduction_for_internal_state(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        internal: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> BjtDynamicReduction {
        let static_internal = [
            internal[IDX_VCX],
            internal[IDX_VCI],
            internal[IDX_VBX],
            internal[IDX_VBI],
            internal[IDX_VEI],
            internal[IDX_VBP],
            internal[IDX_VSI],
            internal[IDX_VRTH],
        ];
        let state = self.intrinsic_state_from_internal_vector(static_internal);
        let eval = self.evaluate_state(
            vc, vb, ve, vs, state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
            state.vsi, state.vrth,
        );
        let base = self.reduced_linearization_from_state_and_eval(state, eval, vc, vb, ve, vs);
        let mut reduction = self.dynamic_reduction_template(base);
        reduction.internal_voltages = internal;

        if !self.uses_vbic_dynamic_charges() {
            return reduction;
        }

        let vrth = internal[IDX_VRTH];
        if !self.self_heating_enabled() {
            let inputs = self
                .dynamic_charge_inputs(reduction.external_voltages, reduction.internal_voltages);
            let mut reduction =
                self.build_dynamic_reduction_from_transport(reduction, inputs.transport, 0.0);
            reduction.internal_voltages[IDX_VXF1] = internal[IDX_VXF1];
            reduction.internal_voltages[IDX_VXF2] = internal[IDX_VXF2];
            return reduction;
        }

        let h = self.thermal_derivative_step(vrth);
        let denom = 2.0 * h;

        let mut plus_internal = internal;
        plus_internal[IDX_VRTH] = vrth + h;
        let mut minus_internal = internal;
        minus_internal[IDX_VRTH] = vrth - h;

        let base_inputs = self.with_temperature_variant(vrth, |model| {
            model.dynamic_charge_inputs(reduction.external_voltages, internal)
        });
        let plus_inputs = self.with_temperature_variant(vrth + h, |model| {
            model.dynamic_charge_inputs(reduction.external_voltages, plus_internal)
        });
        let minus_inputs = self.with_temperature_variant(vrth - h, |model| {
            model.dynamic_charge_inputs(reduction.external_voltages, minus_internal)
        });

        let d_itzf_d_vrth = if self.td > 0.0 {
            (plus_inputs.transport.itzf - minus_inputs.transport.itzf) / denom
        } else {
            0.0
        };
        let mut reduction = self.build_dynamic_reduction_from_transport(
            reduction,
            base_inputs.transport,
            d_itzf_d_vrth,
        );
        reduction.internal_voltages[IDX_VXF1] = internal[IDX_VXF1];
        reduction.internal_voltages[IDX_VXF2] = internal[IDX_VXF2];
        reduction
    }

    pub(crate) fn charge_snapshot_for_dynamic_state(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        internal: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> BjtChargeSnapshot {
        let reduction = self.dynamic_reduction_for_internal_state(vc, vb, ve, vs, internal);

        if !self.uses_vbic_dynamic_charges() {
            let branches = self.legacy_dynamic_charge_branches(&reduction);
            return BjtChargeSnapshot {
                reduction,
                branches,
            };
        }

        if !self.self_heating_enabled() {
            let inputs = self
                .dynamic_charge_inputs(reduction.external_voltages, reduction.internal_voltages);
            return BjtChargeSnapshot {
                reduction,
                branches: self.dynamic_charge_branches_from_inputs(&reduction, inputs),
            };
        }

        let vrth = internal[IDX_VRTH];
        let h = self.thermal_derivative_step(vrth);
        let denom = 2.0 * h;
        let mut branches = self.with_temperature_variant(vrth, |model| {
            let base_inputs = model
                .dynamic_charge_inputs(reduction.external_voltages, reduction.internal_voltages);
            model.dynamic_charge_branches_from_inputs(&reduction, base_inputs)
        });

        let mut plus_reduction = reduction;
        plus_reduction.internal_voltages[IDX_VRTH] = vrth + h;
        let mut minus_reduction = reduction;
        minus_reduction.internal_voltages[IDX_VRTH] = vrth - h;
        let plus_branches = self.with_temperature_variant(vrth + h, |model| {
            let plus_inputs = model.dynamic_charge_inputs(
                plus_reduction.external_voltages,
                plus_reduction.internal_voltages,
            );
            model.dynamic_charge_branches_from_inputs(&plus_reduction, plus_inputs)
        });
        let minus_branches = self.with_temperature_variant(vrth - h, |model| {
            let minus_inputs = model.dynamic_charge_inputs(
                minus_reduction.external_voltages,
                minus_reduction.internal_voltages,
            );
            model.dynamic_charge_branches_from_inputs(&minus_reduction, minus_inputs)
        });
        for branch_idx in 0..BJT_DYNAMIC_CHARGE_COUNT {
            branches[branch_idx].d_internal[IDX_VRTH] =
                (plus_branches[branch_idx].charge - minus_branches[branch_idx].charge) / denom;
        }

        BjtChargeSnapshot {
            reduction,
            branches,
        }
    }

    pub(crate) fn charge_snapshot(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> BjtChargeSnapshot {
        if self.charge_snapshot_cache_valid.get()
            && self.cache_matches_external_biases(vc, vb, ve, vs)
        {
            return self.charge_snapshot_cache.get();
        }

        let snapshot = self.charge_snapshot_from_base(self.reduced_linearization(vc, vb, ve, vs));
        if self.cache_matches_external_biases(vc, vb, ve, vs) {
            self.charge_snapshot_cache.set(snapshot);
            self.charge_snapshot_cache_valid.set(true);
        }
        snapshot
    }

    pub(crate) fn dynamic_internal_state_seed(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> [Value; BJT_INTERNAL_STATE_DIM] {
        if self.cache_matches_external_biases(vc, vb, ve, vs)
            && self.reduced_linearization_cache_valid.get()
        {
            let static_internal = self.internal_state_vector();
            let mut internal = [0.0; BJT_INTERNAL_STATE_DIM];
            internal[..INTERNAL_DIM].copy_from_slice(&static_internal);

            if self.uses_vbic_dynamic_charges() {
                let inputs = if self.self_heating_enabled() {
                    self.with_temperature_variant(static_internal[IDX_VRTH], |model| {
                        model.dynamic_charge_inputs([vc, vb, ve, vs], internal)
                    })
                } else {
                    self.dynamic_charge_inputs([vc, vb, ve, vs], internal)
                };
                internal[IDX_VXF1] = inputs.transport.itzf;
                internal[IDX_VXF2] = inputs.transport.itzf;
            }

            return internal;
        }

        self.charge_snapshot(vc, vb, ve, vs)
            .reduction
            .internal_voltages
    }

    pub(crate) fn external_terminal_currents_at_bias(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> [Value; EXTERNAL_DIM] {
        if self.cache_matches_external_biases(vc, vb, ve, vs) {
            return [self.ic, self.ib, self.ie, self.isub];
        }

        let state = self.intrinsic_state_for_biases(vc, vb, ve, vs);
        let eval = self.evaluate_state(
            vc, vb, ve, vs, state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
            state.vsi, state.vrth,
        );
        let terminal = self.external_terminal_branches(eval);
        [
            terminal[EXT_C].current,
            terminal[EXT_B].current,
            terminal[EXT_E].current,
            terminal[EXT_S].current,
        ]
    }
}
