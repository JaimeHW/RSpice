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
    older: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    derivative_current: &mut [f64; STATE_COUNT],
    derivative_previous: &mut [f64; STATE_COUNT],
    ddt_active: bool,
    ddt_scale: f64,
    ddt_previous_value_scale: f64,
    ddt_older_value_scale: f64,
    ddt_previous_derivative_scale: f64,
    slot: usize,
    value: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated ddt state slot out of range");
    let previous_value = if initialized[slot] { previous[slot] } else { value };
    let older_value = if initialized[slot] { older[slot] } else { value };
    current[slot] = value;
    if ddt_active {
        let result = value * ddt_scale
            - previous_value * ddt_previous_value_scale
            - older_value * ddt_older_value_scale
            - derivative_previous[slot] * ddt_previous_derivative_scale;
        derivative_current[slot] = result;
        result
    } else {
        current[slot] = value;
        previous[slot] = value;
        older[slot] = value;
        derivative_current[slot] = 0.0;
        derivative_previous[slot] = 0.0;
        initialized[slot] = true;
        0.0
    }
}

#[inline]
fn ddt_jacobian(ddt_active: bool, ddt_scale: f64, derivative: f64) -> f64 {
    if ddt_active {
        derivative * ddt_scale
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
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_older = self.ddt_state_older.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_derivative_current = self.ddt_derivative_current.as_mut();
        let ddt_derivative_previous = self.ddt_derivative_previous.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_scale = self.ddt_coefficients.derivative_scale;
        let ddt_previous_value_scale = self.ddt_coefficients.previous_value_scale;
        let ddt_older_value_scale = self.ddt_coefficients.older_value_scale;
        let ddt_previous_derivative_scale = self.ddt_coefficients.previous_derivative_scale;
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        Self::stamp_transient_block_0(ctx, s, p, nodes, param_given);
        Self::stamp_transient_block_1(s, p);

        let eq0_value: f64 = s.v[1];
        stamper.stamp_current_node3_local(
            Some(0),
            Some(1),
            multiplicity * (eq0_value),
            0,
            multiplicity * (s.dn[1][0]),
            1,
            multiplicity * (s.dn[1][1]),
            2,
            multiplicity * (s.dn[1][2]),
        );
        let (eq1_e56, eq1_e56_d_n0, eq1_e56_d_n1, eq1_e56_d_n2,) = {
    if (p.p7 != 0.0) {
        (s.v[44], s.dn[44][0], s.dn[44][1], s.dn[44][2],)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e56;
        stamper.stamp_current_node3_local(
            Some(2),
            None,
            multiplicity * (eq1_value),
            0,
            multiplicity * (eq1_e56_d_n0),
            1,
            multiplicity * (eq1_e56_d_n1),
            2,
            multiplicity * (eq1_e56_d_n2),
        );
        let (eq2_e60, eq2_e60_d_n0, eq2_e60_d_n1, eq2_e60_d_n2,) = {
    if (p.p7 != 0.0) {
        (s.v[43], s.dn[43][0], s.dn[43][1], s.dn[43][2],)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e60;
        stamper.stamp_current_node3_local(
            Some(2),
            None,
            multiplicity * (eq2_value),
            0,
            multiplicity * (eq2_e60_d_n0),
            1,
            multiplicity * (eq2_e60_d_n1),
            2,
            multiplicity * (eq2_e60_d_n2),
        );
        let (eq3_e67, eq3_e67_d_n0, eq3_e67_d_n1, eq3_e67_d_n2,) = {
    if (p.p7 == 0.0) {
        let eq3_e65: f64 = (1000000.0 * s.v[42]);
        (eq3_e65, (1000000.0 * s.dn[42][0]), (1000000.0 * s.dn[42][1]), (1000000.0 * s.dn[42][2]),)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e67;
        stamper.stamp_current_node3_local(
            Some(2),
            None,
            multiplicity * (eq3_value),
            0,
            multiplicity * (eq3_e67_d_n0),
            1,
            multiplicity * (eq3_e67_d_n1),
            2,
            multiplicity * (eq3_e67_d_n2),
        );
        let (eq4_e72, eq4_e72_d_n0, eq4_e72_d_n1, eq4_e72_d_n2,) = {
    if (p.p7 != 0.0) {
        let eq4_e70: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, s.v[45]);
        (eq4_e70, (s.dn[45][0] * ddt_scale), (s.dn[45][1] * ddt_scale), (s.dn[45][2] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e72;
        stamper.stamp_current_node3_local(
            Some(2),
            None,
            multiplicity * (eq4_value),
            0,
            multiplicity * (eq4_e72_d_n0),
            1,
            multiplicity * (eq4_e72_d_n1),
            2,
            multiplicity * (eq4_e72_d_n2),
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

        s.b[51] = param_given[10];
        s.store_scalar(51, if s.b[51] { 1.0 } else { 0.0 });

        if s.b[51] {
            s.store_scalar(13, p.p10);
        }

        if (!s.b[51]) {
            s.store_scalar(13, 1.0);
        }

        s.b[52] = param_given[11];
        s.store_scalar(52, if s.b[52] { 1.0 } else { 0.0 });

        if s.b[52] {
            s.store_scalar(14, (1.0 - (0.01 * p.p11)));
        }

        if (!s.b[52]) {
            s.store_scalar(14, 1.0);
        }

        s.store_scaled_mul(18, 14, 13, 1000000.0);

        s.b[56] = ((p.p3 != 0.0) && (p.p4 != 0.0));
        s.store_scalar(56, if s.b[56] { 1.0 } else { 0.0 });

        if s.b[56] {
            s.store_scalar(17, p.p23);
        }

        s.b[57] = ((p.p3 != 0.0) || (p.p4 != 0.0));
        s.store_scalar(57, if s.b[57] { 1.0 } else { 0.0 });

        if ((!s.b[56]) && s.b[57]) {
            s.store_scalar(17, (p.p23 * 0.5));
        }

        if ((!s.b[56]) && (!s.b[57])) {
            s.store_scalar(17, 0.0);
        }

        s.b[58] = ((param_given[1] && param_given[2]) && (!param_given[0]));
        s.store_scalar(58, if s.b[58] { 1.0 } else { 0.0 });

        s.b[59] = ((p.p2 == 0.0) || (p.p1 == 0.0));
        s.store_scalar(59, if s.b[59] { 1.0 } else { 0.0 });

        if (s.b[58] && s.b[59]) {
            s.store_scalar(19, 0.0);
            s.store_scalar(3, 0.0);
            s.store_scale(20, 18, p.p0);
            s.store_offset(4, 20, p.p22);
        }

        if (s.b[58] && (!s.b[59])) {
            s.store_scale(19, 18, p.p1);
            s.store_add(3, 19, 17);
        }

        s.b[61] = (s.v[3] > 0.0);
        s.store_scalar(61, if s.b[61] { 1.0 } else { 0.0 });

        if ((s.b[58] && (!s.b[59])) && s.b[61]) {
            s.store_scale(4, 3, (p.p17 / p.p2));
            s.store_offset(20, 4, (-p.p22));
        }

        if ((s.b[58] && (!s.b[59])) && (!s.b[61])) {
            s.store_scale(20, 18, p.p0);
            s.store_offset(4, 20, p.p22);
        }

        s.b[63] = (param_given[2] && (!param_given[1]));
        s.store_scalar(63, if s.b[63] { 1.0 } else { 0.0 });

        s.b[64] = (p.p2 == 0.0);
        s.store_scalar(64, if s.b[64] { 1.0 } else { 0.0 });

        if (((!s.b[58]) && s.b[63]) && s.b[64]) {
            s.store_scalar(19, 0.0);
            s.store_scalar(3, 0.0);
            s.store_scale(20, 18, p.p0);
            s.store_offset(4, 20, p.p22);
        }

        s.b[65] = (p.p0 == 0.0);
        s.store_scalar(65, if s.b[65] { 1.0 } else { 0.0 });

        if ((((!s.b[58]) && s.b[63]) && (!s.b[64])) && s.b[65]) {
            s.store_scalar(20, 0.0);
            s.store_scalar(4, 0.0);
            s.store_scale(19, 18, p.p1);
            s.store_add(3, 19, 17);
        }

        if ((((!s.b[58]) && s.b[63]) && (!s.b[64])) && (!s.b[65])) {
            s.store_scale(20, 18, p.p0);
            s.store_offset(4, 20, p.p22);
        }

        s.b[67] = (s.v[4] > 0.0);
        s.store_scalar(67, if s.b[67] { 1.0 } else { 0.0 });

        if (((((!s.b[58]) && s.b[63]) && (!s.b[64])) && (!s.b[65])) && s.b[67]) {
            s.store_scale(3, 4, (p.p2 / p.p17));
            s.store_sub(19, 3, 17);
        }

        if (((((!s.b[58]) && s.b[63]) && (!s.b[64])) && (!s.b[65])) && (!s.b[67])) {
            s.store_scale(19, 18, p.p1);
            s.store_add(3, 19, 17);
        }

        s.b[69] = (p.p0 == 0.0);
        s.store_scalar(69, if s.b[69] { 1.0 } else { 0.0 });

        if (((!s.b[58]) && (!s.b[63])) && s.b[69]) {
            s.store_scalar(20, 0.0);
            s.store_scalar(4, 0.0);
            s.store_scale(19, 18, p.p1);
            s.store_add(3, 19, 17);
        }

        s.b[70] = (p.p1 == 0.0);
        s.store_scalar(70, if s.b[70] { 1.0 } else { 0.0 });

        if ((((!s.b[58]) && (!s.b[63])) && (!s.b[69])) && s.b[70]) {
            s.store_scalar(19, 0.0);
            s.store_scalar(3, 0.0);
            s.store_scale(20, 18, p.p0);
            s.store_offset(4, 20, p.p22);
        }

        if ((((!s.b[58]) && (!s.b[63])) && (!s.b[69])) && (!s.b[70])) {
            s.store_scale(20, 18, p.p0);
            s.store_offset(4, 20, p.p22);
            s.store_scale(19, 18, p.p1);
            s.store_add(3, 19, 17);
        }

        s.b[85] = ((p.p3 != 0.0) && (p.p4 != 0.0));
        s.store_scalar(85, if s.b[85] { 1.0 } else { 0.0 });

        if s.b[85] {
            s.store_scaled_add(46, 19, 20, 2.0);
        }

        s.b[86] = ((p.p3 != 0.0) || (p.p4 != 0.0));
        s.store_scalar(86, if s.b[86] { 1.0 } else { 0.0 });

        if ((!s.b[85]) && s.b[86]) {
            s.store_add_scaled_inputs(46, 19, 2.0, 20, 1.0);
        }

        if ((!s.b[85]) && (!s.b[86])) {
            s.store_scale(46, 19, 2.0);
        }

        s.store_mul(47, 19, 20);

        s.store_add_scaled_ad_lhs(9, A::scale_offset(s.ad_value(46), p.p48, p.p47), 47, p.p49);

        s.store_voltage(42, ctx, nodes, Some(2), None);

        s.store_mul(45, 42, 9);

        let (eq4_e72, eq4_e72_d_n0, eq4_e72_d_n1, eq4_e72_d_n2, eq4_e72_q,) = {
    if (p.p7 != 0.0) {
        let eq4_e70_q: f64 = s.v[45];
        (s.v[45], s.dn[45][0], s.dn[45][1], s.dn[45][2], eq4_e70_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node3(
            Some(nodes[2]),
            None,
            nodes[0],
            multiplicity * (eq4_e72_d_n0),
            nodes[1],
            multiplicity * (eq4_e72_d_n1),
            nodes[2],
            multiplicity * (eq4_e72_d_n2),
        );
    }
}
