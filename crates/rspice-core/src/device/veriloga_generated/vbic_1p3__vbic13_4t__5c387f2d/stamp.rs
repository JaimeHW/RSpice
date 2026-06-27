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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
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
        let v0: f64 = 1.0;
        let v1: f64 = nv1;
        let v2: f64 = nv2;
        let v3: f64 = (v1 - v2);
        let v4: f64 = nv0;
        let v5: f64 = (v1 - v4);
        let v6: f64 = nv12;
        let v7: f64 = nv13;
        let v8: f64 = (v7 - v6);
        let v10: f64 = (v3 * self.scalar_v9);
        let v12: f64 = (v5 * self.scalar_v11);
        let v14: f64 = (self.scalar_v13 * v6);
        let v15: f64 = (self.scalar_v13 * v7);
        let v16: f64 = 0.3333333333333333;
        let v17: f64 = (v15 * v16);
        let v18: f64 = -1.0;

        let d8_dn12: f64 = v18;
        let d8_dn13: f64 = v0;
        stamper.stamp_current_node2_local(
            Some(13),
            None,
            multiplicity * (v8),
            12,
            multiplicity * (d8_dn12),
            13,
            multiplicity * (d8_dn13),
        );
        let d10_dn1: f64 = self.scalar_v9;
        let d10_dn2: f64 = self.scalar_v19;
        let v10_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, v10);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (v10_ddt),
            1,
            multiplicity * (((d10_dn1) * ddt_scale)),
            2,
            multiplicity * (((d10_dn2) * ddt_scale)),
        );
        let d12_dn0: f64 = self.scalar_v20;
        let d12_dn1: f64 = self.scalar_v11;
        let v12_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, v12);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (v12_ddt),
            0,
            multiplicity * (((d12_dn0) * ddt_scale)),
            1,
            multiplicity * (((d12_dn1) * ddt_scale)),
        );
        let d14_dn12: f64 = self.scalar_v13;
        let v14_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, v14);
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (v14_ddt),
            12,
            multiplicity * (((d14_dn12) * ddt_scale)),
        );
        let d17_dn13: f64 = self.scalar_v21;
        let v17_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, v17);
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (v17_ddt),
            13,
            multiplicity * (((d17_dn13) * ddt_scale)),
        );
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        Self::stamp_transient_block_0(ctx, s, p, nodes, param_given);
        Self::stamp_transient_block_1(ctx, s, p, nodes);
        Self::stamp_transient_block_2(s, p);
        Self::stamp_transient_block_3(s, p);

        Self::stamp_transient_equations_block_0(stamper, s, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let v1: f64 = nv1;
        let v2: f64 = nv2;
        let v3: f64 = (v1 - v2);
        let v4: f64 = nv0;
        let v5: f64 = (v1 - v4);
        let v6: f64 = nv12;
        let v7: f64 = nv13;
        let v10: f64 = (v3 * self.scalar_v9);
        let v12: f64 = (v5 * self.scalar_v11);
        let v14: f64 = (self.scalar_v13 * v6);
        let v15: f64 = (self.scalar_v13 * v7);
        let v16: f64 = 0.3333333333333333;
        let v17: f64 = (v15 * v16);

        let d10_dn1: f64 = self.scalar_v9;
        let d10_dn2: f64 = self.scalar_v19;
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (d10_dn1),
            nodes[2],
            multiplicity * (d10_dn2),
        );
        let d12_dn0: f64 = self.scalar_v20;
        let d12_dn1: f64 = self.scalar_v11;
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (d12_dn0),
            nodes[1],
            multiplicity * (d12_dn1),
        );
        let d14_dn12: f64 = self.scalar_v13;
        stamper.stamp_current_reactive_node1(
            Some(nodes[12]),
            None,
            nodes[12],
            multiplicity * (d14_dn12),
        );
        let d17_dn13: f64 = self.scalar_v21;
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (d17_dn13),
        );
        let s = match &mut self.reactive_scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(ReactiveScratch::new_box()).as_mut(),
        };

        Self::stamp_reactive_block_0(ctx, s, p, nodes, param_given);
        Self::stamp_reactive_block_1(s, p);
        Self::stamp_reactive_block_2(s, p);

        Self::stamp_reactive_equations_block_0(stamper, s, nodes, branches, multiplicity);
    }
}
