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
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
        let v4: f64 = nv5;
        let v5: f64 = 1.0;
        let v6: f64 = 0.0;
        let v7: f64 = nv6;
        let v8: f64 = (v4 - v7);
        let v9: f64 = (self.scalar_v3 * v8);
        let v10: f64 = nv9;
        let v53: f64 = (v9 - v10);
        let v54: f64 = (-v53);
        let v55: f64 = 1e-6;
        let v56: f64 = (v10 * v55);
        let v57: f64 = nv8;
        let v58: f64 = (if self.scalar_v16 { v57 } else { v6 });

        let d54_dn5: f64 = self.scalar_v76;
        let d54_dn6: f64 = self.scalar_v77;
        let d54_dn9: f64 = v5;
        stamper.stamp_current_node3_local(
            Some(9),
            None,
            multiplicity * (v54),
            5,
            multiplicity * (d54_dn5),
            6,
            multiplicity * (d54_dn6),
            9,
            multiplicity * (d54_dn9),
        );
        let d56_dn9: f64 = v55;
        stamper.stamp_current_node1_local(
            Some(9),
            None,
            multiplicity * (v56),
            9,
            multiplicity * (d56_dn9),
        );
        let d58_dn8: f64 = self.scalar_v78;
        stamper.stamp_current_node1_local(
            Some(8),
            None,
            multiplicity * (v58),
            8,
            multiplicity * (d58_dn8),
        );
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        Self::stamp_transient_block_0(ctx, s, p, nodes);
        Self::stamp_transient_block_1(ctx, s, p, nodes);

        stamper.stamp_potential_branch_local(
            Some(8),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            None,
            4,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(5),
            5,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            Some(6),
            6,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(4),
            7,
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_1(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_2(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_3(stamper, s, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
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
