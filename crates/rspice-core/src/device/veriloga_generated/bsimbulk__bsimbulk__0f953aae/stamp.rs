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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
        let v0: f64 = 0.0;
        let v1: f64 = 1.0;
        let v30: f64 = nv14;
        let v32: f64 = nv0;
        let v38: f64 = nv16;
        let v39: f64 = nv15;
        let v47: f64 = (v30 - v32);
        let v48: f64 = (v47 * self.scalar_v31);
        let v49: f64 = (v48 * self.scalar_v25);
        let v50: f64 = (if self.scalar_v35 { v49 } else { v0 });

        let d38_dn16: f64 = v1;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (v38),
            16,
            multiplicity * (d38_dn16),
        );
        let d39_dn15: f64 = v1;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (v39),
            15,
            multiplicity * (d39_dn15),
        );
        let d50_dn0: f64 = self.scalar_v56;
        let d50_dn14: f64 = self.scalar_v57;
        stamper.stamp_current_node2_local(
            Some(14),
            Some(0),
            multiplicity * (v50),
            0,
            multiplicity * (d50_dn0),
            14,
            multiplicity * (d50_dn14),
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
        Self::stamp_transient_block_6(s, p, param_given);
        Self::stamp_transient_block_7(ctx, s, p, nodes);
        Self::stamp_transient_block_8(s, p);
        Self::stamp_transient_block_9(s, p, param_given);
        Self::stamp_transient_block_10(ctx, s, p, nodes, param_given);
        Self::stamp_transient_block_11(s, p);
        Self::stamp_transient_block_12(ctx, s, p, nodes);
        Self::stamp_transient_block_13(ctx, s, p, nodes);
        Self::stamp_transient_block_14(ctx, s, p, nodes);
        Self::stamp_transient_block_15(ctx, s, p, nodes);
        Self::stamp_transient_block_16(s, p);
        Self::stamp_transient_block_17(s, p);
        Self::stamp_transient_block_18(ctx, s, p, nodes);
        Self::stamp_transient_block_19(s, p);
        Self::stamp_transient_block_20(ctx, s, p, nodes, param_given);
        Self::stamp_transient_block_21(s, p);
        Self::stamp_transient_block_22(ctx, s, p, nodes);

        stamper.stamp_potential_branch_local(
            Some(4),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            1,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(6),
            2,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            Some(5),
            3,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(8),
            4,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            Some(8),
            5,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(7),
            6,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(10),
            7,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(9),
            8,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            None,
            9,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(12),
            10,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(11),
            11,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(13),
            12,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(14),
            13,
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_1(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_2(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_3(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_4(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_5(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_6(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_7(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_8(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_9(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
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
        Self::stamp_reactive_block_7(s, p);
        Self::stamp_reactive_block_8(s, p, param_given);
        Self::stamp_reactive_block_9(ctx, s, p, nodes, param_given);
        Self::stamp_reactive_block_10(s, p);
        Self::stamp_reactive_block_11(s, p);
        Self::stamp_reactive_block_12(ctx, s, p, nodes);
        Self::stamp_reactive_block_13(ctx, s, p, nodes);
        Self::stamp_reactive_block_14(ctx, s, p, nodes);
        Self::stamp_reactive_block_15(s, p);
        Self::stamp_reactive_block_16(ctx, s, p, nodes);
        Self::stamp_reactive_block_17(s, p);
        Self::stamp_reactive_block_18(s, p);
        Self::stamp_reactive_block_19(ctx, s, p, nodes, param_given);
        Self::stamp_reactive_block_20(ctx, s, p, nodes);
        Self::stamp_reactive_block_21(ctx, s, p, nodes);

        Self::stamp_reactive_equations_block_0(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_1(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_2(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_3(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_4(stamper, s, p, nodes, branches, multiplicity);
    }
}
