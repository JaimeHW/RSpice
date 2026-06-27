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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
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
        let v1: f64 = nv7;
        let v2: f64 = (v1 - v0);
        let v3: f64 = nv1;
        let v4: f64 = (v3 - v0);
        let v5: f64 = nv9;
        let v6: f64 = (v5 - v0);
        let v7: f64 = nv3;
        let v8: f64 = nv0;
        let v11: f64 = 0.0;
        let v13: f64 = 3.0;
        let v56: f64 = nv10;
        let v57: f64 = (if self.scalar_v55 { v56 } else { v11 });
        let v58: f64 = nv11;
        let v59: f64 = (if self.scalar_v55 { v58 } else { v11 });
        let v60: f64 = (self.scalar_v33 * v57);
        let v61: f64 = (v60 * self.scalar_v38);
        let v62: f64 = (if self.scalar_v55 { v61 } else { v11 });
        let v63: f64 = (self.scalar_v33 * v59);
        let v64: f64 = (v63 / v13);
        let v65: f64 = (v64 * self.scalar_v38);
        let v66: f64 = (if self.scalar_v55 { v65 } else { v11 });
        let v67: f64 = nv12;
        let v68: f64 = (if self.scalar_v55 { v67 } else { v11 });
        let v69: f64 = (self.scalar_v35 * v68);
        let v70: f64 = (v69 * self.scalar_v38);
        let v71: f64 = (if self.scalar_v55 { v70 } else { v11 });
        let v73: f64 = (if self.scalar_v72 { v11 } else { v62 });
        let v74: f64 = (if self.scalar_v72 { v11 } else { v66 });
        let v75: f64 = (if self.scalar_v72 { v11 } else { v71 });
        let v98: f64 = nv2;
        let v106: f64 = (v3 - v98);
        let v107: f64 = (v8 - v98);
        let v110: f64 = (self.scalar_v26 * v2);
        let v111: f64 = (self.scalar_v24 * v4);
        let v118: f64 = (v1 - v98);
        let v119: f64 = (self.scalar_v29 * v118);
        let v120: f64 = (self.scalar_v30 * v106);
        let v122: f64 = (self.scalar_v121 * v107);
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
        let d110_dn5: f64 = self.scalar_v160;
        let d110_dn7: f64 = self.scalar_v26;
        let v110_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, v110);
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * (v110_ddt),
            5,
            multiplicity * (((d110_dn5) * ddt_scale)),
            7,
            multiplicity * (((d110_dn7) * ddt_scale)),
        );
        let d111_dn1: f64 = self.scalar_v24;
        let d111_dn5: f64 = self.scalar_v161;
        let v111_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, v111);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(5),
            multiplicity * (v111_ddt),
            1,
            multiplicity * (((d111_dn1) * ddt_scale)),
            5,
            multiplicity * (((d111_dn5) * ddt_scale)),
        );
        let d119_dn2: f64 = self.scalar_v162;
        let d119_dn7: f64 = self.scalar_v29;
        let v119_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, v119);
        stamper.stamp_current_node2_local(
            Some(7),
            Some(2),
            multiplicity * (v119_ddt),
            2,
            multiplicity * (((d119_dn2) * ddt_scale)),
            7,
            multiplicity * (((d119_dn7) * ddt_scale)),
        );
        let d120_dn1: f64 = self.scalar_v30;
        let d120_dn2: f64 = self.scalar_v163;
        let v120_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, v120);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (v120_ddt),
            1,
            multiplicity * (((d120_dn1) * ddt_scale)),
            2,
            multiplicity * (((d120_dn2) * ddt_scale)),
        );
        let d122_dn0: f64 = self.scalar_v121;
        let d122_dn2: f64 = self.scalar_v164;
        let v122_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, v122);
        stamper.stamp_current_node2_local(
            Some(0),
            Some(2),
            multiplicity * (v122_ddt),
            0,
            multiplicity * (((d122_dn0) * ddt_scale)),
            2,
            multiplicity * (((d122_dn2) * ddt_scale)),
        );
        let d73_dn10: f64 = self.scalar_v157;
        let v73_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 15, v73);
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (v73_ddt),
            10,
            multiplicity * (((d73_dn10) * ddt_scale)),
        );
        let d74_dn11: f64 = self.scalar_v158;
        let v74_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 16, v74);
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (v74_ddt),
            11,
            multiplicity * (((d74_dn11) * ddt_scale)),
        );
        let d75_dn12: f64 = self.scalar_v159;
        let v75_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 17, v75);
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (v75_ddt),
            12,
            multiplicity * (((d75_dn12) * ddt_scale)),
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
        Self::stamp_transient_equations_block_3(ctx, stamper, s, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let multiplicity = (*self).multiplicity;
        let v0: f64 = nv5;
        let v1: f64 = nv7;
        let v2: f64 = (v1 - v0);
        let v3: f64 = nv1;
        let v4: f64 = (v3 - v0);
        let v8: f64 = nv0;
        let v11: f64 = 0.0;
        let v13: f64 = 3.0;
        let v56: f64 = nv10;
        let v57: f64 = (if self.scalar_v55 { v56 } else { v11 });
        let v58: f64 = nv11;
        let v59: f64 = (if self.scalar_v55 { v58 } else { v11 });
        let v60: f64 = (self.scalar_v33 * v57);
        let v61: f64 = (v60 * self.scalar_v38);
        let v62: f64 = (if self.scalar_v55 { v61 } else { v11 });
        let v63: f64 = (self.scalar_v33 * v59);
        let v64: f64 = (v63 / v13);
        let v65: f64 = (v64 * self.scalar_v38);
        let v66: f64 = (if self.scalar_v55 { v65 } else { v11 });
        let v67: f64 = nv12;
        let v68: f64 = (if self.scalar_v55 { v67 } else { v11 });
        let v69: f64 = (self.scalar_v35 * v68);
        let v70: f64 = (v69 * self.scalar_v38);
        let v71: f64 = (if self.scalar_v55 { v70 } else { v11 });
        let v73: f64 = (if self.scalar_v72 { v11 } else { v62 });
        let v74: f64 = (if self.scalar_v72 { v11 } else { v66 });
        let v75: f64 = (if self.scalar_v72 { v11 } else { v71 });
        let v98: f64 = nv2;
        let v106: f64 = (v3 - v98);
        let v107: f64 = (v8 - v98);
        let v110: f64 = (self.scalar_v26 * v2);
        let v111: f64 = (self.scalar_v24 * v4);
        let v118: f64 = (v1 - v98);
        let v119: f64 = (self.scalar_v29 * v118);
        let v120: f64 = (self.scalar_v30 * v106);
        let v122: f64 = (self.scalar_v121 * v107);

        let d110_dn5: f64 = self.scalar_v160;
        let d110_dn7: f64 = self.scalar_v26;
        stamper.stamp_current_reactive_node2(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes[5],
            multiplicity * (d110_dn5),
            nodes[7],
            multiplicity * (d110_dn7),
        );
        let d111_dn1: f64 = self.scalar_v24;
        let d111_dn5: f64 = self.scalar_v161;
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[5]),
            nodes[1],
            multiplicity * (d111_dn1),
            nodes[5],
            multiplicity * (d111_dn5),
        );
        let d119_dn2: f64 = self.scalar_v162;
        let d119_dn7: f64 = self.scalar_v29;
        stamper.stamp_current_reactive_node2(
            Some(nodes[7]),
            Some(nodes[2]),
            nodes[2],
            multiplicity * (d119_dn2),
            nodes[7],
            multiplicity * (d119_dn7),
        );
        let d120_dn1: f64 = self.scalar_v30;
        let d120_dn2: f64 = self.scalar_v163;
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (d120_dn1),
            nodes[2],
            multiplicity * (d120_dn2),
        );
        let d122_dn0: f64 = self.scalar_v121;
        let d122_dn2: f64 = self.scalar_v164;
        stamper.stamp_current_reactive_node2(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes[0],
            multiplicity * (d122_dn0),
            nodes[2],
            multiplicity * (d122_dn2),
        );
        let d73_dn10: f64 = self.scalar_v157;
        stamper.stamp_current_reactive_node1(
            Some(nodes[10]),
            None,
            nodes[10],
            multiplicity * (d73_dn10),
        );
        let d74_dn11: f64 = self.scalar_v158;
        stamper.stamp_current_reactive_node1(
            Some(nodes[11]),
            None,
            nodes[11],
            multiplicity * (d74_dn11),
        );
        let d75_dn12: f64 = self.scalar_v159;
        stamper.stamp_current_reactive_node1(
            Some(nodes[12]),
            None,
            nodes[12],
            multiplicity * (d75_dn12),
        );
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
