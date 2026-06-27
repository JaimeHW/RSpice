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
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
        let v0: f64 = 0.0;
        let v104: f64 = nv7;
        let v112: f64 = nv1;
        let v116: f64 = (v112 - v104);
        let v117: f64 = (v116 * self.scalar_v111);
        let v118: f64 = (if self.scalar_v110 { v117 } else { v0 });

        let d118_dn1: f64 = self.scalar_v121;
        let d118_dn7: f64 = self.scalar_v122;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(7),
            multiplicity * (v118),
            1,
            multiplicity * (d118_dn1),
            7,
            multiplicity * (d118_dn7),
        );
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        Self::stamp_transient_block_0(s, p, param_given);
        Self::stamp_transient_block_1(ctx, s, p, nodes, param_given);
        Self::stamp_transient_block_2(s, p);
        Self::stamp_transient_block_3(s, p);
        Self::stamp_transient_block_4(s);
        Self::stamp_transient_block_5(ctx, s, p, nodes);
        Self::stamp_transient_block_6(ctx, s, p, nodes);
        Self::stamp_transient_block_7(s, p);

        stamper.stamp_potential_branch_local(
            Some(0),
            Some(5),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            Some(6),
            1,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(8),
            2,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(7),
            3,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            None,
            4,
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(ctx, stamper, s, nodes, multiplicity);
        Self::stamp_transient_equations_block_1(ctx, stamper, s, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_2(ctx, stamper, s, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
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
        Self::stamp_reactive_block_1(ctx, s, p, nodes, param_given);
        Self::stamp_reactive_block_2(s, p);
        Self::stamp_reactive_block_3(s, p);
        Self::stamp_reactive_block_4(s, p);
        Self::stamp_reactive_block_5(ctx, s, p, nodes);
        Self::stamp_reactive_block_6(s, p);

        Self::stamp_reactive_equations_block_0(ctx, stamper, s, nodes, branches, multiplicity);
    }
}
