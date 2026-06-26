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
        let param_given = self.param_given.as_ref();
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

        Self::stamp_transient_block_0(ctx, s, p, nodes, param_given);
        Self::stamp_transient_block_1(s, p);

        stamper.stamp_potential_branch_local(
            Some(12),
            Some(13),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(5),
            1,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(5),
            2,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(7),
            3,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            4,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            5,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            6,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            7,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            8,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            9,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            10,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            11,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            12,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            Some(2),
            13,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            14,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            15,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            16,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            17,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            18,
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(ctx, stamper, s, p, nodes, branches, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_1(ctx, stamper, s, p, nodes, branches, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_2(ctx, stamper, s, p, nodes, branches, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_3(ctx, stamper, s, p, nodes, multiplicity);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let ctx_temp = ctx.temperature();
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let s = match &mut self.reactive_scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(ReactiveScratch::new_box()).as_mut(),
        };

        s.store_voltage(4, ctx, nodes, Some(8), Some(5));

        s.store_voltage(5, ctx, nodes, Some(3), Some(5));

        s.copy_ad(79, 4);

        s.store_voltage(80, ctx, nodes, Some(7), Some(3));

        s.v[21] = 0.0;

        s.v[20] = 0.0;

        s.v[19] = 0.0;

        s.v[18] = 0.0;

        s.b[82] = param_given[3];
        s.v[82] = if s.b[82] { 1.0 } else { 0.0 };

        if s.b[82] {
            s.store_scalar(11, (p.p3 + 273.15));
        }

        if (!s.b[82]) {
            s.store_scalar(11, (ctx_temp + p.p2));
        }

        s.b[83] = param_given[85];
        s.v[83] = if s.b[83] { 1.0 } else { 0.0 };

        if s.b[83] {
            s.store_scalar(10, (p.p85 + 273.15));
        }

        if (!s.b[83]) {
            s.store_scalar(10, (27.0 + 273.15));
        }

        if (p.p1 != 0.0) {
            s.store_add_ad_rhs(11, 11, A::abs(A::voltage(ctx, nodes, Some(11), None)));
        }

        s.store_abs_ad(12, A::sub(s.ad_value(11), s.ad_value(10)));

        s.b[84] = ((s.v[12] > 0.0) || (p.p57 > 0.0));
        s.v[84] = if s.b[84] { 1.0 } else { 0.0 };

        if s.b[84] {
            s.store_offset_scaled(31, 12, ((p.p61) * (p.p25)), p.p25);
            s.store_offset_scaled(32, 12, ((p.p62) * (p.p28)), p.p28);
            s.store_offset_scaled(34, 12, ((p.p65) * (p.p54)), p.p54);
            s.store_offset_scaled(40, 12, (p.p30 * p.p68), p.p29);
            s.store_offset_scaled(41, 12, (p.p36 * p.p68), p.p35);
        }

        if (!s.b[84]) {
            s.store_scalar(31, p.p25);
            s.store_scalar(32, p.p28);
            s.store_scalar(34, p.p54);
            s.store_scalar(40, p.p29);
            s.store_scalar(41, p.p35);
        }

        s.store_add_scaled_inputs3(22, s.ad_value(40), 1.0, s.ad_value(79), p.p30, s.ad_value(5), p.p37);

        s.store_offset_tanh_ad(67, s.ad_value(22), 1.0);

        s.store_offset_scaled(23, 5, p.p32, p.p31);

        s.store_offset_tanh_ad(68, s.ad_value(23), 1.0);

        s.store_sub_from_scalar_ad(24, p.p33, A::scale(s.ad_value(5), p.p34));

        s.store_offset_tanh_ad(69, s.ad_value(24), ((1.0) + ((-p.p37))));

        s.store_add_scaled_inputs3(25, s.ad_value(41), 1.0, s.ad_value(80), p.p36, s.ad_value(5), (-p.p37));

        s.store_offset_tanh_ad(70, s.ad_value(25), 1.0);

        s.b[94] = (p.p6 == 0.0);
        s.v[94] = if s.b[94] { 1.0 } else { 0.0 };

        s.b[95] = (p.p6 == 1.0);
        s.v[95] = if s.b[95] { 1.0 } else { 0.0 };

        s.b[96] = (p.p6 == 2.0);
        s.v[96] = if s.b[96] { 1.0 } else { 0.0 };

        if s.b[94] {
            s.store_scalar(18, p.p24);
            s.store_scalar(19, p.p26);
        }

        if (s.b[95] && (!s.b[94])) {
            s.store_offset_ad(18, A::mul3(s.ad_value(31), s.ad_value(67), s.ad_value(68)), p.p24);
            s.store_offset_ad(19, A::mul_offset_rhs(s.ad_value(32), A::mul(s.ad_value(69), s.ad_value(70)), (2.0 * p.p37)), p.p26);
        }

        if (s.b[96] && (!(s.b[94] || s.b[95]))) {
            s.store_offset(68, 68, (-p.p37));
            s.store_cosh_ad(71, A::add_scaled_inputs(s.ad_value(40), 1.0, s.ad_value(5), p.p37));
            s.store_ln(74, 71);
            s.store_cosh(72, 22);
            s.store_ln(73, 72);
            s.store_add_scaled_inputs3(77, s.ad_value(40), 1.0, s.ad_value(5), p.p37, s.ad_value(74), 1.0);
            s.store_add_scaled_product_right_ad(20, 79, p.p24, 31, A::add_scaled_product(s.ad_value(79), (2.0 * p.p37), A::add_scaled_inputs3(s.ad_value(22), 1.0, s.ad_value(73), 1.0, s.ad_value(77), -1.0), s.ad_value(68), 1.0 / (p.p30)), 1.0);
            s.store_cosh_ad(71, A::sub_scaled_inputs(s.ad_value(41), 1.0, s.ad_value(5), p.p37));
            s.store_ln(76, 71);
            s.store_cosh(72, 25);
            s.store_ln(75, 72);
            s.store_add_scaled_inputs3(78, s.ad_value(41), 1.0, s.ad_value(5), (-p.p37), s.ad_value(76), 1.0);
            s.store_add_scaled_product_right_ad(21, 80, p.p26, 32, A::add_scaled_product(s.ad_value(80), (2.0 * p.p37), A::add_scaled_inputs3(s.ad_value(25), 1.0, s.ad_value(75), 1.0, s.ad_value(78), -1.0), s.ad_value(69), 1.0 / (p.p36)), 1.0);
            s.store_scalar(18, A::ddx_projection(&s.ad_value(20), Some(8), None));
            s.store_scalar(19, A::ddx_projection(&s.ad_value(21), Some(7), None));
        }

        s.b[97] = (p.p6 == 2.0);
        s.v[97] = if s.b[97] { 1.0 } else { 0.0 };

        s.b[102] = (p.p42 > 0.0);
        s.v[102] = if s.b[102] { 1.0 } else { 0.0 };

        s.b[103] = (p.p50 > 0.0);
        s.v[103] = if s.b[103] { 1.0 } else { 0.0 };

        s.b[105] = ((p.p43 > 0.0) || (p.p44 > 0.0));
        s.v[105] = if s.b[105] { 1.0 } else { 0.0 };

        s.b[106] = (p.p48 > 0.0);
        s.v[106] = if s.b[106] { 1.0 } else { 0.0 };

        s.b[107] = (p.p7 == 0.0);
        s.v[107] = if s.b[107] { 1.0 } else { 0.0 };

        s.b[108] = (p.p7 == 1.0);
        s.v[108] = if s.b[108] { 1.0 } else { 0.0 };

        if ((s.b[108] && (!s.b[107])) && (p.p0 != 0.0)) {
            s.store_scaled_mul(120, 11, 31, (((4.0 * 1.3806503e-23) * p.p73) * (((p.p72 * p.p71)) as f64).sqrt()));
            s.store_scale(118, 120, 3.141592653589793);
        }

        s.b[124] = ((p.p1 != 0.0) && (p.p57 != 0.0));
        s.v[124] = if s.b[124] { 1.0 } else { 0.0 };

        Self::stamp_reactive_equations_block_0(ctx, stamper, s, p, nodes, branches, multiplicity);
    }
}
