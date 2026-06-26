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
        let nv4 = ctx.node_voltage(nodes[4]);
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

        Self::stamp_transient_block_0(ctx, s, p, nodes);
        Self::stamp_transient_block_1(s);
        Self::stamp_transient_block_2(s, p);
        Self::stamp_transient_block_3(s);
        Self::stamp_transient_block_4(ctx, s, p, nodes);

        stamper.stamp_potential_branch_local(
            Some(6),
            Some(2),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(5),
            1,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(9),
            2,
            multiplicity,
        );

        let eq0_value: f64 = s.v[25];
        let eq0_node_derivatives: [f64; 12] = [s.dn[25][0], s.dn[25][1], s.dn[25][2], s.dn[25][3], s.dn[25][4], s.dn[25][5], s.dn[25][6], s.dn[25][7], s.dn[25][8], s.dn[25][9], s.dn[25][10], s.dn[25][11]];
        let eq0_branch_derivatives: [f64; 3] = [s.db[25][0], s.db[25][1], s.db[25][2]];
        stamper.stamp_potential_dense_local(
            0,
            eq0_value,
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
        );
        let eq1_value: f64 = s.v[128];
        let eq1_node_derivatives: [f64; 12] = [s.dn[128][0], s.dn[128][1], s.dn[128][2], s.dn[128][3], s.dn[128][4], s.dn[128][5], s.dn[128][6], s.dn[128][7], s.dn[128][8], s.dn[128][9], s.dn[128][10], s.dn[128][11]];
        let eq1_branch_derivatives: [f64; 3] = [s.db[128][0], s.db[128][1], s.db[128][2]];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let eq2_value: f64 = s.v[24];
        let eq2_node_derivatives: [f64; 12] = [s.dn[24][0], s.dn[24][1], s.dn[24][2], s.dn[24][3], s.dn[24][4], s.dn[24][5], s.dn[24][6], s.dn[24][7], s.dn[24][8], s.dn[24][9], s.dn[24][10], s.dn[24][11]];
        let eq2_branch_derivatives: [f64; 3] = [s.db[24][0], s.db[24][1], s.db[24][2]];
        stamper.stamp_potential_dense_local(
            1,
            eq2_value,
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
        );
        let eq3_value: f64 = 0.0;
        stamper.stamp_potential_const_local(
            2,
            eq3_value,
        );
        let (eq4_e64, eq4_e64_d_n4,) = {
    if s.b[148] {
        let eq4_e61: f64 = (p.p36 * (nv4 - 0.0));
        let eq4_e61_d_n4: f64 = p.p36;
        let eq4_e62: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, eq4_e61);
        let eq4_e62_d_n4: f64 = (eq4_e61_d_n4 * ddt_scale);
        (eq4_e62, eq4_e62_d_n4,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e64;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq4_value),
            4,
            multiplicity * (eq4_e64_d_n4),
        );
        let (eq5_e68, eq5_e68_d_n0, eq5_e68_d_n1, eq5_e68_d_n2, eq5_e68_d_n3, eq5_e68_d_n4, eq5_e68_d_n5, eq5_e68_d_n6, eq5_e68_d_n7, eq5_e68_d_n8, eq5_e68_d_n9, eq5_e68_d_n10, eq5_e68_d_n11, eq5_e68_d_b0, eq5_e68_d_b1, eq5_e68_d_b2,) = {
    if s.b[148] {
        (s.v[133], s.dn[133][0], s.dn[133][1], s.dn[133][2], s.dn[133][3], s.dn[133][4], s.dn[133][5], s.dn[133][6], s.dn[133][7], s.dn[133][8], s.dn[133][9], s.dn[133][10], s.dn[133][11], s.db[133][0], s.db[133][1], s.db[133][2],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e68;
        let eq5_node_derivatives: [f64; 12] = [eq5_e68_d_n0, eq5_e68_d_n1, eq5_e68_d_n2, eq5_e68_d_n3, eq5_e68_d_n4, eq5_e68_d_n5, eq5_e68_d_n6, eq5_e68_d_n7, eq5_e68_d_n8, eq5_e68_d_n9, eq5_e68_d_n10, eq5_e68_d_n11];
        let eq5_branch_derivatives: [f64; 3] = [eq5_e68_d_b0, eq5_e68_d_b1, eq5_e68_d_b2];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e74, eq6_e74_d_n4,) = {
    if s.b[148] {
        let eq6_e72: f64 = ((nv4 - 0.0) / p.p35);
        let eq6_e72_d_n4: f64 = (1.0 / p.p35);
        (eq6_e72, eq6_e72_d_n4,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e74;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq6_value),
            4,
            multiplicity * (eq6_e74_d_n4),
        );
        let (eq7_e81, eq7_e81_d_n4,) = {
    if (!s.b[148]) {
        let eq7_e79: f64 = ((nv4 - 0.0) * 1000000000.0);
        let eq7_e79_d_n4: f64 = 1000000000.0;
        (eq7_e79, eq7_e79_d_n4,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e81;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq7_value),
            4,
            multiplicity * (eq7_e81_d_n4),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let nv4 = ctx.node_voltage(nodes[4]);
        let multiplicity = (*self).multiplicity;
        let s = match &mut self.reactive_scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(ReactiveScratch::new_box()).as_mut(),
        };

        s.b[148] = (p.p35 != 0.0);
        s.v[148] = if s.b[148] { 1.0 } else { 0.0 };

        let (eq4_e64, eq4_e64_d_n4, eq4_e64_q, eq4_e64_q_d_n4,) = {
    if s.b[148] {
        let eq4_e61: f64 = (p.p36 * (nv4 - 0.0));
        let eq4_e61_d_n4: f64 = p.p36;
        let eq4_e62_q: f64 = eq4_e61;
        (eq4_e61, eq4_e61_d_n4, eq4_e62_q, eq4_e61_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (eq4_e64_q_d_n4),
        );
    }
}
