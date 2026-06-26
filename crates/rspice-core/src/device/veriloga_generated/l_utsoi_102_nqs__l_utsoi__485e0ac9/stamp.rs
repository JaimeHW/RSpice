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
        let p = &self.params;
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let param_given = &self.param_given;
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = &mut self.ddt_state_current;
        let ddt_state_previous = &mut self.ddt_state_previous;
        let ddt_state_initialized = &mut self.ddt_state_initialized;
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Box::new(Scratch::new())).as_mut(),
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
        Self::stamp_transient_block_9(s);
        Self::stamp_transient_block_10(s, p);
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
        Self::stamp_transient_block_22(s);
        Self::stamp_transient_block_23(s, p);
        Self::stamp_transient_block_24(s);
        Self::stamp_transient_block_25(s);
        Self::stamp_transient_block_26(s);
        Self::stamp_transient_block_27(s, p);
        Self::stamp_transient_block_28(s, p);
        Self::stamp_transient_block_29(s, p);
        Self::stamp_transient_block_30(s, p);
        Self::stamp_transient_block_31(s, p);

        stamper.stamp_potential_branch(
            Some(nodes[1]),
            Some(nodes[9]),
            branches[0],
            multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[2]),
            Some(nodes[6]),
            branches[1],
            multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[0]),
            Some(nodes[7]),
            branches[2],
            multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[3]),
            Some(nodes[8]),
            branches[3],
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_transient_equations_block_1(ctx, stamper, s, p, nodes, branches, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_2(stamper, s, p, nodes, branches, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_3(ctx, stamper, s, p, nodes, branches, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        let eq39_value: f64 = 0.0;
        stamper.stamp_current_const(
            Some(nodes[5]),
            None,
            multiplicity * (eq39_value),
        );
        Self::stamp_transient_equations_block_4(ctx, stamper, s, nodes, branches, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        let eq45_e779: f64 = (s.v[334] * s.v[1805]);
        let eq45_e779_d_n0: f64 = ((s.dn[334][0] * s.v[1805]) + (s.v[334] * s.dn[1805][0]));
        let eq45_e779_d_n1: f64 = ((s.dn[334][1] * s.v[1805]) + (s.v[334] * s.dn[1805][1]));
        let eq45_e779_d_n2: f64 = ((s.dn[334][2] * s.v[1805]) + (s.v[334] * s.dn[1805][2]));
        let eq45_e779_d_n3: f64 = ((s.dn[334][3] * s.v[1805]) + (s.v[334] * s.dn[1805][3]));
        let eq45_e779_d_n4: f64 = ((s.dn[334][4] * s.v[1805]) + (s.v[334] * s.dn[1805][4]));
        let eq45_e779_d_n5: f64 = ((s.dn[334][5] * s.v[1805]) + (s.v[334] * s.dn[1805][5]));
        let eq45_e779_d_n6: f64 = ((s.dn[334][6] * s.v[1805]) + (s.v[334] * s.dn[1805][6]));
        let eq45_e779_d_n7: f64 = ((s.dn[334][7] * s.v[1805]) + (s.v[334] * s.dn[1805][7]));
        let eq45_e779_d_n8: f64 = ((s.dn[334][8] * s.v[1805]) + (s.v[334] * s.dn[1805][8]));
        let eq45_e779_d_n9: f64 = ((s.dn[334][9] * s.v[1805]) + (s.v[334] * s.dn[1805][9]));
        let eq45_e779_d_n10: f64 = ((s.dn[334][10] * s.v[1805]) + (s.v[334] * s.dn[1805][10]));
        let eq45_e779_d_n11: f64 = ((s.dn[334][11] * s.v[1805]) + (s.v[334] * s.dn[1805][11]));
        let eq45_e779_d_n12: f64 = ((s.dn[334][12] * s.v[1805]) + (s.v[334] * s.dn[1805][12]));
        let eq45_e779_d_n13: f64 = ((s.dn[334][13] * s.v[1805]) + (s.v[334] * s.dn[1805][13]));
        let eq45_e781: f64 = (eq45_e779 * eq39_value);
        let eq45_e781_d_n0: f64 = (eq45_e779_d_n0 * eq39_value);
        let eq45_e781_d_n1: f64 = (eq45_e779_d_n1 * eq39_value);
        let eq45_e781_d_n2: f64 = (eq45_e779_d_n2 * eq39_value);
        let eq45_e781_d_n3: f64 = (eq45_e779_d_n3 * eq39_value);
        let eq45_e781_d_n4: f64 = (eq45_e779_d_n4 * eq39_value);
        let eq45_e781_d_n5: f64 = (eq45_e779_d_n5 * eq39_value);
        let eq45_e781_d_n6: f64 = (eq45_e779_d_n6 * eq39_value);
        let eq45_e781_d_n7: f64 = (eq45_e779_d_n7 * eq39_value);
        let eq45_e781_d_n8: f64 = (eq45_e779_d_n8 * eq39_value);
        let eq45_e781_d_n9: f64 = (eq45_e779_d_n9 * eq39_value);
        let eq45_e781_d_n10: f64 = (eq45_e779_d_n10 * eq39_value);
        let eq45_e781_d_n11: f64 = (eq45_e779_d_n11 * eq39_value);
        let eq45_e781_d_n12: f64 = (eq45_e779_d_n12 * eq39_value);
        let eq45_e781_d_n13: f64 = (eq45_e779_d_n13 * eq39_value);
        let eq45_value: f64 = eq45_e781;
        let eq45_node_derivatives: [f64; 14] = [eq45_e781_d_n0, eq45_e781_d_n1, eq45_e781_d_n2, eq45_e781_d_n3, eq45_e781_d_n4, eq45_e781_d_n5, eq45_e781_d_n6, eq45_e781_d_n7, eq45_e781_d_n8, eq45_e781_d_n9, eq45_e781_d_n10, eq45_e781_d_n11, eq45_e781_d_n12, eq45_e781_d_n13];
        let eq45_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            multiplicity * (eq45_value),
            nodes,
            &eq45_node_derivatives,
            branches,
            &eq45_branch_derivatives,
            multiplicity,
        );
        Self::stamp_transient_equations_block_5(stamper, nodes, multiplicity);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = &self.params;
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let param_given = &self.param_given;
        let multiplicity = (*self).multiplicity;
        let s = match &mut self.reactive_scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Box::new(ReactiveScratch::new())).as_mut(),
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

        Self::stamp_reactive_equations_block_0(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_1(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_2(ctx, stamper, s, nodes, branches, multiplicity);
    }
}
