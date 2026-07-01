#![allow(dead_code, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
#[path = "stamp_blocks_0.rs"]
mod stamp_blocks_0;

const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

#[inline]
fn eval_ddt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    older: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    derivative_current: &mut [f64; STATE_COUNT],
    derivative_previous: &mut [f64; STATE_COUNT],
    ddt_active: bool,
    ddt_scale: f64,
    ddt_previous_value_scale: f64,
    ddt_older_value_scale: f64,
    ddt_previous_derivative_scale: f64,
    slot: usize,
    value: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated ddt state slot out of range");
    let previous_value = if initialized[slot] { previous[slot] } else { value };
    let older_value = if initialized[slot] { older[slot] } else { value };
    current[slot] = value;
    if ddt_active {
        let result = value * ddt_scale
            - previous_value * ddt_previous_value_scale
            - older_value * ddt_older_value_scale
            - derivative_previous[slot] * ddt_previous_derivative_scale;
        derivative_current[slot] = result;
        result
    } else {
        current[slot] = value;
        previous[slot] = value;
        older[slot] = value;
        derivative_current[slot] = 0.0;
        derivative_previous[slot] = 0.0;
        initialized[slot] = true;
        0.0
    }
}

#[inline]
fn ddt_jacobian(ddt_active: bool, ddt_scale: f64, derivative: f64) -> f64 {
    if ddt_active {
        derivative * ddt_scale
    } else {
        0.0
    }
}

#[inline]
fn eval_idt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    ddt_active: bool,
    idt_scale: f64,
    slot: usize,
    value: f64,
    ic: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated idt state slot out of range");
    let previous_value = if initialized[slot] { previous[slot] } else { ic };
    let current_value = if ddt_active {
        previous_value + value * idt_scale
    } else {
        ic
    };
    current[slot] = current_value;
    if !ddt_active {
        previous[slot] = current_value;
        initialized[slot] = true;
    }
    current_value
}

#[inline]
fn idt_jacobian(timestep: f64, derivative: f64) -> f64 {
    if timestep.abs() > Instance::DDT_EPSILON {
        derivative * timestep
    } else {
        0.0
    }
}

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_older = self.ddt_state_older.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_derivative_current = self.ddt_derivative_current.as_mut();
        let ddt_derivative_previous = self.ddt_derivative_previous.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_scale = self.ddt_coefficients.derivative_scale;
        let ddt_previous_value_scale = self.ddt_coefficients.previous_value_scale;
        let ddt_older_value_scale = self.ddt_coefficients.older_value_scale;
        let ddt_previous_derivative_scale = self.ddt_coefficients.previous_derivative_scale;
        let v0: f64 = 1.0;
        let v1: f64 = 0.0;
        let v6: f64 = nv5;
        let v7: f64 = nv4;
        let v8: f64 = (v6 - v7);
        let v9: f64 = nv6;
        let v10: f64 = (v6 - v9);
        let v11: f64 = (self.scalar_v5 * v10);
        let v12: f64 = nv9;
        let v55: f64 = (v11 - v12);
        let v56: f64 = (-v55);
        let v57: f64 = 1e-6;
        let v58: f64 = (v12 * v57);
        let v59: f64 = nv8;
        let v60: f64 = (if self.scalar_v18 { v59 } else { v1 });
        let v71: f64 = (v1 * v10);
        let v72: f64 = (v1 * v8);
        let v73: f64 = (v7 - v9);
        let v74: f64 = (v1 * v73);
        let v83: f64 = -0.0;

        let d56_dn5: f64 = self.scalar_v81;
        let d56_dn6: f64 = self.scalar_v5;
        let d56_dn9: f64 = v0;
        stamper.stamp_current_node3_local(
            Some(9),
            None,
            multiplicity * (v56),
            5,
            multiplicity * (d56_dn5),
            6,
            multiplicity * (d56_dn6),
            9,
            multiplicity * (d56_dn9),
        );
        let d58_dn9: f64 = v57;
        stamper.stamp_current_node1_local(
            Some(9),
            None,
            multiplicity * (v58),
            9,
            multiplicity * (d58_dn9),
        );
        let d60_dn8: f64 = self.scalar_v82;
        stamper.stamp_current_node1_local(
            Some(8),
            None,
            multiplicity * (v60),
            8,
            multiplicity * (d60_dn8),
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            self.scalar_v61,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            self.scalar_v62,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            self.scalar_v67,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            self.scalar_v70,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            None,
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            self.scalar_v70,
        );
        let d71_dn5: f64 = v1;
        let d71_dn6: f64 = v83;
        stamper.stamp_current_node2_local(
            Some(5),
            Some(6),
            multiplicity * (v71),
            5,
            multiplicity * (d71_dn5),
            6,
            multiplicity * (d71_dn6),
        );
        let d72_dn4: f64 = v83;
        let d72_dn5: f64 = v1;
        stamper.stamp_current_node2_local(
            Some(5),
            Some(4),
            multiplicity * (v72),
            4,
            multiplicity * (d72_dn4),
            5,
            multiplicity * (d72_dn5),
        );
        let d74_dn4: f64 = v1;
        let d74_dn6: f64 = v83;
        stamper.stamp_current_node2_local(
            Some(4),
            Some(6),
            multiplicity * (v74),
            4,
            multiplicity * (d74_dn4),
            6,
            multiplicity * (d74_dn6),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(5),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            self.scalar_v76,
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            Some(6),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            self.scalar_v78,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(4),
            7,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            7,
            self.scalar_v80,
        );
        let mut var_arg: f64 = 0.0;
        let mut var_arg_dn3: f64 = 0.0;
        let mut var_arg_dn4: f64 = 0.0;
        let mut var_arg_dn5: f64 = 0.0;
        let mut var_arg_dn6: f64 = 0.0;
        let mut var_le: f64 = 0.0;
        let mut var_le_dn3: f64 = 0.0;
        let mut var_le_dn4: f64 = 0.0;
        let mut var_le_dn5: f64 = 0.0;
        let mut var_le_dn6: f64 = 0.0;
        let mut var_lebv: f64 = 0.0;
        let mut var_lebv_dn3: f64 = 0.0;
        let mut var_lebv_dn4: f64 = 0.0;
        let mut var_lebv_dn5: f64 = 0.0;
        let mut var_lebv_dn6: f64 = 0.0;
        let mut var_weff: f64 = 0.0;
        let mut var_dv0: f64 = 0.0;
        let mut var_dv0_dn3: f64 = 0.0;
        let mut var_dvh: f64 = 0.0;
        let mut var_dvh_dn1: f64 = 0.0;
        let mut var_dvh_dn3: f64 = 0.0;
        let mut var_dvh_dn4: f64 = 0.0;
        let mut var_dvh_dn5: f64 = 0.0;
        let mut var_dvh_dn6: f64 = 0.0;
        let mut var_pwq: f64 = 0.0;
        let mut var_qlo: f64 = 0.0;
        let mut var_qlo_dn1: f64 = 0.0;
        let mut var_qlo_dn3: f64 = 0.0;
        let mut var_qlo_dn4: f64 = 0.0;
        let mut var_qlo_dn5: f64 = 0.0;
        let mut var_qlo_dn6: f64 = 0.0;
        let mut var_qhi: f64 = 0.0;
        let mut var_qhi_dn1: f64 = 0.0;
        let mut var_qhi_dn3: f64 = 0.0;
        let mut var_qhi_dn4: f64 = 0.0;
        let mut var_qhi_dn5: f64 = 0.0;
        let mut var_qhi_dn6: f64 = 0.0;
        let mut var_ttype: f64 = 0.0;
        let mut var_tdev: f64 = 0.0;
        let mut var_tdev_dn3: f64 = 0.0;
        let mut var_tnom: f64 = 0.0;
        let mut var_tamb: f64 = 0.0;
        let mut var_tamb_dn3: f64 = 0.0;
        let mut var_rt: f64 = 0.0;
        let mut var_rt_dn3: f64 = 0.0;
        let mut var_lnrt: f64 = 0.0;
        let mut var_lnrt_dn3: f64 = 0.0;
        let mut var_vt: f64 = 0.0;
        let mut var_vt_dn3: f64 = 0.0;
        let mut var_bf_t: f64 = 0.0;
        let mut var_bf_t_dn3: f64 = 0.0;
        let mut var_bf_t_dn4: f64 = 0.0;
        let mut var_bf_t_dn5: f64 = 0.0;
        let mut var_br_t: f64 = 0.0;
        let mut var_br_t_dn3: f64 = 0.0;
        let mut var_tbeta: f64 = 0.0;
        let mut var_tbeta_dn3: f64 = 0.0;
        let mut var_is_t: f64 = 0.0;
        let mut var_is_t_dn3: f64 = 0.0;
        let mut var_ise_t: f64 = 0.0;
        let mut var_ise_t_dn3: f64 = 0.0;
        let mut var_isc_t: f64 = 0.0;
        let mut var_isc_t_dn3: f64 = 0.0;
        let mut var_cje_t: f64 = 0.0;
        let mut var_cje_t_dn3: f64 = 0.0;
        let mut var_cjc_t: f64 = 0.0;
        let mut var_cjc_t_dn3: f64 = 0.0;
        let mut var_cjs_t: f64 = 0.0;
        let mut var_cjs_t_dn3: f64 = 0.0;
        let mut var_vje_t: f64 = 0.0;
        let mut var_vje_t_dn3: f64 = 0.0;
        let mut var_vjc_t: f64 = 0.0;
        let mut var_vjc_t_dn3: f64 = 0.0;
        let mut var_vjs_t: f64 = 0.0;
        let mut var_vjs_t_dn3: f64 = 0.0;
        let mut var_ijbv_t: f64 = 0.0;
        let mut var_ijbv_t_dn3: f64 = 0.0;
        let mut var_ijbvc_t: f64 = 0.0;
        let mut var_ijbvc_t_dn3: f64 = 0.0;
        let mut var_bvr_t: f64 = 0.0;
        let mut var_bvr_t_dn3: f64 = 0.0;
        let mut var_theexp_t: f64 = 0.0;
        let mut var_theexp_t_dn3: f64 = 0.0;
        let mut var_cje_i: f64 = 0.0;
        let mut var_cjc_i: f64 = 0.0;
        let mut var_cjs_i: f64 = 0.0;
        let mut var_ifwd: f64 = 0.0;
        let mut var_ifwd_dn3: f64 = 0.0;
        let mut var_ifwd_dn4: f64 = 0.0;
        let mut var_ifwd_dn5: f64 = 0.0;
        let mut var_ifwd_dn6: f64 = 0.0;
        let mut var_ibe2: f64 = 0.0;
        let mut var_ibe2_dn3: f64 = 0.0;
        let mut var_ibe2_dn4: f64 = 0.0;
        let mut var_ibe2_dn5: f64 = 0.0;
        let mut var_ibe2_dn6: f64 = 0.0;
        let mut var_ibe: f64 = 0.0;
        let mut var_ibe_dn3: f64 = 0.0;
        let mut var_ibe_dn4: f64 = 0.0;
        let mut var_ibe_dn5: f64 = 0.0;
        let mut var_ibe_dn6: f64 = 0.0;
        let mut var_ibwd: f64 = 0.0;
        let mut var_ibwd_dn3: f64 = 0.0;
        let mut var_ibwd_dn4: f64 = 0.0;
        let mut var_ibwd_dn5: f64 = 0.0;
        let mut var_ibwd_dn6: f64 = 0.0;
        let mut var_ibc2: f64 = 0.0;
        let mut var_ibc2_dn3: f64 = 0.0;
        let mut var_ibc2_dn4: f64 = 0.0;
        let mut var_ibc2_dn5: f64 = 0.0;
        let mut var_ibc2_dn6: f64 = 0.0;
        let mut var_ibc: f64 = 0.0;
        let mut var_ibc_dn3: f64 = 0.0;
        let mut var_ibc_dn4: f64 = 0.0;
        let mut var_ibc_dn5: f64 = 0.0;
        let mut var_ibc_dn6: f64 = 0.0;
        let mut var_ikq1: f64 = 0.0;
        let mut var_ikq1_dn4: f64 = 0.0;
        let mut var_ikq1_dn5: f64 = 0.0;
        let mut var_ikq1_dn6: f64 = 0.0;
        let mut var_kq2: f64 = 0.0;
        let mut var_kq2_dn3: f64 = 0.0;
        let mut var_kq2_dn4: f64 = 0.0;
        let mut var_kq2_dn5: f64 = 0.0;
        let mut var_kq2_dn6: f64 = 0.0;
        let mut var_ikqb: f64 = 0.0;
        let mut var_ikqb_dn3: f64 = 0.0;
        let mut var_ikqb_dn4: f64 = 0.0;
        let mut var_ikqb_dn5: f64 = 0.0;
        let mut var_ikqb_dn6: f64 = 0.0;
        let mut var_itzf: f64 = 0.0;
        let mut var_itzf_dn3: f64 = 0.0;
        let mut var_itzf_dn4: f64 = 0.0;
        let mut var_itzf_dn5: f64 = 0.0;
        let mut var_itzf_dn6: f64 = 0.0;
        let mut var_itr: f64 = 0.0;
        let mut var_itr_dn3: f64 = 0.0;
        let mut var_itr_dn4: f64 = 0.0;
        let mut var_itr_dn5: f64 = 0.0;
        let mut var_itr_dn6: f64 = 0.0;
        let mut var_itzf_f: f64 = 0.0;
        let mut var_itzf_f_dn3: f64 = 0.0;
        let mut var_itzf_f_dn4: f64 = 0.0;
        let mut var_itzf_f_dn5: f64 = 0.0;
        let mut var_itzf_f_dn6: f64 = 0.0;
        let mut var_itzf_f_dn9: f64 = 0.0;
        let mut var_itrev: f64 = 0.0;
        let mut var_itrev_dn3: f64 = 0.0;
        let mut var_itrev_dn4: f64 = 0.0;
        let mut var_itrev_dn5: f64 = 0.0;
        let mut var_itrev_dn6: f64 = 0.0;
        let mut var_re_nom: f64 = 0.0;
        let mut var_rc_nom: f64 = 0.0;
        let mut var_rb_nom: f64 = 0.0;
        let mut var_rb: f64 = 0.0;
        let mut var_rb_dn1: f64 = 0.0;
        let mut var_rb_dn3: f64 = 0.0;
        let mut var_rb_dn5: f64 = 0.0;
        let mut var_rb_dn8: f64 = 0.0;
        let mut var_rc: f64 = 0.0;
        let mut var_rc_dn3: f64 = 0.0;
        let mut var_re: f64 = 0.0;
        let mut var_re_dn2: f64 = 0.0;
        let mut var_re_dn3: f64 = 0.0;
        let mut var_re_dn6: f64 = 0.0;
        let mut var_tff: f64 = 0.0;
        let mut var_tff_dn1: f64 = 0.0;
        let mut var_tff_dn2: f64 = 0.0;
        let mut var_qde: f64 = 0.0;
        let mut var_qde_dn1: f64 = 0.0;
        let mut var_qde_dn2: f64 = 0.0;
        let mut var_qde_dn3: f64 = 0.0;
        let mut var_qde_dn4: f64 = 0.0;
        let mut var_qde_dn5: f64 = 0.0;
        let mut var_qde_dn6: f64 = 0.0;
        let mut var_qdc: f64 = 0.0;
        let mut var_qdc_dn3: f64 = 0.0;
        let mut var_qdc_dn4: f64 = 0.0;
        let mut var_qdc_dn5: f64 = 0.0;
        let mut var_qdc_dn6: f64 = 0.0;
        let mut var_qjs: f64 = 0.0;
        let mut var_qjs_dn2: f64 = 0.0;
        let mut var_qjs_dn3: f64 = 0.0;
        let mut var_qjs_dn4: f64 = 0.0;
        let mut var_qje: f64 = 0.0;
        let mut var_qje_dn1: f64 = 0.0;
        let mut var_qje_dn3: f64 = 0.0;
        let mut var_qje_dn4: f64 = 0.0;
        let mut var_qje_dn5: f64 = 0.0;
        let mut var_qje_dn6: f64 = 0.0;
        let mut var_qjcx: f64 = 0.0;
        let mut var_qjcx_dn1: f64 = 0.0;
        let mut var_qjcx_dn3: f64 = 0.0;
        let mut var_qjcx_dn4: f64 = 0.0;
        let mut var_qjcx_dn5: f64 = 0.0;
        let mut var_qjcx_dn6: f64 = 0.0;
        let mut var_qjcx_1: f64 = 0.0;
        let mut var_qjcx_1_dn1: f64 = 0.0;
        let mut var_qjcx_1_dn3: f64 = 0.0;
        let mut var_qjcx_1_dn4: f64 = 0.0;
        let mut var_qjcx_1_dn5: f64 = 0.0;
        let mut var_qjcx_1_dn6: f64 = 0.0;
        let mut var_qjci: f64 = 0.0;
        let mut var_qjci_dn1: f64 = 0.0;
        let mut var_qjci_dn3: f64 = 0.0;
        let mut var_qjci_dn4: f64 = 0.0;
        let mut var_qjci_dn5: f64 = 0.0;
        let mut var_qjci_dn6: f64 = 0.0;
        let mut var_qjci_1: f64 = 0.0;
        let mut var_qjci_1_dn1: f64 = 0.0;
        let mut var_qjci_1_dn3: f64 = 0.0;
        let mut var_qjci_1_dn4: f64 = 0.0;
        let mut var_qjci_1_dn5: f64 = 0.0;
        let mut var_qjci_1_dn6: f64 = 0.0;
        let mut var_qxf1: f64 = 0.0;
        let mut var_qxf1_dn3: f64 = 0.0;
        let mut var_qxf1_dn4: f64 = 0.0;
        let mut var_qxf1_dn5: f64 = 0.0;
        let mut var_qxf1_dn6: f64 = 0.0;
        let mut var_ovaf: f64 = 0.0;
        let mut var_ovar: f64 = 0.0;
        let mut var_oikf: f64 = 0.0;
        let mut var_oikf_dn4: f64 = 0.0;
        let mut var_oikf_dn5: f64 = 0.0;
        let mut var_oikr: f64 = 0.0;
        let mut var_argt: f64 = 0.0;
        let mut var_argt_dn3: f64 = 0.0;
        let mut var_veci: f64 = 0.0;
        let mut var_veci_dn2: f64 = 0.0;
        let mut var_veci_dn4: f64 = 0.0;
        let mut var_vbiei: f64 = 0.0;
        let mut var_vbiei_dn5: f64 = 0.0;
        let mut var_vbiei_dn6: f64 = 0.0;
        let mut var_vbici: f64 = 0.0;
        let mut var_vbici_dn4: f64 = 0.0;
        let mut var_vbici_dn5: f64 = 0.0;
        let mut var_vbci: f64 = 0.0;
        let mut var_vbci_dn1: f64 = 0.0;
        let mut var_vbci_dn4: f64 = 0.0;
        let mut var_vbbi: f64 = 0.0;
        let mut var_vbbi_dn1: f64 = 0.0;
        let mut var_vbbi_dn5: f64 = 0.0;
        let mut var_veei: f64 = 0.0;
        let mut var_veei_dn2: f64 = 0.0;
        let mut var_veei_dn6: f64 = 0.0;
        let mut var_fact1: f64 = 0.0;
        let mut var_fact2: f64 = 0.0;
        let mut var_fact2_dn3: f64 = 0.0;
        let mut var_egfet: f64 = 0.0;
        let mut var_egfet_dn3: f64 = 0.0;
        let mut var_arg0: f64 = 0.0;
        let mut var_arg0_dn3: f64 = 0.0;
        let mut var_pbfact: f64 = 0.0;
        let mut var_pbfact_dn3: f64 = 0.0;
        let mut var_pbo: f64 = 0.0;
        let mut var_pbo_dn3: f64 = 0.0;
        let mut var_gmaold: f64 = 0.0;
        let mut var_gmaold_dn3: f64 = 0.0;
        let mut var_gmanew: f64 = 0.0;
        let mut var_gmanew_dn3: f64 = 0.0;
        let mut var_cjt: f64 = 0.0;
        let mut var_cjt_dn3: f64 = 0.0;
        let mut var_argbv: f64 = 0.0;
        let mut var_argbv_dn3: f64 = 0.0;
        let mut var_argbv_dn4: f64 = 0.0;
        let mut var_argbv_dn5: f64 = 0.0;
        let mut var_argbv_dn6: f64 = 0.0;
        let mut var_argbvvt: f64 = 0.0;
        let mut var_argbvvt_dn3: f64 = 0.0;
        let mut var_argtr: f64 = 0.0;
        let mut var_argtr_dn3: f64 = 0.0;
        let mut var_isr_t: f64 = 0.0;
        let mut var_isr_t_dn3: f64 = 0.0;
        let mut var_fbwm: f64 = 0.0;
        let mut var_fbwm_dn4: f64 = 0.0;
        let mut var_fbwm_dn5: f64 = 0.0;
        let mut var_vbc: f64 = 0.0;
        let mut var_vbc_dn4: f64 = 0.0;
        let mut var_vbc_dn5: f64 = 0.0;
        let mut var_dkqb: f64 = 0.0;
        let mut var_dkqb_dn3: f64 = 0.0;
        let mut var_dkqb_dn4: f64 = 0.0;
        let mut var_dkqb_dn5: f64 = 0.0;
        let mut var_dkqb_dn6: f64 = 0.0;
        let mut var_vtff: f64 = 0.0;
        let mut var_vtff_dn1: f64 = 0.0;
        let mut var_vtff_dn2: f64 = 0.0;
        let mut var_vtff1: f64 = 0.0;
        let mut var_vtff1_dn1: f64 = 0.0;
        let mut var_vtff1_dn2: f64 = 0.0;
        let mut var_vbesat: f64 = 0.0;
        let mut var_vbesat_dn1: f64 = 0.0;
        let mut var_vbesat_dn5: f64 = 0.0;
        let mut var_veesat: f64 = 0.0;
        let mut var_veesat_dn2: f64 = 0.0;
        let mut var_veesat_dn6: f64 = 0.0;
        let mut var_t0: f64 = 0.0;
        let mut var_t0_dn5: f64 = 0.0;
        let mut var_t0_dn6: f64 = 0.0;
        let mut var_d_ratio: f64 = 0.0;
        let mut var_d_ratio_dn5: f64 = 0.0;
        let mut var_d_ratio_dn6: f64 = 0.0;
        let mut var_d_ratio_dn9: f64 = 0.0;
        let mut var_guard3: f64 = 0.0;
        let mut var_guard4: f64 = 0.0;
        let mut var_guard5: f64 = 0.0;
        let mut var_guard6: f64 = 0.0;
        let mut var_guard7: f64 = 0.0;
        let mut var_guard8: f64 = 0.0;
        let mut var_guard9: f64 = 0.0;
        let mut var_guard10: f64 = 0.0;
        let mut var_guard11: f64 = 0.0;
        let mut var_guard12: f64 = 0.0;
        let mut var_guard13: f64 = 0.0;
        let mut var_guard14: f64 = 0.0;
        let mut var_guard15: f64 = 0.0;
        let mut var_guard16: f64 = 0.0;
        let mut var_guard17: f64 = 0.0;
        let mut var_guard18: f64 = 0.0;
        let mut var_guard19: f64 = 0.0;
        let mut var_guard20: f64 = 0.0;
        let mut var_guard21: f64 = 0.0;
        let mut var_guard22: f64 = 0.0;
        let mut var_guard23: f64 = 0.0;
        let mut var_guard24: f64 = 0.0;
        let mut var_guard25: f64 = 0.0;

        Self::stamp_transient_block_0(ctx, p, nodes, &mut var_arg, &mut var_arg0, &mut var_arg0_dn3, &mut var_arg_dn3, &mut var_arg_dn4, &mut var_arg_dn5, &mut var_arg_dn6, &mut var_argbv, &mut var_argbv_dn3, &mut var_argbv_dn4, &mut var_argbv_dn5, &mut var_argbv_dn6, &mut var_argbvvt, &mut var_argbvvt_dn3, &mut var_argt, &mut var_argt_dn3, &mut var_argtr, &mut var_argtr_dn3, &mut var_bf_t, &mut var_bf_t_dn3, &mut var_bf_t_dn4, &mut var_bf_t_dn5, &mut var_br_t, &mut var_br_t_dn3, &mut var_bvr_t, &mut var_bvr_t_dn3, &mut var_cjc_i, &mut var_cjc_t, &mut var_cjc_t_dn3, &mut var_cje_i, &mut var_cje_t, &mut var_cje_t_dn3, &mut var_cjs_i, &mut var_cjs_t, &mut var_cjs_t_dn3, &mut var_cjt, &mut var_cjt_dn3, &mut var_egfet, &mut var_egfet_dn3, &mut var_fact1, &mut var_fact2, &mut var_fact2_dn3, &mut var_fbwm, &mut var_fbwm_dn4, &mut var_fbwm_dn5, &mut var_gmanew, &mut var_gmanew_dn3, &mut var_gmaold, &mut var_gmaold_dn3, &mut var_guard3, &mut var_guard4, &mut var_ijbv_t, &mut var_ijbv_t_dn3, &mut var_ijbvc_t, &mut var_ijbvc_t_dn3, &mut var_is_t, &mut var_is_t_dn3, &mut var_isc_t, &mut var_isc_t_dn3, &mut var_ise_t, &mut var_ise_t_dn3, &mut var_isr_t, &mut var_isr_t_dn3, &mut var_lnrt, &mut var_lnrt_dn3, &mut var_oikf, &mut var_oikf_dn4, &mut var_oikf_dn5, &mut var_oikr, &mut var_ovaf, &mut var_ovar, &mut var_pbfact, &mut var_pbfact_dn3, &mut var_pbo, &mut var_pbo_dn3, &mut var_rt, &mut var_rt_dn3, &mut var_tamb, &mut var_tamb_dn3, &mut var_tbeta, &mut var_tbeta_dn3, &mut var_tdev, &mut var_tdev_dn3, &mut var_theexp_t, &mut var_theexp_t_dn3, &mut var_tnom, &mut var_ttype, &mut var_vbbi, &mut var_vbbi_dn1, &mut var_vbbi_dn5, &mut var_vbc, &mut var_vbc_dn4, &mut var_vbc_dn5, &mut var_vbci, &mut var_vbci_dn1, &mut var_vbci_dn4, &mut var_vbici, &mut var_vbici_dn4, &mut var_vbici_dn5, &mut var_vbiei, &mut var_vbiei_dn5, &mut var_vbiei_dn6, &mut var_veci, &mut var_veci_dn2, &mut var_veci_dn4, &mut var_veei, &mut var_veei_dn2, &mut var_veei_dn6, &mut var_vjc_t, &mut var_vjc_t_dn3, &mut var_vje_t, &mut var_vje_t_dn3, &mut var_vjs_t, &mut var_vjs_t_dn3, &mut var_vt, &mut var_vt_dn3, &mut var_weff);
        Self::stamp_transient_block_1(p, var_bvr_t, var_bvr_t_dn3, var_guard3, var_guard4, var_ijbv_t, var_ijbv_t_dn3, var_is_t, var_is_t_dn3, var_ise_t, var_ise_t_dn3, var_isr_t, var_isr_t_dn3, var_theexp_t, var_theexp_t_dn3, var_vbici, var_vbici_dn4, var_vbici_dn5, var_vbiei, var_vbiei_dn5, var_vbiei_dn6, var_vt, var_vt_dn3, &mut var_arg, &mut var_arg_dn3, &mut var_arg_dn4, &mut var_arg_dn5, &mut var_arg_dn6, &mut var_argbv, &mut var_argbv_dn3, &mut var_argbv_dn4, &mut var_argbv_dn5, &mut var_argbv_dn6, &mut var_argbvvt, &mut var_argbvvt_dn3, &mut var_guard5, &mut var_guard6, &mut var_guard7, &mut var_guard8, &mut var_guard9, &mut var_ibe2, &mut var_ibe2_dn3, &mut var_ibe2_dn4, &mut var_ibe2_dn5, &mut var_ibe2_dn6, &mut var_ifwd, &mut var_ifwd_dn3, &mut var_ifwd_dn4, &mut var_ifwd_dn5, &mut var_ifwd_dn6, &mut var_itrev, &mut var_itrev_dn3, &mut var_itrev_dn4, &mut var_itrev_dn5, &mut var_itrev_dn6, &mut var_le, &mut var_le_dn3, &mut var_le_dn4, &mut var_le_dn5, &mut var_le_dn6, &mut var_lebv, &mut var_lebv_dn3, &mut var_lebv_dn4, &mut var_lebv_dn5, &mut var_lebv_dn6, &mut var_t0, &mut var_t0_dn5, &mut var_t0_dn6);
        Self::stamp_transient_block_2(ctx, p, nodes, var_bf_t, var_bf_t_dn3, var_bf_t_dn4, var_bf_t_dn5, var_br_t, var_br_t_dn3, var_bvr_t, var_bvr_t_dn3, var_guard9, var_ibe2, var_ibe2_dn3, var_ibe2_dn4, var_ibe2_dn5, var_ibe2_dn6, var_ifwd, var_ifwd_dn3, var_ifwd_dn4, var_ifwd_dn5, var_ifwd_dn6, var_ijbvc_t, var_ijbvc_t_dn3, var_is_t, var_is_t_dn3, var_isc_t, var_isc_t_dn3, var_itrev, var_itrev_dn3, var_itrev_dn4, var_itrev_dn5, var_itrev_dn6, var_oikr, var_ovaf, var_ovar, var_theexp_t, var_theexp_t_dn3, var_vbbi, var_vbbi_dn1, var_vbbi_dn5, var_vbici, var_vbici_dn4, var_vbici_dn5, var_vbiei, var_vbiei_dn5, var_vbiei_dn6, var_vt, var_vt_dn3, &mut var_arg, &mut var_arg_dn3, &mut var_arg_dn4, &mut var_arg_dn5, &mut var_arg_dn6, &mut var_argbv, &mut var_argbv_dn3, &mut var_argbv_dn4, &mut var_argbv_dn5, &mut var_argbv_dn6, &mut var_argbvvt, &mut var_argbvvt_dn3, &mut var_d_ratio, &mut var_d_ratio_dn5, &mut var_d_ratio_dn6, &mut var_d_ratio_dn9, &mut var_dkqb, &mut var_dkqb_dn3, &mut var_dkqb_dn4, &mut var_dkqb_dn5, &mut var_dkqb_dn6, &mut var_guard10, &mut var_guard11, &mut var_guard12, &mut var_ibc, &mut var_ibc2, &mut var_ibc2_dn3, &mut var_ibc2_dn4, &mut var_ibc2_dn5, &mut var_ibc2_dn6, &mut var_ibc_dn3, &mut var_ibc_dn4, &mut var_ibc_dn5, &mut var_ibc_dn6, &mut var_ibe, &mut var_ibe_dn3, &mut var_ibe_dn4, &mut var_ibe_dn5, &mut var_ibe_dn6, &mut var_ibwd, &mut var_ibwd_dn3, &mut var_ibwd_dn4, &mut var_ibwd_dn5, &mut var_ibwd_dn6, &mut var_ikq1, &mut var_ikq1_dn4, &mut var_ikq1_dn5, &mut var_ikq1_dn6, &mut var_ikqb, &mut var_ikqb_dn3, &mut var_ikqb_dn4, &mut var_ikqb_dn5, &mut var_ikqb_dn6, &mut var_itr, &mut var_itr_dn3, &mut var_itr_dn4, &mut var_itr_dn5, &mut var_itr_dn6, &mut var_itzf, &mut var_itzf_dn3, &mut var_itzf_dn4, &mut var_itzf_dn5, &mut var_itzf_dn6, &mut var_itzf_f, &mut var_itzf_f_dn3, &mut var_itzf_f_dn4, &mut var_itzf_f_dn5, &mut var_itzf_f_dn6, &mut var_itzf_f_dn9, &mut var_kq2, &mut var_kq2_dn3, &mut var_kq2_dn4, &mut var_kq2_dn5, &mut var_kq2_dn6, &mut var_le, &mut var_le_dn3, &mut var_le_dn4, &mut var_le_dn5, &mut var_le_dn6, &mut var_lebv, &mut var_lebv_dn3, &mut var_lebv_dn4, &mut var_lebv_dn5, &mut var_lebv_dn6, &mut var_oikf, &mut var_oikf_dn4, &mut var_oikf_dn5, &mut var_vbesat, &mut var_vbesat_dn1, &mut var_vbesat_dn5);
        Self::stamp_transient_block_3(ctx, p, nodes, var_cjc_t, var_cjc_t_dn3, var_cje_t, var_cje_t_dn3, var_cjs_t, var_cjs_t_dn3, var_ifwd, var_ifwd_dn3, var_ifwd_dn4, var_ifwd_dn5, var_ifwd_dn6, var_itr, var_itr_dn3, var_itr_dn4, var_itr_dn5, var_itr_dn6, var_lnrt, var_lnrt_dn3, var_vbci, var_vbci_dn1, var_vbci_dn4, var_vbesat, var_vbesat_dn1, var_vbesat_dn5, var_vbici, var_vbici_dn4, var_vbici_dn5, var_vbiei, var_vbiei_dn5, var_vbiei_dn6, var_veci, var_veci_dn2, var_veci_dn4, var_veei, var_veei_dn2, var_veei_dn6, var_vjc_t, var_vjc_t_dn3, var_vje_t, var_vje_t_dn3, var_vjs_t, var_vjs_t_dn3, &mut var_dv0, &mut var_dv0_dn3, &mut var_dvh, &mut var_dvh_dn1, &mut var_dvh_dn3, &mut var_dvh_dn4, &mut var_dvh_dn5, &mut var_dvh_dn6, &mut var_guard13, &mut var_guard14, &mut var_guard15, &mut var_guard16, &mut var_guard17, &mut var_guard18, &mut var_pwq, &mut var_qdc, &mut var_qdc_dn3, &mut var_qdc_dn4, &mut var_qdc_dn5, &mut var_qdc_dn6, &mut var_qde, &mut var_qde_dn1, &mut var_qde_dn2, &mut var_qde_dn3, &mut var_qde_dn4, &mut var_qde_dn5, &mut var_qde_dn6, &mut var_qhi, &mut var_qhi_dn1, &mut var_qhi_dn3, &mut var_qhi_dn4, &mut var_qhi_dn5, &mut var_qhi_dn6, &mut var_qjcx, &mut var_qjcx_1, &mut var_qjcx_1_dn1, &mut var_qjcx_1_dn3, &mut var_qjcx_1_dn4, &mut var_qjcx_1_dn5, &mut var_qjcx_1_dn6, &mut var_qjcx_dn1, &mut var_qjcx_dn3, &mut var_qjcx_dn4, &mut var_qjcx_dn5, &mut var_qjcx_dn6, &mut var_qje, &mut var_qje_dn1, &mut var_qje_dn3, &mut var_qje_dn4, &mut var_qje_dn5, &mut var_qje_dn6, &mut var_qjs, &mut var_qjs_dn2, &mut var_qjs_dn3, &mut var_qjs_dn4, &mut var_qlo, &mut var_qlo_dn1, &mut var_qlo_dn3, &mut var_qlo_dn4, &mut var_qlo_dn5, &mut var_qlo_dn6, &mut var_rb, &mut var_rb_dn1, &mut var_rb_dn3, &mut var_rb_dn5, &mut var_rb_dn8, &mut var_rc, &mut var_rc_dn3, &mut var_re, &mut var_re_dn2, &mut var_re_dn3, &mut var_re_dn6, &mut var_tff, &mut var_tff_dn1, &mut var_tff_dn2, &mut var_veesat, &mut var_veesat_dn2, &mut var_veesat_dn6, &mut var_vtff, &mut var_vtff1, &mut var_vtff1_dn1, &mut var_vtff1_dn2, &mut var_vtff_dn1, &mut var_vtff_dn2);
        Self::stamp_transient_block_4(p, var_cjc_t, var_cjc_t_dn3, var_dvh, var_dvh_dn1, var_dvh_dn3, var_dvh_dn4, var_dvh_dn5, var_dvh_dn6, var_guard18, var_itzf, var_itzf_dn3, var_itzf_dn4, var_itzf_dn5, var_itzf_dn6, var_pwq, var_ttype, var_vbici, var_vbici_dn4, var_vbici_dn5, var_vjc_t, var_vjc_t_dn3, var_weff, &mut var_guard19, &mut var_guard20, &mut var_guard21, &mut var_guard22, &mut var_guard23, &mut var_guard24, &mut var_guard25, &mut var_qhi, &mut var_qhi_dn1, &mut var_qhi_dn3, &mut var_qhi_dn4, &mut var_qhi_dn5, &mut var_qhi_dn6, &mut var_qjci, &mut var_qjci_1, &mut var_qjci_1_dn1, &mut var_qjci_1_dn3, &mut var_qjci_1_dn4, &mut var_qjci_1_dn5, &mut var_qjci_1_dn6, &mut var_qjci_dn1, &mut var_qjci_dn3, &mut var_qjci_dn4, &mut var_qjci_dn5, &mut var_qjci_dn6, &mut var_qlo, &mut var_qlo_dn1, &mut var_qlo_dn3, &mut var_qlo_dn4, &mut var_qlo_dn5, &mut var_qlo_dn6, &mut var_qxf1, &mut var_qxf1_dn3, &mut var_qxf1_dn4, &mut var_qxf1_dn5, &mut var_qxf1_dn6, &mut var_rb_nom, &mut var_rc_nom, &mut var_re_nom);

        Self::stamp_transient_equations_block_0(ctx, stamper, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, var_bf_t, var_bf_t_dn3, var_bf_t_dn4, var_bf_t_dn5, var_guard13, var_guard20, var_guard21, var_guard22, var_guard23, var_guard24, var_guard25, var_ibc, var_ibc_dn3, var_ibc_dn4, var_ibc_dn5, var_ibc_dn6, var_ibe, var_ibe_dn3, var_ibe_dn4, var_ibe_dn5, var_ibe_dn6, var_ifwd, var_ifwd_dn3, var_ifwd_dn4, var_ifwd_dn5, var_ifwd_dn6, var_rb, var_rb_dn1, var_rb_dn3, var_rb_dn5, var_rb_dn8, var_rc, var_rc_dn3, var_re, var_re_dn2, var_re_dn3, var_re_dn6, var_tff, var_tff_dn1, var_tff_dn2, var_ttype, var_weff);
        Self::stamp_transient_equations_block_1(stamper, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, var_itr, var_itr_dn3, var_itr_dn4, var_itr_dn5, var_itr_dn6, var_itzf_f, var_itzf_f_dn3, var_itzf_f_dn4, var_itzf_f_dn5, var_itzf_f_dn6, var_itzf_f_dn9, var_qdc, var_qdc_dn3, var_qdc_dn4, var_qdc_dn5, var_qdc_dn6, var_qde, var_qde_dn1, var_qde_dn2, var_qde_dn3, var_qde_dn4, var_qde_dn5, var_qde_dn6, var_qjci_1, var_qjci_1_dn1, var_qjci_1_dn3, var_qjci_1_dn4, var_qjci_1_dn5, var_qjci_1_dn6, var_qjcx_1, var_qjcx_1_dn1, var_qjcx_1_dn3, var_qjcx_1_dn4, var_qjcx_1_dn5, var_qjcx_1_dn6, var_qje, var_qje_dn1, var_qje_dn3, var_qje_dn4, var_qje_dn5, var_qje_dn6, var_qjs, var_qjs_dn2, var_qjs_dn3, var_qjs_dn4, var_qxf1, var_qxf1_dn3, var_qxf1_dn4, var_qxf1_dn5, var_qxf1_dn6, var_ttype, var_weff);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let multiplicity = (*self).multiplicity;
        let mut var_arg: f64 = 0.0;
        let mut var_arg_rv: f64 = 0.0;
        let mut var_arg_dn3: f64 = 0.0;
        let mut var_arg_dn4: f64 = 0.0;
        let mut var_arg_dn5: f64 = 0.0;
        let mut var_arg_dn6: f64 = 0.0;
        let mut var_le: f64 = 0.0;
        let mut var_le_rv: f64 = 0.0;
        let mut var_le_dn3: f64 = 0.0;
        let mut var_le_dn4: f64 = 0.0;
        let mut var_le_dn5: f64 = 0.0;
        let mut var_le_dn6: f64 = 0.0;
        let mut var_lebv: f64 = 0.0;
        let mut var_lebv_rv: f64 = 0.0;
        let mut var_lebv_dn3: f64 = 0.0;
        let mut var_lebv_dn4: f64 = 0.0;
        let mut var_lebv_dn5: f64 = 0.0;
        let mut var_lebv_dn6: f64 = 0.0;
        let mut var_weff: f64 = 0.0;
        let mut var_weff_rv: f64 = 0.0;
        let mut var_dv0: f64 = 0.0;
        let mut var_dv0_rv: f64 = 0.0;
        let mut var_dv0_dn3: f64 = 0.0;
        let mut var_dvh: f64 = 0.0;
        let mut var_dvh_rv: f64 = 0.0;
        let mut var_dvh_dn1: f64 = 0.0;
        let mut var_dvh_dn3: f64 = 0.0;
        let mut var_dvh_dn4: f64 = 0.0;
        let mut var_dvh_dn5: f64 = 0.0;
        let mut var_dvh_dn6: f64 = 0.0;
        let mut var_pwq: f64 = 0.0;
        let mut var_pwq_rv: f64 = 0.0;
        let mut var_qlo: f64 = 0.0;
        let mut var_qlo_rv: f64 = 0.0;
        let mut var_qlo_dn1: f64 = 0.0;
        let mut var_qlo_dn3: f64 = 0.0;
        let mut var_qlo_dn4: f64 = 0.0;
        let mut var_qlo_dn5: f64 = 0.0;
        let mut var_qlo_dn6: f64 = 0.0;
        let mut var_qhi: f64 = 0.0;
        let mut var_qhi_rv: f64 = 0.0;
        let mut var_qhi_dn1: f64 = 0.0;
        let mut var_qhi_dn3: f64 = 0.0;
        let mut var_qhi_dn4: f64 = 0.0;
        let mut var_qhi_dn5: f64 = 0.0;
        let mut var_qhi_dn6: f64 = 0.0;
        let mut var_ttype: f64 = 0.0;
        let mut var_ttype_rv: f64 = 0.0;
        let mut var_tdev: f64 = 0.0;
        let mut var_tdev_rv: f64 = 0.0;
        let mut var_tdev_dn3: f64 = 0.0;
        let mut var_tnom: f64 = 0.0;
        let mut var_tnom_rv: f64 = 0.0;
        let mut var_tamb: f64 = 0.0;
        let mut var_tamb_rv: f64 = 0.0;
        let mut var_tamb_dn3: f64 = 0.0;
        let mut var_rt: f64 = 0.0;
        let mut var_rt_rv: f64 = 0.0;
        let mut var_rt_dn3: f64 = 0.0;
        let mut var_lnrt: f64 = 0.0;
        let mut var_lnrt_rv: f64 = 0.0;
        let mut var_lnrt_dn3: f64 = 0.0;
        let mut var_vt: f64 = 0.0;
        let mut var_vt_rv: f64 = 0.0;
        let mut var_vt_dn3: f64 = 0.0;
        let mut var_tbeta: f64 = 0.0;
        let mut var_tbeta_rv: f64 = 0.0;
        let mut var_tbeta_dn3: f64 = 0.0;
        let mut var_is_t: f64 = 0.0;
        let mut var_is_t_rv: f64 = 0.0;
        let mut var_is_t_dn3: f64 = 0.0;
        let mut var_ise_t: f64 = 0.0;
        let mut var_ise_t_rv: f64 = 0.0;
        let mut var_ise_t_dn3: f64 = 0.0;
        let mut var_isc_t: f64 = 0.0;
        let mut var_isc_t_rv: f64 = 0.0;
        let mut var_isc_t_dn3: f64 = 0.0;
        let mut var_cje_t: f64 = 0.0;
        let mut var_cje_t_rv: f64 = 0.0;
        let mut var_cje_t_dn3: f64 = 0.0;
        let mut var_cjc_t: f64 = 0.0;
        let mut var_cjc_t_rv: f64 = 0.0;
        let mut var_cjc_t_dn3: f64 = 0.0;
        let mut var_cjs_t: f64 = 0.0;
        let mut var_cjs_t_rv: f64 = 0.0;
        let mut var_cjs_t_dn3: f64 = 0.0;
        let mut var_vje_t: f64 = 0.0;
        let mut var_vje_t_rv: f64 = 0.0;
        let mut var_vje_t_dn3: f64 = 0.0;
        let mut var_vjc_t: f64 = 0.0;
        let mut var_vjc_t_rv: f64 = 0.0;
        let mut var_vjc_t_dn3: f64 = 0.0;
        let mut var_vjs_t: f64 = 0.0;
        let mut var_vjs_t_rv: f64 = 0.0;
        let mut var_vjs_t_dn3: f64 = 0.0;
        let mut var_ijbv_t: f64 = 0.0;
        let mut var_ijbv_t_rv: f64 = 0.0;
        let mut var_ijbv_t_dn3: f64 = 0.0;
        let mut var_ijbvc_t: f64 = 0.0;
        let mut var_ijbvc_t_rv: f64 = 0.0;
        let mut var_ijbvc_t_dn3: f64 = 0.0;
        let mut var_bvr_t: f64 = 0.0;
        let mut var_bvr_t_rv: f64 = 0.0;
        let mut var_bvr_t_dn3: f64 = 0.0;
        let mut var_theexp_t: f64 = 0.0;
        let mut var_theexp_t_rv: f64 = 0.0;
        let mut var_theexp_t_dn3: f64 = 0.0;
        let mut var_cje_i: f64 = 0.0;
        let mut var_cje_i_rv: f64 = 0.0;
        let mut var_cjc_i: f64 = 0.0;
        let mut var_cjc_i_rv: f64 = 0.0;
        let mut var_cjs_i: f64 = 0.0;
        let mut var_cjs_i_rv: f64 = 0.0;
        let mut var_ifwd: f64 = 0.0;
        let mut var_ifwd_rv: f64 = 0.0;
        let mut var_ifwd_dn3: f64 = 0.0;
        let mut var_ifwd_dn4: f64 = 0.0;
        let mut var_ifwd_dn5: f64 = 0.0;
        let mut var_ifwd_dn6: f64 = 0.0;
        let mut var_ibwd: f64 = 0.0;
        let mut var_ibwd_rv: f64 = 0.0;
        let mut var_ibwd_dn3: f64 = 0.0;
        let mut var_ibwd_dn4: f64 = 0.0;
        let mut var_ibwd_dn5: f64 = 0.0;
        let mut var_ibwd_dn6: f64 = 0.0;
        let mut var_ikq1: f64 = 0.0;
        let mut var_ikq1_rv: f64 = 0.0;
        let mut var_ikq1_dn4: f64 = 0.0;
        let mut var_ikq1_dn5: f64 = 0.0;
        let mut var_ikq1_dn6: f64 = 0.0;
        let mut var_kq2: f64 = 0.0;
        let mut var_kq2_rv: f64 = 0.0;
        let mut var_kq2_dn3: f64 = 0.0;
        let mut var_kq2_dn4: f64 = 0.0;
        let mut var_kq2_dn5: f64 = 0.0;
        let mut var_kq2_dn6: f64 = 0.0;
        let mut var_ikqb: f64 = 0.0;
        let mut var_ikqb_rv: f64 = 0.0;
        let mut var_ikqb_dn3: f64 = 0.0;
        let mut var_ikqb_dn4: f64 = 0.0;
        let mut var_ikqb_dn5: f64 = 0.0;
        let mut var_ikqb_dn6: f64 = 0.0;
        let mut var_itzf: f64 = 0.0;
        let mut var_itzf_rv: f64 = 0.0;
        let mut var_itzf_dn3: f64 = 0.0;
        let mut var_itzf_dn4: f64 = 0.0;
        let mut var_itzf_dn5: f64 = 0.0;
        let mut var_itzf_dn6: f64 = 0.0;
        let mut var_itr: f64 = 0.0;
        let mut var_itr_rv: f64 = 0.0;
        let mut var_itr_dn3: f64 = 0.0;
        let mut var_itr_dn4: f64 = 0.0;
        let mut var_itr_dn5: f64 = 0.0;
        let mut var_itr_dn6: f64 = 0.0;
        let mut var_tff: f64 = 0.0;
        let mut var_tff_rv: f64 = 0.0;
        let mut var_tff_dn1: f64 = 0.0;
        let mut var_tff_dn2: f64 = 0.0;
        let mut var_qde: f64 = 0.0;
        let mut var_qde_rv: f64 = 0.0;
        let mut var_qde_dn1: f64 = 0.0;
        let mut var_qde_dn2: f64 = 0.0;
        let mut var_qde_dn3: f64 = 0.0;
        let mut var_qde_dn4: f64 = 0.0;
        let mut var_qde_dn5: f64 = 0.0;
        let mut var_qde_dn6: f64 = 0.0;
        let mut var_qdc: f64 = 0.0;
        let mut var_qdc_rv: f64 = 0.0;
        let mut var_qdc_dn3: f64 = 0.0;
        let mut var_qdc_dn4: f64 = 0.0;
        let mut var_qdc_dn5: f64 = 0.0;
        let mut var_qdc_dn6: f64 = 0.0;
        let mut var_qjs: f64 = 0.0;
        let mut var_qjs_rv: f64 = 0.0;
        let mut var_qjs_dn2: f64 = 0.0;
        let mut var_qjs_dn3: f64 = 0.0;
        let mut var_qjs_dn4: f64 = 0.0;
        let mut var_qje: f64 = 0.0;
        let mut var_qje_rv: f64 = 0.0;
        let mut var_qje_dn1: f64 = 0.0;
        let mut var_qje_dn3: f64 = 0.0;
        let mut var_qje_dn4: f64 = 0.0;
        let mut var_qje_dn5: f64 = 0.0;
        let mut var_qje_dn6: f64 = 0.0;
        let mut var_qjcx: f64 = 0.0;
        let mut var_qjcx_rv: f64 = 0.0;
        let mut var_qjcx_dn1: f64 = 0.0;
        let mut var_qjcx_dn3: f64 = 0.0;
        let mut var_qjcx_dn4: f64 = 0.0;
        let mut var_qjcx_dn5: f64 = 0.0;
        let mut var_qjcx_dn6: f64 = 0.0;
        let mut var_qjcx_1: f64 = 0.0;
        let mut var_qjcx_1_rv: f64 = 0.0;
        let mut var_qjcx_1_dn1: f64 = 0.0;
        let mut var_qjcx_1_dn3: f64 = 0.0;
        let mut var_qjcx_1_dn4: f64 = 0.0;
        let mut var_qjcx_1_dn5: f64 = 0.0;
        let mut var_qjcx_1_dn6: f64 = 0.0;
        let mut var_qjci: f64 = 0.0;
        let mut var_qjci_rv: f64 = 0.0;
        let mut var_qjci_dn1: f64 = 0.0;
        let mut var_qjci_dn3: f64 = 0.0;
        let mut var_qjci_dn4: f64 = 0.0;
        let mut var_qjci_dn5: f64 = 0.0;
        let mut var_qjci_dn6: f64 = 0.0;
        let mut var_qjci_1: f64 = 0.0;
        let mut var_qjci_1_rv: f64 = 0.0;
        let mut var_qjci_1_dn1: f64 = 0.0;
        let mut var_qjci_1_dn3: f64 = 0.0;
        let mut var_qjci_1_dn4: f64 = 0.0;
        let mut var_qjci_1_dn5: f64 = 0.0;
        let mut var_qjci_1_dn6: f64 = 0.0;
        let mut var_qxf1: f64 = 0.0;
        let mut var_qxf1_rv: f64 = 0.0;
        let mut var_qxf1_dn3: f64 = 0.0;
        let mut var_qxf1_dn4: f64 = 0.0;
        let mut var_qxf1_dn5: f64 = 0.0;
        let mut var_qxf1_dn6: f64 = 0.0;
        let mut var_ovaf: f64 = 0.0;
        let mut var_ovaf_rv: f64 = 0.0;
        let mut var_ovar: f64 = 0.0;
        let mut var_ovar_rv: f64 = 0.0;
        let mut var_oikf: f64 = 0.0;
        let mut var_oikf_rv: f64 = 0.0;
        let mut var_oikf_dn4: f64 = 0.0;
        let mut var_oikf_dn5: f64 = 0.0;
        let mut var_oikr: f64 = 0.0;
        let mut var_oikr_rv: f64 = 0.0;
        let mut var_argt: f64 = 0.0;
        let mut var_argt_rv: f64 = 0.0;
        let mut var_argt_dn3: f64 = 0.0;
        let mut var_veci: f64 = 0.0;
        let mut var_veci_rv: f64 = 0.0;
        let mut var_veci_dn2: f64 = 0.0;
        let mut var_veci_dn4: f64 = 0.0;
        let mut var_vbiei: f64 = 0.0;
        let mut var_vbiei_rv: f64 = 0.0;
        let mut var_vbiei_dn5: f64 = 0.0;
        let mut var_vbiei_dn6: f64 = 0.0;
        let mut var_vbici: f64 = 0.0;
        let mut var_vbici_rv: f64 = 0.0;
        let mut var_vbici_dn4: f64 = 0.0;
        let mut var_vbici_dn5: f64 = 0.0;
        let mut var_vbci: f64 = 0.0;
        let mut var_vbci_rv: f64 = 0.0;
        let mut var_vbci_dn1: f64 = 0.0;
        let mut var_vbci_dn4: f64 = 0.0;
        let mut var_fact1: f64 = 0.0;
        let mut var_fact1_rv: f64 = 0.0;
        let mut var_fact2: f64 = 0.0;
        let mut var_fact2_rv: f64 = 0.0;
        let mut var_fact2_dn3: f64 = 0.0;
        let mut var_egfet: f64 = 0.0;
        let mut var_egfet_rv: f64 = 0.0;
        let mut var_egfet_dn3: f64 = 0.0;
        let mut var_arg0: f64 = 0.0;
        let mut var_arg0_rv: f64 = 0.0;
        let mut var_arg0_dn3: f64 = 0.0;
        let mut var_pbfact: f64 = 0.0;
        let mut var_pbfact_rv: f64 = 0.0;
        let mut var_pbfact_dn3: f64 = 0.0;
        let mut var_pbo: f64 = 0.0;
        let mut var_pbo_rv: f64 = 0.0;
        let mut var_pbo_dn3: f64 = 0.0;
        let mut var_gmaold: f64 = 0.0;
        let mut var_gmaold_rv: f64 = 0.0;
        let mut var_gmaold_dn3: f64 = 0.0;
        let mut var_gmanew: f64 = 0.0;
        let mut var_gmanew_rv: f64 = 0.0;
        let mut var_gmanew_dn3: f64 = 0.0;
        let mut var_cjt: f64 = 0.0;
        let mut var_cjt_rv: f64 = 0.0;
        let mut var_cjt_dn3: f64 = 0.0;
        let mut var_argbv: f64 = 0.0;
        let mut var_argbv_rv: f64 = 0.0;
        let mut var_argbv_dn3: f64 = 0.0;
        let mut var_argbv_dn4: f64 = 0.0;
        let mut var_argbv_dn5: f64 = 0.0;
        let mut var_argbv_dn6: f64 = 0.0;
        let mut var_argbvvt: f64 = 0.0;
        let mut var_argbvvt_rv: f64 = 0.0;
        let mut var_argbvvt_dn3: f64 = 0.0;
        let mut var_argtr: f64 = 0.0;
        let mut var_argtr_rv: f64 = 0.0;
        let mut var_argtr_dn3: f64 = 0.0;
        let mut var_isr_t: f64 = 0.0;
        let mut var_isr_t_rv: f64 = 0.0;
        let mut var_isr_t_dn3: f64 = 0.0;
        let mut var_dkqb: f64 = 0.0;
        let mut var_dkqb_rv: f64 = 0.0;
        let mut var_dkqb_dn3: f64 = 0.0;
        let mut var_dkqb_dn4: f64 = 0.0;
        let mut var_dkqb_dn5: f64 = 0.0;
        let mut var_dkqb_dn6: f64 = 0.0;
        let mut var_vtff: f64 = 0.0;
        let mut var_vtff_rv: f64 = 0.0;
        let mut var_vtff_dn1: f64 = 0.0;
        let mut var_vtff_dn2: f64 = 0.0;
        let mut var_vtff1: f64 = 0.0;
        let mut var_vtff1_rv: f64 = 0.0;
        let mut var_vtff1_dn1: f64 = 0.0;
        let mut var_vtff1_dn2: f64 = 0.0;
        let mut var_t0: f64 = 0.0;
        let mut var_t0_rv: f64 = 0.0;
        let mut var_t0_dn5: f64 = 0.0;
        let mut var_t0_dn6: f64 = 0.0;
        let mut var_guard3: f64 = 0.0;
        let mut var_guard3_rv: f64 = 0.0;
        let mut var_guard4: f64 = 0.0;
        let mut var_guard4_rv: f64 = 0.0;
        let mut var_guard5: f64 = 0.0;
        let mut var_guard5_rv: f64 = 0.0;
        let mut var_guard6: f64 = 0.0;
        let mut var_guard6_rv: f64 = 0.0;
        let mut var_guard7: f64 = 0.0;
        let mut var_guard7_rv: f64 = 0.0;
        let mut var_guard8: f64 = 0.0;
        let mut var_guard8_rv: f64 = 0.0;
        let mut var_guard9: f64 = 0.0;
        let mut var_guard9_rv: f64 = 0.0;
        let mut var_guard10: f64 = 0.0;
        let mut var_guard10_rv: f64 = 0.0;
        let mut var_guard11: f64 = 0.0;
        let mut var_guard11_rv: f64 = 0.0;
        let mut var_guard12: f64 = 0.0;
        let mut var_guard12_rv: f64 = 0.0;
        let mut var_guard13: f64 = 0.0;
        let mut var_guard13_rv: f64 = 0.0;
        let mut var_guard15: f64 = 0.0;
        let mut var_guard15_rv: f64 = 0.0;
        let mut var_guard16: f64 = 0.0;
        let mut var_guard16_rv: f64 = 0.0;
        let mut var_guard17: f64 = 0.0;
        let mut var_guard17_rv: f64 = 0.0;
        let mut var_guard18: f64 = 0.0;
        let mut var_guard18_rv: f64 = 0.0;
        let mut var_guard19: f64 = 0.0;
        let mut var_guard19_rv: f64 = 0.0;
        let mut var_guard20: f64 = 0.0;
        let mut var_guard20_rv: f64 = 0.0;
        let mut var_guard21: f64 = 0.0;
        let mut var_guard21_rv: f64 = 0.0;

        Self::stamp_reactive_block_0(ctx, p, nodes, &mut var_arg, &mut var_arg0, &mut var_arg0_dn3, &mut var_arg0_rv, &mut var_arg_dn3, &mut var_arg_dn4, &mut var_arg_dn5, &mut var_arg_dn6, &mut var_arg_rv, &mut var_argt, &mut var_argt_dn3, &mut var_argt_rv, &mut var_argtr, &mut var_argtr_dn3, &mut var_argtr_rv, &mut var_bvr_t, &mut var_bvr_t_dn3, &mut var_bvr_t_rv, &mut var_cjc_i, &mut var_cjc_i_rv, &mut var_cjc_t, &mut var_cjc_t_dn3, &mut var_cjc_t_rv, &mut var_cje_i, &mut var_cje_i_rv, &mut var_cje_t, &mut var_cje_t_dn3, &mut var_cje_t_rv, &mut var_cjs_i, &mut var_cjs_i_rv, &mut var_cjs_t, &mut var_cjs_t_dn3, &mut var_cjs_t_rv, &mut var_cjt, &mut var_cjt_dn3, &mut var_cjt_rv, &mut var_egfet, &mut var_egfet_dn3, &mut var_egfet_rv, &mut var_fact1, &mut var_fact1_rv, &mut var_fact2, &mut var_fact2_dn3, &mut var_fact2_rv, &mut var_gmanew, &mut var_gmanew_dn3, &mut var_gmanew_rv, &mut var_gmaold, &mut var_gmaold_dn3, &mut var_gmaold_rv, &mut var_guard3, &mut var_guard3_rv, &mut var_ijbv_t, &mut var_ijbv_t_dn3, &mut var_ijbv_t_rv, &mut var_ijbvc_t, &mut var_ijbvc_t_dn3, &mut var_ijbvc_t_rv, &mut var_is_t, &mut var_is_t_dn3, &mut var_is_t_rv, &mut var_isc_t, &mut var_isc_t_dn3, &mut var_isc_t_rv, &mut var_ise_t, &mut var_ise_t_dn3, &mut var_ise_t_rv, &mut var_isr_t, &mut var_isr_t_dn3, &mut var_isr_t_rv, &mut var_lnrt, &mut var_lnrt_dn3, &mut var_lnrt_rv, &mut var_oikf, &mut var_oikf_dn4, &mut var_oikf_dn5, &mut var_oikf_rv, &mut var_oikr, &mut var_oikr_rv, &mut var_ovaf, &mut var_ovaf_rv, &mut var_ovar, &mut var_ovar_rv, &mut var_pbfact, &mut var_pbfact_dn3, &mut var_pbfact_rv, &mut var_pbo, &mut var_pbo_dn3, &mut var_pbo_rv, &mut var_rt, &mut var_rt_dn3, &mut var_rt_rv, &mut var_tamb, &mut var_tamb_dn3, &mut var_tamb_rv, &mut var_tbeta, &mut var_tbeta_dn3, &mut var_tbeta_rv, &mut var_tdev, &mut var_tdev_dn3, &mut var_tdev_rv, &mut var_theexp_t, &mut var_theexp_t_dn3, &mut var_theexp_t_rv, &mut var_tnom, &mut var_tnom_rv, &mut var_ttype, &mut var_ttype_rv, &mut var_vbci, &mut var_vbci_dn1, &mut var_vbci_dn4, &mut var_vbci_rv, &mut var_vbici, &mut var_vbici_dn4, &mut var_vbici_dn5, &mut var_vbici_rv, &mut var_vbiei, &mut var_vbiei_dn5, &mut var_vbiei_dn6, &mut var_vbiei_rv, &mut var_veci, &mut var_veci_dn2, &mut var_veci_dn4, &mut var_veci_rv, &mut var_vjc_t, &mut var_vjc_t_dn3, &mut var_vjc_t_rv, &mut var_vje_t, &mut var_vje_t_dn3, &mut var_vje_t_rv, &mut var_vjs_t, &mut var_vjs_t_dn3, &mut var_vjs_t_rv, &mut var_vt, &mut var_vt_dn3, &mut var_vt_rv, &mut var_weff, &mut var_weff_rv);
        Self::stamp_reactive_block_1(p, var_bvr_t, var_bvr_t_dn3, var_guard3, var_ijbv_t, var_ijbv_t_dn3, var_is_t, var_is_t_dn3, var_ise_t, var_isr_t, var_theexp_t, var_theexp_t_dn3, var_vbici, var_vbici_dn4, var_vbici_dn5, var_vbiei, var_vbiei_dn5, var_vbiei_dn6, var_vt, var_vt_dn3, &mut var_arg, &mut var_arg_dn3, &mut var_arg_dn4, &mut var_arg_dn5, &mut var_arg_dn6, &mut var_arg_rv, &mut var_argbv, &mut var_argbv_dn3, &mut var_argbv_dn4, &mut var_argbv_dn5, &mut var_argbv_dn6, &mut var_argbv_rv, &mut var_argbvvt, &mut var_argbvvt_dn3, &mut var_argbvvt_rv, &mut var_guard4, &mut var_guard4_rv, &mut var_guard5, &mut var_guard5_rv, &mut var_guard6, &mut var_guard6_rv, &mut var_guard7, &mut var_guard7_rv, &mut var_guard8, &mut var_guard8_rv, &mut var_guard9, &mut var_guard9_rv, &mut var_ifwd, &mut var_ifwd_dn3, &mut var_ifwd_dn4, &mut var_ifwd_dn5, &mut var_ifwd_dn6, &mut var_ifwd_rv, &mut var_le, &mut var_le_dn3, &mut var_le_dn4, &mut var_le_dn5, &mut var_le_dn6, &mut var_le_rv, &mut var_lebv, &mut var_lebv_dn3, &mut var_lebv_dn4, &mut var_lebv_dn5, &mut var_lebv_dn6, &mut var_lebv_rv, &mut var_t0, &mut var_t0_dn5, &mut var_t0_dn6, &mut var_t0_rv);
        Self::stamp_reactive_block_2(ctx, p, nodes, var_bvr_t, var_bvr_t_dn3, var_guard9, var_ifwd, var_ifwd_dn3, var_ifwd_dn4, var_ifwd_dn5, var_ifwd_dn6, var_ijbvc_t, var_ijbvc_t_dn3, var_is_t, var_is_t_dn3, var_isc_t, var_oikr, var_ovaf, var_ovar, var_theexp_t, var_theexp_t_dn3, var_vbici, var_vbici_dn4, var_vbici_dn5, var_vbiei, var_vbiei_dn5, var_vbiei_dn6, var_veci, var_vt, var_vt_dn3, &mut var_arg, &mut var_arg_dn3, &mut var_arg_dn4, &mut var_arg_dn5, &mut var_arg_dn6, &mut var_arg_rv, &mut var_argbv, &mut var_argbv_dn3, &mut var_argbv_dn4, &mut var_argbv_dn5, &mut var_argbv_dn6, &mut var_argbv_rv, &mut var_argbvvt, &mut var_argbvvt_dn3, &mut var_argbvvt_rv, &mut var_dkqb, &mut var_dkqb_dn3, &mut var_dkqb_dn4, &mut var_dkqb_dn5, &mut var_dkqb_dn6, &mut var_dkqb_rv, &mut var_guard10, &mut var_guard10_rv, &mut var_guard11, &mut var_guard11_rv, &mut var_guard12, &mut var_guard12_rv, &mut var_guard13, &mut var_guard13_rv, &mut var_guard15, &mut var_guard15_rv, &mut var_ibwd, &mut var_ibwd_dn3, &mut var_ibwd_dn4, &mut var_ibwd_dn5, &mut var_ibwd_dn6, &mut var_ibwd_rv, &mut var_ikq1, &mut var_ikq1_dn4, &mut var_ikq1_dn5, &mut var_ikq1_dn6, &mut var_ikq1_rv, &mut var_ikqb, &mut var_ikqb_dn3, &mut var_ikqb_dn4, &mut var_ikqb_dn5, &mut var_ikqb_dn6, &mut var_ikqb_rv, &mut var_itr, &mut var_itr_dn3, &mut var_itr_dn4, &mut var_itr_dn5, &mut var_itr_dn6, &mut var_itr_rv, &mut var_itzf, &mut var_itzf_dn3, &mut var_itzf_dn4, &mut var_itzf_dn5, &mut var_itzf_dn6, &mut var_itzf_rv, &mut var_kq2, &mut var_kq2_dn3, &mut var_kq2_dn4, &mut var_kq2_dn5, &mut var_kq2_dn6, &mut var_kq2_rv, &mut var_le, &mut var_le_dn3, &mut var_le_dn4, &mut var_le_dn5, &mut var_le_dn6, &mut var_le_rv, &mut var_lebv, &mut var_lebv_dn3, &mut var_lebv_dn4, &mut var_lebv_dn5, &mut var_lebv_dn6, &mut var_lebv_rv, &mut var_oikf, &mut var_oikf_dn4, &mut var_oikf_dn5, &mut var_oikf_rv, &mut var_qdc, &mut var_qdc_dn3, &mut var_qdc_dn4, &mut var_qdc_dn5, &mut var_qdc_dn6, &mut var_qdc_rv, &mut var_qde, &mut var_qde_dn1, &mut var_qde_dn2, &mut var_qde_dn3, &mut var_qde_dn4, &mut var_qde_dn5, &mut var_qde_dn6, &mut var_qde_rv, &mut var_tff, &mut var_tff_dn1, &mut var_tff_dn2, &mut var_tff_rv, &mut var_vtff, &mut var_vtff1, &mut var_vtff1_dn1, &mut var_vtff1_dn2, &mut var_vtff1_rv, &mut var_vtff_dn1, &mut var_vtff_dn2, &mut var_vtff_rv);
        Self::stamp_reactive_block_3(p, var_cjc_t, var_cjc_t_dn3, var_cje_t, var_cje_t_dn3, var_cjs_t, var_cjs_t_dn3, var_guard15, var_itzf, var_itzf_dn3, var_itzf_dn4, var_itzf_dn5, var_itzf_dn6, var_ttype, var_vbci, var_vbci_dn1, var_vbci_dn4, var_vbici, var_vbici_dn4, var_vbici_dn5, var_vbiei, var_vbiei_dn5, var_vbiei_dn6, var_veci, var_veci_dn2, var_veci_dn4, var_vjc_t, var_vjc_t_dn3, var_vje_t, var_vje_t_dn3, var_vjs_t, var_vjs_t_dn3, &mut var_dv0, &mut var_dv0_dn3, &mut var_dv0_rv, &mut var_dvh, &mut var_dvh_dn1, &mut var_dvh_dn3, &mut var_dvh_dn4, &mut var_dvh_dn5, &mut var_dvh_dn6, &mut var_dvh_rv, &mut var_guard16, &mut var_guard16_rv, &mut var_guard17, &mut var_guard17_rv, &mut var_guard18, &mut var_guard18_rv, &mut var_guard19, &mut var_guard19_rv, &mut var_guard20, &mut var_guard20_rv, &mut var_guard21, &mut var_guard21_rv, &mut var_pwq, &mut var_pwq_rv, &mut var_qhi, &mut var_qhi_dn1, &mut var_qhi_dn3, &mut var_qhi_dn4, &mut var_qhi_dn5, &mut var_qhi_dn6, &mut var_qhi_rv, &mut var_qjci, &mut var_qjci_1, &mut var_qjci_1_dn1, &mut var_qjci_1_dn3, &mut var_qjci_1_dn4, &mut var_qjci_1_dn5, &mut var_qjci_1_dn6, &mut var_qjci_1_rv, &mut var_qjci_dn1, &mut var_qjci_dn3, &mut var_qjci_dn4, &mut var_qjci_dn5, &mut var_qjci_dn6, &mut var_qjci_rv, &mut var_qjcx, &mut var_qjcx_1, &mut var_qjcx_1_dn1, &mut var_qjcx_1_dn3, &mut var_qjcx_1_dn4, &mut var_qjcx_1_dn5, &mut var_qjcx_1_dn6, &mut var_qjcx_1_rv, &mut var_qjcx_dn1, &mut var_qjcx_dn3, &mut var_qjcx_dn4, &mut var_qjcx_dn5, &mut var_qjcx_dn6, &mut var_qjcx_rv, &mut var_qje, &mut var_qje_dn1, &mut var_qje_dn3, &mut var_qje_dn4, &mut var_qje_dn5, &mut var_qje_dn6, &mut var_qje_rv, &mut var_qjs, &mut var_qjs_dn2, &mut var_qjs_dn3, &mut var_qjs_dn4, &mut var_qjs_rv, &mut var_qlo, &mut var_qlo_dn1, &mut var_qlo_dn3, &mut var_qlo_dn4, &mut var_qlo_dn5, &mut var_qlo_dn6, &mut var_qlo_rv, &mut var_qxf1, &mut var_qxf1_dn3, &mut var_qxf1_dn4, &mut var_qxf1_dn5, &mut var_qxf1_dn6, &mut var_qxf1_rv);

        Self::stamp_reactive_equations_block_0(ctx, stamper, p, nodes, branches, multiplicity, var_guard13, var_guard20, var_guard21, var_qdc, var_qdc_dn3, var_qdc_dn4, var_qdc_dn5, var_qdc_dn6, var_qde, var_qde_dn1, var_qde_dn2, var_qde_dn3, var_qde_dn4, var_qde_dn5, var_qde_dn6, var_qjci_1, var_qjci_1_dn1, var_qjci_1_dn3, var_qjci_1_dn4, var_qjci_1_dn5, var_qjci_1_dn6, var_qjcx_1, var_qjcx_1_dn1, var_qjcx_1_dn3, var_qjcx_1_dn4, var_qjcx_1_dn5, var_qjcx_1_dn6, var_qje, var_qje_dn1, var_qje_dn3, var_qje_dn4, var_qje_dn5, var_qje_dn6, var_qjs, var_qjs_dn2, var_qjs_dn3, var_qjs_dn4, var_qxf1, var_qxf1_dn3, var_qxf1_dn4, var_qxf1_dn5, var_qxf1_dn6, var_tff, var_tff_dn1, var_tff_dn2, var_ttype, var_weff);
    }
}
