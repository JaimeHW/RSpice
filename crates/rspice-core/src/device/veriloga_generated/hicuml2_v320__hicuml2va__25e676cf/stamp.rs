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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
        let v2: f64 = nv9;
        let v3: f64 = nv3;
        let v6: f64 = 0.0;
        let v64: f64 = (v2 - v3);
        let v65: f64 = (v64 / self.scalar_v34);
        let v66: f64 = (if self.scalar_v37 { v65 } else { v6 });
        let v71: f64 = nv13;
        let v72: f64 = (-v71);
        let v73: f64 = (if self.scalar_v48 { v72 } else { v6 });
        let v74: f64 = (if self.scalar_v48 { v71 } else { v6 });
        let v75: f64 = nv14;
        let v76: f64 = (-v75);
        let v77: f64 = (if self.scalar_v48 { v76 } else { v6 });
        let v78: f64 = (if self.scalar_v48 { v75 } else { v6 });
        let v80: f64 = (if self.scalar_v79 { v71 } else { v6 });
        let v81: f64 = (if self.scalar_v79 { v75 } else { v6 });

        let d66_dn3: f64 = self.scalar_v87;
        let d66_dn9: f64 = self.scalar_v88;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(3),
            multiplicity * (v66),
            3,
            multiplicity * (d66_dn3),
            9,
            multiplicity * (d66_dn9),
        );
        let d73_dn13: f64 = self.scalar_v89;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (v73),
            13,
            multiplicity * (d73_dn13),
        );
        let d74_dn13: f64 = self.scalar_v90;
        stamper.stamp_current_node1_local(
            Some(8),
            Some(6),
            multiplicity * (v74),
            13,
            multiplicity * (d74_dn13),
        );
        let d77_dn14: f64 = self.scalar_v89;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (v77),
            14,
            multiplicity * (d77_dn14),
        );
        let d78_dn14: f64 = self.scalar_v90;
        stamper.stamp_current_node1_local(
            Some(5),
            Some(6),
            multiplicity * (v78),
            14,
            multiplicity * (d78_dn14),
        );
        let d80_dn13: f64 = self.scalar_v91;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (v80),
            13,
            multiplicity * (d80_dn13),
        );
        let d81_dn14: f64 = self.scalar_v91;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (v81),
            14,
            multiplicity * (d81_dn14),
        );
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
        Self::stamp_transient_block_6(ctx, s, p, nodes);
        Self::stamp_transient_block_7(s, p);
        Self::stamp_transient_block_8(ctx, s, p, nodes);
        Self::stamp_transient_block_9(s, p);

        stamper.stamp_potential_branch_local(
            Some(7),
            Some(8),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(7),
            1,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            Some(2),
            2,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(0),
            3,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(3),
            4,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            None,
            5,
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_1(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_2(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_3(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_4(ctx, stamper, s, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
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
        Self::stamp_reactive_block_5(ctx, s, p, nodes);
        Self::stamp_reactive_block_6(s, p);
        Self::stamp_reactive_block_7(ctx, s, p, nodes);

        Self::stamp_reactive_equations_block_0(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_1(ctx, stamper, s, nodes, branches, multiplicity);
    }
}
