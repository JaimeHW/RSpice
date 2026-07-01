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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv6 = ctx.node_voltage(nodes[6]);
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
        let v1: f64 = 0.0;
        let v5: f64 = nv3;
        let v6: f64 = nv4;
        let v7: f64 = (v5 - v6);
        let v42: f64 = nv6;
        let v43: f64 = (if self.scalar_v12 { v42 } else { v1 });
        let v54: f64 = (v1 * v7);
        let v60: f64 = -0.0;

        let d43_dn6: f64 = self.scalar_v59;
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * (v43),
            6,
            multiplicity * (d43_dn6),
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            self.scalar_v44,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            self.scalar_v45,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            self.scalar_v50,
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            self.scalar_v53,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            self.scalar_v53,
        );
        let d54_dn3: f64 = v1;
        let d54_dn4: f64 = v60;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(4),
            multiplicity * (v54),
            3,
            multiplicity * (d54_dn3),
            4,
            multiplicity * (d54_dn4),
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(3),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            self.scalar_v56,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            self.scalar_v58,
        );
        let mut var_arg: f64 = 0.0;
        let mut var_arg_dn2: f64 = 0.0;
        let mut var_arg_dn3: f64 = 0.0;
        let mut var_arg_dn4: f64 = 0.0;
        let mut var_le: f64 = 0.0;
        let mut var_le_dn2: f64 = 0.0;
        let mut var_le_dn3: f64 = 0.0;
        let mut var_le_dn4: f64 = 0.0;
        let mut var_lebv: f64 = 0.0;
        let mut var_lebv_dn2: f64 = 0.0;
        let mut var_lebv_dn3: f64 = 0.0;
        let mut var_lebv_dn4: f64 = 0.0;
        let mut var_weff: f64 = 0.0;
        let mut var_dv0: f64 = 0.0;
        let mut var_dv0_dn2: f64 = 0.0;
        let mut var_dvh: f64 = 0.0;
        let mut var_dvh_dn2: f64 = 0.0;
        let mut var_dvh_dn3: f64 = 0.0;
        let mut var_dvh_dn4: f64 = 0.0;
        let mut var_pwq: f64 = 0.0;
        let mut var_qlo: f64 = 0.0;
        let mut var_qlo_dn2: f64 = 0.0;
        let mut var_qlo_dn3: f64 = 0.0;
        let mut var_qlo_dn4: f64 = 0.0;
        let mut var_qhi: f64 = 0.0;
        let mut var_qhi_dn2: f64 = 0.0;
        let mut var_qhi_dn3: f64 = 0.0;
        let mut var_qhi_dn4: f64 = 0.0;
        let mut var_ttype: f64 = 0.0;
        let mut var_tdev: f64 = 0.0;
        let mut var_tdev_dn2: f64 = 0.0;
        let mut var_tnom: f64 = 0.0;
        let mut var_tamb: f64 = 0.0;
        let mut var_tamb_dn2: f64 = 0.0;
        let mut var_rt: f64 = 0.0;
        let mut var_rt_dn2: f64 = 0.0;
        let mut var_lnrt: f64 = 0.0;
        let mut var_lnrt_dn2: f64 = 0.0;
        let mut var_vt: f64 = 0.0;
        let mut var_vt_dn2: f64 = 0.0;
        let mut var_is_t: f64 = 0.0;
        let mut var_is_t_dn2: f64 = 0.0;
        let mut var_cje_t: f64 = 0.0;
        let mut var_cje_t_dn2: f64 = 0.0;
        let mut var_vje_t: f64 = 0.0;
        let mut var_vje_t_dn2: f64 = 0.0;
        let mut var_ijbv_t: f64 = 0.0;
        let mut var_ijbv_t_dn2: f64 = 0.0;
        let mut var_bvr_t: f64 = 0.0;
        let mut var_bvr_t_dn2: f64 = 0.0;
        let mut var_theexp_t: f64 = 0.0;
        let mut var_theexp_t_dn2: f64 = 0.0;
        let mut var_cje_i: f64 = 0.0;
        let mut var_ifwd: f64 = 0.0;
        let mut var_ifwd_dn2: f64 = 0.0;
        let mut var_ifwd_dn3: f64 = 0.0;
        let mut var_ifwd_dn4: f64 = 0.0;
        let mut var_ibe: f64 = 0.0;
        let mut var_ibe_dn2: f64 = 0.0;
        let mut var_ibe_dn3: f64 = 0.0;
        let mut var_ibe_dn4: f64 = 0.0;
        let mut var_itzf: f64 = 0.0;
        let mut var_itzf_dn2: f64 = 0.0;
        let mut var_itzf_dn3: f64 = 0.0;
        let mut var_itzf_dn4: f64 = 0.0;
        let mut var_itrev: f64 = 0.0;
        let mut var_itrev_dn2: f64 = 0.0;
        let mut var_itrev_dn3: f64 = 0.0;
        let mut var_itrev_dn4: f64 = 0.0;
        let mut var_re_nom: f64 = 0.0;
        let mut var_rb_nom: f64 = 0.0;
        let mut var_rb: f64 = 0.0;
        let mut var_rb_dn0: f64 = 0.0;
        let mut var_rb_dn2: f64 = 0.0;
        let mut var_rb_dn3: f64 = 0.0;
        let mut var_rb_dn6: f64 = 0.0;
        let mut var_re: f64 = 0.0;
        let mut var_re_dn1: f64 = 0.0;
        let mut var_re_dn2: f64 = 0.0;
        let mut var_re_dn4: f64 = 0.0;
        let mut var_tff: f64 = 0.0;
        let mut var_tff_dn0: f64 = 0.0;
        let mut var_tff_dn1: f64 = 0.0;
        let mut var_qde: f64 = 0.0;
        let mut var_qde_dn0: f64 = 0.0;
        let mut var_qde_dn1: f64 = 0.0;
        let mut var_qde_dn2: f64 = 0.0;
        let mut var_qde_dn3: f64 = 0.0;
        let mut var_qde_dn4: f64 = 0.0;
        let mut var_qje: f64 = 0.0;
        let mut var_qje_dn2: f64 = 0.0;
        let mut var_qje_dn3: f64 = 0.0;
        let mut var_qje_dn4: f64 = 0.0;
        let mut var_argt: f64 = 0.0;
        let mut var_argt_dn2: f64 = 0.0;
        let mut var_vbiei: f64 = 0.0;
        let mut var_vbiei_dn3: f64 = 0.0;
        let mut var_vbiei_dn4: f64 = 0.0;
        let mut var_vbbi: f64 = 0.0;
        let mut var_vbbi_dn0: f64 = 0.0;
        let mut var_vbbi_dn3: f64 = 0.0;
        let mut var_veei: f64 = 0.0;
        let mut var_veei_dn1: f64 = 0.0;
        let mut var_veei_dn4: f64 = 0.0;
        let mut var_fact1: f64 = 0.0;
        let mut var_fact2: f64 = 0.0;
        let mut var_fact2_dn2: f64 = 0.0;
        let mut var_egfet: f64 = 0.0;
        let mut var_egfet_dn2: f64 = 0.0;
        let mut var_arg0: f64 = 0.0;
        let mut var_arg0_dn2: f64 = 0.0;
        let mut var_pbfact: f64 = 0.0;
        let mut var_pbfact_dn2: f64 = 0.0;
        let mut var_pbo: f64 = 0.0;
        let mut var_pbo_dn2: f64 = 0.0;
        let mut var_gmaold: f64 = 0.0;
        let mut var_gmaold_dn2: f64 = 0.0;
        let mut var_gmanew: f64 = 0.0;
        let mut var_gmanew_dn2: f64 = 0.0;
        let mut var_cjt: f64 = 0.0;
        let mut var_cjt_dn2: f64 = 0.0;
        let mut var_argbv: f64 = 0.0;
        let mut var_argbv_dn2: f64 = 0.0;
        let mut var_argbv_dn3: f64 = 0.0;
        let mut var_argbv_dn4: f64 = 0.0;
        let mut var_argbvvt: f64 = 0.0;
        let mut var_argbvvt_dn2: f64 = 0.0;
        let mut var_argtr: f64 = 0.0;
        let mut var_argtr_dn2: f64 = 0.0;
        let mut var_isr_t: f64 = 0.0;
        let mut var_isr_t_dn2: f64 = 0.0;
        let mut var_vtff: f64 = 0.0;
        let mut var_vtff_dn0: f64 = 0.0;
        let mut var_vtff_dn1: f64 = 0.0;
        let mut var_vtff1: f64 = 0.0;
        let mut var_vtff1_dn0: f64 = 0.0;
        let mut var_vtff1_dn1: f64 = 0.0;
        let mut var_vbesat: f64 = 0.0;
        let mut var_vbesat_dn0: f64 = 0.0;
        let mut var_vbesat_dn3: f64 = 0.0;
        let mut var_veesat: f64 = 0.0;
        let mut var_veesat_dn1: f64 = 0.0;
        let mut var_veesat_dn4: f64 = 0.0;
        let mut var_t0: f64 = 0.0;
        let mut var_t0_dn3: f64 = 0.0;
        let mut var_t0_dn4: f64 = 0.0;
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

        Self::stamp_transient_block_0(ctx, p, nodes, &mut var_arg, &mut var_arg0, &mut var_arg0_dn2, &mut var_arg_dn2, &mut var_arg_dn3, &mut var_arg_dn4, &mut var_argbv, &mut var_argbv_dn2, &mut var_argbv_dn3, &mut var_argbv_dn4, &mut var_argbvvt, &mut var_argbvvt_dn2, &mut var_argt, &mut var_argt_dn2, &mut var_argtr, &mut var_argtr_dn2, &mut var_bvr_t, &mut var_bvr_t_dn2, &mut var_cje_i, &mut var_cje_t, &mut var_cje_t_dn2, &mut var_cjt, &mut var_cjt_dn2, &mut var_egfet, &mut var_egfet_dn2, &mut var_fact1, &mut var_fact2, &mut var_fact2_dn2, &mut var_gmanew, &mut var_gmanew_dn2, &mut var_gmaold, &mut var_gmaold_dn2, &mut var_guard3, &mut var_guard4, &mut var_guard5, &mut var_guard6, &mut var_ifwd, &mut var_ifwd_dn2, &mut var_ifwd_dn3, &mut var_ifwd_dn4, &mut var_ijbv_t, &mut var_ijbv_t_dn2, &mut var_is_t, &mut var_is_t_dn2, &mut var_isr_t, &mut var_isr_t_dn2, &mut var_itrev, &mut var_itrev_dn2, &mut var_itrev_dn3, &mut var_itrev_dn4, &mut var_le, &mut var_le_dn2, &mut var_le_dn3, &mut var_le_dn4, &mut var_lebv, &mut var_lebv_dn2, &mut var_lebv_dn3, &mut var_lebv_dn4, &mut var_lnrt, &mut var_lnrt_dn2, &mut var_pbfact, &mut var_pbfact_dn2, &mut var_pbo, &mut var_pbo_dn2, &mut var_rt, &mut var_rt_dn2, &mut var_t0, &mut var_t0_dn3, &mut var_t0_dn4, &mut var_tamb, &mut var_tamb_dn2, &mut var_tdev, &mut var_tdev_dn2, &mut var_theexp_t, &mut var_theexp_t_dn2, &mut var_tnom, &mut var_ttype, &mut var_vbbi, &mut var_vbbi_dn0, &mut var_vbbi_dn3, &mut var_vbiei, &mut var_vbiei_dn3, &mut var_vbiei_dn4, &mut var_veei, &mut var_veei_dn1, &mut var_veei_dn4, &mut var_vje_t, &mut var_vje_t_dn2, &mut var_vt, &mut var_vt_dn2, &mut var_weff);
        Self::stamp_transient_block_1(ctx, p, nodes, var_cje_t, var_cje_t_dn2, var_ifwd, var_ifwd_dn2, var_ifwd_dn3, var_ifwd_dn4, var_itrev, var_itrev_dn2, var_itrev_dn3, var_itrev_dn4, var_lnrt, var_lnrt_dn2, var_vbbi, var_vbbi_dn0, var_vbbi_dn3, var_vbiei, var_vbiei_dn3, var_vbiei_dn4, var_veei, var_veei_dn1, var_veei_dn4, var_vje_t, var_vje_t_dn2, var_weff, &mut var_dv0, &mut var_dv0_dn2, &mut var_dvh, &mut var_dvh_dn2, &mut var_dvh_dn3, &mut var_dvh_dn4, &mut var_guard10, &mut var_guard11, &mut var_guard12, &mut var_guard13, &mut var_guard14, &mut var_guard7, &mut var_guard8, &mut var_guard9, &mut var_ibe, &mut var_ibe_dn2, &mut var_ibe_dn3, &mut var_ibe_dn4, &mut var_itzf, &mut var_itzf_dn2, &mut var_itzf_dn3, &mut var_itzf_dn4, &mut var_pwq, &mut var_qde, &mut var_qde_dn0, &mut var_qde_dn1, &mut var_qde_dn2, &mut var_qde_dn3, &mut var_qde_dn4, &mut var_qhi, &mut var_qhi_dn2, &mut var_qhi_dn3, &mut var_qhi_dn4, &mut var_qje, &mut var_qje_dn2, &mut var_qje_dn3, &mut var_qje_dn4, &mut var_qlo, &mut var_qlo_dn2, &mut var_qlo_dn3, &mut var_qlo_dn4, &mut var_rb, &mut var_rb_dn0, &mut var_rb_dn2, &mut var_rb_dn3, &mut var_rb_dn6, &mut var_rb_nom, &mut var_re, &mut var_re_dn1, &mut var_re_dn2, &mut var_re_dn4, &mut var_re_nom, &mut var_tff, &mut var_tff_dn0, &mut var_tff_dn1, &mut var_vbesat, &mut var_vbesat_dn0, &mut var_vbesat_dn3, &mut var_veesat, &mut var_veesat_dn1, &mut var_veesat_dn4, &mut var_vtff, &mut var_vtff1, &mut var_vtff1_dn0, &mut var_vtff1_dn1, &mut var_vtff_dn0, &mut var_vtff_dn1);

        Self::stamp_transient_equations_block_0(ctx, stamper, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, var_guard10, var_guard11, var_guard12, var_guard13, var_guard14, var_guard8, var_ibe, var_ibe_dn2, var_ibe_dn3, var_ibe_dn4, var_ifwd, var_ifwd_dn2, var_ifwd_dn3, var_ifwd_dn4, var_qde, var_qde_dn0, var_qde_dn1, var_qde_dn2, var_qde_dn3, var_qde_dn4, var_qje, var_qje_dn2, var_qje_dn3, var_qje_dn4, var_rb, var_rb_dn0, var_rb_dn2, var_rb_dn3, var_rb_dn6, var_re, var_re_dn1, var_re_dn2, var_re_dn4, var_tff, var_tff_dn0, var_tff_dn1, var_ttype, var_weff);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let multiplicity = (*self).multiplicity;
        let mut var_arg: f64 = 0.0;
        let mut var_arg_rv: f64 = 0.0;
        let mut var_arg_dn2: f64 = 0.0;
        let mut var_arg_dn3: f64 = 0.0;
        let mut var_arg_dn4: f64 = 0.0;
        let mut var_le: f64 = 0.0;
        let mut var_le_rv: f64 = 0.0;
        let mut var_le_dn2: f64 = 0.0;
        let mut var_le_dn3: f64 = 0.0;
        let mut var_le_dn4: f64 = 0.0;
        let mut var_lebv: f64 = 0.0;
        let mut var_lebv_rv: f64 = 0.0;
        let mut var_lebv_dn2: f64 = 0.0;
        let mut var_lebv_dn3: f64 = 0.0;
        let mut var_lebv_dn4: f64 = 0.0;
        let mut var_weff: f64 = 0.0;
        let mut var_weff_rv: f64 = 0.0;
        let mut var_dv0: f64 = 0.0;
        let mut var_dv0_rv: f64 = 0.0;
        let mut var_dv0_dn2: f64 = 0.0;
        let mut var_dvh: f64 = 0.0;
        let mut var_dvh_rv: f64 = 0.0;
        let mut var_dvh_dn2: f64 = 0.0;
        let mut var_dvh_dn3: f64 = 0.0;
        let mut var_dvh_dn4: f64 = 0.0;
        let mut var_pwq: f64 = 0.0;
        let mut var_pwq_rv: f64 = 0.0;
        let mut var_qlo: f64 = 0.0;
        let mut var_qlo_rv: f64 = 0.0;
        let mut var_qlo_dn2: f64 = 0.0;
        let mut var_qlo_dn3: f64 = 0.0;
        let mut var_qlo_dn4: f64 = 0.0;
        let mut var_qhi: f64 = 0.0;
        let mut var_qhi_rv: f64 = 0.0;
        let mut var_qhi_dn2: f64 = 0.0;
        let mut var_qhi_dn3: f64 = 0.0;
        let mut var_qhi_dn4: f64 = 0.0;
        let mut var_ttype: f64 = 0.0;
        let mut var_ttype_rv: f64 = 0.0;
        let mut var_tdev: f64 = 0.0;
        let mut var_tdev_rv: f64 = 0.0;
        let mut var_tdev_dn2: f64 = 0.0;
        let mut var_tnom: f64 = 0.0;
        let mut var_tnom_rv: f64 = 0.0;
        let mut var_tamb: f64 = 0.0;
        let mut var_tamb_rv: f64 = 0.0;
        let mut var_tamb_dn2: f64 = 0.0;
        let mut var_rt: f64 = 0.0;
        let mut var_rt_rv: f64 = 0.0;
        let mut var_rt_dn2: f64 = 0.0;
        let mut var_lnrt: f64 = 0.0;
        let mut var_lnrt_rv: f64 = 0.0;
        let mut var_lnrt_dn2: f64 = 0.0;
        let mut var_vt: f64 = 0.0;
        let mut var_vt_rv: f64 = 0.0;
        let mut var_vt_dn2: f64 = 0.0;
        let mut var_is_t: f64 = 0.0;
        let mut var_is_t_rv: f64 = 0.0;
        let mut var_is_t_dn2: f64 = 0.0;
        let mut var_cje_t: f64 = 0.0;
        let mut var_cje_t_rv: f64 = 0.0;
        let mut var_cje_t_dn2: f64 = 0.0;
        let mut var_vje_t: f64 = 0.0;
        let mut var_vje_t_rv: f64 = 0.0;
        let mut var_vje_t_dn2: f64 = 0.0;
        let mut var_ijbv_t: f64 = 0.0;
        let mut var_ijbv_t_rv: f64 = 0.0;
        let mut var_ijbv_t_dn2: f64 = 0.0;
        let mut var_bvr_t: f64 = 0.0;
        let mut var_bvr_t_rv: f64 = 0.0;
        let mut var_bvr_t_dn2: f64 = 0.0;
        let mut var_theexp_t: f64 = 0.0;
        let mut var_theexp_t_rv: f64 = 0.0;
        let mut var_theexp_t_dn2: f64 = 0.0;
        let mut var_cje_i: f64 = 0.0;
        let mut var_cje_i_rv: f64 = 0.0;
        let mut var_ifwd: f64 = 0.0;
        let mut var_ifwd_rv: f64 = 0.0;
        let mut var_ifwd_dn2: f64 = 0.0;
        let mut var_ifwd_dn3: f64 = 0.0;
        let mut var_ifwd_dn4: f64 = 0.0;
        let mut var_itzf: f64 = 0.0;
        let mut var_itzf_rv: f64 = 0.0;
        let mut var_itzf_dn2: f64 = 0.0;
        let mut var_itzf_dn3: f64 = 0.0;
        let mut var_itzf_dn4: f64 = 0.0;
        let mut var_tff: f64 = 0.0;
        let mut var_tff_rv: f64 = 0.0;
        let mut var_tff_dn0: f64 = 0.0;
        let mut var_tff_dn1: f64 = 0.0;
        let mut var_qde: f64 = 0.0;
        let mut var_qde_rv: f64 = 0.0;
        let mut var_qde_dn0: f64 = 0.0;
        let mut var_qde_dn1: f64 = 0.0;
        let mut var_qde_dn2: f64 = 0.0;
        let mut var_qde_dn3: f64 = 0.0;
        let mut var_qde_dn4: f64 = 0.0;
        let mut var_qje: f64 = 0.0;
        let mut var_qje_rv: f64 = 0.0;
        let mut var_qje_dn2: f64 = 0.0;
        let mut var_qje_dn3: f64 = 0.0;
        let mut var_qje_dn4: f64 = 0.0;
        let mut var_argt: f64 = 0.0;
        let mut var_argt_rv: f64 = 0.0;
        let mut var_argt_dn2: f64 = 0.0;
        let mut var_vbiei: f64 = 0.0;
        let mut var_vbiei_rv: f64 = 0.0;
        let mut var_vbiei_dn3: f64 = 0.0;
        let mut var_vbiei_dn4: f64 = 0.0;
        let mut var_fact1: f64 = 0.0;
        let mut var_fact1_rv: f64 = 0.0;
        let mut var_fact2: f64 = 0.0;
        let mut var_fact2_rv: f64 = 0.0;
        let mut var_fact2_dn2: f64 = 0.0;
        let mut var_egfet: f64 = 0.0;
        let mut var_egfet_rv: f64 = 0.0;
        let mut var_egfet_dn2: f64 = 0.0;
        let mut var_arg0: f64 = 0.0;
        let mut var_arg0_rv: f64 = 0.0;
        let mut var_arg0_dn2: f64 = 0.0;
        let mut var_pbfact: f64 = 0.0;
        let mut var_pbfact_rv: f64 = 0.0;
        let mut var_pbfact_dn2: f64 = 0.0;
        let mut var_pbo: f64 = 0.0;
        let mut var_pbo_rv: f64 = 0.0;
        let mut var_pbo_dn2: f64 = 0.0;
        let mut var_gmaold: f64 = 0.0;
        let mut var_gmaold_rv: f64 = 0.0;
        let mut var_gmaold_dn2: f64 = 0.0;
        let mut var_gmanew: f64 = 0.0;
        let mut var_gmanew_rv: f64 = 0.0;
        let mut var_gmanew_dn2: f64 = 0.0;
        let mut var_cjt: f64 = 0.0;
        let mut var_cjt_rv: f64 = 0.0;
        let mut var_cjt_dn2: f64 = 0.0;
        let mut var_argbv: f64 = 0.0;
        let mut var_argbv_rv: f64 = 0.0;
        let mut var_argbv_dn2: f64 = 0.0;
        let mut var_argbv_dn3: f64 = 0.0;
        let mut var_argbv_dn4: f64 = 0.0;
        let mut var_argbvvt: f64 = 0.0;
        let mut var_argbvvt_rv: f64 = 0.0;
        let mut var_argbvvt_dn2: f64 = 0.0;
        let mut var_argtr: f64 = 0.0;
        let mut var_argtr_rv: f64 = 0.0;
        let mut var_argtr_dn2: f64 = 0.0;
        let mut var_isr_t: f64 = 0.0;
        let mut var_isr_t_rv: f64 = 0.0;
        let mut var_isr_t_dn2: f64 = 0.0;
        let mut var_vtff: f64 = 0.0;
        let mut var_vtff_rv: f64 = 0.0;
        let mut var_vtff_dn0: f64 = 0.0;
        let mut var_vtff_dn1: f64 = 0.0;
        let mut var_vtff1: f64 = 0.0;
        let mut var_vtff1_rv: f64 = 0.0;
        let mut var_vtff1_dn0: f64 = 0.0;
        let mut var_vtff1_dn1: f64 = 0.0;
        let mut var_t0: f64 = 0.0;
        let mut var_t0_rv: f64 = 0.0;
        let mut var_t0_dn3: f64 = 0.0;
        let mut var_t0_dn4: f64 = 0.0;
        let mut var_guard3: f64 = 0.0;
        let mut var_guard3_rv: f64 = 0.0;
        let mut var_guard4: f64 = 0.0;
        let mut var_guard4_rv: f64 = 0.0;
        let mut var_guard5: f64 = 0.0;
        let mut var_guard5_rv: f64 = 0.0;
        let mut var_guard6: f64 = 0.0;
        let mut var_guard6_rv: f64 = 0.0;
        let mut var_guard8: f64 = 0.0;
        let mut var_guard8_rv: f64 = 0.0;
        let mut var_guard9: f64 = 0.0;
        let mut var_guard9_rv: f64 = 0.0;
        let mut var_guard10: f64 = 0.0;
        let mut var_guard10_rv: f64 = 0.0;
        let mut var_guard11: f64 = 0.0;
        let mut var_guard11_rv: f64 = 0.0;

        Self::stamp_reactive_block_0(ctx, p, nodes, &mut var_arg, &mut var_arg0, &mut var_arg0_dn2, &mut var_arg0_rv, &mut var_arg_dn2, &mut var_arg_dn3, &mut var_arg_dn4, &mut var_arg_rv, &mut var_argbv, &mut var_argbv_dn2, &mut var_argbv_dn3, &mut var_argbv_dn4, &mut var_argbv_rv, &mut var_argbvvt, &mut var_argbvvt_dn2, &mut var_argbvvt_rv, &mut var_argt, &mut var_argt_dn2, &mut var_argt_rv, &mut var_argtr, &mut var_argtr_dn2, &mut var_argtr_rv, &mut var_bvr_t, &mut var_bvr_t_dn2, &mut var_bvr_t_rv, &mut var_cje_i, &mut var_cje_i_rv, &mut var_cje_t, &mut var_cje_t_dn2, &mut var_cje_t_rv, &mut var_cjt, &mut var_cjt_dn2, &mut var_cjt_rv, &mut var_egfet, &mut var_egfet_dn2, &mut var_egfet_rv, &mut var_fact1, &mut var_fact1_rv, &mut var_fact2, &mut var_fact2_dn2, &mut var_fact2_rv, &mut var_gmanew, &mut var_gmanew_dn2, &mut var_gmanew_rv, &mut var_gmaold, &mut var_gmaold_dn2, &mut var_gmaold_rv, &mut var_guard3, &mut var_guard3_rv, &mut var_guard4, &mut var_guard4_rv, &mut var_guard5, &mut var_guard5_rv, &mut var_guard6, &mut var_guard6_rv, &mut var_ifwd, &mut var_ifwd_dn2, &mut var_ifwd_dn3, &mut var_ifwd_dn4, &mut var_ifwd_rv, &mut var_ijbv_t, &mut var_ijbv_t_dn2, &mut var_ijbv_t_rv, &mut var_is_t, &mut var_is_t_dn2, &mut var_is_t_rv, &mut var_isr_t, &mut var_isr_t_dn2, &mut var_isr_t_rv, &mut var_le, &mut var_le_dn2, &mut var_le_dn3, &mut var_le_dn4, &mut var_le_rv, &mut var_lebv, &mut var_lebv_dn2, &mut var_lebv_dn3, &mut var_lebv_dn4, &mut var_lebv_rv, &mut var_lnrt, &mut var_lnrt_dn2, &mut var_lnrt_rv, &mut var_pbfact, &mut var_pbfact_dn2, &mut var_pbfact_rv, &mut var_pbo, &mut var_pbo_dn2, &mut var_pbo_rv, &mut var_rt, &mut var_rt_dn2, &mut var_rt_rv, &mut var_t0, &mut var_t0_dn3, &mut var_t0_dn4, &mut var_t0_rv, &mut var_tamb, &mut var_tamb_dn2, &mut var_tamb_rv, &mut var_tdev, &mut var_tdev_dn2, &mut var_tdev_rv, &mut var_theexp_t, &mut var_theexp_t_dn2, &mut var_theexp_t_rv, &mut var_tnom, &mut var_tnom_rv, &mut var_ttype, &mut var_ttype_rv, &mut var_vbiei, &mut var_vbiei_dn3, &mut var_vbiei_dn4, &mut var_vbiei_rv, &mut var_vje_t, &mut var_vje_t_dn2, &mut var_vje_t_rv, &mut var_vt, &mut var_vt_dn2, &mut var_vt_rv, &mut var_weff, &mut var_weff_rv);
        Self::stamp_reactive_block_1(ctx, p, nodes, var_arg, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_cje_t, var_cje_t_dn2, var_guard5, var_ifwd, var_ifwd_dn2, var_ifwd_dn3, var_ifwd_dn4, var_vbiei, var_vbiei_dn3, var_vbiei_dn4, var_vje_t, var_vje_t_dn2, &mut var_dv0, &mut var_dv0_dn2, &mut var_dv0_rv, &mut var_dvh, &mut var_dvh_dn2, &mut var_dvh_dn3, &mut var_dvh_dn4, &mut var_dvh_rv, &mut var_guard10, &mut var_guard10_rv, &mut var_guard11, &mut var_guard11_rv, &mut var_guard8, &mut var_guard8_rv, &mut var_guard9, &mut var_guard9_rv, &mut var_itzf, &mut var_itzf_dn2, &mut var_itzf_dn3, &mut var_itzf_dn4, &mut var_itzf_rv, &mut var_le, &mut var_le_dn2, &mut var_le_dn3, &mut var_le_dn4, &mut var_le_rv, &mut var_pwq, &mut var_pwq_rv, &mut var_qde, &mut var_qde_dn0, &mut var_qde_dn1, &mut var_qde_dn2, &mut var_qde_dn3, &mut var_qde_dn4, &mut var_qde_rv, &mut var_qhi, &mut var_qhi_dn2, &mut var_qhi_dn3, &mut var_qhi_dn4, &mut var_qhi_rv, &mut var_qje, &mut var_qje_dn2, &mut var_qje_dn3, &mut var_qje_dn4, &mut var_qje_rv, &mut var_qlo, &mut var_qlo_dn2, &mut var_qlo_dn3, &mut var_qlo_dn4, &mut var_qlo_rv, &mut var_tff, &mut var_tff_dn0, &mut var_tff_dn1, &mut var_tff_rv, &mut var_vtff, &mut var_vtff1, &mut var_vtff1_dn0, &mut var_vtff1_dn1, &mut var_vtff1_rv, &mut var_vtff_dn0, &mut var_vtff_dn1, &mut var_vtff_rv);

        Self::stamp_reactive_equations_block_0(ctx, stamper, p, nodes, branches, multiplicity, var_guard10, var_guard11, var_guard8, var_qde, var_qde_dn0, var_qde_dn1, var_qde_dn2, var_qde_dn3, var_qde_dn4, var_qje, var_qje_dn2, var_qje_dn3, var_qje_dn4, var_tff, var_tff_dn0, var_tff_dn1, var_ttype, var_weff);
    }
}
