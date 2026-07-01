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

#[derive(Default)]
pub(crate) struct StampLocals {
    pub(crate) var_arg: f64,
    pub(crate) var_arg0: f64,
    pub(crate) var_arg0_dn2: f64,
    pub(crate) var_arg0_rv: f64,
    pub(crate) var_arg_dn2: f64,
    pub(crate) var_arg_dn3: f64,
    pub(crate) var_arg_dn4: f64,
    pub(crate) var_arg_rv: f64,
    pub(crate) var_argbv: f64,
    pub(crate) var_argbv_dn2: f64,
    pub(crate) var_argbv_dn3: f64,
    pub(crate) var_argbv_dn4: f64,
    pub(crate) var_argbv_rv: f64,
    pub(crate) var_argbvvt: f64,
    pub(crate) var_argbvvt_dn2: f64,
    pub(crate) var_argbvvt_rv: f64,
    pub(crate) var_argt: f64,
    pub(crate) var_argt_dn2: f64,
    pub(crate) var_argt_rv: f64,
    pub(crate) var_argtr: f64,
    pub(crate) var_argtr_dn2: f64,
    pub(crate) var_argtr_rv: f64,
    pub(crate) var_bvr_t: f64,
    pub(crate) var_bvr_t_dn2: f64,
    pub(crate) var_bvr_t_rv: f64,
    pub(crate) var_cje_i: f64,
    pub(crate) var_cje_i_rv: f64,
    pub(crate) var_cje_t: f64,
    pub(crate) var_cje_t_dn2: f64,
    pub(crate) var_cje_t_rv: f64,
    pub(crate) var_cjt: f64,
    pub(crate) var_cjt_dn2: f64,
    pub(crate) var_cjt_rv: f64,
    pub(crate) var_dv0: f64,
    pub(crate) var_dv0_dn2: f64,
    pub(crate) var_dv0_rv: f64,
    pub(crate) var_dvh: f64,
    pub(crate) var_dvh_dn2: f64,
    pub(crate) var_dvh_dn3: f64,
    pub(crate) var_dvh_dn4: f64,
    pub(crate) var_dvh_rv: f64,
    pub(crate) var_egfet: f64,
    pub(crate) var_egfet_dn2: f64,
    pub(crate) var_egfet_rv: f64,
    pub(crate) var_fact1: f64,
    pub(crate) var_fact1_rv: f64,
    pub(crate) var_fact2: f64,
    pub(crate) var_fact2_dn2: f64,
    pub(crate) var_fact2_rv: f64,
    pub(crate) var_gmanew: f64,
    pub(crate) var_gmanew_dn2: f64,
    pub(crate) var_gmanew_rv: f64,
    pub(crate) var_gmaold: f64,
    pub(crate) var_gmaold_dn2: f64,
    pub(crate) var_gmaold_rv: f64,
    pub(crate) var_guard10: f64,
    pub(crate) var_guard10_rv: f64,
    pub(crate) var_guard11: f64,
    pub(crate) var_guard11_rv: f64,
    pub(crate) var_guard12: f64,
    pub(crate) var_guard13: f64,
    pub(crate) var_guard14: f64,
    pub(crate) var_guard3: f64,
    pub(crate) var_guard3_rv: f64,
    pub(crate) var_guard4: f64,
    pub(crate) var_guard4_rv: f64,
    pub(crate) var_guard5: f64,
    pub(crate) var_guard5_rv: f64,
    pub(crate) var_guard6: f64,
    pub(crate) var_guard6_rv: f64,
    pub(crate) var_guard7: f64,
    pub(crate) var_guard8: f64,
    pub(crate) var_guard8_rv: f64,
    pub(crate) var_guard9: f64,
    pub(crate) var_guard9_rv: f64,
    pub(crate) var_ibe: f64,
    pub(crate) var_ibe_dn2: f64,
    pub(crate) var_ibe_dn3: f64,
    pub(crate) var_ibe_dn4: f64,
    pub(crate) var_ifwd: f64,
    pub(crate) var_ifwd_dn2: f64,
    pub(crate) var_ifwd_dn3: f64,
    pub(crate) var_ifwd_dn4: f64,
    pub(crate) var_ifwd_rv: f64,
    pub(crate) var_ijbv_t: f64,
    pub(crate) var_ijbv_t_dn2: f64,
    pub(crate) var_ijbv_t_rv: f64,
    pub(crate) var_is_t: f64,
    pub(crate) var_is_t_dn2: f64,
    pub(crate) var_is_t_rv: f64,
    pub(crate) var_isr_t: f64,
    pub(crate) var_isr_t_dn2: f64,
    pub(crate) var_isr_t_rv: f64,
    pub(crate) var_itrev: f64,
    pub(crate) var_itrev_dn2: f64,
    pub(crate) var_itrev_dn3: f64,
    pub(crate) var_itrev_dn4: f64,
    pub(crate) var_itzf: f64,
    pub(crate) var_itzf_dn2: f64,
    pub(crate) var_itzf_dn3: f64,
    pub(crate) var_itzf_dn4: f64,
    pub(crate) var_itzf_rv: f64,
    pub(crate) var_le: f64,
    pub(crate) var_le_dn2: f64,
    pub(crate) var_le_dn3: f64,
    pub(crate) var_le_dn4: f64,
    pub(crate) var_le_rv: f64,
    pub(crate) var_lebv: f64,
    pub(crate) var_lebv_dn2: f64,
    pub(crate) var_lebv_dn3: f64,
    pub(crate) var_lebv_dn4: f64,
    pub(crate) var_lebv_rv: f64,
    pub(crate) var_lnrt: f64,
    pub(crate) var_lnrt_dn2: f64,
    pub(crate) var_lnrt_rv: f64,
    pub(crate) var_pbfact: f64,
    pub(crate) var_pbfact_dn2: f64,
    pub(crate) var_pbfact_rv: f64,
    pub(crate) var_pbo: f64,
    pub(crate) var_pbo_dn2: f64,
    pub(crate) var_pbo_rv: f64,
    pub(crate) var_pwq: f64,
    pub(crate) var_pwq_rv: f64,
    pub(crate) var_qde: f64,
    pub(crate) var_qde_dn0: f64,
    pub(crate) var_qde_dn1: f64,
    pub(crate) var_qde_dn2: f64,
    pub(crate) var_qde_dn3: f64,
    pub(crate) var_qde_dn4: f64,
    pub(crate) var_qde_rv: f64,
    pub(crate) var_qhi: f64,
    pub(crate) var_qhi_dn2: f64,
    pub(crate) var_qhi_dn3: f64,
    pub(crate) var_qhi_dn4: f64,
    pub(crate) var_qhi_rv: f64,
    pub(crate) var_qje: f64,
    pub(crate) var_qje_dn2: f64,
    pub(crate) var_qje_dn3: f64,
    pub(crate) var_qje_dn4: f64,
    pub(crate) var_qje_rv: f64,
    pub(crate) var_qlo: f64,
    pub(crate) var_qlo_dn2: f64,
    pub(crate) var_qlo_dn3: f64,
    pub(crate) var_qlo_dn4: f64,
    pub(crate) var_qlo_rv: f64,
    pub(crate) var_rb: f64,
    pub(crate) var_rb_dn0: f64,
    pub(crate) var_rb_dn2: f64,
    pub(crate) var_rb_dn3: f64,
    pub(crate) var_rb_dn6: f64,
    pub(crate) var_rb_nom: f64,
    pub(crate) var_re: f64,
    pub(crate) var_re_dn1: f64,
    pub(crate) var_re_dn2: f64,
    pub(crate) var_re_dn4: f64,
    pub(crate) var_re_nom: f64,
    pub(crate) var_rt: f64,
    pub(crate) var_rt_dn2: f64,
    pub(crate) var_rt_rv: f64,
    pub(crate) var_t0: f64,
    pub(crate) var_t0_dn3: f64,
    pub(crate) var_t0_dn4: f64,
    pub(crate) var_t0_rv: f64,
    pub(crate) var_tamb: f64,
    pub(crate) var_tamb_dn2: f64,
    pub(crate) var_tamb_rv: f64,
    pub(crate) var_tdev: f64,
    pub(crate) var_tdev_dn2: f64,
    pub(crate) var_tdev_rv: f64,
    pub(crate) var_tff: f64,
    pub(crate) var_tff_dn0: f64,
    pub(crate) var_tff_dn1: f64,
    pub(crate) var_tff_rv: f64,
    pub(crate) var_theexp_t: f64,
    pub(crate) var_theexp_t_dn2: f64,
    pub(crate) var_theexp_t_rv: f64,
    pub(crate) var_tnom: f64,
    pub(crate) var_tnom_rv: f64,
    pub(crate) var_ttype: f64,
    pub(crate) var_ttype_rv: f64,
    pub(crate) var_vbbi: f64,
    pub(crate) var_vbbi_dn0: f64,
    pub(crate) var_vbbi_dn3: f64,
    pub(crate) var_vbesat: f64,
    pub(crate) var_vbesat_dn0: f64,
    pub(crate) var_vbesat_dn3: f64,
    pub(crate) var_vbiei: f64,
    pub(crate) var_vbiei_dn3: f64,
    pub(crate) var_vbiei_dn4: f64,
    pub(crate) var_vbiei_rv: f64,
    pub(crate) var_veei: f64,
    pub(crate) var_veei_dn1: f64,
    pub(crate) var_veei_dn4: f64,
    pub(crate) var_veesat: f64,
    pub(crate) var_veesat_dn1: f64,
    pub(crate) var_veesat_dn4: f64,
    pub(crate) var_vje_t: f64,
    pub(crate) var_vje_t_dn2: f64,
    pub(crate) var_vje_t_rv: f64,
    pub(crate) var_vt: f64,
    pub(crate) var_vt_dn2: f64,
    pub(crate) var_vt_rv: f64,
    pub(crate) var_vtff: f64,
    pub(crate) var_vtff1: f64,
    pub(crate) var_vtff1_dn0: f64,
    pub(crate) var_vtff1_dn1: f64,
    pub(crate) var_vtff1_rv: f64,
    pub(crate) var_vtff_dn0: f64,
    pub(crate) var_vtff_dn1: f64,
    pub(crate) var_vtff_rv: f64,
    pub(crate) var_weff: f64,
    pub(crate) var_weff_rv: f64,
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
        let mut locals = StampLocals::default();

        Self::stamp_transient_block_0(ctx, p, nodes, &mut locals);
        Self::stamp_transient_block_1(ctx, p, nodes, &mut locals);

        Self::stamp_transient_equations_block_0(ctx, stamper, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, &mut locals);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let multiplicity = (*self).multiplicity;
        let mut locals = StampLocals::default();

        Self::stamp_reactive_block_0(ctx, p, nodes, &mut locals);
        Self::stamp_reactive_block_1(ctx, p, nodes, &mut locals);

        Self::stamp_reactive_equations_block_0(ctx, stamper, p, nodes, branches, multiplicity, &mut locals);
    }
}
