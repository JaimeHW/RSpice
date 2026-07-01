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
        let param_given = self.param_given.as_ref();
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
        let mut var_v: f64 = 0.0;
        let mut var_v_dn0: f64 = 0.0;
        let mut var_v_dn1: f64 = 0.0;
        let mut var_i: f64 = 0.0;
        let mut var_i_dn0: f64 = 0.0;
        let mut var_i_dn1: f64 = 0.0;
        let mut var_i_dn2: f64 = 0.0;
        let mut var_leff_um: f64 = 0.0;
        let mut var_weff_um: f64 = 0.0;
        let mut var_r0: f64 = 0.0;
        let mut var_r_dc: f64 = 0.0;
        let mut var_r_dc_dn0: f64 = 0.0;
        let mut var_r_dc_dn1: f64 = 0.0;
        let mut var_r_dc_dn2: f64 = 0.0;
        let mut var_cth: f64 = 0.0;
        let mut var_tinik: f64 = 0.0;
        let mut var_tdevk: f64 = 0.0;
        let mut var_tdevk_dn2: f64 = 0.0;
        let mut var_scalefac: f64 = 0.0;
        let mut var_shrinkl: f64 = 0.0;
        let mut var_delt: f64 = 0.0;
        let mut var_delt_dn2: f64 = 0.0;
        let mut var_tcr: f64 = 0.0;
        let mut var_tcr_dn2: f64 = 0.0;
        let mut var_xleff: f64 = 0.0;
        let mut var_lfactor: f64 = 0.0;
        let mut var_l_um: f64 = 0.0;
        let mut var_w_um: f64 = 0.0;
        let mut var_l_umfore: f64 = 0.0;
        let mut var_g0: f64 = 0.0;
        let mut var_r0_t: f64 = 0.0;
        let mut var_r0_t_dn2: f64 = 0.0;
        let mut var_sqrf: f64 = 0.0;
        let mut var_sqrf_dn0: f64 = 0.0;
        let mut var_sqrf_dn1: f64 = 0.0;
        let mut var_cbrf: f64 = 0.0;
        let mut var_cbrf_dn0: f64 = 0.0;
        let mut var_cbrf_dn1: f64 = 0.0;
        let mut var_tdevc: f64 = 0.0;
        let mut var_tdevc_dn2: f64 = 0.0;
        let mut var_rfactor: f64 = 0.0;
        let mut var_rfactor_dn0: f64 = 0.0;
        let mut var_rfactor_dn1: f64 = 0.0;
        let mut var_vin: f64 = 0.0;
        let mut var_vin_dn0: f64 = 0.0;
        let mut var_vin_dn1: f64 = 0.0;
        let mut var_e: f64 = 0.0;
        let mut var_e_dn0: f64 = 0.0;
        let mut var_e_dn1: f64 = 0.0;
        let mut var_q2e: f64 = 0.0;
        let mut var_q2e_dn0: f64 = 0.0;
        let mut var_q2e_dn1: f64 = 0.0;
        let mut var_q3e: f64 = 0.0;
        let mut var_q3e_dn0: f64 = 0.0;
        let mut var_q3e_dn1: f64 = 0.0;
        let mut var_tc1e: f64 = 0.0;
        let mut var_tc2e: f64 = 0.0;
        let mut var_gth: f64 = 0.0;
        let mut var_vrth: f64 = 0.0;
        let mut var_vrth_dn2: f64 = 0.0;
        let mut var_ith: f64 = 0.0;
        let mut var_ith_dn0: f64 = 0.0;
        let mut var_ith_dn1: f64 = 0.0;
        let mut var_ith_dn2: f64 = 0.0;
        let mut var_irth: f64 = 0.0;
        let mut var_irth_dn2: f64 = 0.0;
        let mut var_qcth: f64 = 0.0;
        let mut var_qcth_dn2: f64 = 0.0;
        let mut var_p_um: f64 = 0.0;
        let mut var_a_um2: f64 = 0.0;
        let mut var_guard41: f64 = 0.0;
        let mut var_guard42: f64 = 0.0;
        let mut var_guard46: f64 = 0.0;
        let mut var_guard47: f64 = 0.0;
        let mut var_guard48: f64 = 0.0;
        let mut var_guard49: f64 = 0.0;
        let mut var_guard51: f64 = 0.0;
        let mut var_guard53: f64 = 0.0;
        let mut var_guard54: f64 = 0.0;
        let mut var_guard55: f64 = 0.0;
        let mut var_guard57: f64 = 0.0;
        let mut var_guard59: f64 = 0.0;
        let mut var_guard60: f64 = 0.0;
        let mut var_guard62: f64 = 0.0;
        let mut var_guard64: f64 = 0.0;
        let mut var_guard70: f64 = 0.0;
        let mut var_guard71: f64 = 0.0;
        let mut var_guard72: f64 = 0.0;
        let mut var_guard73: f64 = 0.0;
        let mut var_guard75: f64 = 0.0;
        let mut var_guard76: f64 = 0.0;
        let mut var_guard78: f64 = 0.0;
        let mut var_guard79: f64 = 0.0;
        let mut var_guard80: f64 = 0.0;
        let mut var_guard82: f64 = 0.0;
        let mut var_guard89: f64 = 0.0;

        Self::stamp_transient_block_0(ctx, p, param_given, &mut var_g0, &mut var_guard41, &mut var_guard42, &mut var_guard46, &mut var_guard47, &mut var_guard48, &mut var_guard49, &mut var_guard51, &mut var_guard53, &mut var_guard54, &mut var_guard55, &mut var_guard57, &mut var_guard59, &mut var_l_um, &mut var_leff_um, &mut var_lfactor, &mut var_r0, &mut var_scalefac, &mut var_shrinkl, &mut var_tdevc, &mut var_tdevc_dn2, &mut var_tinik, &mut var_w_um, &mut var_weff_um, &mut var_xleff);
        Self::stamp_transient_block_1(ctx, p, nodes, var_guard48, var_guard53, var_guard59, var_lfactor, var_tinik, var_xleff, &mut var_a_um2, &mut var_cth, &mut var_delt, &mut var_delt_dn2, &mut var_e, &mut var_e_dn0, &mut var_e_dn1, &mut var_g0, &mut var_gth, &mut var_guard60, &mut var_guard62, &mut var_guard64, &mut var_guard70, &mut var_guard71, &mut var_guard72, &mut var_guard73, &mut var_guard75, &mut var_guard76, &mut var_guard78, &mut var_guard79, &mut var_guard80, &mut var_guard82, &mut var_l_um, &mut var_l_umfore, &mut var_leff_um, &mut var_p_um, &mut var_q2e, &mut var_q2e_dn0, &mut var_q2e_dn1, &mut var_q3e, &mut var_q3e_dn0, &mut var_q3e_dn1, &mut var_r0, &mut var_r0_t, &mut var_r0_t_dn2, &mut var_sqrf, &mut var_sqrf_dn0, &mut var_sqrf_dn1, &mut var_tc1e, &mut var_tc2e, &mut var_tcr, &mut var_tcr_dn2, &mut var_tdevc, &mut var_tdevc_dn2, &mut var_tdevk, &mut var_tdevk_dn2, &mut var_vin, &mut var_vin_dn0, &mut var_vin_dn1, &mut var_vrth, &mut var_vrth_dn2, &mut var_w_um, &mut var_weff_um);
        Self::stamp_transient_block_2(p, var_cth, var_g0, var_gth, var_guard82, var_q3e, var_q3e_dn0, var_q3e_dn1, var_r0, var_r0_t, var_r0_t_dn2, var_sqrf, var_sqrf_dn0, var_sqrf_dn1, var_vin, var_vin_dn0, var_vin_dn1, var_vrth, var_vrth_dn2, &mut var_cbrf, &mut var_cbrf_dn0, &mut var_cbrf_dn1, &mut var_guard89, &mut var_i, &mut var_i_dn0, &mut var_i_dn1, &mut var_i_dn2, &mut var_irth, &mut var_irth_dn2, &mut var_ith, &mut var_ith_dn0, &mut var_ith_dn1, &mut var_ith_dn2, &mut var_qcth, &mut var_qcth_dn2, &mut var_r_dc, &mut var_r_dc_dn0, &mut var_r_dc_dn1, &mut var_r_dc_dn2, &mut var_rfactor, &mut var_rfactor_dn0, &mut var_rfactor_dn1, &mut var_v, &mut var_v_dn0, &mut var_v_dn1);

        let eq0_value: f64 = var_i;
        stamper.stamp_current_node3_local(
            Some(0),
            Some(1),
            multiplicity * (eq0_value),
            0,
            multiplicity * (var_i_dn0),
            1,
            multiplicity * (var_i_dn1),
            2,
            multiplicity * (var_i_dn2),
        );
        let (eq1_e56, eq1_e56_d_n2,) = {
    if (p.p7 != 0.0) {
        (var_irth, var_irth_dn2,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e56;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (eq1_value),
            2,
            multiplicity * (eq1_e56_d_n2),
        );
        let (eq2_e60, eq2_e60_d_n0, eq2_e60_d_n1, eq2_e60_d_n2,) = {
    if (p.p7 != 0.0) {
        (var_ith, var_ith_dn0, var_ith_dn1, var_ith_dn2,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e60;
        stamper.stamp_current_node3_local(
            Some(2),
            None,
            multiplicity * (eq2_value),
            0,
            multiplicity * (eq2_e60_d_n0),
            1,
            multiplicity * (eq2_e60_d_n1),
            2,
            multiplicity * (eq2_e60_d_n2),
        );
        let (eq3_e67, eq3_e67_d_n2,) = {
    if (p.p7 == 0.0) {
        let eq3_e65: f64 = (1000000.0 * var_vrth);
        let eq3_e65_d_n2: f64 = (1000000.0 * var_vrth_dn2);
        (eq3_e65, eq3_e65_d_n2,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e67;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (eq3_value),
            2,
            multiplicity * (eq3_e67_d_n2),
        );
        let (eq4_e72, eq4_e72_d_n2,) = {
    if (p.p7 != 0.0) {
        let eq4_e70: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, var_qcth);
        (eq4_e70, (var_qcth_dn2 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e72;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (eq4_value),
            2,
            multiplicity * (eq4_e72_d_n2),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let mut var_leff_um: f64 = 0.0;
        let mut var_leff_um_rv: f64 = 0.0;
        let mut var_weff_um: f64 = 0.0;
        let mut var_weff_um_rv: f64 = 0.0;
        let mut var_cth: f64 = 0.0;
        let mut var_cth_rv: f64 = 0.0;
        let mut var_scalefac: f64 = 0.0;
        let mut var_scalefac_rv: f64 = 0.0;
        let mut var_shrinkl: f64 = 0.0;
        let mut var_shrinkl_rv: f64 = 0.0;
        let mut var_xleff: f64 = 0.0;
        let mut var_xleff_rv: f64 = 0.0;
        let mut var_lfactor: f64 = 0.0;
        let mut var_lfactor_rv: f64 = 0.0;
        let mut var_l_um: f64 = 0.0;
        let mut var_l_um_rv: f64 = 0.0;
        let mut var_w_um: f64 = 0.0;
        let mut var_w_um_rv: f64 = 0.0;
        let mut var_vrth: f64 = 0.0;
        let mut var_vrth_rv: f64 = 0.0;
        let mut var_vrth_dn2: f64 = 0.0;
        let mut var_qcth: f64 = 0.0;
        let mut var_qcth_rv: f64 = 0.0;
        let mut var_qcth_dn2: f64 = 0.0;
        let mut var_p_um: f64 = 0.0;
        let mut var_p_um_rv: f64 = 0.0;
        let mut var_a_um2: f64 = 0.0;
        let mut var_a_um2_rv: f64 = 0.0;
        let mut var_guard41: f64 = 0.0;
        let mut var_guard41_rv: f64 = 0.0;
        let mut var_guard42: f64 = 0.0;
        let mut var_guard42_rv: f64 = 0.0;
        let mut var_guard46: f64 = 0.0;
        let mut var_guard46_rv: f64 = 0.0;
        let mut var_guard47: f64 = 0.0;
        let mut var_guard47_rv: f64 = 0.0;
        let mut var_guard48: f64 = 0.0;
        let mut var_guard48_rv: f64 = 0.0;
        let mut var_guard49: f64 = 0.0;
        let mut var_guard49_rv: f64 = 0.0;
        let mut var_guard51: f64 = 0.0;
        let mut var_guard51_rv: f64 = 0.0;
        let mut var_guard53: f64 = 0.0;
        let mut var_guard53_rv: f64 = 0.0;
        let mut var_guard54: f64 = 0.0;
        let mut var_guard54_rv: f64 = 0.0;
        let mut var_guard55: f64 = 0.0;
        let mut var_guard55_rv: f64 = 0.0;
        let mut var_guard57: f64 = 0.0;
        let mut var_guard57_rv: f64 = 0.0;
        let mut var_guard59: f64 = 0.0;
        let mut var_guard59_rv: f64 = 0.0;
        let mut var_guard60: f64 = 0.0;
        let mut var_guard60_rv: f64 = 0.0;
        let mut var_guard75: f64 = 0.0;
        let mut var_guard75_rv: f64 = 0.0;
        let mut var_guard76: f64 = 0.0;
        let mut var_guard76_rv: f64 = 0.0;

        Self::stamp_reactive_block_0(p, param_given, &mut var_guard41, &mut var_guard41_rv, &mut var_guard42, &mut var_guard42_rv, &mut var_guard46, &mut var_guard46_rv, &mut var_guard47, &mut var_guard47_rv, &mut var_guard48, &mut var_guard48_rv, &mut var_guard49, &mut var_guard49_rv, &mut var_guard51, &mut var_guard51_rv, &mut var_guard53, &mut var_guard53_rv, &mut var_guard54, &mut var_guard54_rv, &mut var_guard55, &mut var_guard55_rv, &mut var_guard57, &mut var_guard57_rv, &mut var_guard59, &mut var_guard59_rv, &mut var_guard60, &mut var_guard60_rv, &mut var_l_um, &mut var_l_um_rv, &mut var_leff_um, &mut var_leff_um_rv, &mut var_lfactor, &mut var_lfactor_rv, &mut var_scalefac, &mut var_scalefac_rv, &mut var_shrinkl, &mut var_shrinkl_rv, &mut var_w_um, &mut var_w_um_rv, &mut var_weff_um, &mut var_weff_um_rv, &mut var_xleff, &mut var_xleff_rv);
        Self::stamp_reactive_block_1(ctx, p, nodes, var_guard48, var_guard53, var_guard59, var_guard60, var_l_um, var_w_um, var_xleff, &mut var_a_um2, &mut var_a_um2_rv, &mut var_cth, &mut var_cth_rv, &mut var_guard75, &mut var_guard75_rv, &mut var_guard76, &mut var_guard76_rv, &mut var_leff_um, &mut var_leff_um_rv, &mut var_p_um, &mut var_p_um_rv, &mut var_qcth, &mut var_qcth_dn2, &mut var_qcth_rv, &mut var_vrth, &mut var_vrth_dn2, &mut var_vrth_rv);

        let (eq4_e72, eq4_e72_d_n2, eq4_e72_q,) = {
    if (p.p7 != 0.0) {
        let eq4_e70_q: f64 = var_qcth;
        (var_qcth, var_qcth_dn2, eq4_e70_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * (eq4_e72_d_n2),
        );
    }
}
