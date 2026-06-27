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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
        let v0: f64 = nv12;
        let v1: f64 = nv8;
        let v2: f64 = nv10;
        let v5: f64 = nv11;
        let v6: f64 = nv4;
        let v7: f64 = nv16;
        let v8: f64 = 0.0;
        let v10: f64 = 1.0;
        let v11: f64 = 1e-12;
        let v52: f64 = nv6;
        let v53: f64 = (v52 - v6);
        let v54: f64 = (v11 * v53);
        let v60: f64 = nv14;
        let v63: f64 = (v60 - v1);
        let v64: f64 = (v63 / self.scalar_v23);
        let v65: f64 = (if self.scalar_v24 { v64 } else { v8 });
        let v68: f64 = nv13;
        let v69: f64 = (v68 - v2);
        let v70: f64 = (v69 / self.scalar_v25);
        let v71: f64 = (if self.scalar_v26 { v70 } else { v8 });
        let v74: f64 = (v68 - v5);
        let v75: f64 = (v74 / self.scalar_v27);
        let v76: f64 = (if self.scalar_v28 { v75 } else { v8 });
        let v85: f64 = 1e-15;
        let v86: f64 = nv2;
        let v87: f64 = (v0 - v86);
        let v88: f64 = (v87 * v11);
        let v89: f64 = nv17;
        let v90: f64 = (if self.scalar_v41 { v89 } else { v8 });
        let v91: f64 = nv18;
        let v92: f64 = (if self.scalar_v41 { v91 } else { v8 });
        let v93: f64 = nv3;
        let v95: f64 = (v93 * v11);
        let v96: f64 = (if self.scalar_v94 { v95 } else { v8 });
        let v99: f64 = -1e-12;

        let d7_dn16: f64 = v10;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (v7),
            16,
            multiplicity * (d7_dn16),
        );
        let d7_dn16: f64 = v10;
        stamper.stamp_current_node1_local(
            Some(5),
            Some(8),
            multiplicity * (v7),
            16,
            multiplicity * (d7_dn16),
        );
        let d54_dn4: f64 = v99;
        let d54_dn6: f64 = v11;
        stamper.stamp_current_node2_local(
            Some(6),
            Some(4),
            multiplicity * (v54),
            4,
            multiplicity * (d54_dn4),
            6,
            multiplicity * (d54_dn6),
        );
        let d65_dn8: f64 = self.scalar_v103;
        let d65_dn14: f64 = self.scalar_v104;
        stamper.stamp_current_node2_local(
            Some(14),
            Some(8),
            multiplicity * (v65),
            8,
            multiplicity * (d65_dn8),
            14,
            multiplicity * (d65_dn14),
        );
        let d71_dn10: f64 = self.scalar_v107;
        let d71_dn13: f64 = self.scalar_v108;
        stamper.stamp_current_node2_local(
            Some(13),
            Some(10),
            multiplicity * (v71),
            10,
            multiplicity * (d71_dn10),
            13,
            multiplicity * (d71_dn13),
        );
        let d76_dn11: f64 = self.scalar_v111;
        let d76_dn13: f64 = self.scalar_v112;
        stamper.stamp_current_node2_local(
            Some(13),
            Some(11),
            multiplicity * (v76),
            11,
            multiplicity * (d76_dn11),
            13,
            multiplicity * (d76_dn13),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(2),
            multiplicity * (v85),
        );
        stamper.stamp_current_const_local(
            Some(14),
            Some(2),
            multiplicity * (v11),
        );
        let d88_dn2: f64 = v99;
        let d88_dn12: f64 = v11;
        stamper.stamp_current_node2_local(
            Some(12),
            Some(2),
            multiplicity * (v88),
            2,
            multiplicity * (d88_dn2),
            12,
            multiplicity * (d88_dn12),
        );
        let d90_dn17: f64 = self.scalar_v113;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (v90),
            17,
            multiplicity * (d90_dn17),
        );
        let d92_dn18: f64 = self.scalar_v113;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (v92),
            18,
            multiplicity * (d92_dn18),
        );
        let d90_dn17: f64 = self.scalar_v113;
        stamper.stamp_current_node1_local(
            Some(7),
            Some(8),
            multiplicity * (v90),
            17,
            multiplicity * (d90_dn17),
        );
        let d89_dn17: f64 = v10;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (v89),
            17,
            multiplicity * (d89_dn17),
        );
        let d91_dn18: f64 = v10;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (v91),
            18,
            multiplicity * (d91_dn18),
        );
        let d96_dn3: f64 = self.scalar_v114;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (v96),
            3,
            multiplicity * (d96_dn3),
        );
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        Self::stamp_transient_block_0(ctx, s, p, nodes, param_given);
        Self::stamp_transient_block_1(ctx, s, p, branches, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);

        stamper.stamp_potential_branch_local(
            Some(15),
            Some(16),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            1,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            2,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            Some(8),
            3,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            Some(8),
            4,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            Some(10),
            5,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            Some(11),
            6,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            7,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            8,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            9,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(7),
            10,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            11,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            12,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            13,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(2),
            14,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            15,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            16,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            17,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            Some(0),
            18,
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(ctx, stamper, s, p, nodes, branches, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_1(ctx, stamper, s, p, nodes, branches, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_2(ctx, stamper, s, p, nodes, branches, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_3(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
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

        Self::stamp_reactive_block_0(ctx, s, p, nodes, param_given);
        Self::stamp_reactive_block_1(ctx, s, p, branches);

        Self::stamp_reactive_equations_block_0(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_1(ctx, stamper, s, p, nodes, branches, multiplicity);
    }
}
