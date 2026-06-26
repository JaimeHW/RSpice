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
        let p = &self.params;
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let bi0 = ctx.branch_current(branches[0]);
        let bi1 = ctx.branch_current(branches[1]);
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

        Self::stamp_transient_block_0(ctx, s, p, nodes, multiplicity);
        Self::stamp_transient_block_1(s, p);
        Self::stamp_transient_block_2(ctx, s, p, nodes, branches);
        Self::stamp_transient_block_3(s, p);

        stamper.stamp_potential_branch(
            Some(nodes[0]),
            Some(nodes[4]),
            branches[0],
            multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[2]),
            Some(nodes[5]),
            branches[1],
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(stamper, s, nodes, branches, multiplicity);
        let (eq5_e153, eq5_e153_d_n0, eq5_e153_d_n1, eq5_e153_d_n2, eq5_e153_d_n3, eq5_e153_d_n4, eq5_e153_d_n5, eq5_e153_d_b0, eq5_e153_d_b1,) = {
    if s.b[321] {
        let eq5_e149: f64 = (bi0 * s.v[54]);
        let eq5_e149_d_n0: f64 = (bi0 * s.dn[54][0]);
        let eq5_e149_d_n1: f64 = (bi0 * s.dn[54][1]);
        let eq5_e149_d_n2: f64 = (bi0 * s.dn[54][2]);
        let eq5_e149_d_n3: f64 = (bi0 * s.dn[54][3]);
        let eq5_e149_d_n4: f64 = (bi0 * s.dn[54][4]);
        let eq5_e149_d_n5: f64 = (bi0 * s.dn[54][5]);
        let eq5_e149_d_b0: f64 = (s.v[54] + (bi0 * s.db[54][0]));
        let eq5_e149_d_b1: f64 = (bi0 * s.db[54][1]);
        let eq5_e151: f64 = (eq5_e149 * s.v[58]);
        let eq5_e151_d_n0: f64 = ((eq5_e149_d_n0 * s.v[58]) + (eq5_e149 * s.dn[58][0]));
        let eq5_e151_d_n1: f64 = ((eq5_e149_d_n1 * s.v[58]) + (eq5_e149 * s.dn[58][1]));
        let eq5_e151_d_n2: f64 = ((eq5_e149_d_n2 * s.v[58]) + (eq5_e149 * s.dn[58][2]));
        let eq5_e151_d_n3: f64 = ((eq5_e149_d_n3 * s.v[58]) + (eq5_e149 * s.dn[58][3]));
        let eq5_e151_d_n4: f64 = ((eq5_e149_d_n4 * s.v[58]) + (eq5_e149 * s.dn[58][4]));
        let eq5_e151_d_n5: f64 = ((eq5_e149_d_n5 * s.v[58]) + (eq5_e149 * s.dn[58][5]));
        let eq5_e151_d_b0: f64 = ((eq5_e149_d_b0 * s.v[58]) + (eq5_e149 * s.db[58][0]));
        let eq5_e151_d_b1: f64 = ((eq5_e149_d_b1 * s.v[58]) + (eq5_e149 * s.db[58][1]));
        (eq5_e151, eq5_e151_d_n0, eq5_e151_d_n1, eq5_e151_d_n2, eq5_e151_d_n3, eq5_e151_d_n4, eq5_e151_d_n5, eq5_e151_d_b0, eq5_e151_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e153;
        let eq5_node_derivatives: [f64; 6] = [eq5_e153_d_n0, eq5_e153_d_n1, eq5_e153_d_n2, eq5_e153_d_n3, eq5_e153_d_n4, eq5_e153_d_n5];
        let eq5_branch_derivatives: [f64; 2] = [eq5_e153_d_b0, eq5_e153_d_b1];
        stamper.stamp_potential_dense(
            branches[0],
            eq5_value,
            nodes,
            &eq5_node_derivatives,
            branches,
            &eq5_branch_derivatives,
        );
        Self::stamp_transient_equations_block_1(ctx, stamper, s, nodes, branches, multiplicity);
        let (eq7_e170, eq7_e170_d_n0, eq7_e170_d_n1, eq7_e170_d_n2, eq7_e170_d_n3, eq7_e170_d_n4, eq7_e170_d_n5, eq7_e170_d_b0, eq7_e170_d_b1,) = {
    if s.b[322] {
        let eq7_e166: f64 = (bi1 * s.v[55]);
        let eq7_e166_d_n0: f64 = (bi1 * s.dn[55][0]);
        let eq7_e166_d_n1: f64 = (bi1 * s.dn[55][1]);
        let eq7_e166_d_n2: f64 = (bi1 * s.dn[55][2]);
        let eq7_e166_d_n3: f64 = (bi1 * s.dn[55][3]);
        let eq7_e166_d_n4: f64 = (bi1 * s.dn[55][4]);
        let eq7_e166_d_n5: f64 = (bi1 * s.dn[55][5]);
        let eq7_e166_d_b0: f64 = (bi1 * s.db[55][0]);
        let eq7_e166_d_b1: f64 = (s.v[55] + (bi1 * s.db[55][1]));
        let eq7_e168: f64 = (eq7_e166 * s.v[58]);
        let eq7_e168_d_n0: f64 = ((eq7_e166_d_n0 * s.v[58]) + (eq7_e166 * s.dn[58][0]));
        let eq7_e168_d_n1: f64 = ((eq7_e166_d_n1 * s.v[58]) + (eq7_e166 * s.dn[58][1]));
        let eq7_e168_d_n2: f64 = ((eq7_e166_d_n2 * s.v[58]) + (eq7_e166 * s.dn[58][2]));
        let eq7_e168_d_n3: f64 = ((eq7_e166_d_n3 * s.v[58]) + (eq7_e166 * s.dn[58][3]));
        let eq7_e168_d_n4: f64 = ((eq7_e166_d_n4 * s.v[58]) + (eq7_e166 * s.dn[58][4]));
        let eq7_e168_d_n5: f64 = ((eq7_e166_d_n5 * s.v[58]) + (eq7_e166 * s.dn[58][5]));
        let eq7_e168_d_b0: f64 = ((eq7_e166_d_b0 * s.v[58]) + (eq7_e166 * s.db[58][0]));
        let eq7_e168_d_b1: f64 = ((eq7_e166_d_b1 * s.v[58]) + (eq7_e166 * s.db[58][1]));
        (eq7_e168, eq7_e168_d_n0, eq7_e168_d_n1, eq7_e168_d_n2, eq7_e168_d_n3, eq7_e168_d_n4, eq7_e168_d_n5, eq7_e168_d_b0, eq7_e168_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e170;
        let eq7_node_derivatives: [f64; 6] = [eq7_e170_d_n0, eq7_e170_d_n1, eq7_e170_d_n2, eq7_e170_d_n3, eq7_e170_d_n4, eq7_e170_d_n5];
        let eq7_branch_derivatives: [f64; 2] = [eq7_e170_d_b0, eq7_e170_d_b1];
        stamper.stamp_potential_dense(
            branches[1],
            eq7_value,
            nodes,
            &eq7_node_derivatives,
            branches,
            &eq7_branch_derivatives,
        );
        Self::stamp_transient_equations_block_2(ctx, stamper, s, p, nodes, branches, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = &self.params;
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let multiplicity = (*self).multiplicity;
        let s = match &mut self.reactive_scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Box::new(ReactiveScratch::new())).as_mut(),
        };

        Self::stamp_reactive_block_0(ctx, s, p, nodes, multiplicity);
        Self::stamp_reactive_block_1(s, p);

        Self::stamp_reactive_equations_block_0(stamper, s, nodes, branches, multiplicity);
    }
}
