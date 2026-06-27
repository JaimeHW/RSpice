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
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
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
        let v1: f64 = 1.0;
        let v3: f64 = 0.0;
        let v4: f64 = 1e-12;
        let v19: f64 = 10000.0;
        let v36: f64 = nv12;
        let v38: f64 = nv18;
        let v39: f64 = nv13;
        let v43: f64 = nv15;
        let v44: f64 = nv16;
        let v48: f64 = nv10;
        let v56: f64 = nv17;
        let v67: f64 = nv14;
        let v70: f64 = (v48 * v4);
        let v71: f64 = (if self.scalar_v60 { v70 } else { v3 });
        let v73: f64 = (v48 * v19);
        let v74: f64 = (if self.scalar_v72 { v73 } else { v3 });
        let v79: f64 = nv9;
        let v80: f64 = (v79 - v36);
        let v81: f64 = (self.scalar_v34 * v80);
        let v82: f64 = (if self.scalar_v78 { v81 } else { v3 });
        let v83: f64 = nv8;
        let v84: f64 = (v83 - v36);
        let v85: f64 = (self.scalar_v25 * v84);
        let v86: f64 = (if self.scalar_v78 { v85 } else { v3 });
        let v90: f64 = (v38 * v4);
        let v91: f64 = (if self.scalar_v37 { v90 } else { v3 });
        let v92: f64 = (v39 * v4);
        let v93: f64 = (if self.scalar_v37 { v92 } else { v3 });
        let v96: f64 = (v56 * v4);
        let v97: f64 = (if self.scalar_v95 { v96 } else { v3 });
        let v103: f64 = (if self.scalar_v102 { v96 } else { v3 });
        let v107: f64 = (v43 * v4);
        let v108: f64 = (if self.scalar_v42 { v107 } else { v3 });
        let v109: f64 = (v44 * v4);
        let v110: f64 = (if self.scalar_v42 { v109 } else { v3 });
        let v111: f64 = (if self.scalar_v42 { v92 } else { v3 });

        let d67_dn14: f64 = v1;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (v67),
            14,
            multiplicity * (d67_dn14),
        );
        let d71_dn10: f64 = self.scalar_v114;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (v71),
            10,
            multiplicity * (d71_dn10),
        );
        let d74_dn10: f64 = self.scalar_v115;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (v74),
            10,
            multiplicity * (d74_dn10),
        );
        let d82_dn9: f64 = self.scalar_v117;
        let d82_dn12: f64 = self.scalar_v118;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(12),
            multiplicity * (v82),
            9,
            multiplicity * (d82_dn9),
            12,
            multiplicity * (d82_dn12),
        );
        let d86_dn8: f64 = self.scalar_v120;
        let d86_dn12: f64 = self.scalar_v121;
        stamper.stamp_current_node2_local(
            Some(8),
            Some(12),
            multiplicity * (v86),
            8,
            multiplicity * (d86_dn8),
            12,
            multiplicity * (d86_dn12),
        );
        let d91_dn18: f64 = self.scalar_v122;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (v91),
            18,
            multiplicity * (d91_dn18),
        );
        let d93_dn13: f64 = self.scalar_v122;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (v93),
            13,
            multiplicity * (d93_dn13),
        );
        let d97_dn17: f64 = self.scalar_v123;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (v97),
            17,
            multiplicity * (d97_dn17),
        );
        let d103_dn17: f64 = self.scalar_v124;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (v103),
            17,
            multiplicity * (d103_dn17),
        );
        let d108_dn15: f64 = self.scalar_v125;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (v108),
            15,
            multiplicity * (d108_dn15),
        );
        let d110_dn16: f64 = self.scalar_v125;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (v110),
            16,
            multiplicity * (d110_dn16),
        );
        let d111_dn13: f64 = self.scalar_v125;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (v111),
            13,
            multiplicity * (d111_dn13),
        );
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        Self::stamp_transient_block_0(s, p, param_given);
        Self::stamp_transient_block_1(ctx, s, p, nodes, param_given);
        Self::stamp_transient_block_2(ctx, s, p, nodes);
        Self::stamp_transient_block_3(s, p);
        Self::stamp_transient_block_4(s, p);
        Self::stamp_transient_block_5(s);
        Self::stamp_transient_block_6(s, p);
        Self::stamp_transient_block_7(ctx, s, p, nodes);
        Self::stamp_transient_block_8(s);
        Self::stamp_transient_block_9(s);
        Self::stamp_transient_block_10(s);
        Self::stamp_transient_block_11(s);
        Self::stamp_transient_block_12(s);
        Self::stamp_transient_block_13(ctx, s, p, nodes);
        Self::stamp_transient_block_14(s, p);
        Self::stamp_transient_block_15(s);
        Self::stamp_transient_block_16(s, p);
        Self::stamp_transient_block_17(s, p);
        Self::stamp_transient_block_18(s, p);
        Self::stamp_transient_block_19(s, p);
        Self::stamp_transient_block_20(s, p);
        Self::stamp_transient_block_21(ctx, s, p, nodes);
        Self::stamp_transient_block_22(s, p);
        Self::stamp_transient_block_23(s, p);
        Self::stamp_transient_block_24(s, p);
        Self::stamp_transient_block_25(s, p);
        Self::stamp_transient_block_26(s, p);
        Self::stamp_transient_block_27(s, p);
        Self::stamp_transient_block_28(s, p);
        Self::stamp_transient_block_29(s, p);
        Self::stamp_transient_block_30(ctx, s, p, nodes);
        Self::stamp_transient_block_31(ctx, s, p, nodes);
        Self::stamp_transient_block_32(s, p);

        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(2),
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
            Some(1),
            Some(11),
            4,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(12),
            5,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(12),
            6,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(12),
            7,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(18),
            None,
            8,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            None,
            9,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(17),
            None,
            10,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(12),
            11,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(17),
            None,
            12,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(15),
            None,
            13,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(16),
            None,
            14,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            None,
            15,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(18),
            None,
            16,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(15),
            None,
            17,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(16),
            None,
            18,
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_1(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_2(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_3(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_4(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
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
        Self::stamp_reactive_block_5(s);
        Self::stamp_reactive_block_6(s, p);
        Self::stamp_reactive_block_7(ctx, s, p, nodes);
        Self::stamp_reactive_block_8(s);
        Self::stamp_reactive_block_9(s);
        Self::stamp_reactive_block_10(s);
        Self::stamp_reactive_block_11(s);
        Self::stamp_reactive_block_12(s);
        Self::stamp_reactive_block_13(ctx, s, p, nodes);
        Self::stamp_reactive_block_14(s, p);
        Self::stamp_reactive_block_15(s);
        Self::stamp_reactive_block_16(s, p);
        Self::stamp_reactive_block_17(s, p);
        Self::stamp_reactive_block_18(s, p);
        Self::stamp_reactive_block_19(s, p);
        Self::stamp_reactive_block_20(s, p);
        Self::stamp_reactive_block_21(ctx, s, p, nodes);
        Self::stamp_reactive_block_22(s, p);
        Self::stamp_reactive_block_23(s, p);
        Self::stamp_reactive_block_24(s, p);
        Self::stamp_reactive_block_25(s, p);
        Self::stamp_reactive_block_26(s, p);
        Self::stamp_reactive_block_27(s, p);
        Self::stamp_reactive_block_28(s, p);
        Self::stamp_reactive_block_29(s, p);
        Self::stamp_reactive_block_30(ctx, s, p, nodes);
        Self::stamp_reactive_block_31(s, p);

        Self::stamp_reactive_equations_block_0(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_1(ctx, stamper, s, p, nodes, branches, multiplicity);
    }
}
