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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
        let v2: f64 = 0.0;
        let v139: f64 = nv9;
        let v140: f64 = nv6;
        let v141: f64 = nv7;
        let v142: f64 = (v141 - v140);
        let v143: f64 = nv8;
        let v147: f64 = (self.scalar_v146 * v142);
        let v149: f64 = nv1;
        let v150: f64 = (v149 - v139);
        let v151: f64 = (self.scalar_v148 * v150);
        let v152: f64 = (if self.scalar_v119 { v151 } else { v2 });
        let v155: f64 = nv2;
        let v156: f64 = (v155 - v140);
        let v157: f64 = (self.scalar_v154 * v156);
        let v158: f64 = (if self.scalar_v124 { v157 } else { v2 });
        let v161: f64 = nv0;
        let v162: f64 = (v161 - v141);
        let v163: f64 = (self.scalar_v160 * v162);
        let v164: f64 = (if self.scalar_v129 { v163 } else { v2 });
        let v167: f64 = nv3;
        let v168: f64 = (v167 - v143);
        let v169: f64 = (self.scalar_v166 * v168);
        let v170: f64 = (if self.scalar_v134 { v169 } else { v2 });

        stamper.stamp_current_const_local(
            Some(6),
            Some(8),
            multiplicity * (v2),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(8),
            multiplicity * (v2),
        );
        stamper.stamp_current_const_local(
            Some(9),
            Some(8),
            multiplicity * (v2),
        );
        let d147_dn6: f64 = self.scalar_v172;
        let d147_dn7: f64 = self.scalar_v146;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(6),
            multiplicity * (v147),
            6,
            multiplicity * (d147_dn6),
            7,
            multiplicity * (d147_dn7),
        );
        let d152_dn1: f64 = self.scalar_v174;
        let d152_dn9: f64 = self.scalar_v175;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(9),
            multiplicity * (v152),
            1,
            multiplicity * (d152_dn1),
            9,
            multiplicity * (d152_dn9),
        );
        let d158_dn2: f64 = self.scalar_v177;
        let d158_dn6: f64 = self.scalar_v178;
        stamper.stamp_current_node2_local(
            Some(2),
            Some(6),
            multiplicity * (v158),
            2,
            multiplicity * (d158_dn2),
            6,
            multiplicity * (d158_dn6),
        );
        let d164_dn0: f64 = self.scalar_v180;
        let d164_dn7: f64 = self.scalar_v181;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(7),
            multiplicity * (v164),
            0,
            multiplicity * (d164_dn0),
            7,
            multiplicity * (d164_dn7),
        );
        let d170_dn3: f64 = self.scalar_v183;
        let d170_dn8: f64 = self.scalar_v184;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(8),
            multiplicity * (v170),
            3,
            multiplicity * (d170_dn3),
            8,
            multiplicity * (d170_dn8),
        );
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        Self::stamp_transient_block_0(ctx, s, p);
        Self::stamp_transient_block_1(s, p, param_given);
        Self::stamp_transient_block_2(s, p);
        Self::stamp_transient_block_3(s, p, param_given);
        Self::stamp_transient_block_4(s, p);
        Self::stamp_transient_block_5(ctx, s, p, nodes);
        Self::stamp_transient_block_6(ctx, s, p, nodes);
        Self::stamp_transient_block_7(s, p);
        Self::stamp_transient_block_8(s);
        Self::stamp_transient_block_9(s, p);
        Self::stamp_transient_block_10(s);
        Self::stamp_transient_block_11(s);
        Self::stamp_transient_block_12(s);
        Self::stamp_transient_block_13(s, p);
        Self::stamp_transient_block_14(s, p);
        Self::stamp_transient_block_15(s, p);
        Self::stamp_transient_block_16(s, p);
        Self::stamp_transient_block_17(s, p);
        Self::stamp_transient_block_18(s, p);
        Self::stamp_transient_block_19(ctx, s, p, nodes);
        Self::stamp_transient_block_20(s, p);
        Self::stamp_transient_block_21(s);
        Self::stamp_transient_block_22(s, p);
        Self::stamp_transient_block_23(s);
        Self::stamp_transient_block_24(s);
        Self::stamp_transient_block_25(s);
        Self::stamp_transient_block_26(s);
        Self::stamp_transient_block_27(s, p);
        Self::stamp_transient_block_28(s, p);
        Self::stamp_transient_block_29(s, p);
        Self::stamp_transient_block_30(s, p);
        Self::stamp_transient_block_31(s, p);

        stamper.stamp_potential_branch_local(
            Some(1),
            Some(9),
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
            Some(0),
            Some(7),
            2,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(8),
            3,
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_1(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        let eq30_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            None,
            multiplicity * (eq30_value),
        );
        Self::stamp_transient_equations_block_2(ctx, stamper, s, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        let eq36_e707: f64 = (s.v[330] * s.v[1795]);
        let eq36_e707_d_n0: f64 = ((s.dn[330][0] * s.v[1795]) + (s.v[330] * s.dn[1795][0]));
        let eq36_e707_d_n1: f64 = ((s.dn[330][1] * s.v[1795]) + (s.v[330] * s.dn[1795][1]));
        let eq36_e707_d_n2: f64 = ((s.dn[330][2] * s.v[1795]) + (s.v[330] * s.dn[1795][2]));
        let eq36_e707_d_n3: f64 = ((s.dn[330][3] * s.v[1795]) + (s.v[330] * s.dn[1795][3]));
        let eq36_e707_d_n4: f64 = ((s.dn[330][4] * s.v[1795]) + (s.v[330] * s.dn[1795][4]));
        let eq36_e707_d_n5: f64 = ((s.dn[330][5] * s.v[1795]) + (s.v[330] * s.dn[1795][5]));
        let eq36_e707_d_n6: f64 = ((s.dn[330][6] * s.v[1795]) + (s.v[330] * s.dn[1795][6]));
        let eq36_e707_d_n7: f64 = ((s.dn[330][7] * s.v[1795]) + (s.v[330] * s.dn[1795][7]));
        let eq36_e707_d_n8: f64 = ((s.dn[330][8] * s.v[1795]) + (s.v[330] * s.dn[1795][8]));
        let eq36_e707_d_n9: f64 = ((s.dn[330][9] * s.v[1795]) + (s.v[330] * s.dn[1795][9]));
        let eq36_e707_d_b0: f64 = ((s.db[330][0] * s.v[1795]) + (s.v[330] * s.db[1795][0]));
        let eq36_e707_d_b1: f64 = ((s.db[330][1] * s.v[1795]) + (s.v[330] * s.db[1795][1]));
        let eq36_e707_d_b2: f64 = ((s.db[330][2] * s.v[1795]) + (s.v[330] * s.db[1795][2]));
        let eq36_e707_d_b3: f64 = ((s.db[330][3] * s.v[1795]) + (s.v[330] * s.db[1795][3]));
        let eq36_e709: f64 = (eq36_e707 * eq30_value);
        let eq36_e709_d_n0: f64 = (eq36_e707_d_n0 * eq30_value);
        let eq36_e709_d_n1: f64 = (eq36_e707_d_n1 * eq30_value);
        let eq36_e709_d_n2: f64 = (eq36_e707_d_n2 * eq30_value);
        let eq36_e709_d_n3: f64 = (eq36_e707_d_n3 * eq30_value);
        let eq36_e709_d_n4: f64 = (eq36_e707_d_n4 * eq30_value);
        let eq36_e709_d_n5: f64 = (eq36_e707_d_n5 * eq30_value);
        let eq36_e709_d_n6: f64 = (eq36_e707_d_n6 * eq30_value);
        let eq36_e709_d_n7: f64 = (eq36_e707_d_n7 * eq30_value);
        let eq36_e709_d_n8: f64 = (eq36_e707_d_n8 * eq30_value);
        let eq36_e709_d_n9: f64 = (eq36_e707_d_n9 * eq30_value);
        let eq36_e709_d_b0: f64 = (eq36_e707_d_b0 * eq30_value);
        let eq36_e709_d_b1: f64 = (eq36_e707_d_b1 * eq30_value);
        let eq36_e709_d_b2: f64 = (eq36_e707_d_b2 * eq30_value);
        let eq36_e709_d_b3: f64 = (eq36_e707_d_b3 * eq30_value);
        let eq36_value: f64 = eq36_e709;
        let eq36_node_derivatives: [f64; 10] = [eq36_e709_d_n0, eq36_e709_d_n1, eq36_e709_d_n2, eq36_e709_d_n3, eq36_e709_d_n4, eq36_e709_d_n5, eq36_e709_d_n6, eq36_e709_d_n7, eq36_e709_d_n8, eq36_e709_d_n9];
        let eq36_branch_derivatives: [f64; 4] = [eq36_e709_d_b0, eq36_e709_d_b1, eq36_e709_d_b2, eq36_e709_d_b3];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq36_value),
            &eq36_node_derivatives,
            &eq36_branch_derivatives,
            multiplicity,
        );
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

        Self::stamp_reactive_block_0(ctx, s, p);
        Self::stamp_reactive_block_1(s, p, param_given);
        Self::stamp_reactive_block_2(s, p, param_given);
        Self::stamp_reactive_block_3(s, p, param_given);
        Self::stamp_reactive_block_4(s, p);
        Self::stamp_reactive_block_5(ctx, s, p, nodes);
        Self::stamp_reactive_block_6(ctx, s, p, nodes);
        Self::stamp_reactive_block_7(s, p);
        Self::stamp_reactive_block_8(s);
        Self::stamp_reactive_block_9(s, p);
        Self::stamp_reactive_block_10(s);
        Self::stamp_reactive_block_11(s);
        Self::stamp_reactive_block_12(s);
        Self::stamp_reactive_block_13(s, p);
        Self::stamp_reactive_block_14(s, p);
        Self::stamp_reactive_block_15(s, p);
        Self::stamp_reactive_block_16(s, p);
        Self::stamp_reactive_block_17(s, p);
        Self::stamp_reactive_block_18(s, p);
        Self::stamp_reactive_block_19(s, p);
        Self::stamp_reactive_block_20(s);
        Self::stamp_reactive_block_21(s);
        Self::stamp_reactive_block_22(s, p);
        Self::stamp_reactive_block_23(s);
        Self::stamp_reactive_block_24(s);
        Self::stamp_reactive_block_25(s);
        Self::stamp_reactive_block_26(s, p);
        Self::stamp_reactive_block_27(s, p);
        Self::stamp_reactive_block_28(s, p);
        Self::stamp_reactive_block_29(s, p);
        Self::stamp_reactive_block_30(s, p);

        Self::stamp_reactive_equations_block_0(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_1(ctx, stamper, s, p, nodes, branches, multiplicity);
    }
}
