//! BJT dynamic charge reduction, charge snapshots, and private noise ports.

use super::*;

/// A thermal source on the private GP network. Endpoints use the same
/// internal/external coordinates as the charge and residual operators.
pub(crate) struct LegacyBjtThermalNoise {
    pub mechanism: &'static str,
    pub conductance: Value,
    pub terminals: [(Option<usize>, Option<usize>); 2],
}

impl Bjt {
    /// Convert model-oriented VBIC charge and its voltage gradients to a
    /// physical branch current. Thermal and delay states are polarity independent.
    #[inline]
    pub(crate) fn charge_branch_polarity(&self, branch: usize) -> Value {
        if self.uses_vbic_dynamic_charges() && branch < IDX_QCTH {
            self.polarity()
        } else {
            1.0
        }
    }

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

        let (exp_bci, dexp_bci_darg) = self.vbic_general_exp(vbci_eff / vt);
        let (exp_bcx, dexp_bcx_darg) = self.vbic_general_exp(vbcx_eff / vt);
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
    ) -> [BjtCurrentBranch; VBIC_DELAY_BRANCH_COUNT] {
        let mut branches = [BjtCurrentBranch::default(); VBIC_DELAY_BRANCH_COUNT];
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
        branches[3] = self.vbic_delayed_avalanche_branch(reduction, &delta_iciei);

        branches
    }

    /// VBIC 1.3 uses delayed Itxf in Igc. ngspice's older VBIC keeps
    /// static Itzf there, so its BC branch needs no excess-phase correction.
    fn vbic_delayed_avalanche_branch(
        &self,
        reduction: &BjtDynamicReduction,
        forward: &BjtCurrentBranch,
    ) -> BjtCurrentBranch {
        if !self.vbic_13 || self.avc1 <= 0.0 || self.td <= 0.0 {
            return BjtCurrentBranch::default();
        }
        let internal = reduction.internal_voltages;
        let p = self.polarity();
        let voltage = p * (internal[IDX_VBI] - internal[IDX_VCI]);
        let factor_at = |model: &Self| {
            model.vbic13_avalanche_factor(voltage, model.vjc, model.mjc, model.avc1, model.avc2)
        };
        let (factor, slope, thermal_slope) = if self.thermal_model_enabled() {
            let rise = internal[IDX_VRTH];
            let h = self.thermal_derivative_step(rise);
            let (factor, slope) = self.with_temperature_variant(rise, factor_at);
            let plus = self
                .with_temperature_derivative_variant(rise + h, rise, factor_at)
                .0;
            let minus = self
                .with_temperature_derivative_variant(rise - h, rise, factor_at)
                .0;
            (factor, slope, (plus - minus) / (2.0 * h))
        } else {
            let (factor, slope) = factor_at(self);
            (factor, slope, 0.0)
        };
        // Current is physical BI -> CI; residual incidence is incoming current.
        let mut branch = BjtCurrentBranch {
            current: -factor * forward.current,
            d_internal: forward.d_internal.map(|value| -factor * value),
            pos_internal: Some(IDX_VCI),
            neg_internal: Some(IDX_VBI),
            ..Default::default()
        };
        branch.d_internal[IDX_VBI] -= p * slope * forward.current;
        branch.d_internal[IDX_VCI] += p * slope * forward.current;
        branch.d_internal[IDX_VRTH] -= thermal_slope * forward.current;
        branch
    }

    pub(crate) fn vbic_delay_static_thermal_branch(
        &self,
        reduction: &BjtDynamicReduction,
    ) -> BjtCurrentBranch {
        // VBIC 1.3 heats from delayed Itxf. The older ngspice VBIC
        // evaluates Ith from instantaneous Itzf even when TD is enabled
        // (vbicload.c); only its electrical collector current is delayed.
        if !self.vbic_13
            || self.td <= 0.0
            || !self.thermal_model_enabled()
            || !self.vbic_heat_generation
        {
            return BjtCurrentBranch::default();
        }

        let internal = reduction.internal_voltages;
        let delayed = self.vbic_delay_static_branches(reduction);
        let mut branch = BjtCurrentBranch {
            pos_internal: Some(IDX_VRTH),
            ..Default::default()
        };
        for (current, pos, neg) in [
            (delayed[0], IDX_VCI, IDX_VEI),
            (delayed[3], IDX_VBI, IDX_VCI),
        ] {
            let voltage = internal[pos] - internal[neg];
            branch.current -= current.current * voltage;
            for (derivative, current_derivative) in
                branch.d_internal.iter_mut().zip(current.d_internal)
            {
                *derivative -= current_derivative * voltage;
            }
            branch.d_internal[pos] -= current.current;
            branch.d_internal[neg] += current.current;
        }

        branch
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
        reduction.g_ii[IDX_VXF1] = [0.0; BJT_INTERNAL_STATE_DIM];
        reduction.g_ii[IDX_VXF2] = [0.0; BJT_INTERNAL_STATE_DIM];
        for branch in self.vbic_delay_static_branches(&reduction) {
            branch.accumulate_derivatives(
                &mut reduction.g_ii,
                &mut reduction.g_ie,
                &mut reduction.g_ei,
                &mut reduction.g_ee,
            );
        }

        reduction
    }

    fn vbic_transit_time_modulation(
        &self,
        transport: TransportChargeState,
        vbc_eff: Value,
    ) -> (Value, Value, Value) {
        let sg_if = if transport.ifi > 0.0 { 1.0 } else { 0.0 };
        // Transport currents already include AREA*M; ITF is a nominal model
        // current, so its ratio must use the same instance scaling.
        let iitf = if self.itf > 0.0 {
            1.0 / self.itf / self.instance_scale()
        } else {
            0.0
        };
        let ivtf = if self.vtf > 0.0 { 1.0 / self.vtf } else { 0.0 };
        let sl_tf = if self.itf > 0.0 { 0.0 } else { 1.0 };
        let r_if = transport.ifi * sg_if * iitf;
        let dr_if_dvbe_eff = transport.gfi * sg_if * iitf;
        let m_if = r_if / (1.0 + r_if);
        let dm_if_dvbe_eff = dr_if_dvbe_eff / (1.0 + r_if).powi(2);
        let (bc_exp, bc_exp_slope) = self.vbic_general_exp(vbc_eff * ivtf / 1.44);
        let dbc_exp_dvbc_eff = bc_exp_slope * ivtf / 1.44;
        (
            1.0 + self.xtf * bc_exp * (sl_tf + m_if * m_if) * sg_if,
            self.xtf * bc_exp * (2.0 * m_if * dm_if_dvbe_eff) * sg_if,
            self.xtf * dbc_exp_dvbc_eff * (sl_tf + m_if * m_if) * sg_if,
        )
    }

    fn vbic_dynamic_early_thermal_derivatives(
        &self,
        internal: [Value; BJT_INTERNAL_STATE_DIM],
        transport: TransportChargeState,
    ) -> (Value, Value) {
        if !self.vbic_13 || (self.tcvef == 0.0 && self.tcver == 0.0) {
            return (0.0, 0.0);
        }
        let vrth = internal[IDX_VRTH];
        let temperature_slope = self
            .mapped_temperature(self.requested_temperature() + vrth)
            .1;
        self.with_temperature_variant(vrth, |model| {
            let vbe_eff = model.polarity() * (internal[IDX_VBI] - internal[IDX_VEI]);
            let vbc_eff = model.polarity() * (internal[IDX_VBI] - internal[IDX_VCI]);
            let early = model.vbic_early_thermal_derivatives(
                vbe_eff,
                vbc_eff,
                transport,
                temperature_slope,
            );
            let tf_mod = model.vbic_transit_time_modulation(transport, vbc_eff).0;
            (early.itzf, model.tf * tf_mod * early.forward_charge)
        })
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

        let (tf_mod, dtf_mod_dvbe_eff, dtf_mod_dvbc_eff) =
            self.vbic_transit_time_modulation(transport, vbc_eff);
        let tf_base = self.tf * (1.0 + self.qtf * transport.q1);
        let tff = tf_base * tf_mod;
        let dtff_dvbe_eff =
            self.tf * self.qtf * transport.dq1_dvbe_eff * tf_mod + tf_base * dtf_mod_dvbe_eff;
        let dtff_dvbc_eff =
            self.tf * self.qtf * transport.dq1_dvbc_eff * tf_mod + tf_base * dtf_mod_dvbc_eff;

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

        if !self.vbic_three_terminal && (self.cjcp > 0.0 || self.ccso > 0.0) {
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
    /// including the d/dVrth charge column and the
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

        if !self.thermal_model_enabled() {
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
        let plus_inputs = self.with_temperature_derivative_variant(vrth + h, vrth, |model| {
            model.dynamic_charge_inputs(external, plus_internal)
        });
        let minus_inputs = self.with_temperature_derivative_variant(vrth - h, vrth, |model| {
            model.dynamic_charge_inputs(external, minus_internal)
        });

        let (early_itzf, early_qbe) =
            self.vbic_dynamic_early_thermal_derivatives(internal, base_inputs.transport);
        let d_itzf_d_vrth = if self.td > 0.0 {
            (plus_inputs.transport.itzf - minus_inputs.transport.itzf) / denom + early_itzf
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
        let plus_branches = self.with_temperature_derivative_variant(vrth + h, vrth, |model| {
            model.dynamic_charge_branches_from_inputs(&plus_bias, plus_inputs)
        });
        let minus_branches = self.with_temperature_derivative_variant(vrth - h, vrth, |model| {
            model.dynamic_charge_branches_from_inputs(&minus_bias, minus_inputs)
        });
        for branch_idx in 0..BJT_DYNAMIC_CHARGE_COUNT {
            branches[branch_idx].d_internal[IDX_VRTH] =
                (plus_branches[branch_idx].charge - minus_branches[branch_idx].charge) / denom;
        }

        branches[0].d_internal[IDX_VRTH] += early_qbe;
        (branches, base_inputs, d_itzf_d_vrth)
    }

    pub(super) fn dynamic_charge_branches_at_bias(
        &self,
        external: [Value; EXTERNAL_DIM],
        internal: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> [BjtChargeBranch; BJT_DYNAMIC_CHARGE_COUNT] {
        if self.uses_legacy_gummel_poon() {
            self.legacy_dynamic_charge_branches(external, internal)
        } else {
            self.vbic_dynamic_charge_state_at_bias(external, internal, None)
                .0
        }
    }

    pub(super) fn charge_snapshot_from_base(
        &self,
        base: BjtReducedLinearization,
    ) -> BjtChargeSnapshot {
        let mut template = self.dynamic_reduction_template(base);
        if !self.uses_vbic_dynamic_charges() {
            let branches = self.legacy_dynamic_charge_branches(
                template.external_voltages,
                template.internal_voltages,
            );
            return BjtChargeSnapshot {
                reduction: template,
                branches,
            };
        }

        let vrth = template.internal_voltages[IDX_VRTH];
        let inputs = base.cached_dynamic_inputs.unwrap_or_else(|| {
            if self.thermal_model_enabled() {
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
        template.internal_voltages = charge_internal;
        let reduction =
            self.build_dynamic_reduction_from_transport(template, inputs.transport, d_itzf_d_vrth);

        BjtChargeSnapshot {
            reduction,
            branches,
        }
    }

    /// The four-terminal reduction must retain the authored base when an
    /// external BC charge connects it to a privately solved collector. Moving
    /// RBX outside would require a fifth external terminal in that reduction.
    pub(crate) fn can_externalize_legacy_base_lead(&self) -> bool {
        !(Self::series_active(self.rci) && self.cjc != 0.0 && self.xcjc != 1.0)
    }

    fn legacy_external_bc_base_node(&self) -> Option<NodeId> {
        // Standard GP's collector lead is already a circuit node. Keep a
        // nonstandard private RCI extension in its existing reduced system.
        (self.uses_legacy_gummel_poon() && !Self::series_active(self.rci))
            .then_some(self.legacy_base_lead)
            .flatten()
            .map(|(node, _)| node)
    }

    pub(crate) fn legacy_external_bc_charge_nodes(&self) -> Option<[NodeId; 2]> {
        let base = self.legacy_external_bc_base_node()?;
        (self.cjc != 0.0 && self.xcjc != 1.0).then_some([base, self.node_collector])
    }

    pub(crate) fn legacy_external_bc_charge(
        &self,
        solution: &[Value],
    ) -> Option<BjtExternalBcCharge> {
        let nodes = self.legacy_external_bc_charge_nodes()?;
        let capacitance = self.cjc - self.cjc * self.xcjc;
        let voltage = |node: NodeId| {
            if node == 0 {
                0.0
            } else {
                solution.get(node - 1).copied().unwrap_or(Value::NAN)
            }
        };
        let voltage = voltage(nodes[0]) - voltage(nodes[1]);
        let (charge, slope) = self.vbic_depletion_charge_and_derivative(
            self.polarity() * voltage,
            self.vjc,
            self.mjc,
            self.fc,
            0.0,
        );
        Some(BjtExternalBcCharge {
            nodes,
            voltage,
            charge: self.polarity() * capacitance * charge,
            capacitance: capacitance * slope,
        })
    }

    pub(super) fn legacy_dynamic_charge_branches(
        &self,
        external: [Value; EXTERNAL_DIM],
        internal: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> [BjtChargeBranch; BJT_DYNAMIC_CHARGE_COUNT] {
        let mut branches = [BjtChargeBranch::default(); BJT_DYNAMIC_CHARGE_COUNT];
        let vbe = internal[IDX_VBI] - internal[IDX_VEI];
        let vbc = internal[IDX_VBI] - internal[IDX_VCI];
        let collector_terminal = self.legacy_charge_collector_terminal();
        let base_terminal = self.legacy_charge_base_terminal();
        let emitter_terminal = self.legacy_charge_emitter_terminal();
        let substrate_terminal = self.legacy_charge_substrate_terminal();
        let substrate_connection_terminal = self.legacy_charge_substrate_connection_terminal();
        let terminal_voltage = |terminal: (Option<usize>, Option<usize>)| -> Value {
            if let Some(idx) = terminal.0 {
                internal[idx]
            } else if let Some(idx) = terminal.1 {
                external[idx]
            } else {
                0.0
            }
        };
        let vbx = external[EXT_B] - terminal_voltage(collector_terminal);
        let vcs =
            terminal_voltage(substrate_connection_terminal) - terminal_voltage(substrate_terminal);
        let charges = self.legacy_transient_charge_state_with_vbx(vbe, vbc, vbx, vcs);

        // Storage topology belongs to the model, not the current bias. A zero
        // or negative local derivative still owns its accepted charge history;
        // invalid evaluations must reach the solver instead of deleting a branch.
        if self.tf != 0.0 || self.cje != 0.0 || self.cbeo != 0.0 {
            let mut d_internal = [0.0; BJT_INTERNAL_STATE_DIM];
            d_internal[IDX_VBI] = charges.capbe + charges.capbe_vbc;
            d_internal[IDX_VCI] = -charges.capbe_vbc;
            d_internal[IDX_VEI] = -charges.capbe;
            branches[0] =
                Self::charge_branch(charges.qbe, d_internal, base_terminal, emitter_terminal);
        }

        if self.tr != 0.0 || self.cjc * self.xcjc != 0.0 || self.cbco != 0.0 {
            let mut d_internal = [0.0; BJT_INTERNAL_STATE_DIM];
            d_internal[IDX_VBI] = charges.capbc;
            d_internal[IDX_VCI] = -charges.capbc;
            branches[2] =
                Self::charge_branch(charges.qbc, d_internal, base_terminal, collector_terminal);
        }

        if self.cjc * (1.0 - self.xcjc) != 0.0 && self.legacy_external_bc_base_node().is_none() {
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

        if self.cjcp != 0.0 {
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

        let nodes = self.external_terminal_nodes();
        for branch in &mut branches {
            let tied = match (branch.pos_external, branch.neg_external) {
                (Some(pos), Some(neg)) => nodes[pos] == nodes[neg],
                _ => matches!((branch.pos_internal, branch.neg_internal),
                    (Some(pos), Some(neg)) if pos == neg),
            };
            if tied {
                // No incidence means no charge enters the circuit. Remove
                // it before accumulating derivatives, while retaining any
                // branch separated by a real internal series resistance.
                *branch = BjtChargeBranch::default();
            }
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

    /// Frozen-time shot and flicker currents follow the private transient bias.
    pub(crate) fn legacy_noise_branch_currents_at_state(
        &self,
        v: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> (Value, Value, Value) {
        let (linearized, _) = self.linearize_currents_with_branches(
            v[IDX_VBI] - v[IDX_VEI],
            v[IDX_VBX] - v[IDX_VEI],
            v[IDX_VBI] - v[IDX_VCI],
        );
        (linearized.ic.abs(), linearized.ib.abs(), 0.0)
    }

    /// Native noise sources use the same physical intrinsic nodes as charge.
    pub(crate) fn legacy_noise_terminals(&self) -> [(Option<usize>, Option<usize>); 3] {
        [
            self.legacy_charge_collector_terminal(),
            self.legacy_charge_base_terminal(),
            self.legacy_charge_emitter_terminal(),
        ]
    }

    pub(crate) fn legacy_private_resistance_noise(
        &self,
        v: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> [LegacyBjtThermalNoise; 7] {
        let (linearized, _) = self.linearize_currents_with_branches(
            v[IDX_VBI] - v[IDX_VEI],
            v[IDX_VBX] - v[IDX_VEI],
            v[IDX_VBI] - v[IDX_VCI],
        );
        let constant = |resistance| {
            if Self::series_active(resistance) {
                self.guarded_series_resistance(resistance).recip()
            } else {
                0.0
            }
        };
        let collector_outer = if Self::series_active(self.rcx) {
            (Some(IDX_VCX), None)
        } else {
            (None, Some(EXT_C))
        };
        let base_outer = if Self::series_active(self.rbx) {
            (Some(IDX_VBX), None)
        } else {
            (None, Some(EXT_B))
        };
        let source = |mechanism, conductance, pos, neg| LegacyBjtThermalNoise {
            mechanism,
            conductance,
            terminals: [pos, neg],
        };
        [
            source(
                "RC",
                constant(self.rcx),
                (None, Some(EXT_C)),
                collector_outer,
            ),
            // RCI is an existing GP extension using the Kull epi branch. Its
            // self-conductance is supplied by that same physical branch law.
            source(
                "RC",
                self.irci_branch_with_self_conductance(v[IDX_VCX], v[IDX_VCI], v[IDX_VBI])
                    .1,
                collector_outer,
                self.legacy_charge_collector_terminal(),
            ),
            source("RB", constant(self.rbx), (None, Some(EXT_B)), base_outer),
            source(
                "RB",
                self.irbi_branch(linearized, v[IDX_VBX], v[IDX_VBI])
                    .d_internal[IDX_VBX],
                base_outer,
                self.legacy_charge_base_terminal(),
            ),
            source(
                "RE",
                constant(self.re),
                (None, Some(EXT_E)),
                self.legacy_charge_emitter_terminal(),
            ),
            source(
                "RS",
                constant(self.rs),
                (None, Some(EXT_S)),
                self.legacy_charge_substrate_terminal(),
            ),
            source(
                "RBP",
                if Self::series_active(self.rbp) {
                    self.parasitic_transport_state(
                        v[IDX_VBX], v[IDX_VBI], v[IDX_VCI], v[IDX_VBP], v[IDX_VSI],
                    )
                    .qbp / self.guarded_series_resistance(self.rbp)
                } else {
                    0.0
                },
                collector_outer,
                (Some(IDX_VBP), None),
            ),
        ]
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
            BjtNodeVoltages {
                vc,
                vb,
                ve,
                vs,
                vcx: state.vcx,
                vci: state.vci,
                vbx: state.vbx,
                vbi: state.vbi,
                vei: state.vei,
                vbp: state.vbp,
                vsi: state.vsi,
            },
            state.vrth,
        );
        let base = self.reduced_linearization_from_state_and_eval(state, eval, vc, vb, ve, vs);
        let mut reduction = self.dynamic_reduction_template(base);
        reduction.internal_voltages = internal;

        if !self.uses_vbic_dynamic_charges() {
            return reduction;
        }

        let vrth = internal[IDX_VRTH];
        if !self.thermal_model_enabled() {
            let inputs = self
                .dynamic_charge_inputs(reduction.external_voltages, reduction.internal_voltages);
            return self.build_dynamic_reduction_from_transport(reduction, inputs.transport, 0.0);
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
        let plus_inputs = self.with_temperature_derivative_variant(vrth + h, vrth, |model| {
            model.dynamic_charge_inputs(reduction.external_voltages, plus_internal)
        });
        let minus_inputs = self.with_temperature_derivative_variant(vrth - h, vrth, |model| {
            model.dynamic_charge_inputs(reduction.external_voltages, minus_internal)
        });

        let d_itzf_d_vrth = if self.td > 0.0 {
            (plus_inputs.transport.itzf - minus_inputs.transport.itzf) / denom
                + self
                    .vbic_dynamic_early_thermal_derivatives(internal, base_inputs.transport)
                    .0
        } else {
            0.0
        };
        self.build_dynamic_reduction_from_transport(reduction, base_inputs.transport, d_itzf_d_vrth)
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
            let branches = self.legacy_dynamic_charge_branches(
                reduction.external_voltages,
                reduction.internal_voltages,
            );
            return BjtChargeSnapshot {
                reduction,
                branches,
            };
        }

        let branches = self
            .vbic_dynamic_charge_state_at_bias(reduction.external_voltages, internal, None)
            .0;

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
                let inputs = if self.thermal_model_enabled() {
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
            BjtNodeVoltages {
                vc,
                vb,
                ve,
                vs,
                vcx: state.vcx,
                vci: state.vci,
                vbx: state.vbx,
                vbi: state.vbi,
                vei: state.vei,
                vbp: state.vbp,
                vsi: state.vsi,
            },
            state.vrth,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vbic_transit_time_charge_and_derivatives_scale_with_parallel_instances() {
        for level in [4.0, 11.0, 12.0] {
            let params = [
                ("LEVEL", level),
                ("IS", 1e-16),
                ("TF", 2e-9),
                ("QTF", 0.3),
                ("XTF", 2.0),
                ("ITF", 1e-4),
                ("VTF", 2.0),
                ("VEF", 5.0),
                ("VER", 3.0),
                ("IKF", 1e-4),
                ("IKR", 2e-4),
            ]
            .map(|(name, value)| (name.to_owned(), value))
            .into_iter()
            .collect();
            let unit = Bjt::new_npn("q".into(), 1, 2, 0).with_params(&params);
            let external = [0.6, 0.7, 0.0, 0.0];
            let internal = [0.6, 0.6, 0.7, 0.7, 0.0, 0.6, 0.0, 0.0, 0.0, 0.0];
            let expected = unit
                .vbic_dynamic_charge_state_at_bias(external, internal, None)
                .0[0];
            for (area, multiplicity) in [(1.0, 0.25), (1.0, 3.0), (2.0, 3.0)] {
                let scaled = unit
                    .clone()
                    .with_instance_params(&[("AREA".into(), area), ("M".into(), multiplicity)]);
                let actual = scaled
                    .vbic_dynamic_charge_state_at_bias(external, internal, None)
                    .0[0];
                for (index, (actual, expected)) in std::iter::once(actual.charge)
                    .chain(actual.d_internal)
                    .zip(std::iter::once(expected.charge).chain(expected.d_internal))
                    .enumerate()
                {
                    let normalized = actual / (area * multiplicity);
                    // Only the regular thermal column uses a difference of
                    // nearby charges; voltage partials are analytic.
                    let tolerance = if index == IDX_VRTH + 1 { 2e-7 } else { 1e-12 };
                    assert!(
                        (normalized - expected).abs() <= tolerance * expected.abs(),
                        "LEVEL={level} AREA={area} M={multiplicity}: {normalized:e} != {expected:e}"
                    );
                }
            }
        }
    }

    #[test]
    fn vbic13_early_cutoff_keeps_current_charge_and_delay_thermal_slopes_resolvable() {
        // With linear depletion and no rolloff, qb = 1 + Vj/VE and
        // Qbe = TF*(QTF*If + Itzf). These closed forms remain well conditioned
        // one floating-point step from VE=0, unlike a temperature difference.
        for level in [11.0, 12.0] {
            for qbm in [0.0, 1.0] {
                for reverse in [false, true] {
                    for factor in [1e-7, 1e-11, Value::EPSILON] {
                        let tc = (factor - 1.0) / 20.0;
                        let params = [
                            ("LEVEL", level),
                            ("IS", 1e-16),
                            ("XIS", 0.0),
                            ("XISR", 0.0),
                            ("DEAR", 0.0),
                            ("IBEI", 1e-18),
                            ("XII", 0.0),
                            ("MJE", 0.0),
                            ("MJC", 0.0),
                            ("VEF", if reverse { 0.0 } else { 5.0 }),
                            ("VER", if reverse { 5.0 } else { 0.0 }),
                            ("TCVEF", if reverse { 0.0 } else { tc }),
                            ("TCVER", if reverse { tc } else { 0.0 }),
                            ("QBM", qbm),
                            ("QTF", 0.3),
                            ("TF", 1e-9),
                            ("TD", 1e-9),
                            ("RTH", 1000.0),
                            ("CTH", 1e-12),
                            ("GMIN", 0.0),
                        ]
                        .map(|(name, value)| (name.to_owned(), value))
                        .into_iter()
                        .collect();
                        for (bjt, p) in [
                            (Bjt::new_npn("q".into(), 1, 2, 0), 1.0),
                            (Bjt::new_pnp("q".into(), 1, 2, 0), -1.0),
                        ] {
                            let bjt = bjt
                                .with_params(&params)
                                .with_instance_params(&[("M".into(), 3.0)]);
                            let external = [p * 0.5, p * 0.6, 0.0, 0.0];
                            let internal = [
                                p * 0.5,
                                p * 0.5,
                                p * 0.6,
                                p * 0.6,
                                0.0,
                                p * 0.5,
                                0.0,
                                20.0,
                                0.0,
                                0.0,
                            ];
                            let (expected_if, expected_transport, expected_qbe, expected_ibe) = bjt
                                .with_temperature_variant(20.0, |model| {
                                    let vbe = p * (internal[IDX_VBI] - internal[IDX_VEI]);
                                    let vbc = p * (internal[IDX_VBI] - internal[IDX_VCI]);
                                    let junction = if reverse { vbe } else { vbc };
                                    let ve = 5.0 * (1.0 + (model.temperature - model.tnom) * tc);
                                    assert!(ve > 0.0);
                                    let ratio = ve / (ve + junction);
                                    let dratio = 5.0 * tc * junction / (ve + junction).powi(2);
                                    let ifi = model.is * (vbe / model.vt).exp_m1();
                                    let iri = model.is * (vbc / model.vt).exp_m1();
                                    let dif = (ifi * model.ea - (ifi + model.is) * vbe)
                                        / (model.vt * model.temperature);
                                    let dir = (iri * model.ea - (iri + model.is) * vbc)
                                        / (model.vt * model.temperature);
                                    let forward = dif * ratio + ifi * dratio;
                                    let transport = (dif - dir) * ratio + (ifi - iri) * dratio;
                                    let qbe = model.tf * (model.qtf * dif + forward);
                                    let ibe = model.ibei
                                        * (model.eaie * (vbe / model.vt).exp_m1()
                                            - (vbe / model.vt).exp() * vbe)
                                        / (model.vt * model.temperature);
                                    (forward, transport, qbe, ibe)
                                });
                            let (branches, _, d_itzf) =
                                bjt.vbic_dynamic_charge_state_at_bias(external, internal, None);
                            let evaluated = bjt.evaluate_state(
                                BjtNodeVoltages {
                                    vc: external[0],
                                    vb: external[1],
                                    ve: 0.0,
                                    vs: 0.0,
                                    vcx: internal[IDX_VCX],
                                    vci: internal[IDX_VCI],
                                    vbx: internal[IDX_VBX],
                                    vbi: internal[IDX_VBI],
                                    vei: 0.0,
                                    vbp: internal[IDX_VBP],
                                    vsi: 0.0,
                                },
                                20.0,
                            );
                            for (name, actual, expected) in [
                                ("delay", d_itzf, expected_if),
                                (
                                    "transport",
                                    evaluated.iciei.d_internal[IDX_VRTH],
                                    p * expected_transport,
                                ),
                                ("base", evaluated.ibe.d_internal[IDX_VRTH], p * expected_ibe),
                                ("charge", branches[0].d_internal[IDX_VRTH], expected_qbe),
                                (
                                    "thermal storage",
                                    branches[IDX_QCTH].d_internal[IDX_VRTH],
                                    3e-12,
                                ),
                            ] {
                                assert!(
                                    (actual - expected).abs() < 2e-7 * expected.abs(),
                                    "LEVEL={level} QBM={qbm} reverse={reverse} factor={factor} p={p} {name}: {actual:e} != {expected:e}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn vbic13_delayed_avalanche_reduction_matches_nonequilibrium_residual_derivatives() {
        let params = [
            ("LEVEL", 12.0),
            ("PNJMAXI", 1e-9),
            ("NF", 1.1),
            ("NR", 1.2),
            ("TNF", 0.001),
            ("VEF", 5.0),
            ("VER", 3.0),
            ("TCVEF", 0.05),
            ("TCVER", -0.02),
            ("IS", 1e-16),
            ("IBEI", 1e-18),
            ("IBCI", 1e-18),
            ("RCX", 10.0),
            ("RCI", 2.0),
            ("RBX", 5.0),
            ("RBI", 3.0),
            ("RE", 1.0),
            ("RBP", 2.0),
            ("RS", 3.0),
            ("RTH", 1000.0),
            ("AVC1", 0.2),
            ("AVC2", 0.3),
            ("TAVC", 0.01),
            ("TD", 1e-9),
            ("TF", 2e-9),
            ("QTF", 0.3),
            ("XTF", 2.0),
            ("ITF", 1e-4),
            ("VTF", 2.0),
            ("IKF", 1e-6),
            ("IKR", 2e-6),
            ("NKF", 0.4),
            ("QBM", 1.0),
            ("CJE", 1e-12),
            ("CJC", 2e-12),
            ("TMAXCLIP", 100.0),
        ]
        .map(|(name, value)| (name.to_owned(), value))
        .into_iter()
        .collect();
        for (bjt, p) in [
            (Bjt::new_npn("q".into(), 1, 2, 3), 1.0),
            (Bjt::new_pnp("q".into(), 1, 2, 3), -1.0),
        ] {
            let bjt = bjt
                .with_params(&params)
                .with_instance_params(&[("M".into(), 2.0), ("TRISE".into(), 20.0)]);
            let external = [p * 1.8, p * 0.7, 0.0, 0.0];
            for rise in [20.0, 74.0] {
                let state = [
                    p * 1.79,
                    p * 1.6,
                    p * 0.7,
                    p * 0.69,
                    0.0,
                    p * 1.7,
                    0.0,
                    rise,
                    1e-4,
                    2e-4,
                ];
                let evaluate = |state: [Value; BJT_INTERNAL_STATE_DIM]| {
                    let mut reduction = bjt.dynamic_reduction_for_internal_state(
                        external[0],
                        external[1],
                        external[2],
                        external[3],
                        state,
                    );
                    assert_eq!(reduction.internal_voltages[IDX_VXF2], state[IDX_VXF2]);
                    let (static_residual, _) = bjt.intrinsic_state_residual_jacobian(
                        external[0],
                        external[1],
                        external[2],
                        external[3],
                        state[..INTERNAL_DIM].try_into().unwrap(),
                    );
                    let mut residual = [0.0; BJT_INTERNAL_STATE_DIM];
                    residual[..INTERNAL_DIM].copy_from_slice(&static_residual);
                    let heat = bjt.vbic_delay_static_thermal_branch(&reduction);
                    for branch in bjt
                        .vbic_delay_static_branches(&reduction)
                        .into_iter()
                        .chain([heat])
                    {
                        for (sign, row) in [(1.0, branch.pos_internal), (-1.0, branch.neg_internal)]
                        {
                            if let Some(row) = row {
                                residual[row] += sign * branch.current;
                            }
                        }
                    }
                    heat.accumulate_derivatives(
                        &mut reduction.g_ii,
                        &mut reduction.g_ie,
                        &mut reduction.g_ei,
                        &mut reduction.g_ee,
                    );
                    let charges = bjt
                        .vbic_dynamic_charge_state_at_bias(external, state, None)
                        .0;
                    (residual, reduction.g_ii, charges)
                };
                let (_, jacobian, charges) = evaluate(state);
                for column in 0..BJT_INTERNAL_STATE_DIM {
                    let h = if column == IDX_VRTH {
                        1e-3
                    } else if column >= IDX_VXF1 {
                        1e-8
                    } else {
                        1e-6
                    };
                    let mut plus = state;
                    let mut minus = state;
                    plus[column] += h;
                    minus[column] -= h;
                    let (plus, _, plus_charges) = evaluate(plus);
                    let (minus, _, minus_charges) = evaluate(minus);
                    for row in 0..BJT_INTERNAL_STATE_DIM {
                        let fd = (plus[row] - minus[row]) / (2.0 * h);
                        let actual = jacobian[row][column];
                        assert!(
                            (fd - actual).abs() < 2e-5 * actual.abs().max(fd.abs()) + 2e-9,
                            "p={p} rise={rise} row={row} column={column}: {actual:e} != {fd:e}"
                        );
                    }
                    for row in 0..BJT_DYNAMIC_CHARGE_COUNT {
                        let fd = (plus_charges[row].charge - minus_charges[row].charge) / (2.0 * h);
                        let actual = charges[row].d_internal[column];
                        assert!(
                            (fd - actual).abs() < 2e-5 * actual.abs().max(fd.abs()) + 1e-20,
                            "charge p={p} rise={rise} row={row} column={column}: {actual:e} != {fd:e}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn vbic_heat_switch_preserves_thermal_storage_and_disables_delayed_power() {
        let params = [
            ("LEVEL", 11.0),
            ("RTH", 100.0),
            ("CTH", 1e-12),
            ("TD", 2e-11),
        ]
        .map(|(name, value)| (name.to_owned(), value))
        .into_iter()
        .collect();
        let active = Bjt::new_npn("q".into(), 1, 2, 3).with_params(&params);
        let quiet = active
            .clone()
            .with_instance_params(&[("SW_ET".to_string(), 0.0)]);
        let mut reduction = BjtDynamicReduction::default();
        reduction.internal_voltages[IDX_VCI] = 1.2;
        reduction.internal_voltages[IDX_VXF2] = 2e-4;
        reduction.vbic_transport.itzf = 1e-4;
        let delayed_power = active.vbic_delay_static_thermal_branch(&reduction);
        assert!((delayed_power.current + 1.2e-4).abs() < 1e-18);
        assert!(
            !quiet
                .vbic_delay_static_thermal_branch(&reduction)
                .is_active()
        );
        assert_eq!(quiet.thermal_sink_branch(20.0).current, 0.2);
        assert_eq!(quiet.thermal_capacitance(), 1e-12);
        let delayed_current = quiet.vbic_delay_static_branches(&reduction);
        assert_eq!(
            delayed_current[0].current,
            active.vbic_delay_static_branches(&reduction)[0].current
        );
    }

    #[test]
    fn legacy_overlap_charge_uses_physical_terminal_polarity() {
        let params = [("LEVEL", 1.0), ("CBEO", 2e-12), ("CBCO", 3e-12)]
            .map(|(name, value)| (name.to_owned(), value))
            .into_iter()
            .collect();
        for bjt in [
            Bjt::new_npn("q".into(), 1, 2, 3),
            Bjt::new_pnp("q".into(), 1, 2, 3),
        ] {
            let bjt = bjt.with_params(&params);
            assert!(!bjt.uses_vbic_dynamic_charges());
            let charge = bjt.legacy_transient_charge_state_with_vbx(0.2, -0.5, 0.0, 0.0);
            assert_eq!(charge.qbe, 2e-12 * 0.2);
            assert_eq!(charge.capbe, 2e-12);
            assert_eq!(charge.qbc, 3e-12 * -0.5);
            assert_eq!(charge.capbc, 3e-12);
        }
    }

    #[test]
    fn legacy_tied_charge_branches_preserve_substrate_and_internal_series_storage() {
        let params = [("CJE", 1.0), ("CJC", 1.0), ("CJS", 1e-9)]
            .map(|(name, value)| (name.to_owned(), value))
            .into_iter()
            .collect();
        let mut bjt = Bjt::new_npn("q".into(), 1, 1, 1).with_params(&params);
        let reduction = BjtDynamicReduction::default();
        let branches = bjt.legacy_dynamic_charge_branches(
            reduction.external_voltages,
            reduction.internal_voltages,
        );
        assert!(!branches[0].is_active());
        assert!(!branches[2].is_active());
        assert!(branches[7].is_active());
        bjt.rbi = 1.0;
        let branches = bjt.legacy_dynamic_charge_branches(
            reduction.external_voltages,
            reduction.internal_voltages,
        );
        assert!(branches[0].is_active());
        assert!(branches[2].is_active());
        assert!(branches[7].is_active());
    }

    #[test]
    fn legacy_charge_branches_preserve_signed_and_zero_slopes() {
        for polarity in [1.0, -1.0] {
            let params = [("LEVEL", 1.0), ("TF", 1e-9), ("VAR", 0.72)]
                .map(|(name, value)| (name.to_owned(), value))
                .into_iter()
                .collect();
            let mut bjt = if polarity > 0.0 {
                Bjt::new_npn("q".into(), 1, 2, 3)
            } else {
                Bjt::new_pnp("q".into(), 1, 2, 3)
            }
            .with_params(&params);
            bjt.set_junction_gmin(0.0);
            for bias in [0.71, -1e100] {
                let vbe = polarity * bias;
                let charge = bjt.legacy_transient_charge_state_with_vbx(vbe, 0.0, 0.0, 0.0);
                if bias > 0.0 {
                    let h = 1e-7;
                    let derivative = (bjt
                        .legacy_transient_charge_state_with_vbx(vbe + h, 0.0, 0.0, 0.0)
                        .qbe
                        - bjt
                            .legacy_transient_charge_state_with_vbx(vbe - h, 0.0, 0.0, 0.0)
                            .qbe)
                        / (2.0 * h);
                    assert!(derivative < 0.0);
                    assert!(
                        (charge.capbe - derivative).abs() < derivative.abs() * 1e-7,
                        "polarity={polarity}: capbe={} differs from dQ/dV={derivative}",
                        charge.capbe
                    );
                } else {
                    assert_ne!(charge.qbe, 0.0);
                    assert_eq!(charge.capbe, 0.0);
                }
                let mut reduction = BjtDynamicReduction::default();
                reduction.internal_voltages[IDX_VBI] = vbe;
                reduction.internal_voltages[IDX_VCI] = vbe;
                reduction.external_voltages[EXT_B] = vbe;
                reduction.external_voltages[EXT_C] = vbe;
                let branches = bjt.legacy_dynamic_charge_branches(
                    reduction.external_voltages,
                    reduction.internal_voltages,
                );
                assert!(
                    branches[0].is_active(),
                    "bias={bias}: charge branch disappeared"
                );
                assert_eq!(branches[0].charge, charge.qbe);
                assert_eq!(-branches[0].d_internal[IDX_VEI], charge.capbe);
                assert!(branches[1..].iter().all(|branch| !branch.is_active()));
            }
        }
    }
}
