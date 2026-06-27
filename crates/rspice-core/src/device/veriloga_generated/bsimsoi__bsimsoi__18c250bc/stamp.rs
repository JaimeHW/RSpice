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
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
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
        let v4: f64 = 0.0;
        let v293: f64 = nv5;
        let v296: f64 = nv11;
        let v297: f64 = nv12;
        let v298: f64 = nv10;
        let v346: f64 = nv13;
        let v347: f64 = (if self.scalar_v310 { v346 } else { v4 });
        let v355: f64 = nv1;
        let v356: f64 = (v355 - v298);
        let v357: f64 = (self.scalar_v320 * v356);
        let v358: f64 = (v357 * self.scalar_v263);
        let v359: f64 = (if self.scalar_v354 { v358 } else { v4 });
        let v361: f64 = (v293 - v297);
        let v362: f64 = (self.scalar_v320 * v361);
        let v363: f64 = (v362 * self.scalar_v287);
        let v364: f64 = (if (self.scalar_v268 != 0.0) { v363 } else { v4 });
        let v365: f64 = (v293 - v296);
        let v366: f64 = (self.scalar_v320 * v365);
        let v367: f64 = (v366 * self.scalar_v288);
        let v368: f64 = (if (self.scalar_v268 != 0.0) { v367 } else { v4 });

        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            self.scalar_v324,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            self.scalar_v327,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            self.scalar_v331,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            self.scalar_v336,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            self.scalar_v339,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            None,
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            self.scalar_v342,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            self.scalar_v345,
        );
        let d347_dn13: f64 = self.scalar_v380;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (v347),
            13,
            multiplicity * (d347_dn13),
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(7),
            7,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            7,
            self.scalar_v349,
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            Some(8),
            8,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            8,
            self.scalar_v351,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(4),
            9,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            9,
            self.scalar_v352,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(10),
            10,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            10,
            self.scalar_v353,
        );
        let d359_dn1: f64 = self.scalar_v384;
        let d359_dn10: f64 = self.scalar_v385;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(10),
            multiplicity * (v359),
            1,
            multiplicity * (d359_dn1),
            10,
            multiplicity * (d359_dn10),
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(9),
            11,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            11,
            self.scalar_v360,
        );
        let d364_dn5: f64 = self.scalar_v388;
        let d364_dn12: f64 = self.scalar_v389;
        stamper.stamp_current_node2_local(
            Some(5),
            Some(12),
            multiplicity * (v364),
            5,
            multiplicity * (d364_dn5),
            12,
            multiplicity * (d364_dn12),
        );
        let d368_dn5: f64 = self.scalar_v392;
        let d368_dn11: f64 = self.scalar_v393;
        stamper.stamp_current_node2_local(
            Some(5),
            Some(11),
            multiplicity * (v368),
            5,
            multiplicity * (d368_dn5),
            11,
            multiplicity * (d368_dn11),
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(12),
            12,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            12,
            self.scalar_v369,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(11),
            13,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            13,
            self.scalar_v369,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(8),
            14,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            14,
            self.scalar_v308,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            15,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            15,
            self.scalar_v372,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            None,
            16,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            16,
            self.scalar_v375,
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            None,
            17,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            17,
            self.scalar_v377,
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            None,
            18,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            18,
            self.scalar_v379,
        );
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        Self::stamp_transient_block_0(ctx, s, p, param_given);
        Self::stamp_transient_block_1(s, p, param_given);
        Self::stamp_transient_block_2(s, p, param_given);
        Self::stamp_transient_block_3(s, p, param_given);
        Self::stamp_transient_block_4(s, p);
        Self::stamp_transient_block_5(ctx, s, p, nodes);
        Self::stamp_transient_block_6(s, p, param_given);
        Self::stamp_transient_block_7(ctx, s, p, nodes, param_given);
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

        Self::stamp_transient_equations_block_0(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_1(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_2(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_3(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_4(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_5(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_6(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_7(stamper, s, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_8(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_9(stamper, s, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_10(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_11(stamper, s, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
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

        Self::stamp_reactive_block_0(ctx, s, p, param_given);
        Self::stamp_reactive_block_1(s, p, param_given);
        Self::stamp_reactive_block_2(s, p, param_given);
        Self::stamp_reactive_block_3(s, p);
        Self::stamp_reactive_block_4(s, p);
        Self::stamp_reactive_block_5(ctx, s, p, nodes);
        Self::stamp_reactive_block_6(s, p, param_given);
        Self::stamp_reactive_block_7(ctx, s, p, nodes);
        Self::stamp_reactive_block_8(s, p);
        Self::stamp_reactive_block_9(s, p);
        Self::stamp_reactive_block_10(s, p);
        Self::stamp_reactive_block_11(s, p);
        Self::stamp_reactive_block_12(s, p);
        Self::stamp_reactive_block_13(s, p);
        Self::stamp_reactive_block_14(s, p);
        Self::stamp_reactive_block_15(s, p);
        Self::stamp_reactive_block_16(s, p);
        Self::stamp_reactive_block_17(s, p);
        Self::stamp_reactive_block_18(s, p);

        Self::stamp_reactive_equations_block_0(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_1(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_2(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_3(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_4(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_5(stamper, s, p, nodes, branches, multiplicity);
    }
}
