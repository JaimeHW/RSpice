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
        let mut var_arg_dn0: f64 = 0.0;
        let mut var_arg_dn1: f64 = 0.0;
        let mut var_arg_dn2: f64 = 0.0;
        let mut var_arg_dn3: f64 = 0.0;
        let mut var_arg_dn4: f64 = 0.0;
        let mut var_arg_dn5: f64 = 0.0;
        let mut var_arg_dn6: f64 = 0.0;
        let mut var_arg_dn7: f64 = 0.0;
        let mut var_arg_dn8: f64 = 0.0;
        let mut var_arg_dn9: f64 = 0.0;
        let mut var_arg_db0: f64 = 0.0;
        let mut var_arg_db1: f64 = 0.0;
        let mut var_arg_db2: f64 = 0.0;
        let mut var_arg_db3: f64 = 0.0;
        let mut var_arg_db4: f64 = 0.0;
        let mut var_arg_db5: f64 = 0.0;
        let mut var_arg_db6: f64 = 0.0;
        let mut var_arg_db7: f64 = 0.0;
        let mut var_le: f64 = 0.0;
        let mut var_le_dn0: f64 = 0.0;
        let mut var_le_dn1: f64 = 0.0;
        let mut var_le_dn2: f64 = 0.0;
        let mut var_le_dn3: f64 = 0.0;
        let mut var_le_dn4: f64 = 0.0;
        let mut var_le_dn5: f64 = 0.0;
        let mut var_le_dn6: f64 = 0.0;
        let mut var_le_dn7: f64 = 0.0;
        let mut var_le_dn8: f64 = 0.0;
        let mut var_le_dn9: f64 = 0.0;
        let mut var_le_db0: f64 = 0.0;
        let mut var_le_db1: f64 = 0.0;
        let mut var_le_db2: f64 = 0.0;
        let mut var_le_db3: f64 = 0.0;
        let mut var_le_db4: f64 = 0.0;
        let mut var_le_db5: f64 = 0.0;
        let mut var_le_db6: f64 = 0.0;
        let mut var_le_db7: f64 = 0.0;
        let mut var_lebv: f64 = 0.0;
        let mut var_lebv_dn0: f64 = 0.0;
        let mut var_lebv_dn1: f64 = 0.0;
        let mut var_lebv_dn2: f64 = 0.0;
        let mut var_lebv_dn3: f64 = 0.0;
        let mut var_lebv_dn4: f64 = 0.0;
        let mut var_lebv_dn5: f64 = 0.0;
        let mut var_lebv_dn6: f64 = 0.0;
        let mut var_lebv_dn7: f64 = 0.0;
        let mut var_lebv_dn8: f64 = 0.0;
        let mut var_lebv_dn9: f64 = 0.0;
        let mut var_lebv_db0: f64 = 0.0;
        let mut var_lebv_db1: f64 = 0.0;
        let mut var_lebv_db2: f64 = 0.0;
        let mut var_lebv_db3: f64 = 0.0;
        let mut var_lebv_db4: f64 = 0.0;
        let mut var_lebv_db5: f64 = 0.0;
        let mut var_lebv_db6: f64 = 0.0;
        let mut var_lebv_db7: f64 = 0.0;
        let mut var_weff: f64 = 0.0;
        let mut var_dv0: f64 = 0.0;
        let mut var_dv0_dn0: f64 = 0.0;
        let mut var_dv0_dn1: f64 = 0.0;
        let mut var_dv0_dn2: f64 = 0.0;
        let mut var_dv0_dn3: f64 = 0.0;
        let mut var_dv0_dn4: f64 = 0.0;
        let mut var_dv0_dn5: f64 = 0.0;
        let mut var_dv0_dn6: f64 = 0.0;
        let mut var_dv0_dn7: f64 = 0.0;
        let mut var_dv0_dn8: f64 = 0.0;
        let mut var_dv0_dn9: f64 = 0.0;
        let mut var_dv0_db0: f64 = 0.0;
        let mut var_dv0_db1: f64 = 0.0;
        let mut var_dv0_db2: f64 = 0.0;
        let mut var_dv0_db3: f64 = 0.0;
        let mut var_dv0_db4: f64 = 0.0;
        let mut var_dv0_db5: f64 = 0.0;
        let mut var_dv0_db6: f64 = 0.0;
        let mut var_dv0_db7: f64 = 0.0;
        let mut var_dvh: f64 = 0.0;
        let mut var_dvh_dn0: f64 = 0.0;
        let mut var_dvh_dn1: f64 = 0.0;
        let mut var_dvh_dn2: f64 = 0.0;
        let mut var_dvh_dn3: f64 = 0.0;
        let mut var_dvh_dn4: f64 = 0.0;
        let mut var_dvh_dn5: f64 = 0.0;
        let mut var_dvh_dn6: f64 = 0.0;
        let mut var_dvh_dn7: f64 = 0.0;
        let mut var_dvh_dn8: f64 = 0.0;
        let mut var_dvh_dn9: f64 = 0.0;
        let mut var_dvh_db0: f64 = 0.0;
        let mut var_dvh_db1: f64 = 0.0;
        let mut var_dvh_db2: f64 = 0.0;
        let mut var_dvh_db3: f64 = 0.0;
        let mut var_dvh_db4: f64 = 0.0;
        let mut var_dvh_db5: f64 = 0.0;
        let mut var_dvh_db6: f64 = 0.0;
        let mut var_dvh_db7: f64 = 0.0;
        let mut var_pwq: f64 = 0.0;
        let mut var_qlo: f64 = 0.0;
        let mut var_qlo_dn0: f64 = 0.0;
        let mut var_qlo_dn1: f64 = 0.0;
        let mut var_qlo_dn2: f64 = 0.0;
        let mut var_qlo_dn3: f64 = 0.0;
        let mut var_qlo_dn4: f64 = 0.0;
        let mut var_qlo_dn5: f64 = 0.0;
        let mut var_qlo_dn6: f64 = 0.0;
        let mut var_qlo_dn7: f64 = 0.0;
        let mut var_qlo_dn8: f64 = 0.0;
        let mut var_qlo_dn9: f64 = 0.0;
        let mut var_qlo_db0: f64 = 0.0;
        let mut var_qlo_db1: f64 = 0.0;
        let mut var_qlo_db2: f64 = 0.0;
        let mut var_qlo_db3: f64 = 0.0;
        let mut var_qlo_db4: f64 = 0.0;
        let mut var_qlo_db5: f64 = 0.0;
        let mut var_qlo_db6: f64 = 0.0;
        let mut var_qlo_db7: f64 = 0.0;
        let mut var_qhi: f64 = 0.0;
        let mut var_qhi_dn0: f64 = 0.0;
        let mut var_qhi_dn1: f64 = 0.0;
        let mut var_qhi_dn2: f64 = 0.0;
        let mut var_qhi_dn3: f64 = 0.0;
        let mut var_qhi_dn4: f64 = 0.0;
        let mut var_qhi_dn5: f64 = 0.0;
        let mut var_qhi_dn6: f64 = 0.0;
        let mut var_qhi_dn7: f64 = 0.0;
        let mut var_qhi_dn8: f64 = 0.0;
        let mut var_qhi_dn9: f64 = 0.0;
        let mut var_qhi_db0: f64 = 0.0;
        let mut var_qhi_db1: f64 = 0.0;
        let mut var_qhi_db2: f64 = 0.0;
        let mut var_qhi_db3: f64 = 0.0;
        let mut var_qhi_db4: f64 = 0.0;
        let mut var_qhi_db5: f64 = 0.0;
        let mut var_qhi_db6: f64 = 0.0;
        let mut var_qhi_db7: f64 = 0.0;
        let mut var_ttype: f64 = 0.0;
        let mut var_tdev: f64 = 0.0;
        let mut var_tdev_dn0: f64 = 0.0;
        let mut var_tdev_dn1: f64 = 0.0;
        let mut var_tdev_dn2: f64 = 0.0;
        let mut var_tdev_dn3: f64 = 0.0;
        let mut var_tdev_dn4: f64 = 0.0;
        let mut var_tdev_dn5: f64 = 0.0;
        let mut var_tdev_dn6: f64 = 0.0;
        let mut var_tdev_dn7: f64 = 0.0;
        let mut var_tdev_dn8: f64 = 0.0;
        let mut var_tdev_dn9: f64 = 0.0;
        let mut var_tdev_db0: f64 = 0.0;
        let mut var_tdev_db1: f64 = 0.0;
        let mut var_tdev_db2: f64 = 0.0;
        let mut var_tdev_db3: f64 = 0.0;
        let mut var_tdev_db4: f64 = 0.0;
        let mut var_tdev_db5: f64 = 0.0;
        let mut var_tdev_db6: f64 = 0.0;
        let mut var_tdev_db7: f64 = 0.0;
        let mut var_tnom: f64 = 0.0;
        let mut var_tamb: f64 = 0.0;
        let mut var_tamb_dn0: f64 = 0.0;
        let mut var_tamb_dn1: f64 = 0.0;
        let mut var_tamb_dn2: f64 = 0.0;
        let mut var_tamb_dn3: f64 = 0.0;
        let mut var_tamb_dn4: f64 = 0.0;
        let mut var_tamb_dn5: f64 = 0.0;
        let mut var_tamb_dn6: f64 = 0.0;
        let mut var_tamb_dn7: f64 = 0.0;
        let mut var_tamb_dn8: f64 = 0.0;
        let mut var_tamb_dn9: f64 = 0.0;
        let mut var_tamb_db0: f64 = 0.0;
        let mut var_tamb_db1: f64 = 0.0;
        let mut var_tamb_db2: f64 = 0.0;
        let mut var_tamb_db3: f64 = 0.0;
        let mut var_tamb_db4: f64 = 0.0;
        let mut var_tamb_db5: f64 = 0.0;
        let mut var_tamb_db6: f64 = 0.0;
        let mut var_tamb_db7: f64 = 0.0;
        let mut var_rt: f64 = 0.0;
        let mut var_rt_dn0: f64 = 0.0;
        let mut var_rt_dn1: f64 = 0.0;
        let mut var_rt_dn2: f64 = 0.0;
        let mut var_rt_dn3: f64 = 0.0;
        let mut var_rt_dn4: f64 = 0.0;
        let mut var_rt_dn5: f64 = 0.0;
        let mut var_rt_dn6: f64 = 0.0;
        let mut var_rt_dn7: f64 = 0.0;
        let mut var_rt_dn8: f64 = 0.0;
        let mut var_rt_dn9: f64 = 0.0;
        let mut var_rt_db0: f64 = 0.0;
        let mut var_rt_db1: f64 = 0.0;
        let mut var_rt_db2: f64 = 0.0;
        let mut var_rt_db3: f64 = 0.0;
        let mut var_rt_db4: f64 = 0.0;
        let mut var_rt_db5: f64 = 0.0;
        let mut var_rt_db6: f64 = 0.0;
        let mut var_rt_db7: f64 = 0.0;
        let mut var_lnrt: f64 = 0.0;
        let mut var_lnrt_dn0: f64 = 0.0;
        let mut var_lnrt_dn1: f64 = 0.0;
        let mut var_lnrt_dn2: f64 = 0.0;
        let mut var_lnrt_dn3: f64 = 0.0;
        let mut var_lnrt_dn4: f64 = 0.0;
        let mut var_lnrt_dn5: f64 = 0.0;
        let mut var_lnrt_dn6: f64 = 0.0;
        let mut var_lnrt_dn7: f64 = 0.0;
        let mut var_lnrt_dn8: f64 = 0.0;
        let mut var_lnrt_dn9: f64 = 0.0;
        let mut var_lnrt_db0: f64 = 0.0;
        let mut var_lnrt_db1: f64 = 0.0;
        let mut var_lnrt_db2: f64 = 0.0;
        let mut var_lnrt_db3: f64 = 0.0;
        let mut var_lnrt_db4: f64 = 0.0;
        let mut var_lnrt_db5: f64 = 0.0;
        let mut var_lnrt_db6: f64 = 0.0;
        let mut var_lnrt_db7: f64 = 0.0;
        let mut var_vt: f64 = 0.0;
        let mut var_vt_dn0: f64 = 0.0;
        let mut var_vt_dn1: f64 = 0.0;
        let mut var_vt_dn2: f64 = 0.0;
        let mut var_vt_dn3: f64 = 0.0;
        let mut var_vt_dn4: f64 = 0.0;
        let mut var_vt_dn5: f64 = 0.0;
        let mut var_vt_dn6: f64 = 0.0;
        let mut var_vt_dn7: f64 = 0.0;
        let mut var_vt_dn8: f64 = 0.0;
        let mut var_vt_dn9: f64 = 0.0;
        let mut var_vt_db0: f64 = 0.0;
        let mut var_vt_db1: f64 = 0.0;
        let mut var_vt_db2: f64 = 0.0;
        let mut var_vt_db3: f64 = 0.0;
        let mut var_vt_db4: f64 = 0.0;
        let mut var_vt_db5: f64 = 0.0;
        let mut var_vt_db6: f64 = 0.0;
        let mut var_vt_db7: f64 = 0.0;
        let mut var_bf_t: f64 = 0.0;
        let mut var_bf_t_dn0: f64 = 0.0;
        let mut var_bf_t_dn1: f64 = 0.0;
        let mut var_bf_t_dn2: f64 = 0.0;
        let mut var_bf_t_dn3: f64 = 0.0;
        let mut var_bf_t_dn4: f64 = 0.0;
        let mut var_bf_t_dn5: f64 = 0.0;
        let mut var_bf_t_dn6: f64 = 0.0;
        let mut var_bf_t_dn7: f64 = 0.0;
        let mut var_bf_t_dn8: f64 = 0.0;
        let mut var_bf_t_dn9: f64 = 0.0;
        let mut var_bf_t_db0: f64 = 0.0;
        let mut var_bf_t_db1: f64 = 0.0;
        let mut var_bf_t_db2: f64 = 0.0;
        let mut var_bf_t_db3: f64 = 0.0;
        let mut var_bf_t_db4: f64 = 0.0;
        let mut var_bf_t_db5: f64 = 0.0;
        let mut var_bf_t_db6: f64 = 0.0;
        let mut var_bf_t_db7: f64 = 0.0;
        let mut var_br_t: f64 = 0.0;
        let mut var_br_t_dn0: f64 = 0.0;
        let mut var_br_t_dn1: f64 = 0.0;
        let mut var_br_t_dn2: f64 = 0.0;
        let mut var_br_t_dn3: f64 = 0.0;
        let mut var_br_t_dn4: f64 = 0.0;
        let mut var_br_t_dn5: f64 = 0.0;
        let mut var_br_t_dn6: f64 = 0.0;
        let mut var_br_t_dn7: f64 = 0.0;
        let mut var_br_t_dn8: f64 = 0.0;
        let mut var_br_t_dn9: f64 = 0.0;
        let mut var_br_t_db0: f64 = 0.0;
        let mut var_br_t_db1: f64 = 0.0;
        let mut var_br_t_db2: f64 = 0.0;
        let mut var_br_t_db3: f64 = 0.0;
        let mut var_br_t_db4: f64 = 0.0;
        let mut var_br_t_db5: f64 = 0.0;
        let mut var_br_t_db6: f64 = 0.0;
        let mut var_br_t_db7: f64 = 0.0;
        let mut var_tbeta: f64 = 0.0;
        let mut var_tbeta_dn0: f64 = 0.0;
        let mut var_tbeta_dn1: f64 = 0.0;
        let mut var_tbeta_dn2: f64 = 0.0;
        let mut var_tbeta_dn3: f64 = 0.0;
        let mut var_tbeta_dn4: f64 = 0.0;
        let mut var_tbeta_dn5: f64 = 0.0;
        let mut var_tbeta_dn6: f64 = 0.0;
        let mut var_tbeta_dn7: f64 = 0.0;
        let mut var_tbeta_dn8: f64 = 0.0;
        let mut var_tbeta_dn9: f64 = 0.0;
        let mut var_tbeta_db0: f64 = 0.0;
        let mut var_tbeta_db1: f64 = 0.0;
        let mut var_tbeta_db2: f64 = 0.0;
        let mut var_tbeta_db3: f64 = 0.0;
        let mut var_tbeta_db4: f64 = 0.0;
        let mut var_tbeta_db5: f64 = 0.0;
        let mut var_tbeta_db6: f64 = 0.0;
        let mut var_tbeta_db7: f64 = 0.0;
        let mut var_is_t: f64 = 0.0;
        let mut var_is_t_dn0: f64 = 0.0;
        let mut var_is_t_dn1: f64 = 0.0;
        let mut var_is_t_dn2: f64 = 0.0;
        let mut var_is_t_dn3: f64 = 0.0;
        let mut var_is_t_dn4: f64 = 0.0;
        let mut var_is_t_dn5: f64 = 0.0;
        let mut var_is_t_dn6: f64 = 0.0;
        let mut var_is_t_dn7: f64 = 0.0;
        let mut var_is_t_dn8: f64 = 0.0;
        let mut var_is_t_dn9: f64 = 0.0;
        let mut var_is_t_db0: f64 = 0.0;
        let mut var_is_t_db1: f64 = 0.0;
        let mut var_is_t_db2: f64 = 0.0;
        let mut var_is_t_db3: f64 = 0.0;
        let mut var_is_t_db4: f64 = 0.0;
        let mut var_is_t_db5: f64 = 0.0;
        let mut var_is_t_db6: f64 = 0.0;
        let mut var_is_t_db7: f64 = 0.0;
        let mut var_ise_t: f64 = 0.0;
        let mut var_ise_t_dn0: f64 = 0.0;
        let mut var_ise_t_dn1: f64 = 0.0;
        let mut var_ise_t_dn2: f64 = 0.0;
        let mut var_ise_t_dn3: f64 = 0.0;
        let mut var_ise_t_dn4: f64 = 0.0;
        let mut var_ise_t_dn5: f64 = 0.0;
        let mut var_ise_t_dn6: f64 = 0.0;
        let mut var_ise_t_dn7: f64 = 0.0;
        let mut var_ise_t_dn8: f64 = 0.0;
        let mut var_ise_t_dn9: f64 = 0.0;
        let mut var_ise_t_db0: f64 = 0.0;
        let mut var_ise_t_db1: f64 = 0.0;
        let mut var_ise_t_db2: f64 = 0.0;
        let mut var_ise_t_db3: f64 = 0.0;
        let mut var_ise_t_db4: f64 = 0.0;
        let mut var_ise_t_db5: f64 = 0.0;
        let mut var_ise_t_db6: f64 = 0.0;
        let mut var_ise_t_db7: f64 = 0.0;
        let mut var_isc_t: f64 = 0.0;
        let mut var_isc_t_dn0: f64 = 0.0;
        let mut var_isc_t_dn1: f64 = 0.0;
        let mut var_isc_t_dn2: f64 = 0.0;
        let mut var_isc_t_dn3: f64 = 0.0;
        let mut var_isc_t_dn4: f64 = 0.0;
        let mut var_isc_t_dn5: f64 = 0.0;
        let mut var_isc_t_dn6: f64 = 0.0;
        let mut var_isc_t_dn7: f64 = 0.0;
        let mut var_isc_t_dn8: f64 = 0.0;
        let mut var_isc_t_dn9: f64 = 0.0;
        let mut var_isc_t_db0: f64 = 0.0;
        let mut var_isc_t_db1: f64 = 0.0;
        let mut var_isc_t_db2: f64 = 0.0;
        let mut var_isc_t_db3: f64 = 0.0;
        let mut var_isc_t_db4: f64 = 0.0;
        let mut var_isc_t_db5: f64 = 0.0;
        let mut var_isc_t_db6: f64 = 0.0;
        let mut var_isc_t_db7: f64 = 0.0;
        let mut var_cje_t: f64 = 0.0;
        let mut var_cje_t_dn0: f64 = 0.0;
        let mut var_cje_t_dn1: f64 = 0.0;
        let mut var_cje_t_dn2: f64 = 0.0;
        let mut var_cje_t_dn3: f64 = 0.0;
        let mut var_cje_t_dn4: f64 = 0.0;
        let mut var_cje_t_dn5: f64 = 0.0;
        let mut var_cje_t_dn6: f64 = 0.0;
        let mut var_cje_t_dn7: f64 = 0.0;
        let mut var_cje_t_dn8: f64 = 0.0;
        let mut var_cje_t_dn9: f64 = 0.0;
        let mut var_cje_t_db0: f64 = 0.0;
        let mut var_cje_t_db1: f64 = 0.0;
        let mut var_cje_t_db2: f64 = 0.0;
        let mut var_cje_t_db3: f64 = 0.0;
        let mut var_cje_t_db4: f64 = 0.0;
        let mut var_cje_t_db5: f64 = 0.0;
        let mut var_cje_t_db6: f64 = 0.0;
        let mut var_cje_t_db7: f64 = 0.0;
        let mut var_cjc_t: f64 = 0.0;
        let mut var_cjc_t_dn0: f64 = 0.0;
        let mut var_cjc_t_dn1: f64 = 0.0;
        let mut var_cjc_t_dn2: f64 = 0.0;
        let mut var_cjc_t_dn3: f64 = 0.0;
        let mut var_cjc_t_dn4: f64 = 0.0;
        let mut var_cjc_t_dn5: f64 = 0.0;
        let mut var_cjc_t_dn6: f64 = 0.0;
        let mut var_cjc_t_dn7: f64 = 0.0;
        let mut var_cjc_t_dn8: f64 = 0.0;
        let mut var_cjc_t_dn9: f64 = 0.0;
        let mut var_cjc_t_db0: f64 = 0.0;
        let mut var_cjc_t_db1: f64 = 0.0;
        let mut var_cjc_t_db2: f64 = 0.0;
        let mut var_cjc_t_db3: f64 = 0.0;
        let mut var_cjc_t_db4: f64 = 0.0;
        let mut var_cjc_t_db5: f64 = 0.0;
        let mut var_cjc_t_db6: f64 = 0.0;
        let mut var_cjc_t_db7: f64 = 0.0;
        let mut var_cjs_t: f64 = 0.0;
        let mut var_cjs_t_dn0: f64 = 0.0;
        let mut var_cjs_t_dn1: f64 = 0.0;
        let mut var_cjs_t_dn2: f64 = 0.0;
        let mut var_cjs_t_dn3: f64 = 0.0;
        let mut var_cjs_t_dn4: f64 = 0.0;
        let mut var_cjs_t_dn5: f64 = 0.0;
        let mut var_cjs_t_dn6: f64 = 0.0;
        let mut var_cjs_t_dn7: f64 = 0.0;
        let mut var_cjs_t_dn8: f64 = 0.0;
        let mut var_cjs_t_dn9: f64 = 0.0;
        let mut var_cjs_t_db0: f64 = 0.0;
        let mut var_cjs_t_db1: f64 = 0.0;
        let mut var_cjs_t_db2: f64 = 0.0;
        let mut var_cjs_t_db3: f64 = 0.0;
        let mut var_cjs_t_db4: f64 = 0.0;
        let mut var_cjs_t_db5: f64 = 0.0;
        let mut var_cjs_t_db6: f64 = 0.0;
        let mut var_cjs_t_db7: f64 = 0.0;
        let mut var_vje_t: f64 = 0.0;
        let mut var_vje_t_dn0: f64 = 0.0;
        let mut var_vje_t_dn1: f64 = 0.0;
        let mut var_vje_t_dn2: f64 = 0.0;
        let mut var_vje_t_dn3: f64 = 0.0;
        let mut var_vje_t_dn4: f64 = 0.0;
        let mut var_vje_t_dn5: f64 = 0.0;
        let mut var_vje_t_dn6: f64 = 0.0;
        let mut var_vje_t_dn7: f64 = 0.0;
        let mut var_vje_t_dn8: f64 = 0.0;
        let mut var_vje_t_dn9: f64 = 0.0;
        let mut var_vje_t_db0: f64 = 0.0;
        let mut var_vje_t_db1: f64 = 0.0;
        let mut var_vje_t_db2: f64 = 0.0;
        let mut var_vje_t_db3: f64 = 0.0;
        let mut var_vje_t_db4: f64 = 0.0;
        let mut var_vje_t_db5: f64 = 0.0;
        let mut var_vje_t_db6: f64 = 0.0;
        let mut var_vje_t_db7: f64 = 0.0;
        let mut var_vjc_t: f64 = 0.0;
        let mut var_vjc_t_dn0: f64 = 0.0;
        let mut var_vjc_t_dn1: f64 = 0.0;
        let mut var_vjc_t_dn2: f64 = 0.0;
        let mut var_vjc_t_dn3: f64 = 0.0;
        let mut var_vjc_t_dn4: f64 = 0.0;
        let mut var_vjc_t_dn5: f64 = 0.0;
        let mut var_vjc_t_dn6: f64 = 0.0;
        let mut var_vjc_t_dn7: f64 = 0.0;
        let mut var_vjc_t_dn8: f64 = 0.0;
        let mut var_vjc_t_dn9: f64 = 0.0;
        let mut var_vjc_t_db0: f64 = 0.0;
        let mut var_vjc_t_db1: f64 = 0.0;
        let mut var_vjc_t_db2: f64 = 0.0;
        let mut var_vjc_t_db3: f64 = 0.0;
        let mut var_vjc_t_db4: f64 = 0.0;
        let mut var_vjc_t_db5: f64 = 0.0;
        let mut var_vjc_t_db6: f64 = 0.0;
        let mut var_vjc_t_db7: f64 = 0.0;
        let mut var_vjs_t: f64 = 0.0;
        let mut var_vjs_t_dn0: f64 = 0.0;
        let mut var_vjs_t_dn1: f64 = 0.0;
        let mut var_vjs_t_dn2: f64 = 0.0;
        let mut var_vjs_t_dn3: f64 = 0.0;
        let mut var_vjs_t_dn4: f64 = 0.0;
        let mut var_vjs_t_dn5: f64 = 0.0;
        let mut var_vjs_t_dn6: f64 = 0.0;
        let mut var_vjs_t_dn7: f64 = 0.0;
        let mut var_vjs_t_dn8: f64 = 0.0;
        let mut var_vjs_t_dn9: f64 = 0.0;
        let mut var_vjs_t_db0: f64 = 0.0;
        let mut var_vjs_t_db1: f64 = 0.0;
        let mut var_vjs_t_db2: f64 = 0.0;
        let mut var_vjs_t_db3: f64 = 0.0;
        let mut var_vjs_t_db4: f64 = 0.0;
        let mut var_vjs_t_db5: f64 = 0.0;
        let mut var_vjs_t_db6: f64 = 0.0;
        let mut var_vjs_t_db7: f64 = 0.0;
        let mut var_ijbv_t: f64 = 0.0;
        let mut var_ijbv_t_dn0: f64 = 0.0;
        let mut var_ijbv_t_dn1: f64 = 0.0;
        let mut var_ijbv_t_dn2: f64 = 0.0;
        let mut var_ijbv_t_dn3: f64 = 0.0;
        let mut var_ijbv_t_dn4: f64 = 0.0;
        let mut var_ijbv_t_dn5: f64 = 0.0;
        let mut var_ijbv_t_dn6: f64 = 0.0;
        let mut var_ijbv_t_dn7: f64 = 0.0;
        let mut var_ijbv_t_dn8: f64 = 0.0;
        let mut var_ijbv_t_dn9: f64 = 0.0;
        let mut var_ijbv_t_db0: f64 = 0.0;
        let mut var_ijbv_t_db1: f64 = 0.0;
        let mut var_ijbv_t_db2: f64 = 0.0;
        let mut var_ijbv_t_db3: f64 = 0.0;
        let mut var_ijbv_t_db4: f64 = 0.0;
        let mut var_ijbv_t_db5: f64 = 0.0;
        let mut var_ijbv_t_db6: f64 = 0.0;
        let mut var_ijbv_t_db7: f64 = 0.0;
        let mut var_ijbvc_t: f64 = 0.0;
        let mut var_ijbvc_t_dn0: f64 = 0.0;
        let mut var_ijbvc_t_dn1: f64 = 0.0;
        let mut var_ijbvc_t_dn2: f64 = 0.0;
        let mut var_ijbvc_t_dn3: f64 = 0.0;
        let mut var_ijbvc_t_dn4: f64 = 0.0;
        let mut var_ijbvc_t_dn5: f64 = 0.0;
        let mut var_ijbvc_t_dn6: f64 = 0.0;
        let mut var_ijbvc_t_dn7: f64 = 0.0;
        let mut var_ijbvc_t_dn8: f64 = 0.0;
        let mut var_ijbvc_t_dn9: f64 = 0.0;
        let mut var_ijbvc_t_db0: f64 = 0.0;
        let mut var_ijbvc_t_db1: f64 = 0.0;
        let mut var_ijbvc_t_db2: f64 = 0.0;
        let mut var_ijbvc_t_db3: f64 = 0.0;
        let mut var_ijbvc_t_db4: f64 = 0.0;
        let mut var_ijbvc_t_db5: f64 = 0.0;
        let mut var_ijbvc_t_db6: f64 = 0.0;
        let mut var_ijbvc_t_db7: f64 = 0.0;
        let mut var_bvr_t: f64 = 0.0;
        let mut var_bvr_t_dn0: f64 = 0.0;
        let mut var_bvr_t_dn1: f64 = 0.0;
        let mut var_bvr_t_dn2: f64 = 0.0;
        let mut var_bvr_t_dn3: f64 = 0.0;
        let mut var_bvr_t_dn4: f64 = 0.0;
        let mut var_bvr_t_dn5: f64 = 0.0;
        let mut var_bvr_t_dn6: f64 = 0.0;
        let mut var_bvr_t_dn7: f64 = 0.0;
        let mut var_bvr_t_dn8: f64 = 0.0;
        let mut var_bvr_t_dn9: f64 = 0.0;
        let mut var_bvr_t_db0: f64 = 0.0;
        let mut var_bvr_t_db1: f64 = 0.0;
        let mut var_bvr_t_db2: f64 = 0.0;
        let mut var_bvr_t_db3: f64 = 0.0;
        let mut var_bvr_t_db4: f64 = 0.0;
        let mut var_bvr_t_db5: f64 = 0.0;
        let mut var_bvr_t_db6: f64 = 0.0;
        let mut var_bvr_t_db7: f64 = 0.0;
        let mut var_theexp_t: f64 = 0.0;
        let mut var_theexp_t_dn0: f64 = 0.0;
        let mut var_theexp_t_dn1: f64 = 0.0;
        let mut var_theexp_t_dn2: f64 = 0.0;
        let mut var_theexp_t_dn3: f64 = 0.0;
        let mut var_theexp_t_dn4: f64 = 0.0;
        let mut var_theexp_t_dn5: f64 = 0.0;
        let mut var_theexp_t_dn6: f64 = 0.0;
        let mut var_theexp_t_dn7: f64 = 0.0;
        let mut var_theexp_t_dn8: f64 = 0.0;
        let mut var_theexp_t_dn9: f64 = 0.0;
        let mut var_theexp_t_db0: f64 = 0.0;
        let mut var_theexp_t_db1: f64 = 0.0;
        let mut var_theexp_t_db2: f64 = 0.0;
        let mut var_theexp_t_db3: f64 = 0.0;
        let mut var_theexp_t_db4: f64 = 0.0;
        let mut var_theexp_t_db5: f64 = 0.0;
        let mut var_theexp_t_db6: f64 = 0.0;
        let mut var_theexp_t_db7: f64 = 0.0;
        let mut var_cje_i: f64 = 0.0;
        let mut var_cjc_i: f64 = 0.0;
        let mut var_cjs_i: f64 = 0.0;
        let mut var_ifwd: f64 = 0.0;
        let mut var_ifwd_dn0: f64 = 0.0;
        let mut var_ifwd_dn1: f64 = 0.0;
        let mut var_ifwd_dn2: f64 = 0.0;
        let mut var_ifwd_dn3: f64 = 0.0;
        let mut var_ifwd_dn4: f64 = 0.0;
        let mut var_ifwd_dn5: f64 = 0.0;
        let mut var_ifwd_dn6: f64 = 0.0;
        let mut var_ifwd_dn7: f64 = 0.0;
        let mut var_ifwd_dn8: f64 = 0.0;
        let mut var_ifwd_dn9: f64 = 0.0;
        let mut var_ifwd_db0: f64 = 0.0;
        let mut var_ifwd_db1: f64 = 0.0;
        let mut var_ifwd_db2: f64 = 0.0;
        let mut var_ifwd_db3: f64 = 0.0;
        let mut var_ifwd_db4: f64 = 0.0;
        let mut var_ifwd_db5: f64 = 0.0;
        let mut var_ifwd_db6: f64 = 0.0;
        let mut var_ifwd_db7: f64 = 0.0;
        let mut var_ibe2: f64 = 0.0;
        let mut var_ibe2_dn0: f64 = 0.0;
        let mut var_ibe2_dn1: f64 = 0.0;
        let mut var_ibe2_dn2: f64 = 0.0;
        let mut var_ibe2_dn3: f64 = 0.0;
        let mut var_ibe2_dn4: f64 = 0.0;
        let mut var_ibe2_dn5: f64 = 0.0;
        let mut var_ibe2_dn6: f64 = 0.0;
        let mut var_ibe2_dn7: f64 = 0.0;
        let mut var_ibe2_dn8: f64 = 0.0;
        let mut var_ibe2_dn9: f64 = 0.0;
        let mut var_ibe2_db0: f64 = 0.0;
        let mut var_ibe2_db1: f64 = 0.0;
        let mut var_ibe2_db2: f64 = 0.0;
        let mut var_ibe2_db3: f64 = 0.0;
        let mut var_ibe2_db4: f64 = 0.0;
        let mut var_ibe2_db5: f64 = 0.0;
        let mut var_ibe2_db6: f64 = 0.0;
        let mut var_ibe2_db7: f64 = 0.0;
        let mut var_ibe: f64 = 0.0;
        let mut var_ibe_dn0: f64 = 0.0;
        let mut var_ibe_dn1: f64 = 0.0;
        let mut var_ibe_dn2: f64 = 0.0;
        let mut var_ibe_dn3: f64 = 0.0;
        let mut var_ibe_dn4: f64 = 0.0;
        let mut var_ibe_dn5: f64 = 0.0;
        let mut var_ibe_dn6: f64 = 0.0;
        let mut var_ibe_dn7: f64 = 0.0;
        let mut var_ibe_dn8: f64 = 0.0;
        let mut var_ibe_dn9: f64 = 0.0;
        let mut var_ibe_db0: f64 = 0.0;
        let mut var_ibe_db1: f64 = 0.0;
        let mut var_ibe_db2: f64 = 0.0;
        let mut var_ibe_db3: f64 = 0.0;
        let mut var_ibe_db4: f64 = 0.0;
        let mut var_ibe_db5: f64 = 0.0;
        let mut var_ibe_db6: f64 = 0.0;
        let mut var_ibe_db7: f64 = 0.0;
        let mut var_ibwd: f64 = 0.0;
        let mut var_ibwd_dn0: f64 = 0.0;
        let mut var_ibwd_dn1: f64 = 0.0;
        let mut var_ibwd_dn2: f64 = 0.0;
        let mut var_ibwd_dn3: f64 = 0.0;
        let mut var_ibwd_dn4: f64 = 0.0;
        let mut var_ibwd_dn5: f64 = 0.0;
        let mut var_ibwd_dn6: f64 = 0.0;
        let mut var_ibwd_dn7: f64 = 0.0;
        let mut var_ibwd_dn8: f64 = 0.0;
        let mut var_ibwd_dn9: f64 = 0.0;
        let mut var_ibwd_db0: f64 = 0.0;
        let mut var_ibwd_db1: f64 = 0.0;
        let mut var_ibwd_db2: f64 = 0.0;
        let mut var_ibwd_db3: f64 = 0.0;
        let mut var_ibwd_db4: f64 = 0.0;
        let mut var_ibwd_db5: f64 = 0.0;
        let mut var_ibwd_db6: f64 = 0.0;
        let mut var_ibwd_db7: f64 = 0.0;
        let mut var_ibc2: f64 = 0.0;
        let mut var_ibc2_dn0: f64 = 0.0;
        let mut var_ibc2_dn1: f64 = 0.0;
        let mut var_ibc2_dn2: f64 = 0.0;
        let mut var_ibc2_dn3: f64 = 0.0;
        let mut var_ibc2_dn4: f64 = 0.0;
        let mut var_ibc2_dn5: f64 = 0.0;
        let mut var_ibc2_dn6: f64 = 0.0;
        let mut var_ibc2_dn7: f64 = 0.0;
        let mut var_ibc2_dn8: f64 = 0.0;
        let mut var_ibc2_dn9: f64 = 0.0;
        let mut var_ibc2_db0: f64 = 0.0;
        let mut var_ibc2_db1: f64 = 0.0;
        let mut var_ibc2_db2: f64 = 0.0;
        let mut var_ibc2_db3: f64 = 0.0;
        let mut var_ibc2_db4: f64 = 0.0;
        let mut var_ibc2_db5: f64 = 0.0;
        let mut var_ibc2_db6: f64 = 0.0;
        let mut var_ibc2_db7: f64 = 0.0;
        let mut var_ibc: f64 = 0.0;
        let mut var_ibc_dn0: f64 = 0.0;
        let mut var_ibc_dn1: f64 = 0.0;
        let mut var_ibc_dn2: f64 = 0.0;
        let mut var_ibc_dn3: f64 = 0.0;
        let mut var_ibc_dn4: f64 = 0.0;
        let mut var_ibc_dn5: f64 = 0.0;
        let mut var_ibc_dn6: f64 = 0.0;
        let mut var_ibc_dn7: f64 = 0.0;
        let mut var_ibc_dn8: f64 = 0.0;
        let mut var_ibc_dn9: f64 = 0.0;
        let mut var_ibc_db0: f64 = 0.0;
        let mut var_ibc_db1: f64 = 0.0;
        let mut var_ibc_db2: f64 = 0.0;
        let mut var_ibc_db3: f64 = 0.0;
        let mut var_ibc_db4: f64 = 0.0;
        let mut var_ibc_db5: f64 = 0.0;
        let mut var_ibc_db6: f64 = 0.0;
        let mut var_ibc_db7: f64 = 0.0;
        let mut var_ikq1: f64 = 0.0;
        let mut var_ikq1_dn0: f64 = 0.0;
        let mut var_ikq1_dn1: f64 = 0.0;
        let mut var_ikq1_dn2: f64 = 0.0;
        let mut var_ikq1_dn3: f64 = 0.0;
        let mut var_ikq1_dn4: f64 = 0.0;
        let mut var_ikq1_dn5: f64 = 0.0;
        let mut var_ikq1_dn6: f64 = 0.0;
        let mut var_ikq1_dn7: f64 = 0.0;
        let mut var_ikq1_dn8: f64 = 0.0;
        let mut var_ikq1_dn9: f64 = 0.0;
        let mut var_ikq1_db0: f64 = 0.0;
        let mut var_ikq1_db1: f64 = 0.0;
        let mut var_ikq1_db2: f64 = 0.0;
        let mut var_ikq1_db3: f64 = 0.0;
        let mut var_ikq1_db4: f64 = 0.0;
        let mut var_ikq1_db5: f64 = 0.0;
        let mut var_ikq1_db6: f64 = 0.0;
        let mut var_ikq1_db7: f64 = 0.0;
        let mut var_kq2: f64 = 0.0;
        let mut var_kq2_dn0: f64 = 0.0;
        let mut var_kq2_dn1: f64 = 0.0;
        let mut var_kq2_dn2: f64 = 0.0;
        let mut var_kq2_dn3: f64 = 0.0;
        let mut var_kq2_dn4: f64 = 0.0;
        let mut var_kq2_dn5: f64 = 0.0;
        let mut var_kq2_dn6: f64 = 0.0;
        let mut var_kq2_dn7: f64 = 0.0;
        let mut var_kq2_dn8: f64 = 0.0;
        let mut var_kq2_dn9: f64 = 0.0;
        let mut var_kq2_db0: f64 = 0.0;
        let mut var_kq2_db1: f64 = 0.0;
        let mut var_kq2_db2: f64 = 0.0;
        let mut var_kq2_db3: f64 = 0.0;
        let mut var_kq2_db4: f64 = 0.0;
        let mut var_kq2_db5: f64 = 0.0;
        let mut var_kq2_db6: f64 = 0.0;
        let mut var_kq2_db7: f64 = 0.0;
        let mut var_ikqb: f64 = 0.0;
        let mut var_ikqb_dn0: f64 = 0.0;
        let mut var_ikqb_dn1: f64 = 0.0;
        let mut var_ikqb_dn2: f64 = 0.0;
        let mut var_ikqb_dn3: f64 = 0.0;
        let mut var_ikqb_dn4: f64 = 0.0;
        let mut var_ikqb_dn5: f64 = 0.0;
        let mut var_ikqb_dn6: f64 = 0.0;
        let mut var_ikqb_dn7: f64 = 0.0;
        let mut var_ikqb_dn8: f64 = 0.0;
        let mut var_ikqb_dn9: f64 = 0.0;
        let mut var_ikqb_db0: f64 = 0.0;
        let mut var_ikqb_db1: f64 = 0.0;
        let mut var_ikqb_db2: f64 = 0.0;
        let mut var_ikqb_db3: f64 = 0.0;
        let mut var_ikqb_db4: f64 = 0.0;
        let mut var_ikqb_db5: f64 = 0.0;
        let mut var_ikqb_db6: f64 = 0.0;
        let mut var_ikqb_db7: f64 = 0.0;
        let mut var_itzf: f64 = 0.0;
        let mut var_itzf_dn0: f64 = 0.0;
        let mut var_itzf_dn1: f64 = 0.0;
        let mut var_itzf_dn2: f64 = 0.0;
        let mut var_itzf_dn3: f64 = 0.0;
        let mut var_itzf_dn4: f64 = 0.0;
        let mut var_itzf_dn5: f64 = 0.0;
        let mut var_itzf_dn6: f64 = 0.0;
        let mut var_itzf_dn7: f64 = 0.0;
        let mut var_itzf_dn8: f64 = 0.0;
        let mut var_itzf_dn9: f64 = 0.0;
        let mut var_itzf_db0: f64 = 0.0;
        let mut var_itzf_db1: f64 = 0.0;
        let mut var_itzf_db2: f64 = 0.0;
        let mut var_itzf_db3: f64 = 0.0;
        let mut var_itzf_db4: f64 = 0.0;
        let mut var_itzf_db5: f64 = 0.0;
        let mut var_itzf_db6: f64 = 0.0;
        let mut var_itzf_db7: f64 = 0.0;
        let mut var_itr: f64 = 0.0;
        let mut var_itr_dn0: f64 = 0.0;
        let mut var_itr_dn1: f64 = 0.0;
        let mut var_itr_dn2: f64 = 0.0;
        let mut var_itr_dn3: f64 = 0.0;
        let mut var_itr_dn4: f64 = 0.0;
        let mut var_itr_dn5: f64 = 0.0;
        let mut var_itr_dn6: f64 = 0.0;
        let mut var_itr_dn7: f64 = 0.0;
        let mut var_itr_dn8: f64 = 0.0;
        let mut var_itr_dn9: f64 = 0.0;
        let mut var_itr_db0: f64 = 0.0;
        let mut var_itr_db1: f64 = 0.0;
        let mut var_itr_db2: f64 = 0.0;
        let mut var_itr_db3: f64 = 0.0;
        let mut var_itr_db4: f64 = 0.0;
        let mut var_itr_db5: f64 = 0.0;
        let mut var_itr_db6: f64 = 0.0;
        let mut var_itr_db7: f64 = 0.0;
        let mut var_itzf_f: f64 = 0.0;
        let mut var_itzf_f_dn0: f64 = 0.0;
        let mut var_itzf_f_dn1: f64 = 0.0;
        let mut var_itzf_f_dn2: f64 = 0.0;
        let mut var_itzf_f_dn3: f64 = 0.0;
        let mut var_itzf_f_dn4: f64 = 0.0;
        let mut var_itzf_f_dn5: f64 = 0.0;
        let mut var_itzf_f_dn6: f64 = 0.0;
        let mut var_itzf_f_dn7: f64 = 0.0;
        let mut var_itzf_f_dn8: f64 = 0.0;
        let mut var_itzf_f_dn9: f64 = 0.0;
        let mut var_itzf_f_db0: f64 = 0.0;
        let mut var_itzf_f_db1: f64 = 0.0;
        let mut var_itzf_f_db2: f64 = 0.0;
        let mut var_itzf_f_db3: f64 = 0.0;
        let mut var_itzf_f_db4: f64 = 0.0;
        let mut var_itzf_f_db5: f64 = 0.0;
        let mut var_itzf_f_db6: f64 = 0.0;
        let mut var_itzf_f_db7: f64 = 0.0;
        let mut var_itrev: f64 = 0.0;
        let mut var_itrev_dn0: f64 = 0.0;
        let mut var_itrev_dn1: f64 = 0.0;
        let mut var_itrev_dn2: f64 = 0.0;
        let mut var_itrev_dn3: f64 = 0.0;
        let mut var_itrev_dn4: f64 = 0.0;
        let mut var_itrev_dn5: f64 = 0.0;
        let mut var_itrev_dn6: f64 = 0.0;
        let mut var_itrev_dn7: f64 = 0.0;
        let mut var_itrev_dn8: f64 = 0.0;
        let mut var_itrev_dn9: f64 = 0.0;
        let mut var_itrev_db0: f64 = 0.0;
        let mut var_itrev_db1: f64 = 0.0;
        let mut var_itrev_db2: f64 = 0.0;
        let mut var_itrev_db3: f64 = 0.0;
        let mut var_itrev_db4: f64 = 0.0;
        let mut var_itrev_db5: f64 = 0.0;
        let mut var_itrev_db6: f64 = 0.0;
        let mut var_itrev_db7: f64 = 0.0;
        let mut var_re_nom: f64 = 0.0;
        let mut var_rc_nom: f64 = 0.0;
        let mut var_rb_nom: f64 = 0.0;
        let mut var_rb: f64 = 0.0;
        let mut var_rb_dn0: f64 = 0.0;
        let mut var_rb_dn1: f64 = 0.0;
        let mut var_rb_dn2: f64 = 0.0;
        let mut var_rb_dn3: f64 = 0.0;
        let mut var_rb_dn4: f64 = 0.0;
        let mut var_rb_dn5: f64 = 0.0;
        let mut var_rb_dn6: f64 = 0.0;
        let mut var_rb_dn7: f64 = 0.0;
        let mut var_rb_dn8: f64 = 0.0;
        let mut var_rb_dn9: f64 = 0.0;
        let mut var_rb_db0: f64 = 0.0;
        let mut var_rb_db1: f64 = 0.0;
        let mut var_rb_db2: f64 = 0.0;
        let mut var_rb_db3: f64 = 0.0;
        let mut var_rb_db4: f64 = 0.0;
        let mut var_rb_db5: f64 = 0.0;
        let mut var_rb_db6: f64 = 0.0;
        let mut var_rb_db7: f64 = 0.0;
        let mut var_rc: f64 = 0.0;
        let mut var_rc_dn0: f64 = 0.0;
        let mut var_rc_dn1: f64 = 0.0;
        let mut var_rc_dn2: f64 = 0.0;
        let mut var_rc_dn3: f64 = 0.0;
        let mut var_rc_dn4: f64 = 0.0;
        let mut var_rc_dn5: f64 = 0.0;
        let mut var_rc_dn6: f64 = 0.0;
        let mut var_rc_dn7: f64 = 0.0;
        let mut var_rc_dn8: f64 = 0.0;
        let mut var_rc_dn9: f64 = 0.0;
        let mut var_rc_db0: f64 = 0.0;
        let mut var_rc_db1: f64 = 0.0;
        let mut var_rc_db2: f64 = 0.0;
        let mut var_rc_db3: f64 = 0.0;
        let mut var_rc_db4: f64 = 0.0;
        let mut var_rc_db5: f64 = 0.0;
        let mut var_rc_db6: f64 = 0.0;
        let mut var_rc_db7: f64 = 0.0;
        let mut var_re: f64 = 0.0;
        let mut var_re_dn0: f64 = 0.0;
        let mut var_re_dn1: f64 = 0.0;
        let mut var_re_dn2: f64 = 0.0;
        let mut var_re_dn3: f64 = 0.0;
        let mut var_re_dn4: f64 = 0.0;
        let mut var_re_dn5: f64 = 0.0;
        let mut var_re_dn6: f64 = 0.0;
        let mut var_re_dn7: f64 = 0.0;
        let mut var_re_dn8: f64 = 0.0;
        let mut var_re_dn9: f64 = 0.0;
        let mut var_re_db0: f64 = 0.0;
        let mut var_re_db1: f64 = 0.0;
        let mut var_re_db2: f64 = 0.0;
        let mut var_re_db3: f64 = 0.0;
        let mut var_re_db4: f64 = 0.0;
        let mut var_re_db5: f64 = 0.0;
        let mut var_re_db6: f64 = 0.0;
        let mut var_re_db7: f64 = 0.0;
        let mut var_tff: f64 = 0.0;
        let mut var_tff_dn0: f64 = 0.0;
        let mut var_tff_dn1: f64 = 0.0;
        let mut var_tff_dn2: f64 = 0.0;
        let mut var_tff_dn3: f64 = 0.0;
        let mut var_tff_dn4: f64 = 0.0;
        let mut var_tff_dn5: f64 = 0.0;
        let mut var_tff_dn6: f64 = 0.0;
        let mut var_tff_dn7: f64 = 0.0;
        let mut var_tff_dn8: f64 = 0.0;
        let mut var_tff_dn9: f64 = 0.0;
        let mut var_tff_db0: f64 = 0.0;
        let mut var_tff_db1: f64 = 0.0;
        let mut var_tff_db2: f64 = 0.0;
        let mut var_tff_db3: f64 = 0.0;
        let mut var_tff_db4: f64 = 0.0;
        let mut var_tff_db5: f64 = 0.0;
        let mut var_tff_db6: f64 = 0.0;
        let mut var_tff_db7: f64 = 0.0;
        let mut var_qde: f64 = 0.0;
        let mut var_qde_dn0: f64 = 0.0;
        let mut var_qde_dn1: f64 = 0.0;
        let mut var_qde_dn2: f64 = 0.0;
        let mut var_qde_dn3: f64 = 0.0;
        let mut var_qde_dn4: f64 = 0.0;
        let mut var_qde_dn5: f64 = 0.0;
        let mut var_qde_dn6: f64 = 0.0;
        let mut var_qde_dn7: f64 = 0.0;
        let mut var_qde_dn8: f64 = 0.0;
        let mut var_qde_dn9: f64 = 0.0;
        let mut var_qde_db0: f64 = 0.0;
        let mut var_qde_db1: f64 = 0.0;
        let mut var_qde_db2: f64 = 0.0;
        let mut var_qde_db3: f64 = 0.0;
        let mut var_qde_db4: f64 = 0.0;
        let mut var_qde_db5: f64 = 0.0;
        let mut var_qde_db6: f64 = 0.0;
        let mut var_qde_db7: f64 = 0.0;
        let mut var_qdc: f64 = 0.0;
        let mut var_qdc_dn0: f64 = 0.0;
        let mut var_qdc_dn1: f64 = 0.0;
        let mut var_qdc_dn2: f64 = 0.0;
        let mut var_qdc_dn3: f64 = 0.0;
        let mut var_qdc_dn4: f64 = 0.0;
        let mut var_qdc_dn5: f64 = 0.0;
        let mut var_qdc_dn6: f64 = 0.0;
        let mut var_qdc_dn7: f64 = 0.0;
        let mut var_qdc_dn8: f64 = 0.0;
        let mut var_qdc_dn9: f64 = 0.0;
        let mut var_qdc_db0: f64 = 0.0;
        let mut var_qdc_db1: f64 = 0.0;
        let mut var_qdc_db2: f64 = 0.0;
        let mut var_qdc_db3: f64 = 0.0;
        let mut var_qdc_db4: f64 = 0.0;
        let mut var_qdc_db5: f64 = 0.0;
        let mut var_qdc_db6: f64 = 0.0;
        let mut var_qdc_db7: f64 = 0.0;
        let mut var_qjs: f64 = 0.0;
        let mut var_qjs_dn0: f64 = 0.0;
        let mut var_qjs_dn1: f64 = 0.0;
        let mut var_qjs_dn2: f64 = 0.0;
        let mut var_qjs_dn3: f64 = 0.0;
        let mut var_qjs_dn4: f64 = 0.0;
        let mut var_qjs_dn5: f64 = 0.0;
        let mut var_qjs_dn6: f64 = 0.0;
        let mut var_qjs_dn7: f64 = 0.0;
        let mut var_qjs_dn8: f64 = 0.0;
        let mut var_qjs_dn9: f64 = 0.0;
        let mut var_qjs_db0: f64 = 0.0;
        let mut var_qjs_db1: f64 = 0.0;
        let mut var_qjs_db2: f64 = 0.0;
        let mut var_qjs_db3: f64 = 0.0;
        let mut var_qjs_db4: f64 = 0.0;
        let mut var_qjs_db5: f64 = 0.0;
        let mut var_qjs_db6: f64 = 0.0;
        let mut var_qjs_db7: f64 = 0.0;
        let mut var_qje: f64 = 0.0;
        let mut var_qje_dn0: f64 = 0.0;
        let mut var_qje_dn1: f64 = 0.0;
        let mut var_qje_dn2: f64 = 0.0;
        let mut var_qje_dn3: f64 = 0.0;
        let mut var_qje_dn4: f64 = 0.0;
        let mut var_qje_dn5: f64 = 0.0;
        let mut var_qje_dn6: f64 = 0.0;
        let mut var_qje_dn7: f64 = 0.0;
        let mut var_qje_dn8: f64 = 0.0;
        let mut var_qje_dn9: f64 = 0.0;
        let mut var_qje_db0: f64 = 0.0;
        let mut var_qje_db1: f64 = 0.0;
        let mut var_qje_db2: f64 = 0.0;
        let mut var_qje_db3: f64 = 0.0;
        let mut var_qje_db4: f64 = 0.0;
        let mut var_qje_db5: f64 = 0.0;
        let mut var_qje_db6: f64 = 0.0;
        let mut var_qje_db7: f64 = 0.0;
        let mut var_qjcx: f64 = 0.0;
        let mut var_qjcx_dn0: f64 = 0.0;
        let mut var_qjcx_dn1: f64 = 0.0;
        let mut var_qjcx_dn2: f64 = 0.0;
        let mut var_qjcx_dn3: f64 = 0.0;
        let mut var_qjcx_dn4: f64 = 0.0;
        let mut var_qjcx_dn5: f64 = 0.0;
        let mut var_qjcx_dn6: f64 = 0.0;
        let mut var_qjcx_dn7: f64 = 0.0;
        let mut var_qjcx_dn8: f64 = 0.0;
        let mut var_qjcx_dn9: f64 = 0.0;
        let mut var_qjcx_db0: f64 = 0.0;
        let mut var_qjcx_db1: f64 = 0.0;
        let mut var_qjcx_db2: f64 = 0.0;
        let mut var_qjcx_db3: f64 = 0.0;
        let mut var_qjcx_db4: f64 = 0.0;
        let mut var_qjcx_db5: f64 = 0.0;
        let mut var_qjcx_db6: f64 = 0.0;
        let mut var_qjcx_db7: f64 = 0.0;
        let mut var_qjcx_1: f64 = 0.0;
        let mut var_qjcx_1_dn0: f64 = 0.0;
        let mut var_qjcx_1_dn1: f64 = 0.0;
        let mut var_qjcx_1_dn2: f64 = 0.0;
        let mut var_qjcx_1_dn3: f64 = 0.0;
        let mut var_qjcx_1_dn4: f64 = 0.0;
        let mut var_qjcx_1_dn5: f64 = 0.0;
        let mut var_qjcx_1_dn6: f64 = 0.0;
        let mut var_qjcx_1_dn7: f64 = 0.0;
        let mut var_qjcx_1_dn8: f64 = 0.0;
        let mut var_qjcx_1_dn9: f64 = 0.0;
        let mut var_qjcx_1_db0: f64 = 0.0;
        let mut var_qjcx_1_db1: f64 = 0.0;
        let mut var_qjcx_1_db2: f64 = 0.0;
        let mut var_qjcx_1_db3: f64 = 0.0;
        let mut var_qjcx_1_db4: f64 = 0.0;
        let mut var_qjcx_1_db5: f64 = 0.0;
        let mut var_qjcx_1_db6: f64 = 0.0;
        let mut var_qjcx_1_db7: f64 = 0.0;
        let mut var_qjci: f64 = 0.0;
        let mut var_qjci_dn0: f64 = 0.0;
        let mut var_qjci_dn1: f64 = 0.0;
        let mut var_qjci_dn2: f64 = 0.0;
        let mut var_qjci_dn3: f64 = 0.0;
        let mut var_qjci_dn4: f64 = 0.0;
        let mut var_qjci_dn5: f64 = 0.0;
        let mut var_qjci_dn6: f64 = 0.0;
        let mut var_qjci_dn7: f64 = 0.0;
        let mut var_qjci_dn8: f64 = 0.0;
        let mut var_qjci_dn9: f64 = 0.0;
        let mut var_qjci_db0: f64 = 0.0;
        let mut var_qjci_db1: f64 = 0.0;
        let mut var_qjci_db2: f64 = 0.0;
        let mut var_qjci_db3: f64 = 0.0;
        let mut var_qjci_db4: f64 = 0.0;
        let mut var_qjci_db5: f64 = 0.0;
        let mut var_qjci_db6: f64 = 0.0;
        let mut var_qjci_db7: f64 = 0.0;
        let mut var_qjci_1: f64 = 0.0;
        let mut var_qjci_1_dn0: f64 = 0.0;
        let mut var_qjci_1_dn1: f64 = 0.0;
        let mut var_qjci_1_dn2: f64 = 0.0;
        let mut var_qjci_1_dn3: f64 = 0.0;
        let mut var_qjci_1_dn4: f64 = 0.0;
        let mut var_qjci_1_dn5: f64 = 0.0;
        let mut var_qjci_1_dn6: f64 = 0.0;
        let mut var_qjci_1_dn7: f64 = 0.0;
        let mut var_qjci_1_dn8: f64 = 0.0;
        let mut var_qjci_1_dn9: f64 = 0.0;
        let mut var_qjci_1_db0: f64 = 0.0;
        let mut var_qjci_1_db1: f64 = 0.0;
        let mut var_qjci_1_db2: f64 = 0.0;
        let mut var_qjci_1_db3: f64 = 0.0;
        let mut var_qjci_1_db4: f64 = 0.0;
        let mut var_qjci_1_db5: f64 = 0.0;
        let mut var_qjci_1_db6: f64 = 0.0;
        let mut var_qjci_1_db7: f64 = 0.0;
        let mut var_qxf1: f64 = 0.0;
        let mut var_qxf1_dn0: f64 = 0.0;
        let mut var_qxf1_dn1: f64 = 0.0;
        let mut var_qxf1_dn2: f64 = 0.0;
        let mut var_qxf1_dn3: f64 = 0.0;
        let mut var_qxf1_dn4: f64 = 0.0;
        let mut var_qxf1_dn5: f64 = 0.0;
        let mut var_qxf1_dn6: f64 = 0.0;
        let mut var_qxf1_dn7: f64 = 0.0;
        let mut var_qxf1_dn8: f64 = 0.0;
        let mut var_qxf1_dn9: f64 = 0.0;
        let mut var_qxf1_db0: f64 = 0.0;
        let mut var_qxf1_db1: f64 = 0.0;
        let mut var_qxf1_db2: f64 = 0.0;
        let mut var_qxf1_db3: f64 = 0.0;
        let mut var_qxf1_db4: f64 = 0.0;
        let mut var_qxf1_db5: f64 = 0.0;
        let mut var_qxf1_db6: f64 = 0.0;
        let mut var_qxf1_db7: f64 = 0.0;
        let mut var_ovaf: f64 = 0.0;
        let mut var_ovar: f64 = 0.0;
        let mut var_oikf: f64 = 0.0;
        let mut var_oikf_dn0: f64 = 0.0;
        let mut var_oikf_dn1: f64 = 0.0;
        let mut var_oikf_dn2: f64 = 0.0;
        let mut var_oikf_dn3: f64 = 0.0;
        let mut var_oikf_dn4: f64 = 0.0;
        let mut var_oikf_dn5: f64 = 0.0;
        let mut var_oikf_dn6: f64 = 0.0;
        let mut var_oikf_dn7: f64 = 0.0;
        let mut var_oikf_dn8: f64 = 0.0;
        let mut var_oikf_dn9: f64 = 0.0;
        let mut var_oikf_db0: f64 = 0.0;
        let mut var_oikf_db1: f64 = 0.0;
        let mut var_oikf_db2: f64 = 0.0;
        let mut var_oikf_db3: f64 = 0.0;
        let mut var_oikf_db4: f64 = 0.0;
        let mut var_oikf_db5: f64 = 0.0;
        let mut var_oikf_db6: f64 = 0.0;
        let mut var_oikf_db7: f64 = 0.0;
        let mut var_oikr: f64 = 0.0;
        let mut var_argt: f64 = 0.0;
        let mut var_argt_dn0: f64 = 0.0;
        let mut var_argt_dn1: f64 = 0.0;
        let mut var_argt_dn2: f64 = 0.0;
        let mut var_argt_dn3: f64 = 0.0;
        let mut var_argt_dn4: f64 = 0.0;
        let mut var_argt_dn5: f64 = 0.0;
        let mut var_argt_dn6: f64 = 0.0;
        let mut var_argt_dn7: f64 = 0.0;
        let mut var_argt_dn8: f64 = 0.0;
        let mut var_argt_dn9: f64 = 0.0;
        let mut var_argt_db0: f64 = 0.0;
        let mut var_argt_db1: f64 = 0.0;
        let mut var_argt_db2: f64 = 0.0;
        let mut var_argt_db3: f64 = 0.0;
        let mut var_argt_db4: f64 = 0.0;
        let mut var_argt_db5: f64 = 0.0;
        let mut var_argt_db6: f64 = 0.0;
        let mut var_argt_db7: f64 = 0.0;
        let mut var_veci: f64 = 0.0;
        let mut var_veci_dn0: f64 = 0.0;
        let mut var_veci_dn1: f64 = 0.0;
        let mut var_veci_dn2: f64 = 0.0;
        let mut var_veci_dn3: f64 = 0.0;
        let mut var_veci_dn4: f64 = 0.0;
        let mut var_veci_dn5: f64 = 0.0;
        let mut var_veci_dn6: f64 = 0.0;
        let mut var_veci_dn7: f64 = 0.0;
        let mut var_veci_dn8: f64 = 0.0;
        let mut var_veci_dn9: f64 = 0.0;
        let mut var_veci_db0: f64 = 0.0;
        let mut var_veci_db1: f64 = 0.0;
        let mut var_veci_db2: f64 = 0.0;
        let mut var_veci_db3: f64 = 0.0;
        let mut var_veci_db4: f64 = 0.0;
        let mut var_veci_db5: f64 = 0.0;
        let mut var_veci_db6: f64 = 0.0;
        let mut var_veci_db7: f64 = 0.0;
        let mut var_vbiei: f64 = 0.0;
        let mut var_vbiei_dn0: f64 = 0.0;
        let mut var_vbiei_dn1: f64 = 0.0;
        let mut var_vbiei_dn2: f64 = 0.0;
        let mut var_vbiei_dn3: f64 = 0.0;
        let mut var_vbiei_dn4: f64 = 0.0;
        let mut var_vbiei_dn5: f64 = 0.0;
        let mut var_vbiei_dn6: f64 = 0.0;
        let mut var_vbiei_dn7: f64 = 0.0;
        let mut var_vbiei_dn8: f64 = 0.0;
        let mut var_vbiei_dn9: f64 = 0.0;
        let mut var_vbiei_db0: f64 = 0.0;
        let mut var_vbiei_db1: f64 = 0.0;
        let mut var_vbiei_db2: f64 = 0.0;
        let mut var_vbiei_db3: f64 = 0.0;
        let mut var_vbiei_db4: f64 = 0.0;
        let mut var_vbiei_db5: f64 = 0.0;
        let mut var_vbiei_db6: f64 = 0.0;
        let mut var_vbiei_db7: f64 = 0.0;
        let mut var_vbici: f64 = 0.0;
        let mut var_vbici_dn0: f64 = 0.0;
        let mut var_vbici_dn1: f64 = 0.0;
        let mut var_vbici_dn2: f64 = 0.0;
        let mut var_vbici_dn3: f64 = 0.0;
        let mut var_vbici_dn4: f64 = 0.0;
        let mut var_vbici_dn5: f64 = 0.0;
        let mut var_vbici_dn6: f64 = 0.0;
        let mut var_vbici_dn7: f64 = 0.0;
        let mut var_vbici_dn8: f64 = 0.0;
        let mut var_vbici_dn9: f64 = 0.0;
        let mut var_vbici_db0: f64 = 0.0;
        let mut var_vbici_db1: f64 = 0.0;
        let mut var_vbici_db2: f64 = 0.0;
        let mut var_vbici_db3: f64 = 0.0;
        let mut var_vbici_db4: f64 = 0.0;
        let mut var_vbici_db5: f64 = 0.0;
        let mut var_vbici_db6: f64 = 0.0;
        let mut var_vbici_db7: f64 = 0.0;
        let mut var_vbci: f64 = 0.0;
        let mut var_vbci_dn0: f64 = 0.0;
        let mut var_vbci_dn1: f64 = 0.0;
        let mut var_vbci_dn2: f64 = 0.0;
        let mut var_vbci_dn3: f64 = 0.0;
        let mut var_vbci_dn4: f64 = 0.0;
        let mut var_vbci_dn5: f64 = 0.0;
        let mut var_vbci_dn6: f64 = 0.0;
        let mut var_vbci_dn7: f64 = 0.0;
        let mut var_vbci_dn8: f64 = 0.0;
        let mut var_vbci_dn9: f64 = 0.0;
        let mut var_vbci_db0: f64 = 0.0;
        let mut var_vbci_db1: f64 = 0.0;
        let mut var_vbci_db2: f64 = 0.0;
        let mut var_vbci_db3: f64 = 0.0;
        let mut var_vbci_db4: f64 = 0.0;
        let mut var_vbci_db5: f64 = 0.0;
        let mut var_vbci_db6: f64 = 0.0;
        let mut var_vbci_db7: f64 = 0.0;
        let mut var_vbbi: f64 = 0.0;
        let mut var_vbbi_dn0: f64 = 0.0;
        let mut var_vbbi_dn1: f64 = 0.0;
        let mut var_vbbi_dn2: f64 = 0.0;
        let mut var_vbbi_dn3: f64 = 0.0;
        let mut var_vbbi_dn4: f64 = 0.0;
        let mut var_vbbi_dn5: f64 = 0.0;
        let mut var_vbbi_dn6: f64 = 0.0;
        let mut var_vbbi_dn7: f64 = 0.0;
        let mut var_vbbi_dn8: f64 = 0.0;
        let mut var_vbbi_dn9: f64 = 0.0;
        let mut var_vbbi_db0: f64 = 0.0;
        let mut var_vbbi_db1: f64 = 0.0;
        let mut var_vbbi_db2: f64 = 0.0;
        let mut var_vbbi_db3: f64 = 0.0;
        let mut var_vbbi_db4: f64 = 0.0;
        let mut var_vbbi_db5: f64 = 0.0;
        let mut var_vbbi_db6: f64 = 0.0;
        let mut var_vbbi_db7: f64 = 0.0;
        let mut var_veei: f64 = 0.0;
        let mut var_veei_dn0: f64 = 0.0;
        let mut var_veei_dn1: f64 = 0.0;
        let mut var_veei_dn2: f64 = 0.0;
        let mut var_veei_dn3: f64 = 0.0;
        let mut var_veei_dn4: f64 = 0.0;
        let mut var_veei_dn5: f64 = 0.0;
        let mut var_veei_dn6: f64 = 0.0;
        let mut var_veei_dn7: f64 = 0.0;
        let mut var_veei_dn8: f64 = 0.0;
        let mut var_veei_dn9: f64 = 0.0;
        let mut var_veei_db0: f64 = 0.0;
        let mut var_veei_db1: f64 = 0.0;
        let mut var_veei_db2: f64 = 0.0;
        let mut var_veei_db3: f64 = 0.0;
        let mut var_veei_db4: f64 = 0.0;
        let mut var_veei_db5: f64 = 0.0;
        let mut var_veei_db6: f64 = 0.0;
        let mut var_veei_db7: f64 = 0.0;
        let mut var_fact1: f64 = 0.0;
        let mut var_fact2: f64 = 0.0;
        let mut var_fact2_dn0: f64 = 0.0;
        let mut var_fact2_dn1: f64 = 0.0;
        let mut var_fact2_dn2: f64 = 0.0;
        let mut var_fact2_dn3: f64 = 0.0;
        let mut var_fact2_dn4: f64 = 0.0;
        let mut var_fact2_dn5: f64 = 0.0;
        let mut var_fact2_dn6: f64 = 0.0;
        let mut var_fact2_dn7: f64 = 0.0;
        let mut var_fact2_dn8: f64 = 0.0;
        let mut var_fact2_dn9: f64 = 0.0;
        let mut var_fact2_db0: f64 = 0.0;
        let mut var_fact2_db1: f64 = 0.0;
        let mut var_fact2_db2: f64 = 0.0;
        let mut var_fact2_db3: f64 = 0.0;
        let mut var_fact2_db4: f64 = 0.0;
        let mut var_fact2_db5: f64 = 0.0;
        let mut var_fact2_db6: f64 = 0.0;
        let mut var_fact2_db7: f64 = 0.0;
        let mut var_egfet: f64 = 0.0;
        let mut var_egfet_dn0: f64 = 0.0;
        let mut var_egfet_dn1: f64 = 0.0;
        let mut var_egfet_dn2: f64 = 0.0;
        let mut var_egfet_dn3: f64 = 0.0;
        let mut var_egfet_dn4: f64 = 0.0;
        let mut var_egfet_dn5: f64 = 0.0;
        let mut var_egfet_dn6: f64 = 0.0;
        let mut var_egfet_dn7: f64 = 0.0;
        let mut var_egfet_dn8: f64 = 0.0;
        let mut var_egfet_dn9: f64 = 0.0;
        let mut var_egfet_db0: f64 = 0.0;
        let mut var_egfet_db1: f64 = 0.0;
        let mut var_egfet_db2: f64 = 0.0;
        let mut var_egfet_db3: f64 = 0.0;
        let mut var_egfet_db4: f64 = 0.0;
        let mut var_egfet_db5: f64 = 0.0;
        let mut var_egfet_db6: f64 = 0.0;
        let mut var_egfet_db7: f64 = 0.0;
        let mut var_arg0: f64 = 0.0;
        let mut var_arg0_dn0: f64 = 0.0;
        let mut var_arg0_dn1: f64 = 0.0;
        let mut var_arg0_dn2: f64 = 0.0;
        let mut var_arg0_dn3: f64 = 0.0;
        let mut var_arg0_dn4: f64 = 0.0;
        let mut var_arg0_dn5: f64 = 0.0;
        let mut var_arg0_dn6: f64 = 0.0;
        let mut var_arg0_dn7: f64 = 0.0;
        let mut var_arg0_dn8: f64 = 0.0;
        let mut var_arg0_dn9: f64 = 0.0;
        let mut var_arg0_db0: f64 = 0.0;
        let mut var_arg0_db1: f64 = 0.0;
        let mut var_arg0_db2: f64 = 0.0;
        let mut var_arg0_db3: f64 = 0.0;
        let mut var_arg0_db4: f64 = 0.0;
        let mut var_arg0_db5: f64 = 0.0;
        let mut var_arg0_db6: f64 = 0.0;
        let mut var_arg0_db7: f64 = 0.0;
        let mut var_pbfact: f64 = 0.0;
        let mut var_pbfact_dn0: f64 = 0.0;
        let mut var_pbfact_dn1: f64 = 0.0;
        let mut var_pbfact_dn2: f64 = 0.0;
        let mut var_pbfact_dn3: f64 = 0.0;
        let mut var_pbfact_dn4: f64 = 0.0;
        let mut var_pbfact_dn5: f64 = 0.0;
        let mut var_pbfact_dn6: f64 = 0.0;
        let mut var_pbfact_dn7: f64 = 0.0;
        let mut var_pbfact_dn8: f64 = 0.0;
        let mut var_pbfact_dn9: f64 = 0.0;
        let mut var_pbfact_db0: f64 = 0.0;
        let mut var_pbfact_db1: f64 = 0.0;
        let mut var_pbfact_db2: f64 = 0.0;
        let mut var_pbfact_db3: f64 = 0.0;
        let mut var_pbfact_db4: f64 = 0.0;
        let mut var_pbfact_db5: f64 = 0.0;
        let mut var_pbfact_db6: f64 = 0.0;
        let mut var_pbfact_db7: f64 = 0.0;
        let mut var_pbo: f64 = 0.0;
        let mut var_pbo_dn0: f64 = 0.0;
        let mut var_pbo_dn1: f64 = 0.0;
        let mut var_pbo_dn2: f64 = 0.0;
        let mut var_pbo_dn3: f64 = 0.0;
        let mut var_pbo_dn4: f64 = 0.0;
        let mut var_pbo_dn5: f64 = 0.0;
        let mut var_pbo_dn6: f64 = 0.0;
        let mut var_pbo_dn7: f64 = 0.0;
        let mut var_pbo_dn8: f64 = 0.0;
        let mut var_pbo_dn9: f64 = 0.0;
        let mut var_pbo_db0: f64 = 0.0;
        let mut var_pbo_db1: f64 = 0.0;
        let mut var_pbo_db2: f64 = 0.0;
        let mut var_pbo_db3: f64 = 0.0;
        let mut var_pbo_db4: f64 = 0.0;
        let mut var_pbo_db5: f64 = 0.0;
        let mut var_pbo_db6: f64 = 0.0;
        let mut var_pbo_db7: f64 = 0.0;
        let mut var_gmaold: f64 = 0.0;
        let mut var_gmaold_dn0: f64 = 0.0;
        let mut var_gmaold_dn1: f64 = 0.0;
        let mut var_gmaold_dn2: f64 = 0.0;
        let mut var_gmaold_dn3: f64 = 0.0;
        let mut var_gmaold_dn4: f64 = 0.0;
        let mut var_gmaold_dn5: f64 = 0.0;
        let mut var_gmaold_dn6: f64 = 0.0;
        let mut var_gmaold_dn7: f64 = 0.0;
        let mut var_gmaold_dn8: f64 = 0.0;
        let mut var_gmaold_dn9: f64 = 0.0;
        let mut var_gmaold_db0: f64 = 0.0;
        let mut var_gmaold_db1: f64 = 0.0;
        let mut var_gmaold_db2: f64 = 0.0;
        let mut var_gmaold_db3: f64 = 0.0;
        let mut var_gmaold_db4: f64 = 0.0;
        let mut var_gmaold_db5: f64 = 0.0;
        let mut var_gmaold_db6: f64 = 0.0;
        let mut var_gmaold_db7: f64 = 0.0;
        let mut var_gmanew: f64 = 0.0;
        let mut var_gmanew_dn0: f64 = 0.0;
        let mut var_gmanew_dn1: f64 = 0.0;
        let mut var_gmanew_dn2: f64 = 0.0;
        let mut var_gmanew_dn3: f64 = 0.0;
        let mut var_gmanew_dn4: f64 = 0.0;
        let mut var_gmanew_dn5: f64 = 0.0;
        let mut var_gmanew_dn6: f64 = 0.0;
        let mut var_gmanew_dn7: f64 = 0.0;
        let mut var_gmanew_dn8: f64 = 0.0;
        let mut var_gmanew_dn9: f64 = 0.0;
        let mut var_gmanew_db0: f64 = 0.0;
        let mut var_gmanew_db1: f64 = 0.0;
        let mut var_gmanew_db2: f64 = 0.0;
        let mut var_gmanew_db3: f64 = 0.0;
        let mut var_gmanew_db4: f64 = 0.0;
        let mut var_gmanew_db5: f64 = 0.0;
        let mut var_gmanew_db6: f64 = 0.0;
        let mut var_gmanew_db7: f64 = 0.0;
        let mut var_cjt: f64 = 0.0;
        let mut var_cjt_dn0: f64 = 0.0;
        let mut var_cjt_dn1: f64 = 0.0;
        let mut var_cjt_dn2: f64 = 0.0;
        let mut var_cjt_dn3: f64 = 0.0;
        let mut var_cjt_dn4: f64 = 0.0;
        let mut var_cjt_dn5: f64 = 0.0;
        let mut var_cjt_dn6: f64 = 0.0;
        let mut var_cjt_dn7: f64 = 0.0;
        let mut var_cjt_dn8: f64 = 0.0;
        let mut var_cjt_dn9: f64 = 0.0;
        let mut var_cjt_db0: f64 = 0.0;
        let mut var_cjt_db1: f64 = 0.0;
        let mut var_cjt_db2: f64 = 0.0;
        let mut var_cjt_db3: f64 = 0.0;
        let mut var_cjt_db4: f64 = 0.0;
        let mut var_cjt_db5: f64 = 0.0;
        let mut var_cjt_db6: f64 = 0.0;
        let mut var_cjt_db7: f64 = 0.0;
        let mut var_argbv: f64 = 0.0;
        let mut var_argbv_dn0: f64 = 0.0;
        let mut var_argbv_dn1: f64 = 0.0;
        let mut var_argbv_dn2: f64 = 0.0;
        let mut var_argbv_dn3: f64 = 0.0;
        let mut var_argbv_dn4: f64 = 0.0;
        let mut var_argbv_dn5: f64 = 0.0;
        let mut var_argbv_dn6: f64 = 0.0;
        let mut var_argbv_dn7: f64 = 0.0;
        let mut var_argbv_dn8: f64 = 0.0;
        let mut var_argbv_dn9: f64 = 0.0;
        let mut var_argbv_db0: f64 = 0.0;
        let mut var_argbv_db1: f64 = 0.0;
        let mut var_argbv_db2: f64 = 0.0;
        let mut var_argbv_db3: f64 = 0.0;
        let mut var_argbv_db4: f64 = 0.0;
        let mut var_argbv_db5: f64 = 0.0;
        let mut var_argbv_db6: f64 = 0.0;
        let mut var_argbv_db7: f64 = 0.0;
        let mut var_argbvvt: f64 = 0.0;
        let mut var_argbvvt_dn0: f64 = 0.0;
        let mut var_argbvvt_dn1: f64 = 0.0;
        let mut var_argbvvt_dn2: f64 = 0.0;
        let mut var_argbvvt_dn3: f64 = 0.0;
        let mut var_argbvvt_dn4: f64 = 0.0;
        let mut var_argbvvt_dn5: f64 = 0.0;
        let mut var_argbvvt_dn6: f64 = 0.0;
        let mut var_argbvvt_dn7: f64 = 0.0;
        let mut var_argbvvt_dn8: f64 = 0.0;
        let mut var_argbvvt_dn9: f64 = 0.0;
        let mut var_argbvvt_db0: f64 = 0.0;
        let mut var_argbvvt_db1: f64 = 0.0;
        let mut var_argbvvt_db2: f64 = 0.0;
        let mut var_argbvvt_db3: f64 = 0.0;
        let mut var_argbvvt_db4: f64 = 0.0;
        let mut var_argbvvt_db5: f64 = 0.0;
        let mut var_argbvvt_db6: f64 = 0.0;
        let mut var_argbvvt_db7: f64 = 0.0;
        let mut var_argtr: f64 = 0.0;
        let mut var_argtr_dn0: f64 = 0.0;
        let mut var_argtr_dn1: f64 = 0.0;
        let mut var_argtr_dn2: f64 = 0.0;
        let mut var_argtr_dn3: f64 = 0.0;
        let mut var_argtr_dn4: f64 = 0.0;
        let mut var_argtr_dn5: f64 = 0.0;
        let mut var_argtr_dn6: f64 = 0.0;
        let mut var_argtr_dn7: f64 = 0.0;
        let mut var_argtr_dn8: f64 = 0.0;
        let mut var_argtr_dn9: f64 = 0.0;
        let mut var_argtr_db0: f64 = 0.0;
        let mut var_argtr_db1: f64 = 0.0;
        let mut var_argtr_db2: f64 = 0.0;
        let mut var_argtr_db3: f64 = 0.0;
        let mut var_argtr_db4: f64 = 0.0;
        let mut var_argtr_db5: f64 = 0.0;
        let mut var_argtr_db6: f64 = 0.0;
        let mut var_argtr_db7: f64 = 0.0;
        let mut var_isr_t: f64 = 0.0;
        let mut var_isr_t_dn0: f64 = 0.0;
        let mut var_isr_t_dn1: f64 = 0.0;
        let mut var_isr_t_dn2: f64 = 0.0;
        let mut var_isr_t_dn3: f64 = 0.0;
        let mut var_isr_t_dn4: f64 = 0.0;
        let mut var_isr_t_dn5: f64 = 0.0;
        let mut var_isr_t_dn6: f64 = 0.0;
        let mut var_isr_t_dn7: f64 = 0.0;
        let mut var_isr_t_dn8: f64 = 0.0;
        let mut var_isr_t_dn9: f64 = 0.0;
        let mut var_isr_t_db0: f64 = 0.0;
        let mut var_isr_t_db1: f64 = 0.0;
        let mut var_isr_t_db2: f64 = 0.0;
        let mut var_isr_t_db3: f64 = 0.0;
        let mut var_isr_t_db4: f64 = 0.0;
        let mut var_isr_t_db5: f64 = 0.0;
        let mut var_isr_t_db6: f64 = 0.0;
        let mut var_isr_t_db7: f64 = 0.0;
        let mut var_fbwm: f64 = 0.0;
        let mut var_fbwm_dn0: f64 = 0.0;
        let mut var_fbwm_dn1: f64 = 0.0;
        let mut var_fbwm_dn2: f64 = 0.0;
        let mut var_fbwm_dn3: f64 = 0.0;
        let mut var_fbwm_dn4: f64 = 0.0;
        let mut var_fbwm_dn5: f64 = 0.0;
        let mut var_fbwm_dn6: f64 = 0.0;
        let mut var_fbwm_dn7: f64 = 0.0;
        let mut var_fbwm_dn8: f64 = 0.0;
        let mut var_fbwm_dn9: f64 = 0.0;
        let mut var_fbwm_db0: f64 = 0.0;
        let mut var_fbwm_db1: f64 = 0.0;
        let mut var_fbwm_db2: f64 = 0.0;
        let mut var_fbwm_db3: f64 = 0.0;
        let mut var_fbwm_db4: f64 = 0.0;
        let mut var_fbwm_db5: f64 = 0.0;
        let mut var_fbwm_db6: f64 = 0.0;
        let mut var_fbwm_db7: f64 = 0.0;
        let mut var_vbc: f64 = 0.0;
        let mut var_vbc_dn0: f64 = 0.0;
        let mut var_vbc_dn1: f64 = 0.0;
        let mut var_vbc_dn2: f64 = 0.0;
        let mut var_vbc_dn3: f64 = 0.0;
        let mut var_vbc_dn4: f64 = 0.0;
        let mut var_vbc_dn5: f64 = 0.0;
        let mut var_vbc_dn6: f64 = 0.0;
        let mut var_vbc_dn7: f64 = 0.0;
        let mut var_vbc_dn8: f64 = 0.0;
        let mut var_vbc_dn9: f64 = 0.0;
        let mut var_vbc_db0: f64 = 0.0;
        let mut var_vbc_db1: f64 = 0.0;
        let mut var_vbc_db2: f64 = 0.0;
        let mut var_vbc_db3: f64 = 0.0;
        let mut var_vbc_db4: f64 = 0.0;
        let mut var_vbc_db5: f64 = 0.0;
        let mut var_vbc_db6: f64 = 0.0;
        let mut var_vbc_db7: f64 = 0.0;
        let mut var_dkqb: f64 = 0.0;
        let mut var_dkqb_dn0: f64 = 0.0;
        let mut var_dkqb_dn1: f64 = 0.0;
        let mut var_dkqb_dn2: f64 = 0.0;
        let mut var_dkqb_dn3: f64 = 0.0;
        let mut var_dkqb_dn4: f64 = 0.0;
        let mut var_dkqb_dn5: f64 = 0.0;
        let mut var_dkqb_dn6: f64 = 0.0;
        let mut var_dkqb_dn7: f64 = 0.0;
        let mut var_dkqb_dn8: f64 = 0.0;
        let mut var_dkqb_dn9: f64 = 0.0;
        let mut var_dkqb_db0: f64 = 0.0;
        let mut var_dkqb_db1: f64 = 0.0;
        let mut var_dkqb_db2: f64 = 0.0;
        let mut var_dkqb_db3: f64 = 0.0;
        let mut var_dkqb_db4: f64 = 0.0;
        let mut var_dkqb_db5: f64 = 0.0;
        let mut var_dkqb_db6: f64 = 0.0;
        let mut var_dkqb_db7: f64 = 0.0;
        let mut var_vtff: f64 = 0.0;
        let mut var_vtff_dn0: f64 = 0.0;
        let mut var_vtff_dn1: f64 = 0.0;
        let mut var_vtff_dn2: f64 = 0.0;
        let mut var_vtff_dn3: f64 = 0.0;
        let mut var_vtff_dn4: f64 = 0.0;
        let mut var_vtff_dn5: f64 = 0.0;
        let mut var_vtff_dn6: f64 = 0.0;
        let mut var_vtff_dn7: f64 = 0.0;
        let mut var_vtff_dn8: f64 = 0.0;
        let mut var_vtff_dn9: f64 = 0.0;
        let mut var_vtff_db0: f64 = 0.0;
        let mut var_vtff_db1: f64 = 0.0;
        let mut var_vtff_db2: f64 = 0.0;
        let mut var_vtff_db3: f64 = 0.0;
        let mut var_vtff_db4: f64 = 0.0;
        let mut var_vtff_db5: f64 = 0.0;
        let mut var_vtff_db6: f64 = 0.0;
        let mut var_vtff_db7: f64 = 0.0;
        let mut var_vtff1: f64 = 0.0;
        let mut var_vtff1_dn0: f64 = 0.0;
        let mut var_vtff1_dn1: f64 = 0.0;
        let mut var_vtff1_dn2: f64 = 0.0;
        let mut var_vtff1_dn3: f64 = 0.0;
        let mut var_vtff1_dn4: f64 = 0.0;
        let mut var_vtff1_dn5: f64 = 0.0;
        let mut var_vtff1_dn6: f64 = 0.0;
        let mut var_vtff1_dn7: f64 = 0.0;
        let mut var_vtff1_dn8: f64 = 0.0;
        let mut var_vtff1_dn9: f64 = 0.0;
        let mut var_vtff1_db0: f64 = 0.0;
        let mut var_vtff1_db1: f64 = 0.0;
        let mut var_vtff1_db2: f64 = 0.0;
        let mut var_vtff1_db3: f64 = 0.0;
        let mut var_vtff1_db4: f64 = 0.0;
        let mut var_vtff1_db5: f64 = 0.0;
        let mut var_vtff1_db6: f64 = 0.0;
        let mut var_vtff1_db7: f64 = 0.0;
        let mut var_vbesat: f64 = 0.0;
        let mut var_vbesat_dn0: f64 = 0.0;
        let mut var_vbesat_dn1: f64 = 0.0;
        let mut var_vbesat_dn2: f64 = 0.0;
        let mut var_vbesat_dn3: f64 = 0.0;
        let mut var_vbesat_dn4: f64 = 0.0;
        let mut var_vbesat_dn5: f64 = 0.0;
        let mut var_vbesat_dn6: f64 = 0.0;
        let mut var_vbesat_dn7: f64 = 0.0;
        let mut var_vbesat_dn8: f64 = 0.0;
        let mut var_vbesat_dn9: f64 = 0.0;
        let mut var_vbesat_db0: f64 = 0.0;
        let mut var_vbesat_db1: f64 = 0.0;
        let mut var_vbesat_db2: f64 = 0.0;
        let mut var_vbesat_db3: f64 = 0.0;
        let mut var_vbesat_db4: f64 = 0.0;
        let mut var_vbesat_db5: f64 = 0.0;
        let mut var_vbesat_db6: f64 = 0.0;
        let mut var_vbesat_db7: f64 = 0.0;
        let mut var_veesat: f64 = 0.0;
        let mut var_veesat_dn0: f64 = 0.0;
        let mut var_veesat_dn1: f64 = 0.0;
        let mut var_veesat_dn2: f64 = 0.0;
        let mut var_veesat_dn3: f64 = 0.0;
        let mut var_veesat_dn4: f64 = 0.0;
        let mut var_veesat_dn5: f64 = 0.0;
        let mut var_veesat_dn6: f64 = 0.0;
        let mut var_veesat_dn7: f64 = 0.0;
        let mut var_veesat_dn8: f64 = 0.0;
        let mut var_veesat_dn9: f64 = 0.0;
        let mut var_veesat_db0: f64 = 0.0;
        let mut var_veesat_db1: f64 = 0.0;
        let mut var_veesat_db2: f64 = 0.0;
        let mut var_veesat_db3: f64 = 0.0;
        let mut var_veesat_db4: f64 = 0.0;
        let mut var_veesat_db5: f64 = 0.0;
        let mut var_veesat_db6: f64 = 0.0;
        let mut var_veesat_db7: f64 = 0.0;
        let mut var_t0: f64 = 0.0;
        let mut var_t0_dn0: f64 = 0.0;
        let mut var_t0_dn1: f64 = 0.0;
        let mut var_t0_dn2: f64 = 0.0;
        let mut var_t0_dn3: f64 = 0.0;
        let mut var_t0_dn4: f64 = 0.0;
        let mut var_t0_dn5: f64 = 0.0;
        let mut var_t0_dn6: f64 = 0.0;
        let mut var_t0_dn7: f64 = 0.0;
        let mut var_t0_dn8: f64 = 0.0;
        let mut var_t0_dn9: f64 = 0.0;
        let mut var_t0_db0: f64 = 0.0;
        let mut var_t0_db1: f64 = 0.0;
        let mut var_t0_db2: f64 = 0.0;
        let mut var_t0_db3: f64 = 0.0;
        let mut var_t0_db4: f64 = 0.0;
        let mut var_t0_db5: f64 = 0.0;
        let mut var_t0_db6: f64 = 0.0;
        let mut var_t0_db7: f64 = 0.0;
        let mut var_d_ratio: f64 = 0.0;
        let mut var_d_ratio_dn0: f64 = 0.0;
        let mut var_d_ratio_dn1: f64 = 0.0;
        let mut var_d_ratio_dn2: f64 = 0.0;
        let mut var_d_ratio_dn3: f64 = 0.0;
        let mut var_d_ratio_dn4: f64 = 0.0;
        let mut var_d_ratio_dn5: f64 = 0.0;
        let mut var_d_ratio_dn6: f64 = 0.0;
        let mut var_d_ratio_dn7: f64 = 0.0;
        let mut var_d_ratio_dn8: f64 = 0.0;
        let mut var_d_ratio_dn9: f64 = 0.0;
        let mut var_d_ratio_db0: f64 = 0.0;
        let mut var_d_ratio_db1: f64 = 0.0;
        let mut var_d_ratio_db2: f64 = 0.0;
        let mut var_d_ratio_db3: f64 = 0.0;
        let mut var_d_ratio_db4: f64 = 0.0;
        let mut var_d_ratio_db5: f64 = 0.0;
        let mut var_d_ratio_db6: f64 = 0.0;
        let mut var_d_ratio_db7: f64 = 0.0;
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

        Self::stamp_transient_block_0(ctx, p, nodes, &mut var_argt, &mut var_argt_db0, &mut var_argt_db1, &mut var_argt_db2, &mut var_argt_db3, &mut var_argt_db4, &mut var_argt_db5, &mut var_argt_db6, &mut var_argt_db7, &mut var_argt_dn0, &mut var_argt_dn1, &mut var_argt_dn2, &mut var_argt_dn3, &mut var_argt_dn4, &mut var_argt_dn5, &mut var_argt_dn6, &mut var_argt_dn7, &mut var_argt_dn8, &mut var_argt_dn9, &mut var_argtr, &mut var_argtr_db0, &mut var_argtr_db1, &mut var_argtr_db2, &mut var_argtr_db3, &mut var_argtr_db4, &mut var_argtr_db5, &mut var_argtr_db6, &mut var_argtr_db7, &mut var_argtr_dn0, &mut var_argtr_dn1, &mut var_argtr_dn2, &mut var_argtr_dn3, &mut var_argtr_dn4, &mut var_argtr_dn5, &mut var_argtr_dn6, &mut var_argtr_dn7, &mut var_argtr_dn8, &mut var_argtr_dn9, &mut var_bf_t, &mut var_bf_t_db0, &mut var_bf_t_db1, &mut var_bf_t_db2, &mut var_bf_t_db3, &mut var_bf_t_db4, &mut var_bf_t_db5, &mut var_bf_t_db6, &mut var_bf_t_db7, &mut var_bf_t_dn0, &mut var_bf_t_dn1, &mut var_bf_t_dn2, &mut var_bf_t_dn3, &mut var_bf_t_dn4, &mut var_bf_t_dn5, &mut var_bf_t_dn6, &mut var_bf_t_dn7, &mut var_bf_t_dn8, &mut var_bf_t_dn9, &mut var_br_t, &mut var_br_t_db0, &mut var_br_t_db1, &mut var_br_t_db2, &mut var_br_t_db3, &mut var_br_t_db4, &mut var_br_t_db5, &mut var_br_t_db6, &mut var_br_t_db7, &mut var_br_t_dn0, &mut var_br_t_dn1, &mut var_br_t_dn2, &mut var_br_t_dn3, &mut var_br_t_dn4, &mut var_br_t_dn5, &mut var_br_t_dn6, &mut var_br_t_dn7, &mut var_br_t_dn8, &mut var_br_t_dn9, &mut var_bvr_t, &mut var_bvr_t_db0, &mut var_bvr_t_db1, &mut var_bvr_t_db2, &mut var_bvr_t_db3, &mut var_bvr_t_db4, &mut var_bvr_t_db5, &mut var_bvr_t_db6, &mut var_bvr_t_db7, &mut var_bvr_t_dn0, &mut var_bvr_t_dn1, &mut var_bvr_t_dn2, &mut var_bvr_t_dn3, &mut var_bvr_t_dn4, &mut var_bvr_t_dn5, &mut var_bvr_t_dn6, &mut var_bvr_t_dn7, &mut var_bvr_t_dn8, &mut var_bvr_t_dn9, &mut var_fbwm, &mut var_fbwm_db0, &mut var_fbwm_db1, &mut var_fbwm_db2, &mut var_fbwm_db3, &mut var_fbwm_db4, &mut var_fbwm_db5, &mut var_fbwm_db6, &mut var_fbwm_db7, &mut var_fbwm_dn0, &mut var_fbwm_dn1, &mut var_fbwm_dn2, &mut var_fbwm_dn3, &mut var_fbwm_dn4, &mut var_fbwm_dn5, &mut var_fbwm_dn6, &mut var_fbwm_dn7, &mut var_fbwm_dn8, &mut var_fbwm_dn9, &mut var_ijbv_t, &mut var_ijbv_t_db0, &mut var_ijbv_t_db1, &mut var_ijbv_t_db2, &mut var_ijbv_t_db3, &mut var_ijbv_t_db4, &mut var_ijbv_t_db5, &mut var_ijbv_t_db6, &mut var_ijbv_t_db7, &mut var_ijbv_t_dn0, &mut var_ijbv_t_dn1, &mut var_ijbv_t_dn2, &mut var_ijbv_t_dn3, &mut var_ijbv_t_dn4, &mut var_ijbv_t_dn5, &mut var_ijbv_t_dn6, &mut var_ijbv_t_dn7, &mut var_ijbv_t_dn8, &mut var_ijbv_t_dn9, &mut var_is_t, &mut var_is_t_db0, &mut var_is_t_db1, &mut var_is_t_db2, &mut var_is_t_db3, &mut var_is_t_db4, &mut var_is_t_db5, &mut var_is_t_db6, &mut var_is_t_db7, &mut var_is_t_dn0, &mut var_is_t_dn1, &mut var_is_t_dn2, &mut var_is_t_dn3, &mut var_is_t_dn4, &mut var_is_t_dn5, &mut var_is_t_dn6, &mut var_is_t_dn7, &mut var_is_t_dn8, &mut var_is_t_dn9, &mut var_isc_t, &mut var_isc_t_db0, &mut var_isc_t_db1, &mut var_isc_t_db2, &mut var_isc_t_db3, &mut var_isc_t_db4, &mut var_isc_t_db5, &mut var_isc_t_db6, &mut var_isc_t_db7, &mut var_isc_t_dn0, &mut var_isc_t_dn1, &mut var_isc_t_dn2, &mut var_isc_t_dn3, &mut var_isc_t_dn4, &mut var_isc_t_dn5, &mut var_isc_t_dn6, &mut var_isc_t_dn7, &mut var_isc_t_dn8, &mut var_isc_t_dn9, &mut var_ise_t, &mut var_ise_t_db0, &mut var_ise_t_db1, &mut var_ise_t_db2, &mut var_ise_t_db3, &mut var_ise_t_db4, &mut var_ise_t_db5, &mut var_ise_t_db6, &mut var_ise_t_db7, &mut var_ise_t_dn0, &mut var_ise_t_dn1, &mut var_ise_t_dn2, &mut var_ise_t_dn3, &mut var_ise_t_dn4, &mut var_ise_t_dn5, &mut var_ise_t_dn6, &mut var_ise_t_dn7, &mut var_ise_t_dn8, &mut var_ise_t_dn9, &mut var_isr_t, &mut var_isr_t_db0, &mut var_isr_t_db1, &mut var_isr_t_db2, &mut var_isr_t_db3, &mut var_isr_t_db4, &mut var_isr_t_db5, &mut var_isr_t_db6, &mut var_isr_t_db7, &mut var_isr_t_dn0, &mut var_isr_t_dn1, &mut var_isr_t_dn2, &mut var_isr_t_dn3, &mut var_isr_t_dn4, &mut var_isr_t_dn5, &mut var_isr_t_dn6, &mut var_isr_t_dn7, &mut var_isr_t_dn8, &mut var_isr_t_dn9, &mut var_lnrt, &mut var_lnrt_db0, &mut var_lnrt_db1, &mut var_lnrt_db2, &mut var_lnrt_db3, &mut var_lnrt_db4, &mut var_lnrt_db5, &mut var_lnrt_db6, &mut var_lnrt_db7, &mut var_lnrt_dn0, &mut var_lnrt_dn1, &mut var_lnrt_dn2, &mut var_lnrt_dn3, &mut var_lnrt_dn4, &mut var_lnrt_dn5, &mut var_lnrt_dn6, &mut var_lnrt_dn7, &mut var_lnrt_dn8, &mut var_lnrt_dn9, &mut var_oikf, &mut var_oikf_db0, &mut var_oikf_db1, &mut var_oikf_db2, &mut var_oikf_db3, &mut var_oikf_db4, &mut var_oikf_db5, &mut var_oikf_db6, &mut var_oikf_db7, &mut var_oikf_dn0, &mut var_oikf_dn1, &mut var_oikf_dn2, &mut var_oikf_dn3, &mut var_oikf_dn4, &mut var_oikf_dn5, &mut var_oikf_dn6, &mut var_oikf_dn7, &mut var_oikf_dn8, &mut var_oikf_dn9, &mut var_oikr, &mut var_ovaf, &mut var_ovar, &mut var_rt, &mut var_rt_db0, &mut var_rt_db1, &mut var_rt_db2, &mut var_rt_db3, &mut var_rt_db4, &mut var_rt_db5, &mut var_rt_db6, &mut var_rt_db7, &mut var_rt_dn0, &mut var_rt_dn1, &mut var_rt_dn2, &mut var_rt_dn3, &mut var_rt_dn4, &mut var_rt_dn5, &mut var_rt_dn6, &mut var_rt_dn7, &mut var_rt_dn8, &mut var_rt_dn9, &mut var_tamb, &mut var_tamb_db0, &mut var_tamb_db1, &mut var_tamb_db2, &mut var_tamb_db3, &mut var_tamb_db4, &mut var_tamb_db5, &mut var_tamb_db6, &mut var_tamb_db7, &mut var_tamb_dn0, &mut var_tamb_dn1, &mut var_tamb_dn2, &mut var_tamb_dn3, &mut var_tamb_dn4, &mut var_tamb_dn5, &mut var_tamb_dn6, &mut var_tamb_dn7, &mut var_tamb_dn8, &mut var_tamb_dn9, &mut var_tbeta, &mut var_tbeta_db0, &mut var_tbeta_db1, &mut var_tbeta_db2, &mut var_tbeta_db3, &mut var_tbeta_db4, &mut var_tbeta_db5, &mut var_tbeta_db6, &mut var_tbeta_db7, &mut var_tbeta_dn0, &mut var_tbeta_dn1, &mut var_tbeta_dn2, &mut var_tbeta_dn3, &mut var_tbeta_dn4, &mut var_tbeta_dn5, &mut var_tbeta_dn6, &mut var_tbeta_dn7, &mut var_tbeta_dn8, &mut var_tbeta_dn9, &mut var_tdev, &mut var_tdev_db0, &mut var_tdev_db1, &mut var_tdev_db2, &mut var_tdev_db3, &mut var_tdev_db4, &mut var_tdev_db5, &mut var_tdev_db6, &mut var_tdev_db7, &mut var_tdev_dn0, &mut var_tdev_dn1, &mut var_tdev_dn2, &mut var_tdev_dn3, &mut var_tdev_dn4, &mut var_tdev_dn5, &mut var_tdev_dn6, &mut var_tdev_dn7, &mut var_tdev_dn8, &mut var_tdev_dn9, &mut var_tnom, &mut var_vbc, &mut var_vbc_db0, &mut var_vbc_db1, &mut var_vbc_db2, &mut var_vbc_db3, &mut var_vbc_db4, &mut var_vbc_db5, &mut var_vbc_db6, &mut var_vbc_db7, &mut var_vbc_dn0, &mut var_vbc_dn1, &mut var_vbc_dn2, &mut var_vbc_dn3, &mut var_vbc_dn4, &mut var_vbc_dn5, &mut var_vbc_dn6, &mut var_vbc_dn7, &mut var_vbc_dn8, &mut var_vbc_dn9, &mut var_vt, &mut var_vt_db0, &mut var_vt_db1, &mut var_vt_db2, &mut var_vt_db3, &mut var_vt_db4, &mut var_vt_db5, &mut var_vt_db6, &mut var_vt_db7, &mut var_vt_dn0, &mut var_vt_dn1, &mut var_vt_dn2, &mut var_vt_dn3, &mut var_vt_dn4, &mut var_vt_dn5, &mut var_vt_dn6, &mut var_vt_dn7, &mut var_vt_dn8, &mut var_vt_dn9, &mut var_weff);
        Self::stamp_transient_block_1(p, var_rt, var_rt_db0, var_rt_db1, var_rt_db2, var_rt_db3, var_rt_db4, var_rt_db5, var_rt_db6, var_rt_db7, var_rt_dn0, var_rt_dn1, var_rt_dn2, var_rt_dn3, var_rt_dn4, var_rt_dn5, var_rt_dn6, var_rt_dn7, var_rt_dn8, var_rt_dn9, var_tdev, var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tnom, var_vt, var_vt_db0, var_vt_db1, var_vt_db2, var_vt_db3, var_vt_db4, var_vt_db5, var_vt_db6, var_vt_db7, var_vt_dn0, var_vt_dn1, var_vt_dn2, var_vt_dn3, var_vt_dn4, var_vt_dn5, var_vt_dn6, var_vt_dn7, var_vt_dn8, var_vt_dn9, &mut var_arg0, &mut var_arg0_db0, &mut var_arg0_db1, &mut var_arg0_db2, &mut var_arg0_db3, &mut var_arg0_db4, &mut var_arg0_db5, &mut var_arg0_db6, &mut var_arg0_db7, &mut var_arg0_dn0, &mut var_arg0_dn1, &mut var_arg0_dn2, &mut var_arg0_dn3, &mut var_arg0_dn4, &mut var_arg0_dn5, &mut var_arg0_dn6, &mut var_arg0_dn7, &mut var_arg0_dn8, &mut var_arg0_dn9, &mut var_cjc_i, &mut var_cje_i, &mut var_cje_t, &mut var_cje_t_db0, &mut var_cje_t_db1, &mut var_cje_t_db2, &mut var_cje_t_db3, &mut var_cje_t_db4, &mut var_cje_t_db5, &mut var_cje_t_db6, &mut var_cje_t_db7, &mut var_cje_t_dn0, &mut var_cje_t_dn1, &mut var_cje_t_dn2, &mut var_cje_t_dn3, &mut var_cje_t_dn4, &mut var_cje_t_dn5, &mut var_cje_t_dn6, &mut var_cje_t_dn7, &mut var_cje_t_dn8, &mut var_cje_t_dn9, &mut var_cjs_i, &mut var_cjt, &mut var_cjt_db0, &mut var_cjt_db1, &mut var_cjt_db2, &mut var_cjt_db3, &mut var_cjt_db4, &mut var_cjt_db5, &mut var_cjt_db6, &mut var_cjt_db7, &mut var_cjt_dn0, &mut var_cjt_dn1, &mut var_cjt_dn2, &mut var_cjt_dn3, &mut var_cjt_dn4, &mut var_cjt_dn5, &mut var_cjt_dn6, &mut var_cjt_dn7, &mut var_cjt_dn8, &mut var_cjt_dn9, &mut var_egfet, &mut var_egfet_db0, &mut var_egfet_db1, &mut var_egfet_db2, &mut var_egfet_db3, &mut var_egfet_db4, &mut var_egfet_db5, &mut var_egfet_db6, &mut var_egfet_db7, &mut var_egfet_dn0, &mut var_egfet_dn1, &mut var_egfet_dn2, &mut var_egfet_dn3, &mut var_egfet_dn4, &mut var_egfet_dn5, &mut var_egfet_dn6, &mut var_egfet_dn7, &mut var_egfet_dn8, &mut var_egfet_dn9, &mut var_fact1, &mut var_fact2, &mut var_fact2_db0, &mut var_fact2_db1, &mut var_fact2_db2, &mut var_fact2_db3, &mut var_fact2_db4, &mut var_fact2_db5, &mut var_fact2_db6, &mut var_fact2_db7, &mut var_fact2_dn0, &mut var_fact2_dn1, &mut var_fact2_dn2, &mut var_fact2_dn3, &mut var_fact2_dn4, &mut var_fact2_dn5, &mut var_fact2_dn6, &mut var_fact2_dn7, &mut var_fact2_dn8, &mut var_fact2_dn9, &mut var_gmanew, &mut var_gmanew_db0, &mut var_gmanew_db1, &mut var_gmanew_db2, &mut var_gmanew_db3, &mut var_gmanew_db4, &mut var_gmanew_db5, &mut var_gmanew_db6, &mut var_gmanew_db7, &mut var_gmanew_dn0, &mut var_gmanew_dn1, &mut var_gmanew_dn2, &mut var_gmanew_dn3, &mut var_gmanew_dn4, &mut var_gmanew_dn5, &mut var_gmanew_dn6, &mut var_gmanew_dn7, &mut var_gmanew_dn8, &mut var_gmanew_dn9, &mut var_gmaold, &mut var_gmaold_db0, &mut var_gmaold_db1, &mut var_gmaold_db2, &mut var_gmaold_db3, &mut var_gmaold_db4, &mut var_gmaold_db5, &mut var_gmaold_db6, &mut var_gmaold_db7, &mut var_gmaold_dn0, &mut var_gmaold_dn1, &mut var_gmaold_dn2, &mut var_gmaold_dn3, &mut var_gmaold_dn4, &mut var_gmaold_dn5, &mut var_gmaold_dn6, &mut var_gmaold_dn7, &mut var_gmaold_dn8, &mut var_gmaold_dn9, &mut var_ijbvc_t, &mut var_ijbvc_t_db0, &mut var_ijbvc_t_db1, &mut var_ijbvc_t_db2, &mut var_ijbvc_t_db3, &mut var_ijbvc_t_db4, &mut var_ijbvc_t_db5, &mut var_ijbvc_t_db6, &mut var_ijbvc_t_db7, &mut var_ijbvc_t_dn0, &mut var_ijbvc_t_dn1, &mut var_ijbvc_t_dn2, &mut var_ijbvc_t_dn3, &mut var_ijbvc_t_dn4, &mut var_ijbvc_t_dn5, &mut var_ijbvc_t_dn6, &mut var_ijbvc_t_dn7, &mut var_ijbvc_t_dn8, &mut var_ijbvc_t_dn9, &mut var_pbfact, &mut var_pbfact_db0, &mut var_pbfact_db1, &mut var_pbfact_db2, &mut var_pbfact_db3, &mut var_pbfact_db4, &mut var_pbfact_db5, &mut var_pbfact_db6, &mut var_pbfact_db7, &mut var_pbfact_dn0, &mut var_pbfact_dn1, &mut var_pbfact_dn2, &mut var_pbfact_dn3, &mut var_pbfact_dn4, &mut var_pbfact_dn5, &mut var_pbfact_dn6, &mut var_pbfact_dn7, &mut var_pbfact_dn8, &mut var_pbfact_dn9, &mut var_pbo, &mut var_pbo_db0, &mut var_pbo_db1, &mut var_pbo_db2, &mut var_pbo_db3, &mut var_pbo_db4, &mut var_pbo_db5, &mut var_pbo_db6, &mut var_pbo_db7, &mut var_pbo_dn0, &mut var_pbo_dn1, &mut var_pbo_dn2, &mut var_pbo_dn3, &mut var_pbo_dn4, &mut var_pbo_dn5, &mut var_pbo_dn6, &mut var_pbo_dn7, &mut var_pbo_dn8, &mut var_pbo_dn9, &mut var_theexp_t, &mut var_theexp_t_db0, &mut var_theexp_t_db1, &mut var_theexp_t_db2, &mut var_theexp_t_db3, &mut var_theexp_t_db4, &mut var_theexp_t_db5, &mut var_theexp_t_db6, &mut var_theexp_t_db7, &mut var_theexp_t_dn0, &mut var_theexp_t_dn1, &mut var_theexp_t_dn2, &mut var_theexp_t_dn3, &mut var_theexp_t_dn4, &mut var_theexp_t_dn5, &mut var_theexp_t_dn6, &mut var_theexp_t_dn7, &mut var_theexp_t_dn8, &mut var_theexp_t_dn9, &mut var_vjc_t, &mut var_vjc_t_db0, &mut var_vjc_t_db1, &mut var_vjc_t_db2, &mut var_vjc_t_db3, &mut var_vjc_t_db4, &mut var_vjc_t_db5, &mut var_vjc_t_db6, &mut var_vjc_t_db7, &mut var_vjc_t_dn0, &mut var_vjc_t_dn1, &mut var_vjc_t_dn2, &mut var_vjc_t_dn3, &mut var_vjc_t_dn4, &mut var_vjc_t_dn5, &mut var_vjc_t_dn6, &mut var_vjc_t_dn7, &mut var_vjc_t_dn8, &mut var_vjc_t_dn9, &mut var_vje_t, &mut var_vje_t_db0, &mut var_vje_t_db1, &mut var_vje_t_db2, &mut var_vje_t_db3, &mut var_vje_t_db4, &mut var_vje_t_db5, &mut var_vje_t_db6, &mut var_vje_t_db7, &mut var_vje_t_dn0, &mut var_vje_t_dn1, &mut var_vje_t_dn2, &mut var_vje_t_dn3, &mut var_vje_t_dn4, &mut var_vje_t_dn5, &mut var_vje_t_dn6, &mut var_vje_t_dn7, &mut var_vje_t_dn8, &mut var_vje_t_dn9);
        Self::stamp_transient_block_2(ctx, p, nodes, var_bvr_t, var_bvr_t_db0, var_bvr_t_db1, var_bvr_t_db2, var_bvr_t_db3, var_bvr_t_db4, var_bvr_t_db5, var_bvr_t_db6, var_bvr_t_db7, var_bvr_t_dn0, var_bvr_t_dn1, var_bvr_t_dn2, var_bvr_t_dn3, var_bvr_t_dn4, var_bvr_t_dn5, var_bvr_t_dn6, var_bvr_t_dn7, var_bvr_t_dn8, var_bvr_t_dn9, var_cjs_i, var_is_t, var_tdev, var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tnom, var_vjc_t, var_vjc_t_db0, var_vjc_t_db1, var_vjc_t_db2, var_vjc_t_db3, var_vjc_t_db4, var_vjc_t_db5, var_vjc_t_db6, var_vjc_t_db7, var_vjc_t_dn0, var_vjc_t_dn1, var_vjc_t_dn2, var_vjc_t_dn3, var_vjc_t_dn4, var_vjc_t_dn5, var_vjc_t_dn6, var_vjc_t_dn7, var_vjc_t_dn8, var_vjc_t_dn9, var_vt, var_vt_db0, var_vt_db1, var_vt_db2, var_vt_db3, var_vt_db4, var_vt_db5, var_vt_db6, var_vt_db7, var_vt_dn0, var_vt_dn1, var_vt_dn2, var_vt_dn3, var_vt_dn4, var_vt_dn5, var_vt_dn6, var_vt_dn7, var_vt_dn8, var_vt_dn9, &mut var_arg, &mut var_arg0, &mut var_arg0_db0, &mut var_arg0_db1, &mut var_arg0_db2, &mut var_arg0_db3, &mut var_arg0_db4, &mut var_arg0_db5, &mut var_arg0_db6, &mut var_arg0_db7, &mut var_arg0_dn0, &mut var_arg0_dn1, &mut var_arg0_dn2, &mut var_arg0_dn3, &mut var_arg0_dn4, &mut var_arg0_dn5, &mut var_arg0_dn6, &mut var_arg0_dn7, &mut var_arg0_dn8, &mut var_arg0_dn9, &mut var_arg_db0, &mut var_arg_db1, &mut var_arg_db2, &mut var_arg_db3, &mut var_arg_db4, &mut var_arg_db5, &mut var_arg_db6, &mut var_arg_db7, &mut var_arg_dn0, &mut var_arg_dn1, &mut var_arg_dn2, &mut var_arg_dn3, &mut var_arg_dn4, &mut var_arg_dn5, &mut var_arg_dn6, &mut var_arg_dn7, &mut var_arg_dn8, &mut var_arg_dn9, &mut var_argbv, &mut var_argbv_db0, &mut var_argbv_db1, &mut var_argbv_db2, &mut var_argbv_db3, &mut var_argbv_db4, &mut var_argbv_db5, &mut var_argbv_db6, &mut var_argbv_db7, &mut var_argbv_dn0, &mut var_argbv_dn1, &mut var_argbv_dn2, &mut var_argbv_dn3, &mut var_argbv_dn4, &mut var_argbv_dn5, &mut var_argbv_dn6, &mut var_argbv_dn7, &mut var_argbv_dn8, &mut var_argbv_dn9, &mut var_cjc_t, &mut var_cjc_t_db0, &mut var_cjc_t_db1, &mut var_cjc_t_db2, &mut var_cjc_t_db3, &mut var_cjc_t_db4, &mut var_cjc_t_db5, &mut var_cjc_t_db6, &mut var_cjc_t_db7, &mut var_cjc_t_dn0, &mut var_cjc_t_dn1, &mut var_cjc_t_dn2, &mut var_cjc_t_dn3, &mut var_cjc_t_dn4, &mut var_cjc_t_dn5, &mut var_cjc_t_dn6, &mut var_cjc_t_dn7, &mut var_cjc_t_dn8, &mut var_cjc_t_dn9, &mut var_cjs_t, &mut var_cjs_t_db0, &mut var_cjs_t_db1, &mut var_cjs_t_db2, &mut var_cjs_t_db3, &mut var_cjs_t_db4, &mut var_cjs_t_db5, &mut var_cjs_t_db6, &mut var_cjs_t_db7, &mut var_cjs_t_dn0, &mut var_cjs_t_dn1, &mut var_cjs_t_dn2, &mut var_cjs_t_dn3, &mut var_cjs_t_dn4, &mut var_cjs_t_dn5, &mut var_cjs_t_dn6, &mut var_cjs_t_dn7, &mut var_cjs_t_dn8, &mut var_cjs_t_dn9, &mut var_cjt, &mut var_cjt_db0, &mut var_cjt_db1, &mut var_cjt_db2, &mut var_cjt_db3, &mut var_cjt_db4, &mut var_cjt_db5, &mut var_cjt_db6, &mut var_cjt_db7, &mut var_cjt_dn0, &mut var_cjt_dn1, &mut var_cjt_dn2, &mut var_cjt_dn3, &mut var_cjt_dn4, &mut var_cjt_dn5, &mut var_cjt_dn6, &mut var_cjt_dn7, &mut var_cjt_dn8, &mut var_cjt_dn9, &mut var_egfet, &mut var_egfet_db0, &mut var_egfet_db1, &mut var_egfet_db2, &mut var_egfet_db3, &mut var_egfet_db4, &mut var_egfet_db5, &mut var_egfet_db6, &mut var_egfet_db7, &mut var_egfet_dn0, &mut var_egfet_dn1, &mut var_egfet_dn2, &mut var_egfet_dn3, &mut var_egfet_dn4, &mut var_egfet_dn5, &mut var_egfet_dn6, &mut var_egfet_dn7, &mut var_egfet_dn8, &mut var_egfet_dn9, &mut var_fact1, &mut var_fact2, &mut var_fact2_db0, &mut var_fact2_db1, &mut var_fact2_db2, &mut var_fact2_db3, &mut var_fact2_db4, &mut var_fact2_db5, &mut var_fact2_db6, &mut var_fact2_db7, &mut var_fact2_dn0, &mut var_fact2_dn1, &mut var_fact2_dn2, &mut var_fact2_dn3, &mut var_fact2_dn4, &mut var_fact2_dn5, &mut var_fact2_dn6, &mut var_fact2_dn7, &mut var_fact2_dn8, &mut var_fact2_dn9, &mut var_gmanew, &mut var_gmanew_db0, &mut var_gmanew_db1, &mut var_gmanew_db2, &mut var_gmanew_db3, &mut var_gmanew_db4, &mut var_gmanew_db5, &mut var_gmanew_db6, &mut var_gmanew_db7, &mut var_gmanew_dn0, &mut var_gmanew_dn1, &mut var_gmanew_dn2, &mut var_gmanew_dn3, &mut var_gmanew_dn4, &mut var_gmanew_dn5, &mut var_gmanew_dn6, &mut var_gmanew_dn7, &mut var_gmanew_dn8, &mut var_gmanew_dn9, &mut var_gmaold, &mut var_gmaold_db0, &mut var_gmaold_db1, &mut var_gmaold_db2, &mut var_gmaold_db3, &mut var_gmaold_db4, &mut var_gmaold_db5, &mut var_gmaold_db6, &mut var_gmaold_db7, &mut var_gmaold_dn0, &mut var_gmaold_dn1, &mut var_gmaold_dn2, &mut var_gmaold_dn3, &mut var_gmaold_dn4, &mut var_gmaold_dn5, &mut var_gmaold_dn6, &mut var_gmaold_dn7, &mut var_gmaold_dn8, &mut var_gmaold_dn9, &mut var_guard3, &mut var_pbfact, &mut var_pbfact_db0, &mut var_pbfact_db1, &mut var_pbfact_db2, &mut var_pbfact_db3, &mut var_pbfact_db4, &mut var_pbfact_db5, &mut var_pbfact_db6, &mut var_pbfact_db7, &mut var_pbfact_dn0, &mut var_pbfact_dn1, &mut var_pbfact_dn2, &mut var_pbfact_dn3, &mut var_pbfact_dn4, &mut var_pbfact_dn5, &mut var_pbfact_dn6, &mut var_pbfact_dn7, &mut var_pbfact_dn8, &mut var_pbfact_dn9, &mut var_pbo, &mut var_pbo_db0, &mut var_pbo_db1, &mut var_pbo_db2, &mut var_pbo_db3, &mut var_pbo_db4, &mut var_pbo_db5, &mut var_pbo_db6, &mut var_pbo_db7, &mut var_pbo_dn0, &mut var_pbo_dn1, &mut var_pbo_dn2, &mut var_pbo_dn3, &mut var_pbo_dn4, &mut var_pbo_dn5, &mut var_pbo_dn6, &mut var_pbo_dn7, &mut var_pbo_dn8, &mut var_pbo_dn9, &mut var_ttype, &mut var_vbbi, &mut var_vbbi_db0, &mut var_vbbi_db1, &mut var_vbbi_db2, &mut var_vbbi_db3, &mut var_vbbi_db4, &mut var_vbbi_db5, &mut var_vbbi_db6, &mut var_vbbi_db7, &mut var_vbbi_dn0, &mut var_vbbi_dn1, &mut var_vbbi_dn2, &mut var_vbbi_dn3, &mut var_vbbi_dn4, &mut var_vbbi_dn5, &mut var_vbbi_dn6, &mut var_vbbi_dn7, &mut var_vbbi_dn8, &mut var_vbbi_dn9, &mut var_vbci, &mut var_vbci_db0, &mut var_vbci_db1, &mut var_vbci_db2, &mut var_vbci_db3, &mut var_vbci_db4, &mut var_vbci_db5, &mut var_vbci_db6, &mut var_vbci_db7, &mut var_vbci_dn0, &mut var_vbci_dn1, &mut var_vbci_dn2, &mut var_vbci_dn3, &mut var_vbci_dn4, &mut var_vbci_dn5, &mut var_vbci_dn6, &mut var_vbci_dn7, &mut var_vbci_dn8, &mut var_vbci_dn9, &mut var_vbici, &mut var_vbici_db0, &mut var_vbici_db1, &mut var_vbici_db2, &mut var_vbici_db3, &mut var_vbici_db4, &mut var_vbici_db5, &mut var_vbici_db6, &mut var_vbici_db7, &mut var_vbici_dn0, &mut var_vbici_dn1, &mut var_vbici_dn2, &mut var_vbici_dn3, &mut var_vbici_dn4, &mut var_vbici_dn5, &mut var_vbici_dn6, &mut var_vbici_dn7, &mut var_vbici_dn8, &mut var_vbici_dn9, &mut var_vbiei, &mut var_vbiei_db0, &mut var_vbiei_db1, &mut var_vbiei_db2, &mut var_vbiei_db3, &mut var_vbiei_db4, &mut var_vbiei_db5, &mut var_vbiei_db6, &mut var_vbiei_db7, &mut var_vbiei_dn0, &mut var_vbiei_dn1, &mut var_vbiei_dn2, &mut var_vbiei_dn3, &mut var_vbiei_dn4, &mut var_vbiei_dn5, &mut var_vbiei_dn6, &mut var_vbiei_dn7, &mut var_vbiei_dn8, &mut var_vbiei_dn9, &mut var_veci, &mut var_veci_db0, &mut var_veci_db1, &mut var_veci_db2, &mut var_veci_db3, &mut var_veci_db4, &mut var_veci_db5, &mut var_veci_db6, &mut var_veci_db7, &mut var_veci_dn0, &mut var_veci_dn1, &mut var_veci_dn2, &mut var_veci_dn3, &mut var_veci_dn4, &mut var_veci_dn5, &mut var_veci_dn6, &mut var_veci_dn7, &mut var_veci_dn8, &mut var_veci_dn9, &mut var_veei, &mut var_veei_db0, &mut var_veei_db1, &mut var_veei_db2, &mut var_veei_db3, &mut var_veei_db4, &mut var_veei_db5, &mut var_veei_db6, &mut var_veei_db7, &mut var_veei_dn0, &mut var_veei_dn1, &mut var_veei_dn2, &mut var_veei_dn3, &mut var_veei_dn4, &mut var_veei_dn5, &mut var_veei_dn6, &mut var_veei_dn7, &mut var_veei_dn8, &mut var_veei_dn9, &mut var_vjs_t, &mut var_vjs_t_db0, &mut var_vjs_t_db1, &mut var_vjs_t_db2, &mut var_vjs_t_db3, &mut var_vjs_t_db4, &mut var_vjs_t_db5, &mut var_vjs_t_db6, &mut var_vjs_t_db7, &mut var_vjs_t_dn0, &mut var_vjs_t_dn1, &mut var_vjs_t_dn2, &mut var_vjs_t_dn3, &mut var_vjs_t_dn4, &mut var_vjs_t_dn5, &mut var_vjs_t_dn6, &mut var_vjs_t_dn7, &mut var_vjs_t_dn8, &mut var_vjs_t_dn9);
        Self::stamp_transient_block_3(p, var_argbv, var_argbv_db0, var_argbv_db1, var_argbv_db2, var_argbv_db3, var_argbv_db4, var_argbv_db5, var_argbv_db6, var_argbv_db7, var_argbv_dn0, var_argbv_dn1, var_argbv_dn2, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6, var_argbv_dn7, var_argbv_dn8, var_argbv_dn9, var_bvr_t, var_bvr_t_db0, var_bvr_t_db1, var_bvr_t_db2, var_bvr_t_db3, var_bvr_t_db4, var_bvr_t_db5, var_bvr_t_db6, var_bvr_t_db7, var_bvr_t_dn0, var_bvr_t_dn1, var_bvr_t_dn2, var_bvr_t_dn3, var_bvr_t_dn4, var_bvr_t_dn5, var_bvr_t_dn6, var_bvr_t_dn7, var_bvr_t_dn8, var_bvr_t_dn9, var_guard3, var_ijbv_t, var_ijbv_t_db0, var_ijbv_t_db1, var_ijbv_t_db2, var_ijbv_t_db3, var_ijbv_t_db4, var_ijbv_t_db5, var_ijbv_t_db6, var_ijbv_t_db7, var_ijbv_t_dn0, var_ijbv_t_dn1, var_ijbv_t_dn2, var_ijbv_t_dn3, var_ijbv_t_dn4, var_ijbv_t_dn5, var_ijbv_t_dn6, var_ijbv_t_dn7, var_ijbv_t_dn8, var_ijbv_t_dn9, var_is_t, var_is_t_db0, var_is_t_db1, var_is_t_db2, var_is_t_db3, var_is_t_db4, var_is_t_db5, var_is_t_db6, var_is_t_db7, var_is_t_dn0, var_is_t_dn1, var_is_t_dn2, var_is_t_dn3, var_is_t_dn4, var_is_t_dn5, var_is_t_dn6, var_is_t_dn7, var_is_t_dn8, var_is_t_dn9, var_isr_t, var_isr_t_db0, var_isr_t_db1, var_isr_t_db2, var_isr_t_db3, var_isr_t_db4, var_isr_t_db5, var_isr_t_db6, var_isr_t_db7, var_isr_t_dn0, var_isr_t_dn1, var_isr_t_dn2, var_isr_t_dn3, var_isr_t_dn4, var_isr_t_dn5, var_isr_t_dn6, var_isr_t_dn7, var_isr_t_dn8, var_isr_t_dn9, var_theexp_t, var_theexp_t_db0, var_theexp_t_db1, var_theexp_t_db2, var_theexp_t_db3, var_theexp_t_db4, var_theexp_t_db5, var_theexp_t_db6, var_theexp_t_db7, var_theexp_t_dn0, var_theexp_t_dn1, var_theexp_t_dn2, var_theexp_t_dn3, var_theexp_t_dn4, var_theexp_t_dn5, var_theexp_t_dn6, var_theexp_t_dn7, var_theexp_t_dn8, var_theexp_t_dn9, var_vbiei, var_vbiei_db0, var_vbiei_db1, var_vbiei_db2, var_vbiei_db3, var_vbiei_db4, var_vbiei_db5, var_vbiei_db6, var_vbiei_db7, var_vbiei_dn0, var_vbiei_dn1, var_vbiei_dn2, var_vbiei_dn3, var_vbiei_dn4, var_vbiei_dn5, var_vbiei_dn6, var_vbiei_dn7, var_vbiei_dn8, var_vbiei_dn9, var_vt, var_vt_db0, var_vt_db1, var_vt_db2, var_vt_db3, var_vt_db4, var_vt_db5, var_vt_db6, var_vt_db7, var_vt_dn0, var_vt_dn1, var_vt_dn2, var_vt_dn3, var_vt_dn4, var_vt_dn5, var_vt_dn6, var_vt_dn7, var_vt_dn8, var_vt_dn9, &mut var_arg, &mut var_arg_db0, &mut var_arg_db1, &mut var_arg_db2, &mut var_arg_db3, &mut var_arg_db4, &mut var_arg_db5, &mut var_arg_db6, &mut var_arg_db7, &mut var_arg_dn0, &mut var_arg_dn1, &mut var_arg_dn2, &mut var_arg_dn3, &mut var_arg_dn4, &mut var_arg_dn5, &mut var_arg_dn6, &mut var_arg_dn7, &mut var_arg_dn8, &mut var_arg_dn9, &mut var_argbvvt, &mut var_argbvvt_db0, &mut var_argbvvt_db1, &mut var_argbvvt_db2, &mut var_argbvvt_db3, &mut var_argbvvt_db4, &mut var_argbvvt_db5, &mut var_argbvvt_db6, &mut var_argbvvt_db7, &mut var_argbvvt_dn0, &mut var_argbvvt_dn1, &mut var_argbvvt_dn2, &mut var_argbvvt_dn3, &mut var_argbvvt_dn4, &mut var_argbvvt_dn5, &mut var_argbvvt_dn6, &mut var_argbvvt_dn7, &mut var_argbvvt_dn8, &mut var_argbvvt_dn9, &mut var_guard4, &mut var_guard5, &mut var_guard6, &mut var_ifwd, &mut var_ifwd_db0, &mut var_ifwd_db1, &mut var_ifwd_db2, &mut var_ifwd_db3, &mut var_ifwd_db4, &mut var_ifwd_db5, &mut var_ifwd_db6, &mut var_ifwd_db7, &mut var_ifwd_dn0, &mut var_ifwd_dn1, &mut var_ifwd_dn2, &mut var_ifwd_dn3, &mut var_ifwd_dn4, &mut var_ifwd_dn5, &mut var_ifwd_dn6, &mut var_ifwd_dn7, &mut var_ifwd_dn8, &mut var_ifwd_dn9, &mut var_itrev, &mut var_itrev_db0, &mut var_itrev_db1, &mut var_itrev_db2, &mut var_itrev_db3, &mut var_itrev_db4, &mut var_itrev_db5, &mut var_itrev_db6, &mut var_itrev_db7, &mut var_itrev_dn0, &mut var_itrev_dn1, &mut var_itrev_dn2, &mut var_itrev_dn3, &mut var_itrev_dn4, &mut var_itrev_dn5, &mut var_itrev_dn6, &mut var_itrev_dn7, &mut var_itrev_dn8, &mut var_itrev_dn9, &mut var_le, &mut var_le_db0, &mut var_le_db1, &mut var_le_db2, &mut var_le_db3, &mut var_le_db4, &mut var_le_db5, &mut var_le_db6, &mut var_le_db7, &mut var_le_dn0, &mut var_le_dn1, &mut var_le_dn2, &mut var_le_dn3, &mut var_le_dn4, &mut var_le_dn5, &mut var_le_dn6, &mut var_le_dn7, &mut var_le_dn8, &mut var_le_dn9, &mut var_lebv, &mut var_lebv_db0, &mut var_lebv_db1, &mut var_lebv_db2, &mut var_lebv_db3, &mut var_lebv_db4, &mut var_lebv_db5, &mut var_lebv_db6, &mut var_lebv_db7, &mut var_lebv_dn0, &mut var_lebv_dn1, &mut var_lebv_dn2, &mut var_lebv_dn3, &mut var_lebv_dn4, &mut var_lebv_dn5, &mut var_lebv_dn6, &mut var_lebv_dn7, &mut var_lebv_dn8, &mut var_lebv_dn9, &mut var_t0, &mut var_t0_db0, &mut var_t0_db1, &mut var_t0_db2, &mut var_t0_db3, &mut var_t0_db4, &mut var_t0_db5, &mut var_t0_db6, &mut var_t0_db7, &mut var_t0_dn0, &mut var_t0_dn1, &mut var_t0_dn2, &mut var_t0_dn3, &mut var_t0_dn4, &mut var_t0_dn5, &mut var_t0_dn6, &mut var_t0_dn7, &mut var_t0_dn8, &mut var_t0_dn9);
        Self::stamp_transient_block_4(p, var_bvr_t, var_bvr_t_db0, var_bvr_t_db1, var_bvr_t_db2, var_bvr_t_db3, var_bvr_t_db4, var_bvr_t_db5, var_bvr_t_db6, var_bvr_t_db7, var_bvr_t_dn0, var_bvr_t_dn1, var_bvr_t_dn2, var_bvr_t_dn3, var_bvr_t_dn4, var_bvr_t_dn5, var_bvr_t_dn6, var_bvr_t_dn7, var_bvr_t_dn8, var_bvr_t_dn9, var_guard5, var_is_t, var_ise_t, var_ise_t_db0, var_ise_t_db1, var_ise_t_db2, var_ise_t_db3, var_ise_t_db4, var_ise_t_db5, var_ise_t_db6, var_ise_t_db7, var_ise_t_dn0, var_ise_t_dn1, var_ise_t_dn2, var_ise_t_dn3, var_ise_t_dn4, var_ise_t_dn5, var_ise_t_dn6, var_ise_t_dn7, var_ise_t_dn8, var_ise_t_dn9, var_theexp_t, var_vbici, var_vbici_db0, var_vbici_db1, var_vbici_db2, var_vbici_db3, var_vbici_db4, var_vbici_db5, var_vbici_db6, var_vbici_db7, var_vbici_dn0, var_vbici_dn1, var_vbici_dn2, var_vbici_dn3, var_vbici_dn4, var_vbici_dn5, var_vbici_dn6, var_vbici_dn7, var_vbici_dn8, var_vbici_dn9, var_vbiei, var_vbiei_db0, var_vbiei_db1, var_vbiei_db2, var_vbiei_db3, var_vbiei_db4, var_vbiei_db5, var_vbiei_db6, var_vbiei_db7, var_vbiei_dn0, var_vbiei_dn1, var_vbiei_dn2, var_vbiei_dn3, var_vbiei_dn4, var_vbiei_dn5, var_vbiei_dn6, var_vbiei_dn7, var_vbiei_dn8, var_vbiei_dn9, var_vt, var_vt_db0, var_vt_db1, var_vt_db2, var_vt_db3, var_vt_db4, var_vt_db5, var_vt_db6, var_vt_db7, var_vt_dn0, var_vt_dn1, var_vt_dn2, var_vt_dn3, var_vt_dn4, var_vt_dn5, var_vt_dn6, var_vt_dn7, var_vt_dn8, var_vt_dn9, &mut var_arg, &mut var_arg_db0, &mut var_arg_db1, &mut var_arg_db2, &mut var_arg_db3, &mut var_arg_db4, &mut var_arg_db5, &mut var_arg_db6, &mut var_arg_db7, &mut var_arg_dn0, &mut var_arg_dn1, &mut var_arg_dn2, &mut var_arg_dn3, &mut var_arg_dn4, &mut var_arg_dn5, &mut var_arg_dn6, &mut var_arg_dn7, &mut var_arg_dn8, &mut var_arg_dn9, &mut var_argbv, &mut var_argbv_db0, &mut var_argbv_db1, &mut var_argbv_db2, &mut var_argbv_db3, &mut var_argbv_db4, &mut var_argbv_db5, &mut var_argbv_db6, &mut var_argbv_db7, &mut var_argbv_dn0, &mut var_argbv_dn1, &mut var_argbv_dn2, &mut var_argbv_dn3, &mut var_argbv_dn4, &mut var_argbv_dn5, &mut var_argbv_dn6, &mut var_argbv_dn7, &mut var_argbv_dn8, &mut var_argbv_dn9, &mut var_argbvvt, &mut var_argbvvt_db0, &mut var_argbvvt_db1, &mut var_argbvvt_db2, &mut var_argbvvt_db3, &mut var_argbvvt_db4, &mut var_argbvvt_db5, &mut var_argbvvt_db6, &mut var_argbvvt_db7, &mut var_argbvvt_dn0, &mut var_argbvvt_dn1, &mut var_argbvvt_dn2, &mut var_argbvvt_dn3, &mut var_argbvvt_dn4, &mut var_argbvvt_dn5, &mut var_argbvvt_dn6, &mut var_argbvvt_dn7, &mut var_argbvvt_dn8, &mut var_argbvvt_dn9, &mut var_guard10, &mut var_guard7, &mut var_guard8, &mut var_guard9, &mut var_ibe2, &mut var_ibe2_db0, &mut var_ibe2_db1, &mut var_ibe2_db2, &mut var_ibe2_db3, &mut var_ibe2_db4, &mut var_ibe2_db5, &mut var_ibe2_db6, &mut var_ibe2_db7, &mut var_ibe2_dn0, &mut var_ibe2_dn1, &mut var_ibe2_dn2, &mut var_ibe2_dn3, &mut var_ibe2_dn4, &mut var_ibe2_dn5, &mut var_ibe2_dn6, &mut var_ibe2_dn7, &mut var_ibe2_dn8, &mut var_ibe2_dn9, &mut var_itrev, &mut var_itrev_db0, &mut var_itrev_db1, &mut var_itrev_db2, &mut var_itrev_db3, &mut var_itrev_db4, &mut var_itrev_db5, &mut var_itrev_db6, &mut var_itrev_db7, &mut var_itrev_dn0, &mut var_itrev_dn1, &mut var_itrev_dn2, &mut var_itrev_dn3, &mut var_itrev_dn4, &mut var_itrev_dn5, &mut var_itrev_dn6, &mut var_itrev_dn7, &mut var_itrev_dn8, &mut var_itrev_dn9, &mut var_le, &mut var_le_db0, &mut var_le_db1, &mut var_le_db2, &mut var_le_db3, &mut var_le_db4, &mut var_le_db5, &mut var_le_db6, &mut var_le_db7, &mut var_le_dn0, &mut var_le_dn1, &mut var_le_dn2, &mut var_le_dn3, &mut var_le_dn4, &mut var_le_dn5, &mut var_le_dn6, &mut var_le_dn7, &mut var_le_dn8, &mut var_le_dn9, &mut var_lebv, &mut var_lebv_db0, &mut var_lebv_db1, &mut var_lebv_db2, &mut var_lebv_db3, &mut var_lebv_db4, &mut var_lebv_db5, &mut var_lebv_db6, &mut var_lebv_db7, &mut var_lebv_dn0, &mut var_lebv_dn1, &mut var_lebv_dn2, &mut var_lebv_dn3, &mut var_lebv_dn4, &mut var_lebv_dn5, &mut var_lebv_dn6, &mut var_lebv_dn7, &mut var_lebv_dn8, &mut var_lebv_dn9);
        Self::stamp_transient_block_5(p, var_bvr_t, var_bvr_t_db0, var_bvr_t_db1, var_bvr_t_db2, var_bvr_t_db3, var_bvr_t_db4, var_bvr_t_db5, var_bvr_t_db6, var_bvr_t_db7, var_bvr_t_dn0, var_bvr_t_dn1, var_bvr_t_dn2, var_bvr_t_dn3, var_bvr_t_dn4, var_bvr_t_dn5, var_bvr_t_dn6, var_bvr_t_dn7, var_bvr_t_dn8, var_bvr_t_dn9, var_guard10, var_guard9, var_ijbvc_t, var_ijbvc_t_db0, var_ijbvc_t_db1, var_ijbvc_t_db2, var_ijbvc_t_db3, var_ijbvc_t_db4, var_ijbvc_t_db5, var_ijbvc_t_db6, var_ijbvc_t_db7, var_ijbvc_t_dn0, var_ijbvc_t_dn1, var_ijbvc_t_dn2, var_ijbvc_t_dn3, var_ijbvc_t_dn4, var_ijbvc_t_dn5, var_ijbvc_t_dn6, var_ijbvc_t_dn7, var_ijbvc_t_dn8, var_ijbvc_t_dn9, var_is_t, var_is_t_db0, var_is_t_db1, var_is_t_db2, var_is_t_db3, var_is_t_db4, var_is_t_db5, var_is_t_db6, var_is_t_db7, var_is_t_dn0, var_is_t_dn1, var_is_t_dn2, var_is_t_dn3, var_is_t_dn4, var_is_t_dn5, var_is_t_dn6, var_is_t_dn7, var_is_t_dn8, var_is_t_dn9, var_isc_t, var_theexp_t, var_theexp_t_db0, var_theexp_t_db1, var_theexp_t_db2, var_theexp_t_db3, var_theexp_t_db4, var_theexp_t_db5, var_theexp_t_db6, var_theexp_t_db7, var_theexp_t_dn0, var_theexp_t_dn1, var_theexp_t_dn2, var_theexp_t_dn3, var_theexp_t_dn4, var_theexp_t_dn5, var_theexp_t_dn6, var_theexp_t_dn7, var_theexp_t_dn8, var_theexp_t_dn9, var_vbici, var_vbici_db0, var_vbici_db1, var_vbici_db2, var_vbici_db3, var_vbici_db4, var_vbici_db5, var_vbici_db6, var_vbici_db7, var_vbici_dn0, var_vbici_dn1, var_vbici_dn2, var_vbici_dn3, var_vbici_dn4, var_vbici_dn5, var_vbici_dn6, var_vbici_dn7, var_vbici_dn8, var_vbici_dn9, var_vt, var_vt_db0, var_vt_db1, var_vt_db2, var_vt_db3, var_vt_db4, var_vt_db5, var_vt_db6, var_vt_db7, var_vt_dn0, var_vt_dn1, var_vt_dn2, var_vt_dn3, var_vt_dn4, var_vt_dn5, var_vt_dn6, var_vt_dn7, var_vt_dn8, var_vt_dn9, &mut var_arg, &mut var_arg_db0, &mut var_arg_db1, &mut var_arg_db2, &mut var_arg_db3, &mut var_arg_db4, &mut var_arg_db5, &mut var_arg_db6, &mut var_arg_db7, &mut var_arg_dn0, &mut var_arg_dn1, &mut var_arg_dn2, &mut var_arg_dn3, &mut var_arg_dn4, &mut var_arg_dn5, &mut var_arg_dn6, &mut var_arg_dn7, &mut var_arg_dn8, &mut var_arg_dn9, &mut var_argbv, &mut var_argbv_db0, &mut var_argbv_db1, &mut var_argbv_db2, &mut var_argbv_db3, &mut var_argbv_db4, &mut var_argbv_db5, &mut var_argbv_db6, &mut var_argbv_db7, &mut var_argbv_dn0, &mut var_argbv_dn1, &mut var_argbv_dn2, &mut var_argbv_dn3, &mut var_argbv_dn4, &mut var_argbv_dn5, &mut var_argbv_dn6, &mut var_argbv_dn7, &mut var_argbv_dn8, &mut var_argbv_dn9, &mut var_argbvvt, &mut var_argbvvt_db0, &mut var_argbvvt_db1, &mut var_argbvvt_db2, &mut var_argbvvt_db3, &mut var_argbvvt_db4, &mut var_argbvvt_db5, &mut var_argbvvt_db6, &mut var_argbvvt_db7, &mut var_argbvvt_dn0, &mut var_argbvvt_dn1, &mut var_argbvvt_dn2, &mut var_argbvvt_dn3, &mut var_argbvvt_dn4, &mut var_argbvvt_dn5, &mut var_argbvvt_dn6, &mut var_argbvvt_dn7, &mut var_argbvvt_dn8, &mut var_argbvvt_dn9, &mut var_guard11, &mut var_guard12, &mut var_ibwd, &mut var_ibwd_db0, &mut var_ibwd_db1, &mut var_ibwd_db2, &mut var_ibwd_db3, &mut var_ibwd_db4, &mut var_ibwd_db5, &mut var_ibwd_db6, &mut var_ibwd_db7, &mut var_ibwd_dn0, &mut var_ibwd_dn1, &mut var_ibwd_dn2, &mut var_ibwd_dn3, &mut var_ibwd_dn4, &mut var_ibwd_dn5, &mut var_ibwd_dn6, &mut var_ibwd_dn7, &mut var_ibwd_dn8, &mut var_ibwd_dn9, &mut var_le, &mut var_le_db0, &mut var_le_db1, &mut var_le_db2, &mut var_le_db3, &mut var_le_db4, &mut var_le_db5, &mut var_le_db6, &mut var_le_db7, &mut var_le_dn0, &mut var_le_dn1, &mut var_le_dn2, &mut var_le_dn3, &mut var_le_dn4, &mut var_le_dn5, &mut var_le_dn6, &mut var_le_dn7, &mut var_le_dn8, &mut var_le_dn9, &mut var_lebv, &mut var_lebv_db0, &mut var_lebv_db1, &mut var_lebv_db2, &mut var_lebv_db3, &mut var_lebv_db4, &mut var_lebv_db5, &mut var_lebv_db6, &mut var_lebv_db7, &mut var_lebv_dn0, &mut var_lebv_dn1, &mut var_lebv_dn2, &mut var_lebv_dn3, &mut var_lebv_dn4, &mut var_lebv_dn5, &mut var_lebv_dn6, &mut var_lebv_dn7, &mut var_lebv_dn8, &mut var_lebv_dn9);
        Self::stamp_transient_block_6(ctx, p, nodes, var_argbv, var_argbv_db0, var_argbv_db1, var_argbv_db2, var_argbv_db3, var_argbv_db4, var_argbv_db5, var_argbv_db6, var_argbv_db7, var_argbv_dn0, var_argbv_dn1, var_argbv_dn2, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6, var_argbv_dn7, var_argbv_dn8, var_argbv_dn9, var_argbvvt, var_argbvvt_db0, var_argbvvt_db1, var_argbvvt_db2, var_argbvvt_db3, var_argbvvt_db4, var_argbvvt_db5, var_argbvvt_db6, var_argbvvt_db7, var_argbvvt_dn0, var_argbvvt_dn1, var_argbvvt_dn2, var_argbvvt_dn3, var_argbvvt_dn4, var_argbvvt_dn5, var_argbvvt_dn6, var_argbvvt_dn7, var_argbvvt_dn8, var_argbvvt_dn9, var_bf_t, var_bf_t_db0, var_bf_t_db1, var_bf_t_db2, var_bf_t_db3, var_bf_t_db4, var_bf_t_db5, var_bf_t_db6, var_bf_t_db7, var_bf_t_dn0, var_bf_t_dn1, var_bf_t_dn2, var_bf_t_dn3, var_bf_t_dn4, var_bf_t_dn5, var_bf_t_dn6, var_bf_t_dn7, var_bf_t_dn8, var_bf_t_dn9, var_br_t, var_br_t_db0, var_br_t_db1, var_br_t_db2, var_br_t_db3, var_br_t_db4, var_br_t_db5, var_br_t_db6, var_br_t_db7, var_br_t_dn0, var_br_t_dn1, var_br_t_dn2, var_br_t_dn3, var_br_t_dn4, var_br_t_dn5, var_br_t_dn6, var_br_t_dn7, var_br_t_dn8, var_br_t_dn9, var_guard11, var_ibe2, var_ibe2_db0, var_ibe2_db1, var_ibe2_db2, var_ibe2_db3, var_ibe2_db4, var_ibe2_db5, var_ibe2_db6, var_ibe2_db7, var_ibe2_dn0, var_ibe2_dn1, var_ibe2_dn2, var_ibe2_dn3, var_ibe2_dn4, var_ibe2_dn5, var_ibe2_dn6, var_ibe2_dn7, var_ibe2_dn8, var_ibe2_dn9, var_ibwd, var_ibwd_db0, var_ibwd_db1, var_ibwd_db2, var_ibwd_db3, var_ibwd_db4, var_ibwd_db5, var_ibwd_db6, var_ibwd_db7, var_ibwd_dn0, var_ibwd_dn1, var_ibwd_dn2, var_ibwd_dn3, var_ibwd_dn4, var_ibwd_dn5, var_ibwd_dn6, var_ibwd_dn7, var_ibwd_dn8, var_ibwd_dn9, var_ifwd, var_ifwd_db0, var_ifwd_db1, var_ifwd_db2, var_ifwd_db3, var_ifwd_db4, var_ifwd_db5, var_ifwd_db6, var_ifwd_db7, var_ifwd_dn0, var_ifwd_dn1, var_ifwd_dn2, var_ifwd_dn3, var_ifwd_dn4, var_ifwd_dn5, var_ifwd_dn6, var_ifwd_dn7, var_ifwd_dn8, var_ifwd_dn9, var_isc_t, var_isc_t_db0, var_isc_t_db1, var_isc_t_db2, var_isc_t_db3, var_isc_t_db4, var_isc_t_db5, var_isc_t_db6, var_isc_t_db7, var_isc_t_dn0, var_isc_t_dn1, var_isc_t_dn2, var_isc_t_dn3, var_isc_t_dn4, var_isc_t_dn5, var_isc_t_dn6, var_isc_t_dn7, var_isc_t_dn8, var_isc_t_dn9, var_itrev, var_itrev_db0, var_itrev_db1, var_itrev_db2, var_itrev_db3, var_itrev_db4, var_itrev_db5, var_itrev_db6, var_itrev_db7, var_itrev_dn0, var_itrev_dn1, var_itrev_dn2, var_itrev_dn3, var_itrev_dn4, var_itrev_dn5, var_itrev_dn6, var_itrev_dn7, var_itrev_dn8, var_itrev_dn9, var_le, var_le_db0, var_le_db1, var_le_db2, var_le_db3, var_le_db4, var_le_db5, var_le_db6, var_le_db7, var_le_dn0, var_le_dn1, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6, var_le_dn7, var_le_dn8, var_le_dn9, var_lnrt, var_lnrt_db0, var_lnrt_db1, var_lnrt_db2, var_lnrt_db3, var_lnrt_db4, var_lnrt_db5, var_lnrt_db6, var_lnrt_db7, var_lnrt_dn0, var_lnrt_dn1, var_lnrt_dn2, var_lnrt_dn3, var_lnrt_dn4, var_lnrt_dn5, var_lnrt_dn6, var_lnrt_dn7, var_lnrt_dn8, var_lnrt_dn9, var_oikr, var_ovaf, var_ovar, var_vbbi, var_vbbi_db0, var_vbbi_db1, var_vbbi_db2, var_vbbi_db3, var_vbbi_db4, var_vbbi_db5, var_vbbi_db6, var_vbbi_db7, var_vbbi_dn0, var_vbbi_dn1, var_vbbi_dn2, var_vbbi_dn3, var_vbbi_dn4, var_vbbi_dn5, var_vbbi_dn6, var_vbbi_dn7, var_vbbi_dn8, var_vbbi_dn9, var_vbici, var_vbici_db0, var_vbici_db1, var_vbici_db2, var_vbici_db3, var_vbici_db4, var_vbici_db5, var_vbici_db6, var_vbici_db7, var_vbici_dn0, var_vbici_dn1, var_vbici_dn2, var_vbici_dn3, var_vbici_dn4, var_vbici_dn5, var_vbici_dn6, var_vbici_dn7, var_vbici_dn8, var_vbici_dn9, var_vbiei, var_vbiei_db0, var_vbiei_db1, var_vbiei_db2, var_vbiei_db3, var_vbiei_db4, var_vbiei_db5, var_vbiei_db6, var_vbiei_db7, var_vbiei_dn0, var_vbiei_dn1, var_vbiei_dn2, var_vbiei_dn3, var_vbiei_dn4, var_vbiei_dn5, var_vbiei_dn6, var_vbiei_dn7, var_vbiei_dn8, var_vbiei_dn9, var_veei, var_veei_db0, var_veei_db1, var_veei_db2, var_veei_db3, var_veei_db4, var_veei_db5, var_veei_db6, var_veei_db7, var_veei_dn0, var_veei_dn1, var_veei_dn2, var_veei_dn3, var_veei_dn4, var_veei_dn5, var_veei_dn6, var_veei_dn7, var_veei_dn8, var_veei_dn9, &mut var_d_ratio, &mut var_d_ratio_db0, &mut var_d_ratio_db1, &mut var_d_ratio_db2, &mut var_d_ratio_db3, &mut var_d_ratio_db4, &mut var_d_ratio_db5, &mut var_d_ratio_db6, &mut var_d_ratio_db7, &mut var_d_ratio_dn0, &mut var_d_ratio_dn1, &mut var_d_ratio_dn2, &mut var_d_ratio_dn3, &mut var_d_ratio_dn4, &mut var_d_ratio_dn5, &mut var_d_ratio_dn6, &mut var_d_ratio_dn7, &mut var_d_ratio_dn8, &mut var_d_ratio_dn9, &mut var_dkqb, &mut var_dkqb_db0, &mut var_dkqb_db1, &mut var_dkqb_db2, &mut var_dkqb_db3, &mut var_dkqb_db4, &mut var_dkqb_db5, &mut var_dkqb_db6, &mut var_dkqb_db7, &mut var_dkqb_dn0, &mut var_dkqb_dn1, &mut var_dkqb_dn2, &mut var_dkqb_dn3, &mut var_dkqb_dn4, &mut var_dkqb_dn5, &mut var_dkqb_dn6, &mut var_dkqb_dn7, &mut var_dkqb_dn8, &mut var_dkqb_dn9, &mut var_ibc, &mut var_ibc2, &mut var_ibc2_db0, &mut var_ibc2_db1, &mut var_ibc2_db2, &mut var_ibc2_db3, &mut var_ibc2_db4, &mut var_ibc2_db5, &mut var_ibc2_db6, &mut var_ibc2_db7, &mut var_ibc2_dn0, &mut var_ibc2_dn1, &mut var_ibc2_dn2, &mut var_ibc2_dn3, &mut var_ibc2_dn4, &mut var_ibc2_dn5, &mut var_ibc2_dn6, &mut var_ibc2_dn7, &mut var_ibc2_dn8, &mut var_ibc2_dn9, &mut var_ibc_db0, &mut var_ibc_db1, &mut var_ibc_db2, &mut var_ibc_db3, &mut var_ibc_db4, &mut var_ibc_db5, &mut var_ibc_db6, &mut var_ibc_db7, &mut var_ibc_dn0, &mut var_ibc_dn1, &mut var_ibc_dn2, &mut var_ibc_dn3, &mut var_ibc_dn4, &mut var_ibc_dn5, &mut var_ibc_dn6, &mut var_ibc_dn7, &mut var_ibc_dn8, &mut var_ibc_dn9, &mut var_ibe, &mut var_ibe_db0, &mut var_ibe_db1, &mut var_ibe_db2, &mut var_ibe_db3, &mut var_ibe_db4, &mut var_ibe_db5, &mut var_ibe_db6, &mut var_ibe_db7, &mut var_ibe_dn0, &mut var_ibe_dn1, &mut var_ibe_dn2, &mut var_ibe_dn3, &mut var_ibe_dn4, &mut var_ibe_dn5, &mut var_ibe_dn6, &mut var_ibe_dn7, &mut var_ibe_dn8, &mut var_ibe_dn9, &mut var_ikq1, &mut var_ikq1_db0, &mut var_ikq1_db1, &mut var_ikq1_db2, &mut var_ikq1_db3, &mut var_ikq1_db4, &mut var_ikq1_db5, &mut var_ikq1_db6, &mut var_ikq1_db7, &mut var_ikq1_dn0, &mut var_ikq1_dn1, &mut var_ikq1_dn2, &mut var_ikq1_dn3, &mut var_ikq1_dn4, &mut var_ikq1_dn5, &mut var_ikq1_dn6, &mut var_ikq1_dn7, &mut var_ikq1_dn8, &mut var_ikq1_dn9, &mut var_ikqb, &mut var_ikqb_db0, &mut var_ikqb_db1, &mut var_ikqb_db2, &mut var_ikqb_db3, &mut var_ikqb_db4, &mut var_ikqb_db5, &mut var_ikqb_db6, &mut var_ikqb_db7, &mut var_ikqb_dn0, &mut var_ikqb_dn1, &mut var_ikqb_dn2, &mut var_ikqb_dn3, &mut var_ikqb_dn4, &mut var_ikqb_dn5, &mut var_ikqb_dn6, &mut var_ikqb_dn7, &mut var_ikqb_dn8, &mut var_ikqb_dn9, &mut var_itr, &mut var_itr_db0, &mut var_itr_db1, &mut var_itr_db2, &mut var_itr_db3, &mut var_itr_db4, &mut var_itr_db5, &mut var_itr_db6, &mut var_itr_db7, &mut var_itr_dn0, &mut var_itr_dn1, &mut var_itr_dn2, &mut var_itr_dn3, &mut var_itr_dn4, &mut var_itr_dn5, &mut var_itr_dn6, &mut var_itr_dn7, &mut var_itr_dn8, &mut var_itr_dn9, &mut var_itzf, &mut var_itzf_db0, &mut var_itzf_db1, &mut var_itzf_db2, &mut var_itzf_db3, &mut var_itzf_db4, &mut var_itzf_db5, &mut var_itzf_db6, &mut var_itzf_db7, &mut var_itzf_dn0, &mut var_itzf_dn1, &mut var_itzf_dn2, &mut var_itzf_dn3, &mut var_itzf_dn4, &mut var_itzf_dn5, &mut var_itzf_dn6, &mut var_itzf_dn7, &mut var_itzf_dn8, &mut var_itzf_dn9, &mut var_itzf_f, &mut var_itzf_f_db0, &mut var_itzf_f_db1, &mut var_itzf_f_db2, &mut var_itzf_f_db3, &mut var_itzf_f_db4, &mut var_itzf_f_db5, &mut var_itzf_f_db6, &mut var_itzf_f_db7, &mut var_itzf_f_dn0, &mut var_itzf_f_dn1, &mut var_itzf_f_dn2, &mut var_itzf_f_dn3, &mut var_itzf_f_dn4, &mut var_itzf_f_dn5, &mut var_itzf_f_dn6, &mut var_itzf_f_dn7, &mut var_itzf_f_dn8, &mut var_itzf_f_dn9, &mut var_kq2, &mut var_kq2_db0, &mut var_kq2_db1, &mut var_kq2_db2, &mut var_kq2_db3, &mut var_kq2_db4, &mut var_kq2_db5, &mut var_kq2_db6, &mut var_kq2_db7, &mut var_kq2_dn0, &mut var_kq2_dn1, &mut var_kq2_dn2, &mut var_kq2_dn3, &mut var_kq2_dn4, &mut var_kq2_dn5, &mut var_kq2_dn6, &mut var_kq2_dn7, &mut var_kq2_dn8, &mut var_kq2_dn9, &mut var_lebv, &mut var_lebv_db0, &mut var_lebv_db1, &mut var_lebv_db2, &mut var_lebv_db3, &mut var_lebv_db4, &mut var_lebv_db5, &mut var_lebv_db6, &mut var_lebv_db7, &mut var_lebv_dn0, &mut var_lebv_dn1, &mut var_lebv_dn2, &mut var_lebv_dn3, &mut var_lebv_dn4, &mut var_lebv_dn5, &mut var_lebv_dn6, &mut var_lebv_dn7, &mut var_lebv_dn8, &mut var_lebv_dn9, &mut var_oikf, &mut var_oikf_db0, &mut var_oikf_db1, &mut var_oikf_db2, &mut var_oikf_db3, &mut var_oikf_db4, &mut var_oikf_db5, &mut var_oikf_db6, &mut var_oikf_db7, &mut var_oikf_dn0, &mut var_oikf_dn1, &mut var_oikf_dn2, &mut var_oikf_dn3, &mut var_oikf_dn4, &mut var_oikf_dn5, &mut var_oikf_dn6, &mut var_oikf_dn7, &mut var_oikf_dn8, &mut var_oikf_dn9, &mut var_rb, &mut var_rb_db0, &mut var_rb_db1, &mut var_rb_db2, &mut var_rb_db3, &mut var_rb_db4, &mut var_rb_db5, &mut var_rb_db6, &mut var_rb_db7, &mut var_rb_dn0, &mut var_rb_dn1, &mut var_rb_dn2, &mut var_rb_dn3, &mut var_rb_dn4, &mut var_rb_dn5, &mut var_rb_dn6, &mut var_rb_dn7, &mut var_rb_dn8, &mut var_rb_dn9, &mut var_rc, &mut var_rc_db0, &mut var_rc_db1, &mut var_rc_db2, &mut var_rc_db3, &mut var_rc_db4, &mut var_rc_db5, &mut var_rc_db6, &mut var_rc_db7, &mut var_rc_dn0, &mut var_rc_dn1, &mut var_rc_dn2, &mut var_rc_dn3, &mut var_rc_dn4, &mut var_rc_dn5, &mut var_rc_dn6, &mut var_rc_dn7, &mut var_rc_dn8, &mut var_rc_dn9, &mut var_vbesat, &mut var_vbesat_db0, &mut var_vbesat_db1, &mut var_vbesat_db2, &mut var_vbesat_db3, &mut var_vbesat_db4, &mut var_vbesat_db5, &mut var_vbesat_db6, &mut var_vbesat_db7, &mut var_vbesat_dn0, &mut var_vbesat_dn1, &mut var_vbesat_dn2, &mut var_vbesat_dn3, &mut var_vbesat_dn4, &mut var_vbesat_dn5, &mut var_vbesat_dn6, &mut var_vbesat_dn7, &mut var_vbesat_dn8, &mut var_vbesat_dn9, &mut var_veesat, &mut var_veesat_db0, &mut var_veesat_db1, &mut var_veesat_db2, &mut var_veesat_db3, &mut var_veesat_db4, &mut var_veesat_db5, &mut var_veesat_db6, &mut var_veesat_db7, &mut var_veesat_dn0, &mut var_veesat_dn1, &mut var_veesat_dn2, &mut var_veesat_dn3, &mut var_veesat_dn4, &mut var_veesat_dn5, &mut var_veesat_dn6, &mut var_veesat_dn7, &mut var_veesat_dn8, &mut var_veesat_dn9);
        Self::stamp_transient_block_7(ctx, p, nodes, var_cjs_t, var_cjs_t_db0, var_cjs_t_db1, var_cjs_t_db2, var_cjs_t_db3, var_cjs_t_db4, var_cjs_t_db5, var_cjs_t_db6, var_cjs_t_db7, var_cjs_t_dn0, var_cjs_t_dn1, var_cjs_t_dn2, var_cjs_t_dn3, var_cjs_t_dn4, var_cjs_t_dn5, var_cjs_t_dn6, var_cjs_t_dn7, var_cjs_t_dn8, var_cjs_t_dn9, var_ifwd, var_ifwd_db0, var_ifwd_db1, var_ifwd_db2, var_ifwd_db3, var_ifwd_db4, var_ifwd_db5, var_ifwd_db6, var_ifwd_db7, var_ifwd_dn0, var_ifwd_dn1, var_ifwd_dn2, var_ifwd_dn3, var_ifwd_dn4, var_ifwd_dn5, var_ifwd_dn6, var_ifwd_dn7, var_ifwd_dn8, var_ifwd_dn9, var_itr, var_itr_db0, var_itr_db1, var_itr_db2, var_itr_db3, var_itr_db4, var_itr_db5, var_itr_db6, var_itr_db7, var_itr_dn0, var_itr_dn1, var_itr_dn2, var_itr_dn3, var_itr_dn4, var_itr_dn5, var_itr_dn6, var_itr_dn7, var_itr_dn8, var_itr_dn9, var_lnrt, var_lnrt_db0, var_lnrt_db1, var_lnrt_db2, var_lnrt_db3, var_lnrt_db4, var_lnrt_db5, var_lnrt_db6, var_lnrt_db7, var_lnrt_dn0, var_lnrt_dn1, var_lnrt_dn2, var_lnrt_dn3, var_lnrt_dn4, var_lnrt_dn5, var_lnrt_dn6, var_lnrt_dn7, var_lnrt_dn8, var_lnrt_dn9, var_vbiei, var_vbiei_db0, var_vbiei_db1, var_vbiei_db2, var_vbiei_db3, var_vbiei_db4, var_vbiei_db5, var_vbiei_db6, var_vbiei_db7, var_vbiei_dn0, var_vbiei_dn1, var_vbiei_dn2, var_vbiei_dn3, var_vbiei_dn4, var_vbiei_dn5, var_vbiei_dn6, var_vbiei_dn7, var_vbiei_dn8, var_vbiei_dn9, var_veci, var_veci_db0, var_veci_db1, var_veci_db2, var_veci_db3, var_veci_db4, var_veci_db5, var_veci_db6, var_veci_db7, var_veci_dn0, var_veci_dn1, var_veci_dn2, var_veci_dn3, var_veci_dn4, var_veci_dn5, var_veci_dn6, var_veci_dn7, var_veci_dn8, var_veci_dn9, var_veesat, var_veesat_db0, var_veesat_db1, var_veesat_db2, var_veesat_db3, var_veesat_db4, var_veesat_db5, var_veesat_db6, var_veesat_db7, var_veesat_dn0, var_veesat_dn1, var_veesat_dn2, var_veesat_dn3, var_veesat_dn4, var_veesat_dn5, var_veesat_dn6, var_veesat_dn7, var_veesat_dn8, var_veesat_dn9, var_vje_t, var_vje_t_db0, var_vje_t_db1, var_vje_t_db2, var_vje_t_db3, var_vje_t_db4, var_vje_t_db5, var_vje_t_db6, var_vje_t_db7, var_vje_t_dn0, var_vje_t_dn1, var_vje_t_dn2, var_vje_t_dn3, var_vje_t_dn4, var_vje_t_dn5, var_vje_t_dn6, var_vje_t_dn7, var_vje_t_dn8, var_vje_t_dn9, var_vjs_t, var_vjs_t_db0, var_vjs_t_db1, var_vjs_t_db2, var_vjs_t_db3, var_vjs_t_db4, var_vjs_t_db5, var_vjs_t_db6, var_vjs_t_db7, var_vjs_t_dn0, var_vjs_t_dn1, var_vjs_t_dn2, var_vjs_t_dn3, var_vjs_t_dn4, var_vjs_t_dn5, var_vjs_t_dn6, var_vjs_t_dn7, var_vjs_t_dn8, var_vjs_t_dn9, &mut var_dv0, &mut var_dv0_db0, &mut var_dv0_db1, &mut var_dv0_db2, &mut var_dv0_db3, &mut var_dv0_db4, &mut var_dv0_db5, &mut var_dv0_db6, &mut var_dv0_db7, &mut var_dv0_dn0, &mut var_dv0_dn1, &mut var_dv0_dn2, &mut var_dv0_dn3, &mut var_dv0_dn4, &mut var_dv0_dn5, &mut var_dv0_dn6, &mut var_dv0_dn7, &mut var_dv0_dn8, &mut var_dv0_dn9, &mut var_dvh, &mut var_dvh_db0, &mut var_dvh_db1, &mut var_dvh_db2, &mut var_dvh_db3, &mut var_dvh_db4, &mut var_dvh_db5, &mut var_dvh_db6, &mut var_dvh_db7, &mut var_dvh_dn0, &mut var_dvh_dn1, &mut var_dvh_dn2, &mut var_dvh_dn3, &mut var_dvh_dn4, &mut var_dvh_dn5, &mut var_dvh_dn6, &mut var_dvh_dn7, &mut var_dvh_dn8, &mut var_dvh_dn9, &mut var_guard13, &mut var_guard14, &mut var_guard15, &mut var_guard16, &mut var_pwq, &mut var_qdc, &mut var_qdc_db0, &mut var_qdc_db1, &mut var_qdc_db2, &mut var_qdc_db3, &mut var_qdc_db4, &mut var_qdc_db5, &mut var_qdc_db6, &mut var_qdc_db7, &mut var_qdc_dn0, &mut var_qdc_dn1, &mut var_qdc_dn2, &mut var_qdc_dn3, &mut var_qdc_dn4, &mut var_qdc_dn5, &mut var_qdc_dn6, &mut var_qdc_dn7, &mut var_qdc_dn8, &mut var_qdc_dn9, &mut var_qde, &mut var_qde_db0, &mut var_qde_db1, &mut var_qde_db2, &mut var_qde_db3, &mut var_qde_db4, &mut var_qde_db5, &mut var_qde_db6, &mut var_qde_db7, &mut var_qde_dn0, &mut var_qde_dn1, &mut var_qde_dn2, &mut var_qde_dn3, &mut var_qde_dn4, &mut var_qde_dn5, &mut var_qde_dn6, &mut var_qde_dn7, &mut var_qde_dn8, &mut var_qde_dn9, &mut var_qhi, &mut var_qhi_db0, &mut var_qhi_db1, &mut var_qhi_db2, &mut var_qhi_db3, &mut var_qhi_db4, &mut var_qhi_db5, &mut var_qhi_db6, &mut var_qhi_db7, &mut var_qhi_dn0, &mut var_qhi_dn1, &mut var_qhi_dn2, &mut var_qhi_dn3, &mut var_qhi_dn4, &mut var_qhi_dn5, &mut var_qhi_dn6, &mut var_qhi_dn7, &mut var_qhi_dn8, &mut var_qhi_dn9, &mut var_qjs, &mut var_qjs_db0, &mut var_qjs_db1, &mut var_qjs_db2, &mut var_qjs_db3, &mut var_qjs_db4, &mut var_qjs_db5, &mut var_qjs_db6, &mut var_qjs_db7, &mut var_qjs_dn0, &mut var_qjs_dn1, &mut var_qjs_dn2, &mut var_qjs_dn3, &mut var_qjs_dn4, &mut var_qjs_dn5, &mut var_qjs_dn6, &mut var_qjs_dn7, &mut var_qjs_dn8, &mut var_qjs_dn9, &mut var_qlo, &mut var_qlo_db0, &mut var_qlo_db1, &mut var_qlo_db2, &mut var_qlo_db3, &mut var_qlo_db4, &mut var_qlo_db5, &mut var_qlo_db6, &mut var_qlo_db7, &mut var_qlo_dn0, &mut var_qlo_dn1, &mut var_qlo_dn2, &mut var_qlo_dn3, &mut var_qlo_dn4, &mut var_qlo_dn5, &mut var_qlo_dn6, &mut var_qlo_dn7, &mut var_qlo_dn8, &mut var_qlo_dn9, &mut var_rb, &mut var_rb_db0, &mut var_rb_db1, &mut var_rb_db2, &mut var_rb_db3, &mut var_rb_db4, &mut var_rb_db5, &mut var_rb_db6, &mut var_rb_db7, &mut var_rb_dn0, &mut var_rb_dn1, &mut var_rb_dn2, &mut var_rb_dn3, &mut var_rb_dn4, &mut var_rb_dn5, &mut var_rb_dn6, &mut var_rb_dn7, &mut var_rb_dn8, &mut var_rb_dn9, &mut var_rc, &mut var_rc_db0, &mut var_rc_db1, &mut var_rc_db2, &mut var_rc_db3, &mut var_rc_db4, &mut var_rc_db5, &mut var_rc_db6, &mut var_rc_db7, &mut var_rc_dn0, &mut var_rc_dn1, &mut var_rc_dn2, &mut var_rc_dn3, &mut var_rc_dn4, &mut var_rc_dn5, &mut var_rc_dn6, &mut var_rc_dn7, &mut var_rc_dn8, &mut var_rc_dn9, &mut var_re, &mut var_re_db0, &mut var_re_db1, &mut var_re_db2, &mut var_re_db3, &mut var_re_db4, &mut var_re_db5, &mut var_re_db6, &mut var_re_db7, &mut var_re_dn0, &mut var_re_dn1, &mut var_re_dn2, &mut var_re_dn3, &mut var_re_dn4, &mut var_re_dn5, &mut var_re_dn6, &mut var_re_dn7, &mut var_re_dn8, &mut var_re_dn9, &mut var_tff, &mut var_tff_db0, &mut var_tff_db1, &mut var_tff_db2, &mut var_tff_db3, &mut var_tff_db4, &mut var_tff_db5, &mut var_tff_db6, &mut var_tff_db7, &mut var_tff_dn0, &mut var_tff_dn1, &mut var_tff_dn2, &mut var_tff_dn3, &mut var_tff_dn4, &mut var_tff_dn5, &mut var_tff_dn6, &mut var_tff_dn7, &mut var_tff_dn8, &mut var_tff_dn9, &mut var_vtff, &mut var_vtff1, &mut var_vtff1_db0, &mut var_vtff1_db1, &mut var_vtff1_db2, &mut var_vtff1_db3, &mut var_vtff1_db4, &mut var_vtff1_db5, &mut var_vtff1_db6, &mut var_vtff1_db7, &mut var_vtff1_dn0, &mut var_vtff1_dn1, &mut var_vtff1_dn2, &mut var_vtff1_dn3, &mut var_vtff1_dn4, &mut var_vtff1_dn5, &mut var_vtff1_dn6, &mut var_vtff1_dn7, &mut var_vtff1_dn8, &mut var_vtff1_dn9, &mut var_vtff_db0, &mut var_vtff_db1, &mut var_vtff_db2, &mut var_vtff_db3, &mut var_vtff_db4, &mut var_vtff_db5, &mut var_vtff_db6, &mut var_vtff_db7, &mut var_vtff_dn0, &mut var_vtff_dn1, &mut var_vtff_dn2, &mut var_vtff_dn3, &mut var_vtff_dn4, &mut var_vtff_dn5, &mut var_vtff_dn6, &mut var_vtff_dn7, &mut var_vtff_dn8, &mut var_vtff_dn9);
        Self::stamp_transient_block_8(p, var_cjc_t, var_cjc_t_db0, var_cjc_t_db1, var_cjc_t_db2, var_cjc_t_db3, var_cjc_t_db4, var_cjc_t_db5, var_cjc_t_db6, var_cjc_t_db7, var_cjc_t_dn0, var_cjc_t_dn1, var_cjc_t_dn2, var_cjc_t_dn3, var_cjc_t_dn4, var_cjc_t_dn5, var_cjc_t_dn6, var_cjc_t_dn7, var_cjc_t_dn8, var_cjc_t_dn9, var_cje_t, var_cje_t_db0, var_cje_t_db1, var_cje_t_db2, var_cje_t_db3, var_cje_t_db4, var_cje_t_db5, var_cje_t_db6, var_cje_t_db7, var_cje_t_dn0, var_cje_t_dn1, var_cje_t_dn2, var_cje_t_dn3, var_cje_t_dn4, var_cje_t_dn5, var_cje_t_dn6, var_cje_t_dn7, var_cje_t_dn8, var_cje_t_dn9, var_guard16, var_vbci, var_vbci_db0, var_vbci_db1, var_vbci_db2, var_vbci_db3, var_vbci_db4, var_vbci_db5, var_vbci_db6, var_vbci_db7, var_vbci_dn0, var_vbci_dn1, var_vbci_dn2, var_vbci_dn3, var_vbci_dn4, var_vbci_dn5, var_vbci_dn6, var_vbci_dn7, var_vbci_dn8, var_vbci_dn9, var_vbici, var_vbici_db0, var_vbici_db1, var_vbici_db2, var_vbici_db3, var_vbici_db4, var_vbici_db5, var_vbici_db6, var_vbici_db7, var_vbici_dn0, var_vbici_dn1, var_vbici_dn2, var_vbici_dn3, var_vbici_dn4, var_vbici_dn5, var_vbici_dn6, var_vbici_dn7, var_vbici_dn8, var_vbici_dn9, var_vbiei, var_vbiei_db0, var_vbiei_db1, var_vbiei_db2, var_vbiei_db3, var_vbiei_db4, var_vbiei_db5, var_vbiei_db6, var_vbiei_db7, var_vbiei_dn0, var_vbiei_dn1, var_vbiei_dn2, var_vbiei_dn3, var_vbiei_dn4, var_vbiei_dn5, var_vbiei_dn6, var_vbiei_dn7, var_vbiei_dn8, var_vbiei_dn9, var_vjc_t, var_vjc_t_db0, var_vjc_t_db1, var_vjc_t_db2, var_vjc_t_db3, var_vjc_t_db4, var_vjc_t_db5, var_vjc_t_db6, var_vjc_t_db7, var_vjc_t_dn0, var_vjc_t_dn1, var_vjc_t_dn2, var_vjc_t_dn3, var_vjc_t_dn4, var_vjc_t_dn5, var_vjc_t_dn6, var_vjc_t_dn7, var_vjc_t_dn8, var_vjc_t_dn9, var_vje_t, var_vje_t_db0, var_vje_t_db1, var_vje_t_db2, var_vje_t_db3, var_vje_t_db4, var_vje_t_db5, var_vje_t_db6, var_vje_t_db7, var_vje_t_dn0, var_vje_t_dn1, var_vje_t_dn2, var_vje_t_dn3, var_vje_t_dn4, var_vje_t_dn5, var_vje_t_dn6, var_vje_t_dn7, var_vje_t_dn8, var_vje_t_dn9, &mut var_dv0, &mut var_dv0_db0, &mut var_dv0_db1, &mut var_dv0_db2, &mut var_dv0_db3, &mut var_dv0_db4, &mut var_dv0_db5, &mut var_dv0_db6, &mut var_dv0_db7, &mut var_dv0_dn0, &mut var_dv0_dn1, &mut var_dv0_dn2, &mut var_dv0_dn3, &mut var_dv0_dn4, &mut var_dv0_dn5, &mut var_dv0_dn6, &mut var_dv0_dn7, &mut var_dv0_dn8, &mut var_dv0_dn9, &mut var_dvh, &mut var_dvh_db0, &mut var_dvh_db1, &mut var_dvh_db2, &mut var_dvh_db3, &mut var_dvh_db4, &mut var_dvh_db5, &mut var_dvh_db6, &mut var_dvh_db7, &mut var_dvh_dn0, &mut var_dvh_dn1, &mut var_dvh_dn2, &mut var_dvh_dn3, &mut var_dvh_dn4, &mut var_dvh_dn5, &mut var_dvh_dn6, &mut var_dvh_dn7, &mut var_dvh_dn8, &mut var_dvh_dn9, &mut var_guard17, &mut var_guard18, &mut var_pwq, &mut var_qhi, &mut var_qhi_db0, &mut var_qhi_db1, &mut var_qhi_db2, &mut var_qhi_db3, &mut var_qhi_db4, &mut var_qhi_db5, &mut var_qhi_db6, &mut var_qhi_db7, &mut var_qhi_dn0, &mut var_qhi_dn1, &mut var_qhi_dn2, &mut var_qhi_dn3, &mut var_qhi_dn4, &mut var_qhi_dn5, &mut var_qhi_dn6, &mut var_qhi_dn7, &mut var_qhi_dn8, &mut var_qhi_dn9, &mut var_qjcx, &mut var_qjcx_1, &mut var_qjcx_1_db0, &mut var_qjcx_1_db1, &mut var_qjcx_1_db2, &mut var_qjcx_1_db3, &mut var_qjcx_1_db4, &mut var_qjcx_1_db5, &mut var_qjcx_1_db6, &mut var_qjcx_1_db7, &mut var_qjcx_1_dn0, &mut var_qjcx_1_dn1, &mut var_qjcx_1_dn2, &mut var_qjcx_1_dn3, &mut var_qjcx_1_dn4, &mut var_qjcx_1_dn5, &mut var_qjcx_1_dn6, &mut var_qjcx_1_dn7, &mut var_qjcx_1_dn8, &mut var_qjcx_1_dn9, &mut var_qjcx_db0, &mut var_qjcx_db1, &mut var_qjcx_db2, &mut var_qjcx_db3, &mut var_qjcx_db4, &mut var_qjcx_db5, &mut var_qjcx_db6, &mut var_qjcx_db7, &mut var_qjcx_dn0, &mut var_qjcx_dn1, &mut var_qjcx_dn2, &mut var_qjcx_dn3, &mut var_qjcx_dn4, &mut var_qjcx_dn5, &mut var_qjcx_dn6, &mut var_qjcx_dn7, &mut var_qjcx_dn8, &mut var_qjcx_dn9, &mut var_qje, &mut var_qje_db0, &mut var_qje_db1, &mut var_qje_db2, &mut var_qje_db3, &mut var_qje_db4, &mut var_qje_db5, &mut var_qje_db6, &mut var_qje_db7, &mut var_qje_dn0, &mut var_qje_dn1, &mut var_qje_dn2, &mut var_qje_dn3, &mut var_qje_dn4, &mut var_qje_dn5, &mut var_qje_dn6, &mut var_qje_dn7, &mut var_qje_dn8, &mut var_qje_dn9, &mut var_qlo, &mut var_qlo_db0, &mut var_qlo_db1, &mut var_qlo_db2, &mut var_qlo_db3, &mut var_qlo_db4, &mut var_qlo_db5, &mut var_qlo_db6, &mut var_qlo_db7, &mut var_qlo_dn0, &mut var_qlo_dn1, &mut var_qlo_dn2, &mut var_qlo_dn3, &mut var_qlo_dn4, &mut var_qlo_dn5, &mut var_qlo_dn6, &mut var_qlo_dn7, &mut var_qlo_dn8, &mut var_qlo_dn9);
        Self::stamp_transient_block_9(p, var_cjc_t, var_cjc_t_db0, var_cjc_t_db1, var_cjc_t_db2, var_cjc_t_db3, var_cjc_t_db4, var_cjc_t_db5, var_cjc_t_db6, var_cjc_t_db7, var_cjc_t_dn0, var_cjc_t_dn1, var_cjc_t_dn2, var_cjc_t_dn3, var_cjc_t_dn4, var_cjc_t_dn5, var_cjc_t_dn6, var_cjc_t_dn7, var_cjc_t_dn8, var_cjc_t_dn9, var_guard18, var_itzf, var_itzf_db0, var_itzf_db1, var_itzf_db2, var_itzf_db3, var_itzf_db4, var_itzf_db5, var_itzf_db6, var_itzf_db7, var_itzf_dn0, var_itzf_dn1, var_itzf_dn2, var_itzf_dn3, var_itzf_dn4, var_itzf_dn5, var_itzf_dn6, var_itzf_dn7, var_itzf_dn8, var_itzf_dn9, var_qlo, var_qlo_db0, var_qlo_db1, var_qlo_db2, var_qlo_db3, var_qlo_db4, var_qlo_db5, var_qlo_db6, var_qlo_db7, var_qlo_dn0, var_qlo_dn1, var_qlo_dn2, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6, var_qlo_dn7, var_qlo_dn8, var_qlo_dn9, var_ttype, var_weff, &mut var_guard19, &mut var_guard20, &mut var_guard21, &mut var_guard22, &mut var_guard23, &mut var_guard24, &mut var_guard25, &mut var_qhi, &mut var_qhi_db0, &mut var_qhi_db1, &mut var_qhi_db2, &mut var_qhi_db3, &mut var_qhi_db4, &mut var_qhi_db5, &mut var_qhi_db6, &mut var_qhi_db7, &mut var_qhi_dn0, &mut var_qhi_dn1, &mut var_qhi_dn2, &mut var_qhi_dn3, &mut var_qhi_dn4, &mut var_qhi_dn5, &mut var_qhi_dn6, &mut var_qhi_dn7, &mut var_qhi_dn8, &mut var_qhi_dn9, &mut var_qjci, &mut var_qjci_1, &mut var_qjci_1_db0, &mut var_qjci_1_db1, &mut var_qjci_1_db2, &mut var_qjci_1_db3, &mut var_qjci_1_db4, &mut var_qjci_1_db5, &mut var_qjci_1_db6, &mut var_qjci_1_db7, &mut var_qjci_1_dn0, &mut var_qjci_1_dn1, &mut var_qjci_1_dn2, &mut var_qjci_1_dn3, &mut var_qjci_1_dn4, &mut var_qjci_1_dn5, &mut var_qjci_1_dn6, &mut var_qjci_1_dn7, &mut var_qjci_1_dn8, &mut var_qjci_1_dn9, &mut var_qjci_db0, &mut var_qjci_db1, &mut var_qjci_db2, &mut var_qjci_db3, &mut var_qjci_db4, &mut var_qjci_db5, &mut var_qjci_db6, &mut var_qjci_db7, &mut var_qjci_dn0, &mut var_qjci_dn1, &mut var_qjci_dn2, &mut var_qjci_dn3, &mut var_qjci_dn4, &mut var_qjci_dn5, &mut var_qjci_dn6, &mut var_qjci_dn7, &mut var_qjci_dn8, &mut var_qjci_dn9, &mut var_qxf1, &mut var_qxf1_db0, &mut var_qxf1_db1, &mut var_qxf1_db2, &mut var_qxf1_db3, &mut var_qxf1_db4, &mut var_qxf1_db5, &mut var_qxf1_db6, &mut var_qxf1_db7, &mut var_qxf1_dn0, &mut var_qxf1_dn1, &mut var_qxf1_dn2, &mut var_qxf1_dn3, &mut var_qxf1_dn4, &mut var_qxf1_dn5, &mut var_qxf1_dn6, &mut var_qxf1_dn7, &mut var_qxf1_dn8, &mut var_qxf1_dn9, &mut var_rb_nom, &mut var_rc_nom, &mut var_re_nom);

        Self::stamp_transient_equations_block_0(ctx, stamper, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, var_bf_t, var_bf_t_db0, var_bf_t_db1, var_bf_t_db2, var_bf_t_db3, var_bf_t_db4, var_bf_t_db5, var_bf_t_db6, var_bf_t_db7, var_bf_t_dn0, var_bf_t_dn1, var_bf_t_dn2, var_bf_t_dn3, var_bf_t_dn4, var_bf_t_dn5, var_bf_t_dn6, var_bf_t_dn7, var_bf_t_dn8, var_bf_t_dn9, var_guard13, var_guard20, var_guard21, var_ibc, var_ibc_db0, var_ibc_db1, var_ibc_db2, var_ibc_db3, var_ibc_db4, var_ibc_db5, var_ibc_db6, var_ibc_db7, var_ibc_dn0, var_ibc_dn1, var_ibc_dn2, var_ibc_dn3, var_ibc_dn4, var_ibc_dn5, var_ibc_dn6, var_ibc_dn7, var_ibc_dn8, var_ibc_dn9, var_ibe, var_ibe_db0, var_ibe_db1, var_ibe_db2, var_ibe_db3, var_ibe_db4, var_ibe_db5, var_ibe_db6, var_ibe_db7, var_ibe_dn0, var_ibe_dn1, var_ibe_dn2, var_ibe_dn3, var_ibe_dn4, var_ibe_dn5, var_ibe_dn6, var_ibe_dn7, var_ibe_dn8, var_ibe_dn9, var_ifwd, var_ifwd_db0, var_ifwd_db1, var_ifwd_db2, var_ifwd_db3, var_ifwd_db4, var_ifwd_db5, var_ifwd_db6, var_ifwd_db7, var_ifwd_dn0, var_ifwd_dn1, var_ifwd_dn2, var_ifwd_dn3, var_ifwd_dn4, var_ifwd_dn5, var_ifwd_dn6, var_ifwd_dn7, var_ifwd_dn8, var_ifwd_dn9, var_tff, var_tff_db0, var_tff_db1, var_tff_db2, var_tff_db3, var_tff_db4, var_tff_db5, var_tff_db6, var_tff_db7, var_tff_dn0, var_tff_dn1, var_tff_dn2, var_tff_dn3, var_tff_dn4, var_tff_dn5, var_tff_dn6, var_tff_dn7, var_tff_dn8, var_tff_dn9);
        Self::stamp_transient_equations_block_1(ctx, stamper, p, nodes, multiplicity, var_guard20, var_guard21, var_guard22, var_guard23, var_guard24, var_guard25, var_ibc, var_ibc_db0, var_ibc_db1, var_ibc_db2, var_ibc_db3, var_ibc_db4, var_ibc_db5, var_ibc_db6, var_ibc_db7, var_ibc_dn0, var_ibc_dn1, var_ibc_dn2, var_ibc_dn3, var_ibc_dn4, var_ibc_dn5, var_ibc_dn6, var_ibc_dn7, var_ibc_dn8, var_ibc_dn9, var_ibe, var_ibe_db0, var_ibe_db1, var_ibe_db2, var_ibe_db3, var_ibe_db4, var_ibe_db5, var_ibe_db6, var_ibe_db7, var_ibe_dn0, var_ibe_dn1, var_ibe_dn2, var_ibe_dn3, var_ibe_dn4, var_ibe_dn5, var_ibe_dn6, var_ibe_dn7, var_ibe_dn8, var_ibe_dn9, var_rb, var_rb_db0, var_rb_db1, var_rb_db2, var_rb_db3, var_rb_db4, var_rb_db5, var_rb_db6, var_rb_db7, var_rb_dn0, var_rb_dn1, var_rb_dn2, var_rb_dn3, var_rb_dn4, var_rb_dn5, var_rb_dn6, var_rb_dn7, var_rb_dn8, var_rb_dn9, var_rc, var_rc_db0, var_rc_db1, var_rc_db2, var_rc_db3, var_rc_db4, var_rc_db5, var_rc_db6, var_rc_db7, var_rc_dn0, var_rc_dn1, var_rc_dn2, var_rc_dn3, var_rc_dn4, var_rc_dn5, var_rc_dn6, var_rc_dn7, var_rc_dn8, var_rc_dn9, var_re, var_re_db0, var_re_db1, var_re_db2, var_re_db3, var_re_db4, var_re_db5, var_re_db6, var_re_db7, var_re_dn0, var_re_dn1, var_re_dn2, var_re_dn3, var_re_dn4, var_re_dn5, var_re_dn6, var_re_dn7, var_re_dn8, var_re_dn9, var_ttype, var_weff);
        Self::stamp_transient_equations_block_2(stamper, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, var_itr, var_itr_db0, var_itr_db1, var_itr_db2, var_itr_db3, var_itr_db4, var_itr_db5, var_itr_db6, var_itr_db7, var_itr_dn0, var_itr_dn1, var_itr_dn2, var_itr_dn3, var_itr_dn4, var_itr_dn5, var_itr_dn6, var_itr_dn7, var_itr_dn8, var_itr_dn9, var_itzf_f, var_itzf_f_db0, var_itzf_f_db1, var_itzf_f_db2, var_itzf_f_db3, var_itzf_f_db4, var_itzf_f_db5, var_itzf_f_db6, var_itzf_f_db7, var_itzf_f_dn0, var_itzf_f_dn1, var_itzf_f_dn2, var_itzf_f_dn3, var_itzf_f_dn4, var_itzf_f_dn5, var_itzf_f_dn6, var_itzf_f_dn7, var_itzf_f_dn8, var_itzf_f_dn9, var_qdc, var_qdc_db0, var_qdc_db1, var_qdc_db2, var_qdc_db3, var_qdc_db4, var_qdc_db5, var_qdc_db6, var_qdc_db7, var_qdc_dn0, var_qdc_dn1, var_qdc_dn2, var_qdc_dn3, var_qdc_dn4, var_qdc_dn5, var_qdc_dn6, var_qdc_dn7, var_qdc_dn8, var_qdc_dn9, var_qde, var_qde_db0, var_qde_db1, var_qde_db2, var_qde_db3, var_qde_db4, var_qde_db5, var_qde_db6, var_qde_db7, var_qde_dn0, var_qde_dn1, var_qde_dn2, var_qde_dn3, var_qde_dn4, var_qde_dn5, var_qde_dn6, var_qde_dn7, var_qde_dn8, var_qde_dn9, var_qjci_1, var_qjci_1_db0, var_qjci_1_db1, var_qjci_1_db2, var_qjci_1_db3, var_qjci_1_db4, var_qjci_1_db5, var_qjci_1_db6, var_qjci_1_db7, var_qjci_1_dn0, var_qjci_1_dn1, var_qjci_1_dn2, var_qjci_1_dn3, var_qjci_1_dn4, var_qjci_1_dn5, var_qjci_1_dn6, var_qjci_1_dn7, var_qjci_1_dn8, var_qjci_1_dn9, var_qjcx_1, var_qjcx_1_db0, var_qjcx_1_db1, var_qjcx_1_db2, var_qjcx_1_db3, var_qjcx_1_db4, var_qjcx_1_db5, var_qjcx_1_db6, var_qjcx_1_db7, var_qjcx_1_dn0, var_qjcx_1_dn1, var_qjcx_1_dn2, var_qjcx_1_dn3, var_qjcx_1_dn4, var_qjcx_1_dn5, var_qjcx_1_dn6, var_qjcx_1_dn7, var_qjcx_1_dn8, var_qjcx_1_dn9, var_qje, var_qje_db0, var_qje_db1, var_qje_db2, var_qje_db3, var_qje_db4, var_qje_db5, var_qje_db6, var_qje_db7, var_qje_dn0, var_qje_dn1, var_qje_dn2, var_qje_dn3, var_qje_dn4, var_qje_dn5, var_qje_dn6, var_qje_dn7, var_qje_dn8, var_qje_dn9, var_qjs, var_qjs_db0, var_qjs_db1, var_qjs_db2, var_qjs_db3, var_qjs_db4, var_qjs_db5, var_qjs_db6, var_qjs_db7, var_qjs_dn0, var_qjs_dn1, var_qjs_dn2, var_qjs_dn3, var_qjs_dn4, var_qjs_dn5, var_qjs_dn6, var_qjs_dn7, var_qjs_dn8, var_qjs_dn9, var_qxf1, var_qxf1_db0, var_qxf1_db1, var_qxf1_db2, var_qxf1_db3, var_qxf1_db4, var_qxf1_db5, var_qxf1_db6, var_qxf1_db7, var_qxf1_dn0, var_qxf1_dn1, var_qxf1_dn2, var_qxf1_dn3, var_qxf1_dn4, var_qxf1_dn5, var_qxf1_dn6, var_qxf1_dn7, var_qxf1_dn8, var_qxf1_dn9, var_ttype, var_weff);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let multiplicity = (*self).multiplicity;
        let s = match &mut self.reactive_scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(ReactiveScratch::new_box()).as_mut(),
        };

        Self::stamp_reactive_block_0(ctx, s, p, nodes);
        Self::stamp_reactive_block_1(s, p);

        Self::stamp_reactive_equations_block_0(ctx, stamper, s, p, nodes, branches, multiplicity);
    }
}
