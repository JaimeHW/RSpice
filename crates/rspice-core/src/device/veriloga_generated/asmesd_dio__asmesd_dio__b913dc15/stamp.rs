#![allow(dead_code, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

use crate::device::veriloga_generated::support::{AdValue as GenericAdValue, ReactiveScratch as GenericReactiveScratch, Scratch as GenericScratch};

type A = GenericAdValue<{ Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;
type Scratch = GenericScratch<{ Instance::VARIABLE_COUNT }, { Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;
type ReactiveScratch = GenericReactiveScratch<{ Instance::VARIABLE_COUNT }, { Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;

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
        let mut var_arg_dn0: f64 = 0.0;
        let mut var_arg_dn1: f64 = 0.0;
        let mut var_arg_dn2: f64 = 0.0;
        let mut var_arg_dn3: f64 = 0.0;
        let mut var_arg_dn4: f64 = 0.0;
        let mut var_arg_dn5: f64 = 0.0;
        let mut var_arg_dn6: f64 = 0.0;
        let mut var_arg_db0: f64 = 0.0;
        let mut var_arg_db1: f64 = 0.0;
        let mut var_arg_db2: f64 = 0.0;
        let mut var_arg_db3: f64 = 0.0;
        let mut var_arg_db4: f64 = 0.0;
        let mut var_arg_db5: f64 = 0.0;
        let mut var_arg_db6: f64 = 0.0;
        let mut var_le: f64 = 0.0;
        let mut var_le_dn0: f64 = 0.0;
        let mut var_le_dn1: f64 = 0.0;
        let mut var_le_dn2: f64 = 0.0;
        let mut var_le_dn3: f64 = 0.0;
        let mut var_le_dn4: f64 = 0.0;
        let mut var_le_dn5: f64 = 0.0;
        let mut var_le_dn6: f64 = 0.0;
        let mut var_le_db0: f64 = 0.0;
        let mut var_le_db1: f64 = 0.0;
        let mut var_le_db2: f64 = 0.0;
        let mut var_le_db3: f64 = 0.0;
        let mut var_le_db4: f64 = 0.0;
        let mut var_le_db5: f64 = 0.0;
        let mut var_le_db6: f64 = 0.0;
        let mut var_lebv: f64 = 0.0;
        let mut var_lebv_dn0: f64 = 0.0;
        let mut var_lebv_dn1: f64 = 0.0;
        let mut var_lebv_dn2: f64 = 0.0;
        let mut var_lebv_dn3: f64 = 0.0;
        let mut var_lebv_dn4: f64 = 0.0;
        let mut var_lebv_dn5: f64 = 0.0;
        let mut var_lebv_dn6: f64 = 0.0;
        let mut var_lebv_db0: f64 = 0.0;
        let mut var_lebv_db1: f64 = 0.0;
        let mut var_lebv_db2: f64 = 0.0;
        let mut var_lebv_db3: f64 = 0.0;
        let mut var_lebv_db4: f64 = 0.0;
        let mut var_lebv_db5: f64 = 0.0;
        let mut var_lebv_db6: f64 = 0.0;
        let mut var_weff: f64 = 0.0;
        let mut var_dv0: f64 = 0.0;
        let mut var_dv0_dn0: f64 = 0.0;
        let mut var_dv0_dn1: f64 = 0.0;
        let mut var_dv0_dn2: f64 = 0.0;
        let mut var_dv0_dn3: f64 = 0.0;
        let mut var_dv0_dn4: f64 = 0.0;
        let mut var_dv0_dn5: f64 = 0.0;
        let mut var_dv0_dn6: f64 = 0.0;
        let mut var_dv0_db0: f64 = 0.0;
        let mut var_dv0_db1: f64 = 0.0;
        let mut var_dv0_db2: f64 = 0.0;
        let mut var_dv0_db3: f64 = 0.0;
        let mut var_dv0_db4: f64 = 0.0;
        let mut var_dv0_db5: f64 = 0.0;
        let mut var_dv0_db6: f64 = 0.0;
        let mut var_dvh: f64 = 0.0;
        let mut var_dvh_dn0: f64 = 0.0;
        let mut var_dvh_dn1: f64 = 0.0;
        let mut var_dvh_dn2: f64 = 0.0;
        let mut var_dvh_dn3: f64 = 0.0;
        let mut var_dvh_dn4: f64 = 0.0;
        let mut var_dvh_dn5: f64 = 0.0;
        let mut var_dvh_dn6: f64 = 0.0;
        let mut var_dvh_db0: f64 = 0.0;
        let mut var_dvh_db1: f64 = 0.0;
        let mut var_dvh_db2: f64 = 0.0;
        let mut var_dvh_db3: f64 = 0.0;
        let mut var_dvh_db4: f64 = 0.0;
        let mut var_dvh_db5: f64 = 0.0;
        let mut var_dvh_db6: f64 = 0.0;
        let mut var_pwq: f64 = 0.0;
        let mut var_qlo: f64 = 0.0;
        let mut var_qlo_dn0: f64 = 0.0;
        let mut var_qlo_dn1: f64 = 0.0;
        let mut var_qlo_dn2: f64 = 0.0;
        let mut var_qlo_dn3: f64 = 0.0;
        let mut var_qlo_dn4: f64 = 0.0;
        let mut var_qlo_dn5: f64 = 0.0;
        let mut var_qlo_dn6: f64 = 0.0;
        let mut var_qlo_db0: f64 = 0.0;
        let mut var_qlo_db1: f64 = 0.0;
        let mut var_qlo_db2: f64 = 0.0;
        let mut var_qlo_db3: f64 = 0.0;
        let mut var_qlo_db4: f64 = 0.0;
        let mut var_qlo_db5: f64 = 0.0;
        let mut var_qlo_db6: f64 = 0.0;
        let mut var_qhi: f64 = 0.0;
        let mut var_qhi_dn0: f64 = 0.0;
        let mut var_qhi_dn1: f64 = 0.0;
        let mut var_qhi_dn2: f64 = 0.0;
        let mut var_qhi_dn3: f64 = 0.0;
        let mut var_qhi_dn4: f64 = 0.0;
        let mut var_qhi_dn5: f64 = 0.0;
        let mut var_qhi_dn6: f64 = 0.0;
        let mut var_qhi_db0: f64 = 0.0;
        let mut var_qhi_db1: f64 = 0.0;
        let mut var_qhi_db2: f64 = 0.0;
        let mut var_qhi_db3: f64 = 0.0;
        let mut var_qhi_db4: f64 = 0.0;
        let mut var_qhi_db5: f64 = 0.0;
        let mut var_qhi_db6: f64 = 0.0;
        let mut var_ttype: f64 = 0.0;
        let mut var_tdev: f64 = 0.0;
        let mut var_tdev_dn0: f64 = 0.0;
        let mut var_tdev_dn1: f64 = 0.0;
        let mut var_tdev_dn2: f64 = 0.0;
        let mut var_tdev_dn3: f64 = 0.0;
        let mut var_tdev_dn4: f64 = 0.0;
        let mut var_tdev_dn5: f64 = 0.0;
        let mut var_tdev_dn6: f64 = 0.0;
        let mut var_tdev_db0: f64 = 0.0;
        let mut var_tdev_db1: f64 = 0.0;
        let mut var_tdev_db2: f64 = 0.0;
        let mut var_tdev_db3: f64 = 0.0;
        let mut var_tdev_db4: f64 = 0.0;
        let mut var_tdev_db5: f64 = 0.0;
        let mut var_tdev_db6: f64 = 0.0;
        let mut var_tnom: f64 = 0.0;
        let mut var_tamb: f64 = 0.0;
        let mut var_tamb_dn0: f64 = 0.0;
        let mut var_tamb_dn1: f64 = 0.0;
        let mut var_tamb_dn2: f64 = 0.0;
        let mut var_tamb_dn3: f64 = 0.0;
        let mut var_tamb_dn4: f64 = 0.0;
        let mut var_tamb_dn5: f64 = 0.0;
        let mut var_tamb_dn6: f64 = 0.0;
        let mut var_tamb_db0: f64 = 0.0;
        let mut var_tamb_db1: f64 = 0.0;
        let mut var_tamb_db2: f64 = 0.0;
        let mut var_tamb_db3: f64 = 0.0;
        let mut var_tamb_db4: f64 = 0.0;
        let mut var_tamb_db5: f64 = 0.0;
        let mut var_tamb_db6: f64 = 0.0;
        let mut var_rt: f64 = 0.0;
        let mut var_rt_dn0: f64 = 0.0;
        let mut var_rt_dn1: f64 = 0.0;
        let mut var_rt_dn2: f64 = 0.0;
        let mut var_rt_dn3: f64 = 0.0;
        let mut var_rt_dn4: f64 = 0.0;
        let mut var_rt_dn5: f64 = 0.0;
        let mut var_rt_dn6: f64 = 0.0;
        let mut var_rt_db0: f64 = 0.0;
        let mut var_rt_db1: f64 = 0.0;
        let mut var_rt_db2: f64 = 0.0;
        let mut var_rt_db3: f64 = 0.0;
        let mut var_rt_db4: f64 = 0.0;
        let mut var_rt_db5: f64 = 0.0;
        let mut var_rt_db6: f64 = 0.0;
        let mut var_lnrt: f64 = 0.0;
        let mut var_lnrt_dn0: f64 = 0.0;
        let mut var_lnrt_dn1: f64 = 0.0;
        let mut var_lnrt_dn2: f64 = 0.0;
        let mut var_lnrt_dn3: f64 = 0.0;
        let mut var_lnrt_dn4: f64 = 0.0;
        let mut var_lnrt_dn5: f64 = 0.0;
        let mut var_lnrt_dn6: f64 = 0.0;
        let mut var_lnrt_db0: f64 = 0.0;
        let mut var_lnrt_db1: f64 = 0.0;
        let mut var_lnrt_db2: f64 = 0.0;
        let mut var_lnrt_db3: f64 = 0.0;
        let mut var_lnrt_db4: f64 = 0.0;
        let mut var_lnrt_db5: f64 = 0.0;
        let mut var_lnrt_db6: f64 = 0.0;
        let mut var_vt: f64 = 0.0;
        let mut var_vt_dn0: f64 = 0.0;
        let mut var_vt_dn1: f64 = 0.0;
        let mut var_vt_dn2: f64 = 0.0;
        let mut var_vt_dn3: f64 = 0.0;
        let mut var_vt_dn4: f64 = 0.0;
        let mut var_vt_dn5: f64 = 0.0;
        let mut var_vt_dn6: f64 = 0.0;
        let mut var_vt_db0: f64 = 0.0;
        let mut var_vt_db1: f64 = 0.0;
        let mut var_vt_db2: f64 = 0.0;
        let mut var_vt_db3: f64 = 0.0;
        let mut var_vt_db4: f64 = 0.0;
        let mut var_vt_db5: f64 = 0.0;
        let mut var_vt_db6: f64 = 0.0;
        let mut var_is_t: f64 = 0.0;
        let mut var_is_t_dn0: f64 = 0.0;
        let mut var_is_t_dn1: f64 = 0.0;
        let mut var_is_t_dn2: f64 = 0.0;
        let mut var_is_t_dn3: f64 = 0.0;
        let mut var_is_t_dn4: f64 = 0.0;
        let mut var_is_t_dn5: f64 = 0.0;
        let mut var_is_t_dn6: f64 = 0.0;
        let mut var_is_t_db0: f64 = 0.0;
        let mut var_is_t_db1: f64 = 0.0;
        let mut var_is_t_db2: f64 = 0.0;
        let mut var_is_t_db3: f64 = 0.0;
        let mut var_is_t_db4: f64 = 0.0;
        let mut var_is_t_db5: f64 = 0.0;
        let mut var_is_t_db6: f64 = 0.0;
        let mut var_cje_t: f64 = 0.0;
        let mut var_cje_t_dn0: f64 = 0.0;
        let mut var_cje_t_dn1: f64 = 0.0;
        let mut var_cje_t_dn2: f64 = 0.0;
        let mut var_cje_t_dn3: f64 = 0.0;
        let mut var_cje_t_dn4: f64 = 0.0;
        let mut var_cje_t_dn5: f64 = 0.0;
        let mut var_cje_t_dn6: f64 = 0.0;
        let mut var_cje_t_db0: f64 = 0.0;
        let mut var_cje_t_db1: f64 = 0.0;
        let mut var_cje_t_db2: f64 = 0.0;
        let mut var_cje_t_db3: f64 = 0.0;
        let mut var_cje_t_db4: f64 = 0.0;
        let mut var_cje_t_db5: f64 = 0.0;
        let mut var_cje_t_db6: f64 = 0.0;
        let mut var_vje_t: f64 = 0.0;
        let mut var_vje_t_dn0: f64 = 0.0;
        let mut var_vje_t_dn1: f64 = 0.0;
        let mut var_vje_t_dn2: f64 = 0.0;
        let mut var_vje_t_dn3: f64 = 0.0;
        let mut var_vje_t_dn4: f64 = 0.0;
        let mut var_vje_t_dn5: f64 = 0.0;
        let mut var_vje_t_dn6: f64 = 0.0;
        let mut var_vje_t_db0: f64 = 0.0;
        let mut var_vje_t_db1: f64 = 0.0;
        let mut var_vje_t_db2: f64 = 0.0;
        let mut var_vje_t_db3: f64 = 0.0;
        let mut var_vje_t_db4: f64 = 0.0;
        let mut var_vje_t_db5: f64 = 0.0;
        let mut var_vje_t_db6: f64 = 0.0;
        let mut var_ijbv_t: f64 = 0.0;
        let mut var_ijbv_t_dn0: f64 = 0.0;
        let mut var_ijbv_t_dn1: f64 = 0.0;
        let mut var_ijbv_t_dn2: f64 = 0.0;
        let mut var_ijbv_t_dn3: f64 = 0.0;
        let mut var_ijbv_t_dn4: f64 = 0.0;
        let mut var_ijbv_t_dn5: f64 = 0.0;
        let mut var_ijbv_t_dn6: f64 = 0.0;
        let mut var_ijbv_t_db0: f64 = 0.0;
        let mut var_ijbv_t_db1: f64 = 0.0;
        let mut var_ijbv_t_db2: f64 = 0.0;
        let mut var_ijbv_t_db3: f64 = 0.0;
        let mut var_ijbv_t_db4: f64 = 0.0;
        let mut var_ijbv_t_db5: f64 = 0.0;
        let mut var_ijbv_t_db6: f64 = 0.0;
        let mut var_bvr_t: f64 = 0.0;
        let mut var_bvr_t_dn0: f64 = 0.0;
        let mut var_bvr_t_dn1: f64 = 0.0;
        let mut var_bvr_t_dn2: f64 = 0.0;
        let mut var_bvr_t_dn3: f64 = 0.0;
        let mut var_bvr_t_dn4: f64 = 0.0;
        let mut var_bvr_t_dn5: f64 = 0.0;
        let mut var_bvr_t_dn6: f64 = 0.0;
        let mut var_bvr_t_db0: f64 = 0.0;
        let mut var_bvr_t_db1: f64 = 0.0;
        let mut var_bvr_t_db2: f64 = 0.0;
        let mut var_bvr_t_db3: f64 = 0.0;
        let mut var_bvr_t_db4: f64 = 0.0;
        let mut var_bvr_t_db5: f64 = 0.0;
        let mut var_bvr_t_db6: f64 = 0.0;
        let mut var_theexp_t: f64 = 0.0;
        let mut var_theexp_t_dn0: f64 = 0.0;
        let mut var_theexp_t_dn1: f64 = 0.0;
        let mut var_theexp_t_dn2: f64 = 0.0;
        let mut var_theexp_t_dn3: f64 = 0.0;
        let mut var_theexp_t_dn4: f64 = 0.0;
        let mut var_theexp_t_dn5: f64 = 0.0;
        let mut var_theexp_t_dn6: f64 = 0.0;
        let mut var_theexp_t_db0: f64 = 0.0;
        let mut var_theexp_t_db1: f64 = 0.0;
        let mut var_theexp_t_db2: f64 = 0.0;
        let mut var_theexp_t_db3: f64 = 0.0;
        let mut var_theexp_t_db4: f64 = 0.0;
        let mut var_theexp_t_db5: f64 = 0.0;
        let mut var_theexp_t_db6: f64 = 0.0;
        let mut var_cje_i: f64 = 0.0;
        let mut var_ifwd: f64 = 0.0;
        let mut var_ifwd_dn0: f64 = 0.0;
        let mut var_ifwd_dn1: f64 = 0.0;
        let mut var_ifwd_dn2: f64 = 0.0;
        let mut var_ifwd_dn3: f64 = 0.0;
        let mut var_ifwd_dn4: f64 = 0.0;
        let mut var_ifwd_dn5: f64 = 0.0;
        let mut var_ifwd_dn6: f64 = 0.0;
        let mut var_ifwd_db0: f64 = 0.0;
        let mut var_ifwd_db1: f64 = 0.0;
        let mut var_ifwd_db2: f64 = 0.0;
        let mut var_ifwd_db3: f64 = 0.0;
        let mut var_ifwd_db4: f64 = 0.0;
        let mut var_ifwd_db5: f64 = 0.0;
        let mut var_ifwd_db6: f64 = 0.0;
        let mut var_ibe: f64 = 0.0;
        let mut var_ibe_dn0: f64 = 0.0;
        let mut var_ibe_dn1: f64 = 0.0;
        let mut var_ibe_dn2: f64 = 0.0;
        let mut var_ibe_dn3: f64 = 0.0;
        let mut var_ibe_dn4: f64 = 0.0;
        let mut var_ibe_dn5: f64 = 0.0;
        let mut var_ibe_dn6: f64 = 0.0;
        let mut var_ibe_db0: f64 = 0.0;
        let mut var_ibe_db1: f64 = 0.0;
        let mut var_ibe_db2: f64 = 0.0;
        let mut var_ibe_db3: f64 = 0.0;
        let mut var_ibe_db4: f64 = 0.0;
        let mut var_ibe_db5: f64 = 0.0;
        let mut var_ibe_db6: f64 = 0.0;
        let mut var_itzf: f64 = 0.0;
        let mut var_itzf_dn0: f64 = 0.0;
        let mut var_itzf_dn1: f64 = 0.0;
        let mut var_itzf_dn2: f64 = 0.0;
        let mut var_itzf_dn3: f64 = 0.0;
        let mut var_itzf_dn4: f64 = 0.0;
        let mut var_itzf_dn5: f64 = 0.0;
        let mut var_itzf_dn6: f64 = 0.0;
        let mut var_itzf_db0: f64 = 0.0;
        let mut var_itzf_db1: f64 = 0.0;
        let mut var_itzf_db2: f64 = 0.0;
        let mut var_itzf_db3: f64 = 0.0;
        let mut var_itzf_db4: f64 = 0.0;
        let mut var_itzf_db5: f64 = 0.0;
        let mut var_itzf_db6: f64 = 0.0;
        let mut var_itrev: f64 = 0.0;
        let mut var_itrev_dn0: f64 = 0.0;
        let mut var_itrev_dn1: f64 = 0.0;
        let mut var_itrev_dn2: f64 = 0.0;
        let mut var_itrev_dn3: f64 = 0.0;
        let mut var_itrev_dn4: f64 = 0.0;
        let mut var_itrev_dn5: f64 = 0.0;
        let mut var_itrev_dn6: f64 = 0.0;
        let mut var_itrev_db0: f64 = 0.0;
        let mut var_itrev_db1: f64 = 0.0;
        let mut var_itrev_db2: f64 = 0.0;
        let mut var_itrev_db3: f64 = 0.0;
        let mut var_itrev_db4: f64 = 0.0;
        let mut var_itrev_db5: f64 = 0.0;
        let mut var_itrev_db6: f64 = 0.0;
        let mut var_re_nom: f64 = 0.0;
        let mut var_rb_nom: f64 = 0.0;
        let mut var_rb: f64 = 0.0;
        let mut var_rb_dn0: f64 = 0.0;
        let mut var_rb_dn1: f64 = 0.0;
        let mut var_rb_dn2: f64 = 0.0;
        let mut var_rb_dn3: f64 = 0.0;
        let mut var_rb_dn4: f64 = 0.0;
        let mut var_rb_dn5: f64 = 0.0;
        let mut var_rb_dn6: f64 = 0.0;
        let mut var_rb_db0: f64 = 0.0;
        let mut var_rb_db1: f64 = 0.0;
        let mut var_rb_db2: f64 = 0.0;
        let mut var_rb_db3: f64 = 0.0;
        let mut var_rb_db4: f64 = 0.0;
        let mut var_rb_db5: f64 = 0.0;
        let mut var_rb_db6: f64 = 0.0;
        let mut var_re: f64 = 0.0;
        let mut var_re_dn0: f64 = 0.0;
        let mut var_re_dn1: f64 = 0.0;
        let mut var_re_dn2: f64 = 0.0;
        let mut var_re_dn3: f64 = 0.0;
        let mut var_re_dn4: f64 = 0.0;
        let mut var_re_dn5: f64 = 0.0;
        let mut var_re_dn6: f64 = 0.0;
        let mut var_re_db0: f64 = 0.0;
        let mut var_re_db1: f64 = 0.0;
        let mut var_re_db2: f64 = 0.0;
        let mut var_re_db3: f64 = 0.0;
        let mut var_re_db4: f64 = 0.0;
        let mut var_re_db5: f64 = 0.0;
        let mut var_re_db6: f64 = 0.0;
        let mut var_tff: f64 = 0.0;
        let mut var_tff_dn0: f64 = 0.0;
        let mut var_tff_dn1: f64 = 0.0;
        let mut var_tff_dn2: f64 = 0.0;
        let mut var_tff_dn3: f64 = 0.0;
        let mut var_tff_dn4: f64 = 0.0;
        let mut var_tff_dn5: f64 = 0.0;
        let mut var_tff_dn6: f64 = 0.0;
        let mut var_tff_db0: f64 = 0.0;
        let mut var_tff_db1: f64 = 0.0;
        let mut var_tff_db2: f64 = 0.0;
        let mut var_tff_db3: f64 = 0.0;
        let mut var_tff_db4: f64 = 0.0;
        let mut var_tff_db5: f64 = 0.0;
        let mut var_tff_db6: f64 = 0.0;
        let mut var_qde: f64 = 0.0;
        let mut var_qde_dn0: f64 = 0.0;
        let mut var_qde_dn1: f64 = 0.0;
        let mut var_qde_dn2: f64 = 0.0;
        let mut var_qde_dn3: f64 = 0.0;
        let mut var_qde_dn4: f64 = 0.0;
        let mut var_qde_dn5: f64 = 0.0;
        let mut var_qde_dn6: f64 = 0.0;
        let mut var_qde_db0: f64 = 0.0;
        let mut var_qde_db1: f64 = 0.0;
        let mut var_qde_db2: f64 = 0.0;
        let mut var_qde_db3: f64 = 0.0;
        let mut var_qde_db4: f64 = 0.0;
        let mut var_qde_db5: f64 = 0.0;
        let mut var_qde_db6: f64 = 0.0;
        let mut var_qje: f64 = 0.0;
        let mut var_qje_dn0: f64 = 0.0;
        let mut var_qje_dn1: f64 = 0.0;
        let mut var_qje_dn2: f64 = 0.0;
        let mut var_qje_dn3: f64 = 0.0;
        let mut var_qje_dn4: f64 = 0.0;
        let mut var_qje_dn5: f64 = 0.0;
        let mut var_qje_dn6: f64 = 0.0;
        let mut var_qje_db0: f64 = 0.0;
        let mut var_qje_db1: f64 = 0.0;
        let mut var_qje_db2: f64 = 0.0;
        let mut var_qje_db3: f64 = 0.0;
        let mut var_qje_db4: f64 = 0.0;
        let mut var_qje_db5: f64 = 0.0;
        let mut var_qje_db6: f64 = 0.0;
        let mut var_argt: f64 = 0.0;
        let mut var_argt_dn0: f64 = 0.0;
        let mut var_argt_dn1: f64 = 0.0;
        let mut var_argt_dn2: f64 = 0.0;
        let mut var_argt_dn3: f64 = 0.0;
        let mut var_argt_dn4: f64 = 0.0;
        let mut var_argt_dn5: f64 = 0.0;
        let mut var_argt_dn6: f64 = 0.0;
        let mut var_argt_db0: f64 = 0.0;
        let mut var_argt_db1: f64 = 0.0;
        let mut var_argt_db2: f64 = 0.0;
        let mut var_argt_db3: f64 = 0.0;
        let mut var_argt_db4: f64 = 0.0;
        let mut var_argt_db5: f64 = 0.0;
        let mut var_argt_db6: f64 = 0.0;
        let mut var_vbiei: f64 = 0.0;
        let mut var_vbiei_dn0: f64 = 0.0;
        let mut var_vbiei_dn1: f64 = 0.0;
        let mut var_vbiei_dn2: f64 = 0.0;
        let mut var_vbiei_dn3: f64 = 0.0;
        let mut var_vbiei_dn4: f64 = 0.0;
        let mut var_vbiei_dn5: f64 = 0.0;
        let mut var_vbiei_dn6: f64 = 0.0;
        let mut var_vbiei_db0: f64 = 0.0;
        let mut var_vbiei_db1: f64 = 0.0;
        let mut var_vbiei_db2: f64 = 0.0;
        let mut var_vbiei_db3: f64 = 0.0;
        let mut var_vbiei_db4: f64 = 0.0;
        let mut var_vbiei_db5: f64 = 0.0;
        let mut var_vbiei_db6: f64 = 0.0;
        let mut var_vbbi: f64 = 0.0;
        let mut var_vbbi_dn0: f64 = 0.0;
        let mut var_vbbi_dn1: f64 = 0.0;
        let mut var_vbbi_dn2: f64 = 0.0;
        let mut var_vbbi_dn3: f64 = 0.0;
        let mut var_vbbi_dn4: f64 = 0.0;
        let mut var_vbbi_dn5: f64 = 0.0;
        let mut var_vbbi_dn6: f64 = 0.0;
        let mut var_vbbi_db0: f64 = 0.0;
        let mut var_vbbi_db1: f64 = 0.0;
        let mut var_vbbi_db2: f64 = 0.0;
        let mut var_vbbi_db3: f64 = 0.0;
        let mut var_vbbi_db4: f64 = 0.0;
        let mut var_vbbi_db5: f64 = 0.0;
        let mut var_vbbi_db6: f64 = 0.0;
        let mut var_veei: f64 = 0.0;
        let mut var_veei_dn0: f64 = 0.0;
        let mut var_veei_dn1: f64 = 0.0;
        let mut var_veei_dn2: f64 = 0.0;
        let mut var_veei_dn3: f64 = 0.0;
        let mut var_veei_dn4: f64 = 0.0;
        let mut var_veei_dn5: f64 = 0.0;
        let mut var_veei_dn6: f64 = 0.0;
        let mut var_veei_db0: f64 = 0.0;
        let mut var_veei_db1: f64 = 0.0;
        let mut var_veei_db2: f64 = 0.0;
        let mut var_veei_db3: f64 = 0.0;
        let mut var_veei_db4: f64 = 0.0;
        let mut var_veei_db5: f64 = 0.0;
        let mut var_veei_db6: f64 = 0.0;
        let mut var_fact1: f64 = 0.0;
        let mut var_fact2: f64 = 0.0;
        let mut var_fact2_dn0: f64 = 0.0;
        let mut var_fact2_dn1: f64 = 0.0;
        let mut var_fact2_dn2: f64 = 0.0;
        let mut var_fact2_dn3: f64 = 0.0;
        let mut var_fact2_dn4: f64 = 0.0;
        let mut var_fact2_dn5: f64 = 0.0;
        let mut var_fact2_dn6: f64 = 0.0;
        let mut var_fact2_db0: f64 = 0.0;
        let mut var_fact2_db1: f64 = 0.0;
        let mut var_fact2_db2: f64 = 0.0;
        let mut var_fact2_db3: f64 = 0.0;
        let mut var_fact2_db4: f64 = 0.0;
        let mut var_fact2_db5: f64 = 0.0;
        let mut var_fact2_db6: f64 = 0.0;
        let mut var_egfet: f64 = 0.0;
        let mut var_egfet_dn0: f64 = 0.0;
        let mut var_egfet_dn1: f64 = 0.0;
        let mut var_egfet_dn2: f64 = 0.0;
        let mut var_egfet_dn3: f64 = 0.0;
        let mut var_egfet_dn4: f64 = 0.0;
        let mut var_egfet_dn5: f64 = 0.0;
        let mut var_egfet_dn6: f64 = 0.0;
        let mut var_egfet_db0: f64 = 0.0;
        let mut var_egfet_db1: f64 = 0.0;
        let mut var_egfet_db2: f64 = 0.0;
        let mut var_egfet_db3: f64 = 0.0;
        let mut var_egfet_db4: f64 = 0.0;
        let mut var_egfet_db5: f64 = 0.0;
        let mut var_egfet_db6: f64 = 0.0;
        let mut var_arg0: f64 = 0.0;
        let mut var_arg0_dn0: f64 = 0.0;
        let mut var_arg0_dn1: f64 = 0.0;
        let mut var_arg0_dn2: f64 = 0.0;
        let mut var_arg0_dn3: f64 = 0.0;
        let mut var_arg0_dn4: f64 = 0.0;
        let mut var_arg0_dn5: f64 = 0.0;
        let mut var_arg0_dn6: f64 = 0.0;
        let mut var_arg0_db0: f64 = 0.0;
        let mut var_arg0_db1: f64 = 0.0;
        let mut var_arg0_db2: f64 = 0.0;
        let mut var_arg0_db3: f64 = 0.0;
        let mut var_arg0_db4: f64 = 0.0;
        let mut var_arg0_db5: f64 = 0.0;
        let mut var_arg0_db6: f64 = 0.0;
        let mut var_pbfact: f64 = 0.0;
        let mut var_pbfact_dn0: f64 = 0.0;
        let mut var_pbfact_dn1: f64 = 0.0;
        let mut var_pbfact_dn2: f64 = 0.0;
        let mut var_pbfact_dn3: f64 = 0.0;
        let mut var_pbfact_dn4: f64 = 0.0;
        let mut var_pbfact_dn5: f64 = 0.0;
        let mut var_pbfact_dn6: f64 = 0.0;
        let mut var_pbfact_db0: f64 = 0.0;
        let mut var_pbfact_db1: f64 = 0.0;
        let mut var_pbfact_db2: f64 = 0.0;
        let mut var_pbfact_db3: f64 = 0.0;
        let mut var_pbfact_db4: f64 = 0.0;
        let mut var_pbfact_db5: f64 = 0.0;
        let mut var_pbfact_db6: f64 = 0.0;
        let mut var_pbo: f64 = 0.0;
        let mut var_pbo_dn0: f64 = 0.0;
        let mut var_pbo_dn1: f64 = 0.0;
        let mut var_pbo_dn2: f64 = 0.0;
        let mut var_pbo_dn3: f64 = 0.0;
        let mut var_pbo_dn4: f64 = 0.0;
        let mut var_pbo_dn5: f64 = 0.0;
        let mut var_pbo_dn6: f64 = 0.0;
        let mut var_pbo_db0: f64 = 0.0;
        let mut var_pbo_db1: f64 = 0.0;
        let mut var_pbo_db2: f64 = 0.0;
        let mut var_pbo_db3: f64 = 0.0;
        let mut var_pbo_db4: f64 = 0.0;
        let mut var_pbo_db5: f64 = 0.0;
        let mut var_pbo_db6: f64 = 0.0;
        let mut var_gmaold: f64 = 0.0;
        let mut var_gmaold_dn0: f64 = 0.0;
        let mut var_gmaold_dn1: f64 = 0.0;
        let mut var_gmaold_dn2: f64 = 0.0;
        let mut var_gmaold_dn3: f64 = 0.0;
        let mut var_gmaold_dn4: f64 = 0.0;
        let mut var_gmaold_dn5: f64 = 0.0;
        let mut var_gmaold_dn6: f64 = 0.0;
        let mut var_gmaold_db0: f64 = 0.0;
        let mut var_gmaold_db1: f64 = 0.0;
        let mut var_gmaold_db2: f64 = 0.0;
        let mut var_gmaold_db3: f64 = 0.0;
        let mut var_gmaold_db4: f64 = 0.0;
        let mut var_gmaold_db5: f64 = 0.0;
        let mut var_gmaold_db6: f64 = 0.0;
        let mut var_gmanew: f64 = 0.0;
        let mut var_gmanew_dn0: f64 = 0.0;
        let mut var_gmanew_dn1: f64 = 0.0;
        let mut var_gmanew_dn2: f64 = 0.0;
        let mut var_gmanew_dn3: f64 = 0.0;
        let mut var_gmanew_dn4: f64 = 0.0;
        let mut var_gmanew_dn5: f64 = 0.0;
        let mut var_gmanew_dn6: f64 = 0.0;
        let mut var_gmanew_db0: f64 = 0.0;
        let mut var_gmanew_db1: f64 = 0.0;
        let mut var_gmanew_db2: f64 = 0.0;
        let mut var_gmanew_db3: f64 = 0.0;
        let mut var_gmanew_db4: f64 = 0.0;
        let mut var_gmanew_db5: f64 = 0.0;
        let mut var_gmanew_db6: f64 = 0.0;
        let mut var_cjt: f64 = 0.0;
        let mut var_cjt_dn0: f64 = 0.0;
        let mut var_cjt_dn1: f64 = 0.0;
        let mut var_cjt_dn2: f64 = 0.0;
        let mut var_cjt_dn3: f64 = 0.0;
        let mut var_cjt_dn4: f64 = 0.0;
        let mut var_cjt_dn5: f64 = 0.0;
        let mut var_cjt_dn6: f64 = 0.0;
        let mut var_cjt_db0: f64 = 0.0;
        let mut var_cjt_db1: f64 = 0.0;
        let mut var_cjt_db2: f64 = 0.0;
        let mut var_cjt_db3: f64 = 0.0;
        let mut var_cjt_db4: f64 = 0.0;
        let mut var_cjt_db5: f64 = 0.0;
        let mut var_cjt_db6: f64 = 0.0;
        let mut var_argbv: f64 = 0.0;
        let mut var_argbv_dn0: f64 = 0.0;
        let mut var_argbv_dn1: f64 = 0.0;
        let mut var_argbv_dn2: f64 = 0.0;
        let mut var_argbv_dn3: f64 = 0.0;
        let mut var_argbv_dn4: f64 = 0.0;
        let mut var_argbv_dn5: f64 = 0.0;
        let mut var_argbv_dn6: f64 = 0.0;
        let mut var_argbv_db0: f64 = 0.0;
        let mut var_argbv_db1: f64 = 0.0;
        let mut var_argbv_db2: f64 = 0.0;
        let mut var_argbv_db3: f64 = 0.0;
        let mut var_argbv_db4: f64 = 0.0;
        let mut var_argbv_db5: f64 = 0.0;
        let mut var_argbv_db6: f64 = 0.0;
        let mut var_argbvvt: f64 = 0.0;
        let mut var_argbvvt_dn0: f64 = 0.0;
        let mut var_argbvvt_dn1: f64 = 0.0;
        let mut var_argbvvt_dn2: f64 = 0.0;
        let mut var_argbvvt_dn3: f64 = 0.0;
        let mut var_argbvvt_dn4: f64 = 0.0;
        let mut var_argbvvt_dn5: f64 = 0.0;
        let mut var_argbvvt_dn6: f64 = 0.0;
        let mut var_argbvvt_db0: f64 = 0.0;
        let mut var_argbvvt_db1: f64 = 0.0;
        let mut var_argbvvt_db2: f64 = 0.0;
        let mut var_argbvvt_db3: f64 = 0.0;
        let mut var_argbvvt_db4: f64 = 0.0;
        let mut var_argbvvt_db5: f64 = 0.0;
        let mut var_argbvvt_db6: f64 = 0.0;
        let mut var_argtr: f64 = 0.0;
        let mut var_argtr_dn0: f64 = 0.0;
        let mut var_argtr_dn1: f64 = 0.0;
        let mut var_argtr_dn2: f64 = 0.0;
        let mut var_argtr_dn3: f64 = 0.0;
        let mut var_argtr_dn4: f64 = 0.0;
        let mut var_argtr_dn5: f64 = 0.0;
        let mut var_argtr_dn6: f64 = 0.0;
        let mut var_argtr_db0: f64 = 0.0;
        let mut var_argtr_db1: f64 = 0.0;
        let mut var_argtr_db2: f64 = 0.0;
        let mut var_argtr_db3: f64 = 0.0;
        let mut var_argtr_db4: f64 = 0.0;
        let mut var_argtr_db5: f64 = 0.0;
        let mut var_argtr_db6: f64 = 0.0;
        let mut var_isr_t: f64 = 0.0;
        let mut var_isr_t_dn0: f64 = 0.0;
        let mut var_isr_t_dn1: f64 = 0.0;
        let mut var_isr_t_dn2: f64 = 0.0;
        let mut var_isr_t_dn3: f64 = 0.0;
        let mut var_isr_t_dn4: f64 = 0.0;
        let mut var_isr_t_dn5: f64 = 0.0;
        let mut var_isr_t_dn6: f64 = 0.0;
        let mut var_isr_t_db0: f64 = 0.0;
        let mut var_isr_t_db1: f64 = 0.0;
        let mut var_isr_t_db2: f64 = 0.0;
        let mut var_isr_t_db3: f64 = 0.0;
        let mut var_isr_t_db4: f64 = 0.0;
        let mut var_isr_t_db5: f64 = 0.0;
        let mut var_isr_t_db6: f64 = 0.0;
        let mut var_vtff: f64 = 0.0;
        let mut var_vtff_dn0: f64 = 0.0;
        let mut var_vtff_dn1: f64 = 0.0;
        let mut var_vtff_dn2: f64 = 0.0;
        let mut var_vtff_dn3: f64 = 0.0;
        let mut var_vtff_dn4: f64 = 0.0;
        let mut var_vtff_dn5: f64 = 0.0;
        let mut var_vtff_dn6: f64 = 0.0;
        let mut var_vtff_db0: f64 = 0.0;
        let mut var_vtff_db1: f64 = 0.0;
        let mut var_vtff_db2: f64 = 0.0;
        let mut var_vtff_db3: f64 = 0.0;
        let mut var_vtff_db4: f64 = 0.0;
        let mut var_vtff_db5: f64 = 0.0;
        let mut var_vtff_db6: f64 = 0.0;
        let mut var_vtff1: f64 = 0.0;
        let mut var_vtff1_dn0: f64 = 0.0;
        let mut var_vtff1_dn1: f64 = 0.0;
        let mut var_vtff1_dn2: f64 = 0.0;
        let mut var_vtff1_dn3: f64 = 0.0;
        let mut var_vtff1_dn4: f64 = 0.0;
        let mut var_vtff1_dn5: f64 = 0.0;
        let mut var_vtff1_dn6: f64 = 0.0;
        let mut var_vtff1_db0: f64 = 0.0;
        let mut var_vtff1_db1: f64 = 0.0;
        let mut var_vtff1_db2: f64 = 0.0;
        let mut var_vtff1_db3: f64 = 0.0;
        let mut var_vtff1_db4: f64 = 0.0;
        let mut var_vtff1_db5: f64 = 0.0;
        let mut var_vtff1_db6: f64 = 0.0;
        let mut var_vbesat: f64 = 0.0;
        let mut var_vbesat_dn0: f64 = 0.0;
        let mut var_vbesat_dn1: f64 = 0.0;
        let mut var_vbesat_dn2: f64 = 0.0;
        let mut var_vbesat_dn3: f64 = 0.0;
        let mut var_vbesat_dn4: f64 = 0.0;
        let mut var_vbesat_dn5: f64 = 0.0;
        let mut var_vbesat_dn6: f64 = 0.0;
        let mut var_vbesat_db0: f64 = 0.0;
        let mut var_vbesat_db1: f64 = 0.0;
        let mut var_vbesat_db2: f64 = 0.0;
        let mut var_vbesat_db3: f64 = 0.0;
        let mut var_vbesat_db4: f64 = 0.0;
        let mut var_vbesat_db5: f64 = 0.0;
        let mut var_vbesat_db6: f64 = 0.0;
        let mut var_veesat: f64 = 0.0;
        let mut var_veesat_dn0: f64 = 0.0;
        let mut var_veesat_dn1: f64 = 0.0;
        let mut var_veesat_dn2: f64 = 0.0;
        let mut var_veesat_dn3: f64 = 0.0;
        let mut var_veesat_dn4: f64 = 0.0;
        let mut var_veesat_dn5: f64 = 0.0;
        let mut var_veesat_dn6: f64 = 0.0;
        let mut var_veesat_db0: f64 = 0.0;
        let mut var_veesat_db1: f64 = 0.0;
        let mut var_veesat_db2: f64 = 0.0;
        let mut var_veesat_db3: f64 = 0.0;
        let mut var_veesat_db4: f64 = 0.0;
        let mut var_veesat_db5: f64 = 0.0;
        let mut var_veesat_db6: f64 = 0.0;
        let mut var_t0: f64 = 0.0;
        let mut var_t0_dn0: f64 = 0.0;
        let mut var_t0_dn1: f64 = 0.0;
        let mut var_t0_dn2: f64 = 0.0;
        let mut var_t0_dn3: f64 = 0.0;
        let mut var_t0_dn4: f64 = 0.0;
        let mut var_t0_dn5: f64 = 0.0;
        let mut var_t0_dn6: f64 = 0.0;
        let mut var_t0_db0: f64 = 0.0;
        let mut var_t0_db1: f64 = 0.0;
        let mut var_t0_db2: f64 = 0.0;
        let mut var_t0_db3: f64 = 0.0;
        let mut var_t0_db4: f64 = 0.0;
        let mut var_t0_db5: f64 = 0.0;
        let mut var_t0_db6: f64 = 0.0;
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

        Self::stamp_transient_block_0(ctx, p, nodes, &mut var_arg0, &mut var_arg0_db0, &mut var_arg0_db1, &mut var_arg0_db2, &mut var_arg0_db3, &mut var_arg0_db4, &mut var_arg0_db5, &mut var_arg0_db6, &mut var_arg0_dn0, &mut var_arg0_dn1, &mut var_arg0_dn2, &mut var_arg0_dn3, &mut var_arg0_dn4, &mut var_arg0_dn5, &mut var_arg0_dn6, &mut var_argt, &mut var_argt_db0, &mut var_argt_db1, &mut var_argt_db2, &mut var_argt_db3, &mut var_argt_db4, &mut var_argt_db5, &mut var_argt_db6, &mut var_argt_dn0, &mut var_argt_dn1, &mut var_argt_dn2, &mut var_argt_dn3, &mut var_argt_dn4, &mut var_argt_dn5, &mut var_argt_dn6, &mut var_argtr, &mut var_argtr_db0, &mut var_argtr_db1, &mut var_argtr_db2, &mut var_argtr_db3, &mut var_argtr_db4, &mut var_argtr_db5, &mut var_argtr_db6, &mut var_argtr_dn0, &mut var_argtr_dn1, &mut var_argtr_dn2, &mut var_argtr_dn3, &mut var_argtr_dn4, &mut var_argtr_dn5, &mut var_argtr_dn6, &mut var_bvr_t, &mut var_bvr_t_db0, &mut var_bvr_t_db1, &mut var_bvr_t_db2, &mut var_bvr_t_db3, &mut var_bvr_t_db4, &mut var_bvr_t_db5, &mut var_bvr_t_db6, &mut var_bvr_t_dn0, &mut var_bvr_t_dn1, &mut var_bvr_t_dn2, &mut var_bvr_t_dn3, &mut var_bvr_t_dn4, &mut var_bvr_t_dn5, &mut var_bvr_t_dn6, &mut var_cje_i, &mut var_cje_t, &mut var_cje_t_db0, &mut var_cje_t_db1, &mut var_cje_t_db2, &mut var_cje_t_db3, &mut var_cje_t_db4, &mut var_cje_t_db5, &mut var_cje_t_db6, &mut var_cje_t_dn0, &mut var_cje_t_dn1, &mut var_cje_t_dn2, &mut var_cje_t_dn3, &mut var_cje_t_dn4, &mut var_cje_t_dn5, &mut var_cje_t_dn6, &mut var_cjt, &mut var_cjt_db0, &mut var_cjt_db1, &mut var_cjt_db2, &mut var_cjt_db3, &mut var_cjt_db4, &mut var_cjt_db5, &mut var_cjt_db6, &mut var_cjt_dn0, &mut var_cjt_dn1, &mut var_cjt_dn2, &mut var_cjt_dn3, &mut var_cjt_dn4, &mut var_cjt_dn5, &mut var_cjt_dn6, &mut var_egfet, &mut var_egfet_db0, &mut var_egfet_db1, &mut var_egfet_db2, &mut var_egfet_db3, &mut var_egfet_db4, &mut var_egfet_db5, &mut var_egfet_db6, &mut var_egfet_dn0, &mut var_egfet_dn1, &mut var_egfet_dn2, &mut var_egfet_dn3, &mut var_egfet_dn4, &mut var_egfet_dn5, &mut var_egfet_dn6, &mut var_fact1, &mut var_fact2, &mut var_fact2_db0, &mut var_fact2_db1, &mut var_fact2_db2, &mut var_fact2_db3, &mut var_fact2_db4, &mut var_fact2_db5, &mut var_fact2_db6, &mut var_fact2_dn0, &mut var_fact2_dn1, &mut var_fact2_dn2, &mut var_fact2_dn3, &mut var_fact2_dn4, &mut var_fact2_dn5, &mut var_fact2_dn6, &mut var_gmanew, &mut var_gmanew_db0, &mut var_gmanew_db1, &mut var_gmanew_db2, &mut var_gmanew_db3, &mut var_gmanew_db4, &mut var_gmanew_db5, &mut var_gmanew_db6, &mut var_gmanew_dn0, &mut var_gmanew_dn1, &mut var_gmanew_dn2, &mut var_gmanew_dn3, &mut var_gmanew_dn4, &mut var_gmanew_dn5, &mut var_gmanew_dn6, &mut var_gmaold, &mut var_gmaold_db0, &mut var_gmaold_db1, &mut var_gmaold_db2, &mut var_gmaold_db3, &mut var_gmaold_db4, &mut var_gmaold_db5, &mut var_gmaold_db6, &mut var_gmaold_dn0, &mut var_gmaold_dn1, &mut var_gmaold_dn2, &mut var_gmaold_dn3, &mut var_gmaold_dn4, &mut var_gmaold_dn5, &mut var_gmaold_dn6, &mut var_ijbv_t, &mut var_ijbv_t_db0, &mut var_ijbv_t_db1, &mut var_ijbv_t_db2, &mut var_ijbv_t_db3, &mut var_ijbv_t_db4, &mut var_ijbv_t_db5, &mut var_ijbv_t_db6, &mut var_ijbv_t_dn0, &mut var_ijbv_t_dn1, &mut var_ijbv_t_dn2, &mut var_ijbv_t_dn3, &mut var_ijbv_t_dn4, &mut var_ijbv_t_dn5, &mut var_ijbv_t_dn6, &mut var_is_t, &mut var_is_t_db0, &mut var_is_t_db1, &mut var_is_t_db2, &mut var_is_t_db3, &mut var_is_t_db4, &mut var_is_t_db5, &mut var_is_t_db6, &mut var_is_t_dn0, &mut var_is_t_dn1, &mut var_is_t_dn2, &mut var_is_t_dn3, &mut var_is_t_dn4, &mut var_is_t_dn5, &mut var_is_t_dn6, &mut var_isr_t, &mut var_isr_t_db0, &mut var_isr_t_db1, &mut var_isr_t_db2, &mut var_isr_t_db3, &mut var_isr_t_db4, &mut var_isr_t_db5, &mut var_isr_t_db6, &mut var_isr_t_dn0, &mut var_isr_t_dn1, &mut var_isr_t_dn2, &mut var_isr_t_dn3, &mut var_isr_t_dn4, &mut var_isr_t_dn5, &mut var_isr_t_dn6, &mut var_lnrt, &mut var_lnrt_db0, &mut var_lnrt_db1, &mut var_lnrt_db2, &mut var_lnrt_db3, &mut var_lnrt_db4, &mut var_lnrt_db5, &mut var_lnrt_db6, &mut var_lnrt_dn0, &mut var_lnrt_dn1, &mut var_lnrt_dn2, &mut var_lnrt_dn3, &mut var_lnrt_dn4, &mut var_lnrt_dn5, &mut var_lnrt_dn6, &mut var_pbfact, &mut var_pbfact_db0, &mut var_pbfact_db1, &mut var_pbfact_db2, &mut var_pbfact_db3, &mut var_pbfact_db4, &mut var_pbfact_db5, &mut var_pbfact_db6, &mut var_pbfact_dn0, &mut var_pbfact_dn1, &mut var_pbfact_dn2, &mut var_pbfact_dn3, &mut var_pbfact_dn4, &mut var_pbfact_dn5, &mut var_pbfact_dn6, &mut var_pbo, &mut var_pbo_db0, &mut var_pbo_db1, &mut var_pbo_db2, &mut var_pbo_db3, &mut var_pbo_db4, &mut var_pbo_db5, &mut var_pbo_db6, &mut var_pbo_dn0, &mut var_pbo_dn1, &mut var_pbo_dn2, &mut var_pbo_dn3, &mut var_pbo_dn4, &mut var_pbo_dn5, &mut var_pbo_dn6, &mut var_rt, &mut var_rt_db0, &mut var_rt_db1, &mut var_rt_db2, &mut var_rt_db3, &mut var_rt_db4, &mut var_rt_db5, &mut var_rt_db6, &mut var_rt_dn0, &mut var_rt_dn1, &mut var_rt_dn2, &mut var_rt_dn3, &mut var_rt_dn4, &mut var_rt_dn5, &mut var_rt_dn6, &mut var_tamb, &mut var_tamb_db0, &mut var_tamb_db1, &mut var_tamb_db2, &mut var_tamb_db3, &mut var_tamb_db4, &mut var_tamb_db5, &mut var_tamb_db6, &mut var_tamb_dn0, &mut var_tamb_dn1, &mut var_tamb_dn2, &mut var_tamb_dn3, &mut var_tamb_dn4, &mut var_tamb_dn5, &mut var_tamb_dn6, &mut var_tdev, &mut var_tdev_db0, &mut var_tdev_db1, &mut var_tdev_db2, &mut var_tdev_db3, &mut var_tdev_db4, &mut var_tdev_db5, &mut var_tdev_db6, &mut var_tdev_dn0, &mut var_tdev_dn1, &mut var_tdev_dn2, &mut var_tdev_dn3, &mut var_tdev_dn4, &mut var_tdev_dn5, &mut var_tdev_dn6, &mut var_theexp_t, &mut var_theexp_t_db0, &mut var_theexp_t_db1, &mut var_theexp_t_db2, &mut var_theexp_t_db3, &mut var_theexp_t_db4, &mut var_theexp_t_db5, &mut var_theexp_t_db6, &mut var_theexp_t_dn0, &mut var_theexp_t_dn1, &mut var_theexp_t_dn2, &mut var_theexp_t_dn3, &mut var_theexp_t_dn4, &mut var_theexp_t_dn5, &mut var_theexp_t_dn6, &mut var_tnom, &mut var_ttype, &mut var_vbbi, &mut var_vbbi_db0, &mut var_vbbi_db1, &mut var_vbbi_db2, &mut var_vbbi_db3, &mut var_vbbi_db4, &mut var_vbbi_db5, &mut var_vbbi_db6, &mut var_vbbi_dn0, &mut var_vbbi_dn1, &mut var_vbbi_dn2, &mut var_vbbi_dn3, &mut var_vbbi_dn4, &mut var_vbbi_dn5, &mut var_vbbi_dn6, &mut var_vbiei, &mut var_vbiei_db0, &mut var_vbiei_db1, &mut var_vbiei_db2, &mut var_vbiei_db3, &mut var_vbiei_db4, &mut var_vbiei_db5, &mut var_vbiei_db6, &mut var_vbiei_dn0, &mut var_vbiei_dn1, &mut var_vbiei_dn2, &mut var_vbiei_dn3, &mut var_vbiei_dn4, &mut var_vbiei_dn5, &mut var_vbiei_dn6, &mut var_vje_t, &mut var_vje_t_db0, &mut var_vje_t_db1, &mut var_vje_t_db2, &mut var_vje_t_db3, &mut var_vje_t_db4, &mut var_vje_t_db5, &mut var_vje_t_db6, &mut var_vje_t_dn0, &mut var_vje_t_dn1, &mut var_vje_t_dn2, &mut var_vje_t_dn3, &mut var_vje_t_dn4, &mut var_vje_t_dn5, &mut var_vje_t_dn6, &mut var_vt, &mut var_vt_db0, &mut var_vt_db1, &mut var_vt_db2, &mut var_vt_db3, &mut var_vt_db4, &mut var_vt_db5, &mut var_vt_db6, &mut var_vt_dn0, &mut var_vt_dn1, &mut var_vt_dn2, &mut var_vt_dn3, &mut var_vt_dn4, &mut var_vt_dn5, &mut var_vt_dn6, &mut var_weff);
        Self::stamp_transient_block_1(ctx, p, nodes, var_bvr_t, var_bvr_t_db0, var_bvr_t_db1, var_bvr_t_db2, var_bvr_t_db3, var_bvr_t_db4, var_bvr_t_db5, var_bvr_t_db6, var_bvr_t_dn0, var_bvr_t_dn1, var_bvr_t_dn2, var_bvr_t_dn3, var_bvr_t_dn4, var_bvr_t_dn5, var_bvr_t_dn6, var_ijbv_t, var_ijbv_t_db0, var_ijbv_t_db1, var_ijbv_t_db2, var_ijbv_t_db3, var_ijbv_t_db4, var_ijbv_t_db5, var_ijbv_t_db6, var_ijbv_t_dn0, var_ijbv_t_dn1, var_ijbv_t_dn2, var_ijbv_t_dn3, var_ijbv_t_dn4, var_ijbv_t_dn5, var_ijbv_t_dn6, var_is_t, var_is_t_db0, var_is_t_db1, var_is_t_db2, var_is_t_db3, var_is_t_db4, var_is_t_db5, var_is_t_db6, var_is_t_dn0, var_is_t_dn1, var_is_t_dn2, var_is_t_dn3, var_is_t_dn4, var_is_t_dn5, var_is_t_dn6, var_isr_t, var_isr_t_db0, var_isr_t_db1, var_isr_t_db2, var_isr_t_db3, var_isr_t_db4, var_isr_t_db5, var_isr_t_db6, var_isr_t_dn0, var_isr_t_dn1, var_isr_t_dn2, var_isr_t_dn3, var_isr_t_dn4, var_isr_t_dn5, var_isr_t_dn6, var_theexp_t, var_theexp_t_db0, var_theexp_t_db1, var_theexp_t_db2, var_theexp_t_db3, var_theexp_t_db4, var_theexp_t_db5, var_theexp_t_db6, var_theexp_t_dn0, var_theexp_t_dn1, var_theexp_t_dn2, var_theexp_t_dn3, var_theexp_t_dn4, var_theexp_t_dn5, var_theexp_t_dn6, var_ttype, var_vbiei, var_vbiei_db0, var_vbiei_db1, var_vbiei_db2, var_vbiei_db3, var_vbiei_db4, var_vbiei_db5, var_vbiei_db6, var_vbiei_dn0, var_vbiei_dn1, var_vbiei_dn2, var_vbiei_dn3, var_vbiei_dn4, var_vbiei_dn5, var_vbiei_dn6, var_vt, var_vt_db0, var_vt_db1, var_vt_db2, var_vt_db3, var_vt_db4, var_vt_db5, var_vt_db6, var_vt_dn0, var_vt_dn1, var_vt_dn2, var_vt_dn3, var_vt_dn4, var_vt_dn5, var_vt_dn6, &mut var_arg, &mut var_arg_db0, &mut var_arg_db1, &mut var_arg_db2, &mut var_arg_db3, &mut var_arg_db4, &mut var_arg_db5, &mut var_arg_db6, &mut var_arg_dn0, &mut var_arg_dn1, &mut var_arg_dn2, &mut var_arg_dn3, &mut var_arg_dn4, &mut var_arg_dn5, &mut var_arg_dn6, &mut var_argbv, &mut var_argbv_db0, &mut var_argbv_db1, &mut var_argbv_db2, &mut var_argbv_db3, &mut var_argbv_db4, &mut var_argbv_db5, &mut var_argbv_db6, &mut var_argbv_dn0, &mut var_argbv_dn1, &mut var_argbv_dn2, &mut var_argbv_dn3, &mut var_argbv_dn4, &mut var_argbv_dn5, &mut var_argbv_dn6, &mut var_argbvvt, &mut var_argbvvt_db0, &mut var_argbvvt_db1, &mut var_argbvvt_db2, &mut var_argbvvt_db3, &mut var_argbvvt_db4, &mut var_argbvvt_db5, &mut var_argbvvt_db6, &mut var_argbvvt_dn0, &mut var_argbvvt_dn1, &mut var_argbvvt_dn2, &mut var_argbvvt_dn3, &mut var_argbvvt_dn4, &mut var_argbvvt_dn5, &mut var_argbvvt_dn6, &mut var_guard3, &mut var_guard4, &mut var_guard5, &mut var_guard6, &mut var_ifwd, &mut var_ifwd_db0, &mut var_ifwd_db1, &mut var_ifwd_db2, &mut var_ifwd_db3, &mut var_ifwd_db4, &mut var_ifwd_db5, &mut var_ifwd_db6, &mut var_ifwd_dn0, &mut var_ifwd_dn1, &mut var_ifwd_dn2, &mut var_ifwd_dn3, &mut var_ifwd_dn4, &mut var_ifwd_dn5, &mut var_ifwd_dn6, &mut var_itrev, &mut var_itrev_db0, &mut var_itrev_db1, &mut var_itrev_db2, &mut var_itrev_db3, &mut var_itrev_db4, &mut var_itrev_db5, &mut var_itrev_db6, &mut var_itrev_dn0, &mut var_itrev_dn1, &mut var_itrev_dn2, &mut var_itrev_dn3, &mut var_itrev_dn4, &mut var_itrev_dn5, &mut var_itrev_dn6, &mut var_le, &mut var_le_db0, &mut var_le_db1, &mut var_le_db2, &mut var_le_db3, &mut var_le_db4, &mut var_le_db5, &mut var_le_db6, &mut var_le_dn0, &mut var_le_dn1, &mut var_le_dn2, &mut var_le_dn3, &mut var_le_dn4, &mut var_le_dn5, &mut var_le_dn6, &mut var_lebv, &mut var_lebv_db0, &mut var_lebv_db1, &mut var_lebv_db2, &mut var_lebv_db3, &mut var_lebv_db4, &mut var_lebv_db5, &mut var_lebv_db6, &mut var_lebv_dn0, &mut var_lebv_dn1, &mut var_lebv_dn2, &mut var_lebv_dn3, &mut var_lebv_dn4, &mut var_lebv_dn5, &mut var_lebv_dn6, &mut var_t0, &mut var_t0_db0, &mut var_t0_db1, &mut var_t0_db2, &mut var_t0_db3, &mut var_t0_db4, &mut var_t0_db5, &mut var_t0_db6, &mut var_t0_dn0, &mut var_t0_dn1, &mut var_t0_dn2, &mut var_t0_dn3, &mut var_t0_dn4, &mut var_t0_dn5, &mut var_t0_dn6, &mut var_veei, &mut var_veei_db0, &mut var_veei_db1, &mut var_veei_db2, &mut var_veei_db3, &mut var_veei_db4, &mut var_veei_db5, &mut var_veei_db6, &mut var_veei_dn0, &mut var_veei_dn1, &mut var_veei_dn2, &mut var_veei_dn3, &mut var_veei_dn4, &mut var_veei_dn5, &mut var_veei_dn6);
        Self::stamp_transient_block_2(ctx, p, nodes, var_cje_t, var_cje_t_db0, var_cje_t_db1, var_cje_t_db2, var_cje_t_db3, var_cje_t_db4, var_cje_t_db5, var_cje_t_db6, var_cje_t_dn0, var_cje_t_dn1, var_cje_t_dn2, var_cje_t_dn3, var_cje_t_dn4, var_cje_t_dn5, var_cje_t_dn6, var_guard5, var_ifwd, var_ifwd_db0, var_ifwd_db1, var_ifwd_db2, var_ifwd_db3, var_ifwd_db4, var_ifwd_db5, var_ifwd_db6, var_ifwd_dn0, var_ifwd_dn1, var_ifwd_dn2, var_ifwd_dn3, var_ifwd_dn4, var_ifwd_dn5, var_ifwd_dn6, var_lnrt, var_lnrt_db0, var_lnrt_db1, var_lnrt_db2, var_lnrt_db3, var_lnrt_db4, var_lnrt_db5, var_lnrt_db6, var_lnrt_dn0, var_lnrt_dn1, var_lnrt_dn2, var_lnrt_dn3, var_lnrt_dn4, var_lnrt_dn5, var_lnrt_dn6, var_vbbi, var_vbbi_db0, var_vbbi_db1, var_vbbi_db2, var_vbbi_db3, var_vbbi_db4, var_vbbi_db5, var_vbbi_db6, var_vbbi_dn0, var_vbbi_dn1, var_vbbi_dn2, var_vbbi_dn3, var_vbbi_dn4, var_vbbi_dn5, var_vbbi_dn6, var_vbiei, var_vbiei_db0, var_vbiei_db1, var_vbiei_db2, var_vbiei_db3, var_vbiei_db4, var_vbiei_db5, var_vbiei_db6, var_vbiei_dn0, var_vbiei_dn1, var_vbiei_dn2, var_vbiei_dn3, var_vbiei_dn4, var_vbiei_dn5, var_vbiei_dn6, var_veei, var_veei_db0, var_veei_db1, var_veei_db2, var_veei_db3, var_veei_db4, var_veei_db5, var_veei_db6, var_veei_dn0, var_veei_dn1, var_veei_dn2, var_veei_dn3, var_veei_dn4, var_veei_dn5, var_veei_dn6, var_vje_t, var_vje_t_db0, var_vje_t_db1, var_vje_t_db2, var_vje_t_db3, var_vje_t_db4, var_vje_t_db5, var_vje_t_db6, var_vje_t_dn0, var_vje_t_dn1, var_vje_t_dn2, var_vje_t_dn3, var_vje_t_dn4, var_vje_t_dn5, var_vje_t_dn6, &mut var_dv0, &mut var_dv0_db0, &mut var_dv0_db1, &mut var_dv0_db2, &mut var_dv0_db3, &mut var_dv0_db4, &mut var_dv0_db5, &mut var_dv0_db6, &mut var_dv0_dn0, &mut var_dv0_dn1, &mut var_dv0_dn2, &mut var_dv0_dn3, &mut var_dv0_dn4, &mut var_dv0_dn5, &mut var_dv0_dn6, &mut var_dvh, &mut var_dvh_db0, &mut var_dvh_db1, &mut var_dvh_db2, &mut var_dvh_db3, &mut var_dvh_db4, &mut var_dvh_db5, &mut var_dvh_db6, &mut var_dvh_dn0, &mut var_dvh_dn1, &mut var_dvh_dn2, &mut var_dvh_dn3, &mut var_dvh_dn4, &mut var_dvh_dn5, &mut var_dvh_dn6, &mut var_guard10, &mut var_guard7, &mut var_guard8, &mut var_guard9, &mut var_ibe, &mut var_ibe_db0, &mut var_ibe_db1, &mut var_ibe_db2, &mut var_ibe_db3, &mut var_ibe_db4, &mut var_ibe_db5, &mut var_ibe_db6, &mut var_ibe_dn0, &mut var_ibe_dn1, &mut var_ibe_dn2, &mut var_ibe_dn3, &mut var_ibe_dn4, &mut var_ibe_dn5, &mut var_ibe_dn6, &mut var_itrev, &mut var_itrev_db0, &mut var_itrev_db1, &mut var_itrev_db2, &mut var_itrev_db3, &mut var_itrev_db4, &mut var_itrev_db5, &mut var_itrev_db6, &mut var_itrev_dn0, &mut var_itrev_dn1, &mut var_itrev_dn2, &mut var_itrev_dn3, &mut var_itrev_dn4, &mut var_itrev_dn5, &mut var_itrev_dn6, &mut var_itzf, &mut var_itzf_db0, &mut var_itzf_db1, &mut var_itzf_db2, &mut var_itzf_db3, &mut var_itzf_db4, &mut var_itzf_db5, &mut var_itzf_db6, &mut var_itzf_dn0, &mut var_itzf_dn1, &mut var_itzf_dn2, &mut var_itzf_dn3, &mut var_itzf_dn4, &mut var_itzf_dn5, &mut var_itzf_dn6, &mut var_pwq, &mut var_qde, &mut var_qde_db0, &mut var_qde_db1, &mut var_qde_db2, &mut var_qde_db3, &mut var_qde_db4, &mut var_qde_db5, &mut var_qde_db6, &mut var_qde_dn0, &mut var_qde_dn1, &mut var_qde_dn2, &mut var_qde_dn3, &mut var_qde_dn4, &mut var_qde_dn5, &mut var_qde_dn6, &mut var_qhi, &mut var_qhi_db0, &mut var_qhi_db1, &mut var_qhi_db2, &mut var_qhi_db3, &mut var_qhi_db4, &mut var_qhi_db5, &mut var_qhi_db6, &mut var_qhi_dn0, &mut var_qhi_dn1, &mut var_qhi_dn2, &mut var_qhi_dn3, &mut var_qhi_dn4, &mut var_qhi_dn5, &mut var_qhi_dn6, &mut var_qje, &mut var_qje_db0, &mut var_qje_db1, &mut var_qje_db2, &mut var_qje_db3, &mut var_qje_db4, &mut var_qje_db5, &mut var_qje_db6, &mut var_qje_dn0, &mut var_qje_dn1, &mut var_qje_dn2, &mut var_qje_dn3, &mut var_qje_dn4, &mut var_qje_dn5, &mut var_qje_dn6, &mut var_qlo, &mut var_qlo_db0, &mut var_qlo_db1, &mut var_qlo_db2, &mut var_qlo_db3, &mut var_qlo_db4, &mut var_qlo_db5, &mut var_qlo_db6, &mut var_qlo_dn0, &mut var_qlo_dn1, &mut var_qlo_dn2, &mut var_qlo_dn3, &mut var_qlo_dn4, &mut var_qlo_dn5, &mut var_qlo_dn6, &mut var_rb, &mut var_rb_db0, &mut var_rb_db1, &mut var_rb_db2, &mut var_rb_db3, &mut var_rb_db4, &mut var_rb_db5, &mut var_rb_db6, &mut var_rb_dn0, &mut var_rb_dn1, &mut var_rb_dn2, &mut var_rb_dn3, &mut var_rb_dn4, &mut var_rb_dn5, &mut var_rb_dn6, &mut var_re, &mut var_re_db0, &mut var_re_db1, &mut var_re_db2, &mut var_re_db3, &mut var_re_db4, &mut var_re_db5, &mut var_re_db6, &mut var_re_dn0, &mut var_re_dn1, &mut var_re_dn2, &mut var_re_dn3, &mut var_re_dn4, &mut var_re_dn5, &mut var_re_dn6, &mut var_tff, &mut var_tff_db0, &mut var_tff_db1, &mut var_tff_db2, &mut var_tff_db3, &mut var_tff_db4, &mut var_tff_db5, &mut var_tff_db6, &mut var_tff_dn0, &mut var_tff_dn1, &mut var_tff_dn2, &mut var_tff_dn3, &mut var_tff_dn4, &mut var_tff_dn5, &mut var_tff_dn6, &mut var_vbesat, &mut var_vbesat_db0, &mut var_vbesat_db1, &mut var_vbesat_db2, &mut var_vbesat_db3, &mut var_vbesat_db4, &mut var_vbesat_db5, &mut var_vbesat_db6, &mut var_vbesat_dn0, &mut var_vbesat_dn1, &mut var_vbesat_dn2, &mut var_vbesat_dn3, &mut var_vbesat_dn4, &mut var_vbesat_dn5, &mut var_vbesat_dn6, &mut var_veesat, &mut var_veesat_db0, &mut var_veesat_db1, &mut var_veesat_db2, &mut var_veesat_db3, &mut var_veesat_db4, &mut var_veesat_db5, &mut var_veesat_db6, &mut var_veesat_dn0, &mut var_veesat_dn1, &mut var_veesat_dn2, &mut var_veesat_dn3, &mut var_veesat_dn4, &mut var_veesat_dn5, &mut var_veesat_dn6, &mut var_vtff, &mut var_vtff1, &mut var_vtff1_db0, &mut var_vtff1_db1, &mut var_vtff1_db2, &mut var_vtff1_db3, &mut var_vtff1_db4, &mut var_vtff1_db5, &mut var_vtff1_db6, &mut var_vtff1_dn0, &mut var_vtff1_dn1, &mut var_vtff1_dn2, &mut var_vtff1_dn3, &mut var_vtff1_dn4, &mut var_vtff1_dn5, &mut var_vtff1_dn6, &mut var_vtff_db0, &mut var_vtff_db1, &mut var_vtff_db2, &mut var_vtff_db3, &mut var_vtff_db4, &mut var_vtff_db5, &mut var_vtff_db6, &mut var_vtff_dn0, &mut var_vtff_dn1, &mut var_vtff_dn2, &mut var_vtff_dn3, &mut var_vtff_dn4, &mut var_vtff_dn5, &mut var_vtff_dn6);
        Self::stamp_transient_block_3(p, var_weff, &mut var_guard11, &mut var_guard12, &mut var_guard13, &mut var_guard14, &mut var_rb_nom, &mut var_re_nom);

        Self::stamp_transient_equations_block_0(ctx, stamper, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, var_guard10, var_guard11, var_guard12, var_guard13, var_guard14, var_guard8, var_ibe, var_ibe_db0, var_ibe_db1, var_ibe_db2, var_ibe_db3, var_ibe_db4, var_ibe_db5, var_ibe_db6, var_ibe_dn0, var_ibe_dn1, var_ibe_dn2, var_ibe_dn3, var_ibe_dn4, var_ibe_dn5, var_ibe_dn6, var_ifwd, var_ifwd_db0, var_ifwd_db1, var_ifwd_db2, var_ifwd_db3, var_ifwd_db4, var_ifwd_db5, var_ifwd_db6, var_ifwd_dn0, var_ifwd_dn1, var_ifwd_dn2, var_ifwd_dn3, var_ifwd_dn4, var_ifwd_dn5, var_ifwd_dn6, var_rb, var_rb_db0, var_rb_db1, var_rb_db2, var_rb_db3, var_rb_db4, var_rb_db5, var_rb_db6, var_rb_dn0, var_rb_dn1, var_rb_dn2, var_rb_dn3, var_rb_dn4, var_rb_dn5, var_rb_dn6, var_re, var_re_db0, var_re_db1, var_re_db2, var_re_db3, var_re_db4, var_re_db5, var_re_db6, var_re_dn0, var_re_dn1, var_re_dn2, var_re_dn3, var_re_dn4, var_re_dn5, var_re_dn6, var_tff, var_tff_db0, var_tff_db1, var_tff_db2, var_tff_db3, var_tff_db4, var_tff_db5, var_tff_db6, var_tff_dn0, var_tff_dn1, var_tff_dn2, var_tff_dn3, var_tff_dn4, var_tff_dn5, var_tff_dn6, var_weff);
        Self::stamp_transient_equations_block_1(stamper, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, var_ibe, var_ibe_db0, var_ibe_db1, var_ibe_db2, var_ibe_db3, var_ibe_db4, var_ibe_db5, var_ibe_db6, var_ibe_dn0, var_ibe_dn1, var_ibe_dn2, var_ibe_dn3, var_ibe_dn4, var_ibe_dn5, var_ibe_dn6, var_qde, var_qde_db0, var_qde_db1, var_qde_db2, var_qde_db3, var_qde_db4, var_qde_db5, var_qde_db6, var_qde_dn0, var_qde_dn1, var_qde_dn2, var_qde_dn3, var_qde_dn4, var_qde_dn5, var_qde_dn6, var_qje, var_qje_db0, var_qje_db1, var_qje_db2, var_qje_db3, var_qje_db4, var_qje_db5, var_qje_db6, var_qje_dn0, var_qje_dn1, var_qje_dn2, var_qje_dn3, var_qje_dn4, var_qje_dn5, var_qje_dn6, var_ttype, var_weff);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let ctx_temp = ctx.temperature();
        let multiplicity = (*self).multiplicity;
        let s = match &mut self.reactive_scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(ReactiveScratch::new_box()).as_mut(),
        };

        s.store_offset_voltage(12, ctx, nodes, Some(2), None, ((ctx_temp) + (p.p45)));

        if ((1026.85 + 273.15) < (if (s.v[12] > ((-100.0) + 273.15)) { s.v[12] } else { ((-100.0) + 273.15) })) {
            s.store_scalar(10, (1026.85 + 273.15));
        } else {
            if (s.v[12] > ((-100.0) + 273.15)) {
                s.copy_ad(10, 12);
            } else {
                s.store_scalar(10, ((-100.0) + 273.15));
            }
        }

        s.store_scalar(3, (p.p43 * p.p42));

        s.store_scalar(11, (p.p25 + 273.15));

        s.store_scale(15, 10, 8.6170869e-5);

        s.store_scale(13, 10, 1.0 / (s.v[11]));

        s.store_ln(14, 13);

        s.store_add_scaled_ad_rhs(34, 14, p.p22, A::div_scaled_offset_numerator(s.ad_value(13), p.p21, ((-1.0) * p.p21), s.ad_value(15), 1.0));

        s.store_scale(54, 14, p.p23);

        s.store_scaled_exp(16, 34, p.p0);

        s.store_scaled_exp(55, 54, p.p2);

        s.store_offset_scaled(19, 13, ((p.p7) * (p.p47)), (((((((-1.0)) * (p.p7))) + (1.0))) * (p.p47)));

        s.store_offset_scaled(20, 13, ((p.p6) * (p.p5)), (((((((-1.0)) * (p.p6))) + (1.0))) * (p.p5)));

        s.store_offset_scaled(21, 13, ((p.p10) * (p.p9)), (((((((-1.0)) * (p.p10))) + (1.0))) * (p.p9)));

        s.store_scalar(22, p.p16);

        s.store_scalar(43, (s.v[11] / 300.15));

        s.store_scale(44, 10, 0.003331667499583542);

        s.store_sub_from_scalar_ad(45, 1.16, A::div_scaled_product_offset_denominator(s.ad_value(10), s.ad_value(10), 0.000702, s.ad_value(10), 1108.0, 1.0));

        s.store_offset_div_scaled_inputs_indices(46, 45, -1.0, 10, (2.0 * 1.3806226e-23), (1.1150877 / (1.3806226e-23 * (300.15 + 300.15))));

        s.store_scaled_mul_ad(47, A::scale(s.ad_value(15), 2.0), A::add_scaled_inputs(A::ln(s.ad_value(44)), 1.5, s.ad_value(46), 1.6021918e-19), -1.0);

        s.store_offset_scaled(48, 47, (-1.0 / (s.v[43])), ((p.p17) * (1.0 / (s.v[43]))));

        s.store_div_ad_lhs(49, A::sub_from_scalar(p.p17, s.ad_value(48)), 48);

        s.store_div_from_scalar_offset_scaled_input(51, s.v[22], 49, (-p.p18), (((((0.0004 * (s.v[11] - 300.15))) * (p.p18))) + (1.0)));

        s.store_add_scaled_product_indices(18, 47, 1.0, 44, 48, 1.0);

        s.store_div_scaled_inputs2_indices(50, 18, 1.0, 48, (-1.0), 48, 1.0);

        s.store_mul_offset_ad_rhs(17, 51, A::sub_scaled_inputs(A::scaled_offset(s.ad_value(10), (-300.15), 0.0004), p.p18, s.ad_value(50), p.p18), 1.0);

        s.store_scalar(9, p.p29);

        s.store_scaled_voltage(40, ctx, nodes, Some(3), Some(4), s.v[9]);

        s.b[63] = (s.v[16] > 0.0);
        s.store_scalar(63, if s.b[63] { 1.0 } else { 0.0 });

        if s.b[63] {
            s.store_div_scaled_inputs_indices(0, 40, 1.0, 15, p.p1);
            s.store_div_scaled_inputs2_indices(52, 40, -1.0, 20, (-1.0), 15, p.p11);
            s.store_div_scaled_inputs_indices(53, 20, -1.0, 15, p.p11);
        }

        s.b[64] = (s.v[0] > 80.0);
        s.store_scalar(64, if s.b[64] { 1.0 } else { 0.0 });

        if (s.b[63] && s.b[64]) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
            s.store_scalar(0, 80.0);
        }

        if (s.b[63] && (!s.b[64])) {
            s.store_scalar(1, 1.0);
        }

        if s.b[63] {
            s.store_mul_exp_rhs(1, 1, 0);
        }

        if s.b[63] {
            s.store_sub_ad(2, {
                if ((!(s.v[52] >= 37.0)) && (!(s.v[52] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(52))
                } else {
                    {
                        if ((!(s.v[52] >= 37.0)) && (s.v[52] <= (-37.0))) {
                            A::exp(s.ad_value(52))
                        } else {
                            {
                                if (s.v[52] >= 37.0) {
                                    s.ad_value(52)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, {
                if ((!(s.v[53] >= 37.0)) && (!(s.v[53] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(53))
                } else {
                    {
                        if ((!(s.v[53] >= 37.0)) && (s.v[53] <= (-37.0))) {
                            A::exp(s.ad_value(53))
                        } else {
                            {
                                if (s.v[53] >= 37.0) {
                                    s.ad_value(53)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            });
        }

        if s.b[63] {
            s.store_add_scaled_offset_product_rhs_mixed_aii(23, A::div_scaled_product(s.ad_value(19), s.ad_value(2), 1.0, A::scale_offset(A::pow(A::abs(s.ad_value(40)), s.ad_value(21)), p.p8, 1.0), 1.0), (-1.0), 16, 1, (-1.0), 1.0);
        }

        if (!s.b[63]) {
            s.store_scalar(23, 0.0);
        }

        s.b[65] = (s.v[55] > 0.0);
        s.store_scalar(65, if s.b[65] { 1.0 } else { 0.0 });

        if s.b[65] {
            s.store_max_with_scalar_ad(60, A::sub_from_scalar(p.p4, s.ad_value(40)), 0.001);
            s.store_div_scaled_inputs_mixed_ia(0, 40, ((-1.0) * p.p4), A::mul_scaled_lhs(s.ad_value(15), p.p3, s.ad_value(60)), 1.0);
        }

        s.b[66] = (s.v[0] > 80.0);
        s.store_scalar(66, if s.b[66] { 1.0 } else { 0.0 });

        if (s.b[65] && s.b[66]) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
            s.store_scalar(0, 80.0);
        }

        if (s.b[65] && (!s.b[66])) {
            s.store_scalar(1, 1.0);
        }

        if s.b[65] {
            s.store_mul_exp_rhs(1, 1, 0);
        }

        s.copy_ad(25, 23);

        s.store_powf_ad(56, A::abs_scaled_input(A::voltage(ctx, nodes, Some(0), Some(1)), 1.0 / (p.p40)), p.p39);

        s.store_offset_powf_ad(57, A::offset(s.ad_value(56), 1.0), (1.0 / p.p39), (-1.0));

        s.store_offset_scaled(31, 57, ((p.p41) * (p.p19)), p.p19);

        s.store_mul(32, 31, 25);

        s.b[68] = (p.p32 == 1.0);
        s.store_scalar(68, if s.b[68] { 1.0 } else { 0.0 });

        s.store_scale(4, 18, (-p.p24));

        s.store_add(5, 40, 4);

        s.b[69] = (s.v[5] > 0.0);
        s.store_scalar(69, if s.b[69] { 1.0 } else { 0.0 });

        if s.b[69] {
            s.store_scalar(6, (((((-1.0) - p.p18) * (((1.0 - p.p24)) as f64).ln())) as f64).exp());
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(7, 18, 1.0, 6, ((1.0 - p.p24) * (1.0 - p.p24)), 1.0 / ((1.0 - p.p18)));
            s.store_mul_ad_product_lhs_mixed_ia(8, 5, A::offset(A::div_scaled_inputs(s.ad_value(5), (0.5 * p.p18), s.ad_value(18), 1.0), (1.0 - p.p24)), 6);
        }

        if (!s.b[69]) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(7, 18, 1.0, A::exp_scaled_input(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(40), s.ad_value(18)))), (1.0 - p.p18)), 1.0 / ((1.0 - p.p18)));
            s.store_scalar(8, 0.0);
        }

        s.store_mul_add_rhs(33, 17, 7, 8);

        s.b[70] = ((p.p30 == 1.0) && (p.p33 > 0.0));
        s.store_scalar(70, if s.b[70] { 1.0 } else { 0.0 });

        s.b[71] = (((p.p30 == 2.0) && (p.p33 > 0.0)) && (p.p35 > 0.0));
        s.store_scalar(71, if s.b[71] { 1.0 } else { 0.0 });

        Self::stamp_reactive_equations_block_0(ctx, stamper, s, p, nodes, branches, multiplicity);
    }
}
