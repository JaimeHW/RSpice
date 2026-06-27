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
        let v55: f64 = nv17;
        let v66: f64 = nv14;
        let v69: f64 = (v4 * v48);
        let v70: f64 = (if self.scalar_v59 { v69 } else { v3 });
        let v72: f64 = (v19 * v48);
        let v73: f64 = (if self.scalar_v71 { v72 } else { v3 });
        let v78: f64 = nv9;
        let v79: f64 = (v78 - v36);
        let v80: f64 = (self.scalar_v34 * v79);
        let v81: f64 = (if self.scalar_v77 { v80 } else { v3 });
        let v82: f64 = nv8;
        let v83: f64 = (v82 - v36);
        let v84: f64 = (self.scalar_v25 * v83);
        let v85: f64 = (if self.scalar_v77 { v84 } else { v3 });
        let v89: f64 = (v4 * v38);
        let v90: f64 = (if self.scalar_v37 { v89 } else { v3 });
        let v91: f64 = (v4 * v39);
        let v92: f64 = (if self.scalar_v37 { v91 } else { v3 });
        let v95: f64 = (v4 * v55);
        let v96: f64 = (if self.scalar_v94 { v95 } else { v3 });
        let v102: f64 = (if self.scalar_v101 { v95 } else { v3 });
        let v106: f64 = (v4 * v43);
        let v107: f64 = (if self.scalar_v42 { v106 } else { v3 });
        let v108: f64 = (v4 * v44);
        let v109: f64 = (if self.scalar_v42 { v108 } else { v3 });
        let v110: f64 = (if self.scalar_v42 { v91 } else { v3 });

        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            v3,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            v3,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(2),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            self.scalar_v63,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(6),
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            self.scalar_v65,
        );
        let d66_dn14: f64 = v1;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (v66),
            14,
            multiplicity * (d66_dn14),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(11),
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            self.scalar_v68,
        );
        let d70_dn10: f64 = self.scalar_v113;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (v70),
            10,
            multiplicity * (d70_dn10),
        );
        let d73_dn10: f64 = self.scalar_v114;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (v73),
            10,
            multiplicity * (d73_dn10),
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(12),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            self.scalar_v76,
        );
        let d81_dn9: f64 = self.scalar_v116;
        let d81_dn12: f64 = self.scalar_v117;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(12),
            multiplicity * (v81),
            9,
            multiplicity * (d81_dn9),
            12,
            multiplicity * (d81_dn12),
        );
        let d85_dn8: f64 = self.scalar_v119;
        let d85_dn12: f64 = self.scalar_v120;
        stamper.stamp_current_node2_local(
            Some(8),
            Some(12),
            multiplicity * (v85),
            8,
            multiplicity * (d85_dn8),
            12,
            multiplicity * (d85_dn12),
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(12),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            self.scalar_v88,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(12),
            7,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            7,
            self.scalar_v88,
        );
        let d90_dn18: f64 = self.scalar_v121;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (v90),
            18,
            multiplicity * (d90_dn18),
        );
        let d92_dn13: f64 = self.scalar_v121;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (v92),
            13,
            multiplicity * (d92_dn13),
        );
        stamper.stamp_potential_branch_local(
            Some(18),
            None,
            8,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            8,
            self.scalar_v93,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            None,
            9,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            9,
            self.scalar_v93,
        );
        let d96_dn17: f64 = self.scalar_v122;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (v96),
            17,
            multiplicity * (d96_dn17),
        );
        stamper.stamp_potential_branch_local(
            Some(17),
            None,
            10,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            10,
            self.scalar_v99,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(12),
            11,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            11,
            self.scalar_v100,
        );
        let d102_dn17: f64 = self.scalar_v123;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (v102),
            17,
            multiplicity * (d102_dn17),
        );
        stamper.stamp_potential_branch_local(
            Some(17),
            None,
            12,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            12,
            self.scalar_v105,
        );
        let d107_dn15: f64 = self.scalar_v124;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (v107),
            15,
            multiplicity * (d107_dn15),
        );
        let d109_dn16: f64 = self.scalar_v124;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (v109),
            16,
            multiplicity * (d109_dn16),
        );
        let d110_dn13: f64 = self.scalar_v124;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (v110),
            13,
            multiplicity * (d110_dn13),
        );
        stamper.stamp_potential_branch_local(
            Some(15),
            None,
            13,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            13,
            self.scalar_v111,
        );
        stamper.stamp_potential_branch_local(
            Some(16),
            None,
            14,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            14,
            self.scalar_v111,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            None,
            15,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            15,
            self.scalar_v111,
        );
        stamper.stamp_potential_branch_local(
            Some(18),
            None,
            16,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            16,
            self.scalar_v49,
        );
        stamper.stamp_potential_branch_local(
            Some(15),
            None,
            17,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            17,
            self.scalar_v112,
        );
        stamper.stamp_potential_branch_local(
            Some(16),
            None,
            18,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            18,
            self.scalar_v112,
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
        Self::stamp_transient_block_6(s);
        Self::stamp_transient_block_7(s, p);
        Self::stamp_transient_block_8(s, p);
        Self::stamp_transient_block_9(ctx, s, p, nodes);
        Self::stamp_transient_block_10(s);
        Self::stamp_transient_block_11(s);
        Self::stamp_transient_block_12(s);
        Self::stamp_transient_block_13(s);
        Self::stamp_transient_block_14(s);
        Self::stamp_transient_block_15(ctx, s, p, nodes);
        Self::stamp_transient_block_16(s, p);
        Self::stamp_transient_block_17(s);
        Self::stamp_transient_block_18(s, p);
        Self::stamp_transient_block_19(s, p);
        Self::stamp_transient_block_20(s, p);
        Self::stamp_transient_block_21(s, p);
        Self::stamp_transient_block_22(s, p);
        Self::stamp_transient_block_23(s, p);
        Self::stamp_transient_block_24(ctx, s, p, nodes);
        Self::stamp_transient_block_25(s, p);
        Self::stamp_transient_block_26(s, p);
        Self::stamp_transient_block_27(s, p);
        Self::stamp_transient_block_28(s, p);
        Self::stamp_transient_block_29(s, p);
        Self::stamp_transient_block_30(s, p);
        Self::stamp_transient_block_31(s, p);
        Self::stamp_transient_block_32(s, p);
        Self::stamp_transient_block_33(s, p);
        Self::stamp_transient_block_34(ctx, s, p, nodes);
        Self::stamp_transient_block_35(s, p);

        Self::stamp_transient_equations_block_0(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_1(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_2(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
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
