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
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        Self::stamp_transient_block_0(ctx, s, p);
        Self::stamp_transient_block_1(s, p);
        Self::stamp_transient_block_2(s, p);
        Self::stamp_transient_block_3(s, p);
        Self::stamp_transient_block_4(s, p);
        Self::stamp_transient_block_5(s, p);
        Self::stamp_transient_block_6(s, p);
        Self::stamp_transient_block_7(s, p);
        Self::stamp_transient_block_8(s, p);
        Self::stamp_transient_block_9(s, p);
        Self::stamp_transient_block_10(ctx, s, p, nodes);
        Self::stamp_transient_block_11(s, p);
        Self::stamp_transient_block_12(s, p);

        let eq0_e71: f64 = (p.p1 * s.v[0]);
        let eq0_e73: f64 = (eq0_e71 * p.p7);
        let eq0_e75: f64 = (eq0_e73 * s.v[544]);
        let eq0_e75_d_n0: f64 = (eq0_e73 * s.dn[544][0]);
        let eq0_e75_d_n1: f64 = (eq0_e73 * s.dn[544][1]);
        let eq0_value: f64 = eq0_e75;
        stamper.stamp_current_node2(
            Some(nodes[0]),
            Some(nodes[1]),
            multiplicity * (eq0_value),
            nodes[0],
            multiplicity * (eq0_e75_d_n0),
            nodes[1],
            multiplicity * (eq0_e75_d_n1),
        );
        let eq1_e78: f64 = (p.p1 * s.v[0]);
        let eq1_e80: f64 = (eq1_e78 * p.p8);
        let eq1_e82: f64 = (eq1_e80 * s.v[545]);
        let eq1_e82_d_n0: f64 = (eq1_e80 * s.dn[545][0]);
        let eq1_e82_d_n1: f64 = (eq1_e80 * s.dn[545][1]);
        let eq1_e83: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, eq1_e82);
        let eq1_e83_d_n0: f64 = (eq1_e82_d_n0 * ddt_scale);
        let eq1_e83_d_n1: f64 = (eq1_e82_d_n1 * ddt_scale);
        let eq1_value: f64 = eq1_e83;
        stamper.stamp_current_node2(
            Some(nodes[0]),
            Some(nodes[1]),
            multiplicity * (eq1_value),
            nodes[0],
            multiplicity * (eq1_e83_d_n0),
            nodes[1],
            multiplicity * (eq1_e83_d_n1),
        );
        let eq2_value: f64 = 0.0;
        stamper.stamp_current_const(
            Some(nodes[0]),
            Some(nodes[1]),
            multiplicity * (eq2_value),
        );
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
        Self::stamp_reactive_block_1(s, p);
        Self::stamp_reactive_block_2(s, p);

        let eq1_e78: f64 = (p.p1 * s.v[0]);
        let eq1_e80: f64 = (eq1_e78 * p.p8);
        let eq1_e82: f64 = (eq1_e80 * s.v[545]);
        let eq1_e82_d_n0: f64 = (eq1_e80 * s.dn[545][0]);
        let eq1_e82_d_n1: f64 = (eq1_e80 * s.dn[545][1]);
        let eq1_e83_q: f64 = eq1_e82;
        stamper.stamp_current_reactive_node2(
            Some(nodes[0]),
            Some(nodes[1]),
            nodes[0],
            multiplicity * (eq1_e82_d_n0),
            nodes[1],
            multiplicity * (eq1_e82_d_n1),
        );
    }
}
