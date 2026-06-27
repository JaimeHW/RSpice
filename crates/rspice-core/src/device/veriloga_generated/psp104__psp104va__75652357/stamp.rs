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
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
        let v2: f64 = nv6;
        let v3: f64 = nv7;
        let v4: f64 = nv8;
        let v5: f64 = (v2 - v4);
        let v8: f64 = (v3 - v4);
        let v9: f64 = (self.scalar_v7 * v8);
        let v10: f64 = (self.scalar_v7 * v5);

        let d9_dn7: f64 = self.scalar_v7;
        let d9_dn8: f64 = self.scalar_v11;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(8),
            multiplicity * (v9),
            7,
            multiplicity * (d9_dn7),
            8,
            multiplicity * (d9_dn8),
        );
        let d10_dn6: f64 = self.scalar_v7;
        let d10_dn8: f64 = self.scalar_v11;
        stamper.stamp_current_node2_local(
            Some(6),
            Some(8),
            multiplicity * (v10),
            6,
            multiplicity * (d10_dn6),
            8,
            multiplicity * (d10_dn8),
        );
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        Self::stamp_transient_block_0(s, p);
        Self::stamp_transient_block_1(ctx, s, p);
        Self::stamp_transient_block_2(s, p, param_given);
        Self::stamp_transient_block_3(s, p, param_given);
        Self::stamp_transient_block_4(s, p, param_given);
        Self::stamp_transient_block_5(s, p, param_given);
        Self::stamp_transient_block_6(s, p);
        Self::stamp_transient_block_7(s, p);
        Self::stamp_transient_block_8(s, p);
        Self::stamp_transient_block_9(s, p);
        Self::stamp_transient_block_10(s, p);
        Self::stamp_transient_block_11(s, p);
        Self::stamp_transient_block_12(s, p);
        Self::stamp_transient_block_13(s, p);
        Self::stamp_transient_block_14(s, p);
        Self::stamp_transient_block_15(s, p);
        Self::stamp_transient_block_16(s, p);
        Self::stamp_transient_block_17(s, p);
        Self::stamp_transient_block_18(s, p);
        Self::stamp_transient_block_19(s, p);
        Self::stamp_transient_block_20(s, p);
        Self::stamp_transient_block_21(s, p);
        Self::stamp_transient_block_22(s, p);
        Self::stamp_transient_block_23(s, p);
        Self::stamp_transient_block_24(s, p);
        Self::stamp_transient_block_25(s, p);
        Self::stamp_transient_block_26(s, p);
        Self::stamp_transient_block_27(s, p);
        Self::stamp_transient_block_28(ctx, s, p, nodes);
        Self::stamp_transient_block_29(s);
        Self::stamp_transient_block_30(s);
        Self::stamp_transient_block_31(s, p);
        Self::stamp_transient_block_32(s, p);
        Self::stamp_transient_block_33(s, p);
        Self::stamp_transient_block_34(s);
        Self::stamp_transient_block_35(s, p);
        Self::stamp_transient_block_36(s);
        Self::stamp_transient_block_37(s, p);
        Self::stamp_transient_block_38(s, p);
        Self::stamp_transient_block_39(s, p);
        Self::stamp_transient_block_40(s, p);
        Self::stamp_transient_block_41(s, p);
        Self::stamp_transient_block_42(s, p);
        Self::stamp_transient_block_43(s, p);
        Self::stamp_transient_block_44(s, p);

        stamper.stamp_potential_branch_local(
            Some(1),
            Some(5),
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
            Some(8),
            Some(9),
            3,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(9),
            4,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            Some(9),
            5,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(9),
            6,
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(stamper, s, p, multiplicity);
        Self::stamp_transient_equations_block_1(stamper, s, p, multiplicity);
        Self::stamp_transient_equations_block_2(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_3(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_4(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_5(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        let eq46_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(4),
            None,
            multiplicity * (eq46_value),
        );
        Self::stamp_transient_equations_block_6(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        let eq51_e1364: f64 = (s.v[15] * p.p32);
        let eq51_e1364_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq51_e1364_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq51_e1364_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq51_e1364_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq51_e1364_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq51_e1364_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq51_e1364_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq51_e1364_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq51_e1364_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq51_e1364_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq51_e1364_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq51_e1364_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq51_e1364_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq51_e1364_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq51_e1364_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq51_e1364_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq51_e1364_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq51_e1364_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq51_e1364_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq51_e1365: f64 = (eq51_e1364).sqrt();
        let eq51_e1365_d_n0: f64 = (eq51_e1364_d_n0 / (2.0 * eq51_e1365));
        let eq51_e1365_d_n1: f64 = (eq51_e1364_d_n1 / (2.0 * eq51_e1365));
        let eq51_e1365_d_n2: f64 = (eq51_e1364_d_n2 / (2.0 * eq51_e1365));
        let eq51_e1365_d_n3: f64 = (eq51_e1364_d_n3 / (2.0 * eq51_e1365));
        let eq51_e1365_d_n4: f64 = (eq51_e1364_d_n4 / (2.0 * eq51_e1365));
        let eq51_e1365_d_n5: f64 = (eq51_e1364_d_n5 / (2.0 * eq51_e1365));
        let eq51_e1365_d_n6: f64 = (eq51_e1364_d_n6 / (2.0 * eq51_e1365));
        let eq51_e1365_d_n7: f64 = (eq51_e1364_d_n7 / (2.0 * eq51_e1365));
        let eq51_e1365_d_n8: f64 = (eq51_e1364_d_n8 / (2.0 * eq51_e1365));
        let eq51_e1365_d_n9: f64 = (eq51_e1364_d_n9 / (2.0 * eq51_e1365));
        let eq51_e1365_d_n10: f64 = (eq51_e1364_d_n10 / (2.0 * eq51_e1365));
        let eq51_e1365_d_n11: f64 = (eq51_e1364_d_n11 / (2.0 * eq51_e1365));
        let eq51_e1365_d_b0: f64 = (eq51_e1364_d_b0 / (2.0 * eq51_e1365));
        let eq51_e1365_d_b1: f64 = (eq51_e1364_d_b1 / (2.0 * eq51_e1365));
        let eq51_e1365_d_b2: f64 = (eq51_e1364_d_b2 / (2.0 * eq51_e1365));
        let eq51_e1365_d_b3: f64 = (eq51_e1364_d_b3 / (2.0 * eq51_e1365));
        let eq51_e1365_d_b4: f64 = (eq51_e1364_d_b4 / (2.0 * eq51_e1365));
        let eq51_e1365_d_b5: f64 = (eq51_e1364_d_b5 / (2.0 * eq51_e1365));
        let eq51_e1365_d_b6: f64 = (eq51_e1364_d_b6 / (2.0 * eq51_e1365));
        let eq51_e1366: f64 = (s.v[825] * eq51_e1365);
        let eq51_e1366_d_n0: f64 = ((s.dn[825][0] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n0));
        let eq51_e1366_d_n1: f64 = ((s.dn[825][1] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n1));
        let eq51_e1366_d_n2: f64 = ((s.dn[825][2] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n2));
        let eq51_e1366_d_n3: f64 = ((s.dn[825][3] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n3));
        let eq51_e1366_d_n4: f64 = ((s.dn[825][4] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n4));
        let eq51_e1366_d_n5: f64 = ((s.dn[825][5] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n5));
        let eq51_e1366_d_n6: f64 = ((s.dn[825][6] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n6));
        let eq51_e1366_d_n7: f64 = ((s.dn[825][7] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n7));
        let eq51_e1366_d_n8: f64 = ((s.dn[825][8] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n8));
        let eq51_e1366_d_n9: f64 = ((s.dn[825][9] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n9));
        let eq51_e1366_d_n10: f64 = ((s.dn[825][10] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n10));
        let eq51_e1366_d_n11: f64 = ((s.dn[825][11] * eq51_e1365) + (s.v[825] * eq51_e1365_d_n11));
        let eq51_e1366_d_b0: f64 = ((s.db[825][0] * eq51_e1365) + (s.v[825] * eq51_e1365_d_b0));
        let eq51_e1366_d_b1: f64 = ((s.db[825][1] * eq51_e1365) + (s.v[825] * eq51_e1365_d_b1));
        let eq51_e1366_d_b2: f64 = ((s.db[825][2] * eq51_e1365) + (s.v[825] * eq51_e1365_d_b2));
        let eq51_e1366_d_b3: f64 = ((s.db[825][3] * eq51_e1365) + (s.v[825] * eq51_e1365_d_b3));
        let eq51_e1366_d_b4: f64 = ((s.db[825][4] * eq51_e1365) + (s.v[825] * eq51_e1365_d_b4));
        let eq51_e1366_d_b5: f64 = ((s.db[825][5] * eq51_e1365) + (s.v[825] * eq51_e1365_d_b5));
        let eq51_e1366_d_b6: f64 = ((s.db[825][6] * eq51_e1365) + (s.v[825] * eq51_e1365_d_b6));
        let eq51_e1368: f64 = (eq51_e1366 * s.v[855]);
        let eq51_e1368_d_n0: f64 = ((eq51_e1366_d_n0 * s.v[855]) + (eq51_e1366 * s.dn[855][0]));
        let eq51_e1368_d_n1: f64 = ((eq51_e1366_d_n1 * s.v[855]) + (eq51_e1366 * s.dn[855][1]));
        let eq51_e1368_d_n2: f64 = ((eq51_e1366_d_n2 * s.v[855]) + (eq51_e1366 * s.dn[855][2]));
        let eq51_e1368_d_n3: f64 = ((eq51_e1366_d_n3 * s.v[855]) + (eq51_e1366 * s.dn[855][3]));
        let eq51_e1368_d_n4: f64 = ((eq51_e1366_d_n4 * s.v[855]) + (eq51_e1366 * s.dn[855][4]));
        let eq51_e1368_d_n5: f64 = ((eq51_e1366_d_n5 * s.v[855]) + (eq51_e1366 * s.dn[855][5]));
        let eq51_e1368_d_n6: f64 = ((eq51_e1366_d_n6 * s.v[855]) + (eq51_e1366 * s.dn[855][6]));
        let eq51_e1368_d_n7: f64 = ((eq51_e1366_d_n7 * s.v[855]) + (eq51_e1366 * s.dn[855][7]));
        let eq51_e1368_d_n8: f64 = ((eq51_e1366_d_n8 * s.v[855]) + (eq51_e1366 * s.dn[855][8]));
        let eq51_e1368_d_n9: f64 = ((eq51_e1366_d_n9 * s.v[855]) + (eq51_e1366 * s.dn[855][9]));
        let eq51_e1368_d_n10: f64 = ((eq51_e1366_d_n10 * s.v[855]) + (eq51_e1366 * s.dn[855][10]));
        let eq51_e1368_d_n11: f64 = ((eq51_e1366_d_n11 * s.v[855]) + (eq51_e1366 * s.dn[855][11]));
        let eq51_e1368_d_b0: f64 = ((eq51_e1366_d_b0 * s.v[855]) + (eq51_e1366 * s.db[855][0]));
        let eq51_e1368_d_b1: f64 = ((eq51_e1366_d_b1 * s.v[855]) + (eq51_e1366 * s.db[855][1]));
        let eq51_e1368_d_b2: f64 = ((eq51_e1366_d_b2 * s.v[855]) + (eq51_e1366 * s.db[855][2]));
        let eq51_e1368_d_b3: f64 = ((eq51_e1366_d_b3 * s.v[855]) + (eq51_e1366 * s.db[855][3]));
        let eq51_e1368_d_b4: f64 = ((eq51_e1366_d_b4 * s.v[855]) + (eq51_e1366 * s.db[855][4]));
        let eq51_e1368_d_b5: f64 = ((eq51_e1366_d_b5 * s.v[855]) + (eq51_e1366 * s.db[855][5]));
        let eq51_e1368_d_b6: f64 = ((eq51_e1366_d_b6 * s.v[855]) + (eq51_e1366 * s.db[855][6]));
        let eq51_e1370: f64 = (eq51_e1368 * eq46_value);
        let eq51_e1370_d_n0: f64 = (eq51_e1368_d_n0 * eq46_value);
        let eq51_e1370_d_n1: f64 = (eq51_e1368_d_n1 * eq46_value);
        let eq51_e1370_d_n2: f64 = (eq51_e1368_d_n2 * eq46_value);
        let eq51_e1370_d_n3: f64 = (eq51_e1368_d_n3 * eq46_value);
        let eq51_e1370_d_n4: f64 = (eq51_e1368_d_n4 * eq46_value);
        let eq51_e1370_d_n5: f64 = (eq51_e1368_d_n5 * eq46_value);
        let eq51_e1370_d_n6: f64 = (eq51_e1368_d_n6 * eq46_value);
        let eq51_e1370_d_n7: f64 = (eq51_e1368_d_n7 * eq46_value);
        let eq51_e1370_d_n8: f64 = (eq51_e1368_d_n8 * eq46_value);
        let eq51_e1370_d_n9: f64 = (eq51_e1368_d_n9 * eq46_value);
        let eq51_e1370_d_n10: f64 = (eq51_e1368_d_n10 * eq46_value);
        let eq51_e1370_d_n11: f64 = (eq51_e1368_d_n11 * eq46_value);
        let eq51_e1370_d_b0: f64 = (eq51_e1368_d_b0 * eq46_value);
        let eq51_e1370_d_b1: f64 = (eq51_e1368_d_b1 * eq46_value);
        let eq51_e1370_d_b2: f64 = (eq51_e1368_d_b2 * eq46_value);
        let eq51_e1370_d_b3: f64 = (eq51_e1368_d_b3 * eq46_value);
        let eq51_e1370_d_b4: f64 = (eq51_e1368_d_b4 * eq46_value);
        let eq51_e1370_d_b5: f64 = (eq51_e1368_d_b5 * eq46_value);
        let eq51_e1370_d_b6: f64 = (eq51_e1368_d_b6 * eq46_value);
        let eq51_value: f64 = eq51_e1370;
        let eq51_node_derivatives: [f64; 12] = [eq51_e1370_d_n0, eq51_e1370_d_n1, eq51_e1370_d_n2, eq51_e1370_d_n3, eq51_e1370_d_n4, eq51_e1370_d_n5, eq51_e1370_d_n6, eq51_e1370_d_n7, eq51_e1370_d_n8, eq51_e1370_d_n9, eq51_e1370_d_n10, eq51_e1370_d_n11];
        let eq51_branch_derivatives: [f64; 7] = [eq51_e1370_d_b0, eq51_e1370_d_b1, eq51_e1370_d_b2, eq51_e1370_d_b3, eq51_e1370_d_b4, eq51_e1370_d_b5, eq51_e1370_d_b6];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq51_value),
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
            multiplicity,
        );
        Self::stamp_transient_equations_block_7(stamper, multiplicity);
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

        Self::stamp_reactive_block_0(s, p);
        Self::stamp_reactive_block_1(ctx, s, p);
        Self::stamp_reactive_block_2(s, p, param_given);
        Self::stamp_reactive_block_3(s, p, param_given);
        Self::stamp_reactive_block_4(s, p, param_given);
        Self::stamp_reactive_block_5(s, p, param_given);
        Self::stamp_reactive_block_6(s, p);
        Self::stamp_reactive_block_7(s, p);
        Self::stamp_reactive_block_8(ctx, s, p, nodes);
        Self::stamp_reactive_block_9(ctx, s, p, nodes);
        Self::stamp_reactive_block_10(s);
        Self::stamp_reactive_block_11(s);
        Self::stamp_reactive_block_12(s, p);
        Self::stamp_reactive_block_13(s, p);
        Self::stamp_reactive_block_14(s, p);
        Self::stamp_reactive_block_15(s, p);
        Self::stamp_reactive_block_16(s);
        Self::stamp_reactive_block_17(s);
        Self::stamp_reactive_block_18(s, p);
        Self::stamp_reactive_block_19(s, p);
        Self::stamp_reactive_block_20(s, p);
        Self::stamp_reactive_block_21(s, p);
        Self::stamp_reactive_block_22(s, p);

        Self::stamp_reactive_equations_block_0(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_1(ctx, stamper, s, p, nodes, branches, multiplicity);
    }
}
