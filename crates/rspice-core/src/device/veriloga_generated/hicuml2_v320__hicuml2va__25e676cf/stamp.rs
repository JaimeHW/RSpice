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
        let nv5 = ctx.node_voltage(nodes[5]);
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
        let v0: f64 = nv5;
        let v5: f64 = nv9;
        let v6: f64 = (v5 - v0);
        let v7: f64 = nv3;
        let v11: f64 = 0.0;
        let v124: f64 = (v11 * v6);
        let v125: f64 = (if self.scalar_v123 { v124 } else { v11 });
        let v128: f64 = (if self.scalar_v127 { v124 } else { v11 });
        let v129: f64 = (v5 - v7);
        let v130: f64 = (v129 / self.scalar_v90);
        let v131: f64 = (if self.scalar_v93 { v130 } else { v11 });
        let v136: f64 = nv13;
        let v137: f64 = (-v136);
        let v138: f64 = (if self.scalar_v104 { v137 } else { v11 });
        let v139: f64 = (if self.scalar_v104 { v136 } else { v11 });
        let v140: f64 = nv14;
        let v141: f64 = (-v140);
        let v142: f64 = (if self.scalar_v104 { v141 } else { v11 });
        let v143: f64 = (if self.scalar_v104 { v140 } else { v11 });
        let v145: f64 = (if self.scalar_v144 { v136 } else { v11 });
        let v146: f64 = (if self.scalar_v144 { v140 } else { v11 });

        let d125_dn5: f64 = self.scalar_v166;
        let d125_dn9: f64 = self.scalar_v167;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(5),
            multiplicity * (v125),
            5,
            multiplicity * (d125_dn5),
            9,
            multiplicity * (d125_dn9),
        );
        let d128_dn5: f64 = self.scalar_v168;
        let d128_dn9: f64 = self.scalar_v169;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(5),
            multiplicity * (v128),
            5,
            multiplicity * (d128_dn5),
            9,
            multiplicity * (d128_dn9),
        );
        let d131_dn3: f64 = self.scalar_v172;
        let d131_dn9: f64 = self.scalar_v173;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(3),
            multiplicity * (v131),
            3,
            multiplicity * (d131_dn3),
            9,
            multiplicity * (d131_dn9),
        );
        let d138_dn13: f64 = self.scalar_v174;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (v138),
            13,
            multiplicity * (d138_dn13),
        );
        let d139_dn13: f64 = self.scalar_v105;
        stamper.stamp_current_node1_local(
            Some(8),
            Some(6),
            multiplicity * (v139),
            13,
            multiplicity * (d139_dn13),
        );
        let d142_dn14: f64 = self.scalar_v174;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (v142),
            14,
            multiplicity * (d142_dn14),
        );
        let d143_dn14: f64 = self.scalar_v105;
        stamper.stamp_current_node1_local(
            Some(5),
            Some(6),
            multiplicity * (v143),
            14,
            multiplicity * (d143_dn14),
        );
        let d145_dn13: f64 = self.scalar_v175;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (v145),
            13,
            multiplicity * (d145_dn13),
        );
        let d146_dn14: f64 = self.scalar_v175;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (v146),
            14,
            multiplicity * (d146_dn14),
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
        Self::stamp_transient_block_9(s);

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
