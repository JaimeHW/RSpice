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
#[path = "stamp_blocks_1.rs"]
mod stamp_blocks_1;
#[path = "stamp_blocks_2.rs"]
mod stamp_blocks_2;
#[path = "stamp_blocks_3.rs"]
mod stamp_blocks_3;
#[path = "stamp_blocks_4.rs"]
mod stamp_blocks_4;
#[path = "stamp_blocks_5.rs"]
mod stamp_blocks_5;
#[path = "stamp_blocks_6.rs"]
mod stamp_blocks_6;

const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

#[inline]
fn eval_ddt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    ddt_active: bool,
    ddt_scale: f64,
    slot: usize,
    value: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated ddt state slot out of range");
    let previous_value = if initialized[slot] { previous[slot] } else { value };
    current[slot] = value;
    if ddt_active {
        (value - previous_value) * ddt_scale
    } else {
        previous[slot] = value;
        initialized[slot] = true;
        0.0
    }
}

#[inline]
fn ddt_jacobian(timestep: f64, derivative: f64) -> f64 {
    if timestep.abs() > Instance::DDT_EPSILON {
        derivative / timestep
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
        let v0: f64 = 0.0;
        let v61: f64 = nv13;
        let v62: f64 = (if self.scalar_v40 { v61 } else { v0 });
        let v63: f64 = nv12;
        let v64: f64 = (if self.scalar_v40 { v63 } else { v0 });
        let v65: f64 = (if self.scalar_v41 { v61 } else { v0 });
        let v66: f64 = (if self.scalar_v41 { v63 } else { v0 });

        let d62_dn13: f64 = self.scalar_v70;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (v62),
            13,
            multiplicity * (d62_dn13),
        );
        let d64_dn12: f64 = self.scalar_v70;
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (v64),
            12,
            multiplicity * (d64_dn12),
        );
        let d65_dn13: f64 = self.scalar_v71;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (v65),
            13,
            multiplicity * (d65_dn13),
        );
        let d66_dn12: f64 = self.scalar_v71;
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (v66),
            12,
            multiplicity * (d66_dn12),
        );
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        Self::stamp_transient_block_0(s, p, param_given);
        Self::stamp_transient_block_1(s, p);
        Self::stamp_transient_block_2(s, p, param_given);
        Self::stamp_transient_block_3(s, p);
        Self::stamp_transient_block_4(s, p, param_given);
        Self::stamp_transient_block_5(s, p);
        Self::stamp_transient_block_6(s, p);
        Self::stamp_transient_block_7(ctx, s, p, nodes);
        Self::stamp_transient_block_8(ctx, s, p, nodes, param_given);
        Self::stamp_transient_block_9(s, p);
        Self::stamp_transient_block_10(s);
        Self::stamp_transient_block_11(s, p);
        Self::stamp_transient_block_12(s, p);
        Self::stamp_transient_block_13(s, p);
        Self::stamp_transient_block_14(s, p);
        Self::stamp_transient_block_15(s, p);
        Self::stamp_transient_block_16(s, p);
        Self::stamp_transient_block_17(s, p);
        Self::stamp_transient_block_18(s, p);
        Self::stamp_transient_block_19(ctx, s, p, nodes, param_given);
        Self::stamp_transient_block_20(s, p);
        Self::stamp_transient_block_21(s, p);
        Self::stamp_transient_block_22(s, p);
        Self::stamp_transient_block_23(s, p);
        Self::stamp_transient_block_24(s);
        Self::stamp_transient_block_25(ctx, s, p, nodes);
        Self::stamp_transient_block_26(s, p);
        Self::stamp_transient_block_27(s, p);
        Self::stamp_transient_block_28(s);
        Self::stamp_transient_block_29(s, p);
        Self::stamp_transient_block_30(s, p);
        Self::stamp_transient_block_31(s, p);
        Self::stamp_transient_block_32(s, p);
        Self::stamp_transient_block_33(s, p);
        Self::stamp_transient_block_34(s, p);
        Self::stamp_transient_block_35(s, p);
        Self::stamp_transient_block_36(s, p);
        Self::stamp_transient_block_37(ctx, s, p, nodes);
        Self::stamp_transient_block_38(s, p);
        Self::stamp_transient_block_39(ctx, s, p, nodes);
        Self::stamp_transient_block_40(s, p);
        Self::stamp_transient_block_41(s, p);
        Self::stamp_transient_block_42(s, p);
        Self::stamp_transient_block_43(ctx, s, p, nodes, param_given);
        Self::stamp_transient_block_44(s, p);
        Self::stamp_transient_block_45(ctx, s, p, nodes);

        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(12),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(9),
            2,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(6),
            3,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            Some(7),
            4,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(8),
            5,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            None,
            6,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            7,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            8,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(10),
            9,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(11),
            10,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(11),
            11,
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_1(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_2(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_3(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_4(stamper, s, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_5(stamper, s, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_6(ctx, stamper, s, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_7(ctx, stamper, s, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_8(ctx, stamper, s, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let s = match &mut self.reactive_scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(ReactiveScratch::new_box()).as_mut(),
        };

        Self::stamp_reactive_block_0(s, p, param_given);
        Self::stamp_reactive_block_1(s, p);
        Self::stamp_reactive_block_2(s, p, param_given);
        Self::stamp_reactive_block_3(s, p);
        Self::stamp_reactive_block_4(s, p, param_given);
        Self::stamp_reactive_block_5(s, p);
        Self::stamp_reactive_block_6(ctx, s, p, nodes);
        Self::stamp_reactive_block_7(ctx, s, p, nodes);
        Self::stamp_reactive_block_8(ctx, s, p, nodes, param_given);
        Self::stamp_reactive_block_9(s, p);
        Self::stamp_reactive_block_10(s);
        Self::stamp_reactive_block_11(s, p);
        Self::stamp_reactive_block_12(s, p);
        Self::stamp_reactive_block_13(s, p);
        Self::stamp_reactive_block_14(s, p);
        Self::stamp_reactive_block_15(s, p);
        Self::stamp_reactive_block_16(s, p);
        Self::stamp_reactive_block_17(s, p);
        Self::stamp_reactive_block_18(ctx, s, p, nodes, param_given);
        Self::stamp_reactive_block_19(s, p);
        Self::stamp_reactive_block_20(s, p);
        Self::stamp_reactive_block_21(s, p);
        Self::stamp_reactive_block_22(s, p);
        Self::stamp_reactive_block_23(s, p);
        Self::stamp_reactive_block_24(s, p);
        Self::stamp_reactive_block_25(ctx, s, p, nodes);
        Self::stamp_reactive_block_26(s, p);
        Self::stamp_reactive_block_27(s);
        Self::stamp_reactive_block_28(s, p);
        Self::stamp_reactive_block_29(s, p);
        Self::stamp_reactive_block_30(s, p);
        Self::stamp_reactive_block_31(s, p);
        Self::stamp_reactive_block_32(s, p);
        Self::stamp_reactive_block_33(s, p);
        Self::stamp_reactive_block_34(s, p);
        Self::stamp_reactive_block_35(s, p);
        Self::stamp_reactive_block_36(ctx, s, p, nodes);
        Self::stamp_reactive_block_37(ctx, s, p, nodes);
        Self::stamp_reactive_block_38(s, p);
        Self::stamp_reactive_block_39(s, p);
        Self::stamp_reactive_block_40(s, p);
        Self::stamp_reactive_block_41(ctx, s, p, nodes, param_given);
        Self::stamp_reactive_block_42(ctx, s, p, nodes);
        Self::stamp_reactive_block_43(ctx, s, p, nodes);

        Self::stamp_reactive_equations_block_0(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_1(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_2(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_3(stamper, s, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_4(stamper, s, nodes, branches, multiplicity);
    }
}
