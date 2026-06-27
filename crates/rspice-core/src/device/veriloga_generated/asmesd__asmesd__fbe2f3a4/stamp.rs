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
        let nv4 = ctx.node_voltage(nodes[4]);
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
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        Self::stamp_transient_block_0(ctx, s, p, nodes);
        Self::stamp_transient_block_1(ctx, s, p, nodes);

        Self::stamp_transient_equations_block_0(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_1(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_2(stamper, s, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
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
