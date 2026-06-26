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
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        Self::stamp_transient_block_0(ctx, s, p, nodes);
        Self::stamp_transient_block_1(ctx, s, p, nodes);
        Self::stamp_transient_block_2(s, p);
        Self::stamp_transient_block_3(s, p);
        Self::stamp_transient_block_4(s, p);
        Self::stamp_transient_block_5(s, p);
        Self::stamp_transient_block_6(s, p);
        Self::stamp_transient_block_7(s, p);
        Self::stamp_transient_block_8(s, p);
        Self::stamp_transient_block_9(s, p);
        Self::stamp_transient_block_10(s, p);
        Self::stamp_transient_block_11(s, p);
        Self::stamp_transient_block_12(s, p);
        Self::stamp_transient_block_13(s, p);
        Self::stamp_transient_block_14(s, p);
        Self::stamp_transient_block_15(s, p);
        Self::stamp_transient_block_16(s, p);
        Self::stamp_transient_block_17(s, p);
        Self::stamp_transient_block_18(s, p);
        Self::stamp_transient_block_19(s, p);
        Self::stamp_transient_block_20(s, p);
        Self::stamp_transient_block_21(s, p);
        Self::stamp_transient_block_22(s, p);
        Self::stamp_transient_block_23(s, p);
        Self::stamp_transient_block_24(ctx, s, p, nodes);
        Self::stamp_transient_block_25(ctx, s, p, nodes);
        Self::stamp_transient_block_26(s, p);
        Self::stamp_transient_block_27(ctx, s, p, nodes);
        Self::stamp_transient_block_28(ctx, s, p, nodes);
        Self::stamp_transient_block_29(ctx, s, p, nodes);
        Self::stamp_transient_block_30(ctx, s, p, nodes);
        Self::stamp_transient_block_31(ctx, s, p, nodes);
        Self::stamp_transient_block_32(ctx, s, p, nodes);
        Self::stamp_transient_block_33(s, p);
        Self::stamp_transient_block_34(ctx, s, p, nodes);
        Self::stamp_transient_block_35(s, p);
        Self::stamp_transient_block_36(ctx, s, p, nodes);
        Self::stamp_transient_block_37(s, p);
        Self::stamp_transient_block_38(ctx, s, p, nodes);
        Self::stamp_transient_block_39(ctx, s, p, nodes);

        stamper.stamp_potential_branch_local(
            Some(22),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(23),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(24),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(25),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(26),
            None,
            4,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(27),
            None,
            5,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(21),
            None,
            6,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(20),
            None,
            7,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(22),
            None,
            8,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(25),
            None,
            9,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(21),
            None,
            10,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(20),
            None,
            11,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(22),
            None,
            12,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(23),
            None,
            13,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(24),
            None,
            14,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(25),
            None,
            15,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(26),
            None,
            16,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(27),
            None,
            17,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(17),
            Some(16),
            18,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(16),
            Some(15),
            19,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(15),
            Some(14),
            20,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            Some(5),
            21,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(10),
            22,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(11),
            23,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            Some(12),
            24,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(12),
            Some(13),
            25,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            Some(19),
            26,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(18),
            Some(17),
            27,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(28),
            None,
            28,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(29),
            None,
            29,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(8),
            30,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(18),
            31,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(19),
            Some(2),
            32,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(6),
            33,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            Some(7),
            34,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            None,
            35,
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_1(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_2(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_3(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_4(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_5(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_6(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_7(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_8(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_9(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_10(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_11(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_12(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_13(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_14(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_15(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_16(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_17(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_18(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_19(ctx, stamper, s, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_20(ctx, stamper, s, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_21(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_22(ctx, stamper, s, p, nodes, multiplicity);
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
        Self::stamp_reactive_block_1(ctx, s, p, nodes);
        Self::stamp_reactive_block_2(s, p);
        Self::stamp_reactive_block_3(s, p);
        Self::stamp_reactive_block_4(s, p);
        Self::stamp_reactive_block_5(s, p);
        Self::stamp_reactive_block_6(s, p);
        Self::stamp_reactive_block_7(s, p);
        Self::stamp_reactive_block_8(s, p);
        Self::stamp_reactive_block_9(s, p);
        Self::stamp_reactive_block_10(s, p);
        Self::stamp_reactive_block_11(s, p);
        Self::stamp_reactive_block_12(s, p);
        Self::stamp_reactive_block_13(s, p);
        Self::stamp_reactive_block_14(s, p);
        Self::stamp_reactive_block_15(s, p);
        Self::stamp_reactive_block_16(s, p);
        Self::stamp_reactive_block_17(s, p);
        Self::stamp_reactive_block_18(s, p);
        Self::stamp_reactive_block_19(s, p);
        Self::stamp_reactive_block_20(ctx, s, p, nodes);
        Self::stamp_reactive_block_21(ctx, s, p, nodes);

        Self::stamp_reactive_equations_block_0(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_reactive_equations_block_1(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_2(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_3(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_4(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_5(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_6(ctx, stamper, s, p, nodes, branches, multiplicity);
    }
}
